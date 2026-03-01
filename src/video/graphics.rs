use sdl2::render::Canvas;

use crate::{CatEngine, Renderer};
use crate::color::Color;
use crate::shape::point::{self, Point};
use crate::video::surface::Surface;
use std::f64::consts::PI;
use std::fs::ReadDir;
use glam::{ Vec3, Mat4 };
use std::sync::Arc;
use std::ffi::CString;
use std::ptr;
use std::iter;

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let buffer: Vec<u8> = iter::repeat(b' ')
    .take(len)
    .collect();

unsafe { CString::from_vec_unchecked(buffer) }
}

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

#[repr(C)]
#[derive(Clone, Copy)]
struct Vertex {
    pos: [f32; 3],
    tex: [f32; 2],
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

//     pub fn append_cube(&mut self, cube: &Cube) {
//         let base_index = self.vertices.len();

//         // Create all 8 cube corners
//         let top_front_left     = ThirdDimensionCoordinate::new(cube.position.x, cube.position.y, cube.position.z);
//         let top_front_right    = ThirdDimensionCoordinate::new(cube.position.x + cube.width as f64, cube.position.y, cube.position.z);
//         let top_back_left      = ThirdDimensionCoordinate::new(cube.position.x, cube.position.y, cube.position.z - cube.width as f64);
//         let top_back_right     = ThirdDimensionCoordinate::new(cube.position.x + cube.width as f64, cube.position.y, cube.position.z - cube.width as f64);
//         let bottom_front_left  = ThirdDimensionCoordinate::new(cube.position.x, cube.position.y - cube.height as f64, cube.position.z);
//         let bottom_front_right = ThirdDimensionCoordinate::new(cube.position.x + cube.width as f64, cube.position.y - cube.height as f64, cube.position.z);
//         let bottom_back_left   = ThirdDimensionCoordinate::new(cube.position.x, cube.position.y - cube.height as f64, cube.position.z - cube.width as f64);
//         let bottom_back_right  = ThirdDimensionCoordinate::new(cube.position.x + cube.width as f64, cube.position.y - cube.height as f64, cube.position.z - cube.width as f64);

//         // Push vertices
//         self.vertices.push(top_front_left);     // 0
//         self.vertices.push(top_front_right);    // 1
//         self.vertices.push(top_back_left);      // 2
//         self.vertices.push(top_back_right);     // 3
//         self.vertices.push(bottom_front_left);  // 4
//         self.vertices.push(bottom_front_right); // 5
//         self.vertices.push(bottom_back_left);   // 6
//         self.vertices.push(bottom_back_right);  // 7

//         // Add **faces as triangles** (each face = 2 triangles)
//         // Top face
//         self.edges.push((base_index+0, base_index+1, base_index+2, cube.texture_surface.clone()));
//         self.edges.push((base_index+1, base_index+3, base_index+2, cube.texture_surface.clone()));

//         // Bottom face
//         self.edges.push((base_index+4, base_index+6, base_index+5, cube.texture_surface.clone()));
//         self.edges.push((base_index+5, base_index+6, base_index+7, cube.texture_surface.clone()));

//         // Front face
//         self.edges.push((base_index+0, base_index+4, base_index+1, cube.texture_surface.clone()));
//         self.edges.push((base_index+1, base_index+4, base_index+5, cube.texture_surface.clone()));

//         // Back face
//         self.edges.push((base_index+2, base_index+3, base_index+6, cube.texture_surface.clone()));
//         self.edges.push((base_index+3, base_index+7, base_index+6, cube.texture_surface.clone()));

//         // Left face
//         self.edges.push((base_index+0, base_index+2, base_index+4, cube.texture_surface.clone()));
//         self.edges.push((base_index+2, base_index+6, base_index+4, cube.texture_surface.clone()));

//         // Right face
//         self.edges.push((base_index+1, base_index+5, base_index+3, cube.texture_surface.clone()));
//         self.edges.push((base_index+3, base_index+5, base_index+7, cube.texture_surface.clone()));
//         // 0 = origin_point
//         // 1 = top_right_up_point
//         // 2 = top_left_bottom_point
//         // 3 = top_right_bottom_point
//         // 4 = bottom_left_up_point
//         // 5 = bottom_right_up_point
//         // 6 = bottom_left_bottom_point
//         // 7 = bottom_right_bottom_point
//     }
}

