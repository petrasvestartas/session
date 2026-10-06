//! Congruent BReps share one walk: a floor carries hundreds of identical dowels and screws, each
//! moved rigidly, and meshing every one of them again cost most of the walk. The first BRep of a
//! shape is walked and its rows are kept; every later copy gets those rows turned into place.

use super::brep_edges::FacingPair;
use super::encode::pack_facing;
use super::mesh::mesh_spacing;
use super::mesh_ink::Ink;
use super::{Row, WalkCx};
use crate::engine::gpu::CylinderSegment;
use crate::engine::gpu::arena::{ArenaRows, Sample};
use crate::engine::gpu::faces::FaceSource;
use session_rust::brep::BRepRef;
use session_rust::{AABB, BRep, Color, NurbsCurve, RenderVertex};
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Range;

/// Control points this close, relative to the shape's size, count as the same point.
const SAME_POINT: f64 = 1e-6;

/// Weights and knots this close, relative to one or to the knot span, count as the same.
const SAME_NUMBER: f64 = 1e-9;

/// A point at least this far from the first one, relative to the shape's size, sets an axis.
const AXIS_SPAN: f64 = 1e-3;

/// A rigid frame of a BRep, found from its own control points: origin and three unit axes.
#[derive(Clone, Copy)]
pub struct Frame {
    origin: [f64; 3],    // the first surface control point
    axes: [[f64; 3]; 3], // x, y, z as rows, right-handed
}

impl Frame {
    /// Coordinates of `p` along the axes.
    fn local(&self, p: [f64; 3]) -> [f64; 3] {
        let d = sub(p, self.origin);
        [
            dot(d, self.axes[0]),
            dot(d, self.axes[1]),
            dot(d, self.axes[2]),
        ]
    }

    /// The motion that carries a shape placed at `self` onto the same shape placed at `to`.
    fn motion_to(&self, to: &Frame) -> Motion {
        let mut turn = [[0.0; 3]; 3];

        for (i, row) in turn.iter_mut().enumerate() {
            for (j, value) in row.iter_mut().enumerate() {
                *value = (0..3).map(|k| to.axes[k][i] * self.axes[k][j]).sum();
            }
        }

        Motion {
            from: self.origin,
            to: to.origin,
            turn,
        }
    }
}

/// A rotation about `from` followed by the move to `to`.
struct Motion {
    from: [f64; 3],      // origin of the walked shape
    to: [f64; 3],        // origin of the copy
    turn: [[f64; 3]; 3], // rotation matrix
}

impl Motion {
    /// A direction turned.
    fn turn(&self, v: [f64; 3]) -> [f64; 3] {
        [
            dot(self.turn[0], v),
            dot(self.turn[1], v),
            dot(self.turn[2], v),
        ]
    }

    /// A point moved.
    fn point(&self, p: [f64; 3]) -> [f64; 3] {
        let t = self.turn(sub(p, self.from));
        [t[0] + self.to[0], t[1] + self.to[1], t[2] + self.to[2]]
    }

    /// A render position moved.
    fn position(&self, p: [f32; 3]) -> [f32; 3] {
        let q = self.point([p[0] as f64, p[1] as f64, p[2] as f64]);
        [q[0] as f32, q[1] as f32, q[2] as f32]
    }

    /// A render normal turned.
    fn normal(&self, n: [f32; 3]) -> [f32; 3] {
        let t = self.turn([n[0] as f64, n[1] as f64, n[2] as f64]);
        [t[0] as f32, t[1] as f32, t[2] as f32]
    }
}

fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn unit(a: [f64; 3]) -> [f64; 3] {
    let length = dot(a, a).sqrt();
    [a[0] / length, a[1] / length, a[2] / length]
}

/// Every surface control point of a BRep, in storage order.
fn surface_points(b: &BRep) -> Vec<[f64; 3]> {
    let mut points = Vec::new();

    for surface in &b.m_surfaces {
        for i in 0..surface.cv_count(0) {
            for j in 0..surface.cv_count(1) {
                let p = surface.get_cv(i, j).unwrap_or_default();
                points.push([p[0], p[1], p[2]]);
            }
        }
    }

    points
}

