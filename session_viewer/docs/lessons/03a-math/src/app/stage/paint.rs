// --8<-- [start:implementation]
use super::draw::rounded;
use super::font::Fonts;
use super::run::{DPrim, Draw, H, Stage, W};
use super::tex::{Rgba, Shape};
use super::tween::smooth;
use egui::{Color32, Pos2, Rect, Shape as EShape, Stroke, TextureHandle, TextureOptions};
use std::collections::HashMap;
use swash::scale::{Render, ScaleContext, Source, image::Content};
use swash::zeno::Format;

/// Side of the glyph atlas texture, pixels.
const N: usize = 2048;

/// The ETH Zurich and BRG marks, white on clear; the painter tints them with the ink colour.
const LOGOS: [&[u8]; 2] = [
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../assets/stage/logo-eth.png")),
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../assets/stage/logo-brg.png")),
];

// --8<-- [start:atlas]
/// Where one rasterized glyph sits in the atlas.
#[derive(Clone, Copy)]
struct Slot {
    uv: Rect,  // Texture coordinates.
    left: i32, // Bitmap offset from the pen point, pixels.
    top: i32,  // Bitmap top above the baseline, pixels.
    w: u32,    // Bitmap width, pixels.
    h: u32,    // Bitmap height, pixels.
}

/// A decoded picture: its size in pixels and its RGBA bytes.
pub struct Picture {
    pub size: [usize; 2],
    pub rgba: Vec<u8>,
}

/// Decode a PNG file.
pub fn picture(bytes: &[u8]) -> Result<Picture, String> {
    let mut decoder = png::Decoder::new(bytes);
    decoder.set_transformations(png::Transformations::EXPAND | png::Transformations::STRIP_16);
    let mut reader = decoder.read_info().map_err(|e| e.to_string())?;
    let mut data = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut data).map_err(|e| e.to_string())?;
    data.truncate(info.buffer_size());
    let rgba: Vec<u8> = match info.color_type {
        png::ColorType::Rgba => data,
        png::ColorType::Rgb => data
            .chunks(3)
            .flat_map(|p| [p[0], p[1], p[2], 255])
            .collect(),
        png::ColorType::GrayscaleAlpha => data
            .chunks(2)
            .flat_map(|p| [p[0], p[0], p[0], p[1]])
            .collect(),
        png::ColorType::Grayscale => data.iter().flat_map(|g| [*g, *g, *g, 255]).collect(),
        png::ColorType::Indexed => return Err("indexed PNG did not expand".into()),
    };
    Ok(Picture {
        size: [info.width as usize, info.height as usize],
        rgba,
    })
}

/// Rasterized glyphs in one egui texture.
pub struct Atlas {
    tex: Option<TextureHandle>,
    pub pictures: Vec<Picture>, // The deck's pictures, by index.
    picture_tex: Vec<Option<TextureHandle>>, // Their textures, made on first use.
    logos: Vec<Picture>,        // ETH Zurich and BRG, bottom right.
    logo_tex: Vec<TextureHandle>, // Their textures, made on first use.
    slots: HashMap<(u8, u16, u16), Option<Slot>>, // By font, glyph and pixel size; None for no ink.
    x: usize,                   // Packing cursor.
    y: usize,                   // Top of the current shelf.
    row: usize,                 // Height of the current shelf.
    ctx: ScaleContext,
    faces: Vec<swash::FontRef<'static>>,
    pub overflowed: bool, // The atlas was cleared this frame, so it must be painted again.
}

