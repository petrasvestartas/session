use super::deck::{
    Action, CameraSpec, Chapter, Content, Index, Kind, MatrixSpec, ObjSpec, Pos, plain,
};
use super::eval::{self, Points};
use super::font::Fonts;
use super::tex::{self, Expr, Prim, Rgba, Shape};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

/// The slide design space; the canvas shows it letterboxed.
pub const W: f32 = 1600.0;
pub const H: f32 = 900.0;
const TOP: f32 = 128.0;
const BOTTOM: f32 = 150.0;
const MARGIN: f32 = 56.0;
const RELAYOUT_MS: f64 = 650.0;
const START_CAM: Cam = Cam {
    yaw: -60.0,
    pitch: 25.0,
    zoom: 60.0,
    center: [0.0; 3],
};

/// A formula piece in slide pixels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DPrim {
    pub shape: Shape,
    pub x: f32,
    pub y: f32,
    pub color: Rgba,
}

/// What the painter draws, in slide pixels, y down.
#[derive(Clone, Debug)]
pub enum Draw {
    Prim(DPrim),
    Line {
        a: [f32; 2],
        b: [f32; 2],
        width: f32,
        color: Rgba,
        arrow: bool,
        dash: bool,
    },
    Disc {
        c: [f32; 2],
        r: f32,
        color: Rgba,
        ring: Rgba,
    },
    Path {
        pts: Vec<[f32; 2]>,
        closed: bool,
        width: f32,
        color: Rgba,
    },
    Head {
        pts: [[f32; 2]; 3],
        color: Rgba,
    },
}

/// Smootherstep easing of `t` in 0..1.
pub fn smooth(t: f64) -> f64 {
    let t = t.clamp(0.0, 1.0);
    let s = 1.0 - t;
    t * t * t * (10.0 * s * s + 5.0 * s * t + t * t)
}

/// Ease up to 1 at the midpoint and back to 0.
fn there_and_back(t: f64) -> f64 {
    smooth(if t < 0.5 { 2.0 * t } else { 2.0 * (1.0 - t) })
}

/// Blend two colours.
fn mix(a: Rgba, b: Rgba, t: f32) -> Rgba {
    [
        a[0] + (b[0] - a[0]) * t,
        a[1] + (b[1] - a[1]) * t,
        a[2] + (b[2] - a[2]) * t,
        a[3] + (b[3] - a[3]) * t,
    ]
}

/// Blend two numbers.
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// The colour with its alpha scaled by `a`.
fn with_alpha(mut c: Rgba, a: f32) -> Rgba {
    c[3] *= a;
    c
}

/// A value moving from `from` to `to`.
#[derive(Clone, Copy, Debug)]
pub struct Tween<T: Copy> {
    from: T,
    to: T,
    t0: f64,
    dur: f64,
}

/// Values a tween can blend.
pub trait Lerp: Copy {
    fn lerp(a: Self, b: Self, t: f64) -> Self;
}

impl Lerp for f32 {
    fn lerp(a: f32, b: f32, t: f64) -> f32 {
        a + (b - a) * t as f32
    }
}

impl Lerp for [f32; 2] {
    fn lerp(a: Self, b: Self, t: f64) -> Self {
        [f32::lerp(a[0], b[0], t), f32::lerp(a[1], b[1], t)]
    }
}

impl Lerp for [f64; 3] {
    fn lerp(a: Self, b: Self, t: f64) -> Self {
        [
            a[0] + (b[0] - a[0]) * t,
            a[1] + (b[1] - a[1]) * t,
            a[2] + (b[2] - a[2]) * t,
        ]
    }
}

impl Lerp for Rgba {
    fn lerp(a: Self, b: Self, t: f64) -> Self {
        mix(a, b, t as f32)
    }
}

impl<T: Lerp> Tween<T> {
    /// A value at rest.
    fn fixed(v: T) -> Self {
        Tween {
            from: v,
            to: v,
            t0: 0.0,
            dur: 0.0,
        }
    }

    /// The value at time `now`.
    fn at(&self, now: f64) -> T {
        if self.dur <= 0.0 || now >= self.t0 + self.dur {
            return self.to;
        }
        if now <= self.t0 {
            return self.from;
        }
        T::lerp(self.from, self.to, smooth((now - self.t0) / self.dur))
    }

    /// Move from the current value to `to` over `dur` milliseconds.
    fn set(&mut self, to: T, now: f64, dur: f64) {
        self.from = self.at(now);
        self.to = to;
        self.t0 = now;
        self.dur = dur;
    }

    /// Move to `to` when `animate`, otherwise jump there.
    fn retarget(&mut self, to: T, now: f64, dur: f64, animate: bool) {
        if animate {
            self.set(to, now, dur);
        } else {
            *self = Tween::fixed(to);
        }
    }

    /// Jump to the end.
    fn snap(&mut self) {
        self.from = self.to;
        self.dur = 0.0;
    }

    /// Whether the value is still moving at `now`.
    fn busy(&self, now: f64) -> bool {
        self.dur > 0.0 && now < self.t0 + self.dur
    }
}

/// An orthographic view: angles in degrees, zoom in slide pixels per world unit.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cam {
    pub yaw: f64,
    pub pitch: f64,
    pub zoom: f64,
    pub center: [f64; 3],
}

impl Lerp for Cam {
    fn lerp(a: Self, b: Self, t: f64) -> Self {
        Cam {
            yaw: a.yaw + (b.yaw - a.yaw) * t,
            pitch: a.pitch + (b.pitch - a.pitch) * t,
            zoom: a.zoom * (b.zoom / a.zoom).powf(t),
            center: <[f64; 3]>::lerp(a.center, b.center, t),
        }
    }
}

/// The projection of one frame.
#[derive(Clone, Copy)]
pub struct View {
    right: [f64; 3],    // World direction of slide x.
    up: [f64; 3],       // World direction of slide up.
    cam: Cam,           // The authored camera.
    origin: [f32; 2],   // Slide point of the camera centre.
    lens: Option<Lens>, // The real camera, used instead when set.
}

/// The real camera as an affine map from world to slide pixels: `a * p + b`.
#[derive(Clone, Copy, Debug)]
pub struct Lens {
    pub a: [[f64; 3]; 2],
    pub b: [f64; 2],
}

/// A 3D object as the scene should show it this frame.
#[derive(Clone, Debug, PartialEq)]
pub struct Spatial {
    pub id: String,
    pub kind: SpatialKind,
    pub pts: Vec<[f64; 3]>,
    pub color: Rgba,
    pub width: f32,
    pub dash: bool,
    pub reveal: f32,
}

/// The kind of a scene object.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SpatialKind {
    Point,
    Line,
    Polyline,
}

/// Dot product.
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

impl View {
    /// The orthographic view of `cam` with its centre at slide point `origin`.
    fn new(cam: Cam, origin: [f32; 2]) -> Self {
        let (y, p) = (cam.yaw.to_radians(), cam.pitch.to_radians());
        let eye = [p.cos() * y.cos(), p.cos() * y.sin(), p.sin()];
        let right = [-y.sin(), y.cos(), 0.0];
        let up = [
            eye[1] * right[2] - eye[2] * right[1],
            eye[2] * right[0] - eye[0] * right[2],
            eye[0] * right[1] - eye[1] * right[0],
        ];
        View {
            right,
            up,
            cam,
            origin,
            lens: None,
        }
    }

    /// A world point on the slide.
    pub fn project(&self, p: [f64; 3]) -> [f32; 2] {
        if let Some(l) = &self.lens {
            return [
                (dot(l.a[0], p) + l.b[0]) as f32,
                (dot(l.a[1], p) + l.b[1]) as f32,
            ];
        }
        let d = [
            p[0] - self.cam.center[0],
            p[1] - self.cam.center[1],
            p[2] - self.cam.center[2],
        ];
        [
            self.origin[0] + (dot(d, self.right) * self.cam.zoom) as f32,
            self.origin[1] - (dot(d, self.up) * self.cam.zoom) as f32,
        ]
    }
}

/// How a panel block sits in the column.
#[derive(Clone, Copy, PartialEq, Debug)]
enum Align {
    Center,
    Left,
    Equals,
}

/// One object on the stage.
pub struct Obj {
    pub id: String,
    pub content: Content,
    color: Rgba,
    size: f32,  // Pixels per em.
    width: f32, // Line width, pixels.
    dash: bool,
    expr: Rc<Expr>,                      // The laid-out formula, em units.
    groups: HashMap<String, Vec<usize>>, // Named prim sets: cells, rows, parts, row items.
    dynamic: Option<String>,             // TeX with `{=expr}` fields, refilled as points move.
    hidden: HashSet<usize>,              // Prims drawn transparent.
    colors: HashMap<usize, Tween<Rgba>>, // Colour per prim; `usize::MAX` for a 3D object.
    align: Align,
    at: Option<[f32; 2]>, // A fixed slide position instead of the column.
    gap: f32,             // Space above the block, pixels.
    pos: Tween<[f32; 2]>, // Top-left corner on the slide.
    placed: bool,         // Whether `pos` has been laid out.
    fit: f32,             // Shrink factor that fits the column.
    dim: Tween<f32>,      // 0 normal, 1 dimmed.
    opacity: f32,
    geo_from: Option<(Vec<[f64; 3]>, f64, f64)>, // Points a 3D morph starts from, start, duration.
    dir: Option<[f32; 2]>,                       // Authored label direction.
    attach: Option<String>,                      // Object a label follows.
    boxed: Option<(String, f32)>,                // Target and padding of a box outline.
    label_dir: usize,                            // Last chosen label placement.
}

/// Whether content is 3D geometry.
fn is_geo(content: &Content) -> bool {
    matches!(
        content,
        Content::Point(_)
            | Content::Line { .. }
            | Content::Polyline { .. }
            | Content::Cuboid { .. }
    )
}

impl Obj {
    /// A formula, text, matrix or row in the column.
    fn block(&self) -> bool {
        matches!(
            self.content,
            Content::Tex(_)
                | Content::Text(_)
                | Content::Code(_)
                | Content::Matrix(_)
                | Content::Row(_)
        ) && self.boxed.is_none()
    }

    /// A point, line, polyline or cuboid.
    fn geo(&self) -> bool {
        is_geo(&self.content)
    }

    /// A 3D label.
    fn label(&self) -> bool {
        matches!(self.content, Content::Label { .. })
    }

    /// Pixels per em after fitting.
    fn scale(&self) -> f32 {
        self.size * self.fit
    }
}

/// A running effect.
#[derive(Clone, Debug)]
enum Fx {
    Appear {
        id: String,
        style: Style,
        lag: f64,
        subset: Option<Vec<usize>>,
    },
    Vanish {
        draws: Vec<Draw>,
    },
    Morph {
        dst: String,
        set: Vec<usize>,
        src: Vec<DPrim>,
        pairs: Vec<(usize, usize)>,
        outs: Vec<usize>,
        ins: Vec<usize>,
        copy: bool,
    },
    Indicate {
        id: String,
        subset: Option<Vec<usize>>,
        color: Rgba,
    },
}

/// How an added object enters.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Style {
    Write,
    Fade,
    Grow,
    Pop,
    Draw,
    None,
}

/// An effect with its start and duration, milliseconds.
#[derive(Clone, Debug)]
struct Anim {
    fx: Fx,
    t0: f64,
    dur: f64,
}

impl Anim {
    /// Progress at `now`, 0..1.
    fn p(&self, now: f64) -> f64 {
        if self.dur <= 0.0 {
            1.0
        } else {
            ((now - self.t0) / self.dur).clamp(0.0, 1.0)
        }
    }
}

/// The named points and point objects at one time.
struct PointSet<'a> {
    points: &'a HashMap<String, Tween<[f64; 3]>>,
    objs: &'a [Obj],
    now: f64,
}

impl Points for PointSet<'_> {
    fn point(&self, name: &str) -> Option<[f64; 3]> {
        if let Some(t) = self.points.get(name) {
            return Some(t.at(self.now));
        }
        // a point object can be named like a point
        let o = self.objs.iter().find(|o| o.id == name)?;
        if let Content::Point(p) = &o.content {
            if let Pos::Name(n) = p
                && n == name
            {
                return None;
            }
            let inner = PointSet {
                points: self.points,
                objs: &[],
                now: self.now,
            };
            return eval::position(p, &inner).ok();
        }
        None
    }
}

/// A stage deck at one step, with its running animations.
pub struct Stage {
    pub fonts: Rc<Fonts>,
    pub index: Index,
    pub chapters: Vec<Chapter>,
    pub chapter: usize,
    pub step: Option<usize>,
    pub errors: Vec<String>,
    palette: HashMap<String, Rgba>,
    points: HashMap<String, Tween<[f64; 3]>>,
    pub objs: Vec<Obj>,
    aliases: HashMap<String, (String, String)>,
    cam: Tween<Cam>,
    pub user: [f64; 3],
    anims: Vec<Anim>,
    pending: Vec<(f64, Action)>,
    pub title: (String, String, f64),
    pub say: (String, String, f64),
    cache: HashMap<(String, u8, u32), Rc<Expr>>,
    labels: HashMap<String, [f32; 2]>,
    pub split: f32,     // Fraction of the slide width given to the 3D side.
    pub spatial: usize, // Draws before this index are clipped to the 3D side.
    pub scene: bool,    // The viewer's scene draws the 3D objects and its camera supplies `lens`.
    pub lens: Option<Lens>,
    pub objects3: Vec<Spatial>,
    ghosts: Vec<(Vec<Spatial>, f64, f64)>,
    pub cam_serial: u64,
    pub epoch: u64,
    pub now: f64,
    pub changed: bool,
}

