//! Ray-cast picking against cached NURBS/BRep tessellations and curve polylines.

use super::types::*;

impl GpuSession {
    /// Ray-test cached NurbsSurface tessellations (BVH pre-built at load). Returns (guid, hit_point) pairs.
    pub fn pick_nurbssurfaces(
        &mut self,
        origin: &session_rust::Point,
        direction: &session_rust::Vector,
    ) -> Vec<(String, session_rust::Point)> {
        let dir_len = (direction[0]*direction[0] + direction[1]*direction[1] + direction[2]*direction[2]).sqrt();
        if dir_len <= 0.0 { return Vec::new(); }
        let du = session_rust::Vector::new(direction[0]/dir_len, direction[1]/dir_len, direction[2]/dir_len);
        let far = 1e6f32;
        let mut hits: Vec<(String, session_rust::Point, f32)> = Vec::new();
        for (guid, (mesh, xf)) in &mut self.nurbs_pick_meshes {
            // Inverse-transform the ray into the mesh's local frame, test, then map the
            // hit back to world — same matrix-only scheme as `pick_breps`.
            let inv = mat4_inverse_cm(xf);
            let inv_xp = |p: [f32;3]| -> [f32;3] {
                let w = inv[0][0]*p[0]+inv[1][0]*p[1]+inv[2][0]*p[2]+inv[3][0];
                let x = inv[0][1]*p[0]+inv[1][1]*p[1]+inv[2][1]*p[2]+inv[3][1];
                let y = inv[0][2]*p[0]+inv[1][2]*p[1]+inv[2][2]*p[2]+inv[3][2];
                let hw = inv[0][3]*p[0]+inv[1][3]*p[1]+inv[2][3]*p[2]+inv[3][3];
                let s = if hw.abs() > 1e-30 { 1.0 / hw } else { 1.0 };
                [w*s, x*s, y*s]
            };
            let lo = inv_xp([origin[0] as f32, origin[1] as f32, origin[2] as f32]);
            let end_w = [origin[0] as f32+du[0] as f32*far, origin[1] as f32+du[1] as f32*far, origin[2] as f32+du[2] as f32*far];
            let le = inv_xp(end_w);
            let local_ray = session_rust::Line::from_points(
                &session_rust::Point::new(lo[0] as f64, lo[1] as f64, lo[2] as f64),
                &session_rust::Point::new(le[0] as f64, le[1] as f64, le[2] as f64),
            );
            if let Some(lp) = mesh.ray_cast_bvh(&local_ray, 1e-6) {
                let wp = [
                    xf[0][0] as f64*lp[0]+xf[1][0] as f64*lp[1]+xf[2][0] as f64*lp[2]+xf[3][0] as f64,
                    xf[0][1] as f64*lp[0]+xf[1][1] as f64*lp[1]+xf[2][1] as f64*lp[2]+xf[3][1] as f64,
                    xf[0][2] as f64*lp[0]+xf[1][2] as f64*lp[1]+xf[2][2] as f64*lp[2]+xf[3][2] as f64,
                ];
                let p = session_rust::Point::new(wp[0], wp[1], wp[2]);
                let dx = wp[0]-origin[0]; let dy = wp[1]-origin[1]; let dz = wp[2]-origin[2];
                let dist = (dx*dx + dy*dy + dz*dz).sqrt() as f32;
                hits.push((guid.clone(), p, dist));
            }
        }
        hits.sort_by(|a,b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal));
        hits.into_iter().map(|(g,p,_)| (g,p)).collect()
    }

    /// Ray-test pre-tessellated BRep meshes (BVH pre-built at load). Returns (guid, hit_point) pairs.
    pub fn pick_breps(
        &mut self,
        origin: &session_rust::Point,
        direction: &session_rust::Vector,
    ) -> Vec<(String, session_rust::Point)> {
        let dir_len = (direction[0]*direction[0] + direction[1]*direction[1] + direction[2]*direction[2]).sqrt();
        if dir_len <= 0.0 { return Vec::new(); }
        let du = session_rust::Vector::new(direction[0]/dir_len, direction[1]/dir_len, direction[2]/dir_len);
        let far = 1e6f32;
        let mut hits: Vec<(String, session_rust::Point, f32)> = Vec::new();
        for (guid, (mesh, xf)) in &mut self.brep_pick_meshes {
            // Full 4×4 matrix inverse — correct for rotation, translation, AND scale.
            let inv = mat4_inverse_cm(xf);
            let inv_xp = |p: [f32;3]| -> [f32;3] {
                let w = inv[0][0]*p[0]+inv[1][0]*p[1]+inv[2][0]*p[2]+inv[3][0];
                let x = inv[0][1]*p[0]+inv[1][1]*p[1]+inv[2][1]*p[2]+inv[3][1];
                let y = inv[0][2]*p[0]+inv[1][2]*p[1]+inv[2][2]*p[2]+inv[3][2];
                let hw = inv[0][3]*p[0]+inv[1][3]*p[1]+inv[2][3]*p[2]+inv[3][3];
                let s = if hw.abs() > 1e-30 { 1.0 / hw } else { 1.0 };
                [w*s, x*s, y*s]
            };
            let lo = inv_xp([origin[0] as f32, origin[1] as f32, origin[2] as f32]);
            let end_w = [origin[0] as f32+du[0] as f32*far, origin[1] as f32+du[1] as f32*far, origin[2] as f32+du[2] as f32*far];
            let le = inv_xp(end_w);
            let local_ray = session_rust::Line::from_points(
                &session_rust::Point::new(lo[0] as f64, lo[1] as f64, lo[2] as f64),
                &session_rust::Point::new(le[0] as f64, le[1] as f64, le[2] as f64),
            );
            if let Some(lp) = mesh.ray_cast_bvh(&local_ray, 1e-6) {
                // Transform hit back to world space
                let wp = [
                    xf[0][0] as f64*lp[0]+xf[1][0] as f64*lp[1]+xf[2][0] as f64*lp[2]+xf[3][0] as f64,
                    xf[0][1] as f64*lp[0]+xf[1][1] as f64*lp[1]+xf[2][1] as f64*lp[2]+xf[3][1] as f64,
                    xf[0][2] as f64*lp[0]+xf[1][2] as f64*lp[1]+xf[2][2] as f64*lp[2]+xf[3][2] as f64,
                ];
                let p = session_rust::Point::new(wp[0], wp[1], wp[2]);
                let dx = wp[0]-origin[0]; let dy = wp[1]-origin[1]; let dz = wp[2]-origin[2];
                let dist = (dx*dx + dy*dy + dz*dz).sqrt() as f32;
                hits.push((guid.clone(), p, dist));
            }
        }
        hits.sort_by(|a,b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal));
        hits.into_iter().map(|(g,p,_)| (g,p)).collect()
    }

    /// Pick a trimmed surface by clicking near one of its rendered edge curves (cut boundary or
    /// seam). Returns (guid, hit_point) for the nearest edge segment within `radius` (world units)
    /// of the ray, in front of the origin. Lets the user grab the object by its curves, not only
    /// the body — like clicking an edge in a CAD modeller.
    pub fn pick_trimmed_edges(
        &self,
        origin: &session_rust::Point,
        direction: &session_rust::Vector,
        radius: f32,
    ) -> Vec<(String, session_rust::Point)> {
        let dl = (direction[0]*direction[0]+direction[1]*direction[1]+direction[2]*direction[2]).sqrt();
        if dl <= 0.0 { return Vec::new(); }
        let o = [origin[0] as f32, origin[1] as f32, origin[2] as f32];
        let d = [direction[0] as f32/dl as f32, direction[1] as f32/dl as f32, direction[2] as f32/dl as f32];
        let r2 = radius*radius;
        let mut best: Option<(String, [f32;3], f32)> = None;
        // perpendicular distance of point p to the ray + along-ray distance t
        let perp = |p: [f32;3]| -> (f32, f32) {
            let v = [p[0]-o[0], p[1]-o[1], p[2]-o[2]];
            let t = v[0]*d[0]+v[1]*d[1]+v[2]*d[2];
            let c = [o[0]+t*d[0], o[1]+t*d[1], o[2]+t*d[2]];
            let dd = (p[0]-c[0]).powi(2)+(p[1]-c[1]).powi(2)+(p[2]-c[2]).powi(2);
            (dd, t)
        };
        for (guid, range) in &self.guid_to_seg {
            if !self.nurbs_trimmeds.contains_key(guid) { continue; }
            for seg in &self.segments_cpu[range.clone()] {
                // sample the segment (endpoints + midpoint) against the ray
                for s in 0..=4 {
                    let f = s as f32/4.0;
                    let p = [seg.p0[0]+(seg.p1[0]-seg.p0[0])*f, seg.p0[1]+(seg.p1[1]-seg.p0[1])*f, seg.p0[2]+(seg.p1[2]-seg.p0[2])*f];
                    let (dd, t) = perp(p);
                    if t > 0.0 && dd <= r2 {
                        if best.as_ref().map_or(true, |b| t < b.2) { best = Some((guid.clone(), p, t)); }
                    }
                }
            }
        }
        best.map(|(g,p,_)| vec![(g, session_rust::Point::new(p[0] as f64, p[1] as f64, p[2] as f64))]).unwrap_or_default()
    }

    /// Ray-test cached NurbsSurfaceTrimmed tessellations (BVH pre-built). Same matrix-only scheme
    /// as pick_breps. Returns (guid, hit_point) pairs.
    pub fn pick_nurbssurfacetrimmeds(
        &mut self,
        origin: &session_rust::Point,
        direction: &session_rust::Vector,
    ) -> Vec<(String, session_rust::Point)> {
        let dir_len = (direction[0]*direction[0] + direction[1]*direction[1] + direction[2]*direction[2]).sqrt();
        if dir_len <= 0.0 { return Vec::new(); }
        let du = session_rust::Vector::new(direction[0]/dir_len, direction[1]/dir_len, direction[2]/dir_len);
        let far = 1e6f32;
        let mut hits: Vec<(String, session_rust::Point, f32)> = Vec::new();
        for (guid, (mesh, xf)) in &mut self.nurbs_trimmed_pick_meshes {
            let inv = mat4_inverse_cm(xf);
            let inv_xp = |p: [f32;3]| -> [f32;3] {
                let w = inv[0][0]*p[0]+inv[1][0]*p[1]+inv[2][0]*p[2]+inv[3][0];
                let x = inv[0][1]*p[0]+inv[1][1]*p[1]+inv[2][1]*p[2]+inv[3][1];
                let y = inv[0][2]*p[0]+inv[1][2]*p[1]+inv[2][2]*p[2]+inv[3][2];
                let hw = inv[0][3]*p[0]+inv[1][3]*p[1]+inv[2][3]*p[2]+inv[3][3];
                let s = if hw.abs() > 1e-30 { 1.0 / hw } else { 1.0 };
                [w*s, x*s, y*s]
            };
            let lo = inv_xp([origin[0] as f32, origin[1] as f32, origin[2] as f32]);
            let end_w = [origin[0] as f32+du[0] as f32*far, origin[1] as f32+du[1] as f32*far, origin[2] as f32+du[2] as f32*far];
            let le = inv_xp(end_w);
            let local_ray = session_rust::Line::from_points(
                &session_rust::Point::new(lo[0] as f64, lo[1] as f64, lo[2] as f64),
                &session_rust::Point::new(le[0] as f64, le[1] as f64, le[2] as f64),
            );
            if let Some(lp) = mesh.ray_cast_bvh(&local_ray, 1e-6) {
                let wp = [
                    xf[0][0] as f64*lp[0]+xf[1][0] as f64*lp[1]+xf[2][0] as f64*lp[2]+xf[3][0] as f64,
                    xf[0][1] as f64*lp[0]+xf[1][1] as f64*lp[1]+xf[2][1] as f64*lp[2]+xf[3][1] as f64,
                    xf[0][2] as f64*lp[0]+xf[1][2] as f64*lp[1]+xf[2][2] as f64*lp[2]+xf[3][2] as f64,
                ];
                let p = session_rust::Point::new(wp[0], wp[1], wp[2]);
                let dx = wp[0]-origin[0]; let dy = wp[1]-origin[1]; let dz = wp[2]-origin[2];
                let dist = (dx*dx + dy*dy + dz*dz).sqrt() as f32;
                hits.push((guid.clone(), p, dist));
            }
        }
        hits.sort_by(|a,b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal));
        hits.into_iter().map(|(g,p,_)| (g,p)).collect()
    }

    /// Ray-test cached NurbsCurve polylines. Returns (guid, closest_point) pairs within pick_radius.
    pub fn pick_nurbscurves(
        &self,
        origin: &session_rust::Point,
        direction: &session_rust::Vector,
        pick_radius: f32,
    ) -> Vec<(String, session_rust::Point)> {
        let ox = origin[0] as f32; let oy = origin[1] as f32; let oz = origin[2] as f32;
        let dx = direction[0] as f32; let dy = direction[1] as f32; let dz = direction[2] as f32;
        let mut hits: Vec<(String, session_rust::Point, f32)> = Vec::new();
        for (guid, pts) in &self.nc_pick_pts {
            let mut best_t = f32::MAX;
            for seg in pts.windows(2) {
                let [ax, ay, az] = seg[0];
                let [bx, by, bz] = seg[1];
                let wx = ax - ox; let wy = ay - oy; let wz = az - oz;
                let abx = bx - ax; let aby = by - ay; let abz = bz - az;
                let ab2 = abx*abx + aby*aby + abz*abz;
                if ab2 < 1e-10 { continue; }
                let d_dot_ab = dx*abx + dy*aby + dz*abz;
                let w_dot_ab = wx*abx + wy*aby + wz*abz;
                let w_dot_d  = wx*dx + wy*dy + wz*dz;
                let d2 = dx*dx + dy*dy + dz*dz;
                let denom = d2*ab2 - d_dot_ab*d_dot_ab;
                // Robust constrained closest point of ray P(s)=o+s·d (s≥0) vs segment
                // Q(t)=a+t·ab (t∈[0,1]), with w = a−o. Solve, clamp s to the ray, clamp t
                // to the segment, then re-solve s for a clamped t. (The previous closed
                // form had both s and t signs inverted, so the distance was wrong.)
                let mut s = if denom.abs() > 1e-10 {
                    (w_dot_d*ab2 - d_dot_ab*w_dot_ab) / denom
                } else { 0.0 };
                s = s.max(0.0);
                let mut t = (s*d_dot_ab - w_dot_ab) / ab2;
                if t < 0.0 {
                    t = 0.0;
                    s = (w_dot_d / d2).max(0.0);
                } else if t > 1.0 {
                    t = 1.0;
                    s = ((w_dot_d + d_dot_ab) / d2).max(0.0);
                }
                let px = ox + dx*s - (ax + abx*t);
                let py = oy + dy*s - (ay + aby*t);
                let pz = oz + dz*s - (az + abz*t);
                let dist = (px*px + py*py + pz*pz).sqrt();
                if dist < pick_radius && s < best_t {
                    best_t = s;
                    let cx = ax + abx*t; let cy2 = ay + aby*t; let cz2 = az + abz*t;
                    hits.retain(|(g, _, _)| g != guid);
                    hits.push((guid.clone(), session_rust::Point::new(cx as f64, cy2 as f64, cz2 as f64), s));
                }
            }
        }
        hits.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal));
        hits.into_iter().map(|(g, p, _)| (g, p)).collect()
    }
}
