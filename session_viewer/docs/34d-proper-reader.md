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
session_proto/line.proto                    # + import color.proto, + width = 6, + linecolor = 7
bash/gen_proto.sh                           # (run, not edited) regenerates py/cpp/rust bindings
session_rust/src/line.rs                    # pb_dumps/pb_loads wire the two fields
session_py/src/session_py/line.py           # pb_dumps/pb_loads wire the two fields
session_cpp/src/line.cpp                    # pb_dumps/pb_loads wire the two fields
session_data/pdf_to_session.py              # stroke color+width, page arg, --nojson
```

## Step 1 — the schema: `session_proto/line.proto`

Polyline already has the two fields (`double width = 4; Color linecolor = 5;` in
`polyline.proto`) — that's the *pattern*, not the numbers. Field numbers are per-message: Line's
next free numbers are **6 and 7**. Never renumber or reuse an existing field number — old .pb
files address fields by number.

**1a. Find the import block** at the top of `line.proto`:

```proto
import "point.proto";
import "xform.proto";
```

and add the color import above it (imports stay alphabetical):

```proto
import "color.proto";
import "point.proto";
import "xform.proto";
```

**1b. Find the last field of `message Line`:**

```proto
  Xform xform = 5;    // Transformation matrix
```

and insert two lines after it (before the closing `}`). The complete message now reads:

```proto
message Line {
  Point start = 1;    // Start point
  Point end = 2;      // End point
  string guid = 3;    // Unique identifier
  string name = 4;    // Line name
  Xform xform = 5;    // Transformation matrix
  double width = 6;   // Line width
  Color linecolor = 7; // Line color
}
```

**1c. Regenerate all three languages' committed bindings:**

```bash
./bash/gen_proto.sh
```

> The generated files (`session_py/src/session_py/proto/`, `session_cpp/generated/`,
> `session_rust/src/proto/`) must be COMMITTED together with the .proto change — CI runs
> `gen_proto.sh --check` and fails on stale output. The Rust step is a `cargo build --lib`
> (build.rs regenerates `src/proto/`), so `gen_proto.sh` will immediately fail compiling
> `line.rs` — prost added two fields to the `crate::proto::Line` struct that `pb_dumps` doesn't
> fill yet. That error is Step 2a telling you where to go.

## Step 2 — wire the fields, three languages

### 2a. Rust — `session_rust/src/line.rs`

**In `pub fn pb_dumps`, find the tail of the `crate::proto::Line { … }` literal** — the three
lines at the Line level (NOT the `xform: None,` inside the two `crate::proto::Point` literals
above them):

```rust
            guid: self.guid().to_string(),
            name: self.name.clone(),
            xform: None,
```

Insert after that `xform: None,` (the literal's closing `};` follows right after):

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

**In `pub fn pb_loads`, find:**

```rust
        line.set_guid(proto.guid);
        line.name = proto.name;
```

Insert after it (before `Ok(line)`):

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

The guards matter: proto3 encodes "field absent" as `0.0`/unset, so an OLD .pb file (written
before this lesson) loads with `width` still at its constructor default `1.0` and `linecolor`
still black — not width-0 invisible lines.

### 2b. Python — `session_py/src/session_py/line.py`

`Color` is already imported at the top of the file (`from .color import Color`) — nothing to add
there.

**In `def pb_dumps(self)`, find:**

```python
        # Set xform
        proto.xform.name = self.xform.name
        proto.xform.matrix.extend(self.xform.m)
```

Insert after it (before `return proto.SerializeToString()`):

```python
        # Set width and linecolor
        proto.width = self.width
        proto.linecolor.guid = self.linecolor.guid
        proto.linecolor.name = self.linecolor.name
        proto.linecolor.r = self.linecolor.r
        proto.linecolor.g = self.linecolor.g
        proto.linecolor.b = self.linecolor.b
        proto.linecolor.a = self.linecolor.a
```

**In `def pb_loads(cls, data)`, find:**

```python
        # Load xform if present
        if proto.HasField('xform'):
            line.xform = Xform()
            line.xform.name = proto.xform.name
            line.xform.m = list(proto.xform.matrix)
```

Insert after it (before `return line`):

```python
        # Load width and linecolor
        if proto.width > 0.0:
            line.width = proto.width
        if proto.HasField('linecolor'):
            line.linecolor = Color(
                proto.linecolor.r,
                proto.linecolor.g,
                proto.linecolor.b,
                proto.linecolor.a,
                proto.linecolor.name,
            )
```

### 2c. C++ — `session_cpp/src/line.cpp`

**In `std::string Line::pb_dumps() const`, find the xform loop:**

```cpp
    for (int i = 0; i < 16; ++i) {
        proto_xform->add_matrix(xform.m[i]);
    }
