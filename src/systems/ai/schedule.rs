use specs::{Join, Read, ReadStorage, System, WriteStorage};

use crate::{
    city::city::City,
    components::{
        npc::{NPC, NPCState},
        profession::Profession,
        target::Target,
    },
    game::GameTime,
};

pub struct ScheduleSystem;

impl<'a> System<'a> for ScheduleSystem {
    type SystemData = (
        Read<'a, GameTime>,
        Read<'a, City>,
        ReadStorage<'a, NPC>,
        WriteStorage<'a, Profession>,
        WriteStorage<'a, Target>,
    );

    fn run(&mut self, (time, city, npcs, mut professions, mut targets): Self::SystemData) {
        let hour = time.hour();

        for (npc, profession, target) in (&npcs, &mut professions, &mut targets).join() {
            if npc.has_state(NPCState::Hostile)
                || npc.has_state(NPCState::Searching)
                || npc.has_state(NPCState::Fleeing)
            {
                continue;
            }

            let should_work = Profession::should_be_at_work(hour, &profession.job_type);

            if should_work && !profession.at_work {
                if let Some(bid) = profession.employer_building_id {
                    if let Some(entrance) = city.find_building_entrance(bid) {
                        target.x = entrance.x;
                        target.y = entrance.y;
                    }
                }
                profession.at_work = true;
            } else if !should_work && profession.at_work {
                let pos = city.get_random_target();
                target.x = pos.x;
                target.y = pos.y;
                profession.at_work = false;
            }
        }
    }
}
