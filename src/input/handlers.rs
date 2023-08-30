use crate::city::city::City;
use crate::components::inventory::Inventory;
use crate::components::item::Item;
use crate::components::player::Player;
use crate::components::position::Position;
use crate::game::GameState;
use crate::game::PlayerRequest;
use crate::ui::ui::UIState;
use specs::Entity;
use specs::Join;
use specs::World;
use specs::WorldExt;
use tcod::console::Root;
use tcod::input::KEY_PRESSED;
use tcod::input::Key;
use tcod::input::KeyCode;
use tcod::input::KeyCode::*;

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
        };

        let mut game_state = world.write_resource::<GameState>();
        game_state.push_player_request(request);
    }
}

pub trait PlayerRequestHandler {
    fn handle_request(&mut self, world: &World, root: &mut Root) -> bool;
}

pub struct DefaultPlayerRequestHandler {}

impl DefaultPlayerRequestHandler {
    pub fn new() -> Self {
        DefaultPlayerRequestHandler {}
    }

    fn pickup_item(&mut self, world: &World) {
        log::info!("pickup item");
        let mut player_position: Position = Position::zero();
        let mut positions = world.write_storage::<Position>();
        let player_storage = world.read_storage::<Player>();
        let items = world.read_storage::<Item>();
        let mut inventories = world.write_storage::<Inventory>();
        let entities = world.entities();
        let mut ents_to_remove: Vec<Entity> = Vec::new();
        for (pos, _) in (&mut positions, &player_storage).join() {
            player_position = pos.clone();
        }
        let mut game_state = world.write_resource::<GameState>();
        let mut item_found = false;
        for (entity, item, pos) in (&entities, &items, &mut positions).join() {
            if player_position == *pos {
                item_found = true;
                for (_, inventory) in (&player_storage, &mut inventories).join() {
                    if inventory.push_item(item.clone()) {
                        ents_to_remove.push(entity.clone());
                        game_state.push_message(format!("You pick up a {}", item.name));
                        log::info!("inventory: {:?}", inventory);
                    } else {
                        game_state.push_message(format!("You can not pick up the {}", item.name));
                    }
                }
            }
        }
        if !item_found {
            game_state.push_message(format!("There is nothing to pick up."));
        }
        for e in ents_to_remove {
            positions.remove(e);
        }
    }

    pub fn move_player_by(&mut self, dx: f32, dy: f32, world: &World) {
        let mut pos_storage = world.write_storage::<Position>();
        let player_storage = world.read_storage::<Player>();
        for (pos, _) in (&mut pos_storage, &player_storage).join() {
            if !self.blocked(pos.x + dx, pos.y + dy, world) {
                pos.x += dx;
                pos.y += dy;
            }
        }
    }

    pub fn blocked(&self, x: f32, y: f32, world: &World) -> bool {
        let map = world.read_resource::<City>();
        return map.data[y as usize][x as usize].blocked;
    }
}

impl PlayerRequestHandler for DefaultPlayerRequestHandler {
    fn handle_request(&mut self, world: &World, root: &mut Root) -> bool {
        let mut update = false;

        let mut game_state = world.write_resource::<GameState>();

        match game_state.peek_player_request() {
            PlayerRequest::None => update = false,
            PlayerRequest::Quit => update = false,
            PlayerRequest::Wait => update = true,
            PlayerRequest::Move(x, y) => {
                self.move_player_by(x as f32, y as f32, world);
                game_state.pop_player_request();
                update = true;
            }
            PlayerRequest::PickupItem => {
                self.pickup_item(world);
                game_state.pop_player_request();
                update = true;
            }
            PlayerRequest::ToggleFullscreen => {
                root.set_fullscreen(!root.is_fullscreen());
                game_state.pop_player_request();
                update = false;
            }
            _ => {}
        }

        return update;
    }

    // todo: this needs to be a system so all players inherit the behavior
}