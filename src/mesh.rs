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
    pub vertices: Vec<MeshVertex>,
    pub indicies: Vec<u32>,
    pub texture: super::video::surface::Surface,
}

impl Mesh {
    pub fn new(vertices: Vec<MeshVertex>, indicies: Vec<u32>, surface: super::video::surface::Surface) -> Self {
        Self { vertices, indicies, texture: surface }
    }

    pub fn new_from_texture_file(vertices: Vec<MeshVertex>, indicies: Vec<u32>, texture: &str) -> Self {
        let used_texture = super::video::surface::Surface::from_texture(&texture);
        Self { vertices, indicies, texture: used_texture }
    }
}
