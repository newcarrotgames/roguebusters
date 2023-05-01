use specs::{storage::VecStorage, Component};
use specs_derive::Component;

#[derive(Component, Clone, Debug, PartialEq)]
#[storage(VecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub(crate) fn zero() -> Position {
        return Position { x: 0.0, y: 0.0 }
    }
}