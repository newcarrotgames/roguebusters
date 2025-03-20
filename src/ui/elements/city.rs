use crate::{
    city::city::City,
    components::{player::Player, position::Position, renderable::Renderable},
    game::GameState,
    service::screen::ScreenService,
    ui::ui::{UIElement, UIState, LINES_DOUBLE_SINGLE, UI},
};
use specs::{Join, World, WorldExt};
use tcod::{colors::WHITE, console::Offscreen, BackgroundFlag, Console, Map};

const MOVEMENT_VIEW_OFFSET:i32 = 4;

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
        if self.view_offset[0] <= 0
            || self.view_offset[1] <= 0
            || self.view_offset[0] >= map.width - ScreenService::map_area_size()[0]
            || self.view_offset[1] >= map.height - ScreenService::map_area_size()[1]
        {
            return;
        }
        let pos_storage = world.read_storage::<Position>();
        let player_storage = world.read_storage::<Player>();
        let mut pos: Position = Position::zero();
        let screen_center_x:i32 = ScreenService::map_area_size()[0] / 2;
        let screen_center_y:i32 = ScreenService::map_area_size()[1] / 2;

        for (p, _) in (&pos_storage, &player_storage).join() {
            pos = Position { x: p.x, y: p.y }
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

        let mut game_state = world.write_resource::<GameState>();
        game_state.set_view_offset(self.view_offset);
        // log::debug!("update: {} {}", self.view_offset[0], self.view_offset[1]);
    }

    fn render(&mut self, con: &mut Offscreen, world: &World, fov: &Map) {
        UI::draw_labeled_box(
            con,
            [
                0,
                0,
                ScreenService::map_area_size()[0] - 1,
                ScreenService::map_area_size()[1] - 1,
            ],
            WHITE,
            LINES_DOUBLE_SINGLE,
            "City",
        );
        let mut map = world.write_resource::<City>();

        // render environment
        for vy in 1..ScreenService::map_area_size()[1] - 1 {
            for vx in 1..ScreenService::map_area_size()[0] - 1 {
                let x = vx + self.view_offset[0];
                let y = vy + self.view_offset[1];
                let mut wall = map.data[y as usize][x as usize];
                let visible = fov.is_in_fov(x, y);
                if visible {
                    con.set_char_background(vx, vy, wall.bg_color, BackgroundFlag::Set);
                    con.set_default_foreground(wall.fg_color);
                    con.put_char(vx, vy, wall.char, BackgroundFlag::None);
                    wall.seen = true;
                    map.data[y as usize][x as usize] = wall;
                } else if wall.seen {
                    con.set_char_background(vx, vy, UI::fade(wall.bg_color), BackgroundFlag::Set);
                    con.set_default_foreground(UI::fade(wall.fg_color));
                    con.put_char(vx, vy, wall.char, BackgroundFlag::None);
                }
            }
        }

        // render entities
        let pos_storage = world.read_storage::<Position>();
        let ren_storage = world.read_storage::<Renderable>();
        for (pos, ren) in (&pos_storage, &ren_storage).join() {
            let cx = pos.x as i32 - self.view_offset[0];
            let cy = pos.y as i32 - self.view_offset[1];

            // check if offscreen
            if cx < 1
                || cy < 1
                || cx > ScreenService::map_area_size()[0]
                || cy > ScreenService::map_area_size()[1]
            {
                continue;
            }

            let visible = fov.is_in_fov(pos.x as i32, pos.y as i32);
            if !visible {
                continue;
            }

            con.set_default_foreground(WHITE);
            con.put_char(cx, cy, ren.char, BackgroundFlag::None);
        }
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
        false
    }
}
