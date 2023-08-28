use crate::{
    components::{inventory::Inventory, player::Player},
    game::PlayerRequest,
    input::handlers::InputHandler,
    ui::ui::{UIModal, LINES_DOUBLE_SINGLE, UI},
};
use specs::{Join, World, WorldExt};
use tcod::input::KeyCode::*;
use tcod::{
    colors::{RED, WHITE},
    console::Offscreen,
    input::Key,
    Color,
};

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
    fn handle_input(&mut self, key: Key) -> PlayerRequest {
        match key {
            Key {
                code: tcod::input::KeyCode::Up | NumPad8,
                ..
            } => PlayerRequest::Move(0, -1),
            Key {
                code: Down | NumPad2,
                ..
            } => PlayerRequest::Move(0, 1),

            // unknown key
            _ => PlayerRequest::None,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum InventoryModalPlayerRequest { 

}