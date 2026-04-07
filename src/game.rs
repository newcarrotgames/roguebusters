use bracket_lib::prelude::{
    field_of_view, BTerm, GameState as BGameState, Point, RGB,
};
use specs::{Builder, Dispatcher, DispatcherBuilder, Join, World, WorldExt};
use std::collections::HashSet;

use crate::{
    city::city::{City, Rect},
    components::{
        attributes::Attributes,
        combatant::Combatant,
        inventory::Inventory,
        item::Item,
        name::Name,
        npc::NPC,
        npc_memory::NPCMemory,
        player::Player,
        position::Position,
        profession::{JobType, Profession},
        renderable::Renderable,
        target::Target,
    },
    deser::{items::Items, prefabs::Prefabs},
    input::handlers::{DefaultInputHandler, InputHandler},
    names::{NameType, Names},
    service::screen::ScreenService,
    systems::{
        ai::{npc_update::NPCUpdate, schedule::ScheduleSystem},
        combat::combat::Combat,
        item_search::ItemSearch,
        player_action::PlayerAction,
        simple_path::SimplePath,
    },
    ui::{
        modals::{
            crosshairs::CrosshairsInputHandler,
            help::HelpInputHandler,
            inventory::InventoryInputHandler,
            map::MapInputHandler,
            modal_request::ModalPlayerRequest,
        },
        ui::UI,
    },
    MAP_HEIGHT, MAP_WIDTH,
};

const TORCH_RADIUS: i32 = 75;
const DEFAULT_NUM_NPCS: usize = 1000;
const DEFAULT_NUM_ITEMS: usize = 1000;
const TICKS_PER_HOUR: u64 = 60;

// ── game clock ───────────────────────────────────────────────────────────────

pub struct GameTime {
    tick: u64,
}

impl Default for GameTime {
    fn default() -> Self {
        GameTime::new()
    }
}

impl GameTime {
    pub fn new() -> Self {
        GameTime { tick: 8 * TICKS_PER_HOUR }
    }

    pub fn advance(&mut self) {
        self.tick += 1;
    }

    pub fn hour(&self) -> u8 {
        ((self.tick / TICKS_PER_HOUR) % 24) as u8
    }

    #[allow(dead_code)]
    pub fn day(&self) -> u64 {
        self.tick / (TICKS_PER_HOUR * 24)
    }
}

fn npc_color_for_job(job_type: &JobType) -> RGB {
    match job_type {
        JobType::Police   => RGB::from_u8(50, 50, 200),
        JobType::Criminal => RGB::from_u8(200, 50, 50),
        _                 => RGB::from_u8(100, 100, 100),
    }
}

// ── configurable game setup ───────────────────────────────────────────────────

pub struct GameConfig {
    pub map_width:  i32,
    pub map_height: i32,
    pub num_npcs:   usize,
    pub num_items:  usize,
    pub prefabs:    Prefabs,
    pub spawn_business_npcs: bool,
}

impl GameConfig {
    /// Full production world — 1000×1000 city, 1000 NPCs, 1000 items.
    pub fn default_full(prefabs: Prefabs) -> Self {
        GameConfig {
            map_width:  MAP_WIDTH,
            map_height: MAP_HEIGHT,
            num_npcs:   DEFAULT_NUM_NPCS,
            num_items:  DEFAULT_NUM_ITEMS,
            prefabs,
            spawn_business_npcs: true,
        }
    }

    /// Small world suitable for fast `cargo test` runs.
    /// Uses a 300×300 map (city builder requires width/height ≥ 100 for the water
    /// strip; 300 is enough for at least one city block), no prefab decoration, zero
    /// random items (tests spawn their own via `PlayTester::spawn_item_at`), and only
    /// a handful of NPCs.
    #[allow(dead_code)]
    pub fn small_test() -> Self {
        GameConfig {
            map_width:  300,
            map_height: 300,
            num_npcs:   3,
            num_items:  0,
            prefabs:    Prefabs::empty(),
            spawn_business_npcs: false,
        }
    }
}

