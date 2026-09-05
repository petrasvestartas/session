//! Physical planes for large meshes without building topology. Flat interpolation takes
//! its token from the first vertex; the other two corners can retain their shared rows.

use session_rust::{RenderMesh, RenderVertex};
use crate::engine::gpu::arena::FacePlane;
use super::mesh_faces::FaceCx;

/// A raw mesh's unchanged geometry, first-vertex face tokens and physical triangle planes.
pub struct RawFaces {
    pub render: RenderMesh,
    pub ids: Vec<u32>,
    pub planes: Vec<FacePlane>,
}

/// Compute the plane of the positions actually sent to the GPU, with f64 intermediate
/// differences so small valid triangles do not lose their normal during the cross product.
fn triangle_plane(vertices: &[RenderVertex], triangle: &[u32], row: u32) -> FacePlane {
    let point = vertices[triangle[0] as usize].position;
    let b = vertices[triangle[1] as usize].position;
    let c = vertices[triangle[2] as usize].position;
    let u = [b[0] as f64 - point[0] as f64, b[1] as f64 - point[1] as f64, b[2] as f64 - point[2] as f64];
    let v = [c[0] as f64 - point[0] as f64, c[1] as f64 - point[1] as f64, c[2] as f64 - point[2] as f64];
    let n = [u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0]];
    let length = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
    let normal = if length > 0.0 {
        [(n[0] / length) as f32, (n[1] / length) as f32, (n[2] / length) as f32]
    } else {
        [0.0; 3]
    };
    FacePlane { point, instance_id: row, normal, _pad: 0 }
}

/// Give each triangle an unclaimed first vertex, rotating its indices without changing
/// winding. Only triangles whose three vertices already carry tokens need one copied row.
pub fn decorate(mut render: RenderMesh, cx: &FaceCx) -> RawFaces {
    assert_eq!(render.indices.len() % 3, 0, "raw faces must be triangles");
    let mut ids = vec![0; render.vertices.len()];
    let mut planes = Vec::with_capacity(render.indices.len() / 3);
    let mut copies = Vec::new();
    for triangle in render.indices.chunks_exact_mut(3) {
        let offset = u32::try_from(planes.len()).expect("physical face table overflow");
        let token = cx.base.checked_add(offset).expect("physical face token overflow").checked_add(1).expect("physical face token overflow");
        planes.push(triangle_plane(&render.vertices, triangle, cx.row));
        let mut first = None;
        for (corner, vertex) in triangle.iter().enumerate() {
            if ids[*vertex as usize] == 0 {
                first = Some(corner);
                break;
            }
        }
        if let Some(corner) = first {
            triangle.rotate_left(corner);
            ids[triangle[0] as usize] = token;
        } else {
            copies.push(triangle[0]);
            triangle[0] = u32::try_from(ids.len()).expect("raw vertex index overflow");
            ids.push(token);
        }
    }
    // Defer copying until the count is known: a barely-over-capacity mesh should not double
    // a large RenderVertex allocation for its final few copied rows.
    render.vertices.reserve_exact(copies.len());
    for original in copies {
        render.vertices.push(render.vertices[original as usize]);
    }
    RawFaces { render, ids, planes }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Reuse and copying must preserve every interpolated attribute and winding, while each
    /// first vertex identifies its own plane even when the other two have different tokens.
    #[test]
    fn raw_tokens_preserve_shading_and_triangle_planes() {
        let positions = [[0.0, 0.0, 0.0], [4.0, 0.0, 0.0], [0.0, 3.0, 0.0], [0.0, 0.0, 2.0]];
        let mut vertices = Vec::new();
        for (i, position) in positions.into_iter().enumerate() {
            vertices.push(RenderVertex { position, normal: [i as f32, 2.0, 3.0], color: [0.1 * i as f32, 0.2, 0.3, 1.0] });
        }
        let original = RenderMesh { vertices, indices: vec![0, 1, 2, 0, 2, 3, 0, 3, 1, 0, 1, 2, 0, 2, 3] };
        let out = decorate(original.clone(), &FaceCx { base: 40, row: 7 });
        assert_eq!(out.render.vertices.len(), 5, "only the final triangle needs a copied vertex");
        assert_eq!(out.ids.len(), out.render.vertices.len());
        assert_eq!(out.planes.len(), 5);
        for (face, (before, after)) in original.indices.chunks_exact(3).zip(out.render.indices.chunks_exact(3)).enumerate() {
            let first = out.render.vertices[after[0] as usize];
            let mut rotation = None;
            for (corner, index) in before.iter().enumerate() {
                if bytemuck::bytes_of(&original.vertices[*index as usize]) == bytemuck::bytes_of(&first) {
                    rotation = Some(corner);
                    break;
                }
            }
            let rotation = rotation.expect("first vertex retains its original attributes");
            for corner in 0..3 {
                assert_eq!(bytemuck::bytes_of(&original.vertices[before[(corner + rotation) % 3] as usize]), bytemuck::bytes_of(&out.render.vertices[after[corner] as usize]));
            }
            assert_eq!(out.ids[after[0] as usize], face as u32 + 41);
            let plane = out.planes[face];
            assert_eq!(plane.instance_id, 7);
            for index in after {
                let point = out.render.vertices[*index as usize].position;
                let mut distance = 0.0;
                for (axis, value) in point.iter().enumerate() { distance += (value - plane.point[axis]) * plane.normal[axis]; }
                assert!(distance.abs() < 1e-6);
            }
        }
        assert_ne!(out.planes[0].normal, out.planes[1].normal);
        assert_ne!(out.ids[out.render.indices[0] as usize], out.ids[out.render.indices[1] as usize]);
    }
}
