from compas.geometry import Polyline as CPolyline


def to_compas(crv, segments=100):
    d = crv.domain()
    pts = []
    for i in range(segments + 1):
        t = d[0] + (d[1] - d[0]) * i / segments
        p = crv.point_at(t)
        pts.append([p.x, p.y, p.z])
    return CPolyline(pts)
