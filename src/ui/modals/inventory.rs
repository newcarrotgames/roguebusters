use bracket_lib::prelude::{BTerm, Point, RGB, VirtualKeyCode};
use specs::{Join, World, WorldExt};
use std::collections::HashSet;

use crate::{
    components::{inventory::{Inventory, EquipLocation}, player::Player},
    game::{GameState, PlayerRequest},
    input::handlers::InputHandler,
    ui::ui::{UIElement, UIState, LINES_SINGLE, UI},
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
        let mut game_state = world.write_resource::<GameState>();
        match game_state.peek_player_request() {
            PlayerRequest::ModalRequest(ModalPlayerRequest::InventoryRequest(request)) => {
                match request {
                    InventoryModalPlayerRequest::NextItem => {
                        let player_storage    = world.read_storage::<Player>();
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
                            let player_storage    = world.read_storage::<Player>();
                            let inventory_storage = world.read_storage::<Inventory>();
                            for (_, inventory) in (&player_storage, &inventory_storage).join() {
                                self.selected_item = inventory.items().len() as i32 - 1;
                            }
                        }
                    }
                    InventoryModalPlayerRequest::WieldItem => {
                        let player_storage    = world.read_storage::<Player>();
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

    fn render(&mut self, ctx: &mut BTerm, world: &World, _visible: &HashSet<Point>) {
        UI::render_dialog(ctx, INVENTORY_POSITION, RGB::from_u8(255, 255, 255), LINES_SINGLE, "Inventory");
        let player_storage    = world.read_storage::<Player>();
        let inventory_storage = world.read_storage::<Inventory>();
        for (_, inventory) in (&player_storage, &inventory_storage).join() {
            for (i, item) in inventory.items().iter().enumerate() {
                let color = if i == self.selected_item as usize {
                    RGB::from_u8(255, 0, 0)
                } else {
                    RGB::from_u8(255, 255, 255)
                };
                UI::puts(
                    ctx,
                    INVENTORY_POSITION[0] + 2,
                    INVENTORY_POSITION[1] + 1 + i as i32,
                    item.name.as_str(),
                    color,
                );
            }
        }
    }

    fn state(&self) -> UIState { UIState::Active }
    fn set_state(&mut self, _new_state: UIState) { todo!() }
    fn handle_event(&mut self, _event: &str) { todo!() }
    fn is_modal(&self) -> bool { true }
}

// ── input handler ─────────────────────────────────────────────────────────────

pub struct InventoryInputHandler {}

impl InventoryInputHandler {
    pub fn new() -> Self { InventoryInputHandler {} }
}

impl InputHandler for InventoryInputHandler {
    fn handle_input(&mut self, ctx: &BTerm, world: &World) {
        use VirtualKeyCode::*;
        let key = match ctx.key {
            None    => return,
            Some(k) => k,
        };

        let request = match key {
            Up | Numpad8 => PlayerRequest::ModalRequest(ModalPlayerRequest::InventoryRequest(
                InventoryModalPlayerRequest::PreviousItem,
            )),
            Down | Numpad2 => PlayerRequest::ModalRequest(ModalPlayerRequest::InventoryRequest(
                InventoryModalPlayerRequest::NextItem,
            )),
            W => PlayerRequest::ModalRequest(ModalPlayerRequest::InventoryRequest(
                InventoryModalPlayerRequest::WieldItem,
            )),
            Escape => PlayerRequest::CloseCurrentView,
            _      => PlayerRequest::None,
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
