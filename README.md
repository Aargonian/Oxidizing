# Oxidizing - Rusty Processing

Oxidizing is inspired by processing and p5.js. It is intended to be a simple framework
for writing beautiful visualizations using Rust.

## Usage

The library is fairly easy to use. Simply pull in AppState and Runtime, and make
a struct that implements AppState.

The following example is from fluctuate.rs:


```rust
use std::num::NonZeroU32;

use oxidizer::{AppState, Runtime};

pub struct State {
    time: f64,
}

impl State {
    pub fn init() -> Self {
        Self {
            time: 0.0,
        }
    }
}

impl AppState for State {
    fn update(&mut self, delta: f64) {
        self.time += delta;
    }

    fn draw(&self, buffer: &mut [u32], width: NonZeroU32, height: NonZeroU32) {
        let rgb_value = (127.5 * (f64::sin(2.0 * std::f64::consts::PI * self.time) + 1.0)) as u32;

        for y in 0..height.get() {
            for x in 0..width.get() {
                let index = (y * width.get() + x) as usize;
                buffer[index] = (rgb_value << 16) | (rgb_value << 8) | rgb_value; // Assuming RGB
            }
        }
    }
}

fn main() {
    let app = State::init();
    let runtime = Runtime::new(app);
    runtime.run();
}
```
## Note

This project is mostly for personal use, as a neat for me to practice various
visualizations and document them, as well as test different programming concepts
in a visual manner. Feel free to use it for anything you want!
