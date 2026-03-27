use bracket_lib::prelude::{BTerm, Point, RGB};
use specs::{World, WorldExt};
use std::collections::HashSet;

use crate::{
    game::GameState,
    service::screen::ScreenService,
    ui::ui::{UIElement, UIState, LINES_SINGLE, UI},
};

pub struct MessagesUIElement {
    messages: Vec<String>,
}

impl MessagesUIElement {
    pub fn new() -> Self {
        MessagesUIElement { messages: Vec::new() }
    }
}

impl UIElement for MessagesUIElement {
    fn update(&mut self, world: &World) {
        let mut game_state = world.write_resource::<GameState>();
        let mut msgs: Vec<String> = Vec::new();
        while game_state.has_messages() {
            msgs.push(game_state.pop_message());
        }
        for m in msgs.iter().rev() {
            self.messages.push(m.clone());
        }
    }

    fn render(&mut self, ctx: &mut BTerm, _world: &World, _visible: &HashSet<Point>) {
        let mp = ScreenService::messages_area_position();
        let ms = ScreenService::messages_area_size();
        UI::draw_labeled_box(
            ctx,
            [mp[0], mp[1], mp[0] + ms[0] - 1, mp[1] + ms[1] - 1],
            RGB::from_u8(255, 255, 255),
            LINES_SINGLE,
            "Messages",
        );

        let visible_rows = ms[1] - 2;
        let offset = if self.messages.len() as i32 >= visible_rows {
            self.messages.len() as i32 - visible_rows
        } else {
            0
        };

        for i in offset..self.messages.len() as i32 {
            if let Some(msg) = self.messages.get(i as usize) {
                UI::puts(
                    ctx,
                    2,
                    mp[1] + 1 + (i - offset),
                    msg,
                    RGB::from_u8(255, 255, 255),
                );
            }
        }
    }

    fn state(&self) -> UIState { UIState::Active }
    fn set_state(&mut self, _new_state: UIState) { todo!() }
    fn handle_event(&mut self, _event: &str) { todo!() }
    fn is_modal(&self) -> bool { false }
}
