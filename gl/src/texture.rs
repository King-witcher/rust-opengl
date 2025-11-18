use gl46::GLenum;

use super::gl;

pub struct Texture(u32);

#[repr(u32)]
pub enum TextureTarget {
    Texture1D = gl46::GL_TEXTURE_1D.0,
    Texture2D = gl46::GL_TEXTURE_2D.0,
    Texture3D = gl46::GL_TEXTURE_3D.0,
    TextureCubeMap = gl46::GL_TEXTURE_CUBE_MAP.0,
}

impl From<TextureTarget> for GLenum {
    #[inline]
    fn from(value: TextureTarget) -> GLenum {
        GLenum(value as _)
    }
}

impl Texture {
    #[inline]
    pub fn id(&self) -> u32 {
        self.0
    }

    #[inline]
    pub fn r#gen(count: isize) -> Vec<Self> {
        let mut textures = vec![0; count as usize];
        unsafe {
            gl().GenTextures(count as _, textures.as_mut_ptr());
        }
        textures.into_iter().map(Texture).collect()
    }

    #[inline]
    pub fn create(target: TextureTarget, count: isize) -> Vec<Self> {
        let mut textures = vec![0; count as usize];
        unsafe {
            gl().CreateTextures(target.into(), count as _, textures.as_mut_ptr());
        }
        textures.into_iter().map(Texture).collect()
    }

    #[inline]
    pub fn create1(target: TextureTarget) -> Self {
        let mut texture = 0;
        unsafe {
            gl().CreateTextures(target.into(), 1, &mut texture);
        }
        Texture(texture)
    }

    #[inline]
    pub fn gen1() -> Self {
        let mut texture = 0;
        unsafe {
            gl().GenTextures(1, &mut texture);
        }
        Texture(texture)
    }

    #[inline]
    pub unsafe fn parameter_i(&mut self, pname: TextureParameter, param: i32) {
        unsafe {
            gl().TextureParameteri(self.id(), pname.into(), param);
        }
    }

    #[inline]
    pub fn parameter_i_wrap_r(&mut self, wrap_mode: TextureWrapMode) {
        unsafe {
            self.parameter_i(TextureParameter::TextureWrapR, wrap_mode as i32);
        }
    }

    #[inline]
    pub fn parameter_i_wrap_s(&mut self, wrap_mode: TextureWrapMode) {
        unsafe {
            self.parameter_i(TextureParameter::TextureWrapS, wrap_mode as i32);
        }
    }

    #[inline]
    pub fn parameter_i_wrap_t(&mut self, wrap_mode: TextureWrapMode) {
        unsafe {
            self.parameter_i(TextureParameter::TextureWrapT, wrap_mode as i32);
        }
    }

    #[inline]
    pub fn parameter_i_mag_filter(&mut self, filter_mode: InterpolationMode) {
        unsafe {
            self.parameter_i(TextureParameter::TextureMagFilter, filter_mode as i32);
        }
    }

    #[inline]
    pub fn parameter_i_min_filter(
        &mut self,
        texture_filter_mode: InterpolationMode,
        mipmap_interpolation: Option<InterpolationMode>,
    ) {
        unsafe {
            let filter_mode = match mipmap_interpolation {
                Some(InterpolationMode::Nearest) => match texture_filter_mode {
                    InterpolationMode::Nearest => gl46::GL_NEAREST_MIPMAP_NEAREST,
                    InterpolationMode::Linear => gl46::GL_LINEAR_MIPMAP_NEAREST,
                },
                Some(InterpolationMode::Linear) => match texture_filter_mode {
                    InterpolationMode::Nearest => gl46::GL_NEAREST_MIPMAP_LINEAR,
                    InterpolationMode::Linear => gl46::GL_LINEAR_MIPMAP_LINEAR,
                },
                None => match texture_filter_mode {
                    InterpolationMode::Nearest => gl46::GL_NEAREST,
                    InterpolationMode::Linear => gl46::GL_LINEAR,
                },
            };

            self.parameter_i(TextureParameter::TextureMinFilter, filter_mode.0 as i32);
        }
    }

    #[inline]
    pub fn storage_2d(
        &mut self,
        levels: i32,
        internal_format: BaseInternalFormat,
        width: i32,
        height: i32,
    ) {
        unsafe {
            gl().TextureStorage2D(self.id(), levels, internal_format.into(), width, height);
        }
    }

    #[inline]
    pub fn sub_image_2d<T>(
        &mut self,
        level: i32,
        (xoffset, yoffset): (i32, i32),
        (width, height): (i32, i32),
        format: PixelDataFormat,
        r#type: PixelDataType,
        data: *const T,
    ) {
        unsafe {
            gl().TextureSubImage2D(
                self.id(),
                level,
                xoffset,
                yoffset,
                width,
                height,
                format.into(),
                r#type.into(),
                data.cast(),
            );
        }
    }

    #[inline]
    pub fn generate_mipmap(&mut self) {
        unsafe {
            gl().GenerateTextureMipmap(self.id());
        }
    }

    #[inline]
    pub fn bind(&self, target: TextureTarget) {
        unsafe {
            gl().BindTexture(target.into(), self.0);
        }
    }

    #[inline]
    pub fn bind_unit(&self, unit: u32) {
        unsafe {
            gl().BindTextureUnit(unit, self.0);
        }
    }
}

