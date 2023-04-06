use crate::Outcome;

#[inline]
pub(crate) fn delta(s: Outcome, e: f64, k: f64) -> f64 { k * (s - e) }

#[inline]
pub(crate) fn exa(ra: f64, rb: f64) -> f64 {
	1. / (1. + 10f64.powf((rb - ra) / 400.))
}

#[inline]
pub(crate) fn ex(ra: f64, rb: f64) -> (f64, f64) {
	let ea = exa(ra, rb);

	(ea, 1. - ea)
}

#[inline]
pub(crate) fn elo_a(ra: f64, rb: f64, sa: Outcome, k: f64) -> f64 {
	ra + delta(sa, exa(ra, rb), k)
}

pub fn elo(ra: f64, rb: f64, sa: Outcome, k: f64) -> (f64, f64) {
	let (ea, eb) = ex(ra, rb);

	(
		ra + delta(sa, ea, k),
		rb + delta(!sa, eb, k),
	)
}
