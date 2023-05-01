use specs::{storage::VecStorage, Component};
use specs_derive::Component;

#[derive(Component, Clone, Debug)]
#[storage(VecStorage)]
pub struct Item {
    pub name: String,
    pub item_type: String,
    pub subtype: String,
    pub price: f32,
}