pub struct Cube {
    vao: u32,
    vbo: u32,
    texture_index: usize,
    position: Vec3,
    width: f32,
    height: f32,
    depth: f32,
}

impl Cube {
    pub fn new(position: Vec3, width: f32, height: f32, depth: f32, texture_index: usize) -> Self {
        let vertices: [Vertex; 36] = [
            // FRONT
            Vertex { pos: [0.0, 0.0, 0.0], tex: [0.0, 0.0] },
            Vertex { pos: [1.0, 0.0, 0.0], tex: [1.0, 0.0] },
            Vertex { pos: [1.0, 1.0, 0.0], tex: [1.0, 1.0] },
            Vertex { pos: [1.0, 1.0, 0.0], tex: [1.0, 1.0] },
            Vertex { pos: [0.0, 1.0, 0.0], tex: [0.0, 1.0] },
            Vertex { pos: [0.0, 0.0, 0.0], tex: [0.0, 0.0] },

            // BACK
            Vertex { pos: [1.0, 0.0, -1.0], tex: [0.0, 0.0] },
            Vertex { pos: [0.0, 0.0, -1.0], tex: [1.0, 0.0] },
            Vertex { pos: [0.0, 1.0, -1.0], tex: [1.0, 1.0] },
            Vertex { pos: [0.0, 1.0, -1.0], tex: [1.0, 1.0] },
            Vertex { pos: [1.0, 1.0, -1.0], tex: [0.0, 1.0] },
            Vertex { pos: [1.0, 0.0, -1.0], tex: [0.0, 0.0] },

            // LEFT
            Vertex { pos: [0.0, 0.0, -1.0], tex: [0.0, 0.0] },
            Vertex { pos: [0.0, 0.0, 0.0], tex: [1.0, 0.0] },
            Vertex { pos: [0.0, 1.0, 0.0], tex: [1.0, 1.0] },
            Vertex { pos: [0.0, 1.0, 0.0], tex: [1.0, 1.0] },
            Vertex { pos: [0.0, 1.0, -1.0], tex: [0.0, 1.0] },
            Vertex { pos: [0.0, 0.0, -1.0], tex: [0.0, 0.0] },

            // RIGHT
            Vertex { pos: [1.0, 0.0, 0.0], tex: [0.0, 0.0] },
            Vertex { pos: [1.0, 0.0, -1.0], tex: [1.0, 0.0] },
            Vertex { pos: [1.0, 1.0, -1.0], tex: [1.0, 1.0] },
            Vertex { pos: [1.0, 1.0, -1.0], tex: [1.0, 1.0] },
            Vertex { pos: [1.0, 1.0, 0.0], tex: [0.0, 1.0] },
            Vertex { pos: [1.0, 0.0, 0.0], tex: [0.0, 0.0] },

            // TOP
            Vertex { pos: [0.0, 1.0, 0.0], tex: [0.0, 0.0] },
            Vertex { pos: [1.0, 1.0, 0.0], tex: [1.0, 0.0] },
            Vertex { pos: [1.0, 1.0, -1.0], tex: [1.0, 1.0] },
            Vertex { pos: [1.0, 1.0, -1.0], tex: [1.0, 1.0] },
            Vertex { pos: [0.0, 1.0, -1.0], tex: [0.0, 1.0] },
            Vertex { pos: [0.0, 1.0, 0.0], tex: [0.0, 0.0] },

            // BOTTOM
            Vertex { pos: [0.0, 0.0, -1.0], tex: [0.0, 0.0] },
            Vertex { pos: [1.0, 0.0, -1.0], tex: [1.0, 0.0] },
            Vertex { pos: [1.0, 0.0, 0.0], tex: [1.0, 1.0] },
            Vertex { pos: [1.0, 0.0, 0.0], tex: [1.0, 1.0] },
            Vertex { pos: [0.0, 0.0, 0.0], tex: [0.0, 1.0] },
            Vertex { pos: [0.0, 0.0, -1.0], tex: [0.0, 0.0] },
        ];

        let model =
            Mat4::from_translation(position) *
            Mat4::from_scale(Vec3::new(width, height, depth));

        let indices: [u32; 36] = [
            0, 1, 2, 2, 3, 0, // front
            5, 4, 7, 7, 6, 5, // back
            4, 0, 3, 3, 7, 4, // left
            1, 5, 6, 6, 2, 1, // right
            3, 2, 6, 6, 7, 3, // top
            4, 5, 1, 1, 0, 4, // bottom
        ];

        let mut vao = 0;
        let mut vbo = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<Vertex>()) as isize,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            // position
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Vertex>() as i32,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            // tex coords
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Vertex>() as i32,
                (3 * std::mem::size_of::<f32>()) as *const _,
            );
            gl::EnableVertexAttribArray(1);

            gl::BindVertexArray(0);
        }

        Cube {
            vao,
            vbo,
            texture_index,
            position,
            width,
            height,
            depth,
        }
    }

    pub fn draw(&self, renderer: &Renderer, view: Mat4, projection: Mat4) {
        let model =
            Mat4::from_translation(self.position) *
            Mat4::from_scale(Vec3::new(self.width, self.height, self.depth));

        renderer.shader.bind();
        renderer.shader.set_mat4("model", &model);
        renderer.shader.set_mat4("view", &view);
        renderer.shader.set_mat4("projection", &projection);

        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, renderer.textures[self.texture_index]);
            renderer.shader.set_int("tex", 0);

            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
            gl::BindVertexArray(0);
        }
    }
}

