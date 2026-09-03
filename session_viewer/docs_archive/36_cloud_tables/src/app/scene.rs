//! The scene manifest: WHICH files a scene is made of and WHERE each one sits.
//!
//! A drawing is authored at its own page origin, so any number of them loaded raw would stack on
//! top of each other. Placement therefore has to come from somewhere - and the honest place is a
//! text file next to the assets, not arithmetic buried in the GPU layer. Edit `at`, reload, no
//! rebuild; a web deployment can be re-arranged without a compiler.
//!
//! ```json
//! { "items": [ { "file": "pb/draw_pf_he.pb", "name": "HE", "at": [3400, 0, 0] } ] }
//! ```
//! `at` is a translation in world units. `xform` takes all 16 numbers instead when a sheet needs
//! rotation or scale. An item with neither falls back to the auto-grid below.
use serde::Deserialize;
use session_rust::Xform;

/// One manifest entry: a file to load and where to place it. Every file is authored at its own
/// origin, so each item carries a placement transform (`at` or `xform`) — without one, all
/// files would stack at (0,0,0); items with neither get an `auto_grid` slot instead.
#[derive(Deserialize)]
pub struct Item {
    pub file: String,                 // asset path, e.g. "pb/draw_pf_he.pb"
    #[serde(default)]
    pub name: String,                 // display name; empty = use the session's own
    #[serde(default)]
    pub at: Option<[f64; 3]>,         // translation in world units
    #[serde(default)]
    pub xform: Option<[f64; 16]>,     // full 4x4 (wins over `at`); neither = auto_grid
    #[serde(default)]
    pub point_size: f64,              // raw-cloud px for this file; 0 = keep the pb'own
}

/// The parsed scene file: an ordered list of items, loaded in list order.
#[derive(Deserialize)]
pub struct Manifest {
    #[serde(default)]
    pub name: String,
    pub items: Vec<Item>,
}

impl Item {
    /// The placement this item asks for, or `None` when it wants the auto-grid.
    pub fn placement(&self) -> Option<Xform> {
        if let Some(m) = self.xform {
            let mut x = Xform::identity();
            x.m = m;
            return Some(x);
        }
        self.at.map(|a| Xform::translation(a[0], a[1], a[2]))
    }
}

impl Manifest {
    /// JSON first (every existing scene), TOML as the fallback - a .toml manifest gets
    /// real comments and no trailing-comma landmines; both land in the same structs.
    pub fn parse(bytes: &[u8]) -> Option<Self> {
        serde_json::from_slice(bytes).ok()
            .or_else(|| std::str::from_utf8(bytes).ok().and_then(|s| toml::from_str(s).ok()))
    }
}

/// Fallback for items with no `at`/`xform`: lay them out on a grid of `cell` steps, in list order.
/// Deliberately dumb - it exists so a manifest can be written one sheet at a time, not as the way
/// a scene is normally described.
pub fn auto_grid(index: usize, count: usize, cell: [f64; 2]) -> Xform {
    let cols = (count as f64).sqrt().ceil().max(1.0) as usize;
    Xform::translation((index % cols) as f64 * cell[0], (index / cols) as f64 * cell[1], 0.0)
}

// ─────────────────────────────────────────────────────────────────────────────────────────────
// The DOCUMENT side of the scene: manifest above says WHERE, `Scene` below owns WHAT.

use std::collections::{HashMap, HashSet};
use session_rust::{Session, Geometry, Mesh, Line, Point, Polyline, NurbsCurve, RenderVertex, Plane, OBB, PointCloud, Vector};
use session_rust::element::ElementGeometry;
use session_rust::mesh::ColorMode;
use crate::engine::gpu::{ArenaUpload, Instance, CylinderSegment, GlyphPoint};

/// One loaded file: the kernel `Session`
/// kept alive - picking/undo/save need this
/// plus the placement the manifest gave it
pub struct Doc{
    pub name: String,
    pub place: Xform,
    pub session: Session,
    pub cloud_px: f32, // per-file raw_cloud point size, px; 0 = pb's own
}

/// The open document set + the merged GPU tables.
/// `add_file` walks one new session straight into the shared tables
/// rows are appended, never rebuilt
/// It means the progressive loading costs each file only its own walk.
/// Viewer-only bookkeeping (row order, grid to rowm hidden) lives here
/// It never in the kernel type that threee languages share.
pub struct Scene {
    pub docs: Vec<Doc>,
    vert_base: u32,             // arena rows already uploaded - push_mesh bases its indices on this
    pub tables: ArenaUpload,
    order: Vec<String>, // renderable guids, global row order across docs
    pub guid_to_row: HashMap<String, u32>,
    pub hidden: HashSet<String>,
}

impl Scene{
    pub fn new() -> Self{
        Self {
        docs: Vec::new(),
        vert_base: 0,
        tables: ArenaUpload::new(),
        order: Vec::new(),
        guid_to_row: HashMap::new(),
        hidden: HashSet::new(),
        }
    }

    /// Re-flatten EVERY document from its kernel `Session` and re-upload from scratch.
    ///
    /// The blunt instrument, and the reason it exists: once `upload_to` clears the arena
    /// mirror, there is no CPU copy left to patch, so changing an object's GEOMETRY (dragging
    /// a polyline vertex, editing a mesh) has nothing to rewrite. Per-object arena ranges are
    /// what fixes that properly - the planned `guid -> range` map - and until then this gets
    /// the same result by redoing the walk. It costs a full re-walk, so it belongs behind an
    /// edit commit, not behind a drag.
    pub fn rebuild(&mut self, gpu: &mut crate::engine::gpu::Gpu) {
        let docs = std::mem::take(&mut self.docs);
        self.tables = ArenaUpload::new();
        self.order.clear();
        self.guid_to_row.clear();
        self.vert_base = 0;
        gpu.reset_arena();

        for d in docs {
            self.add_file(d.name, d.session, d.place, d.cloud_px);
        }
        self.upload_to(gpu);
    }

