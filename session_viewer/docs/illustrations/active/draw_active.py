#!/usr/bin/env python3
"""Generate active cumulative course illustrations using the BRG Equilibrium SVG palette.

Run `python docs/illustrations/active/draw_active.py`, then measure and pin `textLength` in
Chromium via `docs/check_illustrations.cjs` / `docs/check_svg.py`.
"""
from pathlib import Path
import sys

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent))

import draw  # noqa: E402
from draw import Canvas, KIND, PAL, RADIUS, esc, row, width  # noqa: E402


def save(c: Canvas, name: str) -> None:
    head = (
        f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {c.w} {c.h}" role="img" aria-labelledby="t d">'
        f'<title id="t">{esc(c.title)}</title><desc id="d">{esc(c.desc)}</desc>{draw.MARKER}<style>{draw.STYLE}</style>'
        f'<rect width="100%" height="100%" rx="{RADIUS}" fill="{PAL["black"]}"/>'
    )
    (HERE / name).write_bytes((head + "".join(c.parts) + "</svg>\n").encode("utf-8"))


def course_map() -> None:
    c = Canvas(
        "Nine cumulative checkpoints from empty canvas to the production viewer",
        "Each checkpoint keeps completed files in place and adds modules and registrations: 13 -> 15 -> 42 -> 46 -> 48 -> 69 -> 167 -> 171 -> 370 files.",
        1200,
        360,
    )
    c.text(28, 38, "Nine cumulative checkpoints · zero rewrites", "h")
    c.text(28, 62, "every step keeps earlier files intact and adds its own modules", "s", fill=PAL["text2"])

    r1 = row(
        c,
        84,
        [
            (["1 · frame (13)", "`device.rs · launch.rs`", "WebGPU white canvas"], "cpu"),
            (["2 · painting (15)", "`gpu/ui.rs`", "egui triangle + text"], "gpu"),
            (["3 · mathematics (42)", "`app/stage/*`", "TeX, matrices, 2D flat"], "cpu"),
            (["4 · dock (46)", "`command_dock/*`", "input, chips, history"], "cpu"),
            (["5 · camera (48)", "`camera.rs`", "orbit, fit, ray, NDC"], "cpu"),
        ],
        gap=28,
    )
    y2 = max(b[1] + b[3] for b in r1) + 42
    last1 = r1[-1]
    r2 = row(
        c,
        y2,
        [
            (["6 · rows (69)", "`build.rs · Instance`", "112 B GPU object table"], "gpu"),
            (["7 · geometry (167)", "`engine/* · shaders/*`", "mesh, BRep, ink, tiles"], "gpu"),
            (["8 · decoding (171)", "`decode · validate`", "512 MiB protobuf caps"], "cpu"),
            (["9 · application (370)", "`state.rs · app/*`", "352/352 viewer sources"], "sel"),
        ],
        gap=36,
    )
    c.raw(
        f'<path class="ar" d="M{last1[0] + last1[2] / 2:.1f},{last1[1] + last1[3]:.1f} '
        f'L{last1[0] + last1[2] / 2:.1f},{y2 - 20:.1f} L{r2[0][0] + r2[0][2] / 2:.1f},{y2 - 20:.1f} '
        f'L{r2[0][0] + r2[0][2] / 2:.1f},{y2 - 4:.1f}"/>'
    )
    yb = max(b[1] + b[3] for b in r2) + 32
    c.text(
        28,
        yb,
        "Blue = CPU/Rust state · Pink = GPU/WGSL passes · Yellow = production viewer endpoint (production.html)",
        "s",
        fill=PAL["yellow"],
    )
    c.w = int(max(b[0] + b[2] for b in r1 + r2) + 28)
    c.h = int(yb + 24)
    save(c, "course-map.svg")


