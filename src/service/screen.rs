use std::sync::atomic::{AtomicI32, Ordering};

const SIDEBAR_W: i32 = 20;
const MSG_H: i32 = 11;
const MIN_WIDTH: i32 = 60;
const MIN_HEIGHT: i32 = 25;

static WIDTH: AtomicI32 = AtomicI32::new(80);
static HEIGHT: AtomicI32 = AtomicI32::new(45);

pub struct ScreenService;

impl ScreenService {
    /// Called once per frame (from `tick`) with the current console dimensions.
    pub fn update(w: i32, h: i32) {
        WIDTH.store(w.max(MIN_WIDTH), Ordering::Relaxed);
        HEIGHT.store(h.max(MIN_HEIGHT), Ordering::Relaxed);
    }

    pub fn get_width() -> i32 {
        WIDTH.load(Ordering::Relaxed)
    }

    pub fn get_height() -> i32 {
        HEIGHT.load(Ordering::Relaxed)
    }

    #[allow(dead_code)]
    pub fn sidebar_size() -> [i32; 2] {
        [SIDEBAR_W, Self::get_height()]
    }

    pub fn sidebar_position() -> [i32; 2] {
        [Self::get_width() - SIDEBAR_W, 0]
    }

    pub fn messages_area_size() -> [i32; 2] {
        [Self::get_width() - SIDEBAR_W, MSG_H]
    }

    pub fn messages_area_position() -> [i32; 2] {
        [0, Self::get_height() - MSG_H]
    }

    pub fn map_area_size() -> [i32; 2] {
        [Self::get_width() - SIDEBAR_W, Self::get_height() - MSG_H]
    }

    #[allow(dead_code)]
    pub fn map_area_position() -> [i32; 2] {
        [0, 0]
    }

    /// Returns `[x1, y1, x2, y2]` for a rect of the given size centered on
    /// the current screen.
    pub fn centered_rect(w: i32, h: i32) -> [i32; 4] {
        let x1 = (Self::get_width()  - w) / 2;
        let y1 = (Self::get_height() - h) / 2;
        [x1, y1, x1 + w, y1 + h]
    }
}
