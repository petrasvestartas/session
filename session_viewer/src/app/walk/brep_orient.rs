//! Which way each BRep face's stored normals point, read from the tessellation alone. Two
//! faces that share an edge and walk it in opposite directions agree; a group of faces that
//! encloses negative volume is inside out. The face-use flags are never read, so a file whose
//! uses are flipped inks exactly as one whose uses are not - the orbit_flipped check of the
//! ink suite - and a solid stored inside out is culled as if it were not.

use super::brep_edges::EdgeChain;
use session_rust::{BRep, Mesh};

/// Whether a face of `fm` walks the directed edge `s -> n`: Some(true) when one does and none
/// walks it back, Some(false) for the reverse, None when both or neither (an interior seam,
/// or two keys that are not neighbours).
fn walks(fm: &Mesh, s: usize, n: usize) -> Option<bool> {
    let fwd = occupied_halfedge(fm, s, n);
    let back = occupied_halfedge(fm, n, s);
    match (fwd, back) {
        (true, false) => Some(true),
        (false, true) => Some(false),
        _ => None,
    }
}

/// Whether the directed halfedge exists and belongs to a face.
fn occupied_halfedge(fm: &Mesh, from: usize, to: usize) -> bool {
    let Some(neighbours) = fm.halfedge.get(&from) else {
        return false;
    };
    matches!(neighbours.get(&to), Some(Some(_)))
}

/// The position of vertex `k` of `fm`.
fn at(fm: &Mesh, k: usize) -> [f64; 3] {
    let v = &fm.vertex[&k];
    [v.x, v.y, v.z]
}

/// The neighbour of `s` in `fm` that leaves it most nearly along `dir`: the next sample of the
/// same boundary curve. A maximum, not a threshold.
fn neighbour_along(fm: &Mesh, s: usize, dir: [f64; 3]) -> Option<usize> {
    let p = at(fm, s);
    let mut best: Option<(f64, usize)> = None;
    for &w in fm.halfedge.get(&s)?.keys() {
        let q = at(fm, w);
        let d = [q[0] - p[0], q[1] - p[1], q[2] - p[2]];
        let l = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
        if l == 0.0 {
            continue;
        }
        let c = (d[0] * dir[0] + d[1] * dir[1] + d[2] * dir[2]) / l;
        if match best {
            Some((bc, bw)) => c > bc || (c == bc && w < bw),
            None => true,
        } {
            best = Some((c, w));
        }
    }
    Some(best?.1)
}

/// The vertex of `fm` nearest `p`: where the other face samples the shared edge's start. Two
/// grid faces put that vertex down bit for bit and the distance is zero, but a CDT face
/// re-evaluates the surface and lands an ULP off (the block with hole's rim: 80.0 against
/// 80.00000000000001), so a minimum is taken rather than an equality - and a minimum, with an
/// exact-distance tie broken by the smaller key, is still no tolerance.
fn vertex_at(fm: &Mesh, p: [f64; 3]) -> Option<usize> {
    let mut best: Option<(f64, usize)> = None;
    for (&k, v) in fm.vertex.iter() {
        let d = (v.x - p[0]).powi(2) + (v.y - p[1]).powi(2) + (v.z - p[2]).powi(2);
        if match best {
            Some((bd, bk)) => d < bd || (d == bd && k < bk),
            None => true,
        } {
            best = Some((d, k));
        }
    }
    Some(best?.1)
}

/// Do the owner and the other face walk the chain's first segment in opposite directions?
/// Opposite is what consistent winding means; None when either side cannot say.
fn opposed(fms: &[Mesh], c: &EdgeChain) -> Option<bool> {
    let other = c.other?;
    let (fa, fb) = (&fms[c.face], &fms[other]);
    let (s, n) = (c.keys[0], c.keys[1]);
    let away_a = walks(fa, s, n)?;
    let (ps, pn) = (at(fa, s), at(fa, n));
    let dir = [pn[0] - ps[0], pn[1] - ps[1], pn[2] - ps[2]];
    let sb = vertex_at(fb, ps)?;
    let nb = neighbour_along(fb, sb, dir)?;
    let away_b = walks(fb, sb, nb)?;
    Some(away_a != away_b)
}

/// Six times the signed volume the triangles of face mesh `fm` sweep about the origin. The
/// faces are summed in key order because float addition is not associative: taken in the
/// map's own order the total's bits, and near a flat group its sign, would depend on the
/// hashing - the kernel's `compute_halfedges` was hardened the same way.
fn six_volume(fm: &Mesh) -> f64 {
    let mut keys: Vec<usize> = fm.face.keys().copied().collect();
    keys.sort_unstable();
    let mut v = 0.0;
    for k in keys {
        let verts = &fm.face[&k];
        if verts.len() < 3 {
            continue;
        }
        let (a, b, c) = (at(fm, verts[0]), at(fm, verts[1]), at(fm, verts[2]));
        v += a[0] * (b[1] * c[2] - b[2] * c[1]) - a[1] * (b[0] * c[2] - b[2] * c[0])
            + a[2] * (b[0] * c[1] - b[1] * c[0]);
    }
    v
}

