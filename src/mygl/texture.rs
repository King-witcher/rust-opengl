use gl46::GLenum;

use super::gl;

pub struct Texture(u32);

#[repr(u32)]
pub enum TextureTarget {
    Texture2D = gl46::GL_TEXTURE_2D.0,
    TextureCubeMap = gl46::GL_TEXTURE_CUBE_MAP.0,
}

impl From<TextureTarget> for GLenum {
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
    pub fn gen1() -> Self {
        let mut texture = 0;
        unsafe {
            gl().GenTextures(1, &mut texture);
        }
        Texture(texture)
    }

    #[inline]
    pub fn bind(&self, target: TextureTarget) {
        unsafe {
            gl().BindTexture(target.into(), self.0);
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl().DeleteTextures(1, &self.0);
        }
    }
}
