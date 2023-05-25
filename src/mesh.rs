use std::ptr;

use crate::utils;

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
