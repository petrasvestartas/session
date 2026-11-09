# Authoring session_viewer slides in Python

There is no separate slide scenegraph. A presentation is a list of topics; a topic is a list of ordinary `Session` objects. Each session is serialized to its own protobuf (`.pb`) file.

`force_density.py` is the complete example. `serve(deck, directory=...)` writes:

- `steps/00_00.pb`, `steps/00_01.pb`, …: geometry, text, matrices, transforms, graph relationships, and animation metadata for each reveal.
- `presentation.yaml`: the topic/reveal order and the effects extracted from those sessions.

Within a topic, the viewer loads one `.pb` per click and adds it to the current picture. A new topic clears the previous topic. Use **Left Arrow** and **Right Arrow** to navigate. No on-screen controls are required. Each slide has one file, so its right panel never appears empty while another file loads. Export removes stale `steps/*.pb` files before writing the current deck. Close the running local viewer before regenerating on Windows: an open protobuf file can interrupt replacement and leave an incomplete deck. Check the manifest references and file count after exporting.

To keep the same 3D figure across a topic, set `preserve_spatial` on one object in the base session and put only new mathematics or temporary annotations in later sessions. The base 3D objects stay in place without regrowing; previous equations and temporary annotations are replaced. The small-net and large-net sequences in `force_density.py` use this mode.

```python
base.graph.set_vertex_attribute(a.guid, "preserve_spatial", 1.0)
```

## Minimal editable deck

```python
from session_py import Arrowhead, Color, Line, Point, Session, Text
from session_py.viewer import serve

pink = Color(220 / 255, 75 / 255, 141 / 255)
white = Color.white()

base = Session("1 · A cable\nA named line and a named equation.")
a = Point(0, 0, 0)
a.name = "n0"
b = Point(2, 0, 1)
b.name = "n1"
base.add_point(a, None)
base.add_point(b, None)

cable = Line.from_points(a, b)
cable.name = "e0"                 # Stable names drive effects and links.
cable.width = 3.0
cable.linecolor = pink
cable.arrowhead = Arrowhead.END
base.add_line(cable, None)

# Put this geometry in the navigable 3D region.
for item in (a, b, cable):
    base.graph.set_vertex_attribute(item.guid, "spatial", 1.0)

formula = Text.from_tex(r"f_x=f\dfrac{x_1-x_0}{l}", Point(1200, 320))
formula.name = "formula"          # 2D mathematics; no spatial=True argument.
formula.height = 34
formula.color = white
base.add_text(formula)

next_reveal = Session("1 · A cable\nThe next click adds information.")
note = Text.from_tex(r"q=f/l", Point(1200, 480))
note.name = "q-definition"
note.height = 34
note.color = white
next_reveal.add_text(note)
next_reveal.graph.set_vertex_attribute(note.guid, "grow", 900.0)

deck = [[base, next_reveal]]      # One topic containing two .pb reveals.
viewer = serve(deck, directory="generated")
print(viewer.url)                  # Open this URL in Chrome.
# viewer.close()                   # When finished; keeps generated/*.pb files.
```

Run this with the same Python environment that contains `session_py` and a built `session_viewer/dist`. The complete deck uses these session operations and the `MatrixEquation` authoring API; it has no Manim, MathJax, SVG equations, or third-party renderer.

## Save or reload one session directly

```python
base.pb_dump("scene.pb")           # Write protobuf bytes.
data = base.pb_dumps()             # The same bytes in memory.
restored = Session.pb_load("scene.pb")
assert restored.pb_dumps() == data
```

A `.pb` stores a session, not the order of a whole deck. The generated `presentation.yaml` stores that order and refers to the individual `.pb` files.

## Reveal and animate objects

```python
# Reveal a line, matrix, or equation over 900 ms.
scene.graph.set_vertex_attribute(cable.guid, "grow", 900.0)
scene.graph.set_vertex_attribute(cable.guid, "delay_ms", 300.0)

# Pulse a point without changing its anchor.
scene.graph.set_vertex_attribute(a.guid, "pulse", 1200.0)

# Translate or uniformly scale with ordinary kernel transforms.
from session_py import Xform
scene.animate_xform(a.guid, Xform.translation(1, 0, 0), 1800, 300)

# Morph a source into a target in a later reveal.
next_reveal.add_relationship(cable.guid, other_line.guid, "morph")
next_reveal.graph.set_edge_attribute(
    (cable.guid, other_line.guid), "duration_ms", 1400.0
)
```

