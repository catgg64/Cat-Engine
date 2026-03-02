use sdl2::render::Canvas;

use crate::{CatEngine, Renderer};
use crate::color::Color;
use crate::shape::point::{self, Point};
use crate::video::surface::Surface;
use std::f64::consts::PI;
use std::fs::ReadDir;
use glam::{ Mat4, Vec2, Vec3, Vec4 };
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
struct MeshVertex {
    pub position: Vec3,
    pub uv: Vec2,
}

pub struct Mesh {
    vao: u32,
    vbo: u32,
    ebo: u32,
    index_count: usize,
    texture_index: usize,
}

impl Mesh {
    pub fn new(vertices: &[f32], indices: &[u32], texture_index: usize) -> Self {
        let mut vao = 0;
        let mut vbo = 0;
        let mut ebo = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as isize,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as isize,
                indices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            // Position attribute (location = 0)
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                5 * std::mem::size_of::<f32>() as i32,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            // UV attribute (location = 1)
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                5 * std::mem::size_of::<f32>() as i32,
                (3 * std::mem::size_of::<f32>()) as *const _,
            );
            gl::EnableVertexAttribArray(1);

            gl::BindVertexArray(0);
        }

        Self {
            vao,
            vbo,
            ebo,
            index_count: indices.len(),
            texture_index,
        }
    }

    pub fn draw(&self, renderer: Renderer) {
        let mesh_vertex_path = format!("{}/mesh.vert", env!("CARGO_MANIFEST_DIR"));
        let mesh_fragment_path = format!("{}/mesh.frag", env!("CARGO_MANIFEST_DIR"));
        
        let shader = Shader::new(&mesh_vertex_path, &mesh_fragment_path);
        shader.bind();

        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, renderer.textures[self.texture_index]);
            shader.set_int("tex", 0);

            gl::BindVertexArray(self.vao);
            gl::DrawElements(gl::TRIANGLES, self.index_count as i32, gl::UNSIGNED_INT, std::ptr::null());
            gl::BindVertexArray(0);
        }
    }
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
