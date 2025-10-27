pub struct KWindow {
    sdl_context: sdl2::Sdl,
    sdl_video: sdl2::VideoSubsystem,
    sdl_window: sdl2::video::Window,
    gl_context: sdl2::video::GLContext,
}

pub struct KWindowCreateInfo<'s> {
    pub title: &'s str,
    pub width: u32,
    pub height: u32,
}

impl KWindow {
    pub fn new(create_info: KWindowCreateInfo) -> Self {
        let sdl_context = sdl2::init().expect("Failed to initialize SDL2");
        let sdl_video = sdl_context
            .video()
            .expect("Failed to get SDL2 video subsystem");

        let gl_attr = sdl_video.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4, 6);

        let sdl_window = sdl_video
            .window(create_info.title, create_info.width, create_info.height)
            .opengl()
            .build()
            .expect("Failed to create SDL2 window");

        let gl_context = sdl_window
            .gl_create_context()
            .expect("Failed to create OpenGL context");

        gl::load_with(|s| sdl_video.gl_get_proc_address(s) as _);

        KWindow {
            sdl_context,
            sdl_video,
            sdl_window,
            gl_context,
        }
    }

    pub fn event_pump(&self) -> sdl2::EventPump {
        self.sdl_context
            .event_pump()
            .expect("Failed to get SDL2 event pump")
    }

    pub fn swap_window(&self) {
        self.sdl_window.gl_swap_window();
    }
}