/// Every step of a chapter as (slide, step).
fn steps_of(ch: &Chapter) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    for (s, slide) in ch.slides.iter().enumerate() {
        for t in 0..slide.steps.len() {
            out.push((s, t));
        }
    }
    out
}

/// Index pairs of a longest common subsequence.
fn lcs(a: &[u64], b: &[u64]) -> Vec<(usize, usize)> {
    let (n, m) = (a.len(), b.len());
    let mut dp = vec![0u16; (n + 1) * (m + 1)];
    for i in (0..n).rev() {
        for j in (0..m).rev() {
            dp[i * (m + 1) + j] = if a[i] == b[j] {
                dp[(i + 1) * (m + 1) + j + 1] + 1
            } else {
                dp[(i + 1) * (m + 1) + j].max(dp[i * (m + 1) + j + 1])
            };
        }
    }
    let (mut i, mut j) = (0, 0);
    let mut out = Vec::new();
    while i < n && j < m {
        if a[i] == b[j] {
            out.push((i, j));
            i += 1;
            j += 1;
        } else if dp[(i + 1) * (m + 1) + j] >= dp[i * (m + 1) + j + 1] {
            i += 1;
        } else {
            j += 1;
        }
    }
    out
}

/// Prims named by a morph map key: `[r,c]`, `M[r,:]`, `{part}`, a row item id or a part's TeX.
fn select(e: &Expr, groups: &HashMap<String, Vec<usize>>, key: &str) -> Vec<usize> {
    let k = key.trim();
    if let Some(sel) = k.strip_prefix('[') {
        let cells = sel.trim_end_matches(']').replace(' ', "");
        return groups.get(&cells).cloned().unwrap_or_default();
    }
    if let Some((id, sel)) = k.split_once('[') {
        let cells = sel.trim_end_matches(']').replace(' ', "");
        return groups
            .get(&format!("{id}/{cells}"))
            .cloned()
            .unwrap_or_default();
    }
    if let Some(part) = k.strip_prefix('{') {
        return e.part(&tex::normalize_key(part.strip_suffix('}').unwrap_or(part)));
    }
    match groups.get(k) {
        Some(g) => g.clone(),
        None => e.part(&tex::normalize_key(k)),
    }
}

/// Pair the pieces of two formulas: mapped parts, equal parts, then the longest common glyph run.
fn match_prims(
    src: &Expr,
    sset: &[usize],
    dst: &Expr,
    dset: &[usize],
    map: &[(String, String)],
    sg: &HashMap<String, Vec<usize>>,
    dg: &HashMap<String, Vec<usize>>,
) -> (Vec<(usize, usize)>, Vec<usize>, Vec<usize>) {
    let mut free_s: Vec<usize> = sset.to_vec();
    let mut free_d: Vec<usize> = dset.to_vec();
    let mut pairs = Vec::new();
    let pair_sets = |a: Vec<usize>,
                     b: Vec<usize>,
                     free_s: &mut Vec<usize>,
                     free_d: &mut Vec<usize>,
                     pairs: &mut Vec<(usize, usize)>| {
        let a: Vec<usize> = a.into_iter().filter(|i| free_s.contains(i)).collect();
        let b: Vec<usize> = b.into_iter().filter(|i| free_d.contains(i)).collect();
        let ia: Vec<u64> = a.iter().map(|&i| src.prims[i].shape.ident()).collect();
        let ib: Vec<u64> = b.iter().map(|&i| dst.prims[i].shape.ident()).collect();
        for (x, y) in lcs(&ia, &ib) {
            pairs.push((a[x], b[y]));
            free_s.retain(|&i| i != a[x]);
            free_d.retain(|&i| i != b[y]);
        }
    };
    for (a, b) in map {
        let (from, to) = (select(src, sg, a), select(dst, dg, b));
        pair_sets(from, to, &mut free_s, &mut free_d, &mut pairs);
    }
    let mut seen = HashSet::new();
    for key in &src.parts {
        if !seen.insert(key.clone()) || !dst.parts.contains(key) {
            continue;
        }
        pair_sets(
            src.part(key),
            dst.part(key),
            &mut free_s,
            &mut free_d,
            &mut pairs,
        );
    }
    let ia: Vec<u64> = free_s.iter().map(|&i| src.prims[i].shape.ident()).collect();
    let ib: Vec<u64> = free_d.iter().map(|&i| dst.prims[i].shape.ident()).collect();
    let rest = lcs(&ia, &ib);
    for &(x, y) in &rest {
        pairs.push((free_s[x], free_d[y]));
    }
    let used_s: HashSet<usize> = rest.iter().map(|&(x, _)| free_s[x]).collect();
    let used_d: HashSet<usize> = rest.iter().map(|&(_, y)| free_d[y]).collect();
    let outs = free_s
        .iter()
        .copied()
        .filter(|i| !used_s.contains(i))
        .collect();
    let ins = free_d
        .iter()
        .copied()
        .filter(|i| !used_d.contains(i))
        .collect();
    (pairs, outs, ins)
}

/// A formula piece in slide pixels: scaled by `k`, moved to (ox, oy), coloured.
fn scaled(p: &Prim, k: f32, ox: f32, oy: f32, color: Rgba) -> DPrim {
    let shape = match p.shape {
        Shape::Rect {
            w,
            h,
            radius,
            fill,
            stroke,
        } => Shape::Rect {
            w: w * k,
            h: h * k,
            radius: radius * k,
            fill,
            stroke: stroke * k,
        },
        other => other.resized(k),
    };
    DPrim {
        shape,
        x: ox + p.x * k,
        y: oy + p.y * k,
        color,
    }
}

/// Blend two pieces; the shape is the target's when the kinds differ.
fn lerp_prim(a: &DPrim, b: &DPrim, t: f32) -> DPrim {
    let shape = match (a.shape, b.shape) {
        (
            Shape::Glyph { size: s0, .. },
            Shape::Glyph {
                font,
                gid,
                size: s1,
            },
        ) => Shape::Glyph {
            font,
            gid,
            size: lerp(s0, s1, t),
        },
        (
            Shape::Rect {
                w: w0,
                h: h0,
                radius: r0,
                fill: f0,
                stroke: k0,
            },
            Shape::Rect {
                w,
                h,
                radius,
                fill,
                stroke,
            },
        ) => Shape::Rect {
            w: lerp(w0, w, t),
            h: lerp(h0, h, t),
            radius: lerp(r0, radius, t),
            fill: lerp(f0, fill, t),
            stroke: lerp(k0, stroke, t),
        },
        (
            Shape::Stroke {
                dx: x0,
                dy: y0,
                width: w0,
            },
            Shape::Stroke { dx, dy, width },
        ) => Shape::Stroke {
            dx: lerp(x0, dx, t),
            dy: lerp(y0, dy, t),
            width: lerp(w0, width, t),
        },
        (_, s) => s,
    };
    DPrim {
        shape,
        x: lerp(a.x, b.x, t),
        y: lerp(a.y, b.y, t),
        color: mix(a.color, b.color, t),
    }
}

/// The draw with its alpha scaled by `a`.
fn fade(mut d: Draw, a: f32) -> Draw {
    match &mut d {
        Draw::Prim(p) => p.color[3] *= a,
        Draw::Line { color, .. } | Draw::Path { color, .. } | Draw::Head { color, .. } => {
            color[3] *= a
        }
        Draw::Disc { color, ring, .. } => {
            color[3] *= a;
            ring[3] *= a;
        }
    }
    d
}

/// Outline of a rounded rectangle, seven points per corner.
pub fn rounded(x: f32, y: f32, w: f32, h: f32, r: f32) -> Vec<[f32; 2]> {
    let r = r.min(w / 2.0).min(h / 2.0).max(0.0);
    let mut pts = Vec::new();
    let corners = [
        (x + w - r, y + r, -90.0f32),
        (x + w - r, y + h - r, 0.0),
        (x + r, y + h - r, 90.0),
        (x + r, y + r, 180.0),
    ];
    for (cx, cy, a0) in corners {
        for i in 0..=6 {
            let a = (a0 + 15.0 * i as f32).to_radians();
            pts.push([cx + r * a.cos(), cy + r * a.sin()]);
        }
    }
    pts
}

/// The twelve edges of the box with opposite corners `a` and `b`.
fn cuboid_edges(a: [f64; 3], b: [f64; 3]) -> Vec<([f64; 3], [f64; 3])> {
    let corner = |i: usize| {
        [
            if i & 1 == 0 { a[0] } else { b[0] },
            if i & 2 == 0 { a[1] } else { b[1] },
            if i & 4 == 0 { a[2] } else { b[2] },
        ]
    };
    let mut edges = Vec::new();
    for i in 0..8usize {
        for bit in [1usize, 2, 4] {
            if i & bit == 0 {
                edges.push((corner(i), corner(i | bit)));
            }
        }
    }
    edges
}

