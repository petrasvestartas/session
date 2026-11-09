use super::font::Fonts;
use super::run::{DPrim, Draw, H, Stage, W};
use super::tex::{Rgba, Shape};
use egui::{Color32, Pos2, Rect, Shape as EShape, Stroke, TextureHandle, TextureOptions};
use std::collections::HashMap;
use swash::scale::{Render, ScaleContext, Source, image::Content};
use swash::zeno::Format;

const N: usize = 2048;

#[derive(Clone, Copy)]
struct Slot {
    uv: Rect,
    left: i32,
    top: i32,
    w: u32,
    h: u32,
}

/// Rasterized glyphs in one egui texture.
pub struct Atlas {
    tex: Option<TextureHandle>,
    slots: HashMap<(u8, u16, u16), Option<Slot>>,
    x: usize,
    y: usize,
    row: usize,
    ctx: ScaleContext,
    faces: Vec<swash::FontRef<'static>>,
    pub overflowed: bool,
}

impl Atlas {
    pub fn new(fonts: &Fonts) -> Self {
        Atlas {
            tex: None,
            slots: HashMap::new(),
            x: 1,
            y: 1,
            row: 0,
            ctx: ScaleContext::new(),
            faces: fonts
                .faces
                .iter()
                .filter_map(|f| swash::FontRef::from_index(f.data, 0))
                .collect(),
            overflowed: false,
        }
    }

    fn clear(&mut self) {
        self.slots.clear();
        self.x = 1;
        self.y = 1;
        self.row = 0;
        self.overflowed = true;
    }

    fn slot(&mut self, ctx: &egui::Context, font: u8, gid: u16, px: u16) -> Option<Slot> {
        if let Some(s) = self.slots.get(&(font, gid, px)) {
            return *s;
        }
        let tex = self.tex.get_or_insert_with(|| {
            ctx.load_texture(
                "stage-glyphs",
                egui::ColorImage::filled([N, N], Color32::TRANSPARENT),
                TextureOptions::LINEAR,
            )
        });
        let face = *self.faces.get(font as usize)?;
        let mut scaler = self.ctx.builder(face).size(px as f32).hint(false).build();
        let image = Render::new(&[Source::Outline])
            .format(Format::Alpha)
            .render(&mut scaler, gid);
        let Some(image) = image.filter(|i| {
            i.content == Content::Mask && i.placement.width > 0 && i.placement.height > 0
        }) else {
            self.slots.insert((font, gid, px), None);
            return None;
        };
        let (w, h) = (
            image.placement.width as usize,
            image.placement.height as usize,
        );
        if w + 2 > N || h + 2 > N {
            self.slots.insert((font, gid, px), None);
            return None;
        }
        if self.x + w + 1 > N {
            self.x = 1;
            self.y += self.row + 1;
            self.row = 0;
        }
        if self.y + h + 1 > N {
            self.clear();
            return None;
        }
        let pixels: Vec<Color32> = image
            .data
            .iter()
            .map(|&a| Color32::from_rgba_premultiplied(a, a, a, a))
            .collect();
        tex.set_partial(
            [self.x, self.y],
            egui::ColorImage::new([w, h], pixels),
            TextureOptions::LINEAR,
        );
        let n = N as f32;
        let slot = Slot {
            uv: Rect::from_min_max(
                Pos2::new(self.x as f32 / n, self.y as f32 / n),
                Pos2::new((self.x + w) as f32 / n, (self.y + h) as f32 / n),
            ),
            left: image.placement.left,
            top: image.placement.top,
            w: w as u32,
            h: h as u32,
        };
        self.x += w + 1;
        self.row = self.row.max(h);
        self.slots.insert((font, gid, px), Some(slot));
        Some(slot)
    }
}

fn c32(c: Rgba) -> Color32 {
    let b = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
    Color32::from_rgba_unmultiplied(b(c[0]), b(c[1]), b(c[2]), b(c[3]))
}

/// The slide-to-screen mapping: points = offset + slide * k.
#[derive(Clone, Copy, Debug)]
pub struct Frame {
    pub k: f32,
    pub ox: f32,
    pub oy: f32,
    pub ppp: f32,
}

impl Frame {
    pub fn new(screen: Rect, ppp: f32) -> Self {
        let k = (screen.width() / W).min(screen.height() / H);
        Frame {
            k,
            ox: screen.min.x + (screen.width() - W * k) / 2.0,
            oy: screen.min.y + (screen.height() - H * k) / 2.0,
            ppp,
        }
    }
    pub fn pt(&self, p: [f32; 2]) -> Pos2 {
        Pos2::new(self.ox + p[0] * self.k, self.oy + p[1] * self.k)
    }
    pub fn slide(&self, p: Pos2) -> [f32; 2] {
        [(p.x - self.ox) / self.k, (p.y - self.oy) / self.k]
    }
}

