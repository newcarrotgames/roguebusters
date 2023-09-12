use specs::{Entities, Join, LazyUpdate, Read, ReadStorage, System, WorldExt, Write, WriteStorage};

use crate::{
    components::{
        attributes::Attributes,
        combatant::Combatant,
        inventory::{EquipLocation, Inventory}, npc::{NPC, NPCState},
    },
    game::GameState,
    util::rng::Dice,
};

pub struct Combat;

// resolves combat actions between entities (player and npcs alike)
impl<'a> System<'a> for Combat {
    type SystemData = (
        Write<'a, GameState>,
        Entities<'a>,
        WriteStorage<'a, Combatant>,
        WriteStorage<'a, Attributes>,
        WriteStorage<'a, NPC>,
        ReadStorage<'a, Inventory>,
        // ReadStorage<'a, Position>,
        Read<'a, LazyUpdate>,
    );
    fn run(
        &mut self,
        (mut game_state, entities, combatants, attributes, mut npcs, inventories, lazy): Self::SystemData,
    ) {
        let mut combatants_to_remove = Vec::new();
        for (combatant, target_ent, npc) in (&combatants, &entities, (&mut npcs).maybe()).join() {
            combatants_to_remove.push(target_ent);
            game_state.push_message(format!(
                "Entity {:?} is attacking entity {:?}",
                combatant.entity, target_ent
            ));

            // set npc state to hostile
            if npc.is_some() {
                let mut npc = npc.unwrap();
                if !npc.has_state(NPCState::Hostile) {
                    npc.add_state(NPCState::Hostile);
                }
            }

            let mut dice = Dice::new();

            // todo: just include inventory in the join
            // get attacker's inventory
            let inv = match inventories.get(combatant.entity) {
                Some(inv) => inv,
                None => {
                    game_state
                        .push_message(format!("Entity {:?} has no inventory", combatant.entity));
                    continue;
                }
            };

            // check if attacker has a weapon equipped
            let weapon = match inv.equipped_item(EquipLocation::RightHand) {
                Some(item) => item.clone(),
                None => {
                    game_state.push_message(format!(
                        "Entity {:?} has no weapon equipped",
                        combatant.entity
                    ));
                    continue;
                }
            };

            // todo: just include attributes in the join
            let combatant_attrs = attributes.get(combatant.entity).unwrap();
            // let target_attrs = attributes.get_mut(target_ent).unwrap();

            if dice.roll_1d20() < combatant_attrs.agility() {
                // hit
                game_state.push_message(format!("Entity {:?} was hit", target_ent));

                // target_attrs.set_health(target_attrs.health() - dice.from_int(weapon.damage));
                lazy.exec(move |world| {
                    let mut attrs = world.write_storage::<Attributes>();
                    let mut game_state = world.write_resource::<GameState>();
                    if let Some(target_attrs) = attrs.get_mut(target_ent) {
                        if target_attrs.health() - weapon.damage < 1 {
                            game_state.push_message(format!("Entity {:?} was killed", target_ent));
                            world
                                .entities()
                                .delete(target_ent)
                                .expect("could not remove entity");
                        } else {
                            target_attrs.set_health(target_attrs.health() - weapon.damage);
                            game_state.push_message(format!(
                                "Target health at {:?}",
                                target_attrs.health()
                            ));
                        }
                    }
                });
            } else {
                // miss
                game_state.push_message(format!("Entity {:?} was missed", target_ent));
            }
        }

        // remove combatants
        for ent in combatants_to_remove {
            lazy.remove::<Combatant>(ent);
        }
    }
}
