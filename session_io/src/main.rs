//! PDF -> Session (.pb), all Rust.
//!   cargo run --release -- <file.pdf> <out_stem> [page]
//!
//! MuPDF is the same engine PyMuPDF wraps, so this replaces the python extractor exactly - but a
//! device gets MORE than `get_drawings()` ever exposed: `fill_text` hands over the real font and
//! its glyph outlines, so text no longer needs Ghostscript to flatten it, and the font NAME and
//! the unicode of every character survive the import.
//!
//! What a page holds, and where it lands:
//!   stroke paths -> Line (2 points) / Polyline (a chained run) / NurbsCurve (each cubic, exact)
//!                   dashed strokes are expanded into their on-runs (dash pattern + phase honoured)
//!   fill paths   -> Mesh, contours triangulated with holes, ONE mesh per (layer, colour)
//!   text         -> the same fills, from `Font::outline_glyph_with_ctm`, cached per (font, glyph)
//!   OCG layers   -> session tree groups, one per named CAD layer
//!   page box     -> a closed Polyline, so a sheet's extents are the PAPER, not just its ink
//!   white paths  -> dropped: white on white paper is a knockout mask, not geometry
//!   images/shadings -> NOT imported, counted and warned about
//! PDF y points down -> flipped here; 1 pt = 1 mm; z = 0. Widths are ABSOLUTE mm (a 0.35 pen
//! stores 0.35); the viewer's planar-sheet lane treats them as world-mm lineweights.
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::rc::Rc;

use mupdf::device::NativeDevice;
use mupdf::path::PathWalker;
use mupdf::pdf::PdfDocument;
use mupdf::{ColorParams, Colorspace, Device, Document, Image, Matrix, Path, Rect, Shade, StrokeState, Text};
use rayon::prelude::*;
use session_rust::{Color, Line, Mesh, NurbsCurve, Point, Polyline, Session};

/// Control-polygon length, in points, per flattening sample - squared-scaled, see `bez_steps`.
const BEZ_CHORD: f64 = 0.25;
/// PDF width 0 means "thinnest renderable"; this is what it maps to, in mm.
const HAIRLINE: f64 = 0.1;
/// A path colour this close to white is a knockout mask, not ink.
const WHITE: f32 = 0.99;

type P = [f64; 2];

/// One pen-down run: 2 points is a Line, more is a Polyline.
struct Stroke { layer: usize, w: f64, c: Color, pts: Vec<P> }
/// One cubic, kept ANALYTIC - the kernel has NurbsCurve, so a bezier need not be flattened.
struct Curve { layer: usize, w: f64, c: Color, cv: [P; 4] }
/// One filled region: loops[0..] are border + holes, in any order (`islands` sorts them out).
struct Fill { layer: usize, c: Color, loops: Vec<Vec<P>> }

/// One glyph, triangulated ONCE in glyph space (y negated by the flip-0 walk), reused per
/// occurrence. `bad` = the area self-check failed at cache build.
struct GlyphMesh { islands: Vec<(Vec<P>, Vec<usize>)>, bad: bool, empty: bool }
/// One placed occurrence of a cached glyph: the full text matrix (trm + pen + ctm).
struct GlyphRef { layer: usize, c: Color, m: [f64; 6], g: Rc<GlyphMesh> }

#[derive(Default)]
struct Fonts { glyphs: BTreeMap<String, usize> }

#[derive(Default)]
struct State {
    flip: f64,                 // y' = flip - y : PDF device space is y-down
    layers: Vec<String>,
    layer_stack: Vec<usize>,
    strokes: Vec<Stroke>,
    curves: Vec<Curve>,
    fills: Vec<Fill>,
    glyph_cache: HashMap<(String, i32), Option<Rc<GlyphMesh>>>,
    glyph_refs: Vec<GlyphRef>,
    fonts: Fonts,
    chars: usize,
    white: usize,
    images: usize,
    image_area: f64,           // total placed image area, mm²
    shades: usize,
    no_outline: BTreeMap<String, usize>,   // glyph occurrences with no outline, per font
    // Transparency actually present in the file. Fills go down the triangle pipeline, which does
    // NOT blend, so a translucent hatch would render solid - worth knowing before trusting a colour.
    translucent_fills: usize,
    translucent_strokes: usize,
    min_alpha: f32,
}

impl State {
    fn layer(&self) -> usize {
        *self.layer_stack.last().unwrap_or(&0)
    }
}

/// One path op, already in DEVICE space (ctm applied, y flipped).
enum Seg { Move(P), Line(P), Curve(P, P, P), Close }

/// Collects a mupdf `Path` into ops. `walk` hands us user-space coordinates, so the ctm and the
/// page flip are applied here, once, at the only place they are known.
struct Walk { ctm: Matrix, flip: f64, out: Vec<Seg> }

