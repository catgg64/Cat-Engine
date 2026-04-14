use std::ffi::CString;
use glam::Mat4;

use crate::font::Font;
use crate::pixel::Color;
use crate::sprite::{self, ComplexSpriteList, SpriteList, Sprite};
use crate::video::surface::Surface;
use crate::math::{ Coordinate2D, Coordinate3D, Rect };
use crate::mesh::{ Mesh };

pub mod surface;

pub fn start_uv_elemnt_array() -> (u32, u32, u32, u32) {
    let mut vao = 0;
    let mut vbo = 0;
    let mut ebo = 0;
    let mut uv_vbo = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);
        gl::GenBuffers(1, &mut uv_vbo);
        
        gl::BindVertexArray(vao);
        
        let max_quads = 1000;

        let vertex_size = max_quads * 6 * std::mem::size_of::<Coordinate3D>();
        let uv_size     = max_quads * 4 * std::mem::size_of::<Coordinate2D>();
        let index_size  = max_quads * 6 * std::mem::size_of::<u32>();

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER, vertex_size as isize, std::ptr::null(), gl::DYNAMIC_DRAW);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, index_size as isize, std::ptr::null(), gl::DYNAMIC_DRAW);

        gl::BindBuffer(gl::ARRAY_BUFFER, uv_vbo);
        gl::BufferData(gl::ARRAY_BUFFER, uv_size as isize, std::ptr::null(), gl::DYNAMIC_DRAW);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<crate::math::Coordinate3D>() as i32,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
                
        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, std::mem::size_of::<crate::math::Coordinate2D>() as i32, std::ptr::null());
        gl::EnableVertexAttribArray(1);
    }
    (vao, vbo, ebo, uv_vbo)
}

pub fn start_element_array() -> (u32, u32, u32) {
    let mut vao = 0;
    let mut vbo = 0;
    let mut ebo = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);
        
        gl::BindVertexArray(vao);
        
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER, 4, std::ptr::null(),  gl::DYNAMIC_DRAW);
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
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, 4, std::ptr::null(), gl::DYNAMIC_DRAW);
        
        gl::EnableVertexAttribArray(1);
    }
    (vao, vbo, ebo)
}

