use crate::{
    components::{inventory::Inventory, player::Player},
    game::{PlayerRequest, GameState},
    input::handlers::InputHandler,
    ui::ui::{UIModal, LINES_DOUBLE_SINGLE, UI},
};
use specs::{Join, World, WorldExt};
use tcod::{input::{KeyCode::{*, self}, KEY_PRESSED}, console::Root};
use tcod::{
    colors::{RED, WHITE},
    console::Offscreen,
    input::Key,
    Color,
};

use super::modal_request::ModalPlayerRequest;

const INVENTORY_POSITION: [i32; 4] = [10, 10, 50, 50];

pub struct InventoryUIModal {
    selectedItem: i32,
}

impl InventoryUIModal {
    pub fn new() -> Self {
        InventoryUIModal { selectedItem: 0 }
    }
}

impl UIModal for InventoryUIModal {
    fn render(&mut self, con: &mut Offscreen) {
        UI::render_dialog(
            con,
            INVENTORY_POSITION,
            WHITE,
            LINES_DOUBLE_SINGLE,
            "Inventory",
        );
    }

    fn update(&mut self, con: &mut Offscreen, world: &World) {
        let player_storage = world.read_storage::<Player>();
        let inventory_storage = world.read_storage::<Inventory>();
        for (_, inventory) in (&player_storage, &inventory_storage).join() {
            for (i, item) in inventory.items().iter().enumerate() {
                // find selected inventory item?
                let mut color: Color = WHITE;
                if i == self.selectedItem as usize {
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
            } => PlayerRequest::ModalRequest(ModalPlayerRequest::InventoryRequest(InventoryModalPlayerRequest::PreviousItem)),
            Key {
                code: Down | NumPad2,
                ..
            } => PlayerRequest::ModalRequest(ModalPlayerRequest::InventoryRequest(InventoryModalPlayerRequest::NextItem)),

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
}