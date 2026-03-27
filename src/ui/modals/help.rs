use bracket_lib::prelude::{BTerm, Point, RGB, VirtualKeyCode};
use specs::{World, WorldExt};
use std::collections::HashSet;

use crate::{
    game::{GameState, PlayerRequest},
    input::handlers::InputHandler,
    ui::ui::{UIElement, UIState, LINES_SINGLE, UI},
};

const HELP_POS: [i32; 4] = [10, 2, 70, 42];

// Each row is (key column text, description column text).
const BINDINGS: &[(&str, &str)] = &[
    // section header rows use "" for key and "=== Section ===" for description
    ("",          "--- Movement ---"),
    ("Num 8 / \x18",   "Move North"),
    ("Num 2 / \x19",   "Move South"),
    ("Num 4 / \x1B",   "Move West"),
    ("Num 6 / \x1A",   "Move East"),
    ("Num 7",          "Move NW"),
    ("Num 9",          "Move NE"),
    ("Num 1",          "Move SW"),
    ("Num 3",          "Move SE"),
    ("",          "--- Actions ---"),
    ("P",              "Pick up item"),
    ("W",              "Wield item (on ground)"),
    ("D",              "Drop equipped item"),
    (".",              "Wait a turn"),
    ("S",              "Select target / Attack"),
    ("",          "--- Targeting (press S) ---"),
    ("Num/Arrows",     "Move crosshair"),
    ("Enter",          "Confirm attack"),
    ("Escape",         "Cancel targeting"),
    ("",          "--- Views ---"),
    ("I",              "Inventory"),
    ("M",              "Map"),
    ("H",              "Help (this screen)"),
    ("",          "--- System ---"),
    ("Escape",         "Close current view"),
    ("Shift + Q",      "Quit game"),
];

pub struct HelpUIElement;

impl HelpUIElement {
    pub fn new() -> Self { HelpUIElement }
}

impl UIElement for HelpUIElement {
    fn update(&mut self, _world: &World) {}

    fn render(&mut self, ctx: &mut BTerm, _world: &World, _visible: &HashSet<Point>) {
        let white  = RGB::from_u8(255, 255, 255);
        let yellow = RGB::from_u8(255, 220, 80);
        let grey   = RGB::from_u8(180, 180, 180);

        UI::render_dialog(ctx, HELP_POS, white, LINES_SINGLE, "Help");

        let x      = HELP_POS[0] + 2;
        let key_w  = 16; // column width for the key label
        let mut row = HELP_POS[1] + 2;

        for &(key, desc) in BINDINGS {
            if key.is_empty() {
                // Section header
                UI::puts(ctx, x, row, desc, yellow);
            } else {
                UI::puts(ctx, x,           row, key,  white);
                UI::puts(ctx, x + key_w,   row, desc, grey);
            }
            row += 1;
        }
    }

    fn state(&self) -> UIState { UIState::Active }
    fn set_state(&mut self, _new_state: UIState) { todo!() }
    fn handle_event(&mut self, _event: &str) { todo!() }
    fn is_modal(&self) -> bool { true }
}

// ── input handler ──────────────────────────────────────────────────────────────

pub struct HelpInputHandler;

impl HelpInputHandler {
    pub fn new() -> Self { HelpInputHandler }
}

impl InputHandler for HelpInputHandler {
    fn handle_input(&mut self, ctx: &BTerm, world: &World) {
        let key = match ctx.key {
            None    => return,
            Some(k) => k,
        };

        let request = match key {
            VirtualKeyCode::Escape => PlayerRequest::CloseCurrentView,
            _                      => PlayerRequest::None,
        };

        let mut game_state = world.write_resource::<GameState>();
        game_state.push_player_request(request);
    }
}
