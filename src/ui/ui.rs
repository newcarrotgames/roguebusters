use specs::{World, WorldExt};
use tcod::{
    colors::{BLACK, WHITE},
    console::Offscreen,
    BackgroundFlag, Color, Console
};

use crate::game::{GameState,PlayerRequest};

use super::modals::{inventory::InventoryUIModal, map::MapUIModal};

type LineSet = [u8; 8];
type Quad = [i32; 4];

pub const UI_WIDTH: i32 = 20;
pub const MESSAGES_HEIGHT: i32 = 15;
pub const MAP_SIZE: [i32; 2] = [40, 20];

// const LINES_SINGLE: LineSet = [196, 179, 218, 191, 192, 217, 180, 195];
// const LINES_DOUBLE: LineSet = [205, 186, 201, 187, 200, 188];
// const LINES_SINGLE_DOUBLE: LineSet = [205, 179, 213, 184, 212, 190, 181, 198];
pub const LINES_DOUBLE_SINGLE: LineSet = [196, 186, 214, 183, 211, 189, 180, 195];

const NULLCHAR: char = 0 as char;

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
    modal: Option<Box<dyn UIModal>>,
    view_offset: [i32; 2],
}

impl UI {
    pub fn new(view_offset: [i32; 2]) -> Self {
        UI {
            state: UIState::None,
            modal: None,
            view_offset,
        }
    }

    fn fade(col: Color) -> Color {
        return Color::new(col.r / 4, col.g / 4, col.b / 4);
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
        // give modals dibs on player requests
        if self.modal.is_some() {
            self.modal.as_mut().unwrap().update(con, world);
        }
        let mut game_state = world.write_resource::<GameState>();
        let mut pop_request = true;
        match game_state.peek_player_request() {
            PlayerRequest::CloseCurrentView => self.set_state(UIState::None),
            PlayerRequest::ViewInventory => self.set_state(UIState::Inventory),
            PlayerRequest::ViewMap => self.set_state(UIState::Map),
            // ignore other requests
            _ => pop_request = false
        }
        if pop_request {
            log::info!("UPDATE: {:?}", game_state.peek_player_request());
            game_state.pop_player_request();
        }
    }

    // I'm thinking these static methods should end up in some utility struct, but UI also makes sense
    pub fn render_dialog(con: &mut Offscreen, pos: Quad, col: Color, set: LineSet, title: &str) {
        UI::fill(con, pos, WHITE, ' ');
        UI::draw_labeled_box(con, pos, col, set, title);
    }

    pub fn draw_labeled_box(con: &mut Offscreen, pos: Quad, col: Color, set: LineSet, title: &str) {
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
