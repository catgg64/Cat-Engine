use sdl2::render::Canvas;

use crate::color::Color;
use crate::shape::point::{self, Point};
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
) -> Result<Coordinate, String> {
        let dx = (self.x - camera_x) as f64;
        let dy = (self.y - camera_y) as f64;
        let dz = (self.z - camera_z) as f64;

        if dz <= 0.0 {
            return Err("out of bound error".to_string())
        }

        let fov = fov as f64;

        let mut projected_x: f64 = 0.0;
        let mut projected_y: f64 = 0.0;

        if dz == 0.0 {
            projected_x = (dx * fov);
            projected_y = (dx * fov);
        }
        else {
            projected_x = (dx * fov) / dz;
            projected_y = (dy * fov) / dz;
        }
        let screen_x = screen_width as f64 / 2.0 + projected_x;
        let screen_y = screen_height as f64 / 2.0 + projected_y;

        Ok(Coordinate::new(screen_x as i64, screen_y as i64))
    }
}

pub struct Mesh {
    vertices: Vec<ThirdDimensionCoordinate>,
    edges: Vec<(usize, usize)>,
}

impl Mesh {
    pub fn new(vertices: Vec<ThirdDimensionCoordinate>, edges: Vec<(usize, usize)>) -> Self {
        Self {
            vertices, edges,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<sdl2::video::Window>, color: Color, camera_x: i64, camera_y: i64, camera_z: i64, screen_width: i32, screen_height: i32, fov: i16) {
        for edge in &self.edges {
            //if self.vertices[edge.0].get_xyz().2 - camera_z > 0 || self.vertices[edge.1].get_xyz().2 - camera_z > 0 {
            let point_1 = self.vertices[edge.0].turn_into_xy(camera_x, camera_y, camera_z, screen_width, screen_height, fov).unwrap().turn_into_point();
            let point_2 = self.vertices[edge.1].turn_into_xy(camera_x, camera_y, camera_z, screen_width, screen_height, fov).unwrap().turn_into_point();
            draw::line(canvas, color, point_1, point_2);
            //}
        }
    }

    pub fn append_cube(& mut self, cube: Cube) {
        let base_index = self.vertices.len();

        self.vertices.push(ThirdDimensionCoordinate::new(cube.position.x, cube.position.y, cube.position.z));
        self.vertices.push(ThirdDimensionCoordinate::new(cube.position.x + cube.width, cube.position.y, cube.position.z));
        self.vertices.push(ThirdDimensionCoordinate::new(cube.position.x, cube.position.y, cube.position.z - cube.width));
        self.vertices.push(ThirdDimensionCoordinate::new(cube.position.x + cube.width, cube.position.y, cube.position.z - cube.width));
        self.vertices.push(ThirdDimensionCoordinate::new(cube.position.x, cube.position.y - cube.height, cube.position.z));
        self.vertices.push(ThirdDimensionCoordinate::new(cube.position.x + cube.width, cube.position.y - cube.height, cube.position.z));
        self.vertices.push(ThirdDimensionCoordinate::new(cube.position.x, cube.position.y - cube.height, cube.position.z - cube.width));
        self.vertices.push(ThirdDimensionCoordinate::new(cube.position.x + cube.width, cube.position.y - cube.height, cube.position.z - cube.width));
    
        self.edges.push((base_index + 0, base_index + 1));
        self.edges.push((base_index + 0, base_index + 2));
        self.edges.push((base_index + 3, base_index + 2));
        self.edges.push((base_index + 3, base_index + 1));
        self.edges.push((base_index + 4, base_index + 5));
        self.edges.push((base_index + 4, base_index + 6));
        self.edges.push((base_index + 7, base_index + 6));
        self.edges.push((base_index + 7, base_index + 5));
        self.edges.push((base_index + 0, base_index + 4));
        self.edges.push((base_index + 1, base_index + 5));
        self.edges.push((base_index + 2, base_index + 6));
        self.edges.push((base_index + 3, base_index + 7));

        // 0 = origin_point
        // 1 = top_right_up_point
        // 2 = top_left_bottom_point
        // 3 = top_right_bottom_point
        // 4 = bottom_left_up_point
        // 5 = bottom_right_up_point
        // 6 = bottom_left_bottom_point
        // 7 = bottom_right_bottom_point
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

    pub fn draw(&self, mut canvas: &mut Canvas<sdl2::video::Window>, camera_x: i64, camera_y: i64, camera_z: i64, screen_width: i32, screen_height: i32, fov: i16) {
        let mut try_draw = |a: &Result<Coordinate, String>, 
                b: &Result<Coordinate, String>| {
    
    if let (Ok(p1), Ok(p2)) = (a, b) {
        let _ = draw::line(
            &mut canvas,
            crate::color::Color::new(255, 255, 255),
            p1.turn_into_point(),
            p2.turn_into_point(),
        );
    }
    };
        let origin_point = self.position.turn_into_xy(camera_x, camera_y, camera_z, screen_width, screen_height, fov);
        let top_right_up_point = ThirdDimensionCoordinate::new(self.position.x + self.width, self.position.y, self.position.z).turn_into_xy(camera_x, camera_y, camera_z, screen_width, screen_height, fov);
        let top_left_bottom_point = ThirdDimensionCoordinate::new(self.position.x, self.position.y, self.position.z - self.width).turn_into_xy(camera_x, camera_y, camera_z, screen_width, screen_height, fov);
        let top_right_bottom_point = ThirdDimensionCoordinate::new(self.position.x + self.width, self.position.y, self.position.z - self.width).turn_into_xy(camera_x, camera_y, camera_z, screen_width, screen_height, fov);
        let bottom_left_up_point = ThirdDimensionCoordinate::new(self.position.x, self.position.y - self.height, self.position.z).turn_into_xy(camera_x, camera_y, camera_z, screen_width, screen_height, fov);
        let bottom_right_up_point = ThirdDimensionCoordinate::new(self.position.x + self.width, self.position.y - self.height, self.position.z).turn_into_xy(camera_x, camera_y, camera_z, screen_width, screen_height, fov);
        let bottom_left_bottom_point = ThirdDimensionCoordinate::new(self.position.x, self.position.y - self.height, self.position.z - self.width).turn_into_xy(camera_x, camera_y, camera_z, screen_width, screen_height, fov);
        let bottom_right_bottom_point = ThirdDimensionCoordinate::new(self.position.x + self.width, self.position.y - self.height, self.position.z - self.width).turn_into_xy(camera_x, camera_y, camera_z, screen_width, screen_height, fov);
        

        try_draw( &origin_point, &top_right_up_point);
        try_draw( &origin_point, &top_left_bottom_point);
        try_draw( &top_right_bottom_point, &top_left_bottom_point);
        try_draw( &top_right_bottom_point, &top_right_up_point);
        try_draw( &bottom_left_up_point, &bottom_right_up_point);
        try_draw( &bottom_left_up_point, &bottom_left_bottom_point);
        try_draw( &bottom_right_bottom_point, &bottom_left_bottom_point);
        try_draw( &bottom_right_bottom_point, &bottom_right_up_point);
        try_draw( &origin_point, &bottom_left_up_point);
        try_draw( &top_right_up_point, &bottom_right_up_point);
        try_draw( &top_left_bottom_point, &bottom_left_bottom_point);
        try_draw( &top_right_bottom_point, &bottom_right_bottom_point);
        }
}