/// Collects glyph quads in one mesh, flushed before any other shape so the order holds.
struct Out<'a> {
    painter: &'a egui::Painter,
    mesh: Option<egui::Mesh>,
}

impl Out<'_> {
    fn flush(&mut self) {
        if let Some(m) = self.mesh.take()
            && !m.is_empty()
        {
            self.painter.add(EShape::mesh(m));
        }
    }
    fn add(&mut self, s: EShape) {
        self.flush();
        self.painter.add(s);
    }
}

fn prim(out: &mut Out, atlas: &mut Atlas, f: &Frame, p: &DPrim, snap: bool) {
    if p.color[3] <= 0.002 {
        return;
    }
    match p.shape {
        Shape::Glyph { font, gid, size } => {
            let exact = size * f.k * f.ppp;
            if exact < 1.0 {
                return;
            }
            let q = if exact < 24.0 {
                exact.round()
            } else if exact < 64.0 {
                (exact / 2.0).round() * 2.0
            } else {
                (exact / 4.0).round() * 4.0
            };
            let Some(slot) = atlas.slot(out.painter.ctx(), font, gid, q as u16) else {
                return;
            };
            let s = exact / q;
            let mut x = (f.ox + p.x * f.k) * f.ppp;
            let mut y = (f.oy + p.y * f.k) * f.ppp;
            if snap && (s - 1.0).abs() < 0.02 {
                x = x.round();
                y = y.round();
            }
            let left = x + slot.left as f32 * s;
            let top = y - slot.top as f32 * s;
            let rect = Rect::from_min_size(
                Pos2::new(left / f.ppp, top / f.ppp),
                egui::vec2(slot.w as f32 * s / f.ppp, slot.h as f32 * s / f.ppp),
            );
            let tex = atlas.tex.as_ref().map(|t| t.id()).unwrap_or_default();
            let mesh = out
                .mesh
                .get_or_insert_with(|| egui::Mesh::with_texture(tex));
            mesh.add_rect_with_uv(rect, slot.uv, c32(p.color));
        }
        Shape::Rect {
            w,
            h,
            radius,
            fill,
            stroke,
        } => {
            let r = Rect::from_min_size(f.pt([p.x, p.y]), egui::vec2(w * f.k, h * f.k));
            if radius <= 0.0 && stroke <= 0.0 {
                out.add(EShape::rect_filled(
                    r,
                    0.0,
                    c32([p.color[0], p.color[1], p.color[2], p.color[3] * fill]),
                ));
                return;
            }
            let pts: Vec<Pos2> = rounded_pts(r, radius * f.k);
            let fill_c = c32([p.color[0], p.color[1], p.color[2], p.color[3] * fill]);
            let stroke = if stroke > 0.0 {
                Stroke::new((stroke * f.k).max(1.0 / f.ppp), c32(p.color))
            } else {
                Stroke::NONE
            };
            out.add(EShape::convex_polygon(pts, fill_c, stroke));
        }
        Shape::Stroke { dx, dy, width } => {
            let a = f.pt([p.x, p.y]);
            let b = f.pt([p.x + dx, p.y + dy]);
            out.add(EShape::line_segment(
                [a, b],
                Stroke::new((width * f.k).max(1.0 / f.ppp), c32(p.color)),
            ));
        }
    }
}

fn rounded_pts(r: Rect, radius: f32) -> Vec<Pos2> {
    let rad = radius.min(r.width() / 2.0).min(r.height() / 2.0).max(0.0);
    let mut pts = Vec::new();
    let corners = [
        (r.max.x - rad, r.min.y + rad, -90.0f32),
        (r.max.x - rad, r.max.y - rad, 0.0),
        (r.min.x + rad, r.max.y - rad, 90.0),
        (r.min.x + rad, r.min.y + rad, 180.0),
    ];
    for (cx, cy, a0) in corners {
        for i in 0..=6 {
            let a = (a0 + 15.0 * i as f32).to_radians();
            pts.push(Pos2::new(cx + rad * a.cos(), cy + rad * a.sin()));
        }
    }
    pts
}

