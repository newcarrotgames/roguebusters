use bracket_lib::prelude::{BTerm, Point, RGB};
use specs::{Join, World, WorldExt};
use std::collections::HashSet;

use crate::{
    components::{
        attributes::Attributes,
        inventory::{EquipLocation, Inventory},
        name::Name,
        player::Player,
    },
    service::screen::ScreenService,
    ui::ui::{UIElement, UIState, LINES_SINGLE, UI},
};

pub struct SidebarUIElement {}

impl SidebarUIElement {
    pub fn new() -> Self { SidebarUIElement {} }
}

impl UIElement for SidebarUIElement {
    fn update(&mut self, _world: &World) {}

    fn render(&mut self, ctx: &mut BTerm, world: &World, _visible: &HashSet<Point>) {
        let sp = ScreenService::sidebar_position();
        let white = RGB::from_u8(255, 255, 255);

        UI::draw_labeled_box(
            ctx,
            [sp[0], 0, ScreenService::get_width() - 1, ScreenService::get_height() - 1],
            white,
            LINES_SINGLE,
            "RogueBusters",
        );

        let player_storage = world.read_storage::<Player>();
        let name_storage   = world.read_storage::<Name>();
        for (_, name) in (&player_storage, &name_storage).join() {
            UI::puts(ctx, sp[0] + 2, 2,  "Name:", white);
            UI::puts(ctx, sp[0] + 2, 4,  &name.name, white);
        }

        let attributes_storage = world.read_storage::<Attributes>();
        for (_, attrs) in (&player_storage, &attributes_storage).join() {
            // HP bar — red when low (below 1/3 of max)
            let hp     = attrs.health();
            let max_hp = attrs.stamina();
            let hp_color = if max_hp > 0 && hp * 3 < max_hp {
                RGB::from_u8(255, 80, 80)
            } else {
                white
            };
            UI::puts(ctx, sp[0] + 2,  6, &format!("HP: {:2}/{:2}", hp, max_hp), hp_color);

            UI::puts(ctx, sp[0] + 2,  8, &format!("Brawn:      {:2}", attrs.brawn()),      white);
            UI::puts(ctx, sp[0] + 2,  9, &format!("Agility:    {:2}", attrs.agility()),    white);
            UI::puts(ctx, sp[0] + 2, 10, &format!("Stamina:    {:2}", attrs.stamina()),    white);
            UI::puts(ctx, sp[0] + 2, 11, &format!("Perception: {:2}", attrs.perception()), white);
            UI::puts(ctx, sp[0] + 2, 12, &format!("Fortune:    {:2}", attrs.fortune()),    white);
            UI::puts(ctx, sp[0] + 2, 13, &format!("Charm:      {:2}", attrs.charm()),      white);
        }

        let inventory_storage = world.read_storage::<Inventory>();
        for (_, inventory) in (&player_storage, &inventory_storage).join() {
            UI::puts(ctx, sp[0] + 2, 15, "Wielding:", white);
            let s = inventory
                .equipped_item(EquipLocation::RightHand)
                .map(|i| i.name.as_str())
                .unwrap_or("Nothing");
            UI::puts(ctx, sp[0] + 2, 17, s, white);
        }
    }

    fn state(&self) -> UIState { UIState::Active }
    fn set_state(&mut self, _new_state: UIState) { todo!() }
    fn handle_event(&mut self, _event: &str) { todo!() }
    fn is_modal(&self) -> bool { false }
}
