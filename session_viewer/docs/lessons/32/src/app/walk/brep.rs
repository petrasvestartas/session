use super::brep_edges::{EdgeChain, EdgePen, edge_chains, push_edge_pipes};
use super::brep_orient::face_signs;
use super::curves::{push_polyline, sample_nurbscurve};
use super::encode::{Pen, encode_width, pack_rgba};
// --8<-- [start:step-28a]
use super::mesh::mesh_spacing;
// --8<-- [end:step-28a]
use super::mesh_ink::Ink;
use super::{Row, WalkCx};
use crate::app::knobs;
use crate::engine::gpu::Instance;
use crate::engine::gpu::arena::ArenaRows;
use session_rust::AABB;
use session_rust::{BRep, Color, Mesh, NurbsSurface, RenderMesh};

/// Mesh quality: 5° between samples, chord sag 0.001 of the size.
pub const QUALITY: (f64, f64) = (5.0, 0.001);

/// Every face uploaded so far.
struct Solid {
    pos: Vec<[f32; 3]>, // vertex positions
    tris: Vec<u32>,     // triangle indices into `pos`
    bounds: AABB,       // box of all vertices
}

/// Append one face mesh to the arena.
fn push_face(arena: &mut ArenaRows, rm: &RenderMesh, cx: &WalkCx, solid: &mut Solid, face: usize) {
    // one source face address for every triangle
    let address = arena.face_sources.len() as u32;
    arena
        .face_sources
        .push(crate::engine::gpu::faces::FaceSource {
            parent: cx.row,
            face,
        });
    arena
        .face_ids
        .extend(std::iter::repeat_n(address, rm.indices.len() / 3));
    let base = cx.vert_base + arena.verts.len() as u32; // first GPU vertex index
    let local = solid.pos.len() as u32; // first index in `solid`
    arena.verts.reserve(rm.vertices.len());
    arena.vids.reserve(rm.vertices.len());

    for v in &rm.vertices {
        solid.bounds.union_with_point(
            v.position[0] as f64,
            v.position[1] as f64,
            v.position[2] as f64,
        );
        solid.pos.push(v.position);
        arena.verts.push(*v);
        arena.vids.push(cx.row);
    }

    arena.idx.reserve(rm.indices.len());

    for &i in &rm.indices {
        arena.idx.push(base + i);
        solid.tris.push(local + i);
    }
}

/// A BRep: every face meshed and uploaded, then its edges.
pub fn walk_brep(arena: &mut ArenaRows, ink: &mut Ink, b: &BRep, cx: &WalkCx) -> Row {
    let mut fms = b.face_meshes_q(Some(QUALITY)); // one mesh per face
    let chains = edge_chains(b, &fms); // edge polylines on the meshes
    let signs = face_signs(b, &fms, &chains); // +1 or -1 per face
    let mut solid = Solid {
        pos: Vec::new(),
        tris: Vec::new(),
        bounds: AABB::empty(),
    };
    let mut verts = 0; // vertex total for spacing
    // --8<-- [start:step-28b]
    let mut boundary_vertices = Vec::with_capacity(fms.len()); // per face: vertex key to GPU index
    // --8<-- [end:step-28b]

    for (fi, fm) in fms.iter_mut().enumerate() {
        fm.set_objectcolor(b.surfacecolor.clone());
        verts += fm.vertex.len();
        let mut rm = fm.to_render();

        // an inside-out face: flip normals and winding
        if signs[fi] < 0.0 {
            for vertex in &mut rm.vertices {
                for component in &mut vertex.normal {
                    *component = -*component;
                }
            }

            for triangle in rm.indices.chunks_exact_mut(3) {
                triangle.swap(1, 2);
            }
        }

        // --8<-- [start:step-28c]
        let mut keys: Vec<_> = fm.vertex.keys().copied().collect();
        keys.sort_unstable();
        boundary_vertices.push(
            keys.into_iter()
                .enumerate()
                .map(|(i, key)| (key, arena.verts.len() as u32 + i as u32))
                .collect::<std::collections::HashMap<_, _>>(),
        );
        // --8<-- [end:step-28c]
        let surface_index = b.m_faces[fi].surface_index as usize;
        cache_samples(arena, fm, &rm, &b.m_surfaces[surface_index], surface_index); // uv per vertex for live edits
        push_face(arena, &rm, cx, &mut solid, fi);
    }

    let mut flags = Instance::FLAG_SMOOTH; // vertices are samples

    if !b.is_solid() {
        flags |= Instance::FLAG_OPEN;
    }

    if b.face_count() == 1 {
        flags |= Instance::FLAG_SINGLE;
    }

    let mut row = Row {
        bounds: solid.bounds,
        spacing: mesh_spacing(&solid.bounds, verts),
        flags,
        faces: true,
    };

    if !knobs::no_edges() {
        let pen = Pen {
            row: cx.row,
            radius: encode_width(b.width),
            color: pack_rgba(Color::black().to_f32()),
        };
        let ep = EdgePen::new(&fms, &signs, pen);
        // --8<-- [start:step-28d]
        walk_brep_edges(
            ink,
            b,
            &chains,
            (&ep, &mut row.bounds),
            arena,
            &boundary_vertices,
        );
        // --8<-- [end:step-28d]
    }

    row
}

