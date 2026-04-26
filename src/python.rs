use std::{rc::Rc, cell::{RefCell, Ref}, vec};

use pyo3::prelude::*;
use crate::{
    pixel::Color,
    keyboard::{ Keycode, Scancode },
};

#[pyclass(unsendable)]
pub struct CatEngine {
    engine: crate::CatEngine,
}

#[pymethods]
impl CatEngine {
    #[new]
    fn new(title: String, width: u32, height: u32, flags: Vec<String>) -> PyResult<Self> {
        let mut true_flags = Vec::new();
        for flag in flags {
            match flag.as_str() {
                "vsync" => {true_flags.push(super::CatEngineFlag::Vsync)},
                "dynamicvsync" => {true_flags.push(super::CatEngineFlag::DynamicVsync)},
                "depth" => {true_flags.push(super::CatEngineFlag::DepthBuffer)},
                _ => {},
            }
        }
        let engine = crate::CatEngine::new(&title, width, height, true_flags)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e))?;
        Ok(Self { engine })
    }

    fn update(&mut self) {
        self.engine.update();
    }

    fn enable_fullscreen(&mut self) {
        self.engine.enable_fullscreen();
    }
    
    fn disable_fullscreen(&mut self) {
        self.engine.disable_fullscreen();
    }
    
    fn enable_true_fullscreen(&mut self) {
        self.engine.enable_true_fullscreen();
    }
    
    fn clear_screen(&self, r: u8, g: u8, b: u8, a: u8) {
        self.engine.clear_screen(crate::pixel::Color { r, g, b, a });
    }

    fn is_down_scancode(&self, key: String) -> PyResult<bool> {
        Ok(self.engine.input.is_down({
            match key.as_str() {
                "A" => {Scancode::A}
                "B" => {Scancode::B}
                "C" => {Scancode::C}
                "D" => {Scancode::D}
                "E" => {Scancode::E}
                "F" => {Scancode::F}
                "G" => {Scancode::G}
                "H" => {Scancode::H}
                "I" => {Scancode::I}
                "J" => {Scancode::J}
                "K" => {Scancode::K}
                "L" => {Scancode::L}
                "M" => {Scancode::M}
                "N" => {Scancode::N}
                "O" => {Scancode::O}
                "P" => {Scancode::P}
                "Q" => {Scancode::Q}
                "R" => {Scancode::R}
                "S" => {Scancode::S}
                "T" => {Scancode::T}
                "U" => {Scancode::U}
                "V" => {Scancode::V}
                "W" => {Scancode::W}
                "X" => {Scancode::X}
                "Y" => {Scancode::Y}
                "Z" => {Scancode::Z}
                "up" => {Scancode::Up}
                "down" => {Scancode::Down}
                "left" => {Scancode::Left}
                "right" => {Scancode::Right}
                "backspace" => {Scancode::Backspace}
                "escape" => {Scancode::Escape}
                _ => {
                    panic!("no key named {}", key);
                }
            }
        }))
    }

    fn is_down_keycode(&self, key: String) -> PyResult<bool> {
        Ok(self.engine.input.is_down_keycode({
            match key.as_str() {
                "A" => {Keycode::A}
                "B" => {Keycode::B}
                "C" => {Keycode::C}
                "D" => {Keycode::D}
                "E" => {Keycode::E}
                "F" => {Keycode::F}
                "G" => {Keycode::G}
                "H" => {Keycode::H}
                "I" => {Keycode::I}
                "J" => {Keycode::J}
                "K" => {Keycode::K}
                "L" => {Keycode::L}
                "M" => {Keycode::M}
                "N" => {Keycode::N}
                "O" => {Keycode::O}
                "P" => {Keycode::P}
                "Q" => {Keycode::Q}
                "R" => {Keycode::R}
                "S" => {Keycode::S}
                "T" => {Keycode::T}
                "U" => {Keycode::U}
                "V" => {Keycode::V}
                "W" => {Keycode::W}
                "X" => {Keycode::X}
                "Y" => {Keycode::Y}
                "Z" => {Keycode::Z}
                "up" => {Keycode::Up}
                "down" => {Keycode::Down}
                "left" => {Keycode::Left}
                "right" => {Keycode::Right}
                "backspace" => {Keycode::Backspace}
                "escape" => {Keycode::Escape}
                _ => {
                    panic!("no key named {}", key);
                }
            }
        }))
    }

    #[getter]
    fn mouse_state(&self) -> PyResult<(bool, bool, bool, bool, bool)>{
        let result = self.engine.input.mouse_buttons_down(&self.engine.event_pump);
        Ok((result["left"], result["middle"], result["right"], result["x1"], result["x2"]))
    }

    #[getter]
    fn mouse_pos(&self) -> (i32, i32) {
        self.engine.input.get_mouse_pos(&self.engine.event_pump)
    }

    #[getter]
    fn is_running(&self) -> bool {
        self.engine.running
    }

    fn start_text_input(&mut self) {
        self.engine.start_text_input();
    }
    
    fn stop_text_input(&mut self) {
        self.engine.stop_text_input();
    }

    #[getter]
    fn text_input(&self) -> PyResult<&String> {
        Ok(
            self.engine.input.get_input()
        )
    }
    
    #[getter]
    fn text_input_enabled(&self) -> PyResult<bool> {
        Ok(self.engine.get_text_input_enabled())
    }

    #[getter]
    fn text_input_char(&self) -> PyResult<&String> {
        Ok(self.engine.input.get_char_input())
    }

    #[getter]
    fn text_input_backspace(&self) -> PyResult<bool> {
        Ok(self.engine.input.get_backspace().clone())
    }

    #[getter]
    fn size(&self) -> PyResult<(u32, u32)> {
        Ok(self.engine.get_size())
    }

    #[getter]
    fn mouse_wheel_direction(&self) -> PyResult<(i32, i32)> {
        Ok(self.engine.input.get_mouse_wheel_direction().clone())
    }

    fn clear_text_input(&mut self) {
        self.engine.input.zero_input();
    }

    fn enable_depth_test(&self) {
        self.engine.enable_depth_test();
    }
    
    fn disable_depth_test(&self) {
        self.engine.disable_depth_test();
    }

    fn blit_surface(&mut self, surface: &Surface, pos_x: f32, pos_y: f32) {
        self.engine.renderer.blit(&surface.surface.borrow(), pos_x, pos_y);
    }
    
    fn blit_tileset(&mut self, tile: Tile, tile_set: &mut TileSet, pos_x: f32, pos_y: f32) {
        self.engine.renderer.draw_tileset(tile.tile, &mut tile_set.tile_set.borrow_mut(), pos_x, pos_y);
    }

    fn blit_sprite_list(&mut self, sprite_list: &mut SpriteList, x_offset: f32, y_offset: f32) {
        self.engine.renderer.draw_sprite_list(&mut sprite_list.sprite_list, x_offset, y_offset);
    } 

    fn blit_font(&mut self, font: &Font, text: &str, x: f32, y: f32, size: f32, spacement: u32) {
        self.engine.renderer.draw_font(&font.font, text, x, y, size, spacement);
    }

    fn blit_line(&mut self, p1: Coordinate2D, p2: Coordinate2D, r: u8, g: u8, b: u8, width: f32) {
        self.engine.renderer.draw_line(p1.into_coordinate_2d(), p2.into_coordinate_2d(), &Color{ r, g, b, a: 255}, width);
    }

    fn blit_rect(&mut self, rect: &Rect, r: u8, g: u8, b: u8, a: u8, width: f32) {
        self.engine.renderer.draw_rect(&rect.into_rect(), &Color{ r, g, b, a }, width);
    }
}

