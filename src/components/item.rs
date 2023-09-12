use specs::{storage::VecStorage, Component};
use specs_derive::Component;

#[derive(Component, Clone, Debug, PartialEq)]
#[storage(VecStorage)]
pub struct Item {
    pub name: String,
    pub item_type: String,
    pub subtype: String,
    pub range: i32,
    pub damage: i32,
    pub rate: i32,
    pub accuracy: f32,
    pub ammo: i32,
    pub price: f32,
}

impl Item {
    pub(crate) fn from_itemdata(clone: crate::deser::items::ItemData) -> Item {
        Item {
            name: clone.name.clone(),
            item_type: clone.item_type.clone(),
            subtype: clone.subtype.clone(),
            range: clone.range,
            damage: clone.damage,
            rate: clone.rate,
            accuracy: clone.accuracy.clone(),
            ammo: clone.ammo,
            price: clone.price,
        }
    }
}