impl Walk {
    fn pt(&self, x: f32, y: f32) -> P {
        let (x, y) = (x as f64, y as f64);
        let (a, b, c, d, e, f) = (self.ctm.a as f64, self.ctm.b as f64, self.ctm.c as f64,
                                  self.ctm.d as f64, self.ctm.e as f64, self.ctm.f as f64);
        [a * x + c * y + e, self.flip - (b * x + d * y + f)]
    }
}

impl PathWalker for &mut Walk {
    fn move_to(&mut self, x: f32, y: f32) { let p = self.pt(x, y); self.out.push(Seg::Move(p)); }
    fn line_to(&mut self, x: f32, y: f32) { let p = self.pt(x, y); self.out.push(Seg::Line(p)); }
    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) {
        let (c1, c2, e) = (self.pt(x1, y1), self.pt(x2, y2), self.pt(x3, y3));
        self.out.push(Seg::Curve(c1, c2, e));
    }
    fn close(&mut self) { self.out.push(Seg::Close); }
}

fn walk(path: &Path, ctm: Matrix, flip: f64) -> Vec<Seg> {
    let mut w = Walk { ctm, flip, out: Vec::new() };
    let _ = path.walk(&mut w);
    w.out
}

fn dist(a: P, b: P) -> f64 { ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2)).sqrt() }

/// Samples for one cubic, from its own size. A glyph is ~5.6 pt across and its curves are
/// fractions of that - 2 samples are already finer than a pixel - while a metre-long arc on the
/// same sheet needs a dozen. Every extra vertex is also a face, a halfedge and a triangulation
/// entry in the .pb, so a flat count either facets the big ones or bloats the file.
fn bez_steps(a: P, c1: P, c2: P, b: P) -> usize {
    let d = dist(a, c1) + dist(c1, c2) + dist(c2, b);
    // Floor of 4, not 2: a lowercase 'o' is four cubics, and at 2 samples each it triangulates
    // into a visible octagon. Letters are the smallest curves on the sheet AND the ones the eye
    // judges hardest.
    ((d / BEZ_CHORD).sqrt() as usize).clamp(4, 16)
}

fn bezier(a: P, c1: P, c2: P, b: P, out: &mut Vec<P>) {
    let n = bez_steps(a, c1, c2, b);
    for i in 1..=n {
        let t = i as f64 / n as f64;
        let u = 1.0 - t;
        let (w0, w1, w2, w3) = (u * u * u, 3.0 * u * u * t, 3.0 * u * t * t, t * t * t);
        out.push([w0 * a[0] + w1 * c1[0] + w2 * c2[0] + w3 * b[0],
                  w0 * a[1] + w1 * c1[1] + w2 * c2[1] + w3 * b[1]]);
    }
}

/// Closed contours, beziers flattened - what a filled region needs.
fn contours(segs: &[Seg]) -> Vec<Vec<P>> {
    let mut loops: Vec<Vec<P>> = Vec::new();
    let mut cur: Vec<P> = Vec::new();
    let close = |cur: &mut Vec<P>, loops: &mut Vec<Vec<P>>| {
        if cur.len() > 2 {
            if cur[0] != *cur.last().unwrap() { cur.push(cur[0]); }
            loops.push(std::mem::take(cur));
        } else {
            cur.clear();
        }
    };
    for s in segs {
        match *s {
            Seg::Move(p) => { close(&mut cur, &mut loops); cur.push(p); }
            Seg::Line(p) => cur.push(p),
            Seg::Curve(c1, c2, e) => {
                let a = *cur.last().unwrap_or(&c1);
                bezier(a, c1, c2, e, &mut cur);
            }
            Seg::Close => close(&mut cur, &mut loops),
        }
    }
    close(&mut cur, &mut loops);
    // Flattening can emit points on top of each other (a tiny cubic sampled 4×, a contour that
    // starts where the previous one ended). A zero-length constraint edge is a CDT degeneracy,
    // and one degenerate edge takes the whole letter down with it.
    for lp in loops.iter_mut() {
        lp.dedup_by(|a, b| dist(*a, *b) < 1e-9);
    }
    loops.retain(|lp| lp.len() > 3);
    loops
}

/// Open pen-down chains, beziers flattened - what a dashed stroke needs.
fn flatten_chains(segs: &[Seg]) -> Vec<Vec<P>> {
    let mut out: Vec<Vec<P>> = Vec::new();
    let mut cur: Vec<P> = Vec::new();
    let mut start: Option<P> = None;
    for s in segs {
        match *s {
            Seg::Move(p) => {
                if cur.len() >= 2 { out.push(std::mem::take(&mut cur)); }
                cur.clear();
                cur.push(p);
                start = Some(p);
            }
            Seg::Line(p) => cur.push(p),
            Seg::Curve(c1, c2, e) => {
                let a = *cur.last().unwrap_or(&c1);
                if cur.is_empty() { cur.push(a); }
                bezier(a, c1, c2, e, &mut cur);
            }
            Seg::Close => {
                if let Some(s0) = start {
                    if cur.last().map_or(false, |l| *l != s0) { cur.push(s0); }
                }
                if cur.len() >= 2 { out.push(std::mem::take(&mut cur)); }
                cur.clear();
                if let Some(s0) = start { cur.push(s0); }
            }
        }
    }
    if cur.len() >= 2 { out.push(cur); }
    out
}

