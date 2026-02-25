use std::collections::{HashMap, HashSet};
use sdl2::{ event::Event, keyboard::Scancode};
use std::ops::Index;

pub struct Input {
    pressed: HashSet<Scancode>,
}

impl Input {
    pub fn new() -> Self {
        Self {
            pressed: HashSet::new(),
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

    pub fn update(&mut self, event_pump: &mut sdl2::EventPump) -> bool {
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

