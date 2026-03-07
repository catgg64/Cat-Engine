use std::ffi::CString;
use crate::video::surface::Surface;
use crate::math::{ Coordinate2D };

pub mod surface;

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
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.program_id);
        }
    }
}

pub struct Renderer {
    texture_shader: Shader,
}

impl Renderer {
    pub fn new() -> Self {
        let texture_shader = Shader::new("texture.vert", "texture.frag");
        Self { texture_shader }
    }

    pub fn blit(&self, surface: &mut Surface, pos_x: i32, pos_y: i32) {
        #[rustfmt::skip]
        const VERTICES: [Coordinate2D; 4] = [
            Coordinate2D(-1.0, -1.0),
            Coordinate2D( 0.5, -1.0),
            Coordinate2D( 1.0,  0.3),
            Coordinate2D(-1.0,  0.7),
        ];

        #[rustfmt::skip]
        const INDICES: [u32; 6] = [
            0, 1, 2,
            2, 3, 0
        ];

        let mut vao = 0;
        let mut vbo = 0;
        let mut ebo = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);
            
            gl::BindVertexArray(vao);
            
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            let (_, vertex_bytes, _) = VERTICES.align_to::<u8>();
            gl::BufferData(gl::ARRAY_BUFFER, vertex_bytes.len() as isize, vertex_bytes.as_ptr() as *const _, gl::STATIC_DRAW);
            gl::VertexAttribPointer(
                0,
                2,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Coordinate2D>() as i32,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);
            
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo); 
            let (_, indicies_bytes, _) = INDICES.align_to::<u8>();
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, indicies_bytes.len() as isize, indicies_bytes.as_ptr() as *const _, gl::STATIC_DRAW);
            
            self.texture_shader.bind();
            self.texture_shader.set_int("tex", 0);
            gl::ActiveTexture(gl::TEXTURE0);
            surface.bind();
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }
    }
}