/// Split one flattened chain into its dash ON-runs. `pat` alternates on/off and repeats; an
/// odd-length PDF dash array continues alternating, which duplicating the array reproduces.
fn dash_runs(pts: &[P], pat: &[f64], phase: f64) -> Vec<Vec<P>> {
    let cycle: f64 = pat.iter().sum();
    let mut idx = 0;
    let mut pos = phase.rem_euclid(cycle);
    loop {
        if pos < pat[idx] { break }
        pos -= pat[idx];
        idx = (idx + 1) % pat.len();
    }
    let mut rem = pat[idx] - pos;
    let mut on = idx % 2 == 0;
    let mut runs: Vec<Vec<P>> = Vec::new();
    let mut cur: Vec<P> = Vec::new();
    if on { cur.push(pts[0]); }
    for w2 in pts.windows(2) {
        let (a, b) = (w2[0], w2[1]);
        let len = dist(a, b);
        if len < 1e-12 { continue }
        let mut done = 0.0;
        while rem < len - done {
            done += rem;
            let t = done / len;
            let p = [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t];
            if on {
                cur.push(p);
                if cur.len() >= 2 { runs.push(std::mem::take(&mut cur)); } else { cur.clear(); }
            } else {
                cur.clear();
                cur.push(p);
            }
            on = !on;
            idx = (idx + 1) % pat.len();
            rem = pat[idx];
        }
        rem -= len - done;
        if on { cur.push(b); }
    }
    if on && cur.len() >= 2 { runs.push(cur); }
    runs
}

/// Signed shoelace area of a closed contour (last point repeats the first).
fn area_signed(loop_: &[P]) -> f64 {
    let mut a = 0.0;
    for i in 0..loop_.len() {
        let (p, q) = (loop_[i], loop_[(i + 1) % loop_.len()]);
        a += p[0] * q[1] - q[0] * p[1];
    }
    a * 0.5
}

fn area(loop_: &[P]) -> f64 { area_signed(loop_).abs() }

/// What the fill SHOULD cover: border minus its holes.
fn target_area(loops: &[Vec<P>]) -> f64 {
    if loops.is_empty() { return 0.0 }
    // loops[0] is not necessarily the border - take the largest, like the CDT does.
    let all: Vec<f64> = loops.iter().map(|l| area(l)).collect();
    let max = all.iter().cloned().fold(0.0, f64::max);
    (2.0 * max - all.iter().sum::<f64>()).max(0.0)
}

/// Is `p` inside the closed polygon `poly`? Ray casting - the loops here are glyph contours and
/// hatch outlines, tens of points each.
fn inside(p: P, poly: &[P]) -> bool {
    let mut hit = false;
    let n = poly.len();
    for i in 0..n {
        let (a, b) = (poly[i], poly[(i + 1) % n]);
        if (a[1] > p[1]) != (b[1] > p[1]) {
            let t = (p[1] - a[1]) / (b[1] - a[1]);
            if p[0] < a[0] + t * (b[0] - a[0]) { hit = !hit; }
        }
    }
    hit
}

/// Winding number of `p` in the closed polygon `poly` - the nonzero rule's `inside`.
fn winding(p: P, poly: &[P]) -> i32 {
    let mut wn = 0;
    let n = poly.len();
    for i in 0..n {
        let (a, b) = (poly[i], poly[(i + 1) % n]);
        let cross = (b[0] - a[0]) * (p[1] - a[1]) - (p[0] - a[0]) * (b[1] - a[1]);
        if a[1] <= p[1] {
            if b[1] > p[1] && cross > 0.0 { wn += 1; }
        } else if b[1] <= p[1] && cross < 0.0 {
            wn -= 1;
        }
    }
    wn
}

