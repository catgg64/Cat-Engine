use std::collections::HashMap;

use crate::{math::Coordinate2D, video::surface::Surface};

pub struct Character {
    pub crt: char,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl Clone for Character {
    fn clone(&self) -> Self {
        Self { crt: self.crt.clone(), x: self.x, y: self.y, width: self.width, height: self.height }
    }
}

pub struct Font {
    pub character_list: Vec<Character>,
    pub uvs: HashMap<char, [Coordinate2D; 4]>,
    pub surface: Surface,
}

impl Font {
    //! # Font
    //! 
    //! CatEngine's implementation of a font. Should be feed characters individually, and there is no ttf support yet.
    pub fn new(path: &str, character_list: Vec<Character>) -> Self {
        let mut surface = Surface::from_texture(path);
        let mut uvs: HashMap<char, [Coordinate2D; 4]> = HashMap::new();
        for character in &character_list {
            let w = surface.width as f32;
            let h = surface.height as f32;

            uvs.insert(character.crt.clone(), [
                Coordinate2D(character.x as f32 / w, character.y as f32 / h),
                Coordinate2D((character.x + character.width) as f32 / w, character.y as f32 / h),
                Coordinate2D((character.x + character.width) as f32 / w, (character.y + character.height) as f32 / h),
                Coordinate2D(character.x as f32 / w, (character.y + character.height) as f32 / h),
            ]);
        }
        Self { character_list, uvs, surface }
    }

    pub fn return_character_from_string(&self, crt: char) -> Result<&Character, String> {
        for character in &self.character_list {
            if character.crt as char == crt {
                return Ok(&character)
            }
        }
        Err(format!("no character with such letter: {}", crt))
    }

    pub fn size(&self, text: &str, font_size: f32, spacement: f32) -> (f32, f32) {
        let mut width = 0.0;
        let mut height = 0.0;
        
        for ch in text.chars() {
            if self.return_character_from_string(ch).unwrap().height as f32 > height {
                height = self.return_character_from_string(ch).unwrap().height as f32 * font_size
            }

            width += self.return_character_from_string(ch).unwrap().width as f32 * font_size + spacement
        }

        (width, height)
    }
}