pub struct Shader {
    program_id: u32,
}

impl Shader {
    pub fn new(vertex_src: &str, fragment_src: &str) -> Self {
        let og_vertex_scr = vertex_src.clone();
        let og_fragment_src = fragment_src.clone();
        let vertex_src = std::fs::read_to_string(vertex_src).expect("Failed to read vertex shader");
        let fragment_src = std::fs::read_to_string(fragment_src).expect("Failed to read fragment shader");

        unsafe {
            let v_src = CString::new(vertex_src.as_str()).unwrap();
            let f_src = CString::new(fragment_src.as_str()).unwrap();

            let vertex = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex, 1, &v_src.as_ptr(), std::ptr::null());
            gl::CompileShader(vertex);
            let mut success = 0;
            gl::GetShaderiv(vertex, gl::COMPILE_STATUS, &mut success);

            if success == 0 {
                let mut len = 0;
                gl::GetShaderiv(vertex, gl::INFO_LOG_LENGTH, &mut len);

                let error = create_whitespace_cstring_with_len(len as usize);
                gl::GetShaderInfoLog(
                    vertex,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );

                println!("VERTEX SHADER ERROR: {}", error.to_string_lossy());
                println!("Loading vertex shader from: {}", og_vertex_scr.len());
                println!("Vertex shader source length: {}", vertex_src.len());
            }

            let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment, 1, &f_src.as_ptr(), std::ptr::null());
            gl::CompileShader(fragment);
            let mut success = 0;
            gl::GetShaderiv(fragment, gl::COMPILE_STATUS, &mut success);

            if success == 0 {
                let mut len = 0;
                gl::GetShaderiv(fragment, gl::INFO_LOG_LENGTH, &mut len);

                let error = create_whitespace_cstring_with_len(len as usize);
                gl::GetShaderInfoLog(
                    fragment,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );

                println!("FRAGMENT SHADER ERROR: {}", error.to_string_lossy());
                println!("Loading fragment shader from: {}", og_fragment_src.len());
                println!("Fragment shader source length: {}", fragment_src.len());
            }

            let program = gl::CreateProgram();
            gl::AttachShader(program, vertex);
            gl::AttachShader(program, fragment);
            gl::LinkProgram(program);
            let mut success = 0;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);

            if success == 0 {
                let mut len = 0;
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);

                let error = create_whitespace_cstring_with_len(len as usize);

                gl::GetProgramInfoLog(
                    program,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );

                println!("PROGRAM LINK ERROR: {}", error.to_string_lossy());
            }

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
