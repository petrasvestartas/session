use super::deck::MatrixSpec;
use super::run::Stage;
use super::tex::{Expr, Prim, Rgba, Shape};
use std::collections::HashMap;
use std::rc::Rc;

impl Stage {
    /// A matrix as rounded cells with centered entries, em units.
    pub(super) fn matrix(&mut self, spec: &MatrixSpec) -> (Expr, HashMap<String, Vec<usize>>) {
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
        let mut fills = vec![vec![neutral; cols]; rows];
        let (ink, ground) = (self.color(Some("ink"), "ink"), self.color(Some("bg"), "bg"));
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
                fills[r][c] = color;
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
                // an entry is written in whichever of ink and ground stands out more from its cell
                let light = |c: Rgba| 0.299 * c[0] + 0.587 * c[1] + 0.114 * c[2];
                let fill = light(fills[r][c]);
                let on = if (light(ink) - fill).abs() >= (light(ground) - fill).abs() {
                    ink
                } else {
                    ground
                };
                for p in &ex.prims {
                    let mut q = *p;
                    q.color = q.color.or(Some(on));
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
}