/// Split a path's contours into ISLANDS, each with the holes it actually contains.
///
/// The obvious rule - biggest contour is the border, everything else is a hole - is wrong for
/// text, and that is exactly where it shows: the dot of an `i`, the two bars of an `=`, the
/// umlaut of an `ä` are separate ISLANDS, not holes. `even_odd` picks the PDF fill rule:
/// containment parity (even-odd) or winding numbers (nonzero) - under nonzero, a contour nested
/// inside a same-orientation one is REDUNDANT, not a hole, which parity gets wrong.
fn islands(loops: Vec<Vec<P>>, even_odd: bool) -> Vec<Vec<Vec<P>>> {
    let n = loops.len();
    if n <= 1 { return if loops.is_empty() { Vec::new() } else { vec![loops] } }

    // depth[i] = how many other contours contain contour i (hole->island assignment)
    let mut depth = vec![0usize; n];
    for i in 0..n {
        let probe = loops[i][0];
        for j in 0..n {
            if i != j && inside(probe, &loops[j]) { depth[i] += 1; }
        }
    }

    // 0 = island, 1 = hole, 2 = dropped (nonzero: redundant or degenerate)
    let class: Vec<u8> = (0..n).map(|i| {
        if even_odd { return (depth[i] % 2) as u8 }
        let probe = loops[i][0];
        let wn_out: i32 = (0..n).filter(|&j| j != i).map(|j| winding(probe, &loops[j])).sum();
        let wn_in = wn_out + if area_signed(&loops[i]) >= 0.0 { 1 } else { -1 };
        match (wn_in != 0, wn_out != 0) {
            (true, false) => 0,
            (false, true) => 1,
            _ => 2,
        }
    }).collect();

    // Each hole belongs to the deepest island that contains it.
    let mut out: Vec<Vec<Vec<P>>> = Vec::new();
    let mut index: Vec<Option<usize>> = vec![None; n];
    for i in 0..n {
        if class[i] == 0 {
            index[i] = Some(out.len());
            out.push(vec![loops[i].clone()]);
        }
    }
    for i in 0..n {
        if class[i] == 1 {
            let probe = loops[i][0];
            let mut best: Option<(usize, usize)> = None;   // (depth, island index)
            for j in 0..n {
                if i != j && class[j] == 0 && inside(probe, &loops[j]) {
                    if best.map_or(true, |(d, _)| depth[j] > d) {
                        best = Some((depth[j], index[j].unwrap()));
                    }
                }
            }
            if let Some((_, k)) = best { out[k].push(loops[i].clone()); }
        }
    }
    out
}

fn to_color(cs: &Colorspace, color: &[f32], alpha: f32, cp: ColorParams) -> Color {
    // Colorspaces on a CAD sheet are DeviceGray, DeviceRGB, DeviceCMYK and separations; mupdf
    // converts any of them properly, which hand-rolled gray/cmyk arithmetic would not.
    let rgb = cs.convert_color(color, &Colorspace::device_rgb(), None, cp)
        .unwrap_or_else(|_| vec![0.0, 0.0, 0.0]);
    Color::new(rgb.first().copied().unwrap_or(0.0),
               rgb.get(1).copied().unwrap_or(0.0),
               rgb.get(2).copied().unwrap_or(0.0),
               alpha)
}

fn is_white(c: &Color) -> bool { c.r >= WHITE && c.g >= WHITE && c.b >= WHITE }

/// Triangulate one island (border + holes) with earcut, into raw verts + triangle indices.
///
/// Not the kernel CDT: `Mesh::from_polygon_with_holes` fits an average PLANE through the contour
/// and inserts Delaunay constraints - right for a trimmed surface in 3D, wrong here. It
/// mis-triangulated 28% of this sheet's glyph fills (measured by the area self-check below), and
/// a letter that comes back half-covered is exactly the "scratched" look. Earcut is the algorithm
/// fonts and maps use: ear-clipping with hole bridging, purely 2D, tolerant of contours that
/// touch themselves - which subset glyph outlines routinely do.
fn earcut_raw(loops: &[Vec<P>]) -> (Vec<P>, Vec<usize>) {
    let mut flat: Vec<f64> = Vec::new();
    let mut holes: Vec<usize> = Vec::new();
    for (i, lp) in loops.iter().enumerate() {
        // earcut wants OPEN rings: the closing duplicate would be a zero-length ear.
        let pts = if lp.len() > 1 && lp[0] == *lp.last().unwrap() { &lp[..lp.len() - 1] } else { &lp[..] };
        if pts.len() < 3 { continue }
        if i > 0 { holes.push(flat.len() / 2); }
        for p in pts { flat.push(p[0]); flat.push(p[1]); }
    }
    if flat.len() < 6 { return (Vec::new(), Vec::new()) }
    let Ok(tris) = earcutr::earcut(&flat, &holes, 2) else { return (Vec::new(), Vec::new()) };
    let verts = flat.chunks_exact(2).map(|c| [c[0], c[1]]).collect();
    (verts, tris)
}

fn tri_area(v: &[P], t: &[usize]) -> f64 {
    t.chunks_exact(3).map(|k| {
        let (a, b, c) = (v[k[0]], v[k[1]], v[k[2]]);
        ((b[0] - a[0]) * (c[1] - a[1]) - (c[0] - a[0]) * (b[1] - a[1])).abs() * 0.5
    }).sum()
}

/// Did the triangulation actually cover the polygon? Triangle area vs contour area catches a
/// letter that came back scratched - the defect is invisible in the object count.
fn area_off(loops: &[Vec<P>], v: &[P], t: &[usize]) -> bool {
    let want = target_area(loops);
    want > 1e-9 && (tri_area(v, t) - want).abs() > 0.02 * want
}

