use super::gl;
pub struct Buffer(u32);

#[repr(u32)]
pub enum BufferTarget {
    ArrayBuffer = gl46::GL_ARRAY_BUFFER.0,
    ElementArrayBuffer = gl46::GL_ELEMENT_ARRAY_BUFFER.0,
}

impl From<BufferTarget> for u32 {
    fn from(target: BufferTarget) -> Self {
        target as u32
    }
}

impl From<u32> for BufferTarget {
    fn from(value: u32) -> Self {
        match value {
            x if x == gl46::GL_ARRAY_BUFFER.0 => BufferTarget::ArrayBuffer,
            x if x == gl46::GL_ELEMENT_ARRAY_BUFFER.0 => BufferTarget::ElementArrayBuffer,
            _ => unimplemented!("Unsupported buffer target: {}", value),
        }
    }
}

impl Buffer {
    #[inline]
    pub fn r#gen(n: isize) -> Vec<Self> {
        let mut buffers = vec![0; n as usize];
        unsafe {
            gl().GenBuffers(n as _, buffers.as_mut_ptr());
        }
        buffers.into_iter().map(Buffer).collect()
    }

    #[inline]
    pub fn gen1() -> Self {
        let mut buffer = 0;
        unsafe {
            gl().GenBuffers(1, &mut buffer);
        }
        Buffer(buffer)
    }

    #[inline]
    pub fn id(&self) -> u32 {
        self.0
    }

    #[inline]
    pub fn bind(&self, target: BufferTarget) {
        unsafe {
            gl().BindBuffer(gl46::GLenum(target.into()), self.id());
        }
    }

    #[inline]
    pub fn storage<T>(&mut self, data: Vec<T>, flags: BufferUsage) {
        let size = data.len() * size_of::<T>();
        let data_ptr = data.as_ptr().cast();
        unsafe {
            gl().NamedBufferStorage(
                self.id(),
                size as isize,
                data_ptr,
                gl46::GLbitfield(flags as u32),
            );
        }
    }

    #[inline]
    pub fn delete(self) {}
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            gl().DeleteBuffers(1, &self.0);
        }
    }
}

#[repr(u32)]
pub enum BufferUsage {
    StaticDraw = gl46::GL_STATIC_DRAW.0,
    StreamDraw = gl46::GL_STREAM_DRAW.0,
    DynamicDraw = gl46::GL_DYNAMIC_DRAW.0,
}

impl From<BufferUsage> for u32 {
    fn from(usage: BufferUsage) -> Self {
        usage as u32
    }
}

pub fn buffer_data<T>(target: BufferTarget, data: Vec<T>, usage: BufferUsage) {
    unsafe {
        gl().BufferData(
            gl46::GLenum(target.into()),
            (data.len() * size_of::<T>()) as isize,
            data.as_ptr().cast(),
            gl46::GLenum(usage.into()),
        );
    }
}