    /// Upload the walked tables, then FORGET the arena rows: the GPU is their only holder.
    pub fn upload_to(&mut self, gpu: &mut crate::engine::gpu::Gpu) {
        gpu.set_scene(&self.tables);
        // The arena rows are on the GPU now and nothing reads them back - picking goes through
        // the kernel Meshes in Doc.session, never through these flattened rows. Keep only the
        // running vertex base, so the next file's indices still land in the right place.
        self.vert_base += self.tables.verts.len() as u32;
        self.tables.verts.clear();
        self.tables.verts.shrink_to_fit();
        self.tables.vids.clear();
        self.tables.vids.shrink_to_fit();
        self.tables.idx.clear();
        self.tables.idx.shrink_to_fit();
    }

    /// Walk one session into the shared tables.
    /// We moved out of GPU struct:
    /// - placement = manifest `place` x the session's own `world_xforms()` one downward pass per object `world_xform()` rescans the the tree each call
    /// - `session_order()` is the kernel's canonical order, deterministics across runs and languages - the row a guid gets here in the row it keeps (picking/selection rely on it)
    /// - per file planar test: a sheet flat along its own normal (place + orientation)
    pub fn add_file(&mut self, name: String, session: Session, place: Xform, cloud_px: f32){

        let seg0 = self.tables.segments.len();
        let pipe0 = self.tables.pipes.len();
        let vert0 = self.tables.verts.len();
        let vb = self.vert_base; // read before `t` borrows self.tables
        let sphere0 = self.tables.spheres.len();
        let glyph0 = self.tables.glyphs.len();
        let obj0 = self.tables.objects.len();
        let draw0 = self.tables.cloud_draws.len();

        let world = session.world_xforms();
        let placement = |guid: &str| world.get(guid).cloned().unwrap_or_else(Xform::identity);
        let t = &mut self.tables;
        for guid in session.order() {
            let Some(geom) = session.lookup.get(&guid) else { continue };
            if let Geometry::Element(e) = geom {
                if matches!(e.geometry(), ElementGeometry::None){
                    continue
                }
            }
            let ri = t.objects.len() as u32;
            let flags = if self.hidden.contains(&guid) { Instance::FLAG_HIDDEN } else { 0 };
            let placed = &place * &placement(&guid);
            t.objects.push((placed, [1.0; 4], flags));
            
            match geom{
                // 3D geometry takes the solid lane: edges are real cylinders and vertices - spheres
                Geometry::Mesh(m) => {

                    let b = push_mesh(
                        m, 
                        ri,
                        vb, 
                        &mut t.verts, 
                        &mut t.vids, 
                        &mut t.idx, 
                        &mut t.pipes,
                        &mut t.spheres
                    );
                    if is_print_fill(m) {
                        // The object row for this guid was pushed just above the match - .2 is flags.
                        t.objects.last_mut().unwrap().2 |= Instance::FLAG_PRINT;
                    }
                    // An open mesh (boundary edges) is not a solid: the facing cull would strip
                    // the wireframe off interior surface that is genuinely visible through the
                    // hole while the faces still draw. Meshes only - a BRep tessellation is often
                    // numerically non-watertight and its solids would lose the cull wholesale.
                    if !m.is_closed() {
                        t.objects.last_mut().unwrap().2 |= Instance::FLAG_OPEN;
                    }
                    t.object_bounds.push(b); t.object_spacing.push(mesh_spacing(b, m.number_of_vertices()));
                }
                Geometry::BRep(b) => {
                    let mut bm = b.mesh();
                    bm.set_objectcolor(b.surfacecolor.clone());
                    let bb = push_mesh(
                        &bm, 
                        ri,
                        vb, 
                        &mut t.verts, 
                        &mut t.vids, 
                        &mut t.idx, 
                        &mut t.pipes,
                        &mut t.spheres
                    );
                    t.object_bounds.push(bb); t.object_spacing.push(mesh_spacing(bb, bm.number_of_vertices()));
                }
                Geometry::Line(l) => { t.segments.push(line_to_segment(l, ri)); t.object_bounds.push(None); t.object_spacing.push(0.0); }
                Geometry::Polyline(pl) => { t.segments.extend(polyline_to_segments(pl, ri)); t.object_bounds.push(None); t.object_spacing.push(0.0); }
                Geometry::NurbsCurve(c) => { t.segments.extend(nurbscurve_to_segments(c, ri)); t.object_bounds.push(None); t.object_spacing.push(0.0); }
                Geometry::Point(p) => { t.glyphs.push(point_to_glyph(p, ri)); t.object_bounds.push(None); t.object_spacing.push(0.0); }
                // EVERY cloud takes the splat lane: split flat rows into share tables,
                // one draw record per cloud, and the per cloud point size rides the spacing spacing
                Geometry::PointCloud(pc) => {
                    let first = (t.cloud_pos.len() / 3) as u32;
                    push_cloud(pc, &mut t.cloud_pos, &mut t.cloud_col, &mut t.cloud_nrm);
                    t.cloud_draws.push((first, pc.len() as u32, ri, cloud_spacing(pc)));
                    let px = if cloud_px > 0.0 { cloud_px } else { pc.point_size as f32 };
                    t.object_bounds.push(None);
                    t.object_spacing.push(px);
                }
                Geometry::NurbsSurface(s) => {
                    let mut sm = s.mesh();
                    if let Some(c) = s.facecolors.first() {
                        sm.set_objectcolor(c.clone());
                    }
                    let b = push_mesh(
                        &sm, 
                        ri,
                        vb, 
                        &mut t.verts, 
                        &mut t.vids, 
                        &mut t.idx, 
                        &mut t.pipes,
                        &mut t.spheres
                    );
                    t.object_bounds.push(b); t.object_spacing.push(mesh_spacing(b, sm.number_of_vertices()));
                }
                Geometry::Plane(p) => { t.segments.extend(plane_to_segments(p, ri)); t.object_bounds.push(None); t.object_spacing.push(0.0); }
                Geometry::OBB(b) => { t.segments.extend(obb_to_segments(b, ri)); t.object_bounds.push(None); t.object_spacing.push(0.0); }
                Geometry::Element(e) => match e.geometry() {
                    ElementGeometry::Mesh(m) => {
                        let b = push_mesh(
                            &m, 
                            ri,
                        vb, 
                            &mut t.verts, 
                            &mut t.vids, 
                            &mut t.idx, 
                            &mut t.pipes,
                            &mut t.spheres
                        );
                        if is_print_fill(&m) {
                            t.objects.last_mut().unwrap().2 |= Instance::FLAG_PRINT;
                        }
                        t.object_bounds.push(b); t.object_spacing.push(mesh_spacing(b, m.number_of_vertices()));
                    }
                    ElementGeometry::BRep(b) => {
                        let mut bm = b.mesh();
                        bm.set_objectcolor(b.surfacecolor.clone());
                        let bb = push_mesh(
                            &bm, 
                            ri,
                        vb, 
                            &mut t.verts, 
                            &mut t.vids, 
                            &mut t.idx, 
                            &mut t.pipes,
                            &mut t.spheres
                        );
                        t.object_bounds.push(bb); t.object_spacing.push(mesh_spacing(bb, bm.number_of_vertices()));
                    }
                    ElementGeometry::None => { t.object_bounds.push(None); t.object_spacing.push(0.0); },
                },
            }
            self.guid_to_row.insert(guid.clone(), ri);
            self.order.push(guid);
        }

        // This file extends in world placement
        // Each row through its object's full xform
        // Both the planar test and the scene bounds what is actuall drawn.
        let (mut fmin, mut fmax) = ([f32::INFINITY; 3], [f32::NEG_INFINITY; 3]);
        for (i, v) in t.verts.iter().enumerate().skip(vert0) {
            if let Some(&ri) = t.vids.get(i) {
                if let Some((xf, _, _)) = t.objects.get(ri as usize) {
                    grow_bounds(&mut fmin, &mut fmax, xform_point(xf, v.position));
                }
            }
        }
        
        for s in t.pipes.iter().skip(pipe0).chain(t.segments.iter().skip(seg0)){
            if let Some((xf, _, _)) = t.objects.get(s.instance_id as usize){
                grow_bounds(&mut fmin, &mut fmax, xform_point(xf, s.p0));
                grow_bounds(&mut fmin, &mut fmax, xform_point(xf, s.p1));
            } 
        }

        for s in t.spheres.iter().skip(sphere0).chain(t.glyphs.iter().skip(glyph0)){
            if let Some((xf, _, _)) = t.objects.get(s.instance_id as usize){
                grow_bounds(&mut fmin, &mut fmax, xform_point(xf, s.center));
            } 
        }

        for &(first, count, inst, _) in t.cloud_draws.iter().skip(draw0){
            let Some((xf, _, _)) = t.objects.get(inst as usize) else { continue };
            for i in first as usize..(first + count) as usize {
                let p = [t.cloud_pos[i*3], t.cloud_pos[i*3+1], t.cloud_pos[i*3 + 2]];
                grow_bounds(&mut fmin, &mut fmax, xform_point(xf, p));
            }
        }

        for k in 0..3{
            t.min[k] = t.min[k].min(fmin[k]);
            t.max[k] = t.max[k].max(fmax[k]);
        }

        // 2D drawing sheets
        // flat linework - every PDF conversion gets paper space
        // keep kernel a real print
        // 3D model files keep screen-constant px linework
        // Planar = thin alon the SHEET's normal
        // The 99% path - translation only place, normal is Z+
        // reuseses the z-extent accumulated aboce - no extra work at all
        // only a rotated placement pays one dot-product pass over this file's new rows
        let n = place.transform_vector(&Vector::new(0.0, 0.0, 1.0));
        let thickness = if n[0].abs() < 1e-9 && n[1].abs() < 1e-9 {
            fmax[2] - fmin[2]
        } else {
            let (nx, ny, nz) = (n[0] as f32, n[1] as f32, n[2] as f32);
            let (mut dmin, mut dmax) = (f32::INFINITY, f32::NEG_INFINITY);
            let mut span = |p: [f32; 3]| {
                let d = p[0] * nx + p[1] * ny + p[2] * nz;
                dmin = dmin.min(d);
                dmax = dmax.max(d);
            };
            for (i, v) in t.verts.iter().enumerate().skip(vert0){
                if let Some(&ri) = t.vids.get(i){
                    if let Some((xf, _, _)) = t.objects.get(ri as usize) {
                        span(xform_point(xf, v.position));
                    }
                }
            }
            for s in t.pipes.iter().skip(pipe0).chain(t.segments.iter().skip(seg0)){
                if let Some((xf, _, _)) = t.objects.get(s.instance_id as usize){
                    span(xform_point(xf, s.p0));
                    span(xform_point(xf, s.p1));
                }
            }
            for g in t.spheres.iter().skip(sphere0).chain(t.glyphs.iter().skip(glyph0)){
                if let Some((xf, _, _)) = t.objects.get(g.instance_id as usize) {
                    span(xform_point(xf, g.center));
                }
            }
            dmax - dmin
        };

        let planar = thickness.is_finite() && thickness.abs() < 1e-3;

        if planar {
            for s in t.pipes.iter_mut().skip(pipe0).chain(t.segments.iter_mut().skip(seg0)){
                // A flat sheet is paper: every pen becomes a world-mm radius so widths behave
                // like plotter pens. encode_width already returns a positive mm radius for any
                // authored width, so only the unset default (0.0) needs a value here - 0.5 mm,
                // the usual hairline. This used to read `radius < 0` because widths arrived as
                // NEGATIVE multipliers; they are millimetres now.
                s.radius = if s.radius > 0.0 {
                    s.radius
                } else {
                    0.5
                } 
            }
        }

        let _ = obj0;
        self.docs.push(Doc {
            name,
            place,
            session,
            cloud_px
        });

    }

}


