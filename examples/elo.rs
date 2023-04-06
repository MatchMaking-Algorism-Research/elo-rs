use elo_rs::elo;

fn main() {
	let a = 123.;
	let b = 456.;

	let (a, b) = elo(a, b, 0., 1., 32.);

	assert_eq!(a, -217.59553440106316);
	assert_eq!(b, 217.59553440106316);
}
