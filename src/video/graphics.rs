use sdl2::render::Canvas;

use crate::Renderer;
use crate::color::Color;
use crate::shape::point::{self, Point};
use crate::video::surface::Surface;
use std::f64::consts::PI;
use std::fs::ReadDir;
use glam::Vec3;
use std::sync::Arc;
use std::ffi::CString;
use std::ptr;


pub fn rotate(
    origin: (f64, f64),
    point: (f64, f64),
    angle: f64,
) -> (f64, f64) {
    let (ox, oy) = origin;
    let (px, py) = point;

    let cos_theta = angle.cos();
    let sin_theta = angle.sin();

    let qx = ox + cos_theta * (px - ox) - sin_theta * (py - oy);
    let qy = oy + sin_theta * (px - ox) + cos_theta * (py - oy);

    (qx, qy)
}
pub struct Coordinate {
    x: f64,
    y: f64,
}

impl Coordinate {
    pub fn new(x: f64, y: f64) -> Self {
        Coordinate { x, y }
    }
    
    pub fn get_xy(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    pub fn turn_into_point(&self) -> Point {
        Point::new(self.x, self.y).unwrap()
    }
}

pub struct ThirdDimensionCoordinate {
    x: f64,
    y: f64,
    z: f64,   
}

impl ThirdDimensionCoordinate {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        ThirdDimensionCoordinate { x, y, z }
    }

    pub fn get_xyz(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }

    pub fn turn_into_xy(
    &self,
    camera_x: f64,
    camera_y: f64,
    camera_z: f64,
    screen_width: i32,
    screen_height: i32,
    fov: i16,
    yaw: f64,
    pitch: f64,
) -> Result<Coordinate, String> {
        let dx = (self.x - camera_x) as f64;
        let dy = (self.y - camera_y) as f64;
        let dz = (self.z - camera_z) as f64;

        let fov = fov as f64;
        let yaw_radians = (yaw as f64);
        let pitch_radians = (pitch as f64);

        let mut dx = dx;
        let mut dy = dy;
        let mut dz = dz;

        // Yaw (rotate around Y axis)
        let (new_dx, new_dz) = rotate((0.0, 0.0), (dx, dz), yaw_radians as f64);
        dx = new_dx;
        dz = new_dz;

        // Pitch (rotate around X axis)
        let (new_dy, new_dz) = rotate((0.0, 0.0), (dy, dz), pitch_radians as f64);
        dy = new_dy;
        dz = new_dz;
        let mut projected_x: f64 = 0.0;
        let mut projected_y: f64 = 0.0;

        if dz <= 0.1 {
            return Err("out of bounds".to_string());
        }
        else {
            projected_x = (dx * fov) / dz;
            projected_y = (dy * fov) / dz;
        }

        
        let screen_x = screen_width as f64 / 2.0 + projected_x;
        let screen_y = screen_height as f64 / 2.0 + projected_y;

        Ok(Coordinate::new(screen_x, screen_y))
    }
}

pub struct Mesh {
    vertices: Vec<ThirdDimensionCoordinate>,
    edges: Vec<(usize, usize, usize, crate::video::surface::Surface)>,
}

impl Mesh {
    pub fn new(vertices: Vec<ThirdDimensionCoordinate>, edges: Vec<(usize, usize, usize, crate::video::surface::Surface)>) -> Self {
        Self {
            vertices, edges,
        }
    }

