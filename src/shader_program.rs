mod shader;

use gl::*;
use nalgebra_glm::Mat4;
use std::{ffi::CString, rc::Rc};

pub use shader::ShaderSourceType;

pub struct ShaderProgram {
    gl: Rc<gl46::GlFns>,
    program_id: u32,
}

pub struct ShaderProgramCreateInfo<'s> {
    pub gl: Rc<gl46::GlFns>,
    pub vertex_path: &'s str,
    pub fragment_path: &'s str,
    pub source_type: shader::ShaderSourceType,
}

impl ShaderProgram {
    pub fn new(create_info: ShaderProgramCreateInfo) -> Self {
        let ShaderProgramCreateInfo {
            gl,
            vertex_path: vertex_source,
            fragment_path: fragment_source,
            source_type,
        } = create_info;

        let vertex_create_info = shader::ShaderCreateInfo {
            gl: gl.clone(),
            path: vertex_source,
            shader_type: shader::ShaderType::Vertex,
            source_type,
        };

        let fragment_create_info = shader::ShaderCreateInfo {
            gl: gl.clone(),
            path: fragment_source,
            shader_type: shader::ShaderType::Fragment,
            source_type,
        };

        let vertex = shader::Shader::new(vertex_create_info);
        let fragment = shader::Shader::new(fragment_create_info);

        let program_id = Self::create_gl_shader_program(gl.clone(), vertex, fragment);

        ShaderProgram { gl, program_id }
    }

    pub fn use_program(&self) {
        self.gl.UseProgram(self.program_id);
    }

    pub fn uniform_location(&self, name: &str) -> i32 {
        unsafe {
            let cname = CString::new(name).unwrap();
            self.gl
                .GetUniformLocation(self.program_id, cname.as_ptr() as _)
        }
    }

    pub fn set_uniform_mat_4(&self, location: i32, value: Mat4) {
        unsafe {
            let ptr = &value as *const Mat4 as *const f32;
            self.gl.UniformMatrix4fv(location, 1, 0, ptr);
        }
    }

    fn create_gl_shader_program(
        gl: Rc<gl46::GlFns>,
        vertex: shader::Shader,
        fragment: shader::Shader,
    ) -> u32 {
        unsafe {
            let program = gl.CreateProgram();
            gl.AttachShader(program, vertex.id());
            gl.AttachShader(program, fragment.id());
            gl.LinkProgram(program);

            let mut success: i32 = 0;
            gl.GetProgramiv(program, GL_LINK_STATUS, &mut success);

            if success == 0 {
                let mut length = 0;
                let mut chars = [0u8; 512];
                gl.GetProgramInfoLog(program, 512, &mut length, chars.as_mut_ptr());

                panic!(
                    "Failed to link shader program: {}",
                    std::str::from_utf8(&chars[..length as usize]).unwrap()
                );
            }

            program
        }
    }
}
