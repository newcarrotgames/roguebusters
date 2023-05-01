use specs::{storage::VecStorage, Component};
use specs_derive::Component;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Target {
    pub x: f32,
    pub y: f32,
}