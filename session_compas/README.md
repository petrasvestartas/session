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
session.file_json_dump("scene.json")
session.pb_dump("scene.pb")
```

## Documentation

`compas_viewer` has its own dedicated website — it is **not** bundled with the core COMPAS geometry docs.

| Package | Docs |
|---------|------|
| Viewer (`compas_viewer`) | https://compas.dev/compas_viewer/ |
| Core geometry (`compas`) | https://compas.dev/compas/ |

There is nothing to open locally — just visit the site (e.g. `python -m webbrowser https://compas.dev/compas_viewer/`).

To build the viewer docs offline instead, clone the repo and run Sphinx:

```bash
git clone https://github.com/compas-dev/compas_viewer
cd compas_viewer
pip install -e ".[dev]"
sphinx-build -b html docs docs/_build/html
python -m webbrowser docs/_build/html/index.html
```