pub struct Game {
    pub world:         World,
    pub dispatcher:    Dispatcher<'static, 'static>,
    pub visible:       HashSet<Point>,
    pub ui:            UI,
    pub input_handler: Box<dyn InputHandler>,
}

impl Game {
    /// Convenience constructor used by `main.rs` — builds the full production world.
    pub fn new(prefabs: Prefabs) -> Self {
        Game::new_with_config(GameConfig::default_full(prefabs))
    }

    /// Construct a game world from an explicit `GameConfig`.
    /// No window or bracket-lib context is required — safe to call from tests.
    pub fn new_with_config(config: GameConfig) -> Self {
        log::debug!("creating specs world");
        let mut world = World::new();

        world.register::<Position>();
        world.register::<Renderable>();
        world.register::<Player>();
        world.register::<Target>();
        world.register::<Item>();
        world.register::<Name>();
        world.register::<Inventory>();
        world.register::<Attributes>();
        world.register::<Combatant>();
        world.register::<NPC>();
        world.register::<Profession>();
        world.register::<NPCMemory>();

        let dispatcher = DispatcherBuilder::new()
            .with(PlayerAction,    "player_action",  &[])
            .with(ScheduleSystem,  "schedule",       &["player_action"])
            .with(NPCUpdate,       "npc_update",     &["player_action", "schedule"])
            .with(SimplePath,      "simple_path",    &["player_action", "npc_update"])
            .with(ItemSearch,      "item_search",    &["player_action"])
            .with(Combat,          "combat",         &["npc_update"])
            .build();

        log::debug!("creating city map");
        let mut map = City::new(config.map_width, config.map_height);
        map.build(config.prefabs);

        let names = Names::new();

        // Spawn business NPCs tied to their buildings
        if config.spawn_business_npcs {
            log::debug!("creating business npcs");
            let building_ids: Vec<i32> = map.buildings.keys().copied().collect();
            for &bid in &building_ids {
                let leaf_info: Vec<(String, Rect)> = {
                    let building = &map.buildings[&bid];
                    if building.floors.is_empty() {
                        continue;
                    }
                    building.floors[0]
                        .collect_leaves()
                        .into_iter()
                        .filter(|s| !s.interior_type.is_empty())
                        .map(|s| (s.interior_type.clone(), *s.rect()))
                        .collect()
                };

                for (interior_type, rect) in leaf_info {
                    if let Some((job_type, count)) = Profession::staff_for_interior(&interior_type) {
                        let spawn_pos = map
                            .find_walkable_in_rect(&rect)
                            .unwrap_or_else(|| map.get_random_target());

                        let npc_builder = if job_type == JobType::Criminal {
                            NPC::aggressive
                        } else {
                            NPC::new
                        };

                        for _ in 0..count {
                            world
                                .create_entity()
                                .with(npc_builder())
                                .with(Position { x: spawn_pos.x, y: spawn_pos.y })
                                .with(Renderable {
                                    char: 2 as char,
                                    color: npc_color_for_job(&job_type),
                                })
                                .with(Target { x: spawn_pos.x, y: spawn_pos.y })
                                .with(Name { name: names.get_random_name(NameType::AnyFullName) })
                                .with(Attributes::random())
                                .with(Inventory::new())
                                .with(Profession::with_employer(job_type, bid))
                                .build();
                        }
                    }
                }
            }
        }

        // Spawn civilian NPCs (wander randomly, no profession)
        log::debug!("creating civilian npcs");
        for _ in 0..config.num_npcs {
            let position = map.get_random_target();
            let target   = map.get_random_target();
            world
                .create_entity()
                .with(NPC::new())
                .with(position)
                .with(Renderable { char: 2 as char, color: RGB::from_u8(100, 100, 100) })
                .with(Target { x: target.x, y: target.y })
                .with(Name { name: names.get_random_name(NameType::AnyFullName) })
                .with(Attributes::random())
                .with(Inventory::new())
                .build();
        }

        if config.num_items > 0 {
            log::debug!("creating items");
            let mut items = Items::new();
            items.load_all("data/items");
            for _ in 0..config.num_items {
                let item     = items.random_item();
                let position = map.get_random_target();
                world
                    .create_entity()
                    .with(Item::from_itemdata(item.clone()))
                    .with(Renderable { char: item.char as char, color: RGB::from_u8(255, 255, 255) })
                    .with(position)
                    .build();
            }
        }

        log::debug!("creating player");
        let position = map.get_random_target();
        world
            .create_entity()
            .with(Player {})
            .with(Inventory::new())
            .with(Position { x: position.x, y: position.y })
            .with(Renderable { char: '@', color: RGB::from_u8(255, 255, 0) })
            .with(Name { name: names.get_random_name(NameType::AnyFullName) })
            .with(Attributes::random())
            .build();

        // Compute initial FOV before inserting the map so we can borrow it here.
        let visible: HashSet<Point> = field_of_view(
            Point::new(position.x as i32, position.y as i32),
            TORCH_RADIUS,
            &map,
        )
        .into_iter()
        .collect();

        world.insert(map);

        let map_area    = ScreenService::map_area_size();
        let view_offset = [
            (position.x as i32 - map_area[0] / 2).clamp(0, MAP_WIDTH  - map_area[0]),
            (position.y as i32 - map_area[1] / 2).clamp(0, MAP_HEIGHT - map_area[1]),
        ];

        let ui = UI::new(view_offset);

        let mut initial_state = GameState::new();
        initial_state.set_view_offset(view_offset);
        world.insert(initial_state);
        world.insert(GameTime::new());

        let input_handler: Box<dyn InputHandler> = Box::new(DefaultInputHandler::new());

        log::debug!("Done creating game");
        Game { world, dispatcher, visible, ui, input_handler }
    }

