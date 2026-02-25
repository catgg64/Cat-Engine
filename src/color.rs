use sdl2::pixels::{self};

#[derive(Copy, Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self{
        Color {
            r: r,
            g: g,
            b: b,
        }
    }
    pub fn return_rgb(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }
    pub fn turn_into_sdlcolor(&self) -> pixels::Color {
        pixels::Color::RGB(self.r, self.g, self.b)
    }
}