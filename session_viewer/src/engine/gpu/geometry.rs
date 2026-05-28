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

impl GpuSession {
    pub fn add_nurbscurve(&mut self, curve: &session_rust::NurbsCurve, device: &wgpu::Device, queue: &wgpu::Queue) {
        if !curve.is_valid() { return; }
        let guid = curve.guid().to_string();
        let (pts, _) = curve.to_polyline_adaptive(session_rust::Tolerance::ANGULARDEFLECTION, 0.0, 0.0);
        if pts.len() < 2 { return; }
        let instance_id = self.pick.allocate(&guid);
        let segs = pts_to_segments(&pts, instance_id);
        let seg_start = self.segments_cpu.len();
        self.segments_cpu.extend(segs);
        self.guid_to_seg.insert(guid.clone(), seg_start..self.segments_cpu.len());
        self.segments_dirty = true;
        self.nc_pick_pts.insert(guid.clone(), pts.iter().map(|p| [p[0], p[1], p[2]]).collect());
        let gly_start = self.glyphs_cpu.len();
        for p in [pts.first().unwrap(), pts.last().unwrap()] {
            let center = [p[0], p[1], p[2]];
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
        let mut mesh = session_rust::remesh_nurbssurface_grid::RemeshNurbsSurfaceGrid::from_u_v(
            surface.clone(), 32, 32);
        let (vs, is) = mesh_to_vertices(&mesh);
        if vs.is_empty() { return; }
        let instance_id = self.pick.allocate(&guid);
        self.tri.allocate(&guid, &vs, Some(&is), instance_id, device, queue);
        // Boundary edges: 4 iso-curves.
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
                    session_rust::Tolerance::ANGULARDEFLECTION * 0.1, 0.0, 0.0);
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
        self.nurbs_pick_meshes.insert(guid.clone(), mesh);
        self.nurbs_surfaces.insert(guid.clone(), surface.clone());
        let surf_color = color_to_rgba_f32(surface.facecolors.get(0).unwrap_or(&session_rust::Color::white()));
        self.write_instance_flags(instance_id, surf_color, InstanceData::FLAG_SMOOTH, device, queue);
    }

    pub fn add_geometry(&mut self, guid: &str, geom: &session_rust::session::Geometry, device: &wgpu::Device, queue: &wgpu::Queue) {
        use session_rust::session::Geometry;
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
                self.tri.allocate(guid, &vs, Some(&is), instance_id, device, queue);
                let segs = if m.crease_angle_deg > 0.0 {
                    mesh_crease_edges_to_segments(m, instance_id, m.crease_angle_deg)
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
                self.write_instance_flags(instance_id, color_to_rgba_f32(m.objectcolor()), edge_flag | InstanceData::FLAG_GLYPHS_HIDDEN, device, queue);
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
                // Tessellate each BRep face independently (trimmed where needed, reversed
                // normals applied) so face boundaries are hard edges with correct shading.
                let mut vs: Vec<MeshVertex> = Vec::new();
                let mut is: Vec<u32> = Vec::new();
                for fm in b.face_meshes() {
                    let (fvs, fis) = mesh_to_vertices(&fm);
                    if fvs.is_empty() { continue; }
                    let offset = vs.len() as u32;
                    vs.extend(fvs);
                    is.extend(fis.iter().map(|&i| i + offset));
                }
                let mut bm = b.mesh();
                if !vs.is_empty() {
                    self.tri.allocate(guid, &vs, Some(&is), instance_id, device, queue);
                }
                let xf = b.xform.to_cols();
                // Edge segments stay in local space — inst.model applied in cylinder.wgsl
                let mut edge_segs: Vec<CylinderSegment> = Vec::new();
                if !b.m_curves_3d.is_empty() {
                    for curve in &b.m_curves_3d {
                        if !curve.is_valid() { continue; }
                        let (pts, _) = curve.to_polyline_adaptive(
                            session_rust::Tolerance::ANGULARDEFLECTION, 0.0, 0.0);
                        edge_segs.extend(pts_to_segments(&pts, instance_id));
                    }
                } else {
                    edge_segs.extend(mesh_naked_edges_to_segments(&bm, instance_id));
                }
                if edge_segs.is_empty() && vs.is_empty() { self.pick.release(guid); return; }
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
                            self.tri.allocate(guid, &vs, Some(&is), instance_id, device, queue);
                            self.write_instance(instance_id, color_to_rgba_f32(m.objectcolor()), device, queue);
                        } else {
                            self.pick.release(guid);
                        }
                    }
                    ElementGeometry::BRep(b) => {
                        let m = b.mesh();
                        let (vs, is) = mesh_to_vertices(&m);
                        if !vs.is_empty() {
                            self.tri.allocate(guid, &vs, Some(&is), instance_id, device, queue);
                            self.write_instance(instance_id, color_to_rgba_f32(&b.surfacecolor), device, queue);
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
