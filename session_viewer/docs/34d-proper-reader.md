# 34d A proper CAD reader — Line learns width and color on the wire

> **Big picture.** The PDF drawing rendered all-black at uniform width — and the viewer was
> innocent: the .pb file really said so. Two bugs upstream of rendering: the PDF converter never
> read stroke color/width from the page, and — the deeper one — **`line.proto` had no `width` or
> `linecolor` field at all**. Every other class had them; Line silently dropped both on every
> serialize in every language. This lesson is a KERNEL schema change (the 3-language kind
> CLAUDE.md warns about) plus the converter fix — after it, a file carries what the drawing
> actually looks like: dark-red pens, five distinct lineweights.

## Files we touch

```
session_proto/line.proto                    # + width, + linecolor (the only class missing them)
bash/gen_proto.sh                           # (run, not edited) regenerates py/cpp/rust bindings
session_rust/src/line.rs                    # pb_dumps/pb_loads wire the two fields
session_py/src/session_py/line.py           # to_proto/from_proto likewise
session_cpp/src/line.cpp                    # likewise (polyline.cpp is the pattern)
session_data/pdf_to_session.py              # stroke color+width, page arg, --nojson
```

## Step 1 — the schema: `session_proto/line.proto`

Polyline is the pattern (`width = 4; Color linecolor = 5;`). **Add the import and two fields:**

```proto
import "color.proto";
…
  double width = 6;   // Line width
  Color linecolor = 7; // Line color
```

Then regenerate all three languages' committed bindings:

```bash
./bash/gen_proto.sh
```

> The generated files (`session_py/src/session_py/proto/`, `session_cpp/generated/`,
> `session_rust/src/proto/`) must be COMMITTED together with the .proto change — CI runs
> `gen_proto.sh --check` and fails on stale output. Rust regenerates during its own build, so
> `gen_proto.sh` will immediately fail compiling `line.rs` — that's Step 2 telling you where.

## Step 2 — wire the fields, three languages

**Rust `line.rs`** — `pb_dumps` gains (after `xform: None,` — brep.rs's color block is the
pattern):

```rust
            width: self.width,
            linecolor: Some(crate::proto::Color {
                guid: self.linecolor.guid().to_string(),
                name: self.linecolor.name.clone(),
                r: self.linecolor.r,
                g: self.linecolor.g,
                b: self.linecolor.b,
                a: self.linecolor.a,
            }),
```

and `pb_loads` gains (before `Ok(line)`):

```rust
        if proto.width > 0.0 { line.width = proto.width; }
        if let Some(color) = proto.linecolor {
            line.linecolor.set_guid(color.guid.clone());
            line.linecolor.name = color.name;
            line.linecolor.r = color.r;
            line.linecolor.g = color.g;
            line.linecolor.b = color.b;
            line.linecolor.a = color.a;
        }
```

**Python `line.py`** — `pb_dumps` before `SerializeToString()`: `proto.width = self.width` plus
the six `proto.linecolor.*` assignments; `pb_loads` before `return line`: guard with
`if proto.width > 0.0:` and `if proto.HasField('linecolor'):` → `line.linecolor = Color(r, g, b,
a, name)`. **C++ `line.cpp`** — `proto.set_width(width)` + `proto.mutable_linecolor()` setters in
`pb_dumps`; `proto.width() > 0.0` / `proto.has_linecolor()` reads in `pb_loads` — copy
`polyline.cpp`'s block. The Session/Objects paths route through these same functions in all three
languages, so session round-trips inherit the fix.

Verify with the class tests — they must stay identical ×3:

```bash
./bash/quicktest.sh line --py     # 15/15 (verified)
./bash/quicktest.sh line --rust   # 15/15 (verified)
./bash/quicktest.sh line --cpp    # run when no other build holds the linker — not yet verified
```

## Step 3 — the converter reads the PDF's ink: `session_data/pdf_to_session.py`

PyMuPDF's `get_drawings()` provides `color` (stroke RGB), `fill`, and `width` per path — the
converter ignored all three. **Add a style reader and thread it through every emit site** (plus a
page argument and a `--nojson` flag — the big drawings would write half-GB JSONs):

```python
def path_style(path):
    """Stroke color (fallback: fill, then black) + line width from the PDF path."""
    c = path.get("color") or path.get("fill")
    col = Color(float(c[0]), float(c[1]), float(c[2]), 1.0) if c else Color.black()
    w = path.get("width")
    return col, float(w) if w and w > 0 else 1.0
```

In the path loop: `col, w = path_style(path)`, then every `Line`/`Polyline` gets
`obj.linecolor = col; obj.width = w` before `session.add_*`, and `flush(chain)` becomes
`flush(chain, col, w)`. `PAGE = int(sys.argv[3])` selects the page; `"--nojson" in sys.argv`
skips the JSON dump. Regenerate:

```bash
cd session_data && python pdf_to_session.py   # rewrites 30700_querschnitt_gg.pb
```

## Verify — the file now tells the truth

```python
from session_py import Session
from collections import Counter
s = Session.pb_load('../session_data/30700_querschnitt_gg.pb')
colors = Counter(); widths = Counter()
for g in s.lookup.values():
    if type(g).__name__ in ('Line', 'Polyline'):
        c = g.linecolor
        colors[(round(c.r,2), round(c.g,2), round(c.b,2))] += 1
        widths[round(g.width, 2)] += 1
print(colors.most_common(6)); print(widths.most_common(6))
```

Before the schema fix: `{black: 42232}` / `{1.0: 42232}`. After:

```
colors: dark-red 23838 · grey67 8367 · grey42 5852 · black 1360 · white 1087 …
widths: 0.28 × 32313 · 0.14 × 6169 · 1.0 × 1418 · 0.51 · 0.37 · 0.71
```

The drawing was never black — Line was dropping its color on every save. Colors show in the
viewer immediately (segment rows carry linecolor since 34b); widths render in 34f.

## Two related data fixes (same session, worth recording)

- **`Point.pointcolor` default blue → black** in all three languages (`point.rs` / `point.py` /
  `point.h`) + the three `point_test` repr/equality updates — points are linework-family,
  linework defaults black.
- **`floor_model.pb` carried `linecolors = white`** for all 201 meshes (an exporter artifact —
  `add_face` seeds black, but deserialization trusts the file). Patched in place via
  `session_py`: load → `m.linecolors = [Color.black()]*len` → `pb_dump`.

## Recap

```
Ch 34d: THE WIRE TELLS THE TRUTH. line.proto + width/linecolor (the ONLY class missing them) →
        gen_proto.sh regenerates committed bindings ×3 → pb_dumps/pb_loads wired in rust/py/cpp
        (polyline is the pattern; >0 / HasField guards keep old files loading). Converter reads
        the PDF's stroke color+width per path (fill fallback), takes a page number, --nojson.
        The querschnitt is dark-red with five pen weights — it always was.
```

## Next

`34e-many-files-grid.md` — one drawing proves the reader; nine different drawings at once prove
the architecture. Streaming multi-file load, a cycling grid, and the wasm OOM that had to die
first.
