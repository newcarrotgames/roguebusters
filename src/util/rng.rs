// struct to generate random numbers based on D&D B/X rules
use rand::{rngs::ThreadRng, thread_rng, Rng};

pub struct Dice {
	rng: ThreadRng,
}

impl Dice {
	pub fn new() -> Self {
		Dice {
			rng: thread_rng(),
		}
	}

	fn roll(&mut self, num_dice: u8, num_sides: u8) -> u8 {
		let mut total = 0;
		for _ in 0..num_dice {
			total += self.rng.gen_range(1..num_sides + 1);
		}
		total
	}

	pub fn roll_3d6(&mut self) -> u8 {
		self.roll(3, 6)
	}

	pub fn roll_1d20(&mut self) -> u8 {
		self.roll(1, 20)
	}

	pub fn from_int(&mut self, num: u32) -> u8 {
        return self.roll(1, num as u8);
    }
}