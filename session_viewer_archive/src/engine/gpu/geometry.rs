//! CPU geometry -> GPU buffers: add_geometry and the NURBS curve/surface builders.

use crate::gpu_adapters::{
    color_to_rgba_f32, line_endpoint_glyphs, line_to_segment,
    mesh_crease_edges_to_segments, mesh_edges_to_segments, mesh_naked_edges_to_segments,
    mesh_to_vertices, mesh_vertex_glyphs,
    named_point_to_cross_vertices, obb_to_line_vertices, plane_to_axis_segments, plane_origin_glyph,
    point_to_vertex, pointcloud_to_cloud_points, pts_to_segments,
    polyline_endpoint_glyphs, polyline_to_segments,
};
use super::types::*;

/// DEFAULT viewer tessellation quality for NURBS surfaces and BRep faces (max normal-deviation
/// angle in degrees, chord-height factor). These seed the runtime `GpuSession.tess_angle_deg` /
/// `tess_chord_factor`, which the UI slider tunes. Coarser than before (was 2.5°/0.0006 ≈ 16–23×
/// denser, which made every add/edit/commit slow); 10°/0.003 stays smooth-looking at a fraction
/// of the triangles. Adaptive: flat spans stay ≈2 triangles; only curvature drives subdivision.
pub(crate) const TESS_MAX_ANGLE_DEG: f32 = 10.0;
pub(crate) const TESS_CHORD_FACTOR: f32 = 0.003;

impl GpuSession {
    /// Upload a triangle object into the tri arena for the batched (single-draw) mesh path.
    /// Stamps the per-vertex instance_id, records the local AABB for culling, and stores the
    /// object's global (vbo-absolute) indices so `flush_geometry` can concatenate them.
    fn add_tri_object(&mut self, guid: &str, mut vs: Vec<MeshVertex>, is: Vec<u32>,
                      instance_id: u32, device: &wgpu::Device, queue: &wgpu::Queue) {
        self.object_aabb_local.insert(instance_id, aabb_of_mesh_verts(&vs));
        for v in &mut vs { v.instance_id = instance_id; }
        let slot = self.tri.allocate(guid, &vs, Some(&is), instance_id, device, queue);
        let base = slot.vertex_range.start;
        self.tri_index_cpu.insert(instance_id, is.iter().map(|i| i + base).collect());
        self.mesh_draw_dirty = true;
    }

    pub fn add_nurbscurve(&mut self, curve: &session_rust::NurbsCurve, device: &wgpu::Device, queue: &wgpu::Queue) {
        if !curve.is_valid() { return; }
        let guid = curve.guid().to_string();
        // Match the NurbsSurface iso-curve / BRep edge density (×0.1) so standalone
        // curves are just as smooth, not 24-gon polygonal.
        let (pts, _) = curve.to_polyline_adaptive(session_rust::Tolerance::ANGULARDEFLECTION * 0.02, 0.0, 0.0);
        if pts.len() < 2 { return; }
        let instance_id = self.pick.allocate(&guid);
        let segs = pts_to_segments(&pts, instance_id);
        let seg_start = self.segments_cpu.len();
        self.segments_cpu.extend(segs);
        self.guid_to_seg.insert(guid.clone(), seg_start..self.segments_cpu.len());
        self.segments_dirty = true;
        self.nc_pick_pts.insert(guid.clone(), pts.iter().map(|p| [p[0] as f32, p[1] as f32, p[2] as f32]).collect());
        let gly_start = self.glyphs_cpu.len();
        for p in [pts.first().unwrap(), pts.last().unwrap()] {
            let center = [p[0] as f32, p[1] as f32, p[2] as f32];
            self.glyphs_cpu.push(GlyphPoint { center, radius: 0.0,
                color: [1.0; 4], instance_id, _pad: [0; 3] });
        }
        self.guid_to_glyph.insert(guid.clone(), gly_start..self.glyphs_cpu.len());
        self.glyphs_dirty = true;
        let color = color_to_rgba_f32(curve.linecolors.get(0).unwrap_or(&session_rust::Color::white()));
        self.write_instance(instance_id, color, device, queue);
    }