```

Insert after it (before `return proto.SerializeAsString();`):

```cpp
    // Serialize width and linecolor
    proto.set_width(width);
    auto* color_proto = proto.mutable_linecolor();
    color_proto->set_r(linecolor.r);
    color_proto->set_g(linecolor.g);
    color_proto->set_b(linecolor.b);
    color_proto->set_a(linecolor.a);
    color_proto->set_name(linecolor.name);
```

**In `Line Line::pb_loads(const std::string& data)`, find:**

```cpp
    if (proto.has_xform()) {
        line.xform.name = proto.xform().name();
        for (int i = 0; i < proto.xform().matrix_size() && i < 16; ++i) {
            line.xform.m[i] = proto.xform().matrix(i);
        }
    }
```

Insert after it (before `return line;`):

```cpp
    // Deserialize width and linecolor
    if (proto.width() > 0.0) {
        line.width = proto.width();
    }
    if (proto.has_linecolor()) {
        line.linecolor.r = proto.linecolor().r();
        line.linecolor.g = proto.linecolor().g();
        line.linecolor.b = proto.linecolor().b();
        line.linecolor.a = proto.linecolor().a();
        line.linecolor.name = proto.linecolor().name();
    }
```

(C++ intentionally skips the color guid on both sides — that matches `polyline.cpp`, the class
this block is modeled on.) The Session/Objects paths route through these same functions in all
three languages, so session round-trips inherit the fix.

**Verify** with the class tests — Line has 15 tests, identical ×3:

```bash
./bash/quicktest.sh line --py     # prints: [py-minitest] 15/15 passed
./bash/quicktest.sh line --rust   # prints: [rust-line] 15/15 passed  (runs the full Rust suite)
./bash/quicktest.sh line --cpp    # prints: [cpp-line] 15/15 passed   (rebuilds the C++ suite)
```

(`--rust`/`--cpp` build and run the whole language's minitest, so their output also ends with a
`TOTAL` line — the per-class line above is the one to check.)

## Step 3 — the converter reads the PDF's ink: `session_data/pdf_to_session.py`

PyMuPDF's `get_drawings()` provides `color` (stroke RGB), `fill`, and `width` per path — the
converter ignored all three. Five edits, top to bottom.

**3a. Import + page argument.** Find:

```python
from session_py import Session, Point, Line, Polyline, NurbsCurve
```

replace with:

```python
from session_py import Session, Point, Line, Polyline, NurbsCurve, Color
```

Find:

```python
OUT = sys.argv[2] if len(sys.argv) > 2 else "30700_querschnitt_gg"
```

insert after it:

```python
PAGE = int(sys.argv[3]) if len(sys.argv) > 3 else 0
```

Find `page = doc[0]` and replace with:

```python
page = doc[PAGE]
```

**3b. The style reader.** Find the counter block:

```python
n_segments = 0
```

insert after it (blank line between):

```python
def path_style(path):
    """Stroke color (fallback: fill, then black) + line width from the PDF path."""
    c = path.get("color") or path.get("fill")
    col = Color(float(c[0]), float(c[1]), float(c[2]), 1.0) if c else Color.black()
    w = path.get("width")
    return col, float(w) if w and w > 0 else 1.0
```

**3c. `flush` carries the style.** Find the whole old function:

```python
def flush(chain):
    global n_lines, n_polylines, n_segments
    if len(chain) < 2:
        return
    if len(chain) == 2:
        a, b = chain
        session.add_line(Line(a[0], a[1], a[2], b[0], b[1], b[2]))
        n_lines += 1
    else:
        points = []
        for c in chain:
            points.append(Point(c[0], c[1], c[2]))
        session.add_polyline(Polyline(points))
        n_polylines += 1
    n_segments += len(chain) - 1
```

replace with:

```python
def flush(chain, col, w):
    global n_lines, n_polylines, n_segments
    if len(chain) < 2:
        return
    if len(chain) == 2:
        a, b = chain
        ln = Line(a[0], a[1], a[2], b[0], b[1], b[2])
        ln.linecolor = col
        ln.width = w
        session.add_line(ln)
        n_lines += 1
    else:
        points = []
        for c in chain:
            points.append(Point(c[0], c[1], c[2]))
        pl = Polyline(points)
        pl.linecolor = col
        pl.width = w
        session.add_polyline(pl)
        n_polylines += 1
    n_segments += len(chain) - 1
```

**3d. The path loop.** Find:

```python
for path in page.get_drawings():
    chain = []
```

insert the style read between those two lines:

```python
for path in page.get_drawings():
    col, w = path_style(path)
    chain = []
