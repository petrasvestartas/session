from compas.geometry import Polyline as CPolyline


def to_compas(pl):
    pts = pl.get_points()
    return CPolyline([[p[0], p[1], p[2]] for p in pts])
