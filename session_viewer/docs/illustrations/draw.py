#!/usr/bin/env python3
"""Generate the course illustrations as SVG from a layout that sizes every box from its text.

Palette: the BRG Equilibrium drawing library (navy compression, pink tension, green loads,
yellow hover, orange inspector, pale bands). It is the drawing's own and stays put, because a
reader carries these colours from a diagram to the running viewer. What the Claude Design
system in ../stylesheets/theme.json does own is the chrome: the corner radius of the frame and
of every box, so the drawing and the plate course.css mounts it on agree.

Run `python3 docs/illustrations/draw.py`, then `node docs/check_illustrations.cjs --write`
measures every label in Chrome, fails on any overflow, and pins each measured width.
"""
from pathlib import Path
import json

HERE = Path(__file__).resolve().parent
THEME = json.loads((HERE.parent / "stylesheets/theme.json").read_text())
RADIUS = THEME["radius"]
# The ground, and the dark ink drawn on a light shape. BRG black, not the design system's ink:
# these drawings and the Mermaid diagrams are one set, and the Mermaid ones already carry BRG
# pink. course.css spends the same value on Mermaid so the two never drift.
INK = "#111111"

PAL = {
    "navy": "#0b1957", "pink": "#ce4095", "green": "#3f9c20", "yellow": "#e8ac00",
    "yellow_light": "#f9e08a", "orange": "#e07a26", "ghost": "#9ed4c9", "grey": "#aaaaaa",
    "zero": "#b9b9bd", "pink_band": "#fa9ebc", "blue_band": "#bdbfe8", "zero_band": "#e4e4e7",
    "black": INK, "text2": "#455b6b", "page": "#eef0f2", "white": "#ffffff",
}
# Box kinds: (fill, stroke). CPU/Rust = blue band, GPU/WGSL = pink band, note = zero band,
# selection = yellow light, warning = orange stroke.
# A box says what it is with its fill alone: no outline anywhere, so nothing reads as a border
# around a colour. The stroke is kept equal to the fill so one rect rule serves every renderer.
KIND = {
    "cpu": (PAL["blue_band"], PAL["blue_band"]),
    "gpu": (PAL["pink_band"], PAL["pink_band"]),
    "note": (PAL["zero_band"], PAL["zero_band"]),
    "sel": (PAL["yellow_light"], PAL["yellow_light"]),
    "warn": (PAL["white"], PAL["white"]),
    "plain": (PAL["white"], PAL["white"]),
}
# The page is black like the Mermaid diagrams; boxes are white with black text, free labels are
# light. Colours named for the light palette are remapped when they would vanish on black.
ON_BLACK = {
    PAL["navy"]: "#bdbfe8", PAL["pink"]: "#fa9ebc", PAL["green"]: "#8fd36a", PAL["text2"]: "#c9ccd6",
    PAL["black"]: "#f4f4f6", "#111": "#f4f4f6", PAL["grey"]: "#b9b9bd", PAL["zero"]: "#b9b9bd",
    PAL["white"]: INK,
}
# Average glyph advance per em, calibrated against Chrome on the build host; the checker
# writes `textLength` from real measurements so other machines cannot overflow either.
EM = {"sans": 0.53, "sansb": 0.59, "mono": 0.605}
SIZE = {"h": 20, "l": 15, "s": 13, "m": 12.5}
FONT = {"sans": "system-ui, sans-serif", "mono": '"DejaVu Sans Mono", "Liberation Mono", monospace'}
STYLE = (
    "text{font-family:system-ui,sans-serif;fill:#f4f4f6;font-size:13px}"
    ".h{font-size:20px;font-weight:700}.l{font-size:15px;font-weight:650}"
    ".s{font-size:13px;fill:#c9ccd6}.m{font-family:\"DejaVu Sans Mono\",\"Liberation Mono\",monospace;font-size:12.5px;fill:#f4f4f6}"
    ".ar{fill:none;stroke:#e4e4e7;stroke-width:1.6;marker-end:url(#a)}"
    ".dash{fill:none;stroke:#e07a26;stroke-width:1.8;stroke-dasharray:7 5}"
)
MARKER = ('<defs><marker id="a" markerWidth="9" markerHeight="9" refX="7" refY="3" orient="auto">'
          '<path d="M0,0 L7,3 L0,6" fill="#e4e4e7"/></marker></defs>')


def width(text, cls):
    """Estimated advance of a label; the checker later replaces it with the measured value."""
    if cls == "m":
        return len(text) * SIZE["m"] * EM["mono"]
    if cls == "h":
        return len(text) * SIZE["h"] * EM["sansb"]
    if cls == "l":
        return len(text) * SIZE["l"] * EM["sansb"]
    return len(text) * SIZE["s"] * EM["sans"]


def esc(text):
    return text.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;")


class Canvas:
    """Collects elements; every text is registered with the box it must stay inside."""

    def __init__(self, title, desc, w, h):
        self.title, self.desc, self.w, self.h = title, desc, w, h
        self.parts = []
        self.box_count = 0

    def text(self, x, y, s, cls="s", anchor="start", fill=None, box=None, keep=False):
        """`keep` leaves an explicit fill alone: for a label that sits on a light shape."""
        if fill and box is None and not keep:
            fill = ON_BLACK.get(fill, fill)
        extra = f' style="fill:{fill}"' if fill else ""
        attr = f' text-anchor="{anchor}"' if anchor != "start" else ""
        data = f' data-box="{box}"' if box is not None else ""
        self.parts.append(f'<text class="{cls}" xml:space="preserve" x="{x:.1f}" y="{y:.1f}"{attr}{extra}{data}>{esc(s)}</text>')

    def natural(self, lines, pad=12, title_cls="l", body_cls="s"):
        """The width a box needs for its longest line."""
        title, body = lines[0], [b for b in lines[1:] if b]
        widest = max([width(title, title_cls)] + [width(b.replace("`", ""), body_cls if not b.startswith("`") else "m") for b in body] + [0])
        return widest + 2 * pad

    def box(self, x, y, lines, kind="cpu", w=None, h=None, pad=12, gap=6, title_cls="l", body_cls="s", r=RADIUS):
        """A rounded box whose width fits its longest line unless `w` is given. Returns (x,y,w,h)."""
        fill, stroke = KIND[kind]
        title, body = lines[0], lines[1:]
        body = [b for b in body if b]
        w = w or self.natural(lines, pad, title_cls, body_cls)
        line_h = 20
        h = h or pad + SIZE["l"] + (gap + line_h * len(body) if body else 0) + pad - 2
        ident = self.box_count
        self.box_count += 1
        self.parts.append(f'<rect data-box="{ident}" x="{x:.1f}" y="{y:.1f}" width="{w:.1f}" height="{h:.1f}" rx="{r}" fill="{fill}" stroke="{stroke}" stroke-width="0"/>')
        ty = y + pad + SIZE["l"] - 3
        dark = PAL["yellow_light"] if kind == "sel" else PAL["white"]
        self.text(x + pad, ty, title, title_cls, fill=PAL["black"], box=ident)
        for b in body:
            ty += line_h if b is not body[0] else gap + line_h - 2
            cls = "m" if b.startswith("`") else body_cls
            self.text(x + pad, ty, b.replace("`", ""), cls, fill=PAL["black"] if cls == "m" else PAL["text2"], box=ident)
        return x, y, w, h

    def arrow(self, x1, y1, x2, y2, label=None, cls="ar", above=True):
        self.parts.append(f'<path class="{cls}" d="M{x1:.1f},{y1:.1f} L{x2:.1f},{y2:.1f}"/>')
        if label:
            mx, my = (x1 + x2) / 2, (y1 + y2) / 2
            if abs(y2 - y1) < 1:
                self.text(mx, my - 7 if above else my + 17, label, "s", anchor="middle")
            else:
                self.text(mx + 8, my + 4, label, "s")

    def raw(self, svg):
        swap = {'"#111111"': '"#f4f4f6"', "fill:#fff;": f"fill:{INK};", "fill:#ffffff": f"fill:{INK}", '"#ffffff"': f'"{INK}"'}
        for old, new in swap.items():
            svg = svg.replace(old, new)
        for old, new in ON_BLACK.items():
            if old not in (PAL["black"], "#111", PAL["white"]):
                svg = svg.replace(f'"{old}"', f'"{new}"').replace(f"fill:{old}", f"fill:{new}").replace(f"stroke:{old}", f"stroke:{new}")
        self.parts.append(svg)

    def write(self, name):
        head = (f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {self.w} {self.h}" role="img" aria-labelledby="t d">'
                f'<title id="t">{esc(self.title)}</title><desc id="d">{esc(self.desc)}</desc>{MARKER}<style>{STYLE}</style>'
                f'<rect width="100%" height="100%" rx="{RADIUS}" fill="{PAL["black"]}"/>')
        (HERE / name).write_text(head + "".join(self.parts) + "</svg>\n")


def row(c, y, specs, x0=28, gap=44, labels=None):
    """Lay out boxes left to right with arrows between them; a labelled gap fits its label."""
    rects = []
    x = x0
    for index, (lines, kind) in enumerate(specs):
        rect = c.box(x, y, lines, kind)
        rects.append(rect)
        label = labels[index] if labels and index < len(labels) else None
        x = rect[0] + rect[2] + max(gap, width(label, "s") + 20 if label else 0)
    for index in range(len(rects) - 1):
        a, b = rects[index], rects[index + 1]
        ya = a[1] + a[3] / 2
        label = labels[index] if labels else None
        c.arrow(a[0] + a[2], ya, b[0], ya, label)
    return rects


def spaces():
    c = Canvas("Coordinate spaces from a source point to a framebuffer pixel",
               "A point travels from f64 source coordinates through placement and rebasing to f32 object-relative coordinates, then through view and projection to clip space, the divide by w and the viewport scale. f64 becomes f32 only after the camera anchor is subtracted.",
               1000, 430)
    c.text(28, 40, "One point, six spaces", "h")
    top = row(c, 66, [(["Source · f64", "CAD coordinates", "large offsets kept exact"], "cpu"),
                      (["World · f64", "`Mat4 model × point`", "`math::xform_point_f64`"], "cpu"),
                      (["Object-relative · f32", "`Instance.model` (no translation)", "`translations[row]` (16 B)"], "gpu")],
              labels=["placement", "− anchor"])
    a, b, g = top
    xd = (b[0] + b[2] + g[0]) / 2
    c.raw(f'<path class="dash" d="M{xd:.1f},{g[1] + g[3] / 2 + 12:.1f} L{xd:.1f},{g[1] + g[3] + 14:.1f}"/>')
    c.text(xd, g[1] + g[3] + 30, "f64 → f32 here, after rebasing", "s", anchor="middle", fill=PAL["orange"])
    gx = g[0] + g[2] / 2
    c.arrow(gx, g[1] + g[3] + 40, gx, g[1] + g[3] + 70)
    c.text(gx - 10, g[1] + g[3] + 60, "mvp = projection · view · unit", "s", anchor="end")
    clip = c.box(g[0], g[1] + g[3] + 72, ["Clip · (x, y, z, w)", "`@builtin(position)`"], "gpu", w=g[2])
    ndc = c.box(b[0], clip[1], ["NDC · [−1, 1]²", "depth reversed: near = 1"], "gpu", w=b[2])
    fb = c.box(a[0], clip[1], ["Framebuffer px", "CSS px × devicePixelRatio"], "gpu", w=a[2])
    yc = clip[1] + clip[3] / 2
    c.arrow(clip[0], yc, ndc[0] + ndc[2], yc, "÷ w")
    c.arrow(ndc[0], yc, fb[0] + fb[2], yc, "viewport")
    ny = clip[1] + clip[3] + 24
    c.box(28, ny, ["The camera stays in f64",
                   "Camera { target, distance, orientation } → view_proj_anchored(aspect, anchor)",
                   "one uniform upload per frame; panning rewrites 16 B per object, not the mesh"], "note", w=g[0] + g[2] - 28)
    side = c.box(g[0] + g[2] + 30, 66, ["Who converts",
                                        "`walk/*    world → object f32`",
                                        "`objects.rs  anchor, translations`",
                                        "`camera.rs   view + projection`",
                                        "`frame.rs    mvp uniform`",
                                        "`*.wgsl     mvp * (model·p + t)`",
                                        "`GPU        ÷ w, viewport`",
                                        "Pointer events arrive in CSS px;",
                                        "picking and text convert once."], "note")
    c.w = max(c.w, int(side[0] + side[2] + 28))
    c.write("spaces.svg")


def gpu_data():
    c = Canvas("How a Rust record reaches a WGSL shader",
               "A repr(C) Pod struct is cast to bytes by bytemuck and written into a wgpu buffer. A bind group layout describes the shape, a bind group attaches the buffer at a group and binding number, and the shader declares the same group and binding with a matching struct, and the draw picks one row of it by instance index.",
               1000, 440)
    c.text(28, 40, "Rust struct → buffer → bind group → WGSL", "h")
    r = row(c, 66, [(["Rust · instance.rs", "`#[repr(C)] #[derive(Pod)]`", "`struct Instance {`", "`  model: [f32; 16], // 0`", "`  color: [f32; 4],  // 64`", "`  flags: u32, …     // 80`", "`}                   // 96 B`"], "cpu"),
                    (["wgpu::Buffer", "STORAGE | COPY_DST", "rows × 96 B", "`queue.write_buffer`", "rewrites one row's flags"], "gpu"),
                    (["BindGroup", "layout: Layouts::instance", "`group 2, binding 0`", "`set_bind_group(2, …)`", "rebuilt when the buffer grows"], "gpu"),
                    (["WGSL", "`@group(2) @binding(0)`", "`var<storage, read>`", "`instances:`", "`array<Instance>`"], "gpu")],
            labels=["cast_slice", "entries[]", ""])
    bottom = max(x[1] + x[3] for x in r)
    first, last = r[0], r[-1]
    c.raw(f'<path d="M{first[0] + first[2] / 2:.1f},{bottom + 4:.1f} C{first[0] + first[2] / 2:.1f},{bottom + 44:.1f} {last[0] + last[2] / 2:.1f},{bottom + 44:.1f} {last[0] + last[2] / 2:.1f},{bottom + 4:.1f}" fill="none" stroke="{PAL["yellow"]}" stroke-width="2"/>')
    c.text((first[0] + last[0] + last[2]) / 2, bottom + 62, "same field order, sizes and 16-byte alignment · checked by the layout test", "s", anchor="middle", fill="#8a6a10")
    y2 = bottom + 80
    b1 = c.box(28, y2, ["One row per object",
                        "The draw picks the row; the shader reads it.",
                        "`draw(0..3, row..row + 1)`",
                        "`let row = instance_index;`",
                        "`p = mvp * instances[row].model * pos;`"], "note")
    c.box(b1[0] + b1[2] + 24, y2, ["Group scheme for every draw",
                                   "`0  mvp uniform`",
                                   "`1  line / pen uniform`",
                                   "`2  instances + translations (+ depth for ink)`",
                                   "`3  the lane's own rows`"], "note")
    c.w = max(c.w, int(max(x[0] + x[2] for x in r) + 28))
    c.h = int(y2 + b1[3] + 28)
    c.write("gpu-data.svg")


def ink_visibility():
    c = Canvas("Reversed depth and the ink visibility test",
               "Left: reversed depth maps near to one and far to zero; the depth target clears to zero and opaque faces pass when Greater. Right: a thick stroke covers samples beside its axis; comparing against the surface depth at the offset sample hides a line lying on the surface, so the ink shader transfers the surface depth to the axis through the stored gradient first.",
               1090, 470)
    navy, pink, green, orange = PAL["navy"], PAL["pink"], PAL["green"], PAL["orange"]
    c.text(28, 40, "Reversed depth", "h")
    c.raw(f'<line x1="60" y1="330" x2="60" y2="80" stroke="#111111" stroke-width="1.6" marker-end="url(#a)"/>')
    c.text(70, 92, "depth value", "s")
    for yy, label in ((120, "1.0 · near plane"), (300, "0.0 · far · clear value")):
        c.raw(f'<line x1="44" y1="{yy}" x2="330" y2="{yy}" stroke="{PAL["grey"]}" stroke-width="1"/>')
        c.text(338, yy + 4, label, "m")
    c.raw(f'<rect x="120" y="180" width="160" height="12" fill="{PAL["blue_band"]}" stroke="{navy}" stroke-width="1.5"/>')
    c.text(120, 208, "an opaque face writes its depth", "s")
    c.raw(f'<rect x="180" y="150" width="60" height="12" fill="{PAL["yellow_light"]}" stroke="{PAL["yellow"]}" stroke-width="1.5"/>')
    c.text(248, 160, "nearer fragment: Greater → kept", "s")
    c.raw(f'<rect x="120" y="230" width="90" height="12" fill="{PAL["zero_band"]}" stroke="{PAL["zero"]}" stroke-width="1.5"/>')
    c.text(218, 240, "farther fragment: not Greater → discarded", "s")
    c.box(28, 350, ["The contract, as the code spells it",
                    "`projection: Xform::perspective(fovy, aspect, far, near)`",
                    "`clear: LoadOp::Clear(0.0)     format: Depth32Float`",
                    "`compare: CompareFunction::Greater   (pipelines/mod.rs)`",
                    "More float precision lands where the geometry is: near the eye."], "note")
    x0 = 540
    c.text(x0, 40, "A thick line is not its axis", "h")
    c.raw(f'<polygon points="{x0 + 20},130 {x0 + 500},80 {x0 + 500},310 {x0 + 20},330" fill="{PAL["blue_band"]}" stroke="{navy}" stroke-width="1.5"/>')
    c.text(x0 + 40, 160, "surface: depth changes across x (gradient ∂d/∂x)", "s", fill=PAL["black"], keep=True)
    c.raw(f'<rect x="{x0 + 40}" y="234" width="460" height="32" fill="{PAL["pink_band"]}" stroke="{pink}" stroke-width="1.5"/>')
    c.raw(f'<line x1="{x0 + 40}" y1="250" x2="{x0 + 500}" y2="250" stroke="{pink}" stroke-width="2"/>')
    c.text(x0 + 40, 290, "stroke footprint covers samples beside the axis", "s", fill=PAL["black"], keep=True)
    c.raw(f'<circle cx="{x0 + 250}" cy="261" r="4" fill="{orange}"/>')
    c.text(x0 + 40, 306, "sample here: the surface is nearer → the line is wrongly hidden", "s", fill=PAL["navy"], keep=True)
    c.raw(f'<line x1="{x0 + 250}" y1="261" x2="{x0 + 250}" y2="250" stroke="{green}" stroke-width="2"/>')
    c.raw(f'<circle cx="{x0 + 250}" cy="250" r="4" fill="{green}"/>')
    c.text(x0 + 40, 222, "transfer the surface depth to the axis with the gradient, then compare", "s", fill="#2d7a14", keep=True)
    c.box(x0 - 20, 350, ["The test, as the shader spells it",
                         "`physical pass writes  depth + (∂d/∂x, ∂d/∂y)`",
                         "`d_axis = d_sample + g · (axis − sample)       ink_visibility.wgsl`",
                         "`visible ⇔ line depth ≥ d_axis − tolerance`",
                         "Lesson 18 widens the metadata to carry a triangle address, and adds the finite test."], "note")
    c.write("ink-visibility.svg")


def picking():
    c = Canvas("From a pointer position to a selected source object",
               "A pointer release in CSS pixels becomes a physical pixel window around the cursor. The ID pass renders integer IDs and depth into that scissored window, the window is copied and read asynchronously, the nearest eligible ID is mapped through Scene to a source identity, and State sets the selected flag. Answers from an older generation are dropped.",
               1000, 438)
    c.text(28, 40, "Picking: integer IDs, a small window, one generation", "h")
    upper = [(["Input · input.rs", "pointer up, no drag", "CSS px + modifiers"], "cpu"),
             (["State::request_selection", "PickMode: Object · Edge ·", "Component · Controls", "records generation n"], "cpu"),
             (["id_pass · render.rs", "same lists, opaque, 1×", "scissor = window(cursor)"], "gpu"),
             (["ID targets", "`Rg32Uint  (row, sub)`", "`Depth32Float + metadata`"], "gpu")]
    lower = [(["SelectionMode", "FLAG_SELECTED uploaded", "frame requested", "yellow strokes, black", "silhouette, name plate"], "sel"),
             (["Scene · scene.rs", "`resolve(pick, gpu)`", "`edge_at(pick)`", "`point_at(row, local)`", "row → GUID, edge, point, sheet"], "cpu"),
             (["copy_window → map_async", "a bounded copy, never the frame", "nearest eligible ID wins;", "edges beat faces in Component"], "gpu"),
             (["window · pick.rs", "radius in CSS px × DPR"], "gpu")]
    widths = [max(c.natural(u[0]), c.natural(l[0])) for u, l in zip(upper, lower)]
    gap = 44
    xs = [28]
    for w in widths[:-1]:
        xs.append(xs[-1] + w + gap)
    top = [c.box(x, 66, lines, kind, w=w) for x, w, (lines, kind) in zip(xs, widths, upper)]
    for a, b in zip(top, top[1:]):
        ya = a[1] + a[3] / 2
        c.arrow(a[0] + a[2], ya, b[0], ya)
    y2 = max(x[1] + x[3] for x in top) + 40
    bottom = [c.box(x, y2, lines, kind, w=w, h=125 if kind == "gpu" and lines[0].startswith("window") else None)
              for x, w, (lines, kind) in zip(xs, widths, lower)]
    win = bottom[3]
    gx, gy = win[0] + 14, win[1] + 56
    for i in range(6):
        c.raw(f'<line x1="{gx + i * 20}" y1="{gy}" x2="{gx + i * 20}" y2="{gy + 60}" stroke="{PAL["pink"]}" stroke-width="0.8"/>')
    for j in range(4):
        c.raw(f'<line x1="{gx}" y1="{gy + j * 20}" x2="{gx + 100}" y2="{gy + j * 20}" stroke="{PAL["pink"]}" stroke-width="0.8"/>')
    c.raw(f'<rect x="{gx + 40}" y="{gy + 20}" width="20" height="20" fill="{PAL["yellow"]}"/><circle cx="{gx + 50}" cy="{gy + 30}" r="3" fill="#111111"/>')
    t4 = top[3]
    c.arrow(t4[0] + t4[2] / 2, t4[1] + t4[3], t4[0] + t4[2] / 2, y2)
    ym = y2 + 28
    for a, b in zip(bottom[1:], bottom):
        c.arrow(a[0], ym, b[0] + b[2], ym)
    y3 = max(x[1] + x[3] for x in bottom) + 24
    note = c.box(28, y3, ["Stale answers are dropped",
                          "A camera move, hide/show, scene replacement or mode change bumps the generation; a readback",
                          "that completes for an older generation is ignored, and a drag never selects on release."], "note")
    c.w = max(c.w, int(t4[0] + t4[2] + 28), int(note[0] + note[2] + 28))
    c.h = int(y3 + note[3] + 20)
    c.write("picking.svg")


def text_pipeline():
    c = Canvas("Text from a string to pixels",
               "A string with a font and CSS size is shaped once into glyph runs. Placement decides where the line belongs. The physical raster size is chosen from the device scale, glyph coverage is rasterized into an atlas, and a plate pass and a glyph pass composite the label. Camera motion changes placement only, never the shaped runs.",
               1000, 400)
    c.text(28, 40, "Shape once, place per frame, raster per scale", "h")
    r = row(c, 66, [(["TextLabel · engine/text.rs", "text, font, CSS size,", "color, placement", "bundled Noto font bytes"], "cpu"),
                    (["TextDocument · runs", "`glyph id, advance, offset`", "`cluster → source char`", "cached until text or font change"], "cpu"),
                    (["TextPlacement", "`Screen · Anchor · Nameplate`", "`WorldPlane · WorldBillboard`", "camera changes only this"], "cpu"),
                    (["TextLane::prepare", "physical raster size once", "coverage atlas, bounded cache", "release on scene drop"], "gpu")],
            labels=["shape", "place", "DPR"])
    y2 = max(x[1] + x[3] for x in r) + 36
    last = r[-1]
    c.arrow(last[0] + last[2] / 2, last[1] + last[3], last[0] + last[2] / 2, y2)
    passes = c.box(r[2][0], y2, ["Two passes per label",
                                 "plate: rounded black box with padding (yellow while a source text is selected)",
                                 "glyphs: white letters from atlas coverage, straight alpha, no gray baked in"], "gpu")
    c.raw(f'<rect x="28" y="{y2}" width="250" height="44" rx="22" fill="#111111"/>')
    c.text(52, y2 + 29, "Ø 12 mm · Beam 04", "l", fill="#ffffff")
    c.text(28, y2 + 70, "14 CSS px on any DPR: same size, more pixels", "s")
    y3 = max(passes[1] + passes[3], y2 + 80) + 20
    c.box(28, y3, ["Identity",
                   "Authored text is a source object (app/scene_text.rs): it has a row, is picked by its rounded plate, and can be",
                   "selected, hidden and shown. A derived selected-object name has no row and cannot steal its parent's click."], "note")
    c.w = max(c.w, int(max(passes[0] + passes[2], last[0] + last[2]) + 28))
    c.h = int(y3 + 96)
    c.write("text-pipeline.svg")


def vertex_layout():
    c = Canvas("Rust memory layout versus WGSL alignment",
               "A repr(C) Rust struct and its WGSL mirror must agree on offsets. Scalars are 4 bytes; vec3 is aligned to 16 bytes in WGSL, so a Rust struct that packs three floats without padding shifts every following field. The viewer uses vec4 or explicit padding and asserts size and offsets at compile time.",
               1000, 300)
    c.text(28, 40, "96 bytes, the same on both sides", "h")
    c.text(28, 76, "Instance (instance.rs ↔ scene.wgsl)", "l")

    def cells(y, items, kinds):
        x = 28
        for (label, w), kind in zip(items, kinds):
            fill, stroke = KIND[kind]
            w = max(w, width(label, "m") + 16)
            c.parts.append(f'<rect data-box="c{y}{x:.0f}" x="{x:.1f}" y="{y}" width="{w:.1f}" height="32" fill="{fill}" stroke="{stroke}" stroke-width="0"/>')
            c.text(x + 8, y + 21, label, "m", fill=PAL["black"], box=f"c{y}{x:.0f}")
            x += w
        return x

    end = cells(88, [("model: [f32;16] · mat4x4<f32> · 64 B · offset 0", 420), ("color: [f32;4] · 64", 150), ("flags 80", 70), ("_pad0 84", 96), ("spacing 88", 88), ("_pad 92", 70)],
                ["cpu", "cpu", "gpu", "gpu", "gpu", "note"])
    c.text(28, 146, "const _: () = assert!(size_of::<Instance>() == 96);  the layout test parses the WGSL struct and compares every offset.", "s")
    c.text(28, 186, "The trap: vec3 is 16-byte aligned in WGSL", "l")
    x = cells(198, [("Rust [f32;3] · 12 B", 170), ("next: u32 @12", 120)], ["warn", "warn"])
    c.text(x + 14, 219, "Rust packs the next field at byte 12…", "s")
    x = cells(240, [("WGSL vec3<f32> · 12 B", 170), ("pad", 50), ("next: u32 @16", 120)], ["cpu", "note", "cpu"])
    c.text(x + 14, 261, "…WGSL reads it at 16. Use vec4, or add explicit padding.", "s")
    c.w = max(c.w, int(end + 28))
    c.write("vertex-layout.svg")


