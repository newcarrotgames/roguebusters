use crate::{
    city::{Rect, City},
    components::{
        inventory::Inventory,
        player::Player, position::Position,
    },
    SCREEN_HEIGHT, SCREEN_WIDTH, MAP_HEIGHT, MAP_WIDTH,
};
use specs::{Join, World, WorldExt};
use tcod::{
    colors::{BLACK, WHITE, BLUE},
    console::Offscreen,
    BackgroundFlag, Color, Console,
};

type LineSet = [u8; 8];

pub const UI_WIDTH: i32 = 20;
pub const MESSAGES_HEIGHT: i32 = 15;

const LINES_SINGLE: LineSet = [196, 179, 218, 191, 192, 217, 180, 195];
// const LINES_DOUBLE: LineSet = [205, 186, 201, 187, 200, 188];
const LINES_SINGLE_DOUBLE: LineSet = [205, 179, 213, 184, 212, 190, 181, 198];
const LINES_DOUBLE_SINGLE: LineSet = [196, 186, 214, 183, 211, 189, 180, 195];

const INVENTORY_POSITION: [i32; 4] = [10, 10, 50, 50];
const MAP_POSITION: [i32; 4] = [5, 5, 113, 61];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum UIState {
    None,
    Inventory,
    Map,
}

pub struct UI {
    state: UIState,
    messages: Vec<String>,
}

impl UI {
    pub fn new() -> Self {
        UI {
            state: UIState::None,
            messages: Vec::new(),
        }
    }

    pub fn render(&mut self, con: &mut Offscreen) {
        // draw UI
        let uix = SCREEN_WIDTH - UI_WIDTH;

        // map
        self.draw_labeled_box(
            con,
            [0, 0, SCREEN_WIDTH - UI_WIDTH - 1, SCREEN_HEIGHT - MESSAGES_HEIGHT],
            WHITE,
            LINES_DOUBLE_SINGLE,
            "City"
        );

        // side bar
        self.draw_labeled_box(
            con,
            [
                SCREEN_WIDTH - UI_WIDTH,
                0,
                SCREEN_WIDTH - 1,
                SCREEN_HEIGHT - 1,
            ],
            WHITE,
            LINES_DOUBLE_SINGLE,
            "RogueBusters"
        );

        // message log
        self.draw_labeled_box(
            con,
            [
                0,
                SCREEN_HEIGHT - MESSAGES_HEIGHT + 1,
                SCREEN_WIDTH - UI_WIDTH - 1,
                SCREEN_HEIGHT - 1,
            ],
            WHITE,
            LINES_DOUBLE_SINGLE,
            "Messages",
        );

        // iterate self.messages
        let mut messages_offset = 0;
        if self.messages.len() as i32 >= MESSAGES_HEIGHT - 2 {
            messages_offset = self.messages.len() as i32 - MESSAGES_HEIGHT + 3;
        }
        for i in messages_offset..self.messages.len() as i32 {
            let msg = self.messages.get(i as usize).unwrap().clone();
            self.puts(
                con,
                2,
                SCREEN_HEIGHT - MESSAGES_HEIGHT + 3 + (i - messages_offset) as i32,
                &msg,
                WHITE,
            );
        }

        match self.state  {
            UIState::Inventory => self.render_dialog(
                con,
                INVENTORY_POSITION,
                WHITE,
                LINES_DOUBLE_SINGLE,
                "Inventory",
            ),
            UIState::Map => self.render_dialog(
                con,
                MAP_POSITION,
                WHITE,
                LINES_DOUBLE_SINGLE,
                "Map",
            ),
            UIState::None => (),
        }
    }

    pub fn set_state(&mut self, state: UIState) {
        self.state = state;
    }

