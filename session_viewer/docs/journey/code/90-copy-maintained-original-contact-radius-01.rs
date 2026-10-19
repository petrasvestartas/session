use session_rust::AABB;
pub fn ambient_radius(bounds: &AABB) -> f32 {
    if !bounds.is_valid() {
        return 0.0;
    }
    (0.05 * bounds.hx.hypot(bounds.hy).hypot(bounds.hz)).max(0.01) as f32
}
