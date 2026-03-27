use bracket_lib::prelude::{BTerm, Point, RGB, VirtualKeyCode};
use specs::{Join, World, WorldExt};
use std::collections::HashSet;

use crate::{
    city::city::City,
    components::{player::Player, position::Position},
    game::{GameState, PlayerRequest},
    input::handlers::InputHandler,
    service::screen::ScreenService,
    ui::ui::{UIElement, UIState, LINES_SINGLE, UI},
    MAP_HEIGHT, MAP_WIDTH,
};

pub struct MapUIElement {
    pos: [i32; 4],
}

impl MapUIElement {
    pub fn new() -> Self {
        MapUIElement { pos: Self::calculate_self_pos() }
    }

    fn calculate_self_pos() -> [i32; 4] {
        let x1 = ScreenService::get_width()  / 20;
        let y1 = ScreenService::get_height() / 20;
        let x2 = ScreenService::get_width()  - x1;
        let y2 = ScreenService::get_height() - y1;
        [x1, y1, x2, y2]
    }
}

impl UIElement for MapUIElement {
    fn update(&mut self, _world: &World) {}

    fn render(&mut self, ctx: &mut BTerm, world: &World, _visible: &HashSet<Point>) {
        UI::render_dialog(ctx, self.pos, RGB::from_u8(255, 255, 255), LINES_SINGLE, "Map");

        let map = world.read_resource::<City>();
        let map_x_scale = MAP_WIDTH  / (self.pos[2] - self.pos[0] - 2);
        let map_y_scale = MAP_HEIGHT / (self.pos[3] - self.pos[1] - 2);

        let water_blue = RGB::from_u8(0, 0, 255);
        let black      = RGB::from_u8(0, 0, 0);
        let white      = RGB::from_u8(255, 255, 255);

        for my in self.pos[1] + 1..self.pos[3] {
            for mx in self.pos[0] + 1..self.pos[2] {
                let x    = mx - self.pos[0];
                let y    = my - self.pos[1];
                let tile = map.data[(y * map_y_scale) as usize][(x * map_x_scale) as usize];

                let (fg, bg, ch) = if tile.bg_color == water_blue {
                    (white, water_blue, b' ' as u16)
                } else if tile.building_id == 0
                    && (tile.char == 32 as char || tile.char == 0 as char)
                    && tile.bg_color == black
                {
                    (white, black, b' ' as u16)
                } else {
                    (white, black, 176u16)
                };

                ctx.set(mx, my, fg, bg, ch);
            }
        }

        // draw player marker
        let player_storage   = world.read_storage::<Player>();
        let position_storage = world.read_storage::<Position>();
        for (_, player_pos) in (&player_storage, &position_storage).join() {
            ctx.set(
                self.pos[0] + (player_pos.x as i32 / map_x_scale),
                self.pos[1] + (player_pos.y as i32 / map_y_scale),
                white, black, b'@' as u16,
            );
        }
    }

    fn state(&self) -> UIState { UIState::Active }
    fn set_state(&mut self, _new_state: UIState) { todo!() }
    fn handle_event(&mut self, _event: &str) { todo!() }
    fn is_modal(&self) -> bool { true }
}

// ── input handler ─────────────────────────────────────────────────────────────

pub struct MapInputHandler {}

impl MapInputHandler {
    pub fn new() -> Self { MapInputHandler {} }
}

impl InputHandler for MapInputHandler {
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

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum MapModalPlayerRequest {
    Zoom,
    Move(i32, i32),
}
