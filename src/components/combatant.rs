use specs::{storage::VecStorage, Component, Entity};
use specs_derive::Component;

/// Placed on the ATTACKER entity. Stores the entity being attacked.
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Combatant {
    pub target: Entity,
}