fn line_to_segment(l: &Line, instance_id: u32) -> CylinderSegment {
    CylinderSegment {
        p0: l.start().to_f32(),
        radius: encode_width(l.width),
        p1: l.end().to_f32(),
        instance_id,
        color: pack_rgba(l.linecolor.to_f32()),
        facing: FACING_UNKNOWN, // free-standing linework has no adjacent faces: always drawn
    }
}

fn polyline_to_segments(pl: &Polyline, instance_id: u32) -> Vec<CylinderSegment> {
    let pts = pl.get_points();
    let color = pack_rgba(pl.linecolor.to_f32());
    pts.windows(2).map( |w| CylinderSegment {
        p0: w[0].to_f32(),
        radius: encode_width(pl.width),
        p1: w[1].to_f32(),
        instance_id,
        color,
        facing: FACING_UNKNOWN,
    }).collect()
}

fn nurbscurve_to_segments(c: &NurbsCurve, instance_id: u32) -> Vec<CylinderSegment> {
    // Bounding box of the CONTROL POINTS - cheap, and it bounds the curve (a NURBS curve
    // never leaves its control net), so it stands in for "how big is this curve".
    let (mut lo, mut hi) = ([f64::MAX; 3], [f64::MIN; 3]);
    for i in 0..c.m_cv_count {
        if let Some(cv) = c.cv(i) {
            // Rational curves store WEIGHTED CVs [x*w, y*w, z*w, w] - divide by w to get
            // the real point; non-rational (or w=0 guard) uses the coords as-is.
            let w = if c.m_is_rat && cv.len() > 3 && cv[3] != 0.0 {
                cv[3]
            } else {
                1.0
            };
            for k in 0..3 {
                lo[k] = lo[k].min(cv[k] / w);
                hi[k] = hi[k].max(cv[k] / w);
            }
        }
    }
    // No CV ever grew the box (empty/invalid curve) -> lo is still MAX: nothing to draw.
    if lo[0] > hi[0] {
        return Vec::new();
    }
    // Sample count follows curve SIZE (box diagonal): a 2mm glyph outline gets 4 segments,
    // a metre-long arc ~50 - sqrt scaling, clamped so nothing under- or over-tessellates.
    let size = ((hi[0]-lo[0]).powi(2) + (hi[1]-lo[1]).powi(2) + (hi[2]-lo[2]).powi(2)).sqrt();
    let n = ((size / 0.2).sqrt().ceil() as usize).clamp(4, 64);

    // Evaluate the curve at n+1 evenly spaced parameters across its domain [t0, t1] ...
    let (t0, t1) = c.domain();
    let color = pack_rgba(c.linecolors.first().map(|c| c.to_f32()).unwrap_or([0.0, 0.0, 0.0, 1.0]));
    let radius = encode_width(c.width);
    let pts: Vec<[f32; 3]> = (0..=n)
        .map(|i| c.point_at(t0 + (t1 - t0) * i as f64 / n as f64).to_f32())
        .collect();
    // ... then it IS a polyline: consecutive pairs -> segments, same as polyline_to_segments.
    pts.windows(2).map(|w| CylinderSegment {
        p0: w[0],
        radius,
        p1: w[1],
        instance_id,
        color,
        facing: FACING_UNKNOWN,
    }).collect()
}

