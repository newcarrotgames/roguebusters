use std::collections::VecDeque;

use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::game::PlayerRequest;

use super::play_tester::PlayTester;

/// Decides the next `PlayerRequest` given the current play-tester state.
pub trait Agent {
    fn next_action(&mut self, tester: &PlayTester) -> PlayerRequest;
}

// ── ScriptedAgent ─────────────────────────────────────────────────────────────

/// Replays a fixed sequence of actions, then emits `Wait` indefinitely.
/// Useful for deterministic regression tests.
pub struct ScriptedAgent {
    actions: VecDeque<PlayerRequest>,
}

impl ScriptedAgent {
    pub fn new(actions: impl IntoIterator<Item = PlayerRequest>) -> Self {
        ScriptedAgent {
            actions: actions.into_iter().collect(),
        }
    }
}

impl Agent for ScriptedAgent {
    fn next_action(&mut self, _tester: &PlayTester) -> PlayerRequest {
        self.actions.pop_front().unwrap_or(PlayerRequest::Wait)
    }
}

// ── RandomAgent ───────────────────────────────────────────────────────────────

/// Picks uniformly at random from a small set of safe actions.
/// Provide a seed for reproducible fuzz runs.
pub struct RandomAgent {
    rng: StdRng,
}

impl RandomAgent {
    pub fn new(seed: u64) -> Self {
        RandomAgent {
            rng: StdRng::seed_from_u64(seed),
        }
    }
}

impl Agent for RandomAgent {
    fn next_action(&mut self, _tester: &PlayTester) -> PlayerRequest {
        let choice = self.rng.gen_range(0..10u32);
        match choice {
            0 => PlayerRequest::Move(-1,  0),
            1 => PlayerRequest::Move( 1,  0),
            2 => PlayerRequest::Move( 0, -1),
            3 => PlayerRequest::Move( 0,  1),
            4 => PlayerRequest::Move(-1, -1),
            5 => PlayerRequest::Move( 1, -1),
            6 => PlayerRequest::Move(-1,  1),
            7 => PlayerRequest::Move( 1,  1),
            8 => PlayerRequest::PickupItem,
            _ => PlayerRequest::Wait,
        }
    }
}

// ── WalkAgent ─────────────────────────────────────────────────────────────────

/// Greedily moves the player toward `(target_x, target_y)`, then waits.
pub struct WalkAgent {
    pub target_x: i32,
    pub target_y: i32,
}

impl WalkAgent {
    pub fn new(target_x: i32, target_y: i32) -> Self {
        WalkAgent { target_x, target_y }
    }
}

impl Agent for WalkAgent {
    fn next_action(&mut self, tester: &PlayTester) -> PlayerRequest {
        let pos = tester.player_position();
        let dx = (self.target_x - pos.x as i32).signum();
        let dy = (self.target_y - pos.y as i32).signum();
        if dx == 0 && dy == 0 {
            PlayerRequest::Wait
        } else {
            PlayerRequest::Move(dx, dy)
        }
    }
}
