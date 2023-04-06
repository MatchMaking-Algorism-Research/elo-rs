use std::ops::{Mul, Not, Sub};

#[derive(Clone, Copy)]
pub enum Outcome {
	Win,
	Draw,
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
