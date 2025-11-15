extern crate anyhow;
extern crate gl46;
extern crate sdl2;

mod archive;
mod engine;
mod mygl;
mod scene;
mod shader_program;
mod texture;
mod window;

pub use texture::*;

fn main() {
    let mut engine = engine::KEngine::new(1920, 1080, "Rust OpenGL Window").unwrap();
    engine.run();
}
