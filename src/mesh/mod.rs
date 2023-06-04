pub mod mesh_settings;

use std::ptr;

use crate::{
    material::Material,
    noise_map::{noise_map_settings::NoiseMapSettings, NoiseMap},
    triangle::Triangle,
    utils,
    vertex::Vertex,
};

use self::mesh_settings::MeshSettings;

#[derive(Clone)]
pub struct MeshMaterial {
    pub ambient: Vec<f32>,
    pub diffuse: Vec<f32>,
    pub specular: Vec<f32>,
    pub shininess: Vec<f32>,
}

#[derive(Clone)]
pub struct Mesh {
    pub vertices: Vec<f32>,
    pub indices: Vec<u32>,
    pub normals: Vec<f32>,

    pub material: MeshMaterial,

    pub index_count: i32,

    buffer_ids: Vec<u32>,
}

impl Mesh {
    pub fn create_terrain_mesh(
        materials: &Vec<Material>,
        noise_map_settings: &NoiseMapSettings,
        settings: &MeshSettings,
    ) -> Mesh {
        let map_chunk_size = 241;

        let noise_map = NoiseMap::new(*noise_map_settings);
        let height_map = noise_map.get_height_map();

        let mesh_simplification_increment = if settings.level_of_detail == 0 {
            1
        } else {
            settings.level_of_detail * 2
        };

        let vertices_per_line = (map_chunk_size - 1) / mesh_simplification_increment + 1;

        let mut shape_vertices: Vec<Vertex> = Vec::new();
        let mut shape_triangles: Vec<Triangle> = Vec::new();

        let mut vertices: Vec<f32> = Vec::new();
        let mut normals: Vec<f32> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        let top_left_x = (map_chunk_size - 1) as f32 / -2.0;
        let top_left_z = (map_chunk_size - 1) as f32 / 2.0;

        let mut vertex_index = 0;

        for z in (0..map_chunk_size).step_by(mesh_simplification_increment as usize) {
            for x in (0..map_chunk_size).step_by(mesh_simplification_increment as usize) {
                let noise_height = height_map[x as usize][z as usize] as f32;
                let vertex_height =
                    settings.curve.evaluate(noise_height as f64) as f32 * settings.strength;

                for material in materials.iter() {
                    if noise_height <= material.height_limit {
                        let vertex = Vertex {
                            position: glm::vec3(
                                top_left_x + x as f32,
                                vertex_height,
                                top_left_z - z as f32,
                            ),
                            material: *material,
                        };

                        shape_vertices.push(vertex);
                        break;
                    }
                }

                if x < map_chunk_size - 1 && z < map_chunk_size - 1 {
                    let triangle_1 = Triangle::new(
                        (vertex_index) as usize,
                        (vertex_index + vertices_per_line + 1) as usize,
                        (vertex_index + vertices_per_line) as usize,
                    );

                    let triangle_2 = Triangle::new(
                        (vertex_index + vertices_per_line + 1) as usize,
                        (vertex_index) as usize,
                        (vertex_index + 1) as usize,
                    );

                    shape_triangles.push(triangle_1);
                    shape_triangles.push(triangle_2);
                }
                vertex_index += 1;
            }
        }
        let mut vertex_normals: Vec<glm::Vec3> =
            vec![glm::vec3(0.0, 0.0, 0.0); shape_vertices.len()];

        for triangle in &shape_triangles {
            let vertex_1 = shape_vertices[triangle.a];
            let vertex_2 = shape_vertices[triangle.b];
            let vertex_3 = shape_vertices[triangle.c];

            let ab = vertex_2.position - vertex_1.position;
            let ac = vertex_3.position - vertex_1.position;
            let triangle_normal = glm::cross(&ab, &ac);

            vertex_normals[triangle.a] += triangle_normal;
            vertex_normals[triangle.b] += triangle_normal;
            vertex_normals[triangle.c] += triangle_normal;
        }

        for normal in &vertex_normals {
            let normalized_normal = glm::normalize(normal);
            normals.extend(&normalized_normal);
        }

        for triangle in &shape_triangles {
            indices.extend_from_slice(&[triangle.a as u32, triangle.b as u32, triangle.c as u32]);
        }

        for vertex in &shape_vertices {
            vertices.extend(&vertex.position);
        }

        let mut mesh_material = MeshMaterial {
            ambient: Vec::new(),
            diffuse: Vec::new(),
            specular: Vec::new(),
            shininess: Vec::new(),
        };

        for vertex in &shape_vertices {
            mesh_material.ambient.extend(&vertex.material.ambient);
            mesh_material.diffuse.extend(&vertex.material.diffuse);
            mesh_material.specular.extend(&vertex.material.specular);
            mesh_material.shininess.push(vertex.material.shininess);
        }

        let mesh = Mesh {
            vertices,
            indices,
            normals,
            material: mesh_material,

            index_count: shape_triangles.len() as i32 * 3,

            buffer_ids: Vec::new(),
        };
        mesh
    }

