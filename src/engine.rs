use crate::{
    FilterMode, Texture, TextureCreateInfo, TextureFormat,
    archive::EngineArchive,
    model::{Model, ModelCreateInfo, Polygon, Vertex},
    shader_program::{self, ShaderProgram, ShaderProgramCreateInfo},
    window,
};
use anyhow::Result;
use gl::*;
use image::ImageBuffer;
use nalgebra_glm::{self as glm, Mat4};
use std::{ffi::CStr, rc::Rc};

pub struct KEngine {
    window: window::KWindow,
    archive: EngineArchive,
    gl: Rc<GlFns>,
    main_texture: Texture,

    shader_program: ShaderProgram,
    model: Model,
}

impl KEngine {
    pub fn new(width: u32, height: u32, title: &str) -> Result<Self> {
        let window = window::KWindow::new(window::KWindowCreateInfo {
            title,
            width,
            height,
        });

        let load_function = |s: *const u8| unsafe {
            let str = CStr::from_ptr(s as _)
                .to_str()
                .expect("Failed to convert CStr");
            window.get_proc_address(str)
        };

        let gl = unsafe { GlFns::load_from(&load_function) };
        let gl = gl.expect("Failed to load OpenGL functions");
        let gl = Rc::new(gl);

        unsafe {
            gl.Viewport(0, 0, width as i32, height as i32);
        }

        let archive = EngineArchive::new("base").expect("Failed to load base archive");

        let shader_program = ShaderProgram::new(ShaderProgramCreateInfo {
            gl: gl.clone(),
            vertex_path: "base/shaders/vertex.vert.spv",
            fragment_path: "base/shaders/fragment.frag.spv",
            source_type: shader_program::ShaderSourceType::SPIRV,
        });

        let main_texture = Texture::from(TextureCreateInfo {
            gl: gl.clone(),
            rgba_image: load_image_from_archive(&archive, "bianca.jpg")?,
            internal_format: TextureFormat::RGBA,
            mip_level: 0,
            wrap_s: crate::texture::WrapMode::Repeat,
            wrap_t: crate::texture::WrapMode::Repeat,
            min_filter: FilterMode::Linear,
            mag_filter: FilterMode::Linear,
            mipmap_interpolation: Some(FilterMode::Linear),
        });

        let model = Self::load_cube(gl.clone());

        Ok(KEngine {
            gl,
            main_texture,
            archive,
            window,
            shader_program,
            model,
        })
    }

    pub fn run(&self) {
        unsafe {
            self.model.bind();
            self.shader_program.use_program();

            let mut model_mat = glm::rotate(
                &Mat4::identity(),
                45.0f32.to_radians(),
                &glm::vec3(1.0, 0.0, 0.0),
            );
            let view_mat = glm::translate(&Mat4::identity(), &glm::vec3(0.0, 0.0, -3.0));
            let proj_mat = glm::perspective(16.0 / 9.0, 60.0f32.to_radians(), 0.1, 100.0);

            self.shader_program.set_uniform_mat_4(1, view_mat);
            self.shader_program.set_uniform_mat_4(2, proj_mat);

            let mut event_pump = self.window.event_pump();
            self.gl.ClearColor(0.2, 0.1, 0.3, 1.0);
            self.gl.Enable(GL_DEPTH_TEST);
            self.main_texture.bind(0);

            'main_loop: loop {
                for event in event_pump.poll_iter() {
                    match event {
                        sdl2::event::Event::Quit { .. } => break 'main_loop,
                        _ => {}
                    }
                }

                model_mat = glm::rotate(&model_mat, 0.1f32.to_radians(), &glm::vec3(0.0, 1.0, 0.0));
                self.shader_program.set_uniform_mat_4(0, model_mat);

                self.draw_frame();
            }
        }
    }

    fn draw_frame(&self) {
        unsafe {
            self.gl.Clear(GL_COLOR_BUFFER_BIT);
            self.gl.Clear(GL_DEPTH_BUFFER_BIT);
            self.gl.DrawElements(
                GL_TRIANGLES,
                self.model.vertex_count() as i32,
                GL_UNSIGNED_INT,
                0 as _,
            );
            self.window.swap_window();
        }
    }

    fn load_cube(gl: Rc<GlFns>) -> Model {
        let vertices = vec![
            Vertex {
                position: [-1.0, -1.0, 1.0],
                color: [1.0, 0.0, 0.0],
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [-1.0, 1.0, 1.0],
                color: [1.0, 0.0, 0.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [1.0, 1.0, 1.0],
                color: [1.0, 0.0, 0.0],
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                position: [1.0, -1.0, 1.0],
                color: [1.0, 0.0, 0.0],
                tex_coords: [1.0, 1.0],
            },
            Vertex {
                position: [-1.0, -1.0, -1.0],
                color: [1.0, 0.0, 0.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [-1.0, 1.0, -1.0],
                color: [1.0, 0.0, 0.0],
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [1.0, 1.0, -1.0],
                color: [1.0, 0.0, 0.0],
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
            vertices: &vertices,
            polygons: &polygons,
        };

        Model::new(create_info)
    }
}

fn load_image_from_archive(
    archive: &EngineArchive,
    path: &str,
) -> Result<ImageBuffer<image::Rgba<u8>, Vec<u8>>> {
    let bytes = archive.load(path)?;
    let image = image::load_from_memory(&bytes)?;
    Ok(image.into_rgba8())
}
