#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod pixel;

struct CatEngine {
    sdl_context: sdl2::Sdl,
    window: sdl2::video::Window,
    video_subsystem: sdl2::VideoSubsystem,
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

        let gl_context = window.gl_create_context();
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);
        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
            gl::Enable(gl::DEPTH_TEST);
        }
        let mut running: bool = true;

        Ok(CatEngine { 
            sdl_context,
            window, 
            video_subsystem, 
            running 
        })
    }

    pub fn update(&self) {
        self.window.gl_swap_window();
    }

    pub fn clear_screen(&self, color: pixel::Color) {
        unsafe {
            gl::ClearColor(color.r as f32,color.g as f32,color.b as f32,color.a as f32);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }
}