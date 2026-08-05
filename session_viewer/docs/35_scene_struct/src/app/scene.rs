//! `Scene` — the open DOCUMENT. It owns the kernel `Session` (the same guid → Geometry
//! data structure session_py/session_cpp use) plus the viewer-only bookkeeping the kernel
//! must not know about: row order, guid→row map, hidden set. Everything document-shaped
//! (reconcile 38, picking 42, undo 51) talks to THIS type; `Gpu` only ever sees the flat
//! `ArenaUpload` that `build()` emits.

use std::collections::{HashMap, HashSet};
use session_rust::{Session, Geometry, Mesh, Line, Point, Polyline, Xform, RenderVertex};
use session_rust::mesh::ColorMode;
use crate::engine::gpu::{Instance, CylinderSegment, GlyphPoint, ArenaUpload};

pub struct Scene {
    pub session: Session,                // THE document — kernel type, source of truth
    order: Vec<String>,                  // renderable guids in fixed row order
    pub guid_to_row: HashMap<String, u32>,
    pub hidden: HashSet<String>,
}

impl Scene {
    pub fn new(session: Session) -> Self {
        let mut order = Vec::new();
        let mut guid_to_row = HashMap::new();
        // session.order() is the kernel's CANONICAL order — deterministic across runs and
        // languages; Scene keeps the renderable subset of it.
        for guid in session.order() {
            if matches!(&session.lookup[&guid], Geometry::Mesh(_) | Geometry::BRep(_) | Geometry::Line(_) |
                              Geometry::Polyline(_) | Geometry::Point(_)) {
                guid_to_row.insert(guid.clone(), order.len() as u32);
                order.push(guid);
            }
        }
        Self { session, order, guid_to_row, hidden: HashSet::new() }
    }

    /// The lesson-34 walk, moved out of `Gpu`. Emits the TRUE per-object transform (placement lives
    /// in `mesh.xform`; `to_render` ignores it) — 33's rebuild_instances rebases it every frame.
    /// `ri` is the objects_base row, never the lookup index, so nothing reads a stale instance row.
    pub fn build(&self) -> ArenaUpload {
        let mut verts: Vec<RenderVertex> = Vec::new();
        let mut vids: Vec<u32> = Vec::new();
        let mut idx: Vec<u32> = Vec::new();
        let mut segments: Vec<CylinderSegment> = Vec::new();
        let mut glyphs: Vec<GlyphPoint> = Vec::new();
        let mut objects_base: Vec<(Xform, [f32; 4], u32)> = Vec::with_capacity(self.order.len());

        for (ri, guid) in self.order.iter().enumerate() {
            let ri = ri as u32;
            let flags = if self.hidden.contains(guid) { Instance::FLAG_HIDDEN } else { 0 };
            match &self.session.lookup[guid] {
                Geometry::Mesh(m) => {
                    // white TINT (34h) — the real colors ride the rows; placement = xform
                    objects_base.push((m.xform.clone(), [1.0; 4], flags));
                    push_mesh(m, ri, &mut verts, &mut vids, &mut idx, &mut segments, &mut glyphs);
                }
                Geometry::BRep(b) => {
                    let mut bm = b.mesh();
                    bm.set_objectcolor(b.surfacecolor.clone());   // 34h's surfacecolor bake
                    objects_base.push((b.xform.clone(), [1.0; 4], flags));
                    push_mesh(&bm, ri, &mut verts, &mut vids, &mut idx, &mut segments, &mut glyphs);
                }
                Geometry::Line(l) => {
                    objects_base.push((l.xform.clone(), [1.0; 4], flags));
                    segments.push(line_to_segment(l, ri));
                }
                Geometry::Polyline(pl) => {
                    objects_base.push((pl.xform.clone(), [1.0; 4], flags));
                    segments.extend(polyline_to_segments(pl, ri));
                }
                Geometry::Point(p) => {
                    objects_base.push((p.xform.clone(), [1.0; 4], flags));
                    glyphs.push(point_to_glyph(p, ri));
                }
                _ => {} // Scene::new only put renderable guids into order
            }
        }

        // 34f's paper-space lane, moved here with the walk: planar (z ≡ 0) sheets get
        // world-mm lineweights; 3D files keep screen-constant px.
        let (mut zmin, mut zmax) = (f32::INFINITY, f32::NEG_INFINITY);
        for s in &segments {
            zmin = zmin.min(s.p0[2].min(s.p1[2]));
            zmax = zmax.max(s.p0[2].max(s.p1[2]));
        }
        if zmin.is_finite() && (zmax - zmin).abs() < 1e-3 {
            for s in &mut segments {
                s.radius = if s.radius < 0.0 { -s.radius * 0.5 } else { 0.5 };
            }
        }

        ArenaUpload { verts, vids, idx, objects_base, segments, glyphs }
    }
}

