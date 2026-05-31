//! Construction plane resolved from the active camera, plus ray->plane click placement.
//!
//! The plane follows the view: in ortho the camera forward axis selects the plane
//! (Top/Bottom -> XY, Left/Right -> YZ, an orbited front -> XZ); perspective falls
//! back to XY. The plane passes through the world origin.

use session_rust::{Line, Plane, Point, Vector};
use crate::camera::{Camera, ProjMode};
use crate::pick::Ray;

/// Construction plane through the world origin, chosen from the camera orientation.
pub fn resolve(camera: &Camera) -> Plane {
    if camera.proj_mode == ProjMode::Perspective {
        return Plane::xy_plane();
    }
    // Camera forward in world space is the third row of the view matrix.
    let v = camera.view_matrix();
    let (fx, fy, fz) = (v[0][2].abs(), v[1][2].abs(), v[2][2].abs());
    if fz >= fx && fz >= fy {
        Plane::xy_plane()
    } else if fx >= fy {
        Plane::from_point_normal(Point::new(0.0, 0.0, 0.0), Vector::new(1.0, 0.0, 0.0))
    } else {
        Plane::from_point_normal(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0))
    }
}

/// Intersect a screen ray with the construction plane. Falls back to the plane
/// projection of the ray origin if the ray is parallel to the plane.
pub fn ray_to_plane(ray: &Ray, plane: &Plane) -> Point {
    let o = ray.origin;
    let d = ray.direction;
    const FAR: f32 = 1.0e6; // mm; two points define the infinite line for line_plane
    let line = Line::new(
        o[0] as f64, o[1] as f64, o[2] as f64,
        (o[0] + d[0] * FAR) as f64, (o[1] + d[1] * FAR) as f64, (o[2] + d[2] * FAR) as f64,
    );
    match session_rust::intersection::line_plane(&line, plane, false) {
        Some(p) => p,
        None => plane.projection(&Point::new(o[0] as f64, o[1] as f64, o[2] as f64)),
    }
}
