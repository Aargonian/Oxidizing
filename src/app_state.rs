use std::num::NonZeroU32;

pub trait AppState {
    fn update(&mut self, delta: f64);
    fn draw(&self, buffer: &mut [u32], width: NonZeroU32, height: NonZeroU32);
}
