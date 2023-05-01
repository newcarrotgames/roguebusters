/*
WieldItem,
    DropItem,
    PickupItem,
    UseItem,
    Quit,
    None,
 */

 use specs::{Entities, Entity, Join, ReadStorage, System, Write, WriteStorage};

 use crate::{
	 components::{inventory::Inventory, item::Item, player::Player, position::Position},
	 game::{GameState, PlayerRequest},
 };
 
 pub struct ItemSearch;
 
 impl<'a> System<'a> for ItemSearch {
	 type SystemData = (
		 Entities<'a>,
		 Write<'a, GameState>,
		 ReadStorage<'a, Player>,
		 ReadStorage<'a, Item>,
		 WriteStorage<'a, Position>,
		 WriteStorage<'a, Inventory>,
	 );
	 fn run(
		 &mut self,
		 (entities, mut game_state, player, items, mut positions, mut inventories): Self::SystemData,
	 ) {
		 let mut player_position: Position = Position::zero();
		 for (_, pos) in (&player, &positions).join() {
			 player_position = pos.clone();
		 }
		 let mut ents_to_remove: Vec<Entity> = Vec::new();
		 for (entity, item, pos) in (&entities, &items, &mut positions).join() {
			 if player_position == *pos {
				 if game_state.get_player_request() == Some(PlayerRequest::PickupItem) {
					 for (_, inventory) in (&player, &mut inventories).join() {
						 if inventory.push_item(item.clone()) {
							 ents_to_remove.push(entity.clone());
							 game_state.add_message(format!("You pick up a {}", item.name));
						 } else {
							 game_state
								 .add_message(format!("You can not pick up the {}", item.name));
						 }
					 }
				 } else {
					 game_state.add_message(format!("You see a {}", item.name));
				 }
			 }
		 }
 
		 for e in ents_to_remove {
			 positions.remove(e);
		 }
	 }
 }
 