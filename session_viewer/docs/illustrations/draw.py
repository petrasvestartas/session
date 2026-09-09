#!/usr/bin/env python3
"""Generate the course illustrations as SVG from a layout that sizes every box from its text.

Palette: the BRG Equilibrium drawing library (navy compression, pink tension, green loads,
yellow hover, orange inspector, pale bands). Run `python3 docs/illustrations/draw.py`, then
`node docs/check_illustrations.cjs` measures every label in Chrome and fails on any overflow.
"""
from pathlib import Path

HERE = Path(__file__).resolve().parent

PAL = {
    "navy": "#1a1eb2", "pink": "#ce4095", "green": "#3f9c20", "yellow": "#e8ac00",
    "yellow_light": "#f9e08a", "orange": "#e07a26", "ghost": "#9ed4c9", "grey": "#aaaaaa",
    "zero": "#b9b9bd", "pink_band": "#f0bcdb", "blue_band": "#bdbfe8", "zero_band": "#e4e4e7",
    "black": "#111111", "text2": "#455b6b", "page": "#eef0f2", "white": "#ffffff",
}
# Box kinds: (fill, stroke). CPU/Rust = blue band, GPU/WGSL = pink band, note = zero band,
# selection = yellow light, warning = orange stroke.
KIND = {
    "cpu": (PAL["blue_band"], PAL["navy"]),
    "gpu": (PAL["pink_band"], PAL["pink"]),
    "note": (PAL["zero_band"], PAL["zero"]),
    "sel": (PAL["yellow_light"], PAL["yellow"]),
    "warn": (PAL["white"], PAL["orange"]),
    "plain": (PAL["white"], PAL["grey"]),
}
# Average glyph advance per em, calibrated against Chrome on the build host; the checker
# writes `textLength` from real measurements so other machines cannot overflow either.
EM = {"sans": 0.53, "sansb": 0.59, "mono": 0.605}
SIZE = {"h": 20, "l": 15, "s": 13, "m": 12.5}
FONT = {"sans": "system-ui, sans-serif", "mono": '"DejaVu Sans Mono", "Liberation Mono", monospace'}
STYLE = (
    "text{font-family:system-ui,sans-serif;fill:#111111;font-size:13px}"
    ".h{font-size:20px;font-weight:700}.l{font-size:15px;font-weight:650}"
    ".s{font-size:13px;fill:#455b6b}.m{font-family:\"DejaVu Sans Mono\",\"Liberation Mono\",monospace;font-size:12.5px;fill:#111111}"
    ".ar{fill:none;stroke:#111111;stroke-width:1.6;marker-end:url(#a)}"
    ".dash{fill:none;stroke:#e07a26;stroke-width:1.8;stroke-dasharray:7 5}"
)
MARKER = ('<defs><marker id="a" markerWidth="9" markerHeight="9" refX="7" refY="3" orient="auto">'
          '<path d="M0,0 L7,3 L0,6" fill="#111111"/></marker></defs>')


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

    def text(self, x, y, s, cls="s", anchor="start", fill=None, box=None):
        extra = f' style="fill:{fill}"' if fill else ""
        attr = f' text-anchor="{anchor}"' if anchor != "start" else ""
        data = f' data-box="{box}"' if box is not None else ""
        self.parts.append(f'<text class="{cls}" xml:space="preserve" x="{x:.1f}" y="{y:.1f}"{attr}{extra}{data}>{esc(s)}</text>')

    def natural(self, lines, pad=12, title_cls="l", body_cls="s"):
        """The width a box needs for its longest line."""
        title, body = lines[0], [b for b in lines[1:] if b]
        widest = max([width(title, title_cls)] + [width(b.replace("`", ""), body_cls if not b.startswith("`") else "m") for b in body] + [0])
        return widest + 2 * pad

    def box(self, x, y, lines, kind="cpu", w=None, h=None, pad=12, gap=6, title_cls="l", body_cls="s", r=8):
        """A rounded box whose width fits its longest line unless `w` is given. Returns (x,y,w,h)."""
        fill, stroke = KIND[kind]
        title, body = lines[0], lines[1:]
        body = [b for b in body if b]
        w = w or self.natural(lines, pad, title_cls, body_cls)
        line_h = 20
        h = h or pad + SIZE["l"] + (gap + line_h * len(body) if body else 0) + pad - 2
        ident = self.box_count
        self.box_count += 1
        self.parts.append(f'<rect data-box="{ident}" x="{x:.1f}" y="{y:.1f}" width="{w:.1f}" height="{h:.1f}" rx="{r}" fill="{fill}" stroke="{stroke}" stroke-width="1.5"/>')
        ty = y + pad + SIZE["l"] - 3
        self.text(x + pad, ty, title, title_cls, box=ident)
        for b in body:
            ty += line_h if b is not body[0] else gap + line_h - 2
            cls = "m" if b.startswith("`") else body_cls
            self.text(x + pad, ty, b.replace("`", ""), cls, box=ident)
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
        self.parts.append(svg)

    def write(self, name):
        head = (f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {self.w} {self.h}" role="img" aria-labelledby="t d">'
                f'<title id="t">{esc(self.title)}</title><desc id="d">{esc(self.desc)}</desc>{MARKER}<style>{STYLE}</style>'
                f'<rect width="100%" height="100%" rx="14" fill="{PAL["page"]}"/>')
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
               "A repr(C) Pod struct is cast to bytes by bytemuck and written into a wgpu buffer. A bind group layout describes the shape, a bind group attaches the buffer at a group and binding number, and the shader declares the same group and binding with a matching struct. Meshes are vertex-pulled from storage buffers.",
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
    b1 = c.box(28, y2, ["Vertex pulling · arena.rs, triangle.wgsl",
                        "No vertex buffer: positions live in storage at group 3.",
                        "`let v = face_indices[vertex_index];`",
                        "`let row = face_objects[v / 3];`",
                        "`p = mvp * (instances[row].model * pos + translations[row]);`"], "note")
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
                    "`compare: CompareFunction::Greater   (physical.wgsl)`",
                    "More float precision lands where the geometry is: near the eye."], "note")
    x0 = 540
    c.text(x0, 40, "A thick line is not its axis", "h")
    c.raw(f'<polygon points="{x0 + 20},130 {x0 + 500},80 {x0 + 500},310 {x0 + 20},330" fill="{PAL["blue_band"]}" stroke="{navy}" stroke-width="1.5"/>')
    c.text(x0 + 40, 160, "surface: depth changes across x (gradient ∂d/∂x)", "s")
    c.raw(f'<rect x="{x0 + 40}" y="234" width="460" height="32" fill="{PAL["pink_band"]}" stroke="{pink}" stroke-width="1.5"/>')
    c.raw(f'<line x1="{x0 + 40}" y1="250" x2="{x0 + 500}" y2="250" stroke="{pink}" stroke-width="2"/>')
    c.text(x0 + 40, 290, "stroke footprint covers samples beside the axis", "s")
    c.raw(f'<circle cx="{x0 + 250}" cy="261" r="4" fill="{orange}"/>')
    c.text(x0 + 40, 306, "sample here: the surface is nearer → the line is wrongly hidden", "s", fill=orange)
    c.raw(f'<line x1="{x0 + 250}" y1="261" x2="{x0 + 250}" y2="250" stroke="{green}" stroke-width="2"/>')
    c.raw(f'<circle cx="{x0 + 250}" cy="250" r="4" fill="{green}"/>')
    c.text(x0 + 40, 222, "transfer the surface depth to the axis with the gradient, then compare", "s", fill=green)
    c.box(x0 - 20, 350, ["The test, as the shader spells it",
                         "`physical pass writes  depth + (∂d/∂x, ∂d/∂y, primitive id)`",
                         "`d_axis = d_sample + g · (axis − sample)       ink_visibility.wgsl`",
                         "`visible ⇔ line depth ≥ d_axis − tolerance`",
                         "Lesson 18 adds the finite-triangle test for planes that end before the axis."], "note")
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
             (["Scene · scene.rs", "`object_at(row)`", "`edge_at(row, sub)`", "`face_at(row, sub)`", "row → GUID, edge, face, control"], "cpu"),
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
    c.text(28, 76, "Instance (instance.rs ↔ triangle.wgsl)", "l")

    def cells(y, items, kinds):
        x = 28
        for (label, w), kind in zip(items, kinds):
            fill, stroke = KIND[kind]
            w = max(w, width(label, "m") + 16)
            c.raw(f'<rect data-box="c{y}{x:.0f}" x="{x:.1f}" y="{y}" width="{w:.1f}" height="32" fill="{fill}" stroke="{stroke}" stroke-width="1.5"/>')
            c.text(x + 8, y + 21, label, "m", box=f"c{y}{x:.0f}")
            x += w
        return x

    end = cells(88, [("model: [f32;16] · mat4x4<f32> · 64 B · offset 0", 420), ("color: [f32;4] · 64", 150), ("flags 80", 70), ("thickness 84", 96), ("spacing 88", 88), ("_pad 92", 70)],
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
    steps = [("Prepare visibility", "project and bin triangles only when camera or geometry changed", "cpu"),
             ("Physical surfaces", "opaque faces and clouds write depth + primitive metadata", "cpu"),
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


if __name__ == "__main__":
    for draw in (spaces, gpu_data, ink_visibility, picking, text_pipeline, vertex_layout, ownership, frame, finite_triangle):
        draw()
    print("wrote 9 illustrations")
