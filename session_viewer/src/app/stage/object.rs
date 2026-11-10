use super::deck::{Content, ObjSpec, Pos};
use super::eval::{self, Points};
use super::run::Stage;
use super::tex::{self, Expr, Rgba};
use super::tween::Tween;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

/// How a panel block sits in the column.
#[derive(Clone, Copy, PartialEq, Debug)]
pub(super) enum Align {
    Center,
    Left,
    Equals,
}

/// One object on the stage.
pub struct Obj {
    pub id: String,
    pub content: Content,
    pub(super) color: Rgba,
    pub(super) size: f32,  // Pixels per em.
    pub(super) width: f32, // Line width, pixels.
    pub(super) dash: bool,
    pub(super) expr: Rc<Expr>, // The laid-out formula, em units.
    pub(super) groups: HashMap<String, Vec<usize>>, // Named prim sets: cells, rows, parts, row items.
    pub(super) dynamic: Option<String>, // TeX with `{=expr}` fields, refilled as points move.
    pub(super) hidden: HashSet<usize>,  // Prims drawn transparent.
    pub(super) colors: HashMap<usize, Tween<Rgba>>, // Colour per prim; `usize::MAX` for a 3D object.
    pub(super) align: Align,
    pub(super) at: Option<[f32; 2]>, // A fixed slide position instead of the column.
    pub(super) gap: f32,             // Space above the block, pixels.
    pub(super) pos: Tween<[f32; 2]>, // Top-left corner on the slide.
    pub(super) placed: bool,         // Whether `pos` has been laid out.
    pub(super) fit: f32,             // Shrink factor that fits the column.
    pub(super) dim: Tween<f32>,      // 0 normal, 1 dimmed.
    pub(super) opacity: f32,
    pub(super) geo_from: Option<(Vec<[f64; 3]>, f64, f64)>, // Points a 3D morph starts from, start, duration.
    pub(super) dir: Option<[f32; 2]>,                       // Authored label direction.
    pub(super) attach: Option<String>,                      // Object a label follows.
    pub(super) boxed: Option<(String, f32)>,                // Target and padding of a box outline.
    pub(super) label_dir: usize,                            // Last chosen label placement.
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
    pub(super) fn block(&self) -> bool {
        matches!(
            self.content,
            Content::Tex(_)
                | Content::Text(_)
                | Content::Code(_)
                | Content::Image { .. }
                | Content::Matrix(_)
                | Content::Row(_)
        ) && self.boxed.is_none()
    }

    /// A point, line, polyline or cuboid.
    pub(super) fn geo(&self) -> bool {
        is_geo(&self.content)
    }

    /// A 3D label.
    pub(super) fn label(&self) -> bool {
        matches!(self.content, Content::Label { .. })
    }

    /// Pixels per em after fitting.
    pub(super) fn scale(&self) -> f32 {
        self.size * self.fit
    }
}

/// The named points and point objects at one time.
pub(super) struct PointSet<'a> {
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

impl Stage {
    /// The index of the object `id`.
    pub(super) fn find(&self, id: &str) -> Option<usize> {
        self.objs.iter().position(|o| o.id == id)
    }

    /// The object and prims of `id`, `M[r,c]`, `M[r,:]`, `eq{x_k}` or a row item id; None for all.
    pub(super) fn resolve(&self, r: &str) -> Option<(usize, Option<Vec<usize>>)> {
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
    pub(super) fn make(&mut self, spec: &ObjSpec) -> Obj {
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
        // a fixed node is a small dot: half the radius of a free one
        let fixed_dot =
            matches!(spec.content, Content::Point(_)) && spec.color.as_deref() == Some("fixed");
        let width = spec.width.map(|w| w as f32).unwrap_or(
            if matches!(spec.content, Content::Cuboid { .. }) || fixed_dot {
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
    pub(super) fn filled(&self, content: &Content) -> (Content, Option<String>) {
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
    pub(super) fn fill(&self, src: &str) -> String {
        eval::fill(src, &self.points_at(self.now))
    }

    /// The named points and point objects at `now`.
    pub(super) fn points_at(&self, now: f64) -> PointSet<'_> {
        PointSet {
            points: &self.points,
            objs: &self.objs,
            now,
        }
    }

    /// Add an object, replacing one with the same id but keeping its place.
    pub(super) fn insert(&mut self, mut obj: Obj) {
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
}
