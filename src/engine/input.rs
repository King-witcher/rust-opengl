pub use sdl2::event::Event;
pub use sdl2::keyboard::Scancode;
pub use sdl2::mouse::MouseButton;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Input {
    keys_down: [bool; 512],
    keys_pressed: [bool; 512],
    mouse_down: u8,
    mouse_pressed: u8,
    mouse_pos: (i32, i32),
    mouse_rel: (i32, i32),
    events: Rc<[Event]>,
    pub(crate) exit: bool,
}

impl Input {
    pub(crate) fn new() -> Self {
        Input {
            keys_down: [false; 512],
            keys_pressed: [false; 512],
            mouse_down: 0,
            mouse_pressed: 0,
            mouse_pos: (0, 0),
            mouse_rel: (0, 0),
            events: Rc::new([]),
            exit: false,
        }
    }

    pub(crate) fn update(&mut self, events: impl Iterator<Item = Event>) {
        self.mouse_rel = (0, 0);
        self.keys_pressed = [false; 512];
        self.mouse_pressed = 0;

        let events = events
            .inspect(|event| match event {
                Event::Quit { .. } => self.exit = true,
                Event::MouseMotion {
                    x, y, xrel, yrel, ..
                } => {
                    self.mouse_pos = (*x, *y);
                    self.mouse_rel = (self.mouse_rel.0 + *xrel, self.mouse_rel.1 + *yrel);
                }
                Event::KeyDown {
                    scancode: Some(scancode),
                    ..
                } => {
                    self.keys_pressed[*scancode as usize] = true;
                    self.keys_down[*scancode as usize] = true;
                }
                Event::KeyUp {
                    scancode: Some(scancode),
                    ..
                } => {
                    self.keys_down[*scancode as usize] = false;
                }
                Event::MouseButtonDown { mouse_btn, .. } => {
                    self.mouse_pressed |= 1 << (*mouse_btn as u8);
                    self.mouse_down |= 1 << (*mouse_btn as u8);
                }
                Event::MouseButtonUp { mouse_btn, .. } => {
                    self.mouse_down &= !(1 << (*mouse_btn as u8));
                }
                _ => {}
            })
            .collect();
        self.events = events;
    }
}

impl Input {
    pub fn is_key_down(&self, scancode: Scancode) -> bool {
        self.keys_down[scancode as usize]
    }

    pub fn was_key_pressed(&self, scancode: Scancode) -> bool {
        self.keys_pressed[scancode as usize]
    }

    pub fn is_mouse_down(&self, button: MouseButton) -> bool {
        (self.mouse_down & (1 << (button as u8))) != 0
    }

    pub fn was_mouse_pressed(&self, button: MouseButton) -> bool {
        (self.mouse_pressed & (1 << (button as u8))) != 0
    }

    pub fn mouse_pos(&self) -> (i32, i32) {
        self.mouse_pos
    }

    pub fn mouse_rel(&self) -> (i32, i32) {
        self.mouse_rel
    }

    pub fn events(&self) -> impl Iterator<Item = &Event> {
        self.events.iter()
    }
}
