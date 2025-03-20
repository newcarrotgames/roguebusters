use crate::{
    components::{inventory::{Inventory, EquipLocation}, player::Player},
    game::{GameState, PlayerRequest},
    input::handlers::InputHandler,
    ui::ui::{UIElement, UIState, LINES_DOUBLE_SINGLE, UI},
};
use specs::{Join, World, WorldExt};
use tcod::{
    colors::{RED, WHITE},
    console::Offscreen,
    input::Key,
    Color,
};
use tcod::{
    console::Root,
    input::{
        KeyCode::{self, *},
        KEY_PRESSED,
    },
    Map,
};

use super::modal_request::ModalPlayerRequest;

const INVENTORY_POSITION: [i32; 4] = [10, 10, 50, 50];

pub struct InventoryUIElement {
    selected_item: i32,
}

impl InventoryUIElement {
    pub fn new() -> Self {
        InventoryUIElement { selected_item: 0 }
    }
}

impl UIElement for InventoryUIElement {
    fn update(&mut self, world: &World) {
        // grab game_state from world
        let mut game_state = world.write_resource::<GameState>();
        match game_state.peek_player_request() {
            PlayerRequest::ModalRequest(ModalPlayerRequest::InventoryRequest(request)) => {
                match request {
                    InventoryModalPlayerRequest::NextItem => {
                        let player_storage = world.read_storage::<Player>();
                        let inventory_storage = world.read_storage::<Inventory>();
                        for (_, inventory) in (&player_storage, &inventory_storage).join() {
                            if self.selected_item < inventory.items().len() as i32 - 1 {
                                self.selected_item += 1;
                            } else {
                                self.selected_item = 0;
                            }
                        }
                    }
                    InventoryModalPlayerRequest::PreviousItem => {
                        if self.selected_item > 0 {
                            self.selected_item -= 1;
                        } else {
                            let player_storage = world.read_storage::<Player>();
                            let inventory_storage = world.read_storage::<Inventory>();
                            for (_, inventory) in (&player_storage, &inventory_storage).join() {
                                self.selected_item = inventory.items().len() as i32 - 1;
                            }
                        }
                    }
                    InventoryModalPlayerRequest::WieldItem => {
                        let player_storage = world.read_storage::<Player>();
                        let mut inventory_storage = world.write_storage::<Inventory>();
                        for (_, inventory) in (&player_storage, &mut inventory_storage).join() {
                            let item = inventory.get_item(self.selected_item as usize);
                            inventory.equip(item.clone(), EquipLocation::RightHand);
                            log::debug!("Wielding item: {}", item.name);
                            game_state.push_message(format!("Wielding item: {}", item.name));
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn render(&mut self, con: &mut Offscreen, world: &World, fov: &Map) {
        UI::render_dialog(
            con,
            INVENTORY_POSITION,
            WHITE,
            LINES_DOUBLE_SINGLE,
            "Inventory",
        );
        let player_storage = world.read_storage::<Player>();
        let inventory_storage = world.read_storage::<Inventory>();
        for (_, inventory) in (&player_storage, &inventory_storage).join() {
            for (i, item) in inventory.items().iter().enumerate() {
                // find selected inventory item?
                let mut color: Color = WHITE;
                if i == self.selected_item as usize {
                    color = RED;
                }
                UI::puts(
                    con,
                    INVENTORY_POSITION[0] + 2,
                    INVENTORY_POSITION[1] + 1 + i as i32,
                    item.name.as_str(),
                    color,
                );
            }
        }
    }

    fn state(&self) -> UIState {
        UIState::Active
    }

    fn set_state(&mut self, new_state: UIState) {
        todo!()
    }

    fn handle_event(&mut self, event: &str) {
        todo!()
    }

    fn is_modal(&self) -> bool {
        true
    }
}

pub struct InventoryInputHandler {}

impl InventoryInputHandler {
    pub fn new() -> Self {
        InventoryInputHandler {}
    }
}

impl InputHandler for InventoryInputHandler {
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
            Key {
                code: tcod::input::KeyCode::Up | NumPad8,
                ..
            } => PlayerRequest::ModalRequest(ModalPlayerRequest::InventoryRequest(
                InventoryModalPlayerRequest::PreviousItem,
            )), // seems a bit wordy?
            Key {
                code: Down | NumPad2,
                ..
            } => PlayerRequest::ModalRequest(ModalPlayerRequest::InventoryRequest(
                InventoryModalPlayerRequest::NextItem,
            )),

            Key {
                printable: 'w',
                pressed: true,
                ..
            } => PlayerRequest::ModalRequest(ModalPlayerRequest::InventoryRequest(
                InventoryModalPlayerRequest::WieldItem,
            )),

            // close inventory
            Key { code: Escape, .. } => PlayerRequest::CloseCurrentView,

            // unknown key
            _ => PlayerRequest::None,
        };

        let mut game_state = world.write_resource::<GameState>();
        game_state.push_player_request(request);
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum InventoryModalPlayerRequest {
    NextItem,
    PreviousItem,
    WieldItem,
}
