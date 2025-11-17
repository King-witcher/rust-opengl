use std::ffi::c_void;

use super::gl;
pub struct VertexArray(u32);

impl VertexArray {
    #[inline]
    pub fn new() -> Self {
        Self::create1()
    }

    #[inline]
    pub fn r#gen(n: isize) -> Vec<Self> {
        let mut arrays = vec![0; n as usize];
        unsafe {
            gl().GenVertexArrays(n as _, arrays.as_mut_ptr());
        }
        arrays.into_iter().map(VertexArray).collect()
    }

    #[inline]
    pub fn gen1() -> Self {
        let mut array = 0;
        unsafe {
            gl().GenVertexArrays(1, &mut array);
        }
        VertexArray(array)
    }

    #[inline]
    pub fn create(n: isize) -> Vec<Self> {
        let mut arrays = vec![0; n as usize];
        unsafe {
            gl().CreateVertexArrays(n as _, arrays.as_mut_ptr());
        }
        arrays.into_iter().map(VertexArray).collect()
    }

    #[inline]
    pub fn create1() -> Self {
        let mut array = 0;
        unsafe {
            gl().CreateVertexArrays(1, &mut array);
        }
        VertexArray(array)
    }

    #[inline]
    pub fn id(&self) -> u32 {
        self.0
    }

    #[inline]
    pub fn bind(&self) {
        gl().BindVertexArray(self.id());
    }

    #[inline]
    pub fn enable_attrib(&mut self, index: u32) {
        unsafe {
            gl().EnableVertexArrayAttrib(self.id(), index);
        }
    }

    #[inline]
    pub fn disable_attrib(&self, index: u32) {
        unsafe {
            gl().DisableVertexArrayAttrib(self.id(), index);
        }
    }

    #[inline]
    pub fn delete(self) {}
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl().DeleteVertexArrays(1, &self.0);
        }
    }
}

pub fn enable_vertex_attrib_array(index: u32) {
    unsafe {
        gl().EnableVertexAttribArray(index);
    }
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum VertexAttribPointerType {
    Float = gl46::GL_FLOAT.0,
    Double = gl46::GL_DOUBLE.0,
    Int = gl46::GL_INT.0,
    UnsignedInt = gl46::GL_UNSIGNED_INT.0,
    Short = gl46::GL_SHORT.0,
    UnsignedShort = gl46::GL_UNSIGNED_SHORT.0,
    Byte = gl46::GL_BYTE.0,
    UnsignedByte = gl46::GL_UNSIGNED_BYTE.0,
}

impl From<VertexAttribPointerType> for u32 {
    fn from(value: VertexAttribPointerType) -> Self {
        value as u32
    }
}

pub unsafe fn vertex_attrib_pointer(
    location: u32,
    components: i32,
    r#type: VertexAttribPointerType,
    normalized: bool,
    stride: usize,
    offset: usize,
) {
    if components < 1 || components > 4 {
        panic!("components must be between 1 and 4");
    }
    unsafe {
        gl().VertexAttribPointer(
            location,
            components,
            gl46::GLenum(r#type.into()),
            if normalized { 0 } else { 1 },
            stride as i32,
            offset as *const c_void,
        );
    }
}
