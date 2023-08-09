use lazy_static::lazy_static;
use std::sync::Mutex;

struct InputHandlerService {
    handler: InputHandler,
}

impl InputHandlerService {
    fn new() -> InputHandlerService {
        InputHandlerService {
            // Initialize fields
        }
    }

    fn set_handler(&self, handler: InputHandler) {
        self.handler = handler;
    }
}

lazy_static! {
    static ref INPUT_HANDLER: Mutex<InputHandlerService> = Mutex::new(InputHandlerService::new());
}

// fn main() {
//     let s = INPUT_HANDLER.lock().unwrap();
//     s.some_method();
// }