fn draw(out: &mut Out, atlas: &mut Atlas, f: &Frame, d: &Draw, snap: bool) {
    match d {
        Draw::Prim(p) => prim(out, atlas, f, p, snap),
        Draw::Line {
            a,
            b,
            width,
            color,
            arrow,
            dash,
        } => {
            if color[3] <= 0.002 {
                return;
            }
            let (pa, pb) = (f.pt(*a), f.pt(*b));
            let w = (width * f.k).max(0.8);
            let stroke = Stroke::new(w, c32(*color));
            let v = pb - pa;
            let len = v.length();
            if len < 0.5 {
                return;
            }
            let dir = v / len;
            let mut end = pb;
            if *arrow {
                let head = (7.0 + width * 2.6) * f.k;
                let head = head.min(len * 0.6);
                let side = head * 0.58;
                let base = pb - dir * head;
                let n = egui::vec2(-dir.y, dir.x);
                end = pb - dir * head * 0.8;
                out.add(EShape::convex_polygon(
                    vec![pb, base + n * side * 0.5, base - n * side * 0.5],
                    c32(*color),
                    Stroke::NONE,
                ));
            }
            if *dash {
                let dl = 7.0 * f.k;
                for s in EShape::dashed_line(&[pa, end], stroke, dl, dl * 0.8) {
                    out.add(s);
                }
            } else {
                out.add(EShape::line_segment([pa, end], stroke));
            }
        }
        Draw::Disc { c, r, color, ring } => {
            let p = f.pt(*c);
            if *r <= 0.0 {
                return;
            }
            out.add(EShape::circle_filled(p, (r + 2.5) * f.k, c32(*ring)));
            out.add(EShape::circle_filled(p, r * f.k, c32(*color)));
        }
        Draw::Path {
            pts,
            closed,
            width,
            color,
        } => {
            if pts.len() < 2 || color[3] <= 0.002 {
                return;
            }
            let p: Vec<Pos2> = pts.iter().map(|q| f.pt(*q)).collect();
            let stroke = Stroke::new((width * f.k).max(0.8), c32(*color));
            if *closed {
                out.add(EShape::closed_line(p, stroke));
            } else {
                out.add(EShape::line(p, stroke));
            }
        }
    }
}

fn text(
    stage: &mut Stage,
    out: &mut Out,
    atlas: &mut Atlas,
    f: &Frame,
    src: &str,
    bold: bool,
    at: [f32; 2],
    size: f32,
    width: f32,
    color: Rgba,
) -> [f32; 2] {
    if src.is_empty() || color[3] <= 0.0 {
        return [0.0, 0.0];
    }
    let e = stage.chrome(src, bold, width / size);
    for p in &e.prims {
        let c = p
            .color
            .map_or(color, |c| [c[0], c[1], c[2], c[3] * color[3]]);
        let shape = match p.shape {
            Shape::Glyph { font, gid, size: s } => Shape::Glyph {
                font,
                gid,
                size: s * size,
            },
            Shape::Rect {
                w,
                h,
                radius,
                fill,
                stroke,
            } => Shape::Rect {
                w: w * size,
                h: h * size,
                radius,
                fill,
                stroke,
            },
            Shape::Stroke { dx, dy, width } => Shape::Stroke {
                dx: dx * size,
                dy: dy * size,
                width: width * size,
            },
        };
        let d = DPrim {
            shape,
            x: at[0] + p.x * size,
            y: at[1] + e.h * size + p.y * size,
            color: c,
        };
        prim(out, atlas, f, &d, true);
    }
    [e.w * size, (e.h + e.d) * size]
}

