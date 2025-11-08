use nalgebra_glm::{self as glm, Mat4, Vec3};

pub struct Camera {
    position: Vec3,
    projection_matrix: Mat4,

    yaw: f32,
    pitch: f32,
    bob: f32,

    camera_matrix: Mat4,
    direction: Vec3,
}

pub enum CameraType {
    Perspective {
        fov: f32,
        aspect: f32,
    },
    Orthographic {
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
    },
}

pub struct CameraCreateInfo {
    pub position: Vec3,
    pub camera_type: CameraType,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    const UP: Vec3 = Vec3::new(0.0, 1.0, 0.0);

    pub fn camera_matrix(&self) -> Mat4 {
        self.camera_matrix
    }

    pub fn position(&self) -> Vec3 {
        self.position
    }

    pub fn yaw(&self) -> f32 {
        self.yaw
    }

    pub fn pitch(&self) -> f32 {
        self.pitch
    }

    pub fn bob(&self) -> f32 {
        self.bob
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
        self.update_camera_matrix();
    }

    pub fn set_yaw(&mut self, yaw: f32) {
        self.yaw = yaw;
        self.update_camera_matrix();
    }

    pub fn set_pitch(&mut self, pitch: f32) {
        self.pitch = pitch.clamp(-89.9, 89.9);
        self.update_camera_matrix();
    }

    pub fn set_bob(&mut self, bob: f32) {
        self.bob = bob;
        self.update_camera_matrix();
    }

    pub fn translate(&mut self, delta: Vec3) {
        self.position += delta;
        self.update_camera_matrix();
    }

    pub fn rotate(&mut self, delta_yaw: f32, delta_pitch: f32) {
        self.yaw += delta_yaw;
        self.pitch += delta_pitch;
        self.pitch = self.pitch.clamp(-89.9, 89.9);
        self.update_camera_matrix();
    }

    #[inline]
    fn update_camera_matrix(&mut self) {
        self.direction = Vec3::new(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        )
        .normalize();

        let view = glm::look_at(&self.position, &(self.position + self.direction), &Self::UP);
        self.camera_matrix = self.projection_matrix * view;
    }
}

impl From<CameraCreateInfo> for Camera {
    fn from(create_info: CameraCreateInfo) -> Self {
        let CameraCreateInfo {
            position,
            camera_type,
            near,
            far,
        } = create_info;

        let projection_matrix = match camera_type {
            CameraType::Perspective { fov, aspect } => {
                let w = (fov.to_radians() / 2.0).tan();
                let h = w / aspect;
                let fovy = 2.0 * h.atan();

                glm::perspective(aspect, fovy, near, far)
            }
            CameraType::Orthographic {
                left,
                right,
                bottom,
                top,
            } => glm::ortho(left, right, bottom, top, near, far),
        };

        let mut camera = Camera {
            camera_matrix: Mat4::identity(),
            direction: Vec3::new(0.0, 0.0, -1.0),
            projection_matrix,
            position,
            yaw: -90.0,
            pitch: 0.0,
            bob: 0.0,
        };

        camera.update_camera_matrix();

        camera
    }
}
