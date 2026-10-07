pub fn unit(vector: [f32; 3]) -> Result<[f32; 3], &'static str> {
    if vector.iter().any(|v| !v.is_finite()) { return Err("Display normals must be finite"); }
    let scale = vector.iter().map(|v| v.abs()).fold(0.0, f32::max);
    if scale == 0.0 { return Ok([0.0; 3]); }
    let vector = vector.map(|v| v / scale); let length = vector.iter().map(|v| v * v).sum::<f32>().sqrt();
    Ok(vector.map(|v| v / length))
}
