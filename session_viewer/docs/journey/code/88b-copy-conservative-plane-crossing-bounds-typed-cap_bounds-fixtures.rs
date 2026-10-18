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
#[cfg(test)]
mod tests{use super::*;use crate::clipping_geometry::{Mode,clip_plane,plane_from};use session_rust::Xform;
#[test]
fn a_plane_crosses_the_boxes_it_passes_through() {
    let plane = clip_plane(
        &plane_from(Mode::Xy, &[[0.0, 0.0, 50.0]], 10.0).unwrap(),
        &Xform::identity(),
    )
    .unwrap();
    let tilted = clip_plane(
        &plane_from(Mode::Normal, &[[0.0, 0.0, 0.0], [1.0, 1.0, 1.0]], 10.0).unwrap(),
        &Xform::identity(),
    )
    .unwrap();
    let block = |z: f64| AABB::new(0.0, 0.0, z, 10.0, 10.0, 10.0);
    let origin = [0.0; 3];
    assert!(plane.crosses(&block(45.0), origin));
    assert!(plane.crosses(&block(60.0), origin), "touching from above");
    assert!(!plane.crosses(&block(61.0), origin));
    assert!(!plane.crosses(&block(-100.0), origin));
    assert!(tilted.crosses(&block(0.0), origin));
    assert!(!tilted.crosses(&AABB::new(40.0, 40.0, 40.0, 5.0, 5.0, 5.0), origin));

    // a face on the plane whose box rounds a hair short of it still counts
    let bottom = clip_plane(
        &plane_from(Mode::Normal, &[[0.0, 0.0, 5.9], [0.0, 0.0, -4.1]], 10.0).unwrap(),
        &Xform::identity(),
    )
    .unwrap();
    let mut b = AABB::empty();
    b.union_with_point(13.7, -21.3, 5.9 + 1e-9);
    b.union_with_point(113.7, 58.7, 65.9);
    assert!(bottom.crosses(&b, [63.7, 18.7, 35.9]));
}
}