def ownership():
    c = Canvas("Who owns what in the viewer",
               "Browser input asks State for named actions. State selects, hides and replaces through Scene, which retains the source documents; geometry preparation turns them into typed Upload rows for the GPU owner. State also gives the GPU its frame input and visibility flags, and the GPU answers picks asynchronously with a row that Scene maps back to a source identity.",
               1000, 470)
    c.text(28, 40, "One source of truth, one GPU owner", "h")
    upper = [(["Scene · retained CPU data", "Session documents + placements", "GUIDs, faces, edges, controls"], "cpu"),
             (["Geometry preparation · app/walk", "source geometry → typed Upload rows", "triangles, strokes, points, source IDs"], "cpu"),
             (["Gpu · engine/gpu", "buffers, pipelines, targets", "color, depth, integer IDs"], "gpu")]
    lower = [(["Browser input", "lib.rs, app/input.rs, touch.rs", "pointer, keyboard, loader Msg"], "cpu"),
             (["State · coordination", "camera, selection, redraw demand", "scene text, source queries"], "cpu")]
    widths = [max(c.natural(upper[0][0]), c.natural(lower[0][0])), max(c.natural(upper[1][0]), c.natural(lower[1][0])), c.natural(upper[2][0])]
    gap = 96
    xs = [28, 28 + widths[0] + gap, 28 + widths[0] + gap + widths[1] + gap]
    scene, prep, gpu = [c.box(x, 66, lines, kind, w=w) for x, w, (lines, kind) in zip(xs, widths, upper)]
    yb = 280
    inp, state = [c.box(x, yb, lines, kind, w=w) for x, w, (lines, kind) in zip(xs, widths, lower)]
    c.arrow(scene[0] + scene[2], scene[1] + 40, prep[0], scene[1] + 40, "walk")
    c.arrow(prep[0] + prep[2], prep[1] + 40, gpu[0], prep[1] + 40, "Upload")
    c.arrow(inp[0] + inp[2], state[1] + 40, state[0], state[1] + 40, "actions")
    sx = state[0] + 40
    scx = scene[0] + scene[2] / 2
    ym = (scene[1] + scene[3] + state[1]) / 2
    c.raw(f'<path class="ar" d="M{sx:.1f},{state[1]:.1f} L{sx:.1f},{ym:.1f} L{scx:.1f},{ym:.1f} L{scx:.1f},{scene[1] + scene[3]:.1f}"/>')
    c.text((sx + scx) / 2, ym - 8, "select · hide · replace document", "s", anchor="middle")
    gx1 = gpu[0] + gpu[2] / 3
    y1 = state[1] + 22
    c.raw(f'<path class="ar" d="M{state[0] + state[2]:.1f},{y1:.1f} L{gx1:.1f},{y1:.1f} L{gx1:.1f},{gpu[1] + gpu[3]:.1f}"/>')
    c.text(state[0] + state[2] + 12, y1 - 8, "frame input · visibility flags", "s")
    gx2 = gpu[0] + 2 * gpu[2] / 3
    y2 = state[1] + state[3] - 22
    c.raw(f'<path class="ar" style="stroke-dasharray:6 5" d="M{gx2:.1f},{gpu[1] + gpu[3]:.1f} L{gx2:.1f},{y2:.1f} L{state[0] + state[2]:.1f},{y2:.1f}"/>')
    c.text(state[0] + state[2] + 12, y2 + 18, "async pick: row → Scene → source identity", "s")
    c.text(28, 440, "Camera motion reuses source geometry; selection changes flags, not tessellation.", "s")
    c.w = int(max(gpu[0] + gpu[2] + 28, state[0] + state[2] + 12 + width("async pick: row → Scene → source identity", "s") + 28))
    c.write("ownership.svg")


def frame():
    c = Canvas("Draw order of one frame",
               "Six ordered passes: prepare finite visibility, draw physical surfaces that establish depth, draw source ink and the selected solid edges, composite one black silhouette from the ordinary and selected masks, draw selected standalone curves over coincident mesh ink, then markers, controls and text. Picking repeats the same lists in a separate ID pass.",
               1000, 470)
    c.text(28, 40, "Draw order is part of the visual contract", "h")
    steps = [("Prepare visibility (lesson 18)", "project and bin triangles only when camera or geometry changed", "gpu"),
             ("Physical surfaces", "opaque faces and clouds write depth + primitive metadata", "gpu"),
             ("Source ink and selected solid edges", "ordinary strokes, source-face highlight, yellow solid boundaries", "gpu"),
             ("One black silhouette", "max(ordinary, selected) mask coverage, composited once", "note"),
             ("Selected standalone curves", "yellow polylines above coincident mesh ink; true occlusion still hides", "gpu"),
             ("Markers, controls and text", "authored text has identity; derived names are overlays", "gpu")]
    y = 64
    for number, (title, body, kind) in enumerate(steps, 1):
        c.raw(f'<circle cx="46" cy="{y + 30}" r="16" fill="{PAL["navy"]}"/>')
        c.text(46, y + 35, str(number), "l", anchor="middle", fill="#ffffff")
        rect = c.box(76, y, [title, body], kind, w=880)
        y += rect[3] + 10
    c.text(28, y + 22, "Picking repeats the same lists in a separate single-sample ID pass under the same toggles.", "s")
    c.h = y + 44
    c.write("frame.svg")


def finite_triangle():
    c = Canvas("Finite triangle versus infinite plane",
               "In screen space a finite triangle's depth plane continues beyond its edges. The old test transferred that plane's depth to the stroke axis even where the axis lies outside the triangle, so visible seams were hidden. The finite check asks whether the triangle contains the axis point; only a finite, nearer hit can hide the ink.",
               1000, 520)
    navy, yellow, green = PAL["navy"], PAL["yellow"], PAL["green"]
    ax, ay = 120, 440
    bx, by = 460, 160
    ex = 700
    c.text(28, 40, "A nearby plane is not an infinite occluder", "h")
    c.raw(f'<polygon points="{ax},{ay} {bx},{by} {bx},{ay}" fill="{PAL["blue_band"]}" stroke="{navy}" stroke-width="1.6"/>')
    c.raw(f'<polygon points="{bx},{by} {ex},{ay} {bx},{ay}" fill="none" stroke="{navy}" stroke-width="1.4" stroke-dasharray="7 6"/>')
    band_x, band_w, axis_x = 500, 60, 530
    c.raw(f'<rect x="{band_x}" y="130" width="{band_w}" height="340" fill="{PAL["yellow_light"]}" fill-opacity="0.85" stroke="{yellow}" stroke-width="1.4"/>')
    c.raw(f'<line x1="{axis_x}" y1="130" x2="{axis_x}" y2="470" stroke="#111111" stroke-width="2"/>')
    c.text(axis_x, 120, "stroke footprint · axis", "s", anchor="middle")
    fy = 300
    c.raw(f'<circle cx="470" cy="{fy}" r="5" fill="{navy}"/>')
    c.raw(f'<circle cx="{axis_x}" cy="{fy}" r="5" fill="{green}"/>')
    c.raw(f'<line x1="475" y1="{fy}" x2="{axis_x - 5}" y2="{fy}" stroke="{green}" stroke-width="1.6"/>')
    c.raw(f'<line x1="470" y1="{fy}" x2="330" y2="200" stroke="{navy}" stroke-width="1"/>')
    c.text(60, 178, "fringe sample: inside the finite triangle,", "s")
    c.text(60, 196, "which really is nearer here", "s")
    c.raw(f'<line x1="{axis_x + 5}" y1="{fy}" x2="{band_x + band_w + 40}" y2="{fy}" stroke="{green}" stroke-width="1"/>')
    c.text(band_x + band_w + 46, fy - 4, "axis sample: outside the triangle;", "s", fill=green)
    c.text(band_x + band_w + 46, fy + 14, "only the extended plane is nearer", "s", fill=green)
    c.text(28, 480, "solid = finite triangle · dashed = the same plane, extended", "s")
    old = c.box(860, 130, ["Old rejection",
                           "transfer the plane depth to the axis",
                           "the plane continues past the edge",
                           "visible ink is wrongly hidden"], "cpu")
    fin = c.box(860, old[1] + old[3] + 24, ["Finite visibility check",
                                            "does this triangle contain the axis?",
                                            "no → it cannot occlude that point",
                                            "then test the other triangles in the tile",
                                            "only a finite, nearer hit hides the ink"], "gpu", w=old[2])
    c.w = int(max(old[0] + old[2], fin[0] + fin[2]) + 28)
    c.write("finite-triangle.svg")


def first_frame():
    c = Canvas("One WebGPU frame, CPU side and GPU side",
               "Long-lived objects are created once: instance, surface, adapter, device and queue, then a pipeline and a bind group. Every frame the CPU records commands into an encoder inside a render pass and submits them; the GPU executes them against the surface texture, which is then presented.",
               1000, 470)
    c.text(28, 40, "Created once, recorded every frame, executed by the GPU", "h")
    once = row(c, 66, [(["Instance", "`Backends::BROWSER_WEBGPU`"], "cpu"),
                       (["Surface", "`create_surface(canvas)`"], "cpu"),
                       (["Adapter", "`compatible_surface`"], "cpu"),
                       (["Device + Queue", "`request_device`"], "cpu")],
               labels=["", "", ""])
    last = once[-1]
    pipe = c.box(last[0] + last[2] + 44, 66, ["Pipeline · BindGroup", "immutable, reused every frame"], "gpu")
    c.arrow(last[0] + last[2], 66 + 30, pipe[0], 66 + 30)
    c.text(28, 150, "once", "l", fill=PAL["text2"])
    y = 190
    c.text(28, y, "every frame", "l", fill=PAL["text2"])
    cpu = c.box(28, y + 14, ["CPU · render_frame()",
                             "`resize once → configure surface`",
                             "`get_current_texture()`",
                             "`encoder.begin_render_pass { clear }`",
                             "`  set_pipeline · set_bind_group · draw(0..3)`",
                             "`queue.submit([encoder.finish()])`"], "cpu")
    gpu = c.box(cpu[0] + cpu[2] + 70, y + 14, ["GPU · executes the submitted list",
                                                "clear the surface texture",
                                                "run vs_main three times",
                                                "rasterize, run fs_main per pixel",
                                                "write the color attachment"], "gpu", h=cpu[3])
    c.arrow(cpu[0] + cpu[2], y + 14 + 40, gpu[0], y + 14 + 40, "submit")
    c.arrow(gpu[0] + gpu[2] / 2, gpu[1] + gpu[3], gpu[0] + gpu[2] / 2, gpu[1] + gpu[3] + 40)
    c.text(gpu[0] + gpu[2] / 2 + 10, gpu[1] + gpu[3] + 30, "output.present()", "s")
    c.box(28, gpu[1] + gpu[3] + 52, ["The pass borrows the encoder",
                                      "the braces around `begin_render_pass` end the borrow before `encoder.finish()`;",
                                      "a pipeline is expensive to create and never changes, so it lives on the struct"], "note", w=gpu[0] + gpu[2] - 28)
    c.w = int(max(pipe[0] + pipe[2], gpu[0] + gpu[2]) + 28)
    c.h = int(gpu[1] + gpu[3] + 52 + 96 + 20)
    c.write("first-frame.svg")


def cad_contract():
    c = Canvas("A CAD face from source to GPU rows",
               "A BRep face is a surface with trim loops and edge identities in f64. The kernel meshes each face into positions with u,v, per-face normals and boundary provenance tags. The viewer converts once to object-relative f32 and produces arena rows for triangles, pipe rows for boundary strokes carrying source edge ids, and glyph rows for vertices.",
               1000, 470)
    c.text(28, 40, "One face, three representations", "h")
    r = row(c, 66, [(["Source · BRep face (f64)", "surface + trim loops", "oriented edge uses", "original edge ids"], "cpu"),
                    (["Kernel · Mesh per face", "`positions · u,v · normals`", "`boundary/{loop}/{sample}`", "`boundary_interval/{loop}/{seg}`", "one normal set per face"], "cpu"),
                    (["Viewer · rows for the lanes", "`ArenaRows  verts · idx`", "`SegRows    pipes + edge ids`", "`GlyphRows  vertex spheres`", "f32, object-relative"], "gpu")],
            labels=["face_meshes_q", "walk/brep · mesh"])
    y = max(x[1] + x[3] for x in r) + 30
    c.box(28, y, ["The same conversion for triangles and boundary endpoints",
                  "a boundary stroke reuses the face mesh's own nodes, converted by the same f64 → f32 step;",
                  "converting the two independently reintroduces the gap the shared samples removed"], "note", w=r[-1][0] + r[-1][2] - 28)
    c.box(28, y + 96, ["Identity survives every arrow",
                       "parent row → source face index → original edge id: picking maps a triangle or pipe back to CAD,",
                       "never to a tessellation artefact"], "note", w=r[-1][0] + r[-1][2] - 28)
    c.w = int(r[-1][0] + r[-1][2] + 28)
    c.h = int(y + 96 + 96 + 20)
    c.write("cad-contract.svg")


def shared_boundary():
    c = Canvas("Independent chords versus one shared boundary chain",
               "Left: two faces and the ink sample one common edge independently, so three polylines approximate one "
               "curve and cross, separate or hide each other as the camera moves. Right: one canonical chain is chosen, "
               "both face meshes are constrained to those exact points and the ink reuses them, so the seam is one curve "
               "everywhere.",
               1000, 470)
    navy, pink, green, grey = PAL["navy"], PAL["pink"], PAL["green"], PAL["grey"]
    import math
    def arc(x0, y0, w, h, n, wobble=0.0, phase=0.0):
        pts = []
        for i in range(n + 1):
            t = i / n
            a = math.pi * (0.15 + 0.7 * t)
            x = x0 + w * t
            y = y0 - h * math.sin(a) + wobble * math.sin(6 * t * math.pi + phase)
            pts.append((x, y))
        return pts
    def poly(pts, stroke, width=2, dash=None, dots=None):
        d = " ".join(f"{x:.1f},{y:.1f}" for x, y in pts)
        extra = f' stroke-dasharray="{dash}"' if dash else ""
        c.raw(f'<polyline points="{d}" fill="none" stroke="{stroke}" stroke-width="{width}"{extra}/>')
        if dots:
            for x, y in pts:
                c.raw(f'<circle cx="{x:.1f}" cy="{y:.1f}" r="3.2" fill="{dots}"/>')
    # left panel: before
    c.text(28, 40, "Before: three approximations of one edge", "h")
    c.raw(f'<rect x="40" y="70" width="420" height="250" rx="10" fill="{PAL["white"]}" stroke="{grey}" stroke-width="1"/>')
    c.raw(f'<polygon points="60,300 60,120 250,140 250,290" fill="{PAL["blue_band"]}" fill-opacity="0.6" stroke="none"/>')
    c.raw(f'<polygon points="250,140 440,110 440,300 250,290" fill="{PAL["pink_band"]}" fill-opacity="0.6" stroke="none"/>')
    poly(arc(120, 300, 250, 120, 5, 0, 0), navy, 2, dots=navy)
    poly(arc(120, 300, 250, 120, 7, 6, 1.0), pink, 2, dots=pink)
    poly(arc(120, 300, 250, 120, 12, 4, 2.0), "#111111", 1.6, dash="6 4")
    c.text(60, 335, "face A chords (5)", "s", fill=navy)
    c.text(200, 335, "face B chords (7)", "s", fill=pink)
    c.text(340, 335, "ink sampled apart", "s")
    c.text(60, 358, "they cross and separate; the seam z-fights and breaks as the camera moves", "s")
    # right panel: after
    x0 = 520
    c.text(x0, 40, "After: one canonical chain", "h")
    c.raw(f'<rect x="{x0 + 12}" y="70" width="420" height="250" rx="10" fill="{PAL["white"]}" stroke="{grey}" stroke-width="1"/>')
    c.raw(f'<polygon points="{x0 + 32},300 {x0 + 32},120 {x0 + 222},140 {x0 + 222},290" fill="{PAL["blue_band"]}" fill-opacity="0.6" stroke="none"/>')
    c.raw(f'<polygon points="{x0 + 222},140 {x0 + 412},110 {x0 + 412},300 {x0 + 222},290" fill="{PAL["pink_band"]}" fill-opacity="0.6" stroke="none"/>')
    chain = arc(x0 + 92, 300, 250, 120, 8, 0, 0)
    poly(chain, "#111111", 2.4, dots=green)
    c.text(x0 + 32, 335, "one chain, refined once; both meshes constrained to it", "s")
    c.text(x0 + 32, 358, "ink drawn from those same mesh nodes: one curve, no seam", "s")
    c.box(28, 390, ["Where it lives",
                    "`brep.rs phase 2      chooses and refines the chain, once per edge`",
                    "`TrimLoops            carries the given XYZ into mesh_loops`",
                    "`brep_edges.rs        reads the chain back as pipes with the original edge id`"], "note", w=944)
    c.w = 1000
    c.h = 390 + 104 + 20
    c.write("shared-boundary.svg")


def trims_seams():
    c = Canvas("Trim loops select part of a surface; a periodic seam is one curve used twice",
               "Left: in the surface's u,v rectangle the outer loop and an inner loop select the face; the constrained triangulation keeps loop edges as triangle edges and leaves the hole empty. Right: a cylinder unrolled in u,v has its seam curve at u=0 and again at u=1: two face uses with different parameters map to the same XYZ points.",
               1100, 470)
    navy, pink, green, grey = PAL["navy"], PAL["pink"], PAL["green"], PAL["grey"]
    c.text(28, 40, "Trims: the u,v rectangle is not the face", "h")
    # uv rectangle
    x0, y0, w, h = 60, 90, 360, 230
    c.raw(f'<rect x="{x0}" y="{y0}" width="{w}" height="{h}" fill="{PAL["white"]}" stroke="{grey}" stroke-width="1"/>')
    # outer loop (trimmed region) as a polygon, inner loop as a hole
    outer = [(x0+30,y0+200),(x0+40,y0+40),(x0+200,y0+20),(x0+330,y0+60),(x0+320,y0+210),(x0+150,y0+215)]
    inner = [(x0+150,y0+90),(x0+230,y0+80),(x0+245,y0+140),(x0+170,y0+155)]
    pts=lambda ps: " ".join(f"{x},{y}" for x,y in ps)
    c.raw(f'<path d="M{pts(outer).replace(" "," L")} Z M{pts(inner).replace(" "," L")} Z" fill="{PAL["blue_band"]}" fill-opacity="0.7" fill-rule="evenodd" stroke="none"/>')
    # a few interior triangles hinting constrained triangulation
    tri = [((x0+40,y0+40),(x0+150,y0+90),(x0+30,y0+200)),((x0+200,y0+20),(x0+150,y0+90),(x0+40,y0+40)),((x0+200,y0+20),(x0+230,y0+80),(x0+150,y0+90)),((x0+330,y0+60),(x0+245,y0+140),(x0+230,y0+80)),((x0+320,y0+210),(x0+245,y0+140),(x0+330,y0+60)),((x0+150,y0+215),(x0+170,y0+155),(x0+320,y0+210)),((x0+30,y0+200),(x0+150,y0+90),(x0+170,y0+155)),((x0+30,y0+200),(x0+170,y0+155),(x0+150,y0+215))]
    for a,b,d in tri:
        c.raw(f'<polygon points="{pts([a,b,d])}" fill="none" stroke="{grey}" stroke-width="0.8"/>')
    c.raw(f'<polygon points="{pts(outer)}" fill="none" stroke="{navy}" stroke-width="2.2"/>')
    c.raw(f'<polygon points="{pts(inner)}" fill="none" stroke="{pink}" stroke-width="2.2"/>')
    c.text(x0, y0 + h + 24, "outer loop", "s", fill=navy)
    c.text(x0 + 110, y0 + h + 24, "inner loop = hole, left empty", "s", fill=pink)
    c.text(x0, y0 + h + 46, "loop edges are triangle edges; grey diagonals are", "s")
    c.text(x0, y0 + h + 64, "interior and never drawn as CAD edges", "s")
    c.text(x0 + w - 12, y0 - 8, "u →", "s", anchor="end")
    c.text(x0 - 8, y0 + 14, "v", "s", anchor="end")
    # right: seam
    X = 560
    c.text(X, 40, "Seam: one XYZ curve, two parameter uses", "h")
    c.raw(f'<rect x="{X + 20}" y="{y0}" width="200" height="{h}" fill="{PAL["pink_band"]}" fill-opacity="0.6" stroke="{grey}" stroke-width="1"/>')
    c.raw(f'<line x1="{X + 20}" y1="{y0}" x2="{X + 20}" y2="{y0 + h}" stroke="{green}" stroke-width="3"/>')
    c.raw(f'<line x1="{X + 220}" y1="{y0}" x2="{X + 220}" y2="{y0 + h}" stroke="{green}" stroke-width="3"/>')
    c.text(X + 20, y0 - 8, "u = 0", "s", anchor="middle", fill=green)
    c.text(X + 220, y0 - 8, "u = 1", "s", anchor="middle", fill=green)
    c.text(X + 120, y0 + h / 2, "unrolled cylinder face", "s", anchor="middle", fill=PAL["black"], keep=True)
    # cylinder sketch
    cx, cy = X + 340, y0 + h / 2
    c.raw(f'<ellipse cx="{cx}" cy="{y0 + 30}" rx="60" ry="18" fill="none" stroke="{grey}" stroke-width="1.2"/>')
    c.raw(f'<line x1="{cx - 60}" y1="{y0 + 30}" x2="{cx - 60}" y2="{y0 + h - 30}" stroke="{grey}" stroke-width="1.2"/>')
    c.raw(f'<line x1="{cx + 60}" y1="{y0 + 30}" x2="{cx + 60}" y2="{y0 + h - 30}" stroke="{grey}" stroke-width="1.2"/>')
    c.raw(f'<path d="M{cx - 60},{y0 + h - 30} A60,18 0 0 0 {cx + 60},{y0 + h - 30}" fill="none" stroke="{grey}" stroke-width="1.2"/>')
    c.raw(f'<line x1="{cx}" y1="{y0 + 48}" x2="{cx}" y2="{y0 + h - 12}" stroke="{green}" stroke-width="3"/>')
    c.text(cx, y0 + h + 24, "the seam in 3D", "s", anchor="middle", fill=green)
    c.text(X, y0 + h + 46, "same XYZ, different (u,v): keep both face uses;", "s")
    c.text(X, y0 + h + 64, "a pole collapses many (u,v) onto one point", "s")
    c.box(28, 402, ["The consumer's rule",
                    "walk_surface keeps the producer's constrained mesh and its loop provenance; map_surface_boundaries names natural",
                    "boundaries from u/v extremes, so a repeated seam use or a zero-length pole edge never becomes an invented CAD edge."], "note", w=1044)
    c.h = 402 + 84 + 20
    c.write("trims-seams.svg")


def normals():
    c = Canvas("Where a shading normal comes from, and how it moves",
               "Left: the normal is the normalized cross product of the surface derivatives; at a pole or apex that "
               "product vanishes and incident triangle normals are averaged instead. Middle: a C0 crease needs two "
               "normals at one position, or the fold smears. Right: under nonuniform scale only the cofactor matrix "
               "keeps the normal perpendicular.",
               1250, 480)
    navy, pink, green, orange, grey = PAL["navy"], PAL["pink"], PAL["green"], PAL["orange"], PAL["grey"]
    def arrow(x1, y1, x2, y2, color, w=2):
        c.raw(f'<line x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}" stroke="{color}" stroke-width="{w}"/>')
        import math
        a = math.atan2(y2 - y1, x2 - x1)
        for sgn in (1, -1):
            c.raw(f'<line x1="{x2}" y1="{y2}" x2="{x2 - 9 * math.cos(a - sgn * 0.45):.1f}" y2="{y2 - 9 * math.sin(a - sgn * 0.45):.1f}" stroke="{color}" stroke-width="{w}"/>')
    # panel 1: analytic vs fallback
    c.text(28, 40, "1 · Analytic normal or fallback", "h")
    c.raw(f'<path d="M50,230 Q180,90 330,220" fill="none" stroke="{navy}" stroke-width="2.5"/>')
    c.raw(f'<circle cx="190" cy="150" r="4" fill="{navy}"/>')
    arrow(190, 150, 250, 138, grey, 1.6); c.text(256, 142, "∂u", "s")
    arrow(190, 150, 176, 100, grey, 1.6); c.text(150, 96, "∂v", "s")
    arrow(190, 150, 230, 84, green, 2.4); c.text(236, 82, "n = normalize(∂u × ∂v)", "s", fill=green)
    c.text(50, 250, "smooth interior: derivatives define the normal", "s")
    c.raw(f'<polygon points="90,410 130,330 170,410" fill="{PAL["blue_band"]}" stroke="{navy}" stroke-width="1.5"/>')
    c.raw(f'<polygon points="130,330 170,410 220,390" fill="{PAL["blue_band"]}" stroke="{navy}" stroke-width="1.5"/>')
    c.raw(f'<circle cx="130" cy="330" r="4" fill="{orange}"/>')
    arrow(130, 330, 130, 284, orange, 2.4)
    c.text(232, 352, "pole: ∂u × ∂v = 0", "s", fill=orange)
    c.text(232, 370, "fallback: mean of the", "s", fill=orange)
    c.text(232, 388, "fan's face normals", "s", fill=orange)
    c.text(50, 446, "a sentinel +Z is never accepted as a derivative", "s")
    # panel 2: crease
    X = 440
    c.text(X, 40, "2 · A crease keeps two normals", "h")
    c.raw(f'<path d="M{X + 20},220 L{X + 150},120 L{X + 290},210" fill="none" stroke="{navy}" stroke-width="2.5"/>')
    c.raw(f'<circle cx="{X + 150}" cy="120" r="4.5" fill="{pink}"/>')
    arrow(X + 150, 120, X + 118, 75, green, 2.4); c.text(X + 40, 70, "left side normal", "s", fill=green)
    arrow(X + 150, 120, X + 186, 70, green, 2.4); c.text(X + 192, 66, "right side normal", "s", fill=green)
    c.text(X + 20, 250, "two shading vertices at one XYZ, one per side", "s")
    c.raw(f'<path d="M{X + 20},430 L{X + 150},330 L{X + 290},420" fill="none" stroke="{navy}" stroke-width="2.5"/>')
    c.raw(f'<circle cx="{X + 150}" cy="330" r="4.5" fill="{grey}"/>')
    arrow(X + 150, 330, X + 156, 282, orange, 2.4); c.text(X + 164, 288, "one averaged normal", "s", fill=orange)
    c.text(X + 20, 462, "shared vertex: the fold is lit as if round", "s")
    # panel 3: transform
    Y = 860
    c.text(Y, 40, "3 · Transforming a normal", "h")
    c.raw(f'<polygon points="{Y + 20},200 {Y + 120},140 {Y + 220},200" fill="none" stroke="{navy}" stroke-width="2.5"/>')
    arrow(Y + 120, 140, Y + 120, 84, green, 2.4); c.text(Y + 128, 88, "n", "s", fill=green)
    c.text(Y + 20, 222, "before: n ⟂ surface", "s")
    c.raw(f'<polygon points="{Y + 20},410 {Y + 120},310 {Y + 220},410" fill="none" stroke="{navy}" stroke-width="2.5"/>')
    arrow(Y + 120, 310, Y + 120, 254, orange, 2.4); c.text(Y + 128, 262, "M·n", "s", fill=orange)
    arrow(Y + 120, 310, Y + 158, 270, green, 2.4); c.text(Y + 166, 276, "cofactor(M)·n", "s", fill=green)
    c.text(Y + 20, 444, "after a stretch in y: only the cofactor form stays ⟂", "s")
    c.text(Y + 20, 464, "normalize in the fragment stage: interpolation shrinks it", "s")
    c.h = 480
    c.write("normals.svg")


