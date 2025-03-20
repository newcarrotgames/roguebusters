use game::Game;
use service::screen::ScreenService;
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

mod service {
    pub mod screen;
}

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
    pub mod attributes;
    pub mod combatant;
    pub mod npc;
}

mod systems {
    pub mod simple_path;
    pub mod item_search;
    pub mod combat {
        pub mod combat;
    }
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
        pub mod crosshairs;
    }
    pub mod elements {
        pub mod city;
        pub mod messages;
        pub mod sidebar;
    }
}

mod util {
    pub mod rng;
}

use crate::deser::prefabs::Prefabs;

// size of the map
const MAP_WIDTH: i32 = 1000;
const MAP_HEIGHT: i32 = 1000;

const LIMIT_FPS: i32 = 20; // 20 frames-per-second maximum

pub const TILE_SIZE: i32 = 16;

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

    // Get screen width and height using rdev

    log::info!("Getting screen info");

    // let (screen_width, screen_height) = get_screen_size().unwrap_or((120, 68)); // Default fallback
    // log::info("Screen width, height: {:?}, {:?}", screen_width, screen_height);
    
    // let (w, h) = display_size().unwrap();
    // SCREEN_WIDTH = w / TILE_SIZE as u64;
    // SCREEN_HEIGHT = h / TILE_SIZE as u64;
    // println!("My screen size : {:?}x{:?}", SCREEN_WIDTH, SCREEN_HEIGHT);


    // Initialize screen size
    ScreenService::initialize();

    // Access screen width and height
    let screen_width = ScreenService::get_width();
    let screen_height = ScreenService::get_height();

    log::info!("Screen size: {}x{}", screen_width, screen_height);


    log::debug!("loading prefabs");
    let mut prefabs = Prefabs::new("data/prefabs");
    prefabs.load_all();

    log::debug!("creating tcod console");
    let root = Root::initializer()
        .font("fonts/6YQgQ.png", FontLayout::AsciiInRow)
        .font_type(FontType::Greyscale)
        .size(ScreenService::get_width(), ScreenService::get_height())
        .title("RogueBusters")
        // .fullscreen(true)
        .init();
    let con = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);
    tcod::system::set_fps(LIMIT_FPS);

    log::debug!("creating specs world");
    
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
