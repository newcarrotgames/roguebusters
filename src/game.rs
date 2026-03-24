use bracket_lib::prelude::{
    field_of_view, Algorithm2D, BaseMap, BTerm, GameState as BGameState, Point, RGB,
};
use specs::{Builder, Dispatcher, DispatcherBuilder, Join, World, WorldExt};
use std::collections::HashSet;

use crate::{
    city::city::City,
    components::{
        attributes::Attributes,
        combatant::Combatant,
        inventory::Inventory,
        item::Item,
        name::Name,
        npc::NPC,
        player::Player,
        position::Position,
        renderable::Renderable,
        target::Target,
    },
    deser::{items::Items, prefabs::Prefabs},
    input::handlers::{DefaultInputHandler, InputHandler},
    names::{NameType, Names},
    service::screen::ScreenService,
    systems::{
        combat::combat::Combat,
        item_search::ItemSearch,
        player_action::PlayerAction,
        simple_path::SimplePath,
    },
    ui::{
        modals::{
            crosshairs::CrosshairsInputHandler,
            inventory::InventoryInputHandler,
            map::MapInputHandler,
            modal_request::ModalPlayerRequest,
        },
        ui::UI,
    },
    MAP_HEIGHT, MAP_WIDTH,
};

const TORCH_RADIUS: i32   = 75;

pub struct Game {
    pub world:         World,
    pub dispatcher:    Dispatcher<'static, 'static>,
    pub visible:       HashSet<Point>,
    pub ui:            UI,
    pub input_handler: Box<dyn InputHandler>,
}

impl Game {
    pub fn new(prefabs: Prefabs) -> Self {
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

        let dispatcher = DispatcherBuilder::new()
            .with(PlayerAction, "player_action", &[])
            .with(SimplePath,   "simple_path",   &["player_action"])
            .with(ItemSearch,   "item_search",   &["player_action"])
            .with(Combat,       "combat",        &[])
            .build();

        log::debug!("creating city map");
        let mut map = City::new(MAP_WIDTH, MAP_HEIGHT);
        map.build(prefabs);

        let names = Names::new();

        log::debug!("creating npcs");
        for _ in 0..1000 {
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

        log::debug!("creating items");
        let mut items = Items::new();
        items.load_all("data/items");
        for _ in 0..1000 {
            let item     = items.random_item();
            let position = map.get_random_target();
            world
                .create_entity()
                .with(Item::from_itemdata(item.clone()))
                .with(Renderable { char: item.char as char, color: RGB::from_u8(255, 255, 255) })
                .with(position)
                .build();
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

        let mut game_state = self.world.write_resource::<GameState>();

        // swap input handler for modals
        match game_state.peek_player_request() {
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
        player_request != PlayerRequest::Quit
    }

    /// Advance the game simulation one tick and recompute FOV.
    pub fn update_game(&mut self) {
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
    UseItem,
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
}

impl GameState {
    fn new() -> GameState {
        GameState {
            messages:       Vec::new(),
            player_request: None,
            view_offset:    [0, 0],
            should_tick:    false,
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
