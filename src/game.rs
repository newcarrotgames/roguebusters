use crate::{
    city::city::City, components::{
        attributes::Attributes, combatant::Combatant, inventory::Inventory, item::Item, name::Name, npc::NPC, player::Player, position::Position, renderable::Renderable, target::Target
    }, deser::{items::Items, prefabs::Prefabs}, input::handlers::{
        DefaultInputHandler, InputHandler,
    }, names::{NameType, Names}, service::screen::ScreenService, systems::{combat::combat::Combat, item_search::ItemSearch, player_action::PlayerAction, simple_path::SimplePath}, ui::{
        modals::{
            crosshairs::CrosshairsInputHandler, inventory::InventoryInputHandler, map::MapInputHandler, modal_request::ModalPlayerRequest
        },
        ui::UI,
    }, MAP_HEIGHT, MAP_WIDTH
};
use specs::{Builder, Dispatcher, DispatcherBuilder, Join, World, WorldExt};
use tcod::{
    colors::WHITE,
    console::{blit, Offscreen, Root},
    map::FovAlgorithm,
    Console, Map as FovMap,
};

const TORCH_RADIUS: i32 = 75;
const FOV_LIGHT_WALLS: bool = true;
const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic;
const NUM_NPCS: i32 = 1000;
const NUM_ITEMS: i32 = 1000;

// todo: break this up?
pub struct Game<'a> {
    pub root: Root,
    pub con: Offscreen,
    pub world: World,
    pub dispatcher: Dispatcher<'a, 'a>,
    pub fov: FovMap,
    pub ui: UI,
    pub input_handler: Box<dyn InputHandler>,
}

