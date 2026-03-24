use crate::{mesh::Mesh, video::surface::TileSet};
use crate::video::surface::Surface;

use super::video::{ surface, CatEngineShader };

pub enum Sprite<'a> {
    Surface(f32, f32, Surface, CatEngineShader),
    Tile(f32, f32, &'a TileSet, u32, CatEngineShader),
}

pub struct SpriteList<'a> {
    pub sprite_list: Vec<Sprite<'a>>,
    pub tile_set: TileSet,
}

impl SpriteList<'_> {
    pub fn new(mut sprite_list: Vec<Sprite>) {
        let (mut size_x, mut size_y) = (0, 0);

        for sprite in &sprite_list {
            match sprite {
                Sprite::Surface(x, y, surface, shader) => {
                    size_x += surface.width as i32;
                    size_y = {
                        if size_y > surface.height {
                            size_y
                        }
                        else {
                            surface.height
                        }
                    }
                }
                Sprite::Tile(x, y, tile_set, tile, _) => {
                    size_x += tile_set.tile_list[*tile as usize].width as i32;
                    size_y = {
                        if size_y > tile_set.tile_list[*tile as usize].height as u32  {
                            size_y
                        }
                        else {
                            tile_set.tile_list[*tile as usize].height 
                        }
                    }
                }
            }
        }
        
        let mut tile_set = TileSet::new(size_x as usize, size_y as usize);
        let mut offset_x_counter = 0;

        for sprite in sprite_list.iter_mut() {
            match sprite {
                Sprite::Surface(x, y, surface, shader) => {
                    tile_set.blit(&surface, offset_x_counter, 0);
                    offset_x_counter += surface.width;
                }
                Sprite::Tile(x, y, tile_set:&'a mut TileSet, u32, CatEngineShader) => {
                    let used_tile = &tile_set.tile_list[*tile as usize];
                    tile_set.blit(&tile_set.surface.return_true_crop(used_tile.x, used_tile.y, used_tile.width, used_tile.height), offset_x_counter, 0);
                    offset_x_counter += tile_set.tile_list[*tile as usize].width;
                }
            }
        }
    }
}