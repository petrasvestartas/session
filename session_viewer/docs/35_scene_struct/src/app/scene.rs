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
    pub fn parse(bytes: &[u8]) -> Option<Self> {
        serde_json::from_slice(bytes).ok()
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
use crate::engine::gpu::{ArenaUpload, Instance, CylinderSegment, GlyphPoint, CloudDraw};

/// One loaded file: the kernel `Session`
/// kept alive - picking/undo/save need this
/// plus the placement the manifest gave it
pub struct Doc{
    pub name: String,
    pub place: Xform,
    pub session: Session,
}

/// The open document set + the merged GPU tables.
/// `add_file` walks one new session straight into the shared tables
/// rows are appended, never rebuilt
/// It means the progressive loading costs each file only its own walk.
/// Viewer-only bookkeeping (row order, grid to rowm hidden) lives here
/// It never in the kernel type that threee languages share.

/// A cloud whose points never became kernel objects: the loader streamed them from the file
/// straight into GPU memory. This struct is the ENTIRE CPU-side footprint of a 13.8M-point
/// scan - a name, a placement, a count, and the instance row it draws with.
pub struct CloudSlot {
    pub name: String,
    pub place: Xform,
    pub count: u32,
    pub instance: u32,
}

pub struct Scene {
    pub docs: Vec<Doc>,
    pub clouds: Vec<CloudSlot>, // streamed clouds - no Doc, no Session, no points on the CPU
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
        clouds: Vec::new(),
        vert_base: 0,
        tables: ArenaUpload::new(),
        order: Vec::new(),
        guid_to_row: HashMap::new(),
        hidden: HashSet::new(),
        }
    }

    /// Widen the shared walk box by a streamed cloud's world AABB. Without this the box lives
    /// only in `Gpu` and the next `set_scene` from a real walk would replace it.
    pub fn grow_bounds(&mut self, min: [f32; 3], max: [f32; 3]) {
        for k in 0..3 {
            self.tables.min[k] = self.tables.min[k].min(min[k]);
            self.tables.max[k] = self.tables.max[k].max(max[k]);
        }
    }

    /// Reserve the document row for a cloud that is about to stream in. Called before a single
    /// point has been fetched: the count comes from the file's packed-double length prefix.
    /// Returns the instance row the streamed points will draw against.
    pub fn begin_cloud(&mut self, name: String, place: Xform, count: u32) -> u32 {
        let row = self.tables.objects.len() as u32;
        self.tables.objects.push((place.clone(), [1.0; 4], 0));
        // Keep the row bookkeeping aligned - `order` is indexed by row everywhere else.
        let guid = format!("cloud:{name}");
        self.guid_to_row.insert(guid.clone(), row);
        self.order.push(guid);
        self.clouds.push(CloudSlot { name, place, count, instance: row });
        row
    }

    /// Re-flatten EVERY document from its kernel `Session` and re-upload from scratch.
    ///
    /// The blunt instrument, and the reason it exists: once `upload_to` clears the arena
    /// mirror, there is no CPU copy left to patch, so changing an object's GEOMETRY (dragging
    /// a polyline vertex, editing a mesh) has nothing to rewrite. Per-object arena ranges are
    /// what fixes that properly - the planned `guid -> range` map - and until then this gets
    /// the same result by redoing the walk. It costs a full re-walk, so it belongs behind an
    /// edit commit, not behind a drag.
    ///
    /// Streamed clouds are NOT re-walked: their points exist only on the GPU and are still
    /// there. Only their instance row is re-issued, because rebuilding shifts every row index.
    pub fn rebuild(&mut self, gpu: &mut crate::engine::gpu::Gpu) {
        let docs = std::mem::take(&mut self.docs);
        let clouds = std::mem::take(&mut self.clouds);
        self.tables = ArenaUpload::new();
        self.order.clear();
        self.guid_to_row.clear();
        self.vert_base = 0;
        gpu.reset_arena();

        for d in docs {
            self.add_file(d.name, d.session, d.place);
        }
        // Clouds keep their GPU rows; only the instance they draw against is re-issued, and the
        // Gpu's draw list is patched to match. Order is preserved on both sides, so index i
        // here is index i there.
        for (i, c) in clouds.into_iter().enumerate() {
            let row = self.begin_cloud(c.name, c.place, c.count);
            if let Some(d) = gpu.cloud_draws.get_mut(i) {
                d.instance = row;
            }
        }
        self.upload_to(gpu);
    }

    /// Upload, then FORGET the cloud rows. The GPU is now the only holder of those points.
    /// Only `points` is cleared - the other lanes are still uploaded cumulatively, because
    /// only the point lane has an append path (Gpu::set_scene).
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
        self.tables.cloud_pos.clear();
        self.tables.cloud_pos.shrink_to_fit();
        self.tables.cloud_col.clear();
        self.tables.cloud_col.shrink_to_fit();
        self.tables.clouds.clear();
    }

    /// Walk one session into the shared tables.
    /// We moved out of GPU struct:
    /// - placement = manifest `place` x the session's own `world_xforms()` one downward pass per object `world_xform()` rescans the the tree each call
    /// - `session_order()` is the kernel's canonical order, deterministics across runs and languages - the row a guid gets here in the row it keeps (picking/selection rely on it)
    /// - per file planar test: a sheet flat along its own normal (place + orientation)
    pub fn add_file(&mut self, name: String, session: Session, place: Xform){

        let seg0 = self.tables.segments.len();
        let pipe0 = self.tables.pipes.len();
        let vert0 = self.tables.verts.len();
        let vb = self.vert_base; // read before `t` borrows self.tables
        let sphere0 = self.tables.spheres.len();
        let glyph0 = self.tables.glyphs.len();
        let cloud0 = self.tables.clouds.len();
        let obj0 = self.tables.objects.len();

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
                // A cloud picks its lane by SIZE, not by camera state - so nothing changes while
                // you orbit. A handful of points are worth round sized dots (32b's demo clouds);
                // a scan is a clump, and the raw lane draws it one vertex and one pixel per point.
                Geometry::PointCloud(pc) if pc.len() >= CLOUD_RAW_MIN => {
                    push_cloud(pc, ri, t);
                    t.object_bounds.push(None); t.object_spacing.push(0.0);
                }
                Geometry::PointCloud(pc) => { t.glyphs.extend(pointcloud_to_glyphs(pc, ri)); t.object_bounds.push(None); t.object_spacing.push(0.0); }
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

        for ci in cloud0..t.clouds.len(){
            let c = t.clouds[ci];
            if let Some((xf, _, _)) = t.objects.get(c.instance as usize){
                for i in c.base as usize..(c.base + c.count) as usize {
                    let p = [t.cloud_pos[i * 3], t.cloud_pos[i * 3 + 1], t.cloud_pos[i * 3 + 2]];
                    grow_bounds(&mut fmin, &mut fmax, xform_point(xf, p));
                }
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
            for ci in cloud0..t.clouds.len(){
                let c = t.clouds[ci];
                if let Some((xf, _, _)) = t.objects.get(c.instance as usize) {
                    for i in c.base as usize..(c.base + c.count) as usize {
                        let p = [t.cloud_pos[i * 3], t.cloud_pos[i * 3 + 1], t.cloud_pos[i * 3 + 2]];
                        span(xform_point(xf, p));
                    }
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
            session
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
    let rm = m.to_render();

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
    if m.widths().len() == 1 && m.widths()[0] == 0.0 { return None }

    if std::env::var("VIEWER_NO_EDGES").is_ok() { return None }

    // ONE edge walk, shared by the pipes below and the vertex widths further down.
    let edges = m.edges_with_colors();

    // Face normals once for the whole mesh, so the per-edge adjacency lookup below is two map
    // reads instead of a cross product each time. These are MESH-LOCAL, matching p0/p1 - the
    // shader rotates them by the instance model the same way it transforms the endpoints.
    let fnormals = m.face_normals();

    for (i, (a, b, col)) in edges.iter().cloned().enumerate(){
        if hidden(i) {
            continue
        }
        let pa = m.vertex_point(a).unwrap();
        let pb = m.vertex_point(b).unwrap();

        // The two faces sharing this edge, so the shader can decide whether it faces the camera
        // without asking the depth buffer. An edge with both faces turned away is HIDDEN and the
        // shader drops it; one that keeps only one is a silhouette. This is the whole point of the
        // exercise: a pen has width, so ink tested against the surface it decorates either gets
        // cut by it or has to float in front of it, and no offset wins at every slant. Deciding
        // visibility from the geometry instead sidesteps the trade entirely.
        let f = m.edge_faces(a, b).unwrap_or_default();
        let facing = pack_facing(
            f.first().and_then(|&k| fnormals.get(&k).cloned()),
            f.get(1).and_then(|&k| fnormals.get(&k).cloned()),
        );

        segments.push(
            CylinderSegment{
                p0: pa.to_f32(),
                radius: encode_width(width_at(i)),
                p1: pb.to_f32(),
                instance_id: ri,
                color: pack_rgba(col.to_f32()),
                facing,
            }
        )
    }

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
    let mut vbest: std::collections::HashMap<usize, (f64, usize)> = std::collections::HashMap::new();
    for (i, (a, b, _)) in edges.iter().cloned().enumerate(){
        if hidden(i){ // A vertex whose every edge is hidden gets no dot either
            continue;
        }
        let w = width_at(i);
        for vk in [a, b] {
            let e = vbest.entry(vk).or_insert((w, i));
            if w > e.0 {
                *e = (w, i);
            }
        }
    }

    // Incident EDGES per vertex, for the face list below. Hidden edges contribute faces too:
    // a hidden edge's adjacent face can still carry a visible band from another edge, and the
    // dot must hug that face to stay in front of it.
    let mut vedges: std::collections::HashMap<usize, Vec<usize>> = std::collections::HashMap::new();
    for (i, (a, b, _)) in edges.iter().enumerate(){
        vedges.entry(*a).or_default().push(i);
        vedges.entry(*b).or_default().push(i);
    }

    // VIEWER_NO_DOTS drops the per-vertex dots, so the harness can tell how much of a dense
    // wireframe's ink is dots and how much is edges.
    if std::env::var("VIEWER_NO_DOTS").is_ok() { return local_bounds }

    for (i, vk) in m.vertices().into_iter().enumerate(){
        let Some(&(vw, ei)) = vbest.get(&vk) else { continue };
        let p = m.vertex_point(vk).unwrap();

        // Face keys, widest edge's pair first, then every other incident edge's, deduped. The
        // row carries up to SIX normals (3 words x oct16 pair): a trihedral corner needs three,
        // and hugging only the widest edge's two leaves the third face's band able to bite a
        // sector out of the disc at grazing slants - the marker is meant to go in front.
        let mut fkeys: Vec<usize> = Vec::new();
        let mut take = |ei: usize, fkeys: &mut Vec<usize>| {
            let (a, b, _) = edges[ei].clone();
            for fk in m.edge_faces(a, b).unwrap_or_default() {
                if !fkeys.contains(&fk) { fkeys.push(fk); }
            }
        };
        take(ei, &mut fkeys);
        if let Some(incident) = vedges.get(&vk) {
            for &j in incident { take(j, &mut fkeys); }
        }
        let codes: Vec<u32> = fkeys.iter()
            .filter_map(|fk| fnormals.get(fk))
            .filter_map(oct16)
            .take(6)
            .collect();
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
                center: p.to_f32(),
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

/// One glyph per point. `point_size` rides the same width encoding as every other pen, and
/// a cloud with fewer colors than points falls back to black for the tail.
/// Above this many points a cloud stops being decorated dots and becomes a raw clump: one
/// vertex, one pixel, opaque. Below it the sized round dots of the glyph lane still read better,
/// and 100k of them is a frame cost nobody notices.
const CLOUD_RAW_MIN: usize = 100_000;

/// Above this many triangles a mesh draws as TRIANGLES ONLY - no per-edge cylinder, no
/// per-vertex sphere. Below it, the wireframe and vertex dots are what make a CAD solid
/// readable. At 200k the bunny (69k tri) keeps its wireframe and the armadillo and dragon
/// do not - which is the honest line until an impostor makes the decoration cheap. A PDF fill (tens of triangles) and a
/// demo box (12) stay decorated; a scan does not.
const MESH_RAW_MIN: usize = 200_000;

/// The raw lane's rows. Same walk as the glyph version, minus the radius - a cloud has no pen
/// per point - and 32 B per row instead of 48.
///
/// It writes STRAIGHT into the shared table instead of collecting a Vec the caller then extends:
/// `Vec::extend` from an owned iterator always memcpies into the destination and drops the
/// source, so a 13.8M-point scan built the same 441 MB table twice and peaked at 843 MB against
/// a heap that practically ends around 2 GB. Reserving once and pushing peaks at 423 MB.
///
/// It also reads the kernel's FLAT arrays rather than `get_point`/`get_color`, which each build a
/// `Point`/`Color` - three String allocations per point, measured at 1.08 s against 0.24 s for
/// the flat walk on this scan, all of it allocator churn on the wasm main thread.
fn push_cloud(pc: &PointCloud, instance_id: u32, t: &mut ArenaUpload){
    let coords = pc.coords();
    let colors = pc.colors();
    let n = pc.len();
    let base = (t.cloud_pos.len() / 3) as u32;
    t.cloud_pos.reserve_exact(n * 3);
    t.cloud_col.reserve_exact(n);
    for i in 0..n {
        t.cloud_pos.push(coords[i * 3] as f32);
        t.cloud_pos.push(coords[i * 3 + 1] as f32);
        t.cloud_pos.push(coords[i * 3 + 2] as f32);
        let c = i * 4;
        // RGBA8 in one u32, little-endian byte order so the shader's unpack4x8unorm reads
        // x=r y=g b=b w=a. 4 B a point instead of four f32s: the colour is 8-bit at the source
        // (the proto carries 0-255) and it was being widened to 16 B for nothing.
        t.cloud_col.push(if c + 3 < colors.len() {
            ((colors[c] as u32) & 255)
                | (((colors[c + 1] as u32) & 255) << 8)
                | (((colors[c + 2] as u32) & 255) << 16)
                | (((colors[c + 3] as u32) & 255) << 24)
        } else {
            0xff00_0000
        });
    }
    t.clouds.push(CloudDraw { base, count: n as u32, instance: instance_id });
}

fn pointcloud_to_glyphs(pc: &PointCloud, instance_id: u32) -> Vec<GlyphPoint>{
    let radius = encode_width(pc.point_size);
    let colors = pc.color_count();
    (0..pc.len()).map(|i| GlyphPoint{
        center: pc.get_point(i).to_f32(),
        radius,
        color: if i < colors {pc.get_color(i).to_f32()} else { [0.0, 0.0, 0.0, 1.0]},
        instance_id,
        facing: FACING_UNKNOWN, // a cloud point has no surface to hug
        facing_ext: [FACING_UNKNOWN; 2],
    }).collect()
}