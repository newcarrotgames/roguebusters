use specs::{World, WorldExt, Join};
use tcod::{
    colors::{BLACK, WHITE},
    console::Offscreen,
    BackgroundFlag, Color, Console, Map
};

use crate::{game::{GameState, PlayerRequest}, components::{position::Position, player::Player}};

use super::{elements::{city::CityUIElement, messages::MessagesUIElement, sidebar::SidebarUIElement}, modals::{inventory::InventoryUIElement, map::MapUIElement, crosshairs::CrosshairsUIElement}};

type Coord = [i32; 2];
type LineSet = [u8; 8];
type Quad = [i32; 4];

pub const UI_WIDTH: i32 = 20;
pub const MESSAGES_HEIGHT: i32 = 15;
pub const MAP_SIZE: [i32; 2] = [53, 99];

// const LINES_SINGLE: LineSet = [196, 179, 218, 191, 192, 217, 180, 195];
// const LINES_DOUBLE: LineSet = [205, 186, 201, 187, 200, 188];
// const LINES_SINGLE_DOUBLE: LineSet = [205, 179, 213, 184, 212, 190, 181, 198];
pub const LINES_DOUBLE_SINGLE: LineSet = [196, 186, 214, 183, 211, 189, 180, 195];

// Enum to describe the state of the UI element
#[derive(Debug, PartialEq, Eq)]
pub enum UIState {
    Active,
    Inactive,
    Hidden,
}

// Trait that describes a generic UI element
pub trait UIElement {
    // Get the state of the UI element
    fn state(&self) -> UIState;

    // Set the state of the UI element
    fn set_state(&mut self, new_state: UIState);

    // Function to update the UI element (e.g., each game loop iteration)
    fn update(&mut self, world: &World);

    // Function to render the UI element
    fn render(&mut self, con: &mut Offscreen, world: &World, fov: &Map);

    // Function to handle user input or other interactions
    fn handle_event(&mut self, event: &str);

    // Function to check if the UI element is modal
    fn is_modal(&self) -> bool;
}

pub struct UI {
    // vector for storing present UI elements
    elements: Vec<Box<dyn UIElement>>,
}

impl UI {
    pub fn new(view_offset: [i32; 2]) -> Self {
        // add initial UI elements
        let mut elements: Vec<Box<dyn UIElement>> = Vec::new();
        elements.push(Box::new(CityUIElement::new(view_offset)));
        elements.push(Box::new(MessagesUIElement::new()));
        elements.push(Box::new(SidebarUIElement::new()));
        UI {
            elements
        }
    }

    pub fn update(&mut self, world: &World) {
        // iterate elements and update
        for element in self.elements.iter_mut() {
            element.update(world);
        }

        // grab the game_state
        let game_state = world.read_resource::<GameState>();
        // if the game_state has a modal, add it to the UI
        match game_state.peek_player_request() {
            PlayerRequest::ViewInventory =>
                self.elements.push(Box::new(InventoryUIElement::new())),
            PlayerRequest::ViewMap =>
                self.elements.push(Box::new(MapUIElement::new())),
            PlayerRequest::Selection => {
                let position_storage = world.read_storage::<Position>();
                let player_storage = world.read_storage::<Player>();
                for (pos, _) in (&position_storage, &player_storage).join() {
                    let view_offset = game_state.get_view_offset();
                    let cursor_pos = [pos.x as i32 - view_offset[0], pos.y as i32 - view_offset[1]];
                    log::info!("cursor_pos: {:?}", cursor_pos);
                    self.elements.push(Box::new(CrosshairsUIElement::new(cursor_pos)));
                }
            }
            _ => {}
        }
    }

    pub fn render(&mut self, con: &mut Offscreen, world: &World, fov: &Map) {
        // iterate elements and render
        for element in self.elements.iter_mut() {
            if element.state() != UIState::Hidden {
                element.render(con, world, fov);
            }
        }
    }

    pub fn close_current_view(&mut self) {
        // find the modal and remove it
        let mut index:i32 = -1;
        for (i, element) in self.elements.iter().enumerate() {
            if element.is_modal() {
                index = i as i32;
            }
        }
        if index != -1 {
            self.elements.remove(index as usize);
        }
    }

    // -------------------------------------------------------------------------------------------------------------
    // -- STATIC DRAWING METHODS -----------------------------------------------------------------------------------
    // -------------------------------------------------------------------------------------------------------------

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

    pub fn fade(col: Color) -> Color {
        return Color::new(col.r / 4, col.g / 4, col.b / 4);
    }
}
