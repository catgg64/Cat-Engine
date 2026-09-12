#[derive(Clone, Debug)]
pub enum ProjectionType {
    Projection(f32, f32, f32, f32),
    Ortographic(f32, f32, f32, f32, f32, f32),
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Projection {
    pub camera_position_x: f32,
    pub camera_position_y: f32,
    pub camera_position_z: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub front_x: f32,
    pub front_y: f32,
    pub front_z: f32,
    view_matrix: glam::Mat4,
    projection: glam::Mat4,
    pub projection_type: ProjectionType,
}

impl Projection {
    pub fn new(projection_type: ProjectionType, cam_x: f32, cam_y: f32, cam_z: f32, yaw: f32, pitch: f32) -> Self {
        let (camera_position_x, camera_position_y, camera_position_z) = (cam_x, cam_y, cam_z);

        let (front_x, front_y, front_z) = (
            yaw.cos() * pitch.cos(),
            pitch.sin(),
            -yaw.sin() * pitch.cos()
        );

        let view_matrix = glam::camera::rh::view::look_at_mat4(
            glam::Vec3 { x: camera_position_x, y: camera_position_y, z: camera_position_z },
            glam::Vec3 { x: camera_position_x, y: camera_position_y, z: camera_position_z } + glam::Vec3 { x: front_x, y: front_y, z: front_z },
            glam::Vec3::Y,
        );

        let projection = match projection_type {
            ProjectionType::Projection(fov, aspect_ratio, z_near, z_far) => {
                glam::camera::rh::proj::directx::perspective(fov, aspect_ratio, z_near, z_far)
            }

            ProjectionType::Ortographic(left, right, bottom, top, near, far) => {
                glam::camera::rh::proj::directx::orthographic(left, right, bottom, top, near, far)
            }
        };
        Self { camera_position_x, camera_position_y, camera_position_z, yaw, pitch, front_x, front_y, front_z, view_matrix, projection, projection_type }
    }

    pub fn get_view(&self) -> [[f32; 4]; 4] {
        self.view_matrix.to_cols_array_2d()
    }

    pub fn get_proj(&self) -> [[f32; 4]; 4] {
        self.projection.to_cols_array_2d()
    }


    pub fn recalc(&mut self) {
        (self.front_x, self.front_y, self.front_z) = (
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            -self.yaw.sin() * self.pitch.cos()
        );

        self.view_matrix = glam::camera::rh::view::look_at_mat4(
            glam::Vec3 { x: self.camera_position_x, y: self.camera_position_y, z: self.camera_position_z },
            glam::Vec3 { x: self.camera_position_x, y: self.camera_position_y, z: self.camera_position_z } + glam::Vec3 { x: self.front_x, y: self.front_y, z: self.front_z },
            glam::Vec3::Y,
        );

        self.projection = match self.projection_type {
            ProjectionType::Projection(fov, aspect_ratio, z_near, z_far) => {
                glam::camera::rh::proj::directx::perspective(fov, aspect_ratio, z_near, z_far)
            }

            ProjectionType::Ortographic(left, right, bottom, top, near, far) => {
                glam::camera::rh::proj::directx::orthographic(left, right, bottom, top, near, far)
            }
        };
    }
}
