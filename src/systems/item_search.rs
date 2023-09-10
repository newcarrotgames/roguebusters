use specs::{Join, ReadStorage, System, Write, WriteStorage};

use crate::{
    components::{item::Item, player::Player, position::Position},
    game::GameState,
};

pub struct ItemSearch;

impl<'a> System<'a> for ItemSearch {
    type SystemData = (
        Write<'a, GameState>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Item>,
        WriteStorage<'a, Position>,
    );
    fn run(
        &mut self,
        (mut game_state, player, items, mut positions): Self::SystemData,
    ) {
        let mut player_position: Position = Position::zero();
        for (_, pos) in (&player, &positions).join() {
            player_position = pos.clone();
        }
        for (item, pos) in (&items, &mut positions).join() {
            if player_position == *pos {
                game_state.push_message(format!("You see a {}", item.name));
            }
        }
    }
}
