use bracket_lib::prelude::{BTerm, RGB};
use specs::{World, WorldExt, Join};
use std::collections::HashSet;
use bracket_lib::prelude::Point;

use crate::{
    game::{GameState, PlayerRequest},
    components::{position::Position, player::Player},
};
use super::{
    elements::{city::CityUIElement, messages::MessagesUIElement, sidebar::SidebarUIElement},
    modals::{crosshairs::CrosshairsUIElement, help::HelpUIElement, inventory::InventoryUIElement, map::MapUIElement},
};

type Quad = [i32; 4];
pub type LineSet = [u8; 8];

pub const LINES_SINGLE: LineSet = [196, 179, 218, 191, 192, 217, 180, 195];
// const LINES_DOUBLE: LineSet = [205, 186, 201, 187, 200, 188];
// const LINES_SINGLE_DOUBLE: LineSet = [205, 179, 213, 184, 212, 190, 181, 198];
// const LINES_DOUBLE_SINGLE: LineSet = [196, 186, 214, 183, 211, 189, 180, 195];

#[derive(Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum UIState {
    Active,
    Inactive,
    Hidden,
}

#[allow(dead_code)]
pub trait UIElement {
    fn state(&self) -> UIState;
    fn set_state(&mut self, new_state: UIState);
    fn update(&mut self, world: &World);
    fn render(&mut self, ctx: &mut BTerm, world: &World, visible: &HashSet<Point>);
    fn handle_event(&mut self, event: &str);
    fn is_modal(&self) -> bool;
}

pub struct UI {
    elements: Vec<Box<dyn UIElement>>,
}

impl UI {
    pub fn new(view_offset: [i32; 2]) -> Self {
        let mut elements: Vec<Box<dyn UIElement>> = Vec::new();
        elements.push(Box::new(CityUIElement::new(view_offset)));
        elements.push(Box::new(MessagesUIElement::new()));
        elements.push(Box::new(SidebarUIElement::new()));
        UI { elements }
    }

    pub fn update(&mut self, world: &World) {
        for element in self.elements.iter_mut() {
            element.update(world);
        }

        let game_state = world.read_resource::<GameState>();
        match game_state.peek_player_request() {
            PlayerRequest::ViewHelp =>
                self.elements.push(Box::new(HelpUIElement::new())),
            PlayerRequest::ViewInventory =>
                self.elements.push(Box::new(InventoryUIElement::new())),
            PlayerRequest::ViewMap =>
                self.elements.push(Box::new(MapUIElement::new())),
            PlayerRequest::Selection => {
                let position_storage = world.read_storage::<Position>();
                let player_storage   = world.read_storage::<Player>();
                for (pos, _) in (&position_storage, &player_storage).join() {
                    let view_offset  = game_state.get_view_offset();
                    let cursor_pos   = [pos.x as i32 - view_offset[0], pos.y as i32 - view_offset[1]];
                    log::debug!("cursor_pos: {:?}", cursor_pos);
                    self.elements.push(Box::new(CrosshairsUIElement::new(cursor_pos)));
                }
            }
            _ => {}
        }
    }

    pub fn render(&mut self, ctx: &mut BTerm, world: &World, visible: &HashSet<Point>) {
        for element in self.elements.iter_mut() {
            if element.state() != UIState::Hidden {
                element.render(ctx, world, visible);
            }
        }
    }

    pub fn close_current_view(&mut self) {
        let mut index: i32 = -1;
        for (i, element) in self.elements.iter().enumerate() {
            if element.is_modal() {
                index = i as i32;
            }
        }
        if index != -1 {
            self.elements.remove(index as usize);
        }
    }

    // ── static drawing helpers ─────────────────────────────────────────────

    pub fn render_dialog(ctx: &mut BTerm, pos: Quad, fg: RGB, set: LineSet, title: &str) {
        UI::fill(ctx, pos, RGB::from_u8(255, 255, 255), ' ');
        UI::draw_labeled_box(ctx, pos, fg, set, title);
    }

    pub fn draw_labeled_box(ctx: &mut BTerm, pos: Quad, fg: RGB, set: LineSet, title: &str) {
        UI::line_rect(ctx, pos, fg, set);
        UI::puts(ctx, pos[0] + 3, pos[1], &format!(" {} ", title), fg);
        ctx.set(pos[0] + 2,                  pos[1], fg, RGB::from_u8(0, 0, 0), set[6] as u16);
        ctx.set(pos[0] + title.len() as i32 + 5, pos[1], fg, RGB::from_u8(0, 0, 0), set[7] as u16);
    }

    /// Print a string at (x, y) with the given foreground colour.
    /// Uses black as background — callers that need a different background
    /// should draw the background layer first with `fill`.
    pub fn puts(ctx: &mut BTerm, x: i32, y: i32, s: &str, fg: RGB) {
        ctx.print_color(x, y, fg, RGB::from_u8(0, 0, 0), s);
    }

    pub fn line_rect(ctx: &mut BTerm, pos: Quad, fg: RGB, set: LineSet) {
        let bg = RGB::from_u8(0, 0, 0);
        for x in pos[0] + 1..pos[2] {
            ctx.set(x, pos[1], fg, bg, set[0] as u16);
            ctx.set(x, pos[3], fg, bg, set[0] as u16);
        }
        for y in pos[1] + 1..pos[3] {
            ctx.set(pos[0], y, fg, bg, set[1] as u16);
            ctx.set(pos[2], y, fg, bg, set[1] as u16);
        }
        ctx.set(pos[0], pos[1], fg, bg, set[2] as u16);
        ctx.set(pos[2], pos[1], fg, bg, set[3] as u16);
        ctx.set(pos[0], pos[3], fg, bg, set[4] as u16);
        ctx.set(pos[2], pos[3], fg, bg, set[5] as u16);
    }

    pub fn fill(ctx: &mut BTerm, pos: Quad, fg: RGB, ch: char) {
        let bg = RGB::from_u8(32, 32, 16);
        for x in pos[0] + 1..pos[2] {
            for y in pos[1] + 1..pos[3] {
                ctx.set(x, y, fg, bg, ch as u16);
            }
        }
    }

    /// Darken a colour to 1/4 brightness for the "seen but not visible" effect.
    pub fn fade(col: RGB) -> RGB {
        RGB::from_f32(col.r / 4.0, col.g / 4.0, col.b / 4.0)
    }
}
