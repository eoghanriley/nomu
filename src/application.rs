use crate::event;

pub struct Application {
    pub event_queue: Vec<event::EventTypes>,
}

impl Application {
    pub fn run(&self) {
        loop {
            continue;
        }
    }

    pub fn new() -> Application {
        Application {
            event_queue: event::init(),
        }
    }
}