def shaping():
    c = Canvas("Shaping: characters become positioned glyphs",
               "A string is not a row of glyph bitmaps. Shaping chooses glyphs and pen advances: a kerning pair pulls the second glyph back, a ligature turns two characters into one glyph, a combining accent adds a glyph with no advance, and a space advances the pen without any pixels. Clusters map each glyph back to the source characters.",
               1100, 470)
    navy, pink, green, orange, grey = PAL["navy"], PAL["pink"], PAL["green"], PAL["orange"], PAL["grey"]
    c.text(28, 40, "The pen moves by advances, not by bitmap widths", "h")
    base = 200
    c.raw(f'<line x1="60" y1="{base}" x2="1040" y2="{base}" stroke="{grey}" stroke-width="1"/>')
    c.text(1040, base + 18, "baseline", "s", anchor="end")
    # pen boxes: (x, advance, glyph text, note, color)
    cells = [(60, 92, "A", "advance 92", navy), (152, 84, "V", "kerned: −14", pink), (236, 64, "e", "", navy), (300, 44, " ", "space: advance, no ink", orange),
             (344, 96, "ﬁ", "ligature: 2 chars → 1 glyph", green), (440, 70, "n", "", navy), (510, 66, "é", "e + combining ´ (advance 0)", green), (576, 60, "s", "", navy)]
    for x, adv, glyph, note, color in cells:
        c.raw(f'<rect x="{x}" y="{base - 110}" width="{adv}" height="110" fill="none" stroke="{grey}" stroke-width="1" stroke-dasharray="3 3"/>')
        if glyph.strip():
            c.raw(f'<text x="{x + adv / 2}" y="{base - 8}" text-anchor="middle" style="font-family:serif;font-size:96px;fill:{color}">{glyph}</text>')
        c.raw(f'<line x1="{x}" y1="{base + 4}" x2="{x}" y2="{base + 14}" stroke="{color}" stroke-width="2"/>')
    c.text(60, base + 40, "advance 92", "s", fill=navy)
    c.text(152, base + 40, "V kerned −14", "s", fill=pink)
    c.text(300, base + 40, "space: pen moves, no ink", "s", fill=orange)
    c.text(344, base + 62, "ligature: two chars, one glyph", "s", fill=green)
    c.text(510, base + 84, "combining accent: a glyph with advance 0", "s", fill=green)
    c.text(60, base + 110, "Each dashed box is one pen advance; a glyph's ink may overhang it (V, ﬁ) or be empty (space).", "s")
    r = row(c, 350, [(["TextLabel", "text · font · CSS size"], "cpu"),
                     (["Cosmic Text shaping", "bundled Noto bytes", "kerning · ligatures · fallback"], "cpu"),
                     (["TextRun · glyph runs", "`glyph id · advance · offset`", "`cluster → source char`"], "cpu"),
                     (["TextDocument", "cached until text or font changes"], "gpu")],
            labels=["shape", "", ""])
    c.w = int(max(x[0] + x[2] for x in r) + 28)
    c.h = 470
    c.write("shaping.svg")


def text_placement():
    c = Canvas("Five placements, one shaped line",
               "One shaped line placed five ways: Screen at a fixed CSS position; Anchor at a world point with a screen "
               "offset; Nameplate on a rounded plate; WorldPlane in a fixed plane, so it foreshortens; WorldBillboard "
               "turning to face the camera. Rasterization follows device scale: a 14 CSS px label keeps its size on a "
               "DPR 2 screen and gets twice the pixels.",
               1100, 470)
    navy, pink, green, grey = PAL["navy"], PAL["pink"], PAL["green"], PAL["grey"]
    c.text(28, 40, "Placement decides where; raster size decides how many pixels", "h")
    kinds = [("Screen", "fixed CSS position", "left · top"), ("Anchor", "world point + screen offset", "follows the object"), ("Nameplate", "centered on a world anchor", "rounded plate, overlay depth"),
             ("WorldPlane", "in a fixed plane, world height", "foreshortens with the view"), ("WorldBillboard", "world point, world height", "turns to face the camera")]
    x = 28
    rects = []
    for title, a, b in kinds:
        rect = c.box(x, 66, [title, a, b], "cpu")
        rects.append(rect); x = rect[0] + rect[2] + 16
    # sketches: a plate per kind
    for (title, _, _), rect in zip(kinds, rects):
        cx = rect[0] + rect[2] / 2
        y = 200
        if title == "WorldPlane":
            c.raw(f'<polygon points="{cx - 50},{y + 24} {cx + 40},{y + 4} {cx + 40},{y + 34} {cx - 50},{y + 54}" fill="#111111"/>')
            c.raw(f'<text x="{cx - 5}" y="{y + 33}" text-anchor="middle" style="fill:#fff;font-size:12px;font-family:system-ui,sans-serif" transform="rotate(-12.5 {cx - 5} {y + 29})">Beam 04</text>')
        else:
            c.raw(f'<rect x="{cx - 46}" y="{y + 10}" width="92" height="30" rx="{15 if title == "Nameplate" else 4}" fill="#111111"/>')
            c.raw(f'<text x="{cx}" y="{y + 30}" text-anchor="middle" style="fill:#fff;font-size:13px;font-family:system-ui,sans-serif">Beam 04</text>')
        if title in ("Anchor", "Nameplate", "WorldPlane", "WorldBillboard"):
            c.raw(f'<circle cx="{cx}" cy="{y + 70}" r="4" fill="{green}"/>')
            c.raw(f'<line x1="{cx}" y1="{y + 40}" x2="{cx}" y2="{y + 66}" stroke="{green}" stroke-width="1.4"/>')
            c.text(cx, y + 92, "world anchor", "s", anchor="middle", fill=green)
    # DPR row
    y = 330
    c.text(28, y, "Same label, two device scales", "l")
    c.raw(f'<rect x="28" y="{y + 14}" width="140" height="34" rx="17" fill="#111111"/><text x="98" y="{y + 36}" text-anchor="middle" style="fill:#fff;font-size:14px;font-family:system-ui,sans-serif">Beam 04</text>')
    c.text(28, y + 70, "DPR 1: 14 CSS px = 14 physical px", "s")
    c.raw(f'<rect x="300" y="{y + 14}" width="140" height="34" rx="17" fill="#111111"/><text x="370" y="{y + 36}" text-anchor="middle" style="fill:#fff;font-size:14px;font-family:system-ui,sans-serif">Beam 04</text>')
    c.text(300, y + 70, "DPR 2: 14 CSS px = 28 physical px, rasterized once at that size", "s")
    c.text(28, y + 100, "The shaped runs are identical; TextFrame::scale applies the device scale exactly once, in TextLane::prepare.", "s")
    c.w = int(rects[-1][0] + rects[-1][2] + 28)
    c.h = 470
    c.write("text-placement.svg")


def controls():
    c = Canvas("Display vertices are not controls",
               "The screen shows a curve as many short chords and a surface as a tessellation grid. F10 asks the source "
               "geometry for its real controls: a curve's control points and polygon, a surface's control net, a mesh's "
               "original vertices. A picked marker answers with a ControlId into the source, never the temporary marker "
               "slot.",
               1180, 500)
    navy, pink, green, yellow, grey = PAL["navy"], PAL["pink"], PAL["green"], PAL["yellow"], PAL["grey"]
    c.text(28, 40, "What the screen draws versus what F10 shows", "h")
    P = [(60, 290), (170, 130), (330, 350), (470, 170)]
    def bez(t):
        u = 1 - t
        return (u**3 * P[0][0] + 3 * u * u * t * P[1][0] + 3 * u * t * t * P[2][0] + t**3 * P[3][0],
                u**3 * P[0][1] + 3 * u * u * t * P[1][1] + 3 * u * t * t * P[2][1] + t**3 * P[3][1])
    pts = [bez(i / 40) for i in range(41)]
    c.text(60, 84, "display: 40 chords, 41 vertices", "l")
    c.raw('<polyline points="' + " ".join(f"{x:.1f},{y:.1f}" for x, y in pts) + '" fill="none" stroke="#111111" stroke-width="1.5"/>')
    for x, y in pts:
        c.raw(f'<circle cx="{x:.1f}" cy="{y:.1f}" r="2.6" fill="{grey}"/>')
    dx = 600
    c.text(60 + dx, 84, "F10: 4 control points, one control polygon", "l")
    pts2 = [(x + dx, y) for x, y in pts]
    c.raw('<polyline points="' + " ".join(f"{x:.1f},{y:.1f}" for x, y in pts2) + '" fill="none" stroke="#111111" stroke-width="1.5"/>')
    c.raw('<polyline points="' + " ".join(f"{x + dx},{y}" for x, y in P) + f'" fill="none" stroke="{navy}" stroke-width="1.5" stroke-dasharray="5 4"/>')
    for i, (x, y) in enumerate(P):
        fill = yellow if i == 2 else navy
        c.raw(f'<rect x="{x + dx - 6}" y="{y - 6}" width="12" height="12" fill="{fill}" stroke="#111111" stroke-width="1"/>')
    c.text(1152, P[2][1] - 14, "picked: ControlId::Curve { curve: 0, point: 2 }", "s", anchor="end", fill=pink)
    c.text(P[0][0] + dx - 4, P[0][1] + 24, "point 0", "s", fill=navy)
    c.text(P[1][0] + dx + 12, P[1][1] + 4, "point 1", "s", fill=navy)
    c.text(P[3][0] + dx - 4, P[3][1] - 12, "point 3", "s", fill=navy)
    c.text(28, 392, "What each family reports as its controls", "l")
    x = 28
    for lines, kind in [(["Mesh", "original vertex keys"], "cpu"), (["Curve · Surface", "control points · control net", "links = control polygon"], "cpu"),
                        (["BRep", "topological vertices"], "cpu"), (["Cloud", "source rows by page (HTTP Range)"], "gpu")]:
        rect = c.box(x, 404, lines, kind)
        x = rect[0] + rect[2] + 24
    c.w = max(int(x + 4), 1180)
    c.h = 500
    c.write("controls.svg")

def loading():
    c = Canvas("Staged replacement with generations",
               "A route change starts a new request generation. Every fetched document is validated and decoded, then staged in manifest order; nothing on screen changes until the whole set is ready, when the staged set replaces the scene in one step. Documents that finish after a newer generation started are dropped, so a slow first request can never overwrite a faster second one.",
               1180, 470)
    navy, pink, green, yellow, orange, grey = PAL["navy"], PAL["pink"], PAL["green"], PAL["yellow"], PAL["orange"], PAL["grey"]
    c.text(28, 40, "Two requests in flight: the older generation loses", "h")
    x0, x1 = 150, 1150
    for i, (label, y) in enumerate([("generation 1", 120), ("generation 2", 230), ("screen", 340)]):
        c.text(28, y + 5, label, "l")
        c.raw(f'<line x1="{x0}" y1="{y}" x2="{x1}" y2="{y}" stroke="{grey}" stroke-width="1"/>')
    c.raw(f'<polygon points="{x1},{340} {x1 - 8},{335} {x1 - 8},{345}" fill="{grey}"/>')
    c.text(x1 - 28, 364, "time", "s")
    # generation 1: three files, one slow
    def seg(x, w, y, color, text):
        c.raw(f'<rect x="{x}" y="{y - 14}" width="{w}" height="28" rx="4" fill="{color}"/>')
        c.text(x + w / 2, y + 5, text, "s", anchor="middle", fill="#ffffff")
    seg(160, 120, 120, navy, "manifest a.yaml")
    seg(290, 110, 120, navy, "mesh.pb ✓")
    seg(410, 110, 120, navy, "brep.pb ✓")
    seg(530, 300, 120, grey, "cloud.pb (slow) → dropped")
    c.raw(f'<line x1="600" y1="150" x2="600" y2="210" stroke="{pink}" stroke-width="1.5" stroke-dasharray="4 3"/>')
    c.text(608, 184, "route changes: generation 2 starts", "s", fill=pink)
    seg(600, 120, 230, pink, "manifest b.yaml")
    seg(730, 110, 230, pink, "mesh.pb ✓")
    seg(850, 110, 230, pink, "text items ✓")
    c.raw(f'<rect x="970" y="216" width="130" height="28" rx="4" fill="{green}"/>')
    c.text(1035, 235, "staged → swap", "s", anchor="middle", fill="#ffffff")
    c.raw(f'<line x1="1035" y1="244" x2="1035" y2="326" stroke="{green}" stroke-width="1.5"/>')
    c.raw(f'<polygon points="1035,326 1030,318 1040,318" fill="{green}"/>')
    seg(160, 860, 340, "#d9dbe0", "previous scene stays visible and interactive")
    seg(1035, 64, 340, green, "scene b")
    c.text(160, 392, "A document is accepted only if its generation is still the current one; decoding yields between slices, so the page never freezes while a large file is read.", "s")
    c.w = 1180
    c.h = 430
    c.write("loading.svg")


def metadata_window():
    c = Canvas("One window instead of one request per field",
               "A streamed cloud file is small protobuf fields (count, bounds, the LOD node table) between very large "
               "arrays. The LOD walk needs only the small ones. Checkpoint 14 spent one HTTP Range request per header "
               "and one per array body; MetadataWindow reads 64 KiB at once, or what is left of the message, and "
               "skips each large array by its declared length.",
               1180, 470)
    navy, pink, green, yellow, orange, grey = PAL["navy"], PAL["pink"], PAL["green"], PAL["yellow"], PAL["orange"], PAL["grey"]
    c.text(28, 40, "The file on the server, and what the LOD walk touches", "h")
    fields = [("count", 40, navy), ("bounds", 60, navy), ("coords · 96 MiB", 300, grey), ("colors · 24 MiB", 170, grey), ("nodes", 90, navy), ("ids · 32 MiB", 200, grey), ("levels", 70, navy)]
    x = 180; y = 110; h = 36; starts = []
    for name, w, color in fields:
        starts.append((x, w, color))
        c.raw(f'<rect x="{x}" y="{y}" width="{w}" height="{h}" fill="{color}" stroke="#eef0f2" stroke-width="2"/>')
        if w >= 60:
            c.text(x + w / 2, y + 23, name, "s", anchor="middle", fill="#ffffff")
        x += w
    c.text(180, y + 60, "count", "s", fill=navy)
    c.text(180, y - 10, "tag · length · payload for every field; a large array is skipped by its length, never read", "s")
    # checkpoint 14 row
    y14 = 210
    c.text(28, y14 + 5, "checkpoint 14", "l")
    c.text(180, y14 - 16, "one Range request per header and one per small array body", "s", fill=pink)
    for (x, w, color) in starts:
        if color == navy:
            for k in (0, 1):
                c.raw(f'<rect x="{x + 4 + k * (w - 8) / 2}" y="{y14 - 6}" width="{(w - 8) / 2 - 2}" height="12" rx="2" fill="{pink}"/>')
    c.text(1125, y14 + 5, "8 requests", "s", fill=pink)
    # checkpoint 15 row
    y15 = 290
    c.text(28, y15 + 5, "checkpoint 15", "l")
    c.text(180, y15 - 16, "MetadataWindow: one read of at least 64 KiB, small fields served from the cache", "s", fill=green)
    c.raw(f'<rect x="{starts[0][0]}" y="{y15 - 8}" width="{starts[1][0] + starts[1][1] - starts[0][0] + 70}" height="16" rx="3" fill="{green}"/>')
    c.text(starts[0][0] + 6, y15 + 4, "window", "s", fill="#ffffff")
    c.raw(f'<rect x="{starts[4][0]}" y="{y15 - 8}" width="{starts[4][1]}" height="16" rx="3" fill="{green}"/>')
    c.raw(f'<rect x="{starts[6][0]}" y="{y15 - 8}" width="{starts[6][1]}" height="16" rx="3" fill="{green}"/>')
    c.text(1125, y15 + 5, "3 requests", "s", fill=green)
    c.text(28, 350, "A read outside the window refills it under the same ETag; a changed ETag fails the read rather than mixing two revisions of the file.", "s")
    c.text(28, 376, "Skipped arrays never decide the window size: the 64 KiB minimum applies to the small fields, and a small array larger than that reads its own length.", "s")
    c.w = 1220
    c.h = 410
    c.write("metadata-window.svg")


def source_cache():
    c = Canvas("Measuring retained documents without retaining them",
               "Scene owns each source document through an Rc; SourceCache keeps only a Weak handle beside the payload "
               "figure it computed. Matching pointers reuse that figure, a replaced document costs one new walk, and a "
               "document Scene drops is freed at once; the cache reports the drop instead of holding the bytes alive.",
               1180, 470)
    navy, pink, green, yellow, grey = PAL["navy"], PAL["pink"], PAL["green"], PAL["yellow"], PAL["grey"]
    c.text(28, 40, "Strong ownership in Scene, weak identity in the cache", "h")
    a = c.box(28, 80, ["Scene", "docs: Vec<FileDoc>", "each holds one Rc<Session>"], "cpu")
    d1 = c.box(360, 80, ["Session A · Rc count 1", "meshes · breps · clouds"], "gpu")
    d2 = c.box(360, 170, ["Session B · Rc count 1", "curves · text"], "gpu")
    k = c.box(700, 80, ["SourceCache", "Weak<Session> per document", "Payload { known bytes, scans }"], "cpu")
    def arrow(x1, y1, x2, y2, color, dash=""):
        c.raw(f'<line x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}" stroke="{color}" stroke-width="1.6"{dash}/>')
        c.raw(f'<polygon points="{x2},{y2} {x2 - 9},{y2 - 5} {x2 - 9},{y2 + 5}" fill="{color}"/>')
    arrow(a[0] + a[2], a[1] + 40, d1[0], d1[1] + 30, navy)
    arrow(a[0] + a[2], a[1] + 52, d2[0], d2[1] + 30, navy)
    c.text(a[0] + a[2] + 12, a[1] + 28, "Rc (strong)", "s", fill=navy)
    dash = ' stroke-dasharray="5 4"'
    c.raw(f'<line x1="{k[0]}" y1="{k[1] + 30}" x2="{d1[0] + d1[2]}" y2="{d1[1] + 30}" stroke="{grey}" stroke-width="1.6"{dash}/>')
    c.raw(f'<polygon points="{d1[0] + d1[2]},{d1[1] + 30} {d1[0] + d1[2] + 9},{d1[1] + 25} {d1[0] + d1[2] + 9},{d1[1] + 35}" fill="{grey}"/>')
    c.raw(f'<line x1="{k[0]}" y1="{k[1] + 52}" x2="{d2[0] + d2[2]}" y2="{d2[1] + 30}" stroke="{grey}" stroke-width="1.6"{dash}/>')
    c.raw(f'<polygon points="{d2[0] + d2[2]},{d2[1] + 30} {d2[0] + d2[2] + 9},{d2[1] + 25} {d2[0] + d2[2] + 9},{d2[1] + 35}" fill="{grey}"/>')
    c.text(d2[0] + d2[2] + 12, d2[1] + d2[3] + 18, "Weak (no ownership), dashed", "s", fill=grey)
    y = 290
    c.text(28, y, "Three snapshots", "l")
    r1 = c.box(28, y + 14, ["snapshot 1", "identities new → walk A, walk B", "scans = 2"], "note")
    r2 = c.box(r1[0] + r1[2] + 24, y + 14, ["snapshot 2 · same Rc pointers", "cached figures reused", "scans = 2"], "note")
    r3 = c.box(r2[0] + r2[2] + 24, y + 14, ["snapshot 3 · B replaced by B'", "A reused, B' walked once", "scans = 3; B freed immediately"], "note")
    c.text(28, y + 130, "The figure is a known payload: geometry arrays the walk can size exactly; anything it cannot size is listed under exclusions, never guessed.", "s")
    c.w = max(int(r3[0] + r3[2] + 28), 1180)
    c.h = 450
    c.write("source-cache.svg")


def joins():
    c = Canvas("One join plane per shared vertex",
               "One ribbon per segment. Independent ribbons either overlap at a bend, darkening the joint where coverage "
               "adds up, or leave a wedge open on the outer side. The joined lane gives each segment its neighbours and "
               "cuts both ribbons at one join plane through the shared vertex, so a dense polyline and a coarse one join "
               "alike.",
               1180, 470)
    navy, pink, green, yellow, grey = PAL["navy"], PAL["pink"], PAL["green"], PAL["yellow"], PAL["grey"]
    c.text(28, 40, "Two ribbons at a bend, before and after the join plane", "h")
    import math
    def ribbon(ax, ay, bx, by, w, fill, opacity):
        dx, dy = bx - ax, by - ay
        n = math.hypot(dx, dy)
        nx, ny = -dy / n * w / 2, dx / n * w / 2
        c.raw(f'<polygon points="{ax + nx:.1f},{ay + ny:.1f} {bx + nx:.1f},{by + ny:.1f} {bx - nx:.1f},{by - ny:.1f} {ax - nx:.1f},{ay - ny:.1f}" fill="{fill}" fill-opacity="{opacity}"/>')
    # left: independent ribbons (overlap darkens, outer wedge opens)
    c.text(60, 84, "independent ribbons", "l")
    A, V, B = (80, 300), (260, 150), (440, 300)
    ribbon(*A, *V, 36, "#111111", 0.55)
    ribbon(*V, *B, 36, "#111111", 0.55)
    c.raw(f'<circle cx="{V[0]}" cy="{V[1]}" r="4" fill="{pink}"/>')
    c.text(V[0] + 30, V[1] - 46, "overlap adds coverage: a darker dot", "s", fill=pink)
    c.text(V[0] + 30, V[1] - 30, "outer side: an open wedge", "s", fill=pink)
    c.text(60, 350, "the same stroke with 40 short segments shows 39 such dots", "s")
    # right: joined ribbons cut at the join plane
    c.text(660, 84, "joined lane: cut at the join plane", "l")
    A2, V2, B2 = (680, 300), (860, 150), (1040, 300)
    def unit(p, q):
        dx, dy = q[0] - p[0], q[1] - p[1]
        n = math.hypot(dx, dy)
        return dx / n, dy / n
    d1 = unit(A2, V2); d2 = unit(V2, B2)
    bis = (d1[0] - d2[0], d1[1] - d2[1])
    nb = math.hypot(*bis); bis = (bis[0] / nb, bis[1] / nb)
    # miter extent along the bisector so the outer edges meet
    cos_half = (-(d1[0] * d2[0] + d1[1] * d2[1]) + 1) / 2
    w = 36
    ml = (w / 2) / max(math.sqrt(max(cos_half, 1e-6)), 0.2)
    def side(p, d, sgn):
        return (p[0] - d[1] * sgn * w / 2, p[1] + d[0] * sgn * w / 2)
    a_in, a_out = side(A2, d1, 1), side(A2, d1, -1)
    b_in, b_out = side(B2, d2, 1), side(B2, d2, -1)
    m_out = (V2[0] + bis[0] * ml, V2[1] + bis[1] * ml)
    m_in = (V2[0] - bis[0] * ml, V2[1] - bis[1] * ml)
    # choose orientation so that m_out is on the outer side (above the vertex)
    if m_out[1] > V2[1]:
        m_out, m_in = m_in, m_out
    c.raw(f'<polygon points="{a_out[0]:.1f},{a_out[1]:.1f} {m_out[0]:.1f},{m_out[1]:.1f} {m_in[0]:.1f},{m_in[1]:.1f} {a_in[0]:.1f},{a_in[1]:.1f}" fill="#111111" fill-opacity="0.55"/>')
    c.raw(f'<polygon points="{m_out[0]:.1f},{m_out[1]:.1f} {b_out[0]:.1f},{b_out[1]:.1f} {b_in[0]:.1f},{b_in[1]:.1f} {m_in[0]:.1f},{m_in[1]:.1f}" fill="#111111" fill-opacity="0.55"/>')
    c.raw(f'<line x1="{m_out[0]:.1f}" y1="{m_out[1]:.1f}" x2="{m_in[0]:.1f}" y2="{m_in[1]:.1f}" stroke="{green}" stroke-width="2" stroke-dasharray="5 4"/>')
    c.raw(f'<circle cx="{V2[0]}" cy="{V2[1]}" r="4" fill="{green}"/>')
    c.text(V2[0] + 30, V2[1] - 46, "join plane through the shared vertex", "s", fill=green)
    c.text(V2[0] + 30, V2[1] - 30, "no overlap, no wedge, uniform coverage", "s", fill=green)
    c.text(660, 350, "each GPU row carries prev and next; the shader clips its ribbon at both join planes", "s")
    c.text(28, 400, "Chains are marked by the producers (Step 12), the row gains neighbours (Step 13), and the shader computes one plane per shared vertex (Step 14).", "s")
    c.w = 1180
    c.h = 430
    c.write("joins.svg")


