use specs::{storage::VecStorage, Component};
use specs_derive::Component;

use crate::util::rng::Dice;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Attributes {
    brawn: u8,
    agility: u8,
    stamina: u8,
    perception: u8,
    fortune: u8,
    charm: u8,
    health: u8,
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

    pub fn brawn(&self) -> u8 {
        self.brawn
    }

    pub fn agility(&self) -> u8 {
        self.agility
    }

    pub fn stamina(&self) -> u8 {
        self.stamina
    }

    pub fn perception(&self) -> u8 {
        self.perception
    }

    pub fn fortune(&self) -> u8 {
        self.fortune
    }

    pub fn charm(&self) -> u8 {
        self.charm
    }

    pub fn set_brawn(&mut self, brawn: u8) {
        self.brawn = brawn;
    }

    pub fn set_agility(&mut self, agility: u8) {
        self.agility = agility;
    }

    pub fn set_stamina(&mut self, stamina: u8) {
        self.stamina = stamina;
    }

    pub fn set_perception(&mut self, perception: u8) {
        self.perception = perception;
    }

    pub fn set_fortune(&mut self, fortune: u8) {
        self.fortune = fortune;
    }

    pub fn set_charm(&mut self, charm: u8) {
        self.charm = charm;
    }

    pub fn health(&self) -> u8 {
        self.health
    }

    pub fn set_health(&mut self, health: u8) {
        self.health = health;
    }
}