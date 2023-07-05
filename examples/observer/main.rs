use std::cell::RefCell;

struct Subject<F: FnMut()> {
    callback: Option<RefCell<F>>,
}

impl<F: FnMut()> Subject<F> {
    fn new() -> Subject<F> {
        Subject { callback: None }
    }
    fn attach(&mut self, callback: F) {
        self.callback = Some(RefCell::new(callback));
    }
    // fn detach(&mut self, callback: &'a F) {
    //     if let Some(idx) = self.callbacks.iter().position(|x| *x == callback) {
    //         self.callbacks.remove(idx);
    //     }
    // }
    fn notify(&mut self) {
        if let Some(callback) = &self.callback {
            (callback.borrow_mut())();
        }
    }
}

fn main() {
    // Setup the logging interface.
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .format_timestamp_nanos()
        .init();

    let mut subject = Subject::new();
    let observer_a = || {
        println!("Callback a received event!");
    };
    // let observer_b = || { println!("Callback b received event!"); };

    subject.attach(&observer_a);
    // subject.attach(&observer_a);
    // subject.attach(&observer_b);
    subject.notify();
    subject.notify();
    subject.notify();

    observer_a();

    // subject.detach(&observer_b);
    // subject.notify_observers();
}