fn point_to_glyph(p: &Point, instance_id: u32) -> GlyphPoint {
    GlyphPoint {
        center: p.to_f32(),
        radius: encode_width(p.width),
        color: p.pointcolor.to_f32(),
        instance_id,
        facing: FACING_UNKNOWN, // a free point decorates no surface
        facing_ext: [FACING_UNKNOWN; 2],
    }
}

/// The kernel's `width` is in MILLIMETRES - the drawings lane talks in 0.09-0.5 mm plot pens
/// and `Line`/`Polyline` default to 1.0. This used to return `-(w)`, and a NEGATIVE radius means
/// "multiply the global pen" to every shader - so a 30 mm polyline became 2 px x 30 = a 60 px
/// half-width, a 120 px slab. Millimetres were being read as a multiplier.
///
/// Now: an explicit width is a world-mm RADIUS (half the width, positive => the projected
/// branch), and only the untouched 1.0 default falls back to the screen-constant pen. That
/// keeps mesh edges - which never set a width - at a zoom-independent 2 px, while a pen someone
/// actually authored measures what it says.
fn encode_width(w: f64) -> f32{
    if w.is_finite() && w > 0.0 && (w - 1.0).abs() > 1e-9 {
        (w as f32) * 0.5
    } else {
        0.0
    }
}

/// RGBA8 in one word, low byte red - the layout `unpack4x8unorm` expects in WGSL.
fn pack_rgba(c: [f32; 4]) -> u32 {
    let q = |v: f32| ((v.clamp(0.0, 1.0) * 255.0 + 0.5) as u32) & 0xff;
    q(c[0]) | q(c[1]) << 8 | q(c[2]) << 16 | q(c[3]) << 24
}

/// A unit vector in 16 bits, octahedral: project onto the octahedron, fold the lower hemisphere
/// out across the diagonals, and store the two coordinates as signed bytes. ~1.4 degrees of error,
/// which is generous for a value only ever used for the SIGN of a dot product.
fn oct16(n: &Vector) -> Option<u32> {
    let l = n[0].abs() + n[1].abs() + n[2].abs();
    if !(l > 0.0) {
        return None;
    }
    let (mut x, mut y) = (n[0] / l, n[1] / l);
    if n[2] < 0.0 {
        // signNotZero, NOT signum. `f64::signum(0.0)` is 0.0, which folds (0,0,-1) onto (0,0) -
        // the code for (0,0,+1) - so the two poles collided. On an axis-aligned box that is the
        // top and bottom faces, i.e. most of its edges, and the collision then landed on the
        // all-zero "no adjacency" sentinel: the facing test silently did nothing for them.
        let s = |v: f64| if v < 0.0 { -1.0 } else { 1.0 };
        let (ax, ay) = (x.abs(), y.abs());
        (x, y) = ((1.0 - ay) * s(x), (1.0 - ax) * s(y));
    }
    let q = |v: f64| (((v.clamp(-1.0, 1.0) * 127.0).round() as i32) as u32) & 0xff;
    Some(q(x) | q(y) << 8)
}

