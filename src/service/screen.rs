// Fixed console dimensions — all layout is derived from these two constants.
// To resize the visible area change these values and recompile; the bracket-lib
// window will scale automatically via with_fitscreen.
pub const CONSOLE_WIDTH: i32 = 80;
pub const CONSOLE_HEIGHT: i32 = 45;

const SIDEBAR_W: i32 = 20;   // rightmost ~25 % of console width
const MSG_H: i32 = 11;        // bottom 24 % of console height

pub struct ScreenService;

impl ScreenService {
    pub fn get_width() -> i32 {
        CONSOLE_WIDTH
    }

    pub fn get_height() -> i32 {
        CONSOLE_HEIGHT
    }

    pub fn sidebar_size() -> [i32; 2] {
        [SIDEBAR_W, CONSOLE_HEIGHT]
    }

    pub fn sidebar_position() -> [i32; 2] {
        [CONSOLE_WIDTH - SIDEBAR_W, 0]
    }

    pub fn messages_area_size() -> [i32; 2] {
        [CONSOLE_WIDTH - SIDEBAR_W, MSG_H]
    }

    pub fn messages_area_position() -> [i32; 2] {
        [0, CONSOLE_HEIGHT - MSG_H]
    }

    pub fn map_area_size() -> [i32; 2] {
        [CONSOLE_WIDTH - SIDEBAR_W, CONSOLE_HEIGHT - MSG_H]
    }

    pub fn map_area_position() -> [i32; 2] {
        [0, 0]
    }
}