    /// Called each frame by bracket-lib's tick callback.
    pub fn update(&mut self, ctx: &BTerm) -> bool {
        self.input_handler.handle_input(ctx, &self.world);

        let should_tick = {
            let mut game_state = self.world.write_resource::<GameState>();
            // ToggleFullscreen: consume the request; actual toggle not supported by bracket-lib
            if game_state.peek_player_request() == PlayerRequest::ToggleFullscreen {
                game_state.pop_player_request();
            }
            let tick = game_state.should_tick();
            game_state.clear_tick();
            tick
        };

        if should_tick {
            self.update_game();
        }

        // update UI after the player has moved so the viewport scrolls
        self.ui.update(&self.world);

        // If a UI element (e.g. crosshairs) pushed an action request during its
        // update, run the simulation now so PlayerAction can consume it before
        // the request is popped at the end of this frame.
        let ui_triggered_tick = {
            let mut game_state = self.world.write_resource::<GameState>();
            let tick = game_state.should_tick();
            game_state.clear_tick();
            tick
        };
        if ui_triggered_tick {
            self.update_game();
        }

        let mut game_state = self.world.write_resource::<GameState>();

        // swap input handler for modals
        match game_state.peek_player_request() {
            PlayerRequest::ViewHelp => {
                self.input_handler = Box::new(HelpInputHandler::new());
            }
            PlayerRequest::ViewInventory => {
                self.input_handler = Box::new(InventoryInputHandler::new());
            }
            PlayerRequest::ViewMap => {
                self.input_handler = Box::new(MapInputHandler::new());
            }
            PlayerRequest::Selection => {
                self.input_handler = Box::new(CrosshairsInputHandler::new());
            }
            PlayerRequest::CloseCurrentView => {
                self.ui.close_current_view();
                self.input_handler = Box::new(DefaultInputHandler::new());
            }
            _ => {}
        }

        let player_request = game_state.pop_player_request();
        if player_request == PlayerRequest::Quit {
            return false;
        }

        // Keep running unless the player has died
        !game_state.game_over
    }

