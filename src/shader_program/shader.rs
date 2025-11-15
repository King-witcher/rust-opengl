use crate::mygl;

pub struct Shader {
    shader: mygl::Shader,
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
    pub fn new(code: ShaderCode, shader_type: mygl::ShaderType) -> Result<Self, String> {
        match code {
            ShaderCode::GLSL(source) => Self::from_glsl(source, shader_type),
            ShaderCode::SPIRV(binary) => Self::from_spirv(binary, shader_type),
        }
    }

    pub fn from_glsl(source: &str, shader_type: mygl::ShaderType) -> Result<Self, String> {
        let mut shader = mygl::Shader::create(shader_type);
        shader.source(&[source]);
        shader.compile()?;
        Ok(Self { shader })
    }

    pub fn from_spirv(binary: &[u8], shader_type: mygl::ShaderType) -> Result<Self, String> {
        let mut shader = mygl::Shader::create(shader_type);
        shader.binary(binary);
        shader.specialize("main", &[])?;

        unimplemented!("SPIR-V shader creation is not implemented yet.");
    }

    pub fn shader(&self) -> &mygl::Shader {
        &self.shader
    }

    #[inline]
    pub fn id(&self) -> u32 {
        self.shader.id()
    }
}

impl From<mygl::Shader> for Shader {
    fn from(shader: mygl::Shader) -> Self {
        Self { shader }
    }
}

impl From<Shader> for mygl::Shader {
    fn from(shader: Shader) -> Self {
        shader.shader
    }
}
