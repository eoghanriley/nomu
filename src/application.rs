pub trait Application {
    fn run() {
        loop {
            continue;
        }
    }

    fn init() -> (Vec<crate::event::EventTypes>) {
        crate::logger::init();
        crate::event::init()
    }
}
