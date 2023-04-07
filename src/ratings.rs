//! Vector of ratings of all players.
use crate::{elo_with_k, IPlayer, Outcome, Player, K};

pub struct Ratings<T: IPlayer> {
	players: Vec<T>,
	k: f64,
}

impl<T: IPlayer> Default for Ratings<T> {
	fn default() -> Self {
		Self {
			players: vec![],
			k: K,
		}
	}
}

impl<T: IPlayer> Ratings<T> {
	pub fn new(players: Vec<T>) -> Self { Self::new_with_k(players, K) }

	pub fn new_with_k(players: Vec<T>, k: f64) -> Self { Self { players, k } }

	pub fn as_slice(&self) -> &[T] { &self.players }

	pub fn get(&self, idx: usize) -> Option<&T> { self.players.get(idx) }

	pub unsafe fn get_unchecked(&self, idx: usize) -> &T { &self.players[idx] }

	pub fn r#match(
		&mut self,
		a: usize,
		b: usize,
		result: Outcome,
	) -> Result<(), ()> {
		if a == b
			|| self.players.get(a).is_none()
			|| self.players.get(a).is_none()
		{
			Err(())
		} else {
			unsafe { self.match_unchecked(a, b, result) };
			Ok(())
		}
	}

	pub unsafe fn match_unchecked(
		&mut self,
		a: usize,
		b: usize,
		result: Outcome,
	) {
		let (a, b) = (
			self.players.get_unchecked_mut(a) as *mut T,
			self.players.get_unchecked_mut(b) as *mut T,
		);
		let (ar, br) = ((*a).rating(), (*b).rating());
		let (ar_new, br_new) = elo_with_k(ar, br, result, self.k);
		(*a).update(ar_new);
		(*b).update(br_new);
	}

	pub unsafe fn match_with_ref(
		&mut self,
		a: &mut T,
		b: &mut T,
		result: Outcome,
	) {
		let (ar, br) = (a.rating(), b.rating());
		let (ar_new, br_new) = elo_with_k(ar, br, result, self.k);
		a.update(ar_new);
		b.update(br_new);
	}
}

impl<T> Ratings<T>
where T: IPlayer + Clone
{
	pub fn fill(starting_rate: T, length: usize) -> Self {
		Self::fill_with_k(starting_rate, length, K)
	}

	pub fn fill_with_k(starting_rate: T, length: usize, k: f64) -> Self {
		Self::new_with_k(vec![starting_rate; length], k)
	}
}

impl From<Ratings<Player>> for Vec<Player> {
	fn from(value: Ratings<Player>) -> Self { value.players }
}

impl From<Vec<f64>> for Ratings<Player> {
	fn from(value: Vec<f64>) -> Self {
		Self::new(
			value
				.into_iter()
				.map(Player::new)
				.collect(),
		)
	}
}
