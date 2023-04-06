use elo_rs::{Loss, Player};

fn main() {
	let mut player = Player::new(123.);

	player.update_rating(456., Loss);

	assert_eq!(player.rating(), 118.89736249706061);
}
