use rand::Rng;
use specs::{Join, System, WriteStorage, ReadStorage, Read};

use crate::{components::{position::Position, player::Player}, city::City};

pub struct RandomMove;

impl<'a> System<'a> for RandomMove {
    type SystemData = (Read<'a, City>, WriteStorage<'a, Position>, ReadStorage<'a, Player>);
    fn run(&mut self, data: Self::SystemData) {
        let mut rng = rand::thread_rng();
        let (map, mut pos, player) = data;
        for (p, _) in (&mut pos, !&player).join() {
            let dx = 1 - rng.gen_range(0..3);
            let dy = 1 - rng.gen_range(0..3);
            let x = p.x as i32 + dx;
            let y = p.y as i32 + dy;
            if !map.data[y as usize][x as usize].blocked {
                p.x += dx as f32;
                p.y += dy as f32;
            }
        }
    }
}