/// The frame of a BRep and its size: the first control point, the first one clearly away from
/// it for x, the first one clearly off that line for z. None for a flat line or nothing.
fn frame_of(points: &[[f64; 3]]) -> Option<(Frame, f64)> {
    let origin = *points.first()?;
    let size = points
        .iter()
        .map(|p| dot(sub(*p, origin), sub(*p, origin)).sqrt())
        .fold(0.0, f64::max);

    if !(size.is_finite() && size > 0.0) {
        return None;
    }

    let x = points
        .iter()
        .map(|p| sub(*p, origin))
        .find(|d| dot(*d, *d).sqrt() > AXIS_SPAN * size)?;
    let x = unit(x);
    let z = points
        .iter()
        .map(|p| cross(x, sub(*p, origin)))
        .find(|c| dot(*c, *c).sqrt() > AXIS_SPAN * size)?;
    let z = unit(z);
    let y = cross(z, x);
    let frame = Frame {
        origin,
        axes: [x, y, z],
    };

    Some((frame, size))
}

/// What makes two BReps the same shape: their topology, parameters, weights, colours and widths
/// exactly, and their 3D points in their own frames up to `SAME_POINT`.
struct ShapeKey {
    words: Vec<u64>, // the shape, word by word
    frame: Frame,    // where this BRep lies
    step: f64,       // the length one point word counts
}

impl ShapeKey {
    /// The key of `b`; None when it has no frame.
    fn of(b: &BRep) -> Option<Self> {
        let (frame, size) = frame_of(&surface_points(b))?;
        let mut key = Self {
            words: Vec::new(),
            frame,
            step: size * SAME_POINT,
        };
        key.push_counts(b);
        key.push_float(b.width);
        key.push_color(&b.surfacecolor);

        for surface in &b.m_surfaces {
            key.push_words(&[
                surface.m_dim as u64,
                surface.m_is_rat as u64,
                surface.m_order[0] as u64,
                surface.m_order[1] as u64,
                surface.m_cv_count[0] as u64,
                surface.m_cv_count[1] as u64,
            ]);
            key.push_knots(&surface.m_nurbsknot[0]);
            key.push_knots(&surface.m_nurbsknot[1]);

            for i in 0..surface.cv_count(0) {
                for j in 0..surface.cv_count(1) {
                    let p = surface.get_cv(i, j).unwrap_or_default();
                    key.push_point([p[0], p[1], p[2]]);
                    key.push_number(surface.weight(i, j));
                }
            }
        }

        for curve in &b.m_curves_3d {
            key.push_curve_shape(curve);

            for i in 0..curve.cv_count() {
                let p = curve.get_cv(i).unwrap_or_default();
                key.push_point([p[0], p[1], p[2]]);
                key.push_number(curve.weight(i));
            }
        }

        for curve in &b.m_curves_2d {
            key.push_curve_shape(curve);
            key.push_parameters(&curve.m_cv);
        }

        for vertex in &b.m_vertices {
            key.push_point([vertex.point[0], vertex.point[1], vertex.point[2]]);
        }

        key.push_topology(b);
        Some(key)
    }

    fn push_words(&mut self, words: &[u64]) {
        self.words.extend_from_slice(words);
    }

    fn push_float(&mut self, value: f64) {
        self.words.push(value.to_bits());
    }

    /// A weight, rounded to `SAME_NUMBER`.
    fn push_number(&mut self, value: f64) {
        self.words.push((value / SAME_NUMBER).round() as i64 as u64);
    }

    /// A knot vector, each knot rounded to `SAME_NUMBER` of the whole span.
    fn push_knots(&mut self, knots: &[f64]) {
        let span = knots.last().zip(knots.first()).map_or(0.0, |(z, a)| z - a);
        self.words.push(knots.len() as u64);

        if !(span.is_finite() && span > 0.0) {
            self.words.extend(knots.iter().map(|k| k.to_bits()));
            return;
        }

        for knot in knots {
            self.words
                .push(((knot - knots[0]) / (span * SAME_NUMBER)).round() as i64 as u64);
        }

        self.push_number(knots[0] / span);
        self.push_number(span);
    }