impl Atlas {
    /// An empty atlas over the stage fonts.
    pub fn new(fonts: &Fonts) -> Self {
        Atlas {
            tex: None,
            pictures: Vec::new(),
            picture_tex: Vec::new(),
            logos: LOGOS.iter().filter_map(|b| picture(b).ok()).collect(),
            logo_tex: Vec::new(),
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

    /// Forget every glyph and start packing again.
    fn clear(&mut self) {
        self.slots.clear();
        self.x = 1;
        self.y = 1;
        self.row = 0;
        self.overflowed = true;
    }

    /// The atlas slot of a glyph at `px` pixels, rasterized on first use.
    fn slot(&mut self, ctx: &egui::Context, font: u8, gid: u16, px: u16) -> Option<Slot> {
        let key = (font, gid, px);
        if let Some(s) = self.slots.get(&key) {
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
            self.slots.insert(key, None);
            return None;
        };
        let (w, h) = (
            image.placement.width as usize,
            image.placement.height as usize,
        );
        if w + 2 > N || h + 2 > N {
            self.slots.insert(key, None);
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
        self.slots.insert(key, Some(slot));
        Some(slot)
    }
}

// --8<-- [end:atlas]

/// An egui colour from an RGBA in 0..1.
fn c32(c: Rgba) -> Color32 {
    let b = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
    Color32::from_rgba_unmultiplied(b(c[0]), b(c[1]), b(c[2]), b(c[3]))
}

// --8<-- [start:frame]
/// The slide-to-screen mapping: points = offset + slide * k.
#[derive(Clone, Copy, Debug)]
pub struct Frame {
    pub k: f32,
    pub ox: f32,
    pub oy: f32,
    pub ppp: f32,
    below: Option<(f32, [f32; 3])>, // On a tall screen: the split and the column's own scale and offset.
}

impl Frame {
    /// The largest letterboxed slide that fits `screen`.
    pub fn new(screen: Rect, ppp: f32) -> Self {
        let k = (screen.width() / W).min(screen.height() / H);
        Frame {
            k,
            ox: screen.min.x + (screen.width() - W * k) / 2.0,
            oy: screen.min.y + (screen.height() - H * k) / 2.0,
            ppp,
            below: None,
        }
    }

    /// On a screen taller than wide, the 3D side fills the top and the column sits below it.
    pub fn stacked(self, screen: Rect, split: f32) -> Self {
        if screen.height() < 1.1 * screen.width() {
            return self;
        }
        let (left, right) = (split * W, (1.0 - split) * W);
        let fit = (screen.height() / (H * screen.width() * (1.0 / left + 1.0 / right))).min(1.0);
        let (top, bottom) = (fit * screen.width() / left, fit * screen.width() / right);
        Frame {
            k: top,
            ox: screen.min.x + (screen.width() - left * top) / 2.0,
            oy: screen.min.y,
            ppp: self.ppp,
            below: Some((
                left,
                [
                    bottom,
                    screen.min.x + (screen.width() - right * bottom) / 2.0 - left * bottom,
                    screen.min.y + H * top,
                ],
            )),
        }
    }

    /// The frame that draws what sits at slide `x`: the 3D side or the column.
    pub fn side(&self, x: f32) -> Frame {
        match self.below {
            Some((split, [k, ox, oy])) if x >= split => Frame {
                k,
                ox,
                oy,
                ppp: self.ppp,
                below: None,
            },
            _ => Frame {
                below: None,
                ..*self
            },
        }
    }

    /// True when the column is stacked under the 3D side.
    pub fn is_stacked(&self) -> bool {
        self.below.is_some()
    }

    /// A slide point on screen.
    pub fn pt(&self, p: [f32; 2]) -> Pos2 {
        Pos2::new(self.ox + p[0] * self.k, self.oy + p[1] * self.k)
    }

    /// A screen point on the slide.
    pub fn slide(&self, p: Pos2) -> [f32; 2] {
        match self.below {
            Some((_, [k, ox, oy])) if p.y >= oy => [(p.x - ox) / k, (p.y - oy) / k],
            _ => [(p.x - self.ox) / self.k, (p.y - self.oy) / self.k],
        }
    }
}

// --8<-- [end:frame]

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

/// Paint one formula piece; `snap` puts unscaled glyphs on whole pixels.
// --8<-- [start:formula]
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
            let fill_c = c32([p.color[0], p.color[1], p.color[2], p.color[3] * fill]);
            if radius <= 0.0 && stroke <= 0.0 {
                out.add(EShape::rect_filled(r, 0.0, fill_c));
                return;
            }
            let corners = rounded(r.min.x, r.min.y, r.width(), r.height(), radius * f.k);
            let pts: Vec<Pos2> = corners.iter().map(|q| Pos2::new(q[0], q[1])).collect();
            let stroke = if stroke > 0.0 {
                Stroke::new((stroke * f.k).max(1.0 / f.ppp), c32(p.color))
            } else {
                Stroke::NONE
            };
            out.add(EShape::convex_polygon(pts, fill_c, stroke));
        }
        Shape::Image { index, w, h } => {
            let i = index as usize;
            let Some(picture) = atlas.pictures.get(i) else {
                return;
            };
            if atlas.picture_tex.len() <= i {
                atlas.picture_tex.resize(i + 1, None);
            }
            let ctx = out.painter.ctx().clone();
            let texture = atlas.picture_tex[i].get_or_insert_with(|| {
                let image = egui::ColorImage::from_rgba_unmultiplied(picture.size, &picture.rgba);
                ctx.load_texture(format!("stage-picture-{i}"), image, TextureOptions::LINEAR)
            });
            let rect = Rect::from_min_size(f.pt([p.x, p.y]), egui::vec2(w * f.k, h * f.k));
            let uv = Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0));
            out.flush();
            let tint = c32([1.0, 1.0, 1.0, p.color[3]]);
            out.painter.image(texture.id(), rect, uv, tint);
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

/// ETH Zurich | BRG at the bottom right of the column, in `ink`.
fn logos(out: &mut Out, atlas: &mut Atlas, f: &Frame, ink: Rgba) {
    const HIGH: f32 = 21.0; // height of the ETH mark, slide pixels
    const GAP: f32 = 13.0; // either side of the rule between the marks
    const RIGHT: f32 = 40.0; // from the right edge of the slide
    const BASE: f32 = H - 30.0; // bottom of both marks
    if atlas.logos.len() < 2 {
        return;
    }
    if atlas.logo_tex.is_empty() {
        let ctx = out.painter.ctx().clone();
        let options = TextureOptions {
            mipmap_mode: Some(egui::TextureFilter::Linear),
            ..TextureOptions::LINEAR
        };
        for (i, p) in atlas.logos.iter().enumerate() {
            let image = egui::ColorImage::from_rgba_unmultiplied(p.size, &p.rgba);
            atlas
                .logo_tex
                .push(ctx.load_texture(format!("stage-logo-{i}"), image, options));
        }
    }
    // BRG is a heavy face: a little smaller than ETH reads as the same size
    let heights = [HIGH, HIGH * 0.82];
    let widths: Vec<f32> = atlas
        .logos
        .iter()
        .zip(heights)
        .map(|(p, h)| h * p.size[0] as f32 / p.size[1].max(1) as f32)
        .collect();
    let mut x = W - RIGHT - widths[0] - 2.0 * GAP - widths[1];
    let uv = Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0));
    out.flush();
    let tint = c32(ink);
    for i in 0..2 {
        let rect = Rect::from_min_max(f.pt([x, BASE - heights[i]]), f.pt([x + widths[i], BASE]));
        out.painter.image(atlas.logo_tex[i].id(), rect, uv, tint);
        x += widths[i] + GAP;
        if i == 0 {
            out.painter.line_segment(
                [f.pt([x, BASE - HIGH - 2.0]), f.pt([x, BASE + 2.0])],
                Stroke::new((1.2 * f.k).max(1.0 / f.ppp), tint),
            );
            x += GAP;
        }
    }
}