/// `facing` value meaning "this edge has no adjacency, always draw it".
///
/// It cannot be 0: (0, 0) is the honest encoding of +Z. All four corners of the octahedral square
/// collapse onto -Z, so the all-ones word is a value the encoder can produce but never needs, which
/// makes it the one safe sentinel here.
pub const FACING_UNKNOWN: u32 = u32::MAX;

/// The two faces an edge belongs to, packed into one word for the shader's facing test.
///
/// `FACING_UNKNOWN` means "no adjacency known, always draw" - see the constant for why it is the
/// all-ones word and not 0.
fn pack_facing(n0: Option<Vector>, n1: Option<Vector>) -> u32 {
    let pair = match (n0, n1) {
        (Some(a), Some(b)) => (oct16(&a), oct16(&b)),
        // A naked edge is visible whenever its single face is, so duplicating the one normal is
        // the correct answer and needs no special case in the shader.
        (Some(a), None) | (None, Some(a)) => (oct16(&a), oct16(&a)),
        _ => (None, None),
    };
    match pair {
        (Some(a), Some(b)) => {
            let v = a | b << 16;
            if v == FACING_UNKNOWN { FACING_UNKNOWN } else { v }
        }
        _ => FACING_UNKNOWN,
    }
}

/// Typical distance between a mesh's vertices, world units: the AABB diagonal over the square root
/// of the vertex count. A surface mesh spreads its vertices over an AREA, so the count's square
/// root is what scales with the extent, which makes this a good proxy for "how far apart are
/// neighbouring vertices" without walking the edges a second time. The ink lanes drop their
/// markers once it projects below a few pixels - see WIRE_MIN_PX in ribbon.wgsl.
fn mesh_spacing(bounds: Option<([f32; 3], [f32; 3])>, verts: usize) -> f32 {
    let Some((lo, hi)) = bounds else { return 0.0 };
    if verts < 2 {
        return 0.0;
    }
    let d = ((hi[0]-lo[0]).powi(2) + (hi[1]-lo[1]).powi(2) + (hi[2]-lo[2]).powi(2)).sqrt();
    d / (verts as f32).sqrt()
}

/// A fill (every PDF glyph, every poché region) broadcasts a single width of 0 - print, not
/// surface. One test in one place: it drives the wireframe skip in push_mesh AND
/// Instance::FLAG_PRINT (flat lighting, so the sheet reads the same from the back), and the
/// two cannot drift apart.
fn is_print_fill(m: &Mesh) -> bool {
    m.widths().len() == 1 && m.widths()[0] == 0.0
}

/// Debug toggles read ONCE per process instead of once per mesh. An env lookup is a linear
/// scan of the environment block, and a sheet can hold tens of thousands of tiny fill meshes -
/// at three reads per mesh (PROFILE, NO_EDGES, NO_DOTS) that alone was ~30 ms against HEAD's
/// two on a 33 MB sheet. These are launch-time harness toggles; setting one mid-process was
/// never a use case.
fn env_flag(name: &str, slot: &'static std::sync::OnceLock<bool>) -> bool {
    *slot.get_or_init(|| std::env::var(name).is_ok())
}
static VIEWER_PROFILE: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
static VIEWER_NO_EDGES: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
static VIEWER_NO_DOTS: std::sync::OnceLock<bool> = std::sync::OnceLock::new();

