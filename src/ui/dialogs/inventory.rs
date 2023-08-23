use crate::{
    components::{inventory::Inventory, player::Player}, ui::ui::{UI, UIModal, LINES_DOUBLE_SINGLE},
};
use specs::{Join, World, WorldExt};
use tcod::{console::Offscreen, colors::WHITE};

const INVENTORY_POSITION: [i32; 4] = [10, 10, 50, 50];

pub struct InventoryUIModal {}

impl InventoryUIModal {
    pub fn new() -> Self {
        InventoryUIModal {}
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
                UI::puts(
                    con,
                    INVENTORY_POSITION[0] + 2,
                    INVENTORY_POSITION[1] + 1 + i as i32,
                    item.name.as_str(),
                    WHITE,
                );
            }
        }
    }
}