    /// Pcurve control values, rounded to `SAME_NUMBER` of the largest of them.
    fn push_parameters(&mut self, values: &[f64]) {
        let size = values.iter().fold(1.0, |m: f64, v| m.max(v.abs()));
        self.words.push(values.len() as u64);
        self.words.extend(
            values
                .iter()
                .map(|v| (v / (size * SAME_NUMBER)).round() as i64 as u64),
        );
    }

    /// A 3D point in the BRep's frame, rounded to the step.
    fn push_point(&mut self, p: [f64; 3]) {
        for value in self.frame.local(p) {
            self.words.push((value / self.step).round() as i64 as u64);
        }
    }

    fn push_color(&mut self, color: &Color) {
        for value in [color.r, color.g, color.b, color.a] {
            self.words.push(value.to_bits() as u64);
        }
    }

    fn push_curve_shape(&mut self, curve: &NurbsCurve) {
        self.push_words(&[
            curve.m_dim as u64,
            curve.m_is_rat as u64,
            curve.m_order as u64,
            curve.m_cv_count as u64,
        ]);
        self.push_knots(&curve.m_nurbsknot);
    }

    fn push_refs(&mut self, refs: &[BRepRef]) {
        self.words.push(refs.len() as u64);

        for r in refs {
            self.push_words(&[r.index as i64 as u64, r.orientation as u64]);
        }
    }

    fn push_counts(&mut self, b: &BRep) {
        self.push_words(&[
            b.m_surfaces.len() as u64,
            b.m_curves_3d.len() as u64,
            b.m_curves_2d.len() as u64,
            b.m_vertices.len() as u64,
            b.m_edges.len() as u64,
            b.m_wires.len() as u64,
            b.m_faces.len() as u64,
            b.m_shells.len() as u64,
            b.m_solids.len() as u64,
        ]);
    }

    fn push_topology(&mut self, b: &BRep) {
        for edge in &b.m_edges {
            self.push_words(&[
                edge.curve_3d_index as i64 as u64,
                edge.start_vertex as i64 as u64,
                edge.end_vertex as i64 as u64,
                edge.degenerated as u64,
                edge.pcurves.len() as u64,
            ]);

            for pcurve in &edge.pcurves {
                self.push_words(&[
                    pcurve.surface_index as i64 as u64,
                    pcurve.curve_2d_index as i64 as u64,
                    pcurve.curve_2d_index_2 as i64 as u64,
                ]);
            }
        }

        for wire in &b.m_wires {
            self.push_refs(&wire.edges);
        }

        for face in &b.m_faces {
            self.words.push(face.surface_index as i64 as u64);
            self.push_refs(&face.wires);

            match &face.facecolor {
                Some(color) => self.push_color(color),
                None => self.words.push(u64::MAX),
            }
        }

        for shell in &b.m_shells {
            self.push_refs(&shell.faces);
        }

        for solid in &b.m_solids {
            self.push_refs(&solid.shells);
        }
    }
}

/// Lengths of the tables a BRep walk appends to, taken before it.
pub struct Marks {
    verts: usize,          // arena vertices
    idx: usize,            // triangle indices
    face_ids: usize,       // source face per triangle
    faces: usize,          // face sources
    boundaries: usize,     // surface boundaries
    samples: usize,        // surface samples
    pipes: usize,          // edge pipes
    chains: usize,         // pipe chains
    untouched: [usize; 8], // tables a recording cannot replay; they must not grow
}