def ribbon():
    c = Canvas("A stroke segment is a screen-space quad",
               "A segment has no mesh. The vertex shader emits six vertices per row, placing a camera-facing quad around the projected axis, and the half-width at each end travels down as a scalar so the trapezoid resolves per pixel. Coverage is not a distance ramp: band_area integrates the pixel box against the capsule exactly, so a thin stroke cannot beat with its own subpixel phase.",
               1180, 460)
    import math
    pink, green, grey = PAL["pink"], PAL["green"], PAL["grey"]
    c.text(28, 40, "Six vertices, then exact coverage", "h")

    c.text(60, 86, "no vertex buffer: the quad is built in the shader", "l")
    ax, ay, bx, by = 110.0, 320.0, 440.0, 190.0
    h0, h1 = 40.0, 22.0
    dx, dy = bx - ax, by - ay
    n = math.hypot(dx, dy)
    nx, ny = -dy / n, dx / n
    p = [(ax + nx * h0, ay + ny * h0), (bx + nx * h1, by + ny * h1),
         (bx - nx * h1, by - ny * h1), (ax - nx * h0, ay - ny * h0)]
    c.raw('<polygon points="' + " ".join(f"{x:.1f},{y:.1f}" for x, y in p) + '" fill="#ffffff" fill-opacity="0.10" stroke="#111111" stroke-width="1.4"/>')
    c.raw(f'<line x1="{p[0][0]:.1f}" y1="{p[0][1]:.1f}" x2="{p[2][0]:.1f}" y2="{p[2][1]:.1f}" stroke="{grey}" stroke-width="1" stroke-dasharray="4 4"/>')
    c.raw(f'<line x1="{ax}" y1="{ay}" x2="{bx}" y2="{by}" stroke="{pink}" stroke-width="1.8"/>')
    for x, y in p:
        c.raw(f'<circle cx="{x:.1f}" cy="{y:.1f}" r="4.5" fill="#111111"/>')
    c.raw(f'<line x1="{ax:.1f}" y1="{ay:.1f}" x2="{p[0][0]:.1f}" y2="{p[0][1]:.1f}" stroke="{green}" stroke-width="1.6"/>')
    c.raw(f'<line x1="{bx:.1f}" y1="{by:.1f}" x2="{p[1][0]:.1f}" y2="{p[1][1]:.1f}" stroke="{green}" stroke-width="1.6"/>')
    c.text(96, 372, "two triangles share the dashed diagonal", "s")
    c.text(96, 394, "the axis is the row; the corners are computed", "s", fill=pink)
    c.text(96, 416, "one half-width per end, resolved per pixel", "s", fill=green)

    c.text(660, 86, "coverage is an area, not a distance", "l")
    gx, gy, cell = 700.0, 150.0, 58.0
    for i in range(4):
        for j in range(3):
            c.raw(f'<rect x="{gx + i * cell:.1f}" y="{gy + j * cell:.1f}" width="{cell}" height="{cell}" fill="none" stroke="{grey}" stroke-width="0.9"/>')
    edge_y0, edge_y1 = gy + 22.0, gy + 3 * cell - 30.0
    c.raw(f'<path d="M{gx:.1f},{edge_y0:.1f} L{gx + 4 * cell:.1f},{edge_y1:.1f}" stroke="{pink}" stroke-width="2"/>')
    # the one pixel whose box is split by the capsule edge: the shaded part is band_area
    px, py = gx + cell, gy + cell
    top = edge_y0 + (edge_y1 - edge_y0) * (cell / (4 * cell))
    bot = edge_y0 + (edge_y1 - edge_y0) * (2 * cell / (4 * cell))
    c.raw(f'<polygon points="{px:.1f},{top:.1f} {px + cell:.1f},{bot:.1f} {px + cell:.1f},{py + cell:.1f} {px:.1f},{py + cell:.1f}" fill="{pink}" fill-opacity="0.55"/>')
    c.raw(f'<rect x="{px:.1f}" y="{py:.1f}" width="{cell}" height="{cell}" fill="none" stroke="#111111" stroke-width="2"/>')
    c.text(660, 372, "band_area integrates one pixel box against the capsule", "s")
    c.text(660, 394, "the shaded part is the pixel's coverage, exactly", "s", fill=pink)
    c.text(660, 416, "a distance ramp beats with the line's subpixel phase; this does not", "s")
    c.write("ribbon.svg")


def markers():
    c = Canvas("Two ways to cover a disc",
               "A vertex marker is a sphere: the template corner is pushed out in clip space by the pixel radius plus half the feather, so four vertices always contain the antialiased disc. A free dot needs no template at all, because one equilateral triangle whose incircle is the disc covers it with three vertices.",
               1180, 430)
    import math
    pink, green, yellow, grey = PAL["pink"], PAL["green"], PAL["yellow"], PAL["grey"]
    c.text(28, 40, "Four vertices, or three", "h")

    c.text(60, 86, "sphere marker: a quad from the template", "l")
    cx, cy, r = 250.0, 258.0, 76.0
    feather = 14.0
    half = r + feather
    c.raw(f'<rect x="{cx - half:.1f}" y="{cy - half:.1f}" width="{2 * half:.1f}" height="{2 * half:.1f}" fill="#ffffff" fill-opacity="0.08" stroke="#111111" stroke-width="1.4"/>')
    c.raw(f'<circle cx="{cx}" cy="{cy}" r="{half:.1f}" fill="none" stroke="{green}" stroke-width="1.2" stroke-dasharray="5 4"/>')
    c.raw(f'<circle cx="{cx}" cy="{cy}" r="{r}" fill="{pink}" fill-opacity="0.45" stroke="{pink}" stroke-width="1.6"/>')
    for sx in (-1, 1):
        for sy in (-1, 1):
            c.raw(f'<circle cx="{cx + sx * half:.1f}" cy="{cy + sy * half:.1f}" r="4.5" fill="#111111"/>')
    c.text(60, 372, "four template corners, offset in clip space", "s")
    c.text(60, 394, "pixel radius plus half the feather, so the quad contains the disc", "s", fill=green)

    c.text(660, 86, "free dot: one equilateral triangle", "l")
    dx0, dy0, dr = 880.0, 258.0, 76.0
    verts = [(dx0 + 2 * dr * math.cos(math.radians(a)), dy0 + 2 * dr * math.sin(math.radians(a))) for a in (-90, 30, 150)]
    c.raw('<polygon points="' + " ".join(f"{x:.1f},{y:.1f}" for x, y in verts) + '" fill="#ffffff" fill-opacity="0.08" stroke="#111111" stroke-width="1.4"/>')
    c.raw(f'<circle cx="{dx0}" cy="{dy0}" r="{dr}" fill="{pink}" fill-opacity="0.45" stroke="{pink}" stroke-width="1.6"/>')
    for x, y in verts:
        c.raw(f'<circle cx="{x:.1f}" cy="{y:.1f}" r="4.5" fill="#111111"/>')
    c.raw(f'<line x1="{dx0}" y1="{dy0}" x2="{dx0}" y2="{dy0 + dr}" stroke="{yellow}" stroke-width="1.4"/>')
    c.raw(f'<circle cx="{dx0}" cy="{dy0}" r="3" fill="{yellow}"/>')
    c.text(660, 372, "three vertices, no template and no vertex buffer", "s")
    c.text(660, 394, "the incircle is the visible disc; the corners are never seen", "s", fill=pink)
    c.write("markers.svg")


