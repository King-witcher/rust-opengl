use std::rc::Rc;

use gl::*;
use image::{EncodableLayout, RgbaImage};

pub struct Texture {
    gl: Rc<GlFns>,
    width: i32,
    height: i32,
    texture_id: u32,
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
    pub gl: Rc<GlFns>,
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
        unsafe {
            let TextureCreateInfo {
                gl,
                rgba_image,
                internal_format,
                mip_level,
                wrap_s,
                wrap_t,
                min_filter,
                mag_filter,
                mipmap_interpolation,
            } = info;

            let mut texture_id = 0;
            gl.GenTextures(1, &mut texture_id);
            gl.BindTexture(GL_TEXTURE_2D, texture_id);

            gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, wrap_s as i32);
            gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, wrap_t as i32);
            gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, mag_filter as i32);

            match mipmap_interpolation {
                Some(FilterMode::Nearest) => {
                    if matches!(min_filter, FilterMode::Nearest) {
                        gl.TexParameteri(
                            GL_TEXTURE_2D,
                            GL_TEXTURE_MIN_FILTER,
                            GL_NEAREST_MIPMAP_NEAREST.0 as i32,
                        );
                    } else {
                        gl.TexParameteri(
                            GL_TEXTURE_2D,
                            GL_TEXTURE_MIN_FILTER,
                            GL_LINEAR_MIPMAP_NEAREST.0 as i32,
                        );
                    }
                }
                Some(FilterMode::Linear) => {
                    if matches!(min_filter, FilterMode::Nearest) {
                        gl.TexParameteri(
                            GL_TEXTURE_2D,
                            GL_TEXTURE_MIN_FILTER,
                            GL_NEAREST_MIPMAP_LINEAR.0 as i32,
                        );
                    } else {
                        gl.TexParameteri(
                            GL_TEXTURE_2D,
                            GL_TEXTURE_MIN_FILTER,
                            GL_LINEAR_MIPMAP_LINEAR.0 as i32,
                        );
                    }
                }
                None => {
                    gl.TexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, min_filter as i32);
                }
            }

            gl.TexImage2D(
                GL_TEXTURE_2D,
                mip_level,
                internal_format as _,
                rgba_image.width() as _,
                rgba_image.height() as _,
                0,
                GL_RGBA,
                GL_UNSIGNED_BYTE,
                rgba_image.as_bytes().as_ptr() as _,
            );

            gl.GenerateMipmap(GL_TEXTURE_2D);

            Texture {
                gl,
                width: rgba_image.width() as _,
                height: rgba_image.height() as _,
                texture_id,
            }
        }
    }
}

impl Texture {
    pub fn bind(&self, unit: u32) {
        unsafe {
            let texture_unit = GL_TEXTURE0.0 + unit;
            let texture_unit = GLenum(texture_unit);
            self.gl.ActiveTexture(texture_unit);
            self.gl.BindTexture(GL_TEXTURE_2D, self.texture_id);
        }
    }

    pub fn get(&self) -> u32 {
        self.texture_id
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteTextures(1, &self.texture_id);
        }
    }
}
