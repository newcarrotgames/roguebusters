use crate::game::GameState;
use crate::game::PlayerRequest;
use specs::World;
use specs::WorldExt;
use tcod::console::Root;
use tcod::input::Key;
use tcod::input::KeyCode;
use tcod::input::KeyCode::*;
use tcod::input::KEY_PRESSED;

pub trait InputHandler {
    fn handle_input(&mut self, root: &Root, world: &World);
}

pub struct DefaultInputHandler {}

impl DefaultInputHandler {
    pub fn new() -> Self {
        DefaultInputHandler {}
    }
}

impl InputHandler for DefaultInputHandler {
    fn handle_input(&mut self, root: &Root, world: &World) {
        let key = root.check_for_keypress(KEY_PRESSED);
        if key == None {
            return;
        }
        let actual_key = key.unwrap();
        if actual_key.code == KeyCode::Text {
            return;
        }

        let request = match actual_key {
            // Alt+Enter: toggle fullscreen
            Key {
                code: Enter,
                alt: true,
                ..
            } => PlayerRequest::ToggleFullscreen,

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

            // drop equipped item
            Key {
                printable: 'd',
                pressed: true,
                ..
            } => PlayerRequest::DropItem,

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

            // shoot/selection
            Key { printable: 's', .. } => {
                log::debug!("SELECTION!!");
                PlayerRequest::Selection
            }

            // quit
            Key {
                printable: 'q',
                shift: true,
                ..
            } => PlayerRequest::Quit,

            // unknown key
            _ => PlayerRequest::None,
        };

        let mut game_state = world.write_resource::<GameState>();
        game_state.push_player_request(request);
    }
}

