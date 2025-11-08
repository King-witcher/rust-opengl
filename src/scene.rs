use camera::Camera;
use model::Model;

pub mod camera;
pub mod model;

pub struct Scene {
    pub models: Vec<Model>,
    pub camera: Camera,
}

pub struct SceneCreateInfo {
    pub models: Vec<Model>,
    pub camera: Camera,
}

impl Scene {
    pub fn render(&self) {
        for model in &self.models {
            model.render(&self.camera);
        }
    }
}

impl From<SceneCreateInfo> for Scene {
    fn from(create_info: SceneCreateInfo) -> Self {
        Scene {
            models: create_info.models,
            camera: create_info.camera,
        }
    }
}
