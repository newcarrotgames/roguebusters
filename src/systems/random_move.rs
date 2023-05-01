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


// pub struct Renderable;

// impl<'a> System<'a> for Draw {
//     type SystemData = WriteStorage<'a, Object>;
//     fn run(&mut self, mut obj: Self::SystemData) {
//         for obj in obj.join() {
//             con.set_default_foreground(self.color);
//             let cx = self.x - view_offset[0];
//             let cy = self.y - view_offset[1];
//             // check if offscreen
//             if cx < 0 || cy < 0  || cx > SCREEN_WIDTH || cy > SCREEN_HEIGHT  {
//                 return;
//             }
//             con.put_char(cx, cy, self.char, BackgroundFlag::None);
//         }
//     }
// }