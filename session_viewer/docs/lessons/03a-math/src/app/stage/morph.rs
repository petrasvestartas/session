// --8<-- [start:implementation]
use super::deck::Content;
use super::run::{Anim, DPrim, Fx, Stage};
use super::tex::{self, Expr, Prim, Rgba, Shape};
use super::tween::{lerp, mix};
use std::collections::{HashMap, HashSet};

/// Index pairs of a longest common subsequence.
// --8<-- [start:matching]
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
pub(super) fn match_prims(
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

// --8<-- [end:matching]
/// A formula piece in slide pixels: scaled by `k`, moved to (ox, oy), coloured.
pub(super) fn scaled(p: &Prim, k: f32, ox: f32, oy: f32, color: Rgba) -> DPrim {
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
pub(super) fn lerp_prim(a: &DPrim, b: &DPrim, t: f32) -> DPrim {
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

impl Stage {
    /// Change object `i` to new content, morphing its pieces when `animate`.
    pub(super) fn morph_to(
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
}
// --8<-- [end:implementation]
