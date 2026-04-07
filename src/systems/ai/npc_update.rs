use specs::{Entities, Join, Read, ReadStorage, System, WriteStorage};

use crate::{
    city::city::City,
    components::{
        attributes::Attributes,
        combatant::Combatant,
        npc::{NPC, NPCState},
        npc_memory::NPCMemory,
        player::Player,
        position::Position,
        target::Target,
    },
};

pub struct NPCUpdate;

fn has_line_of_sight(city: &City, x0: i32, y0: i32, x1: i32, y1: i32) -> bool {
    let mut x = x0;
    let mut y = y0;
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        if x == x1 && y == y1 {
            return true;
        }
        let uy = y as usize;
        let ux = x as usize;
        if uy >= city.data.len() || ux >= city.data[uy].len() {
            return false;
        }
        if (x != x0 || y != y0) && city.data[uy][ux].block_sight {
            return false;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            if x == x1 {
                return true;
            }
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            if y == y1 {
                return true;
            }
            err += dx;
            y += sy;
        }
    }
}

impl<'a> System<'a> for NPCUpdate {
    type SystemData = (
        Entities<'a>,
        Read<'a, City>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Attributes>,
        WriteStorage<'a, NPC>,
        WriteStorage<'a, Target>,
        WriteStorage<'a, NPCMemory>,
        WriteStorage<'a, Combatant>,
    );

    fn run(
        &mut self,
        (entities, city, players, positions, attrs, mut npcs, mut targets, mut memories, mut combatants): Self::SystemData,
    ) {
        let player_data = (&entities, &players, &positions)
            .join()
            .map(|(e, _, p)| (e, p.x, p.y))
            .next();

        let (player_ent, px, py) = match player_data {
            Some(d) => d,
            None => return,
        };

        for (npc_ent, npc, pos, attr) in (&entities, &mut npcs, &positions, &attrs).join() {
            let perception = attr.perception();
            let detection_range = (perception / 2 + 5) as f32;

            let dist = ((pos.x - px).powi(2) + (pos.y - py).powi(2)).sqrt();

            let can_see_player = dist <= detection_range
                && has_line_of_sight(&city, pos.x as i32, pos.y as i32, px as i32, py as i32);

            if can_see_player {
                let _ = memories.insert(
                    npc_ent,
                    NPCMemory {
                        last_seen_x: px,
                        last_seen_y: py,
                        search_ticks_remaining: perception * 2,
                    },
                );
            }

            let current_state = if npc.has_state(NPCState::Fleeing) {
                Some(NPCState::Fleeing)
            } else if npc.has_state(NPCState::Hostile) {
                Some(NPCState::Hostile)
            } else if npc.has_state(NPCState::Searching) {
                Some(NPCState::Searching)
            } else {
                None
            };

            // Idle NPCs that aren't aggressive don't enter combat on their own
            if current_state.is_none() && !npc.hostile_on_sight {
                continue;
            }

            let flee_threshold = (attr.brawn() + perception) / 2;
            let should_flee = attr.health() < flee_threshold;

            if current_state == Some(NPCState::Searching) {
                if let Some(memory) = memories.get_mut(npc_ent) {
                    memory.search_ticks_remaining -= 1;
                }
            }

            let search_expired = memories
                .get(npc_ent)
                .map(|m| m.search_ticks_remaining <= 0)
                .unwrap_or(true);

            let next_state = if should_flee && current_state.is_some() {
                Some(NPCState::Fleeing)
            } else if current_state == Some(NPCState::Fleeing) {
                if !should_flee || dist > detection_range * 2.0 {
                    None
                } else {
                    Some(NPCState::Fleeing)
                }
            } else if can_see_player {
                Some(NPCState::Hostile)
            } else if current_state == Some(NPCState::Hostile) {
                if memories.get(npc_ent).is_none() {
                    let _ = memories.insert(
                        npc_ent,
                        NPCMemory {
                            last_seen_x: px,
                            last_seen_y: py,
                            search_ticks_remaining: perception * 2,
                        },
                    );
                }
                Some(NPCState::Searching)
            } else if current_state == Some(NPCState::Searching) {
                if search_expired { None } else { Some(NPCState::Searching) }
            } else {
                current_state
            };

            if current_state != next_state {
                if let Some(old) = current_state {
                    npc.remove_state(old);
                }
                if let Some(new) = next_state {
                    npc.add_state(new);
                }
                if next_state.is_none() {
                    memories.remove(npc_ent);
                }
            }

            if let Some(target) = targets.get_mut(npc_ent) {
                match next_state {
                    Some(NPCState::Hostile) => {
                        target.x = px;
                        target.y = py;
                    }
                    Some(NPCState::Searching) => {
                        if let Some(memory) = memories.get(npc_ent) {
                            target.x = memory.last_seen_x;
                            target.y = memory.last_seen_y;
                        }
                    }
                    Some(NPCState::Fleeing) => {
                        target.x = (pos.x + (pos.x - px)).clamp(1.0, (city.width - 2) as f32);
                        target.y = (pos.y + (pos.y - py)).clamp(1.0, (city.height - 2) as f32);
                    }
                    _ => {}
                }
            }

            if next_state == Some(NPCState::Hostile) {
                let dx = (pos.x - px).abs();
                let dy = (pos.y - py).abs();
                if dx <= 1.0 && dy <= 1.0 {
                    let _ = combatants.insert(npc_ent, Combatant { target: player_ent });
                }
            }
        }
    }
}
