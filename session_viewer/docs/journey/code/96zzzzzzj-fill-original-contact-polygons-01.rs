pub fn polygon_normal(count: usize, point: impl Fn(usize) -> [f64; 3]) -> Option<[f64; 3]> {
    if count < 3 {