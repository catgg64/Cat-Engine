use std::{rc::Rc, cell::{RefCell, Ref}, vec};

use pyo3::prelude::*;
use crate::{CatEngine, keyboard, math::Coordinate2D, sprite::{Sprite, SpriteList}, video::{Renderer, surface::{self, Surface, Tile, TileSet}}};

#[pyclass(unsendable)]
pub struct PyCatEngine {
    engine: CatEngine,
}

#[pymethods]
impl PyCatEngine {
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
        let engine = CatEngine::new(&title, width, height, true_flags)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e))?;
        Ok(Self { engine })
    }

    fn update(&mut self) {
        self.engine.update();
    }

    fn clear_screen(&self, r: u8, g: u8, b: u8, a: u8) {
        self.engine.clear_screen(crate::pixel::Color { r, g, b, a });
    }

    fn is_down_scancode(&self, key: String) -> PyResult<bool> {
        Ok(self.engine.input.is_down({
            match key.as_str() {
                "A" => {keyboard::Scancode::A}
                "B" => {keyboard::Scancode::B}
                "C" => {keyboard::Scancode::C}
                "D" => {keyboard::Scancode::D}
                "E" => {keyboard::Scancode::E}
                "F" => {keyboard::Scancode::F}
                "G" => {keyboard::Scancode::G}
                "H" => {keyboard::Scancode::H}
                "I" => {keyboard::Scancode::I}
                "J" => {keyboard::Scancode::J}
                "K" => {keyboard::Scancode::K}
                "L" => {keyboard::Scancode::L}
                "M" => {keyboard::Scancode::M}
                "N" => {keyboard::Scancode::N}
                "O" => {keyboard::Scancode::O}
                "P" => {keyboard::Scancode::P}
                "Q" => {keyboard::Scancode::Q}
                "R" => {keyboard::Scancode::R}
                "S" => {keyboard::Scancode::S}
                "T" => {keyboard::Scancode::T}
                "U" => {keyboard::Scancode::U}
                "V" => {keyboard::Scancode::V}
                "W" => {keyboard::Scancode::W}
                "X" => {keyboard::Scancode::X}
                "Y" => {keyboard::Scancode::Y}
                "Z" => {keyboard::Scancode::Z}
                "up" => {keyboard::Scancode::Up}
                "down" => {keyboard::Scancode::Down}
                "left" => {keyboard::Scancode::Left}
                "right" => {keyboard::Scancode::Right}
                _ => {
                    panic!("no key named {}", key);
                }
            }
        }))
    }

    fn is_down_keycode(&self, key: String) -> PyResult<bool> {
        Ok(self.engine.input.is_down_keycode({
            match key.as_str() {
                "A" => {keyboard::Keycode::A}
                "B" => {keyboard::Keycode::B}
                "C" => {keyboard::Keycode::C}
                "D" => {keyboard::Keycode::D}
                "E" => {keyboard::Keycode::E}
                "F" => {keyboard::Keycode::F}
                "G" => {keyboard::Keycode::G}
                "H" => {keyboard::Keycode::H}
                "I" => {keyboard::Keycode::I}
                "J" => {keyboard::Keycode::J}
                "K" => {keyboard::Keycode::K}
                "L" => {keyboard::Keycode::L}
                "M" => {keyboard::Keycode::M}
                "N" => {keyboard::Keycode::N}
                "O" => {keyboard::Keycode::O}
                "P" => {keyboard::Keycode::P}
                "Q" => {keyboard::Keycode::Q}
                "R" => {keyboard::Keycode::R}
                "S" => {keyboard::Keycode::S}
                "T" => {keyboard::Keycode::T}
                "U" => {keyboard::Keycode::U}
                "V" => {keyboard::Keycode::V}
                "W" => {keyboard::Keycode::W}
                "X" => {keyboard::Keycode::X}
                "Y" => {keyboard::Keycode::Y}
                "Z" => {keyboard::Keycode::Z}
                "up" => {keyboard::Keycode::Up}
                "down" => {keyboard::Keycode::Down}
                "left" => {keyboard::Keycode::Left}
                "right" => {keyboard::Keycode::Right}
                _ => {
                    panic!("no key named {}", key);
                }
            }
        }))
    }

    #[getter]
    fn is_running(&self) -> bool {
        self.engine.running
    }

    fn blit_surface(&mut self, surface: &PySurface, pos_x: f32, pos_y: f32, pos_z: f32) {
        self.engine.renderer.blit(&surface.surface.borrow(), pos_x, pos_y, pos_z);
    }
    
    fn blit_tileset(&mut self, tile: u32, tile_set: &mut PyTileSet, pos_x: f32, pos_y: f32) {
        self.engine.renderer.draw_tileset(tile, &mut tile_set.tile_set.borrow_mut(), pos_x, pos_y);
    }

    fn blit_sprite_list(&mut self, sprite_list: &mut PySpriteList, x_offset: f32, y_offset: f32) {
        self.engine.renderer.draw_sprite_list(&mut sprite_list.sprite_list, x_offset, y_offset);
    } 
}

#[pyclass(unsendable)]
pub struct PySurface {
    surface: Rc<RefCell<Surface>>,
}