impl Drop for Texture {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            gl().DeleteTextures(1, &self.0);
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum TextureParameter {
    DepthStencilTextureMode = gl46::GL_DEPTH_STENCIL_TEXTURE_MODE.0,
    TextureBaseLevel = gl46::GL_TEXTURE_BASE_LEVEL.0,
    TextureCompareFunc = gl46::GL_TEXTURE_COMPARE_FUNC.0,
    TextureCompareMode = gl46::GL_TEXTURE_COMPARE_MODE.0,
    TextureLodBias = gl46::GL_TEXTURE_LOD_BIAS.0,
    TextureMinFilter = gl46::GL_TEXTURE_MIN_FILTER.0,
    TextureMagFilter = gl46::GL_TEXTURE_MAG_FILTER.0,
    TextureMinLod = gl46::GL_TEXTURE_MIN_LOD.0,
    TextureMaxLod = gl46::GL_TEXTURE_MAX_LOD.0,
    TextureMaxLevel = gl46::GL_TEXTURE_MAX_LEVEL.0,
    TextureSwizzleR = gl46::GL_TEXTURE_SWIZZLE_R.0,
    TextureSwizzleG = gl46::GL_TEXTURE_SWIZZLE_G.0,
    TextureSwizzleB = gl46::GL_TEXTURE_SWIZZLE_B.0,
    TextureSwizzleA = gl46::GL_TEXTURE_SWIZZLE_A.0,
    TextureWrapS = gl46::GL_TEXTURE_WRAP_S.0,
    TextureWrapT = gl46::GL_TEXTURE_WRAP_T.0,
    TextureWrapR = gl46::GL_TEXTURE_WRAP_R.0,
}

impl From<TextureParameter> for u32 {
    #[inline]
    fn from(value: TextureParameter) -> Self {
        value as u32
    }
}

impl From<TextureParameter> for GLenum {
    #[inline]
    fn from(value: TextureParameter) -> Self {
        GLenum(value as _)
    }
}

#[inline]
pub unsafe fn tex_parameter_i(target: TextureTarget, pname: TextureParameter, param: i32) {
    unsafe {
        gl().TexParameteri(target.into(), pname.into(), param.into());
    }
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum TextureWrapMode {
    ClampToEdge = gl46::GL_CLAMP_TO_EDGE.0,
    ClampToBorder = gl46::GL_CLAMP_TO_BORDER.0,
    MirroredRepeat = gl46::GL_MIRRORED_REPEAT.0,
    Repeat = gl46::GL_REPEAT.0,
    MirrorClampToEdge = gl46::GL_MIRROR_CLAMP_TO_EDGE.0,
}

#[inline]
pub fn tex_parameter_i_wrap_s(target: TextureTarget, param: TextureWrapMode) {
    unsafe {
        tex_parameter_i(target, TextureParameter::TextureWrapS, param as i32);
    }
}

#[inline]
pub fn tex_parameter_i_wrap_t(target: TextureTarget, param: TextureWrapMode) {
    unsafe {
        tex_parameter_i(target, TextureParameter::TextureWrapT, param as i32);
    }
}

#[inline]
pub fn tex_parameter_i_wrap_r(target: TextureTarget, param: TextureWrapMode) {
    unsafe {
        tex_parameter_i(target, TextureParameter::TextureWrapR, param as i32);
    }
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum InterpolationMode {
    Nearest = gl46::GL_NEAREST.0,
    Linear = gl46::GL_LINEAR.0,
}

#[inline]
pub fn tex_parameter_i_mag_filter(target: TextureTarget, filter_mode: InterpolationMode) {
    unsafe {
        tex_parameter_i(
            target,
            TextureParameter::TextureMagFilter,
            filter_mode as i32,
        );
    }
}

#[inline]
pub fn tex_parameter_i_min_filter(
    target: TextureTarget,
    texture_filter_mode: InterpolationMode,
    mipmap_interpolation: Option<InterpolationMode>,
) {
    unsafe {
        let filter_mode = match mipmap_interpolation {
            Some(InterpolationMode::Nearest) => match texture_filter_mode {
                InterpolationMode::Nearest => gl46::GL_NEAREST_MIPMAP_NEAREST,
                InterpolationMode::Linear => gl46::GL_LINEAR_MIPMAP_NEAREST,
            },
            Some(InterpolationMode::Linear) => match texture_filter_mode {
                InterpolationMode::Nearest => gl46::GL_NEAREST_MIPMAP_LINEAR,
                InterpolationMode::Linear => gl46::GL_LINEAR_MIPMAP_LINEAR,
            },
            None => match texture_filter_mode {
                InterpolationMode::Nearest => gl46::GL_NEAREST,
                InterpolationMode::Linear => gl46::GL_LINEAR,
            },
        };

        tex_parameter_i(
            target,
            TextureParameter::TextureMinFilter,
            filter_mode.0 as i32,
        );
    }
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum PixelDataFormat {
    Red = gl46::GL_RED.0,
    RG = gl46::GL_RG.0,
    RGB = gl46::GL_RGB.0,
    BGR = gl46::GL_BGR.0,
    RGBA = gl46::GL_RGBA.0,
    BGRA = gl46::GL_BGRA.0,
    RedInteger = gl46::GL_RED_INTEGER.0,
    RGInteger = gl46::GL_RG_INTEGER.0,
    RGBInteger = gl46::GL_RGB_INTEGER.0,
    BGRInteger = gl46::GL_BGR_INTEGER.0,
    RGBAInteger = gl46::GL_RGBA_INTEGER.0,
    BGRAInteger = gl46::GL_BGRA_INTEGER.0,
    StencilIndex = gl46::GL_STENCIL_INDEX.0,
    DepthComponent = gl46::GL_DEPTH_COMPONENT.0,
    DepthStencil = gl46::GL_DEPTH_STENCIL.0,
}

impl From<PixelDataFormat> for GLenum {
    #[inline]
    fn from(value: PixelDataFormat) -> Self {
        GLenum(value as _)
    }
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum BaseInternalFormat {
    DepthComponent = gl46::GL_DEPTH_COMPONENT.0,
    DepthStencil = gl46::GL_DEPTH_STENCIL.0,
    Red = gl46::GL_RED.0,
    RG = gl46::GL_RG.0,
    RGB = gl46::GL_RGB.0,
    RGBA = gl46::GL_RGBA.0,
}

impl From<BaseInternalFormat> for GLenum {
    #[inline]
    fn from(value: BaseInternalFormat) -> Self {
        GLenum(value as _)
    }
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum PixelDataType {
    UnsignedByte = gl46::GL_UNSIGNED_BYTE.0,
    Byte = gl46::GL_BYTE.0,
    UnsignedShort = gl46::GL_UNSIGNED_SHORT.0,
    Short = gl46::GL_SHORT.0,
    UnsignedInt = gl46::GL_UNSIGNED_INT.0,
    Int = gl46::GL_INT.0,
    HalfFloat = gl46::GL_HALF_FLOAT.0,
    Float = gl46::GL_FLOAT.0,
    UnsignedByte332 = gl46::GL_UNSIGNED_BYTE_3_3_2.0,
    UnsignedByte233Rev = gl46::GL_UNSIGNED_BYTE_2_3_3_REV.0,
    UnsignedShort565 = gl46::GL_UNSIGNED_SHORT_5_6_5.0,
    UnsignedShort565Rev = gl46::GL_UNSIGNED_SHORT_5_6_5_REV.0,
    UnsignedShort4444 = gl46::GL_UNSIGNED_SHORT_4_4_4_4.0,
    UnsignedShort4444Rev = gl46::GL_UNSIGNED_SHORT_4_4_4_4_REV.0,
    UnsignedShort5551 = gl46::GL_UNSIGNED_SHORT_5_5_5_1.0,
    UnsignedShort1555Rev = gl46::GL_UNSIGNED_SHORT_1_5_5_5_REV.0,
    UnsignedInt8888 = gl46::GL_UNSIGNED_INT_8_8_8_8.0,
    UnsignedInt8888Rev = gl46::GL_UNSIGNED_INT_8_8_8_8_REV.0,
    UnsignedInt1010102 = gl46::GL_UNSIGNED_INT_10_10_10_2.0,
    UnsignedInt2101010Rev = gl46::GL_UNSIGNED_INT_2_10_10_10_REV.0,
}

impl From<PixelDataType> for GLenum {
    #[inline]
    fn from(value: PixelDataType) -> Self {
        GLenum(value as _)
    }
}

#[inline]
#[doc = "https://registry.khronos.org/OpenGL-Refpages/gl4/html/glTexImage2D.xhtml"]
pub unsafe fn tex_image_2d<T>(
    target: TextureTarget,
    level: i32,
    internal_format: BaseInternalFormat,
    width: i32,
    height: i32,
    format: PixelDataFormat,
    r#type: PixelDataType,
    data: *const T,
) {
    unsafe {
        gl().TexImage2D(
            target.into(),
            level,
            internal_format as i32,
            width,
            height,
            0,
            GLenum(format as u32),
            GLenum(r#type as u32),
            data.cast(),
        );
    }
}

#[inline]
pub fn generate_mipmap(target: TextureTarget) {
    unsafe {
        gl().GenerateMipmap(target.into());
    }
}

#[inline]
pub unsafe fn active_texture(texture_unit: u32) {
    unsafe {
        gl().ActiveTexture(gl46::GLenum(texture_unit));
    }
}

#[inline]
pub fn active_texture_gl_texture(offset: u32) {
    let texture_unit = gl46::GL_TEXTURE0.0 + offset;
    unsafe {
        active_texture(texture_unit);
    }
}
