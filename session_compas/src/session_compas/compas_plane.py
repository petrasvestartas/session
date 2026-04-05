from compas.geometry import Line as CLine
from compas.geometry import Polyline as CPolyline


def to_compas(plane, size=None):
    o = plane.origin
    x = plane.x_axis
    y = plane.y_axis
    z = plane.z_axis
    s = size if size is not None else plane.width
    c0 = [o[0] - x[0]*s - y[0]*s, o[1] - x[1]*s - y[1]*s, o[2] - x[2]*s - y[2]*s]
    c1 = [o[0] + x[0]*s - y[0]*s, o[1] + x[1]*s - y[1]*s, o[2] + x[2]*s - y[2]*s]
    c2 = [o[0] + x[0]*s + y[0]*s, o[1] + x[1]*s + y[1]*s, o[2] + x[2]*s + y[2]*s]
    c3 = [o[0] - x[0]*s + y[0]*s, o[1] - x[1]*s + y[1]*s, o[2] - x[2]*s + y[2]*s]
    tip = [o[0] + z[0]*s, o[1] + z[1]*s, o[2] + z[2]*s]
    rect = CPolyline([c0, c1, c2, c3, c0])
    normal = CLine([o[0], o[1], o[2]], tip)
    return rect, normal
