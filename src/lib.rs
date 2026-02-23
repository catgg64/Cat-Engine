use sdl2::{libc::sleep, pixels::Color, render::TextureCreator, sys::True, video::WindowContext, *};
use color::*;
use input::*;
pub mod color;
pub mod input;
pub mod video;
pub mod shape;

pub struct CatEngine {
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
    event_pump: sdl2::EventPump,
    pub screen_rect: sdl2::rect::Rect,
    pub input: input::Input,
    pub running: bool,
    pub texture_creator: TextureCreator<WindowContext>,
}

impl CatEngine{
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self, String>{
        let context: Sdl = sdl2::init()?;
        let video_subsystem: VideoSubsystem = context.video()?;
        let window = video_subsystem
            .window(title, width, height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;let event_pump: EventPump = context.event_pump()?;
        let canvas = window
            .into_canvas()
            .accelerated()
            .build()
            .map_err(|e| e.to_string())?;

        let screen_rect = sdl2::rect::Rect::new(0, 0, width, height);
        let input: Input = input::Input::new();
        let mut running: bool = true;
        let texture_creator: TextureCreator<WindowContext> = canvas.texture_creator();

        Ok(Self {
            canvas, 
            event_pump,
            screen_rect,
            input,
            running,
            texture_creator,
        })
    }
    
    

    pub fn update(&mut self) {
        self.running = self.input.update(&mut self.event_pump);
        self.canvas.present();
    }

    

}

pub mod keyboard {
    pub use sdl2::keyboard::{Keycode, Scancode, Mod};
    // You can add your own helpers here later if you want
}