Give every source and target a unique, stable `name`. Spatial line morphs copy geometry: the original 3D arrows remain visible. Equation morphs replace the source expression. Supported text effects are `grow`, `pulse`, and `cells`; transforms support translation and uniform scaling.

## Link geometry and matrices

```python
from session_py import Matrix

matrix = Matrix.from_rows([[1, -1], [1, 0]])
matrix.name = "C"
matrix.cell = 80
scene.add_matrix(matrix, None)

scene.add_relationship(cable.guid, matrix.guid, "highlight:0")    # Row 0.
scene.add_relationship(a.guid, matrix.guid, "highlight:0,0")     # One cell.
scene.add_relationship(b.guid, matrix.guid, "highlight:0,1")
```

Hover or click the geometry to highlight its linked matrix row or entry. A matrix target can also be named programmatically as `C[0,1]`.

## Arrange and animate matrix calculations

`MatrixEquation` is the beginner-facing authoring API in `session_py.viewer`. It creates ordinary kernel `Matrix` and `Text` components, session transforms, and graph effects; it does not introduce another renderer or scene format.

```python
from session_py import Session
from session_py.viewer import MatrixEquation

scene = Session("2 · Multiply\nEach row of A multiplies the column x.")
equation = MatrixEquation(scene, left=920, center_y=280, cell=52)
a = equation.matrix("A", [[1, 2], [3, 4]], r"\mathbf{A}")
equation.symbol(r"\times", "times")
x = equation.matrix("x", [[5], [6]], r"\mathbf{x}")
equation.symbol("=", "equals")
b = equation.matrix("B", [[17], [39]], r"\mathbf{B}")

for row in range(2):
    equation.product(a, x, b, row, 0, at=(1240, 480 + row * 70), delay=row * 1800)

# Entrywise addition uses the same centered layout and cell flashes.
# equation.sum(a, a, a + a, row=0, col=0, at=(1240, 620))
```

Matrices in one `MatrixEquation` share a visual centerline. Multiplication uses `\times`, and every operator is exactly between the adjacent matrix boundaries. The viewer draws rounded cells without decorative brackets and typesets any value that does not already have an explicit cell label. Values are centered, rounded to at most two decimals, and shrunk only when necessary to fit their cells. `product` writes the real dot product without redundant parentheses, uses `≈` when rounded substitutions are inexact, and flashes each row/column operand before the result. `sum` does the same for entrywise addition. `equation.morph(source_text, target_text, duration=1200, delay=0)` transforms one equation into another. Delays are milliseconds.

Use `equation.matrix(...)` instead of manually placing values and titles. For a custom matrix layout, the lower-level transform remains available:

```python
screen = Xform()
screen.m[5] = -1
screen.m[13] = 760
scene.set_xform(matrix.guid, screen * Xform.translation(1200, 200, 0))
```

The presentation design area is 1600 × 760. The left region is navigable 3D; the right region is flat 2D mathematics. Spatial text created with `Text.from_tex(..., spatial=True)` has fixed-size white glyphs with a black shader outline. Set `text.outline` explicitly when a different outline width is needed.

## Make one slide and open it in Chrome

Save this as `one_slide.py`. It is a complete slide: the viewer typesets the matrices and animates `1 × 5 + 2 × 6 = 17` with linked cell highlights.

```python
import threading
import webbrowser
from pathlib import Path

from session_py import Session
from session_py.viewer import MatrixEquation, serve

scene = Session("1 · Matrix multiplication\nWatch the first row multiply the column vector.")
equation = MatrixEquation(scene, left=900, center_y=280, cell=64)
a = equation.matrix("A", [[1, 2], [3, 4]], r"\mathbf{A}")
equation.symbol(r"\times", "times")
x = equation.matrix("x", [[5], [6]], r"\mathbf{x}")
equation.symbol("=", "equals")
b = equation.matrix("B", [[17], [39]], r"\mathbf{B}")
equation.product(a, x, b, row=0, col=0, at=(1240, 520))

viewer = serve([[scene]], directory=Path(__file__).with_name("generated-one-slide"))
webbrowser.open(viewer.url)
threading.Event().wait()
```

