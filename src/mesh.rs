use std::ptr;

use crate::{material::Material, triangle::Triangle, utils, vertex::Vertex};

pub struct MeshMaterial {
    pub ambient: Vec<f32>,
    pub diffuse: Vec<f32>,
    pub specular: Vec<f32>,
    pub shininess: Vec<f32>,
}

pub struct Mesh {
    pub vertices: Vec<f32>,
    pub indices: Vec<u32>,
    pub normals: Vec<f32>,

    pub material: MeshMaterial,

    pub index_count: i32,
}

impl Mesh {
    pub fn mesh_from_height_map(height_map: &Vec<Vec<f64>>) -> Mesh {
        let width = height_map.len() as u32;
        let height = height_map[0].len() as u32;

        let mut shape_vertices: Vec<Vertex> = Vec::new();
        let mut shape_triangles: Vec<Triangle> = Vec::new();

        let mut vertices: Vec<f32> = Vec::new();
        let mut normals: Vec<f32> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        for x in 0..width {
            for z in 0..height {
                let vertex_height = height_map[x as usize][z as usize] as f32;

                let vertex = Vertex {
                    position: glm::vec3(x as f32, vertex_height, z as f32),
                    material: Material::standard_material(),
                };

                shape_vertices.push(vertex);

                if x < width - 1 && z < height - 1 {
                    let triangle_1 = Triangle::new(
                        (x * height + (z + 1)) as usize,
                        ((x + 1) * height + z) as usize,
                        (x * height + z) as usize,
                    );

                    let triangle_2 = Triangle::new(
                        (x * height + (z + 1)) as usize,
                        ((x + 1) * height + (z + 1)) as usize,
                        ((x + 1) * height + z) as usize,
                    );

                    shape_triangles.push(triangle_1);
                    shape_triangles.push(triangle_2);
                }
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
        };
        mesh
    }
    pub unsafe fn create_vao(&self) -> u32 {
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

        vao_ids
    }
}
