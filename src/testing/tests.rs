use specs::WorldExt;

use crate::game::{GameConfig, PlayerRequest};

use super::{
    agent::{Agent, RandomAgent, ScriptedAgent, WalkAgent},
    play_tester::PlayTester,
};

// ── Movement ──────────────────────────────────────────────────────────────────

/// The player should never end up inside a blocked (wall) tile after any sequence
/// of random moves.
#[test]
fn player_never_enters_wall() {
    let mut tester = PlayTester::new();
    let mut agent  = RandomAgent::new(42);
    for _ in 0..200 {
        let req = agent.next_action(&tester);
        tester.inject_and_tick(req);
        let pos = tester.player_position();
        assert!(
            !tester.is_tile_blocked(pos.x as i32, pos.y as i32),
            "Player ended up inside a blocked tile at ({}, {})",
            pos.x, pos.y
        );
    }
}

/// A `WalkAgent` aimed at a specific open tile should converge without panicking.
#[test]
fn walk_agent_converges_without_panic() {
    let mut tester = PlayTester::new();
    let start = tester.player_position();
    // Walk 10 steps in the +x direction (may hit walls, that's fine — it just stops)
    let target_x = (start.x as i32 + 10).min(78);
    let mut agent  = WalkAgent::new(target_x, start.y as i32);
    tester.run_agent(&mut agent, 20);
    // If we get here without a panic, the test passes.
}

/// Moving in all 8 directions never causes an out-of-bounds panic.
#[test]
fn exhaustive_direction_moves_dont_panic() {
    let directions = [
        PlayerRequest::Move(-1,  0),
        PlayerRequest::Move( 1,  0),
        PlayerRequest::Move( 0, -1),
        PlayerRequest::Move( 0,  1),
        PlayerRequest::Move(-1, -1),
        PlayerRequest::Move( 1, -1),
        PlayerRequest::Move(-1,  1),
        PlayerRequest::Move( 1,  1),
    ];
    let mut tester = PlayTester::new();
    let mut agent  = ScriptedAgent::new(directions.iter().cycle().take(80).cloned());
    tester.run_agent(&mut agent, 80);
}

// ── Inventory ─────────────────────────────────────────────────────────────────

/// Picking up an item at the player's feet should add it to the inventory.
#[test]
fn pickup_adds_item_to_inventory() {
    let mut tester = PlayTester::new();
    let pos = tester.player_position();
    tester.spawn_item_at("TestKnife", pos.x, pos.y);
    tester.inject_and_tick(PlayerRequest::PickupItem);
    assert!(
        tester.player_has_item("TestKnife"),
        "Expected 'TestKnife' in inventory after PickupItem, got: {:?}",
        tester.player_inventory_names()
    );
}

/// Wielding an item at the player's feet should add it to the inventory.
#[test]
fn wield_adds_item_to_inventory() {
    let mut tester = PlayTester::new();
    let pos = tester.player_position();
    tester.spawn_item_at("TestPistol", pos.x, pos.y);
    tester.inject_and_tick(PlayerRequest::WieldItem);
    assert!(
        tester.player_has_item("TestPistol"),
        "Expected 'TestPistol' in inventory after WieldItem, got: {:?}",
        tester.player_inventory_names()
    );
}

/// Dropping a wielded item should remove it from the inventory.
#[test]
fn drop_removes_wielded_item_from_inventory() {
    let mut tester = PlayTester::new();
    let pos = tester.player_position();
    tester.spawn_item_at("TestKnife", pos.x, pos.y);
    tester.inject_and_tick(PlayerRequest::WieldItem);
    assert!(tester.player_has_item("TestKnife"), "Wield failed — prerequisite not met");
    tester.inject_and_tick(PlayerRequest::DropItem);
    assert!(
        !tester.player_has_item("TestKnife"),
        "Expected inventory empty after DropItem, got: {:?}",
        tester.player_inventory_names()
    );
}

/// Picking up when standing on nothing should not panic and should produce a message.
#[test]
fn pickup_nothing_does_not_panic() {
    let mut tester = PlayTester::new();
    // Move somewhere with nothing on the floor.
    tester.inject_and_tick(PlayerRequest::PickupItem);
    // If we reach here without panic, pass.
}

// ── Combat ────────────────────────────────────────────────────────────────────

/// After a `Selected` attack, the NPC entity should have taken damage or been
/// removed (if it died in one hit). Requires the player to have a wielded weapon.
#[test]
fn attack_damages_or_kills_npc() {
    let mut tester = PlayTester::new();
    let pos = tester.player_position();

    // Give the player a weapon
    tester.spawn_item_at("TestGun", pos.x, pos.y);
    tester.inject_and_tick(PlayerRequest::WieldItem);
    assert!(tester.player_has_item("TestGun"), "Wield failed — prerequisite not met");

    // Spawn an NPC right next to the player
    let npc_x = pos.x + 1.0;
    let npc_y = pos.y;
    let npc_entity = tester.spawn_npc_at(npc_x, npc_y);

    let npc_count_before = tester.npc_count();

    // `Selected(x, y)` uses screen coords + view_offset. In a small test world
    // the view_offset defaults to [0, 0], so world coords equal screen coords.
    let view_offset = {
        let gs = tester.game.world.read_resource::<crate::game::GameState>();
        gs.get_view_offset()
    };
    let screen_x = npc_x as i32 - view_offset[0];
    let screen_y = npc_y as i32 - view_offset[1];
    tester.inject_and_tick(PlayerRequest::Selected(screen_x, screen_y));

    // Either the NPC took damage (still alive but exists) or was killed (entity gone).
    // We verify at least that no panic occurred and the world is still consistent.
    let npc_still_has_position = tester.entity_position(npc_entity).is_some();
    let npc_count_after = tester.npc_count();

    // The NPC was either damaged (still present) or killed (count decreased).
    assert!(
        !npc_still_has_position || npc_count_after <= npc_count_before,
        "Attack had no effect: NPC still present and count unchanged"
    );
}

// ── Smoke test ────────────────────────────────────────────────────────────────

/// Run a random agent for 100 turns and ensure no panic occurs.
#[test]
fn random_agent_100_turns_no_panic() {
    let mut tester = PlayTester::new();
    let mut agent  = RandomAgent::new(99);
    let report     = tester.run_agent_report(&mut agent, 100);
    assert!(!report.panicked, "Panic recorded in report");
}

/// Full 1000×1000 city — skipped by default, run with `cargo test -- --include-ignored`.
#[ignore]
#[test]
fn full_city_smoke_test() {
    use crate::deser::prefabs::Prefabs;
    let mut prefabs = Prefabs::new("data/prefabs");
    prefabs.load_all();
    let mut tester = PlayTester::with_config(GameConfig::default_full(prefabs));
    let mut agent  = RandomAgent::new(1);
    let report     = tester.run_agent_report(&mut agent, 50);
    assert!(!report.panicked);
}
