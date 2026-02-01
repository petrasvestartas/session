# session_rhino

RhinoCommon converters for session_py geometry. Pure RhinoCommon — no rhinoscriptsyntax.

## Install

```bash
./session_rhino/rhino.sh
```

This installs both `session_py` and `session_rhino` into Rhino's Python 3.9 environment.

## API

Each module exposes two functions:

- `to_rhino(obj)` — convert a single session_py object to its RhinoCommon equivalent
- `add(obj_or_list)` — convert and add to the active Rhino document, returns list of GUIDs

| Module | session_py | RhinoCommon | Notes |
|--------|-----------|-------------|-------|
| `rhino_point` | Point | Point3d | Applies pointcolor |
| `rhino_line` | Line | Line | Applies linecolor |
| `rhino_plane` | Plane | Plane | Draws rectangle + colored axis lines |
| `rhino_mesh` | Mesh | Mesh | Uses `to_vertices_and_faces()` |
| `rhino_nurbscurve` | NurbsCurve | NurbsCurve | Handles rational/non-rational, applies linecolor |

## Usage

```python
import session_rhino.rhino_point
import session_rhino.rhino_line
import session_rhino.rhino_plane
import session_rhino.rhino_mesh
import session_rhino.rhino_nurbscurve
```

### Examples

```python
from session_py import Point
from session_py import Line
from session_py import NurbsCurve
from session_py import Plane
import session_rhino.rhino_point
import session_rhino.rhino_line
import session_rhino.rhino_plane
import session_rhino.rhino_nurbscurve

# Single object
session_rhino.rhino_point.add(Point(1, 2, 3))

# List of objects
session_rhino.rhino_point.add([Point(0, 0, 0), Point(1, 0, 0), Point(2, 0, 0)])

# Line
session_rhino.rhino_line.add(Line(0, 0, 0, 5, 5, 0))

# Plane with axis visualization (scale controls rectangle and axis length)
session_rhino.rhino_plane.add(Plane.xy_plane(), scale=2.0)

# NurbsCurve
pts = [Point(0, 0, 0), Point(1, 1, 0), Point(2, 0, 0), Point(3, 1, 0)]
crv = NurbsCurve.create(periodic=False, degree=2, points=pts)
session_rhino.rhino_nurbscurve.add(crv)

# Convert without adding to document
rpt = session_rhino.rhino_point.to_rhino(Point(1, 2, 3))
```

### Plane visualization

`add()` for planes draws:
- A rectangle centered at the origin
- X axis line in red
- Y axis line in green
- Z axis line in blue

The `scale` parameter (default `1.0`) controls the size of the rectangle and axis lines.

### Hot reload

After editing source files:

```python
from session_py.reload import reload_package
reload_package("session_py")
reload_package("session_rhino")
```
