mod shader;

pub use crate::shader_program::shader::ShaderCode;
use crate::{mygl, shader_program::shader::Shader};
use nalgebra_glm::Mat4;
pub use shader::ShaderSourceType;

pub struct ShaderProgram {
    program: mygl::ShaderProgram,
}

impl ShaderProgram {
    pub fn new(vertex: ShaderCode, fragment: ShaderCode) -> Result<Self, String> {
        let vertex_shader = Shader::new(vertex, mygl::ShaderType::Vertex)?;
        let fragment_shader = Shader::new(fragment, mygl::ShaderType::Fragment)?;

        let mut program = mygl::ShaderProgram::create();

        program.attach_shader(vertex_shader.into());
        program.attach_shader(fragment_shader.into());
        program.link()?;

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