fn push_mesh(
    m: &Mesh,
    ri: u32,
    base_off: u32,
    verts: &mut Vec<RenderVertex>,
    vids: &mut Vec<u32>,
    idx: &mut Vec<u32>,
    segments: &mut Vec<CylinderSegment>,
    glyphs: &mut Vec<GlyphPoint>
) -> Option<([f32; 3], [f32; 3])> {
    let base = base_off + verts.len() as u32; // GPU rows already uploaded + rows pending in this delta
    // VIEWER_PROFILE=1 times the walk's stages. HARNESS-ONLY, and the cfg is load-bearing, not
    // tidiness: `Instant::now()` on wasm32-unknown-unknown does not return a dummy, it PANICS
    // ("time not implemented on this platform"), and this line runs for every mesh - so an
    // ungated clock here kills the browser build on the first mesh it walks.
    #[cfg(not(target_arch = "wasm32"))]
    let prof = env_flag("VIEWER_PROFILE", &VIEWER_PROFILE);
    #[cfg(not(target_arch = "wasm32"))]
    let mut lap = std::time::Instant::now();
    #[cfg(not(target_arch = "wasm32"))]
    let mut mark = |name: &str, lap: &mut std::time::Instant| {
        if prof { eprintln!("  push_mesh {name:<20} {:?}", lap.elapsed()); *lap = std::time::Instant::now(); }
    };
    // Same signature on wasm so every `mark(..)` call site below stays identical.
    #[cfg(target_arch = "wasm32")]
    let mut lap = ();
    #[cfg(target_arch = "wasm32")]
    let mut mark = |_name: &str, _lap: &mut ()| {};
    let rm = m.to_render();
    mark("to_render", &mut lap);

    // The mesh-local AABB rides the object row, so the edge lanes can be told "the eye is inside
    // this solid" (Instance::FLAG_INSIDE) - the facing cull's premise, both faces away = hidden,
    // is only valid for an eye OUTSIDE. Computed even when the wireframe below is skipped: the
    // flag costs nothing and the lanes ignore it when there are no edges.
    let mut lo = [f32::INFINITY; 3];
    let mut hi = [f32::NEG_INFINITY; 3];
    for v in &rm.vertices{
        grow_bounds(&mut lo, &mut hi, v.position);
        verts.push(*v);
        vids.push(ri);
    }
    let local_bounds = if lo[0] <= hi[0] { Some((lo, hi)) } else { None };
    for &i in &rm.indices{
        idx.push(base+i);
    }
    mark("vert+idx push", &mut lap);

    // A DENSE mesh gets no wireframe and no vertex dots. This is the same call the cloud lane
    // makes at CLOUD_RAW_MIN, and for the same reason - decoration that is free on a CAD solid
    // is ruinous on a scan.
    //
    // Measured on the Stanford ladder (1.29M mesh triangles): the per-edge cylinders and
    // per-vertex spheres added 23.2M and 92.9M triangles respectively - 90x the geometry they
    // were decorating - and 118 MB of segment/glyph tables against 25 MB of actual mesh arena.
    // The walk cost 12.4 s, most of it in edges_with_colors() building 1.9M edges and a HashSet.
    //
    // Selection is NOT affected: picking a vertex, an edge or a whole mesh reads the kernel
    // Mesh (positions, indices, BVH), never these drawn tubes and dots. When a dense mesh is
    // selected, its wireframe can be emitted for that one mesh on demand.
    if rm.indices.len() / 3 > MESH_RAW_MIN {
        return None;
    }

    // Edge width 0 = hidden wireframe, A mesh only has explicit widths if someone called
    // set_linecolors, so the 1.0 default below leaves every ordinary mesh untouched - but a triangulated PDF
    // fill (a letter, a pocket region) ask for no wireframe at all, and without
    // this every glyph would render outlined in tubes and dotted at each vertex.
    // A single width broadcasts to every edge - one entry instead of one per edge, which for
    // thousands of small glyph meshes is the difference between a lean .pb and a fat one.
    let width_at = |i: usize| -> f64 {
        let w = m.widths();
        if w.len() == 1 {
            w[0]
        } else {
            w.get(i).copied().unwrap_or(1.0)
        }
    };

    let hidden = |i: usize| width_at(i) == 0.0;

    // A fill (every PDF glyph, every poché region) broadcasts a single width of 0 - no wireframe
    // at all. Leave before edges_with_colors, which builds a HashSet over the faces: for sheets
    // made of hundreds of thousands of tiny fills, that set was the walk's biggest single cost
    // and every edge it produced was then skipped.
    if is_print_fill(m) { return None }

    if env_flag("VIEWER_NO_EDGES", &VIEWER_NO_EDGES) { return None }

    // ONE edge walk, shared by the pipes below and the vertex widths further down.
    let edges = m.edges_with_colors();
    mark("edges_with_colors", &mut lap);

    // Face normals once for the whole mesh, so the per-edge adjacency lookup below is two map
    // reads instead of a cross product each time. These are MESH-LOCAL, matching p0/p1 - the
    // shader rotates them by the instance model the same way it transforms the endpoints.
    let fnormals = m.face_normals();
    mark("face_normals", &mut lap);

    // Vertex keys are arbitrary usizes; everything below indexes by SLOT, the key's position in
    // the sorted order m.vertices() emits. One map build here replaces ~250k kernel map lookups
    // over the three passes below.
    // Vertex keys are arbitrary usizes, but in practice they are dense ids: a Vec indexed BY
    // KEY (u32::MAX sentinel) turns every key->slot lookup below into an array read, where a
    // HashMap trades the same cost the kernel's vertex_point was just removed for. Sparse key
    // spaces (a mesh after deletions) fall back to the map.
    let keys = m.vertices();
    let max_key = keys.last().copied().unwrap_or(0);
    let dense = max_key < 4 * keys.len().max(1);
    let mut slot_vec: Vec<u32> = Vec::new();
    let mut slot_map: std::collections::HashMap<usize, u32> = std::collections::HashMap::new();
    if dense {
        slot_vec = vec![u32::MAX; max_key + 1];
        for (s, &k) in keys.iter().enumerate() { slot_vec[k] = s as u32; }
    } else {
        slot_map = keys.iter().enumerate().map(|(s, &k)| (k, s as u32)).collect();
    }
    let slot = |k: usize| -> usize {
        if dense { slot_vec[k] as usize } else { slot_map[&k] as usize }
    };

    // Positions by slot, from the KERNEL's vertex map - one lookup per vertex instead of two
    // per edge. NOT rm.vertices[slot]: to_render DUPLICATES vertices for per-face colors, so
    // its rows are in vertex order only when no duplication happens (the colors_widths boxes
    // are exactly the case where it does).
    let vpos: Vec<[f32; 3]> = keys.iter().map(|&k| m.vertex_point(k).unwrap().to_f32()).collect();

    // Each edge's adjacent faces, kept for the dots pass: edge_faces allocates a Vec per call,
    // and the dots used to repeat it per incident edge per vertex - the walk's biggest cost.
    // Hidden edges contribute faces too (their face can still carry a visible band), so the
    // call happens even when the segment below is skipped.
    let mut edge_faces: Vec<Vec<usize>> = Vec::with_capacity(edges.len());

    // A DENSE wireframe draws BLACK, whatever linecolors the file carries: at scan density an
    // edge's color is a property of the tessellation, not of the model, and per-edge pens
    // stopped being readable thousands of edges ago. Authored colors on ordinary meshes (the
    // red pen box) are always honored - the gate sits far above any CAD part.
    let black_wire = edges.len() >= WIREFRAME_BLACK_MIN;

    for (i, (a, b, col)) in edges.iter().cloned().enumerate(){
        let f = m.edge_faces(a, b).unwrap_or_default();

        // The two faces sharing this edge, so the shader can decide whether it faces the camera
        // without asking the depth buffer. An edge with both faces turned away is HIDDEN and the
        // shader drops it; one that keeps only one is a silhouette. This is the whole point of the
        // exercise: a pen has width, so ink tested against the surface it decorates either gets
        // cut by it or has to float in front of it, and no offset wins at every slant. Deciding
        // visibility from the geometry instead sidesteps the trade entirely.
        let facing = pack_facing(
            f.first().and_then(|&k| fnormals.get(&k).cloned()),
            f.get(1).and_then(|&k| fnormals.get(&k).cloned()),
        );
        edge_faces.push(f);

        if hidden(i) {
            continue
        }
        segments.push(
            CylinderSegment{
                p0: vpos[slot(a)],
                radius: encode_width(width_at(i)),
                p1: vpos[slot(b)],
                instance_id: ri,
                color: if black_wire { pack_rgba([0.0, 0.0, 0.0, 1.0]) } else { pack_rgba(col.to_f32()) },
                facing,
            }
        )
    }
    mark("pipe loop", &mut lap);

    // Dots are used for user set pointcolors.
    // The auto-seeded white vec is filtered by the mode gate.
    // m.vertices() is sorted  - the same order to_render indexes pointcolors by.
    let pc = m.get_pointcolors();
    let dots_colored = m.color_mode == ColorMode::POINTCOLORS && pc.len() == m.number_of_vertices();

    // A vertex sphere must be as fas as the pipes.
    // The kernel has no per-vertex width, so take the widest incident edge - and remember WHICH
    // edge it was. The dot inherits that edge's pen color and leads its `facing` adjacency, so
    // the sphere lane hugs the faces the bands meeting at the vertex already hug: a marker
    // floating on the old constant lift loses the depth test to its own hugged bands over most
    // of its disc at close zoom, and shows up as a lopsided chunk smaller than the band width.
    // Same two passes, by slot instead of by key: vbest as a flat Vec (sentinel -inf = no
    // visible edge yet; widths can be NEGATIVE world-mm radii, so the sentinel is not 0) and
    // the incident list as CSR (degree count, prefix sum, fill) instead of a Vec per vertex.
    let mut vbest = vec![(f64::NEG_INFINITY, 0usize); keys.len()];
    for (i, (a, b, _)) in edges.iter().cloned().enumerate(){
        if hidden(i){ // A vertex whose every edge is hidden gets no dot either
            continue;
        }
        let w = width_at(i);
        for vk in [a, b] {
            let e = &mut vbest[slot(vk)];
            if w > e.0 {
                *e = (w, i);
            }
        }
    }

    // Incident EDGES per vertex, for the face list below. Hidden edges contribute faces too:
    // a hidden edge's adjacent face can still carry a visible band from another edge, and the
    // dot must hug that face to stay in front of it.
    let mut vstart = vec![0u32; keys.len() + 1];
    for (a, b, _) in edges.iter(){
        vstart[slot(*a) + 1] += 1;
        vstart[slot(*b) + 1] += 1;
    }
    for i in 0..keys.len(){
        vstart[i + 1] += vstart[i];
    }
    let mut vinc = vec![0u32; 2 * edges.len()];
    let mut cur = vstart.clone();
    for (i, (a, b, _)) in edges.iter().enumerate(){
        for vk in [*a, *b] {
            let s = slot(vk);
            vinc[cur[s] as usize] = i as u32;
            cur[s] += 1;
        }
    }
    mark("vbest+vedges", &mut lap);

    // VIEWER_NO_DOTS drops the per-vertex dots, so the harness can tell how much of a dense
    // wireframe's ink is dots and how much is edges.
    if env_flag("VIEWER_NO_DOTS", &VIEWER_NO_DOTS) { return local_bounds }

    // Widest edge's faces first, then every other incident edge's, deduped - one reused Vec,
    // and the face lists cached from the pipe pass instead of a kernel call per incident edge.
    let mut fkeys: Vec<usize> = Vec::new();
    let mut codes: Vec<u32> = Vec::new();
    for i in 0..keys.len(){
        let (vw, ei) = vbest[i];
        if vw == f64::NEG_INFINITY { continue }

        // Face keys, widest edge's pair first, then every other incident edge's, deduped. The
        // row carries up to SIX normals (3 words x oct16 pair): a trihedral corner needs three,
        // and hugging only the widest edge's two leaves the third face's band able to bite a
        // sector out of the disc at grazing slants - the marker is meant to go in front.
        fkeys.clear();
        let take = |ei: usize, fkeys: &mut Vec<usize>| {
            for &fk in &edge_faces[ei] {
                if !fkeys.contains(&fk) { fkeys.push(fk); }
            }
        };
        take(ei, &mut fkeys);
        for &j in &vinc[vstart[i] as usize..vstart[i + 1] as usize] {
            take(j as usize, &mut fkeys);
        }
        codes.clear();
        codes.extend(
            fkeys.iter()
                .filter_map(|fk| fnormals.get(fk))
                .filter_map(oct16)
                .take(6),
        );
        // pack_facing's rules: a lone normal is duplicated, none at all is FACING_UNKNOWN, and a
        // pair colliding with the all-ones sentinel collapses to it (accepted loss, same as edges).
        let word = |k: usize| -> u32 {
            match (codes.get(2 * k).copied(), codes.get(2 * k + 1).copied()) {
                (Some(a), b) => {
                    let v = a | b.unwrap_or(a) << 16;
                    if v == FACING_UNKNOWN { FACING_UNKNOWN } else { v }
                }
                _ => FACING_UNKNOWN,
            }
        };
        glyphs.push(
            GlyphPoint {
                center: vpos[i],
                radius: encode_width(vw),
                // No pointcolors -> fixed near-black marker, whatever the pen color is: the dot
                // must read as a DOT so the joint can be checked by eye (following the pen color
                // hid the marker exactly where checking happens - black on a black-penned cube).
                color: if dots_colored { pc[i].to_f32() } else { [0.1, 0.1, 0.1, 1.0] },
                instance_id: ri,
                facing: word(0),
                facing_ext: [word(1), word(2)],
            }
        );
    }
    mark("dots loop", &mut lap);
    local_bounds
}