/// Paint the stage; returns the chapter tabs for clicks.
pub fn paint(
    stage: &mut Stage,
    atlas: &mut Atlas,
    painter: &egui::Painter,
    f: &Frame,
    screen: Rect,
) -> Vec<(Rect, usize)> {
    let bg = stage.palette("bg");
    painter.add(EShape::rect_filled(screen, 0.0, c32(bg)));
    let snap = !stage.animating();
    let draws = stage.frame(stage.now);
    let mut out = Out {
        painter,
        mesh: None,
    };
    let faint = stage.palette("faint");
    let x = stage.split * W;
    out.add(EShape::line_segment(
        [f.pt([x, 96.0]), f.pt([x, H - 56.0])],
        Stroke::new(1.0, c32([faint[0], faint[1], faint[2], 0.5])),
    ));
    // the 3D layer stays left of the divider, whatever the camera does
    let clip = Rect::from_min_max(f.pt([0.0, 0.0]), f.pt([x, H]));
    let spatial = painter.with_clip_rect(clip.intersect(screen));
    let mut left = Out { painter: &spatial, mesh: None };
    for d in &draws[..stage.spatial.min(draws.len())] {
        draw(&mut left, atlas, f, d, snap);
    }
    left.flush();
    for d in &draws[stage.spatial.min(draws.len())..] {
        draw(&mut out, atlas, f, d, snap);
    }
    // chrome: chapter, title, narration, tabs, progress
    let ink = stage.palette("ink");
    let dim = stage.palette("dim");
    let accent = stage.palette("accent");
    let now = stage.now;
    let ch = stage.chapters[stage.chapter].title.clone();
    let head = format!("{} · {}", stage.chapter + 1, ch);
    text(
        stage,
        &mut out,
        atlas,
        f,
        &head,
        false,
        [40.0, 30.0],
        17.0,
        900.0,
        dim,
    );
    let x0 = stage.split * W + 33.6;
    let pw = W - 56.0 - x0;
    // the old text leaves first, then the new one rises in: they never overlap
    let fade = |t0: f64| (((now - t0) / 650.0).clamp(0.0, 1.0)) as f32;
    let out_a = |t: f32| 1.0 - super::run::smooth((t / 0.4) as f64) as f32;
    let in_a = |t: f32| super::run::smooth(((t - 0.4) / 0.6) as f64) as f32;
    let (title, old_title, t_title) = stage.title.clone();
    let ft = fade(t_title);
    if ft < 1.0 {
        text(stage, &mut out, atlas, f, &old_title, true, [x0, 46.0], 30.0, pw, [ink[0], ink[1], ink[2], out_a(ft)]);
    }
    text(stage, &mut out, atlas, f, &title, true, [x0, 46.0 + (1.0 - in_a(ft)) * 8.0], 30.0, pw, [ink[0], ink[1], ink[2], in_a(ft)]);
    let (say, old_say, t_say) = stage.say.clone();
    let fs = fade(t_say);
    let say_color = [
        ink[0] * 0.86 + dim[0] * 0.14,
        ink[1] * 0.86 + dim[1] * 0.14,
        ink[2] * 0.86 + dim[2] * 0.14,
        1.0,
    ];
    if fs < 1.0 {
        text(stage, &mut out, atlas, f, &old_say, false, [x0, H - 128.0], 23.0, pw, [say_color[0], say_color[1], say_color[2], out_a(fs)]);
    }
    let rise = (1.0 - in_a(fs)) * 6.0;
    text(stage, &mut out, atlas, f, &say, false, [x0, H - 128.0 + rise], 23.0, pw, [say_color[0], say_color[1], say_color[2], in_a(fs)]);
    // chapter tabs
    let n = stage.chapters.len();
    let mut tabs = Vec::new();
    for i in 0..n {
        let cx = W - 56.0 - (n - 1 - i) as f32 * 34.0 - 13.0;
        let r = Rect::from_center_size(f.pt([cx, 40.0]), egui::vec2(26.0 * f.k, 26.0 * f.k));
        let current = i == stage.chapter;
        let fill = if current {
            accent
        } else {
            [faint[0], faint[1], faint[2], 0.0]
        };
        out.add(EShape::circle_filled(r.center(), 13.0 * f.k, c32(fill)));
        out.add(EShape::circle_stroke(
            r.center(),
            13.0 * f.k,
            Stroke::new(1.2, c32(if current { accent } else { dim })),
        ));
        let label = format!("{}", i + 1);
        let tc = if current { stage.palette("bg") } else { dim };
        let e = stage.chrome(&label, current, 100.0);
        let size = 15.0;
        let at = [
            cx - e.w * size / 2.0,
            40.0 - (e.h - e.d) * size / 2.0 - e.h * size + e.h * size,
        ];
        text(
            stage,
            &mut out,
            atlas,
            f,
            &label,
            current,
            [at[0], 40.0 - e.h * size / 2.0 - 1.0],
            size,
            100.0,
            tc,
        );
        tabs.push((r, i));
    }
    // progress along the chapter
    let steps = stage.steps().max(1);
    let done = stage.step.map_or(0, |g| g + 1);
    let y = H - 6.0;
    out.add(EShape::line_segment(
        [f.pt([0.0, y]), f.pt([W, y])],
        Stroke::new(2.0 * f.k, c32([faint[0], faint[1], faint[2], 0.6])),
    ));
    out.add(EShape::line_segment(
        [f.pt([0.0, y]), f.pt([W * done as f32 / steps as f32, y])],
        Stroke::new(2.0 * f.k, c32(accent)),
    ));
    let (s, _) = stage.position();
    let slides = stage.chapters[stage.chapter].slides.len();
    let counter = format!("slide {} of {}", s + 1, slides);
    let e = stage.chrome(&counter, false, 400.0);
    text(
        stage,
        &mut out,
        atlas,
        f,
        &counter,
        false,
        [W - 56.0 - e.w * 14.0, H - 40.0],
        14.0,
        400.0,
        dim,
    );
    text(
        stage,
        &mut out,
        atlas,
        f,
        "← → step   ↑ ↓ slide   drag to orbit",
        false,
        [40.0, H - 40.0],
        14.0,
        600.0,
        [dim[0], dim[1], dim[2], 0.7],
    );
    if !stage.errors.is_empty() {
        let msg = stage
            .errors
            .iter()
            .rev()
            .take(3)
            .cloned()
            .collect::<Vec<_>>()
            .join(" · ");
        let red = [0.95, 0.45, 0.45, 1.0];
        text(
            stage,
            &mut out,
            atlas,
            f,
            &msg.replace('$', ""),
            false,
            [40.0, 70.0],
            15.0,
            1500.0,
            red,
        );
    }
    out.flush();
    tabs
}
