use crate::event;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[derive(Debug)]
pub struct Application {
    pub event_queue: Vec<event::EventTypes>,
}

impl Application {
    // NOTE: If running this on wayland currently a window will not show up
    // since nothing is being drawn to it currently
    pub fn run(&self) {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        event_loop.run(move |event, _, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => {}
            },
            _ => {}
        });
    }

    pub fn new() -> Self {
        crate::logger::init();

        Application {
            event_queue: event::init(),
        }
    }
}
