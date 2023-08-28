use crate::{
    components::{
        inventory::Inventory, item::Item, name::Name, player::Player, position::Position,
        renderable::Renderable, target::Target, equipped::Equipped,
    },
    deser::{items::Items, prefabs::Prefabs},
    names::{NameType, Names},
    systems::{item_search::ItemSearch, simple_path::SimplePath},
    MAP_HEIGHT, MAP_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH, city::city::City, input::handlers::{InputHandler, DefaultInputHandler}, ui::ui::{UI, MESSAGES_HEIGHT, UI_WIDTH, UIState}, render::renderer::Renderer,
};
use specs::{
    Builder, Dispatcher, DispatcherBuilder, Entity, Join, World,
    WorldExt,
};
use tcod::{
    colors::WHITE,
    console::{blit, Offscreen, Root},
    map::{FovAlgorithm, Map as FovMap},
    BackgroundFlag, Color, Console, input::{KEY_PRESSED, KEY_PRESS, KeyCode},
};

const TORCH_RADIUS: i32 = 75;
const FOV_LIGHT_WALLS: bool = true;
const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic;
const NUM_NPCS: i32 = 100;
const NUM_ITEMS: i32 = 100;

// todo: break this up?
pub struct Game<'a> {
    pub root: Root,
    pub con: Offscreen,
    pub world: World,
    pub dispatcher: Dispatcher<'a, 'a>,
    pub fov: FovMap,
    pub ui: UI,
    pub input_handler: Box<dyn InputHandler>,
    pub renderer: Renderer,
}