pub fn xform_point(xf: &Xform, p: [f32; 3]) -> [f32; 3] {
    let x = p[0] as f64;
    let y = p[1] as f64;
    let z = p[2] as f64;
    [
        (xf.m[0] * x + xf.m[4] * y + xf.m[8] * z + xf.m[12]) as f32,
        (xf.m[1] * x + xf.m[5] * y + xf.m[9] * z + xf.m[13]) as f32,
        (xf.m[2] * x + xf.m[6] * y + xf.m[10] * z + xf.m[14]) as f32,
    ]
}

fn grow_bounds(min: &mut [f32; 3], max: &mut [f32; 3], p: [f32; 3]) {
    for k in 0..3 {
        min[k] = min[k].min(p[k]);
        max[k] = max[k].max(p[k]);
    }
}


/// A plane is infinite - draw a fix sqzare around its origin, spanned by its x/y axes
/// Half-extent in world mm (a 1 m quare)
const PLANE_SIZE: f64 = 500.0;

fn plane_to_segments(pl: &Plane, instance_id: u32) -> Vec<CylinderSegment> {
    let (o, x, y) = (pl.origin(), pl.x_axis(), pl.y_axis());
    let corner = |sx: f64, sy: f64| -> [f32; 3]{
         [0usize, 1, 2].map(|k| (o[k] + (x[k] * sx + y[k] * sy) * PLANE_SIZE) as f32)
    };
    let c = [corner(1.0, 1.0), corner(-1.0, 1.0), corner(-1.0, -1.0), corner(1.0, -1.0)];
    let color = pack_rgba(pl.linecolor.to_f32());
    let radius = encode_width(pl.width);
    (0..4).map(|i| CylinderSegment { p0:c[i], radius, p1: c[(i+1) % 4], instance_id, color, facing: FACING_UNKNOWN }).collect()
}