    pub fn delete_buffers(&mut self) {
        unsafe {
            gl::DeleteBuffers(self.buffer_ids.len() as i32, self.buffer_ids.as_ptr());
        }
    }
    pub unsafe fn create_vao(&mut self) -> u32 {
        let mut vao_ids: u32 = 0;
        gl::GenVertexArrays(1, &mut vao_ids as *mut u32);
        gl::BindVertexArray(vao_ids);

        let mut vbo_ids: u32 = 0;
        gl::GenBuffers(1, &mut vbo_ids as *mut u32);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_ids);

        gl::BufferData(
            gl::ARRAY_BUFFER,
            utils::byte_size_of_array(&self.vertices),
            utils::pointer_to_array(&self.vertices),
            gl::STATIC_DRAW,
        );

        self.buffer_ids.push(vbo_ids);

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            utils::size_of::<f32>() * 3,
            ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        //Ambient buffer
        let mut ambient_vbo_ids: u32 = 1;
        gl::GenBuffers(1, &mut ambient_vbo_ids as *mut u32);
        gl::BindBuffer(gl::ARRAY_BUFFER, ambient_vbo_ids);

        gl::BufferData(
            gl::ARRAY_BUFFER,
            utils::byte_size_of_array(&self.material.ambient),
            utils::pointer_to_array(&self.material.ambient),
            gl::STATIC_DRAW,
        );

        self.buffer_ids.push(ambient_vbo_ids);

        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            utils::size_of::<f32>() * 3,
            ptr::null(),
        );
        gl::EnableVertexAttribArray(1);

        //Diffuse buffer
        let mut diffuse_vbo_ids: u32 = 2;
        gl::GenBuffers(1, &mut diffuse_vbo_ids as *mut u32);
        gl::BindBuffer(gl::ARRAY_BUFFER, diffuse_vbo_ids);

        gl::BufferData(
            gl::ARRAY_BUFFER,
            utils::byte_size_of_array(&self.material.diffuse),
            utils::pointer_to_array(&self.material.diffuse),
            gl::STATIC_DRAW,
        );

        self.buffer_ids.push(diffuse_vbo_ids);

        gl::VertexAttribPointer(
            2,
            3,
            gl::FLOAT,
            gl::FALSE,
            utils::size_of::<f32>() * 3,
            ptr::null(),
        );
        gl::EnableVertexAttribArray(2);

        //Specular buffer
        let mut specular_vbo_ids: u32 = 3;
        gl::GenBuffers(1, &mut specular_vbo_ids as *mut u32);
        gl::BindBuffer(gl::ARRAY_BUFFER, specular_vbo_ids);

        gl::BufferData(
            gl::ARRAY_BUFFER,
            utils::byte_size_of_array(&self.material.specular),
            utils::pointer_to_array(&self.material.specular),
            gl::STATIC_DRAW,
        );

        self.buffer_ids.push(specular_vbo_ids);

        gl::VertexAttribPointer(
            3,
            3,
            gl::FLOAT,
            gl::FALSE,
            utils::size_of::<f32>() * 3,
            ptr::null(),
        );
        gl::EnableVertexAttribArray(3);

        //Shininess buffer
        let mut shininess_vbo_ids: u32 = 4;
        gl::GenBuffers(1, &mut shininess_vbo_ids as *mut u32);
        gl::BindBuffer(gl::ARRAY_BUFFER, shininess_vbo_ids);

        gl::BufferData(
            gl::ARRAY_BUFFER,
            utils::byte_size_of_array(&self.material.shininess),
            utils::pointer_to_array(&self.material.shininess),
            gl::STATIC_DRAW,
        );

        self.buffer_ids.push(shininess_vbo_ids);

        gl::VertexAttribPointer(
            4,
            1,
            gl::FLOAT,
            gl::FALSE,
            utils::size_of::<f32>() * 1,
            ptr::null(),
        );
        gl::EnableVertexAttribArray(4);

        //Normal buffer
        let mut normvec_vbo_ids: u32 = 5;
        gl::GenBuffers(1, &mut normvec_vbo_ids as *mut u32);
        gl::BindBuffer(gl::ARRAY_BUFFER, normvec_vbo_ids);

        gl::BufferData(
            gl::ARRAY_BUFFER,
            utils::byte_size_of_array(&self.normals),
            utils::pointer_to_array(&self.normals),
            gl::STATIC_DRAW,
        );

        self.buffer_ids.push(normvec_vbo_ids);

        gl::VertexAttribPointer(
            5,
            3,
            gl::FLOAT,
            gl::FALSE,
            utils::size_of::<f32>() * 3,
            ptr::null(),
        );
        gl::EnableVertexAttribArray(5);

        let mut ibo_ids: u32 = 0;
        gl::GenBuffers(1, &mut ibo_ids as *mut u32);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo_ids);

        // * Fill it with data
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            utils::byte_size_of_array(&self.indices),
            utils::pointer_to_array(&self.indices),
            gl::STATIC_DRAW,
        );

        self.buffer_ids.push(ibo_ids);

        vao_ids
    }
}
