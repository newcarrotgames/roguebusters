use lazy_static::lazy_static;
use rdev::display_size;
use std::sync::Mutex;

lazy_static! {
    static ref SCREEN_WIDTH: Mutex<i32> = Mutex::new(120);
    static ref SCREEN_HEIGHT: Mutex<i32> = Mutex::new(68);
    static ref SIDEBAR_SIZE: Mutex<[i32; 2]> = Mutex::new([0, 0]);
    static ref SIDEBAR_POS: Mutex<[i32; 2]> = Mutex::new([0, 0]);
    static ref MESSAGES_AREA_SIZE: Mutex<[i32; 2]> = Mutex::new([0, 0]);
    static ref MESSAGES_AREA_POS: Mutex<[i32; 2]> = Mutex::new([0, 0]);
    static ref MAP_AREA_SIZE: Mutex<[i32; 2]> = Mutex::new([0, 0]);
    static ref MAP_AREA_POS: Mutex<[i32; 2]> = Mutex::new([0, 0]);
}

pub struct ScreenService;

impl ScreenService {
    pub fn initialize() {
        let (w, h) = display_size().unwrap_or((1920, 1080));

        // Assume TILE_SIZE is 16, adjust accordingly
        let screen_width = (w / 16) as i32;
        let screen_height = (h / 16) as i32;

        // Update SCREEN_WIDTH and SCREEN_HEIGHT
        *SCREEN_WIDTH.lock().unwrap() = screen_width;
        *SCREEN_HEIGHT.lock().unwrap() = screen_height;

        // Sidebar takes up roughly 20% width, and full height
        let sidebar_width = (screen_width as f32 * 0.2) as i32;
        let sidebar_height = screen_height;

        // Sidebar positioned at right side of screen
        let sidebar_x = screen_width - sidebar_width;
        let sidebar_y = 0;

        *SIDEBAR_SIZE.lock().unwrap() = [sidebar_width, sidebar_height];
        *SIDEBAR_POS.lock().unwrap() = [sidebar_x, sidebar_y];

        // Messages area occupies the bottom 25% of screen height, excluding sidebar
        let messages_area_width = screen_width - sidebar_width;
        let messages_area_height = (screen_height as f32 * 0.25) as i32;

        let messages_area_x = 0;
        let messages_area_y = screen_height - messages_area_height;

        *MESSAGES_AREA_SIZE.lock().unwrap() = [messages_area_width, messages_area_height];
        *MESSAGES_AREA_POS.lock().unwrap() = [messages_area_x, messages_area_y];

        // Map area occupies the remaining top-left area
        let map_area_width = messages_area_width;
        let map_area_height = screen_height - messages_area_height;

        let map_area_x = 0;
        let map_area_y = 0;

        *MAP_AREA_SIZE.lock().unwrap() = [map_area_width, map_area_height];
        *MAP_AREA_POS.lock().unwrap() = [map_area_x, map_area_y];

        log::info!(
            "Screen initialized: {}x{}, Sidebar: pos={:?}, size={:?}, Messages Area: pos={:?}, size={:?}, Map Area: pos={:?}, size={:?}",
            screen_width,
            screen_height,
            *SIDEBAR_POS.lock().unwrap(),
            *SIDEBAR_SIZE.lock().unwrap(),
            *MESSAGES_AREA_POS.lock().unwrap(),
            *MESSAGES_AREA_SIZE.lock().unwrap(),
            *MAP_AREA_POS.lock().unwrap(),
            *MAP_AREA_SIZE.lock().unwrap()
        );
    }

    pub fn get_width() -> i32 {
        *SCREEN_WIDTH.lock().unwrap()
    }

    pub fn get_height() -> i32 {
        *SCREEN_HEIGHT.lock().unwrap()
    }

    pub fn sidebar_size() -> [i32; 2] {
        *SIDEBAR_SIZE.lock().unwrap()
    }

    pub fn sidebar_position() -> [i32; 2] {
        *SIDEBAR_POS.lock().unwrap()
    }

    pub fn messages_area_size() -> [i32; 2] {
        *MESSAGES_AREA_SIZE.lock().unwrap()
    }

    pub fn messages_area_position() -> [i32; 2] {
        *MESSAGES_AREA_POS.lock().unwrap()
    }

    pub fn map_area_size() -> [i32; 2] {
        *MAP_AREA_SIZE.lock().unwrap()
    }

    pub fn map_area_position() -> [i32; 2] {
        *MAP_AREA_POS.lock().unwrap()
    }
}
