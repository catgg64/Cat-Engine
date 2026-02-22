use sdl2::{libc::sleep, pixels::Color, sys::True, *};
use color::*;
use input::*;
pub mod color;
pub mod input;

pub struct CatEngine {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    event_pump: sdl2::EventPump,
    screen_rect: sdl2::rect::Rect,
    pub input: input::Input,
    running: bool,
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
        let scancode = sdl2::keyboard::Scancode

        Ok(Self {
            canvas,
            event_pump,
            screen_rect,
            input,
            running
        })
    }
    
    pub fn clear_color(&mut self, color: color::Color) {
        let (r, g, b) = color.return_rgb();
        self.canvas.set_draw_color(sdl2::pixels::Color::RGB(r, g, b));
        self.canvas.fill_rect(self.screen_rect);
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