/// Every BRep edge as pipes, or as a ribbon when no mesh owns it.
fn walk_brep_edges(
    ink: &mut Ink,
    b: &BRep,
    chains: &[Option<EdgeChain>],
    out: (&EdgePen, &mut AABB),
    // --8<-- [start:step-28e]
    arena: &mut ArenaRows,
    boundary_vertices: &[std::collections::HashMap<usize, u32>],
) {
    let (ep, bounds) = out;

    for (ei, chain) in chains.iter().enumerate() {
        match chain {
            Some(c) => {
                let mut pipe = ink.seg.pipes.len() as u32;
                push_edge_pipes(ink.seg, c, ep, bounds);
                // remember which vertices each pipe joins
                for pair in c.keys.windows(2) {
                    let ends = [
                        boundary_vertices[c.face][&pair[0]],
                        boundary_vertices[c.face][&pair[1]],
                    ];
                    let a = arena.verts[ends[0] as usize].position;
                    let b = arena.verts[ends[1] as usize].position;
                    if a == b || !a.into_iter().chain(b).all(f32::is_finite) {
                        continue;
                    }
                    arena.surface_boundaries.push((pipe, ends));
                    pipe += 1;
                }
                // --8<-- [end:step-28e]
            }
            None => {
                if !b.m_edges[ei].degenerated && !b.edge_faces(ei).is_empty() {
                    log::warn!(
                        "BREP {:?} edge {ei}: missing tessellation boundary mapping; analytic display fallback is not a coherent CAD boundary",
                        b.name
                    );
                }

                push_curve_ribbon(ink, b, ei, (&ep.pen, bounds));
            }
        }
    }
}

/// An edge sampled from its 3D curve as a ribbon.
fn push_curve_ribbon(ink: &mut Ink, b: &BRep, ei: usize, out: (&Pen, &mut AABB)) {
    let edge = &b.m_edges[ei];

    if edge.degenerated || edge.curve_3d_index < 0 {
        return;
    }

    let points: Vec<[f32; 3]> = sample_nurbscurve(&b.m_curves_3d[edge.curve_3d_index as usize])
        .into_iter()
        .map(super::curves::render_position)
        .collect();

    if points.len() < 2 {
        return;
    }

    push_polyline(ink.seg, &points, out.0, out.1);
}

