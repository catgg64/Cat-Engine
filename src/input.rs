use std::collections::{HashMap, HashSet};
use sdl2::{ Sdl, event::Event, keyboard::Scancode };
use std::ops::Index;
use sdl2::mouse::MouseUtil;
use sdl2::video::Window;

use crate::math;
use crate::video::Renderer;

pub struct Input {
    pressed: HashSet<Scancode>,
    mouse_util: MouseUtil,
    mouse_delta: (i32, i32),
}

impl Input {
    pub fn new(sdl_context: &Sdl) -> Self {
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

    /// Checks if a key is down. Should be used with the keyboard module.
    pub fn is_down(&self, key: Scancode) -> bool {
        self.pressed.contains(&key)
    }

    /// Returns a HashMap with the buttons and if they are pressed or not.
    /// The values are: "left", "middle", "right", "x1" and "x2".
    pub fn mouse_buttons_down(&self, event_pump: &mut sdl2::EventPump) -> HashMap<&str, bool> {
        let mut pressed = HashMap::new();
        pressed.insert("left", event_pump.mouse_state().left());
        pressed.insert("middle", event_pump.mouse_state().middle());
        pressed.insert("right", event_pump.mouse_state().right());
        pressed.insert("x1", event_pump.mouse_state().x1());
        pressed.insert("x2", event_pump.mouse_state().x2());
        pressed
    }

    /// Gets the mouse's position
    pub fn get_mouse_pos(&self, event_pump: &mut sdl2::EventPump) -> (i32, i32) {
        (event_pump.mouse_state().x(), event_pump.mouse_state().y())
    }

    /// Sets a relative mouse position, often used for 3D games to control pitch and yaw.
    pub fn set_relative_mouse_position(&self) {
        self.mouse_util.set_relative_mouse_mode(true);
    }

    pub fn set_mouse_visibility(&self, visibility: bool) {
        self.mouse_util.show_cursor(visibility);
    }

    pub fn set_mouse_position(&self, position: math::Coordinate2D, window: &Window) {
        self.mouse_util.warp_mouse_in_window(window, position.0 as i32, position.1 as i32);
    }

    /// Gets the mouse's position after the last tick. Often combined with set_relative_mouse_position.
    pub fn get_mouse_delta(&self) -> (i32, i32) {
        self.mouse_delta
    }

    /// Updates the Yaw and Pitch. Used in 3D games.
    pub fn update_yaw_and_pitch(
        &self,
        sensitivity: f32,
        yaw: &mut f64,
        pitch: &mut f64,
    ) {
        *yaw -= (self.mouse_delta.0 as f64 * sensitivity as f64).to_radians();
        *pitch -= (self.mouse_delta.1 as f64 * sensitivity as f64).to_radians();
        // Optional: clamp pitch so camera doesn’t flip
        *pitch = pitch.clamp(-1.570, 1.570); // ±89 degrees in radians
    }
    
    pub fn update(&mut self, event_pump: &mut sdl2::EventPump, renderer: &mut Renderer) -> bool {
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
                sdl2::event::Event::Window { win_event, .. } => {
                    match win_event {
                        sdl2::event::WindowEvent::Resized(w, h) |
                        sdl2::event::WindowEvent::SizeChanged(w, h) => {
                            unsafe {
                                gl::Viewport(0, 0, w, h);
                            }
                            let projection = glam::Mat4::perspective_rh_gl(renderer.fov.to_radians(), w as f32 / h as f32, renderer.near_plane, renderer.far_plane);
                            renderer.true_set_projection(projection);
                        }
                        _ => {}

                    }
                }
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