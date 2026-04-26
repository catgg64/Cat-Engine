use crate::math::Coordinate3D;
use crate::{CatEngine, sprite};
use crate::video::surface::{ TileSet, Tile };
use crate::video::surface::Surface;
use std::ops::Add;
use std::rc::{ Rc, Weak };
use std::cell::RefCell;
use super::video:: CatEngineShader ;

pub enum Sprite {
    Surface(f32, f32, f32, Rc<RefCell<Surface>>, CatEngineShader, bool, f32),
    Tile(f32, f32, f32, Rc<RefCell<TileSet>>, Tile, CatEngineShader, bool, f32),
    Batch(Rc<RefCell<TileSet>>, Vec<(Tile, f32, f32, bool, f32)>)
}


impl Sprite {
    pub fn get_x(&self) -> f32 {
        match self {
            Sprite::Surface(x, _, _, _, _, _, _) => *x,
            Sprite::Tile(x, _, _, _, _, _, _, _) => *x,
            Sprite::Batch(tile_set, tiles) => {
                let mut lowest: f32 = 0.0;
                let mut vertices: Vec<Coordinate3D> = vec![];
                let borrowed_tileset = tile_set.borrow();

                for tile in tiles.iter() {
                    if tile.3 {
                        vertices.push(Coordinate3D(tile.0.vertices[0].0 + tile.1, tile.0.vertices[0].1 + tile.2, tile.0.vertices[0].1 + tile.2 + tile.4));
                        vertices.push(Coordinate3D(tile.0.vertices[1].0 + tile.1, tile.0.vertices[1].1 + tile.2, tile.0.vertices[0].1 + tile.2 + tile.4));
                        vertices.push(Coordinate3D(tile.0.vertices[2].0 + tile.1, tile.0.vertices[2].1 + tile.2, tile.0.vertices[0].1 + tile.2 + tile.4));
                        vertices.push(Coordinate3D(tile.0.vertices[3].0 + tile.1, tile.0.vertices[3].1 + tile.2, tile.0.vertices[0].1 + tile.2 + tile.4));
                    } else {
                        vertices.push(Coordinate3D(tile.0.vertices[0].0 + tile.1, tile.0.vertices[0].1 + tile.2, tile.0.vertices[0].2));
                        vertices.push(Coordinate3D(tile.0.vertices[1].0 + tile.1, tile.0.vertices[1].1 + tile.2, tile.0.vertices[1].2));
                        vertices.push(Coordinate3D(tile.0.vertices[2].0 + tile.1, tile.0.vertices[2].1 + tile.2, tile.0.vertices[2].2));
                        vertices.push(Coordinate3D(tile.0.vertices[3].0 + tile.1, tile.0.vertices[3].1 + tile.2, tile.0.vertices[3].2));
                    }
                }

                for vertex in vertices.iter() {
                    if vertex.0 < lowest {
                        lowest = vertex.0
                    };
                }

                lowest
            },
        }
    }
    
    pub fn get_y(&self) -> f32 {
        match self {
            Sprite::Surface(_, y, _, _, _, _, _) => *y,
            Sprite::Tile(_, y, _, _, _, _, _, _) => *y,
            Sprite::Batch(_, _) => {0.0},
        }
    }

    pub fn get_z(&self) -> f32 {
        match self {
            Sprite::Surface(_, _, z, _, _, _, _) => *z,
            Sprite::Tile(_, _, z, _, _, _, _, _) => *z,
            Sprite::Batch(_, _) => {0.0},
        }
    }

    pub fn get_width(&self) -> f32 {
        match self {
            Sprite::Surface(_, _, _, surface, _, _, _) => surface.borrow().width as f32,
            Sprite::Tile(_, _, _, tile_set, tile, _, _, _) => tile.visual_width as f32,
            Sprite::Batch(_, _) => {0.0},
        }
    }
    
    pub fn get_height(&self) -> f32 {
        match self {
            Sprite::Surface(_, _, _, surface, _, _, _) => surface.borrow().height as f32,
            Sprite::Tile(_, _, _, tile_set, tile, _, _, _) => tile.visual_height as f32,
            Sprite::Batch(_, _) => {0.0},
        }
    }

    pub fn get_ysort(&self) -> bool {
        match self {
            Sprite::Surface(_, _, _, _, _, ysort, _) => *ysort,
            Sprite::Tile(_, _, _, _, _, _, ysort, _) => *ysort,
            Sprite::Batch(_, _) => {false},
        }
    }

    pub fn set_z(&mut self, z: f32) {
        match self {
            Sprite::Surface(_, _, t, _, _, _, _) => *t = z,
            Sprite::Tile(_, _, t, _, _, _, _, _) => *t = z,
            Sprite::Batch(_, _) => {},
        }
    }

    pub fn is_not_batch(& self) -> bool {
        match self {
            Sprite::Surface(_, _, _, _, _, _, _) => true,
            Sprite::Tile(_, _, _, _, _, _, _, _) => true,
            Sprite::Batch(_, _) => {false},
        }
    }
}

impl Clone for Sprite {
    fn clone(&self) -> Self {
        match self {
            Self::Surface(x, y, z, surface, shader, ysort, ysort_origin) => {
                Self::Surface(x.clone(), y.clone(), z.clone(), surface.clone(), CatEngineShader::TextureShader, ysort.clone(), ysort_origin.clone())
            }
            Self::Tile(x, y, z, tile_set, tile, shader, ysort, ysort_origin) => {
                Self::Tile(x.clone(), y.clone(), z.clone(), tile_set.clone(), tile.clone(), CatEngineShader::TextureShader, ysort.clone(), ysort_origin.clone())
            }
            Self::Batch(tile_set, tiles) => {
                Self::Batch(tile_set.clone(), tiles.clone())
            }
        }
    }
}

pub struct SpriteList {
    pub sprite_list: Vec<Sprite>
}

impl SpriteList {
    pub fn new() -> Self {
        SpriteList { sprite_list: vec![] }
    }

    pub fn update(&mut self, mut sprite_list: Vec<Sprite>) {
        for sprite in sprite_list.iter_mut() {
            if sprite.get_ysort() {
                sprite.set_z(sprite.get_y() + sprite.get_height());
            }
        }
        self.sprite_list = sprite_list;
    }

    /// Clones the value instead of moving it.
    pub fn clone_update(&mut self, mut sprite_list: Vec<Sprite>) {
        for sprite in sprite_list.iter_mut() {
            if sprite.get_ysort() {
                sprite.set_z(sprite.get_y() + sprite.get_height());
            }
        }
        self.sprite_list = sprite_list.clone();
    }

    pub fn sort_by_z(&mut self) {
        self.sprite_list.sort_by(|a, b| {
            (a.get_z()).partial_cmp(&b.get_z()).unwrap()
        });
    }
}