/// Outline + triangulate one glyph ONCE, in glyph space. None = mupdf has no outline for it
/// (a Type 3 procstream glyph, or a broken font program).
fn build_glyph(font: &mupdf::Font, gid: i32) -> Option<GlyphMesh> {
    let outline = font.outline_glyph_with_ctm(gid, &Matrix::IDENTITY).ok().flatten()?;
    let loops = contours(&walk(&outline, Matrix::IDENTITY, 0.0));
    let empty = loops.is_empty();   // an EMPTY outline (space-like glyph) is fine, not a failure
    let mut out = Vec::new();
    let mut bad = false;
    // Glyph outlines fill by the NONZERO rule.
    for island in islands(loops, false) {
        let (v, t) = earcut_raw(&island);
        if t.is_empty() { bad = true; continue }
        if area_off(&island, &v, &t) { bad = true; }
        out.push((v, t));
    }
    Some(GlyphMesh { islands: out, bad, empty })
}

struct Collector(Rc<RefCell<State>>);

impl Collector {
    /// A filled path -> islands in the current layer, under the path's fill rule.
    fn add_fill(&self, path: &Path, ctm: Matrix, c: Color, even_odd: bool) {
        let mut st = self.0.borrow_mut();
        if is_white(&c) { st.white += 1; return }
        if c.a < 0.999 { st.translucent_fills += 1; st.min_alpha = st.min_alpha.min(c.a); }
        let loops = contours(&walk(path, ctm, st.flip));
        if loops.is_empty() { return }
        let layer = st.layer();
        for island in islands(loops, even_odd) {
            st.fills.push(Fill { layer, c: c.clone(), loops: island });
        }
    }
}

impl NativeDevice for Collector {
    fn fill_path(&mut self, path: &Path, even_odd: bool, ctm: Matrix,
                 cs: &Colorspace, color: &[f32], alpha: f32, cp: ColorParams) {
        self.add_fill(path, ctm, to_color(cs, color, alpha, cp), even_odd);
    }

    fn stroke_path(&mut self, path: &Path, stroke: &StrokeState, ctm: Matrix,
                   cs: &Colorspace, color: &[f32], alpha: f32, cp: ColorParams) {
        let c = to_color(cs, color, alpha, cp);
        let mut st = self.0.borrow_mut();
        if is_white(&c) { st.white += 1; return }
        if c.a < 0.999 { st.translucent_strokes += 1; st.min_alpha = st.min_alpha.min(c.a); }
        // Line width is in USER space; the ctm scales it. expansion() is the average scale factor
        // mupdf itself uses for exactly this. Widths are stored as absolute mm (1 pt = 1 mm).
        let exp = ctm.expansion() as f64;
        let w = stroke.line_width() as f64 * exp;
        let w = if w > 0.0 { w } else { HAIRLINE };
        let layer = st.layer();
        let segs = walk(path, ctm, st.flip);

        // Dash lengths are user-space too. mupdf does NOT pre-flatten dashes for a custom
        // device, so the pattern is walked here; a dashed cubic cannot stay analytic.
        let pat: Vec<f64> = stroke.dashes().iter().map(|d| *d as f64 * exp).collect();
        if !pat.is_empty() && pat.iter().sum::<f64>() > 1e-9 {
            let pat: Vec<f64> = if pat.len() % 2 == 1 { pat.iter().chain(&pat).cloned().collect() } else { pat };
            let phase = stroke.dash_phase() as f64 * exp;
            for chain in flatten_chains(&segs) {
                for run in dash_runs(&chain, &pat, phase) {
                    st.strokes.push(Stroke { layer, w, c: c.clone(), pts: run });
                }
            }
            return;
        }

        let mut chain: Vec<P> = Vec::new();
        let mut start: Option<P> = None;
        for s in &segs {
            match *s {
                Seg::Move(p) => {
                    if chain.len() >= 2 { st.strokes.push(Stroke { layer, w, c: c.clone(), pts: std::mem::take(&mut chain) }); }
                    chain.clear();
                    chain.push(p);
                    start = Some(p);
                }
                Seg::Line(p) => chain.push(p),
                // A cubic stays a cubic: the kernel HAS NurbsCurve, so nothing is flattened here.
                Seg::Curve(c1, c2, e) => {
                    let a = *chain.last().unwrap_or(&c1);
                    if chain.len() >= 2 { st.strokes.push(Stroke { layer, w, c: c.clone(), pts: std::mem::take(&mut chain) }); }
                    chain.clear();
                    st.curves.push(Curve { layer, w, c: c.clone(), cv: [a, c1, c2, e] });
                    chain.push(e);
                }
                // Close back to the SUBPATH start - which a curve's chain reset must not lose.
                Seg::Close => {
                    if let Some(s0) = start {
                        if chain.last().map_or(false, |l| *l != s0) { chain.push(s0); }
                    }
                    if chain.len() >= 2 { st.strokes.push(Stroke { layer, w, c: c.clone(), pts: std::mem::take(&mut chain) }); }
                    chain.clear();
                    if let Some(s0) = start { chain.push(s0); }
                }
            }
        }
        if chain.len() >= 2 { st.strokes.push(Stroke { layer, w, c, pts: chain }); }
    }

