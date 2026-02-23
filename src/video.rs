use super::CatEngine;
use crate::color::Color;

pub fn clear_color(engine: &mut CatEngine, color: Color) {
    let (r, g, b) = color.return_rgb();
    engine.canvas.set_draw_color(sdl2::pixels::Color::RGB(r, g, b));
    engine.canvas.fill_rect(engine.screen_rect);
}
