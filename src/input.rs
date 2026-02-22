use std::collections::HashMap;
use sdl2::keyboard::Scancode;
use std::ops::Index;

struct Input {
    keys: HashMap<Scancode, bool>
}

impl Input {
    pub fn new() -> Self {
        let mut keys = HashMap::new();
        
        for key in Scancode::iter() {
            keys.insert(key, false);
        }
        
        Self { keys }
    }
    pub fn is_down(&self, key: Scancode) -> bool {
        self.pressed.contains(&key)
    }
}

impl Index<Scancode> for Input {
    type Output = bool;

    fn index(&self, key: Scancode) -> &Self::Output {
        &self.keys[&key]
    }
}