#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use glam::Mat4;

use crate::video::Renderer;

pub mod pixel;
pub mod video;
pub mod math;
pub mod input;
pub mod mesh;
pub mod sprite;
pub mod font;

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
    pub fn new(title: &str, width: u32, height: u32) -> Result<CatEngine, String> {
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
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::BLEND);
            gl::Disable(gl::CULL_FACE);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        video_subsystem.gl_set_swap_interval(-1).unwrap_or_else(|_| {
            video_subsystem.gl_set_swap_interval(1).unwrap();
        });

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

    pub fn update(&mut self) {
        self.window.gl_swap_window();
        self.running = self.input.update(&mut self.event_pump, &mut self.renderer);
    }

    pub fn clear_screen(&self, color: pixel::Color) {
        let (true_color_r, true_color_g, true_color_b, true_color_a) = (color.r as f32 / 255.0, color.g as f32 / 255.0, color.b as f32 / 255.0, color.a as f32 / 255.0);
        unsafe {
            gl::ClearColor(true_color_r as f32,true_color_g as f32,true_color_b as f32,true_color_a as f32);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

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

    pub fn set_fov(&mut self, fov: f32, near_plane: f32, far_plane: f32) {
        let projection = glam::Mat4::perspective_rh_gl(fov.to_radians(), self.screen_width as f32 / self.screen_height as f32, near_plane, far_plane);
        self.renderer.set_projection(projection, fov, near_plane, far_plane);
    }

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

    pub fn enable_true_fullscreen(&mut self) {
        self.window.set_fullscreen(sdl2::video::FullscreenType::True).unwrap();
    }

    pub fn disable_fullscreen(&mut self) {
        self.window.set_fullscreen(sdl2::video::FullscreenType::Off).unwrap();
    }
}

pub mod keyboard {
    pub use sdl2::keyboard::{Keycode, Scancode, Mod};
    // TODO: Add helpers    
}

pub mod opengl {
    pub use gl::*;
}

pub mod sdl {
    pub use sdl2::*;
}