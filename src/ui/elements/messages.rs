use crate::{
    game::GameState, service::screen::{self, ScreenService}, ui::ui::{UIElement, LINES_DOUBLE_SINGLE, UI}
};
use specs::{World, WorldExt};
use tcod::{colors::WHITE, console::Offscreen, Map};

pub struct MessagesUIElement {
    messages: Vec<String>,
}

impl MessagesUIElement {
    pub fn new() -> Self {
        MessagesUIElement {
            messages: Vec::new(),
        }
    }
}

impl UIElement for MessagesUIElement {
    fn update(&mut self, world: &World) {
        let mut game_state = world.write_resource::<GameState>();
        let mut messages: Vec<String> = Vec::new();
        while game_state.has_messages() {
            let msg = game_state.pop_message();
            messages.push(msg);
        }
        for x in messages.iter().rev() {
            self.messages.push(x.clone());
        }
    }

    fn render(&mut self, con: &mut Offscreen, world: &World, fov: &Map) {
        UI::draw_labeled_box(
            con,
            [
                ScreenService::messages_area_position()[0],
                ScreenService::messages_area_position()[1],
                ScreenService::messages_area_position()[0] + ScreenService::messages_area_size()[0] - 1,
                ScreenService::messages_area_position()[1] + ScreenService::messages_area_size()[1] - 1,
            ],
            WHITE,
            LINES_DOUBLE_SINGLE,
            "Messages",
        );

        let mut messages_offset = 0;

        if self.messages.len() as i32 >= ScreenService::messages_area_size()[1] - 2 {
            messages_offset = self.messages.len() as i32 - ScreenService::messages_area_size()[1] + 2;
        }
        
        for i in messages_offset..self.messages.len() as i32 {
            let msg = self.messages.get(i as usize).unwrap().clone();
            UI::puts(
                con,
                2,
                ScreenService::messages_area_position()[1] + 1 + (i - messages_offset) as i32,
                &msg,
                WHITE,
            );
        }
    }

    fn state(&self) -> crate::ui::ui::UIState {
        crate::ui::ui::UIState::Active
    }

    fn set_state(&mut self, new_state: crate::ui::ui::UIState) {
        todo!()
    }

    fn handle_event(&mut self, event: &str) {
        todo!()
    }

    fn is_modal(&self) -> bool {
        false
    }
}
