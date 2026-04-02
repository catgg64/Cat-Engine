#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(warnings)]

use glam::Mat4;

#[cfg(feature = "python")]
use crate::{python::{PyCoordinate2D, PySpriteList, PyTile, PyTileSet}};
use crate::{video::Renderer};

pub mod pixel;
pub mod video;
pub mod math;
pub mod input;
pub mod mesh;
pub mod sprite;
pub mod font;

#[derive(PartialEq)]
pub enum CatEngineFlag {
    /// Enables the Depth Buffer.
    /// Don't use on 2D, but must be used on 3D.
    DepthBuffer,
    /// Enables Vsync.
    Vsync,
    /// Enables DynamicVsync.
    DynamicVsync,
}

pub struct CatEngine {
    sdl_context: sdl2::Sdl,
    window: sdl2::video::Window,
    video_subsystem: sdl2::VideoSubsystem,
    gl_context: sdl2::video::GLContext,
    event_pump: sdl2::EventPump,
    pub renderer: Renderer,
    pub input: input::Input,
    pub running: bool,
    pub screen_width: u32,
    pub screen_height: u32,
}

impl CatEngine {
    //! # CatEngine
    //!
    //! A simple, funcional renderer API used to make 2D and 3D games.
    
    /// Starts the Engine.
    /// 
    /// # Examples
    /// ```ignore
    /// let catengine = CatEngine::new("Window!", 800, 800, vec![CatEngine::CatEngineFlag::Vsync]);
    /// ```
    pub fn new(title: &str, width: u32, height: u32, flags: Vec<CatEngineFlag>) -> Result<CatEngine, String> {
        if width > 10000 || height > 10000 {
            return Err("window is way too big!".to_string())
        }
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window(title, width, height)
            .resizable()
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();
        let event_pump: sdl2::EventPump = sdl_context.event_pump().unwrap();

        let gl_context = window.gl_create_context().unwrap();
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);
        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
            if flags.contains(&CatEngineFlag::DepthBuffer) {
                gl::Enable(gl::DEPTH_TEST);
                println!("depth buffer on!");
            }
            gl::Enable(gl::BLEND);
            gl::Disable(gl::CULL_FACE);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        video_subsystem.gl_set_swap_interval(0); // vsync OFF

        if flags.contains(&CatEngineFlag::DynamicVsync) {
            video_subsystem.gl_set_swap_interval(-1).unwrap_or_else(|_| {
                video_subsystem.gl_set_swap_interval(1).unwrap();
            });
            println!("has dynamic vsync")
        }

        if flags.contains(&CatEngineFlag::Vsync) {
            video_subsystem.gl_set_swap_interval(1).unwrap();
            println!("has regular vsync")
        }

        let mut renderer = Renderer::new(width, height, 67.0, 0.1, 1000.0);
        let mut input = input::Input::new(&sdl_context);
        let mut running: bool = true;

        Ok(CatEngine { 
            sdl_context,
            window, 
            video_subsystem, 
            gl_context,
            event_pump,
            renderer,
            input,
            running,
            screen_width: width,
            screen_height: height,
        })
    }

    /// Updates the screen. Should be ran every frame after rendering.
    pub fn update(&mut self) {
        self.window.gl_swap_window();
        self.running = self.input.update(&mut self.event_pump, &mut self.renderer);
    }

    /// Clears the whole screen. Should be done every frame before rendering is done.
    pub fn clear_screen(&self, color: pixel::Color) {
        let (true_color_r, true_color_g, true_color_b, true_color_a) = (color.r as f32 / 255.0, color.g as f32 / 255.0, color.b as f32 / 255.0, color.a as f32 / 255.0);
        unsafe {
            gl::ClearColor(true_color_r as f32,true_color_g as f32,true_color_b as f32,true_color_a as f32);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    /// Gets camera specs. Used on 3D.
    pub fn get_camera_specs(&self, cam_x: f32, cam_y: f32, cam_z: f32, yaw: f32, pitch: f32) -> (f32, f32, f32, f32, f32, f32, Mat4) {
        let (camera_position_x, camera_position_y, camera_position_z) = (cam_x, cam_y, cam_z);

        let (front_x, front_y, front_z) = (
            yaw.cos() * pitch.cos(),
            pitch.sin(),
            -yaw.sin() * pitch.cos()
        );

        let view_matrix = Mat4::look_at_rh(
            glam::Vec3 { x: camera_position_x, y: camera_position_y, z: camera_position_z },
            glam::Vec3 { x: camera_position_x, y: camera_position_y, z: camera_position_z } + glam::Vec3 { x: front_x, y: front_y, z: front_z },
            glam::Vec3::Y,
        );

        (camera_position_x, camera_position_y, camera_position_z, front_x, front_y, front_z, view_matrix)
    }

    /// Sets the FOV, near plane and far plane.
    /// Fov is the divisor of Z in projection,
    /// Near plane is how close objects need to be to get cut off,
    /// And far plane is how far objects need to be to get cut off.
    /// 
    /// Again, used on 3D.
    pub fn set_fov(&mut self, fov: f32, near_plane: f32, far_plane: f32) {
        let projection = glam::Mat4::perspective_rh_gl(fov.to_radians(), self.screen_width as f32 / self.screen_height as f32, near_plane, far_plane);
        self.renderer.set_projection(projection, fov, near_plane, far_plane);
    }

    /// Enables fullscreen.
    pub fn enable_fullscreen(&mut self) {
        self.window.set_fullscreen(sdl2::video::FullscreenType::Desktop).unwrap();
        let display_mode = self.video_subsystem.current_display_mode(0).unwrap();

        let width = display_mode.w;
        let height = display_mode.h;

        self.screen_width = width as u32;
        self.screen_height = height as u32;


        unsafe {
            gl::Viewport(0, 0, width, height);
        }
        let projection = glam::Mat4::perspective_rh_gl(self.renderer.fov.to_radians(), self.screen_width as f32 / self.screen_height as f32, self.renderer.near_plane, self.renderer.far_plane);
        self.renderer.true_set_projection(projection);
    }

    /// Enables "true" fullscreen, the resolution of the computer will need to be swapped
    /// in order for this to work. Regular, "fake" fullscreen works better in most cases.
    pub fn enable_true_fullscreen(&mut self) {
        self.window.set_fullscreen(sdl2::video::FullscreenType::True).unwrap();
    }

    /// Disables any sort of fullscreen.
    pub fn disable_fullscreen(&mut self) {
        self.window.set_fullscreen(sdl2::video::FullscreenType::Off).unwrap();
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
        }
        self.screen_width = width;
        self.screen_height = height;
        self.renderer.set_size(width, height);
    }
}

pub mod keyboard {
    //! # Keyboard
    //!
    //! Bindings for Keycode and Scancode.
    //! Keycode is the key that such press is trying to represent, and may very depending on keyboard layouts.
    //! Scancode is the literal location of a key, and is always the same.
    pub use sdl2::keyboard::{Keycode, Scancode, Mod};
    // TODO: Add helpers    
}

pub mod opengl {
    /// Raw binding of opengl.
    pub use gl::*;
}

pub mod sdl {
    /// Raw binding of sdl.
    pub use sdl2::*;
}

#[cfg(feature = "python")]
mod python;

#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use python::{ PyCatEngine, PySurface };

#[cfg(feature = "python")]
#[pymodule]
fn catengine(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyCatEngine>()?;
    m.add_class::<PySurface>()?;
    m.add_class::<PyCoordinate2D>()?;
    m.add_class::<PyTileSet>()?;
    m.add_class::<PyTile>()?;
    m.add_class::<PySpriteList>()?;
    Ok(())
}