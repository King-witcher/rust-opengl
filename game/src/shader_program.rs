mod shader;

use crate::shader_program::shader::Shader;
pub use crate::shader_program::shader::ShaderCode;
use gl;
use nalgebra_glm::Mat4;

pub struct ShaderProgram {
    program: gl::ShaderProgram,
}

impl ShaderProgram {
    pub fn new(vertex: ShaderCode, fragment: ShaderCode) -> Result<Self, String> {
        let vertex_shader = Shader::new(vertex, gl::ShaderType::Vertex);
        let fragment_shader = Shader::new(fragment, gl::ShaderType::Fragment);

        let mut program = gl::ShaderProgram::create();

        program.attach_shader(vertex_shader.into());
        program.attach_shader(fragment_shader.into());
        program.link();

        Ok(Self { program })
    }

    pub fn r#use(&self) {
        self.program.r#use();
    }

    pub fn uniform_location(&self, name: &str) -> i32 {
        self.program.get_uniform_location(name)
    }

    pub fn set_uniform_mat_4(&self, location: i32, value: Mat4) {
        self.program.set_uniform_mat_4(location, value);
    }
}
