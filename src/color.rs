pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self{
        Color {
            r: r,
            g: g,
            b: b,
        }
    }
    pub fn return_rgb(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }
}