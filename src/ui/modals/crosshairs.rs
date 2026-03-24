use bracket_lib::prelude::{BTerm, Point, RGB, VirtualKeyCode};
use specs::{World, WorldExt};
use std::collections::HashSet;

use crate::{
    game::{GameState, PlayerRequest},
    input::handlers::InputHandler,
    ui::ui::{UIElement, UIState},
};

use super::modal_request::ModalPlayerRequest;

pub struct CrosshairsUIElement {
    position:     [i32; 2],
    old_position: [i32; 2],
}

impl CrosshairsUIElement {
    pub fn new(position: [i32; 2]) -> Self {
        CrosshairsUIElement { position, old_position: position }
    }
}

impl UIElement for CrosshairsUIElement {
    fn update(&mut self, world: &World) {
        let mut game_state = world.write_resource::<GameState>();
        match game_state.peek_player_request() {
            PlayerRequest::ModalRequest(ModalPlayerRequest::CrosshairsRequest(request)) => {
                match request {
                    CrosshairsModalPlayerRequest::Move(x, y) => {
                        self.old_position = self.position;
                        self.position[0] += x;
                        self.position[1] += y;
                    }
                    CrosshairsModalPlayerRequest::Select => {
                        game_state.push_message("Selected target for attack".to_string());
                        game_state.push_player_request(PlayerRequest::Selected(
                            self.position[0],
                            self.position[1],
                        ));
                    }
                    CrosshairsModalPlayerRequest::Cancel => {
                        game_state.push_message("Cancelled attack".to_string());
                        game_state.push_player_request(PlayerRequest::CloseCurrentView);
                    }
                }
            }
            _ => {}
        }
    }

    fn render(&mut self, ctx: &mut BTerm, _world: &World, _visible: &HashSet<Point>) {
        // Highlight the cursor cell: dark grey bg, light grey fg, space glyph.
        ctx.set(
            self.position[0],
            self.position[1],
            RGB::from_u8(160, 160, 160),  // light grey fg
            RGB::from_u8(63, 63, 63),     // dark grey bg
            b' ' as u16,
        );
    }

    fn state(&self) -> UIState { UIState::Active }
    fn set_state(&mut self, _new_state: UIState) { todo!() }
    fn handle_event(&mut self, _event: &str) { todo!() }
    fn is_modal(&self) -> bool { true }
}

// ── input handler ─────────────────────────────────────────────────────────────

pub struct CrosshairsInputHandler {}

impl CrosshairsInputHandler {
    pub fn new() -> Self { CrosshairsInputHandler {} }
}

impl InputHandler for CrosshairsInputHandler {
    fn handle_input(&mut self, ctx: &BTerm, world: &World) {
        use VirtualKeyCode::*;
        let key = match ctx.key {
            None    => return,
            Some(k) => k,
        };

        let request = match key {
            Numpad9  => req_move( 1, -1),
            Up | Numpad8  => req_move( 0, -1),
            Numpad7  => req_move(-1, -1),
            Right | Numpad6 => req_move( 1,  0),
            Left  | Numpad4 => req_move(-1,  0),
            Numpad3  => req_move( 1,  1),
            Down | Numpad2  => req_move( 0,  1),
            Numpad1  => req_move(-1,  1),
            Return | NumpadEnter => PlayerRequest::ModalRequest(
                ModalPlayerRequest::CrosshairsRequest(CrosshairsModalPlayerRequest::Select),
            ),
            Escape => PlayerRequest::ModalRequest(
                ModalPlayerRequest::CrosshairsRequest(CrosshairsModalPlayerRequest::Cancel),
            ),
            _ => PlayerRequest::None,
        };

        let mut game_state = world.write_resource::<GameState>();
        game_state.push_player_request(request);
    }
}

fn req_move(dx: i32, dy: i32) -> PlayerRequest {
    PlayerRequest::ModalRequest(ModalPlayerRequest::CrosshairsRequest(
        CrosshairsModalPlayerRequest::Move(dx, dy),
    ))
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum CrosshairsModalPlayerRequest {
    Move(i32, i32),
    Select,
    Cancel,
}
