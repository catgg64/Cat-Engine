pub use cgmath;

#[derive(Debug, Clone)]
pub struct Coordinate2D {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone)]
pub struct Coordinate3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Clone, Debug)]
pub enum Range<T> {
    Range(std::ops::Range<T>),
    Full,
}

pub struct Camera {
    eye: cgmath::Point3<f32>,
    target: cgmath::Point3<f32>,
    up: cgmath::Vector3<f32>,
    aspect: f32,
    fov: f32,
    znear: f32,
    zfar: f32,
}

impl Camera {
    pub fn new(eye: (f32, f32, f32), target: (f32, f32, f32), up: (f32, f32, f32), aspect: f32, fov: f32, znear: f32, zfar: f32) -> Self {
        Self{
            eye: eye.into(),
            target: target.into(),
            up: up.into(),
            aspect,
            fov,
            znear,
            zfar,
        }
    }

    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = cgmath::perspective(cgmath::Deg(self.fov), self.aspect, self.znear, self.zfar);

        #[rustfmt::skip]
        pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::from_cols(
            cgmath::Vector4::new(1.0, 0.0, 0.0, 0.0),
            cgmath::Vector4::new(0.0, 1.0, 0.0, 0.0),
            cgmath::Vector4::new(0.0, 0.0, 0.5, 0.0),
            cgmath::Vector4::new(0.0, 0.0, 0.5, 1.0),
        );

        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }

    pub fn get_raw_projection_matrix(&self) -> [[f32; 4]; 4] {
        self.build_view_projection_matrix().into()
    }
}
