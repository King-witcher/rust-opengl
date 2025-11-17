use crate::{
    Texture, TextureCreateInfo,
    archive::EngineArchive,
    scene::{
        Scene, SceneCreateInfo,
        camera::{Camera, CameraCreateInfo, CameraType},
        model::{Model, ModelCreateInfo, Polygon, Vertex},
    },
    shader_program::{ShaderCode, ShaderProgram},
    window,
};
use anyhow::{Result, anyhow};
use gl::{self, Capability};
use image::ImageBuffer;
use nalgebra_glm::{self as glm};
use sdl2::keyboard::Scancode;
use std::{rc::Rc, time::Instant};

pub mod input;

pub struct KEngine {
    window: window::KWindow,
    archive: EngineArchive,
    scene: Scene,
}

impl KEngine {
    pub fn new(width: u32, height: u32, title: &str) -> Result<Self> {
        let window = window::KWindow::new(window::KWindowCreateInfo {
            title,
            width,
            height,
        });

        gl::load_fns(|s| window.get_proc_address(s)).unwrap();

        let archive = EngineArchive::new("base").expect("Failed to load base archive");

        let shader_program = create_shader_program().map_err(|e| anyhow!(e))?;
        let shader_program = Rc::new(shader_program);

        let scene = load_scene(&archive, shader_program)?;

        Ok(KEngine {
            archive,
            window,
            scene,
        })
    }

    pub fn run(&mut self) {
        gl::clear_color(0.1, 0.1, 0.1, 1.0);
        gl::enable(Capability::DepthTest);

        self.window.set_relative_mouse_mode(true);

        let mut input = input::Input::new();
        let mut event_pump = self.window.event_pump();

        let mut last_frame = Instant::now();
        let mut delta_time;
        loop {
            self.draw_frame();

            let events = event_pump.poll_iter();
            input.update(events);
            if input.exit {
                break;
            }

            delta_time = last_frame.elapsed().as_secs_f32();
            last_frame = Instant::now();

            self.process_input(&mut input, delta_time);

            if input.exit {
                break;
            }
        }
    }

    fn process_input(&mut self, input: &mut input::Input, delta_time: f32) {
        if input.was_key_pressed(Scancode::Escape) {
            input.exit = true;
        }

        let camera = &mut self.scene.camera;

        let mouse_rel = input.mouse_rel();
        let delta_yaw = -mouse_rel.0 as f32 * 0.022 * 2.2;
        let delta_pitch = -mouse_rel.1 as f32 * 0.022 * 2.2;
        let right = camera
            .direction()
            .cross(&glm::vec3(0.0, 1.0, 0.0))
            .normalize();

        camera.rotate(delta_yaw, delta_pitch);

        if input.is_key_down(Scancode::W) {
            camera.translate(camera.direction() * delta_time * 5.0);
        }

        if input.is_key_down(Scancode::S) {
            camera.translate(-camera.direction() * delta_time * 5.0);
        }

        if input.is_key_down(Scancode::A) {
            camera.translate(-right * delta_time * 5.0);
        }

        if input.is_key_down(Scancode::D) {
            camera.translate(right * delta_time * 5.0);
        }

        if input.is_key_down(Scancode::Space) {
            camera.translate(glm::vec3(0.0, 1.0, 0.0) * delta_time * 5.0);
        }

        if input.is_key_down(Scancode::LAlt) {
            camera.translate(glm::vec3(0.0, -1.0, 0.0) * delta_time * 5.0);
        }
    }

    fn draw_frame(&self) {
        gl::clear(gl::ClearMask::ColorBufferBit);
        gl::clear(gl::ClearMask::DepthBufferBit);
        self.scene.render();
        self.window.swap_window();
    }
}

