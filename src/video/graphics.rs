use crate::shape::point::Point;

pub struct Coordinate {
    x: i64,
    y: i64,
}

impl Coordinate {
    pub fn new(x: i64, y: i64) -> Self {
        Coordinate { x, y }
    }
    
    pub fn get_xy(&self) -> (i64, i64) {
        (self.x, self.y)
    }

    pub fn turn_into_point(&self) -> Point {
        Point::new(self.x, self.y).unwrap()
    }
}

pub struct ThirdDimensionCoordinate {
    x: i64,
    y: i64,
    z: i64,   
}

impl ThirdDimensionCoordinate {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        ThirdDimensionCoordinate { x, y, z }
    }

    pub fn get_xyz(&self) -> (i64, i64, i64) {
        (self.x, self.y, self.z)
    }

    pub fn turn_into_xy(
    &self,
    camera_x: i64,
    camera_y: i64,
    camera_z: i64,
    screen_width: i32,
    screen_height: i32,
    fov: i64,
) -> Coordinate {
        let dx = (self.x - camera_x) as f64;
        let dy = (self.y - camera_y) as f64;
        let dz = (self.z - camera_z) as f64;

        if dz <= 0.0 {
            return Coordinate::new(0, 0);
        }

        let fov = fov as f64;

        let projected_x = (dx * fov) / dz;
        let projected_y = (dy * fov) / dz;

        let screen_x = screen_width as f64 / 2.0 + projected_x;
        let screen_y = screen_height as f64 / 2.0 + projected_y;

        Coordinate::new(screen_x as i64, screen_y as i64)
    }
}