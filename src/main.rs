use bracket_lib::prelude::{BError, BTermBuilder, main_loop};
use game::Game;
use simple_logger::SimpleLogger;

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
    pub mod player_action;
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

pub const MAP_WIDTH:  i32 = 1000;
pub const MAP_HEIGHT: i32 = 1000;

fn main() -> BError {
    SimpleLogger::new().with_level(log::LevelFilter::Info).init().unwrap();

    log::debug!("loading prefabs");
    let mut prefabs = Prefabs::new("data/prefabs");
    prefabs.load_all();

    log::debug!("creating bracket-lib context");
    let context = BTermBuilder::new()
        .with_title("RogueBusters")
        .with_fps_cap(20.0)
        .with_resource_path("tilesets/")
        .with_font("latest.png", 16, 16)
        .with_tile_dimensions(24, 24)
        .with_simple_console(80, 45, "latest.png")
        .build()?;

    log::debug!("creating game");
    let mut game = Game::new(prefabs);
    game.update_game(); // prime the carburetor — runs initial simulation tick + FOV

    main_loop(context, game)
}
