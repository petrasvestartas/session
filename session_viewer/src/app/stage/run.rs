use super::deck::{Action, Chapter, Content, Index, Kind};
use super::draw::fade;
use super::font::Fonts;
use super::geometry::{Spatial, SpatialKind};
use super::morph::lerp_prim;
use super::object::Obj;
use super::tex::{Expr, Rgba, Shape};
use super::tween::{Tween, mix, smooth, there_and_back, with_alpha};
use super::view::{Cam, Lens};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

/// The slide design space; the canvas shows it letterboxed.
pub const W: f32 = 1600.0;
pub const H: f32 = 900.0;
pub(super) const TOP: f32 = 128.0;
pub(super) const BOTTOM: f32 = 150.0;
pub(super) const MARGIN: f32 = 56.0;
pub(super) const RELAYOUT_MS: f64 = 650.0;
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
    Path {
        pts: Vec<[f32; 2]>,
        closed: bool,
        width: f32,
        color: Rgba,
    },
}

/// A running effect.
#[derive(Clone, Debug)]
pub(super) enum Fx {
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
pub(super) enum Style {
    Write,
    Fade,
    Grow,
    Pop,
    Draw,
    None,
}

/// An effect with its start and duration, milliseconds.
#[derive(Clone, Debug)]
pub(super) struct Anim {
    pub(super) fx: Fx,
    pub(super) t0: f64,
    pub(super) dur: f64,
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

/// A stage deck at one step, with its running animations.
pub struct Stage {
    pub fonts: Rc<Fonts>,
    pub index: Index,
    pub chapters: Vec<Chapter>,
    pub chapter: usize,
    pub step: Option<usize>,
    pub errors: Vec<String>,
    pub(super) palette: HashMap<String, Rgba>,
    pub(super) points: HashMap<String, Tween<[f64; 3]>>,
    pub objs: Vec<Obj>,
    pub(super) aliases: HashMap<String, (String, String)>,
    pub(super) cam: Tween<Cam>,
    pub user: [f64; 3],
    pub(super) anims: Vec<Anim>,
    pending: Vec<(f64, Action)>,
    pub title: (String, String, f64),
    pub say: (String, String, f64),
    pub(super) cache: HashMap<(String, u8, u32), Rc<Expr>>,
    pub(super) labels: HashMap<String, [f32; 2]>,
    pub split: f32,     // Fraction of the slide width given to the 3D side.
    pub spatial: usize, // Draws before this index are clipped to the 3D side.
    pub pictures: HashMap<String, (u16, f32)>, // Index and width over height of each picture.
    pub lens: Option<Lens>,
    pub objects3: Vec<Spatial>,
    pub(super) ghosts: Vec<(Vec<Spatial>, f64, f64)>,
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
            lens: None,
            objects3: Vec::new(),
            pictures: HashMap::new(),
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
    pub(super) fn duration(&self, a: &Action) -> f64 {
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
        if let Some(g) = self.chapters[self.chapter].grid.clone() {
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
}
