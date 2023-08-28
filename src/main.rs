use game::Game;
// use log::LevelFilter;
use simple_logger::SimpleLogger;
use tcod::console::*;
// use kira::{
//     manager::{
//         AudioManager, AudioManagerSettings,
//         backend::cpal::CpalBackend,
//     },
//     sound::static_sound::{StaticSoundData, StaticSoundSettings},
// };

mod game;
mod names;

mod input {
    pub mod handlers;
}

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
    pub mod equipped;
}

mod systems {
    pub mod simple_path;
    pub mod item_search;
}

mod city {
    pub mod city;
    pub mod building;
}

mod ui {
    pub mod ui;
    pub mod modals {
        pub mod modal_request;
        pub mod inventory;
        pub mod map;
    }
}

mod render {
    pub mod renderer;
}

use crate::deser::prefabs::Prefabs;

// size of the map
const MAP_WIDTH: i32 = 1000;
const MAP_HEIGHT: i32 = 1000;

// actual size of the window
pub const SCREEN_WIDTH: i32 = 120;
pub const SCREEN_HEIGHT: i32 = 68;

const LIMIT_FPS: i32 = 20; // 20 frames-per-second maximum

fn main() {
    SimpleLogger::new().with_level(log::LevelFilter::Info).init().unwrap();

    // env_logger::Builder::new()
    //     .format(|buf, record| {
    //         writeln!(
    //             buf,
    //             "{}:{} {} [{}] - {}",
    //             record.file().unwrap_or("unknown"),
    //             record.line().unwrap_or(0),
    //             chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
    //             record.level(),
    //             record.args()
    //         )
    //     })
    //     .filter(Some("roguebusters"), LevelFilter::Debug)
    //     .init();

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

    // --- music start ---

    // Create an audio manager. This plays sounds and manages resources.
    // let mut manager = AudioManager::<CpalBackend>::new(AudioManagerSettings::default()).unwrap();
    // let sound_data = StaticSoundData::from_file("sound/Rhapsody-in-Blue.ogg", StaticSoundSettings::default()).unwrap();
    // manager.play(sound_data).unwrap();

    // --- music end -----

    // prime the carburetor
    game.update_game();

    while !game.root.window_closed() {
        if !game.update() {
            break
        }
        game.render();
    }
}
