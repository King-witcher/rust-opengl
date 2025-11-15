use gl46;
use std::ffi::{CStr, c_void};

mod shader;
mod shader_program;
mod texture;

pub use shader::*;
pub use shader_program::*;
pub use texture::*;

static mut GL: Option<gl46::GlFns> = None;

/// Loads the OpenGL function pointers into a global static variable.
pub fn load_fns(loader: impl Fn(&str) -> *const c_void) -> Result<(), &'static str> {
    unsafe {
        let load_fns = |ptr: *const u8| -> *const c_void {
            let cstr = CStr::from_ptr(ptr.cast());
            let str = cstr.to_str().unwrap();
            loader(str) as _
        };
        let gl = gl46::GlFns::load_from(&load_fns)?;
        GL = Some(gl);
        Ok(())
    }
}

#[inline]
pub fn gl() -> &'static gl46::GlFns {
    unsafe { GL.as_ref().expect("OpenGL functions not initialized") }
}

pub unsafe fn viewport(x: i32, y: i32, width: i32, height: i32) {
    unsafe { gl().Viewport(x, y, width, height) }
}

pub fn tex_parameter_i(target: TextureTarget, pname: impl Into<u32>, param: impl Into<i32>) {
    unsafe {
        gl().TexParameteri(target.into(), gl46::GLenum(pname.into()), param.into());
    }
}

pub fn tex_image_2d(
    target: TextureTarget,
    level: i32,
    internal_format: impl Into<i32>,
    width: i32,
    height: i32,
    format: impl Into<u32>,
    type_: impl Into<u32>,
    data: &[u8],
) {
    unsafe {
        gl().TexImage2D(
            target.into(),
            level,
            internal_format.into(),
            width,
            height,
            0,
            gl46::GLenum(format.into()),
            gl46::GLenum(type_.into()),
            data.as_ptr().cast(),
        );
    }
}

pub fn generate_mipmap(target: TextureTarget) {
    unsafe {
        gl().GenerateMipmap(target.into());
    }
}

pub fn active_texture(texture_unit: u32) {
    unsafe {
        gl().ActiveTexture(gl46::GLenum(texture_unit));
    }
}
