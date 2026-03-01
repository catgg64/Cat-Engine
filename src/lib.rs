use sdl2::{ render::TextureCreator, video::Window, video::WindowContext, *};
use input::*;
use sdl2::video::GLProfile;
use video::surface::Surface;
use video::graphics::Shader;
use glam::{ Mat4, Vec3 };

use crate::video::surface;
pub mod color;
pub mod input;
pub mod video;
pub mod shape;

pub struct CatEngine {
    pub _gl_context: sdl2::video::GLContext,
    event_pump: sdl2::EventPump,
    pub screen_rect: sdl2::rect::Rect,
    pub input: input::Input,
    pub running: bool,
    pub window: Window,
    pub renderer: Renderer,
    fov: i16,
}

impl CatEngine{
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self, String>{
        let context: Sdl = sdl2::init()?;
        let video_subsystem: VideoSubsystem = context.video()?;
        let window = video_subsystem
            .window(title, width, height)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;let event_pump: EventPump = context.event_pump()?;
        let _gl_context = window.gl_create_context()?;
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);
        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
        }
        let screen_rect = sdl2::rect::Rect::new(0, 0, width, height);
        let input: Input = input::Input::new(context);
        let mut running: bool = true;
        let mut renderer = Renderer::new(width as f32, height as f32);
        let fov = 300;
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
        }

        Ok(Self {
            _gl_context,
            event_pump,
            screen_rect,
            input,
            running,
            window,
            renderer,
            fov,
        })
    }
    
    

    pub fn update(&mut self) {
        //unsafe {
        //    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        //}
        self.running = self.input.update(&mut self.event_pump);
        self.window.gl_swap_window();
    }

    pub fn set_fov(&mut self, fov: i16) {
        self.fov = fov
    }    

    pub fn get_fov(&mut self) -> i16 {
        self.fov
    }

    pub fn get_camera_specs(&self, cam_x: f32, cam_y: f32, cam_z: f32, yaw: f32, pitch: f32) -> (Vec3, Vec3, Mat4){
        let camera_position = Vec3::new(cam_x, cam_y, cam_z);

        let front = Vec3::new(
            yaw.cos() * pitch.cos(),
            pitch.sin(),
            yaw.sin() * pitch.cos(),
        ).normalize();

        let view_matrix = Mat4::look_at_rh(
            camera_position,
            camera_position + front,
            Vec3::Y,
        );

        (camera_position, front, view_matrix)
    }

    pub fn clear_color(&self, color: color::Color) {
        unsafe {
            let (r, g, b) = color.return_rgb();
            gl::ClearColor(r as f32, g as f32, b as f32, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }
}

pub struct Renderer {
    quad_vao: u32,
    shader: Shader,
    projection: Mat4,
    line_vao: u32,
    line_vbo: u32,
    line_shader: Shader,
    cube_vao: u32,
    cube_vbo: u32,
    textures: Vec<u32>,
}
impl Renderer {
    pub fn new(screen_width: f32, screen_height: f32) -> Self {
        let mut line_vao = 0;
        let mut line_vbo = 0;
        let mut quad_vao = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut line_vao);
            gl::GenBuffers(1, &mut line_vbo);

            gl::BindVertexArray(line_vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, line_vbo);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                (6 * std::mem::size_of::<f32>()) as isize,
                std::ptr::null(),
                gl::DYNAMIC_DRAW,
            );

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                3 * std::mem::size_of::<f32>() as i32,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);
            let mut quad_vbo = 0;

            unsafe {
                gl::Enable(gl::DEPTH_TEST);
                // Quad vertices (2 triangles)
                // position        // texcoord
                let vertices: [f32; 30] = [
                    // first triangle
                    0.0, 0.0, 0.0,  0.0, 0.0,
                    1.0, 0.0, 0.0,  1.0, 0.0,
                    1.0, 1.0, 0.0,  1.0, 1.0,

                    // second triangle
                    0.0, 0.0, 0.0,  0.0, 0.0,
                    1.0, 1.0, 0.0,  1.0, 1.0,
                    0.0, 1.0, 0.0,  0.0, 1.0,
                ];

                gl::GenVertexArrays(1, &mut quad_vao);
                gl::GenBuffers(1, &mut quad_vbo);

                gl::BindVertexArray(quad_vao);
                gl::BindBuffer(gl::ARRAY_BUFFER, quad_vbo);

                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    (vertices.len() * std::mem::size_of::<f32>()) as isize,
                    vertices.as_ptr() as *const _,
                    gl::STATIC_DRAW,
                );

                let stride = 5 * std::mem::size_of::<f32>() as i32;

                // position attribute (location = 0)
                gl::VertexAttribPointer(
                    0,
                    3,
                    gl::FLOAT,
                    gl::FALSE,
                    stride,
                    std::ptr::null(),
                );
                gl::EnableVertexAttribArray(0);

                // texcoord attribute (location = 1)
                gl::VertexAttribPointer(
                    1,
                    2,
                    gl::FLOAT,
                    gl::FALSE,
                    stride,
                    (3 * std::mem::size_of::<f32>()) as *const _,
                );
                gl::EnableVertexAttribArray(1);
            }
        }

        let vertices: [f32; 180] = [
            // positions       // UVs
            -0.5,-0.5, 0.5, 0.0,0.0,
            0.5,-0.5, 0.5, 1.0,0.0,
            0.5, 0.5, 0.5, 1.0,1.0,
            0.5, 0.5, 0.5, 1.0,1.0,
            -0.5, 0.5, 0.5, 0.0,1.0,
            -0.5,-0.5, 0.5, 0.0,0.0,

            -0.5,-0.5,-0.5, 1.0,0.0,
            -0.5, 0.5,-0.5, 1.0,1.0,
            0.5, 0.5,-0.5, 0.0,1.0,
            0.5, 0.5,-0.5, 0.0,1.0,
            0.5,-0.5,-0.5, 0.0,0.0,
            -0.5,-0.5,-0.5, 1.0,0.0,

            -0.5, 0.5, 0.5, 1.0,0.0,
            -0.5, 0.5,-0.5, 1.0,1.0,
            -0.5,-0.5,-0.5, 0.0,1.0,
            -0.5,-0.5,-0.5, 0.0,1.0,
            -0.5,-0.5, 0.5, 0.0,0.0,
            -0.5, 0.5, 0.5, 1.0,0.0,

            0.5, 0.5, 0.5, 0.0,0.0,
            0.5,-0.5,-0.5, 1.0,1.0,
            0.5, 0.5,-0.5, 0.0,1.0,
            0.5,-0.5,-0.5, 1.0,1.0,
            0.5, 0.5, 0.5, 0.0,0.0,
            0.5,-0.5, 0.5, 1.0,0.0,

            -0.5, 0.5,-0.5, 0.0,1.0,
            -0.5, 0.5, 0.5, 0.0,0.0,
            0.5, 0.5, 0.5, 1.0,0.0,
            0.5, 0.5, 0.5, 1.0,0.0,
            0.5, 0.5,-0.5, 1.0,1.0,
            -0.5, 0.5,-0.5, 0.0,1.0,

            -0.5,-0.5,-0.5, 1.0,1.0,
            0.5,-0.5, 0.5, 0.0,0.0,
            -0.5,-0.5, 0.5, 1.0,0.0,
            0.5,-0.5, 0.5, 0.0,0.0,
            -0.5,-0.5,-0.5, 1.0,1.0,
            0.5,-0.5,-0.5, 0.0,1.0,
        ];

        let mut cube_vao = 0;
        let mut cube_vbo = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut cube_vao);
            gl::GenBuffers(1, &mut cube_vbo);

            gl::BindVertexArray(cube_vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, cube_vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as isize,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 5 * 4, std::ptr::null());
            gl::EnableVertexAttribArray(0);

            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 5 * 4, (3 * 4) as *const _);
            gl::EnableVertexAttribArray(1);
        }
        
        let line_shader = Shader::new("line.vert", "line.frag");

        let shader = Shader::new("cube.vert", "cube.frag");

        let projection = Mat4::perspective_rh_gl(
            45.0_f32.to_radians(),
            screen_width / screen_height,
            0.1,
            100.0,
        );
        shader.bind();
        shader.set_mat4("projection", &projection);

        line_shader.bind();
        line_shader.set_mat4("projection", &projection);

        Renderer {
            quad_vao, // <-- you must initialize this properly later
            shader,
            projection,
            line_vao,
            line_vbo,
            line_shader,
            cube_vao,
            cube_vbo,
            textures: Vec::new(),
        }
    }
    

    pub fn draw(&self, surface: &Surface, x: f32, y: f32) {
        unsafe {
            self.shader.bind();

            gl::BindTexture(gl::TEXTURE_2D, surface.texture_id);

            let model = Mat4::from_translation(Vec3::new(x, y, 0.0))
                * Mat4::from_scale(Vec3::new(surface.width as f32, surface.height as f32, 1.0));

            self.shader.set_mat4("model", &model);

            gl::BindVertexArray(self.quad_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }
    }
    pub fn draw_line(
        &self,
        start: crate::shape::point::Point,
        end: crate::shape::point::Point,
        color: Vec3,
    ) {
        let vertices = [
            start.x as f32, start.y as f32, 0.0,
            end.x as f32,   end.y as f32,   0.0,
        ];

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.line_vbo);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                3 * std::mem::size_of::<f32>() as i32,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                (vertices.len() * std::mem::size_of::<f32>()) as isize,
                vertices.as_ptr() as *const _,
            );

            self.line_shader.bind();
            self.line_shader.set_vec3("color", color);

            gl::BindVertexArray(self.line_vao);
            gl::DrawArrays(gl::LINES, 0, 2);
        }
    }

    pub fn draw_cube(&self, view: Mat4, position: Vec3, texture_index: usize) {
        self.shader.bind();

        self.shader.set_mat4("projection", &self.projection);
        self.shader.set_mat4("view", &view);

        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.textures[texture_index]);
        }

        self.shader.set_int("tex", 0);

        let model = Mat4::from_translation(position);
        self.shader.set_mat4("model", &model);

        unsafe {
            gl::BindVertexArray(self.cube_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }
    }
    pub fn draw_textured_quad(
        &mut self,
        verts: &[Vec3; 4],
        texture_index: usize,
        camera_matrix: [[f32; 4]; 4],
    ) {
        let mut screen_coords = [Vec3::ZERO; 4];

        for i in 0..4 {
            let v = verts[i];
            // Convert Vec3 to Vec4 (x, y, z, 1.0)
            let vec4 = [v.x, v.y, v.z, 1.0];

            // Matrix multiply (4x4 * 4x1)
            let mut result = [0.0; 4];
            for row in 0..4 {
                result[row] =
                    camera_matrix[row][0] * vec4[0] +
                    camera_matrix[row][1] * vec4[1] +
                    camera_matrix[row][2] * vec4[2] +
                    camera_matrix[row][3] * vec4[3];
            }

            // Perspective divide
            let w = result[3];
            screen_coords[i] = Vec3 {
                x: result[0] / w,
                y: result[1] / w,
                z: result[2] / w,
            };
        }

        // UVs for the quad
        let uv = [
            (0.0, 0.0),
            (1.0, 0.0),
            (1.0, 1.0),
            (0.0, 1.0),
        ];

        // Draw two triangles (quad)
        self.draw_triangle(screen_coords[0], screen_coords[1], screen_coords[2], uv[0], uv[1], uv[2], texture_index);
        self.draw_triangle(screen_coords[0], screen_coords[2], screen_coords[3], uv[0], uv[2], uv[3], texture_index);
    }

    // You’d still need a working draw_triangle, for example:
    pub fn draw_triangle(
        &mut self,
        v0: Vec3,
        v1: Vec3,
        v2: Vec3,
        uv0: (f32, f32),
        uv1: (f32, f32),
        uv2: (f32, f32),
        texture_index: usize,
    ) {
        // For now, simplest thing: just draw edges as lines
        self.draw_line(shape::point::Point { x: v0.x as f64, y: v0.y as f64 }, shape::point::Point { x: v1.x as f64, y: v1.y as f64 }, Vec3::new(1.0,1.0,1.0));
        self.draw_line(shape::point::Point { x: v1.x as f64, y: v1.y as f64 }, shape::point::Point { x: v2.x as f64, y: v2.y as f64 }, Vec3::new(1.0,1.0,1.0));
        self.draw_line(shape::point::Point { x: v2.x as f64, y: v2.y as f64 }, shape::point::Point { x: v0.x as f64, y: v0.y as f64 }, Vec3::new(1.0,1.0,1.0));
        // Later, you can fill and sample the texture properly
    }
}
pub mod keyboard {
    pub use sdl2::keyboard::{Keycode, Scancode, Mod};
    // You can add your own helpers here later if you want
}