use elo_rs::elo;

fn main() {
	let a = 123.;
	let b = 456.;

	let (a, b) = elo(a, b, 0., 1., 32.);

	assert_eq!(a, 118.89736249706061);
	assert_eq!(b, 460.1026375029394);
}