pub fn update_uv_element_array(&mut vao: &mut u32, &mut vbo: &mut u32, &mut ebo: &mut u32, &mut uv_vbo: &mut u32, vertices: &[Coordinate3D; 4], uvs: Vec<Coordinate2D>, indicies: &Vec<u32>) {
    unsafe {
        gl::BindVertexArray(vao);
        
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        let (_, vertex_bytes, _) = vertices.align_to::<u8>();

        gl::BufferData(gl::ARRAY_BUFFER, vertex_bytes.len() as isize, std::ptr::null(), gl::DYNAMIC_DRAW);
        gl::BufferSubData(
            gl::ARRAY_BUFFER,
            0,
            vertex_bytes.len() as isize,
            vertex_bytes.as_ptr() as *const _,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<Coordinate3D>() as i32,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
        
        gl::BindBuffer(gl::ARRAY_BUFFER, uv_vbo);
        let (_, uv_bytes, _) = uvs.align_to::<u8>();

        gl::BufferSubData(
            gl::ARRAY_BUFFER,
            0,
            uv_bytes.len() as isize,
            uv_bytes.as_ptr() as *const _,
        );

        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<Coordinate2D>() as i32,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(1);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        let (_, index_bytes, _) = indicies.align_to::<u8>();

        gl::BufferSubData(
            gl::ELEMENT_ARRAY_BUFFER,
            0,
            index_bytes.len() as isize,
            index_bytes.as_ptr() as *const _,
        );
    }
}

pub fn independent_update_uv_element_array_2d(&mut vao: &mut u32, &mut vbo: &mut u32, &mut ebo: &mut u32, &mut uv_vbo: &mut u32, vertices: Vec<Coordinate2D>, uvs: Vec<Coordinate2D>, indicies: &Vec<u32>) {
    unsafe {
        gl::BindVertexArray(vao);
        
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        let (_, vertex_bytes, _) = vertices.align_to::<u8>();

        gl::BufferData(gl::ARRAY_BUFFER, vertex_bytes.len() as isize, std::ptr::null(), gl::DYNAMIC_DRAW);
        gl::BufferSubData(
            gl::ARRAY_BUFFER,
            0,
            vertex_bytes.len() as isize,
            vertex_bytes.as_ptr() as *const _,
        );

        gl::VertexAttribPointer(
            0,
            2,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<Coordinate2D>() as i32,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
        
        gl::BindBuffer(gl::ARRAY_BUFFER, uv_vbo);
        let (_, uv_bytes, _) = uvs.align_to::<u8>();

        gl::BufferSubData(
            gl::ARRAY_BUFFER,
            0,
            uv_bytes.len() as isize,
            uv_bytes.as_ptr() as *const _,
        );

        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<Coordinate2D>() as i32,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(1);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        let (_, index_bytes, _) = indicies.align_to::<u8>();

        gl::BufferSubData(
            gl::ELEMENT_ARRAY_BUFFER,
            0,
            index_bytes.len() as isize,
            index_bytes.as_ptr() as *const _,
        );
    }
}

pub fn independent_update_uv_element_array_3d(&mut vao: &mut u32, &mut vbo: &mut u32, &mut ebo: &mut u32, &mut uv_vbo: &mut u32, vertices: Vec<Coordinate3D>, uvs: Vec<Coordinate2D>, indicies: &Vec<u32>) {
    unsafe {
        gl::BindVertexArray(vao);
        
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        let (_, vertex_bytes, _) = vertices.align_to::<u8>();

        gl::BufferData(gl::ARRAY_BUFFER, vertex_bytes.len() as isize, std::ptr::null(), gl::DYNAMIC_DRAW);
        gl::BufferSubData(
            gl::ARRAY_BUFFER,
            0,
            vertex_bytes.len() as isize,
            vertex_bytes.as_ptr() as *const _,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<Coordinate3D>() as i32,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
        
        gl::BindBuffer(gl::ARRAY_BUFFER, uv_vbo);
        let (_, uv_bytes, _) = uvs.align_to::<u8>();

        gl::BufferSubData(
            gl::ARRAY_BUFFER,
            0,
            uv_bytes.len() as isize,
            uv_bytes.as_ptr() as *const _,
        );

        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<Coordinate2D>() as i32,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(1);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        let (_, index_bytes, _) = indicies.align_to::<u8>();

        gl::BufferSubData(
            gl::ELEMENT_ARRAY_BUFFER,
            0,
            index_bytes.len() as isize,
            index_bytes.as_ptr() as *const _,
        );
    }
}

pub fn update_element_array(&mut vao: &mut u32, &mut vbo: &mut u32, &mut ebo: &mut u32, vertices: Vec<Coordinate2D>, indicies: Vec<u32>) {
    unsafe {
        gl::BindVertexArray(vao);
        
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        let (_, vertex_bytes, _) = vertices.align_to::<u8>();
        gl::BufferData(gl::ARRAY_BUFFER, vertex_bytes.len() as isize, vertex_bytes.as_ptr() as *const _, gl::DYNAMIC_DRAW);
        
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        let (_, indices_bytes, _) = indicies.align_to::<u8>();
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, indices_bytes.len() as isize, indices_bytes.as_ptr() as *const _, gl::DYNAMIC_DRAW);
    }
}

pub fn start_test_element_array() -> (u32, u32, u32, u32) {
    let mut test_vao = 0;
    let mut test_vbo = 0;
    let mut test_ebo = 0;
    let mut color_vbo = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut test_vao);
        gl::GenBuffers(1, &mut test_vbo);
        gl::GenBuffers(1, &mut test_ebo);
        gl::GenBuffers(1, &mut color_vbo);
        
        gl::BindVertexArray(test_vao);
        
        gl::BindBuffer(gl::ARRAY_BUFFER, test_vbo);
        gl::BufferData(gl::ARRAY_BUFFER, 0, std::ptr::null(),  gl::DYNAMIC_DRAW);
        gl::VertexAttribPointer(
            0,
            2,
            gl::FLOAT,
            gl::FALSE,
            2 * std::mem::size_of::<f32>() as i32, std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
        
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, test_ebo); 
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, 0, std::ptr::null(), gl::DYNAMIC_DRAW);
        
        gl::BindBuffer(gl::ARRAY_BUFFER, color_vbo);
        gl::BufferData(gl::ARRAY_BUFFER, 0, std::ptr::null(), gl::DYNAMIC_DRAW);
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE,
            (3 * std::mem::size_of::<f32>()) as i32, std::ptr::null());
        gl::EnableVertexAttribArray(1);
    }
    (test_vao, test_vbo, test_ebo, color_vbo)
}

pub fn start_uv_3d_elemnt_array(position_lenght: i32, uv_lenght: i32) -> (u32, u32, u32, u32) {
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
            position_lenght,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<crate::math::Coordinate3D>() as i32,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
        
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo); 
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, 0, std::ptr::null(), gl::DYNAMIC_DRAW);
        
        gl::GenBuffers(1, &mut uv_vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, uv_vbo);
        gl::BufferData(gl::ARRAY_BUFFER, 0, std::ptr::null(), gl::DYNAMIC_DRAW);
        gl::VertexAttribPointer(1, uv_lenght, gl::FLOAT, gl::FALSE, std::mem::size_of::<crate::math::Coordinate2D>() as i32, std::ptr::null());
        gl::EnableVertexAttribArray(1);
    }
    (vao, vbo, ebo, uv_vbo)
}

pub fn update_uv_3d_element_array(&mut vao: &mut u32, &mut vbo: &mut u32, &mut ebo: &mut u32, &mut uv_vbo: &mut u32, vertices: Vec<Coordinate3D>, uvs: Vec<Coordinate2D>, indicies: &Vec<u32>) {
    unsafe {
        gl::BindVertexArray(vao);
        
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        let (_, vertex_bytes, _) = vertices.align_to::<u8>();
        gl::BufferData(gl::ARRAY_BUFFER, vertex_bytes.len() as isize, vertex_bytes.as_ptr() as *const _, gl::DYNAMIC_DRAW);
        gl::EnableVertexAttribArray(0);

        gl::BindBuffer(gl::ARRAY_BUFFER, uv_vbo);
        let (_, uv_bytes, _) = uvs.align_to::<u8>();
        gl::BufferData(gl::ARRAY_BUFFER, uv_bytes.len() as isize, uv_bytes.as_ptr() as *const _, gl::DYNAMIC_DRAW);
        gl::EnableVertexAttribArray(1);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        let (_, indices_bytes, _) = indicies.align_to::<u8>();
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, indices_bytes.len() as isize, indices_bytes.as_ptr() as *const _, gl::DYNAMIC_DRAW);
    }
}

