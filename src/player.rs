use crate::{elo_a, Outcome, K};

/// Single scala value that represents rating of a player
pub trait IPlayer {
	/// Returns the current rating
	fn rating(&self) -> f64;

	/// Updates rating with new value
	fn update(&mut self, rating: f64);
}

#[derive(Clone, Copy)]
pub struct Player(f64);

impl Player {
	/// Constructs a new player with given start rating.
	///
	/// ## Examples:
	///
	/// ```
	/// let player_a = Player::new(123.); 
	/// ```
	pub fn new(rating: f64) -> Self { Self(rating) }

	/// Updates rating according to match outcome.
	///
	/// This method assumes K as 32 by default.
	/// If you want to use a different K value, try
	/// [`update_rating_with_k`](Self::update_rating_with_k).
	///
	/// ## Example:
	///
	/// ```
	/// # use elo_rs::*;
	/// let mut a = Player::new(100.);
	/// a.update_rating(120., Win);
	/// assert_eq!(a.rating().floor(), 116.);
	/// ```
	pub fn update_rating(&mut self, opponent_rating: f64, outcome: Outcome) {
		self.update_rating_with_k(opponent_rating, outcome, K)
	}

	/// Updates rating with given K value.
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