def stage_slide() -> None:
    c = Canvas(
        "1600x900 logical slide frame and mathematical stage pipeline",
        "YAML deck actions build TeX expressions and matrices in a 1600x900 logical frame, aligned on '=' or centered in fixed em slots, then sampled at clock time t into the glyph atlas and egui painter.",
        1220,
        430,
    )
    c.text(28, 38, "1600 × 900 logical slide · layout once, sample every frame", "h")

    sx, sy, sw, sh = 28, 68, 424, 240
    c.raw(f'<rect x="{sx}" y="{sy}" width="{sw}" height="{sh}" rx="6" fill="#1e1e26" stroke="#8a8fb0" stroke-width="1.4"/>')
    split_x = sx + int(sw * 0.45)
    c.raw(f'<line x1="{split_x}" y1="{sy + 14}" x2="{split_x}" y2="{sy + sh - 14}" stroke="#4a4e68" stroke-width="1.2"/>')
    c.text(sx + 14, sy + 24, "SLIDE TITLE", "m", fill=PAL["grey"])
    c.text(split_x + 14, sy + 24, "say: step caption", "s", fill=PAL["text2"])
    c.text(sx + 38, sy + 126, "scene half", "l", fill=PAL["blue_band"])
    c.text(sx + 20, sy + 150, "(3D net in ch. 9.8)", "s", fill=PAL["text2"])

    eq_x = split_x + 80
    c.raw(f'<line x1="{eq_x}" y1="{sy + 50}" x2="{eq_x}" y2="{sy + 118}" stroke="{PAL["yellow"]}" stroke-width="1.4" stroke-dasharray="4 3"/>')
    c.text(eq_x - 8, sy + 74, "x", "m", anchor="end")
    c.text(eq_x + 6, sy + 74, "= (a + b) / c", "m")
    c.text(eq_x - 8, sy + 102, "x + y", "m", anchor="end")
    c.text(eq_x + 6, sy + 102, "= √(a² + b²)", "m")
    c.text(eq_x + 18, sy + 138, "Expr::align ('=' axis)", "s", anchor="middle", fill=PAL["yellow"])

    mx, my = split_x + 54, sy + 156
    for r_i in range(2):
        for c_i in range(2):
            col = PAL["pink_band"] if r_i == c_i else PAL["blue_band"]
            val = "2" if r_i == c_i else "-1"
            bx, by = mx + c_i * 40, my + r_i * 32
            c.raw(f'<rect x="{bx}" y="{by}" width="36" height="26" rx="4" fill="{col}"/>')
            c.text(bx + 18, by + 18, val, "m", anchor="middle", fill=PAL["black"], keep=True)
    c.text(mx + 90, my + 30, "M[r, c] cells", "s", fill=PAL["pink"])
    c.text(sx, sy + sh + 24, "16:9 desktop: side-by-side · 390×844 phone: stacked", "s", fill=PAL["text2"])

    rx = 484
    p1 = row(
        c,
        68,
        [
            (["deck.rs · YAML", "white default ground", "`eq · matrix · drawing`"], "cpu"),
            (["tex.rs + font.rs", "OpenType MATH metrics", "`Rc<Expr>` primitives"], "cpu"),
            (["actions.rs + morph.rs", "forward & replay steps", "`Tween<T>` + `-1.2` slot"], "cpu"),
        ],
        x0=rx,
        gap=22,
    )
    py2 = max(b[1] + b[3] for b in p1) + 48
    p2 = row(
        c,
        py2,
        [
            (["draw.rs + flat.rs", "prims, 24×0.30 arrows", "hide empty matrix cells"], "gpu"),
            (["paint.rs · Atlas", "`swash` glyph coverage", "egui textured quads"], "gpu"),
        ],
        x0=rx,
        gap=34,
    )
    ax1 = p1[2][0] + p1[2][2] / 2
    ax2 = p2[0][0] + p2[0][2] / 2
    ymid = py2 - 22
    c.raw(
        f'<path class="ar" d="M{ax1:.1f},{p1[2][1] + p1[2][3]:.1f} '
        f'L{ax1:.1f},{ymid:.1f} L{ax2:.1f},{ymid:.1f} L{ax2:.1f},{py2 - 4:.1f}"/>'
    )
    c.text((ax1 + ax2) / 2, ymid - 6, "sample at clock t ∈ [0, 1]", "s", anchor="middle")

    ny = max(sy + sh + 42, py2 + p2[0][3] + 24)
    right_edge = p1[-1][0] + p1[-1][2]
    note = c.box(
        28,
        ny,
        [
            "Kernel motion values (session_rust): clock eases t ∈ [0, 1]; kernel interpolates geometry",
            "`Vector::lerp · Color::lerp · Line::lerp · Polyline::resampled · Polyline::morph · Xform::interpolate`",
        ],
        "note",
        w=right_edge - 28,
    )
    c.w = int(right_edge + 28)
    c.h = int(note[1] + note[3] + 22)
    save(c, "stage-slide.svg")


def dock_flow() -> None:
    c = Canvas(
        "Command dock state versus one-frame command output",
        "CommandLine persists input text, history, completion candidates and focus across frames; view::show returns Option<String> once when Enter or an option chip commits.",
        1220,
        320,
    )
    c.text(28, 38, "Persistent input state · one-frame returned command", "h")
    r = row(
        c,
        72,
        [
            (["WindowEvent", "keys, Tab, ↑/↓, Esc", "pointer on option chips"], "cpu"),
            (["CommandLine · mod.rs", "persists across frames:", "`buf · history · focus`"], "cpu"),
            (["view::show · view.rs", "paints bottom bar + chips", "returns `Option<String>`"], "gpu"),
            (["Caller (1× execution)", "runs `Hello / Color / Clear`", "never re-runs on redraw"], "sel"),
        ],
        gap=58,
        labels=["events", "state", "Some(cmd)"],
    )
    yb = max(b[1] + b[3] for b in r) + 28
    note = c.box(
        28,
        yb,
        [
            "Why state and execution stay separate",
            "Redrawing a frame must never execute the last command twice; Option<String> is consumed once after egui finishes.",
            "The Commands trait supplies vocabulary, prefix completion and option chips (Blue, Pink) without scene coupling.",
        ],
        "note",
        w=r[-1][0] + r[-1][2] - 28,
    )
    c.w = int(r[-1][0] + r[-1][2] + 28)
    c.h = int(note[1] + note[3] + 22)
    save(c, "dock-flow.svg")


