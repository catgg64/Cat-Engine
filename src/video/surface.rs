use crate::math::{ Coordinate2D, Coordinate3D, Rect };

pub struct Surface {
    pub texture_id: u32,
    pub width: u32,
    pub height: u32,
    pub corners:  [Coordinate2D; 4],
    pub vertices: [Coordinate3D; 4],
    pub data: Vec<u8>,
}

impl Surface {
    //! # Surface
    //! 
    //! Often used for images.
    
    /// Start off a brand new, empty surface.
    pub fn new(width: usize, height: usize) -> Self {
        let mut data = vec![0u8; width * height * 4];

        let mut texture_id = 0;
        
        unsafe {
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
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
            Coordinate2D(0.0, 0.0),
            Coordinate2D(1.0, 0.0),
            Coordinate2D(1.0, 1.0),
            Coordinate2D(0.0, 1.0),
        ];

        let mut vertices = [
            Coordinate3D(0.0, 0.0, 0.0),
            Coordinate3D(width as f32, 0.0, 0.0),
            Coordinate3D(width as f32, height as f32, 0.0),
            Coordinate3D(0.0, height as f32, 0.0),
        ];

        Self { texture_id, width: width as u32, height: height as u32, corners, vertices, data }
    }

    pub fn from_width_height_verticies_data(width: usize, vertices: [Coordinate3D; 4], corners: [Coordinate2D; 4], data: Vec<u8>, height: usize) -> Self {
        let mut texture_id = 0;
        
        unsafe {
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
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

        Self { texture_id, width: width as u32, height: height as u32, corners, vertices, data }
    }

    /// Starts a surface from a texture.
    pub fn from_texture(path: &str) -> Self {
        let image = image::open(path).expect("Error loading the image: ");
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let mut data = image.into_raw();
        
        let mut texture_id = 0;
        
        unsafe {
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
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
            Coordinate2D(0.0, 0.0),
            Coordinate2D(1.0, 0.0),
            Coordinate2D(1.0, 1.0),
            Coordinate2D(0.0, 1.0),
        ];

        let mut vertices = [
            Coordinate3D(0.0, 0.0, 0.0),
            Coordinate3D(width as f32, 0.0, 0.0),
            Coordinate3D(width as f32, height as f32, 0.0),
            Coordinate3D(0.0, height as f32, 0.0),
        ];

        Surface {
            texture_id,
            width,
            height,
            corners,
            vertices,
            data,
        }
    }

    /// Blits a Surface on top of itself.
    pub fn blit(&mut self, surface: &Surface, x_offset: u32, y_offset: u32) {
        for y in 0..surface.height as usize {
            for x in 0..surface.width as usize {
                let dst_x = x + x_offset as usize;
                let dst_y = y + y_offset as usize;

                if dst_x >= self.width as usize || dst_y >= self.height as usize {
                    continue;
                }

                let atlas_index =
                    (dst_y * self.width as usize + dst_x) * 4;

                let src_index =
                    (y * surface.width as usize + x) * 4;

                self.data[atlas_index..atlas_index + 4]
                    .copy_from_slice(&surface.data[src_index..src_index + 4]);
            }
        }
        self.upload();
    }

    pub fn upload(&mut self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
            gl::TexImage2D(gl::TEXTURE_2D,
                           0,
                           gl::RGBA as i32,
                           self.width as i32,
                           self.height as i32,
                           0,
                           gl::RGBA,
                           gl::UNSIGNED_BYTE,
                           self.data.as_ptr() as *const _);
        }
    }

    pub fn crop(&mut self, x: u32, y: u32, width: u32, height: u32) {
        self.corners = [
            Coordinate2D(x as f32 / self.width as f32,           y as f32 / self.height as f32),
            Coordinate2D((x + width) as f32 / self.width as f32, y as f32 / self.height as f32),
            Coordinate2D((x + width) as f32 / self.width as f32, (y + height) as f32 / self.height as f32),
            Coordinate2D(x as f32 / self.width as f32,           (y + height) as f32 / self.height as f32),
        ];
    }

    pub fn stretch(&mut self, width: i32, height: i32) {
        self.vertices = [
            Coordinate3D(0.0, 0.0, self.vertices[0].2),
            Coordinate3D(width as f32, 0.0, self.vertices[1].2),
            Coordinate3D(width as f32, height as f32, self.vertices[2].2),
            Coordinate3D(0.0, height as f32, self.vertices[3].2),
        ];
        self.width = width as u32;
        self.height = height as u32;
    }

    pub fn set_z(&mut self, z: f32) {
        for vertex in self.vertices.iter_mut() {
            vertex.2 = z;
        }
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

    pub fn return_true_crop(&self, x: u32, y: u32, width: u32, height: u32) -> Self {
        let mut new_data = vec![0u8; (width * height * 4) as usize];

        for row in 0..height {
            for col in 0..width {
                let src_x = x + col;
                let src_y = y + row;

                let src_index =
                    ((src_y * self.width + src_x) * 4) as usize;

                let dst_index =
                    ((row * width + col) * 4) as usize;

                new_data[dst_index..dst_index + 4]
                    .copy_from_slice(&self.data[src_index..src_index + 4]);
            }
        }

        let mut new_surface = Surface::new(width as usize, height as usize);
        new_surface.data = new_data;
        new_surface.upload();

        new_surface
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
        }
    }