#[pymethods]
impl PySurface {
    #[new]
    fn new(width: usize, height: usize) -> PyResult<Self> {
        Ok(Self{ surface: Rc::new(RefCell::new(surface::Surface::new(width, height))) })
    }

    #[staticmethod]
    fn from_texture(path: String) -> PyResult<Self> {
        Ok(Self {
            surface: Rc::new(RefCell::new(surface::Surface::from_texture(&path)))
        })
    }

    fn upload(&mut self) {
        self.surface.borrow_mut().upload();
    }

    fn blit(&mut self, surface: &PySurface, offset_x: u32, offset_y: u32) {
        self.surface.borrow_mut().blit(&surface.surface.borrow(), offset_x, offset_y);
    }

    fn crop(&mut self, x: u32, y: u32, width: u32, height: u32) {
        self.surface.borrow_mut().crop(x, y, width, height);
    }

    fn stretch(&mut self, width: u32, height: u32) {
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
}

impl PySurface {
    pub fn get_surface(&self) -> Ref<Surface> {
        self.surface.borrow()
    }
}

#[pyclass(unsendable)]
#[derive(Debug)]
pub struct PyCoordinate2D(pub f32, pub f32);

impl Clone for PyCoordinate2D {
    fn clone(&self) -> Self {
        Self(self.0, self.1)
    }
}

#[pyclass(unsendable)]
pub struct PyTile {
    tile: Tile,
}

impl Clone for PyTile {
    fn clone(&self) -> Self {
        Self { tile: self.tile.clone() }
    }
}

#[pymethods]
impl PyTile {
    #[new]
    fn new(py: Python, corners: Vec<(f32, f32)>, vertices: Vec<(f32, f32)>, x: u32, y: u32, width: u32, height: u32) -> PyResult<PyTile> {
        let corners: Vec<PyCoordinate2D> = corners
            .into_iter()
            .map(|(x, y)| PyCoordinate2D { 0: x, 1: y })
            .collect();

        let vertices: Vec<PyCoordinate2D> = vertices
            .into_iter()
            .map(|(x, y)| PyCoordinate2D { 0: x, 1: y })
            .collect();
        
        if corners.len() != 4 || vertices.len() != 4 {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "Expected exactly 4 corners and 4 vertices"
            ));
        }

        let corners: [PyCoordinate2D; 4] = corners.try_into().unwrap();
        let vertices: [PyCoordinate2D; 4] = vertices.try_into().unwrap();


        Ok(Self {
            tile: Tile::new(corners.map(|x| Coordinate2D{ 0: x.0, 1: x.1}), vertices.map(|x| Coordinate2D{ 0: x.0, 1: x.1}), x, y, width, height)
            }
        )
    }
}

#[pyclass(unsendable)]
pub struct PyTileSet {
    tile_set: Rc<RefCell<TileSet>>,
}

#[pymethods]
impl PyTileSet {
    #[new]
    fn new(width: usize, height: usize) -> PyResult<Self> {
        Ok(Self{ tile_set: Rc::new(RefCell::new(surface::TileSet::new(width, height))) })
    }

    #[staticmethod]
    fn from_texture(path: String) -> PyResult<Self> {
        Ok(Self {
            tile_set: Rc::new(RefCell::new(surface::TileSet::from_texture(&path)))
        })
    }

    fn simple_append_tile(&mut self, x: u32, y: u32, width: u32, height: u32) -> PyResult<u32> {
        Ok(self.tile_set.borrow_mut().simple_append_tile(x, y, width, height))
    }

    fn blit(&mut self, surface: &PySurface, offset_x: u32, offset_y: u32) {
        self.tile_set.borrow_mut().blit(&surface.surface.borrow(), offset_x, offset_y);
    }

    fn append_tile(&mut self, tile: PyTile) -> PyResult<u32>{
        Ok(self.tile_set.borrow_mut().append_tile(tile.tile))
    } 

    fn stretch_tile(&mut self, tile: u32, width: u32, height: u32) {
        self.tile_set.borrow_mut().stretch_tile(tile, width, height);
    }
}

#[pyclass(unsendable)]
pub struct PySpriteList {
    sprite_list: SpriteList,
    true_sprite_list: Vec<Sprite>
}

#[pymethods]
impl PySpriteList {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self { sprite_list: SpriteList { sprite_list: vec![] }, true_sprite_list: vec![] })
    }

    fn append_surface(&mut self, x: f32, y: f32, z: f32, surface: &PySurface, ysort: bool) {
        self.true_sprite_list.push(crate::sprite::Sprite::Surface(x, y, z, Rc::clone(&surface.surface), crate::video::CatEngineShader::TestShader, ysort));
    }

    fn append_tile(&mut self, x: f32, y: f32, z: f32, tile: u32, tile_set: &PyTileSet, ysort: bool) {
        self.true_sprite_list.push(crate::sprite::Sprite::Tile(x, y, z, Rc::clone(&tile_set.tile_set), tile, crate::video::CatEngineShader::TextureShader, ysort));
    }

    fn update(&mut self) {
        self.sprite_list.update(std::mem::take(&mut self.true_sprite_list));
        self.true_sprite_list = vec![];
    }
}