use super::brep_edges::EdgeChain;
use session_rust::{BRep, Mesh};

/// Which way this face walks the edge s -> n: Some(true) forwards, Some(false) backwards, None if unclear.
fn walks(fm: &Mesh, s: usize, n: usize) -> Option<bool> {
    let fwd = occupied_halfedge(fm, s, n);
    let back = occupied_halfedge(fm, n, s);

    match (fwd, back) {
        (true, false) => Some(true),
        (false, true) => Some(false),
        _ => None,
    }
}

/// A halfedge is one direction of an edge; it is occupied when some face walks from -> to.
fn occupied_halfedge(fm: &Mesh, from: usize, to: usize) -> bool {
    let Some(neighbours) = fm.halfedge.get(&from) else {
        return false;
    };
    matches!(neighbours.get(&to), Some(Some(_)))
}

fn at(fm: &Mesh, k: usize) -> [f64; 3] {
    let v = &fm.vertex[&k];
    [v.x, v.y, v.z]
}

/// Each face is meshed on its own, so the other face's copy of an edge is found by direction, not by key.
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

        let c = (d[0] * dir[0] + d[1] * dir[1] + d[2] * dir[2]) / l; // cosine to `dir`

        // a match as the if condition: true when this neighbour beats the best so far
        if match best {
            Some((bc, bw)) => c > bc || (c == bc && w < bw), // a tie goes to the smaller key, whatever the HashMap order
            None => true,
        } {
            best = Some((c, w));
        }
    }

    Some(best?.1)
}

/// Nearest vertex; a tie goes to the smaller key.
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

/// Neighbouring faces agree on which side is outside when they walk their shared edge in opposite directions.
fn opposed(fms: &[Mesh], c: &EdgeChain) -> Option<bool> {
    let other = c.other?;
    let (fa, fb) = (&fms[c.face], &fms[other]);
    let (s, n) = (c.keys[0], c.keys[1]);
    let away_a = walks(fa, s, n)?; // the owner face's way along s -> n
    let (ps, pn) = (at(fa, s), at(fa, n));
    let dir = [pn[0] - ps[0], pn[1] - ps[1], pn[2] - ps[2]];
    let sb = vertex_at(fb, ps)?; // same start on the other face
    let nb = neighbour_along(fb, sb, dir)?; // same next point there
    let away_b = walks(fb, sb, nb)?;
    Some(away_a != away_b)
}

/// a · (b × c) per triangle: six times the signed volume of the cone from the origin to the face.
fn six_volume(fm: &Mesh) -> f64 {
    let mut keys: Vec<usize> = fm.face.keys().copied().collect();
    keys.sort_unstable(); // the same sum, bit for bit, on every run
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

/// +1 or -1 per face so every normal points outward.
pub fn face_signs(b: &BRep, fms: &[Mesh], chains: &[Option<EdgeChain>]) -> Vec<f64> {
    let nf = fms.len();

    // an open shell keeps its authored normals
    if !b.is_solid() {
        return vec![1.0; nf];
    }

    let mut adjacent: Vec<Vec<(usize, bool)>> = vec![Vec::new(); nf]; // per face: (neighbour, opposed)

    for c in chains.iter().flatten() {
        if let (Some(other), Some(opp)) = (c.other, opposed(fms, c)) {
            adjacent[c.face].push((other, opp));
            adjacent[other].push((c.face, opp));
        }
    }

    let mut sign = vec![0.0f64; nf]; // 0 = not visited

    for start in 0..nf {
        if sign[start] != 0.0 {
            continue;
        }

        sign[start] = 1.0;
        let mut group = vec![start]; // faces reached from start; also the queue, read from `head`
        let mut head = 0;

        // breadth first: a neighbour takes our sign if the pair agrees, else the opposite
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

        // negative volume: the whole group is inside out
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

    /// Flip sign of every face of `b`.
    fn signs_of(b: &BRep) -> Vec<f64> {
        let fms = b.face_meshes_q(Some(QUALITY));
        let chains = edge_chains(b, &fms);
        face_signs(b, &fms, &chains)
    }

    /// Reverse the first two face uses.
    fn flipped(mut b: BRep) -> BRep {
        for face in b.m_shells[0].faces.iter_mut().take(2) {
            face.orientation = brep_reverse(face.orientation);
        }

        b
    }

    /// Kernel solids need no face flipped.
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

    /// An open shell has no face flipped.
    #[test]
    fn open_shell_preserves_authored_orientation() {
        let mut b = flipped(BRep::create_cylinder(150.0, 400.0));
        b.m_solids.clear();
        assert!(signs_of(&b).iter().all(|&sign| sign == 1.0));
    }

    /// A flipped face keeps the same outward normal.
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

    /// A flipped cylinder draws the same pipes.
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
                    // --8<-- [start:step-9]
                    attributes: false,
                    // --8<-- [end:step-9]
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
