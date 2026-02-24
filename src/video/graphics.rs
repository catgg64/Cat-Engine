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

    pub fn turn_into_xy(&self, camera_x: i64, camera_y: i64, camera_z: i64, screen_width: i32, screen_height: i32, fov: i64) -> Coordinate {
        let x_player_distance = camera_x - self.x;
        let y_player_distance = camera_y - self.y;
        let z_player_distance = camera_z - self.z;


        let calc_x  = screen_width as i64 / 2 + x_player_distance / (z_player_distance/ fov);
        let calc_y =  screen_height as i64 / 2 + y_player_distance / (z_player_distance / fov);
        Coordinate::new(calc_x, calc_y)
    }
}