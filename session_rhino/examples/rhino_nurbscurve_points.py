#! python3
# venv: session_py

from session_py.reload import reload_all
reload_all()

from session_py import NurbsCurve
from session_py import Point
from session_py import Color
from session_rhino.session import Session

session = Session()

points = [
    Point(0.0, 0.0, 0.0),
    Point(1.0, 2.0, 0.0),
    Point(2.0, 0.0, 0.0),
    Point(3.0, 2.0, 0.0),
    Point(4.0, 0.0, 0.0)
]

curve = NurbsCurve.create(False, 2, points)
session.add(curve)

# to_polyline_adaptive
adaptive_pts, adaptive_params = curve.to_polyline_adaptive(0.1, 0.0, 0.0)
for p in adaptive_pts:
    p.pointcolor = Color(255, 0, 0, 0)
session.add(adaptive_pts)

# divide_by_count
div_pts, div_params = curve.divide_by_count(10, True)
for p in div_pts:
    p.pointcolor = Color(0, 255, 0, 0)
session.add(div_pts)

# divide_by_length
len_pts, len_params = curve.divide_by_length(0.5)
for p in len_pts:
    p.pointcolor = Color(0, 0, 255, 0)
session.add(len_pts)

session.draw(delete=True)