impl Marks {
    /// The lengths now.
    pub fn of(arena: &ArenaRows, ink: &Ink) -> Self {
        Self {
            verts: arena.verts.len(),
            idx: arena.idx.len(),
            face_ids: arena.face_ids.len(),
            faces: arena.face_sources.len(),
            boundaries: arena.surface_boundaries.len(),
            samples: arena.surface_samples.len(),
            pipes: ink.seg.pipes.len(),
            chains: ink.seg.pipe_chains.len(),
            untouched: [
                arena.idx_print.len(),
                arena.idx_text.len(),
                ink.seg.ribbons.len(),
                ink.seg.ribbon_chains.len(),
                ink.seg.ribbon_heads.len(),
                ink.seg.sheet_rows.len(),
                ink.glyph.spheres.len(),
                ink.glyph.dots.len(),
            ],
        }
    }
}

/// The rows one BRep walk appended, relative to where they started, to be replayed for a copy.
pub struct Recording {
    frame: Frame,                     // where the walked BRep lies
    verts: Vec<RenderVertex>,         // face vertices
    idx: Vec<u32>,                    // triangle indices from the first vertex
    face_ids: Vec<u32>,               // per triangle: face source from the first
    faces: Vec<usize>,                // per face source: BRep face index
    boundaries: Vec<(u32, [u32; 2])>, // pipe and vertex pair, both from the first
    samples: Vec<Sample>,             // surface samples, vertex from the first
    pipes: Vec<CylinderSegment>,      // edge pipes
    normals: Vec<FacingPair>,         // per pipe: the normals its facing word packs
    pipe_ids: Vec<u32>,               // per pipe: BRep edge
    sags: Vec<f32>,                   // per pipe: rise off the faces
    chains: Vec<Range<u32>>,          // pipe runs, from the first pipe
    padded: bool,                     // the walk padded the pipe ids of earlier pipes
    vertex_total: usize,              // face mesh vertices, for the spacing
    flags: u32,                       // row flags
    faces_drawn: bool,                // the row drew faces
}

impl Recording {
    /// Keep what the walk of a BRep at `frame` appended since `marks`; None when it wrote a table
    /// this replay does not cover, or its pipes and normals disagree.
    pub fn take(
        frame: Frame,
        (arena, ink): (&ArenaRows, &Ink),
        marks: &Marks,
        cx: &WalkCx,
        walked: (&Row, usize, Vec<FacingPair>),
    ) -> Option<Self> {
        let (row, vertex_total, normals) = walked;
        let after = Marks::of(arena, ink);

        if after.untouched != marks.untouched || normals.len() != after.pipes - marks.pipes {
            return None;
        }

        let base = cx.vert_base + marks.verts as u32;
        let first_pipe = marks.pipes as u32;
        let first_vert = marks.verts as u32;
        // each pipe run pads the pipe ids and sags to the pipes before it, then adds its own
        let padded = ink.seg.pipe_chains.len() > marks.chains;
        let pipes = if padded {
            marks.pipes..ink.seg.pipes.len()
        } else {
            0..0
        };

        Some(Self {
            frame,
            verts: arena.verts[marks.verts..].to_vec(),
            idx: arena.idx[marks.idx..].iter().map(|i| i - base).collect(),
            face_ids: arena.face_ids[marks.face_ids..]
                .iter()
                .map(|id| id - marks.faces as u32)
                .collect(),
            faces: arena.face_sources[marks.faces..]
                .iter()
                .map(|s| s.face)
                .collect(),
            boundaries: arena.surface_boundaries[marks.boundaries..]
                .iter()
                .map(|(pipe, ends)| {
                    (
                        pipe - first_pipe,
                        [ends[0] - first_vert, ends[1] - first_vert],
                    )
                })
                .collect(),
            samples: arena.surface_samples[marks.samples..]
                .iter()
                .map(|s| Sample {
                    index: s.index - first_vert,
                    ..*s
                })
                .collect(),
            pipes: ink.seg.pipes[pipes.clone()].to_vec(),
            normals,
            pipe_ids: ink.seg.pipe_ids[pipes.clone()].to_vec(),
            sags: ink.seg.pipe_sags[pipes].to_vec(),
            chains: ink.seg.pipe_chains[marks.chains..]
                .iter()
                .map(|c| c.start - first_pipe..c.end - first_pipe)
                .collect(),
            padded,
            vertex_total,
            flags: row.flags,
            faces_drawn: row.faces,
        })
    }