#[pyclass(unsendable)]
pub struct Surface {
    surface: Rc<RefCell<crate::video::surface::Surface>>,
}

#[pymethods]
impl Surface {
    #[new]
    fn new(width: usize, height: usize) -> PyResult<Self> {
        Ok(Self{ surface: Rc::new(RefCell::new(crate::video::surface::Surface::new(width, height))) })
    }

    #[staticmethod]
    fn from_texture(path: String) -> PyResult<Self> {
        Ok(Self {
            surface: Rc::new(RefCell::new(crate::video::surface::Surface::from_texture(&path)))
        })
    }

    fn upload(&mut self) {
        self.surface.borrow_mut().upload();
    }

    fn blit(&mut self, surface: &Surface, offset_x: u32, offset_y: u32) {
        self.surface.borrow_mut().blit(&surface.surface.borrow(), offset_x, offset_y);
    }

    fn crop(&mut self, x: u32, y: u32, width: u32, height: u32) {
        self.surface.borrow_mut().crop(x, y, width, height);
    }

    fn stretch(&mut self, width: i32, height: i32) {
        self.surface.borrow_mut().stretch(width, height);
    }

    fn fliph(&mut self) {
        self.surface.borrow_mut().fliph();
    }
    
    fn flipv(&mut self) {
        self.surface.borrow_mut().flipv();
    }

