use crate::{mesh::Mesh, video::surface::TileSet};
use crate::video::surface::Surface;

use super::video::{ surface, CatEngineShader };

pub enum Sprite<'a> {
    Surface(f32, f32, Surface, CatEngineShader),
    Tile(f32, f32, &'a TileSet, u32, CatEngineShader),
}