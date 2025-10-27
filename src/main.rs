use crate::window::KWindow;

extern crate gl;
extern crate sdl2;

mod engine;
mod window;

fn main() {
    let window_create_info = window::KWindowCreateInfo {
        title: "KEngine Window",
        width: 800,
        height: 600,
    };

    let kwindow = KWindow::new(window_create_info);

    let mut event_pump = kwindow.event_pump();

    'main_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main_loop,
                _ => {}
            }
        }

        kwindow.swap_window();
    }
}
