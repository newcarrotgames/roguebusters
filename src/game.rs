use crate::{
    components::{
        inventory::Inventory, item::Item, name::Name, player::Player, position::Position,
        renderable::Renderable, target::Target,
    },
    deser::{items::Items, prefabs::Prefabs},
    names::{NameType, Names},
    systems::{item_search::ItemSearch, simple_path::SimplePath},
    ui::{MESSAGES_HEIGHT, UI, UI_WIDTH},
    MAP_HEIGHT, MAP_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH, city::city::City,
};
use specs::{
    Builder, Dispatcher, DispatcherBuilder, Entity, Join, World,
    WorldExt,
};
use tcod::{
    colors::WHITE,
    console::{blit, Offscreen, Root},
    map::{FovAlgorithm, Map as FovMap},
    BackgroundFlag, Color, Console,
};

const TORCH_RADIUS: i32 = 60;
const FOV_LIGHT_WALLS: bool = true;
const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic;

// todo: break this up?
pub struct Game<'a> {
    pub root: Root,
    pub con: Offscreen,
    pub view_offset: [i32; 2],
    pub world: World,
    pub dispatcher: Dispatcher<'a, 'a>,
    pub fov: FovMap,
    pub game_state: GameState,
    pub ui: UI,
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
        for _ in 0..10 {
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
                .build();
        }

        log::info!("creating items");

        let mut items = Items::new();
        items.load_all("data/items");

        // add items
        for _ in 0..10 {
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

        let ui = UI::new();

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

        world.insert(GameState::new());

        log::info!("done creating game, returning player");

        Game {
            root,
            con,
            view_offset,
            world,
            dispatcher,
            fov,
            game_state: GameState::new(),
            ui,
        }
    }

    pub fn handle_request(&mut self, request: PlayerRequest) -> bool {
        let update;

        match request {
            PlayerRequest::WieldItem => update = true,
            PlayerRequest::DropItem => update = true,
            PlayerRequest::PickupItem => {
                self.pickup_item();
                update = true;
            }
            PlayerRequest::UseItem => update = true,
            PlayerRequest::Quit => update = false,
            PlayerRequest::None => update = false,
            PlayerRequest::Move(x, y) => {
                self.move_player_by(x as f32, y as f32);
                update = true;
            }
            PlayerRequest::Wait => update = true,
        }

        return update;
    }

    fn get_player_request(&mut self) -> PlayerRequest {
        let mut game_state = self.world.write_resource::<GameState>();
        let request = game_state.pop_player_request();
        request
    }

    pub fn update(&mut self) -> bool {
        let request = self.get_player_request();
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

    // pub fn get_player_tile(&mut self) -> Tile {
    //     return self.get_tile(self.get_player_pos());
    // }

    pub fn render_map(&mut self) {
        // let player_tile = self.get_player_tile();
        let mut map = self.world.write_resource::<City>();
        for vy in 0..self.root.height() - MESSAGES_HEIGHT {
            for vx in 0..self.root.width() - UI_WIDTH {
                let x = vx + self.view_offset[0];
                let y = vy + self.view_offset[1];
                let mut wall = map.data[y as usize][x as usize];
                let visible = self.fov.is_in_fov(x, y);
                // let char:char;
                // if wall.building_id > 0 && wall.building_id != player_tile.building_id {
                //     char = wall.exterior_char;
                // } else {
                //     char = wall.char;
                // }
                if visible {
                    self.con
                        .set_char_background(vx, vy, wall.bg_color, BackgroundFlag::Set);
                    self.con.set_default_foreground(wall.fg_color);
                    self.con.put_char(vx, vy, wall.char, BackgroundFlag::None);
                    wall.seen = true;
                    map.data[y as usize][x as usize] = wall;
                } else if wall.seen {
                    self.con.set_char_background(
                        vx,
                        vy,
                        self.fade(wall.bg_color),
                        BackgroundFlag::Set,
                    );
                    self.con.set_default_foreground(self.fade(wall.fg_color));
                    self.con.put_char(vx, vy, wall.char, BackgroundFlag::None);
                }
            }
        }
    }

    pub fn render_entities(&mut self) {
        let pos_storage = self.world.read_storage::<Position>();
        let ren_storage = self.world.read_storage::<Renderable>();
        for (pos, ren) in (&pos_storage, &ren_storage).join() {
            let cx = pos.x as i32 - self.view_offset[0];
            let cy = pos.y as i32 - self.view_offset[1];

            // check if offscreen
            if cx < 0 || cy < 0 || cx > SCREEN_WIDTH - 20 || cy > SCREEN_HEIGHT - MESSAGES_HEIGHT {
                continue;
            }

            let visible = self.fov.is_in_fov(pos.x as i32, pos.y as i32);
            if !visible {
                continue;
            }

            self.con.set_default_foreground(WHITE);
            self.con.put_char(cx, cy, ren.char, BackgroundFlag::None);
        }
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

    pub fn render_done(&mut self) {
        blit(
            &self.con,
            (0, 0),
            (SCREEN_WIDTH, SCREEN_HEIGHT),
            &mut self.root,
            (0, 0),
            1.0,
            1.0,
        );
    }

    pub fn render(&mut self) {
        self.con.clear();
        // render the screen
        self.render_map();
        self.render_entities();
        self.ui.render(&mut self.con);
        self.ui.update(&mut self.con, &self.world);
        self.render_done();
        self.root.flush();
    }

    pub fn blocked(&self, x: f32, y: f32) -> bool {
        let map = self.world.read_resource::<City>();
        return map.data[y as usize][x as usize].blocked;
    }

    pub fn move_player_by(&mut self, dx: f32, dy: f32) {
        let mut pos_storage = self.world.write_storage::<Position>();
        let player_storage = self.world.read_storage::<Player>();
        for (pos, _) in (&mut pos_storage, &player_storage).join() {
            if !self.blocked(pos.x + dx, pos.y + dy) {
                pos.x += dx;
                pos.y += dy;
                let map = self.world.read_resource::<City>();
                if self.view_offset[0] == 0
                    || self.view_offset[1] == 0
                    || self.view_offset[0] >= map.width
                    || self.view_offset[1] >= map.height
                {
                    continue;
                }
                if pos.x as i32 - self.view_offset[0] > self.root.width() - 63 {
                    self.view_offset[0] += 1;
                }
                if pos.y as i32 - self.view_offset[1] > self.root.height() - 35 {
                    self.view_offset[1] += 1;
                }
                if pos.x as i32 - self.view_offset[0] < 40 {
                    self.view_offset[0] -= 1;
                }
                if pos.y as i32 - self.view_offset[1] < 25 {
                    self.view_offset[1] -= 1;
                }
            }
        }
    }

    fn fade(&self, col: Color) -> Color {
        return Color::new(col.r / 4, col.g / 4, col.b / 4);
    }

    pub(crate) fn request(&mut self, req: PlayerRequest) {
        let mut game_state = self.world.write_resource::<GameState>();
        game_state.set_player_request(req);
    }

    fn pickup_item(&mut self) {
        log::info!("pickup item");
        let mut player_position: Position = Position::zero();
        let mut positions = self.world.write_storage::<Position>();
        let player_storage = self.world.read_storage::<Player>();
        let items = self.world.read_storage::<Item>();
        let mut inventories = self.world.write_storage::<Inventory>();
        let entities = self.world.entities();
        let mut ents_to_remove: Vec<Entity> = Vec::new();
        for (pos, _) in (&mut positions, &player_storage).join() {
            player_position = pos.clone();
        }
        let mut game_state = self.world.write_resource::<GameState>();
        let mut item_found = false;
        for (entity, item, pos) in (&entities, &items, &mut positions).join() {
            if player_position == *pos {
                item_found = true;
                for (_, inventory) in (&player_storage, &mut inventories).join() {
                    if inventory.push_item(item.clone()) {
                        ents_to_remove.push(entity.clone());
                        game_state.add_message(format!("You pick up a {}", item.name));
                        log::info!("inventory: {:?}", inventory);
                    } else {
                        game_state.add_message(format!("You can not pick up the {}", item.name));
                    }
                }
            }
        }
        if !item_found {
            game_state.add_message(format!("There is nothing to pick up."));
        }
        for e in ents_to_remove {
            positions.remove(e);
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum PlayerRequest {
    WieldItem,
    DropItem,
    PickupItem,
    UseItem,
    Quit,
    Move(i32, i32),
    None,
    Wait,
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
