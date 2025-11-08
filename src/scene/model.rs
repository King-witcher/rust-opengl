use std::rc::Rc;

use gl::{
    GL_ARRAY_BUFFER, GL_ELEMENT_ARRAY_BUFFER, GL_STATIC_DRAW, GL_TRIANGLES, GL_UNSIGNED_INT, GLenum,
};
use nalgebra_glm::Mat4;

use crate::{Texture, scene::camera::Camera, shader_program::ShaderProgram};

pub struct Model {
    gl: Rc<gl46::GlFns>,
    vbo: u32,
    ebo: u32,
    vao: u32,
    vertex_count: i32,
    model_matrix: Mat4,
    texture: Rc<Texture>,
    shader_program: Rc<ShaderProgram>,
    model_location: i32,
    camera_location: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub tex_coords: [f32; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Polygon {
    pub indices: [u32; 3],
}

pub struct ModelCreateInfo {
    pub gl: Rc<gl46::GlFns>,
    pub vertices: Vec<Vertex>,
    pub polygons: Vec<Polygon>,
    pub model_matrix: Mat4,
    pub texture: Rc<Texture>,
    pub shader_program: Rc<ShaderProgram>,
}

struct AttributeDescription {
    pub location: u32,
    pub components: i32,
    pub data_type: GLenum,
    pub stride: usize,
    pub offset: usize,
}

impl Model {
    pub fn new(create_info: ModelCreateInfo) -> Self {
        let ModelCreateInfo {
            gl,
            vertices,
            polygons,
            texture,
            model_matrix,
            shader_program,
        } = create_info;

        let vertex_count = (polygons.len() * 3) as i32;
        let mut vao = 0;
        let mut vbo = 0;
        let mut ebo = 0;

        unsafe {
            gl.GenVertexArrays(1, &mut vao);
            gl.GenBuffers(1, &mut vbo);
            gl.GenBuffers(1, &mut ebo);

            gl.BindVertexArray(vao);
            gl.BindBuffer(GL_ARRAY_BUFFER, vbo);
            gl.BindBuffer(GL_ELEMENT_ARRAY_BUFFER, ebo);

            gl.BufferData(
                GL_ARRAY_BUFFER,
                (vertices.len() * size_of::<Vertex>()) as _,
                vertices.as_ptr() as _,
                GL_STATIC_DRAW,
            );
            gl.BufferData(
                GL_ELEMENT_ARRAY_BUFFER,
                (polygons.len() * size_of::<Polygon>()) as _,
                polygons.as_ptr() as _,
                GL_STATIC_DRAW,
            );

            let attribute_descriptions = Self::default_attribute_descriptions();
            for attr in attribute_descriptions.iter() {
                gl.EnableVertexAttribArray(attr.location);
                gl.VertexAttribPointer(
                    attr.location,
                    attr.components,
                    attr.data_type,
                    0,
                    attr.stride as _,
                    attr.offset as _,
                );
            }

            let model_location = shader_program.uniform_location("model");
            let camera_location = shader_program.uniform_location("camera");

            Self {
                gl,
                vbo,
                ebo,
                vao,
                texture,
                vertex_count,
                model_matrix,
                shader_program,
                model_location,
                camera_location,
            }
        }
    }

    pub fn bind(&self) {
        self.gl.BindVertexArray(self.vao);
    }

    pub fn rotate(&mut self, rotation: &Mat4) {
        self.model_matrix = rotation * self.model_matrix;
    }

    pub fn render(&self, camera: &Camera) {
        self.shader_program.use_program();
        self.texture.bind(0);
        self.bind();

        self.shader_program
            .set_uniform_mat_4(self.model_location, self.model_matrix);
        self.shader_program
            .set_uniform_mat_4(self.camera_location, *camera.get_camera_matrix());

        unsafe {
            self.gl.DrawElements(
                GL_TRIANGLES,
                self.vertex_count as _,
                GL_UNSIGNED_INT,
                0 as _,
            );
        }
    }

    fn default_attribute_descriptions() -> Vec<AttributeDescription> {
        vec![
            AttributeDescription {
                location: 0,
                components: 3,
                data_type: gl::GL_FLOAT,
                stride: std::mem::size_of::<Vertex>(),
                offset: std::mem::offset_of!(Vertex, position),
            },
            AttributeDescription {
                location: 1,
                components: 3,
                data_type: gl::GL_FLOAT,
                stride: std::mem::size_of::<Vertex>(),
                offset: std::mem::offset_of!(Vertex, color),
            },
            AttributeDescription {
                location: 2,
                components: 2,
                data_type: gl::GL_FLOAT,
                stride: std::mem::size_of::<Vertex>(),
                offset: std::mem::offset_of!(Vertex, tex_coords),
            },
        ]
    }
}

impl Drop for Model {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteVertexArrays(1, &self.vao);
            self.gl.DeleteBuffers(1, &self.ebo);
            self.gl.DeleteBuffers(1, &self.vbo);
        }
    }
}
