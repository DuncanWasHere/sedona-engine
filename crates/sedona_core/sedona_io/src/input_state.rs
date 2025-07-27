use std::collections::HashSet;
use std::time::Instant;
use winit::keyboard::KeyCode;

#[derive(Debug)]
pub struct InputState {
    pub keys_held: HashSet<KeyCode>,
    mouse_delta: (f32, f32),
    start_time: Instant,
    last_frame_time: Instant,
}

impl InputState {
    pub fn new() -> InputState {
        Self {
            keys_held: HashSet::new(),
            mouse_delta: (0.0, 0.0),
            start_time: Instant::now(),
            last_frame_time: Instant::now(),
        }
    }

    pub fn reset_frame_time(&mut self) {
        self.last_frame_time = Instant::now();
    }

    pub fn reset_frame_input(&mut self) {
        self.mouse_delta = (0.0, 0.0);
    }

    pub fn add_mouse_delta(&mut self, dx: f64, dy: f64) {
        self.mouse_delta.0 += dx as f32;
        self.mouse_delta.1 += dy as f32;
    }

    pub fn keys_held(&self) -> &HashSet<KeyCode> {
        &self.keys_held
    }

    pub fn mouse_delta(&self) -> (f32, f32) {
        self.mouse_delta
    }

    pub fn dt(&self) -> f32 {
        self.last_frame_time.elapsed().as_secs_f32()
    }

    pub fn run_time(&self) -> f32 {
        self.start_time.elapsed().as_secs_f32()
    }
}

impl Default for InputState {
    fn default() -> Self {
        Self::new()
    }
}
