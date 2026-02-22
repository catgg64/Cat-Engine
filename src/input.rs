use std::collections::HashSet;
use sdl2::{EventPump, event::Event, keyboard::Scancode};
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

    pub fn update(&mut self, mut event_pump: EventPump) -> bool {
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