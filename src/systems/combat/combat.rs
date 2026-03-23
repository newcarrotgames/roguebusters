use specs::{Entities, Join, Read, ReadStorage, System, Write, WriteStorage};

use crate::{
    components::{
        attributes::Attributes,
        combatant::Combatant,
        inventory::{EquipLocation, Inventory},
        npc::{NPC, NPCState},
    },
    game::GameState,
    util::rng::Dice,
};

pub struct Combat;

/// Resolves combat actions between entities (player and NPCs alike).
///
/// `Combatant` is placed on the ATTACKER and stores the target entity.
/// Pass 1 iterates attackers — inventory and attributes come straight from
/// the join, eliminating the previous random-access lookups.
/// Pass 2 applies damage to targets after the join borrow is released,
/// removing the need for the previous `lazy.exec` workaround.
impl<'a> System<'a> for Combat {
    type SystemData = (
        Write<'a, GameState>,
        Entities<'a>,
        WriteStorage<'a, Combatant>,
        WriteStorage<'a, Attributes>,
        WriteStorage<'a, NPC>,
        ReadStorage<'a, Inventory>,
    );

    fn run(
        &mut self,
        (mut game_state, entities, mut combatants, mut attributes, mut npcs, inventories): Self::SystemData,
    ) {
        struct AttackResult {
            attacker: specs::Entity,
            target: specs::Entity,
            damage: i32,
        }

        let mut results: Vec<AttackResult> = Vec::new();
        let mut attackers_done: Vec<specs::Entity> = Vec::new();

        // Pass 1: evaluate each pending attack.
        // Attacker's Inventory and Attributes are available directly from the
        // join — no per-entity random-access lookups required.
        for (attacker_ent, combatant, inv, attrs) in
            (&entities, &combatants, &inventories, &attributes).join()
        {
            attackers_done.push(attacker_ent);

            let weapon = match inv.equipped_item(EquipLocation::RightHand) {
                Some(item) => item.clone(),
                None => {
                    game_state.push_message(format!(
                        "Entity {:?} has no weapon equipped",
                        attacker_ent
                    ));
                    continue;
                }
            };

            let mut dice = Dice::new();
            if dice.roll_1d20() < attrs.agility() {
                game_state.push_message(format!("Entity {:?} was hit", combatant.target));
                results.push(AttackResult {
                    attacker: attacker_ent,
                    target: combatant.target,
                    damage: weapon.damage,
                });
            } else {
                game_state.push_message(format!("Entity {:?} was missed", combatant.target));
            }
        }

        // Pass 2: apply damage.
        // The join borrow on `attributes` has been released so `get_mut` is
        // now available. Entity deletion is safe here — specs defers it until
        // `world.maintain()`.
        for result in results {
            // make the target NPC hostile if it was hit
            if let Some(npc) = npcs.get_mut(result.target) {
                if !npc.has_state(NPCState::Hostile) {
                    npc.add_state(NPCState::Hostile);
                }
            }

            if let Some(target_attrs) = attributes.get_mut(result.target) {
                if target_attrs.health() - result.damage < 1 {
                    game_state
                        .push_message(format!("Entity {:?} was killed", result.target));
                    entities
                        .delete(result.target)
                        .expect("could not remove entity");
                } else {
                    target_attrs.set_health(target_attrs.health() - result.damage);
                    game_state.push_message(format!(
                        "Target health at {:?}",
                        target_attrs.health()
                    ));
                }
            }
        }

        // Remove the Combatant component from each attacker now that the
        // attack has been resolved.
        for ent in attackers_done {
            combatants.remove(ent);
        }
    }
}
