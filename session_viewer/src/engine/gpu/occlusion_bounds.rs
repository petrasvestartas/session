//! Conservative screen bounds for the physical depth writers. Leaving a bound only skips
//! a texture lookup; uncertain projections keep the full viewport and normal visibility.

use crate::math::Aabb;

/// A physical object's projection and raster footprint, all in framebuffer pixels.
pub struct Projection<'a> {
    pub mvp: &'a [f32; 16],
    pub size: (u32, u32),
    pub padding: f64,
}

/// Bound a float dot product and propagated input uncertainty. Sixteen float epsilons
/// cover the matrix multiply plus the separate anchored translation used by `place()`.
fn transform(matrix: &[f32; 16], point: [f64; 4], error: [f64; 4]) -> ([f64; 4], [f64; 4]) {
    let mut value = [0.0; 4];
    let mut uncertainty = [0.0; 4];
    for axis in 0..4 {
        let mut magnitude = 0.0;
        for column in 0..4 {
            let coefficient = matrix[column * 4 + axis] as f64;
            value[axis] += coefficient * point[column];
            magnitude += (coefficient * point[column]).abs();
            uncertainty[axis] += coefficient.abs() * error[column];
        }
        uncertainty[axis] += 16.0 * f32::EPSILON as f64 * magnitude;
    }
    (value, uncertainty)
}

/// Project all eight corners with outward numerical bounds. A box touching either depth
/// clip plane or the eye falls back to full-screen visibility instead of dividing there.
pub fn project(bounds: &Aabb, model: &[f32; 16], projection: &Projection<'_>) -> Option<[f32; 4]> {
    if !bounds.is_finite() { return None; }
    let mut low = [f64::INFINITY; 2];
    let mut high = [f64::NEG_INFINITY; 2];
    for corner in 0..8 {
        let point = std::array::from_fn(|axis| {
            if axis == 3 { 1.0 } else if corner & (1 << axis) == 0 { bounds.min[axis] as f64 } else { bounds.max[axis] as f64 }
        });
        let (mut world, mut world_error) = transform(model, point, [0.0; 4]);
        world[3] = 1.0;
        world_error[3] = 0.0;
        let (clip, error) = transform(projection.mvp, world, world_error);
        let near_w = clip[3] - error[3];
        if !clip.iter().chain(error.iter()).all(|v| v.is_finite())
            || near_w <= 0.0 || clip[2] - error[2] <= 0.0 || clip[2] + error[2] >= near_w {
            return None;
        }
        for axis in 0..2 {
            for numerator in [clip[axis] - error[axis], clip[axis] + error[axis]] {
                for denominator in [near_w, clip[3] + error[3]] {
                    let ndc = numerator / denominator;
                    let pixel = if axis == 0 { (ndc * 0.5 + 0.5) * projection.size.0 as f64 }
                        else { (0.5 - ndc * 0.5) * projection.size.1 as f64 };
                    low[axis] = low[axis].min(pixel);
                    high[axis] = high[axis].max(pixel);
                }
            }
        }
    }
    let rect = [(low[0] - projection.padding).floor() as f32, (low[1] - projection.padding).floor() as f32,
        (high[0] + projection.padding).ceil() as f32, (high[1] + projection.padding).ceil() as f32];
    rect.iter().all(|v| v.is_finite()).then_some(rect)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every interior sample stays in the conservative rectangle under affine placement,
    /// perspective, and nonuniform scale; near-plane crossings require the fallback.
    #[test]
    fn conservative_projection_and_near_fallback() {
        let bounds = Aabb { min: [-1.0, -2.0, -0.5], max: [1.0, 2.0, 0.5] };
        let model = [2.0, 0.2, 0.0, 0.0, 0.3, 0.4, 0.1, 0.0, 0.0, 0.1, 3.0, 0.0, 20.0, -8.0, 40.0, 1.0];
        let mvp = [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0];
        let projection = Projection { mvp: &mvp, size: (1400, 900), padding: 2.0 };
        let rect = project(&bounds, &model, &projection).expect("box is beyond the near plane");
        for x in 0..7 {
            for y in 0..7 {
                for z in 0..7 {
                    let point = [-1.0 + x as f32 / 3.0, -2.0 + y as f32 * 2.0 / 3.0, -0.5 + z as f32 / 6.0, 1.0];
                    // Exercise the rounded shader path independently of the interval helper.
                    let shader_transform = |matrix: &[f32; 16], point: [f32; 4]| -> [f32; 4] {
                        std::array::from_fn(|axis| (0..4).fold(0.0, |sum, column| matrix[column * 4 + axis].mul_add(point[column], sum)))
                    };
                    let clip = shader_transform(&mvp, shader_transform(&model, point));
                    let pixel = [(clip[0] / clip[3] * 0.5 + 0.5) * 1400.0, (0.5 - clip[1] / clip[3] * 0.5) * 900.0];
                    assert!(pixel[0] >= rect[0] && pixel[0] <= rect[2]);
                    assert!(pixel[1] >= rect[1] && pixel[1] <= rect[3]);
                }
            }
        }
        let mut crossing = model;
        crossing[14] = 1.0;
        assert!(project(&bounds, &crossing, &projection).is_none());
    }
}
