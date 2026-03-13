pub struct MeshVertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub uv: (f32, f32, f32),
}

impl MeshVertex {
    pub fn new(x: f32, y: f32, z: f32, uv: (f32, f32, f32)) -> Self {
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
        let (vao, vbo, ebo, uv_vbo) = super::video::start_uv_3d_elemnt_array();
        Self { vao, vbo, ebo, uv_vbo, vertices, indicies, texture: surface }
    }

    pub fn new_from_texture_file(vertices: Vec<MeshVertex>, indicies: Vec<u32>, texture: &str) -> Self {
        let (vao, vbo, ebo, uv_vbo) = super::video::start_uv_3d_elemnt_array();
        let used_texture = super::video::surface::Surface::from_texture(&texture);
        Self { vao, vbo, ebo, uv_vbo, vertices, indicies, texture: used_texture }
    }
}
