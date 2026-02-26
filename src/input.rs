use std::collections::{HashMap, HashSet};
use sdl2::rect::Point;
use sdl2::{ Sdl, event::Event, keyboard::Scancode };
use std::ops::Index;
use sdl2::mouse::MouseUtil;
use sdl2::video::Window;

pub struct Input {
    pressed: HashSet<Scancode>,
    mouse_util: MouseUtil,
    mouse_delta: (i32, i32),
}

impl Input {
    pub fn new(sdl_context: Sdl) -> Self {
        let mouse_util = sdl_context.mouse();
        Self {
            pressed: HashSet::new(),
            mouse_util,
            mouse_delta: (0, 0),
        }
    }

    pub fn press(&mut self, key: Scancode) {
        self.pressed.insert(key);
    }

    pub fn release(&mut self, key: Scancode) {
        self.pressed.remove(&key);
    }

    pub fn is_down(&self, key: Scancode) -> bool {
        self.pressed.contains(&key)
    }

    pub fn mouse_buttons_down(&self, event_pump: &mut sdl2::EventPump) -> HashMap<&str, bool> {
        let mut pressed = HashMap::new();
        pressed.insert("left", event_pump.mouse_state().left());
        pressed.insert("middle", event_pump.mouse_state().middle());
        pressed.insert("right", event_pump.mouse_state().right());
        pressed.insert("x1", event_pump.mouse_state().x1());
        pressed.insert("x2", event_pump.mouse_state().x2());
        pressed
    }

    pub fn get_mouse_pos(&self, event_pump: &mut sdl2::EventPump) -> (i32, i32) {
        (event_pump.mouse_state().x(), event_pump.mouse_state().y())
    }

    pub fn set_relative_mouse_position(&self) {
        self.mouse_util.set_relative_mouse_mode(true);
    }

    pub fn set_mouse_visibility(&self, visibility: bool) {
        self.mouse_util.show_cursor(visibility);
    }

    pub fn set_mouse_position(&self, position: crate::shape::point::Point, window: &Window) {
        self.mouse_util.warp_mouse_in_window(window, position.return_xy().0 as i32, position.return_xy().1 as i32);
    }

    pub fn get_mouse_delta(&self) -> (i32, i32) {
        self.mouse_delta
    }

    pub fn update_yaw_and_pitch(&self, sensitivity: f32) -> (i32, i32) {
        let yaw = self.mouse_delta.0 as f32 * sensitivity;
        let pitch = self.mouse_delta.1 as f32 * sensitivity;
        (yaw as i32, pitch as i32)
    }

    pub fn update(&mut self, event_pump: &mut sdl2::EventPump) -> bool {
        self.mouse_delta = (0, 0);
        let mut running: bool = true;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    running = false
                },
                Event::KeyDown { scancode: Some(key), .. } => {
                    self.press(key);
                },
                Event::KeyUp { scancode: Some(key), .. } => {
                    self.release(key);
                },
                Event::MouseMotion { xrel, yrel, .. } => {
                    self.mouse_delta = (xrel, yrel);
                },
                _ => {},
            }
        }
        running
    }

}

impl Index<Scancode> for Input {
    type Output = bool;

    fn index(&self, key: Scancode) -> &Self::Output {
        if self.pressed.contains(&key) {
            &true
        } else {
            &false
        }
    }
}

