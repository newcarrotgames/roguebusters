use rand::Rng;
use specs::{Join, System, WriteStorage, ReadStorage, Read};

use crate::{components::{position::Position, player::Player}, map::Map};

pub struct NPCUpdate {
	
}