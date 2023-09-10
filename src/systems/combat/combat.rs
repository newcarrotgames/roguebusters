use specs::{Join, System, Write, WriteStorage, Entities, LazyUpdate, Read, ReadStorage, WorldExt};

use crate::{components::{combatant::Combatant, attributes::Attributes, inventory::{Inventory, EquipLocation}}, game::GameState, util::rng::Dice};

pub struct Combat;

// resolves combat actions between entities (player and npcs alike)
impl<'a> System<'a> for Combat {
    type SystemData = (
        Write<'a, GameState>,
        Entities<'a>,
        WriteStorage<'a, Combatant>,
        WriteStorage<'a, Attributes>,
        ReadStorage<'a, Inventory>,
        // ReadStorage<'a, Position>,
        Read<'a, LazyUpdate>,
    );
    fn run(&mut self, (mut game_state, entities, combatants, attributes, inventories, lazy): Self::SystemData) {
        for (combatant, target_ent) in (&combatants, &entities).join() {
            game_state.push_message(format!(
                "Entity {:?} is attacking entity {:?}",
                combatant.entity, target_ent
            ));

            // roll for accuracy
            let mut dice = Dice::new();

            // get attacker's inventory
            let inv = inventories.get(combatant.entity).unwrap();
            let weapon = inv.equipped_item(EquipLocation::RightHand).unwrap().clone();

            let combatant_attrs = attributes.get(combatant.entity).unwrap();
            // let target_attrs = attributes.get_mut(target_ent).unwrap();
            
            if dice.roll_1d20() > combatant_attrs.agility() {
                // hit
                game_state.push_message(format!("Entity {:?} was hit", target_ent));

                // target_attrs.set_health(target_attrs.health() - dice.from_int(weapon.damage));
                lazy.exec(move |world| {
                    // This will get executed after the system runs, and it allows you to make mutable changes
                    let mut attrs = world.write_storage::<Attributes>();
                    let mut game_state = world.write_resource::<GameState>();
                    if let Some(target_attrs) = attrs.get_mut(target_ent) {
                        target_attrs.set_health(target_attrs.health() - weapon.damage as u8);
                        game_state.push_message(format!("Target health at {:?}", target_attrs.health()));
                    }
                });
            } else {
                // miss
                game_state.push_message(format!("Entity {:?} was missed", target_ent));
            }

            // don't forget to remove component after we're done
            lazy.remove::<Combatant>(target_ent);
        }
    }
}
