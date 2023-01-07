#[derive(Debug)]
pub enum MouseEvents {
    MouseButtonPressed {
        name: String,
        left: bool,
        x: u32,
        y: u32,
    },
}

#[derive(Debug)]
pub enum EventTypes {
    MouseEvents(MouseEvents),
}

pub fn init() -> Vec<EventTypes> {
    let mut event_queue: Vec<EventTypes> = Vec::new();
    event_queue
}
