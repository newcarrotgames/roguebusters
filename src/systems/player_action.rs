use bracket_lib::prelude::RGB;
use specs::{Builder, Entities, Join, LazyUpdate, Read, ReadStorage, System, Write, WorldExt, WriteStorage};

use crate::{
    city::city::City,
    components::{
        combatant::Combatant,
        inventory::{EquipLocation, Inventory},
        item::Item,
        player::Player,
        position::Position,
        renderable::Renderable,
    },
    game::{GameState, PlayerRequest},
};

pub struct PlayerAction;

impl<'a> System<'a> for PlayerAction {
    type SystemData = (
        Write<'a, GameState>,
        Entities<'a>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Item>,
        WriteStorage<'a, Inventory>,
        WriteStorage<'a, Combatant>,
        Read<'a, City>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut game_state,
            entities,
            mut positions,
            players,
            items,
            mut inventories,
            mut combatants,
            city,
            lazy,
        ) = data;

        match game_state.peek_player_request() {
            PlayerRequest::Move(dx, dy) => {
                game_state.pop_player_request();
                for (_, pos) in (&players, &mut positions).join() {
                    let nx = pos.x + dx as f32;
                    let ny = pos.y + dy as f32;
                    if !city.data[ny as usize][nx as usize].blocked {
                        pos.x = nx;
                        pos.y = ny;
                    }
                }
            }

            PlayerRequest::PickupItem => {
                game_state.pop_player_request();
                let mut player_pos = Position::zero();
                let mut player_ent = None;
                for (ent, _, pos) in (&entities, &players, &positions).join() {
                    player_pos = pos.clone();
                    player_ent = Some(ent);
                }

                let mut ents_to_remove: Vec<specs::Entity> = Vec::new();
                let mut item_found = false;

                if let Some(p_ent) = player_ent {
                    for (ent, item, pos) in (&entities, &items, &positions).join() {
                        if *pos == player_pos {
                            item_found = true;
                            if let Some(inventory) = inventories.get_mut(p_ent) {
                                if inventory.push_item(item.clone()) {
                                    ents_to_remove.push(ent);
                                    game_state
                                        .push_message(format!("You pick up a {}", item.name));
                                } else {
                                    game_state.push_message(format!(
                                        "You cannot pick up the {}",
                                        item.name
                                    ));
                                }
                            }
                            break;
                        }
                    }
                }

                if !item_found {
                    game_state.push_message("There is nothing to pick up.".to_string());
                }

                for e in ents_to_remove {
                    positions.remove(e);
                }
            }

            PlayerRequest::WieldItem => {
                game_state.pop_player_request();
                let mut player_pos = Position::zero();
                let mut player_ent = None;
                for (ent, _, pos) in (&entities, &players, &positions).join() {
                    player_pos = pos.clone();
                    player_ent = Some(ent);
                }

                let mut ents_to_remove: Vec<specs::Entity> = Vec::new();
                let mut item_found = false;

                if let Some(p_ent) = player_ent {
                    for (ent, item, pos) in (&entities, &items, &positions).join() {
                        if *pos == player_pos {
                            item_found = true;
                            if let Some(inventory) = inventories.get_mut(p_ent) {
                                let item_clone = item.clone();
                                inventory.push_item(item_clone.clone());
                                inventory.equip(item_clone, EquipLocation::RightHand);
                                ents_to_remove.push(ent);
                                game_state.push_message(format!("You wield a {}", item.name));
                            }
                            break;
                        }
                    }
                }

                if !item_found {
                    game_state.push_message("There is nothing to wield here.".to_string());
                }

                for e in ents_to_remove {
                    positions.remove(e);
                }
            }

            PlayerRequest::DropItem => {
                game_state.pop_player_request();
                let mut player_pos = Position::zero();
                let mut player_ent = None;
                for (ent, _, pos) in (&entities, &players, &positions).join() {
                    player_pos = pos.clone();
                    player_ent = Some(ent);
                }

                if let Some(p_ent) = player_ent {
                    if let Some(inventory) = inventories.get_mut(p_ent) {
                        if let Some(equipped) =
                            inventory.equipped_item(EquipLocation::RightHand).cloned()
                        {
                            let item_name = equipped.name.clone();
                            let item_char = equipped.char;
                            inventory.unequip(equipped.clone(), EquipLocation::RightHand);
                            inventory.remove_item(&equipped);
                            let drop_pos = player_pos.clone();
                            lazy.exec(move |world| {
                                world
                                    .create_entity()
                                    .with(equipped)
                                    .with(drop_pos)
                                    .with(Renderable {
                                        char: item_char,
                                        color: RGB::from_u8(255, 255, 255),
                                    })
                                    .build();
                            });
                            game_state.push_message(format!("You drop the {}", item_name));
                        } else {
                            game_state
                                .push_message("You have nothing equipped to drop.".to_string());
                        }
                    }
                }
            }

            PlayerRequest::Selected(x, y) => {
                game_state.pop_player_request();
                let view_offset = game_state.get_view_offset();
                let tx = (x + view_offset[0]) as f32;
                let ty = (y + view_offset[1]) as f32;

                let player_ent = (&entities, &players).join().map(|(e, _)| e).last();
                let target_ent = (&entities, &positions)
                    .join()
                    .filter(|(_, pos)| pos.x == tx && pos.y == ty)
                    .map(|(e, _)| e)
                    .last();

                match (player_ent, target_ent) {
                    (Some(p_ent), Some(t_ent)) => {
                        let has_weapon = inventories
                            .get(p_ent)
                            .and_then(|inv| inv.equipped_item(EquipLocation::RightHand))
                            .is_some();
                        if has_weapon {
                            // Combatant goes on the ATTACKER; target stored inside
                            let _ = combatants.insert(p_ent, Combatant { target: t_ent });
                            game_state.push_player_request(PlayerRequest::CloseCurrentView);
                        } else {
                            game_state.push_message(
                                "You do not have an equipped weapon.".to_string(),
                            );
                        }
                    }
                    _ => {
                        game_state.push_message("No target found!".to_string());
                    }
                }
            }

            PlayerRequest::Wait => {
                game_state.pop_player_request();
            }

            _ => {}
        }
    }
}
