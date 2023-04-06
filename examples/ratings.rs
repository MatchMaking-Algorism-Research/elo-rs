use elo_rs::{Outcome, Player, Ratings};

fn main() {
	let mut rating = Ratings::new(
		vec![Player::new(123.), Player::new(456.)],
		32.,
	);

	rating
		.r#match(0, 1, Outcome::Loss)
		.unwrap();

	let players = rating.into_vec();

	assert_eq!(players[0].rating(), 118.89736249706061);
	assert_eq!(players[1].rating(), 460.1026375029394);
}