    /// Append the recorded rows for a copy at `frame`, in row `cx.row`, as its own walk would.
    fn replay(&self, frame: &Frame, arena: &mut ArenaRows, ink: &mut Ink, cx: &WalkCx) -> Row {
        let motion = self.frame.motion_to(frame);
        let first_vert = arena.verts.len() as u32;
        let base = cx.vert_base + first_vert;
        let first_face = arena.face_sources.len() as u32;
        let mut bounds = AABB::empty();

        arena.verts.reserve(self.verts.len());

        for v in &self.verts {
            let position = motion.position(v.position);
            bounds.union_with_point(position[0] as f64, position[1] as f64, position[2] as f64);
            arena.verts.push(RenderVertex {
                position,
                normal: motion.normal(v.normal),
                color: v.color,
            });
        }

        let spacing = mesh_spacing(&bounds, self.vertex_total);
        arena
            .vids
            .resize(arena.vids.len() + self.verts.len(), cx.row);
        arena.idx.extend(self.idx.iter().map(|i| i + base));
        arena
            .face_ids
            .extend(self.face_ids.iter().map(|id| id + first_face));
        arena
            .face_sources
            .extend(self.faces.iter().map(|&face| FaceSource {
                parent: cx.row,
                face,
            }));
        arena
            .surface_samples
            .extend(self.samples.iter().map(|s| Sample {
                index: s.index + first_vert,
                ..*s
            }));

        if self.padded {
            ink.seg.pipe_ids.resize(ink.seg.pipes.len(), u32::MAX);
            ink.seg.pipe_sags.resize(ink.seg.pipes.len(), 0.0);
        }

        let first_pipe = ink.seg.pipes.len() as u32;
        arena
            .surface_boundaries
            .extend(self.boundaries.iter().map(|(pipe, ends)| {
                (
                    pipe + first_pipe,
                    [ends[0] + first_vert, ends[1] + first_vert],
                )
            }));
        ink.seg.pipes.reserve(self.pipes.len());

        for (pipe, (first, second)) in self.pipes.iter().zip(&self.normals) {
            let p0 = motion.position(pipe.p0);
            let p1 = motion.position(pipe.p1);
            bounds.union_with_point(p0[0] as f64, p0[1] as f64, p0[2] as f64);
            bounds.union_with_point(p1[0] as f64, p1[1] as f64, p1[2] as f64);
            let first = first.map(|n| motion.turn(n));
            let second = second.map(|n| motion.turn(n));
            ink.seg.pipes.push(CylinderSegment {
                p0,
                p1,
                instance_id: cx.row,
                facing: pack_facing(first.as_ref(), second.as_ref()),
                ..*pipe
            });
        }

        ink.seg.pipe_ids.extend_from_slice(&self.pipe_ids);
        ink.seg.pipe_sags.extend_from_slice(&self.sags);
        ink.seg.pipe_chains.extend(
            self.chains
                .iter()
                .map(|c| c.start + first_pipe..c.end + first_pipe),
        );

        Row {
            bounds,
            spacing,
            flags: self.flags,
            faces: self.faces_drawn,
        }
    }
}

/// Walks kept by shape while a document is walked; None while no `SharedWalks` is alive.
type Shapes = HashMap<Vec<u64>, Option<Recording>>;

thread_local! {
    static SHAPES: RefCell<Option<Shapes>> = const { RefCell::new(None) };
}

/// While one is alive, congruent BReps are walked once; the kept walks are freed when the
/// outermost one drops, so a document's copies share and nothing outlives its walk.
pub struct SharedWalks {
    outer: bool, // this one started the sharing
}

impl SharedWalks {
    /// Start sharing, or join the sharing already running.
    pub fn begin() -> Self {
        let outer = SHAPES.with(|shapes| {
            let mut shapes = shapes.borrow_mut();
            let outer = shapes.is_none();

            if outer {
                *shapes = Some(HashMap::new());
            }

            outer
        });

        Self { outer }
    }
}

