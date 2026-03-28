use std::collections::HashMap;

use crate::math::Coordinate2D;

pub struct Character {
    crt: String,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

pub struct Font {
    character_list: Vec<Character>,
    uvs: HashMap<String, [Coordinate2D; 4]>,
}

impl Font {
    pub fn new(character_list: Vec<Character>) -> Self {
        let mut uvs: HashMap<String, [Coordinate2D; 4]> = HashMap::new();
        for character in &character_list {
            uvs.insert(character.crt.clone(), [
                Coordinate2D{0: character.x as f32, 1: character.y as f32},
                Coordinate2D{0: character.x as f32 + character.width as f32, 1: character.y as f32},
                Coordinate2D{0: character.x as f32 + character.width as f32, 1: character.y as f32 + character.height as f32},
                Coordinate2D{0: character.x as f32, 1: character.y as f32 + character.height as f32},
            ]);
        }
        Self { character_list, uvs }
    }
}