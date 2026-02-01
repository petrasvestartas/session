#! python3
# venv: session_py

from session_py.reload import reload_all
reload_all()

from session_py import NurbsCurve, Point, Plane
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
session.add(curve)
print(curve.domain())

# point_at = curve.point_at(curve.domain_start())
# print(point_at)
# print(curve.domain())
# session.add(point_at)

# derivatives = curve.evaluate(0.5, 2)
# tangent = curve.tangent_at(0.5)

# o, t, n, b = curve.frame_at(0.5, True)
# session.add(Plane(o, t, n), scale=0.3)

# o, t, n, b = curve.perpendicular_frame_at(0.5, True)
# session.add(Plane(o, t, n), scale=0.3)

params = []
for i in range(10):
    params.append(9*i/10)
params.append(9)
print(params)
frames = curve.get_perpendicular_frames(params, False)
planes = [Plane(o, t, n) for o, t, n, b in frames]
session.add(planes, scale=0.2)

# session.add([curve.point_at_start(), curve.point_at_middle(), curve.point_at_end()])

# curve.set_start_point(Point(1.957614, 1.140253, 2.0))
# curve.set_end_point(Point(2.15032, 1.868606, 2.0))
session.add(curve)

session.draw(delete=True)
