use specs::{storage::VecStorage, Component};
use specs_derive::Component;

use super::item::Item;

pub enum EquipLocation {
    LeftHand,
    RightHand,
    Body,
    Head,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Equipped {
    left_hand: Option<Item>,
	right_hand: Option<Item>,
	body: Option<Item>,
	head: Option<Item>,
}

impl Equipped {
    pub fn new() -> Self {
        Equipped { left_hand: None, right_hand: None, body: None, head: None }
    }

    pub fn equip_item(&mut self, item: Item, location: EquipLocation) -> bool {
		match location {
			EquipLocation::LeftHand => {
				if self.left_hand.is_some() {
					return false;
				}
				self.left_hand = Some(item);
				return true;
			}
			EquipLocation::RightHand => {
				if self.right_hand.is_some() {
					return false;
				}
				self.right_hand = Some(item);
				return true;
			}
			EquipLocation::Body => {
				if self.body.is_some() {
					return false;
				}
				self.body = Some(item);
				return true;
			}
			EquipLocation::Head => {
				if self.head.is_some() {
					return false;
				}
				self.head = Some(item);
				return true;
			}
		}
    }

	pub fn unequip_item(&mut self, location: EquipLocation) -> bool {
		match location {
			EquipLocation::LeftHand => {
				if self.left_hand.is_none() {
					return false;
				}
				self.left_hand = None;
				return true;
			}
			EquipLocation::RightHand => {
				if self.right_hand.is_none() {
					return false;
				}
				self.right_hand = None;
				return true;
			}
			EquipLocation::Body => {
				if self.body.is_none() {
					return false;
				}
				self.body = None;
				return true;
			}
			EquipLocation::Head => {
				if self.head.is_none() {
					return false;
				}
				self.head = None;
				return true;
			}
		}
	}
}