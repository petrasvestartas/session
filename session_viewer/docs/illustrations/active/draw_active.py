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
        "Each checkpoint keeps completed files in place and adds modules and registrations: 13 -> 15 -> 41 -> 45 -> 47 -> 68 -> 166 -> 170 -> 369 files.",
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
            (["3 · mathematics (41)", "`app/stage/*`", "TeX, matrices, tweens"], "cpu"),
            (["4 · dock (45)", "`command_dock/*`", "input, chips, history"], "cpu"),
            (["5 · camera (47)", "`camera.rs`", "orbit, fit, ray, NDC"], "cpu"),
        ],
        gap=28,
    )
    y2 = max(b[1] + b[3] for b in r1) + 42
    last1 = r1[-1]
    r2 = row(
        c,
        y2,
        [
            (["6 · rows (68)", "`build.rs · Instance`", "112 B GPU object table"], "gpu"),
            (["7 · geometry (166)", "`engine/* · shaders/*`", "mesh, BRep, ink, tiles"], "gpu"),
            (["8 · decoding (170)", "`decode · validate`", "512 MiB protobuf caps"], "cpu"),
            (["9 · application (369)", "`state.rs · app/*`", "351/351 viewer sources"], "sel"),
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
            (["deck.rs · YAML", "dark default ground", "`eq · matrix · row(w)`"], "cpu"),
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
            (["draw.rs · prims_of(t)", "sample pos, RGB, alpha", "hide empty matrix cells"], "gpu"),
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
    print(f"Generated {len(list(HERE.glob('*.svg')))} SVGs in {HERE}")
