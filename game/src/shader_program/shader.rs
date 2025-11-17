use gl;

pub struct Shader {
    shader: gl::Shader,
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum ShaderSourceType {
    GLSL,
    SPIRV,
}

pub enum ShaderCode<'a> {
    GLSL(&'a str),
    SPIRV(&'a [u8]),
}

impl Shader {
    pub fn new(code: ShaderCode, shader_type: gl::ShaderType) -> Result<Self, String> {
        match code {
            ShaderCode::GLSL(source) => Self::from_glsl(source, shader_type),
            ShaderCode::SPIRV(binary) => Self::from_spirv(binary, shader_type),
        }
    }

    pub fn from_glsl(source: &str, shader_type: gl::ShaderType) -> Result<Self, String> {
        let mut shader = gl::Shader::create(shader_type);
        shader.source(&[source]);
        shader.compile()?;

        Ok(Self { shader })
    }

    pub fn from_spirv(binary: &[u8], shader_type: gl::ShaderType) -> Result<Self, String> {
        let mut shader = gl::Shader::create(shader_type);
        shader.binary(binary);
        shader.specialize("main", &[])?;

        Ok(Self { shader })
    }

    pub fn shader(&self) -> &gl::Shader {
        &self.shader
    }

    #[inline]
    pub fn id(&self) -> u32 {
        self.shader.id()
    }
}

impl From<gl::Shader> for Shader {
    fn from(shader: gl::Shader) -> Self {
        Self { shader }
    }
}

impl From<Shader> for gl::Shader {
    fn from(shader: Shader) -> Self {
        shader.shader
    }
}
