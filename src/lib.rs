fn delta(s: f64, e: f64, k: f64) -> f64 { k * (s - e) }

fn ex(ra: f64, rb: f64) -> (f64, f64) {
	let ea = 1. / (1. + 10f64.powf((rb - ra) / 400.));

	(ea, 1. - ea)
}

pub fn elo(ra: f64, rb: f64, sa: f64, sb: f64, k: f64) -> (f64, f64) {
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

	pub fn update_rating(&mut self, rb: f64, sa: f64, k: f64) {
		self.0 = elo(self.0, rb, sa, 1.0 - sa, k).0
	}
}
