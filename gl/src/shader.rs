use std::ffi::CString;

use gl46::{GLenum, GL_SHADER_BINARY_FORMAT_SPIR_V, GL_SPIR_V_BINARY};

use super::gl;

pub struct Shader(u32);

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum ShaderType {
    Compute = gl46::GL_COMPUTE_SHADER.0,
    Vertex = gl46::GL_VERTEX_SHADER.0,
    TessControl = gl46::GL_TESS_CONTROL_SHADER.0,
    TessEvaluation = gl46::GL_TESS_EVALUATION_SHADER.0,
    Geometry = gl46::GL_GEOMETRY_SHADER.0,
    Fragment = gl46::GL_FRAGMENT_SHADER.0,
}

impl Shader {
    #[inline]
    pub fn create(shader_type: ShaderType) -> Self {
        let shader_id = gl().CreateShader(GLenum(shader_type as _));
        Self(shader_id)
    }

    #[inline]
    pub fn id(&self) -> u32 {
        self.0
    }

    pub fn source(&mut self, strings: &[&str]) {
        let lengths: Vec<_> = strings.iter().map(|s| s.len()).collect();
        let c_strings: Vec<_> = strings.iter().map(|s| s.as_ptr() as *const u8).collect();

        unsafe {
            gl().ShaderSource(
                self.id(),
                c_strings.len() as i32,
                c_strings.as_ptr(),
                lengths.as_ptr() as *const i32,
            )
        };
    }

    pub fn binary(&mut self, binary: &[u8]) {
        let gl = gl();

        unsafe {
            gl.ShaderBinary(
                1,
                &self.id(),
                GL_SHADER_BINARY_FORMAT_SPIR_V,
                binary.as_ptr().cast(),
                binary.len() as i32,
            )
        };
    }

    pub fn compile(&mut self) {
        let gl = gl();
        gl.CompileShader(self.id());

        if self.get_iv(gl46::GL_COMPILE_STATUS.0) == 0 {
            let log = self.get_info_log();
            panic!("Shader compilation failed: {}", log);
        }
    }

    pub fn specialize(&mut self, entry_point: &str, specialization_constants: &[(u32, u32)]) {
        let gl = gl();

        let c_entry = CString::new(entry_point)
            .map_err(|_| "Illegal null byte in entry point name.")
            .expect("Failed to create CString.");

        let constant_indexes = specialization_constants
            .iter()
            .map(|(index, _)| *index)
            .collect::<Vec<u32>>();

        let constant_values = specialization_constants
            .iter()
            .map(|(_, value)| *value)
            .collect::<Vec<u32>>();

        unsafe {
            gl.SpecializeShader(
                self.id(),
                c_entry.as_ptr().cast(),
                specialization_constants.len() as u32,
                constant_indexes.as_ptr(),
                constant_values.as_ptr(),
            );
        };

        if self.get_iv(GL_SPIR_V_BINARY.0) == 0 {
            let log = self.get_info_log();
            panic!("Shader specialization failed: {}", log);
        }
    }

    #[inline]
    pub fn get_iv(&self, pname: u32) -> i32 {
        let mut value = 0;
        unsafe {
            gl().GetShaderiv(self.id(), gl46::GLenum(pname), &mut value);
        }
        value
    }

    #[inline]
    pub fn get_info_log(&self) -> String {
        const BUFFER_SIZE: usize = 1024;
        let gl = gl();
        let mut len = 0;
        let mut buffer = [0u8; BUFFER_SIZE];
        unsafe {
            gl.GetShaderInfoLog(
                self.id(),
                BUFFER_SIZE as i32,
                &mut len,
                &mut buffer as *mut _,
            )
        };
        String::from_utf8_lossy(&buffer[..len as usize]).to_string()
    }

    pub fn delete(self) {}
}

impl Drop for Shader {
    fn drop(&mut self) {
        gl().DeleteShader(self.id());
    }
}