impl Drop for SharedWalks {
    fn drop(&mut self) {
        if self.outer {
            SHAPES.with(|shapes| shapes.borrow_mut().take());
        }
    }
}

/// What `walk_brep` does with one BRep while walks are shared.
pub enum Share {
    Off,                     // not sharing: walk it
    Replayed(Row),           // a copy: its rows are appended
    Record(Vec<u64>, Frame), // the first of its shape: walk it, then `keep`
}

/// Replay `b` when a congruent BRep was walked before; otherwise say whether to record it.
pub fn replay(b: &BRep, arena: &mut ArenaRows, ink: &mut Ink, cx: &WalkCx) -> Share {
    SHAPES.with(|shapes| {
        let shapes = shapes.borrow();
        let Some(shapes) = shapes.as_ref() else {
            return Share::Off;
        };
        let Some(key) = ShapeKey::of(b) else {
            return Share::Off;
        };

        match shapes.get(&key.words) {
            Some(Some(recording)) => Share::Replayed(recording.replay(&key.frame, arena, ink, cx)),
            Some(None) => Share::Off,
            None => Share::Record(key.words, key.frame),
        }
    })
}

/// Keep the walk of the first BRep of a shape, or remember that its shape cannot be replayed.
pub fn keep(words: Vec<u64>, recording: Option<Recording>) {
    SHAPES.with(|shapes| {
        if let Some(shapes) = shapes.borrow_mut().as_mut() {
            shapes.insert(words, recording);
        }
    });
}

#[cfg(test)]
mod tests {
    use crate::engine::gpu::segments::is_silhouette;
    use super::*;
    use crate::app::walk::brep::walk_brep;
    use crate::app::walk::encode::FACING_UNKNOWN;
    use crate::engine::gpu::glyphs::GlyphRows;
    use crate::engine::gpu::segments::SegRows;
    use session_rust::{Vector, Xform};

    /// Every table a BRep walk writes.
    #[derive(Default)]
    struct Tables {
        arena: ArenaRows,
        seg: SegRows,
        glyph: GlyphRows,
    }

    /// Where one walk starts in its tables.
    #[derive(Clone, Copy)]
    struct Start {
        verts: usize, // arena vertices
        idx: usize,   // triangle indices
        pipes: usize, // edge pipes
        faces: usize, // face sources
    }

    impl Tables {
        /// Walk `b` into these tables in row 4, after 7 vertices already on the GPU.
        fn walk(&mut self, b: &BRep) -> Start {
            let start = Start {
                verts: self.arena.verts.len(),
                idx: self.arena.idx.len(),
                pipes: self.seg.pipes.len(),
                faces: self.arena.face_sources.len(),
            };
            let cx = WalkCx {
                vert_base: 7,
                cloud_px: 0.0,
                row: 4,
                attributes: false,
            };
            let mut ink = Ink {
                seg: &mut self.seg,
                glyph: &mut self.glyph,
            };
            walk_brep(&mut self.arena, &mut ink, b, &cx);
            start
        }
    }

    /// A block with a bore, the shape every copy below starts from.
    fn bored_block() -> BRep {
        BRep::create_block_with_hole(80.0, 50.0, 30.0, 9.0)
    }

    /// A rotation about a skew axis, then a move far from the origin.
    fn skew_placement() -> Xform {
        let turn = Xform::rotation(&Vector::new(0.3, -0.5, 0.8), 37.0, true);
        &Xform::translation(2500.0, -1200.0, 900.0) * &turn
    }

    /// A quarter turn about z and a whole-millimetre move: exact in floating point.
    fn square_placement() -> Xform {
        &Xform::translation(2500.0, -1200.0, 900.0) * &Xform::rotation_z(90.0, true)
    }

    /// Kept walks while sharing.
    fn kept() -> usize {
        SHAPES.with(|shapes| shapes.borrow().as_ref().map_or(0, |shapes| shapes.len()))
    }

