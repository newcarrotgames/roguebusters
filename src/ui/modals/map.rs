use crate::{
    city::city::City,
    components::{player::Player, position::Position},
    game::{GameState, PlayerRequest},
    input::handlers::InputHandler,
    ui::ui::{UIElement, UIState, LINES_DOUBLE_SINGLE, UI},
    MAP_HEIGHT, MAP_WIDTH,
};
use specs::{Join, World, WorldExt};
use tcod::{
    colors::{BLACK, BLUE, WHITE},
    console::{Offscreen, Root},
    input::{
        Key,
        KeyCode::{self, *},
        KEY_PRESSED,
    },
    BackgroundFlag, Console, Map,
};

const MAP_POSITION: [i32; 4] = [5, 5, 113, 61];

pub struct MapUIElement {}

impl MapUIElement {
    pub fn new() -> Self {
        MapUIElement {}
    }
}

impl UIElement for MapUIElement {
    fn update(&mut self, world: &World) {
        // nothing for now
    }

    fn render(&mut self, con: &mut Offscreen, world: &World, fov: &Map) {
        UI::render_dialog(con, MAP_POSITION, WHITE, LINES_DOUBLE_SINGLE, "Map");
        // draw map view
        let map = world.read_resource::<City>();
        let map_x_scale = MAP_WIDTH / (MAP_POSITION[2] - MAP_POSITION[0] - 2);
        let map_y_scale = MAP_HEIGHT / (MAP_POSITION[3] - MAP_POSITION[1] - 2);
        for my in MAP_POSITION[1] + 1..MAP_POSITION[3] {
            for mx in MAP_POSITION[0] + 1..MAP_POSITION[2] {
                let x = mx - MAP_POSITION[0];
                let y = my - MAP_POSITION[1];
                let wall = map.data[(y * map_y_scale) as usize][(x * map_x_scale) as usize];
                con.set_default_foreground(WHITE);
                let mut c = 176 as char;
                if wall.bg_color == BLUE {
                    con.set_default_background(BLUE);
                    c = ' ' as char;
                } else {
                    con.set_default_background(BLACK);
                }

                if wall.building_id == 0
                    && (wall.char == 32 as char || wall.char == 0 as char)
                    && wall.bg_color == BLACK
                {
                    c = ' ';
                }
                con.put_char(mx, my, c, BackgroundFlag::Set);
            }
        }
        con.set_default_background(BLACK);

        // draw player
        let player_storage = world.read_storage::<Player>();
        let position_storage = world.read_storage::<Position>();
        for (_, player_position) in (&player_storage, &position_storage).join() {
            con.put_char(
                MAP_POSITION[0] + (player_position.x as i32 / map_x_scale),
                MAP_POSITION[1] + (player_position.y as i32 / map_y_scale),
                '@',
                BackgroundFlag::None,
            );
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
        true
    }
}

pub struct MapInputHandler {}

impl MapInputHandler {
    pub fn new() -> Self {
        MapInputHandler {}
    }
}

impl InputHandler for MapInputHandler {
    fn handle_input(&mut self, root: &Root, world: &World) {
        let key = root.check_for_keypress(KEY_PRESSED);
        if key == None {
            return;
        }
        let actual_key = key.unwrap();
        if actual_key.code == KeyCode::Text {
            // not sure why tcod does this
            return;
        }
        let request = match actual_key {
            // Key {
            //     code: Up | NumPad8, ..
            // } => PlayerRequest::Move(0, -1),
            // Key {
            //     code: Down | NumPad2,
            //     ..
            // } => PlayerRequest::Move(0, 1),
            // Key {
            //     code: Left | NumPad4,
            //     ..
            // } => PlayerRequest::Move(0, 1),
            // Key {
            //     code: Right | NumPad6,
            //     ..
            // } => PlayerRequest::Move(0, 1),

            // close map
            Key { code: Escape, .. } => PlayerRequest::CloseCurrentView,

            // unknown key
            _ => PlayerRequest::None,
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
