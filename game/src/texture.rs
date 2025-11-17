use gl::{self, TextureTarget};

use image::RgbaImage;

pub struct Texture {
    texture: gl::Texture,
}

pub struct TextureCreateInfo {
    pub rgba_image: RgbaImage,
    pub internal_format: gl::BaseInternalFormat,
    pub mip_level: i32,
    pub wrap_s: gl::TextureWrapMode,
    pub wrap_t: gl::TextureWrapMode,
    pub min_filter: gl::InterpolationMode,
    pub mag_filter: gl::InterpolationMode,
    pub mipmap_interpolation: Option<gl::InterpolationMode>,
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

        let mut texture = gl::Texture::gen1();
        texture.bind(TextureTarget::Texture2D);
        unsafe { gl::active_texture(0) };
        texture.parameter_i_wrap_s(wrap_s);
        texture.parameter_i_wrap_t(wrap_t);
        texture.parameter_i_mag_filter(mag_filter);
        texture.parameter_i_min_filter(min_filter, mipmap_interpolation);

        unsafe {
            gl::tex_image_2d(
                TextureTarget::Texture2D,
                mip_level,
                internal_format,
                rgba_image.width() as i32,
                rgba_image.height() as i32,
                gl::PixelDataFormat::RGBA,
                gl::PixelDataType::UnsignedByte,
                rgba_image.as_ptr(),
            )
        }

        texture.generate_mipmap();

        Texture { texture }
    }
}

impl Texture {
    pub fn bind_to_unit(&self, unit: u32) {
        self.texture.bind_texture_unit(unit);
    }

    pub fn id(&self) -> u32 {
        self.texture.id()
    }
}
