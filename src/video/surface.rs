use crate::{math::Coordinate2D};

pub struct Surface {
    pub texture_id: u32,
    pub width: u32,
    pub height: u32,
    pub corners:  [Coordinate2D; 4],
    pub data: Vec<u8>,
}

impl Surface {
    pub fn from_texture(path: &str) -> Self {
        let image = image::open(path).expect("Error loading the image: ");
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let data = image.into_raw();
        
        let mut texture_id = 0;
        
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
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
            Coordinate2D(1.0, 1.0),
            Coordinate2D(1.0, 0.0),
            Coordinate2D(0.0, 0.0),
            Coordinate2D(0.0, 1.0),
        ];

        Surface {
            texture_id,
            width,
            height,
            corners,
            data,
        }
    }

    pub fn crop(&mut self, x: u32, y: u32, width: u32, height: u32) {
        self.corners = [
            Coordinate2D((x + width) as f32 / self.width as f32, (y + height) as f32 / self.height as f32),
            Coordinate2D((x + width) as f32 / self.width as f32, y as f32 / self.height as f32),
            Coordinate2D(x as f32 / self.width as f32,           y as f32 / self.height as f32),
            Coordinate2D(x as f32 / self.width as f32,           (y + height) as f32 / self.height as f32),
        ];
        self.width = width;
        self.height = height;
    }

    pub fn stretch(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    pub fn flipv(&mut self) {
        for corner in self.corners.iter_mut() {
            corner.1 = 1.0 - corner.1;
        }
    }

    pub fn fliph(&mut self) {
        for corner in self.corners.iter_mut() {
            corner.0 = 1.0 - corner.0;
        }
    }



    pub fn bind(&mut self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
        }
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.texture_id);
        }
    }
}