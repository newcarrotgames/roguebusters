use crate::{
    game::{GameState, PlayerRequest},
    input::handlers::InputHandler,
    ui::ui::{UIElement, UIState},
};
use specs::{World, WorldExt};
use tcod::{
    colors::{DARKER_GREY, LIGHT_GREY},
    console::Offscreen,
    input::Key,
    BackgroundFlag, Console,
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

pub struct CrosshairsUIElement {
    position: [i32; 2],
    old_position: [i32; 2],
    // old_bg: Option<Color>,
    // old_fg: Option<Color>,
    // old_char: Option<char>,
}

impl CrosshairsUIElement {
    pub fn new(position: [i32; 2]) -> Self {
        CrosshairsUIElement {
            position,
            old_position: position,
            // old_bg: None,
            // old_fg: None,
            // old_char: None,
        }
    }
}

impl UIElement for CrosshairsUIElement {
    fn update(&mut self, world: &World) {
        // grab game_state from world
        let mut game_state = world.write_resource::<GameState>();
        match game_state.peek_player_request() {
            PlayerRequest::ModalRequest(ModalPlayerRequest::CrosshairsRequest(request)) => {
                match request {
                    CrosshairsModalPlayerRequest::Move(x, y) => {
                        //log::debug!("Moving crosshairs by {}, {}", x, y);
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

    fn render(&mut self, con: &mut Offscreen, _world: &World, _fov: &Map) {
        // if self.old_position != self.position {
        //     if self.old_bg != None {
        //         con.set_char_background(
        //             self.old_position[0],
        //             self.old_position[1],
        //             self.old_bg.unwrap(),
        //             BackgroundFlag::Set,
        //         );
        //         con.set_char_foreground(
        //             self.old_position[0],
        //             self.old_position[1],
        //             self.old_fg.unwrap(),
        //         );
        //         con.put_char(
        //             self.old_position[0],
        //             self.old_position[1],
        //             self.old_char.unwrap(),
        //             BackgroundFlag::None,
        //         );
        //     }
        //     self.old_bg = Some(con.get_char_background(self.position[0], self.position[1]));
        //     self.old_fg = Some(con.get_char_foreground(self.position[0], self.position[1]));
        //     self.old_char = Some(con.get_char(self.position[0], self.position[1]));
        // }
        con.set_char_background(
            self.position[0],
            self.position[1],
            DARKER_GREY,
            BackgroundFlag::Set,
        );
        con.set_char_foreground(self.position[0], self.position[1], LIGHT_GREY);
        // con.put_char(
        //     self.position[0],
        //     self.position[1],
        //     '+',
        //     BackgroundFlag::None,
        // );
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

pub struct CrosshairsInputHandler {}

impl CrosshairsInputHandler {
    pub fn new() -> Self {
        CrosshairsInputHandler {}
    }
}

impl InputHandler for CrosshairsInputHandler {
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
            // movement keys
            Key { code: NumPad9, .. } => PlayerRequest::ModalRequest(
                ModalPlayerRequest::CrosshairsRequest(CrosshairsModalPlayerRequest::Move(1, -1)),
            ),
            Key {
                code: Up | NumPad8, ..
            } => PlayerRequest::ModalRequest(ModalPlayerRequest::CrosshairsRequest(
                CrosshairsModalPlayerRequest::Move(0, -1),
            )),
            Key { code: NumPad7, .. } => PlayerRequest::ModalRequest(
                ModalPlayerRequest::CrosshairsRequest(CrosshairsModalPlayerRequest::Move(-1, -1)),
            ),
            Key {
                code: Right | NumPad6,
                ..
            } => PlayerRequest::ModalRequest(ModalPlayerRequest::CrosshairsRequest(
                CrosshairsModalPlayerRequest::Move(1, 0),
            )),
            Key {
                code: Left | NumPad4,
                ..
            } => PlayerRequest::ModalRequest(ModalPlayerRequest::CrosshairsRequest(
                CrosshairsModalPlayerRequest::Move(-1, 0),
            )),
            Key { code: NumPad3, .. } => PlayerRequest::ModalRequest(
                ModalPlayerRequest::CrosshairsRequest(CrosshairsModalPlayerRequest::Move(1, 1)),
            ),
            Key {
                code: Down | NumPad2,
                ..
            } => PlayerRequest::ModalRequest(ModalPlayerRequest::CrosshairsRequest(
                CrosshairsModalPlayerRequest::Move(0, 1),
            )),
            Key { code: NumPad1, .. } => PlayerRequest::ModalRequest(
                ModalPlayerRequest::CrosshairsRequest(CrosshairsModalPlayerRequest::Move(-1, 1)),
            ),

            // select
            Key {
                code: Enter | NumPadEnter,
                ..
            } => PlayerRequest::ModalRequest(ModalPlayerRequest::CrosshairsRequest(
                CrosshairsModalPlayerRequest::Select,
            )),

            // cancel
            Key { code: Escape, .. } => PlayerRequest::ModalRequest(
                ModalPlayerRequest::CrosshairsRequest(CrosshairsModalPlayerRequest::Cancel),
            ),

            // unknown key
            _ => PlayerRequest::None,
        };

        let mut game_state = world.write_resource::<GameState>();
        game_state.push_player_request(request);
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum CrosshairsModalPlayerRequest {
    Move(i32, i32),
    Select,
    Cancel,
}
