use bracket_lib::prelude::{BTerm, VirtualKeyCode};
use crate::game::{GameState, PlayerRequest};
use specs::World;
use specs::WorldExt;

pub trait InputHandler {
    fn handle_input(&mut self, ctx: &BTerm, world: &World);
}

pub struct DefaultInputHandler {}

impl DefaultInputHandler {
    pub fn new() -> Self {
        DefaultInputHandler {}
    }
}

impl InputHandler for DefaultInputHandler {
    fn handle_input(&mut self, ctx: &BTerm, world: &World) {
        use VirtualKeyCode::*;

        let key = match ctx.key {
            None => return,
            Some(k) => k,
        };

        let request = match key {
            // Alt+Enter: toggle fullscreen (no-op in bracket-lib; left as request for future use)
            Return if ctx.alt => PlayerRequest::ToggleFullscreen,

            // movement
            Up     | Numpad8 => PlayerRequest::Move(0, -1),
            Down   | Numpad2 => PlayerRequest::Move(0,  1),
            Left   | Numpad4 => PlayerRequest::Move(-1, 0),
            Right  | Numpad6 => PlayerRequest::Move( 1, 0),
            Numpad9 => PlayerRequest::Move( 1, -1),
            Numpad7 => PlayerRequest::Move(-1, -1),
            Numpad3 => PlayerRequest::Move( 1,  1),
            Numpad1 => PlayerRequest::Move(-1,  1),

            // actions
            W => PlayerRequest::WieldItem,
            P => PlayerRequest::PickupItem,
            D => PlayerRequest::DropItem,
            Period => PlayerRequest::Wait,

            // views
            H => PlayerRequest::ViewHelp,
            I => PlayerRequest::ViewInventory,
            M => PlayerRequest::ViewMap,
            S => { log::debug!("SELECTION!!"); PlayerRequest::Selection }

            // quit (Shift+Q)
            Q if ctx.shift => PlayerRequest::Quit,

            _ => PlayerRequest::None,
        };

        let mut game_state = world.write_resource::<GameState>();
        game_state.push_player_request(request);
    }
}
