use crate::{
    FilterMode, Texture, TextureCreateInfo, TextureFormat,
    archive::EngineArchive,
    mygl,
    scene::{
        Scene, SceneCreateInfo,
        camera::{Camera, CameraCreateInfo, CameraType},
        model::{Model, ModelCreateInfo, Polygon, Vertex},
    },
    shader_program::{ShaderCode, ShaderProgram},
    window,
};
use anyhow::{Result, anyhow};
use gl46::*;
use image::ImageBuffer;
use nalgebra_glm::{self as glm};
use sdl2::keyboard::Scancode;
use std::{rc::Rc, time::Instant};

pub mod input;

pub struct KEngine {
    window: window::KWindow,
    archive: EngineArchive,
    gl: Rc<GlFns>,
    scene: Scene,
}

impl KEngine {
    pub fn new(width: u32, height: u32, title: &str) -> Result<Self> {
        let window = window::KWindow::new(window::KWindowCreateInfo {
            title,
            width,
            height,
        });

        mygl::load_fns(|s| window.get_proc_address(s)).unwrap();

        unsafe {
            mygl::viewport(0, 0, width as i32, height as i32);
        }

        let archive = EngineArchive::new("base").expect("Failed to load base archive");

        let shader_program = create_shader_program().map_err(|e| anyhow!(e))?;
        let shader_program = Rc::new(shader_program);

        let main_texture = Texture::from(TextureCreateInfo {
            rgba_image: load_image_from_archive(&archive, "container2.png")?,
            internal_format: TextureFormat::RGBA,
            mip_level: 0,
            wrap_s: crate::texture::WrapMode::Repeat,
            wrap_t: crate::texture::WrapMode::Repeat,
            min_filter: FilterMode::Linear,
            mag_filter: FilterMode::Nearest,
            mipmap_interpolation: Some(FilterMode::Linear),
        });
        let main_texture = Rc::new(main_texture);

        let model = Self::load_cube(gl.clone(), shader_program.clone(), main_texture.clone());

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

        Ok(KEngine {
            gl,
            archive,
            window,
            scene,
        })
    }

    pub fn run(&mut self) {
        unsafe {
            // self.gl.ClearColor(0.05, 0.05, 0.05, 1.0);
            self.gl.Enable(GL_DEPTH_TEST);
            self.gl.Enable(GL_BLEND);
            self.gl.BlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);

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
        unsafe {
            self.gl.Clear(GL_COLOR_BUFFER_BIT);
            self.gl.Clear(GL_DEPTH_BUFFER_BIT);
            self.scene.render();
            self.window.swap_window();
        }
    }

    fn load_cube(gl: Rc<GlFns>, shader_program: Rc<ShaderProgram>, texture: Rc<Texture>) -> Model {
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
            gl,
            vertices,
            polygons,
            model_matrix: glm::identity(),
            shader_program,
            texture,
        };

        Model::new(create_info)
    }
}

fn create_shader_program() -> Result<ShaderProgram, String> {
    let vertex_binary = include_bytes!("../base/shaders/vertex.vert.spv");
    let fragment_binary = include_bytes!("../base/shaders/fragment.frag.spv");

    ShaderProgram::new(
        ShaderCode::SPIRV(vertex_binary),
        ShaderCode::SPIRV(fragment_binary),
    )
}

fn load_image_from_archive(
    archive: &EngineArchive,
    path: &str,
) -> Result<ImageBuffer<image::Rgba<u8>, Vec<u8>>> {
    let bytes = archive.load(path)?;
    let image = image::load_from_memory(&bytes)?;
    Ok(image.into_rgba8())
}