    fn fill_text(&mut self, text: &Text, ctm: Matrix, cs: &Colorspace, color: &[f32],
                 alpha: f32, cp: ColorParams) {
        let c = to_color(cs, color, alpha, cp);
        let mut st = self.0.borrow_mut();
        if is_white(&c) { st.white += 1; return }
        if c.a < 0.999 { st.translucent_fills += 1; st.min_alpha = st.min_alpha.min(c.a); }
        let layer = st.layer();
        for span in text.spans() {
            let font = span.font();
            let name = font.name().to_string();
            for item in span.items() {
                if item.gid() < 0 { continue }   // no glyph (a space, or an unmapped code)
                // The glyph's placement: the span's text matrix carries scale/rotation, the item
                // carries the pen position, and ctm puts it on the page.
                let mut m = span.trm();
                m.e = item.x();
                m.f = item.y();
                m.concat(ctm.clone());
                let g = st.glyph_cache.entry((name.clone(), item.gid()))
                    .or_insert_with(|| build_glyph(&font, item.gid()).map(Rc::new))
                    .clone();
                match g {
                    Some(g) => st.glyph_refs.push(GlyphRef {
                        layer, c: c.clone(),
                        m: [m.a as f64, m.b as f64, m.c as f64, m.d as f64, m.e as f64, m.f as f64],
                        g,
                    }),
                    None => *st.no_outline.entry(name.clone()).or_insert(0) += 1,
                }
                *st.fonts.glyphs.entry(name.clone()).or_insert(0) += 1;
                st.chars += 1;
            }
        }
    }

    // A stroked glyph (outline text) is rare but real - treat it as a fill of the same outline.
    fn stroke_text(&mut self, text: &Text, _s: &StrokeState, ctm: Matrix, cs: &Colorspace,
                   color: &[f32], alpha: f32, cp: ColorParams) {
        self.fill_text(text, ctm, cs, color, alpha, cp);
    }

    fn fill_image(&mut self, _img: &Image, cmt: Matrix, _alpha: f32, _cp: ColorParams) {
        let mut st = self.0.borrow_mut();
        st.images += 1;
        // An image fills the unit square under its matrix; |det| is its placed area.
        st.image_area += (cmt.a as f64 * cmt.d as f64 - cmt.b as f64 * cmt.c as f64).abs();
    }

    fn fill_image_mask(&mut self, _img: &Image, cmt: Matrix, _cs: &Colorspace,
                       _color: &[f32], _alpha: f32, _cp: ColorParams) {
        let mut st = self.0.borrow_mut();
        st.images += 1;
        st.image_area += (cmt.a as f64 * cmt.d as f64 - cmt.b as f64 * cmt.c as f64).abs();
    }

    fn fill_shade(&mut self, _shade: &Shade, _cmt: Matrix, _alpha: f32, _cp: ColorParams) {
        self.0.borrow_mut().shades += 1;
    }

    fn begin_layer(&mut self, name: &str) {
        let mut st = self.0.borrow_mut();
        let id = match st.layers.iter().position(|l| l == name) {
            Some(i) => i,
            None => { st.layers.push(name.to_string()); st.layers.len() - 1 }
        };
        st.layer_stack.push(id);
    }

    fn end_layer(&mut self) {
        self.0.borrow_mut().layer_stack.pop();
    }
}

/// Write out every font program the PDF embeds. A sheet names its faces (ArialMT, DINOffc-Light,
/// subset-tagged WZLYOP+Arial) but the glyphs we import are OUTLINES - the font itself is the only
/// way to ever re-typeset that text, so it is worth keeping next to the drawing.
/// FontFile = Type1, FontFile2 = TrueType, FontFile3 = CFF/OpenType.
fn dump_fonts(src: &str, dir: &std::path::Path) -> usize {
    let Ok(doc) = PdfDocument::open(src) else { return 0 };
    let Ok(count) = doc.count_objects() else { return 0 };
    let mut n = 0;
    for num in 1..count as i32 {
        let Ok(obj) = doc.new_indirect(num, 0) else { continue };
        let Ok(Some(obj)) = obj.resolve() else { continue };
        for (key, ext) in [("FontFile", "pfb"), ("FontFile2", "ttf"), ("FontFile3", "cff")] {
            let Ok(Some(ff)) = obj.get_dict(key) else { continue };
            let Ok(bytes) = ff.read_stream() else { continue };
            if bytes.is_empty() { continue }
            // The descriptor names the face; fall back to the object number.
            let name = obj.get_dict("FontName").ok().flatten()
                .and_then(|o| o.as_name().ok())
                .map(|v| String::from_utf8_lossy(&v).to_string())
                .unwrap_or_else(|| format!("font_{num}"));
            let safe: String = name.chars().map(|c| if c.is_alphanumeric() || c == '-' || c == '+' { c } else { '_' }).collect();
            let path = dir.join(format!("{safe}.{ext}"));
            if !path.exists() && std::fs::write(&path, &bytes).is_ok() { n += 1; }
        }
    }
    n
}