def instance_112() -> None:
    c = Canvas(
        "112-byte Instance row and 16-byte camera-relative translation",
        "Every scene object occupies one 112-byte Instance row plus one 16-byte vec4 translation rebased around the camera anchor. Rust #[repr(C)] and WGSL scene.wgsl share exact field offsets.",
        1220,
        360,
    )
    c.text(28, 38, "112 bytes per object row · exact Rust ↔ WGSL mirror", "h")
    c.text(28, 66, "Instance (src/engine/gpu/instance/layout.rs ↔ src/shaders/scene.wgsl)", "l")

    cells_spec = [
        ("0 · model", "[f32; 16] · 64 B", "cpu", 196),
        ("64 · color", "[f32; 4] · 16 B", "cpu", 154),
        ("80 · flags", "u32 · 4 B", "gpu", 118),
        ("84 · ao_radius", "f32 · 4 B", "gpu", 146),
        ("88 · spacing", "f32 · 4 B", "gpu", 132),
        ("92 · _pad (edge)", "u32 · 4 B", "sel", 162),
        ("96 · effect", "[f32; 4] · 16 B", "gpu", 160),
    ]
    x = 28.0
    y = 84
    for top_s, bot_s, kind, w_min in cells_spec:
        fill, stroke = KIND[kind]
        w_box = max(w_min, width(top_s, "m") + 24, width(bot_s, "s") + 24)
        ident = c.box_count
        c.box_count += 1
        c.parts.append(
            f'<rect data-box="{ident}" x="{x:.1f}" y="{y}" width="{w_box:.1f}" height="58" rx="5" fill="{fill}" stroke="{stroke}" stroke-width="0"/>'
        )
        c.text(x + 12, y + 22, top_s, "m", fill=PAL["black"], box=ident)
        c.text(x + 12, y + 45, bot_s, "s", fill=PAL["text2"], box=ident)
        x += w_box + 8

    total_w = x - 8 - 28
    c.text(
        28,
        166,
        "assert!(size_of::<Instance>() == 112);  ·  _pad @ 92 packs RGB edge colour; effect @ 96 holds [reveal, point_scale, 0, 0]",
        "s",
        fill=PAL["yellow"],
    )

    b1 = c.box(
        28,
        188,
        [
            "Group 2 · Binding 0: instances[] (N × 112 B)",
            "Rotation + scale in `model` (`f32`); translation zeroed here",
            "Rewriting tint, `FLAG_VISIBLE`, or `effect[0]` reveal = 112 B write",
        ],
        "gpu",
        w=total_w * 0.50,
    )
    b2 = c.box(
        b1[0] + b1[2] + 20,
        188,
        [
            "Group 2 · Binding 1: translations[] (N × 16 B)",
            "`f64` world origin − `f64` camera anchor → `vec4<f32>`",
            "Eliminates `f32` jitter far from `(0, 0, 0)` without re-uploading meshes",
        ],
        "cpu",
        w=total_w - b1[2] - 20,
    )
    yb = max(b1[1] + b1[3], b2[1] + b2[3]) + 26
    c.text(
        28,
        yb,
        "build.rs expands #include \"scene.wgsl\" once per shader into OUT_DIR/shaders/ and strips // comments so Rust shader!() embeds exact WGSL.",
        "s",
        fill=PAL["text2"],
    )
    c.w = int(x - 8 + 28)
    c.h = int(yb + 24)
    save(c, "instance-112.svg")


def decode_bounds() -> None:
    c = Canvas(
        "Bounded protobuf validation, conversion and cooperative yielding",
        "Raw bytes pass four explicit gates: 512 MiB body cap, wire-tag/varint validation, 2,000,000 geometry object counter, and an explicit 64-level tree stack, yielding to the browser every 2,048 items.",
        1220,
        330,
    )
    c.text(28, 38, "Four explicit bounds before any scene upload", "h")
    r = row(
        c,
        72,
        [
            (["1 · Body limit", "`len <= 512 MiB`", "larger clouds → stream"], "warn"),
            (["2 · Wire reader", "`validate::varint`", "checked field bounds"], "cpu"),
            (["3 · Kernel objects", "`<= 2,000,000` items", "`Rc` shared in GUID map"], "cpu"),
            (["4 · Tree stack", "explicit `Vec`, depth `<= 64`", "no unbounded recursion"], "sel"),
        ],
        gap=52,
        labels=["bytes", "prost", "tree"],
    )
    b0 = r[0]
    c.raw(
        f'<rect x="{b0[0]}" y="{b0[1]}" width="{b0[2]:.1f}" height="{b0[3]:.1f}" rx="{RADIUS}" fill="none" stroke="{PAL["orange"]}" stroke-width="1.8" stroke-dasharray="7 5"/>'
    )
    yb = max(b[1] + b[3] for b in r) + 28
    note = c.box(
        28,
        yb,
        [
            "Cooperative browser responsiveness (yielding.rs)",
            "Every 2,048 decoded records, wasm awaits a zero-millisecond browser timer (`yield_now`) so input and status frames never freeze.",
            "Unknown valid protobuf fields skip cleanly via next-offset; truncated varints or out-of-range mesh indices fail with the filename.",
        ],
        "note",
        w=r[-1][0] + r[-1][2] - 28,
    )
    c.w = int(r[-1][0] + r[-1][2] + 28)
    c.h = int(note[1] + note[3] + 22)
    save(c, "decode-bounds.svg")


def stage_scene_sync() -> None:
    c = Canvas(
        "Real-scene stage frame order and force-density equilibrium",
        "Tick stage clock -> apply orthographic camera -> build 2x4 world-to-slide lens -> paint equations and projected labels -> sync kernel geometry rows in Scene3.",
        1220,
        340,
    )
    c.text(28, 38, "One coordinate bridge · 3D kernel scene ↔ 2D math column", "h")
    r = row(
        c,
        72,
        [
            (["1 · Clock + Net", "`D_N x_N = p − D_F x_F`", "solves free XYZ + `M`"], "cpu"),
            (["2 · Camera + lens", "orthographic scene half", "`2 × 4` world → slide map"], "cpu"),
            (["3 · Stage paint", "TeX + projected labels", "fly-outs across halves"], "gpu"),
            (["4 · Scene3::sync", "`Point · Line · Polyline`", "updates 112 B GPU rows"], "sel"),
        ],
        gap=58,
        labels=["solve", "anchors", "Spatial[]"],
    )
    yb = max(b[1] + b[3] for b in r) + 28
    note = c.box(
        28,
        yb,
        [
            "Why sync runs AFTER paint in the same frame",
            "Presentation::paint evaluates current Spatial descriptions at clock t; Scene3::sync then updates row transforms and reveal",
            "before State::render encodes GPU passes — zero 1-frame lag, and moving a cable updates one 112 B row instead of a mesh.",
        ],
        "note",
        w=r[-1][0] + r[-1][2] - 28,
    )
    c.w = int(r[-1][0] + r[-1][2] + 28)
    c.h = int(note[1] + note[3] + 22)
    save(c, "stage-scene-sync.svg")


