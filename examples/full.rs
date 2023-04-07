use std::{f64::NAN, iter::repeat_with, path::Path};

use elo_rs::{Draw, IPlayer, Loss, Outcome, Ratings, Win};
use plotters::{
	prelude::{BitMapBackend, ChartBuilder, Circle, IntoDrawingArea},
	style::{full_palette::PURPLE, Color, IntoFont, WHITE},
};
use rand::{seq::index::sample, thread_rng, Rng};

const N: usize = 1000;

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

	for i in 0..=100000 {
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

		if matches!(i, 0 | 2000 | 10000 | 100000) {
			draw(
				ratings.as_slice(),
				&format!("iter = {i}"),
				format!("plots/p-{i}.png"),
			);
		}
	}
}

fn draw<Ap>(v: &[P], title: &str, f: Ap)
where Ap: AsRef<Path> {
	let root = BitMapBackend::new(f.as_ref(), (1024, 1024)).into_drawing_area();

	root.fill(&WHITE).unwrap();

	let it = v.iter().map(|p| p.rating());
	let r_mx = it.clone().fold(NAN, f64::max);
	let r_mi = it.fold(NAN, f64::min);

	let mut chart = ChartBuilder::on(&root)
		.caption(title, ("sans-serif", 50).into_font())
		.margin(15)
		.x_label_area_size(45)
		.y_label_area_size(60)
		.build_cartesian_2d(0f64..1f64, r_mi..r_mx)
		.unwrap();
	chart
		.configure_mesh()
		.x_desc("Winning Probability")
		.y_desc("Elo rating")
		.axis_desc_style(("sans-serif", 25).into_font())
		.max_light_lines(4)
		.draw()
		.unwrap();
	chart
		.draw_series(
			v.into_iter()
				.map(|p| (p.winning_percentage, p.rating()))
				.map(|(x, y)| Circle::new((x, y), 5, PURPLE.filled())),
		)
		.unwrap();

	root.present().unwrap();
}
