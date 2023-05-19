use specs::{Join, Read, ReadStorage, System, WriteStorage};

use crate::{
    components::{position::Position, target::Target}, city::city::City,
};

pub struct SimplePath;

impl<'a> System<'a> for SimplePath {
    type SystemData = (
        Read<'a, City>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Target>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (map, mut pos, target) = data;
        for (p, t) in (&mut pos, &target).join() {
            // distance to target
            let dx = t.x - p.x;
            let dy = t.y - p.y;

            // move 1 cell towards target
            let mut vx = 0.0;
            let mut vy = 0.0;

            if dx > 0.0 {
                vx = 1.0;
            } else if dx < 0.0 {
                vx = -1.0;
            }
            if dy > 0.0 {
                vy = 1.0;
            } else if dy < 0.0 {
                vy = -1.0;
            }

            // get new position
            let x = p.x + vx;
            let y = p.y + vy;

            // check map for empty space (try to walk around objects)
            if !map.data[y as usize][x as usize].blocked {
                p.x = x;
                p.y = y;
            } else if !map.data[p.y as usize][x as usize].blocked {
                p.x = x;
            } else if !map.data[y as usize][p.x as usize].blocked {
                p.y = y;
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
