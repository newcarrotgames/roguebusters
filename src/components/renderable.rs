use bracket_lib::prelude::RGB;
use specs::{storage::VecStorage, Component};
use specs_derive::Component;

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Renderable {
    pub char: char,
    pub color: RGB,
}
