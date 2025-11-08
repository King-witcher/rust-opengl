pub struct KWindow {
    sdl_context: sdl2::Sdl,
    _sdl_video: sdl2::VideoSubsystem,
    sdl_window: sdl2::video::Window,
    _gl_context: sdl2::video::GLContext,
}

pub struct KWindowCreateInfo<'s> {
    pub title: &'s str,
    pub width: u32,
    pub height: u32,
}

impl KWindow {
    pub fn new(create_info: KWindowCreateInfo) -> Self {
        let sdl_context = sdl2::init().expect("Failed to initialize SDL2");
        let _sdl_video = sdl_context
            .video()
            .expect("Failed to get SDL2 video subsystem");

        let gl_attr = _sdl_video.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4, 6);

        let sdl_window = _sdl_video
            .window(create_info.title, create_info.width, create_info.height)
            .position(-1800, 100)
            .fullscreen_desktop()
            .opengl()
            .build()
            .expect("Failed to create SDL2 window");

        let _gl_context = sdl_window
            .gl_create_context()
            .expect("Failed to create OpenGL context");

        _sdl_video
            .gl_set_swap_interval(0)
            .expect("Failed to disable vsync");

        KWindow {
            sdl_context,
            _sdl_video,
            sdl_window,
            _gl_context,
        }
    }

    pub fn get_proc_address(&self, s: &str) -> *const std::ffi::c_void {
        self._sdl_video.gl_get_proc_address(s) as _
    }

    pub fn event_pump(&self) -> sdl2::EventPump {
        self.sdl_context
            .event_pump()
            .expect("Failed to get SDL2 event pump")
    }

    pub fn swap_window(&self) {
        self.sdl_window.gl_swap_window();
    }

    pub fn set_relative_mouse_mode(&self, enabled: bool) {
        self.sdl_context.mouse().set_relative_mouse_mode(enabled);
    }
}
