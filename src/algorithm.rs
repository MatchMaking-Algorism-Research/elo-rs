use crate::Outcome;

pub(crate) const K: f64 = 32.;

#[inline]
fn delta(o: Outcome, e: f64, k: f64) -> f64 { k * (o - e) }

#[inline]
fn ex(ra: f64, rb: f64) -> f64 { 1. / (1. + 10f64.powf((rb - ra) / 400.)) }

#[inline]
fn expect(ra: f64, rb: f64) -> (f64, f64) {
	let ea = ex(ra, rb);

	(ea, 1. - ea)
}

#[inline]
pub(crate) fn elo_a(ra: f64, rb: f64, o: Outcome, k: f64) -> f64 {
	ra + delta(o, ex(ra, rb), k)
}

/// Calculates Elo rating pairs according to given match outcome.
///
/// ## Examples
///
/// ```
/// # use elo_rs::*;
/// let a = 100.;
/// let b = 150.;
/// let (new_a, new_b) = elo(a, b, Loss);
/// assert_eq!(new_a, 86.2868197570682);
/// assert_eq!(new_b, 163.7131802429318);
/// ```
pub fn elo(rating_a: f64, rating_b: f64, outcome: Outcome) -> (f64, f64) {
	elo_with_k(rating_a, rating_b, outcome, K)
}

pub fn elo_with_k(
	rating_a: f64,
	rating_b: f64,
	outcome: Outcome,
	k: f64,
) -> (f64, f64) {
	let (ea, eb) = expect(rating_a, rating_b);

	(
		rating_a + delta(outcome, ea, k),
		rating_b + delta(!outcome, eb, k),
	)
}
