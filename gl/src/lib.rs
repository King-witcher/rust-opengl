use gl46::{self, GLenum};
use std::{
    ffi::{c_void, CStr},
    fmt::{Debug, Display},
};

mod buffer;
mod debug;
mod shader;
mod shader_program;
mod texture;
mod vertex_array;

pub use buffer::*;
pub use debug::*;
pub use shader::*;
pub use shader_program::*;
pub use texture::*;
pub use vertex_array::*;

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
#[allow(static_mut_refs)]
pub fn gl() -> &'static gl46::GlFns {
    unsafe {
        if cfg!(debug_assertions) {
            if GL.is_none() {
                panic!("OpenGL functions not initialized. Did you forget to call gl::load_fns()?");
            }
        }
        GL.as_ref().unwrap_unchecked()
    }
}

pub unsafe fn viewport(x: i32, y: i32, width: i32, height: i32) {
    unsafe { gl().Viewport(x, y, width, height) }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Capability {
    DepthTest = gl46::GL_DEPTH_TEST.0,
    Blend = gl46::GL_BLEND.0,
    CullFace = gl46::GL_CULL_FACE.0,
    DebugOutput = gl46::GL_DEBUG_OUTPUT.0,
    DebugOutputSynchronous = gl46::GL_DEBUG_OUTPUT_SYNCHRONOUS.0,
}

impl From<Capability> for u32 {
    fn from(value: Capability) -> Self {
        value as u32
    }
}

impl From<u32> for Capability {
    fn from(value: u32) -> Self {
        match value {
            x if x == gl46::GL_DEPTH_TEST.0 => Capability::DepthTest,
            x if x == gl46::GL_BLEND.0 => Capability::Blend,
            x if x == gl46::GL_CULL_FACE.0 => Capability::CullFace,
            _ => unimplemented!("Unsupported capability: {}", value),
        }
    }
}

pub fn enable(capability: Capability) {
    unsafe {
        gl().Enable(gl46::GLenum(capability.into()));
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClearMask {
    ColorBufferBit = gl46::GL_COLOR_BUFFER_BIT.0,
    DepthBufferBit = gl46::GL_DEPTH_BUFFER_BIT.0,
    StencilBufferBit = gl46::GL_STENCIL_BUFFER_BIT.0,
}

impl From<ClearMask> for u32 {
    fn from(value: ClearMask) -> Self {
        value as u32
    }
}

impl From<u32> for ClearMask {
    fn from(value: u32) -> Self {
        match value {
            x if x == gl46::GL_COLOR_BUFFER_BIT.0 => ClearMask::ColorBufferBit,
            x if x == gl46::GL_DEPTH_BUFFER_BIT.0 => ClearMask::DepthBufferBit,
            x if x == gl46::GL_STENCIL_BUFFER_BIT.0 => ClearMask::StencilBufferBit,
            _ => unimplemented!("Unsupported clear mask: {}", value),
        }
    }
}

pub fn clear(mask: ClearMask) {
    unsafe {
        gl().Clear(gl46::GLbitfield(mask.into()));
    }
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum DrawMode {
    Points = gl46::GL_POINTS.0,
    Lines = gl46::GL_LINES.0,
    LineStrip = gl46::GL_LINE_STRIP.0,
    Triangles = gl46::GL_TRIANGLES.0,
    TriangleStrip = gl46::GL_TRIANGLE_STRIP.0,
    TriangleFan = gl46::GL_TRIANGLE_FAN.0,
}

pub fn draw_elements(mode: DrawMode, count: i32, r#type: VertexAttribPointerType, offset: usize) {
    unsafe {
        gl().DrawElements(
            gl46::GLenum(mode as u32),
            count,
            gl46::GLenum(r#type.into()),
            offset as *const c_void,
        );
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendFactor {
    Zero = gl46::GL_ZERO.0,
    One = gl46::GL_ONE.0,
    SrcAlpha = gl46::GL_SRC_ALPHA.0,
    OneMinusSrcAlpha = gl46::GL_ONE_MINUS_SRC_ALPHA.0,
}

impl From<BlendFactor> for u32 {
    fn from(value: BlendFactor) -> Self {
        value as u32
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendEquation {
    FuncAdd = gl46::GL_FUNC_ADD.0,
    FuncSubtract = gl46::GL_FUNC_SUBTRACT.0,
    FuncReverseSubtract = gl46::GL_FUNC_REVERSE_SUBTRACT.0,
}

impl From<BlendEquation> for u32 {
    fn from(value: BlendEquation) -> Self {
        value as u32
    }
}

pub fn blend_func(sfactor: BlendFactor, dfactor: BlendFactor) {
    unsafe {
        gl().BlendFunc(gl46::GLenum(sfactor.into()), gl46::GLenum(dfactor.into()));
    }
}

pub fn clear_color(r: f32, g: f32, b: f32, a: f32) {
    unsafe {
        gl().ClearColor(r, g, b, a);
    }
}
