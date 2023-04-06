use crate::Outcome;

pub(crate) fn delta(s: Outcome, e: f64, k: f64) -> f64 { k * (s - e) }

pub(crate) fn ex(ra: f64, rb: f64) -> (f64, f64) {
	let ea = 1. / (1. + 10f64.powf((rb - ra) / 400.));

	(ea, 1. - ea)
}

pub fn elo(ra: f64, rb: f64, sa: Outcome, k: f64) -> (f64, f64) {
	let (ea, eb) = ex(ra, rb);

	(
		ra + delta(sa, ea, k),
		rb + delta(!sa, eb, k),
	)
}