fn points(p: &[P]) -> Vec<Point> {
    p.iter().map(|q| Point::new(q[0], q[1], 0.0)).collect()
}

fn ckey(c: &Color) -> [u8; 4] {
    [(c.r.clamp(0.0, 1.0) * 255.0).round() as u8,
     (c.g.clamp(0.0, 1.0) * 255.0).round() as u8,
     (c.b.clamp(0.0, 1.0) * 255.0).round() as u8,
     (c.a.clamp(0.0, 1.0) * 255.0).round() as u8]
}

fn main() {
    let mut args = std::env::args().skip(1);
    let src = args.next().expect("usage: pdf_import <file.pdf> <out_stem> [page]");
    let stem = args.next().unwrap_or_else(|| src.trim_end_matches(".pdf").to_string());
    let page_no = args.next().and_then(|p| p.parse::<i32>().ok()).unwrap_or(0);

    let t0 = std::time::Instant::now();
    let doc = Document::open(&src).expect("cannot open pdf");
    let page = doc.load_page(page_no).expect("no such page");
    let Rect { x0, y0, x1, y1 } = page.bounds().expect("no page box");

    let state = Rc::new(RefCell::new(State {
        // Device space is y-down; the sheet is authored y-up. One flip, applied in `Walk::pt`.
        flip: (y0 + y1) as f64,
        layers: vec!["0 unlayered".to_string()],
        min_alpha: 1.0,
        ..Default::default()
    }));
    let device = Device::from_native(Collector(state.clone())).expect("device");
    page.run(&device, &Matrix::IDENTITY).expect("run page");
    drop(device);
    let st = state.borrow();
    let t_read = t0.elapsed();

    // Path fills first: the triangulation is the long pole, and it is rayon-parallel.
    // Glyphs were triangulated once each at cache build; here they only get placed.
    let t1 = std::time::Instant::now();
    let tri_fills: Vec<(Vec<P>, Vec<usize>, bool)> = st.fills.par_iter()
        .map(|f| {
            let (v, t) = earcut_raw(&f.loops);
            let bad = !t.is_empty() && area_off(&f.loops, &v, &t);
            (v, t, bad)
        })
        .collect();
    let t_cdt = t1.elapsed();

    let name = std::path::Path::new(&stem).file_name().unwrap().to_string_lossy().to_string();
    let mut s = Session::new(&name);
    let mut groups: BTreeMap<usize, std::rc::Rc<std::cell::RefCell<session_rust::tree::TreeNode>>> = BTreeMap::new();
    macro_rules! group {
        ($i:expr) => {{
            let i = $i;
            groups.entry(i)
                .or_insert_with(|| s.add_group(st.layers.get(i).map(|x| x.as_str()).unwrap_or("0 unlayered")))
                .clone()
        }};
    }

    let (mut n_lines, mut n_polylines) = (0, 0);
    for sk in &st.strokes {
        let g = group!(sk.layer);
        let pts = points(&sk.pts);
        if pts.len() == 2 {
            let mut ln = Line::from_points(&pts[0], &pts[1]);
            ln.linecolor = sk.c.clone();
            ln.width = sk.w;
            s.add_line(ln, Some(&g));
            n_lines += 1;
        } else {
            let mut pl = Polyline::new(pts);
            pl.linecolor = sk.c.clone();
            pl.width = sk.w;
            s.add_polyline(pl, Some(&g));
            n_polylines += 1;
        }
    }

    for cv in &st.curves {
        let g = group!(cv.layer);
        let mut nc = NurbsCurve::create(false, 3, &points(&cv.cv));
        nc.linecolors = vec![cv.c.clone()];   // a curve carries a vec, not a single linecolor
        nc.width = cv.w;
        s.add_nurbscurve(nc, Some(&g));
    }

    // All fills of one (layer, colour) merge into ONE mesh: a sheet has tens of pens, not tens
    // of thousands - per-object overhead (guid, graph node, xform, proto framing) was most of
    // the .pb and most of the viewer's parse time.
    let (mut n_dropped, mut n_bad) = (0, 0);
    let mut buckets: BTreeMap<(usize, [u8; 4]), (Color, Vec<P>, Vec<usize>)> = BTreeMap::new();
    let push_part = |buckets: &mut BTreeMap<(usize, [u8; 4]), (Color, Vec<P>, Vec<usize>)>,
                         layer: usize, c: &Color, verts: &[P], tris: &[usize]| {
        let e = buckets.entry((layer, ckey(c))).or_insert_with(|| (c.clone(), Vec::new(), Vec::new()));
        let base = e.1.len();
        e.1.extend_from_slice(verts);
        e.2.extend(tris.iter().map(|t| t + base));
    };
    for (f, (v, t, bad)) in st.fills.iter().zip(&tri_fills) {
        if t.is_empty() { n_dropped += 1; continue }
        if *bad { n_bad += 1; }
        push_part(&mut buckets, f.layer, &f.c, v, t);
    }
    for gr in &st.glyph_refs {
        if gr.g.islands.is_empty() { if !gr.g.empty { n_dropped += 1; } continue }
        let [a, b, c2, d, e, f] = gr.m;
        for (v, t) in &gr.g.islands {
            let tv: Vec<P> = v.iter().map(|q| {
                let (x, y) = (q[0], -q[1]);   // cache space negated y (flip-0 walk)
                [a * x + c2 * y + e, st.flip - (b * x + d * y + f)]
            }).collect();
            push_part(&mut buckets, gr.layer, &gr.c, &tv, t);
        }
    }
    n_bad += st.glyph_cache.values().flatten().filter(|g| g.bad).count();

    let mut n_meshes = 0;
    for ((layer, _), (c, verts, tris)) in buckets {
        let mut m = Mesh::new();
        for p in &verts { m.add_vertex(Point::new(p[0], p[1], 0.0), None); }
        for t in tris.chunks_exact(3) { m.add_face(vec![t[0], t[1], t[2]], None); }
        if m.number_of_faces() == 0 { continue }
        m.set_objectcolor(c);
        // A fill is flat colour: drop the auto-seeded per-vertex/per-face vecs, which would
        // otherwise dominate the .pb.
        m.clear_pointcolors();
        m.clear_facecolors();
        // ONE transparent, zero-width entry: the viewer broadcasts a single width to every edge,
        // and reads width 0 as "no wireframe" - a letter renders solid, not outlined and dotted.
        m.set_linecolors(vec![Color::new(0.0, 0.0, 0.0, 0.0)], vec![0.0]);
        let g = group!(layer);
        s.add_mesh(m, Some(&g));
        n_meshes += 1;
    }

    // The sheet's paper edge, so extents are the PAPER and not the ink.
    let page_group = s.add_group("page");
    let (px0, py0, px1, py1) = (x0 as f64, st.flip - y1 as f64, x1 as f64, st.flip - y0 as f64);
    let mut border = Polyline::new(vec![
        Point::new(px0, py0, 0.0),
        Point::new(px1, py0, 0.0),
        Point::new(px1, py1, 0.0),
        Point::new(px0, py1, 0.0),
        Point::new(px0, py0, 0.0),
    ]);
    border.linecolor = Color::black();
    border.width = 0.35;
    s.add_polyline(border, Some(&page_group));

    // Fonts land next to the .pb, in the assets tree: <assets>/pb/<sheet>.pb -> <assets>/fonts/
    let out_dir = std::path::Path::new(&stem).parent().unwrap_or(std::path::Path::new("."));
    let font_dir = out_dir.parent().map(|p| p.join("fonts")).unwrap_or_else(|| out_dir.join("fonts"));
    std::fs::create_dir_all(&font_dir).ok();
    let n_fonts = dump_fonts(&src, &font_dir);

    let t2 = std::time::Instant::now();
    let out = format!("{stem}.pb");
    s.pb_dump(&out);

    println!("{name}: {n_lines} lines, {n_polylines} polylines, {} curves, {n_meshes} meshes, \
              {} layers, {} chars ({} unique glyphs), {} white dropped",
        st.curves.len(), st.layers.len(), st.chars, st.glyph_cache.len(), st.white);
    if st.translucent_fills + st.translucent_strokes > 0 {
        println!("  alpha: {} translucent fills, {} translucent strokes, min alpha {:.2} \
                  (fills do NOT blend in the viewer yet)",
            st.translucent_fills, st.translucent_strokes, st.min_alpha);
    }
    if st.images > 0 {
        println!("  WARNING: {} raster images (≈{:.0} mm² of paper) NOT imported", st.images, st.image_area);
    }
    if st.shades > 0 {
        println!("  WARNING: {} gradient shadings NOT imported", st.shades);
    }
    for (f, n) in &st.no_outline {
        println!("  WARNING: {n} glyphs from font {f} have no outlines (Type 3?) and are MISSING");
    }
    if n_dropped > 0 {
        println!("  WARNING: {n_dropped} fills failed to triangulate and are MISSING from the sheet");
    }
    if n_bad > 0 {
        println!("  WARNING: {n_bad} fills triangulated badly (area off by >2%) - letters will look scratched");
    }
    let mut fonts: Vec<_> = st.fonts.glyphs.iter().collect();
    fonts.sort_by_key(|(_, n)| std::cmp::Reverse(**n));
    println!("  fonts: {}", fonts.iter().take(6)
        .map(|(f, n)| format!("{f}×{n}")).collect::<Vec<_>>().join(", "));
    if n_fonts > 0 { println!("  fonts: {n_fonts} embedded programs -> {}", font_dir.display()); }
    println!("  page {:.0}×{:.0} pt · read {:.1}s · triangulate {:.1}s · write {:.1}s -> {out}",
        px1 - px0, py1 - py0, t_read.as_secs_f64(), t_cdt.as_secs_f64(), t2.elapsed().as_secs_f64());
}
