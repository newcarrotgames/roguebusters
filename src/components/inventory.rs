use specs::{storage::VecStorage, Component};
use specs_derive::Component;

use super::item::Item;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Inventory {
    items: Vec<Item>,
}

impl Inventory {
    pub fn new() -> Self {
        Inventory { items: Vec::new() }
    }

    pub fn push_item(&mut self, item: Item) -> bool {
        // check if the ent has room for the item
        self.items.push(item);
        true
    }

    pub fn items(&self) -> &Vec<Item> {
        &self.items
    }
}