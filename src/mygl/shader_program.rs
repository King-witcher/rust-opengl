use std::ffi::CString;

use nalgebra_glm::Mat4;

use super::gl;

pub struct ShaderProgram(u32);

impl ShaderProgram {
    #[inline]
    pub fn id(&self) -> u32 {
        self.0
    }

    #[inline]
    pub fn create() -> Self {
        let program_id = gl().CreateProgram();
        ShaderProgram(program_id)
    }

    #[inline]
    pub fn attach_shader(&mut self, shader: super::Shader) {
        gl().AttachShader(self.id(), shader.id());
    }

    pub fn link(&mut self) -> Result<(), String> {
        let gl = gl();
        gl.LinkProgram(self.id());

        if self.get_iv(gl46::GL_LINK_STATUS.0) == 0 {
            let log = self.get_info_log();
            return Err(log);
        }

        Ok(())
    }

    pub fn r#use(&self) {
        gl().UseProgram(self.id());
    }

    #[inline]
    pub fn get_uniform_location(&self, name: &str) -> i32 {
        let cname = CString::new(name).expect("Uniform name contains null byte.");
        unsafe { gl().GetUniformLocation(self.id(), cname.as_ptr() as _) }
    }

    #[inline]
    pub fn set_uniform_mat_4(&self, location: i32, value: Mat4) {
        let ptr = &value as *const Mat4 as *const f32;
        unsafe {
            gl().UniformMatrix4fv(location, 1, 0, ptr);
        }
    }

    #[inline]
    pub fn get_iv(&self, pname: u32) -> i32 {
        let mut value = 0;
        unsafe {
            gl().GetProgramiv(self.id(), gl46::GLenum(pname), &mut value);
        }
        value
    }

    #[inline]
    pub fn get_info_log(&self) -> String {
        let gl = gl();
        let mut len = 0;
        unsafe { gl.GetProgramInfoLog(self.id(), 0, &mut len, std::ptr::null_mut()) };
        let mut buffer = vec![0u8; len as usize];
        unsafe { gl.GetProgramInfoLog(self.id(), len, &mut len, buffer.as_mut_ptr()) };
        String::from_utf8_lossy(&buffer[..len as usize]).to_string()
    }

    #[inline]
    pub fn delete(self) {}
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        gl().DeleteProgram(self.id());
    }
}
