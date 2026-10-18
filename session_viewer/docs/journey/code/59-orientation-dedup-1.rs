[0], pn[1] - ps[1], pn[2] - ps[2]];
    let sb = vertex_at(fb, ps)?;
    let nb = neighbour_along(fb, sb, dir)?;
    let away_b = walks(fb, sb, nb)?;
    Some(away_a != away_b)
}

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

pub fn face_signs(b: &BRep, fms: &[Mesh], chains: &[Option<EdgeChain>]) -> Vec<f64> {
    let nf = fms.len();

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