impl Game<'_> {
    pub fn new(root: Root, con: Offscreen, prefabs: Prefabs) -> Self {
        let w = con.width();
        let h = con.height();

        log::debug!("creating specs world");

        // create specs world
        let mut world = World::new();

        // register components
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

        // create specs dispatcher
        let dispatcher = DispatcherBuilder::new()
            .with(PlayerAction, "player_action", &[])
            .with(SimplePath, "simple_path", &["player_action"])
            .with(ItemSearch, "item_search", &["player_action"])
            .with(Combat, "combat", &[])
            .build();

        log::debug!("creating city map");
        let mut map = City::new(w, h);
        map.build(prefabs);

        let mut fov = FovMap::new(w, h);

        // populate the FOV map, according to the generated map
        // todo: FOV map will need to be recalculated when going up/down stairs
        for y in 0..h {
            for x in 0..w {
                fov.set(
                    x,
                    y,
                    !map.data[y as usize][x as usize].block_sight,
                    !map.data[y as usize][x as usize].blocked,
                );
            }
        }

        let names = Names::new();

        log::debug!("creating npcs");

        // add npcs
        for _ in 0..NUM_NPCS {
            let position = map.get_random_target();
            let target = map.get_random_target();
            world
                .create_entity()
                .with(NPC::new())
                .with(position)
                .with(Renderable {
                    char: 2 as char,
                    color: tcod::colors::Color { r: 100, g: 100, b: 100 },
                })
                .with(Target {
                    x: target.x,
                    y: target.y,
                })
                .with(Name {
                    name: names.get_random_name(NameType::AnyFullName),
                })
                .with(Attributes::random())
                .with(Inventory::new())
                .build();
        }

        log::debug!("creating items");

        // todo: should items static?
        let mut items = Items::new();
        items.load_all("data/items");

        // add items
        for _ in 0..NUM_ITEMS {
            let item = items.random_item();
            let position = map.get_random_target();
            world
                .create_entity()
                .with(Item::from_itemdata(item.clone()))
                .with(Renderable {
                    char: item.char as char,
                    color: WHITE,
                })
                .with(position)
                .build();
        }

        log::debug!("creating player");

        // add player
        let position = map.get_random_target();
        world
            .create_entity()
            .with(Player {})
            .with(Inventory::new())
            .with(Position {
                x: position.x,
                y: position.y,
            })
            .with(Renderable {
                char: '@',
                color: tcod::colors::YELLOW,
            })
            .with(Name {
                name: names.get_random_name(NameType::AnyFullName),
            })
            .with(Attributes::random())
            .build();

        // add city/map grid as resource
        world.insert(map);

        let map_area = ScreenService::map_area_size();
        let view_offset = [
            (position.x as i32 - map_area[0] / 2)
                .clamp(0, MAP_WIDTH - map_area[0]),
            (position.y as i32 - map_area[1] / 2)
                .clamp(0, MAP_HEIGHT - map_area[1]),
        ];

        let ui = UI::new(view_offset);

        let mut initial_state = GameState::new();
        initial_state.set_view_offset(view_offset);
        world.insert(initial_state);

        let input_handler: Box<dyn InputHandler> = Box::new(DefaultInputHandler::new());

        log::debug!("Done creating game, returning...");

        Game {
            root,
            con,
            world,
            dispatcher,
            fov,
            ui,
            input_handler,
        }
    }

    pub fn update(&mut self) -> bool {
        self.input_handler.handle_input(&self.root, &self.world);

        let should_tick = {
            let mut game_state = self.world.write_resource::<GameState>();
            if game_state.peek_player_request() == PlayerRequest::ToggleFullscreen {
                self.root.set_fullscreen(!self.root.is_fullscreen());
                game_state.pop_player_request();
            }
            let tick = game_state.should_tick();
            game_state.clear_tick();
            tick
        };

        if should_tick {
            self.update_game();
        }

        // update UI after the player has moved so the viewport scrolls to the new position
        self.ui.update(&self.world);

        let mut game_state = self.world.write_resource::<GameState>();

        // swap input handler for modals
        match game_state.peek_player_request() {
            PlayerRequest::ViewInventory => {
                self.input_handler = Box::new(InventoryInputHandler::new())
            }
            PlayerRequest::ViewMap => self.input_handler = Box::new(MapInputHandler::new()),
            PlayerRequest::Selection => self.input_handler = Box::new(CrosshairsInputHandler::new()),
            PlayerRequest::CloseCurrentView => {
                self.ui.close_current_view();
                self.input_handler = Box::new(DefaultInputHandler::new())
            }
            _ => {}
        }
        let player_request = game_state.pop_player_request();
        player_request != PlayerRequest::Quit
    }

    pub fn update_game(&mut self) -> bool {
        self.dispatcher.dispatch(&mut self.world);
        self.world.maintain();
        let player_pos = self.get_player_pos();
        self.fov.compute_fov(
            player_pos.x as i32,
            player_pos.y as i32,
            TORCH_RADIUS,
            FOV_LIGHT_WALLS,
            FOV_ALGO,
        );
        true
    }

    // todo: stop calling this every frame (when possible)
    pub fn render(&mut self) {
        self.con.clear();
        self.ui.render(&mut self.con, &self.world, &self.fov);
        self.root.flush();
        blit(
            &self.con,
            (0, 0),
            (ScreenService::get_width(), ScreenService::get_height()),
            &mut self.root,
            (0, 0),
            1.0,
            1.0,
        );
    }

    pub fn get_player_pos(&self) -> Position {
        let pos_storage = self.world.read_storage::<Position>();
        let player_storage = self.world.read_storage::<Player>();
        let mut position: Position = Position::zero();
        for (pos, _) in (&pos_storage, &player_storage).join() {
            position = Position { x: pos.x, y: pos.y }
        }
        return position;
    }
}

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
    messages: Vec<String>,
    player_request: Option<PlayerRequest>,
    view_offset: [i32; 2],
    should_tick: bool,
}

impl GameState {
    fn new() -> GameState {
        GameState {
            messages: Vec::new(),
            player_request: None,
            view_offset: [0, 0],
            should_tick: false,
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

    /// Returns true if the game simulation should advance a tick this frame,
    /// which is set when the player issues an action that consumes a turn.
    pub fn should_tick(&self) -> bool {
        self.should_tick
    }

    pub fn clear_tick(&mut self) {
        self.should_tick = false;
    }

    pub fn pop_player_request(&mut self) -> PlayerRequest {
        if self.player_request == None {
            return PlayerRequest::None;
        }
        let request: PlayerRequest = self.player_request.unwrap();
        self.player_request = None;
        request
    }

    pub fn peek_player_request(&self) -> PlayerRequest {
        self.player_request.unwrap_or(PlayerRequest::None)
    }

    pub fn push_message(&mut self, msg: String) {
        self.messages.push(msg);
    }

    pub fn pop_message(&mut self) -> String {
        self.messages.pop().unwrap_or_default()
    }

    pub fn has_messages(&self) -> bool {
        self.messages.len() > 0
    }

    pub fn set_view_offset(&mut self, view_offset: [i32; 2]) {
        self.view_offset = view_offset;
    }

    pub fn get_view_offset(&self) -> [i32; 2] {
        self.view_offset
    }
}
