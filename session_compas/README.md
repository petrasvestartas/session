# session_compas

COMPAS converters and viewer for `session_py` geometry.

Converts `session_py` objects (Point, Line, Polyline, Mesh, NurbsCurve, NurbsSurface, PointCloud) to COMPAS geometry and displays them with `compas_viewer`.

## Install

```bash
pip install session_compas
```

## Usage

### Load and view a protobuf/JSON session

```python
from session_compas.session import Session

session = Session.load("scene.pb")
session.show()
```

### Convert individual objects

```python
from session_py import Point, Mesh
from session_compas.session import Session

s = Session()
s.add(Point(1, 2, 3))
s.show()
```

### Convert without viewer

```python
from session_compas.compas_point import to_compas
from session_py import Point

cp = to_compas(Point(1, 2, 3))  # compas.geometry.Point
```

### Serialize session_py to JSON/protobuf

```python
from session_py.session import Session

session = Session()
session.objects.points.append(Point(1, 2, 3))
session.json_dump("scene.json")
session.pb_dump("scene.pb")
```
