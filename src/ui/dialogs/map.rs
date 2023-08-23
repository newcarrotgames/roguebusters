use crate::{
    components::{player::Player, position::Position},
    MAP_HEIGHT, MAP_WIDTH, city::city::City, ui::ui::{UIModal, LINES_DOUBLE_SINGLE, UI},
};
use specs::{Join, World, WorldExt};
use tcod::{console::Offscreen, colors::{WHITE, BLUE, BLACK}, Console, BackgroundFlag};

const MAP_POSITION: [i32; 4] = [5, 5, 113, 61];

pub struct MapUIModal {}

impl MapUIModal {
    pub fn new() -> Self {
        MapUIModal {}
    }
}

impl UIModal for MapUIModal {
    fn render(&mut self, con: &mut Offscreen) {
        UI::render_dialog(con, MAP_POSITION, WHITE, LINES_DOUBLE_SINGLE, "Map");
    }

    fn update(&mut self, con: &mut Offscreen, world: &World) {
        // draw map view
		let map = world.read_resource::<City>();
		let map_x_scale = MAP_WIDTH / (MAP_POSITION[2] - MAP_POSITION[0] - 2);
		let map_y_scale = MAP_HEIGHT / (MAP_POSITION[3] - MAP_POSITION[1] - 2);
		for my in MAP_POSITION[1] + 1..MAP_POSITION[3] {
			for mx in MAP_POSITION[0] + 1..MAP_POSITION[2]  {
				let x = mx - MAP_POSITION[0];
				let y = my - MAP_POSITION[1];
				let wall = map.data[(y * map_y_scale) as usize][(x * map_x_scale) as usize];
				con.set_default_foreground(WHITE);
				let mut c = 176 as char;
				if wall.bg_color == BLUE {
					con.set_default_background(BLUE);
					c = ' ' as char;
				} else {
					con.set_default_background(BLACK);
				}
				
				if wall.building_id == 0 && (wall.char == 32 as char || wall.char == 0 as char) && wall.bg_color == BLACK {
					c = ' ';
				}
				con.put_char(mx, my, c, BackgroundFlag::Set);
			}
		}
		con.set_default_background(BLACK);

		// draw player
		let player_storage = world.read_storage::<Player>();
		let position_storage = world.read_storage::<Position>();
		for (_, player_position) in (&player_storage, &position_storage).join() {
			con.put_char(
				MAP_POSITION[0] + (player_position.x as i32 / map_x_scale),
				MAP_POSITION[1] + (player_position.y as i32 / map_y_scale),
				'@',
				BackgroundFlag::None,
			);
		}
    }
}
