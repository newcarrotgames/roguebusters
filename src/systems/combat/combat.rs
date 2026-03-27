use specs::{Entities, Join, ReadStorage, System, Write, WriteStorage};

use crate::{
    components::{
        attributes::Attributes,
        combatant::Combatant,
        inventory::{EquipLocation, Inventory},
        name::Name,
        npc::{NPC, NPCState},
        player::Player,
    },
    game::GameState,
    util::rng::Dice,
};

pub struct Combat;

/// Resolves combat actions between entities (player and NPCs alike).
///
/// `Combatant` is placed on the ATTACKER and stores the target entity.
/// Pass 1 collects attack results.
/// Pass 2 applies damage and removes dead entities.
impl<'a> System<'a> for Combat {
    type SystemData = (
        Write<'a, GameState>,
        Entities<'a>,
        WriteStorage<'a, Combatant>,
        WriteStorage<'a, Attributes>,
        WriteStorage<'a, NPC>,
        ReadStorage<'a, Inventory>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, Player>,
    );

    fn run(
        &mut self,
        (mut game_state, entities, mut combatants, mut attributes, mut npcs, inventories, names, players): Self::SystemData,
    ) {
        struct AttackResult {
            target:  specs::Entity,
            damage:  i32,
        }

        let mut results:       Vec<AttackResult>    = Vec::new();
        let mut attackers_done: Vec<specs::Entity>  = Vec::new();

        // Pass 1: evaluate each pending attack.
        for (attacker_ent, combatant, inv, attrs) in
            (&entities, &combatants, &inventories, &attributes).join()
        {
            attackers_done.push(attacker_ent);

            let damage = match inv.equipped_item(EquipLocation::RightHand) {
                Some(item) => item.damage,
                // Unarmed: brawn / 4 (minimum 1)
                None       => std::cmp::max(1, attrs.brawn() / 4),
            };

            let attacker_name = names.get(attacker_ent).map(|n| n.name.as_str()).unwrap_or("Someone");
            let target_name   = names.get(combatant.target).map(|n| n.name.as_str()).unwrap_or("Someone");

            let attacker_is_player = players.get(attacker_ent).is_some();
            let target_is_player   = players.get(combatant.target).is_some();

            let attacker_label = if attacker_is_player { "You".to_string() } else { attacker_name.to_string() };
            let target_label   = if target_is_player   { "you".to_string() } else { target_name.to_string()   };

            let mut dice = Dice::new();
            if dice.roll_1d20() < attrs.agility() {
                game_state.push_message(format!(
                    "{} hit{} {} for {} damage.",
                    attacker_label,
                    if attacker_is_player { "" } else { "s" },
                    target_label,
                    damage
                ));
                results.push(AttackResult { target: combatant.target, damage });
            } else {
                game_state.push_message(format!(
                    "{} missed {}.",
                    attacker_label, target_label
                ));
            }
        }

        // Pass 2: apply damage, mark hostility, handle death.
        for result in results {
            if let Some(npc) = npcs.get_mut(result.target) {
                if !npc.has_state(NPCState::Hostile) {
                    npc.add_state(NPCState::Hostile);
                }
            }

            if let Some(target_attrs) = attributes.get_mut(result.target) {
                let new_hp = target_attrs.health() - result.damage;
                if new_hp < 1 {
                    let target_name = names.get(result.target).map(|n| n.name.as_str()).unwrap_or("Someone");
                    let is_player   = players.get(result.target).is_some();

                    if is_player {
                        game_state.push_message("You are dead!".to_string());
                        game_state.game_over = true;
                    } else {
                        game_state.push_message(format!("{} is killed!", target_name));
                        entities.delete(result.target).expect("could not remove entity");
                    }
                } else {
                    target_attrs.set_health(new_hp);

                    let is_player = players.get(result.target).is_some();
                    if is_player {
                        let max_hp = target_attrs.stamina();
                        game_state.push_message(format!(
                            "HP: {}/{}",
                            new_hp, max_hp
                        ));
                    }
                }
            }
        }

        for ent in attackers_done {
            combatants.remove(ent);
        }
    }
}