    /// Walk `b` alone, as if no sharing ran.
    fn walk_alone(b: &BRep) -> Tables {
        let mut alone = Tables::default();
        let shapes = SHAPES.with(|shapes| shapes.borrow_mut().take());
        alone.walk(b);
        SHAPES.with(|kept| *kept.borrow_mut() = shapes);
        alone
    }

    /// Pipes per BRep edge.
    fn pipes_per_edge(ids: &[u32]) -> std::collections::BTreeMap<u32, usize> {
        let mut count = std::collections::BTreeMap::new();

        for id in ids {
            *count.entry(*id).or_insert(0) += 1;
        }

        count
    }

    /// Distance from `p` to the segment `a`-`b`.
    fn segment_distance(p: [f32; 3], a: [f32; 3], b: [f32; 3]) -> f64 {
        let f = |v: [f32; 3]| [v[0] as f64, v[1] as f64, v[2] as f64];
        let (p, a, b) = (f(p), f(a), f(b));
        let ab = sub(b, a);
        let t = (dot(sub(p, a), ab) / dot(ab, ab).max(1e-30)).clamp(0.0, 1.0);
        let q = [a[0] + t * ab[0], a[1] + t * ab[1], a[2] + t * ab[2]];
        dot(sub(p, q), sub(p, q)).sqrt()
    }

    /// One direction of a facing word, unpacked from its octahedral bytes.
    fn unpack(half: u32) -> [f64; 3] {
        let byte = |b: u32| (b as u8 as i8) as f64 / 127.0;
        let (x, y) = (byte(half & 0xff), byte(half >> 8 & 0xff));
        let z = 1.0 - x.abs() - y.abs();
        let (x, y) = if z < 0.0 {
            ((1.0 - y.abs()) * x.signum(), (1.0 - x.abs()) * y.signum())
        } else {
            (x, y)
        };
        unit([x, y, z])
    }

    /// Two facing words name the same two directions within a quantization step; the corners of
    /// the octahedral square all name straight down.
    fn same_facing(a: u32, b: u32) -> bool {
        if a == FACING_UNKNOWN || b == FACING_UNKNOWN {
            return a == b;
        }

        [0, 16]
            .iter()
            .all(|&s| dot(unpack(a >> s & 0xffff), unpack(b >> s & 0xffff)) > 0.999)
    }

    /// The copy's walk from `at` in `shared` has the triangles, faces, vertices and pipes per edge
    /// of its own walk alone; every pipe end lies on the edges that walk draws, within the sag of
    /// a display chord, since a mesh vertex halfway between two curve samples may replace either;
    /// a pipe with the same ends there has the same facing word.
    fn assert_same_shape(shared: &Tables, at: Start, alone: &Tables) {
        assert_eq!(shared.arena.verts.len() - at.verts, alone.arena.verts.len());

        for (a, b) in shared.arena.verts[at.verts..]
            .iter()
            .zip(&alone.arena.verts)
        {
            for k in 0..3 {
                assert!((a.position[k] - b.position[k]).abs() < 1e-3);
                assert!((a.normal[k] - b.normal[k]).abs() < 1e-5);
            }
        }

        let shift = at.verts as u32;
        let idx: Vec<u32> = shared.arena.idx[at.idx..]
            .iter()
            .map(|i| i - shift)
            .collect();
        assert_eq!(idx, alone.arena.idx);
        let faces = |tables: &Tables, from: usize| -> Vec<(u32, usize)> {
            tables.arena.face_sources[from..]
                .iter()
                .map(|s| (s.parent, s.face))
                .collect()
        };
        assert_eq!(faces(shared, at.faces), faces(alone, 0));
        let ids = &shared.seg.pipe_ids[at.pipes..];
        assert_eq!(pipes_per_edge(ids), pipes_per_edge(&alone.seg.pipe_ids));
        let silhouettes = |sags: &[f32]| sags.iter().filter(|sag| is_silhouette(**sag)).count();
        assert_eq!(silhouettes(&shared.seg.pipe_sags[at.pipes..]), silhouettes(&alone.seg.pipe_sags));
        let edge_pipes = ids.iter().filter(|id| **id != u32::MAX).count();
        let mut same_ends = 0;

        for (pipe, id) in shared.seg.pipes[at.pipes..].iter().zip(ids).filter(|(_, id)| **id != u32::MAX) {
            let edge: Vec<&CylinderSegment> = alone
                .seg
                .pipes
                .iter()
                .zip(&alone.seg.pipe_ids)
                .filter(|(_, other)| *other == id)
                .map(|(other, _)| other)
                .collect();

            for end in [pipe.p0, pipe.p1] {
                let on_edge = edge
                    .iter()
                    .any(|other| segment_distance(end, other.p0, other.p1) < 0.01);
                assert!(on_edge, "pipe end {end:?} off edge {id}");
            }

            let close = |a: [f32; 3], b: [f32; 3]| (0..3).all(|k| (a[k] - b[k]).abs() < 1e-3);

            if let Some(twin) = edge
                .iter()
                .find(|o| close(o.p0, pipe.p0) && close(o.p1, pipe.p1))
            {
                assert!(same_facing(pipe.facing, twin.facing), "facing of edge {id}");
                same_ends += 1;
            }

            assert_eq!((pipe.instance_id, pipe.color), (4, edge[0].color));
        }

        assert!(
            same_ends * 10 >= edge_pipes * 9,
            "{same_ends} of {} pipes match",
            edge_pipes
        );
    }

