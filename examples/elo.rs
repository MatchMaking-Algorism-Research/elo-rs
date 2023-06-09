use elo_rs::{elo, Loss};

fn main() {
	let a = 123.;
	let b = 456.;

	let (a, b) = elo(a, b, Loss);

	assert_eq!(a, 118.89736249706061);
	assert_eq!(b, 460.1026375029394);
}
