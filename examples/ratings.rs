use elo_rs::{IPlayer, Loss, Player, Ratings};

fn main() {
	let mut rating: Ratings<Player> = vec![123., 456.].into();

	rating.r#match(0, 1, Loss).unwrap();

	let players: Vec<_> = rating.into();

	assert_eq!(players[0].rating(), 118.89736249706061);
	assert_eq!(players[1].rating(), 460.1026375029394);
}