    pub fn add_nurbssurface(&mut self, surface: &session_rust::NurbsSurface, device: &wgpu::Device, queue: &wgpu::Queue) {
        let guid = surface.guid().to_string();
        // Adaptive curvature-driven density (same controls used for BRep faces below), so
        // a standalone NurbsSurface and the equivalent BRep face tessellate identically.
        let mut mesh = session_rust::remesh_nurbssurface_grid::RemeshNurbsSurfaceGrid::from_u_v_q(
            surface.clone(), 0, 0, self.tess_angle_deg as f64, self.tess_chord_factor as f64);
        let (vs, is) = mesh_to_vertices(&mesh);
        if vs.is_empty() { return; }
        // Degenerate guard: a surface scaled toward zero collapses its control points to
        // ~one point (or underflows to NaN). Building the triangle BVH / morton codes on a
        // zero-extent or non-finite mesh is fragile, so drop it rather than crash — the
        // object just isn't drawn while it has no extent.
        {
            let (mut mn, mut mx) = ([f32::INFINITY; 3], [f32::NEG_INFINITY; 3]);
            let mut finite = true;
            for v in &vs {
                for k in 0..3 {
                    let c = v.position[k];
                    if !c.is_finite() { finite = false; }
                    if c < mn[k] { mn[k] = c; }
                    if c > mx[k] { mx[k] = c; }
                }
            }
            let extent = (mx[0]-mn[0]).max(mx[1]-mn[1]).max(mx[2]-mn[2]);
            if !finite || !extent.is_finite() || extent < 1e-4 {
                self.remove(&guid);
                return;
            }
        }
        // Idempotent: see add_geometry — prevents orphaned edge segments on re-add.
        self.remove(&guid);
        let instance_id = self.pick.allocate(&guid);
        self.add_tri_object(&guid, vs, is, instance_id, device, queue);
        // Boundary edges: 4 iso-curves at the domain edges (these include the seam line on a
        // closed surface — mesh naked edges miss the seam since the grid wraps there).
        // iso_curve(0,v) = u-direction curve at fixed v → use v-domain (dir=1) for parameter.
        // iso_curve(1,u) = v-direction curve at fixed u → use u-domain (dir=0) for parameter.
        let mut edge_segs: Vec<CylinderSegment> = Vec::new();
        let domains = [surface.domain(0), surface.domain(1)];
        for (iso_dir, param_dir, t_is_max) in [(0usize, 1usize, false), (0, 1, true), (1, 0, false), (1, 0, true)] {
            let t = if let Some((t0, t1)) = domains[param_dir] {
                if t_is_max { t1 } else { t0 }
            } else { continue };
            if let Some(crv) = surface.iso_curve(iso_dir, t) {
                let (pts, _) = crv.to_polyline_adaptive(
                    session_rust::Tolerance::ANGULARDEFLECTION * 0.02, 0.0, 0.0);
                edge_segs.extend(pts_to_segments(&pts, instance_id));
            }
        }
        if !edge_segs.is_empty() {
            if let Some(lc) = surface.linecolors.get(0) {
                let c = color_to_rgba_f32(lc);
                for seg in &mut edge_segs { seg.color = c; }
            }
            let seg_start = self.segments_cpu.len();
            self.segments_cpu.extend(edge_segs);
            self.guid_to_seg.insert(guid.clone(), seg_start..self.segments_cpu.len());
            self.segments_dirty = true;
        }
        mesh.ensure_triangle_bvh();
        // Tessellation is treated as the local pick mesh; the pose lives in the instance
        // model (identity at add). A later transform updates the stored xf only.
        self.nurbs_pick_meshes.insert(guid.clone(), (mesh, identity_matrix()));
        self.nurbs_surfaces.insert(guid.clone(), surface.clone());
        let surf_color = color_to_rgba_f32(surface.facecolors.get(0).unwrap_or(&session_rust::Color::white()));
        self.write_instance_flags(instance_id, surf_color, InstanceData::FLAG_SMOOTH, device, queue);
    }

