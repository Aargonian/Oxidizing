use std::{rc::Rc, time::Instant, num::NonZeroU32};

use instant::Duration;
use softbuffer::Context;
use winit::{event_loop::{EventLoop, ControlFlow}, window::{Window, WindowBuilder}, event::{Event, WindowEvent}};

use crate::app_state::AppState;

pub struct Runtime<S: AppState> {
    state: S,
    event_loop: EventLoop<()>,
    window: Rc<Window>,
    context: Context<Rc<Window>>,
    frame_duration: Duration,
}

impl<S: AppState> Runtime<S> {
    pub fn new(state: S) -> Self {
        let event_loop = EventLoop::new().unwrap();
        let window = Rc::new(WindowBuilder::new().build(&event_loop).unwrap());
        let context = softbuffer::Context::new(window.clone()).unwrap();
        let frame_duration = std::time::Duration::from_secs_f64(1.0 / 60.0);

        Self {
            state,
            event_loop,
            window,
            context,
            frame_duration,
        }
    }

    pub fn run(mut self) {
        let mut last_update = Instant::now();
        let mut last_frame_time = Instant::now();
        let mut surface = softbuffer::Surface::new(&self.context, self.window.clone()).unwrap();

        self.event_loop.run(move |event, elwt| {
            elwt.set_control_flow(ControlFlow::Poll);
                                                                                                 //
            match event {
                Event::AboutToWait => {
                    if last_update.elapsed() >= self.frame_duration {
                        last_update = Instant::now();
                        let delta = last_frame_time.elapsed().as_secs_f64();
                        last_frame_time = Instant::now();

                        self.state.update(delta);
                        self.window.request_redraw();
                    }
                },
                Event::WindowEvent { window_id, event: WindowEvent::RedrawRequested} if window_id == self.window.id() => {
                    let (width, height) = {
                        let size = self.window.inner_size();
                        (size.width, size.height)
                    };
                    surface.resize(NonZeroU32::new(width).unwrap(),
                                   NonZeroU32::new(height).unwrap(),
                    ).unwrap();

                    let mut buffer = surface.buffer_mut().unwrap();

                    // Call your draw function here with the buffer, width, and height
                    self.state.draw(&mut buffer, NonZeroU32::new(width).unwrap(), NonZeroU32::new(height).unwrap());

                    buffer.present().unwrap();
                },
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == self.window.id() => {
                    elwt.exit();
                }
                _ => {}
            }
        }).unwrap();
    }
}
