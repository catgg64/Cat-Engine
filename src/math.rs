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
}

impl Clone for Coordinate2D {
    fn clone(&self) -> Self {
        Coordinate2D(self.0, self.1)
    }
}

pub struct Coordinate3D(pub f64, pub f64, pub f64);

pub fn pixels_to_gl_coordinates(pos_x: f32, pos_y: f32, screen_width: u32, screen_height: u32) -> (f32, f32) {
    let sw = screen_width as f32;
    let sh = screen_height as f32;

    let x1 = -1.0 + pos_x * (2.0 / sw);
    let y1 = 1.0 - pos_y * (2.0 / sh);

    (x1, y1)
}