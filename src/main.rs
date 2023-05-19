use game::Game;
use simple_logger::SimpleLogger;
use tcod::console::*;
use kira::{
    manager::{
        AudioManager, AudioManagerSettings,
        backend::cpal::CpalBackend,
    },
    sound::static_sound::{StaticSoundData, StaticSoundSettings},
};

mod game;
mod input;
mod ui;
mod names;
mod test;

mod deser {
    pub mod prefabs;
    pub mod templates;
    pub mod items;
    pub mod generators;
}

mod components {
    pub mod position;
    pub mod renderable;
    pub mod player;
    pub mod target;
    pub mod name;
    pub mod inventory;
    pub mod item;
}

mod systems {
    pub mod simple_path;
    pub mod item_search;
}

mod city {
    pub mod city;
    pub mod building;
}

use crate::{input::Input, deser::prefabs::Prefabs};

// size of the map
const MAP_WIDTH: i32 = 1000;
const MAP_HEIGHT: i32 = 1000;

// actual size of the window
pub const SCREEN_WIDTH: i32 = 120;
pub const SCREEN_HEIGHT: i32 = 68;

const LIMIT_FPS: i32 = 20; // 20 frames-per-second maximum

fn main() {
    SimpleLogger::new().with_level(log::LevelFilter::Info).init().unwrap();

    log::info!("loading prefabs");
    let mut prefabs = Prefabs::new("data/prefabs");
    prefabs.load_all();

    log::info!("creating tcod console");
    let root = Root::initializer()
        .font("fonts/6YQgQ.png", FontLayout::AsciiInRow)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("RogueBusters")
        .init();
    let con = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);
    tcod::system::set_fps(LIMIT_FPS);

    log::info!("creating specs world");
    
    let mut game = Game::new(root, con, prefabs);
    let input_handler = Input::new();

    // --- music start ---

    // Create an audio manager. This plays sounds and manages resources.
    // let mut manager = AudioManager::<CpalBackend>::new(AudioManagerSettings::default()).unwrap();
    // let sound_data = StaticSoundData::from_file("sound/Rhapsody-in-Blue.ogg", StaticSoundSettings::default()).unwrap();
    // manager.play(sound_data).unwrap();

    // --- music end -----

    // prime the carburetor
    game.update_game();

    while !game.root.window_closed() {
        input_handler.handle_keys(&mut game);
        if !game.update() {
            break
        }
        game.render();
    }
}
