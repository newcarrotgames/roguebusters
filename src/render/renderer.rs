use crate::{
    city::city::City,
    components::{
        player::Player, position::Position, renderable::Renderable,
    },
    ui::ui::UI,
    SCREEN_HEIGHT, SCREEN_WIDTH,
};
use specs::{Join, World, WorldExt};
use tcod::{
    colors::WHITE,
    console::{Offscreen, blit, Root},
    BackgroundFlag, Color, Console, Map as FovMap,
};

type LineSet = [u8; 8];

pub const UI_WIDTH: i32 = 20;
pub const MESSAGES_HEIGHT: i32 = 15;
pub const MAP_SIZE: [i32; 2] = [54, 100];

pub const LINES_DOUBLE_SINGLE: LineSet = [196, 186, 214, 183, 211, 189, 180, 195];

pub trait UIModal {
    fn render(&mut self, con: &mut Offscreen);
    fn update(&mut self, con: &mut Offscreen, world: &World);
}

pub struct Renderer {
    messages: Vec<String>,
    modal: Option<Box<dyn UIModal>>,
    view_offset: [i32; 2],
}

impl Renderer {
    pub fn new(view_offset: [i32; 2]) -> Self {
        Renderer {
            messages: Vec::new(),
            modal: None,
            view_offset,
        }
    }

    pub fn render_ui(&mut self, con: &mut Offscreen) {
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

    pub fn render_map(&mut self, con: &mut Offscreen, world: &World, fov: &FovMap) {
        let mut map = world.write_resource::<City>();
        for vy in 0..MAP_SIZE[0] {
            for vx in 0..MAP_SIZE[1] {
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
                    con.set_char_background(vx, vy, self.fade(wall.bg_color), BackgroundFlag::Set);
                    con.set_default_foreground(self.fade(wall.fg_color));
                    con.put_char(vx, vy, wall.char, BackgroundFlag::None);
                }
            }
        }
    }

    pub fn render_entities(&mut self, con: &mut Offscreen, world: &World, fov: &FovMap) {
        let pos_storage = world.read_storage::<Position>();
        let ren_storage = world.read_storage::<Renderable>();
        for (pos, ren) in (&pos_storage, &ren_storage).join() {
            let cx = pos.x as i32 - self.view_offset[0];
            let cy = pos.y as i32 - self.view_offset[1];

            // check if offscreen
            if cx < 0 || cy < 0 || cx > SCREEN_WIDTH - 20 || cy > SCREEN_HEIGHT - MESSAGES_HEIGHT {
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

    fn fade(&self, col: Color) -> Color {
        return Color::new(col.r / 4, col.g / 4, col.b / 4);
    }

    pub fn render(&mut self, con: &mut Offscreen, world: &World, root: &mut Root, fov: &FovMap) {
        con.clear();
        // render the screen
		self.update_view_offset(world, root);
        self.render_map(con, world, fov);
        self.render_entities(con, world, fov);
        self.render_ui(con);
        // self.ui.update(&mut self.con, &self.world);
        self.render_done(con, root);
    }

    pub fn render_done(&mut self, con: &mut Offscreen, root: &mut Root) {
        root.flush();
        blit(
            con,
            (0, 0),
            (SCREEN_WIDTH, SCREEN_HEIGHT),
            root,
            (0, 0),
            1.0,
            1.0,
        );
    }

    pub fn update_view_offset(&mut self, world: &World, root: &mut Root) {
        let map = world.read_resource::<City>();
        if self.view_offset[0] <= 0
            || self.view_offset[1] <= 0
            || self.view_offset[0] >= map.width - 63
            || self.view_offset[1] >= map.height - 35
        {
            return;
        }
        let pos_storage = world.read_storage::<Position>();
        let player_storage = world.read_storage::<Player>();
        let mut pos: Position = Position::zero();
        for (p, _) in (&pos_storage, &player_storage).join() {
            pos = Position { x: p.x, y: p.y }
        }
        if pos.x as i32 - self.view_offset[0] > root.width() - 63 {
            self.view_offset[0] += 1;
        }
        if pos.y as i32 - self.view_offset[1] > root.height() - 35 {
            self.view_offset[1] += 1;
        }
        if pos.x as i32 - self.view_offset[0] < 40 {
            self.view_offset[0] -= 1;
        }
        if pos.y as i32 - self.view_offset[1] < 25 {
            self.view_offset[1] -= 1;
        }
    }
}