    pub fn update(&mut self, con: &mut Offscreen, world: &World) {
        if self.state == UIState::Inventory {
            let player_storage = world.read_storage::<Player>();
            let inventory_storage = world.read_storage::<Inventory>();
            for (_, inventory) in (&player_storage, &inventory_storage).join() {
                for (i, item) in inventory.items().iter().enumerate() {
                    self.puts(
                        con,
                        INVENTORY_POSITION[0] + 2,
                        INVENTORY_POSITION[1] + 1 + i as i32,
                        item.name.as_str(),
                        WHITE,
                    );
                }
            }
        } else if self.state == UIState::Map {
            // draw city
            let map = world.read_resource::<City>();
            let map_x_scale = MAP_WIDTH / (MAP_POSITION[2] - MAP_POSITION[0] - 2);
            let map_y_scale = MAP_HEIGHT / (MAP_POSITION[3] - MAP_POSITION[1] - 2);
            for my in MAP_POSITION[1] + 1..MAP_POSITION[3]  {
                for mx in MAP_POSITION[0] + 1..MAP_POSITION[2] {
                    let x = mx - MAP_POSITION[0];
                    let y = my - MAP_POSITION[1];
                    let wall = map.data[(y * map_y_scale) as usize][(x * map_x_scale) as usize];
                    con.set_default_foreground(WHITE);
                    if wall.bg_color == BLUE {
                        con.set_default_background(BLUE);
                    } else {
                        con.set_default_background(BLACK);
                    }
                    let mut c = 177 as char;
                    if wall.building_id == 0 && (wall.char == 32 as char || wall.char == 0 as char) {
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
                    BackgroundFlag::None);
            }
        }
    }

    fn render_dialog(
        &mut self,
        con: &mut Offscreen,
        pos: Rect,
        col: Color,
        set: LineSet,
        title: &str,
    ) {
        self.fill(con, pos, WHITE, ' ');
        self.draw_labeled_box(con, pos, col, set, title);
    }

    fn draw_labeled_box(
        &mut self,
        con: &mut Offscreen,
        pos: Rect,
        col: Color,
        set: LineSet,
        title: &str,
    ) {
        self.line_rect(con, pos, col, set);
        self.puts(
            con,
            pos[0] + 3,
            pos[1],
            format!(" {} ", title).as_str(),
            WHITE,
        );
        con.put_char(pos[0] + 2, pos[1], set[6] as char, BackgroundFlag::Set);
        con.put_char(
            pos[0] + title.len() as i32 + 5,
            pos[1],
            set[7] as char,
            BackgroundFlag::Set,
        );
    }

    pub fn add_message(&mut self, msg: &str) {
        log::info!("{}", msg);
        self.messages.push(msg.to_string());
    }

    pub fn puts(&mut self, con: &mut Offscreen, x: i32, y: i32, s: &str, col: Color) {
        con.set_default_foreground(col);
        for (i, c) in s.chars().enumerate() {
            con.put_char(x + i as i32, y, c, BackgroundFlag::None);
        }
    }

    pub fn line_rect(&mut self, con: &mut Offscreen, pos: Rect, col: Color, set: LineSet) {
        // rect properties struct
        con.set_default_foreground(col);
        con.set_default_background(BLACK);
        for x in pos[0] + 1..pos[2] {
            // top and bottom
            con.put_char(x, pos[1], set[0] as char, BackgroundFlag::Set);
            con.put_char(x, pos[3], set[0] as char, BackgroundFlag::Set);
        }
        for y in pos[1] + 1..pos[3] {
            // left and right
            con.put_char(pos[0], y, set[1] as char, BackgroundFlag::Set);
            con.put_char(pos[2], y, set[1] as char, BackgroundFlag::Set);
        }
        // corners
        con.put_char(pos[0], pos[1], set[2] as char, BackgroundFlag::Set);
        con.put_char(pos[2], pos[1], set[3] as char, BackgroundFlag::Set);
        con.put_char(pos[0], pos[3], set[4] as char, BackgroundFlag::Set);
        con.put_char(pos[2], pos[3], set[5] as char, BackgroundFlag::Set);
    }

    fn fill(&mut self, con: &mut Offscreen, pos: Rect, col: Color, char: char) {
        con.set_default_foreground(col);
        con.set_default_background(Color::new(32, 32, 16));
        for x in pos[0] + 1..pos[2] {
            for y in pos[1] + 1..pos[3] {
                con.put_char(x, y, char, BackgroundFlag::Set);
            }
        }
    }
}