def one_gesture_112() -> None:
    c = Canvas(
        "A drag is three moments, and only the last one writes the document",
        "Grabbing remembers base_local and base_place. Every drag frame writes a preview into the 112-byte Instance row + 16-byte translation without touching Session. Releasing commits one undoable transaction.",
        1200,
        480,
    )
    c.text(28, 40, "One gesture · 60 Hz GPU preview · 1 undo transaction", "h")

    xs = [28, 420, 812]
    titles = ["1 · grab", "2 · drag (every frame)", "3 · release"]
    for x_i, t in zip(xs, titles):
        c.text(x_i, 76, t, "l")

    a = c.box(
        xs[0],
        96,
        [
            "remembered at press",
            "`base_local`: object transform",
            "`base_place`: drawn placement",
            "`Drag`: ray hit on handle",
        ],
        "cpu",
        w=360,
    )
    b = c.box(
        xs[1],
        96,
        [
            "written to GPU only",
            "`delta = gizmo.update(drag, ray)`",
            "`place = delta * base_place`",
            "`objects.set_placement(row, place)`",
        ],
        "gpu",
        w=360,
    )
    d = c.box(
        xs[2],
        96,
        [
            "committed to Session",
            "`local = delta * base_local`",
            "`session.begin(\"move\")`",
            "`session.set_xform(guid, local)`",
            "`session.commit()`",
        ],
        "cpu",
        w=360,
    )
    y = max(a[1] + a[3], b[1] + b[3], d[1] + d[3])
    c.arrow(a[0] + a[2] + 6, a[1] + 40, b[0] - 6, a[1] + 40)
    c.arrow(b[0] + b[2] + 6, b[1] + 40, d[0] - 6, b[1] + 40)
    c.raw(
        f'<path d="M{b[0] + b[2] - 30},{b[1] + b[3] + 10} a 24 24 0 1 1 -38 0" '
        f'stroke="{PAL["pink_band"]}" stroke-width="2" fill="none" marker-end="url(#a)"/>'
    )
    c.text(b[0] + b[2] - 50, b[1] + b[3] + 70, "60 Hz preview", "s", anchor="middle", fill=PAL["pink"])

    costs = [
        ("112 B instance + 16 B translation", PAL["pink_band"], b[0]),
        ("1 undoable history op", PAL["blue_band"], d[0]),
    ]
    cy = y + 92
    for label, colour, x_i in costs:
        w = width(label, "s") + 28
        c.raw(f'<rect x="{x_i}" y="{cy}" width="{w:.1f}" height="28" rx="6" fill="{colour}"/>')
        c.text(x_i + 14, cy + 19, label, "s", fill=PAL["black"], keep=True)
    c.text(xs[0], cy + 19, "0 GPU / 0 history writes", "s", fill=PAL["text2"])

    n = c.box(
        28,
        cy + 48,
        [
            "Why drag frames bypass Session",
            "Writing 60 document ops per second would make Ctrl+Z undo a single 16 ms frame instead of the whole drag.",
            "Measuring both preview and commit from base_local / base_place makes dropped frames drift-free.",
        ],
        "note",
        w=1144,
    )
    c.h = int(n[1] + n[3] + 26)
    c.w = 1200
    save(c, "one-gesture-112.svg")


def math_atlas() -> None:
    c = Canvas(
        "Stage glyph coverage atlas and textured quad batching",
        "Glyphs are keyed by (font_idx, glyph_id, px_tenths), rasterized once by swash with a 1 px gutter, and batched into ordered egui textured quads alongside matrix cells and rules.",
        1200,
        320,
    )
    c.text(28, 38, "Rasterize once per (font, glyph_id, px) · batch textured quads", "h")
    r = row(
        c,
        72,
        [
            (["1 · Atlas key", "`(font, glyph_id, px)`", "quantized pixel size"], "cpu"),
            (["2 · swash::scale", "8-bit alpha coverage", "whitespace uses 0 px²"], "cpu"),
            (["3 · Shelf packer", "`1 px` transparent gutter", "overflow triggers rebuild"], "gpu"),
            (["4 · Out::flush", "merges adjacent glyphs", "preserves cell/rule order"], "sel"),
        ],
        gap=48,
        labels=["miss", "alpha", "UVs"],
    )
    yb = max(b[1] + b[3] for b in r) + 28
    note = c.box(
        28,
        yb,
        [
            "Why mathematics owns its own glyph atlas instead of glyphon",
            "TeX fractions, radicals and extensible delimiters address exact OpenType MATH glyph IDs and baseline bearings directly.",
            "1 px gutters prevent bilinear texture sampling from bleeding neighbour ink into fraction bars or matrix digits.",
        ],
        "note",
        w=r[-1][0] + r[-1][2] - 28,
    )
    c.w = int(r[-1][0] + r[-1][2] + 28)
    c.h = int(note[1] + note[3] + 22)
    save(c, "math-atlas.svg")


