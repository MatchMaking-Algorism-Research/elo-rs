mod outcome;

pub use outcome::*;

fn delta(s: Outcome, e: f64, k: f64) -> f64 { k * (s - e) }

fn ex(ra: f64, rb: f64) -> (f64, f64) {
	let ea = 1. / (1. + 10f64.powf((rb - ra) / 400.));

	(ea, 1. - ea)
}

pub fn elo(ra: f64, rb: f64, sa: Outcome, sb: Outcome, k: f64) -> (f64, f64) {
	let (ea, eb) = ex(ra, rb);

	(
		ra + delta(sa, ea, k),
		rb + delta(sb, eb, k),
	)
}

pub struct Player(f64);

impl Player {
	pub fn new(ra: f64) -> Self { Self(ra) }

	pub fn rating(&self) -> f64 { self.0 }

	pub fn update_rating(&mut self, rb: f64, sa: Outcome, k: f64) {
		self.0 = elo(self.0, rb, sa, !sa, k).0
	}
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
			(*a).update_rating(br, result, self.k);
			(*b).update_rating(ar, !result, self.k);
		}
		Ok(())
	}
}
