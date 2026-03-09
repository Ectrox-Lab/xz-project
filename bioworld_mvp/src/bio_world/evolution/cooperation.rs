pub fn cooperation_success(attackers: usize, synchrony: f64, signal_investment: f64, threshold: usize, x: f64, y: f64) -> bool {
    attackers >= threshold && synchrony > x && signal_investment > y
}