/// One `+1.0` or `-1.0` per face mesh: multiply the kernel's normal by it to point outward.
/// A breadth-first walk over the faces through their shared edges makes neighbours agree;
/// each connected group is then turned outward by the sign of the volume it encloses. Both
/// steps read the tessellation, never `BRepOrientation`.
pub fn face_signs(b: &BRep, fms: &[Mesh], chains: &[Option<EdgeChain>]) -> Vec<f64> {
    let nf = fms.len();
    // An open shell has no enclosed-volume orientation; retain its authored face uses.
    if !b.is_solid() {
        return vec![1.0; nf];
    }
    let mut adjacent: Vec<Vec<(usize, bool)>> = vec![Vec::new(); nf];
    for c in chains.iter().flatten() {
        if let (Some(other), Some(opp)) = (c.other, opposed(fms, c)) {
            adjacent[c.face].push((other, opp));
            adjacent[other].push((c.face, opp));
        }
    }
    let mut sign = vec![0.0f64; nf];
    for start in 0..nf {
        if sign[start] != 0.0 {
            continue;
        }
        sign[start] = 1.0;
        let mut group = vec![start];
        let mut head = 0;
        while head < group.len() {
            let f = group[head];
            head += 1;
            for &(g, opp) in &adjacent[f] {
                if sign[g] == 0.0 {
                    sign[g] = if opp { sign[f] } else { -sign[f] };
                    group.push(g);
                }
            }
        }
        // Volume as the group's own winding sweeps it: negative means every face is inside out.
        let mut volume = 0.0;
        for &face in &group {
            volume += sign[face] * six_volume(&fms[face]);
        }
        if volume < 0.0 {
            for &f in &group {
                sign[f] = -sign[f];
            }
        }
    }
    sign
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::walk::brep::QUALITY;
    use crate::app::walk::brep_edges::edge_chains;
    use session_rust::brep::brep_reverse;

    /// The signs of `b`'s faces at the viewer's quality.
    fn signs_of(b: &BRep) -> Vec<f64> {
        let fms = b.face_meshes_q(Some(QUALITY));
        let chains = edge_chains(b, &fms);
        face_signs(b, &fms, &chains)
    }

    /// The probe's flip: the first two face uses of the first shell reversed (mk_brep_probe).
    fn flipped(mut b: BRep) -> BRep {
        for face in b.m_shells[0].faces.iter_mut().take(2) {
            face.orientation = brep_reverse(face.orientation);
        }
        b
    }

    /// A solid the kernel builds outward keeps every sign at +1.
    #[test]
    fn kernel_solids_keep_their_normals() {
        for b in [
            BRep::create_cylinder(150.0, 400.0),
            BRep::create_cone(150.0, 400.0),
            BRep::create_box(400.0, 300.0, 250.0),
            BRep::create_block_with_hole(500.0, 300.0, 200.0, 80.0),
        ] {
            assert!(signs_of(&b).iter().all(|&s| s == 1.0), "{}", b.name);
        }
    }

    #[test]
    fn open_shell_preserves_authored_orientation() {
        let mut b = flipped(BRep::create_cylinder(150.0, 400.0));
        b.m_solids.clear();
        assert!(signs_of(&b).iter().all(|&sign| sign == 1.0));
    }

    /// Reversing two of the cylinder's three face uses negates those two meshes' normals in
    /// the kernel; the signs undo exactly that, so the outward normals the pipes read are the
    /// same bits in both files.
    #[test]
    fn flipped_uses_change_no_outward_normal() {
        let ok = BRep::create_cylinder(150.0, 400.0);
        let fl = flipped(BRep::create_cylinder(150.0, 400.0));
        let (fms_ok, fms_fl) = (
            ok.face_meshes_q(Some(QUALITY)),
            fl.face_meshes_q(Some(QUALITY)),
        );
        let signs = face_signs(&fl, &fms_fl, &edge_chains(&fl, &fms_fl));
        assert_eq!(signs[0], -1.0);
        assert_eq!(signs[1], -1.0);
        assert_eq!(signs[2], 1.0);
        for fi in 0..3 {
            for (key, vd) in fms_ok[fi].vertex.iter() {
                let n_ok = vd.normal().unwrap();
                let n_fl = fms_fl[fi].vertex[key].normal().unwrap();
                assert_eq!(
                    [
                        n_fl[0] * signs[fi],
                        n_fl[1] * signs[fi],
                        n_fl[2] * signs[fi]
                    ],
                    n_ok
                );
            }
        }
    }

    /// The pipes of the ok and the flipped cylinder are the same rows: same ends, same
    /// facing words - what the orbit check's mask diff measures at the pixel level.
    #[test]
    fn flipped_uses_change_no_pipe() {
        use crate::app::walk::WalkCx;
        use crate::app::walk::brep::walk_brep;
        use crate::app::walk::mesh_ink::Ink;
        use crate::engine::gpu::arena::ArenaRows;
        use crate::engine::gpu::glyphs::GlyphRows;
        use crate::engine::gpu::segments::SegRows;
        let mut pipes = Vec::new();
        for b in [
            BRep::create_cylinder(150.0, 400.0),
            flipped(BRep::create_cylinder(150.0, 400.0)),
        ] {
            let mut arena = ArenaRows::default();
            let mut seg = SegRows::default();
            let mut glyph = GlyphRows::default();
            let mut ink = Ink {
                seg: &mut seg,
                glyph: &mut glyph,
            };
            walk_brep(
                &mut arena,
                &mut ink,
                &b,
                &WalkCx {
                    vert_base: 0,
                    cloud_px: 0.0,
                    row: 0,
                },
            );
            pipes.push(
                seg.pipes
                    .iter()
                    .map(|p| (p.p0, p.p1, p.facing))
                    .collect::<Vec<_>>(),
            );
        }
        assert_eq!(pipes[0], pipes[1]);
    }
}
