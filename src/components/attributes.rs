use specs::{storage::VecStorage, Component};
use specs_derive::Component;

use crate::util::rng::Dice;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Attributes {
    brawn: i32,
    agility: i32,
    stamina: i32,
    perception: i32,
    fortune: i32,
    charm: i32,
    health: i32,
}

impl Attributes {
    pub fn new() -> Self {
        Attributes {
            brawn: 0,
            agility: 0,
            stamina: 0,
            perception: 0,
            fortune: 0,
            charm: 0,
            health: 0,
        }
    }

    // random attributes
    pub fn random() -> Self {
        let mut dice = Dice::new();
        let mut attributes = Attributes::new();
        attributes.set_brawn(dice.roll_3d6());
        attributes.set_agility(dice.roll_3d6());
        attributes.set_stamina(dice.roll_3d6());
        attributes.set_perception(dice.roll_3d6());
        attributes.set_fortune(dice.roll_3d6());
        attributes.set_charm(dice.roll_3d6());
        attributes.set_health(attributes.stamina());
        attributes
    }

    pub fn brawn(&self) -> i32 {
        self.brawn
    }

    pub fn agility(&self) -> i32 {
        self.agility
    }

    pub fn stamina(&self) -> i32 {
        self.stamina
    }

    pub fn perception(&self) -> i32 {
        self.perception
    }

    pub fn fortune(&self) -> i32 {
        self.fortune
    }

    pub fn charm(&self) -> i32 {
        self.charm
    }

    pub fn set_brawn(&mut self, brawn: i32) {
        self.brawn = brawn;
    }

    pub fn set_agility(&mut self, agility: i32) {
        self.agility = agility;
    }

    pub fn set_stamina(&mut self, stamina: i32) {
        self.stamina = stamina;
    }

    pub fn set_perception(&mut self, perception: i32) {
        self.perception = perception;
    }

    pub fn set_fortune(&mut self, fortune: i32) {
        self.fortune = fortune;
    }

    pub fn set_charm(&mut self, charm: i32) {
        self.charm = charm;
    }

    pub fn health(&self) -> i32 {
        self.health
    }

    pub fn set_health(&mut self, health: i32) {
        self.health = health;
    }
}