#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::video::Renderer;

pub mod pixel;
pub mod video;
pub mod math;

pub struct CatEngine {
    sdl_context: sdl2::Sdl,
    window: sdl2::video::Window,
    video_subsystem: sdl2::VideoSubsystem,
    gl_context: sdl2::video::GLContext,
    pub renderer: Renderer,
    pub running: bool,
}

impl CatEngine {
    pub fn new(title: &str, width: u32, height: u32) -> Result<CatEngine, String> {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window(title, width, height)
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
            gl::Disable(gl::DEPTH_TEST);
        }
        let mut renderer = Renderer::new();
        let mut running: bool = true;

        Ok(CatEngine { 
            sdl_context,
            window, 
            video_subsystem, 
            gl_context,
            renderer,
            running 
        })
    }

    pub fn update(&self) {
        self.window.gl_swap_window();
    }

    pub fn clear_screen(&self, color: pixel::Color) {
        let (true_color_r, true_color_g, true_color_b, true_color_a) = (color.r / 255, color.g / 255, color.b / 255, color.a / 255);
        unsafe {
            gl::ClearColor(true_color_r as f32,true_color_g as f32,true_color_b as f32,true_color_a as f32);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}