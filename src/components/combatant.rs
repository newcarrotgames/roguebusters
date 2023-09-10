use specs::{storage::VecStorage, Component, Entity};
use specs_derive::Component;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Combatant {
	pub entity: Entity,
}