    pub fn draw(&mut self, renderer: &Renderer, color: Color, camera_x: f64, camera_y: f64, camera_z: f64, screen_width: i32, screen_height: i32, fov: i16, yaw: f64, pitch: f64) {
        for edge in &mut self.edges {
            let a = self.vertices[edge.0]
                .turn_into_xy(camera_x, camera_y, camera_z, screen_width, screen_height, fov, yaw, pitch);

            let b = self.vertices[edge.1]
                .turn_into_xy(camera_x, camera_y, camera_z, screen_width, screen_height, fov, yaw, pitch);

            let c = self.vertices[edge.2]
                .turn_into_xy(camera_x, camera_y, camera_z, screen_width, screen_height, fov, yaw, pitch);

            if let (Ok(p1), Ok(p2), Ok(p3)) = (a, b, c) {
                let (r, g, b) = color.return_rgb();
                let new_cords = [[p1.x, p1.y], [p1.x, p3.y], [p2.x, p2.y], [p2.x, p1.y]];
                let new_cords_as_tuples: [(f32, f32); 4] = [
                    (new_cords[0][0] as f32, new_cords[0][1] as f32),
                    (new_cords[1][0] as f32, new_cords[1][1] as f32),
                    (new_cords[2][0] as f32, new_cords[2][1] as f32),
                    (new_cords[3][0] as f32, new_cords[3][1] as f32),
                ];

                renderer.draw_line(p1.turn_into_point(), p2.turn_into_point(), Vec3::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0));
                renderer.draw_line(p1.turn_into_point(), p3.turn_into_point(), Vec3::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0));
                renderer.draw_line(p2.turn_into_point(), p3.turn_into_point(), Vec3::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0));
                renderer.draw(&edge.3, p1.x as f32, p1.y as f32);

            }
        }
    }

    pub fn append_cube(&mut self, cube: &Cube) {
        let base_index = self.vertices.len();

        // Create all 8 cube corners
        let top_front_left     = ThirdDimensionCoordinate::new(cube.position.x, cube.position.y, cube.position.z);
        let top_front_right    = ThirdDimensionCoordinate::new(cube.position.x + cube.width as f64, cube.position.y, cube.position.z);
        let top_back_left      = ThirdDimensionCoordinate::new(cube.position.x, cube.position.y, cube.position.z - cube.width as f64);
        let top_back_right     = ThirdDimensionCoordinate::new(cube.position.x + cube.width as f64, cube.position.y, cube.position.z - cube.width as f64);
        let bottom_front_left  = ThirdDimensionCoordinate::new(cube.position.x, cube.position.y - cube.height as f64, cube.position.z);
        let bottom_front_right = ThirdDimensionCoordinate::new(cube.position.x + cube.width as f64, cube.position.y - cube.height as f64, cube.position.z);
        let bottom_back_left   = ThirdDimensionCoordinate::new(cube.position.x, cube.position.y - cube.height as f64, cube.position.z - cube.width as f64);
        let bottom_back_right  = ThirdDimensionCoordinate::new(cube.position.x + cube.width as f64, cube.position.y - cube.height as f64, cube.position.z - cube.width as f64);

        // Push vertices
        self.vertices.push(top_front_left);     // 0
        self.vertices.push(top_front_right);    // 1
        self.vertices.push(top_back_left);      // 2
        self.vertices.push(top_back_right);     // 3
        self.vertices.push(bottom_front_left);  // 4
        self.vertices.push(bottom_front_right); // 5
        self.vertices.push(bottom_back_left);   // 6
        self.vertices.push(bottom_back_right);  // 7

        // Add **faces as triangles** (each face = 2 triangles)
        // Top face
        self.edges.push((base_index+0, base_index+1, base_index+2, cube.texture_surface.clone()));
        self.edges.push((base_index+1, base_index+3, base_index+2, cube.texture_surface.clone()));

        // Bottom face
        self.edges.push((base_index+4, base_index+6, base_index+5, cube.texture_surface.clone()));
        self.edges.push((base_index+5, base_index+6, base_index+7, cube.texture_surface.clone()));

        // Front face
        self.edges.push((base_index+0, base_index+4, base_index+1, cube.texture_surface.clone()));
        self.edges.push((base_index+1, base_index+4, base_index+5, cube.texture_surface.clone()));

        // Back face
        self.edges.push((base_index+2, base_index+3, base_index+6, cube.texture_surface.clone()));
        self.edges.push((base_index+3, base_index+7, base_index+6, cube.texture_surface.clone()));

        // Left face
        self.edges.push((base_index+0, base_index+2, base_index+4, cube.texture_surface.clone()));
        self.edges.push((base_index+2, base_index+6, base_index+4, cube.texture_surface.clone()));

        // Right face
        self.edges.push((base_index+1, base_index+5, base_index+3, cube.texture_surface.clone()));
        self.edges.push((base_index+3, base_index+5, base_index+7, cube.texture_surface.clone()));
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
    texture_surface: Surface,
}

impl Cube {
    pub fn new(position: ThirdDimensionCoordinate, width: i64, height: i64) -> Self {
        let texture_surface: Surface = Surface::new("mel.png");
        Self { position, width, height, texture_surface }
    }

