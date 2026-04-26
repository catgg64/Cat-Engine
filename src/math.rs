use std::ops::Add;

#[repr(C)]
pub struct Coordinate2D(pub f32, pub f32);

impl Coordinate2D {
    pub fn turn_into_gl_coordinates(&mut self, screen_width: u32, screen_height: u32) {
        let coord = pixels_to_gl_coordinates(self.0, self.1, screen_width, screen_height);
        self.0 = coord.0;
        self.1 = coord.1;
    }

    pub fn return_into_gl_coordinates(&self, screen_width: u32, screen_height: u32) -> Self {
        let coord = pixels_to_gl_coordinates(self.0, self.1, screen_width, screen_height);
        Self {
            0: coord.0,
            1: coord.1,
        }
    }

    pub fn turn_into_uv_gl_coordinates(&mut self, screen_width: u32, screen_height: u32) {
        let coord = pixels_to_uv(self.0, self.1, screen_width, screen_height);
        self.0 = coord.0;
        self.1 = coord.1;
    }

    pub fn return_into_uv_gl_coordinates(&self, screen_width: u32, screen_height: u32) -> Self {
        let coord = pixels_to_uv(self.0, self.1, screen_width, screen_height);
        Self {
            0: coord.0,
            1: coord.1,
        }
    }

    pub fn to_tuple(&self) -> (f32, f32) {
        (self.0, self.1)
    }
}

impl Clone for Coordinate2D {
    fn clone(&self) -> Self {
        Coordinate2D(self.0, self.1)
    }
}

impl Add for Coordinate2D {
    type Output = Coordinate2D;

    fn add(self, rhs: Self) -> Self::Output {
        Coordinate2D{ 0: self.0 + rhs.0, 1: self.1 + rhs.1 }
    }
}

impl Copy for Coordinate2D {
    
}

#[repr(C)]
pub struct Coordinate3D(pub f32, pub f32, pub f32);

impl Coordinate3D {
    pub fn to_tuple(&self) -> (f32, f32, f32) {
        (self.0, self.1, self.2)
    }
}

impl Copy for Coordinate3D {
    
}

pub fn pixels_to_gl_coordinates(pos_x: f32, pos_y: f32, screen_width: u32, screen_height: u32) -> (f32, f32) {
    let sw = screen_width as f32;
    let sh = screen_height as f32;

    let x1 = -1.0 + pos_x * (2.0 / sw);
    let y1 = 1.0 - pos_y * (2.0 / sh);

    (x1, y1)
}

pub fn pixels_to_uv(
    x: f32,
    y: f32,
    tex_width: u32,
    tex_height: u32
) -> (f32, f32) {
    (
        x / tex_width as f32,
        y / tex_height as f32,
    )
}

impl Clone for Coordinate3D {
    fn clone(&self) -> Self {
        Self(self.0, self.1, self.2)
    }
}

pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    pub fn colliderect(&self, rect: &Self) -> bool {
        if self.x + self.width > rect.x
        && self.x < rect.x + rect.width
        && self.y + self.height > rect.y 
        && self.y < rect.y + rect.height {
            true
        } else {
            false
        }
    }

    pub fn collidepoint(&self, point: &Coordinate2D) -> bool {
        if self.x < point.0
        && self.y < point.1
        && self.width + self.x > point.0
        && self.height + self.y > point.1 {
            true
        } else {
            false
        }
    }
}