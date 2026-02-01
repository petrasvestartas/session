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

# Greville abscissae are parameter values where each CV has most influence
# Useful for controlling curve by moving CVs that lie on or near the curve
greville = curve.get_greville_abcissae()
greville_pts = [curve.point_at(t) for t in greville]
for p in greville_pts:
    p.pointcolor = Color(255, 0, 0, 0)
session.add(greville_pts)

# Control vertices for comparison
cvs = [curve.get_cv(i) for i in range(curve.cv_count())]
for p in cvs:
    p.pointcolor = Color(0, 255, 0, 0)
session.add(cvs)

session.draw(delete=True)
