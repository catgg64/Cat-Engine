use crate::sprite;
use crate::video::surface::{ TileSet, Tile };
use crate::video::surface::Surface;

use super::video:: CatEngineShader ;

pub enum Sprite<'a> {
    Surface(f32, f32, f32, f32, Surface, CatEngineShader),
    Tile(f32, f32, f32, f32, &'a TileSet, u32, CatEngineShader),
}

pub struct SpriteList<'a> {
    pub sprite_list: Vec<(Sprite<'a>, u32)>,
    pub tile_set: TileSet,
}

impl<'a> SpriteList<'a> {
    pub fn new(mut sprite_list: Vec<Sprite<'a>>) -> Self {
        let (mut size_x, mut size_y) = (0, 0);

        for sprite in &sprite_list {
            match &sprite {
                Sprite::Surface(x, y, _, _, surface, shader) => {
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
                Sprite::Tile(x, y, _, _, tile_set, tile, _) => {
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
        
        let mut main_tile_set = TileSet::new(size_x as usize, size_y as usize);
        let mut offset_x_counter = 0;
        let mut true_sprite_list: Vec<(Sprite<'a>, u32)> = vec![];

        for sprite in sprite_list {
            true_sprite_list.push((sprite, 0));
        }

        for sprite in &mut true_sprite_list {
            match &sprite.0 {
                Sprite::Surface(x, y, _, _, surface, _) => {
                    main_tile_set.blit(&surface, offset_x_counter, 0);
                    sprite.1 = main_tile_set.simple_append_tile(offset_x_counter, 0, surface.width, surface.height);
                    offset_x_counter += surface.width;
                }
                Sprite::Tile(x, y, _, _, tile_set, tile, _) => {
                    let used_tile = &tile_set.tile_list[*tile as usize];
                    main_tile_set.blit(&tile_set.surface.return_true_crop(used_tile.x, used_tile.y, used_tile.width, used_tile.height), offset_x_counter, 0);
                    sprite.1 = main_tile_set.simple_append_tile(offset_x_counter, 0, used_tile.width, used_tile.height);
                    offset_x_counter += tile_set.tile_list[*tile as usize].width;
                }
            }
        }
        Self { sprite_list: true_sprite_list, tile_set: main_tile_set }
    }
}