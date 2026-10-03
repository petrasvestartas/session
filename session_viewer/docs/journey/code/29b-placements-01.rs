pub fn valid(matrix: &[f64]) -> bool {
    matrix.len() == 16 && matrix.iter().all(|value| value.is_finite())
        && [matrix[3], matrix[7], matrix[11], matrix[15]] == [0.0, 0.0, 0.0, 1.0]
}
