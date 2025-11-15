use crate::mygl;

use gl46::*;
use image::{EncodableLayout, RgbaImage};

pub struct Texture {
    width: i32,
    height: i32,
    texture: mygl::Texture,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum TextureFormat {
    RGB = GL_RGB.0,
    RGBA = GL_RGBA.0,
    ARGB = GL_RGB8.0,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum WrapMode {
    Repeat = GL_REPEAT.0,
    MirroredRepeat = GL_MIRRORED_REPEAT.0,
    ClampToEdge = GL_CLAMP_TO_EDGE.0,
    ClampToBorder = GL_CLAMP_TO_BORDER.0,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum FilterMode {
    Nearest = GL_NEAREST.0,
    Linear = GL_LINEAR.0,
}

pub struct TextureCreateInfo {
    pub rgba_image: RgbaImage,
    pub internal_format: TextureFormat,
    pub mip_level: i32,
    pub wrap_s: WrapMode,
    pub wrap_t: WrapMode,
    pub min_filter: FilterMode,
    pub mag_filter: FilterMode,
    pub mipmap_interpolation: Option<FilterMode>,
}

impl From<TextureCreateInfo> for Texture {
    fn from(info: TextureCreateInfo) -> Self {
        let TextureCreateInfo {
            rgba_image,
            internal_format,
            mip_level,
            wrap_s,
            wrap_t,
            min_filter,
            mag_filter,
            mipmap_interpolation,
        } = info;

        const TARGET: mygl::TextureTarget = mygl::TextureTarget::Texture2D;

        let texture = mygl::Texture::gen1();
        texture.bind(TARGET);
        mygl::tex_parameter_i(TARGET, GL_TEXTURE_WRAP_S.0, wrap_s as i32);
        mygl::tex_parameter_i(TARGET, GL_TEXTURE_WRAP_S.0, wrap_s as i32);
        mygl::tex_parameter_i(TARGET, GL_TEXTURE_WRAP_T.0, wrap_t as i32);
        mygl::tex_parameter_i(TARGET, GL_TEXTURE_MAG_FILTER.0, mag_filter as i32);

        match mipmap_interpolation {
            Some(FilterMode::Nearest) => {
                if matches!(min_filter, FilterMode::Nearest) {
                    mygl::tex_parameter_i(
                        TARGET,
                        GL_TEXTURE_MIN_FILTER.0,
                        GL_NEAREST_MIPMAP_NEAREST.0 as i32,
                    );
                } else {
                    mygl::tex_parameter_i(
                        TARGET,
                        GL_TEXTURE_MIN_FILTER.0,
                        GL_LINEAR_MIPMAP_NEAREST.0 as i32,
                    );
                }
            }
            Some(FilterMode::Linear) => {
                if matches!(min_filter, FilterMode::Nearest) {
                    mygl::tex_parameter_i(
                        TARGET,
                        GL_TEXTURE_MIN_FILTER.0,
                        GL_NEAREST_MIPMAP_LINEAR.0 as i32,
                    );
                } else {
                    mygl::tex_parameter_i(
                        TARGET,
                        GL_TEXTURE_MIN_FILTER.0,
                        GL_LINEAR_MIPMAP_LINEAR.0 as i32,
                    );
                }
            }
            None => {
                mygl::tex_parameter_i(TARGET, GL_TEXTURE_MIN_FILTER.0, min_filter as i32);
            }
        }

        mygl::tex_image_2d(
            TARGET,
            mip_level,
            internal_format as i32,
            rgba_image.width() as _,
            rgba_image.height() as _,
            GL_RGBA.0,
            GL_UNSIGNED_BYTE.0,
            rgba_image.as_bytes(),
        );

        mygl::generate_mipmap(TARGET);

        Texture {
            width: rgba_image.width() as _,
            height: rgba_image.height() as _,
            texture,
        }
    }
}

impl Texture {
    pub fn bind_to_unit(&self, unit: u32) {
        let texture_unit = GL_TEXTURE0.0 + unit;
        let texture_unit = GLenum(texture_unit);
        mygl::active_texture(texture_unit.0);
        self.texture.bind(mygl::TextureTarget::Texture2D);
    }

    pub fn id(&self) -> u32 {
        self.texture.id()
    }
}
