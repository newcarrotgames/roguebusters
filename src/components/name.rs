use specs::{storage::VecStorage, Component};
use specs_derive::Component;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Name {
    pub name: String,
}