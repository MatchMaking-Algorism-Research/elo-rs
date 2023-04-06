pub mod algorithm;
mod outcome;

pub use algorithm::*;
pub use outcome::*;

pub struct Player(f64);

impl Player {
	pub fn new(rating: f64) -> Self { Self(rating) }

	pub fn rating(&self) -> f64 { self.0 }

	pub fn update_rating(
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

impl Ratings {
	pub fn new(players: Vec<Player>, k: f64) -> Self { Self { players, k } }

	pub fn into_vec(self) -> Vec<Player> { self.players }

	pub fn r#match(
		&mut self,
		a: usize,
		b: usize,
		result: Outcome,
	) -> Result<(), ()> {
		if a == b {
			return Err(());
		}
		let (Some(a), Some(b)) = (
			self.players
				.get_mut(a)
				.map(|t| t as *mut Player),
			self.players
				.get_mut(b)
				.map(|t| t as *mut Player),
		) else {
			return Err(());
		};
		unsafe {
			let (ar, br) = ((*a).rating(), (*b).rating());
			let (ar_new, br_new) = elo_with_k(ar, br, result, self.k);
			(*a).set_rating(ar_new);
			(*b).set_rating(br_new);
		}
		Ok(())
	}
}
