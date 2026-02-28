use crate::color::Color;

pub mod graphics;
pub mod surface {
    use crate::color::Color;

    pub struct Surface {
        pub texture_id: u32,
        pub width: u32,
        pub height: u32,
        // corners in (x, y) relative to quad origin
        pub corners: [(f32, f32); 4],
        pub raw_data: Vec<u8>,
    }

    impl Surface {
        pub fn new(path: &str) -> Self {
            let img = image::open(path).expect("Failed to load image");
            let img = img.flipv().into_rgba8();
            let (width, height) = img.dimensions();
            let data = img.into_raw();
            let raw_data = data.clone();

            let mut texture_id = 0;

            unsafe {
                gl::GenTextures(1, &mut texture_id);
                gl::BindTexture(gl::TEXTURE_2D, texture_id);

                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

                gl::TexImage2D(
                    gl::TEXTURE_2D,
                    0,
                    gl::RGBA as i32,
                    width as i32,
                    height as i32,
                    0,
                    gl::RGBA,
                    gl::UNSIGNED_BYTE,
                    data.as_ptr() as *const _
                );
            }

            Surface {
                texture_id,
                width,
                height,
                corners: [
                    (0.0, 0.0),
                    (width as f32, 0.0),
                    (width as f32, height as f32),
                    (0.0, height as f32),
                ],
                raw_data,
            }
        }

        /// Stretch / move each corner individually
        pub fn stretch(&mut self, new_corners: [(f32, f32); 4]) {
            self.corners = new_corners;
        }
    }
}

impl Clone for surface::Surface {
    fn clone(&self) -> Self {
        // NOTE: we need a copy of the original image data
        // Let's assume you store it in `raw_data: Vec<u8>` in Surface
        let mut new_texture = 0;
        unsafe {
            gl::GenTextures(1, &mut new_texture);
            gl::BindTexture(gl::TEXTURE_2D, new_texture);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                self.width as i32,
                self.height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                self.raw_data.as_ptr() as *const _,
            );
        }

        Self {
            texture_id: new_texture,
            width: self.width,
            height: self.height,
            corners: self.corners,  // copy the corners too
            raw_data: self.raw_data.clone(), // store pixel copy so we can clone later
        }
    }
}

pub mod image {
    use sdl2::*;
    use sdl2::render::{Canvas, TextureCreator};
    use sdl2::video::{ WindowContext };
    use crate::shape::rect::Rect;
    use sdl2::pixels::PixelFormatEnum;

    //pub mod draw {
    //    use sdl2::surface::Surface;
//
//        use crate::shape;
//        use crate::color;
//
//        pub fn line(surface: &mut Surface, color: color::Color, point_1: shape::point::Point, point_2: shape::point::Point) -> Result<(), String> {
//            surface.set_draw_color(color.turn_into_sdlcolor());
//            surface.draw_line(point_1.turn_into_sdl_point(), point_2.turn_into_sdl_point())
//        }
//    }

}

//pub fn blit(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) 
