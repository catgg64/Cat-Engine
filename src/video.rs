use crate::color::Color;
use sdl2::*;
use sdl2::render::TextureCreator;
use sdl2::video::{ WindowContext };

pub fn clear_color(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, color: Color, screen_rect: sdl2::rect::Rect) {
    let (r, g, b) = color.return_rgb();
    canvas.set_draw_color(sdl2::pixels::Color::RGB(r, g, b));
    canvas.fill_rect(screen_rect);
}

pub mod image {
    use sdl2::*;
    use sdl2::render::TextureCreator;
    use sdl2::video::{ WindowContext };
    use sdl2::rect;
    use crate::shape::rect::Rect;
    use sdl2::pixels::PixelFormatEnum;


    pub fn load<'a>(texture_creator: &'a TextureCreator<WindowContext>, texture: String, width: u32, height: u32) -> Result<render::Texture<'a>, String>{
        
        let img = image::open(texture)
            .expect("Failed to load image");

        let img = img.to_rgba8();
        let (width, height) = img.dimensions();
        let pixels = img.into_raw();
        
        let mut texture = texture_creator
            .create_texture_streaming(
                PixelFormatEnum::RGBA32,
                width,
                height,
            )
            .map_err(|e| e.to_string())?;
            

        texture.update(None, &pixels, (4 * width) as usize);
        
        Ok(texture)
        }
        
        pub fn blit(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, texture: &render::Texture, region: Rect) -> Result<(), String>{
            let region_grabbed = region.get_x_y_sizex_sizey();
            let actual_rect = sdl2::rect::Rect::new(region_grabbed.0 as i32, region_grabbed.1 as i32, region_grabbed.2 as u32, region_grabbed.3 as u32);
            canvas.copy(texture, None, actual_rect)
    }

    pub mod draw {
        use sdl2::rect::{ Rect, Point };
        use crate::shape;
        use crate::color;

        pub fn line(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, color: color::Color, point_1: shape::Point::Point, point_2: shape::Point::Point) -> Result<(), String> {
            canvas.set_draw_color(color.turn_into_sdlcolor());
            canvas.draw_line(point_1.turn_into_sdl_point(), point_2.turn_into_sdl_point())
        }
    }

}

//pub fn blit(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) 
