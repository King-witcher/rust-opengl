use std::rc::Rc;

use gl;
use nalgebra_glm::Mat4;

use crate::{Texture, scene::camera::Camera, shader_program::ShaderProgram};

pub struct Model {
    vbo: gl::Buffer,
    ebo: gl::Buffer,
    vertex_array: gl::VertexArray,
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
    pub vertices: Vec<Vertex>,
    pub polygons: Vec<Polygon>,
    pub model_matrix: Mat4,
    pub texture: Rc<Texture>,
    pub shader_program: Rc<ShaderProgram>,
}

pub struct AttributeDescription {
    pub location: u32,
    pub components: i32,
    pub data_type: gl::VertexAttribPointerType,
    pub stride: usize,
    pub offset: usize,
}

pub trait IModel {
    fn shader_program() -> Rc<ShaderProgram>;
    fn attribute_descriptions() -> Vec<AttributeDescription>;
}

impl Model {
    pub fn new(create_info: ModelCreateInfo) -> Self {
        let ModelCreateInfo {
            vertices,
            polygons,
            texture,
            model_matrix,
            shader_program,
        } = create_info;

        let vertex_count = (polygons.len() * 3) as i32;

        let mut vertex_array = gl::VertexArray::create1();
        vertex_array.bind();

        let mut vertex_buffer = gl::Buffer::gen1();
        vertex_buffer.bind(gl::BufferTarget::ArrayBuffer);
        gl::buffer_data(
            gl::BufferTarget::ArrayBuffer,
            vertices,
            gl::BufferUsage::StaticDraw,
        );
        // vertex_buffer.storage(vertices, gl::BufferUsage::StaticDraw);

        let mut index_buffer = gl::Buffer::gen1();
        index_buffer.bind(gl::BufferTarget::ElementArrayBuffer);
        gl::buffer_data(
            gl::BufferTarget::ElementArrayBuffer,
            polygons,
            gl::BufferUsage::StaticDraw,
        );
        // index_buffer.storage(polygons, gl::BufferUsage::StaticDraw);

        let attribute_descriptions = Self::default_attribute_descriptions();

        for attr in attribute_descriptions.iter() {
            vertex_array.enable_attrib(attr.location);
            unsafe {
                gl::vertex_attrib_pointer(
                    attr.location,
                    attr.components,
                    attr.data_type,
                    false,
                    attr.stride,
                    attr.offset,
                );
            }
        }

        let model_location = shader_program.uniform_location("model");
        let camera_location = shader_program.uniform_location("camera");

        Self {
            vbo: vertex_buffer,
            ebo: index_buffer,
            vertex_array,
            texture,
            vertex_count,
            model_matrix,
            shader_program,
            model_location,
            camera_location,
        }
    }

    pub fn bind(&self) {
        self.vertex_array.bind();
    }

    pub fn rotate(&mut self, rotation: &Mat4) {
        self.model_matrix = rotation * self.model_matrix;
    }

    pub fn render(&self, camera: &Camera) {
        self.shader_program.r#use();
        self.texture.bind_to_unit(0);
        self.bind();

        self.shader_program
            .set_uniform_mat_4(self.model_location, self.model_matrix);
        self.shader_program
            .set_uniform_mat_4(self.camera_location, camera.camera_matrix());

        gl::draw_elements(
            gl::DrawMode::Triangles,
            self.vertex_count,
            gl::VertexAttribPointerType::UnsignedInt,
            0,
        );
    }

    fn default_attribute_descriptions() -> Vec<AttributeDescription> {
        vec![
            AttributeDescription {
                location: 0,
                components: 3,
                data_type: gl::VertexAttribPointerType::Float,
                stride: std::mem::size_of::<Vertex>(),
                offset: std::mem::offset_of!(Vertex, position),
            },
            AttributeDescription {
                location: 1,
                components: 3,
                data_type: gl::VertexAttribPointerType::Float,
                stride: std::mem::size_of::<Vertex>(),
                offset: std::mem::offset_of!(Vertex, color),
            },
            AttributeDescription {
                location: 2,
                components: 2,
                data_type: gl::VertexAttribPointerType::Float,
                stride: std::mem::size_of::<Vertex>(),
                offset: std::mem::offset_of!(Vertex, tex_coords),
            },
        ]
    }
}
