use image::ImageError;

use crate::{math::Coordinate2D};

pub struct Surface {
    pub texture_id: u32,
    pub width: u32,
    pub height: u32,
    pub corners:  [Coordinate2D; 4],
    pub data: Vec<u8>,
    pub vao: u32,
    pub vbo: u32,
    pub ebo: u32,
    pub uv_vbo: u32,
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
            Coordinate2D{0: 0.0, 1: 0.0},
            Coordinate2D{0: width as f32, 1: 0.0},
            Coordinate2D{0: width as f32, 1: height as f32},
            Coordinate2D{0: 0.0, 1: height as f32},
        ];

        let mut vao = 0;
        let mut vbo = 0;
        let mut ebo = 0;
        let mut uv_vbo = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);
            
            gl::BindVertexArray(vao);
            
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER, 0, std::ptr::null(),  gl::DYNAMIC_DRAW);
            gl::VertexAttribPointer(
                0,
                2,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<crate::math::Coordinate2D>() as i32,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);
            
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo); 
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, 0, std::ptr::null(), gl::DYNAMIC_DRAW);
            
            gl::GenBuffers(1, &mut uv_vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, uv_vbo);
            gl::BufferData(gl::ARRAY_BUFFER, 0, std::ptr::null(), gl::DYNAMIC_DRAW);
            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, std::mem::size_of::<crate::math::Coordinate2D>() as i32, std::ptr::null());
            gl::EnableVertexAttribArray(1);
        }
        

        Surface {
            texture_id,
            width,
            height,
            corners,
            data,
            vao,
            vbo,
            ebo,
            uv_vbo,
        }
    }

    pub fn strech(&mut self, new_corners: [Coordinate2D; 4]) {
        self.corners = new_corners
    }

    pub fn bind(&mut self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
        }
    }

    pub fn update_gl_proprieties(&mut self, pos_x: i32, pos_y: i32) {

    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.texture_id);  // or whatever your texture ID is
        }
    }
}