def lod():
    c = Canvas("One node, one decision",
               "The LOD walk is pure CPU and asks one question per node: how wide does this node's point spacing land on screen? Wider than lod_px and the walk descends, picking up the finer subsample each child owns. No wider, and the node is drawn whole from its own subsample and nothing below it is read.",
               1180, 480)
    pink, green, grey = PAL["pink"], PAL["green"], PAL["grey"]
    c.text(28, 40, "Descend, or stop", "h")

    def node(x, y, size, step, tint):
        c.raw(f'<rect x="{x:.1f}" y="{y:.1f}" width="{size:.1f}" height="{size:.1f}" fill="{tint}" fill-opacity="0.14" stroke="#111111" stroke-width="1.3"/>')
        count = int(size // step)
        start = (size - (count - 1) * step) / 2
        for i in range(count):
            for j in range(count):
                c.raw(f'<circle cx="{x + start + step * i:.1f}" cy="{y + start + step * j:.1f}" r="1.9" fill="{grey}"/>')

    def ruler(x, y, length, colour):
        c.raw(f'<line x1="{x:.1f}" y1="{y:.1f}" x2="{x + length:.1f}" y2="{y:.1f}" stroke="{colour}" stroke-width="2"/>')
        for at in (x, x + length):
            c.raw(f'<line x1="{at:.1f}" y1="{y - 5:.1f}" x2="{at:.1f}" y2="{y + 5:.1f}" stroke="{colour}" stroke-width="2"/>')

    c.text(110, 86, "spacing wider than lod_px: descend", "l", fill=pink)
    for i in (0, 1):
        for j in (0, 1):
            node(110 + i * 120, 120 + j * 120, 120, 14.0, pink)
    ruler(110, 396, 44, pink)
    c.text(168, 400, "this node's spacing — too wide, so its eight children are visited too", "s", fill=pink)
    ruler(110, 428, 24, grey)
    c.text(168, 432, "lod_px", "s")

    c.text(700, 86, "spacing fits: draw the node whole", "l", fill=green)
    node(700, 120, 240, 34.0, green)
    ruler(700, 396, 16, green)
    c.text(758, 400, "this node's spacing — fits, so the node itself is drawn", "s", fill=green)
    ruler(700, 428, 24, grey)
    c.text(758, 432, "lod_px", "s")

    c.text(28, 464, "Each node owns its subsample, so descending only adds detail; the finest spacing found below a node travels back up to size the discs it is drawn with.", "s")
    c.write("lod.svg")


def arena():
    c = Canvas("One arena, three index runs",
               "Every mesh puts its vertices into one growable arena and its object row into a parallel table of "
               "the same length. A mesh is then a range of indices, not a buffer of its own: a draw binds the two "
               "vertex buffers, binds one index run, and calls draw_indexed once for the whole run.",
               1180, 452)
    grey = PAL["grey"]
    lav, pnk, grn, zer = PAL["blue_band"], PAL["pink_band"], "#bfe3a8", PAL["zero_band"]
    c.text(28, 40, "A mesh is a range, not a buffer of its own", "h")

    x0, w0 = 244.0, 856.0

    def band(x, y, w, h, fill, label, cls="s"):
        c.parts.append(f'<rect x="{x:.1f}" y="{y:.1f}" width="{w:.1f}" height="{h:.1f}" rx="{RADIUS}" fill="{fill}"/>')
        if label:
            c.text(x + 10, y + h / 2 + 5, label, cls, fill=PAL["black"], keep=True)

    c.text(28, 116, "three index runs", "l")
    c.text(28, 138, "u32, one bound at a time", "s")
    for name, a, b, fill in (("faces · lit triangles", 0.00, 0.46, lav),
                             ("print · flat fills", 0.50, 0.74, pnk),
                             ("text · outline glyphs", 0.78, 1.00, grn)):
        band(x0 + w0 * a, 100, w0 * (b - a), 40, fill, name)

    meshes = [("mesh A", 0.00, 0.22, lav), ("mesh B", 0.22, 0.62, pnk),
              ("mesh C", 0.62, 0.80, grn), ("mesh D", 0.80, 1.00, zer)]
    c.text(28, 224, "verts", "l")
    c.text(28, 246, "position · normal · colour", "s")
    c.text(28, 268, "@location(0) (1) (2)", "m", fill=grey)
    for name, a, b, fill in meshes:
        band(x0 + w0 * a, 212, w0 * (b - a), 48, fill, name)

    c.text(28, 312, "vids", "l")
    c.text(28, 334, "one object row per vertex", "s")
    c.text(28, 356, "@location(3)", "m", fill=grey)
    for name, a, b, fill in meshes:
        band(x0 + w0 * a, 300, w0 * (b - a), 34, fill, "", "m")
    c.text(x0 + 10, 322, "7 7 7 7", "m", fill=PAL["black"], keep=True)
    c.text(x0 + w0 * 0.24, 322, "8 8 8 8 8 8 8 8", "m", fill=PAL["black"], keep=True)
    c.text(x0 + w0 * 0.64, 322, "9 9 9", "m", fill=PAL["black"], keep=True)

    # An index run names vertices anywhere in the arena: the mesh's base is folded in on the CPU.
    for at in (0.10, 0.34, 0.56, 0.70, 0.90):
        c.arrow(x0 + w0 * at, 144, x0 + w0 * at, 206)

    c.box(28, 380, ["One draw for a whole run",
                    "`set_vertex_buffer(0, verts) · set_vertex_buffer(1, vids)`",
                    "`set_index_buffer(run) · draw_indexed(0..run.len())`"], "note", w=700)
    c.box(756, 380, ["Not vertex pulling",
                     "that lane binds no vertex buffer at all",
                     "`faces.rs` · `vs_face` · lesson 17"], "sel", w=344)
    c.h = 380 + 86 + 34
    c.text(28, c.h - 16, "The mesh's base is folded into the indices on the CPU when the run is built, so a run addresses the whole arena directly.", "s", fill=PAL["yellow"])
    c.write("arena.svg")


def stages():
    c = Canvas("Three vertices in, thousands of fragments out",
               "The two shaders you write never meet. vs_main runs once per vertex; its only required output is a clip "
               "position. Between them sits the rasterizer, which you do not write: it works out which pixels the "
               "triangle covers and blends the vertex outputs across them. fs_main runs once per covered pixel and sees "
               "only the blend.",
               1180, 540)
    navy, pink, green, grey, yellow = PAL["navy"], PAL["pink"], PAL["green"], PAL["grey"], PAL["yellow"]
    c.text(28, 40, "The two shaders never meet", "h")

    def triangle(ox, oy):
        return [(ox + 40.0, oy + 40.0), (ox + 250.0, oy + 10.0), (ox + 150.0, oy + 180.0)]

    def inside(p, tri):
        (x1, y1), (x2, y2), (x3, y3) = tri
        d = (y2 - y3) * (x1 - x3) + (x3 - x2) * (y1 - y3)
        a = ((y2 - y3) * (p[0] - x3) + (x3 - x2) * (p[1] - y3)) / d
        b = ((y3 - y1) * (p[0] - x3) + (x1 - x3) * (p[1] - y3)) / d
        return a >= 0 and b >= 0 and a + b <= 1

    c.text(60, 86, "vs_main", "l", fill=navy)
    c.text(60, 112, "3 invocations", "s", fill=navy)
    ta = triangle(60, 150)
    c.raw('<polygon points="' + " ".join(f"{x:.1f},{y:.1f}" for x, y in ta) + '" fill="none" stroke="#111111" stroke-width="1.4"/>')
    for x, y in ta:
        c.raw(f'<circle cx="{x:.1f}" cy="{y:.1f}" r="7" fill="{navy}"/>')
    c.text(60, 400, "one call per vertex; the only output it owes", "s")
    c.text(60, 422, "is @builtin(position), in clip space", "s")

    c.text(450, 86, "the rasterizer", "l", fill=grey)
    c.text(450, 112, "fixed function: you do not write it", "s", fill=grey)
    tb = triangle(450, 150)
    step = 13.0
    for i in range(20):
        for j in range(15):
            px, py = 450 + 30 + step * i, 150 + step * j
            if inside((px + step / 2, py + step / 2), tb):
                c.raw(f'<rect x="{px:.1f}" y="{py:.1f}" width="{step - 1.4:.1f}" height="{step - 1.4:.1f}" fill="{pink}" fill-opacity="0.34"/>')
    c.raw('<polygon points="' + " ".join(f"{x:.1f},{y:.1f}" for x, y in tb) + '" fill="none" stroke="#111111" stroke-width="1.4"/>')
    for x, y in tb:
        c.raw(f'<circle cx="{x:.1f}" cy="{y:.1f}" r="7" fill="{navy}"/>')
    c.text(450, 400, "it finds the covered pixels and blends the", "s")
    c.text(450, 422, "three vertex outputs across every one of them", "s")

    c.text(830, 86, "fs_main", "l", fill=pink)
    c.text(830, 112, "one invocation per covered pixel", "s", fill=pink)
    tc = triangle(830, 150)
    c.raw('<polygon points="' + " ".join(f"{x:.1f},{y:.1f}" for x, y in tc) + '" fill="none" stroke="#111111" stroke-width="1.4"/>')
    frag = (tc[0][0] + 110.0, tc[0][1] + 90.0)
    for px, py in tc:
        c.raw(f'<line x1="{px:.1f}" y1="{py:.1f}" x2="{frag[0]:.1f}" y2="{frag[1]:.1f}" stroke="{grey}" stroke-width="1" stroke-dasharray="4 4"/>')
        c.raw(f'<circle cx="{px:.1f}" cy="{py:.1f}" r="7" fill="{navy}"/>')
    c.raw(f'<rect x="{frag[0] - 8:.1f}" y="{frag[1] - 8:.1f}" width="16" height="16" fill="{yellow}"/>')
    c.text(830, 400, "it never sees a vertex, only the blend", "s")
    c.text(830, 422, "w0·v0 + w1·v1 + w2·v2, summing to one", "s", fill=yellow)

    c.arrow(330, 250, 470, 250)
    c.arrow(730, 250, 870, 250)
    c.text(28, 484, "This is why a value cannot travel from one pixel to its neighbour.", "s")
    c.text(28, 506, "Anything a fragment needs is a uniform, a buffer it can index, or something a vertex handed to the rasterizer first.", "s")
    c.write("stages.svg")


def interpolate():
    c = Canvas("What the rasterizer hands over",
               "A value carried from the vertex shader to the fragment shader is blended, and the blend is perspective-correct: evenly spaced pixels do not correspond to evenly spaced points on the surface. Marking an output flat turns the blending off, so every fragment of the triangle reads one vertex's value unchanged. Both are used here: a colour interpolates, a stroke's half-width does not.",
               1180, 500)
    navy, pink, green, grey, yellow = PAL["navy"], PAL["pink"], PAL["green"], PAL["grey"], PAL["yellow"]
    c.text(28, 40, "Blended, or not blended", "h")

    c.text(60, 86, "the blend is perspective-correct", "l", fill=pink)
    ex, ey = 90.0, 250.0
    sx = 250.0
    ax, ay, bx2, by2 = 420.0, 170.0, 580.0, 330.0

    def to_screen(px, py):
        return ey + (sx - ex) / (px - ex) * (py - ey)

    # Sample the screen only across the span the surface actually subtends, so every ray lands
    # on the segment and the uneven spacing that comes back is the whole point of the picture.
    sa, sb = to_screen(ax, ay), to_screen(bx2, by2)
    c.raw(f'<line x1="{sx}" y1="{min(sa, sb) - 40:.1f}" x2="{sx}" y2="{max(sa, sb) + 40:.1f}" stroke="{grey}" stroke-width="1.4"/>')
    c.text(214, min(sa, sb) - 52, "screen", "s")
    c.raw(f'<line x1="{ax}" y1="{ay}" x2="{bx2}" y2="{by2}" stroke="{pink}" stroke-width="2.4"/>')
    c.raw(f'<circle cx="{ex}" cy="{ey}" r="5" fill="{yellow}"/>')
    c.text(60, 292, "eye", "s", fill=yellow)
    sxx, syy = bx2 - ax, by2 - ay
    for k in range(7):
        py = sa + (sb - sa) * k / 6.0
        dxr, dyr = sx - ex, py - ey
        u = ((ax - ex) * syy - (ay - ey) * sxx) / (dxr * syy - dyr * sxx)
        hx, hy = ex + dxr * u, ey + dyr * u
        c.raw(f'<line x1="{sx:.1f}" y1="{py:.1f}" x2="{hx:.1f}" y2="{hy:.1f}" stroke="{grey}" stroke-width="0.9" stroke-dasharray="4 4"/>')
        c.raw(f'<circle cx="{sx:.1f}" cy="{py:.1f}" r="3.4" fill="#111111"/>')
        c.raw(f'<circle cx="{hx:.1f}" cy="{hy:.1f}" r="3.4" fill="{pink}"/>')
    c.text(430, 378, "the surface, seen edge on", "s", fill=pink)
    c.text(60, 412, "seven evenly spaced pixels, seven points that are not evenly spaced", "s")
    c.text(60, 434, "dividing by w is what keeps the two in step", "s", fill=pink)

    c.text(700, 86, "@interpolate(flat): no blend at all", "l", fill=green)
    tri = [(760.0, 320.0), (930.0, 160.0), (1100.0, 320.0)]
    c.raw('<polygon points="' + " ".join(f"{x:.1f},{y:.1f}" for x, y in tri) + '" fill="{}" fill-opacity="0.30" stroke="#111111" stroke-width="1.4"/>'.format(green))
    for x, y in tri:
        c.raw(f'<circle cx="{x:.1f}" cy="{y:.1f}" r="6" fill="#111111"/>')
    c.raw(f'<circle cx="{tri[0][0]:.1f}" cy="{tri[0][1]:.1f}" r="9" fill="{green}"/>')
    c.text(700, 380, "one vertex decides for the whole triangle", "s", fill=green)
    c.text(700, 412, "a stroke's half-width goes down flat and is resolved per pixel:", "s")
    c.text(700, 434, "a width blended over a trapezoid would be projective, and wrong", "s")
    c.write("interpolate.svg")


def frustum():
    c = Canvas("The frustum becomes a cube",
               "The projection matrix and the divide by w take everything inside the viewing frustum and land it in a cube. Depth does not survive that trip evenly: with the near and far planes swapped, so that near maps to one and far to zero, the float values crowd where the geometry is instead of where it is not.",
               1180, 500)
    navy, pink, green, grey, yellow = PAL["navy"], PAL["pink"], PAL["green"], PAL["grey"], PAL["yellow"]
    c.text(28, 40, "Depth does not survive evenly", "h")

    c.text(60, 86, "view space: metres from the eye", "l")
    ex, ey = 100.0, 250.0
    near_x, far_x = 200.0, 560.0
    half_near, half_far = 44.0, 128.0
    c.raw(f'<polygon points="{near_x},{ey - half_near} {far_x},{ey - half_far} {far_x},{ey + half_far} {near_x},{ey + half_near}" fill="#ffffff" fill-opacity="0.07" stroke="#111111" stroke-width="1.3"/>')
    for side in (-1, 1):
        c.raw(f'<line x1="{ex:.1f}" y1="{ey:.1f}" x2="{far_x:.1f}" y2="{ey + side * half_far:.1f}" stroke="{grey}" stroke-width="1" stroke-dasharray="5 5"/>')
    c.raw(f'<circle cx="{ex}" cy="{ey}" r="5" fill="{yellow}"/>')
    c.text(62, 300, "eye", "s", fill=yellow)
    depths = [(1.0, near_x), (1.5, 260.0), (2.5, 350.0), (4.0, 450.0), (8.0, far_x)]
    for scale, x in depths:
        c.raw(f'<circle cx="{x:.1f}" cy="{ey:.1f}" r="4.5" fill="{pink}"/>')
    c.text(60, 400, "five points, spread out through the frustum", "s", fill=pink)
    c.text(60, 422, "at 1×, 1.5×, 2.5×, 4× and 8× the near distance", "s")

    c.text(700, 86, "clip, then divide by w: the cube", "l")
    cx0, cy0, cs = 740.0, 130.0, 240.0
    c.raw(f'<rect x="{cx0}" y="{cy0}" width="{cs}" height="{cs}" fill="#ffffff" fill-opacity="0.07" stroke="#111111" stroke-width="1.3"/>')
    for scale, _ in depths:
        ndc = 1.0 / scale
        y = cy0 + (1.0 - ndc) * cs
        c.raw(f'<line x1="{cx0:.1f}" y1="{y:.1f}" x2="{cx0 + cs:.1f}" y2="{y:.1f}" stroke="{pink}" stroke-width="1.8"/>')
    c.text(1000, 148, "near = 1", "s", fill=green)
    c.text(1000, 380, "far = 0", "s", fill=green)
    c.text(700, 400, "the same five points: the far ones crowd into a thin band", "s", fill=pink)
    c.text(700, 422, "and that band sits at zero, where float32 is densest — which is the swap", "s")
    c.text(28, 458, "Only x and y go on to the viewport map, from [-1, 1] to pixels.", "s")
    c.text(28, 480, "Depth is compared and never displayed, so all that matters is that two nearby surfaces land on two different floats.", "s")
    c.write("frustum.svg")


def camera_basis():
    c = Canvas("Three gestures, three fields",
               "Each gesture changes exactly one camera field: orbit turns the orientation about the target, pan slides "
               "the target across the camera's own plane, the wheel scales the distance and never reaches zero. The view "
               "matrix is rebuilt from the three, so no gesture can reach a state the others cannot undo.",
               1180, 420)
    navy, pink, green, grey, yellow = PAL["navy"], PAL["pink"], PAL["green"], PAL["grey"], PAL["yellow"]
    c.text(28, 40, "Each gesture moves one field", "h")
    import math

    def eye(x, y, angle, colour):
        dx, dy = math.cos(angle), math.sin(angle)
        px, py = -dy, dx
        c.raw(f'<polygon points="{x + dx * 18:.1f},{y + dy * 18:.1f} {x - dx * 10 + px * 13:.1f},{y - dy * 10 + py * 13:.1f} {x - dx * 10 - px * 13:.1f},{y - dy * 10 - py * 13:.1f}" fill="{colour}"/>')

    panels = [
        (90.0, "orbit", "the orientation turns; target and distance hold", pink),
        (490.0, "pan", "the target slides in the camera's plane", green),
        (890.0, "wheel", "the distance scales, and never reaches zero", yellow),
    ]
    for ox, name, note, colour in panels:
        c.text(ox, 92, name, "l", fill=colour)
        tx, ty, radius = ox + 110.0, 230.0, 90.0
        c.raw(f'<circle cx="{tx:.1f}" cy="{ty:.1f}" r="4.5" fill="#111111"/>')
        c.text(tx - 22, ty + 30, "target", "s")
        if name == "orbit":
            c.raw(f'<circle cx="{tx:.1f}" cy="{ty:.1f}" r="{radius:.1f}" fill="none" stroke="{grey}" stroke-width="1" stroke-dasharray="5 5"/>')
            for angle in (math.radians(200), math.radians(250)):
                eye(tx + math.cos(angle) * radius, ty + math.sin(angle) * radius, angle + math.pi, colour)
            c.raw(f'<path d="M{tx + math.cos(math.radians(200)) * (radius + 22):.1f},{ty + math.sin(math.radians(200)) * (radius + 22):.1f} A{radius + 22:.1f},{radius + 22:.1f} 0 0 1 {tx + math.cos(math.radians(250)) * (radius + 22):.1f},{ty + math.sin(math.radians(250)) * (radius + 22):.1f}" fill="none" stroke="{colour}" stroke-width="2"/>')
        elif name == "pan":
            eye(tx - radius, ty, 0.0, colour)
            c.raw(f'<line x1="{tx:.1f}" y1="{ty:.1f}" x2="{tx + 70:.1f}" y2="{ty - 44:.1f}" stroke="{colour}" stroke-width="2"/>')
            c.raw(f'<circle cx="{tx + 70:.1f}" cy="{ty - 44:.1f}" r="4.5" fill="{colour}"/>')
        else:
            far_x, near_x = tx - 150.0, tx - 58.0
            c.raw(f'<line x1="{far_x:.1f}" y1="{ty:.1f}" x2="{tx:.1f}" y2="{ty:.1f}" stroke="{grey}" stroke-width="1" stroke-dasharray="5 5"/>')
            eye(far_x, ty, 0.0, grey)
            eye(near_x, ty, 0.0, colour)
            c.arrow(far_x + 26, ty + 46, near_x - 6, ty + 46)
        c.text(ox, 342, note, "s", fill=colour)
    c.text(28, 392, "The view-projection is rebuilt from target, distance and orientation every frame, so the three gestures compose in any order and each one is exactly undoable.", "s")
    c.write("camera-basis.svg")


def masks():
    c = Canvas("One border, however many masks",
               "The silhouette is not drawn from geometry. Ordinary and selected ink each rasterize into an R8 coverage mask, the compositor takes the larger of the two so a selected edge thickens without doubling, and a pooled copy holding the maximum of every block lets a fragment with no ink anywhere near it return zero without entering the search loop at all.",
               1180, 500)
    import math
    pink, green, yellow, grey = PAL["pink"], PAL["green"], PAL["yellow"], PAL["grey"]
    c.text(28, 40, "Two coverage masks, pooled, then one border", "h")

    def pentagon(cx, cy, r=86.0):
        return [(cx + math.cos(math.radians(-90 + k * 72)) * r, cy + math.sin(math.radians(-90 + k * 72)) * r) for k in range(5)]

    def outline(pts, colour, width, opacity=1.0):
        c.raw('<polygon points="' + " ".join(f"{x:.1f},{y:.1f}" for x, y in pts) + f'" fill="none" stroke="{colour}" stroke-width="{width}" stroke-opacity="{opacity}" stroke-linejoin="round"/>')

    def near(p, pts, reach):
        best = 1e9
        for i in range(len(pts)):
            (ax, ay), (bx, by) = pts[i], pts[(i + 1) % len(pts)]
            dx, dy = bx - ax, by - ay
            t = max(0.0, min(1.0, ((p[0] - ax) * dx + (p[1] - ay) * dy) / (dx * dx + dy * dy)))
            best = min(best, math.hypot(p[0] - (ax + dx * t), p[1] - (ay + dy * t)))
        return best <= reach

    c.text(60, 86, "two masks", "l", fill=pink)
    one = pentagon(190, 250)
    outline(one, yellow, 18, 0.55)
    outline(one, pink, 6)
    c.text(60, 386, "ordinary ink is thin, selected ink is thicker,", "s")
    c.text(60, 408, "and each lands in its own R8 attachment", "s", fill=pink)

    c.text(460, 86, "max, not sum", "l", fill=green)
    two = pentagon(590, 250)
    outline(two, green, 18, 0.75)
    c.text(460, 386, "the larger of the two, so a selected edge", "s")
    c.text(460, 408, "thickens instead of doubling in weight", "s", fill=green)

    c.text(850, 86, "block maxima", "l", fill=yellow)
    three = pentagon(980, 250)
    outline(three, grey, 18, 0.35)
    block = 27.0
    for i in range(9):
        for j in range(9):
            bx, by = 980 - 121 + block * i, 250 - 121 + block * j
            lit = near((bx + block / 2, by + block / 2), three, 9 + block * 0.7)
            colour, opacity = (yellow, 0.42) if lit else (grey, 0.10)
            c.raw(f'<rect x="{bx:.1f}" y="{by:.1f}" width="{block - 2:.1f}" height="{block - 2:.1f}" fill="{colour}" fill-opacity="{opacity}"/>')
    c.text(850, 386, "a block that holds no ink answers zero", "s")
    c.text(850, 408, "and the fragment never enters the search loop", "s", fill=yellow)

    c.arrow(320, 250, 440, 250)
    c.arrow(720, 250, 830, 250)
    c.text(28, 462, "What comes out is one black border of uniform width, whatever mix of ordinary and selected ink produced the coverage underneath it.", "s")
    c.write("masks.svg")


def device_scale():
    c = Canvas("Three rulers over the same glass",
               "A pointer position and a rendered pixel are measured in different units. winit reports at the browser's ratio whatever ?dpr= asks for, so every arriving position is multiplied by surface_per_physical - the cap over the ratio, and 1 when there is no cap - before it is used. Picks, zooms and drags then read against the surface that is actually drawn.",
               1180, 430)
    pink, green, yellow, grey, navy = PAL["pink"], PAL["green"], PAL["yellow"], PAL["grey"], PAL["navy"]
    c.text(28, 40, "The pointer and the surface are different units", "h")
    c.text(60, 86, "the same strip of screen, measured three ways", "l")

    left, width = 268.0, 852.0
    at = 0.5

    def ruler(y, cells, colour, label):
        c.text(60, y + 5, label, "s", fill=colour)
        c.raw(f'<line x1="{left:.1f}" y1="{y:.1f}" x2="{left + width:.1f}" y2="{y:.1f}" stroke="{colour}" stroke-width="1.8"/>')
        for k in range(cells + 1):
            x = left + width * k / cells
            c.raw(f'<line x1="{x:.1f}" y1="{y - 9:.1f}" x2="{x:.1f}" y2="{y + 9:.1f}" stroke="{colour}" stroke-width="1.8"/>')

    ruler(160, 6, grey, "CSS px")
    ruler(232, 12, pink, "device px · ratio 2")
    ruler(304, 6, green, "surface px · ?dpr=1 caps it")
    mark = left + width * at
    c.raw(f'<line x1="{mark:.1f}" y1="142" x2="{mark:.1f}" y2="322" stroke="{yellow}" stroke-width="1.4" stroke-dasharray="5 5"/>')
    for y in (160, 232, 304):
        c.raw(f'<circle cx="{mark:.1f}" cy="{y:.1f}" r="5" fill="{yellow}"/>')
    c.text(mark + 14, 134, "one point on the glass", "s", fill=yellow)

    c.text(60, 366, "winit reports 6 device px; × surface_per_physical 0.5 = 3 surface px", "s", fill=yellow)
    c.text(60, 388, "the cap over the ratio, and 1 when nothing is capped, applied on arrival", "s")
    c.text(60, 410, "At two physical pixels per CSS pixel the density already halves the stair-steps, so samples_for drops to 1×: the same edge for a quarter of the attachment memory.", "s")
    c.write("device-scale.svg")


def section_plane():
    c = Canvas("Where the cut happens",
               "Three cuts of one solid. Vertex-stage rejection drops whole triangles, so the cut edge is a staircase of "
               "mesh edges. Fragment-stage discard cuts exactly on the plane, but leaves a hollow shell showing the "
               "inside of the far wall. A cap makes it read as solid.",
               1180, 500)
    import math
    pink, green, yellow, grey = PAL["pink"], PAL["green"], PAL["yellow"], PAL["grey"]
    c.text(28, 40, "A plane, and three ways it looks", "h")

    cols, rows, cell = 6, 5, 38.0
    span, height = cols * cell, rows * cell
    angle = math.radians(-14)
    normal = (math.cos(angle), math.sin(angle))

    def grid(ox, oy):
        out = []
        for i in range(cols):
            for j in range(rows):
                x, y = ox + i * cell, oy + j * cell
                out.append([(x, y), (x + cell, y), (x, y + cell)])
                out.append([(x + cell, y), (x + cell, y + cell), (x, y + cell)])
        return out

    def side(p, origin):
        return (p[0] - origin[0]) * normal[0] + (p[1] - origin[1]) * normal[1]

    def clip(pts, origin):
        """Sutherland-Hodgman against the half-plane behind the section plane."""
        out = []
        for i in range(len(pts)):
            a, b = pts[i], pts[(i + 1) % len(pts)]
            da, db = side(a, origin), side(b, origin)
            if da <= 0:
                out.append(a)
            if (da <= 0) != (db <= 0):
                t = da / (da - db)
                out.append((a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t))
        return out

    def poly(pts, opacity, stroke, width=0.9):
        if len(pts) < 3:
            return
        c.raw('<polygon points="' + " ".join(f"{x:.1f},{y:.1f}" for x, y in pts) +
              f'" fill="{grey}" fill-opacity="{opacity}" stroke="{stroke}" stroke-width="{width}"/>')

    for ox, mode, colour, head in ((60, "vertex", pink, "vertex stage: a staircase"),
                                   (460, "fragment", green, "fragment stage: an exact cut"),
                                   (850, "capped", yellow, "and a cap, or you see inside")):
        c.text(ox, 86, head, "l", fill=colour)
        left, top = ox + 8.0, 150.0
        origin = (left + span * 0.62, top + height / 2)
        c.raw(f'<rect x="{left:.1f}" y="{top:.1f}" width="{span:.1f}" height="{height:.1f}" fill="none" stroke="{grey}" stroke-width="1" stroke-dasharray="3 5"/>')
        for tri in grid(left, top):
            if mode == "vertex":
                if all(side(p, origin) <= 0 for p in tri):
                    poly(tri, 0.34, "#111111")
            else:
                poly(clip(tri, origin), 0.34, "#111111")
        along = (-normal[1], normal[0])
        reach = height / 2 + 26
        ax, ay = origin[0] - along[0] * reach, origin[1] - along[1] * reach
        bx, by = origin[0] + along[0] * reach, origin[1] + along[1] * reach
        if mode == "capped":
            back = 7.0
            c.raw(f'<polygon points="{ax:.1f},{ay:.1f} {bx:.1f},{by:.1f} {bx - normal[0] * back:.1f},{by - normal[1] * back:.1f} {ax - normal[0] * back:.1f},{ay - normal[1] * back:.1f}" fill="{yellow}"/>')
        if mode != "vertex":
            c.raw(f'<line x1="{ax:.1f}" y1="{ay:.1f}" x2="{bx:.1f}" y2="{by:.1f}" stroke="{green if mode == "fragment" else yellow}" stroke-width="3"/>')
        c.raw(f'<line x1="{ax:.1f}" y1="{ay:.1f}" x2="{bx:.1f}" y2="{by:.1f}" stroke="{grey}" stroke-width="1.6" stroke-dasharray="7 5"/>')

    c.text(60, 392, "the plane crosses a triangle, so the whole triangle goes:", "s")
    c.text(60, 414, "the edge follows the mesh, not the plane", "s", fill=pink)
    c.text(460, 392, "discard decides per pixel, so the edge is the plane", "s")
    c.text(460, 414, "but the opening shows the inside of the far wall", "s", fill=green)
    c.text(850, 392, "back faces painted flat already read as a solid cut", "s")
    c.text(850, 414, "stencilling the cap properly is the stretch goal", "s", fill=yellow)
    c.text(28, 468, "The dashed rectangle is the uncut solid. A discarding fragment shader gives up early depth testing, which is why the plane wants to compile out when it is off.", "s")
    c.write("section-plane.svg")


def three_declarations():
    c = Canvas("One thing, declared three times",
               "Almost every wgpu validation error is the same bug in different clothes: a vertex attribute, a binding, a uniform field or a texture format is declared in three places and only two of them were changed. The message names one of the three; the stale one is usually a different one.",
               1180, 530)
    pink, green, yellow, grey, navy = PAL["pink"], PAL["green"], PAL["yellow"], PAL["grey"], PAL["navy"]
    c.text(28, 40, "Name the thing, then check all three", "h")

    apex, bl, br = (590.0, 190.0), (270.0, 410.0), (910.0, 410.0)
    # WGSL is the corner left behind, so BOTH edges that touch it disagree - that is the whole point.
    c.raw(f'<line x1="{apex[0]}" y1="{apex[1]}" x2="{bl[0]}" y2="{bl[1]}" stroke="{green}" stroke-width="2.4"/>')
    c.raw(f'<line x1="{bl[0]}" y1="{bl[1]}" x2="{br[0]}" y2="{br[1]}" stroke="{pink}" stroke-width="2.8" stroke-dasharray="8 6"/>')
    c.raw(f'<line x1="{apex[0]}" y1="{apex[1]}" x2="{br[0]}" y2="{br[1]}" stroke="{pink}" stroke-width="2.8" stroke-dasharray="8 6"/>')
    for x, y in (apex, bl):
        c.raw(f'<circle cx="{x}" cy="{y}" r="9" fill="{navy}"/>')
    c.raw(f'<circle cx="{br[0]}" cy="{br[1]}" r="11" fill="{pink}"/>')

    c.text(520, 132, "Rust struct", "l")
    c.text(452, 156, "#[repr(C)] fields and offsets", "s")
    c.text(196, 452, "layout", "l")
    c.text(96, 476, "vertex attributes · bind-group entries", "s")
    c.text(846, 452, "WGSL", "l", fill=pink)
    c.text(680, 476, "@location · @binding · struct members — the stale one", "s", fill=pink)
    c.text(742, 268, "both edges that touch it disagree", "s", fill=pink)
    c.text(298, 268, "these two agree: you changed both", "s", fill=green)
    c.text(28, 512, "The error names one corner. Name the thing yourself, then read all three declarations of it: the stale one is rarely the corner the message pointed at.", "s")
    c.write("three-declarations.svg")


def sheet_cost():
    c = Canvas("A sheet is one pen, not ten thousand objects",
               "Loaded as objects, every line pays for a GUID string, a name, a colour and four copies of itself, so a "
               "51 MB sheet lifts the wasm heap by 300 MiB. As one segment batch a line is a few numbers and a source "
               "id; GUID, name and kind sit in a side table read by byte range only on selection.",
               1180, 480)
    pink, green, yellow, grey, navy = PAL["pink"], PAL["green"], PAL["yellow"], PAL["grey"], PAL["navy"]
    c.text(28, 40, "What one line of a drawing costs", "h")

    c.text(60, 86, "one object per line", "l", fill=pink)
    fields = [("guid", 132.0, pink), ("name", 74.0, grey), ("colour", 60.0, navy), ("coords × 4", 130.0, grey)]
    for row in range(5):
        y = 146.0 + row * 40
        x = 60.0
        for label, width, colour in fields:
            c.raw(f'<rect x="{x:.1f}" y="{y:.1f}" width="{width:.1f}" height="26" fill="{colour}" fill-opacity="0.26" stroke="{colour}" stroke-width="1"/>')
            if row == 0:
                c.text(x + 4, y - 10, label, "s", fill=colour)
            x += width + 4
    c.text(60, 384, "one line, four copies and a string that is never read", "s")
    c.text(60, 406, "51 MB of drawing lifts the wasm heap by 300 MiB", "s", fill=pink)

    c.text(660, 86, "one batch, one side table", "l", fill=green)
    bx, by, bw = 660.0, 146.0, 440.0
    cells = 22
    for k in range(cells):
        x = bx + bw * k / cells
        colour = yellow if k == 14 else green
        c.raw(f'<rect x="{x:.1f}" y="{by:.1f}" width="{bw / cells - 2:.1f}" height="34" fill="{colour}" fill-opacity="0.30" stroke="{colour}" stroke-width="1"/>')
    c.text(660, 136, "segments: coords · colour · width · source_id", "s", fill=green)
    mx, my, mw = 780.0, 288.0, 200.0
    c.raw(f'<rect x="{mx:.1f}" y="{my:.1f}" width="{mw:.1f}" height="34" fill="{grey}" fill-opacity="0.22" stroke="{grey}" stroke-width="1"/>')
    c.text(mx - 4, my + 56, ".meta side table: guid · name · kind", "s")
    sel = bx + bw * 14 / cells + 8
    c.raw(f'<path d="M{sel:.1f},{by + 34:.1f} C{sel:.1f},{by + 86:.1f} {mx + 40:.1f},{my - 54:.1f} {mx + 40:.1f},{my - 2:.1f}" fill="none" stroke="{yellow}" stroke-width="1.6" stroke-dasharray="5 4"/>')
    c.text(660, 384, "streamed by HTTP Range, the way a point cloud is", "s")
    c.text(660, 406, "two ranged reads, and only when something is selected", "s", fill=yellow)
    c.text(28, 456, "The batch is what the GPU draws; the side table is what a human asks for. Keeping them apart is what makes a sheet openable on a machine that would otherwise die without a word.", "s")
    c.write("sheet-cost.svg")


def history():
    c = Canvas("A removal is a record, not a gap",
               "Edits are grouped into transactions, and a removal's record is the tombstone undo restores from, so nothing is destroyed at the moment it disappears. The cursor moves back and forward through committed transactions; a save purges the whole buffer, as Rhino does, and history never crosses pb or JSON, so an opened file always starts clean.",
               1180, 480)
    pink, green, yellow, grey, navy = PAL["pink"], PAL["green"], PAL["yellow"], PAL["grey"], PAL["navy"]
    c.text(28, 40, "The buffer behind undo", "h")

    line_y = 246.0
    steps = [("add 3 walls", navy), ("move beam", navy), ("remove column", pink), ("add rail", grey)]
    width, gap = 200.0, 34.0
    for k, (label, colour) in enumerate(steps):
        x = 120.0 + k * (width + gap)
        faded = 0.10 if k == 3 else 0.30
        c.raw(f'<rect x="{x:.1f}" y="{line_y - 30:.1f}" width="{width:.1f}" height="60" rx="{RADIUS}" fill="{colour}" fill-opacity="{faded}" stroke="{colour}" stroke-width="1.4"/>')
        c.text(x + 12, line_y + 6, label, "s", fill=colour)
        c.text(x + 12, line_y - 44, f"transaction {k + 1}", "s")
        if k:
            c.raw(f'<line x1="{x - gap:.1f}" y1="{line_y}" x2="{x:.1f}" y2="{line_y}" stroke="{grey}" stroke-width="1.6"/>')
    tomb_x = 120.0 + 2 * (width + gap)
    c.raw(f'<rect x="{tomb_x + 12:.1f}" y="{line_y + 48:.1f}" width="178" height="34" rx="{RADIUS}" fill="{pink}" fill-opacity="0.22" stroke="{pink}" stroke-width="1.2" stroke-dasharray="5 4"/>')
    c.text(tomb_x + 22, line_y + 70, "tombstone: the column", "s", fill=pink)

    cursor = 120.0 + 3 * (width + gap) - gap / 2
    c.raw(f'<line x1="{cursor:.1f}" y1="{line_y - 84:.1f}" x2="{cursor:.1f}" y2="{line_y + 40:.1f}" stroke="{yellow}" stroke-width="2.4"/>')
    c.text(cursor - 24, line_y - 94, "cursor", "s", fill=yellow)
    c.arrow(cursor - 20, line_y - 64, cursor - 160, line_y - 64, "undo")
    c.arrow(cursor + 20, line_y - 64, cursor + 160, line_y - 64, "redo")

    c.text(110, 384, "a removal keeps its record, so undo restores from it rather than rebuilding", "s", fill=pink)
    c.text(110, 406, "redo past the cursor is discarded the moment a new transaction commits", "s")
    c.text(110, 428, "pb_dump and file_json_dump purge the whole buffer: history is memory only,", "s", fill=green)
    c.text(110, 450, "so a file that is opened again always starts clean", "s", fill=green)
    c.write("history.svg")



def toolchain():
    c = Canvas("Four tools, four artefacts",
               "cargo produces a .wasm a browser cannot load on its own, wasm-bindgen writes the JavaScript that can, "
               "Trunk assembles the page around it, and the browser runs start().",
               1180, 340)
    c.text(28, 40, "From source to a running page", "h")
    c.text(28, 68, "each tool hands the next one file", "l", fill=PAL["grey"])
    y = 130
    a = c.box(60, y, ["cargo", "src/lib.rs", "Cargo.toml"], "cpu")
    b = c.box(a[0] + a[2] + 70, y, ["wasm-bindgen", "session_viewer.wasm"], "cpu")
    d = c.box(b[0] + b[2] + 70, y, ["Trunk", "JS glue + .wasm"], "cpu")
    e = c.box(d[0] + d[2] + 70, y, ["browser", "index.html + bundle"], "gpu")
    for lhs, rhs, label in ((a, b, ".wasm"), (b, d, "JS glue"), (d, e, "dist/")):
        c.arrow(lhs[0] + lhs[2], y + lhs[3] / 2, rhs[0] - 8, y + lhs[3] / 2, label)
    c.text(60, 268, "A .wasm is not loadable on its own: wasm-bindgen writes the JavaScript that instantiates it", "s")
    c.text(60, 292, "and calls start(); Trunk runs both and assembles the page that loads the pair.", "s")
    c.write("toolchain.svg")


def gpu_objects():
    c = Canvas("Instance, surface, adapter, device",
               "The instance picks the backend, the surface is the canvas you present to, the adapter is one physical "
               "GPU chosen to be compatible with that surface, and the device is the handle every later resource comes from.",
               1180, 400)
    c.text(28, 40, "The four objects before any drawing", "h")
    c.text(28, 68, "each one is made from the one before it", "l", fill=PAL["grey"])
    y = 130
    a = c.box(60, y, ["Instance", "which backend", "BROWSER_WEBGPU"], "cpu", w=162)
    b = c.box(a[0] + a[2] + 60, y, ["Surface", "the canvas", "you present to it"], "gpu")
    d = c.box(b[0] + b[2] + 60, y, ["Adapter", "one physical GPU", "compatible_surface"], "gpu")
    e = c.box(d[0] + d[2] + 60, y, ["Device + Queue", "makes every resource", "and takes every command"], "gpu")
    mid = y + a[3] / 2
    c.arrow(a[0] + a[2], mid, b[0] - 8, mid, "canvas")
    c.arrow(b[0] + b[2], mid, d[0] - 8, mid, "request")
    c.arrow(d[0] + d[2], mid, e[0] - 8, mid, "request")
    c.text(60, 320, "The adapter is requested WITH the surface: a GPU that cannot present to this canvas is no use,", "s")
    c.text(60, 344, "and asking for it later is how a device ends up unable to show anything.", "s", fill=PAL["yellow"])
    c.write("gpu-objects.svg")


def clip_space():
    c = Canvas("Clip space and the viewport flip",
               "The shader computes three positions in clip space, a square two units across with y up; the viewport "
               "transform turns that into pixels with y down. The triangle does not move: the numbers on the axes "
               "turn over, and flipping the sign yourself on top of that is what makes a first image upside down.",
               1180, 458)
    navy, pink, yellow, grey = PAL["blue_band"], PAL["pink_band"], PAL["yellow"], PAL["zero"]
    c.text(28, 40, "Two coordinate systems, one flip", "h")

    def panel(x, y, w, h):
        c.parts.append(f'<rect x="{x:.1f}" y="{y:.1f}" width="{w:.1f}" height="{h:.1f}" rx="{RADIUS}" fill="#e4e4e7" fill-opacity="0.10"/>')

    def tri(pts, fill):
        d = " ".join(f"{px:.1f},{py:.1f}" for px, py in pts)
        c.parts.append(f'<polygon points="{d}" fill="{fill}" fill-opacity="0.85"/>')

    # The same three corners, given once. Each side places them with its own mapping.
    corners = [(-0.65, -0.55), (0.65, -0.55), (0.0, 0.72)]

    c.text(90, 96, "clip space, after the divide by w", "l", fill=navy)
    ox, oy, side = 110.0, 126.0, 210.0
    panel(ox, oy, side, side)
    # y up: +1 sits at the top of the square, so the screen row is (1 - y) / 2.
    left = [(ox + (x + 1) / 2 * side, oy + (1 - y) / 2 * side) for x, y in corners]
    tri(left, navy)
    c.parts.append(f'<line x1="{ox}" y1="{oy + side / 2:.1f}" x2="{ox + side}" y2="{oy + side / 2:.1f}" stroke="{grey}" stroke-width="1" stroke-opacity="0.55"/>')
    c.parts.append(f'<line x1="{ox + side / 2:.1f}" y1="{oy}" x2="{ox + side / 2:.1f}" y2="{oy + side}" stroke="{grey}" stroke-width="1" stroke-opacity="0.55"/>')
    c.text(ox + side / 2, oy - 12, "y = +1", "s", anchor="middle", fill=navy)
    c.text(ox + side / 2, oy + side + 22, "y = -1", "s", anchor="middle", fill=navy)
    c.text(ox - 10, oy + side / 2 + 5, "x = -1", "s", anchor="end", fill=navy)
    c.text(ox + side + 10, oy + side / 2 + 5, "x = +1", "s", fill=navy)
    c.text(left[2][0] + 10, left[2][1] - 8, "( 0.0, +0.72 )", "m", fill=navy)

    c.text(700, 96, "framebuffer, after the viewport transform", "l", fill=pink)
    px, py, pw, ph = 700.0, 126.0, 280.0, 210.0
    panel(px, py, pw, ph)
    # y down: the same corner lands on the same row, now counted from the top edge.
    right = [(px + (x + 1) / 2 * pw, py + (1 - y) / 2 * ph) for x, y in corners]
    tri(right, pink)
    c.text(px + pw / 2, py - 12, "y = 0", "s", anchor="middle", fill=pink)
    c.text(px + pw / 2, py + ph + 22, "y = h", "s", anchor="middle", fill=pink)
    c.text(px - 10, py + ph / 2 + 5, "x = 0", "s", anchor="end", fill=pink)
    c.text(px + pw + 10, py + ph / 2 + 5, "x = w", "s", fill=pink)
    c.text(right[2][0] + 10, right[2][1] - 8, "( 0.50 w, 0.14 h )", "m", fill=pink)

    c.arrow(400, oy + side / 2, 618, py + ph / 2, "viewport transform")
    c.text(90, 392, "Both squares hold the same triangle in the same place: the apex is half way across and near the top in each.", "s")
    c.text(90, 416, "Only the numbers turn over - +1 at the top becomes 0, -1 at the bottom becomes h - so an upside-down first", "s")
    c.text(90, 440, "image is a sign you flipped y a second time yourself, not a broken shader.", "s", fill=yellow)
    c.write("clip-space.svg")


def instancing():
    c = Canvas("One draw, two counters",
               "A draw call carries two ranges: vertex_index walks the three corners, instance_index walks the object "
               "rows, and every invocation reads only the row its instance_index names.",
               1180, 420)
    navy, pink = PAL["navy"], PAL["pink"]
    c.text(28, 40, "One call, one buffer, a hundred objects", "h")
    c.text(28, 68, "draw(0..3, 0..100)", "l", fill=PAL["grey"])
    a = c.box(60, 120, ["vertex_index", "0, 1, 2", "the three corners"], "cpu")
    b = c.box(60, 250, ["instance_index", "0 .. 99", "which object row"], "cpu")
    d = c.box(520, 120, ["vs_main", "runs 3 x 100 times", "reads instances[instance_index]"], "gpu")
    e = c.box(900, 250, ["instances[]", "one 96-byte row", "per object"], "gpu")
    c.arrow(a[0] + a[2], 120 + a[3] / 2, d[0] - 8, 120 + a[3] / 2)
    c.arrow(b[0] + b[2], 250 + b[3] / 2, d[0] + d[2] / 3, 120 + d[3] + 8)
    c.arrow(d[0] + d[2], 120 + d[3] / 2, e[0] + e[2] / 2, 250 - 8)
    c.text(60, 376, "Nothing is bound per object: the row is an index, so adding the hundredth object costs one row,", "s")
    c.text(60, 400, "not one draw call.", "s", fill=PAL["yellow"])
    c.write("instancing.svg")


def cpu_gpu():
    c = Canvas("Two sides and a narrow wire",
               "The CPU side you may read and change at any time; the GPU side you send bytes and commands to and "
               "cannot read back casually. Between them is a narrow wire, and almost every mistake is on it.",
               1180, 340)
    c.text(28, 40, "Where does this thing live?", "h")
    a = c.box(60, 110, ["CPU · Rust", "scene, documents, ids, input", "read and change any time"], "cpu")
    b = c.box(760, 110, ["GPU", "buffers, textures, pipelines", "write only; no casual read back"], "gpu")
    wx = a[0] + a[2] + 40
    w = c.box(wx, 110, ["the wire",
                        "`write_buffer · create_buffer_init`",
                        "`vertex layout · bind group · @binding`"], "note", w=b[0] - wx - 40)
    mid = w[1] + w[3] / 2
    c.arrow(a[0] + a[2], mid, w[0] - 8, mid)
    c.arrow(w[0] + w[2], mid, b[0] - 8, mid)
    c.text(60, 258, "Ask this of every value in the course: which side is it on, and who owns it? A bug you cannot", "s")
    c.text(60, 282, "place on this picture is usually a wire bug - three declarations that must agree and do not.", "s", fill=PAL["yellow"])
    c.write("cpu-gpu.svg")


def loop():
    c = Canvas("Read, type, check, understand",
               "One step is four moves: read what it adds, type the block, check that it compiles, then read "
               "why the important lines are there.",
               1180, 320)
    c.text(28, 40, "How one step goes", "h")
    y = 110
    a = c.box(60, y, ["read the idea", "one or two sentences"], "note")
    b = c.box(a[0] + a[2] + 60, y, ["type the block", "by hand, not pasted"], "cpu")
    d = c.box(b[0] + b[2] + 60, y, ["check", "cargo check · the browser"], "cpu")
    e = c.box(d[0] + d[2] + 60, y, ["read the reasoning", "why it is that way"], "sel")
    mid = y + a[3] / 2
    for lhs, rhs in ((a, b), (b, d), (d, e)):
        c.arrow(lhs[0] + lhs[2], mid, rhs[0] - 8, mid)
    c.text(60, 262, "Nothing is hidden: the answer and the reasoning are on the page. The order is what matters -", "s")
    c.text(60, 286, "type it before you read why, and the reasoning lands on something you have already built.", "s", fill=PAL["yellow"])
    c.write("loop.svg")


def tiles():
    c = Canvas("A screen index, one triangle at a time",
               "The screen is a grid of tiles. Each triangle is projected once and a quad over its tile bounds is "
               "rasterized; covered_tile discards the tiles the polygon cannot actually touch, and the survivors are "
               "counted, prefix-summed and written into one flat pool. An ink fragment reads only its own tile's range, "
               "not every triangle in the scene.",
               1180, 510)
    pink, green, navy, yellow, grey = PAL["pink"], PAL["green"], PAL["navy"], PAL["yellow"], PAL["grey"]
    c.text(28, 40, "Only the tiles a triangle can touch", "h")

    cols, rows, tile = 8, 5, 56.0
    ox, oy = 80.0, 150.0
    tri = [(190.0, 196.0), (416.0, 244.0), (268.0, 392.0)]

    def inside(p):
        (x1, y1), (x2, y2), (x3, y3) = tri
        d = (y2 - y3) * (x1 - x3) + (x3 - x2) * (y1 - y3)
        a = ((y2 - y3) * (p[0] - x3) + (x3 - x2) * (p[1] - y3)) / d
        b = ((y3 - y1) * (p[0] - x3) + (x1 - x3) * (p[1] - y3)) / d
        return a >= 0 and b >= 0 and a + b <= 1

    # The quad covers TILE bounds, not the raw bbox: snap outward, or tiles it touches sit outside it.
    lo_i = int((min(p[0] for p in tri) - ox) // tile)
    hi_i = int((max(p[0] for p in tri) - ox) // tile)
    lo_j = int((min(p[1] for p in tri) - oy) // tile)
    hi_j = int((max(p[1] for p in tri) - oy) // tile)
    lo_x, hi_x = ox + lo_i * tile, ox + (hi_i + 1) * tile
    lo_y, hi_y = oy + lo_j * tile, oy + (hi_j + 1) * tile
    touched = 0
    bounded = 0
    for i in range(cols):
        for j in range(rows):
            x, y = ox + i * tile, oy + j * tile
            in_bounds = lo_i <= i <= hi_i and lo_j <= j <= hi_j
            hit = any(inside((x + tile * u, y + tile * v))
                      for u in (0.12, 0.5, 0.88) for v in (0.12, 0.5, 0.88))
            if hit:
                touched += 1
                c.raw(f'<rect x="{x:.1f}" y="{y:.1f}" width="{tile - 2:.1f}" height="{tile - 2:.1f}" fill="{green}" fill-opacity="0.30" stroke="{green}" stroke-width="1"/>')
            elif in_bounds:
                bounded += 1
                c.raw(f'<rect x="{x:.1f}" y="{y:.1f}" width="{tile - 2:.1f}" height="{tile - 2:.1f}" fill="none" stroke="{pink}" stroke-width="1" stroke-dasharray="4 4"/>')
            else:
                c.raw(f'<rect x="{x:.1f}" y="{y:.1f}" width="{tile - 2:.1f}" height="{tile - 2:.1f}" fill="none" stroke="{grey}" stroke-width="0.7"/>')
    c.raw(f'<rect x="{lo_x:.1f}" y="{lo_y:.1f}" width="{hi_x - lo_x:.1f}" height="{hi_y - lo_y:.1f}" fill="none" stroke="{yellow}" stroke-width="1.6" stroke-dasharray="7 5"/>')
    c.raw('<polygon points="' + " ".join(f"{x:.1f},{y:.1f}" for x, y in tri) + f'" fill="none" stroke="#111111" stroke-width="2"/>')
    c.text(60, 86, "one projected triangle, and the tiles its quad covers", "l")
    c.text(60, 466, f"{touched} tiles the polygon touches, kept", "s", fill=green)
    c.text(60, 488, f"{bounded} inside the bounds it cannot reach, discarded by covered_tile", "s", fill=pink)

    c.text(660, 86, "one flat pool, not a quota per tile", "l", fill=navy)
    px, py, pw = 660.0, 180.0, 460.0
    spans = [(0.00, 0.09, grey), (0.09, 0.15, grey), (0.15, 0.40, navy), (0.40, 0.47, grey),
             (0.47, 0.55, grey), (0.55, 0.62, grey), (0.62, 0.78, navy), (0.78, 1.00, grey)]
    for a, b, colour in spans:
        c.raw(f'<rect x="{px + pw * a:.1f}" y="{py:.1f}" width="{pw * (b - a) - 2:.1f}" height="46" fill="{colour}" fill-opacity="0.26" stroke="{colour}" stroke-width="1"/>')
    c.text(660, 166, "references: (primitive, nearest possible depth)", "s")
    c.raw(f'<line x1="{px + pw * 0.15:.1f}" y1="{py + 58:.1f}" x2="{px + pw * 0.40:.1f}" y2="{py + 58:.1f}" stroke="{navy}" stroke-width="2"/>')
    c.text(px + pw * 0.15, py + 80, "a dense tile borrows", "s", fill=navy)
    c.text(px + pw * 0.15, py + 102, "space a sparse one never used", "s", fill=navy)
    c.text(660, 340, "fs_count counts per tile, a prefix sum turns the counts", "s")
    c.text(660, 362, "into offsets, and fs_fill writes into the range it was given", "s")
    c.text(660, 466, "An ink fragment scans one tile's range,", "s", fill=yellow)
    c.text(660, 488, "never the whole scene.", "s", fill=yellow)
    c.write("tiles.svg")


def splat_resolve():
    c = Canvas("A point pass of its own, folded back into the scene",
               "Points are not drawn in the face pass. They rasterize as discs into a private pair of targets at one "
               "sample, the radius coming from the spacing the LOD walk chose; a fullscreen resolve inside the face pass "
               "shades from neighbouring depths and writes frag_depth under the scene's Greater test. That is how a "
               "cloud occludes a wall and is occluded by it.",
               1180, 480)
    pink, green, navy, yellow, grey = PAL["pink"], PAL["green"], PAL["navy"], PAL["yellow"], PAL["grey"]
    c.text(28, 40, "Two passes, one depth buffer in the end", "h")

    c.text(60, 86, "a point becomes a disc", "l", fill=navy)
    c.raw(f'<circle cx="110" cy="210" r="4" fill="{navy}"/>')
    c.text(60, 250, "one position", "s")
    c.arrow(140, 210, 196, 210)
    c.raw(f'<circle cx="250" cy="210" r="34" fill="{navy}" fill-opacity="0.34" stroke="{navy}" stroke-width="1.6"/>')
    c.raw(f'<line x1="250" y1="210" x2="284" y2="210" stroke="{yellow}" stroke-width="2"/>')
    c.text(210, 274, "radius from the spacing", "s", fill=yellow)
    c.text(210, 296, "the LOD walk chose", "s", fill=yellow)

    c.text(430, 86, "its own targets, one sample", "l", fill=pink)
    for k, (label, colour) in enumerate((("depth", pink), ("colour", pink))):
        x = 430.0 + k * 150
        c.raw(f'<rect x="{x:.1f}" y="150" width="126" height="96" fill="{colour}" fill-opacity="0.18" stroke="{colour}" stroke-width="1.3"/>')
        c.text(x + 12, 204, label, "s", fill=colour)
    c.text(430, 274, "not the face pass's attachments,", "s")
    c.text(430, 296, "and never multisampled", "s", fill=pink)

    c.text(790, 86, "resolved inside the face pass", "l", fill=green)
    c.raw(f'<rect x="790" y="150" width="300" height="96" fill="{green}" fill-opacity="0.16" stroke="{green}" stroke-width="1.3"/>')
    c.text(804, 186, "splat_resolve · fullscreen", "s", fill=green)
    c.text(804, 220, "EDL from neighbouring depths", "s")
    c.text(790, 274, "writes frag_depth under the scene's Greater test,", "s")
    c.text(790, 296, "so a cloud occludes a wall and a wall occludes it", "s", fill=green)

    c.arrow(330, 198, 420, 198)
    c.arrow(720, 198, 780, 198)
    c.text(28, 404, "The private pair exists because a point is a disc, not a triangle: it needs no multisampling and it must not pay for the face pass's.", "s")
    c.text(28, 426, "Folding it back as frag_depth is what keeps one depth buffer authoritative for the whole frame.", "s", fill=yellow)
    c.text(28, 456, "The targets are created on the first frame that has points, so a scene without a cloud never allocates them.", "s")
    c.write("splat-resolve.svg")


def pick_window():
    c = Canvas("A pick draws a window, not the canvas",
               "One click renders a 19 x 19 attachment: a 13 x 13 readback window plus a 3-cell halo, because an edge "
               "texel fits its plane from its neighbours and cleared neighbours would make a stroke pickable where it is "
               "invisible. Two uniforms carry the canvas in: origin is the attachment's top-left, frame names the canvas the "
               "tiles were binned for.",
               1180, 616)
    lav, pnk, zer, yel = PAL["blue_band"], PAL["pink_band"], PAL["zero_band"], PAL["yellow_light"]
    c.text(28, 40, "361 texels, not 1.6 million", "h")

    # Left: the canvas, with the window at its true relative size.
    c.text(28, 92, "the canvas · 1600 × 1000", "l")
    cw, ch = 320.0, 200.0
    c.parts.append(f'<rect x="28" y="110" width="{cw}" height="{ch}" rx="{RADIUS}" fill="{zer}" fill-opacity="0.22"/>')
    ux, uy = 28 + cw * 109 / 1600, 110 + ch * 259 / 1000
    c.parts.append(f'<rect x="{ux - 4:.1f}" y="{uy - 4:.1f}" width="8" height="8" fill="{yel}"/>')
    c.text(ux + 14, uy + 4, "cursor (109, 259) · 19 × 19", "s", fill=PAL["yellow"])
    c.text(28, 334, "the yellow square, at the scale of the canvas", "s")
    c.text(28, 356, "it is drawn on. That is the whole pick.", "s")

    # Centre: the attachment, blown up one cell per texel.
    gx, gy, cell = 420.0, 110.0, 13.0
    c.text(gx, 92, "the attachment · PickView { x: 100, y: 250, w: 19, h: 19 }", "l")
    for j in range(19):
        for i in range(19):
            edge = i < 3 or i > 15 or j < 3 or j > 15
            fill = zer if edge else lav
            if i == 9 and j == 9:
                fill = yel
            c.parts.append(f'<rect x="{gx + i * cell:.1f}" y="{gy + j * cell:.1f}" width="{cell - 1:.1f}" height="{cell - 1:.1f}" fill="{fill}"/>')
    gw = 19 * cell
    c.text(28, 396, "pale · PICK_HALO = 3, rendered, never copied", "s")
    c.text(28, 418, "lavender · the window, 2 × PICK_RADIUS + 1 = 13", "s", fill=PAL["navy"])
    c.text(28, 440, "yellow · the cursor texel", "s", fill=PAL["yellow"])

    # Right: everything write_pick changes.
    c.box(714, 110, ["what write_pick changes",
                     "`mvp′ = clip_transform(frame) × mvp`",
                     "`vp_w, vp_h = 19, 19`",
                     "`origin = (100, 250)`",
                     "`frame  = (1600, 1000)`",
                     "`proj_y × 1000/19 · ortho_h ÷ 1000/19`"], "note", w=386)
    c.box(714, 300, ["and which pass sees what",
                     "physical ID pass · no scissor, halo included",
                     "ink ID pass · `set_scissor_rect(3, 3, 13, 13)`",
                     "`copy_window` · ids only, 8 B a texel"], "note", w=386)

    c.box(28, 470, ["Drop the halo",
                    "an edge texel fits its plane from cleared neighbours,",
                    "so a stroke is pickable exactly where it is invisible"], "gpu", w=556)
    c.box(614, 470, ["Drop origin",
                     "the axis is looked up in tile (0, 0)'s list, and ink is",
                     "judged against triangles elsewhere on the screen"], "gpu", w=538)
    c.text(28, 600, "16 bytes a texel: ids 8 · depth 4 · metadata 4. The window costs 5.8 KB; the same target at canvas size would cost 26 MB.", "s", fill=PAL["yellow"])
    c.write("pick-window.svg")


def attachment_cost():
    c = Canvas("Where the video memory goes",
               "Three attachments per physical pixel: colour in the surface format, depth as Depth32Float, "
               "metadata as Rg16Float. That is 12 bytes a pixel at one sample and 48 at four, so the sample "
               "count is a four-times decision on the largest allocation the viewer makes - and every rule "
               "about picking, device scale and device loss is about those same bytes.",
               1180, 580)
    lav, pnk, zer = PAL["blue_band"], PAL["pink_band"], PAL["zero_band"]
    c.text(28, 40, "Forty-eight bytes, times every pixel on the glass", "h")

    x0, scale = 250.0, 12.5  # pixels per byte
    def bar(y, samples, label, inside):
        x = x0
        for name, one, fill in (("colour", 4, lav), ("depth", 4, zer), ("metadata", 4, pnk)):
            w = one * samples * scale
            c.parts.append(f'<rect x="{x:.1f}" y="{y:.1f}" width="{w:.1f}" height="46" rx="{RADIUS}" fill="{fill}"/>')
            if inside:
                c.text(x + 10, y + 20, name, "s", fill=PAL["black"], keep=True)
                c.text(x + 10, y + 38, f"{one} B × {samples} = {one * samples}", "m", fill=PAL["black"], keep=True)
            x += w + 4
        if not inside:
            c.text(x + 14, y + 30, "the same three, one sample each: 4 + 4 + 4", "s")
        c.text(28, y + 22, label, "l")
        c.text(28, y + 44, f"{12 * samples} bytes a pixel", "s")

    bar(96, 4, "4 samples", True)
    bar(168, 1, "1 sample", False)
    c.text(x0, 232, "exactly four times · this is what samples_for is spending", "s", fill=PAL["yellow"])

    c.text(28, 268, "and the canvas multiplies it", "l")
    c.text(28, 300, "1600 × 1000  = 1.6 Mpx", "m")
    c.text(300, 300, "19 MB at 1×", "m", fill=PAL["navy"])
    c.text(460, 300, "77 MB at 4×", "m", fill=PAL["pink"])
    c.text(28, 324, "3840 × 2160  = 8.3 Mpx", "m")
    c.text(300, 324, "100 MB at 1×", "m", fill=PAL["navy"])
    c.text(460, 324, "398 MB at 4×", "m", fill=PAL["pink"])

    c.text(28, 372, "Three levers hold it down", "l")
    w = 364
    c.box(28, 390, ["the pick pass draws a window",
                    "19 × 19 × 16 B = 5.8 KB, where a canvas-sized",
                    "ID target would be 26 MB"], "note", w=w)
    c.box(28 + w + 20, 390, ["device scale spends the samples",
                             "`samples_for` returns 1× from 2 physical",
                             "pixels per CSS pixel: the density has",
                             "already halved the stair-steps"], "note", w=w)
    c.box(28 + 2 * (w + 20), 390, ["a lost device reloads plainly",
                                   "once at scale 1 with antialiasing off,",
                                   "every other query kept, and the status",
                                   "line says so"], "note", w=w)
    c.text(28, 564, "Per-pixel attachments are the largest allocation the viewer makes, and the sample count multiplies all three of them at once.", "s", fill=PAL["yellow"])
    c.write("attachment-cost.svg")


def tile_pool():
    c = Canvas("The pool measures itself, one frame late",
               "The scan records how many words its lists really needed. That number is copied out after the submit and "
               "read next frame, so a pool that was too small grows one frame late. The cost is one frame of "
               "conservative ink - overflowed tiles keep rejecting the plane - never a wrong pixel.",
               1180, 592)
    c.text(28, 40, "A pool too small costs one frame, never a wrong pixel", "h")

    x0, colw, gap = 212.0, 300.0, 16.0
    cols = [x0 + i * (colw + gap) for i in range(3)]
    for x, name in zip(cols, ("frame N", "frame N + 1", "frame N + 2")):
        c.text(x, 84, name, "l")

    lanes = [
        ("GPU", 102, [
            (["project · clear · count · scan ×3 · fill", "tile 812: the cursor runs past its count", "the overflow word is set, nothing written"], "gpu"),
            (["all of it runs again", "the key was cleared, and this time", "every list fits in the new pool"], "gpu"),
            (["nothing runs at all", "ProjectionKey unchanged: same matrix,", "same geometry revision"], "note"),
        ]),
        ("an ink fragment", 224, [
            (["conservative for one frame", "an overflowed tile keeps rejecting the plane,", "so ink that should show stays hidden"], "cpu"),
            (["walk the axis pixel's tile list", "and run the finite test"], "cpu"),
            (["walk the axis pixel's tile list", "and run the finite test"], "cpu"),
        ]),
        ("CPU", 340, [
            (["copy the first record", "16 B out of the tile buffer,", "then map_report() after the submit"], "note"),
            (["prepare() reads it back", "the report reached capacity, so it is a floor:", "double, and ask again next frame"], "note"),
            (["nothing to do"], "note"),
        ]),
    ]
    for name, y, boxes in lanes:
        c.text(28, y + 26, name, "l")
        for x, (lines, kind) in zip(cols, boxes):
            c.box(x, y, lines, kind, w=colw, h=96)

    c.box(28, 456, ["One flat array, not a quota per tile",
                    "`tiles × 2 + triangles × 8` references to start, never below 32768",
                    "words and never above 32 a tile: a dense tile borrows the space a sparse one never used"], "note", w=740)
    c.box(788, 456, ["Sums saturate at capacity",
                     "so an oversubscribed pool can never",
                     "wrap into a plausible offset"], "note", w=364)
    c.text(28, 578, "The scan reports what it needed; the number comes back a frame later.", "s", fill=PAL["yellow"])
    c.write("tile-pool.svg")


def pick_modes():
    c = Canvas("What the ID pass draws in each pick mode",
               "The ID pass repeats the colour list, opaque and at one sample; each pick mode narrows it to the lanes "
               "that mode is about. A lane draws ids under the toggle that hides it in the colour frame - what a lane "
               "hides it cannot pick - except authored text, which covers geometry in every mode.",
               1180, 588)
    lav, yel, zer = PAL["blue_band"], PAL["yellow_light"], PAL["zero_band"]
    c.text(28, 40, "One list, five answers", "h")

    cols = ["face|ids", "component|ids", "splat|ids", "edge|ids", "pipe|ids", "ribbon|ids",
            "sphere|ids", "marker|dots", "control|dots", "arena|text", "authored|text", "source|ids"]
    rows = [
        ("Object", {0: "", 2: "", 4: "edges", 5: "lines", 6: "edges+", 7: "points", 9: "", 10: ""}),
        ("Edge", {0: "", 2: "", 3: "edges", 10: ""}),
        ("Component", {1: "", 2: "", 3: "edges", 10: ""}),
        ("Controls { cloud: false }", {0: "", 2: "", 8: "", 10: ""}),
        ("Controls { cloud: true }", {0: "", 2: "", 10: ""}),
        ("source query · F10", {0: "first", 2: "first", 11: ""}),
    ]
    lx, x0, cw, rh, y0 = 28.0, 262.0, 76.0, 44.0, 148.0

    # The one lane every mode draws, called out before the cells so the eye lands on it.
    c.parts.append(f'<rect x="{x0 + 10 * cw - 3:.1f}" y="{y0 - 46:.1f}" width="{cw + 6:.1f}" height="{5 * rh + 50:.1f}" rx="{RADIUS}" fill="{yel}" fill-opacity="0.22"/>')

    for i, head in enumerate(cols):
        top, bottom = head.split("|")
        c.text(x0 + i * cw + cw / 2, y0 - 28, top, "s", anchor="middle")
        c.text(x0 + i * cw + cw / 2, y0 - 10, bottom, "s", anchor="middle")

    for j, (name, drawn) in enumerate(rows):
        y = y0 + j * rh
        c.text(lx, y + rh / 2 + 5, name, "s")
        for i in range(len(cols)):
            if i not in drawn:
                continue
            fill = yel if i == 10 else lav
            c.parts.append(f'<rect x="{x0 + i * cw + 3:.1f}" y="{y + 4:.1f}" width="{cw - 6:.1f}" height="{rh - 8:.1f}" rx="4" fill="{fill}"/>')
            if drawn[i]:
                c.text(x0 + i * cw + cw / 2, y + rh / 2 + 5, drawn[i], "s", anchor="middle", fill=PAL["black"], keep=True)

    gy = y0 + 5 * rh
    c.parts.append(f'<rect x="{x0 + 3 * cw:.1f}" y="{gy + 4:.1f}" width="{7 * cw:.1f}" height="{rh - 8:.1f}" rx="4" fill="{zer}" fill-opacity="0.16"/>')
    c.text(x0 + 3 * cw + 10, gy + rh / 2 + 5, "returns before the ink pass", "s")

    c.text(28, 430, "A tag names the toggle that gates the cell: edges = show_mesh_edges · lines = show_lines · points = show_points · edges+ = both edges and markers.", "s")
    c.text(28, 452, "first = drawn only on the first page of a source query, before the accumulated ids exist.", "s")
    c.box(28, 478, ["The rule",
                    "a lane draws its ids under the same toggle that hides it in the colour frame.",
                    "What a lane hides it cannot pick, or a user selects what they cannot see."], "note", w=700)
    c.box(756, 478, ["The exception",
                     "authored text is pickable in every mode,",
                     "exactly as its visible plane covers geometry"], "sel", w=396)
    c.write("pick-modes.svg")


def cloud_pick():
    c = Canvas("The display has a residency budget; a pick does not",
               "A streamed cloud shows a bounded prefix, so a click cannot ask the screen: the nearest displayed "
               "point at a coarse level of detail is a point the user never clicked. It asks the file instead. "
               "Every octree node whose cube meets the click window is eligible, resident or not, and the answer "
               "is accumulated one page at a time against the depth the frame already has.",
               1180, 604)
    lav, pnk, zer, dark = PAL["blue_band"], PAL["pink_band"], PAL["zero_band"], "#55555e"
    c.text(28, 40, "A click asks the file, not the screen", "h")

    # Left: what is resident.
    c.text(28, 92, "what is on screen", "l")
    c.parts.append(f'<rect x="28" y="106" width="318" height="200" rx="{RADIUS}" fill="{zer}" fill-opacity="0.10"/>')
    seed = 7
    for i in range(150):
        seed = (seed * 1103515245 + 12345) % 2147483648
        x = 40 + (seed >> 7) % 294
        seed = (seed * 1103515245 + 12345) % 2147483648
        y = 118 + (seed >> 7) % 176
        c.parts.append(f'<circle cx="{x}" cy="{y}" r="2" fill="{zer}" fill-opacity="0.55"/>')
    c.parts.append(f'<rect x="150" y="186" width="40" height="40" fill="{pnk}" fill-opacity="0.42"/>')
    c.text(200, 210, "the click, ± PICK_RADIUS", "s", fill=PAL["pink"])
    c.text(28, 330, "a resident prefix: a bounded fraction", "s")
    c.text(28, 352, "of this cloud's points", "s")
    c.text(28, 382, "ask the screen and you get the nearest", "s", fill=PAL["pink"])
    c.text(28, 404, "displayed point, which at a coarse level", "s", fill=PAL["pink"])
    c.text(28, 426, "of detail is not the one that was clicked", "s", fill=PAL["pink"])

    # Centre: which nodes are eligible.
    c.text(400, 92, "what the click asks", "l")
    cells = {(0, 0): lav, (1, 0): dark, (1, 1): lav, (2, 1): dark, (2, 2): lav, (3, 2): dark}
    for (i, j), fill in cells.items():
        c.parts.append(f'<rect x="{400 + i * 78:.1f}" y="{106 + j * 66:.1f}" width="72" height="60" rx="4" fill="{fill}"/>')
    for i in range(4):
        for j in range(3):
            if (i, j) not in cells:
                c.parts.append(f'<rect x="{400 + i * 78:.1f}" y="{106 + j * 66:.1f}" width="72" height="60" rx="4" fill="{zer}" fill-opacity="0.08"/>')
    c.text(400, 330, "lavender · resident · dark · never downloaded", "s")
    c.text(400, 352, "eligible_ranges walks both", "m")
    c.text(400, 382, "kept, not rejected, when the answer is not sure:", "s")
    c.text(400, 404, "a cube crossing the eye plane, and a cube whose", "s")
    c.text(400, 426, "projection is not finite — the arithmetic that", "s")
    c.text(400, 448, "would reject it is the arithmetic that failed", "s")

    # Right: one page per frame.
    c.text(772, 92, "one page per frame", "l")
    c.box(772, 106, ["PAGE_POINTS = 65,536 points",
                     "≈ 1.5 MiB · one HTTP Range"], "gpu", w=380)
    c.box(772, 180, ["upload the page as ID targets"], "note", w=380)
    c.box(772, 236, ["the ID pass runs against the frame's",
                     "retained depth, so a candidate behind",
                     "a wall loses"], "note", w=380)
    c.box(772, 322, ["fold the nearest into the winner"], "note", w=380)
    c.text(772, 404, "the first page clears the accumulated ids, later", "s")
    c.text(772, 426, "pages load them; no colour frame is presented", "s")
    c.text(772, 448, "while a page owns the readback", "s")

    c.box(28, 486, ["QueryView freezes the click",
                    "its matrix, canvas size, pixel and radius. Every page is tested against that one",
                    "projection, never against the camera as it is now."], "note", w=560)
    c.box(616, 486, ["Cancellation is ownership",
                     "every page carries the query's generation, and dropping the Query flips its",
                     "token — nothing has to remember to clear a flag."], "note", w=536)
    c.text(28, 592, "When every eligible page has answered, two more range reads bring the winner's exact position and its original id.", "s", fill=PAL["yellow"])
    c.write("cloud-pick.svg")


def group_two():
    c = Canvas("Group 2 in three shapes",
               "Group 2 is the same slot in three shapes: two bindings for a physical lane, six for ink once it "
               "reads the scene's depth and gradient, eight once it also reads the projected triangles and the "
               "tile lists. Both sample counts are always in the layout, so the pair this frame does not use is "
               "a 1 x 1 placeholder - a bind group must satisfy every entry of its layout, read or not.",
               1180, 570)
    lav, zer, yel = PAL["blue_band"], PAL["zero_band"], PAL["yellow_light"]
    c.text(28, 40, "One layout, both sample counts", "h")

    cols = [
        (28.0, "instance layout · every physical lane",
         [("object rows · 96 B each", lav), ("anchored translations · 16 B each", lav)]),
        (400.0, "ink instance layout · this lesson",
         [("object rows", lav), ("anchored translations", lav),
          ("depth · single-sampled", lav), ("depth · multisampled", zer)]),
        (772.0, "and it grows: 05, then 18",
         [("object rows", lav), ("anchored translations", lav),
          ("depth · single-sampled", lav), ("depth · multisampled", zer),
          ("gradient · single-sampled (05)", lav), ("gradient · multisampled (05)", zer),
          ("projected triangles (18)", lav), ("tile headers + pool (18)", lav)]),
    ]
    w, rh, y0 = 380.0, 40.0, 140.0
    for x, head, rows in cols:
        c.text(x, 92, head, "l")
        for i, (name, fill) in enumerate(rows):
            y = y0 + i * rh
            c.parts.append(f'<rect x="{x:.1f}" y="{y:.1f}" width="{w:.1f}" height="{rh - 6:.1f}" rx="4" fill="{fill}"/>')
            c.text(x + 10, y + 22, str(i), "m", fill=PAL["black"], keep=True)
            c.text(x + 34, y + 22, name, "s", fill=PAL["black"], keep=True)
            if fill is zer:
                c.text(x + w - 10, y + 22, "1 × 1 placeholder", "s", anchor="end", fill=PAL["black"], keep=True)

    c.text(28, 240, "split so a re-anchor rewrites", "s")
    c.text(28, 262, "16 bytes an object, not 96", "s")
    c.box(28, 300, ["This frame is at one sample",
                    "so the multisampled views are",
                    "1 × 1 textures nobody reads. At 4×",
                    "the pair swaps: 2 and 4 become",
                    "the placeholders instead."], "sel", w=340)

    c.box(28, 474, ["A bind group must satisfy every entry of its layout, read or not.",
                    "That is why the unused sample count is a placeholder and not an absent binding — and why",
                    "flipping the sample count rebuilds bind groups rather than pipelines."], "note", w=1124)
    c.write("group-two.svg")


def side_table():
    c = Canvas("Entity id is the record index",
               "A sheet's side table is addressed by arithmetic rather than searched: record_at(id) is 8 + 16 x id, "
               "so one identity costs a 16-byte read to find the blob and one more read to fetch it. The 15 MB "
               "table is never downloaded.",
               1180, 464)
    lav, pnk, zer, yel = PAL["blue_band"], PAL["pink_band"], PAL["zero_band"], PAL["yellow_light"]
    c.text(28, 40, "Two small reads, never the 15 MB", "h")

    bx, by, bh = 40.0, 150.0, 56.0
    c.text(bx, 110, "sheet.meta · 15 MB, never downloaded whole", "l")
    c.text(bx, 138, "head · 8 B", "s")
    x = bx
    for label, w, fill in (("SHM1", 44, zer), ("count", 44, zer)):
        c.parts.append(f'<rect x="{x:.1f}" y="{by:.1f}" width="{w:.1f}" height="{bh:.1f}" fill="{fill}"/>')
        c.text(x + w / 2, by + bh / 2 + 5, label, "m", anchor="middle", fill=PAL["black"], keep=True)
        x += w + 2
    c.text(bx, by + bh + 24, "HEAD_BYTES = 8, read once a sheet and cached with its ETag · RECORD_BYTES = 16", "s")
    head_end = x + 10
    x = head_end
    for i in range(7):
        fill = yel if i == 4 else lav
        c.parts.append(f'<rect x="{x:.1f}" y="{by:.1f}" width="72" height="{bh:.1f}" fill="{fill}"/>')
        c.text(x + 36, by + bh / 2 + 5, f"rec {i}", "m", anchor="middle", fill=PAL["black"], keep=True)
        x += 74
    rec_end = x
    c.text(head_end, 138, "records · offset : u64 LE · length : u64 LE", "s")
    c.parts.append(f'<rect x="{rec_end + 10:.1f}" y="{by:.1f}" width="{1140 - rec_end - 10:.1f}" height="{bh:.1f}" rx="4" fill="{pnk}" fill-opacity="0.5"/>')
    c.text(rec_end + 24, by + bh / 2 + 5, "JSON blobs · guid · name · kind · width · colour", "s", fill=PAL["black"], keep=True)
    c.text(rec_end + 10, 138, "record_at(count)", "m", fill=PAL["pink"])

    c.box(28, 262, ["a picked ribbon names a sheet row and entity 4711"], "cpu", w=520)
    c.text(28, 330, "read 16 B at  8 + 16 × 4711 = 75,384", "m", fill=PAL["yellow"])
    c.text(28, 354, "then read  length  bytes at  offset", "m", fill=PAL["yellow"])
    # The two reads, drawn: the record cell, then the blob the record points at.
    c.arrow(300, 318, 468, 216)
    c.arrow(522, 178, 676, 178)

    c.box(596, 262, ["Three rules on those two reads",
                     "both carry the sheet's revision: a table whose ETag moved is refused, not mixed",
                     "a blob longer than 64 KiB is not one entity's record",
                     "dropping the Query cancels its callback, exactly as a cloud page does"], "note", w=556)
    c.text(28, 450, "One line of arithmetic is the whole design: the id is the index, so there is nothing to scan.", "s", fill=PAL["yellow"])
    c.write("side-table.svg")


def msaa_budget():
    c = Canvas("Where 4x is affordable",
               "Multisampling is spent only where an adapter can carry it: 4x needs solid geometry on the GPU and a "
               "canvas within this adapter's pixel budget, and lesson 17 adds a third condition, a device scale "
               "below two physical pixels per CSS pixel. In a browser the adapter never says what it is - wgpu "
               "reports DeviceType::Other for every one - so Other is its own budget, not a synonym for integrated.",
               1180, 596)
    lav, pnk, zer, yel = PAL["blue_band"], PAL["pink_band"], PAL["zero_band"], PAL["yellow_light"]
    c.text(28, 40, "One gate, then the adapter's budget", "h")

    c.box(28, 76, ["solid geometry on the GPU?",
                   "faces, pipes or spheres — ribbons, dots",
                   "and splats antialias themselves"], "note", w=430)
    c.box(490, 76, ["no → 1×"], "gpu", w=196)
    c.box(722, 76, ["lesson 17 adds a second gate",
                    "device scale below two physical",
                    "pixels per CSS pixel"], "sel", w=430)
    c.text(28, 196, "?msaa= is forced and wins over everything below.", "s", fill=PAL["yellow"])

    heads = ["DiscreteGpu", "IntegratedGpu\nVirtualGpu", "Cpu", "Other"]
    budget = ["9,000,000 px", "2,500,000 px", "never 4×", "4,200,000 px"]
    rows = [("1920 × 1080 = 2.1 Mpx", [1, 1, 0, 1]),
            ("2560 × 1440 = 3.7 Mpx", [1, 0, 0, 1]),
            ("3840 × 2160 = 8.3 Mpx", [1, 0, 0, 0])]
    lx, x0, cw, rh, y0 = 28.0, 300.0, 212.0, 46.0, 260.0
    for i, (head, b) in enumerate(zip(heads, budget)):
        x = x0 + i * cw
        for k, part in enumerate(head.split("\n")):
            c.text(x + cw / 2, y0 - 42 + k * 18, part, "s", anchor="middle")
        c.text(x + cw / 2, y0 - 8, b, "m", anchor="middle", fill=PAL["grey"])
    c.text(lx, y0 - 8, "pixels this adapter carries at 4×", "s")
    for j, (name, cells) in enumerate(rows):
        y = y0 + j * rh
        c.text(lx, y + 28, name, "m")
        for i, on in enumerate(cells):
            x = x0 + i * cw
            c.parts.append(f'<rect x="{x + 4:.1f}" y="{y + 4:.1f}" width="{cw - 8:.1f}" height="{rh - 10:.1f}" rx="4" fill="{lav if on else zer}" fill-opacity="{1.0 if on else 0.18}"/>')
            # Dark ink on the lavender fill; the empty cell's label stays light on the ground.
            c.text(x + cw / 2, y + 28, "4×" if on else "1×", "l", anchor="middle",
                   fill=PAL["black"] if on else PAL["grey"], keep=on)

    c.parts.append(f'<rect x="{x0 + 3 * cw:.1f}" y="{y0 - 62:.1f}" width="{cw:.1f}" height="{3 * rh + 66:.1f}" rx="{RADIUS}" fill="{pnk}" fill-opacity="0.16"/>')
    c.box(28, 424, ["The browser is ALWAYS 'Other'",
                    "wgpu's WebGPU backend reports `DeviceType::Other` for every adapter, because WebGPU",
                    "exposes no such field. It is a discrete GPU exactly as often as it is not — read it as",
                    "\"probably integrated\" and an ordinary 2560 × 1440 window gives up its samples for nothing."], "gpu", w=1124)
    c.text(28, 552, "Measured: discrete at 3840 × 2160 costs 8.2 ms at 4× against 6.9 at 1×. An Intel iGPU on the same scene: 108.9 against 46.5,", "s")
    c.text(28, 574, "and shrinking the canvas does not buy it back (92.4 ms at 2108 × 1186) — so a big canvas gives up the samples, not the pixels.", "s", fill=PAL["yellow"])
    c.write("msaa-budget.svg")


def tombstone():
    c = Canvas("What a removal has to remember",
               "Undo is not 'remember the object', it is 'remember every slot the object was in'. One removal "
               "empties five live tables, and the tombstone records the position it held in each: the typed "
               "list, the guid lookup, the transform, the tree with its whole subtree, and every incident graph "
               "edge. It can only be built while _detach is emptying them, because that is the last moment every "
               "one of those positions is still known.",
               1180, 574)
    lav, pnk, zer = PAL["blue_band"], PAL["pink_band"], PAL["zero_band"]
    c.text(28, 40, "Remember every slot, not the object", "h")

    tables = [
        (["typed list", "the object at obj_index in its collection,", "so order() survives a round trip"], "guid · obj · collection · obj_index"),
        (["lookup", "guid → object"], "guid"),
        (["transforms", "the local xform, or none"], "xform"),
        (["tree", "parent_guid, the index among the siblings,", "and the node with its whole subtree"], "parent_guid · index · node"),
        (["graph", "the node attribute, and every incident edge", "as (other guid, attribute, forward)"], "attribute · edges"),
    ]
    y = 96
    c.text(28, 82, "the five live tables, before the removal", "l")
    c.text(470, 82, "what the tombstone keeps of each", "l")
    for lines, fields in tables:
        r = c.box(28, y, lines, "cpu", w=420)
        c.arrow(452, y + r[3] / 2, 466, y + r[3] / 2)
        c.parts.append(f'<rect x="470" y="{y:.1f}" width="330" height="{r[3]:.1f}" rx="{RADIUS}" fill="{pnk}"/>')
        c.text(482, y + r[3] / 2 + 5, fields, "m", fill=PAL["black"], keep=True)
        y += r[3] + 10

    c.box(828, 96, ["_attach puts every one back",
                    "the same list position, the same",
                    "sibling index, the same subtree,",
                    "and the edges whose other end",
                    "still exists"], "cpu", w=324)
    c.box(828, 256, ["clone, not duplicate",
                     "a snapshot must still name the",
                     "object it stands for, and duplicate",
                     "mints a fresh guid"], "sel", w=324)
    c.box(828, 392, ["absolute, never deltas",
                     "Replace and Xform carry the before",
                     "and after values, so replaying one",
                     "cannot depend on the state it is",
                     "replayed into"], "note", w=324)

    c.text(28, 546, "The tombstone is built while _detach empties the tables: that is the only moment when every position it has to remember is still known.", "s", fill=PAL["yellow"])
    c.write("tombstone.svg")


def band_coverage():
    """04b: a stroke's alpha is the exact area of one pixel inside the band."""
    import math
    c = Canvas("A stroke's alpha is the area of one pixel square inside the band",
               "The left panel shows one pixel square crossed by a capsule's two parallel edges; the part of the square inside the band is the returned alpha. The right panel shows that area as one trapezoid CDF evaluated at hw minus d and at hw plus d.",
               1040, 560)
    c.text(28, 40, "Coverage is an area, and the area is a trapezoid", "h")

    # ---- left: one pixel, one band ----
    c.text(28, 74, "one pixel, one band", "l")
    ox, oy, cell = 56, 96, 58
    grid = []
    for i in range(5):
        for j in range(5):
            grid.append(f'<rect x="{ox+i*cell}" y="{oy+j*cell}" width="{cell}" height="{cell}" fill="none" stroke="#3a3a42" stroke-width="1"/>')
    c.raw("".join(grid))
    px, py = ox + 2 * cell, oy + 2 * cell          # the studied pixel
    ang = math.radians(20.0)
    ux, uy = math.cos(ang), -math.sin(ang)          # axis direction
    nx, ny = -uy, ux                                # unit normal
    cx, cy = px + cell * 0.5, py - cell * 0.06      # a point on the axis, above the pixel centre
    hw = cell * 0.62
    def band_poly(scale):
        far = 190.0
        pts = [(cx + ux * far + nx * hw * scale, cy + uy * far + ny * hw * scale),
               (cx - ux * far + nx * hw * scale, cy - uy * far + ny * hw * scale),
               (cx - ux * far - nx * hw * scale, cy - uy * far - ny * hw * scale),
               (cx + ux * far - nx * hw * scale, cy + uy * far - ny * hw * scale)]
        return " ".join(f"{x:.1f},{y:.1f}" for x, y in pts)
    c.raw(f'<clipPath id="cbp"><rect x="{px}" y="{py}" width="{cell}" height="{cell}"/></clipPath>')
    c.raw(f'<clipPath id="cgr"><rect x="{ox}" y="{oy}" width="{5*cell}" height="{5*cell}"/></clipPath>')
    c.raw(f'<g clip-path="url(#cgr)"><polygon points="{band_poly(1)}" fill="{PAL["pink_band"]}" opacity="0.45"/></g>')
    c.raw(f'<polygon points="{band_poly(1)}" fill="{PAL["yellow_light"]}" clip-path="url(#cbp)"/>')
    c.raw(f'<rect x="{px}" y="{py}" width="{cell}" height="{cell}" fill="none" stroke="#f4f4f6" stroke-width="2.4"/>')
    # the axis itself
    c.raw(f'<g clip-path="url(#cgr)"><path d="M{cx-ux*200:.1f},{cy-uy*200:.1f} L{cx+ux*200:.1f},{cy+uy*200:.1f}" stroke="{PAL["pink_band"]}" stroke-width="1.6" fill="none"/></g>')
    # pixel centre, and d to the axis
    mx, my = px + cell / 2, py + cell / 2
    t = (mx - cx) * nx + (my - cy) * ny
    fx, fy = mx - nx * t, my - ny * t
    c.raw(f'<circle cx="{mx:.1f}" cy="{my:.1f}" r="3.2" fill="#f4f4f6"/>')
    c.arrow(mx, my, fx, fy)
    c.text((mx + fx) / 2 + 12, (my + fy) / 2 + 4, "d", "m", fill=PAL["black"], keep=True)
    # the band width
    jx, jy = cx + ux * 84, cy + uy * 84
    c.raw(f'<path d="M{jx-nx*hw:.1f},{jy-ny*hw:.1f} L{jx+nx*hw:.1f},{jy+ny*hw:.1f}" stroke="#f4f4f6" stroke-width="1.4" fill="none"/>')
    c.text(jx + nx * hw + 10, jy + ny * hw + 16, "2 · hw", "m")
    c.text(ox, oy + 5 * cell + 26, "the yellow area IS the returned alpha, not a sample of a ramp", "s", fill=PAL["yellow"])
    c.text(ox, oy + 5 * cell + 46, "g = v / d, the unit gradient of the distance field", "m")

    # ---- right: the pixel square projected onto g ----
    bx = 430
    c.text(bx, 74, "the same pixel projected onto g", "l")
    ax0, ax1 = bx + 30, bx + 520
    base = 300
    top = 150
    hi, lo = 0.72, 0.24                       # in axis units, an example gradient
    def X(t):
        return ax0 + (t + 1.0) * (ax1 - ax0) / 2.0
    c.raw(f'<polygon points="{X(-hi):.1f},{base} {X(-lo):.1f},{top} {X(lo):.1f},{top} {X(hi):.1f},{base}" fill="{PAL["zero_band"]}"/>')
    c.raw(f'<path d="M{ax0},{base} L{ax1},{base}" stroke="#f4f4f6" stroke-width="1.4" fill="none"/>')
    c.text(ax1 + 8, base + 5, "t", "m")
    for t, lab in ((-hi, "−hi"), (-lo, "−lo"), (lo, "lo"), (hi, "hi")):
        c.raw(f'<path d="M{X(t):.1f},{base} L{X(t):.1f},{base+7}" stroke="#f4f4f6" stroke-width="1.4"/>')
        c.text(X(t), base + 24, lab, "m", anchor="middle")
    for t, lab in ((0.08, "hw − d"), (0.58, "hw + d")):
        c.raw(f'<path d="M{X(t):.1f},{top-26:.1f} L{X(t):.1f},{base:.1f}" stroke="{PAL["pink_band"]}" stroke-width="1.8" fill="none"/>')
        c.raw(f'<circle cx="{X(t):.1f}" cy="{base:.1f}" r="3.4" fill="{PAL["pink_band"]}"/>')
        c.text(X(t), top - 34, lab, "m", anchor="middle", fill=PAL["pink"])
    c.text(bx + 30, base + 60, "band_area = box_cdf(hw − d) + box_cdf(hw + d) − 1", "m")
    n = c.box(bx + 30, base + 76, ["the trapezoid is the gradient",
                                   "`hi = (|g.x| + |g.y|) / 2`   half its base",
                                   "`lo = ||g.x| − |g.y|| / 2`   half its flat top",
                                   "axis-aligned: hi = lo = 0.5, a box",
                                   "at 45°: hi = 0.7071, lo = 0, a triangle"], "note")
    c.w = max(c.w, int(n[0] + n[2] + 28))
    c.text(28, 520, "A distance ramp is sampled at pixel centres, spaced cos(angle) apart, so it beats with the line's subpixel phase: 22.2 % at a 1.5 px pen.", "s", fill=PAL["text2"])
    c.text(28, 540, "Pixel boxes tile the plane, so summing their true areas cannot beat. FILTER_REACH = 0.70711 is half a pixel's diagonal, and the quad grows by exactly that.", "s", fill=PAL["text2"])
    c.write("band-coverage.svg")


def disc_coverage():
    """04c: one radius in corner space, two primitives, one clamped ramp."""
    import math
    c = Canvas("One radius, two primitives, one clamped ramp",
               "A quad template and a triangle template both carry a corner whose length 1 is the far edge of the disc, and the antialiasing ramp is clamped to twice the ink's half width so a thin pen still reaches full opacity.",
               1060, 520)
    c.text(28, 40, "One radius, two primitives, one clamped ramp", "h")

    # ---- panel 1: corner space ----
    c.text(28, 74, "corner space", "l")
    def template(cx, cy, pts, r):
        poly = " ".join(f"{cx+x*r:.1f},{cy+y*r:.1f}" for x, y in pts)
        c.raw(f'<polygon points="{poly}" fill="none" stroke="#f4f4f6" stroke-width="1.6"/>')
        c.raw(f'<circle cx="{cx}" cy="{cy}" r="{r}" fill="none" stroke="{PAL["green"]}" stroke-width="1.6" stroke-dasharray="6 5"/>')
        c.raw(f'<circle cx="{cx}" cy="{cy}" r="{r*0.62:.1f}" fill="{PAL["pink_band"]}"/>')
        for x, y in pts:
            c.raw(f'<circle cx="{cx+x*r:.1f}" cy="{cy+y*r:.1f}" r="3.6" fill="#f4f4f6"/>')
    template(112, 196, [(-1, -1), (1, -1), (1, 1), (-1, 1)], 54)
    template(268, 178, [(0, -2), (-1.7320508, 1), (1.7320508, 1)], 54)
    c.text(112, 272, "quad · 4 corners", "s", anchor="middle")
    c.text(268, 272, "dot · 3 corners", "s", anchor="middle")
    c.text(28, 300, "|corner| = 1 is the quad's edge midpoint AND the", "s", fill=PAL["green"])
    c.text(28, 318, "triangle's incircle, so the disc fits in either.", "s", fill=PAL["green"])

    # ---- panel 2: corner to pixels ----
    cx0 = 400
    c.text(cx0, 74, "corner to pixels", "l")
    l1 = ["clip offset", "`corner * (px + 0.5 * feather)`", "`* 2 / (vp_w, vp_h) * clip.w`"]
    l2 = ["pixels from the centre", "`d = |corner| * (px + 0.5 * feather)`"]
    cwide = max(c.natural(l1), c.natural(l2))
    b1 = c.box(cx0, 92, l1, "gpu", w=cwide)
    b2 = c.box(cx0, b1[1] + b1[3] + 26, l2, "gpu", w=cwide)
    c.arrow(cx0 + b1[2] / 2, b1[1] + b1[3], cx0 + b1[2] / 2, b2[1])
    b3 = c.box(cx0, b2[1] + b2[3] + 16, ["px = to_px(radius, clip.w)",
                                         "floored at 0.5 px",
                                         "wider than the viewport: parked", "outside the clip volume"], "note", w=b1[2])

    # ---- panel 3: the ramp ----
    gx0 = cx0 + b1[2] + 56
    c.text(gx0, 74, "the ramp", "l")
    left, right = gx0 + 34, gx0 + 300
    base, topy = 300, 120
    def PX(d):
        return left + d * (right - left) / 3.0
    def PY(a):
        return base - a * (base - topy)
    c.raw(f'<path d="M{left},{base} L{right},{base}" stroke="#f4f4f6" stroke-width="1.4" fill="none"/>')
    c.raw(f'<path d="M{left},{base} L{left},{topy-10}" stroke="#f4f4f6" stroke-width="1.4" fill="none"/>')
    c.raw(f'<path d="M{left},{PY(1):.1f} L{right},{PY(1):.1f}" stroke="#55555f" stroke-width="1" stroke-dasharray="4 4" fill="none"/>')
    for d in (1, 2, 3):
        c.raw(f'<path d="M{PX(d):.1f},{base} L{PX(d):.1f},{base+6}" stroke="#f4f4f6" stroke-width="1.2"/>')
        c.text(PX(d), base + 22, str(d), "m", anchor="middle")
    c.text(right + 8, base + 5, "d px", "s")
    c.text(left - 8, PY(1) + 4, "1", "m", anchor="end")
    c.text(left - 8, base + 4, "0", "m", anchor="end")
    # clamped: f = 1.5, px = 0.75 -> alpha 1 until d = 0, zero at 1.5
    c.raw(f'<path d="M{PX(0):.1f},{PY(1):.1f} L{PX(0.75):.1f},{PY(1):.1f} L{PX(1.5):.1f},{PY(0):.1f}" stroke="{PAL["green"]}" stroke-width="2.4" fill="none"/>')
    # unclamped: f = 4, alpha(0) = 0.6875, zero at 2.75
    c.raw(f'<path d="M{PX(0):.1f},{PY(0.6875):.1f} L{PX(2.75):.1f},{PY(0):.1f}" stroke="{PAL["orange"]}" stroke-width="2.2" stroke-dasharray="7 5" fill="none"/>')
    c.text(PX(0.1), PY(1) - 12, "f = min(feather, 2 · px)", "m", fill=PAL["green"])
    c.text(PX(0.55), PY(0.60) + 2, "f = feather, unclamped", "m", fill=PAL["orange"])
    c.raw(f'<path d="M{PX(0.05):.1f},{PY(0.6875):.1f} L{PX(0.05):.1f},{PY(1):.1f}" stroke="{PAL["orange"]}" stroke-width="1.4" fill="none"/>')
    c.text(PX(0.12), PY(0.84) + 4, "never reaches 1", "s", fill=PAL["orange"])
    note = c.box(gx0, base + 46, ["px = 0.75, feather = 4",
                                  "`clamped   f = 1.5  → alpha(0) = 1.00`",
                                  "`unclamped f = 4.0  → alpha(0) = 0.69`",
                                  "the fade always ends inside the quad"], "note", w=right - gx0 + 34)
    c.text(28, 470, "alpha = clamp((px + 0.5 · f − d) / f, 0, 1); a free dot then multiplies by its hairline fade, floored at HAIRLINE_MIN_ALPHA = 0.5.", "m")
    c.h = 500
    c.w = max(c.w, int(note[0] + note[2] + 28))
    c.write("disc-coverage.svg")


def carry_verdict():
    """05: where a carried prediction has to land on the depth line."""
    c = Canvas("Where a carried ink depth has to land",
               "Two reverse-Z depth lines. When the texel under the fragment is farther than the ink's axis the test is one sided; when it is nearer the predicted depth must fall inside the tolerance band on both sides, so a surface that does not pass through the axis cannot uncover covered ink.",
               1040, 560)
    c.text(28, 40, "Where the predicted depth has to land", "h")

    def line(y, title, note, z_left, window):
        c.text(28, y - 76, title, "l")
        x0, x1 = 60, 700
        c.raw(f'<path d="M{x0},{y} L{x1},{y}" stroke="#f4f4f6" stroke-width="1.6" fill="none"/>')
        c.text(x0, y + 26, "far · 0.0 · the clear value", "s", fill=PAL["text2"])
        c.text(x1, y + 26, "near · 1.0", "s", anchor="end", fill=PAL["text2"])
        axis = (x0 + x1) / 2 + 40
        tol = 46
        if window:
            c.raw(f'<rect x="{axis-tol}" y="{y-16}" width="{2*tol}" height="32" fill="{PAL["green"]}" opacity="0.34"/>')
        else:
            c.raw(f'<rect x="{x0}" y="{y-16}" width="{axis+tol-x0}" height="32" fill="{PAL["green"]}" opacity="0.34"/>')
        c.raw(f'<path d="M{axis},{y-22} L{axis},{y+22}" stroke="#f4f4f6" stroke-width="2"/>')
        c.text(axis, y - 58, "axis.depth", "m", anchor="middle")
        for s in (-1, 1):
            c.raw(f'<path d="M{axis+s*tol},{y-12} L{axis+s*tol},{y+12}" stroke="#f4f4f6" stroke-width="1.2"/>')
        zx = axis - 150 if z_left else axis + 120
        c.raw(f'<circle cx="{zx}" cy="{y}" r="6" fill="{PAL["blue_band"]}"/>')
        c.text(zx, y - 34, "z · the depth already there", "s", anchor="middle", fill=PAL["blue_band"])
        px = axis + (28 if z_left else -20)
        c.raw(f'<circle cx="{px}" cy="{y}" r="6" fill="{PAL["green"]}"/>')
        c.text(px + 14, y + 66, "predicted", "s", fill=PAL["green"])
        c.arrow(zx + 8, y + 26, px - 8, y + 26, "+ gradient · (axis.at − pixel)", above=False)
        return c.box(760, y - 48, note, "note", w=252)

    c.text(28, 64, "reverse-Z: the buffer clears to 0.0 and nearer is greater →", "s", fill=PAL["text2"])
    line(170, "the texel is FARTHER than the axis   z ≤ depth",
         ["one-sided", "`predicted ≤ depth + tolerance`", "ink still overhangs a", "silhouette at full width"], True, False)
    line(370, "the texel is NEARER   z > depth + |depth| · 2⁻¹⁹",
         ["two-sided", "`|predicted − depth| ≤ tolerance`", "the surface must pass", "THROUGH the axis"], False, True)

    tb = c.box(28, 470, ["tolerance",
                         "`|depth| · 1.9073486e-6  +  |slope| · 0.00390625 · (1 + lever)`",
                         "2⁻¹⁹: about 16 ULPs of the depth",
                         "2⁻⁸: the rasterizer snaps vertices to 1/256 px, so a fitted slope is",
                         "wrong by that much, times the lever it was carried across"], "note", w=984)
    c.text(28, tb[1] + tb[3] + 30, "One verdict, three callers: the plane carry, the neighbouring-pair carry and the disc's two-axis carry all end here.", "s", fill=PAL["yellow"])
    c.h = int(tb[1] + tb[3] + 52)
    c.write("carry-verdict.svg")


def depth_modes():
    """04a: six depth modes are three compare functions crossed with the write flag."""
    c = Canvas("The six depth modes are three compare functions crossed with the write flag",
               "A matrix whose rows are write and no write and whose columns are Greater, GreaterEqual and Always. Five cells hold a named mode and the pair that state() returns for it; the sixth is empty because a pass that always passes has nothing to write, and Detached sits off the matrix because it has no depth attachment at all.",
               1060, 520)
    c.text(28, 40, "Three compares × one write flag", "h")
    cols = [("Greater", "strictly nearer wins"), ("GreaterEqual", "ties are kept"), ("Always", "no test")]
    cw, ch = 262, 122
    x0, y0 = 176, 130
    for i, (name, sub) in enumerate(cols):
        c.text(x0 + i * cw + (cw - 24) / 2, y0 - 36, name, "l", anchor="middle")
        c.text(x0 + i * cw + (cw - 24) / 2, y0 - 16, sub, "s", anchor="middle")
    c.text(x0 - 18, y0 + ch / 2 + 5, "write", "l", anchor="end")
    c.text(x0 - 18, y0 + ch + 24 + ch / 2 + 5, "no write", "l", anchor="end")
    cells = [[("Opaque", "`(true, Greater)`", "solids and the depth prepass"),
              ("OpaqueEqual", "`(true, GreaterEqual)`", "source-point queries: a", "resident point ties with itself"),
              None],
             [("ReadOnly", "`(false, Greater)`", "sheet fills and the grid"),
              ("ReadOnlyEqual", "`(false, GreaterEqual)`", "blended ink, tying with its own", "prepass and with faces"),
              ("Always", "`(false, Always)`", "the background")]]
    for r, rowcells in enumerate(cells):
        for i, cell in enumerate(rowcells):
            x, y = x0 + i * cw, y0 + r * (ch + 24)
            if cell is None:
                c.raw(f'<rect x="{x}" y="{y}" width="{cw-24}" height="{ch}" rx="{RADIUS}" fill="#23232a"/>')
                c.text(x + 16, y + 40, "no caller", "s", fill=PAL["grey"])
                c.text(x + 16, y + 62, "a pass that always passes", "s", fill=PAL["grey"])
                c.text(x + 16, y + 84, "has nothing to write", "s", fill=PAL["grey"])
                continue
            c.box(x, y, list(cell), "gpu", w=cw - 24, h=ch)
    ny = y0 + 2 * (ch + 24) + 6
    note = c.box(28, ny, ["one pair, not two enums", "`fn state(self) -> (bool, CompareFunction)`"], "note")
    d = c.box(note[0] + note[2] + 24, ny, ["Detached · the sixth mode, off the matrix",
                                           "no depth attachment at all, so nothing to compare against:",
                                           "a full-screen pass over a texture"], "note")
    c.text(28, d[1] + d[3] + 32, "Reverse-Z: the depth buffer clears to 0.0 and nearer is greater, so Greater is the ordinary solid test and GreaterEqual is the one that lets a tie through.", "s", fill=PAL["yellow"])
    c.w = max(c.w, int(d[0] + d[2] + 28), int(x0 + 3 * cw + 4))
    c.h = int(d[1] + d[3] + 54)
    c.write("depth-modes.svg")


def ink_thresholds():
    """06: one angle sorts a shared edge into ink or nothing, and the thresholds are ordered."""
    import math
    c = Canvas("One angle decides whether a shared edge gets ink",
               "The cosine between two face normals sorts a shared edge into one of three verdicts. The thresholds sit in an order that has to hold: the mesher may turn 5 degrees between samples, so a sampling seam can never reach the 25 degree crease test, and the packed normal code is only good to 1.4 degrees, so the test runs on the walk's f64 normals.",
               1120, 620)
    c.text(28, 40, "One angle decides: does this edge get ink?", "h")
    ax0, ax1, ay = 176, 1092, 232
    lo, hi = math.log10(0.001), math.log10(180.0)
    def X(deg):
        return ax0 + (math.log10(max(deg, 0.001)) - lo) * (ax1 - ax0) / (hi - lo)
    c.raw(f'<path d="M{ax0},{ay} L{ax1},{ay}" stroke="#f4f4f6" stroke-width="1.6" fill="none"/>')
    for d in (0.001, 0.01, 0.1, 1, 10, 100):
        c.raw(f'<path d="M{X(d):.1f},{ay} L{X(d):.1f},{ay+7}" stroke="#6a6a74" stroke-width="1.2"/>')
        c.text(X(d), ay + 24, f"{d:g}", "s", anchor="middle", fill=PAL["grey"])
    c.text(ax1, ay + 48, "degrees between the two faces' outward normals", "s", anchor="end", fill=PAL["text2"])
    ticks = [(0.0026, PAL["navy"], "`COPLANAR_DOT = 1.0 − 1e-9`", "0.0026°: below this the faces are one flat region"),
             (1.4, PAL["grey"], "oct16 quantisation", "1.4°: the smallest turn the packed code can see"),
             (5.0, PAL["green"], "`QUALITY = (5.0, 0.001)`", "5°: the most a tessellation may turn between samples"),
             (25.0, PAL["pink"], "`CREASE_COS = 0.906_307_787`", "25°: a genuine fold")]
    for i, (deg, col, top, sub) in enumerate(ticks):
        x = X(deg)
        shown = ON_BLACK.get(col, col)
        c.raw(f'<path d="M{x:.1f},{ay} L{x:.1f},{ay-34}" stroke="{shown}" stroke-width="2.6"/>')
        ty = 96 if i % 2 == 0 else 150
        c.text(x, ty, top.replace("`", ""), "m" if top.startswith("`") else "s", anchor="middle", fill=col)
        c.text(x, ty + 18, sub, "s", anchor="middle", fill=PAL["text2"])
        c.raw(f'<path d="M{x:.1f},{ty+26:.1f} L{x:.1f},{ay-36:.1f}" stroke="#4a4a54" stroke-width="1"/>')
    # the two verdict bands
    def band(y, name, segs):
        c.text(28, y + 26, name, "s", fill=PAL["blue_band"])
        for a, b, fill, label in segs:
            xa, xb = X(a), X(b)
            c.raw(f'<rect x="{xa:.1f}" y="{y}" width="{xb-xa:.1f}" height="38" rx="4" fill="{fill}"/>')
            if label:
                c.text((xa + xb) / 2, y + 24, label, "s", anchor="middle", fill=PAL["black"], keep=True)
    band(300, "authored Mesh", [(0.001, 0.0026, PAL["zero_band"], ""),
                                (0.0026, 180, PAL["blue_band"], "ink: every edge the author put there")])
    band(366, "tessellation", [(0.001, 0.0026, PAL["zero_band"], ""),
                               (0.0026, 25, PAL["zero_band"], "dropped: this is the sampling grid, not the shape"),
                               (25, 180, PAL["pink_band"], "ink: a crease")])
    c.raw(f'<path d="M{X(25):.1f},{300} L{X(25):.1f},{404}" stroke="{PAL["pink_band"]}" stroke-width="1.6" stroke-dasharray="6 5" fill="none"/>')
    c.text(X(0.0026) - 6, 438, "VIEWER_ALL_EDGES re-opens the flat sliver", "m", fill=PAL["orange"])
    c.text(X(25) - 6, 458, "VIEWER_SEAMS re-opens the grid", "m", anchor="end", fill=PAL["orange"])
    b1 = c.box(28, 480, ["A border has no angle to test",
                         "`edge_faces[ei][1] == u32::MAX`",
                         "one face only, so the verdict is ink before any cosine is",
                         "computed: at any angle, in both bands"], "sel")
    b2 = c.box(b1[0] + b1[2] + 24, 480, ["Why the order of the ticks is the argument",
                                         "5° is the most the mesher may turn between samples, so a sampling seam",
                                         "cannot reach 25°: the crease test never mistakes the grid for the shape",
                                         "1.4° is the packed code's error, so the test runs on the walk's f64",
                                         "normals and never on the codes the shader reads"], "note")
    c.w = max(c.w, int(b2[0] + b2[2] + 28))
    c.h = int(b2[1] + b2[3] + 32)
    c.write("ink-thresholds.svg")


def three_normals():
    """09: three normals at one vertex, and what each is allowed to decide."""
    import math
    c = Canvas("Three normals at one vertex, and what each is allowed to decide",
               "A shading normal is an average chosen to make a tessellation look smooth. A cone's apex fan averages to straight up, so asking that average whether the seam faces away answered yes from every side and the seam vanished. The cull indexes each triangle's own normal by the exact bits of its edge endpoints instead.",
               1120, 560)
    c.text(28, 40, "Three normals at one vertex, and what each decides", "h")

    # ---- left: the cone and its seam ----
    c.text(40, 76, "the cone", "l")
    apex = (188, 110)
    bl, br, by = 96, 280, 300
    c.raw(f'<path d="M{apex[0]},{apex[1]} L{bl},{by} A 92 26 0 0 0 {br},{by} Z" fill="{PAL["blue_band"]}" opacity="0.30" stroke="#9a9aa4" stroke-width="1.2"/>')
    c.raw(f'<ellipse cx="188" cy="{by}" rx="92" ry="26" fill="none" stroke="#9a9aa4" stroke-width="1.2" stroke-dasharray="5 4"/>')
    c.raw(f'<path d="M{apex[0]},{apex[1]} L{188+52},{by+18}" stroke="#f4f4f6" stroke-width="3"/>')
    c.text(258, 340, "the seam", "s")
    c.raw(f'<circle cx="{apex[0]}" cy="{apex[1]}" r="30" fill="none" stroke="#6a6a74" stroke-width="1.2" stroke-dasharray="5 4"/>')
    c.arrow(apex[0] + 32, apex[1] + 6, 372, 150, "zoom")

    # ---- middle: the apex fan ----
    fx, fy = 520, 190
    c.text(392, 76, "the apex, enlarged", "l")
    fan = []
    for k in range(4):
        a0 = math.radians(200 + k * 35)
        a1 = math.radians(200 + (k + 1) * 35)
        p0 = (fx + 110 * math.cos(a0), fy + 110 * math.sin(a0))
        p1 = (fx + 110 * math.cos(a1), fy + 110 * math.sin(a1))
        fan.append((p0, p1))
        c.raw(f'<polygon points="{fx},{fy} {p0[0]:.1f},{p0[1]:.1f} {p1[0]:.1f},{p1[1]:.1f}" fill="{PAL["blue_band"]}" opacity="0.55" stroke="#9a9aa4" stroke-width="0.9"/>')
    seam = fan[1][1]
    c.raw(f'<path d="M{fx},{fy} L{seam[0]:.1f},{seam[1]:.1f}" stroke="#f4f4f6" stroke-width="3"/>')
    c.arrow(fx, fy, fx, fy - 84)
    c.text(fx + 128, fy - 96, "shading normal", "s", fill=PAL["green"])
    c.text(fx + 128, fy - 78, "the fan's mean · lighting only", "s", fill=PAL["green"])
    for (p0, p1), lean in ((fan[1], -1), (fan[2], 1)):
        cx_ = (fx + p0[0] + p1[0]) / 3
        cy_ = (fy + p0[1] + p1[1]) / 3
        c.raw(f'<path d="M{cx_:.1f},{cy_:.1f} L{cx_+lean*54:.1f},{cy_+22:.1f}" stroke="{PAL["pink_band"]}" stroke-width="2.6" fill="none"/>')
        c.raw(f'<circle cx="{cx_+lean*54:.1f}" cy="{cy_+22:.1f}" r="4" fill="{PAL["pink_band"]}"/>')
    c.text(392, 326, "facet normals: each triangle's own cross product,", "s", fill=PAL["pink"])
    c.text(392, 344, "turned outward by the face sign · the facing cull", "s", fill=PAL["pink"])
    c.text(392, 368, "oct16 → facing word (1.4°) · what the shader compares", "m")

    # ---- right: the consequence ----
    c.text(860, 76, "the same seam, two culls", "l")
    w1 = c.box(860, 96, ["cull with the shading normal",
                         "both average to the same up vector",
                         "→ both face away → the seam is culled",
                         "and disappears"], "warn", w=292)
    c.raw(f'<rect x="860" y="96" width="292" height="{w1[3]:.1f}" rx="{RADIUS}" fill="none" stroke="{PAL["orange"]}" stroke-width="1.8" stroke-dasharray="7 5"/>')
    c.box(860, w1[1] + w1[3] + 22, ["cull with the facet normals",
                                    "the two triangles splay left and right",
                                    "→ one of them faces the eye",
                                    "→ the seam is drawn"], "cpu", w=292)
    n = c.box(28, 402, ["The refusals built into the lookup",
                        "the key is the two endpoint positions' exact bits, smallest first, so winding cannot change the answer",
                        "a periodic face's seam has both its triangles in one mesh and keeps both",
                        "more than two incident facets, or a facet missing on the lending face, returns FACING_UNKNOWN — always draw — rather than a guess"], "note")
    c.w = max(c.w, int(n[0] + n[2] + 28), 1180)
    c.h = int(n[1] + n[3] + 32)
    c.write("three-normals.svg")


def frame_passes():
    """12: the frame as six passes, and which one owns the physical depth."""
    c = Canvas("One frame, six passes, and who touches each attachment",
               "A matrix of the six render passes encode_frame records against the five attachments they can touch. The physical depth is cleared and written by the face pass alone; every later pass either attaches it read-only or samples it, and the pick passes use a second depth of their own.",
               1200, 620)
    c.text(28, 40, "One frame, six passes", "h")
    c.text(28, 62, "in the order encode_frame records them", "s", fill=PAL["text2"])
    lx, lw = 28, 244
    cx0, cwid, gap = 292, 166, 8
    hy, ry, rh, rgap = 106, 156, 54, 10
    cols = [("surface", "colour"), ("physical", "Depth32Float"), ("gradient", "Rg16Float"),
            ("coverage", "R8Unorm"), ("ID", "Rg32Uint")]
    for i, (a, b) in enumerate(cols):
        x = cx0 + i * (cwid + gap) + cwid / 2
        c.text(x, hy, a, "l", anchor="middle")
        c.text(x, hy + 18, b, "m", anchor="middle", fill=PAL["text2"])
    # the depth column's highlight, drawn under the cells
    dx = cx0 + 1 * (cwid + gap)
    c.raw(f'<rect x="{dx-6}" y="{ry+rh+rgap-6}" width="{cwid+12}" height="{4*(rh+rgap)+6}" rx="8" fill="{PAL["yellow_light"]}" opacity="0.16"/>')
    rows = [
        ("`splat.points`", "only when the cloud, camera or knobs moved", None),
        ("`face pass`", "backdrop, grid, faces, cloud resolve",
         [("w", "Clear → Store"), ("w", "Clear 0.0 → Store"), ("w", "Clear → Store"), ("-", ""), ("-", "")]),
        ("`selection coverage`", "only when a selected face is visible",
         [("-", ""), ("r", "attached read-only"), ("-", ""), ("w", "Clear → Store"), ("-", "")]),
        ("`ink pass`", "the scene list, resolved at 4×",
         [("w", "Load → Store"), ("r", "read-only AND sampled"), ("r", "sampled"), ("r", "sampled"), ("-", "")]),
        ("`pick pass`", "only when a pick is pending",
         [("-", ""), ("w", "its own · Clear 0.0"), ("w", "its own · Clear"), ("-", ""), ("w", "Clear → Store")]),
        ("`pick ink`", "the same list, narrowed by mode",
         [("-", ""), ("r", "its own, read-only"), ("r", "its own, sampled"), ("-", ""), ("w", "Load → Store")]),
    ]
    fill = {"w": (PAL["pink_band"], 1.0), "r": ("#6b2f4a", 1.0), "-": ("#23232a", 1.0)}
    y = ry
    for label, gloss, cells in rows:
        c.text(lx, y + 22, label.replace("`", ""), "m")
        c.text(lx, y + 42, gloss, "s", fill=PAL["text2"])
        if cells is None:
            w = 5 * cwid + 4 * gap
            c.raw(f'<rect x="{cx0}" y="{y}" width="{w}" height="{rh}" rx="6" fill="{PAL["zero_band"]}" opacity="0.5"/>')
            c.text(cx0 + w / 2, y + 24, "its own private pair, one sample always:", "s", anchor="middle", fill=PAL["black"], keep=True)
            c.text(cx0 + w / 2, y + 42, "Rgba8Unorm colour + Depth32Float, cleared transparent and 0.0", "s", anchor="middle", fill=PAL["black"], keep=True)
        else:
            for i, (kind, txt) in enumerate(cells):
                x = cx0 + i * (cwid + gap)
                col, op = fill[kind]
                c.raw(f'<rect x="{x}" y="{y}" width="{cwid}" height="{rh}" rx="6" fill="{col}" opacity="{op}"/>')
                if not txt:
                    c.text(x + cwid / 2, y + rh / 2 + 5, "—", "s", anchor="middle", fill=PAL["grey"])
                    continue
                words, line, lines = txt.split(" "), "", []
                for word in words:
                    trial = (line + " " + word).strip()
                    if width(trial, "s") > cwid - 18:
                        lines.append(line); line = word
                    else:
                        line = trial
                lines.append(line)
                ty = y + rh / 2 + 5 - (len(lines) - 1) * 9
                for ln in lines:
                    c.text(x + cwid / 2, ty, ln, "s", anchor="middle",
                           fill=PAL["black"] if kind == "w" else None, keep=(kind == "w"))
                    ty += 18
        y += rh + rgap
    c.text(dx + cwid / 2, y + 8, "one writer", "s", anchor="middle", fill=PAL["yellow"])
    lg = y + 34
    for i, (kind, name) in enumerate((("w", "written"), ("r", "read"), ("-", "untouched"))):
        x = lx + i * 150
        col, op = fill[kind]
        c.raw(f'<rect x="{x}" y="{lg}" width="26" height="16" rx="4" fill="{col}" opacity="{op}"/>')
        c.text(x + 34, lg + 13, name, "s", fill=PAL["text2"])
    c.text(lx, lg + 68, "The physical depth is cleared and written by the face pass alone; the two passes after it attach or sample it, and the pick passes carry a second depth of their own.", "s", fill=PAL["yellow"])
    c.text(lx, lg + 44, "then copy_window: copy_texture_to_buffer, 8 B a texel → queue.submit → pick.map → output.present", "s", fill=PAL["text2"])
    c.w, c.h = int(cx0 + 5 * cwid + 4 * gap + 28), int(lg + 92)
    c.write("frame-passes.svg")


def producer_contract():
    """06: what crosses a producer's edge, and the one index it has to get right."""
    c = Canvas("What crosses a producer's edge",
               "A producer is handed one kernel geometry, the row it is filling and the vertex base its indices must start from. It appends to the lane tables and reports one Row measured in the object's own space. The file, the document, the camera and the selection are not in scope.",
               1200, 600)
    c.text(28, 40, "What crosses a producer's edge", "h")
    # the fence
    fence = ["the file", "the document", "the camera", "the selection"]
    fx = 340
    for i, name in enumerate(fence):
        w = width(name, "s") + 26
        c.raw(f'<rect x="{fx}" y="70" width="{w:.1f}" height="26" rx="6" fill="#2a2a32"/>')
        c.text(fx + w / 2, 88, name, "s", anchor="middle", fill=PAL["grey"])
        c.raw(f'<path d="M{fx+6},{92} L{fx+w-6:.1f},{74}" stroke="#6a6a74" stroke-width="1.4"/>')
        fx += w + 12
    c.text(fx + 6, 88, "never in scope", "s", fill=PAL["grey"])

    mid = c.box(340, 116, ["one producer",
                           "`walk_mesh(arena, ink, m, mc)`",
                           "`walk_brep(arena, ink, brep, cx)`",
                           "`walk_line(seg, l, row)`",
                           "one geometry, one row, nothing else"], "note", w=372)
    a = c.box(28, 120, ["the geometry", "`&Mesh  &BRep  &NurbsCurve`", "f64, the kernel's own"], "cpu")
    b = c.box(28, a[1] + a[3] + 24, ["WalkCx · the row it is filling",
                                     "`vert_base` arena rows already on the GPU",
                                     "`row` this object's row",
                                     "`cloud_px` the file's point-size override"], "cpu")
    c.arrow(a[0] + a[2], a[1] + a[3] / 2, mid[0], a[1] + a[3] / 2)
    c.arrow(b[0] + b[2], b[1] + b[3] / 2, mid[0], b[1] + b[3] / 2)
    out = c.box(mid[0] + mid[2] + 44, 140, ["Row · what it reports",
                                            "`bounds` an Aabb in the object's OWN space",
                                            "`spacing` diagonal / √vertices",
                                            "`flags` PRINT · SMOOTH · SINGLE · OPEN",
                                            "`faces` did it draw triangles?"], "cpu")
    c.arrow(mid[0] + mid[2], out[1] + 40, out[0], out[1] + 40)
    c.text(out[0], out[1] + out[3] + 20, "Row::thin: a box and nothing else, for linework and points", "s", fill=PAL["text2"])

    ty = mid[1] + mid[3] + 40
    tabs = [("`ArenaRows`", "verts · vids · idx"), ("`SegRows`", "pipes · ribbons"), ("`GlyphRows`", "spheres · dots")]
    tx = 340
    for label, body in tabs:
        r = c.box(tx, ty, [label.replace("`", ""), body], "gpu")
        c.arrow(r[0] + r[2] / 2, r[1], r[0] + r[2] / 2, mid[1] + mid[3] + 8)
        tx = r[0] + r[2] + 16
    c.text(340, ty + 90, "append-only: a producer never reads a table back", "s", fill=PAL["text2"])

    ruler_x = out[0]
    cap_y = out[1] + out[3] + 56
    c.text(ruler_x, cap_y, "arena.verts · one index space", "s")
    zones = [(PAL["yellow_light"], "this object", 70), (PAL["blue_band"], "earlier objects of this file", 62), ("#3a3a44", "already uploaded", 52)]
    zy = cap_y + 14
    for colr, name, hgt in zones:
        c.raw(f'<rect x="{ruler_x}" y="{zy}" width="44" height="{hgt}" rx="4" fill="{colr}"/>')
        c.text(ruler_x + 56, zy + hgt / 2 + 5, name, "s", fill=PAL["text2"])
        zy += hgt + 6
    c.text(ruler_x, zy + 26, "base = cx.vert_base + arena.verts.len()", "m")
    c.text(ruler_x, zy + 46, "idx.push(base + i) — wrong here is silent", "s", fill=PAL["yellow"])
    c.w = int(max(ruler_x + 320, out[0] + out[2] + 28))
    c.h = int(zy + 70)
    c.write("producer-contract.svg")


if __name__ == "__main__":
    for draw in (spaces, gpu_data, ink_visibility, picking, text_pipeline, vertex_layout, ownership, frame, finite_triangle, first_frame, cad_contract, shared_boundary, trims_seams, normals, shaping, text_placement, controls, loading, metadata_window, source_cache, joins, ribbon, markers, lod, arena, stages, interpolate, frustum, camera_basis, masks, device_scale, toolchain, gpu_objects, clip_space, instancing, cpu_gpu, loop, section_plane, three_declarations, sheet_cost, history, tiles, splat_resolve, pick_window, attachment_cost, tile_pool, pick_modes, cloud_pick, group_two, side_table, msaa_budget, tombstone, band_coverage, disc_coverage, carry_verdict, depth_modes, ink_thresholds, three_normals, frame_passes, producer_contract):
        draw()
    print(f'wrote {len(list(HERE.glob("*.svg")))} illustrations')
