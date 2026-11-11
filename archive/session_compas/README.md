# session_compas

COMPAS converters for `session_py` geometry and a publisher to the live viewer.

Converts `session_py` objects (Point, Line, Polyline, Mesh, NurbsCurve, NurbsSurface, PointCloud, Plane, BRep) to COMPAS geometry, and publishes a session to https://petrasvestartas.github.io/session/ the same way the wood C++ projects do.

## Install

```bash
pip install session_compas
```

## Usage

### Publish a session to the live viewer

```python
from session_py import Point
from session_py import Session
from session_compas import publish

session = Session()
session.add_point(Point(1, 2, 3))
publish(session)
```

`publish` also takes a path to an existing `.pb`, and `notify=False` skips the ntfy ping to open pages. It runs the superproject's `bash/publish-scene.sh`, found by walking up from the package or set with `SESSION_PUBLISH_SCRIPT`. The script overwrites the one R2 slot `pb/view_live.pb` and skips unchanged bytes.

### Convert to COMPAS

```python
from session_compas import to_compas
from session_py import Point

cp = to_compas(Point(1, 2, 3))  # compas.geometry.Point
```