impl Game<'_> {
    pub fn new(root: Root, con: Offscreen, prefabs: Prefabs) -> Self {
        let w = con.width();
        let h = con.height();

        log::info!("creating specs world");

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
        world.register::<Equipped>();

        // create specs dispatcher
        let dispatcher = DispatcherBuilder::new()
            .with(SimplePath, "simple_path", &[])
            .with(ItemSearch, "item_search", &[])
            .build();

        log::info!("creating city map");
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

        log::info!("creating npcs");

        // add npcs
        for _ in 0..NUM_NPCS {
            let position = map.get_random_target();
            let target = map.get_random_target();
            world
                .create_entity()
                .with(position)
                .with(Renderable {
                    char: 2 as char,
                    color: WHITE,
                })
                .with(Target {
                    x: target.x,
                    y: target.y,
                })
                .with(Name {
                    name: names.get_random_name(NameType::AnyFullName),
                })
                .with(Equipped::new())
                .build();
        }

        log::info!("creating items");

        // todo: should items static?
        let mut items = Items::new();
        items.load_all("data/items");

        // add items
        for _ in 0..NUM_ITEMS {
            let item = items.random_item();
            let position = map.get_random_target();
            world
                .create_entity()
                .with(Item {
                    name: item.name.clone(),
                    item_type: item.item_type.clone(),
                    subtype: item.subtype.clone(),
                    price: item.price,
                })
                .with(Renderable {
                    char: item.char as char,
                    color: WHITE,
                })
                .with(position)
                .build();
        }

        log::info!("creating player");

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
                color: WHITE,
            })
            .build();

        world.insert(map);

        let mut view_offset = [(position.x - 50.0) as i32, (position.y - 30.0) as i32];
        if view_offset[0] < 0 {
            view_offset[0] = 0;
        }
        if view_offset[1] < 0 {
            view_offset[1] = 0;
        }

        if view_offset[0] > MAP_WIDTH - SCREEN_WIDTH {
            view_offset[0] = MAP_WIDTH - SCREEN_WIDTH
        }

        if view_offset[0] > MAP_HEIGHT - SCREEN_HEIGHT {
            view_offset[0] = MAP_HEIGHT - SCREEN_HEIGHT
        }

        let ui = UI::new(view_offset);

        world.insert(GameState::new());

        log::info!("done creating game, returning player");

        let input_handler: Box<dyn InputHandler> = Box::new(DefaultInputHandler::new());

        let renderer = Renderer::new(view_offset);

        Game {
            root,
            con,
            world,
            dispatcher,
            fov,
            ui,
            input_handler,
            renderer,
        }
    }

    // pub fn handle_request(&mut self, request: PlayerRequest) -> bool {
    //     let update;

    //     match request {
    //         PlayerRequest::WieldItem => update = true,
    //         PlayerRequest::DropItem => update = true,
    //         PlayerRequest::PickupItem => {
    //             self.pickup_item();
    //             update = true;
    //         }
    //         PlayerRequest::UseItem => update = true,
    //         PlayerRequest::Quit => update = false,
    //         PlayerRequest::None => update = false,
    //         PlayerRequest::Move(x, y) => {
    //             self.move_player_by(x as f32, y as f32);
    //             update = true;
    //         }
    //         PlayerRequest::Wait => update = true,
    //         PlayerRequest::ToggleFullscreen => {
    //             let fullscreen = self.root.is_fullscreen();
    //             self.root.set_fullscreen(!fullscreen);
    //             update = false;
    //         }
    //         PlayerRequest::ViewInventory => {
    //             self.ui.set_state(UIState::Inventory);
    //             update = false;
    //         }
    //         PlayerRequest::ViewMap => {
    //             self.ui.set_state(UIState::Map);
    //             update = false;
    //         }
    //         PlayerRequest::CloseCurrentView => {
    //             self.ui.set_state(UIState::None);
    //             update = false;
    //         }
    //         PlayerRequest::ModalRequest(_) => todo!(),
    //     }

    //     return update;
    // }

    fn get_player_request(&mut self) -> PlayerRequest {
        let mut game_state = self.world.write_resource::<GameState>();
        let request = game_state.pop_player_request();
        request
    }

    pub fn update(&mut self) -> bool {
        let mut request:PlayerRequest = PlayerRequest::None;
        let key = self.root.check_for_keypress(KEY_PRESSED);
        if key != None {
            let actual_key = key.unwrap();
            // log::info!("key pressed: {:?}", actual_key);
            if actual_key.code != KeyCode::Text {
                request = self.input_handler.handle_input(actual_key);
            }
        }
        
        if request == PlayerRequest::Quit {
            return false;
        }

        if self.handle_request(request) {
            self.update_game();
        }
        true
    }

    pub fn update_game(&mut self) {
        self.dispatcher.dispatch(&mut self.world);
        self.world.maintain();
        let player_pos = self.get_player_pos();
        let mut game_state = self.world.write_resource::<GameState>();
        while game_state.has_messages() {
            self.ui.add_message(game_state.pop_message().as_str());
        }
        self.fov.compute_fov(
            player_pos.x as i32,
            player_pos.y as i32,
            TORCH_RADIUS,
            FOV_LIGHT_WALLS,
            FOV_ALGO,
        );
    }

    pub fn render(&mut self) {
        self.renderer.render(&mut self.con, &self.world, &mut self.root, &self.fov);
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

    // 
    pub fn blocked(&self, x: f32, y: f32) -> bool {
        let map = self.world.read_resource::<City>();
        return map.data[y as usize][x as usize].blocked;
    }

    

    pub(crate) fn request(&mut self, req: PlayerRequest) {
        let mut game_state = self.world.write_resource::<GameState>();
        game_state.set_player_request(req);
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
    Wait,
    WieldItem,
    ModalRequest(ModalPlayerRequest)
}

#[derive(Default)]
pub struct GameState {
    player_request: Option<PlayerRequest>,
    messages: Vec<String>,
}

impl GameState {
    fn new() -> GameState {
        GameState {
            player_request: None,
            messages: Vec::new(),
        }
    }

    pub fn set_player_request(&mut self, req: PlayerRequest) {
        self.player_request = Some(req);
    }

    pub fn get_player_request(&mut self) -> Option<PlayerRequest> {
        self.player_request
    }

    pub fn pop_player_request(&mut self) -> PlayerRequest {
        if self.player_request == None || self.player_request == Some(PlayerRequest::None) {
            return PlayerRequest::None;
        }
        let r = self.player_request;
        self.player_request = Some(PlayerRequest::None);
        r.unwrap()
    }

    pub fn add_message(&mut self, msg: String) {
        self.messages.push(msg);
    }

    pub fn has_messages(&self) -> bool {
        self.messages.len() > 0
    }

    pub fn pop_message(&mut self) -> String {
        self.messages.pop().unwrap_or_default()
    }
}