    /// First-class NurbsSurfaceTrimmed: tessellate (mesh_render → plane-cut or CDT), smooth-shade,
    /// draw the trim boundary as selectable cylinder segments (the mesh naked edges = the cut
    /// curve / hole rims), and store a local pick mesh so a transform is matrix-only (BRep-style).
    /// This is why it is NOT baked into a generic Mesh: the boundary stays a selectable curve and
    /// the object moves without re-tessellation.
    pub fn add_nurbssurfacetrimmed(&mut self, ts: &session_rust::NurbsSurfaceTrimmed, device: &wgpu::Device, queue: &wgpu::Queue) {
        let guid = ts.guid().to_string();
        let mut mesh = ts.mesh_render(self.tess_angle_deg as f64, self.tess_chord_factor as f64);
        let (vs, is) = mesh_to_vertices(&mesh);
        if vs.is_empty() { self.remove(&guid); return; }
        self.remove(&guid);
        let instance_id = self.pick.allocate(&guid);
        self.add_tri_object(&guid, vs, is, instance_id, device, queue);
        // Edges = (a) the new trim/cut curve (mesh naked edges) PLUS (b) the underlying surface's
        // natural seam/boundary iso-curves, clipped to the kept side of the cut. Like a real CAD
        // trimmed face: the original surface seams remain and are cut by the trim, alongside the
        // new outline. (mesh_naked_edges_to_segments emits alpha=0 — force opaque black.)
        let mut edge_segs = mesh_naked_edges_to_segments(&mesh, instance_id);
        for seg in &mut edge_segs { seg.color = [0.0, 0.0, 0.0, 1.0]; }
        // (b) seam iso-curves at the surface domain extremes, clipped by the cut plane (keep the
        // sub-segments where (p-q0).n <= 0). Drawn finely like NurbsSurface iso-curves so the seam
        // reads as a smooth curve, not facets.
        if let (Some(q0), Some(nrm)) = (ts.cut_q0.as_ref(), ts.cut_n.as_ref()) {
            let (qx, qy, qz) = (q0[0] as f32, q0[1] as f32, q0[2] as f32);
            let nl = ((nrm[0]*nrm[0]+nrm[1]*nrm[1]+nrm[2]*nrm[2]) as f32).sqrt().max(1e-12);
            let (nx, ny, nz) = (nrm[0] as f32/nl, nrm[1] as f32/nl, nrm[2] as f32/nl);
            let f3 = |p: &session_rust::Point| (p[0] as f32-qx)*nx + (p[1] as f32-qy)*ny + (p[2] as f32-qz)*nz;
            let surface = ts.surface();
            let domains = [surface.domain(0), surface.domain(1)];
            for (iso_dir, param_dir, t_is_max) in [(0usize,1usize,false),(0,1,true),(1,0,false),(1,0,true)] {
                let t = match domains[param_dir] { Some((t0,t1)) => if t_is_max { t1 } else { t0 }, None => continue };
                if let Some(crv) = surface.iso_curve(iso_dir, t) {
                    let (pts, _) = crv.to_polyline_adaptive(session_rust::Tolerance::ANGULARDEFLECTION * 0.02, 0.0, 0.0);
                    for w in pts.windows(2) {
                        if f3(&w[0]) <= 1e-3 && f3(&w[1]) <= 1e-3 {
                            edge_segs.push(CylinderSegment {
                                p0: [w[0][0] as f32, w[0][1] as f32, w[0][2] as f32], radius: 0.0,
                                p1: [w[1][0] as f32, w[1][1] as f32, w[1][2] as f32], instance_id, color: [0.0, 0.0, 0.0, 1.0],
                            });
                        }
                    }
                }
            }
        }
        if !edge_segs.is_empty() {
            let seg_start = self.segments_cpu.len();
            self.segments_cpu.extend(edge_segs);
            self.guid_to_seg.insert(guid.clone(), seg_start..self.segments_cpu.len());
            self.segments_dirty = true;
        }
        mesh.ensure_triangle_bvh();
        self.nurbs_trimmed_pick_meshes.insert(guid.clone(), (mesh, identity_matrix()));
        self.nurbs_trimmeds.insert(guid.clone(), ts.clone());
        let surf_color = color_to_rgba_f32(&ts.surfacecolor);
        self.write_instance_flags(instance_id, surf_color, InstanceData::FLAG_SMOOTH, device, queue);
    }

