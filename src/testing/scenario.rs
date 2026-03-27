use crate::components::item::Item;

use super::play_tester::PlayTester;

/// Fluent builder that modifies a `PlayTester`'s world after initial construction.
#[derive(Default)]
pub struct ScenarioBuilder {
    player_pos:  Option<(f32, f32)>,
    items:       Vec<(String, f32, f32)>,
    npcs:        Vec<(f32, f32)>,
}

impl ScenarioBuilder {
    pub fn new() -> Self {
        ScenarioBuilder::default()
    }

    /// Teleport the player to the given world coordinates.
    pub fn player_at(mut self, x: f32, y: f32) -> Self {
        self.player_pos = Some((x, y));
        self
    }

    /// Spawn an item with the given name at world coordinates.
    pub fn item_at(mut self, name: &str, x: f32, y: f32) -> Self {
        self.items.push((name.to_string(), x, y));
        self
    }

    /// Spawn a generic NPC at world coordinates.
    pub fn npc_at(mut self, x: f32, y: f32) -> Self {
        self.npcs.push((x, y));
        self
    }

    /// Apply all staged mutations to the play-tester's world.
    pub fn apply(self, tester: &mut PlayTester) {
        if let Some((x, y)) = self.player_pos {
            tester.teleport_player(x, y);
        }
        for (name, x, y) in self.items {
            tester.spawn_item_at(&name, x, y);
        }
        for (x, y) in self.npcs {
            tester.spawn_npc_at(x, y);
        }
    }
}

/// Helper: build a minimal `Item` component from just a name (for test scenarios).
pub fn make_test_item(name: &str) -> Item {
    Item {
        name:      name.to_string(),
        item_type: "test".to_string(),
        subtype:   "test".to_string(),
        range:     1,
        damage:    1,
        rate:      1,
        accuracy:  1.0,
        ammo:      0,
        price:     0.0,
        char:      '!',
    }
}
