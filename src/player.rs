use crate::{elo_a, Outcome, K};

pub trait IPlayer {
	fn rating(&self) -> f64;

	fn update(&mut self, rating: f64);
}

#[derive(Clone, Copy)]
pub struct Player(f64);

impl Player {
	pub fn new(rating: f64) -> Self { Self(rating) }

	pub fn update_rating(&mut self, opponent_rating: f64, outcome: Outcome) {
		self.update_rating_with_k(opponent_rating, outcome, K)
	}

	pub fn update_rating_with_k(
		&mut self,
		opponent_rating: f64,
		outcome: Outcome,
		k: f64,
	) {
		self.0 = elo_a(self.0, opponent_rating, outcome, k)
	}
}

impl IPlayer for Player {
	fn rating(&self) -> f64 { self.0 }

	fn update(&mut self, rating: f64) { self.0 = rating; }
}

impl From<f64> for Player {
	fn from(value: f64) -> Self { Self::new(value) }
}

impl From<i32> for Player {
	fn from(value: i32) -> Self { Self::new(value as f64) }
}
