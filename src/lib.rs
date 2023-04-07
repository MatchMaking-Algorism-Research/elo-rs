pub mod algorithm;
mod outcome;

pub use algorithm::*;
pub use outcome::*;

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

pub struct Ratings<T: IPlayer> {
	players: Vec<T>,
	k: f64,
}

impl<T: IPlayer> Default for Ratings<T> {
	fn default() -> Self {
		Self {
			players: vec![],
			k: K,
		}
	}
}

impl Ratings<Player> {
	pub fn new(players: Vec<Player>) -> Self { Self::new_with_k(players, K) }

	pub fn new_with_k(players: Vec<Player>, k: f64) -> Self {
		Self { players, k }
	}

	pub fn fill(starting_rate: f64, length: usize) -> Self {
		Self::fill_with_k(starting_rate, length, K)
	}

	pub fn fill_with_k(starting_rate: f64, length: usize, k: f64) -> Self {
		Self::new_with_k(
			vec![Player::new(starting_rate); length],
			k,
		)
	}
}

impl<T: IPlayer> Ratings<T> {
	pub fn r#match(
		&mut self,
		a: usize,
		b: usize,
		result: Outcome,
	) -> Result<(), ()> {
		if a == b
			|| self.players.get(a).is_none()
			|| self.players.get(a).is_none()
		{
			Err(())
		} else {
			unsafe { self.match_unchecked(a, b, result) };
			Ok(())
		}
	}

	pub unsafe fn match_unchecked(
		&mut self,
		a: usize,
		b: usize,
		result: Outcome,
	) {
		let (a, b) = (
			self.players.get_unchecked_mut(a) as *mut T,
			self.players.get_unchecked_mut(b) as *mut T,
		);
		let (ar, br) = ((*a).rating(), (*b).rating());
		let (ar_new, br_new) = elo_with_k(ar, br, result, self.k);
		(*a).update(ar_new);
		(*b).update(br_new);
	}
}

impl From<Ratings<Player>> for Vec<Player> {
	fn from(value: Ratings<Player>) -> Self { value.players }
}

impl From<Vec<f64>> for Ratings<Player> {
	fn from(value: Vec<f64>) -> Self {
		Self::new(
			value
				.into_iter()
				.map(Player::new)
				.collect(),
		)
	}
}