pub struct Shader {
    program_id: u32,
}

impl Shader {
    //! # Shader
    //! 
    //! A set of instructions to be read throught the GPU.
    //! Vertex shaders resemble the corners, and fragment shaders resemble each pixel.
    
    /// Creates a new shader
    /// 
    /// # Examples
    /// ```ignore
    /// let mut shader = Shader::new("vertex.vert, fragment.frag");
    /// ```
    pub fn new(vert_shader_path: &str, frag_shader_path: &str) -> Shader {
        let vert_shader_file = std::fs::read_to_string(format!("{}", vert_shader_path)).expect("failed loading the vertex file: ");
        let frag_shader_file = std::fs::read_to_string(format!("{}", frag_shader_path)).expect("failed loading the fragment file: ");
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

    /// Binds the shader.
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

    /// Gets the location of an attribute.
    pub fn get_attrib_location(&self, attrib: &str) -> Result<i32, ()> {
        unsafe {
            let attrib = CString::new(attrib).unwrap();
            Ok(gl::GetAttribLocation(self.program_id, attrib.as_ptr()) as i32)
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

pub enum CatEngineShader {
    /// Helps set up shaders. On this version, this enum doesn't do anything, and is always set to TextureShader. I plan on getting this to work.
    /// You can set up your custom one, values aPOS and aUV, both vec2, are already set up. Plans on letting addition of many other input values.
    Shader(Shader),
    /// A simple, Texture shader that blits in a texture.
    TextureShader,
    /// A shader used for 3D.
    ThirdDimensionShader,
    /// A basic Shader used for tests that sets up a Triangle with colors green, red and blue.
    TestShader,
}

pub struct Renderer {
    pub projection: glam::Mat4,
    pub orthographic_projection: glam::Mat4,
    texture_indexes: Vec<u32>,
    texture_shader: Shader,
    texture_vao: u32,
    texture_vbo: u32,
    texture_ebo: u32,
    texture_uv_vbo: u32,
    sprite_vao: u32,
    sprite_vbo: u32,
    sprite_ebo: u32,
    sprite_uv_vbo: u32,
    triangle_shader: Shader,
    triangle_vao: u32,
    triangle_vbo: u32,
    triangle_ebo: u32,
    triangle_uv_vbo: u32,
    test_shader: Shader,
    test_vao: u32,
    test_vbo: u32,
    test_ebo: u32,
    test_color_vbo: u32,
    triangle3d_shader: Shader,
    triangle3d_vao: u32,
    triangle3d_vbo: u32,
    triangle3d_ebo: u32,
    triangle3d_uv_vbo: u32,
    screen_width: u32,
    screen_height: u32,
    pub fov: f32,
    pub near_plane: f32,
    pub far_plane: f32,
}

impl Renderer {
    //! # Renderer
    //! 
    //! Used for Rendering.

    pub fn new(screen_width: u32, screen_height: u32, fov: f32, near_plane: f32, far_plane: f32) -> Self {
        let texture_shader = Shader::new(format!("{}/texture.vert", env!("CARGO_MANIFEST_DIR")).as_str(), format!("{}/texture.frag", env!("CARGO_MANIFEST_DIR")).as_str());
        let triangle_shader = Shader::new(format!("{}/triangle.vert", env!("CARGO_MANIFEST_DIR")).as_str(), format!("{}/triangle.frag", env!("CARGO_MANIFEST_DIR")).as_str());
        let triangle3d_shader = Shader::new(format!("{}/triangle3d.vert", env!("CARGO_MANIFEST_DIR")).as_str(), format!("{}/triangle3d.frag", env!("CARGO_MANIFEST_DIR")).as_str());
        let test_shader = Shader::new(format!("{}/opengltest.vert", env!("CARGO_MANIFEST_DIR")).as_str(), format!("{}/opengltest.frag", env!("CARGO_MANIFEST_DIR")).as_str());
        let (texture_vao, texture_vbo, texture_ebo, texture_uv_vbo) = start_uv_elemnt_array();
        let (sprite_vao, sprite_vbo, sprite_ebo, sprite_uv_vbo) = start_uv_elemnt_array();
        let (triangle_vao, triangle_vbo, triangle_ebo, triangle_uv_vbo) = start_uv_elemnt_array();
        let (test_vao, test_vbo, test_ebo, test_color_vbo) = start_test_element_array();
        let (triangle3d_vao, triangle3d_vbo, triangle3d_ebo, triangle3d_uv_vbo) = start_uv_3d_elemnt_array(3, 2);
        let projection = glam::Mat4::perspective_rh_gl(fov.to_radians(), screen_width as f32 / screen_height as f32, near_plane, far_plane);
        let orthographic_projection = glam::Mat4::orthographic_rh_gl(0.0, screen_width as f32, screen_height as f32, 0.0, -10000.0, 10000.0);

        Self { projection, orthographic_projection, texture_indexes: vec![
            0, 1, 2,
            2, 3, 0
        ], 
        texture_shader, texture_vao, texture_vbo, texture_ebo, texture_uv_vbo, sprite_vao, sprite_vbo, sprite_ebo, sprite_uv_vbo, triangle_shader, triangle_vao, triangle_vbo, triangle_ebo, triangle_uv_vbo, triangle3d_shader, triangle3d_vao, triangle3d_vbo, triangle3d_ebo, triangle3d_uv_vbo, test_shader, test_vao, test_vbo, test_ebo, test_color_vbo, screen_width, screen_height, fov, near_plane, far_plane }
    }

    pub fn set_projection(&mut self, projection: Mat4, fov: f32, near_plane: f32, far_plane: f32) {
        self.projection = projection;
        self.fov = fov;
        self.near_plane = near_plane;
        self.far_plane = far_plane;
    }

    pub fn true_set_projection(&mut self, projection: Mat4) {
        self.projection = projection;
    }

    /// Sets up size independentally. You should always use CatEngine's implementation for that.
    pub fn set_size(&mut self, width: u32, height: u32, stretch_mode: &String) {
        self.screen_width = width;
        self.screen_height = height;

        if stretch_mode == "normal" {
            self.orthographic_projection = glam::Mat4::orthographic_rh_gl(0.0, width as f32, height as f32, 0.0, -10000.0, 10000.0);
        }
    }

    /// Blits a surface onto the screen. For proper Z control, use sprites.
    /// 
    /// # Examples
    /// ```ignore
    /// let catengine = CatEngine::new("Surface", 800, 800, vec![]);
    /// let surface = Surface::from_texture("mel.png");
    /// while catengine.running {
    ///     catengine.renderer.blit(surface, 20.0, 20.0, 100.0, 100.0);
    ///     // Most values here are arbitrary.
    /// }
    /// ```
    pub fn blit(&mut self, surface: & Surface, pos_x: f32, pos_y: f32) {
        let model = Mat4::from_translation(glam::vec3(pos_x, pos_y, surface.vertices[0].2));
        
        update_uv_element_array(&mut self.texture_vao, &mut self.texture_vbo, &mut self.texture_ebo, &mut self.texture_uv_vbo, &surface.vertices, surface.corners.to_vec(), &self.texture_indexes);
        
        unsafe {
            self.texture_shader.bind();
            self.texture_shader.set_int("tex", 0);
            self.texture_shader.set_mat4("model", model.to_cols_array());
            self.texture_shader.set_mat4("projection", self.orthographic_projection.to_cols_array());
            gl::ActiveTexture(gl::TEXTURE0);
            surface.bind();
            gl::BindVertexArray(self.texture_vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }
    }

    /// Draws a Tile from a TileSet.
    /// 
    /// # Examples
    /// ```ignore
    /// let catengine = CatEngine::new("TileSet", 800, 800, vec![]);
    /// let atlas = TileSet::from_texture("atlas.png");
    /// let tile_1 = atlas.simple_append_tile(0, 0, 16, 16)
    /// while catengine.running {
    ///     catengine.renderer.draw_tileset(tile_1, &mut atlas, 20.0, 20.0);
    ///     // Most values here are arbitrary.
    /// }
    /// ```
    pub fn draw_tileset(&mut self, tile: u32, tile_set: &mut surface::TileSet, x: f32, y: f32) {
        let used_tile = &tile_set.tile_list[tile as usize];
        let vertices = &used_tile.vertices;

        let model = Mat4::from_translation(glam::vec3(x, y, used_tile.vertices[0].2));
        
        update_uv_element_array(&mut self.texture_vao, &mut self.texture_vbo, &mut self.texture_ebo, &mut self.texture_uv_vbo, vertices, used_tile.corners.to_vec(), &self.texture_indexes);
        
        unsafe {
            self.texture_shader.bind();
            self.texture_shader.set_int("tex", 0);
            self.texture_shader.set_mat4("model", model.to_cols_array());
            self.texture_shader.set_mat4("projection", self.orthographic_projection.to_cols_array());
            gl::ActiveTexture(gl::TEXTURE0);
            tile_set.surface.bind();
            gl::BindVertexArray(self.texture_vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }
    }

    pub fn draw_tile_list(&mut self, tile_set: std::rc::Rc<std::cell::RefCell<surface::TileSet>>, tiles: Vec<(u32, f32, f32, bool, f32)>, offset_x: f32, offset_y: f32, screen_width: u32, screen_height: u32) {
        let model = Mat4::from_translation(glam::vec3(offset_x, offset_y, 0.0));
        let borrowed_tileset = tile_set.borrow();

        let mut vertices = vec![];
        #[rustfmt::skip]
        let mut indicies: Vec<u32> = vec![];
        let mut uvs: Vec<Coordinate2D> = vec![];
        let mut current_sprite = 0;
        
        for tile in &tiles {
            let x = borrowed_tileset.tile_list[tile.0.to_owned() as usize].vertices[0].0 + tile.1;
            let y = borrowed_tileset.tile_list[tile.0.to_owned() as usize].vertices[0].1 + tile.2;
            let width = borrowed_tileset.tile_list[tile.0.to_owned() as usize].vertices[1].0 + tile.1;
            let height = borrowed_tileset.tile_list[tile.0.to_owned() as usize].vertices[2].1 + tile.2;
            let width_2 = borrowed_tileset.tile_list[tile.0.to_owned() as usize].vertices[1].0 + tile.1 / 2.0;
            let height_2 = borrowed_tileset.tile_list[tile.0.to_owned() as usize].vertices[2].1 + tile.2 / 2.0;
            if x + width_2 * 2.0 - offset_x > 0.0 
            && x - offset_x < self.screen_width as f32 
            && y + height_2 * 2.0 - offset_y > 0.0 
            && y - offset_y < self.screen_height as f32
            {
                uvs.push(borrowed_tileset.tile_list[tile.0.to_owned() as usize].corners[0].clone());
                uvs.push(borrowed_tileset.tile_list[tile.0.to_owned() as usize].corners[1].clone());
                uvs.push(borrowed_tileset.tile_list[tile.0.to_owned() as usize].corners[2].clone());
                uvs.push(borrowed_tileset.tile_list[tile.0.to_owned() as usize].corners[3].clone());
                if tile.3 {
                    vertices.push(Coordinate3D(borrowed_tileset.tile_list[tile.0.to_owned() as usize].vertices[0].0 + tile.1, borrowed_tileset.tile_list[tile.0.to_owned() as usize].vertices[0].1 + tile.2, borrowed_tileset.tile_list[tile.0 as usize].vertices[2].1 + tile.2 + tile.4));
                    vertices.push(Coordinate3D(borrowed_tileset.tile_list[tile.0.to_owned() as usize].vertices[1].0 + tile.1, borrowed_tileset.tile_list[tile.0.to_owned() as usize].vertices[1].1 + tile.2, borrowed_tileset.tile_list[tile.0 as usize].vertices[2].1 + tile.2 + tile.4));
                    vertices.push(Coordinate3D(borrowed_tileset.tile_list[tile.0.to_owned() as usize].vertices[2].0 + tile.1, borrowed_tileset.tile_list[tile.0.to_owned() as usize].vertices[2].1 + tile.2, borrowed_tileset.tile_list[tile.0 as usize].vertices[2].1 + tile.2 + tile.4));
                    vertices.push(Coordinate3D(borrowed_tileset.tile_list[tile.0.to_owned() as usize].vertices[3].0 + tile.1, borrowed_tileset.tile_list[tile.0.to_owned() as usize].vertices[3].1 + tile.2, borrowed_tileset.tile_list[tile.0 as usize].vertices[2].1 + tile.2 + tile.4));
                } else {
                    vertices.push(Coordinate3D(borrowed_tileset.tile_list[tile.0.to_owned() as usize].vertices[0].0 + tile.1, borrowed_tileset.tile_list[tile.0.to_owned() as usize].vertices[0].1 + tile.2, borrowed_tileset.tile_list[tile.0.to_owned() as usize].vertices[0].2));
                    vertices.push(Coordinate3D(borrowed_tileset.tile_list[tile.0.to_owned() as usize].vertices[1].0 + tile.1, borrowed_tileset.tile_list[tile.0.to_owned() as usize].vertices[1].1 + tile.2, borrowed_tileset.tile_list[tile.0.to_owned() as usize].vertices[1].2));
                    vertices.push(Coordinate3D(borrowed_tileset.tile_list[tile.0.to_owned() as usize].vertices[2].0 + tile.1, borrowed_tileset.tile_list[tile.0.to_owned() as usize].vertices[2].1 + tile.2, borrowed_tileset.tile_list[tile.0.to_owned() as usize].vertices[2].2));
                    vertices.push(Coordinate3D(borrowed_tileset.tile_list[tile.0.to_owned() as usize].vertices[3].0 + tile.1, borrowed_tileset.tile_list[tile.0.to_owned() as usize].vertices[3].1 + tile.2, borrowed_tileset.tile_list[tile.0.to_owned() as usize].vertices[3].2));
                }
                indicies.push(current_sprite);
                indicies.push(current_sprite + 1);
                indicies.push(current_sprite + 2);
                indicies.push(current_sprite);
                indicies.push(current_sprite + 2);
                indicies.push(current_sprite + 3);
                current_sprite += 4;
            }
        }

        independent_update_uv_element_array_3d(&mut self.texture_vao, &mut self.texture_vbo, &mut self.texture_ebo, &mut self.texture_uv_vbo, vertices, uvs, &indicies);
        
        unsafe {
            self.texture_shader.bind();
            self.texture_shader.set_int("tex", 0);
            self.texture_shader.set_mat4("model", model.to_cols_array());
            self.texture_shader.set_mat4("projection", self.orthographic_projection.to_cols_array());
            gl::ActiveTexture(gl::TEXTURE0);
            borrowed_tileset.surface.bind();
            gl::BindVertexArray(self.texture_vao);
            gl::DrawElements(gl::TRIANGLES, indicies.len() as i32, gl::UNSIGNED_INT, std::ptr::null());
        }
    }

    /// Draws a triangle.
    pub fn draw_triangle(&mut self, p1: Coordinate2D, p2: Coordinate2D, p3: Coordinate2D,  surface: &mut Surface, uvs: Vec<Coordinate2D>) {
        let vertices = vec![p1, p2, p3];

        let indicies: Vec<u32> = vec![
            0, 1, 2
        ];

        let model = Mat4::IDENTITY;

        independent_update_uv_element_array_2d(&mut self.triangle_vao, &mut self.triangle_vbo, &mut self.triangle_ebo, &mut self.triangle_uv_vbo, vertices, uvs, &self.texture_indexes);
        
        unsafe {
            self.triangle_shader.bind();
            self.triangle_shader.set_int("tex", 0);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindVertexArray(self.triangle_vao);
            surface.bind();
            gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, std::ptr::null());
        }
    }

    /// Draws a "opengl triangle", with every vertex having their own color. Mostly used for testing.
    pub fn draw_test_opengl_triangle(&mut self, p1: Coordinate2D, p2: Coordinate2D, p3: Coordinate2D) {
        let mut vertices: Vec<Coordinate2D> = Vec::new();
        vertices.push(p1.return_into_gl_coordinates(self.screen_width, self.screen_height));
        vertices.push(p2.return_into_gl_coordinates(self.screen_width, self.screen_height));
        vertices.push(p3.return_into_gl_coordinates(self.screen_width, self.screen_height));

        #[rustfmt::skip]
        let indicies: Vec<u32> = vec![
            0, 1, 2,
        ];

        let data: Vec<f32> = vec![
            // position      // color
            vertices[0].0, vertices[0].1,  1.0, 0.0, 0.0,
            vertices[1].0, vertices[1].1,  0.0, 1.0, 0.0,
            vertices[2].0, vertices[2].1,  0.0, 0.0, 1.0,
        ];
        
        unsafe {

            let stride = (5 * std::mem::size_of::<f32>()) as i32;

            gl::BindVertexArray(self.test_vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.test_vbo);
            gl::BufferData(gl::ARRAY_BUFFER, (data.len() * std::mem::size_of::<f32>()) as isize, data.as_ptr() as *const _, gl::DYNAMIC_DRAW);

            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
            gl::EnableVertexAttribArray(0);

            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (2 * std::mem::size_of::<f32>()) as *const _);
            gl::EnableVertexAttribArray(1);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.test_ebo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (indicies.len() * std::mem::size_of::<u32>()) as isize, indicies.as_ptr() as *const _, gl::DYNAMIC_DRAW);
        }

        
        unsafe {
            self.test_shader.bind();
            gl::BindVertexArray(self.test_vao);
            gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, std::ptr::null());
        }
    }

    /// Draws a triangle in 3D.
    pub fn draw_3d_triangle(&mut self, p1: Coordinate3D, p2: Coordinate3D, p3: Coordinate3D, surface: &mut Surface, uvs: Vec<Coordinate2D>, view: Mat4) {
        let model = Mat4::from_scale_rotation_translation(
        glam::Vec3::new(5.0, 5.0, 5.0), // scale up to match cubes
        glam::Quat::IDENTITY,
        glam::Vec3::new(0.0, 0.0, -5.0),
        );
        let mut vertices: Vec<Coordinate3D> = Vec::new();
        vertices.push(p1);
        vertices.push(p2);
        vertices.push(p3);
        #[rustfmt::skip]
        let indicies: Vec<u32> = vec![
            0, 1, 2,
        ];

        self.triangle3d_shader.bind();
        self.triangle3d_shader.set_mat4("model", model.to_cols_array());
        self.triangle3d_shader.set_mat4("view", view.to_cols_array());
        self.triangle3d_shader.set_mat4("projection", self.projection.to_cols_array());



        update_uv_3d_element_array(&mut self.triangle3d_vao, &mut self.triangle3d_vbo, &mut self.triangle3d_ebo, &mut self.triangle3d_uv_vbo, vertices, uvs, &indicies);
        
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            self.triangle3d_shader.bind();
            self.triangle3d_shader.set_int("tex", 0);
            gl::BindVertexArray(self.triangle3d_vao);
            surface.bind();
            gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, std::ptr::null());
        }
    }

