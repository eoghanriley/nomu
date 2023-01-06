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
    //let v: Vec<i32> = Vec::new();
    event_queue.push(EventTypes::MouseEvents(MouseEvents::MouseButtonPressed {
        name: String::from("MouseButtonPressed"),
        left: true,
        x: 0,
        y: 0,
    }));
    event_queue
}