Build the viewer only when its Rust or shader code changes:

```bash
cd session/session_viewer
bash ../../wood/tools/buildslot.sh viewer-open env CARGO_BUILD_JOBS=8 NO_COLOR=false trunk build
```

Then run the Python script from the `session/session_py` environment, replacing the path with where you saved it:

```bash
cd session/session_py
uv run --offline --no-sync python ../session_viewer/examples/linkwitz/one_slide.py
```

Python writes `generated-one-slide/presentation.yaml` and `generated-one-slide/steps/00_00.pb`, starts a local server, and opens the slide in Chrome. Use Left/Right Arrow when you add more slides. Stop the script with Ctrl+C. To add another independent slide, append `scene2` as `serve([[scene], [scene2]], ...)`; to add a reveal on the same canvas, use `serve([[scene, reveal]], ...)`.

## Use it in Jupyter

For a computed 3D cable net, no explicit session construction is necessary:

```python
from session_py.viewer import show

nodes = [(0, 0, 0), (2, 0, 1), (0, 2, 1), (2, 2, 0)]
edges = [(0, 1), (1, 2), (2, 3), (3, 0), (0, 2)]
show(nodes, edges, height=640)
```

For an editable presentation:

```python
show(deck, directory="generated", height=760)
```

`show(...)` embeds the local viewer in an iframe. `serve(...)` instead returns a running viewer with a `.url` and `.close()`, which is useful in a standalone script. The protobuf files remain available for inspection, version control, or regeneration; the Python source remains the editable master.

## Embed the finished animation in the teaching notebook

For the JupyterLite notebook at `library.html#/notebook/session-viewer`, use **static hosting**, not `serve(...)`. JupyterLite runs in the browser and cannot open your computer's local Python server. The animation itself needs no Python execution after export.

Deploy the current `session_viewer/dist/` build to the existing directory:

```text
Teaching/CAS/website/session-viewer/
```

Then copy the generated deck into a child directory with this exact layout:

```text
Teaching/CAS/website/session-viewer/force-density/presentation.yaml
Teaching/CAS/website/session-viewer/force-density/steps/00_00.pb
Teaching/CAS/website/session-viewer/force-density/steps/00_01.pb
...
```

Do not add another `generated` path component: `presentation.yaml` refers to its own `steps/` directory. Commit and publish those static files with the Teaching website. The resulting student URL is:

```text
https://brg-teaching.github.io/Teaching/CAS/website/session-viewer/?presentation=force-density/presentation.yaml
```

### Simplest notebook integration: a link

Put this in a Markdown cell, replacing the example URL only if you chose another deck directory:

```markdown
[Open the force-density animation](https://brg-teaching.github.io/Teaching/CAS/website/session-viewer/?presentation=force-density/presentation.yaml)
```

This is the recommended classroom option: it opens the full viewer, gives the canvas keyboard focus, and leaves notebook navigation unaffected. Students use **Left Arrow** and **Right Arrow**. They can orbit/zoom the 3D region with the mouse; the equations remain flat.

### Inline notebook integration

Use one code cell if the animation should appear inside the notebook:

```python
from IPython.display import IFrame

url = (
    "https://brg-teaching.github.io/Teaching/CAS/website/"
    "session-viewer/?presentation=force-density/presentation.yaml"
)
IFrame(src=url, width="100%", height=760)
```

For keyboard navigation in an inline frame, click the drawing once to focus it, then use the arrow keys. Same-origin hosting avoids cross-origin frame restrictions. WebGPU still requires a compatible browser and HTTPS, which GitHub Pages provides.

The editable Python source stays in this repository. Republish `force-density/` only when the deck changes; republish the viewer build as well when shader or viewer behavior changes. No local server, MathJax, Manim, or notebook extension is involved in playback.