// --8<-- [start:step-28f]
/// A NURBS surface as a fixed UV grid with its four border edges.
pub fn walk_surface(arena: &mut ArenaRows, ink: &mut Ink, s: &NurbsSurface, cx: &WalkCx) -> Row {
    let (Some((u0, u1)), Some((v0, v1))) = (s.domain(0), s.domain(1)) else {
        return Row::thin(AABB::empty());
    };
    let nu = (s.m_cv_count[0] * 4).clamp(16, 96); // grid steps in u
    let nv = (s.m_cv_count[1] * 4).clamp(16, 96); // grid steps in v
    let first = arena.verts.len(); // index of this surface's first vertex
    let base = cx.vert_base + first as u32; // first GPU vertex index
    let color = s.facecolors.first().cloned().unwrap_or_default().to_f32();
    let mut bounds = AABB::empty();
    // one vertex per grid point
    for i in 0..=nu {
        for j in 0..=nv {
            let u = u0 + (u1 - u0) * i as f64 / nu as f64;
            let v = v0 + (v1 - v0) * j as f64 / nv as f64;
            let point = s.point_at(u, v).unwrap_or_default();
            let normal = s.normal_at(u, v);
            bounds.union_with_point(point[0], point[1], point[2]);
            arena
                .surface_samples
                .push(crate::app::surface_preview::Sample {
                    index: arena.verts.len() as u32,
                    surface: 0,
                    uv: [u, v],
                    sign: 1.0,
                });
            arena.verts.push(session_rust::RenderVertex {
                position: point.to_f32(),
                normal: [normal[0] as f32, normal[1] as f32, normal[2] as f32],
                color,
            });
            arena.vids.push(cx.row);
        }
    }
    let address = arena.face_sources.len() as u32;
    arena
        .face_sources
        .push(crate::engine::gpu::faces::FaceSource {
            parent: cx.row,
            face: 0,
        });
    // two triangles per grid cell
    for i in 0..nu {
        for j in 0..nv {
            let a = base + (i * (nv + 1) + j) as u32;
            let b = a + (nv + 1) as u32;
            arena.idx.extend_from_slice(&[a, b, b + 1, a, b + 1, a + 1]);
            arena.face_ids.extend_from_slice(&[address, address]);
        }
    }
    if !knobs::no_edges() {
        ink.seg.pipe_ids.resize(ink.seg.pipes.len(), u32::MAX);
        // the four border edges, skipping a closed direction
        for edge in 0..4 {
            let direction = edge / 2;
            if s.is_closed(direction) {
                continue;
            }
            let indices: Vec<_> = match edge {
                0 => (0..=nv).collect(),
                1 => (0..=nv).map(|j| nu * (nv + 1) + j).collect(),
                2 => (0..=nu).map(|i| i * (nv + 1)).collect(),
                _ => (0..=nu).map(|i| i * (nv + 1) + nv).collect(),
            };
            let start = ink.seg.pipes.len() as u32;
            for ends in indices.windows(2) {
                ink.seg.pipes.push(crate::engine::gpu::CylinderSegment {
                    p0: arena.verts[first + ends[0]].position,
                    p1: arena.verts[first + ends[1]].position,
                    radius: encode_width(s.width),
                    instance_id: cx.row,
                    color: super::encode::BLACK,
                    facing: super::encode::FACING_UNKNOWN,
                });
                arena.surface_boundaries.push((
                    ink.seg.pipes.len() as u32 - 1,
                    [(first + ends[0]) as u32, (first + ends[1]) as u32],
                ));
                ink.seg.pipe_ids.push(edge as u32);
            }
            ink.seg.pipe_chains.push(start..ink.seg.pipes.len() as u32);
        }
    }
    Row {
        bounds,
        spacing: mesh_spacing(&bounds, (nu + 1) * (nv + 1)),
        flags: Instance::FLAG_SINGLE | Instance::FLAG_SMOOTH | Instance::FLAG_OPEN,
        faces: true,
        // --8<-- [end:step-28f]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::walk::brep_edges::edge_chains;
    use crate::engine::gpu::Instance;
    use crate::engine::gpu::glyphs::GlyphRows;
    use crate::engine::gpu::segments::SegRows;

    /// Walk one BRep at row 5.
    fn walked(b: &BRep) -> (ArenaRows, SegRows, GlyphRows, Row) {
        let mut arena = ArenaRows::default();
        let mut seg = SegRows::default();
        let mut glyph = GlyphRows::default();
        let cx = WalkCx {
            vert_base: 100,
            cloud_px: 0.0,
            row: 5,
        };
        let row = {
            let mut ink = Ink {
                seg: &mut seg,
                glyph: &mut glyph,
            };
            walk_brep(&mut arena, &mut ink, b, &cx)
        };
        (arena, seg, glyph, row)
    }

    // --8<-- [start:step-28g]
    /// The teapot's pipes lie on its authored edge curves.
    #[test]
    fn teapot_ink_is_only_authored_patch_boundaries() {
        let scene = session_rust::Session::pb_load(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/pb/view_mixed_teapot.pb"
        ));
        let brep = &scene.objects.breps[0];
        let (arena, seg, glyph, _) = walked(brep);
        assert!(seg.ribbons.is_empty() && glyph.spheres.is_empty());
        assert_eq!(arena.surface_boundaries.len(), seg.pipes.len());
        let curves: Vec<Vec<_>> = brep
            .m_edges
            .iter()
            .map(|edge| {
                if edge.curve_3d_index < 0 {
                    return Vec::new();
                }
                let curve = &brep.m_curves_3d[edge.curve_3d_index as usize];
                let (a, b) = curve.domain();
                (0..=1024)
                    .map(|i| curve.point_at(a + (b - a) * i as f64 / 1024.0).to_f32())
                    .collect()
            })
            .collect();
        for &(pipe, ends) in &arena.surface_boundaries {
            let edge = seg.pipe_ids[pipe as usize] as usize;
            for index in ends {
                let p = arena.verts[index as usize].position;
                let distance = curves[edge]
                    .windows(2)
                    .map(|pair| {
                        let d = std::array::from_fn::<_, 3, _>(|i| pair[1][i] - pair[0][i]);
                        let q = std::array::from_fn::<_, 3, _>(|i| p[i] - pair[0][i]);
                        let length: f32 = d.iter().map(|v| v * v).sum();
                        let t = ((0..3).map(|i| q[i] * d[i]).sum::<f32>() / length.max(1e-20))
                            .clamp(0.0, 1.0);
                        (0..3)
                            .map(|i| (q[i] - d[i] * t).powi(2))
                            .sum::<f32>()
                            .sqrt()
                    })
                    .fold(f32::INFINITY, f32::min);
                assert!(
                    distance < 0.01,
                    "edge {edge} endpoint {p:?} is {distance} from its authored curve"
                );
            }
            assert_eq!(
                seg.pipes[pipe as usize].p0,
                arena.verts[ends[0] as usize].position
            );
            assert_eq!(
                seg.pipes[pipe as usize].p1,
                arena.verts[ends[1] as usize].position
            );
            assert!((seg.pipe_ids[pipe as usize] as usize) < brep.m_edges.len());
        }
    }

    /// A cylinder uploads unwelded faces with unit normals.
// --8<-- [end:step-28g]
    #[test]
    fn cylinder_walks_unwelded_with_normals() {
        let b = BRep::create_cylinder(150.0, 400.0);
        let fms = b.face_meshes_q(Some(QUALITY));
        let verts: usize = fms.iter().map(|m| m.vertex.len()).sum();
        let tris: usize = fms.iter().map(|m| m.to_render().indices.len()).sum();
        let segments: usize = edge_chains(&b, &fms)
            .iter()
            .flatten()
            .map(|c| c.keys.len() - 1)
            .sum();
        let (arena, seg, glyph, row) = walked(&b);
        assert_eq!(arena.verts.len(), verts);
        assert_eq!(arena.vids.len(), verts);
        assert_eq!(arena.idx.len(), tris);
        assert!(
            arena
                .idx
                .iter()
                .all(|&i| i >= 100 && i < 100 + verts as u32)
        );

        for v in &arena.verts {
            let n = v.normal;
            let l = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
            assert!((l - 1.0).abs() < 1e-3, "normal {n:?}");
        }

        assert_eq!(seg.pipes.len(), segments);
        assert!(seg.ribbons.is_empty());
        assert!(glyph.spheres.is_empty());
        assert_ne!(row.flags & Instance::FLAG_SMOOTH, 0);
        assert_eq!(row.flags & Instance::FLAG_OPEN, 0);
        assert!(row.faces);
        assert!(row.bounds.diagonal() > 400.0);
    }

    /// A shell without a solid is flagged open.
    #[test]
    fn open_brep_is_flagged_open() {
        let mut b = BRep::create_box(400.0, 300.0, 250.0);
        b.m_solids.clear();
        let (_, _, _, row) = walked(&b);
        assert_ne!(row.flags & Instance::FLAG_OPEN, 0);
    }

    /// Flipped face uses upload the same vertices and triangles.
    #[test]
    fn reversed_solid_uses_repair_faces_and_boundaries_together() {
        let b = BRep::create_cylinder(150.0, 400.0);
        let mut reversed = b.clone();

        for face in reversed.m_shells[0].faces.iter_mut().take(2) {
            face.orientation = session_rust::brep::brep_reverse(face.orientation);
        }

        let (expected, _, _, _) = walked(&b);
        let (actual, _, _, _) = walked(&reversed);
        assert_eq!(actual.idx.len(), expected.idx.len());

        for (a, b) in actual.idx.chunks_exact(3).zip(expected.idx.chunks_exact(3)) {
            assert!(a == b || a == [b[1], b[2], b[0]] || a == [b[2], b[0], b[1]]);
        }

        assert_eq!(actual.verts.len(), expected.verts.len());

        for (a, b) in actual.verts.iter().zip(&expected.verts) {
            assert_eq!(a.position, b.position);
            assert_eq!(a.normal, b.normal);
        }
    }

    /// A surface's four border edges carry ids 0 to 3.
    #[test]
    fn surface_domain_edges_have_source_ids_and_open_visibility() {
        let b = BRep::create_box(40.0, 30.0, 25.0);
        let mut arena = ArenaRows::default();
        let mut seg = SegRows::default();
        let mut glyph = GlyphRows::default();
        let mut ink = Ink {
            seg: &mut seg,
            glyph: &mut glyph,
        };
        let cx = WalkCx {
            vert_base: 0,
            cloud_px: 0.0,
            row: 7,
        };
        let row = walk_surface(&mut arena, &mut ink, &b.m_surfaces[0], &cx);
        assert_ne!(row.flags & Instance::FLAG_OPEN, 0);
        assert_eq!(seg.pipe_ids.len(), seg.pipes.len());
        let mut ids = seg.pipe_ids.clone();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids, vec![0, 1, 2, 3]);
    }

    /// Sphere poles and box corners keep unit normals.
    #[test]
    fn poles_and_sharp_faces_keep_finite_unit_normals() {
        for b in [
            BRep::create_sphere(180.0),
            BRep::create_cone(150.0, 400.0),
            BRep::create_box(40.0, 30.0, 25.0),
        ] {
            let (arena, seg, _, _) = walked(&b);

            for vertex in &arena.verts {
                let [x, y, z] = vertex.normal;
                assert!(
                    (x * x + y * y + z * z - 1.0).abs() < 1e-5,
                    "{} normal {:?}",
                    b.name,
                    vertex.normal
                );
                assert!(vertex.position.iter().all(|value| value.is_finite()));
            }

            for pipe in &seg.pipes {
                assert_ne!(pipe.p0, pipe.p1);
            }
        }

        let (arena, _, _, _) = walked(&BRep::create_box(40.0, 30.0, 25.0));

        for vertex in &arena.verts {
            assert_eq!(
                vertex
                    .normal
                    .iter()
                    .filter(|component| component.abs() > 0.0)
                    .count(),
                1
            );
        }
    }

    /// Each pyramid side keeps its own normal at the apex.
    #[test]
    fn pyramid_planar_faces_keep_constant_normals_through_collapsed_apex() {
        /// Dot product in f32.
        fn dot(a: [f32; 3], b: [f32; 3]) -> f32 {
            a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
        }
        let pyramid = BRep::create_pyramid(400.0, 350.0);
        let (arena, _, _, _) = walked(&pyramid);
        let mut apex_normals = Vec::new();

        for triangle in arena.idx.chunks_exact(3) {
            let a = arena.verts[triangle[0] as usize - 100];
            let b = arena.verts[triangle[1] as usize - 100];
            let c = arena.verts[triangle[2] as usize - 100];
            let mut ab = [0.0; 3];
            let mut ac = [0.0; 3];

            for axis in 0..3 {
                ab[axis] = b.position[axis] - a.position[axis];
                ac[axis] = c.position[axis] - a.position[axis];
            }

            let mut face = [
                ab[1] * ac[2] - ab[2] * ac[1],
                ab[2] * ac[0] - ab[0] * ac[2],
                ab[0] * ac[1] - ab[1] * ac[0],
            ];
            let length = dot(face, face).sqrt();

            for component in &mut face {
                *component /= length;
            }

            for vertex in [a, b, c] {
                let normal = vertex.normal;
                assert!(
                    dot(normal, face) > 1.0 - 1e-6,
                    "planar face normal {normal:?} differs from {face:?}"
                );

                if vertex.position[2] == 350.0 {
                    apex_normals.push(normal);
                }
            }
        }

        assert_eq!(
            apex_normals.len(),
            4,
            "each separate planar side owns its apex vertex"
        );

        for first in 0..apex_normals.len() {
            for second in first + 1..apex_normals.len() {
                assert!(
                    dot(apex_normals[first], apex_normals[second]) < 0.9,
                    "distinct BRep faces must not share a smoothed apex normal"
                );
            }
        }
    }
}

/// Remember each vertex's uv on its surface.
fn cache_samples(
    arena: &mut ArenaRows,
    mesh: &Mesh,
    render: &RenderMesh,
    surface: &NurbsSurface,
    index: usize,
) {
    let mut rows: Vec<_> = mesh.vertex.iter().collect();
    rows.sort_unstable_by_key(|&(key, _)| *key);

    if rows.len() != render.vertices.len() {
        return; // vertices were split, no mapping
    }

    for (offset, ((_, vertex), rendered)) in rows.into_iter().zip(&render.vertices).enumerate() {
        let (Some(&u), Some(&v)) = (vertex.attributes.get("u"), vertex.attributes.get("v")) else {
            continue;
        };
        let normal = surface.normal_at(u, v);
        // sign flips when the rendered normal was reversed
        let dot = (0..3)
            .map(|d| normal[d] * rendered.normal[d] as f64)
            .sum::<f64>();
        arena
            .surface_samples
            .push(crate::app::surface_preview::Sample {
                index: arena.verts.len() as u32 + offset as u32,
                surface: index as u32,
                uv: [u, v],
                sign: if dot < 0.0 { -1.0 } else { 1.0 },
            });
    }
}