    fn bind(&self) {
        self.surface.borrow_mut().bind();
    }

    fn get_rect(&self) -> Rect {
        Rect{ x: self.surface.borrow().get_rect().x, y: self.surface.borrow().get_rect().y, width: self.surface.borrow().get_rect().width, height: self.surface.borrow().get_rect().height  }
    }

    #[getter]
    fn flipped_x(&self) -> PyResult<bool> {
        Ok(self.surface.borrow().flipped_x)
    }
    
    #[getter]
    fn flipped_y(&self) -> PyResult<bool> {
        Ok(self.surface.borrow().flipped_y)
    }
}

impl Surface {
    pub fn get_surface(&self) -> Ref<crate::video::surface::Surface> {
        self.surface.borrow()
    }
}

#[pyclass(unsendable)]
#[derive(Debug)]
pub struct Coordinate2D(pub f32, pub f32);

#[pymethods]
impl Coordinate2D {
    #[new]
    fn new(x: f32, y: f32) -> Self {
        Self{ 0: x, 1: y }
    }
}

impl Coordinate2D {
    pub fn into_coordinate_2d(&self) -> crate::math::Coordinate2D {
        crate::math::Coordinate2D{ 0: self.0, 1: self.1 }
    }
}

impl Clone for Coordinate2D {
    fn clone(&self) -> Self {
        Self(self.0, self.1)
    }
}

#[pyclass(unsendable)]
#[derive(Debug)]
pub struct Coordinate3D(pub f32, pub f32, pub f32);

impl Clone for Coordinate3D {
    fn clone(&self) -> Self {
        Self(self.0, self.1, self.2)
    }
}


#[pyclass(unsendable)]
pub struct Tile {
    tile: crate::video::surface::Tile,
}

impl Clone for Tile {
    fn clone(&self) -> Self {
        Self { tile: self.tile.clone() }
    }
}

#[pymethods]
impl Tile {
    #[staticmethod]
    fn new(py: Python, corners: Vec<(f32, f32)>, vertices: Vec<(f32, f32)>, x: u32, y: u32, z: f32, width: u32, height: u32) -> PyResult<Tile> {
        let corners: Vec<Coordinate2D> = corners
            .into_iter()
            .map(|(x, y)| Coordinate2D { 0: x, 1: y })
            .collect();

        let vertices: Vec<Coordinate3D> = vertices
            .into_iter()
            .map(|(x, y)| Coordinate3D { 0: x, 1: y, 2: z })
            .collect();
        
        if corners.len() != 4 || vertices.len() != 4 {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "Expected exactly 4 corners and 4 vertices"
            ));
        }

        let corners: [Coordinate2D; 4] = corners.try_into().unwrap();
        let vertices: [Coordinate3D; 4] = vertices.try_into().unwrap();


        Ok(Self {
            tile: crate::video::surface::Tile::new(corners.map(|x| crate::math::Coordinate2D{ 0: x.0, 1: x.1}), vertices.map(|x| crate::math::Coordinate3D{ 0: x.0, 1: x.1, 2: x.2}), x, y, width, height)
            }
        )
    }

    #[new]
    fn simple_new(x: u32, y: u32, z: f32, width: u32, height: u32, tile_set: &TileSet) -> PyResult<Self> {
        Ok(Self{ tile: crate::video::surface::Tile::simple_new(x, y, z, width, height, &tile_set.tile_set.borrow()) })
    }

    fn stretch(&mut self, width: f32, height: f32) {
        self.tile.stretch(width, height);
    }

    fn fliph(&mut self) {
        self.tile.fliph();
    }

    fn flipv(&mut self) {
        self.tile.flipv();
    }

    fn get_rect(&self) -> PyResult<Rect> {
        let tile_rect = self.tile.get_rect();
        Ok(Rect{ x: tile_rect.x, y: tile_rect.y, width: tile_rect.width, height: tile_rect.height })
    }
}

#[pyclass(unsendable)]
pub struct TileSet {
    tile_set: Rc<RefCell<crate::video::surface::TileSet>>,
}

#[pymethods]
impl TileSet {
    #[new]
    fn new(width: usize, height: usize) -> PyResult<Self> {
        Ok(Self{ tile_set: Rc::new(RefCell::new(crate::video::surface::TileSet::new(width, height))) })
    }

    #[staticmethod]
    fn from_texture(path: String) -> PyResult<Self> {
        Ok(Self {
            tile_set: Rc::new(RefCell::new(crate::video::surface::TileSet::from_texture(&path)))
        })
    }

