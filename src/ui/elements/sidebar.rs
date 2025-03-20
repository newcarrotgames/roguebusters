use crate::{
    components::{
        attributes::Attributes,
        inventory::{EquipLocation, Inventory},
        name::Name,
        player::Player,
    },
    ui::ui::{UIElement, UIState, LINES_DOUBLE_SINGLE, UI},
    service::screen::ScreenService
};
use specs::{Join, World, WorldExt};
use tcod::{colors::WHITE, console::Offscreen, Map};

pub struct SidebarUIElement {}

impl SidebarUIElement {
    pub fn new() -> Self {
        SidebarUIElement {}
    }
}

impl UIElement for SidebarUIElement {
    fn update(&mut self, _world: &World) {}

    fn render(&mut self, con: &mut Offscreen, world: &World, _fov: &Map) {
        // side bar
        UI::draw_labeled_box(
            con,
            [
                ScreenService::sidebar_position()[0],
                0,
                ScreenService::get_width() - 1,
                ScreenService::get_height() - 1,
            ],
            WHITE,
            LINES_DOUBLE_SINGLE,
            "RogueBusters",
        );

        let player_storage = world.read_storage::<Player>();
        let name_storage = world.read_storage::<Name>();
        for (_, name) in (&player_storage, &name_storage).join() {
            UI::puts(
                con,
                ScreenService::sidebar_position()[0] + 2,
                2,
                &format!("Name:"),
                WHITE,
            );
            UI::puts(
                con,
                ScreenService::sidebar_position()[0] + 2,
                4,
                &format!("{}", name.name),
                WHITE,
            );
        }

        let attributes_storage = world.read_storage::<Attributes>();
        for (_, attrs) in (&player_storage, &attributes_storage).join() {
            UI::puts(
                con,
                ScreenService::sidebar_position()[0] + 2,
                6,
                &format!("Brawn:      {:2}", attrs.brawn()),
                WHITE,
            );
            UI::puts(
                con,
                ScreenService::sidebar_position()[0] + 2,
                7,
                &format!("Agility:    {:2}", attrs.agility()),
                WHITE,
            );
            UI::puts(
                con,
                ScreenService::sidebar_position()[0] + 2,
                8,
                &format!("Stamina:    {:2}", attrs.stamina()),
                WHITE,
            );
            UI::puts(
                con,
                ScreenService::sidebar_position()[0] + 2,
                9,
                &format!("Perception: {:2}", attrs.perception()),
                WHITE,
            );
            UI::puts(
                con,
                ScreenService::sidebar_position()[0] + 2,
                10,
                &format!("Fortune:    {:2}", attrs.fortune()),
                WHITE,
            );
            UI::puts(
                con,
                ScreenService::sidebar_position()[0] + 2,
                11,
                &format!("Charm:      {:2}", attrs.charm()),
                WHITE,
            );
        }

        let inventory_storage = world.read_storage::<Inventory>();
        for (_, inventory) in (&player_storage, &inventory_storage).join() {
            UI::puts(
                con,
                ScreenService::sidebar_position()[0] + 2,
                13,
                &format!("Wielding:"),
                WHITE,
            );
            let s;
            if inventory.equipped_item(EquipLocation::RightHand) != None {
                s = inventory
                    .equipped_item(EquipLocation::RightHand)
                    .unwrap()
                    .name
                    .as_str();
            } else {
                s = "Nothing";
            }
            UI::puts(con, ScreenService::sidebar_position()[0] + 2, 15, s, WHITE);
        }
    }

    fn state(&self) -> UIState {
        UIState::Active
    }

    fn set_state(&mut self, _new_state: UIState) {
        todo!()
    }

    fn handle_event(&mut self, _event: &str) {
        todo!()
    }

    fn is_modal(&self) -> bool {
        false
    }
}
