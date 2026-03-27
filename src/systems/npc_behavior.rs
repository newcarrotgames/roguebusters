use specs::{Entities, Join, ReadStorage, System, WriteStorage};

use crate::components::{
    combatant::Combatant,
    npc::{NPC, NPCState},
    player::Player,
    position::Position,
    target::Target,
};

/// Drives hostile NPC behaviour each tick:
///   1. Retargets them toward the player so SimplePath moves them correctly.
///   2. Inserts a `Combatant` component when adjacent so Combat resolves the attack.
pub struct NPCBehavior;

impl<'a> System<'a> for NPCBehavior {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, NPC>,
        WriteStorage<'a, Target>,
        WriteStorage<'a, Combatant>,
    );

    fn run(&mut self, (entities, players, positions, npcs, mut targets, mut combatants): Self::SystemData) {
        // Locate the player entity and its current world position.
        let player_data = (&entities, &players, &positions)
            .join()
            .map(|(e, _, p)| (e, p.x, p.y))
            .next();

        let (player_ent, px, py) = match player_data {
            Some(d) => d,
            None    => return,
        };

        for (npc_ent, npc, pos) in (&entities, &npcs, &positions).join() {
            if !npc.has_state(NPCState::Hostile) {
                continue;
            }

            // Redirect this NPC's pathfinding target toward the player.
            if let Some(target) = targets.get_mut(npc_ent) {
                target.x = px;
                target.y = py;
            }

            // Attack if standing in an adjacent or diagonal cell.
            let dx = (pos.x - px).abs();
            let dy = (pos.y - py).abs();
            if dx <= 1.0 && dy <= 1.0 {
                let _ = combatants.insert(npc_ent, Combatant { target: player_ent });
            }
        }
    }
}
