use crate::{CatEngine, sprite};
use crate::video::surface::{ TileSet, Tile };
use crate::video::surface::Surface;
use std::rc::{ Rc, Weak };
use std::cell::RefCell;
use super::video:: CatEngineShader ;

pub enum ComplexSprite<'a> {
    Surface(f32, f32, f32, f32, Surface, CatEngineShader),
    Tile(f32, f32, f32, f32, &'a TileSet, u32, CatEngineShader),
}

pub struct ComplexSpriteList<'a> {
    pub sprite_list: Vec<(ComplexSprite<'a>, u32)>,
    pub tile_set: TileSet,
}

impl<'a> ComplexSpriteList<'a> {
    pub fn new(mut sprite_list: Vec<ComplexSprite<'a>>) -> Self {
        let (mut size_x, mut size_y): (u32, u32) = (0, 0);

        for sprite in &sprite_list {
            match &sprite {
                ComplexSprite::Surface(x, y, _, _, surface, shader) => {
                    size_x = size_x.saturating_add(surface.width);
                    size_y = size_y.max(surface.height);
                }
                ComplexSprite::Tile(x, y, _, _, tile_set, tile, _) => {
                    let t = &tile_set.tile_list[*tile as usize];
                    size_x = size_x.saturating_add(t.width);
                    size_y = size_y.max(t.height as u32);
                }
            }
        }
        
        let mut main_tile_set = TileSet::new(size_x as usize, size_y as usize);
        let mut offset_x_counter = 0;
        let mut true_sprite_list: Vec<(ComplexSprite<'a>, u32)> = vec![];

        for sprite in sprite_list {
            true_sprite_list.push((sprite, 0));
        }

        for sprite in &mut true_sprite_list {
            match &sprite.0 {
                ComplexSprite::Surface(x, y, _, _, surface, _) => {
                    main_tile_set.blit(&surface, offset_x_counter, 0);
                    sprite.1 = main_tile_set.simple_append_tile(offset_x_counter, 0, 0.0, surface.width, surface.height);
                    offset_x_counter += surface.width;
                }
                ComplexSprite::Tile(x, y, _, _, tile_set, tile, _) => {
                    let used_tile = &tile_set.tile_list[*tile as usize];
                    main_tile_set.blit(&tile_set.surface.return_true_crop(used_tile.x, used_tile.y, used_tile.width, used_tile.height), offset_x_counter, 0);
                    sprite.1 = main_tile_set.simple_append_tile(offset_x_counter, 0, 0.0, used_tile.width, used_tile.height);
                    offset_x_counter += tile_set.tile_list[*tile as usize].width;
                }
            }
        }
        Self { sprite_list: true_sprite_list, tile_set: main_tile_set }
    }
}

pub enum Sprite {
    Surface(f32, f32, f32, Rc<RefCell<Surface>>, CatEngineShader, bool),
    Tile(f32, f32, f32, Rc<RefCell<TileSet>>, u32, CatEngineShader, bool),
}


impl Sprite {
    pub fn get_x(&self) -> f32 {
        match self {
            Sprite::Surface(x, _, _, _, _, _) => *x,
            Sprite::Tile(x, _, _, _, _, _, _) => *x,
        }
    }
    
    pub fn get_y(&self) -> f32 {
        match self {
            Sprite::Surface(_, y, _, _, _, _,) => *y,
            Sprite::Tile(_, y, _, _, _, _, _) => *y,
        }
    }

    pub fn get_z(&self) -> f32 {
        match self {
            Sprite::Surface(_, _, z, _, _, _) => *z,
            Sprite::Tile(_, _, z, _, _, _, _) => *z,
        }
    }

    pub fn get_width(&self) -> f32 {
        match self {
            Sprite::Surface(_, _, _, surface, _, _) => surface.borrow().width as f32,
            Sprite::Tile(_, _, _, tile_set, tile, _, _) => tile_set.borrow().tile_list[*tile as usize].visual_width as f32,
        }
    }
    
    pub fn get_height(&self) -> f32 {
        match self {
            Sprite::Surface(_, _, _, surface, _, _) => surface.borrow().height as f32,
            Sprite::Tile(_, _, _, tile_set, tile, _, _) => tile_set.borrow().tile_list[*tile as usize].visual_height as f32,
        }
    }

    pub fn get_ysort(&self) -> bool {
        match self {
            Sprite::Surface(_, _, _, _, _, ysort) => *ysort,
            Sprite::Tile(_, _, _, _, _, _, ysort) => *ysort,
        }
    }

    pub fn set_z(&mut self, z: f32) {
        match self {
            Sprite::Surface(_, _, t, _, _, _) => *t = z,
            Sprite::Tile(_, _, t, _, _, _, _) => *t = z,
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

    pub fn sort_by_z(&mut self) {
        self.sprite_list.sort_by(|a, b| {
            (a.get_z()).partial_cmp(&b.get_z()).unwrap()
        });
    }
}