fn delta(s: f64, e: f64, k: f64) -> f64 { k * (s - e) }

fn ex(ra: f64, rb: f64) -> (f64, f64) {
	let ea = 10f64.powf((rb - ra) / 400.);

	(ea, 1. - ea)
}

pub fn elo(ra: f64, rb: f64, sa: f64, sb: f64, k: f64) -> (f64, f64) {
	let (ea, eb) = ex(ra, rb);

	(delta(sa, ea, k), delta(sb, eb, k))
}
