use bracket_lib::prelude::RGB;
use specs::{Builder, Entity, Join, WorldExt};

use crate::{
    components::{
        attributes::Attributes,
        inventory::Inventory,
        name::Name,
        npc::NPC,
        player::Player,
        position::Position,
        renderable::Renderable,
        target::Target,
    },
    game::{Game, GameConfig, GameState, PlayerRequest},
};

use super::{
    agent::Agent,
    report::TestReport,
    scenario::make_test_item,
};

pub struct PlayTester {
    pub game: Game,
}

impl PlayTester {
    /// Create a play-tester with a small, fast world (no disk I/O for items/prefabs).
    pub fn new() -> Self {
        let game = Game::new_with_config(GameConfig::small_test());
        PlayTester { game }
    }

    /// Create a play-tester with explicit configuration.
    pub fn with_config(config: GameConfig) -> Self {
        let game = Game::new_with_config(config);
        PlayTester { game }
    }

    // ── Drive ─────────────────────────────────────────────────────────────────

    /// Push a `PlayerRequest` directly into `GameState` and run one ECS tick.
    pub fn inject_and_tick(&mut self, req: PlayerRequest) {
        {
            let mut gs = self.game.world.write_resource::<GameState>();
            gs.push_player_request(req);
        }
        self.game.update_game();
    }

    /// Run an agent for `turns` ticks, ignoring game-over.
    pub fn run_agent(&mut self, agent: &mut dyn Agent, turns: usize) {
        for _ in 0..turns {
            let req = agent.next_action(self);
            self.inject_and_tick(req);
        }
    }

    /// Like `run_agent` but returns a `TestReport`. Catches panics so a single
    /// bad tick doesn't abort the whole test suite.
    pub fn run_agent_report(&mut self, agent: &mut dyn Agent, turns: usize) -> TestReport {
        let mut report = TestReport::new();
        for _ in 0..turns {
            if self.is_game_over() {
                report.game_over = true;
                break;
            }
            let req = agent.next_action(self);
            // Drain messages before the tick so we capture what the tick produces.
            let req_copy = req;
            {
                let mut gs = self.game.world.write_resource::<GameState>();
                gs.push_player_request(req_copy);
            }
            self.game.update_game();
            report.turns_run += 1;
            // Collect any new messages into the report notes.
            let mut gs = self.game.world.write_resource::<GameState>();
            while gs.has_messages() {
                report.add_note(gs.pop_message());
            }
        }
        if self.is_game_over() {
            report.game_over = true;
        }
        report
    }

    // ── Inspect world state ───────────────────────────────────────────────────

    pub fn player_position(&self) -> Position {
        self.game.get_player_pos()
    }

    /// Names of all items currently in the player's inventory bag.
    pub fn player_inventory_names(&self) -> Vec<String> {
        let inventories = self.game.world.read_storage::<Inventory>();
        let players     = self.game.world.read_storage::<Player>();
        for (inv, _) in (&inventories, &players).join() {
            return inv.items().iter().map(|i| i.name.clone()).collect();
        }
        Vec::new()
    }

    pub fn player_has_item(&self, name: &str) -> bool {
        self.player_inventory_names()
            .iter()
            .any(|n| n.eq_ignore_ascii_case(name))
    }

    /// All messages currently queued in `GameState` (does not consume them).
    pub fn messages(&self) -> Vec<String> {
        // GameState doesn't expose a peek-all, so we read the resource directly.
        // Reading requires a write lock because the pop_message API is &mut.
        // Instead we expose a clone of the internal vec via a read borrow.
        // Since GameState fields are private we drain and re-insert.
        let mut gs = self.game.world.write_resource::<GameState>();
        let mut msgs = Vec::new();
        while gs.has_messages() {
            msgs.push(gs.pop_message());
        }
        // Re-push in original order (pop reverses because it's a Vec).
        msgs.reverse();
        for m in &msgs {
            gs.push_message(m.clone());
        }
        msgs
    }

    pub fn is_tile_blocked(&self, x: i32, y: i32) -> bool {
        use crate::city::city::City;
        let city = self.game.world.read_resource::<City>();
        let y = y.max(0) as usize;
        let x = x.max(0) as usize;
        if y >= city.data.len() || city.data[y].is_empty() || x >= city.data[y].len() {
            return true;
        }
        city.data[y][x].blocked
    }

    pub fn npc_count(&self) -> usize {
        let npcs = self.game.world.read_storage::<NPC>();
        (&npcs).join().count()
    }

    pub fn is_game_over(&self) -> bool {
        self.game.world.read_resource::<GameState>().game_over
    }

    // ── Mutate world (scenario helpers) ──────────────────────────────────────

    /// Teleport the player to world coordinates `(x, y)`.
    pub fn teleport_player(&mut self, x: f32, y: f32) {
        let mut positions = self.game.world.write_storage::<Position>();
        let players       = self.game.world.read_storage::<Player>();
        for (pos, _) in (&mut positions, &players).join() {
            pos.x = x;
            pos.y = y;
        }
    }

    /// Spawn a named item at world coordinates so it can be picked up.
    pub fn spawn_item_at(&mut self, name: &str, x: f32, y: f32) {
        let item = make_test_item(name);
        self.game.world
            .create_entity()
            .with(item)
            .with(Position { x, y })
            .with(Renderable { char: '!', color: RGB::from_u8(255, 255, 0) })
            .build();
        self.game.world.maintain();
    }

    /// Spawn a generic hostile NPC at world coordinates. Returns the entity.
    pub fn spawn_npc_at(&mut self, x: f32, y: f32) -> Entity {
        let entity = self.game.world
            .create_entity()
            .with(NPC::new())
            .with(Position { x, y })
            .with(Renderable { char: 'N', color: RGB::from_u8(255, 0, 0) })
            .with(Target { x, y })
            .with(Name { name: "Test NPC".to_string() })
            .with(Attributes::random())
            .with(Inventory::new())
            .build();
        self.game.world.maintain();
        entity
    }

    /// Returns the `Position` of a specific entity, if it still has one.
    pub fn entity_position(&self, entity: Entity) -> Option<Position> {
        let positions = self.game.world.read_storage::<Position>();
        positions.get(entity).cloned()
    }
}
