use crate::game::PlayerRequest;
use tcod::input::Key;
use tcod::input::KeyCode::*;

pub trait InputHandler {
    fn handle_input(&mut self, key: Key) -> PlayerRequest;
}

pub struct DefaultInputHandler {}

impl DefaultInputHandler {
    pub fn new() -> Self {
        DefaultInputHandler {}
    }
}

impl InputHandler for DefaultInputHandler {
    fn handle_input(&mut self, key: Key) -> PlayerRequest {
        // info!("key: {:?}", key);

        match key {
            // Alt+Enter: toggle fullscreen
            Key {
                code: Enter,
                alt: true,
                ..
            } => {
                return PlayerRequest::ToggleFullscreen;
            }

            // movement keys
            Key {
                code: Up | NumPad8, ..
            } => PlayerRequest::Move(0, -1),
            Key {
                code: Down | NumPad2,
                ..
            } => PlayerRequest::Move(0, 1),
            Key {
                code: Left | NumPad4,
                ..
            } => PlayerRequest::Move(-1, 0),
            Key {
                code: Right | NumPad6,
                ..
            } => PlayerRequest::Move(1, 0),
            Key { code: NumPad9, .. } => PlayerRequest::Move(1, -1),
            Key { code: NumPad7, .. } => PlayerRequest::Move(-1, -1),
            Key { code: NumPad3, .. } => PlayerRequest::Move(1, 1),
            Key { code: NumPad1, .. } => PlayerRequest::Move(-1, 1),

            // wield item on the ground
            Key {
                printable: 'w',
                pressed: true,
                ..
            } => PlayerRequest::WieldItem,

            // pickup item on the ground
            Key {
                printable: 'p',
                pressed: true,
                ..
            } => PlayerRequest::PickupItem,

            // wait
            Key {
                printable: '.',
                pressed: true,
                ..
            } => PlayerRequest::Wait,

            // inventory
            Key { printable: 'i', .. } => PlayerRequest::ViewInventory,

            // map
            Key { printable: 'm', .. } => PlayerRequest::ViewMap,

            // close any open dialogs
            Key { code: Escape, .. } => PlayerRequest::CloseCurrentView,

            // quit
            Key {
                printable: 'q',
                shift: true,
                ..
            } => {
                PlayerRequest::Quit
            }

            // unknown key
            _ => PlayerRequest::None
        }
    }
}