```

Then replace **all five** `flush(chain)` calls in the loop with `flush(chain, col, w)`. They are:
in the `op == "l"` else-branch, at the top of the `op == "c"`, `op == "re"`, and `op == "qu"`
branches, and the final `flush(chain)` after the `for item` loop. Old and new, at each of the
five places:

```python
                flush(chain)          # ← old
                flush(chain, col, w)  # ← new
```

Still inside the loop, three emit sites gain the style. In the `op == "c"` branch, find:

```python
            session.add_nurbscurve(NurbsCurve.create(False, 3, points))
```

replace with (NurbsCurve has `linecolors`, a per-segment *list*, not a single `linecolor` — only
width is set here):

```python
            nc = NurbsCurve.create(False, 3, points)
            nc.width = w
            session.add_nurbscurve(nc)
```

In the `op == "re"` branch, find:

```python
            session.add_polyline(Polyline(corners))
```

replace with:

```python
            pl = Polyline(corners)
            pl.linecolor = col
            pl.width = w
            session.add_polyline(pl)
```

In the `op == "qu"` branch, find:

```python
            session.add_polyline(Polyline(points))
```

replace with:

```python
            pl = Polyline(points)
            pl.linecolor = col
            pl.width = w
            session.add_polyline(pl)
```

**3e. `--nojson` + the print.** The big drawings would write half-GB JSONs. Find the last four
lines of the file:

```python
session.pb_dump(OUT + ".pb")
session.file_json_dump(OUT + ".json")
print(f"pages=1/{len(doc)} lines={n_lines} polylines={n_polylines} curves={n_curves} segments~{n_segments}")
print(f"wrote {OUT}.pb + {OUT}.json")
```

replace with:

```python
session.pb_dump(OUT + ".pb")
if "--nojson" not in sys.argv:
    session.file_json_dump(OUT + ".json")
print(f"page={PAGE + 1}/{len(doc)} lines={n_lines} polylines={n_polylines} curves={n_curves} segments~{n_segments}")
print(f"wrote {OUT}.pb + {OUT}.json")
```

**Regenerate the fixture:**

```bash
cd session_data && python pdf_to_session.py   # rewrites 30700_querschnitt_gg.pb (+ .json)
```

## Verify — the file now tells the truth

Run this from the repo root with a Python that has `session_py` installed (the repo venv:
`./uvsession/bin/python`):

```python
from session_py import Session
from collections import Counter
s = Session.pb_load('session_data/30700_querschnitt_gg.pb')
colors = Counter(); widths = Counter()
for g in s.lookup.values():
    if type(g).__name__ in ('Line', 'Polyline'):
        c = g.linecolor
        colors[(round(c.r,2), round(c.g,2), round(c.b,2))] += 1
        widths[round(g.width, 2)] += 1
print(colors.most_common(6)); print(widths.most_common(6))
```

Before the schema fix: one bucket each — black × 42232, width 1.0 × 42232. After, the measured
output over the same 42,232 Line/Polyline rows:

```
[((0.6, 0.11, 0.12), 23838), ((0.67, 0.67, 0.67), 8367), ((0.42, 0.42, 0.42), 5852),
 ((0.0, 0.0, 0.0), 1360), ((1.0, 1.0, 1.0), 1087), ((0.5, 0.5, 0.5), 663)]
[(0.28, 32313), (0.14, 6169), (1.0, 1418), (0.51, 879), (0.37, 768), (0.71, 685)]
```

Dark-red `(0.60, 0.11, 0.12)` × 23838, three greys, black, white — and five real pen weights.
The drawing was never black; Line was dropping its color on every save. Colors show in the
viewer immediately (`walk_session` puts `linecolor` into each segment's instance row since 34b);
widths render in 34f.

## Two related data fixes (same session, worth recording)

- **`Point.pointcolor` default blue → black** in all three languages (`point.rs` / `point.py` /
  `point.h`) + the three `point_test` repr/equality updates — points are linework-family,
  linework defaults black.
- **`floor_model.pb` carried `linecolors = white`** for all 201 meshes (an exporter artifact —
  `add_face` seeds black, but deserialization trusts the file). Patched in place via
  `session_py`: load → `m.linecolors = [Color.black()]*len` → `pb_dump`.

## Recap

```
Ch 34d: THE WIRE TELLS THE TRUTH. line.proto + width=6/linecolor=7 (the ONLY class missing them)
        → gen_proto.sh regenerates committed bindings ×3 → pb_dumps/pb_loads wired in rust/py/cpp
        (polyline is the pattern; >0 / HasField guards keep old files loading). Converter reads
        the PDF's stroke color+width per path (fill fallback), takes a page number, --nojson.
        The querschnitt is dark-red with five pen weights — it always was.
```

## Next

`34e-many-files-grid.md` — one drawing proves the reader; nine different drawings at once prove
the architecture. Streaming multi-file load, a cycling grid, and the wasm OOM that had to die
first.