    pub fn get_rect(&self) -> Rect {
        Rect { x: 0.0, y: 0.0, width: self.vertices[1].0, height: self.vertices[2].1 }
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.texture_id);
        }
    }
}

impl Clone for Surface {
    fn clone(&self) -> Self {
        Self::from_width_height_verticies_data(self.width as usize, self.vertices.clone(), self.corners.clone(), self.data.clone(), self.height as usize)
    }
}

#[derive(Clone)]
pub struct Tile {
    pub corners: [Coordinate2D; 4],
    pub vertices: [Coordinate3D; 4],
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub visual_width: f32,
    pub visual_height: f32,
}

impl Tile {
    pub fn new(corners: [Coordinate2D; 4], vertices: [Coordinate3D; 4], x: u32, y: u32, width: u32, height: u32) -> Self {
        let used_corners = [corners[0].clone(), corners[1].clone(), corners[2].clone(), corners[3].clone()];
        let used_vertices = [vertices[0].clone(), vertices[1].clone(), vertices[2].clone(), vertices[3].clone()];
        
        Self { corners: used_corners, vertices: used_vertices, x, y, width, height, visual_width: width as f32, visual_height: height as f32 }
    }
}

pub struct TileSet {
    pub tile_list: Vec<Tile>,
    pub surface: Surface,
    pub width: u32,
    pub height: u32,
}

impl TileSet {
    pub fn new(width: usize, height: usize) -> Self {
        let mut surface = Surface::new(width, height);
        Self { tile_list: vec![], surface, width: width as u32, height: height as u32 }
    }

    pub fn from_texture(path: &str) -> Self {
        let mut surface = Surface::from_texture(path);
        let (width, height) = (surface.width, surface.height);
        let mut tile_list: Vec<Tile> = vec![];

        Self { tile_list, surface, width, height }
    }

    pub fn simple_append_tile(&mut self, x: u32, y: u32, z: f32, width: u32, height: u32) -> u32 {
        self.tile_list.push(
            Tile { corners: [
                // Nah bru i give up just asking gpt this shi man
                Coordinate2D(x as f32 / self.surface.width as f32, y as f32 / self.surface.height as f32),
                Coordinate2D((x as f32 + width as f32) / self.surface.width as f32, y as f32 / self.surface.height as f32),
                Coordinate2D((x as f32 + width as f32) / self.surface.width as f32, (y as f32 + height as f32) / self.surface.height as f32),
                Coordinate2D(x as f32 / self.surface.width as f32, (y as f32 + height as f32) / self.surface.height as f32),
                ], 
            vertices: [
                Coordinate3D(0.0, 0.0, z),
                Coordinate3D(width as f32, 0.0, z),
                Coordinate3D(width as f32, height as f32, z),
                Coordinate3D(0.0, height as f32, z),
                ],
            x,
            y,
            width,
            height,
            visual_width: width as f32, 
            visual_height: height as f32
            }
        );
        (self.tile_list.len() - (1 as usize)) as u32
    }

    pub fn blit(&mut self, surface: &Surface, offset_x: u32, offset_y: u32) {
        self.surface.blit(&surface, offset_x, offset_y);
    }

    pub fn append_tile(&mut self, tile: Tile) -> u32 {
        self.tile_list.push(tile);
        (self.tile_list.len() - (1 as usize)) as u32
    }

    pub fn stretch_tile(&mut self, tile: u32, width: i32, height: i32) {
        self.tile_list[tile as usize].vertices = [
            Coordinate3D(0.0, 0.0, 0.0),
            Coordinate3D(width as f32, 0.0, 0.0),
            Coordinate3D(width as f32, height as f32, 0.0),
            Coordinate3D(0.0, height as f32, 0.0),
        ];
        self.tile_list[tile as usize].visual_width = width as f32;
        self.tile_list[tile as usize].visual_height = height as f32;
    }

    pub fn set_tile_z(&mut self, tile: u32, z: f32) {
        for vertex in self.tile_list[tile as usize].vertices.iter_mut() {
            vertex.2 = z;
        }
    }

    pub fn flipv_tile(&mut self, tile: u32) {
        for corner in self.tile_list[tile as usize].corners.iter_mut() {
            corner.1 = 1.0 - corner.1;
        }
    }

    pub fn fliph_tile(&mut self, tile: u32) {
        for corner in self.tile_list[tile as usize].corners.iter_mut() {
            corner.0 = 1.0 - corner.1;
        }
    }
    
    pub fn get_tile_rect(&self, tile: u32) -> Rect {
        Rect { x: 0.0, y: 0.0, width: self.tile_list[tile as usize].vertices[1].0, height: self.tile_list[tile as usize].vertices[2].1 }
    }
}

impl PartialEq for TileSet {
    fn eq(&self, other: &Self) -> bool {
        self.surface.texture_id == other.surface.texture_id
    }
    
    fn ne(&self, other: &Self) -> bool {
        self.surface.texture_id != other.surface.texture_id
    }
}