fn load_cube(shader_program: Rc<ShaderProgram>, texture: Rc<Texture>) -> Model {
    let vertices = vec![
        Vertex {
            position: [-1.0, -1.0, 1.0],
            color: [0.0, 0.0, 1.0],
            tex_coords: [0.0, 1.0],
        },
        Vertex {
            position: [-1.0, 1.0, 1.0],
            color: [0.0, 1.0, 1.0],
            tex_coords: [0.0, 0.0],
        },
        Vertex {
            position: [1.0, 1.0, 1.0],
            color: [1.0, 1.0, 1.0],
            tex_coords: [1.0, 0.0],
        },
        Vertex {
            position: [1.0, -1.0, 1.0],
            color: [1.0, 0.0, 1.0],
            tex_coords: [1.0, 1.0],
        },
        Vertex {
            position: [-1.0, -1.0, -1.0],
            color: [0.0, 0.0, 0.0],
            tex_coords: [0.0, 0.0],
        },
        Vertex {
            position: [-1.0, 1.0, -1.0],
            color: [0.0, 1.0, 0.0],
            tex_coords: [0.0, 1.0],
        },
        Vertex {
            position: [1.0, 1.0, -1.0],
            color: [1.0, 1.0, 0.0],
            tex_coords: [1.0, 1.0],
        },
        Vertex {
            position: [1.0, -1.0, -1.0],
            color: [1.0, 0.0, 0.0],
            tex_coords: [1.0, 0.0],
        },
    ];

    let polygons = vec![
        Polygon { indices: [0, 1, 2] },
        Polygon { indices: [2, 3, 0] },
        Polygon { indices: [4, 5, 6] },
        Polygon { indices: [6, 7, 4] },
        Polygon { indices: [0, 4, 7] },
        Polygon { indices: [7, 3, 0] },
        Polygon { indices: [1, 5, 6] },
        Polygon { indices: [6, 2, 1] },
        Polygon { indices: [1, 0, 4] },
        Polygon { indices: [4, 5, 1] },
        Polygon { indices: [3, 2, 6] },
        Polygon { indices: [6, 7, 3] },
    ];

    let create_info = ModelCreateInfo {
        vertices,
        polygons,
        model_matrix: glm::identity(),
        shader_program,
        texture,
    };

    Model::new(create_info)
}

fn load_scene(archive: &EngineArchive, shader_program: Rc<ShaderProgram>) -> Result<Scene> {
    let main_texture = Texture::from(TextureCreateInfo {
        rgba_image: load_image_from_archive(archive, "container2.png")?,
        internal_format: gl::BaseInternalFormat::RGBA,
        mip_level: 0,
        wrap_s: gl::TextureWrapMode::Repeat,
        wrap_t: gl::TextureWrapMode::Repeat,
        min_filter: gl::InterpolationMode::Linear,
        mag_filter: gl::InterpolationMode::Nearest,
        mipmap_interpolation: Some(gl::InterpolationMode::Linear),
    });
    let main_texture = Rc::new(main_texture);

    let model = load_cube(shader_program, main_texture.clone());

    let camera = Camera::from(CameraCreateInfo {
        camera_type: CameraType::Perspective {
            fov: 90.0,
            aspect: 16.0 / 9.0,
        },
        far: 1000.0,
        near: 0.01,
        position: glm::vec3(0.0, 0.0, 0.0),
    });

    let scene = Scene::from(SceneCreateInfo {
        models: vec![model],
        camera,
    });

    Ok(scene)
}

fn create_shader_program() -> Result<ShaderProgram, String> {
    let vertex = include_str!("shaders/vertex.vert");
    let fragment = include_str!("shaders/fragment.frag");

    ShaderProgram::new(ShaderCode::GLSL(vertex), ShaderCode::GLSL(fragment))
}

fn load_image_from_archive(
    archive: &EngineArchive,
    path: &str,
) -> Result<ImageBuffer<image::Rgba<u8>, Vec<u8>>> {
    let bytes = archive.load(path)?;
    let image = image::load_from_memory(&bytes)?;
    Ok(image.into_rgba8())
}
