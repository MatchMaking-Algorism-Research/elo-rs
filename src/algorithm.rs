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
