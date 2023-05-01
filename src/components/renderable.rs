use specs::{storage::VecStorage, Component};
use specs_derive::Component;
use tcod::Color;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Renderable {
    pub char: char,
    pub color: Color,
}