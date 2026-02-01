#! python3
# venv: session_py

from session_py.reload import reload_all
reload_all()

from session_py import NurbsCurve
from session_py import Point
from session_rhino.session import Session

session = Session()

points = [
    Point(1.957614, 1.140253, -0.191281),
    Point(0.912252, 1.886721, 0),
    Point(3.089381, 2.701879, -0.696251),
    Point(5.015145, 1.189141, 0.35799),
    Point(1.854155, 0.514663, 0.347694),
    Point(3.309532, 1.328666, 0),
    Point(3.544072, 2.194233, 0.696217),
    Point(2.903513, 2.091287, 0.696217),
    Point(2.752484, 1.45432, 0),
    Point(2.406227, 1.288248, 0),
    Point(2.15032, 1.868606, 0)
]

curve = NurbsCurve.create(False, 2, points)
print(curve.domain())

frames = curve.get_perpendicular_planes(10)
print(frames)
session.add(frames, scale=0.2)

session.add(curve)

session.draw(delete=True)
