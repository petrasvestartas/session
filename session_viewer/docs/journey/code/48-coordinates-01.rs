pub fn checked(point: [f64; 3]) -> Result<[f64; 3], &'static str> {
    if point.iter().any(|v| !v.is_finite() || v.abs() > 1e12) { return Err("Coordinates must be finite and within ±1e12"); }
    Ok(point)
}

pub fn parse(text: &str) -> Result<[f64; 3], &'static str> {
    let parts: Vec<_> = text.split(',').map(str::trim).collect();
    if !(2..=3).contains(&parts.len()) { return Err("Use world coordinates x,y,z or x,y"); }
    let mut point = [0.; 3];
    for (value, text) in point.iter_mut().zip(parts) { *value = text.parse().map_err(|_| "Coordinates must be numbers")?; }
    checked(point)
}

// A screen ray intersects the construction plane; parallel or clipped rays cannot supply a drawing point.
pub fn on_xy(camera: &crate::camera::Camera, screen: [f32; 2]) -> Option<[f64; 3]> {
    let ray = camera.ray(screen)?;
    if ray.direction[2].abs() < 1e-10 { return None; }
    let distance = -ray.origin[2] / ray.direction[2];
    if distance < 0. || distance > ray.max_distance { return None; }
    checked(std::array::from_fn(|i| ray.origin[i] + distance * ray.direction[i])).ok()
}