    fn blit(&mut self, surface: &Surface, offset_x: u32, offset_y: u32) {
        self.tile_set.borrow_mut().blit(&surface.surface.borrow(), offset_x, offset_y);
    }
}

#[pyclass(unsendable)]
pub struct SpriteList {
    sprite_list: crate::sprite::SpriteList,
    true_sprite_list: Vec<crate::sprite::Sprite>
}

#[pymethods]
impl SpriteList {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self { sprite_list: crate::sprite::SpriteList { sprite_list: vec![] }, true_sprite_list: vec![] })
    }

    fn append_surface(&mut self, x: f32, y: f32, z: f32, surface: &Surface, ysort: bool, ysort_origin: f32) {
        self.true_sprite_list.push(crate::sprite::Sprite::Surface(x, y, z, Rc::clone(&surface.surface), crate::video::CatEngineShader::TestShader, ysort, ysort_origin));
    }

    fn append_tile(&mut self, x: f32, y: f32, z: f32, tile: Tile, tile_set: &TileSet, ysort: bool, ysort_origin: f32) {
        self.true_sprite_list.push(crate::sprite::Sprite::Tile(x, y, z, Rc::clone(&tile_set.tile_set), tile.tile, crate::video::CatEngineShader::TextureShader, ysort, ysort_origin));
    }

    fn update(&mut self) {
        self.sprite_list.update(std::mem::take(&mut self.true_sprite_list));
        self.true_sprite_list = vec![];
    }

    fn clone_update(&mut self) {
        self.sprite_list.clone_update(self.true_sprite_list.clone());
    }


    fn no_reset_update(&mut self) {
        self.sprite_list.update(std::mem::take(&mut self.true_sprite_list));
    }

    fn reset_sprite_list(&mut self) {
        self.true_sprite_list = vec![];
        self.sprite_list.update(std::mem::take(&mut self.true_sprite_list));
    }

    fn print_sprite_list_width(&self) {
        println!("{}", self.true_sprite_list.len());
    }
}

#[pyclass(unsendable)]
pub struct Character {
    character: crate::font::Character,
}

#[pymethods]
impl Character {
    #[new]
    fn new(chr: char, x: u32, y: u32, width: u32, height: u32) -> Self {
        Character{ character: crate::font::Character{ crt: chr, x, y, width, height } }
    }
}

#[pyclass(unsendable)]
pub struct Font {
    font: crate::font::Font,
}

#[pymethods]
impl Font {
    #[new]
    fn new(py: Python, path: &str, character_list: Vec<Py<Character>>) -> PyResult<Self> {
        let mut chars = vec![];

        for ch in character_list {
            let borrowed = ch.borrow(py);
            chars.push(borrowed.character.clone());
        }

        Ok(Self {
            font: crate::font::Font::new(path, chars)
        })
    }

    fn size(&self, text: &str, font_size: f32, spacement: f32) -> PyResult<(f32, f32)> {
        let mut width = 0.0;
        let mut height = 0.0;
        
        for ch in text.chars() {
            if self.font.return_character_from_string(ch).unwrap().height as f32 > height {
                height = self.font.return_character_from_string(ch).unwrap().height as f32 * font_size
            }

            width += self.font.return_character_from_string(ch).unwrap().width as f32 * font_size + spacement
        }

        Ok((width, height))
    }
}

#[pyclass(unsendable)]
pub struct Rect {
    #[pyo3(get, set)]
    pub x: f32,
    #[pyo3(get, set)]
    pub y: f32,
    #[pyo3(get, set)]
    pub width: f32,
    #[pyo3(get, set)]
    pub height: f32,
}

#[pymethods]
impl Rect {
    #[new]
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self{ x, y, width, height }
    }
    
    pub fn colliderect(&self, rect: &Self) -> PyResult<bool> {
        if self.x + self.width > rect.x
        && self.x < rect.x + rect.width
        && self.y + self.height > rect.y 
        && self.y < rect.y + rect.height {
            Ok(true)
        } else {
            Ok(false)
        }
    }
    
    pub fn collidepoint(&self, point: &Coordinate2D) -> PyResult<bool> {
        if self.x < point.0
        && self.y < point.1
        && self.width + self.x > point.0
        && self.height + self.y > point.1 {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl Rect {
    pub fn into_rect(&self) -> crate::math::Rect {
        crate::math::Rect::new(self.x, self.y, self.width, self.height)
    }
}