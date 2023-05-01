use specs::{storage::HashMapStorage, Component};
use specs_derive::Component;

#[derive(Component, Debug)]
#[storage(HashMapStorage)]
pub struct Player {}