    /// Exactly the rows of its own walk alone: every vertex, index, pipe, id, sag and facing word.
    fn assert_same(shared: &Tables, at: Start, alone: &Tables) {
        assert_same_shape(shared, at, alone);
        assert_eq!(shared.seg.pipes.len() - at.pipes, alone.seg.pipes.len());

        for (a, b) in shared.seg.pipes[at.pipes..].iter().zip(&alone.seg.pipes) {
            assert_eq!(
                (a.p0, a.p1, a.facing, a.radius),
                (b.p0, b.p1, b.facing, b.radius)
            );
        }

        assert_eq!(shared.seg.pipe_ids[at.pipes..], alone.seg.pipe_ids[..]);
        assert_eq!(shared.seg.pipe_sags[at.pipes..], alone.seg.pipe_sags[..]);
    }

    /// A copy turned a quarter and moved replays the first walk and gets the rows its own walk
    /// would; nothing is kept once sharing ends.
    #[test]
    fn a_moved_copy_replays_its_shape() {
        let first = bored_block();
        let copy = first.transformed(&square_placement());
        let mut shared = Tables::default();
        let walks = SharedWalks::begin();
        shared.walk(&first);
        let at = shared.walk(&copy);
        assert_eq!(kept(), 1, "the copy replays");
        assert_same_shape(&shared, at, &walk_alone(&copy));
        drop(walks);
        assert!(SHAPES.with(|shapes| shapes.borrow().is_none()));
    }

    /// A copy turned about a skew axis replays the same triangles and the same edges; a mirrored
    /// copy and a wider bore are walked on their own.
    #[test]
    fn turned_mirrored_and_other_shapes() {
        let first = bored_block();
        let turned = first.transformed(&skew_placement());
        let mirrored = first.transformed(&Xform::scale_xyz(-1.0, 1.0, 1.0));
        let wider =
            BRep::create_block_with_hole(80.0, 50.0, 30.0, 9.5).transformed(&skew_placement());
        let mut shared = Tables::default();
        let _walks = SharedWalks::begin();
        shared.walk(&first);
        let at = shared.walk(&turned);
        assert_eq!(kept(), 1, "the turned copy replays");
        assert_same_shape(&shared, at, &walk_alone(&turned));

        // a shape walked on its own is exactly its walk alone; a wrong replay would be off
        for other in [&mirrored, &wider] {
            let at = shared.walk(other);
            assert_same(&shared, at, &walk_alone(other));
        }

        assert!(kept() >= 2, "a wider bore keys apart");
    }
}
