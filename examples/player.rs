use elo_rs::{Outcome, Player};

fn main() {
	let mut player = Player::new(123.);

	player.update_rating(456., Outcome::Loss, 32.);

	assert_eq!(player.rating(), 118.89736249706061);
}
