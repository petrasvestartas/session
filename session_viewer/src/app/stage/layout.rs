use super::deck::{Content, PINNED};
use super::object::Align;
use super::run::{BOTTOM, H, MARGIN, RELAYOUT_MS, Stage, TOP, W};
use super::tex::{self, Expr, Prim, Rgba, Shape};
use super::tween::Tween;
use std::collections::HashMap;
use std::rc::Rc;

impl Stage {
    /// A palette colour or hex colour, `fallback` when `name` is None.
    pub(super) fn color(&self, name: Option<&str>, fallback: &str) -> Rgba {
        let name = name.unwrap_or(fallback);
        self.palette
            .get(name)
            .copied()
            .or_else(|| tex::hex(name))
            .or_else(|| self.palette.get("ink").copied())
            .unwrap_or([1.0; 4])
    }

    /// Cached layout: mode 0 display TeX, 1 inline TeX, 2 prose, 3 bold prose.
    pub(super) fn typeset(&mut self, src: &str, mode: u8, width: f32) -> Rc<Expr> {
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

    /// A picture of the deck, `height` em tall and centred on the math axis.
    fn picture(&self, src: &str, height: f32) -> Expr {
        let mut picture = Expr::default();
        match self.pictures.get(src) {
            Some(&(index, aspect)) => {
                let (w, h) = (aspect * height, height);
                picture.prims.push(Prim {
                    shape: Shape::Image { index, w, h },
                    x: 0.0,
                    y: -h / 2.0 - self.fonts.c.axis,
                    color: None,
                    part: None,
                });
                picture.w = w;
                picture.h = h / 2.0 + self.fonts.c.axis;
                picture.d = h / 2.0 - self.fonts.c.axis;
            }
            None => picture.unknown.push(format!("picture {src}")),
        }
        picture
    }

    /// Left and right edge of the column.
    fn panel(&self) -> (f32, f32) {
        (self.split * W + MARGIN * 0.6, W - MARGIN)
    }

    /// Lay out content `size` pixels per em, with its named prim groups.
    pub(super) fn compile(
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
            Content::Label { tex, .. } => match tex.strip_prefix(PINNED) {
                Some(picture) => {
                    let (src, height) = picture.rsplit_once(':').unwrap_or((picture, "7"));
                    Rc::new(self.picture(src, height.parse().unwrap_or(7.0)))
                }
                None => self.typeset(tex, 1, 0.0),
            },
            Content::Text(s) => {
                let (x0, x1) = self.panel();
                self.typeset(s, 2, (x1 - x0) / size)
            }
            Content::Code(s) => Rc::new(tex::code(&self.fonts, s, &self.palette)),
            Content::Image { src, height } => Rc::new(self.picture(src, *height)),
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

    /// Place every panel block: stacked from the top, equals aligned, wide blocks shrunk.
    pub(super) fn relayout(&mut self, now: f64, animate: bool) {
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

    /// A palette colour, ink when unknown.
    pub fn palette(&self, name: &str) -> Rgba {
        self.color(Some(name), "ink")
    }

    /// Chrome prose wrapped at `width` em.
    pub fn chrome(&mut self, text: &str, bold: bool, width: f32) -> Rc<Expr> {
        self.typeset(text, if bold { 3 } else { 2 }, width)
    }
}
