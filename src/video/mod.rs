use std::ffi::CString;

use crate::video::surface::Surface;
use crate::math::{ Coordinate2D };

pub mod surface;

pub fn start_elemnt_array() -> (u32, u32, u32, u32) {
    let mut vao = 0;
    let mut vbo = 0;
    let mut ebo = 0;
    let mut uv_vbo = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);
        
        gl::BindVertexArray(vao);
        
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER, 0, std::ptr::null(),  gl::DYNAMIC_DRAW);
        gl::VertexAttribPointer(
            0,
            2,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<crate::math::Coordinate2D>() as i32,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
        
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo); 
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, 0, std::ptr::null(), gl::DYNAMIC_DRAW);
        
        gl::GenBuffers(1, &mut uv_vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, uv_vbo);
        gl::BufferData(gl::ARRAY_BUFFER, 0, std::ptr::null(), gl::DYNAMIC_DRAW);
        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, std::mem::size_of::<crate::math::Coordinate2D>() as i32, std::ptr::null());
        gl::EnableVertexAttribArray(1);
    }
    (vao, vbo, ebo, uv_vbo)
}

pub fn update_elemnt_array(&mut vao: &mut u32, &mut vbo: &mut u32, &mut ebo: &mut u32, &mut uv_vbo: &mut u32, vertices: Vec<Coordinate2D>, uvs: Vec<Coordinate2D>, indicies: Vec<u32>) {
    unsafe {
        gl::BindVertexArray(vao);
        
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        let (_, vertex_bytes, _) = vertices.align_to::<u8>();
        gl::BufferData(gl::ARRAY_BUFFER, vertex_bytes.len() as isize, vertex_bytes.as_ptr() as *const _, gl::DYNAMIC_DRAW);
        
        gl::BindBuffer(gl::ARRAY_BUFFER, uv_vbo);
        let (_, uv_bytes, _) = uvs.align_to::<u8>();
        gl::BufferData(gl::ARRAY_BUFFER, uv_bytes.len() as isize, uv_bytes.as_ptr() as *const _, gl::DYNAMIC_DRAW);
        
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        let (_, indices_bytes, _) = indicies.align_to::<u8>();
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, indices_bytes.len() as isize, indices_bytes.as_ptr() as *const _, gl::DYNAMIC_DRAW);
    }
}

pub struct Shader {
    program_id: u32,
}

impl Shader {
    pub fn new(vert_shader_path: &str, frag_shader_path: &str) -> Shader {
        let vert_shader_file = std::fs::read_to_string(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), vert_shader_path)).expect("failed loading the vertex file: ");
        let frag_shader_file = std::fs::read_to_string(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), frag_shader_path)).expect("failed loading the fragment file: ");
        let vert_src = CString::new(vert_shader_file).unwrap();
        let frag_src = CString::new(frag_shader_file).unwrap();

        unsafe {
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            assert_ne!(vertex_shader, 0);
            gl::ShaderSource(
                vertex_shader,
                1,
                &(vert_src.as_ptr()),
                std::ptr::null(),
            );
            gl::CompileShader(vertex_shader);
            let mut success = 0;
            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetShaderInfoLog(
                vertex_shader,
                1024,
                &mut log_len,
                v.as_mut_ptr().cast(),
            );
                v.set_len(log_len.try_into().unwrap());
                panic!("Vertex Compile Error: {}", String::from_utf8_lossy(&v));
            }
            
            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            assert_ne!(fragment_shader, 0);
            gl::ShaderSource(
                fragment_shader,
                1,
                &(frag_src.as_ptr()),
                std::ptr::null(),
            );
            gl::CompileShader(fragment_shader);
            let mut success = 0;
            gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetShaderInfoLog(
                fragment_shader,
                1024,
                &mut log_len,
                v.as_mut_ptr().cast(),
                );
                v.set_len(log_len.try_into().unwrap());
                panic!("Fragment Compile Error: {}", String::from_utf8_lossy(&v));
            }

            let shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);
        
            let mut success = 0;
            gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetProgramInfoLog(
                shader_program,
                1024,
                &mut log_len,
                v.as_mut_ptr().cast(),
                );
                v.set_len(log_len.try_into().unwrap());
                panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
             }

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            Shader { program_id: shader_program }
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.program_id)
        }
    }

    pub fn set_int(&self, name: &str, value: i32) {
        let cname = CString::new(name).unwrap();
        unsafe {
            gl::Uniform1i(
                gl::GetUniformLocation(self.program_id, cname.as_ptr()),
                value,
            )
        }
    }

    pub fn set_vect2(&self, name: &str, x: f32, y: f32) {
        let cname = CString::new(name).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.program_id, cname.as_ptr());
            gl::Uniform2f(location, x, y)
        }
    }
    
    pub fn set_float(&self, name: &str, value: f32) {
       let cname = CString::new(name).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.program_id, cname.as_ptr());
            gl::Uniform1f(location, value);
        }
    }

    pub fn set_mat4(&self, name: &str, mat: [f32; 16]) {
        unsafe {
            let location = gl::GetUniformLocation(
                self.program_id,
                std::ffi::CString::new(name).unwrap().as_ptr()
            );
            gl::UniformMatrix4fv(location, 1, gl::FALSE, mat.as_ptr());
        }
    }

}


impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program_id);
        }
    }
}

pub struct Renderer {
    texture_shader: Shader,
    texture_vao: u32,
    texture_vbo: u32,
    texture_ebo: u32,
    texture_uv_vbo: u32,
    triangle_shader: Shader,
    triangle_vao: u32,
    triangle_vbo: u32,
    triangle_ebo: u32,
    triangle_uv_vbo: u32,
    screen_width: u32,
    screen_height: u32,
}

impl Renderer {
    pub fn new(screen_width: u32, screen_height: u32) -> Self {
        let texture_shader = Shader::new("texture.vert", "texture.frag");
        let triangle_shader = Shader::new("triangle.vert", "triangle.frag");
        let (texture_vao, texture_vbo, texture_ebo, texture_uv_vbo) = start_elemnt_array();
        let (triangle_vao, triangle_vbo, triangle_ebo, triangle_uv_vbo) = start_elemnt_array();

        Self { texture_shader, texture_vao, texture_vbo, texture_ebo, texture_uv_vbo, triangle_shader, triangle_vao, triangle_vbo, triangle_ebo, triangle_uv_vbo, screen_width, screen_height }
    }
    
    pub fn blit(&mut self, surface: &mut Surface, pos_x: f32, pos_y: f32) {
        let sw = self.screen_width as f32;
        let sh: f32 = self.screen_height as f32;

        let x1 = -1.0 + pos_x * (2.0 / sw);
        let y1 = 1.0 - pos_y * (2.0 / sh);
        let x0 = x1 + (surface.width as f32 * (2.0 / sw));
        let y0 = y1 - (surface.height as f32 * (2.0 / sh));

        let mut vertices: Vec<Coordinate2D> = Vec::new();
        vertices.push(Coordinate2D(x0, y0));
        vertices.push(Coordinate2D(x0, y1));
        vertices.push(Coordinate2D(x1, y1));
        vertices.push(Coordinate2D(x1, y0));

        #[rustfmt::skip]
        let indicies: Vec<u32> = vec![
            0, 1, 2,
            2, 3, 0
        ];
        
        update_elemnt_array(&mut self.texture_vao, &mut self.texture_vbo, &mut self.texture_ebo, &mut self.texture_uv_vbo, vertices, surface.corners.to_vec(), indicies);
        
        unsafe {
            self.texture_shader.bind();
            self.texture_shader.set_int("tex", 0);
            gl::ActiveTexture(gl::TEXTURE0);
            surface.bind();
            gl::BindVertexArray(self.texture_vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }
    }

    pub fn draw_triangle(&mut self, p1: Coordinate2D, p2: Coordinate2D, p3: Coordinate2D, uvs: Vec<Coordinate2D>, surface: &mut Surface) {
        let mut vertices: Vec<Coordinate2D> = Vec::new();
        vertices.push(p1.return_into_gl_coordinates(self.screen_width, self.screen_height));
        vertices.push(p2.return_into_gl_coordinates(self.screen_width, self.screen_height));
        vertices.push(p3.return_into_gl_coordinates(self.screen_width, self.screen_height));

        #[rustfmt::skip]
        let indicies: Vec<u32> = vec![
            0, 1, 2,
        ];

        update_elemnt_array(&mut self.triangle_vao, &mut self.triangle_vbo, &mut self.triangle_ebo, &mut self.triangle_uv_vbo, vertices, uvs, indicies);
        
        unsafe {
            self.triangle_shader.bind();
            self.triangle_shader.set_int("tex", 0);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindVertexArray(self.triangle_vao);
            surface.bind();
            gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, std::ptr::null());
        }
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.texture_vbo);
            gl::DeleteBuffers(1, &self.texture_uv_vbo);
            gl::DeleteBuffers(1, &self.texture_ebo);
            gl::DeleteVertexArrays(1, &self.texture_vao);
        }
    }
}