impl Stage {
    /// A deck at the start of its first chapter.
    pub fn new(fonts: Rc<Fonts>, index: Index, chapters: Vec<Chapter>) -> Self {
        let mut s = Stage {
            fonts,
            index,
            chapters,
            chapter: 0,
            step: None,
            errors: Vec::new(),
            palette: HashMap::new(),
            points: HashMap::new(),
            objs: Vec::new(),
            aliases: HashMap::new(),
            cam: Tween::fixed(START_CAM),
            user: [0.0, 0.0, 1.0],
            anims: Vec::new(),
            pending: Vec::new(),
            title: (String::new(), String::new(), 0.0),
            say: (String::new(), String::new(), 0.0),
            cache: HashMap::new(),
            labels: HashMap::new(),
            split: 0.46,
            spatial: 0,
            scene: false,
            lens: None,
            objects3: Vec::new(),
            ghosts: Vec::new(),
            cam_serial: 0,
            epoch: 0,
            now: 0.0,
            changed: true,
        };
        s.reset(0);
        s
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Navigation
    // ═══════════════════════════════════════════════════════════════════════════

    /// Steps in the current chapter.
    pub fn steps(&self) -> usize {
        steps_of(&self.chapters[self.chapter]).len()
    }

    /// The current (slide, step within the slide).
    pub fn position(&self) -> (usize, usize) {
        let st = steps_of(&self.chapters[self.chapter]);
        self.step.map_or((0, 0), |g| st[g])
    }

    /// Whether anything is still moving.
    pub fn animating(&self) -> bool {
        !self.pending.is_empty()
            || !self.ghosts.is_empty()
            || self.anims.iter().any(|a| a.t0 + a.dur > self.now)
            || self.cam.busy(self.now)
            || self.points.values().any(|p| p.busy(self.now))
            || self.objs.iter().any(|o| {
                o.pos.busy(self.now)
                    || o.dim.busy(self.now)
                    || o.colors.values().any(|c| c.busy(self.now))
            })
            || self.now < self.title.2 + 650.0
            || self.now < self.say.2 + 650.0
    }

    /// Fresh chapter state before its first step.
    fn reset(&mut self, chapter: usize) {
        self.chapter = chapter.min(self.chapters.len() - 1);
        let ch = &self.chapters[self.chapter];
        self.palette = ch.palette.clone();
        self.points = ch
            .points
            .iter()
            .map(|(k, v)| (k.clone(), Tween::fixed(*v)))
            .collect();
        self.split = ch.split;
        self.objs.clear();
        self.aliases.clear();
        self.anims.clear();
        self.pending.clear();
        self.labels.clear();
        self.step = None;
        let first = ch.slides[0].title.clone();
        self.title = (first, String::new(), 0.0);
        self.say = (String::new(), String::new(), 0.0);
        let spec = ch.camera.clone();
        let cam = self.camera_for(&spec, START_CAM, true);
        self.cam = Tween::fixed(cam);
        self.cam_serial += 1;
        self.epoch += 1;
        self.ghosts.clear();
        self.user = [0.0, 0.0, 1.0];
        self.changed = true;
    }

    /// Advance: finish a running step first, then play the next.
    pub fn next(&mut self, now: f64) {
        self.now = now;
        if self.animating() {
            self.finish();
        }
        let n = self.steps();
        let g = self.step.map_or(0, |g| g + 1);
        if g < n {
            self.begin(g, now, true);
        } else if self.chapter + 1 < self.chapters.len() {
            self.reset(self.chapter + 1);
            self.begin(0, now, true);
        }
    }

    /// Show the end of the step before, without animation.
    pub fn prev(&mut self, now: f64) {
        self.now = now;
        match self.step {
            Some(g) if g > 0 => self.rebuild(self.chapter, Some(g - 1)),
            Some(_) => self.rebuild(self.chapter, None),
            None if self.chapter > 0 => {
                let last = steps_of(&self.chapters[self.chapter - 1]).len() - 1;
                self.rebuild(self.chapter - 1, Some(last));
            }
            None => {}
        }
    }

    /// Jump to the first step of the next or previous slide and play it.
    pub fn slide(&mut self, delta: i32, now: f64) {
        self.now = now;
        let st = steps_of(&self.chapters[self.chapter]);
        let (s, _) = self.position();
        let unopened = if self.step.is_none() && delta > 0 {
            -1
        } else {
            0
        };
        let target = s as i32 + delta + unopened;
        if target < 0 {
            self.rebuild(self.chapter, None);
            return;
        }
        match st.iter().position(|&(ss, _)| ss as i32 == target) {
            Some(g) => {
                self.rebuild(self.chapter, if g == 0 { None } else { Some(g - 1) });
                self.begin(g, now, true);
            }
            None if delta > 0 && self.chapter + 1 < self.chapters.len() => {
                self.chapter_go(self.chapter + 1, now)
            }
            None => {}
        }
    }

    /// Open chapter `c` and play its first step.
    pub fn chapter_go(&mut self, c: usize, now: f64) {
        self.now = now;
        self.reset(c);
        self.begin(0, now, true);
    }

    /// Show the end of step `g` (or the empty chapter) without animation.
    pub fn rebuild(&mut self, chapter: usize, g: Option<usize>) {
        self.reset(chapter);
        if let Some(g) = g {
            for k in 0..=g {
                self.begin(k, self.now, false);
            }
        }
        self.changed = true;
    }

    /// Jump to a 1-based chapter and step from the page address.
    pub fn open_at(&mut self, chapter: usize, step: usize, now: f64) {
        self.now = now;
        let c = chapter.saturating_sub(1).min(self.chapters.len() - 1);
        let n = steps_of(&self.chapters[c]).len();
        if step == 0 {
            self.reset(c);
            self.begin(0, now, true);
        } else {
            self.rebuild(c, Some((step - 1).min(n - 1)));
        }
    }

    /// Apply every pending action and end every animation.
    fn finish(&mut self) {
        let pending = std::mem::take(&mut self.pending);
        for (_, a) in pending {
            self.apply(&a, self.now, false);
        }
        self.anims.clear();
        self.cam.snap();
        for p in self.points.values_mut() {
            p.snap();
        }
        for o in &mut self.objs {
            o.pos.snap();
            o.dim.snap();
            for c in o.colors.values_mut() {
                c.snap();
            }
            o.geo_from = None;
        }
        self.title.2 = -1e9;
        self.say.2 = -1e9;
        self.changed = true;
    }

    /// Start step `g`: schedule its actions from `now`, or apply them at once.
    fn begin(&mut self, g: usize, now: f64, animate: bool) {
        let st = steps_of(&self.chapters[self.chapter]);
        let Some(&(s, t)) = st.get(g) else { return };
        let slide = self.chapters[self.chapter].slides[s].clone();
        let step = slide.steps[t].clone();
        self.step = Some(g);
        let mut actions: Vec<Action> = Vec::new();
        if t == 0 && s > 0 && !slide.keep {
            actions.push(Action {
                kind: Kind::Clear,
                dur: Some(0.35),
                with: false,
                delay: 0.0,
            });
        }
        if self.title.0 != slide.title {
            let old = std::mem::replace(&mut self.title.0, slide.title.clone());
            self.title.1 = old;
            self.title.2 = if animate { now } else { -1e9 };
        }
        let say = step.say.clone().unwrap_or_default();
        if self.say.0 != say {
            let old = std::mem::replace(&mut self.say.0, say);
            self.say.1 = old;
            self.say.2 = if animate { now } else { -1e9 };
        }
        for a in step.actions {
            if let Kind::Play(children) = a.kind {
                for (i, mut c) in children.into_iter().enumerate() {
                    c.with = i > 0 || a.with;
                    c.delay += if i == 0 { a.delay } else { 0.0 };
                    actions.push(c);
                }
            } else {
                actions.push(a);
            }
        }
        if !animate {
            for a in &actions {
                self.apply(a, now, false);
            }
            self.finish();
            return;
        }
        let mut t_end = now + 60.0;
        let mut prev_start = t_end;
        for a in actions {
            let start = if a.with { prev_start } else { t_end } + a.delay * 1000.0;
            let dur = self.duration(&a);
            prev_start = start;
            t_end = t_end.max(start + dur);
            self.pending.push((start, a));
        }
        self.changed = true;
    }

    /// How long an action takes, milliseconds.
    fn duration(&self, a: &Action) -> f64 {
        if let Some(d) = a.dur {
            return d * 1000.0;
        }
        1000.0
            * match &a.kind {
                Kind::Add(objs) => {
                    if objs
                        .iter()
                        .all(|o| matches!(o.content, Content::Point(_) | Content::Label { .. }))
                    {
                        0.4
                    } else if objs
                        .iter()
                        .any(|o| matches!(o.content, Content::Matrix(_) | Content::Row(_)))
                    {
                        0.8
                    } else {
                        0.6
                    }
                }
                Kind::Remove(_) => 0.35,
                Kind::Morph { .. } | Kind::Derive { .. } | Kind::Set { .. } => 0.8,
                Kind::Fly { .. } => 0.8,
                Kind::Indicate(..) => 0.6,
                Kind::Box { .. } => 0.7,
                Kind::Color { .. } => 0.35,
                Kind::Move { .. } => 1.0,
                Kind::Camera(_) => 1.0,
                Kind::Wait => 1.0,
                Kind::Dim(_) | Kind::Undim(_) => 0.3,
                Kind::Clear => 0.35,
                Kind::Show(_) | Kind::Hide(_) => 0.4,
                Kind::Play(_) => 1.0,
            }
    }

    /// Advance the clock: run actions whose time has come and drop finished animations.
    pub fn tick(&mut self, now: f64) {
        self.now = now;
        while let Some(i) = self.pending.iter().position(|(t, _)| *t <= now) {
            let (t, a) = self.pending.remove(i);
            self.apply(&a, t, true);
            self.changed = true;
        }
        self.anims.retain(|a| a.t0 + a.dur > now);
        for o in &mut self.objs {
            if o.geo_from.as_ref().is_some_and(|(_, t0, d)| t0 + d <= now) {
                o.geo_from = None;
            }
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Objects
    // ═══════════════════════════════════════════════════════════════════════════

    /// A palette colour or hex colour, `fallback` when `name` is None.
    fn color(&self, name: Option<&str>, fallback: &str) -> Rgba {
        let name = name.unwrap_or(fallback);
        self.palette
            .get(name)
            .copied()
            .or_else(|| tex::hex(name))
            .unwrap_or([1.0; 4])
    }

    /// Cached layout: mode 0 display TeX, 1 inline TeX, 2 prose, 3 bold prose.
    fn typeset(&mut self, src: &str, mode: u8, width: f32) -> Rc<Expr> {
        let key = (src.to_string(), mode, width.to_bits());
        if let Some(e) = self.cache.get(&key) {
            return e.clone();
        }
        let e = Rc::new(match mode {
            0 => tex::typeset(&self.fonts, src, &self.palette, false),
            1 => tex::typeset(&self.fonts, src, &self.palette, true),
            2 => tex::prose(&self.fonts, src, &self.palette, width, false),
            _ => {
                let bold = format!("**{src}**");
                tex::prose(&self.fonts, &bold, &self.palette, width, false)
            }
        });
        for u in &e.unknown {
            let msg = format!("unknown TeX command {u} in `{src}`");
            if !self.errors.contains(&msg) {
                self.errors.push(msg);
            }
        }
        if self.cache.len() > 4000 {
            self.cache.clear();
        }
        self.cache.insert(key, e.clone());
        e
    }

    /// Left and right edge of the column.
    fn panel(&self) -> (f32, f32) {
        (self.split * W + MARGIN * 0.6, W - MARGIN)
    }

    /// A matrix as rounded cells with centered entries, em units.
    fn matrix(&mut self, spec: &MatrixSpec) -> (Expr, HashMap<String, Vec<usize>>) {
        let e = 0.8f32;
        let blank = Rc::new(Expr::default());
        let entries: Vec<Vec<Rc<Expr>>> = spec
            .rows
            .iter()
            .map(|r| {
                r.iter()
                    .map(|t| {
                        if spec.pattern {
                            blank.clone()
                        } else {
                            self.typeset(t, 1, 0.0)
                        }
                    })
                    .collect()
            })
            .collect();
        let maxw = entries
            .iter()
            .flatten()
            .map(|x| x.w * e)
            .fold(0.0f32, f32::max);
        let maxh = entries
            .iter()
            .flatten()
            .map(|x| (x.h + x.d) * e)
            .fold(0.0f32, f32::max);
        let longest = spec.rows.len().max(spec.rows[0].len()) as f32;
        let ch = match spec.cell {
            Some(c) => c as f32,
            None if spec.pattern => 9.0 / longest,
            None => 1.3f32.max(maxh + 0.5),
        };
        let cw = if spec.pattern {
            ch
        } else if maxw + 0.45 > ch * 1.15 {
            maxw + 0.5
        } else {
            ch.max(maxw + 0.45)
        };
        let g = if spec.pattern { 0.0 } else { 0.07 * ch.min(cw) };
        let (rows, cols) = (spec.rows.len(), spec.rows[0].len());
        let mw = cols as f32 * cw + (cols - 1) as f32 * g;
        let mh = rows as f32 * ch + (rows - 1) as f32 * g;
        let axis = self.fonts.c.axis;
        let ls = 0.62f32;
        let rl: Vec<Rc<Expr>> = spec
            .rowlabels
            .iter()
            .map(|t| self.typeset(t, 1, 0.0))
            .collect();
        let cl: Vec<Rc<Expr>> = spec
            .collabels
            .iter()
            .map(|t| self.typeset(t, 1, 0.0))
            .collect();
        let rlw = rl.iter().map(|x| x.w * ls).fold(0.0f32, f32::max);
        let left = if rl.is_empty() { 0.0 } else { rlw + 0.3 };
        let top = -(axis + mh / 2.0);
        let mut out = Expr {
            w: left + mw,
            h: axis + mh / 2.0,
            d: mh / 2.0 - axis,
            ..Expr::default()
        };
        let mut groups: HashMap<String, Vec<usize>> = HashMap::new();
        let push = |out: &mut Expr, p: Prim| {
            out.prims.push(p);
            out.prims.len() - 1
        };
        let neutral = self.color(Some("cell"), "cell");
        let mut cell_idx = vec![vec![usize::MAX; cols]; rows];
        if spec.pattern {
            push(
                &mut out,
                Prim {
                    shape: Shape::Rect {
                        w: mw,
                        h: mh,
                        radius: 0.12,
                        fill: 1.0,
                        stroke: 0.0,
                    },
                    x: left,
                    y: top,
                    color: Some(neutral),
                    part: None,
                },
            );
        }
        for r in 0..rows {
            for c in 0..cols {
                let ascii = spec.rows[r][c].replace('−', "-");
                let value = ascii.trim().parse::<f64>().ok();
                let named = spec
                    .colors
                    .get(r)
                    .and_then(|row| row.get(c))
                    .cloned()
                    .flatten();
                let entry = spec.rows[r][c].trim();
                let sign = match value {
                    Some(v) if v > 1e-12 => 1,
                    Some(v) if v < -1e-12 => -1,
                    Some(_) => 0,
                    None if entry.is_empty() => 0,
                    None if entry.starts_with('-') => -1,
                    None => 1,
                };
                let color = match (named, sign) {
                    (Some(n), _) => self.color(Some(&n), "cell"),
                    (None, 1) if spec.signs => self.color(Some("pos"), "pos"),
                    (None, -1) if spec.signs => self.color(Some("neg"), "neg"),
                    _ => neutral,
                };
                if spec.pattern && sign == 0 {
                    continue;
                }
                let x = left + c as f32 * (cw + g);
                let y = top + r as f32 * (ch + g);
                cell_idx[r][c] = push(
                    &mut out,
                    Prim {
                        shape: Shape::Rect {
                            w: cw,
                            h: ch,
                            radius: if spec.pattern { 0.0 } else { 0.16 * ch.min(cw) },
                            fill: 1.0,
                            stroke: 0.0,
                        },
                        x,
                        y,
                        color: Some(color),
                        part: None,
                    },
                );
            }
        }
        for r in 0..rows {
            for c in 0..cols {
                let ex = &entries[r][c];
                let x = left + c as f32 * (cw + g) + (cw - ex.w * e) / 2.0;
                let y = top + r as f32 * (ch + g) + ch / 2.0 + (ex.h - ex.d) * e / 2.0;
                if cell_idx[r][c] == usize::MAX {
                    continue;
                }
                let mut ids = vec![cell_idx[r][c]];
                for p in &ex.prims {
                    let mut q = *p;
                    q.x = x + p.x * e;
                    q.y = y + p.y * e;
                    q.shape = p.shape.resized(e);
                    q.part = None;
                    ids.push(push(&mut out, q));
                }
                for k in [format!("{r},{c}"), format!("{r},:"), format!(":,{c}")] {
                    groups.entry(k).or_default().extend(ids.iter().copied());
                }
                groups
                    .entry("entries".into())
                    .or_default()
                    .extend(ids[1..].iter().copied());
                groups
                    .entry("cells".into())
                    .or_default()
                    .push(cell_idx[r][c]);
            }
        }
        let dim = self.color(Some("dim"), "dim");
        for (r, lab) in rl.iter().enumerate() {
            let y = top + r as f32 * (ch + g) + ch / 2.0 + (lab.h - lab.d) * ls / 2.0;
            let x = left - 0.3 - lab.w * ls;
            for p in &lab.prims {
                let mut q = *p;
                q.x = x + p.x * ls;
                q.y = y + p.y * ls;
                if let Shape::Glyph { font, gid, size } = p.shape {
                    q.shape = Shape::Glyph {
                        font,
                        gid,
                        size: size * ls,
                    };
                }
                q.color = q.color.or(Some(dim));
                let i = push(&mut out, q);
                groups.entry("labels".into()).or_default().push(i);
            }
        }
        for (c, lab) in cl.iter().enumerate() {
            let y = top - 0.28;
            let x = left + c as f32 * (cw + g) + (cw - lab.w * ls) / 2.0;
            out.h = out.h.max(-(y - lab.h * ls));
            for p in &lab.prims {
                let mut q = *p;
                q.x = x + p.x * ls;
                q.y = y - lab.d * ls + p.y * ls;
                if let Shape::Glyph { font, gid, size } = p.shape {
                    q.shape = Shape::Glyph {
                        font,
                        gid,
                        size: size * ls,
                    };
                }
                q.color = q.color.or(Some(dim));
                let i = push(&mut out, q);
                groups.entry("labels".into()).or_default().push(i);
            }
        }
        if let Some(name) = &spec.name {
            let nb = self.typeset(name, 1, 0.0);
            let ns = 0.9;
            let y = top + mh + 0.3 + nb.h * ns;
            let x = left + (mw - nb.w * ns) / 2.0;
            out.d = y + nb.d * ns;
            for p in &nb.prims {
                let mut q = *p;
                q.x = x + p.x * ns;
                q.y = y + p.y * ns;
                q.shape = match p.shape {
                    Shape::Glyph { font, gid, size } => Shape::Glyph {
                        font,
                        gid,
                        size: size * ns,
                    },
                    s => s,
                };
                let i = push(&mut out, q);
                groups.entry("name".into()).or_default().push(i);
            }
            out.w = out.w.max(x + nb.w * ns);
        }
        // a name or label wider than the cells must not reach into the neighbours
        let min_x = out.prims.iter().map(|p| p.x).fold(0.0f32, f32::min);
        if min_x < 0.0 {
            for p in &mut out.prims {
                p.x -= min_x;
            }
            out.w -= min_x;
        }
        (out, groups)
    }

    /// Lay out content `size` pixels per em, with its named prim groups.
    fn compile(
        &mut self,
        content: &Content,
        size: f32,
        row_color: Option<Rgba>,
    ) -> (Rc<Expr>, HashMap<String, Vec<usize>>) {
        let mut groups: HashMap<String, Vec<usize>> = HashMap::new();
        let parts = |e: &Expr, groups: &mut HashMap<String, Vec<usize>>| {
            for key in &e.parts {
                groups.insert(format!("part:{key}"), e.part(key));
            }
        };
        let expr = match content {
            Content::Tex(s) => self.typeset(s, 0, 0.0),
            Content::Label { tex, .. } => self.typeset(tex, 1, 0.0),
            Content::Text(s) => {
                let (x0, x1) = self.panel();
                self.typeset(s, 2, (x1 - x0) / size)
            }
            Content::Code(s) => Rc::new(tex::code(&self.fonts, s, &self.palette)),
            Content::Matrix(spec) => {
                let (e, g) = self.matrix(spec);
                groups = g;
                Rc::new(e)
            }
            Content::Row(items) => {
                let mut out = Expr::default();
                let mut x = 0.0f32;
                let mut prev_matrix = false;
                for (i, item) in items.iter().enumerate() {
                    let is_matrix = matches!(item.content, Content::Matrix(_));
                    let gap = if is_matrix && prev_matrix { 0.42 } else { 0.3 };
                    prev_matrix = is_matrix;
                    let color = item
                        .color
                        .as_deref()
                        .map(|c| self.color(Some(c), "ink"))
                        .or(row_color);
                    let (e, g) = self.compile(&item.content, size, None);
                    if i > 0 {
                        x += gap;
                    }
                    let base = out.prims.len();
                    for p in &e.prims {
                        let mut q = *p;
                        q.x += x;
                        if q.color.is_none() {
                            q.color = color;
                        }
                        q.part = None;
                        out.prims.push(q);
                    }
                    let range: Vec<usize> = (base..out.prims.len()).collect();
                    if let Some(id) = &item.id {
                        groups.insert(id.clone(), range.clone());
                        for (k, v) in &g {
                            groups
                                .insert(format!("{id}/{k}"), v.iter().map(|i| i + base).collect());
                        }
                    }
                    for key in &e.parts {
                        groups
                            .entry(format!("part:{key}"))
                            .or_default()
                            .extend(e.part(key).iter().map(|i| i + base));
                    }
                    out.h = out.h.max(e.h);
                    out.d = out.d.max(e.d);
                    x += e.w;
                }
                out.w = x;
                Rc::new(out)
            }
            _ => Rc::new(Expr::default()),
        };
        parts(&expr, &mut groups);
        (expr, groups)
    }

    /// The index of the object `id`.
    fn find(&self, id: &str) -> Option<usize> {
        self.objs.iter().position(|o| o.id == id)
    }

    /// The object and prims of `id`, `M[r,c]`, `M[r,:]`, `eq{x_k}` or a row item id; None for all.
    fn resolve(&self, r: &str) -> Option<(usize, Option<Vec<usize>>)> {
        let r = r.trim();
        let cut = r.find(['[', '{']).unwrap_or(r.len());
        let (base, sel) = (&r[..cut], &r[cut..]);
        let (oi, prefix) = match self.find(base) {
            Some(i) => (i, String::new()),
            None => {
                let (obj, prefix) = self.aliases.get(base)?;
                (self.find(obj)?, prefix.clone())
            }
        };
        let key = if let Some(s) = sel.strip_prefix('[') {
            s.trim_end_matches(']').replace(' ', "")
        } else if let Some(s) = sel.strip_prefix('{') {
            format!(
                "part:{}",
                tex::normalize_key(s.strip_suffix('}').unwrap_or(s))
            )
        } else {
            String::new()
        };
        let o = &self.objs[oi];
        let full = match (prefix.is_empty(), key.is_empty()) {
            (true, true) => return Some((oi, None)),
            (true, false) => key,
            (false, true) => prefix,
            (false, false) => format!("{prefix}/{key}"),
        };
        o.groups.get(&full).map(|g| (oi, Some(g.clone())))
    }

    /// Build an object from its spec.
    fn make(&mut self, spec: &ObjSpec) -> Obj {
        let label = matches!(spec.content, Content::Label { .. });
        let geo = is_geo(&spec.content);
        let size = spec.size.map(|s| s as f32).unwrap_or(match &spec.content {
            Content::Label { .. } => 24.0,
            Content::Text(_) => 25.0,
            Content::Code(_) => 19.0,
            Content::Matrix(_) | Content::Row(_) => 30.0,
            _ => 34.0,
        });
        let fallback = if geo { "bar" } else { "ink" };
        let color = self.color(spec.color.as_deref(), fallback);
        let (content, dynamic) = self.filled(&spec.content);
        let (expr, groups) = self.compile(&content, size, None);
        let mut hidden = HashSet::new();
        if let Content::Matrix(m) = &spec.content
            && m.empty
        {
            hidden.extend(groups.get("entries").cloned().unwrap_or_default());
        }
        if let Content::Row(items) = &spec.content {
            for it in items {
                if let (Some(id), Content::Matrix(m)) = (&it.id, &it.content)
                    && m.empty
                    && let Some(entries) = groups.get(&format!("{id}/entries"))
                {
                    hidden.extend(entries.iter().copied());
                }
            }
        }
        let width = spec.width.map(|w| w as f32).unwrap_or(
            if matches!(spec.content, Content::Cuboid { .. }) {
                1.6
            } else {
                3.2
            },
        );
        Obj {
            id: spec.id.clone(),
            content,
            color,
            size,
            width,
            dash: spec.dash || matches!(spec.content, Content::Cuboid { .. }),
            expr,
            groups,
            dynamic,
            hidden,
            colors: HashMap::new(),
            align: match spec.align.as_deref() {
                Some("=") | Some("eq") | Some("equals") => Align::Equals,
                Some("left") => Align::Left,
                None if matches!(spec.content, Content::Code(_)) => Align::Left,
                _ => Align::Center,
            },
            at: spec.at.map(|a| [a[0] as f32, a[1] as f32]),
            gap: spec
                .gap
                .map(|g| g as f32)
                .unwrap_or(if label { 0.0 } else { 24.0 }),
            pos: Tween::fixed([0.0, 0.0]),
            placed: false,
            fit: 1.0,
            dim: Tween::fixed(0.0),
            opacity: spec.opacity.map(|o| o as f32).unwrap_or(1.0),
            geo_from: None,
            dir: spec.dir.map(|d| [d[0] as f32, d[1] as f32]),
            attach: None,
            boxed: None,
            label_dir: 0,
        }
    }

    /// Content with its `{=expr}` fields filled in, and its template when it has any.
    fn filled(&self, content: &Content) -> (Content, Option<String>) {
        let dynamic = match content {
            Content::Tex(s) | Content::Label { tex: s, .. } if s.contains("{=") => Some(s.clone()),
            _ => None,
        };
        let content = match (content, &dynamic) {
            (Content::Tex(_), Some(d)) => Content::Tex(self.fill(d)),
            (Content::Label { at, .. }, Some(d)) => Content::Label {
                at: at.clone(),
                tex: self.fill(d),
            },
            (c, _) => c.clone(),
        };
        (content, dynamic)
    }

    /// TeX with its `{=expr}` fields filled at the current point values.
    fn fill(&self, src: &str) -> String {
        eval::fill(src, &self.points_at(self.now))
    }

    /// The named points and point objects at `now`.
    fn points_at(&self, now: f64) -> PointSet<'_> {
        PointSet {
            points: &self.points,
            objs: &self.objs,
            now,
        }
    }

    /// Add an object, replacing one with the same id but keeping its place.
    fn insert(&mut self, mut obj: Obj) {
        if let Some(i) = self.find(&obj.id) {
            let old = self.objs.remove(i);
            obj.pos = old.pos;
            obj.placed = old.placed && obj.block();
        }
        if let Content::Row(items) = &obj.content {
            for it in items {
                if let Some(id) = &it.id {
                    self.aliases
                        .insert(id.clone(), (obj.id.clone(), id.clone()));
                }
            }
        }
        self.objs.push(obj);
    }

    /// Place every panel block: stacked from the top, equals aligned, wide blocks shrunk.
    fn relayout(&mut self, now: f64, animate: bool) {
        let (x0, x1) = self.panel();
        let pw = x1 - x0;
        let idx: Vec<usize> = (0..self.objs.len())
            .filter(|&i| self.objs[i].block() && self.objs[i].at.is_none())
            .collect();
        for &i in &idx {
            let o = &mut self.objs[i];
            o.fit = (pw / (o.expr.w * o.size).max(1.0)).min(1.0);
        }
        let mut lefts = vec![0.0f32; idx.len()];
        let mut k = 0;
        while k < idx.len() {
            let o = &self.objs[idx[k]];
            if o.align == Align::Equals && o.expr.align.is_some() {
                let mut end = k;
                while end < idx.len()
                    && self.objs[idx[end]].align == Align::Equals
                    && self.objs[idx[end]].expr.align.is_some()
                {
                    end += 1;
                }
                // the group shares one equals column and shrinks together when it is too wide
                let measure = |s: &Stage| {
                    let l = (k..end)
                        .map(|j| {
                            let o = &s.objs[idx[j]];
                            o.expr.align.unwrap() * o.scale()
                        })
                        .fold(0.0f32, f32::max);
                    let r = (k..end)
                        .map(|j| {
                            let o = &s.objs[idx[j]];
                            (o.expr.w - o.expr.align.unwrap()) * o.scale()
                        })
                        .fold(0.0f32, f32::max);
                    (l, r)
                };
                let (mut l, mut r) = measure(self);
                if l + r > pw {
                    let f = pw / (l + r);
                    for j in k..end {
                        let o = &mut self.objs[idx[j]];
                        o.fit = (o.fit * f).min(f);
                    }
                    (l, r) = measure(self);
                }
                let left = x0 + ((pw - l - r) / 2.0).max(0.0);
                for j in k..end {
                    let o = &self.objs[idx[j]];
                    lefts[j] = left + l - o.expr.align.unwrap() * o.scale();
                }
                k = end;
            } else {
                let w = o.expr.w * o.scale();
                lefts[k] = if o.align == Align::Left {
                    x0
                } else {
                    x0 + (pw - w) / 2.0
                };
                k += 1;
            }
        }
        let mut y = TOP;
        let mut tops = Vec::new();
        for (j, &i) in idx.iter().enumerate() {
            let o = &self.objs[i];
            if j > 0 {
                y += o.gap;
            }
            tops.push(y);
            y += (o.expr.h + o.expr.d) * o.scale();
        }
        let avail = H - BOTTOM - TOP;
        let shift = (y - TOP - avail).max(0.0);
        for (j, &i) in idx.iter().enumerate() {
            let target = [lefts[j], tops[j] - shift];
            let o = &mut self.objs[i];
            if !o.placed || !animate {
                o.pos = Tween::fixed(target);
                o.placed = true;
            } else if (o.pos.to[0] - target[0]).abs() > 0.1 || (o.pos.to[1] - target[1]).abs() > 0.1
            {
                o.pos.set(target, now, RELAYOUT_MS);
            }
        }
        for o in &mut self.objs {
            if let (true, Some(at)) = (o.block(), o.at) {
                o.pos = Tween::fixed(at);
                o.placed = true;
            }
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Geometry
    // ═══════════════════════════════════════════════════════════════════════════

    /// The authored camera with the user's orbit and zoom applied.
    fn view(&self, now: f64) -> View {
        let mut cam = self.cam.at(now);
        cam.yaw += self.user[0];
        cam.pitch = (cam.pitch + self.user[1]).clamp(-89.0, 89.9);
        cam.zoom *= self.user[2];
        let mut v = View::new(cam, self.origin());
        v.lens = self.lens;
        v
    }

    /// The authored camera at `now`, and whether it is still moving.
    pub fn camera(&self, now: f64) -> (Cam, bool) {
        (self.cam.at(now), self.cam.busy(now))
    }

    /// Where the camera centre sits on the slide.
    pub fn origin(&self) -> [f32; 2] {
        [self.split * W / 2.0, (TOP - 30.0 + H - 70.0) / 2.0]
    }

    /// The camera a spec asks for from `cur`; `fit_all` frames every point by default.
    fn camera_for(&self, spec: &CameraSpec, cur: Cam, fit_all: bool) -> Cam {
        let mut cam = cur;
        if let Some(v) = &spec.view {
            let (y, p) = match v.as_str() {
                "front" => (-90.0, 0.0),
                "back" => (90.0, 0.0),
                "top" => (-90.0, 89.9),
                "side" | "right" => (0.0, 0.0),
                "left" => (180.0, 0.0),
                _ => (-58.0, 26.0),
            };
            cam.yaw = y;
            cam.pitch = p;
        }
        if let Some(y) = spec.yaw {
            cam.yaw = y;
        }
        if let Some(p) = spec.pitch {
            cam.pitch = p;
        }
        let ps = self.points_at(self.now);
        let mut fit: Vec<[f64; 3]> = spec
            .fit
            .iter()
            .filter_map(|p| eval::position(p, &ps).ok())
            .collect();
        if fit.is_empty() && fit_all && spec.zoom.is_none() {
            fit = self.points.values().map(|t| t.to).collect();
        }
        if !fit.is_empty() {
            let n = fit.len() as f64;
            let mut c = [0.0; 3];
            for p in &fit {
                for k in 0..3 {
                    c[k] += p[k] / n;
                }
            }
            cam.center = c;
            cam.zoom = 1.0;
            let v = View::new(cam, [0.0, 0.0]);
            let pr: Vec<[f32; 2]> = fit.iter().map(|&p| v.project(p)).collect();
            let (mut lo, mut hi) = ([f32::MAX; 2], [f32::MIN; 2]);
            for p in &pr {
                for k in 0..2 {
                    lo[k] = lo[k].min(p[k]);
                    hi[k] = hi[k].max(p[k]);
                }
            }
            let (bx, by) = (
                ((lo[0] + hi[0]) / 2.0) as f64,
                ((lo[1] + hi[1]) / 2.0) as f64,
            );
            for k in 0..3 {
                cam.center[k] += v.right[k] * bx - v.up[k] * by;
            }
            let pw = self.split * W - 120.0;
            let ph = H - TOP - 70.0 - 90.0;
            let zx = pw / (hi[0] - lo[0]).max(1e-3);
            let zy = ph / (hi[1] - lo[1]).max(1e-3);
            cam.zoom = zx.min(zy) as f64;
        }
        if let Some(c) = &spec.center
            && let Ok(p) = eval::position(c, &ps)
        {
            cam.center = p;
        }
        if let Some(z) = spec.zoom {
            cam.zoom = z;
        }
        cam
    }

    /// Control points of a 3D object at the current point values.
    fn ctrl(&self, o: &Obj, now: f64) -> Vec<[f64; 3]> {
        let ps = self.points_at(now);
        let p = |pos: &Pos| eval::position(pos, &ps).unwrap_or([0.0; 3]);
        match &o.content {
            Content::Point(a) => vec![p(a)],
            Content::Line { a, b, .. } | Content::Cuboid { a, b } => vec![p(a), p(b)],
            Content::Polyline { pts, .. } => pts.iter().map(p).collect(),
            Content::Label { at, .. } => match &o.attach {
                Some(id) => self.find(id).map(|i| {
                    let c = self.ctrl(&self.objs[i], now);
                    let n = c.len().max(1) as f64;
                    let mut m = [0.0; 3];
                    for q in &c {
                        for k in 0..3 {
                            m[k] += q[k] / n;
                        }
                    }
                    vec![m]
                }),
                None => None,
            }
            .unwrap_or_else(|| vec![p(at)]),
            _ => Vec::new(),
        }
    }

    /// Control points during a morph, blended by the kernel for points, lines and polylines.
    fn geo_points(&self, o: &Obj, now: f64) -> Vec<[f64; 3]> {
        use session_rust::{Line, Point, Polyline};
        let live = self.ctrl(o, now);
        let Some((from, t0, dur)) = &o.geo_from else {
            return live;
        };
        let t = smooth(((now - t0) / dur).clamp(0.0, 1.0));
        let pt = |p: &[f64; 3]| Point::new(p[0], p[1], p[2]);
        let xyz = |p: Point| [p[0], p[1], p[2]];
        match (from.as_slice(), live.as_slice()) {
            ([a], [b]) => vec![xyz(Point::lerp(&pt(a), &pt(b), t))],
            ([a0, a1], [b0, b1]) => {
                let from = Line::from_points(&pt(a0), &pt(a1));
                let to = Line::from_points(&pt(b0), &pt(b1));
                let line = Line::lerp(&from, &to, t);
                vec![xyz(line.start()), xyz(line.end())]
            }
            (a, b) if a.len() >= 2 && b.len() >= 2 => {
                let from = Polyline::new(a.iter().map(pt).collect());
                let to = Polyline::new(b.iter().map(pt).collect());
                let morph = Polyline::morph(&from, &to, t);
                morph.get_points().into_iter().map(xyz).collect()
            }
            _ => live,
        }
    }

    /// A 3D object in world space for the scene: cuboids become twelve dashed edges.
    fn geo_spatial(&self, o: &Obj, now: f64, grow: f32, alpha: f32) -> Vec<Spatial> {
        let pts = self.geo_points(o, now);
        let base = o.colors.get(&usize::MAX).map_or(o.color, |c| c.at(now));
        let color = with_alpha(base, alpha * o.opacity * (1.0 - 0.7 * o.dim.at(now)));
        let one = |kind, pts: Vec<[f64; 3]>, id: String, dash: bool| Spatial {
            id,
            kind,
            pts,
            color,
            width: o.width,
            dash,
            reveal: grow.clamp(0.0, 1.0),
        };
        match &o.content {
            Content::Point(_) => vec![one(SpatialKind::Point, pts, o.id.clone(), false)],
            Content::Line { arrow, .. } => {
                let _ = arrow;
                vec![one(SpatialKind::Line, pts, o.id.clone(), o.dash)]
            }
            Content::Polyline { closed, .. } => {
                let mut pts = pts;
                if *closed && !pts.is_empty() {
                    pts.push(pts[0]);
                }
                vec![one(SpatialKind::Polyline, pts, o.id.clone(), o.dash)]
            }
            Content::Cuboid { .. } => {
                let mut out = Vec::new();
                for (k, (p, q)) in cuboid_edges(pts[0], pts[1]).into_iter().enumerate() {
                    let id = format!("{}#{k}", o.id);
                    out.push(one(SpatialKind::Line, vec![p, q], id, true));
                }
                out
            }
            _ => Vec::new(),
        }
    }

    /// The solid head of an arrow drawn by the scene: a triangle at its tip, a fixed size on the slide.
    fn arrow_head(&self, o: &Obj, view: &View, now: f64, grow: f32, alpha: f32) -> Option<Draw> {
        if !matches!(o.content, Content::Line { arrow: true, .. }) || grow < 0.95 {
            return None;
        }
        let pts = self.geo_points(o, now);
        let (tail, tip) = (view.project(pts[0]), view.project(pts[1]));
        let (dx, dy) = (tip[0] - tail[0], tip[1] - tail[1]);
        let length = dx.hypot(dy);
        if length < 1.0 {
            return None;
        }
        let (ux, uy) = (dx / length, dy / length);
        let head = (9.0 + 2.6 * o.width).min(0.6 * length);
        let side = 0.36 * head;
        let back = [tip[0] - ux * head, tip[1] - uy * head];
        let base = o.colors.get(&usize::MAX).map_or(o.color, |c| c.at(now));
        let color = with_alpha(base, alpha * o.opacity * (1.0 - 0.7 * o.dim.at(now)));
        Some(Draw::Head {
            pts: [
                tip,
                [back[0] - uy * side, back[1] + ux * side],
                [back[0] + uy * side, back[1] - ux * side],
            ],
            color,
        })
    }

    /// The lines and discs of a 3D object, grown to `grow` of its length.
    fn geo_draws(&self, o: &Obj, view: &View, now: f64, grow: f32, alpha: f32) -> Vec<Draw> {
        let pts = self.geo_points(o, now);
        let base = o.colors.get(&usize::MAX).map_or(o.color, |c| c.at(now));
        let color = with_alpha(base, alpha * o.opacity * (1.0 - 0.7 * o.dim.at(now)));
        let bg = with_alpha(self.color(Some("bg"), "bg"), alpha);
        let g = grow.clamp(0.0, 1.0);
        let mut out = Vec::new();
        match &o.content {
            Content::Point(_) => {
                let full = o.width.max(2.0) * 2.0;
                let r = if g >= 1.0 {
                    full
                } else {
                    let ease = (1.70158 + 1.0) * g * g * g - 1.70158 * g * g;
                    full * ease.clamp(0.0, 1.3)
                };
                out.push(Draw::Disc {
                    c: view.project(pts[0]),
                    r,
                    color,
                    ring: bg,
                });
            }
            Content::Line { arrow, .. } => {
                let (a, b) = (view.project(pts[0]), view.project(pts[1]));
                let b = [lerp(a[0], b[0], g), lerp(a[1], b[1], g)];
                out.push(Draw::Line {
                    a,
                    b,
                    width: o.width,
                    color,
                    arrow: *arrow,
                    dash: o.dash,
                });
            }
            Content::Polyline { closed, .. } => {
                let mut sp: Vec<[f32; 2]> = pts.iter().map(|&p| view.project(p)).collect();
                if *closed && !sp.is_empty() {
                    sp.push(sp[0]);
                }
                let total: f32 = sp
                    .windows(2)
                    .map(|w| (w[1][0] - w[0][0]).hypot(w[1][1] - w[0][1]))
                    .sum();
                let mut left = total * g;
                let mut path = vec![sp[0]];
                for w in sp.windows(2) {
                    let l = (w[1][0] - w[0][0]).hypot(w[1][1] - w[0][1]);
                    if left >= l {
                        path.push(w[1]);
                        left -= l;
                    } else {
                        let t = left / l.max(1e-6);
                        path.push([lerp(w[0][0], w[1][0], t), lerp(w[0][1], w[1][1], t)]);
                        break;
                    }
                }
                if o.dash {
                    for w in path.windows(2) {
                        out.push(Draw::Line {
                            a: w[0],
                            b: w[1],
                            width: o.width,
                            color,
                            arrow: false,
                            dash: true,
                        });
                    }
                } else {
                    out.push(Draw::Path {
                        pts: path,
                        closed: false,
                        width: o.width,
                        color,
                    });
                }
            }
            Content::Cuboid { .. } => {
                for (p, q) in cuboid_edges(pts[0], pts[1]) {
                    let (p, q) = (view.project(p), view.project(q));
                    let q = [lerp(p[0], q[0], g), lerp(p[1], q[1], g)];
                    out.push(Draw::Line {
                        a: p,
                        b: q,
                        width: o.width,
                        color,
                        arrow: false,
                        dash: true,
                    });
                }
            }
            _ => {}
        }
        out
    }

    /// Screen anchor of a 3D label, offset so the label avoids lines, points and other labels.
    fn place_labels(&mut self, view: &View, now: f64) {
        let mut segs: Vec<([f32; 2], [f32; 2])> = Vec::new();
        let mut discs: Vec<[f32; 2]> = Vec::new();
        for o in &self.objs {
            if !o.geo() {
                continue;
            }
            let world = self.geo_points(o, now);
            let pts: Vec<[f32; 2]> = world.iter().map(|&p| view.project(p)).collect();
            match &o.content {
                Content::Point(_) => discs.push(pts[0]),
                Content::Line { .. } => segs.push((pts[0], pts[1])),
                Content::Polyline { .. } => segs.extend(pts.windows(2).map(|w| (w[0], w[1]))),
                _ => {}
            }
        }
        let right = self.split * W - 12.0;
        let mut boxes: Vec<[f32; 4]> = Vec::new();
        let mut placed = HashMap::new();
        let ids: Vec<usize> = (0..self.objs.len())
            .filter(|&i| self.objs[i].label())
            .collect();
        for i in ids {
            let o = &self.objs[i];
            let anchor3 = self.ctrl(o, now).first().copied().unwrap_or([0.0; 3]);
            let a = view.project(anchor3);
            let (w, h) = (o.expr.w * o.size, (o.expr.h + o.expr.d) * o.size);
            let place = |ang: f32, gap: f32| -> [f32; 4] {
                let (dx, dy) = (ang.cos(), -ang.sin());
                // push the box out until its edge, not its center, is `gap` from the anchor
                let reach = (w / 2.0 / dx.abs().max(1e-3)).min(h / 2.0 / dy.abs().max(1e-3));
                let cx = a[0] + dx * (gap + reach);
                let cy = a[1] + dy * (gap + reach);
                [cx - w / 2.0, cy - h / 2.0, cx + w / 2.0, cy + h / 2.0]
            };
            let score = |b: [f32; 4]| -> f32 {
                let mut s = 0.0;
                for q in &boxes {
                    let ix = (b[2].min(q[2]) - b[0].max(q[0]) + 6.0).max(0.0);
                    let iy = (b[3].min(q[3]) - b[1].max(q[1]) + 6.0).max(0.0);
                    if ix > 0.0 && iy > 0.0 {
                        s += 300.0 + ix * iy * 4.0;
                    }
                }
                let e = [b[0] - 4.0, b[1] - 4.0, b[2] + 4.0, b[3] + 4.0];
                for (p, q) in &segs {
                    let l = clip_len(*p, *q, e);
                    if l > 0.0 {
                        s += 120.0 + l * 6.0;
                    }
                }
                for d in &discs {
                    if (d[0] - a[0]).abs() < 0.5 && (d[1] - a[1]).abs() < 0.5 {
                        continue;
                    }
                    let inside_x = d[0] > b[0] - 9.0 && d[0] < b[2] + 9.0;
                    if inside_x && d[1] > b[1] - 9.0 && d[1] < b[3] + 9.0 {
                        s += 500.0;
                    }
                }
                if b[0] < 8.0 || b[2] > right || b[1] < TOP - 40.0 || b[3] > H - 50.0 {
                    s += 800.0;
                }
                s
            };
            let n = 16;
            let angle = |k: usize| {
                std::f32::consts::FRAC_PI_4 - k as f32 * std::f32::consts::TAU / n as f32
            };
            let chosen = if let Some(d) = o.dir {
                if d[0] == 0.0 && d[1] == 0.0 {
                    place(0.0, -w.max(h))
                } else {
                    place(d[1].atan2(d[0]), 10.0)
                }
            } else {
                let cur = o.label_dir;
                let cand = |k: usize| place(angle(k % n), if k >= n { 22.0 } else { 9.0 });
                let mut best = (score(cand(cur)) - 15.0, cur);
                for k in 0..2 * n {
                    let pref = (k % n) as f32 * 0.4 + if k >= n { 25.0 } else { 0.0 };
                    let s = score(cand(k)) + pref;
                    if s < best.0 {
                        best = (s, k);
                    }
                }
                self.objs[i].label_dir = best.1;
                cand(best.1)
            };
            boxes.push(chosen);
            let o = &self.objs[i];
            placed.insert(o.id.clone(), [chosen[0], chosen[1] + o.expr.h * o.size]);
        }
        self.labels = placed;
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Drawing
    // ═══════════════════════════════════════════════════════════════════════════

    /// The prims of a 2D object or label at rest, in slide pixels; hidden prims are transparent.
    fn prims_of(&self, i: usize, now: f64) -> Vec<DPrim> {
        let o = &self.objs[i];
        let (ox, oy, k) = if o.label() {
            match self.labels.get(&o.id) {
                Some(p) => (p[0], p[1], o.size),
                None => return Vec::new(),
            }
        } else {
            let p = o.pos.at(now);
            (p[0], p[1] + o.expr.h * o.scale(), o.scale())
        };
        let mut alpha = o.opacity * (1.0 - 0.65 * o.dim.at(now));
        if o.block() {
            let top = o.pos.at(now)[1];
            let bottom = top + (o.expr.h + o.expr.d) * k;
            if bottom < TOP + 4.0 {
                alpha *= ((bottom - (TOP - 60.0)) / 64.0).clamp(0.0, 1.0);
            }
        }
        let mut out = o
            .expr
            .prims
            .iter()
            .enumerate()
            .map(|(j, p)| {
                let base = o
                    .colors
                    .get(&j)
                    .map(|c| c.at(now))
                    .or(p.color)
                    .unwrap_or(o.color);
                let a = if o.hidden.contains(&j) { 0.0 } else { alpha };
                scaled(p, k, ox, oy, with_alpha(base, a))
            })
            .collect::<Vec<_>>();
        // a cell with a hidden entry stays neutral, so its colour does not give the value away
        if !o.hidden.is_empty() {
            let faint = with_alpha(self.color(Some("faint"), "faint"), alpha);
            for (rect, entries) in cells_of(&o.groups) {
                if !entries.is_empty() && entries.iter().all(|e| o.hidden.contains(e)) {
                    let shape = o.expr.prims[rect].shape;
                    if let (
                        Some(q),
                        Shape::Rect {
                            w,
                            h,
                            radius,
                            stroke,
                            ..
                        },
                    ) = (out.get_mut(rect), shape)
                    {
                        q.color = faint;
                        q.shape = Shape::Rect {
                            w: w * k,
                            h: h * k,
                            radius: radius * k,
                            fill: 0.5,
                            stroke: stroke * k,
                        };
                    }
                }
            }
        }
        out
    }

    /// Ink box of a piece: min x, min y, max x, max y.
    pub fn bounds(&self, p: &DPrim) -> [f32; 4] {
        match p.shape {
            Shape::Glyph { font, gid, size } => {
                let m = self.fonts.metrics(font, gid);
                [
                    p.x + m.left * size,
                    p.y - m.height * size,
                    p.x + m.right * size,
                    p.y + m.depth * size,
                ]
            }
            Shape::Rect { w, h, .. } => [p.x, p.y, p.x + w, p.y + h],
            Shape::Stroke { dx, dy, .. } => [
                p.x.min(p.x + dx),
                p.y.min(p.y + dy),
                p.x.max(p.x + dx),
                p.y.max(p.y + dy),
            ],
        }
    }

    /// Ink box of the visible pieces, or of `subset` whether visible or not.
    fn bbox(&self, prims: &[DPrim], subset: Option<&[usize]>) -> Option<[f32; 4]> {
        let mut b = [f32::MAX, f32::MAX, f32::MIN, f32::MIN];
        let mut any = false;
        let mut take = |p: &DPrim| {
            if p.color[3] <= 0.001 && subset.is_none() {
                return;
            }
            let q = self.bounds(p);
            if q[2] - q[0] <= 0.0 && q[3] - q[1] <= 0.0 {
                return;
            }
            b = [
                b[0].min(q[0]),
                b[1].min(q[1]),
                b[2].max(q[2]),
                b[3].max(q[3]),
            ];
            any = true;
        };
        match subset {
            Some(s) => s.iter().filter_map(|&i| prims.get(i)).for_each(&mut take),
            None => prims.iter().for_each(&mut take),
        }
        any.then_some(b)
    }

    /// What object `i` draws at rest, for its fade-out.
    fn snapshot(&self, i: usize, now: f64, view: &View) -> Vec<Draw> {
        let o = &self.objs[i];
        if o.geo() {
            return self.geo_draws(o, view, now, 1.0, 1.0);
        }
        if o.boxed.is_some() {
            return self.box_draws(i, now, 1.0);
        }
        self.prims_of(i, now).into_iter().map(Draw::Prim).collect()
    }

    /// The rounded outline of box object `i`, drawn to `grow` of its length.
    fn box_draws(&self, i: usize, now: f64, grow: f32) -> Vec<Draw> {
        let o = &self.objs[i];
        let Some((target, pad)) = &o.boxed else {
            return Vec::new();
        };
        let Some((ti, subset)) = self.resolve(target) else {
            return Vec::new();
        };
        let prims = self.prims_of(ti, now);
        let Some(b) = self.bbox(&prims, subset.as_deref()) else {
            return Vec::new();
        };
        let mut pts = rounded(
            b[0] - pad,
            b[1] - pad,
            b[2] - b[0] + 2.0 * pad,
            b[3] - b[1] + 2.0 * pad,
            9.0,
        );
        pts.push(pts[0]);
        let n = ((pts.len() as f32) * grow.clamp(0.0, 1.0)).ceil() as usize;
        pts.truncate(n.max(2));
        let color = with_alpha(o.color, o.opacity * (1.0 - 0.7 * o.dim.at(now)));
        vec![Draw::Path {
            pts,
            closed: false,
            width: o.width,
            color,
        }]
    }

    /// Everything to paint this frame, back to front.
    pub fn frame(&mut self, now: f64) -> Vec<Draw> {
        self.now = now;
        // dynamic formulas follow moving points
        for i in 0..self.objs.len() {
            if let Some(src) = self.objs[i].dynamic.clone() {
                let filled = self.fill(&src);
                let content = match &self.objs[i].content {
                    Content::Tex(old) if *old != filled => Some(Content::Tex(filled)),
                    Content::Label { at, tex } if *tex != filled => Some(Content::Label {
                        at: at.clone(),
                        tex: filled,
                    }),
                    _ => None,
                };
                if let Some(c) = content {
                    let size = self.objs[i].size;
                    let (e, g) = self.compile(&c, size, None);
                    let o = &mut self.objs[i];
                    o.content = c;
                    o.expr = e;
                    o.groups = g;
                }
            }
        }
        let view = self.view(now);
        self.place_labels(&view, now);
        let mut out = Vec::new();
        let mut objects3: Vec<Spatial> = Vec::new();
        if let Some(g) = self.chapters[self.chapter]
            .grid
            .clone()
            .filter(|_| self.scene)
        {
            // the floor grid as thin kernel lines in the scene
            let color = with_alpha(self.color(Some("faint"), "faint"), 0.6);
            let n = (g.size / g.step).round() as i32;
            let h = g.size / 2.0;
            for i in 0..=n {
                let t = -h + i as f64 * g.step;
                let (cx, cy) = (g.center[0], g.center[1]);
                let along_y = vec![[cx + t, cy - h, g.z], [cx + t, cy + h, g.z]];
                let along_x = vec![[cx - h, cy + t, g.z], [cx + h, cy + t, g.z]];
                for (k, pts) in [along_y, along_x].into_iter().enumerate() {
                    objects3.push(Spatial {
                        id: format!("grid#{k}/{i}"),
                        kind: SpatialKind::Line,
                        pts,
                        color,
                        width: 1.2,
                        dash: false,
                        reveal: 1.0,
                    });
                }
            }
        }
        if let Some(g) = self.chapters[self.chapter]
            .grid
            .clone()
            .filter(|_| !self.scene)
        {
            let tilt = (view.cam.pitch.to_radians().sin().abs() as f32 * 3.0).clamp(0.0, 1.0);
            let col = with_alpha(self.color(Some("faint"), "faint"), 0.55 * tilt);
            let n = (g.size / g.step).round() as i32;
            let h = g.size / 2.0;
            let (cx, cy) = (g.center[0], g.center[1]);
            for i in 0..=n {
                let t = -h + i as f64 * g.step;
                let along_y = ([cx + t, cy - h, g.z], [cx + t, cy + h, g.z]);
                let along_x = ([cx - h, cy + t, g.z], [cx + h, cy + t, g.z]);
                for (a, b) in [along_y, along_x] {
                    out.push(Draw::Line {
                        a: view.project(a),
                        b: view.project(b),
                        width: 1.0,
                        color: col,
                        arrow: false,
                        dash: false,
                    });
                }
            }
        }
        if self.chapters[self.chapter].axes && !self.scene {
            for (k, name) in ["x", "y", "z"].iter().enumerate() {
                let mut e = [0.0; 3];
                e[k] = 1.0;
                let a = view.project([0.0; 3]);
                let b = view.project(e);
                let color = with_alpha(self.color(Some(name), "ink"), 0.9);
                out.push(Draw::Line {
                    a,
                    b,
                    width: 2.0,
                    color,
                    arrow: true,
                    dash: false,
                });
            }
        }
        let mut appear: HashMap<String, (Style, f64, f64, Option<Vec<usize>>)> = HashMap::new();
        let mut suppressed: HashMap<String, HashSet<usize>> = HashMap::new();
        let mut indicate: Vec<(String, Option<Vec<usize>>, Rgba, f64)> = Vec::new();
        for a in &self.anims {
            let p = a.p(now);
            match &a.fx {
                Fx::Appear {
                    id,
                    style,
                    lag,
                    subset,
                } => {
                    appear.insert(id.clone(), (*style, p, *lag, subset.clone()));
                }
                Fx::Morph { dst, set, .. } => {
                    suppressed
                        .entry(dst.clone())
                        .or_default()
                        .extend(set.iter().copied());
                }
                Fx::Indicate { id, subset, color } => {
                    indicate.push((id.clone(), subset.clone(), *color, p))
                }
                Fx::Vanish { .. } => {}
            }
        }
        let mut lines = Vec::new();
        let mut discs = Vec::new();
        let mut texts = Vec::new();
        let mut labels = Vec::new();
        let mut boxes = Vec::new();
        for i in 0..self.objs.len() {
            let o = &self.objs[i];
            let ap = appear.get(&o.id);
            if o.geo() {
                let (grow, alpha) = match ap {
                    Some((Style::Fade, p, ..)) => (1.0, smooth(*p) as f32),
                    Some((Style::None, ..)) | None => (1.0, 1.0),
                    Some((_, p, ..)) => (smooth(*p) as f32, if *p > 0.0 { 1.0 } else { 0.0 }),
                };
                objects3.extend(self.geo_spatial(o, now, grow, alpha));
                if self.scene {
                    lines.extend(self.arrow_head(o, &view, now, grow, alpha));
                    continue;
                }
                let d = self.geo_draws(o, &view, now, grow, alpha);
                if matches!(o.content, Content::Point(_)) {
                    discs.extend(d);
                } else {
                    lines.extend(d);
                }
                continue;
            }
            if o.boxed.is_some() {
                let g = ap.map_or(1.0, |a| smooth(a.1) as f32);
                boxes.extend(self.box_draws(i, now, g));
                continue;
            }
            let mut prims = self.prims_of(i, now);
            if let Some(s) = suppressed.get(&o.id) {
                for &j in s {
                    if let Some(p) = prims.get_mut(j) {
                        p.color[3] = 0.0;
                    }
                }
            }
            if let Some((style, p, lag, subset)) = ap {
                let order: Vec<usize> = match subset {
                    Some(s) => s.clone(),
                    None => (0..prims.len()).collect(),
                };
                let n = order.len().max(1) as f64;
                let lag = if *style == Style::Write { *lag } else { 0.0 };
                let d = 1.0 / (1.0 + (n - 1.0) * lag);
                for (rank, &j) in order.iter().enumerate() {
                    let pj = ((p - rank as f64 * lag * d) / d).clamp(0.0, 1.0);
                    let e = smooth(pj) as f32;
                    if let Some(q) = prims.get_mut(j) {
                        q.color[3] *= e;
                        if *style != Style::Fade {
                            match &mut q.shape {
                                Shape::Rect { w, h, .. } => {
                                    let s = 0.7 + 0.3 * e;
                                    q.x += *w * (1.0 - s) / 2.0;
                                    q.y += *h * (1.0 - s) / 2.0;
                                    *w *= s;
                                    *h *= s;
                                }
                                _ => q.y += (1.0 - e) * 10.0,
                            }
                        }
                    }
                }
            }
            for (id, subset, color, p) in &indicate {
                if *id != o.id {
                    continue;
                }
                let tb = there_and_back(*p) as f32;
                let set: Vec<usize> = subset.clone().unwrap_or_else(|| (0..prims.len()).collect());
                if let Some(b) = self.bbox(&prims, Some(&set)) {
                    let (cx, cy) = ((b[0] + b[2]) / 2.0, (b[1] + b[3]) / 2.0);
                    let s = 1.0 + 0.2 * tb;
                    for &j in &set {
                        if let Some(q) = prims.get_mut(j) {
                            q.x = cx + (q.x - cx) * s;
                            q.y = cy + (q.y - cy) * s;
                            q.shape = match q.shape {
                                Shape::Stroke { dx, dy, width } => Shape::Stroke {
                                    dx: dx * s,
                                    dy: dy * s,
                                    width,
                                },
                                other => other.resized(s),
                            };
                            let a = q.color[3];
                            q.color = mix(q.color, *color, tb);
                            q.color[3] = a;
                        }
                    }
                }
            }
            let visible = prims
                .into_iter()
                .filter(|p| p.color[3] > 0.002)
                .map(Draw::Prim);
            if o.label() {
                labels.extend(visible);
            } else {
                texts.extend(visible);
            }
        }
        // removed 3D objects fade out as ghosts
        self.ghosts.retain(|(_, t0, dur)| t0 + dur > now);
        for (list, t0, dur) in &self.ghosts {
            let f = 1.0 - smooth(((now - t0) / dur).clamp(0.0, 1.0)) as f32;
            for g in list {
                let mut g = g.clone();
                g.color[3] *= f;
                objects3.push(g);
            }
        }
        self.objects3 = objects3;
        out.extend(lines);
        out.extend(discs);
        out.extend(labels);
        self.spatial = out.len();
        out.extend(texts);
        out.extend(boxes);
        // overlays: ghosts of removed objects and morphing pieces
        for a in &self.anims {
            let p = a.p(now);
            if now < a.t0 {
                match &a.fx {
                    Fx::Vanish { draws } => out.extend(draws.iter().cloned()),
                    Fx::Morph {
                        src,
                        outs,
                        copy: false,
                        ..
                    } => out.extend(outs.iter().map(|&i| Draw::Prim(src[i]))),
                    _ => {}
                }
                continue;
            }
            match &a.fx {
                Fx::Vanish { draws } => {
                    let f = 1.0 - smooth(p) as f32;
                    out.extend(draws.iter().cloned().map(|d| fade(d, f)));
                }
                Fx::Morph {
                    dst,
                    src,
                    pairs,
                    outs,
                    ins,
                    copy,
                    ..
                } => {
                    let Some(di) = self.find(dst) else { continue };
                    let live = self.prims_of(di, now);
                    let t = smooth(p) as f32;
                    if !copy {
                        let f = (1.0 - p as f32 * 1.7).clamp(0.0, 1.0);
                        for &i in outs {
                            let mut q = src[i];
                            q.color[3] *= f;
                            out.push(Draw::Prim(q));
                        }
                    }
                    for &(s, d) in pairs {
                        if let Some(target) = live.get(d) {
                            let mut tgt = *target;
                            let mut from = src[s];
                            if tgt.color[3] <= 0.0 {
                                tgt.color[3] = from.color[3];
                            }
                            if from.color[3] <= 0.0 {
                                from.color[3] = tgt.color[3];
                            }
                            out.push(Draw::Prim(lerp_prim(&from, &tgt, t)));
                        }
                    }
                    let f = ((p as f32 - 0.35) / 0.65).clamp(0.0, 1.0);
                    for &d in ins {
                        if let Some(target) = live.get(d) {
                            let mut q = *target;
                            q.color[3] *= smooth(f as f64) as f32;
                            out.push(Draw::Prim(q));
                        }
                    }
                }
                _ => {}
            }
        }
        out
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Actions
    // ═══════════════════════════════════════════════════════════════════════════

    /// Run one action at time `now`, animated or applied at once.
    fn apply(&mut self, a: &Action, now: f64, animate: bool) {
        self.now = now;
        let dur = self.duration(a);
        let view = self.view(now);
        let push = |s: &mut Stage, fx: Fx| {
            if animate {
                s.anims.push(Anim { fx, t0: now, dur });
            }
        };
        match &a.kind {
            Kind::Add(specs) => {
                let n = specs.len().max(1) as f64;
                for (k, spec) in specs.iter().enumerate() {
                    let mut obj = self.make(spec);
                    let style = match spec.anim.as_deref() {
                        Some("fade") => Style::Fade,
                        Some("grow") => Style::Grow,
                        Some("pop") => Style::Pop,
                        Some("none") => Style::None,
                        Some("write") => Style::Write,
                        _ if obj.geo() => {
                            if matches!(obj.content, Content::Point(_)) {
                                Style::Pop
                            } else {
                                Style::Grow
                            }
                        }
                        _ => Style::Write,
                    };
                    let id = obj.id.clone();
                    let mut extra = Vec::new();
                    if let Some(l) = &spec.label {
                        let content = Content::Label {
                            at: Pos::Name(id.clone()),
                            tex: l.clone(),
                        };
                        let mut label = plain(format!("{id}.label"), content);
                        label.color = spec.label_color.clone().or(spec.color.clone());
                        label.dir = spec.dir;
                        let mut lab = self.make(&label);
                        lab.attach = Some(id.clone());
                        extra.push(lab);
                    }
                    if obj.geo() {
                        obj.dir = None;
                    }
                    self.insert(obj);
                    for e in extra {
                        let eid = e.id.clone();
                        self.insert(e);
                        push(
                            self,
                            Fx::Appear {
                                id: eid,
                                style: Style::Write,
                                lag: 0.08,
                                subset: None,
                            },
                        );
                    }
                    let lag = spec
                        .lag
                        .unwrap_or(if style == Style::Write { 0.06 } else { 0.0 });
                    if animate {
                        let start = now + dur * 0.15 * k as f64 / n;
                        self.anims.push(Anim {
                            fx: Fx::Appear {
                                id,
                                style,
                                lag,
                                subset: None,
                            },
                            t0: start,
                            dur: dur * (1.0 - 0.15 * (n - 1.0) / n),
                        });
                    }
                }
                self.relayout(now, animate);
            }
            Kind::Remove(ids) => {
                let mut gone: Vec<String> = Vec::new();
                for id in ids {
                    gone.push(id.clone());
                    gone.push(format!("{id}.label"));
                }
                let boxed: Vec<String> = self
                    .objs
                    .iter()
                    .filter(|o| {
                        o.boxed.as_ref().is_some_and(|(t, _)| {
                            gone.iter().any(|g| {
                                t == g
                                    || t.starts_with(&format!("{g}["))
                                    || t.starts_with(&format!("{g}{{"))
                            })
                        })
                    })
                    .map(|o| o.id.clone())
                    .collect();
                gone.extend(boxed);
                for id in gone {
                    if let Some(i) = self.find(&id) {
                        if self.scene && self.objs[i].geo() {
                            // the scene fades its own copy of the object
                            let ghost = self.geo_spatial(&self.objs[i], now, 1.0, 1.0);
                            self.objs.remove(i);
                            if animate {
                                self.ghosts.push((ghost, now, dur));
                            }
                            continue;
                        }
                        let draws = self.snapshot(i, now, &view);
                        self.objs.remove(i);
                        push(self, Fx::Vanish { draws });
                    }
                }
                self.relayout(now, animate);
            }
            Kind::Clear => {
                let ids: Vec<String> = self
                    .objs
                    .iter()
                    .filter(|o| o.block() || o.boxed.is_some())
                    .map(|o| o.id.clone())
                    .collect();
                for id in ids {
                    if let Some(i) = self.find(&id) {
                        let draws = self.snapshot(i, now, &view);
                        self.objs.remove(i);
                        push(self, Fx::Vanish { draws });
                    }
                }
                self.aliases
                    .retain(|_, (o, _)| self.objs.iter().any(|x| x.id == *o));
                self.relayout(now, animate);
            }
            Kind::Morph { target, to, map } => {
                let Some(i) = self.find(target) else {
                    self.errors.push(format!("morph: no object {target}"));
                    return;
                };
                self.morph_to(i, to.clone(), map, now, dur, animate);
            }
            Kind::Set { target, tex: value } => {
                let cut = target.find('[').unwrap_or(target.len());
                let (base, sel) = (
                    &target[..cut],
                    target[cut..].trim_matches(|c| c == '[' || c == ']'),
                );
                let rc: Vec<usize> = sel
                    .split(',')
                    .filter_map(|s| s.trim().parse().ok())
                    .collect();
                let (oid, item) = match self.find(base) {
                    Some(_) => (base.to_string(), None),
                    None => match self.aliases.get(base) {
                        Some((o, it)) => (o.clone(), Some(it.clone())),
                        None => {
                            self.errors.push(format!("set: no matrix {base}"));
                            return;
                        }
                    },
                };
                let Some(i) = self.find(&oid) else { return };
                if rc.len() != 2 {
                    return;
                }
                let mut content = self.objs[i].content.clone();
                let spec: Option<&mut MatrixSpec> = match (&mut content, &item) {
                    (Content::Matrix(m), None) => Some(m),
                    (Content::Row(items), Some(it)) => items
                        .iter_mut()
                        .find(|x| x.id.as_deref() == Some(it))
                        .and_then(|x| match &mut x.content {
                            Content::Matrix(m) => Some(m),
                            _ => None,
                        }),
                    _ => None,
                };
                if let Some(m) = spec
                    && let Some(cell) = m.rows.get_mut(rc[0]).and_then(|r| r.get_mut(rc[1]))
                {
                    *cell = value.clone();
                }
                let hidden_before = self.objs[i].hidden.clone();
                self.morph_to(i, content, &[], now, dur, animate);
                let cell = match &item {
                    Some(it) => format!("{it}/{},{}", rc[0], rc[1]),
                    None => format!("{},{}", rc[0], rc[1]),
                };
                let shown = self.objs[i].groups.get(&cell);
                let keep: HashSet<usize> = hidden_before
                    .into_iter()
                    .filter(|j| !shown.is_some_and(|g| g.contains(j)))
                    .collect();
                self.objs[i].hidden = keep;
            }
            Kind::Derive { from, obj, map } => {
                let Some((si, subset)) = self.resolve(from) else {
                    self.errors.push(format!("derive: no source {from}"));
                    return;
                };
                let src_prims = self.prims_of(si, now);
                let src_expr = self.objs[si].expr.clone();
                let src_geo = self.objs[si].geo().then(|| self.ctrl(&self.objs[si], now));
                let mut o = self.make(obj);
                let id = o.id.clone();
                if let Some(g) = src_geo {
                    o.geo_from = Some((g, now, dur));
                    self.insert(o);
                    if !animate {
                        self.objs.last_mut().unwrap().geo_from = None;
                    }
                    self.relayout(now, animate);
                    return;
                }
                self.insert(o);
                self.relayout(now, animate);
                let di = self.find(&id).unwrap();
                let sset: Vec<usize> = subset.unwrap_or_else(|| {
                    (0..src_expr.prims.len())
                        .filter(|j| !self.objs[si].hidden.contains(j))
                        .collect()
                });
                let dst = &self.objs[di];
                let dset: Vec<usize> = (0..dst.expr.prims.len()).collect();
                let sg = &self.objs[si].groups;
                let (pairs, _, ins) =
                    match_prims(&src_expr, &sset, &dst.expr, &dset, map, sg, &dst.groups);
                push(
                    self,
                    Fx::Morph {
                        dst: id,
                        set: dset,
                        src: src_prims,
                        pairs,
                        outs: Vec::new(),
                        ins,
                        copy: true,
                    },
                );
            }
            Kind::Fly { from, to, keep } => {
                let (Some((si, ss)), Some((di, ds))) = (self.resolve(from), self.resolve(to))
                else {
                    self.errors
                        .push(format!("fly: cannot resolve {from} -> {to}"));
                    return;
                };
                let src_prims = self.prims_of(si, now);
                let src_expr = self.objs[si].expr.clone();
                let sset: Vec<usize> = ss.unwrap_or_else(|| (0..src_expr.prims.len()).collect());
                let dset: Vec<usize> =
                    ds.unwrap_or_else(|| (0..self.objs[di].expr.prims.len()).collect());
                for j in &dset {
                    self.objs[di].hidden.remove(j);
                }
                let dst_expr = self.objs[di].expr.clone();
                let none = HashMap::new();
                let (pairs, _, ins) =
                    match_prims(&src_expr, &sset, &dst_expr, &dset, &[], &none, &none);
                let id = self.objs[di].id.clone();
                if !keep {
                    for j in &sset {
                        self.objs[si].hidden.insert(*j);
                    }
                }
                push(
                    self,
                    Fx::Morph {
                        dst: id,
                        set: dset,
                        src: src_prims,
                        pairs,
                        outs: Vec::new(),
                        ins,
                        copy: true,
                    },
                );
            }
            Kind::Indicate(refs, color) => {
                let c = self.color(color.as_deref(), "accent");
                for r in refs {
                    if let Some((i, subset)) = self.resolve(r) {
                        let id = self.objs[i].id.clone();
                        push(
                            self,
                            Fx::Indicate {
                                id,
                                subset,
                                color: c,
                            },
                        );
                    } else {
                        self.errors.push(format!("indicate: no {r}"));
                    }
                }
            }
            Kind::Box { id, target, color } => {
                let c = self.color(color.as_deref(), "accent");
                if self.resolve(target).is_none() {
                    self.errors.push(format!("box: no {target}"));
                    return;
                }
                let mut spec = plain(id.clone(), Content::Tex(String::new()));
                spec.width = Some(2.4);
                let mut o = self.make(&spec);
                o.color = c;
                o.width = 2.4;
                o.boxed = Some((target.clone(), 7.0));
                self.insert(o);
                push(
                    self,
                    Fx::Appear {
                        id: id.clone(),
                        style: Style::Draw,
                        lag: 0.0,
                        subset: None,
                    },
                );
            }
            Kind::Color { target, color } => {
                let Some((i, subset)) = self.resolve(target) else {
                    self.errors.push(format!("color: no {target}"));
                    return;
                };
                let reset = color == "reset" || color == "default";
                let c = self.color(Some(color), "ink");
                let o = &mut self.objs[i];
                let set: Vec<usize> = subset.unwrap_or_else(|| (0..o.expr.prims.len()).collect());
                if o.geo() {
                    let cur = o.colors.get(&usize::MAX).map_or(o.color, |t| t.at(now));
                    let to = if reset { o.color } else { c };
                    let mut tw = Tween::fixed(cur);
                    tw.retarget(to, now, dur, animate);
                    o.colors.insert(usize::MAX, tw);
                    return;
                }
                if o.label() && set.len() == o.expr.prims.len() && !reset {
                    o.color = c;
                }
                let cell = |p: &Prim| matches!(p.shape, Shape::Rect { fill, .. } if fill > 0.0);
                let cells = set.iter().any(|&j| cell(&o.expr.prims[j]));
                let bright = 0.3 * c[0] + 0.6 * c[1] + 0.1 * c[2] > 0.6;
                let entry = if bright {
                    self.palette["bg"]
                } else {
                    self.palette["ink"]
                };
                for j in set {
                    let c = if cells && !cell(&o.expr.prims[j]) {
                        entry
                    } else {
                        c
                    };
                    let base = o.expr.prims[j].color.unwrap_or(o.color);
                    let cur = o.colors.get(&j).map(|t| t.at(now)).unwrap_or(base);
                    let to = if reset { base } else { with_alpha(c, base[3]) };
                    let mut tw = Tween::fixed(cur);
                    tw.retarget(to, now, dur, animate);
                    o.colors.insert(j, tw);
                }
            }
            Kind::Move { point, to } => {
                let ps = self.points_at(now);
                let Ok(p) = eval::position(to, &ps) else {
                    self.errors.push(format!("move: bad target for {point}"));
                    return;
                };
                let tw = self
                    .points
                    .entry(point.clone())
                    .or_insert_with(|| Tween::fixed(p));
                tw.retarget(p, now, dur, animate);
            }
            Kind::Camera(spec) => {
                let cur = self.cam.to;
                let cam = self.camera_for(spec, cur, false);
                self.cam.retarget(cam, now, dur, animate);
                self.cam_serial += 1;
            }
            Kind::Wait => {}
            Kind::Dim(refs) | Kind::Undim(refs) => {
                let v = if matches!(a.kind, Kind::Dim(_)) {
                    1.0
                } else {
                    0.0
                };
                for r in refs {
                    if let Some(i) = self.find(r) {
                        self.objs[i].dim.retarget(v, now, dur, animate);
                    }
                }
            }
            Kind::Show(refs) => {
                for r in refs {
                    if let Some((i, subset)) = self.resolve(r) {
                        let set: Vec<usize> =
                            subset.unwrap_or_else(|| (0..self.objs[i].expr.prims.len()).collect());
                        for j in &set {
                            self.objs[i].hidden.remove(j);
                        }
                        let id = self.objs[i].id.clone();
                        push(
                            self,
                            Fx::Appear {
                                id,
                                style: Style::Write,
                                lag: 0.05,
                                subset: Some(set),
                            },
                        );
                    }
                }
            }
            Kind::Hide(refs) => {
                for r in refs {
                    if let Some((i, subset)) = self.resolve(r) {
                        let set: Vec<usize> =
                            subset.unwrap_or_else(|| (0..self.objs[i].expr.prims.len()).collect());
                        let prims = self.prims_of(i, now);
                        let draws = set
                            .iter()
                            .filter_map(|&j| prims.get(j).copied())
                            .map(Draw::Prim)
                            .collect();
                        for j in &set {
                            self.objs[i].hidden.insert(*j);
                        }
                        push(self, Fx::Vanish { draws });
                    }
                }
            }
            Kind::Play(children) => {
                for c in children {
                    self.apply(c, now, animate);
                }
            }
        }
        self.changed = true;
    }

    /// Change object `i` to new content, morphing its pieces when `animate`.
    fn morph_to(
        &mut self,
        i: usize,
        to: Content,
        map: &[(String, String)],
        now: f64,
        dur: f64,
        animate: bool,
    ) {
        let o = &self.objs[i];
        if o.geo() {
            let from = self.geo_points(o, now);
            let o = &mut self.objs[i];
            o.content = to;
            if animate {
                o.geo_from = Some((from, now, dur));
            }
            return;
        }
        let src = self.prims_of(i, now);
        let src_expr = o.expr.clone();
        let old_groups = o.groups.clone();
        let sset: Vec<usize> = (0..src_expr.prims.len())
            .filter(|j| !o.hidden.contains(j))
            .collect();
        let size = o.size;
        let (to, dynamic) = self.filled(&to);
        let (expr, groups) = self.compile(&to, size, None);
        let id = self.objs[i].id.clone();
        {
            let o = &mut self.objs[i];
            o.content = to;
            o.expr = expr.clone();
            o.groups = groups;
            o.hidden.clear();
            o.colors.clear();
            o.dynamic = dynamic;
        }
        if let Content::Row(items) = &self.objs[i].content.clone() {
            for it in items {
                if let Some(rid) = &it.id {
                    self.aliases.insert(rid.clone(), (id.clone(), rid.clone()));
                }
            }
        }
        self.relayout(now, animate);
        if animate {
            let dset: Vec<usize> = (0..expr.prims.len()).collect();
            let dg = &self.objs[i].groups;
            let (pairs, outs, ins) =
                match_prims(&src_expr, &sset, &expr, &dset, map, &old_groups, dg);
            self.anims.push(Anim {
                fx: Fx::Morph {
                    dst: id,
                    set: dset,
                    src,
                    pairs,
                    outs,
                    ins,
                    copy: false,
                },
                t0: now,
                dur,
            });
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Inspection
    // ═══════════════════════════════════════════════════════════════════════════

    /// Overlapping panel blocks and labels, for the browser checks.
    pub fn overlaps(&mut self) -> Vec<(String, String)> {
        let now = self.now;
        let mut boxes: Vec<(String, [f32; 4])> = Vec::new();
        for i in 0..self.objs.len() {
            let o = &self.objs[i];
            if !(o.block() || o.label()) {
                continue;
            }
            let prims = self.prims_of(i, now);
            if let Some(b) = self.bbox(&prims, None) {
                boxes.push((o.id.clone(), b));
            }
        }
        let mut out = Vec::new();
        for a in 0..boxes.len() {
            for b in a + 1..boxes.len() {
                let (p, q) = (boxes[a].1, boxes[b].1);
                let ix = p[2].min(q[2]) - p[0].max(q[0]);
                let iy = p[3].min(q[3]) - p[1].max(q[1]);
                if ix > 1.0 && iy > 1.0 {
                    out.push((boxes[a].0.clone(), boxes[b].0.clone()));
                }
            }
        }
        out
    }

    /// Ink boxes of the panel blocks and labels, for the browser checks.
    pub fn item_boxes(&self) -> Vec<(String, [f32; 4])> {
        let now = self.now;
        (0..self.objs.len())
            .filter_map(|i| {
                let o = &self.objs[i];
                if !(o.block() || o.label()) {
                    return None;
                }
                let prims = self.prims_of(i, now);
                self.bbox(&prims, None).map(|b| (o.id.clone(), b))
            })
            .collect()
    }

    /// A palette colour, ink when unknown.
    pub fn palette(&self, name: &str) -> Rgba {
        self.color(Some(name), "ink")
    }

    /// Chrome prose wrapped at `width` em.
    pub fn chrome(&mut self, text: &str, bold: bool, width: f32) -> Rc<Expr> {
        self.typeset(text, if bold { 3 } else { 2 }, width)
    }
}

/// Length of the part of segment p-q inside box b.
fn clip_len(p: [f32; 2], q: [f32; 2], b: [f32; 4]) -> f32 {
    let (dx, dy) = (q[0] - p[0], q[1] - p[1]);
    let (mut t0, mut t1) = (0.0f32, 1.0f32);
    let sides = [
        (-dx, p[0] - b[0]),
        (dx, b[2] - p[0]),
        (-dy, p[1] - b[1]),
        (dy, b[3] - p[1]),
    ];
    for (pk, qk) in sides {
        if pk.abs() < 1e-9 {
            if qk < 0.0 {
                return 0.0;
            }
        } else {
            let r = qk / pk;
            if pk < 0.0 {
                t0 = t0.max(r);
            } else {
                t1 = t1.min(r);
            }
        }
    }
    if t1 > t0 {
        (t1 - t0) * dx.hypot(dy)
    } else {
        0.0
    }
}

/// Matrix cells as (rect, entries) from the `r,c` groups.
fn cells_of(groups: &HashMap<String, Vec<usize>>) -> Vec<(usize, Vec<usize>)> {
    groups
        .iter()
        .filter(|(k, _)| {
            let last = k.rsplit('/').next().unwrap_or(k);
            let mut it = last.split(',');
            let index = |s: &str| s.parse::<usize>().is_ok();
            match (it.next(), it.next(), it.next()) {
                (Some(r), Some(c), None) => index(r) && index(c),
                _ => false,
            }
        })
        .filter_map(|(_, v)| v.split_first().map(|(r, e)| (*r, e.to_vec())))
        .collect()
}
