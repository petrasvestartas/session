use crate::app::gizmo::{ARM, BALL_AT, HUB};
use std::f32::consts::{FRAC_PI_2, PI, TAU};

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: u32,
    handle: u32,
}

const COLORS: [u32; 3] = [0xff2424e8, 0xff30b820, 0xffef6628];
const SIDES: usize = 24;
const SHAFT: f32 = 2.2;
const TIP: f32 = 14.0;

/// One immutable mesh in CSS-pixel units, with ten independently highlighted handles.
pub fn vertices() -> Vec<Vertex> {
    let mut out = Vec::with_capacity(18_576);
    for (axis, color) in COLORS.into_iter().enumerate() {
        let arm = ARM as f32;
        let shaft = [(HUB as f32, SHAFT), (arm - TIP, SHAFT)];
        lathe(&mut out, axis, color, axis as u32, &shaft);
        let cone = [(arm - TIP, 0.0), (arm - TIP, 5.5), (arm, 0.0)];
        lathe(&mut out, axis, color, axis as u32, &cone);
        sphere(&mut out, axis, BALL_AT as f32, 5.0, color, axis as u32 + 6);
        surface(&mut out, 48, 12, color, axis as u32 + 3, |s, t| {
            let angle = s * FRAC_PI_2;
            let tube = t * TAU;
            let radius = arm + 1.8 * tube.cos();
            let position = [
                1.8 * tube.sin(),
                -angle.cos() * radius,
                -angle.sin() * radius,
            ];
            orient(position, axis)
        });
    }
    sphere(&mut out, 0, 0.0, HUB as f32, 0xffd8d8d8, 9);
    out
}

fn orient(p: [f32; 3], axis: usize) -> [f32; 3] {
    match axis {
        0 => p,
        1 => [p[2], p[0], p[1]],
        _ => [p[1], p[2], p[0]],
    }
}

fn lathe(out: &mut Vec<Vertex>, axis: usize, color: u32, handle: u32, profile: &[(f32, f32)]) {
    for pair in profile.windows(2) {
        let [(x0, r0), (x1, r1)] = [pair[0], pair[1]];
        surface(out, 1, SIDES, color, handle, |s, t| {
            let (sin, cos) = (t * TAU).sin_cos();
            let r = r0 + (r1 - r0) * s;
            let position = [x0 + (x1 - x0) * s, r * cos, r * sin];
            orient(position, axis)
        });
    }
}

fn sphere(out: &mut Vec<Vertex>, axis: usize, at: f32, radius: f32, color: u32, handle: u32) {
    surface(out, 12, SIDES, color, handle, |s, t| {
        let (sin, cos) = (s * PI).sin_cos();
        let (v, u) = (t * TAU).sin_cos();
        let position = [at + radius * cos, radius * sin * u, radius * sin * v];
        orient(position, axis)
    });
}

fn surface(
    out: &mut Vec<Vertex>,
    rows: usize,
    columns: usize,
    color: u32,
    handle: u32,
    sample: impl Fn(f32, f32) -> [f32; 3],
) {
    for row in 0..rows {
        for column in 0..columns {
            for (i, j) in [(0, 0), (1, 0), (1, 1), (0, 0), (1, 1), (0, 1)] {
                let position = sample(
                    (row + i) as f32 / rows as f32,
                    (column + j) as f32 / columns as f32,
                );
                out.push(Vertex {
                    position,
                    color,
                    handle,
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mesh_is_bounded_finite_and_covers_all_handles() {
        let mesh = vertices();
        assert!(mesh.len() < 20_000);
        let mut handles = [false; 10];
        for vertex in mesh {
            handles[vertex.handle as usize] = true;
            assert!(
                vertex
                    .position
                    .iter()
                    .all(|v| v.is_finite() && v.abs() <= ARM as f32 + 2.0)
            );
        }
        assert!(handles.into_iter().all(|v| v));
    }
}
