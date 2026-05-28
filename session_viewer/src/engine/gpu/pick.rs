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
        let end = session_rust::Point::new(origin[0]+du[0]*far, origin[1]+du[1]*far, origin[2]+du[2]*far);
        let ray = session_rust::Line::from_points(origin, &end);
        let mut hits: Vec<(String, session_rust::Point, f32)> = Vec::new();
        for (guid, mesh) in &mut self.nurbs_pick_meshes {
            if let Some(p) = mesh.ray_cast_bvh(&ray, 1e-6) {
                let dx = p[0]-origin[0]; let dy = p[1]-origin[1]; let dz = p[2]-origin[2];
                let dist = (dx*dx + dy*dy + dz*dz).sqrt();
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
            let lo = inv_xp([origin[0], origin[1], origin[2]]);
            let end_w = [origin[0]+du[0]*far, origin[1]+du[1]*far, origin[2]+du[2]*far];
            let le = inv_xp(end_w);
            let local_ray = session_rust::Line::from_points(
                &session_rust::Point::new(lo[0], lo[1], lo[2]),
                &session_rust::Point::new(le[0], le[1], le[2]),
            );
            if let Some(lp) = mesh.ray_cast_bvh(&local_ray, 1e-6) {
                // Transform hit back to world space
                let wp = [
                    xf[0][0]*lp[0]+xf[1][0]*lp[1]+xf[2][0]*lp[2]+xf[3][0],
                    xf[0][1]*lp[0]+xf[1][1]*lp[1]+xf[2][1]*lp[2]+xf[3][1],
                    xf[0][2]*lp[0]+xf[1][2]*lp[1]+xf[2][2]*lp[2]+xf[3][2],
                ];
                let p = session_rust::Point::new(wp[0], wp[1], wp[2]);
                let dx = wp[0]-origin[0]; let dy = wp[1]-origin[1]; let dz = wp[2]-origin[2];
                let dist = (dx*dx + dy*dy + dz*dz).sqrt();
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
        let ox = origin[0]; let oy = origin[1]; let oz = origin[2];
        let dx = direction[0]; let dy = direction[1]; let dz = direction[2];
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
                let (s, t) = if denom.abs() < 1e-10 {
                    (0.0f32, w_dot_ab / ab2)
                } else {
                    let s = (d_dot_ab*w_dot_ab - ab2*w_dot_d) / denom;
                    let t = (d_dot_ab*s - w_dot_ab) / (-ab2);
                    (s.max(0.0), t.clamp(0.0, 1.0))
                };
                let px = ox + dx*s - (ax + abx*t);
                let py = oy + dy*s - (ay + aby*t);
                let pz = oz + dz*s - (az + abz*t);
                let dist = (px*px + py*py + pz*pz).sqrt();
                if dist < pick_radius && s < best_t {
                    best_t = s;
                    let cx = ax + abx*t; let cy2 = ay + aby*t; let cz2 = az + abz*t;
                    hits.retain(|(g, _, _)| g != guid);
                    hits.push((guid.clone(), session_rust::Point::new(cx, cy2, cz2), s));
                }
            }
        }
        hits.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal));
        hits.into_iter().map(|(g, p, _)| (g, p)).collect()
    }
}
