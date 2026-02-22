use sdl2::{libc::sleep, pixels::Color, *};
pub mod color;

pub struct Engine {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    event_pump: sdl2::EventPump,
    screen_rect: sdl2::rect::Rect,
}

impl Engine{
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

        Ok(Self {
            canvas,
            event_pump,
            screen_rect,
        })
    }
    
    pub fn clear_color(&mut self, color: color::Color) {
        let (r, g, b) = color.return_rgb();
        self.canvas.set_draw_color(sdl2::pixels::Color::RGB(r, g, b));
    }

}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
