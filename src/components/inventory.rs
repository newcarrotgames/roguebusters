use specs::{storage::VecStorage, Component};
use specs_derive::Component;

use super::item::Item;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Inventory {
    items: Vec<Item>,
    equipped: Equipped,
}

impl Inventory {
    pub fn new() -> Self {
        Inventory {
            items: Vec::new(),
            equipped: Equipped::new(),
        }
    }

    pub fn push_item(&mut self, item: Item) -> bool {
        // check if the ent has room for the item
        self.items.push(item);
        true
    }

    pub fn items(&self) -> &Vec<Item> {
        &self.items
    }

    pub fn equipped(&self) -> &Equipped {
        &self.equipped
    }

    pub fn equip(&mut self, item: Item, location: EquipLocation) {
        match location {
            EquipLocation::LeftHand => self.equipped.left_hand = Some(item),
            EquipLocation::RightHand => self.equipped.right_hand = Some(item),
            EquipLocation::Body => self.equipped.body = Some(item),
            EquipLocation::Head => self.equipped.head = Some(item),
        }
    }

    pub fn unequip(&mut self, item: Item, location: EquipLocation) {
        match location {
            EquipLocation::LeftHand => self.equipped.left_hand = None,
            EquipLocation::RightHand => self.equipped.right_hand = None,
            EquipLocation::Body => self.equipped.body = None,
            EquipLocation::Head => self.equipped.head = None,
        }
    }

    pub fn equipped_item(&self, location: EquipLocation) -> Option<&Item> {
        match location {
            EquipLocation::LeftHand => self.equipped.left_hand.as_ref(),
            EquipLocation::RightHand => self.equipped.right_hand.as_ref(),
            EquipLocation::Body => self.equipped.body.as_ref(),
            EquipLocation::Head => self.equipped.head.as_ref(),
        }
    }

    pub(crate) fn get_item(&self, selected_item: usize) -> Item {
        self.items[selected_item].clone()
    }
}

pub enum EquipLocation {
    LeftHand,
    RightHand,
    Body,
    Head,
}

#[derive(Debug)]
pub struct Equipped {
    left_hand: Option<Item>,
    right_hand: Option<Item>,
    body: Option<Item>,
    head: Option<Item>,
}

impl Equipped {
    pub fn new() -> Self {
        Equipped {
            left_hand: None,
            right_hand: None,
            body: None,
            head: None,
        }
    }
}
