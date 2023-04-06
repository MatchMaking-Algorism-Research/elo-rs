pub mod algorithm;
mod outcome;

pub use algorithm::*;
pub use outcome::*;

pub struct Player(f64);

impl Player {
	pub fn new(rating: f64) -> Self { Self(rating) }

	pub fn rating(&self) -> f64 { self.0 }

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

	fn set_rating(&mut self, rating: f64) { self.0 = rating; }
}

pub struct Ratings {
	players: Vec<Player>,
	k: f64,
}

impl Default for Ratings {
	fn default() -> Self {
		Self {
			players: vec![],
			k: K,
		}
	}
}

impl Ratings {
	pub fn new(players: Vec<Player>) -> Self { Self::new_with_k(players, K) }

	pub fn new_with_k(players: Vec<Player>, k: f64) -> Self {
		Self { players, k }
	}

	pub fn into_vec(self) -> Vec<Player> { self.players }

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
			self.players.get_unchecked_mut(a) as *mut Player,
			self.players.get_unchecked_mut(b) as *mut Player,
		);
		let (ar, br) = ((*a).rating(), (*b).rating());
		let (ar_new, br_new) = elo_with_k(ar, br, result, self.k);
		(*a).set_rating(ar_new);
		(*b).set_rating(br_new);
	}
}