// ── geometry → GPU-row converters, moved up from engine/gpu (34's adapters.rs + push_mesh) ──
// They name Mesh/Line/Point — document types — which is exactly why they live in the app layer.

fn line_to_segment(l: &Line, instance_id: u32) -> CylinderSegment{
    CylinderSegment {
        p0: l.start().to_f32(),
        radius: encode_width(l.width),
        p1: l.end().to_f32(),
        instance_id,
        color: l.linecolor.to_f32(),
    }
}

fn polyline_to_segments(pl: &Polyline, instance_id: u32) -> Vec<CylinderSegment>{
    let pts = pl.get_points();
    let color = pl.linecolor.to_f32();
    pts.windows(2).map(|w| CylinderSegment{
        p0: w[0].to_f32(),
        radius: encode_width(pl.width),
        p1: w[1].to_f32(),
        instance_id,
        color,
    }).collect()
}

fn point_to_glyph(p: &Point, instance_id: u32) -> GlyphPoint{
    GlyphPoint {
        center: p.to_f32(),
        radius: encode_width(p.width),
        color: p.pointcolor.to_f32(),
        instance_id,
        _pad: [0; 3],
    }
}

/// Kernel width (dimensionless, default 1.0) → the radius encoding's NEGATIVE lane (px
/// multiplier); 0.0 = plain global default. `Scene::build` flips negatives into the POSITIVE
/// (world-mm) lane for planar 2D drawings — paper-space lineweights that scale with zoom.
fn encode_width(w: f64) -> f32 {
    if w.is_finite() && w > 0.0 && (w - 1.0).abs() > 1e-9 { -(w as f32) } else { 0.0 }
}

fn push_mesh(
    m: &Mesh,
    ri: u32,
    verts: &mut Vec<RenderVertex>,
    vids: &mut Vec<u32>,
    idx: &mut Vec<u32>,
    segments: &mut Vec<CylinderSegment>,
    glyphs: &mut Vec<GlyphPoint>
){
    let base = verts.len() as u32;
    let rm = m.to_render();
    for v in &rm.vertices{
        verts.push(*v);
        vids.push(ri);
    }
    for &i in &rm.indices{
        idx.push(base+i);
    }

    for (i, (a, b, col)) in m.edges_with_colors().into_iter().enumerate(){
        let pa = m.vertex_point(a).unwrap();
        let pb = m.vertex_point(b).unwrap();
        segments.push(
            CylinderSegment{
                p0: pa.to_f32(),
                radius: encode_width(m.widths().get(i).copied().unwrap_or(1.0)),
                p1: pb.to_f32(),
                instance_id: ri,
                color: col.to_f32()
            }
        )
    }

    // Dots honor user-set pointcolors; the auto-seeded white vec is filtered by the MODE gate.
    // m.vertices() is sorted — the same order to_render indexes pointcolors by.
    let pc = m.get_pointcolors();
    let dots_colored = m.color_mode == ColorMode::POINTCOLORS && pc.len() == m.number_of_vertices();
    for (i, vk) in m.vertices().into_iter().enumerate(){
        let p = m.vertex_point(vk).unwrap();
        glyphs.push(
            GlyphPoint {
                center: p.to_f32(),
                radius: 0.0,                       // no per-vertex width exists in the kernel
                color: if dots_colored { pc[i].to_f32() } else { [0.1, 0.1, 0.1, 1.0] },
                instance_id: ri,
                _pad: [0;3] }
        );
    }
}