    /// Advance the game simulation one tick and recompute FOV.
    pub fn update_game(&mut self) {
        {
            let mut time = self.world.write_resource::<GameTime>();
            time.advance();
        }
        self.dispatcher.dispatch(&mut self.world);
        self.world.maintain();

        let player_pos = self.get_player_pos();
        let visible: HashSet<Point> = {
            let city = self.world.read_resource::<City>();
            field_of_view(
                Point::new(player_pos.x as i32, player_pos.y as i32),
                TORCH_RADIUS,
                &*city,
            )
            .into_iter()
            .collect()
        };
        self.visible = visible;
    }

    pub fn render(&mut self, ctx: &mut BTerm) {
        self.ui.render(ctx, &self.world, &self.visible);
    }

    pub fn get_player_pos(&self) -> Position {
        let pos_storage    = self.world.read_storage::<Position>();
        let player_storage = self.world.read_storage::<Player>();
        let mut position   = Position::zero();
        for (pos, _) in (&pos_storage, &player_storage).join() {
            position = Position { x: pos.x, y: pos.y };
        }
        position
    }
}

// ── bracket-lib game loop integration ────────────────────────────────────────

impl BGameState for Game {
    fn tick(&mut self, ctx: &mut BTerm) {
        let (w, h) = ctx.get_char_size();
        ScreenService::update(w as i32, h as i32);
        ctx.cls();
        let should_continue = self.update(ctx);
        self.render(ctx);
        if !should_continue {
            ctx.quit();
        }
    }
}

// ── player requests & world state ────────────────────────────────────────────

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum PlayerRequest {
    CloseCurrentView,
    DropItem,
    Move(i32, i32),
    None,
    PickupItem,
    Quit,
    ToggleFullscreen,
    #[allow(dead_code)]
    UseItem,
    ViewHelp,
    ViewInventory,
    ViewMap,
    Selection,
    Wait,
    WieldItem,
    Selected(i32, i32),
    ModalRequest(ModalPlayerRequest),
}

#[derive(Default)]
pub struct GameState {
    messages:       Vec<String>,
    player_request: Option<PlayerRequest>,
    view_offset:    [i32; 2],
    should_tick:    bool,
    pub game_over:  bool,
}

impl GameState {
    fn new() -> GameState {
        GameState {
            messages:       Vec::new(),
            player_request: None,
            view_offset:    [0, 0],
            should_tick:    false,
            game_over:      false,
        }
    }

    pub fn push_player_request(&mut self, request: PlayerRequest) {
        match request {
            PlayerRequest::Move(_, _)
            | PlayerRequest::PickupItem
            | PlayerRequest::WieldItem
            | PlayerRequest::DropItem
            | PlayerRequest::Wait
            | PlayerRequest::Selected(_, _) => {
                self.should_tick = true;
            }
            _ => {}
        }
        self.player_request = Some(request);
    }

    pub fn should_tick(&self) -> bool { self.should_tick }
    pub fn clear_tick(&mut self) { self.should_tick = false; }

    pub fn pop_player_request(&mut self) -> PlayerRequest {
        if self.player_request.is_none() {
            return PlayerRequest::None;
        }
        let request = self.player_request.unwrap();
        self.player_request = None;
        request
    }

    pub fn peek_player_request(&self) -> PlayerRequest {
        self.player_request.unwrap_or(PlayerRequest::None)
    }

    pub fn push_message(&mut self, msg: String) { self.messages.push(msg); }
    pub fn pop_message(&mut self) -> String { self.messages.pop().unwrap_or_default() }
    pub fn has_messages(&self) -> bool { !self.messages.is_empty() }

    pub fn set_view_offset(&mut self, view_offset: [i32; 2]) { self.view_offset = view_offset; }
    pub fn get_view_offset(&self) -> [i32; 2] { self.view_offset }
}