/// Paint one draw item.
fn draw(out: &mut Out, atlas: &mut Atlas, f: &Frame, d: &Draw, snap: bool) {
    match d {
        Draw::Prim(p) => prim(out, atlas, f, p, snap),
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

// --8<-- [end:formula]

/// Paint chrome text at `at`, `size` pixels per em; returns its width and height.
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
        let d = DPrim {
            shape: p.shape.resized(size),
            x: at[0] + p.x * size,
            y: at[1] + e.h * size + p.y * size,
            color: c,
        };
        prim(out, atlas, f, &d, true);
    }
    [e.w * size, (e.h + e.d) * size]
}

/// Where the step's sentence starts, above the column.
const SAY: f32 = 40.0;
/// The size of every line of running text on the slide, pixels per em.
const TEXT: f32 = 23.0;

/// Paint the stage.
pub fn paint(
    stage: &mut Stage,
    atlas: &mut Atlas,
    painter: &egui::Painter,
    f: &Frame,
    screen: Rect,
) {
    let bg = stage.palette("bg");
    // the scene shows through on the 3D side; the column gets an opaque ground
    let whole = f;
    let (scene_side, f) = (&whole.side(0.0), &whole.side(W));
    let corner = f.pt([stage.split * W, 0.0]);
    let ground = if whole.is_stacked() {
        Rect::from_min_max(Pos2::new(screen.min.x, corner.y), screen.max)
    } else {
        Rect::from_min_max(Pos2::new(corner.x, screen.min.y), screen.max)
    };
    painter.add(EShape::rect_filled(ground, 0.0, c32(bg)));
    let snap = !stage.animating();
    let draws = stage.frame(stage.now);
    let mut out = Out {
        painter,
        mesh: None,
    };
    let faint = stage.palette("faint");
    let x = stage.split * W;
    if !whole.is_stacked() {
        out.add(EShape::line_segment(
            [f.pt([x, 96.0]), f.pt([x, H - 56.0])],
            Stroke::new(1.0, c32([faint[0], faint[1], faint[2], 0.5])),
        ));
    }
    // the 3D layer stays on its side of the divider, whatever the camera does
    let clip = Rect::from_min_max(scene_side.pt([0.0, 0.0]), scene_side.pt([x, H]));
    let spatial = painter.with_clip_rect(clip.intersect(screen));
    let mut left = Out {
        painter: &spatial,
        mesh: None,
    };
    let (spatial_draws, panel_draws) = draws.split_at(stage.spatial.min(draws.len()));
    for d in spatial_draws {
        draw(&mut left, atlas, scene_side, d, snap);
    }
    left.flush();
    for d in panel_draws {
        draw(&mut out, atlas, f, d, snap);
    }
    // chrome: chapter and slide, the step's sentence, progress
    let ink = stage.palette("ink");
    let now = stage.now;
    let chapter_title = &stage.chapters[stage.chapter].title;
    // where we are, in the size of every other line of text on the slide
    let head = if *chapter_title == stage.title.0 {
        chapter_title.clone()
    } else {
        format!("{} · {}", chapter_title, stage.title.0)
    };
    text(
        stage,
        &mut out,
        atlas,
        scene_side,
        &head,
        false,
        [40.0, SAY],
        TEXT,
        stage.split * W - 80.0,
        ink,
    );
    let x0 = stage.split * W + 33.6;
    let pw = W - 56.0 - x0;
    // the old text leaves first, then the new one rises in: they never overlap
    let fade = |t0: f64| (((now - t0) / 650.0).clamp(0.0, 1.0)) as f32;
    let out_a = |t: f32| 1.0 - smooth((t / 0.4) as f64) as f32;
    let in_a = |t: f32| smooth(((t - 0.4) / 0.6) as f64) as f32;
    let (say, old_say, t_say) = stage.say.clone();
    let fs = fade(t_say);
    let say_color = ink;
    if fs < 1.0 {
        let leaving = [say_color[0], say_color[1], say_color[2], out_a(fs)];
        text(
            stage,
            &mut out,
            atlas,
            f,
            &old_say,
            false,
            [x0, SAY],
            TEXT,
            pw,
            leaving,
        );
    }
    let rise = (1.0 - in_a(fs)) * 6.0;
    let entering = [say_color[0], say_color[1], say_color[2], in_a(fs)];
    text(
        stage,
        &mut out,
        atlas,
        f,
        &say,
        false,
        [x0, SAY + rise],
        TEXT,
        pw,
        entering,
    );
    // one bar for the whole presentation: chapters follow each other without a break
    let (done, steps) = stage.progress();
    let strip = 7.0 * f.k;
    let bottom = screen.bottom();
    let reached = screen.left() + screen.width() * done as f32 / steps as f32;
    let track = Rect::from_min_max(Pos2::new(screen.left(), bottom - strip), screen.max);
    out.add(EShape::rect_filled(
        track,
        0.0,
        c32([faint[0], faint[1], faint[2], 0.6]),
    ));
    let filled = Rect::from_min_max(track.min, Pos2::new(reached, bottom));
    out.add(EShape::rect_filled(filled, 0.0, c32(ink)));
    logos(&mut out, atlas, f, ink);
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
            scene_side,
            &msg.replace('$', ""),
            false,
            [40.0, 70.0],
            15.0,
            1500.0,
            red,
        );
    }
    out.flush();
}
// --8<-- [end:implementation]
