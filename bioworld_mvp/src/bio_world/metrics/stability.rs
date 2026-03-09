pub fn hazard_rate(cdi: f64, i_crit: f64) -> f64 {
    if cdi >= i_crit { 0.003 } else { 0.003 + (i_crit - cdi).abs() * 0.02 }
}

pub fn extinction_probability(h: f64, dt: f64) -> f64 {
    1.0 - (-h * dt).exp()
}