    pub fn draw(&self, mut renderer: &mut Renderer, camera_x: f64, camera_y: f64, camera_z: f64, screen_width: i32, screen_height: i32, fov: i16, yaw: f64, pitch: f64) {
        let mut try_draw = |a: &Result<Coordinate, String>, 
                b: &Result<Coordinate, String>| {
    
    if let (Ok(p1), Ok(p2)) = (a, b) {
        let _ = renderer.draw_line(
            p1.turn_into_point(),
            p2.turn_into_point(),
            Vec3 { x: 1.0, y: 1.0, z: 1.0 }
        );
    }
    };
        let origin_point = self.position.turn_into_xy(camera_x, camera_y, camera_z, screen_width, screen_height, fov, yaw, pitch);
        let top_right_up_point = ThirdDimensionCoordinate::new(self.position.x + self.width as f64, self.position.y, self.position.z).turn_into_xy(camera_x, camera_y, camera_z, screen_width, screen_height, fov, yaw, pitch);
        let top_left_bottom_point = ThirdDimensionCoordinate::new(self.position.x, self.position.y, self.position.z - self.width as f64).turn_into_xy(camera_x, camera_y, camera_z, screen_width, screen_height, fov, yaw, pitch);
        let top_right_bottom_point = ThirdDimensionCoordinate::new(self.position.x + self.width as f64, self.position.y, self.position.z - self.width as f64).turn_into_xy(camera_x, camera_y, camera_z, screen_width, screen_height, fov, yaw, pitch);
        let bottom_left_up_point = ThirdDimensionCoordinate::new(self.position.x, self.position.y - self.height as f64, self.position.z).turn_into_xy(camera_x, camera_y, camera_z, screen_width, screen_height, fov, yaw, pitch);
        let bottom_right_up_point = ThirdDimensionCoordinate::new(self.position.x + self.width as f64, self.position.y - self.height as f64, self.position.z).turn_into_xy(camera_x, camera_y, camera_z, screen_width, screen_height, fov, yaw, pitch);
        let bottom_left_bottom_point = ThirdDimensionCoordinate::new(self.position.x, self.position.y - self.height as f64, self.position.z - self.width as f64).turn_into_xy(camera_x, camera_y, camera_z, screen_width, screen_height, fov, yaw, pitch);
        let bottom_right_bottom_point = ThirdDimensionCoordinate::new(self.position.x + self.width as f64, self.position.y - self.height as f64, self.position.z - self.width as f64).turn_into_xy(camera_x, camera_y, camera_z, screen_width, screen_height, fov, yaw, pitch);
        

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

pub struct Shader {
    program_id: u32,
}

impl Shader {
    pub fn new(vertex_src: &str, fragment_src: &str) -> Self {
        let vertex_src = std::fs::read_to_string(vertex_src).expect("Failed to read vertex shader");
        let fragment_src = std::fs::read_to_string(fragment_src).expect("Failed to read fragment shader");

        unsafe {
            let v_src = CString::new(vertex_src.as_str()).unwrap();
            let f_src = CString::new(fragment_src.as_str()).unwrap();

            let vertex = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex, 1, &v_src.as_ptr(), std::ptr::null());
            gl::CompileShader(vertex);

            let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment, 1, &f_src.as_ptr(), std::ptr::null());
            gl::CompileShader(fragment);
            
            let program = gl::CreateProgram();
            gl::AttachShader(program, vertex);
            gl::AttachShader(program, fragment);
            gl::LinkProgram(program);

            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);

            Shader { program_id: program }
        }
    }
    pub fn bind(&self) {
            unsafe {
                gl::UseProgram(self.program_id);
            }
        }
    
    pub fn set_mat4(&self, name: &str, mat: &glam::Mat4) {
        unsafe {
            let location = gl::GetUniformLocation(
                self.program_id,
                std::ffi::CString::new(name).unwrap().as_ptr()
            );
            gl::UniformMatrix4fv(location, 1, gl::FALSE, mat.to_cols_array().as_ptr());
        }
    }

    pub fn set_vec3(&self, name: &str, value: glam::Vec3) {
        let cname = CString::new(name).expect("Uniform name had null byte");

        unsafe {
            let location = gl::GetUniformLocation(self.program_id, cname.as_ptr());
            gl::Uniform3f(location, value.x, value.y, value.z);
        }
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.program_id);
        }
    }
    pub fn set_int(&self, name: &str, value: i32) {
        let cname = CString::new(name).unwrap();
        unsafe {
            gl::Uniform1i(
                gl::GetUniformLocation(self.program_id, cname.as_ptr()),
                value,
            );
        }
    }
}
