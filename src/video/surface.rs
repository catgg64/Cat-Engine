use crate::{math::Coordinate2D, video::surface};

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
            Coordinate2D(x as f32 / self.width as f32,           y as f32 / self.height as f32),
            Coordinate2D((x + width) as f32 / self.width as f32, y as f32 / self.height as f32),
            Coordinate2D((x + width) as f32 / self.width as f32, (y + height) as f32 / self.height as f32),
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

pub struct Tile {
    pub corners: [Coordinate2D; 4],
    pub vertices: [Coordinate2D; 4],
}

impl Tile {
    pub fn new(corners: [Coordinate2D; 4], vertices: [Coordinate2D; 4], screen_width: u32, screen_height: u32) -> Self {
        let used_corners = [corners[0].clone(), corners[1].clone(), corners[2].clone(), corners[3].clone()];
        let used_vertices = [vertices[0].clone(), vertices[1].clone(), vertices[2].clone(), vertices[3].clone()];
        
        Self { corners: used_corners, vertices: used_vertices }
    }
}

pub struct TileSet {
    pub tile_list: Vec<Tile>,
    pub surface: Surface,
    pub width: u32,
    pub height: u32,
}

impl TileSet {
    pub fn from_texture(path: &str) -> Self {
        let mut surface = Surface::from_texture(path);
        let (width, height) = (surface.width, surface.height);
        let mut tile_list: Vec<Tile> = vec![];

        Self { tile_list, surface, width, height }
    }

    pub fn simple_append_tile(&mut self, x: u32, y: u32, width: u32, height: u32) -> u32 {
        self.tile_list.push(
            Tile { corners: [
                // Nah bru i give up just asking gpt this shi man
                Coordinate2D(x as f32 / self.surface.width as f32, y as f32 / self.surface.height as f32),
                Coordinate2D((x as f32 + width as f32) / self.surface.width as f32, y as f32 / self.surface.height as f32),
                Coordinate2D((x as f32 + width as f32) / self.surface.width as f32, (y as f32 + height as f32) / self.surface.height as f32),
                Coordinate2D(x as f32 / self.surface.width as f32, (y as f32 + height as f32) / self.surface.height as f32),
                ], 
            vertices: [
                Coordinate2D(0.0, 0.0),
                Coordinate2D(self.width as f32, 0.0),
                Coordinate2D(self.width as f32, self.height as f32),
                Coordinate2D(0.0, self.height as f32),
                ]
            }
        );
        (self.tile_list.len() - (1 as usize)) as u32
    }

    pub fn append_tile(&mut self, tile: Tile) -> u32 {
        self.tile_list.push(tile);
        (self.tile_list.len() - (1 as usize)) as u32
    }

    pub fn stretch_tile(&mut self, tile: u32, width: u32, height: u32) {
        self.tile_list[tile as usize].vertices = [
            Coordinate2D(0.0, 0.0),
            Coordinate2D(width as f32, 0.0),
            Coordinate2D(width as f32, height as f32),
            Coordinate2D(0.0, height as f32),
        ]
    }
}