def doc_identity() -> None:
    c = Canvas(
        "Document identity, display rows and multi-lane GPU footprints",
        "FileDoc wraps Rc<Session> with a file transform; (doc_idx, GUID) maps to a stable display row while Footprint tracks relocating vertex/stroke spans across GPU lanes.",
        1200,
        330,
    )
    c.text(28, 38, "Stable (doc_idx, GUID) row identity · relocatable GPU Footprint", "h")
    r = row(
        c,
        72,
        [
            (["FileDoc · scene.rs", "`Rc<Session>` + `Xform`", "2 placements share 1 doc"], "cpu"),
            (["(doc_idx, GUID)", "`guid_to_row` lookup", "stable across edits/undo"], "sel"),
            (["Footprint · rows.rs", "`One · Many · Cloud · None`", "arena / pipe / ribbon spans"], "gpu"),
            (["Hierarchy · 128/page", "flat pre-order `Vec`", "`depth · subtree_end`"], "cpu"),
        ],
        gap=46,
        labels=["index", "row", "panel"],
    )
    yb = max(b[1] + b[3] for b in r) + 28
    note = c.box(
        28,
        yb,
        [
            "Why GUID alone cannot be the viewer's object key",
            "Loading the same .pb twice under two manifest transforms shares Rc<Session>; (doc_idx, GUID) hides or moves one placement cleanly.",
            "Separating row ID from Footprint lets 16 MiB compaction repack GPU buffers without invalidating selection or layer overrides.",
        ],
        "note",
        w=r[-1][0] + r[-1][2] - 28,
    )
    c.w = int(r[-1][0] + r[-1][2] + 28)
    c.h = int(note[1] + note[3] + 22)
    save(c, "doc-identity.svg")


def debug_oracles() -> None:
    c = Canvas(
        "Four failure boundaries and four independent diagnostic oracles",
        "Isolate startup/HTTP failures with viewerDiagnostics, scene/memory drift with ?inspect=1, incremental sync bugs with scene_sync::verify, and shader/occlusion bugs with selftest ray/ID oracles.",
        1200,
        330,
    )
    c.text(28, 38, "Four failure boundaries · four independent oracles", "h")
    r = row(
        c,
        72,
        [
            (["1 · Startup & HTTP", "`viewerDiagnostics`", "adapter, phases, 1st error"], "cpu"),
            (["2 · Live scene state", "`?inspect=1` JSON", "rows, lanes, `known_bytes`"], "cpu"),
            (["3 · Incremental sync", "`scene_sync::verify`", "150-step fuzz vs fresh walk"], "sel"),
            (["4 · Shader & GPU", "`selftest::oracle` + IDs", "CPU ray check + WARP/GPU"], "gpu"),
        ],
        gap=44,
        labels=["loaded?", "staged?", "drawn?"],
    )
    yb = max(b[1] + b[3] for b in r) + 28
    note = c.box(
        28,
        yb,
        [
            "Never judge a GPU pass by its own output alone",
            "selftest::oracle casts independent CPU rays to verify hidden-line ink; selftest::lifecycle checks both RGB and pick-ID images.",
            "Software WARP witnesses isolate pipeline logic, while hardware driver ABI bugs (e.g. Android arrow_corner) require target-device runs.",
        ],
        "note",
        w=r[-1][0] + r[-1][2] - 28,
    )
    c.w = int(r[-1][0] + r[-1][2] + 28)
    c.h = int(note[1] + note[3] + 22)
    save(c, "debug-oracles.svg")


def frame_clear() -> None:
    c = Canvas(
        "Tutorial 1 startup and white-frame submission loop",
        "DOM #canvas CSS dimensions times devicePixelRatio initialize wgpu SurfaceConfiguration; async device::open posts Ready via EventLoopProxy; Frame::draw records LoadOp::Clear(WHITE), drops the pass borrow, submits to queue and presents.",
        1200,
        330,
    )
    c.text(28, 38, "Async GPU startup · CPU command recording · queue presentation", "h")
    r = row(
        c,
        72,
        [
            (["1 · #canvas + winit", "`client_w * dpr` pixels", "`EventLoopProxy<Ready>`"], "cpu"),
            (["2 · device::open", "`256 MiB` storage cap", "sRGB + error `Mutex`"], "cpu"),
            (["3 · CommandEncoder", "`LoadOp::Clear(WHITE)`", "drop `_pass` → `finish()`"], "gpu"),
            (["4 · Queue + Surface", "`queue.submit([buf])`", "`output.present()`"], "sel"),
        ],
        gap=46,
        labels=["spawn_local", "Ready", "submit"],
    )
    yb = max(b[1] + b[3] for b in r) + 28
    note = c.box(
        28,
        yb,
        [
            "Three invariants introduced in Tutorial 1 and kept through Tutorial 9",
            "Size the surface from DOM clientWidth × devicePixelRatio immediately — winit's first resize can arrive after device creation.",
            "Drop _pass before encoder.finish() to release the mutable borrow, and return false so static frames sleep until input or resize.",
        ],
        "note",
        w=r[-1][0] + r[-1][2] - 28,
    )
    c.w = int(r[-1][0] + r[-1][2] + 28)
    c.h = int(note[1] + note[3] + 22)
    save(c, "frame-clear.svg")


