use std::collections::HashMap;

use crate::math::Coordinate2D;

pub struct Character {
    pub crt: String,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

pub struct Font {
    pub character_list: Vec<Character>,
    pub uvs: HashMap<String, [Coordinate2D; 4]>,
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

    pub fn return_character_from_string(&self, crt: &str) -> Result<&Character, String> {
        for character in &self.character_list {
            if character.crt == crt {
                return Ok(&character)
            }
        }
        Err("no character with such letter".to_string())
    }
}