    pub fn add_geometry(&mut self, guid: &str, geom: &session_rust::session::Geometry, device: &wgpu::Device, queue: &wgpu::Queue) {
        use session_rust::session::Geometry;
        // Idempotent: drop any stale buffers for this guid first. Segment/glyph/cloud
        // data is drawn whole-buffer (not per-guid), so a re-add without this would
        // orphan the previous entries and render them forever as a ghost duplicate.
        self.remove(guid);
        let instance_id = self.pick.allocate(guid);

        match geom {
            Geometry::Point(p) => {
                if !p.name.is_empty() && p.name != "my_point" {
                    // Named point → 3-axis crosshair in line arena + text label.
                    let (vs, is) = named_point_to_cross_vertices(p);
                    self.line.allocate(guid, &vs, Some(&is), instance_id, device, queue);
                    self.write_instance(instance_id, color_to_rgba_f32(&p.pointcolor), device, queue);
                } else {
                    let vs = point_to_vertex(p);
                    self.point.allocate(guid, &vs, None, instance_id, device, queue);
                    self.write_instance(instance_id, color_to_rgba_f32(&p.pointcolor), device, queue);
                }
            }
            Geometry::Line(l) => {
                let seg = line_to_segment(l, instance_id);
                let seg_start = self.segments_cpu.len();
                self.segments_cpu.push(seg);
                self.guid_to_seg.insert(guid.to_string(), seg_start..self.segments_cpu.len());
                self.segments_dirty = true;
                let gly_start = self.glyphs_cpu.len();
                self.glyphs_cpu.extend(line_endpoint_glyphs(l, instance_id));
                self.guid_to_glyph.insert(guid.to_string(), gly_start..self.glyphs_cpu.len());
                self.glyphs_dirty = true;
                self.write_instance(instance_id, color_to_rgba_f32(&l.linecolor), device, queue);
            }
            Geometry::Polyline(pl) => {
                let segs = polyline_to_segments(pl, instance_id);
                if segs.is_empty() { self.pick.release(guid); return; }
                let seg_start = self.segments_cpu.len();
                self.segments_cpu.extend(segs);
                self.guid_to_seg.insert(guid.to_string(), seg_start..self.segments_cpu.len());
                self.segments_dirty = true;
                let gly_start = self.glyphs_cpu.len();
                self.glyphs_cpu.extend(polyline_endpoint_glyphs(pl, instance_id));
                self.guid_to_glyph.insert(guid.to_string(), gly_start..self.glyphs_cpu.len());
                self.glyphs_dirty = true;
                self.write_instance(instance_id, color_to_rgba_f32(&pl.linecolor), device, queue);
            }
            Geometry::PointCloud(pc) => {
                let pts = pointcloud_to_cloud_points(pc, instance_id);
                if pts.is_empty() { self.pick.release(guid); return; }
                let start = self.clouds_cpu.len();
                self.clouds_cpu.extend(pts);
                self.guid_to_cloud.insert(guid.to_string(), start..self.clouds_cpu.len());
                self.clouds_dirty = true;
                self.write_instance(instance_id, [1.0, 1.0, 1.0, 1.0], device, queue);
            }
            Geometry::Mesh(m) => {
                let (vs, is) = mesh_to_vertices(m);
                self.add_tri_object(guid, vs, is, instance_id, device, queue);
                let segs = if m.crease_angle_deg > 0.0 {
                    mesh_crease_edges_to_segments(m, instance_id, m.crease_angle_deg as f32)
                } else {
                    mesh_edges_to_segments(m, instance_id)
                };
                if !segs.is_empty() {
                    let seg_start = self.segments_cpu.len();
                    self.segments_cpu.extend(segs);
                    self.guid_to_seg.insert(guid.to_string(), seg_start..self.segments_cpu.len());
                    self.segments_dirty = true;
                }
                let gly_start = self.glyphs_cpu.len();
                self.glyphs_cpu.extend(mesh_vertex_glyphs(m, instance_id));
                if self.glyphs_cpu.len() > gly_start {
                    self.guid_to_glyph.insert(guid.to_string(), gly_start..self.glyphs_cpu.len());
                    self.glyphs_dirty = true;
                }
                let edge_flag = if m.crease_angle_deg > 0.0 { 0 } else { InstanceData::FLAG_EDGES_HIDDEN };
                // Meshes carrying ANALYTIC vertex normals (nx/ny/nz) — e.g. trimmed/cut
                // NURBS surfaces — shade smoothly from those normals. Without this they
                // render faceted, so the CDT triangulation of a cut curved surface shows
                // through as the "strange tessellation". Plain meshes (no stored normals,
                // e.g. boxes) keep hard faceted shading.
                let smooth_flag = if m.vertex.values().any(|vd| vd.attributes.contains_key("nx")) {
                    InstanceData::FLAG_SMOOTH
                } else { 0 };
                self.write_instance_flags(instance_id, color_to_rgba_f32(m.objectcolor()), edge_flag | smooth_flag | InstanceData::FLAG_GLYPHS_HIDDEN, device, queue);
            }
            Geometry::Plane(pl) => {
                let segs = plane_to_axis_segments(pl, self.plane_scale, instance_id);
                let seg_start = self.segments_cpu.len();
                self.segments_cpu.extend(segs);
                self.guid_to_seg.insert(guid.to_string(), seg_start..self.segments_cpu.len());
                self.segments_dirty = true;
                let glyph = plane_origin_glyph(pl, instance_id);
                let gly_start = self.glyphs_cpu.len();
                self.glyphs_cpu.push(glyph);
                self.guid_to_glyph.insert(guid.to_string(), gly_start..self.glyphs_cpu.len());
                self.glyphs_dirty = true;
                self.write_instance(instance_id, [1.0, 1.0, 1.0, 1.0], device, queue);
            }
            Geometry::OBB(bb) => {
                let (vs, is) = obb_to_line_vertices(bb);
                self.line.allocate(guid, &vs, Some(&is), instance_id, device, queue);
                self.write_instance(instance_id, [1.0, 1.0, 1.0, 1.0], device, queue);
            }
            Geometry::BRep(b) => {
                // Tessellate each BRep face independently at the viewer tessellation quality
                // (trimmed where needed, reversed normals applied, seams coordinated) so the
                // faces are dense enough to hug the boundary curves.
                let face_ms = b.face_meshes_q(Some((self.tess_angle_deg as f64, self.tess_chord_factor as f64)));
                let mut vs: Vec<MeshVertex> = Vec::new();
                let mut is: Vec<u32> = Vec::new();
                for fm in &face_ms {
                    let (fvs, fis) = mesh_to_vertices(fm);
                    if fvs.is_empty() { continue; }
                    let offset = vs.len() as u32;
                    vs.extend(fvs);
                    is.extend(fis.iter().map(|&i| i + offset));
                }
                let mut bm = b.mesh();
                let had_tris = !vs.is_empty();
                if had_tris {
                    self.add_tri_object(guid, vs, is, instance_id, device, queue);
                }
                let xf_f64 = b.xform.to_cols();
                let xf = [
                    [xf_f64[0][0] as f32, xf_f64[0][1] as f32, xf_f64[0][2] as f32, xf_f64[0][3] as f32],
                    [xf_f64[1][0] as f32, xf_f64[1][1] as f32, xf_f64[1][2] as f32, xf_f64[1][3] as f32],
                    [xf_f64[2][0] as f32, xf_f64[2][1] as f32, xf_f64[2][2] as f32, xf_f64[2][3] as f32],
                    [xf_f64[3][0] as f32, xf_f64[3][1] as f32, xf_f64[3][2] as f32, xf_f64[3][3] as f32],
                ];
                // Edges: the smooth NURBS edge curves (like the NurbsSurface iso-curves),
                // sampled finely and independent of the mesh — so they stay smooth even
                // where the facets are coarse (Rhino-style). Local space; inst.model in shader.
                let mut edge_segs: Vec<CylinderSegment> = Vec::new();
                if !b.m_curves_3d.is_empty() {
                    for curve in &b.m_curves_3d {
                        if !curve.is_valid() { continue; }
                        let (pts, _) = curve.to_polyline_adaptive(
                            session_rust::Tolerance::ANGULARDEFLECTION * 0.02, 0.0, 0.0);
                        edge_segs.extend(pts_to_segments(&pts, instance_id));
                    }
                } else {
                    edge_segs.extend(mesh_naked_edges_to_segments(&bm, instance_id));
                }
                if edge_segs.is_empty() && !had_tris { self.pick.release(guid); return; }
                if !edge_segs.is_empty() {
                    let seg_start = self.segments_cpu.len();
                    self.segments_cpu.extend(edge_segs);
                    self.guid_to_seg.insert(guid.to_string(), seg_start..self.segments_cpu.len());
                    self.segments_dirty = true;
                }
                let surf_color = color_to_rgba_f32(&b.surfacecolor);
                self.write_instance_model_flags(instance_id, surf_color, InstanceData::FLAG_SMOOTH, xf, device, queue);
                bm.ensure_triangle_bvh();
                self.brep_pick_meshes.insert(guid.to_string(), (bm, xf));
            }
            Geometry::Element(e) => {
                use session_rust::element::ElementGeometry;
                match e.geometry() {
                    ElementGeometry::Mesh(m) => {
                        let (vs, is) = mesh_to_vertices(m);
                        if !vs.is_empty() {
                            let c = color_to_rgba_f32(m.objectcolor());
                            self.add_tri_object(guid, vs, is, instance_id, device, queue);
                            self.write_instance(instance_id, c, device, queue);
                        } else {
                            self.pick.release(guid);
                        }
                    }
                    ElementGeometry::BRep(b) => {
                        let m = b.mesh();
                        let (vs, is) = mesh_to_vertices(&m);
                        if !vs.is_empty() {
                            let c = color_to_rgba_f32(&b.surfacecolor);
                            self.add_tri_object(guid, vs, is, instance_id, device, queue);
                            self.write_instance(instance_id, c, device, queue);
                        } else {
                            self.pick.release(guid);
                        }
                    }
                    ElementGeometry::None => {
                        self.pick.release(guid);
                    }
                }
            }
        }
        // Record default tints for reset_color. Only set if guid survived (not early-released).
        if let Some(id) = self.pick.instance_id(guid) {
            let idx = id as usize;
            if idx < self.instances_cpu.len() {
                self.default_tints.insert(guid.to_string(), self.instances_cpu[idx].color);
                self.default_face_tints.insert(guid.to_string(), self.instances_cpu[idx].face_color);
            }
        }
    }
}
