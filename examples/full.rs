use std::iter::repeat_with;

use elo_rs::{Draw, IPlayer, Loss, Outcome, Ratings, Win};
use rand::{seq::index::sample, thread_rng, Rng};

const N: usize = 100;

#[derive(Debug)]
struct P {
	rating: f64,
	winning_percentage: f64,
}

impl P {
	fn new(rating: f64, winning_percentage: f64) -> Self {
		Self {
			rating,
			winning_percentage,
		}
	}

	fn r#match<R: Rng>(rng: &mut R, a: &Self, b: &Self) -> Outcome {
		match (
			rng.gen_bool(a.winning_percentage),
			rng.gen_bool(b.winning_percentage),
		) {
			(true, false) => Win,
			(false, true) => Loss,
			_ => Draw,
		}
	}
}

impl IPlayer for P {
	fn rating(&self) -> f64 { self.rating }

	fn update(&mut self, rating: f64) { self.rating = rating }
}

fn main() {
	let mut rng = thread_rng();

	let mut ratings = Ratings::new(
		repeat_with(|| rng.gen())
			.map(|wp| P::new(100., wp))
			.take(N)
			.collect::<Vec<_>>(),
	);

	for _ in 0..=1000 {
		let idv = sample(&mut rng, N, 2);
		let (ai, bi) = (idv.index(0), idv.index(1));

		unsafe {
			ratings.match_unchecked(
				ai,
				bi,
				P::r#match(
					&mut rng,
					ratings.get_unchecked(ai),
					ratings.get_unchecked(bi),
				),
			);
		}
	}

	println!("{:?}", Vec::from(ratings));
}
