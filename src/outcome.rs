//! Result of each game.
use std::ops::{Mul, Not, Sub};

/// Represents the outcome of a game.
///
/// ## Note
///
/// In most cases, the result is interpreted as the 'left hand side'
/// player getting the outcome. See the example below:
///
/// ```
/// # use elo_rs::*;
/// let mut ratings = Ratings::from(vec![10., 20.]);
///
/// ratings.r#match(0, 1, Win) // player 0 won, player 1 lost the game.
/// ratings.r#match(0, 1, Draw) // both players drew the game.
/// ratings.r#match(0, 1, Loss) // player 0 lost, player 1 won the game.
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
	/// Represents that the player won the game.
	///
	/// Has a value representation of `1`.
	Win,
	/// Represents that the game ended in a draw.
	///
	/// Has a value representation of `0.5`.
	Draw,
	/// Represents that the player lost the game.
	///
	/// Has a value representation of `0`.
	Loss,
}

pub use Outcome::*;

impl From<Outcome> for f64 {
	#[inline]
	fn from(value: Outcome) -> Self {
		match value {
			Win => 1.,
			Draw => 0.5,
			Loss => 0.,
		}
	}
}

impl Sub<f64> for Outcome {
	type Output = f64;

	#[inline]
	fn sub(self, rhs: f64) -> Self::Output { f64::from(self) - rhs }
}

impl Mul<f64> for Outcome {
	type Output = f64;

	#[inline]
	fn mul(self, rhs: f64) -> Self::Output { f64::from(self) * rhs }
}

impl Not for Outcome {
	type Output = Self;

	/// inverse the Outcome.
	///
	/// Example:
	///
	/// ```
	/// # use elo_rs::*;
	/// let res = Loss;
	/// assert_eq!(!res, Win);
	///
	/// assert_eq!(!Win, Loss);
	/// assert_eq!(!Draw, Draw);
	/// assert_eq!(!!Win, Win);
	/// ```
	#[inline]
	fn not(self) -> Self::Output {
		// 1. - f64::from(self)
		match self {
			Win => Loss,
			Draw => Draw,
			Loss => Win,
		}
	}
}