    /// Draws a mesh.
    pub fn draw_mesh(&mut self, pos_x: f32, pos_y: f32, pos_z: f32, mesh: &mut Mesh, view: &Mat4) {
        let model = Mat4::from_scale_rotation_translation(
            glam::Vec3::new(5.0, 5.0, 5.0),
            glam::Quat::IDENTITY,
            glam::Vec3::new(0.0 + pos_x, 0.0 + pos_y, 0.0 + pos_z),
        );

        self.triangle3d_shader.bind();
        self.triangle3d_shader.set_mat4("model", model.to_cols_array());
        self.triangle3d_shader.set_mat4("view", view.to_cols_array());
        self.triangle3d_shader.set_mat4("projection", self.projection.to_cols_array());


        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            mesh.texture.bind();
            self.triangle3d_shader.set_int("tex", 0);

            gl::BindVertexArray(mesh.vao);
            gl::DrawElements(gl::TRIANGLES, mesh.indicies.len() as i32, gl::UNSIGNED_INT, std::ptr::null());
            gl::BindVertexArray(0);
        }
    }

    /// Draws a "complex" sprite list. It's faster than a regular sprite list, but it is simpler and has better support for now.
    pub fn draw_complex_sprite_list(&mut self, sprite_list: &mut ComplexSpriteList, offset_x: f32, offset_y: f32) {
        let mut vertices = vec![];
        #[rustfmt::skip]
        let mut indicies: Vec<u32> = vec![];
        let mut uvs: Vec<Coordinate2D> = vec![];
        
        for tile in &sprite_list.sprite_list {
            uvs.push(sprite_list.tile_set.tile_list[tile.1.to_owned() as usize].corners[0].clone());
            uvs.push(sprite_list.tile_set.tile_list[tile.1.to_owned() as usize].corners[1].clone());
            uvs.push(sprite_list.tile_set.tile_list[tile.1.to_owned() as usize].corners[2].clone());
            uvs.push(sprite_list.tile_set.tile_list[tile.1.to_owned() as usize].corners[3].clone());
        }
         
        let model = Mat4::from_translation(glam::vec3(offset_x, offset_y, 0.0));
        let mut current_sprite: u32 = 0;
        
        for sprite in &sprite_list.sprite_list {
            match &sprite.0 {
                sprite::ComplexSprite::Surface(x, y, width, height, surface, _shader ) => {
                    vertices.push(Coordinate2D(x.to_owned(), y.to_owned()));
                    vertices.push(Coordinate2D(x.to_owned() + width.to_owned(), y.to_owned()));
                    vertices.push(Coordinate2D(x.to_owned() + width.to_owned(), y.to_owned() + height.to_owned()));
                    vertices.push(Coordinate2D(x.to_owned(), y.to_owned() + height.to_owned()));
                }
                sprite::ComplexSprite::Tile(x, y, width, height, tile_set, tile, _shader) => {
                    vertices.push(Coordinate2D(x.to_owned(), y.to_owned()));
                    vertices.push(Coordinate2D(x.to_owned() + width.to_owned(), y.to_owned()));
                    vertices.push(Coordinate2D(x.to_owned() + width.to_owned(), y.to_owned() + height.to_owned()));
                    vertices.push(Coordinate2D(x.to_owned(), y.to_owned() + height.to_owned()));
                }
            }
            indicies.push(current_sprite);
            indicies.push(current_sprite + 1);
            indicies.push(current_sprite + 2);
            indicies.push(current_sprite);
            indicies.push(current_sprite + 2);
            indicies.push(current_sprite + 3);
            current_sprite += 4;
        }

        independent_update_uv_element_array_2d(&mut self.sprite_vao, &mut self.sprite_vbo, &mut self.sprite_ebo, &mut self.sprite_uv_vbo, vertices, uvs, &self.texture_indexes);

        unsafe {
            self.texture_shader.bind();
            self.texture_shader.set_int("tex", 0);
            self.texture_shader.set_mat4("model", model.to_cols_array());
            self.texture_shader.set_mat4("projection", self.orthographic_projection.to_cols_array());
            gl::ActiveTexture(gl::TEXTURE0);
            sprite_list.tile_set.surface.bind();
            gl::BindVertexArray(self.sprite_vao);
            gl::DrawElements(gl::TRIANGLES, sprite_list.sprite_list.len() as i32 * 6, gl::UNSIGNED_INT, std::ptr::null());
        }
    }

    /// Draws a SpriteList.
    pub fn draw_sprite_list(&mut self, sprite_list: &mut SpriteList, offset_x: f32, offset_y: f32) {
        use std::collections::HashMap;
        use std::rc::Rc;
        use std::cell::RefCell;

        let mut tile_batches: HashMap<
        *const std::cell::RefCell<surface::TileSet>,
        (Rc<RefCell<surface::TileSet>>, Vec<(u32, f32, f32, bool, f32)>)
        > = HashMap::new();
        for sprite in &sprite_list.sprite_list {
            let x = sprite.get_x();
            let y = sprite.get_y();
            let width = sprite.get_width();
            let height = sprite.get_height();
            
            // CULLING (same as yours)
            if !(x + width * 2.0 - offset_x > 0.0 
                && x - offset_x < self.screen_width as f32 
                && (y + height) * 2.0 - offset_y > 0.0 
                && y - offset_y < self.screen_height as f32)
                || !sprite.is_not_batch() {
                    continue;
                }
                
                match sprite {
                    // Surfaces → draw immediately
                    Sprite::Surface(x, y, z, surface, _, _) => {
                        surface.borrow_mut().set_z(*z);
                        self.blit(&surface.borrow(), *x - offset_x, *y - offset_y);
                    }
                    
                    // Tiles → batch them
                    Sprite::Tile(x, y, z, tile_set, tile, _, ysort) => {
                        let key = std::rc::Rc::as_ptr(tile_set);
                        
                        let entry = tile_batches.entry(key).or_insert((
                            tile_set.clone(),
                            Vec::new()
                        ));
                        
                        entry.1.push((
                            *tile,
                            *x - offset_x,
                            *y - offset_y,
                            *ysort,
                            *z
                        ));
                    }
                    
                    // already batched
                    Sprite::Batch(tile_set, tile_list) => {
                        self.draw_tile_list(tile_set.clone(), tile_list.clone(), offset_x, offset_y, self.screen_width, self.screen_height);
                    }
                }
            }
            
            // DRAW ALL BATCHES
            for (_, (tile_set, tiles)) in tile_batches {
                self.draw_tile_list(tile_set, tiles, 0.0, 0.0, self.screen_width, self.screen_height);
            }
    }

    /// Draws a font.
    pub fn draw_font(&mut self, font: &Font, text: &str, x: f32, y: f32, size: f32, spacement: u32) {
        let mut vertices: Vec<Coordinate2D> = vec![];
        #[rustfmt::skip]
        let mut indicies: Vec<u32> = vec![];
        
        
        let mut current_sprite: u32 = 0;
        let mut uvs: Vec<Coordinate2D> = vec![];
        let mut cursor_x = 0;

        for character in text.chars() {
            let ch = character.to_string();
            let glyph = font.return_character_from_string(character).unwrap();

            let uv = font.uvs.get(&character).unwrap();

            uvs.push(Coordinate2D(uv[0].0, uv[0].1));
            uvs.push(Coordinate2D(uv[1].0, uv[1].1));
            uvs.push(Coordinate2D(uv[2].0, uv[2].1));
            uvs.push(Coordinate2D(uv[3].0, uv[3].1));

            let y = 0.0;
            let w = glyph.width as f32;
            let h = glyph.height as f32;
                        
            vertices.push(Coordinate2D(cursor_x as f32, 0.0));
            vertices.push(Coordinate2D(cursor_x as f32 + w * size, 0.0));
            vertices.push(Coordinate2D(cursor_x as f32 + w * size, h * size));
            vertices.push(Coordinate2D(cursor_x as f32, h * size));
            cursor_x += glyph.width * size as u32 + spacement;

            indicies.push(current_sprite);
            indicies.push(current_sprite + 1);
            indicies.push(current_sprite + 2);
            indicies.push(current_sprite);
            indicies.push(current_sprite + 2);
            indicies.push(current_sprite + 3);

            current_sprite += 4;
        }
         
        let model = Mat4::from_translation(glam::vec3(x, y, 0.0));
        let lenght = indicies.len() as i32;

        independent_update_uv_element_array_2d(&mut self.texture_vao, &mut self.texture_vbo, &mut self.texture_ebo, &mut self.texture_uv_vbo, vertices.to_vec(), uvs, &indicies);
        
        unsafe {
            self.texture_shader.bind();
            self.texture_shader.set_int("tex", 0);
            self.texture_shader.set_mat4("model", model.to_cols_array());
            self.texture_shader.set_mat4("projection", self.orthographic_projection.to_cols_array());
            gl::ActiveTexture(gl::TEXTURE0);
            font.surface.bind();
            gl::BindVertexArray(self.texture_vao);
            gl::DrawElements(gl::TRIANGLES, lenght, gl::UNSIGNED_INT, std::ptr::null());
        }
    }

    pub fn draw_line(&mut self, p1: Coordinate2D, p2: Coordinate2D, color: &Color, width: f32) {
        let p1 = p1.return_into_gl_coordinates(self.screen_width, self.screen_height);
        let p2 = p2.return_into_gl_coordinates(self.screen_width, self.screen_height);

        let data: Vec<f32> = vec![
            // position      // color
            p1.0, p1.1,     color.r as f32 / 255.0, color.g as f32 / 255.0, color.b as f32 / 255.0,
            p2.0, p2.1,     color.r as f32 / 255.0, color.g as f32 / 255.0, color.b as f32 / 255.0,
        ];

        unsafe {
            gl::LineWidth(width);

            let stride = (5 * std::mem::size_of::<f32>()) as i32;

            gl::BindVertexArray(self.test_vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.test_vbo);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                (data.len() * std::mem::size_of::<f32>()) as isize,
                data.as_ptr() as *const _,
                gl::DYNAMIC_DRAW
            );

            // position
            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
            gl::EnableVertexAttribArray(0);

            // color
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (2 * std::mem::size_of::<f32>()) as *const _
            );
            gl::EnableVertexAttribArray(1);

            self.test_shader.bind();
            gl::BindVertexArray(self.test_vao);

            gl::DrawArrays(gl::LINES, 0, 2);
        }
    }

    pub fn draw_rect(&mut self, rect: &Rect, color: &Color, width: f32) {
        self.draw_line(Coordinate2D(rect.x, rect.y), Coordinate2D(rect.width + rect.x, rect.y), &color, width);
        self.draw_line(Coordinate2D(rect.x, rect.y), Coordinate2D(rect.x, rect.height + rect.y), &color, width);
        self.draw_line(Coordinate2D(rect.width + rect.x, rect.height + rect.y), Coordinate2D(rect.width + rect.x, rect.y), &color, width);
        self.draw_line(Coordinate2D(rect.width + rect.x, rect.height + rect.y), Coordinate2D(rect.x, rect.height + rect.y), &color, width);
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

