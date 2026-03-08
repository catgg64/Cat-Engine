use image::ImageError;

use crate::video::surface;

pub struct Surface {
    pub texture_id: u32,
    pub width: u32,
    pub height: u32,
    pub corners:  [(u32, u32); 4],
    pub data: Vec<u8>
}

impl Surface {
    pub fn from_texture(path: &str) -> Self {
        let image = image::open(path).expect("Error loading the image: ");
        let image = image.flipv().into_rgba8();
        let (width, height) = image.dimensions();
        let data = image.into_raw();
        
        let mut texture_id = 0;
        
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexImage2D(gl::TEXTURE_2D,
                           0,
                           gl::RGBA as i32,
                           width as i32,
                           height as i32,
                           0,
                           gl::RGBA,
                           gl::UNSIGNED_BYTE,
                           data.as_ptr() as *const _);
        }

        let mut corners = [
            (0, 0),
            (width, 0),
            (width, height),
            (0, height),
        ];

        Surface {
            texture_id,
            width,
            height,
            corners,
            data,
        }
    }

    pub fn strech(&mut self, new_corners: [(u32, u32); 4]) {
        self.corners = new_corners
    }

    pub fn bind(&mut self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
        }
    }

    pub fn load(&mut self) -> Result<(), ImageError>{
        unsafe {
            self.bind();

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                self.width as i32,
                self.height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                self.data.as_ptr() as *const _,
            );
            Ok(())
        }
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.texture_id);  // or whatever your texture ID is
        }
    }
}