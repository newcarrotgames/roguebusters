use specs::{storage::HashMapStorage, Component};
use specs_derive::Component;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
#[allow(dead_code)]
pub enum NPCState {
    Hostile,
	Searching,
	Fleeing,
	Dead,
	Hiding
}

#[derive(Component, Debug)]
#[storage(HashMapStorage)]
pub struct NPC {
	states: Vec<NPCState>,
	pub hostile_on_sight: bool,
}

impl NPC {
	pub fn new() -> NPC {
		NPC {
			states: Vec::new(),
			hostile_on_sight: false,
		}
	}

	pub fn aggressive() -> NPC {
		NPC {
			states: Vec::new(),
			hostile_on_sight: true,
		}
	}

	pub fn add_state(&mut self, state: NPCState) {
		self.states.push(state);
	}

	pub fn has_state(&self, state: NPCState) -> bool {
		self.states.contains(&state)
	}

	#[allow(dead_code)]
	pub fn get_states(&self) -> &Vec<NPCState> {
		&self.states
	}

	pub fn remove_state(&mut self, state: NPCState) {
		self.states.retain(|&s| s != state);
	}

	#[allow(dead_code)]
	pub fn remove_all_states(&mut self) {
		self.states.clear();
	}
}