/// A box is its 12 edges: bottom loop, top loop, four verticals - `corner()` orders tge bottom face
/// face 0-3 and the top 4-7 with i / i+4 vertically aligned.
/// The OBB type carries no pen, so the edges draw black at screen-constant width (radius 0.0 = global default)
fn obb_to_segments(b: &OBB, instance_id: u32) -> Vec<CylinderSegment>{
    const EDGES: [[usize; 2]; 12] = [
        [0, 1], 
        [1, 2], 
        [2, 3], 
        [3, 0],
        [4, 5], 
        [5, 6], 
        [6, 7], 
        [7, 4], 
        [0, 4], 
        [1, 5], 
        [2, 6], 
        [3, 7]
    ];

    let c = b.corners_f32();
    EDGES.iter().map(|&[i, j]| CylinderSegment { p0: c[i], radius: 0.0, p1: c[j], instance_id, color: pack_rgba([0.0, 0.0, 0.0, 1.0]), facing: FACING_UNKNOWN }).collect()
    
}

/// Above this many triangles a mesh draws as TRIANGLES ONLY - no per-edge cylinder, no
/// per-vertex sphere. Below it, the wireframe and vertex dots are what make a CAD solid
/// readable. At 200k the bunny (69k tri) keeps its wireframe and the armadillo and dragon
/// do not - which is the honest line until an impostor makes the decoration cheap. A PDF fill (tens of triangles) and a
/// demo box (12) stay decorated; a scan does not.
const MESH_RAW_MIN: usize = 200_000;

/// At or above this many edges a mesh's wireframe draws BLACK whatever the file says - see
/// push_mesh. 104,288 on the bunny; 12 on a box, whose authored red pen always survives.
const WIREFRAME_BLACK_MIN: usize = 10_000;

/// The raw lane's rows, written straight into the shared table,
/// reading the kernel's flat arrays rather than get_point/get_color (no per_point allocs)
fn push_cloud(pc: &PointCloud, pos: &mut Vec<f32>, col: &mut Vec<u32>, nrm: &mut Vec<u32>){
    let coords = pc.coords();
    let colors = pc.colors();
    let normals = pc.normals();
    let n = pc.len();
    pos.reserve(n*3);
    col.reserve(n);
    nrm.reserve(n);
    for i in 0..n{
        pos.push(coords[i*3] as f32);
        pos.push(coords[i*3+1] as f32);
        pos.push(coords[i*3+2] as f32);

        // Normal, oct16-packed into 16 bits
        // All-ones = this point has nor normal: a scan without them still pays the 4 B,
        // but the shading branch stays uniform per cloud, which is what the GPU wants.
        nrm.push(if i*3 + 2 < normals.len() {
            let v = Vector::new(normals[i*3],normals[i*3+1],normals[i*3+2]);
            oct16(&v).unwrap_or(u32::MAX)
        } else {
            u32::MAX
        });
        let c = i * 4;

        // The colour is 8-bit at the source (proto 0-255):
        // pack it back to the four bytes it is, instrad of four f32s carying four bytes of information
        col.push(if c + 3 < colors.len() {
            (colors[c] as u32 & 255) | (colors[c + 1] as u32 & 255) << 8 | (colors[c+2] as u32 & 255) << 16 | (colors[c + 3] as u32 & 255) << 24
        } else {
            0xff00_0000
        });
    }
   
}

/// Median distance between consecutive points - a scanner emits angular neighbours in order,
/// so successive points are usually adjacent on the surface, which makes this a cheap and
/// honest estimate of the clouds's point spacing (world units). 
/// Potree gets the same number from its octree, we sample it.
/// Drives the attenuated world-sized splat radius.
fn cloud_spacing(pc: &PointCloud) -> f32{
    let c = pc.coords();
    let n = pc.len();
    if n < 2 {
        return 20.0;
    }
    let step = (n / 1024).max(1);
    let mut d: Vec<f64> = Vec::with_capacity(1024);
    let mut i = 0;
    while i + 1 < n {
        let  (a, b) = (i * 3, (i + 1) * 3);
        let dd = (c[a] - c[b]).powi(2) + (c[a + 1] - c[b + 1]).powi(2) + (c[a + 2] - c[b + 2]).powi(2);
        if dd> 0.0 {
            d.push(dd.sqrt());
        }
        i += step;
    }
    if d.is_empty() {
        return 20.0;
    }
    d.sort_by(|x, y| x.partial_cmp(y).unwrap());
    d[d.len() / 2] as f32
}