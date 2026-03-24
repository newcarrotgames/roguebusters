use bracket_lib::prelude::{BTerm, Point, RGB};
use specs::{Join, World, WorldExt};
use std::collections::HashSet;

use crate::{
    city::city::City,
    components::{player::Player, position::Position, renderable::Renderable},
    game::GameState,
    service::screen::ScreenService,
    ui::ui::{UIElement, UIState, LINES_DOUBLE_SINGLE, UI},
};

const MOVEMENT_VIEW_OFFSET: i32 = 4;

pub struct CityUIElement {
    view_offset: [i32; 2],
}

impl CityUIElement {
    pub fn new(view_offset: [i32; 2]) -> Self {
        CityUIElement { view_offset }
    }
}

impl UIElement for CityUIElement {
    fn update(&mut self, world: &World) {
        let map = world.read_resource::<City>();
        let pos_storage    = world.read_storage::<Position>();
        let player_storage = world.read_storage::<Player>();
        let mut pos        = Position::zero();
        let screen_center_x = ScreenService::map_area_size()[0] / 2;
        let screen_center_y = ScreenService::map_area_size()[1] / 2;

        for (p, _) in (&pos_storage, &player_storage).join() {
            pos = Position { x: p.x, y: p.y };
        }

        if pos.x as i32 - self.view_offset[0] > screen_center_x + MOVEMENT_VIEW_OFFSET {
            self.view_offset[0] += 1;
        }
        if pos.y as i32 - self.view_offset[1] > screen_center_y + MOVEMENT_VIEW_OFFSET {
            self.view_offset[1] += 1;
        }
        if pos.x as i32 - self.view_offset[0] < screen_center_x - MOVEMENT_VIEW_OFFSET {
            self.view_offset[0] -= 1;
        }
        if pos.y as i32 - self.view_offset[1] < screen_center_y - MOVEMENT_VIEW_OFFSET {
            self.view_offset[1] -= 1;
        }

        let max_x = (map.width  - ScreenService::map_area_size()[0]).max(0);
        let max_y = (map.height - ScreenService::map_area_size()[1]).max(0);
        self.view_offset[0] = self.view_offset[0].clamp(0, max_x);
        self.view_offset[1] = self.view_offset[1].clamp(0, max_y);

        let mut game_state = world.write_resource::<GameState>();
        game_state.set_view_offset(self.view_offset);
    }

    fn render(&mut self, ctx: &mut BTerm, world: &World, visible: &HashSet<Point>) {
        UI::draw_labeled_box(
            ctx,
            [0, 0, ScreenService::map_area_size()[0] - 1, ScreenService::map_area_size()[1] - 1],
            RGB::from_u8(255, 255, 255),
            LINES_DOUBLE_SINGLE,
            "City",
        );

        let mut map = world.write_resource::<City>();

        // render terrain
        for vy in 1..ScreenService::map_area_size()[1] - 1 {
            for vx in 1..ScreenService::map_area_size()[0] - 1 {
                let x = vx + self.view_offset[0];
                let y = vy + self.view_offset[1];
                let mut tile = map.data[y as usize][x as usize];
                let is_visible = visible.contains(&Point::new(x, y));
                if is_visible {
                    ctx.set(vx, vy, tile.fg_color, tile.bg_color, tile.char as u16);
                    tile.seen = true;
                    map.data[y as usize][x as usize] = tile;
                } else if tile.seen {
                    ctx.set(vx, vy, UI::fade(tile.fg_color), UI::fade(tile.bg_color), tile.char as u16);
                }
            }
        }

        // render entities
        let pos_storage = world.read_storage::<Position>();
        let ren_storage = world.read_storage::<Renderable>();
        for (pos, ren) in (&pos_storage, &ren_storage).join() {
            let cx = pos.x as i32 - self.view_offset[0];
            let cy = pos.y as i32 - self.view_offset[1];

            if cx < 1 || cy < 1
                || cx > ScreenService::map_area_size()[0]
                || cy > ScreenService::map_area_size()[1]
            {
                continue;
            }

            if !visible.contains(&Point::new(pos.x as i32, pos.y as i32)) {
                continue;
            }

            // Use the floor tile's bg so entity sprites blend with the floor.
            let mx   = (cx + self.view_offset[0]) as usize;
            let my   = (cy + self.view_offset[1]) as usize;
            let tile = map.data[my][mx];
            ctx.set(cx, cy, ren.color, tile.bg_color, ren.char as u16);
        }
    }

    fn state(&self) -> UIState { UIState::Active }
    fn set_state(&mut self, _new_state: UIState) { todo!() }
    fn handle_event(&mut self, _event: &str) { todo!() }
    fn is_modal(&self) -> bool { false }
}
