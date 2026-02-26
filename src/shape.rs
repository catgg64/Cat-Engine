pub mod rect {
    pub struct Rect {
        x: i64,
        y: i64,
        size_x: i64,
        size_y: i64,
    }

    impl Rect {
        pub fn new(x: i64, y: i64, size_x: i64, size_y: i64) -> Result<Rect, String>{
            Ok (Self {
                x: x, 
                y: y, 
                size_x: size_x, 
                size_y: size_y,
                })
        }

        pub fn colliderect(&self, rect: Rect) -> bool {
            if self.x > rect.x || self.x < rect.size_x || self.y > rect.y || self.y < rect.size_y{
                true
            }
            else {
                false
            }
        }

        pub fn get_x_y_sizex_sizey(&self) -> (i64, i64, i64, i64) {
            (self.x, self.y, self.size_x, self.size_y)
        }
    }
}

pub mod point {
    pub struct Point {
        x: f64,
        y: f64,
    }

    impl Point {
        pub fn new(x: f64, y: f64) -> Result<Point, String> {
            Ok (Self {
                x: x,
                y: y,
            })
        }

        pub fn return_xy(&self) -> (f64, f64) {
            (self.x, self.y)
        }

        pub fn turn_into_sdl_point(&self) -> sdl2::rect::Point {
            sdl2::rect::Point::new(self.x as i32, self.y as i32)
        }
    }
}
