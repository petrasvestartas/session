use session_rust::AABB;
use crate::clipping_geometry::ClipPlane;
impl ClipPlane{
    pub fn crosses(&self, b: &AABB, anchor: [f64; 3]) -> bool {
        let reach =
            self.normal[0].abs() * b.hx + self.normal[1].abs() * b.hy + self.normal[2].abs() * b.hz;
        let center = [b.cx, b.cy, b.cz];
        let size = b.hx + b.hy + b.hz;
        let local: f64 = (0..3).map(|i| (center[i] - anchor[i]).abs()).sum();
        let world: f64 = center.iter().map(|v| v.abs()).sum();
        // 32 times CLIP_SLACK in clip.wgsl, plus the f32 rounding of the vertices themselves
        let slack =
            (local + size + self.distance(anchor).abs()) / 4096.0 + (world + size) / 1048576.0;
        self.distance(center).abs() <= reach + slack
    }
}
