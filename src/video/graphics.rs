use sdl2::libc::winsize;

use crate::shape::point::Point;
use crate::video::image::draw;

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
    fov: i16,
) -> Coordinate {
        let x_player_distance = camera_x - self.x;
        let y_player_distance = camera_y - self.y;
        let z_player_distance = camera_z - self.z;
        if z_player_distance > 0 { 
            let calc_x = screen_width as f64 / 2.0 + x_player_distance as f64 / (z_player_distance as f64 / fov as f64);
            let calc_y = screen_height as f64 / 2.0 + y_player_distance as f64 / (z_player_distance as f64 / fov as f64);
            Coordinate::new(calc_x as i64, calc_y as i64) } 
        else { 
            Coordinate::new(0, 0)
        }
    }
}

pub struct Cube {
    position: ThirdDimensionCoordinate,
    width: i64,
    height: i64,
}

impl Cube {
    pub fn new(position: ThirdDimensionCoordinate, width: i64, height: i64) -> Self {
        Self { position, width, height }
    }

    pub fn draw(&self, cat_engine: &mut super::super::CatEngine, camera_x: i64, camera_y: i64, camera_z: i64) {
        let origin_point = self.position.turn_into_xy(camera_x, camera_y, camera_z, cat_engine.screen_rect.x, cat_engine.screen_rect.y, cat_engine.fov);
        let top_right_up_point = ThirdDimensionCoordinate::new(self.position.x + self.width, self.position.y, self.position.z).turn_into_xy(camera_x, camera_y, camera_z, cat_engine.screen_rect.width() as i32, cat_engine.screen_rect.height() as i32, cat_engine.fov);
        let top_left_bottom_point = ThirdDimensionCoordinate::new(self.position.x, self.position.y, self.position.z - self.width).turn_into_xy(camera_x, camera_y, camera_z, cat_engine.screen_rect.width() as i32, cat_engine.screen_rect.height() as i32, cat_engine.fov);
        let top_right_bottom_point = ThirdDimensionCoordinate::new(self.position.x + self.width, self.position.y, self.position.z - self.width).turn_into_xy(camera_x, camera_y, camera_z, cat_engine.screen_rect.width() as i32, cat_engine.screen_rect.height() as i32, cat_engine.fov);
        let bottom_left_up_point = ThirdDimensionCoordinate::new(self.position.x + self.width, self.position.y - self.height, self.position.z).turn_into_xy(camera_x, camera_y, camera_z, cat_engine.screen_rect.width() as i32, cat_engine.screen_rect.height() as i32, cat_engine.fov);
        let bottom_right_up_point = ThirdDimensionCoordinate::new(self.position.x + self.width, self.position.y - self.height, self.position.z).turn_into_xy(camera_x, camera_y, camera_z, cat_engine.screen_rect.width() as i32, cat_engine.screen_rect.height() as i32, cat_engine.fov);
        let bottom_left_bottom_point = ThirdDimensionCoordinate::new(self.position.x, self.position.y - self.height, self.position.z - self.width).turn_into_xy(camera_x, camera_y, camera_z, cat_engine.screen_rect.width() as i32, cat_engine.screen_rect.height() as i32, cat_engine.fov);
        let bottom_right_bottom_point = ThirdDimensionCoordinate::new(self.position.x + self.width, self.position.y - self.height, self.position.z - self.width).turn_into_xy(camera_x, camera_y, camera_z, cat_engine.screen_rect.width() as i32, cat_engine.screen_rect.height() as i32, cat_engine.fov);


        draw::line(&mut cat_engine.canvas, crate::color::Color::new(255, 255, 255), origin_point.turn_into_point(), top_right_up_point.turn_into_point()).unwrap();
        draw::line(&mut cat_engine.canvas, crate::color::Color::new(255, 255, 255), origin_point.turn_into_point(), top_left_bottom_point.turn_into_point()).unwrap();
        draw::line(&mut cat_engine.canvas, crate::color::Color::new(255, 255, 255), top_right_bottom_point.turn_into_point(), top_left_bottom_point.turn_into_point()).unwrap();
        draw::line(&mut cat_engine.canvas, crate::color::Color::new(255, 255, 255), top_right_bottom_point.turn_into_point(), top_right_up_point.turn_into_point()).unwrap();
        draw::line(&mut cat_engine.canvas, crate::color::Color::new(255, 255, 255), bottom_left_up_point.turn_into_point(), bottom_right_up_point.turn_into_point()).unwrap();
        draw::line(&mut cat_engine.canvas, crate::color::Color::new(255, 255, 255), bottom_left_up_point.turn_into_point(), bottom_left_bottom_point.turn_into_point()).unwrap();
        draw::line(&mut cat_engine.canvas, crate::color::Color::new(255, 255, 255), bottom_right_bottom_point.turn_into_point(), bottom_left_bottom_point.turn_into_point()).unwrap();
        draw::line(&mut cat_engine.canvas, crate::color::Color::new(255, 255, 255), bottom_right_bottom_point.turn_into_point(), bottom_right_up_point.turn_into_point()).unwrap();
        draw::line(&mut cat_engine.canvas, crate::color::Color::new(255, 255, 255), origin_point.turn_into_point(), bottom_right_up_point.turn_into_point()).unwrap();
        draw::line(&mut cat_engine.canvas, crate::color::Color::new(255, 255, 255), top_right_up_point.turn_into_point(), bottom_right_up_point.turn_into_point()).unwrap();
        draw::line(&mut cat_engine.canvas, crate::color::Color::new(255, 255, 255), top_left_bottom_point.turn_into_point(), bottom_left_bottom_point.turn_into_point()).unwrap();
        draw::line(&mut cat_engine.canvas, crate::color::Color::new(255, 255, 255), top_right_bottom_point.turn_into_point(), bottom_right_bottom_point.turn_into_point()).unwrap();
        }
}