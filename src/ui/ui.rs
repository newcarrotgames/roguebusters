use crate::{
    components::{inventory::Inventory, player::Player, position::Position},
    MAP_HEIGHT, MAP_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH, city::city::City,
};
use specs::{Join, World, WorldExt};
use tcod::{
    colors::{BLACK, BLUE, WHITE},
    console::Offscreen,
    BackgroundFlag, Color, Console,
};

use super::dialogs::{inventory::InventoryUIModal, map::MapUIModal};

type LineSet = [u8; 8];
type Quad = [i32; 4];

pub const UI_WIDTH: i32 = 20;
pub const MESSAGES_HEIGHT: i32 = 15;

// const LINES_SINGLE: LineSet = [196, 179, 218, 191, 192, 217, 180, 195];
// const LINES_DOUBLE: LineSet = [205, 186, 201, 187, 200, 188];
// const LINES_SINGLE_DOUBLE: LineSet = [205, 179, 213, 184, 212, 190, 181, 198];
pub const LINES_DOUBLE_SINGLE: LineSet = [196, 186, 214, 183, 211, 189, 180, 195];

const NULLCHAR:char = 0 as char;

pub trait UIModal {
    fn render(&mut self, con: &mut Offscreen);
	fn update(&mut self, con: &mut Offscreen, world: &World);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum UIState {
    None,
    Inventory,
    Map,
}

pub struct UI {
    state: UIState,
    messages: Vec<String>,
    modal: Option<Box<dyn UIModal>>,
}

impl UI {
    pub fn new() -> Self {
        UI {
            state: UIState::None,
            messages: Vec::new(),
            modal: None,
        }
    }

    pub fn render(&mut self, con: &mut Offscreen) {
        // draw UI
        let uix = SCREEN_WIDTH - UI_WIDTH;

        // map
        UI::draw_labeled_box(
            con,
            [
                0,
                0,
                SCREEN_WIDTH - UI_WIDTH - 1,
                SCREEN_HEIGHT - MESSAGES_HEIGHT,
            ],
            WHITE,
            LINES_DOUBLE_SINGLE,
            "City",
        );

        // side bar
        UI::draw_labeled_box(
            con,
            [
                SCREEN_WIDTH - UI_WIDTH,
                0,
                SCREEN_WIDTH - 1,
                SCREEN_HEIGHT - 1,
            ],
            WHITE,
            LINES_DOUBLE_SINGLE,
            "RogueBusters",
        );

        // message log
        UI::draw_labeled_box(
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
            UI::puts(
                con,
                2,
                SCREEN_HEIGHT - MESSAGES_HEIGHT + 3 + (i - messages_offset) as i32,
                &msg,
                WHITE,
            );
        }

        // draw modals
        if self.modal.is_some() {
            self.modal.as_mut().unwrap().render(con);
        }
    }

    pub fn set_state(&mut self, state: UIState) {
        if state == UIState::Inventory {
            self.modal = Some(Box::new(InventoryUIModal::new()));
        } else if state == UIState::Map {
            self.modal = Some(Box::new(MapUIModal::new()));
        } else if state == UIState::None {
            self.modal = None;
        }
    }

    pub fn update(&mut self, con: &mut Offscreen, world: &World) {
        if self.modal.is_some() {
            self.modal.as_mut().unwrap().update(con, world);
        }
    }

    pub fn render_dialog(
        con: &mut Offscreen,
        pos: Quad,
        col: Color,
        set: LineSet,
        title: &str,
    ) {
        UI::fill(con, pos, WHITE, ' ');
        UI::draw_labeled_box(con, pos, col, set, title);
    }

    pub fn draw_labeled_box(
        con: &mut Offscreen,
        pos: Quad,
        col: Color,
        set: LineSet,
        title: &str,
    ) {
        UI::line_rect(con, pos, col, set);
        UI::puts(
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

    pub fn puts(con: &mut Offscreen, x: i32, y: i32, s: &str, col: Color) {
        con.set_default_foreground(col);
        for (i, c) in s.chars().enumerate() {
            con.put_char(x + i as i32, y, c, BackgroundFlag::None);
        }
    }

    pub fn line_rect(con: &mut Offscreen, pos: Quad, col: Color, set: LineSet) {
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

    pub fn fill(con: &mut Offscreen, pos: Quad, col: Color, char: char) {
        con.set_default_foreground(col);
        con.set_default_background(Color::new(32, 32, 16));
        for x in pos[0] + 1..pos[2] {
            for y in pos[1] + 1..pos[3] {
                con.put_char(x, y, char, BackgroundFlag::Set);
            }
        }
    }
}