def stage_layout_1600() -> None:
    c = Canvas(
        "1600x900 presentation slide layout, white-paper graphic-statics style and upright portrait stacking",
        "Each stage chapter shares a 1600x900 logical slide on white paper (#ffffff default, toggled to #000000 with T), split at split=0.46 (x=736) between the clipped 3D WebGPU scene half and the opaque bg derivation column, with accent blue titles and 4 px whole-deck progress bar, non-overlapping 650 ms say narration, ETH Zurich | BRG marks, and Frame::stacked portrait reflow when height >= 1.1 * width.",
        1220,
        420,
    )
    c.text(28, 38, "1600 × 900 presentation layout · split = 0.46 (x = 736) · white-paper default", "h")

    # Left: 1600x900 desktop slide blueprint on white paper (append rects directly to c.parts to preserve exact #ffffff/#111111)
    sx, sy, sw, sh = 28, 62, 640, 304
    split_x = sx + int(sw * 0.46)  # sx + 294 = 322
    c.parts.append(f'<rect x="{sx}" y="{sy}" width="{sw}" height="{sh}" rx="6" fill="#ffffff" stroke="#8a9099" stroke-width="1.5"/>')
    # Faint vertical divider (y = 96..844 in 900 space)
    c.parts.append(f'<line x1="{split_x}" y1="{sy + 34}" x2="{split_x}" y2="{sy + sh - 22}" stroke="#d0d6dc" stroke-width="1.4"/>')

    # Top chrome: title at [40, 40] in accent blue, say at [769.6, 40] in ink
    c.text(sx + 16, sy + 24, "FORCE DENSITY MATRICES", "s", fill="#0066a6", keep=True)
    c.text(sx + 16, sy + 42, "[40, 40] · 23 px bold · accent", "s", fill="#4c535d", keep=True)
    c.text(split_x + 16, sy + 24, "say: step sentence ($math$, **bold**)", "s", fill="#111111", keep=True)
    c.text(split_x + 16, sy + 42, "[769.6, 40] · 650 ms out → +6 px rise", "s", fill="#0066a6", keep=True)

    # 3D scene half diagram (0..736): black 3.2 px cables, open-circle paper-centre nodes, elongated #007bc4 load arrow
    gx, gy = sx + 148, sy + 182
    pts3 = [
        (gx - 90, gy + 44, "n₁", -7, -7, "end"),
        (gx - 28, gy - 68, "n₃", 8, -3, "start"),
        (gx + 92, gy + 36, "n₄", 8, -4, "start"),
        (gx - 34, gy + 20, "n₂", 4, 16, "start"),
    ]
    for px, py, _, _, _, _ in pts3:
        c.parts.append(f'<line x1="{gx}" y1="{gy}" x2="{px}" y2="{py}" stroke="#111111" stroke-width="2.1"/>')
    # Elongated blue load arrow p from n0 (24 x 0.30 proportion, tip on endpoint)
    c.parts.append(f'<line x1="{gx}" y1="{gy}" x2="{gx}" y2="{gy + 36}" stroke="#007bc4" stroke-width="2.2"/>')
    c.parts.append(f'<polygon points="{gx - 3.6},{gy + 34} {gx + 3.6},{gy + 34} {gx},{gy + 52}" fill="#007bc4"/>')
    # Open-circle nodes with white paper centre
    for px, py, lbl, dx, dy, anc in pts3:
        c.parts.append(f'<circle cx="{px}" cy="{py}" r="4.2" fill="#ffffff" stroke="#111111" stroke-width="1.8"/>')
        c.text(px + dx, py + dy, lbl, "s", anchor=anc, fill="#111111", keep=True)
    c.parts.append(f'<circle cx="{gx}" cy="{gy}" r="4.5" fill="#ffffff" stroke="#111111" stroke-width="1.9"/>')
    c.text(gx + 9, gy - 7, "n₀ (open ring)", "s", fill="#111111", keep=True)
    c.text(gx + 9, gy + 38, "p (load)", "s", fill="#0066a6", keep=True)
    c.text(sx + 16, sy + sh - 20, "WebGPU 3D (0..736 px, clipped)", "s", fill="#0066a6", keep=True)

    # Right column inner layout box: TOP=150, BOTTOM=70, x0=769.6..1544 (MARGIN=56)
    cx0, cy0, cw0, ch0 = split_x + 16, sy + 56, sw - (split_x - sx) - 32, sh - 96
    c.parts.append(f'<rect x="{cx0}" y="{cy0}" width="{cw0}" height="{ch0}" rx="4" fill="none" stroke="#b8c0c8" stroke-width="1.1" stroke-dasharray="4 3"/>')
    c.text(cx0 + 10, cy0 + 18, "TOP = 150 · x = 769.6..1544 (MARGIN = 56)", "s", fill="#4c535d", keep=True)
    c.text(cx0 + 10, cy0 + 43, "eq / tex (36 px): '=' axis + {{part}}", "m", fill="#111111", keep=True)
    c.text(cx0 + 10, cy0 + 69, "matrix / row (30 px): C → C_N, C_F", "m", fill="#0066a6", keep=True)
    c.text(cx0 + 10, cy0 + 95, "drawing: (flat.rs) 2D polygons & arrows", "m", fill="#b81858", keep=True)
    c.text(cx0 + 10, cy0 + 121, "text 25 · label 24 · say 23 · code 19 px", "m", fill="#111111", keep=True)
    c.text(cx0 + 10, cy0 + 147, "overflow > 680 px → 650 ms upward shift", "s", fill="#4c535d", keep=True)

    # Bottom-right ETH | BRG marks & continuous 4 px accent progress bar
    c.text(sx + sw - 16, sy + sh - 20, "ETH zürich | BRG  (21 px · ink)", "s", anchor="end", fill="#111111", keep=True)
    c.parts.append(f'<rect x="{sx + 1}" y="{sy + sh - 6}" width="{sw - 2}" height="5" fill="#e4e7e8"/>')
    c.parts.append(f'<rect x="{sx + 1}" y="{sy + sh - 6}" width="{int(sw * 0.44)}" height="5" fill="#007bc4"/>')

    # Middle: Upright 390x844 Frame::stacked diagram (x = 692..864)
    px0, py0, pw0, ph0 = 692, 62, 172, 304
    c.parts.append(f'<rect x="{px0}" y="{py0}" width="{pw0}" height="{ph0}" rx="8" fill="#181b24" stroke="#5b6278" stroke-width="1.5"/>')
    mid_y = py0 + 144
    c.parts.append(f'<line x1="{px0 + 1}" y1="{mid_y}" x2="{px0 + pw0 - 1}" y2="{mid_y}" stroke="#4a4e68" stroke-width="1.2"/>')
    c.text(px0 + 86, py0 + 24, "Frame::stacked", "m", anchor="middle", fill=PAL["yellow"])
    c.text(px0 + 86, py0 + 44, "h >= 1.1 w (390×844)", "s", anchor="middle", fill=PAL["grey"])
    c.text(px0 + 86, py0 + 86, "Top: 3D WebGPU", "s", anchor="middle", fill=PAL["blue_band"])
    c.text(px0 + 86, py0 + 108, "hold-drag = orbit", "s", anchor="middle", fill=PAL["text2"])
    c.text(px0 + 86, mid_y + 28, "Bottom: column", "s", anchor="middle", fill="#f4f6fa")
    c.text(px0 + 86, mid_y + 52, "say + eq + matrix", "s", anchor="middle", fill=PAL["pink"])
    c.text(px0 + 86, mid_y + 76, "swipe ←/→ = step", "s", anchor="middle", fill=PAL["text2"])
    c.text(px0 + pw0 - 10, py0 + ph0 - 16, "ETH | BRG", "s", anchor="end", fill="#f4f6fa")
    c.parts.append(f'<rect x="{px0 + 1}" y="{py0 + ph0 - 6}" width="{pw0 - 2}" height="5" fill="#3f4548"/>')
    c.parts.append(f'<rect x="{px0 + 1}" y="{py0 + ph0 - 6}" width="76" height="5" fill="#2f8bff"/>')

    # Right: Routing & Controls cards (x = 888..1192)
    b1 = c.box(
        888,
        62,
        [
            "Local vs live URL routing",
            "`?stage=decks/<name>/index.yaml`",
            "`&data=off` → local Trunk (`decks/`)",
            "`6` decks · `27` chapters shipped",
            "`&chapter=1..7&step=N&theme=dark`",
        ],
        "cpu",
        w=304,
    )
    b2 = c.box(
        888,
        b1[1] + b1[3] + 16,
        [
            "Zero-chrome stage controls",
            "`#viewer-docs` removed on `?stage=`",
            "Left half: drag orbit, wheel, 2× click",
            "Right half: click / `Space` / `←` `→`",
            "`1..9` chapter · `↑`/`↓` slide · `T` ground",
        ],
        "sel",
        w=304,
    )
    yb = max(sy + sh, b2[1] + b2[3]) + 24
    c.text(
        28,
        yb,
        "index.html preloads NewCMMath-Book, NewCM10-Book, NewCM10-Bold and NewCMMono10-Book plus index.yaml and all chapter YAMLs while WASM downloads.",
        "s",
        fill=PAL["text2"],
    )
    c.w = 1220
    c.h = int(yb + 22)
    save(c, "stage-layout-1600.svg")


