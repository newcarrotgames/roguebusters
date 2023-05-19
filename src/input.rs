use tcod::input::KEY_PRESSED;
use tcod::input::Key;
use tcod::input::KeyCode::*;
use crate::game::Game;
use crate::game::PlayerRequest;
use crate::ui::UIState;

pub struct Input {}

impl Input {
    pub fn new() -> Self {
        Input {}
    }

    pub fn handle_keys(&self, game: &mut Game) {
        let key = game.root.check_for_keypress(KEY_PRESSED);
        if key == None {
            return;
        }

        // log::info!("key: {:?}", key);

        match key.unwrap() {
            Key {
                code: Enter,
                alt: true,
                ..
            } => {
                // Alt+Enter: toggle fullscreen
                let fullscreen = game.root.is_fullscreen();
                game.root.set_fullscreen(!fullscreen);
            }

            // movement keys
            Key { code: Up | NumPad8, .. } => game.request(PlayerRequest::Move(0, -1)),
            Key { code: Down | NumPad2, .. } => game.request(PlayerRequest::Move(0, 1)),
            Key { code: Left | NumPad4, .. } => game.request(PlayerRequest::Move(-1, 0)),
            Key { code: Right | NumPad6, .. } => game.request(PlayerRequest::Move(1, 0)),
            Key { code: NumPad9, .. } => game.request(PlayerRequest::Move(1, -1)),
            Key { code: NumPad7, .. } => game.request(PlayerRequest::Move(-1, -1)),
            Key { code: NumPad3, .. } => game.request(PlayerRequest::Move(1, 1)),
            Key { code: NumPad1, .. } => game.request(PlayerRequest::Move(-1, 1)),

            // wield item on the ground
            Key { printable: 'w', pressed: true, .. } => {
                game.request(PlayerRequest::WieldItem);
            }

            // pickup item on the ground
            Key { printable: 'p', pressed: true, .. } => {
                game.request(PlayerRequest::PickupItem);
            }

            // wait
            Key { printable: '.', pressed: true, .. } => {
                game.request(PlayerRequest::Wait);
            }

            // inventory
            Key { printable: 'i', .. } => game.ui.set_state(UIState::Inventory),

            // map
            Key { printable: 'm', .. } => game.ui.set_state(UIState::Map),

            // close any open dialogs
            Key { code: Escape, .. } => game.ui.set_state(UIState::None),

            // quit
            Key { printable: 'q', shift: true, .. } => {
                game.request(PlayerRequest::Quit);
            }

            _ => {}
        }
    }
}
