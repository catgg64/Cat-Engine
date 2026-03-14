use crate::math::{Coordinate2D, Coordinate3D};

pub struct MeshVertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub uv: (f32, f32),
}

impl MeshVertex {
    pub fn new(x: f32, y: f32, z: f32, uv: (f32, f32)) -> Self {
        MeshVertex { x, y, z, uv }
    }
}

pub struct Mesh {
    pub vao: u32,
    pub vbo: u32,
    pub ebo: u32,
    pub uv_vbo: u32,
    pub vertices: Vec<MeshVertex>,
    pub indicies: Vec<u32>,
    pub texture: super::video::surface::Surface,
}

impl Mesh {
    pub fn new(vertices: Vec<MeshVertex>, indicies: Vec<u32>, surface: super::video::surface::Surface) -> Self {
        let (mut vao, mut vbo, mut ebo, mut uv_vbo) = super::video::start_uv_3d_elemnt_array(3, 2);
        let mut uvs = Vec::new();
        let mut actual_verticies = Vec::new();
        for vertex in vertices.iter() {
            actual_verticies.push(Coordinate3D{0: vertex.x, 1: vertex.y, 2: vertex.z});
            uvs.push(Coordinate2D{ 0: vertex.uv.0, 1: vertex.uv.1 });
        }
        super::video::update_uv_3d_element_array(&mut vao, &mut vbo, &mut ebo, &mut uv_vbo, actual_verticies, uvs, &indicies);
        Self { vao, vbo, ebo, uv_vbo, vertices, indicies, texture: surface }
    }

    pub fn new_from_texture_file(vertices: Vec<MeshVertex>, indicies: Vec<u32>, texture: &str) -> Self {
        let (mut vao, mut vbo, mut ebo, mut uv_vbo) = super::video::start_uv_3d_elemnt_array(3, 2);
        let mut uvs = Vec::new();
        let mut actual_verticies = Vec::new();
        for vertex in vertices.iter() {
            actual_verticies.push(Coordinate3D{0: vertex.x, 1: vertex.y, 2: vertex.z});
            uvs.push(Coordinate2D{ 0: vertex.uv.0, 1: vertex.uv.1 });
        }
        super::video::update_uv_3d_element_array(&mut vao, &mut vbo, &mut ebo, &mut uv_vbo, actual_verticies, uvs, &indicies);
        let used_texture = super::video::surface::Surface::from_texture(&texture);
        Self { vao, vbo, ebo, uv_vbo, vertices, indicies, texture: used_texture }
    }
}
