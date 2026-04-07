use specs::{storage::HashMapStorage, Component};
use specs_derive::Component;

#[derive(Component, Debug)]
#[storage(HashMapStorage)]
pub struct NPCMemory {
    pub last_seen_x: f32,
    pub last_seen_y: f32,
    pub search_ticks_remaining: i32,
}
