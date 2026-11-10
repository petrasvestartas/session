use super::deck::{Action, Content, Kind, MatrixSpec, Pos, plain};
use super::eval;
use super::morph::match_prims;
use super::run::{Anim, Draw, Fx, Stage, Style};
use super::tex::{Prim, Shape};
use super::tween::{Tween, with_alpha};
use std::collections::{HashMap, HashSet};

impl Stage {
    /// Run one action at time `now`, animated or applied at once.
    pub(super) fn apply(&mut self, a: &Action, now: f64, animate: bool) {
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
}