def stage_palette_style() -> None:
    c = Canvas(
        "Dual-ground graphic-statics palette, co-highlighted matrix cells, open-ring nodes and 6-stop force ramp",
        "theme_palette defaults to Theme::Light (#ffffff) and swaps to Theme::Dark (#000000) on T; numbers stay dark inside sign/yellow cells while titles and labels highlight in accent blue, vectors use 24x0.30 arrowheads, and show: forces maps 6 rank stops.",
        1220,
        396,
    )
    c.text(28, 38, "Graphic-statics dual-ground palette (white default, T swaps to black)", "h")
    c.text(
        28,
        62,
        "Decks name semantic roles (ink, load, accent, pos, neg, yellow, blue_band, pink_band) so T flips paper ↔ black at the same step.",
        "s",
        fill=PAL["text2"],
    )

    swatches = [
        ("bg / ink / bar", "#111111", "#ffffff", "#f4f4f6", "#000000", "cables, rings, TeX, logos"),
        ("blue · accent · load", "#007bc4", "#ffffff", "#2f8bff", "#000000", "loads, titles, bar, focus"),
        ("pink · x,y,z · pos", "#ff2d7a", "#ffffff", "#ff4fa3", "#000000", "coords, +1 (#ff5c9a)"),
        ("neg · cell / faint", "#4c9bff", "#ffffff", "#4c9bff", "#000000", "−1 cells, #e4e7e8 / #3f4548"),
        ("yellow · active", "#ffc400", "#ffffff", "#ffd60a", "#000000", "active cells & washes"),
        ("blue / pink_band", "#a9c9ff", "#ffffff", "#46616e", "#000000", "planes & #ff8fbd fills"),
    ]
    sw_w = 186
    gap = 10
    for idx, (title, light_fg, light_bg, dark_fg, dark_bg, role) in enumerate(swatches):
        x = 28 + idx * (sw_w + gap)
        y = 80
        c.parts.append(f'<rect x="{x}" y="{y}" width="{sw_w}" height="122" rx="6" fill="#181b24" stroke="#4a4e68" stroke-width="1.2"/>')
        c.text(x + 10, y + 20, title, "m", fill="#f4f6fa")
        # Light default chip on top (real #ffffff ground with dark #111111 label)
        c.parts.append(f'<rect x="{x + 10}" y="{y + 30}" width="{sw_w - 20}" height="28" rx="4" fill="{light_bg}" stroke="#8a9099" stroke-width="1"/>')
        c.parts.append(f'<circle cx="{x + 24}" cy="{y + 44}" r="7" fill="{light_fg}" stroke="#111111" stroke-width="0.7"/>')
        c.text(x + 38, y + 49, f"light {light_fg}", "s", fill="#111111", keep=True)
        # Dark toggled chip below
        c.parts.append(f'<rect x="{x + 10}" y="{y + 64}" width="{sw_w - 20}" height="28" rx="4" fill="{dark_bg}" stroke="#3b4354" stroke-width="1"/>')
        c.parts.append(f'<circle cx="{x + 24}" cy="{y + 78}" r="7" fill="{dark_fg}"/>')
        c.text(x + 38, y + 83, f"dark {dark_fg}", "s", fill="#f4f6fa", keep=True)
        c.text(x + 10, y + 112, role, "s", fill=PAL["grey"])

    # Bottom row: 3 visual style mechanics
    by = 220
    # Card 1: Sign cells & co-highlighted symbols
    c.parts.append(f'<rect x="28" y="{by}" width="410" height="132" rx="6" fill="#181b24" stroke="#4a4e68" stroke-width="1.2"/>')
    c.text(42, by + 24, "1 · Sign cells & co-highlighted symbols", "l", fill="#f4f6fa")
    pills = [
        ("+1", "#ff5c9a", "#111111", "pos"),
        ("−1", "#4c9bff", "#111111", "neg"),
        ("0", "#e4e7e8", "#111111", "cell"),
        ("1", "#ffc400", "#111111", "yellow"),
    ]
    for pi, (val, pfill, pfg, pname) in enumerate(pills):
        px = 42 + pi * 92
        py = by + 38
        c.parts.append(f'<rect x="{px}" y="{py}" width="76" height="38" rx="6" fill="{pfill}" stroke="#3b4354" stroke-width="1"/>')
        c.text(px + 38, py + 25, val, "m", anchor="middle", fill=pfg, keep=True)
        c.text(px + 38, py + 56, pname, "s", anchor="middle", fill=PAL["grey"])
    c.text(42, by + 118, "numbers stay ink; matrix_name_key & r,: / :,c use accent", "s", fill=PAL["yellow"])

    # Card 2: Open-circle nodes & elongated 24x0.30 arrowheads
    c.parts.append(f'<rect x="454" y="{by}" width="362" height="132" rx="6" fill="#181b24" stroke="#4a4e68" stroke-width="1.2"/>')
    c.text(468, by + 24, "2 · Open rings & 24 × 0.30 arrowheads", "l", fill="#f4f6fa")
    # Small white paper preview strip inside Card 2
    c.parts.append(f'<rect x="468" y="{by + 38}" width="334" height="40" rx="4" fill="#ffffff" stroke="#8a9099" stroke-width="1"/>')
    c.parts.append(f'<line x1="490" y1="{by + 58}" x2="642" y2="{by + 58}" stroke="#007bc4" stroke-width="2.6"/>')
    c.parts.append(f'<polygon points="640,{by + 53.8} 640,{by + 62.2} 668,{by + 58}" fill="#007bc4"/>')
    c.parts.append(f'<circle cx="490" cy="{by + 58}" r="5.5" fill="#ffffff" stroke="#111111" stroke-width="2.2"/>')
    c.parts.append(f'<line x1="668" y1="{by + 48}" x2="668" y2="{by + 68}" stroke="#111111" stroke-width="1.6" stroke-dasharray="3 2"/>')
    c.text(684, by + 63, "tip = end", "s", fill="#111111", keep=True)
    c.text(468, by + 98, "0.58 bg centre in 3D Point · r = 5.5 px in flat.rs", "s", fill="#7cc4ff")
    c.text(468, by + 118, "24 pen lengths × 0.30 aspect (arrow.wgsl & flat.rs)", "s", fill=PAL["text2"])

    # Card 3: 6-stop force-rank cable ramp (show: forces)
    c.parts.append(f'<rect x="832" y="{by}" width="360" height="132" rx="6" fill="#181b24" stroke="#4a4e68" stroke-width="1.2"/>')
    c.text(846, by + 24, "3 · 6-stop force ramp (show: forces)", "l", fill="#f4f6fa")
    ramp_cols = ["#182c5b", "#007bc4", "#00c46a", "#ffc400", "#ff7a1a", "#ff2d7a"]
    for ri, rc in enumerate(ramp_cols):
        rx = 846 + ri * 54
        rh = 4 + ri * 3
        ry = by + 54 - rh // 2
        c.parts.append(f'<rect x="{rx}" y="{ry}" width="48" height="{rh}" rx="2" fill="{rc}" stroke="#4a4e68" stroke-width="0.6"/>')
    c.text(846, by + 82, "navy → blue → green → yellow → orange → pink", "s", fill=PAL["pink"])
    c.text(846, by + 108, "width = 1.2 + 2.8 √(f_k / f_max)  (1.2..4.0 px)", "m", fill="#f4f6fa")

    c.w = 1220
    c.h = int(by + 132 + 24)
    save(c, "stage-palette-style.svg")


if __name__ == "__main__":
    course_map()
    stage_slide()
    dock_flow()
    instance_112()
    decode_bounds()
    stage_scene_sync()
    one_gesture_112()
    math_atlas()
    doc_identity()
    debug_oracles()
    frame_clear()
    stage_layout_1600()
    stage_palette_style()
    print(f"Generated {len(list(HERE.glob('*.svg')))} SVGs in {HERE}")
