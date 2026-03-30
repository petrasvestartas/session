from compas.geometry import Point as CPoint


def to_compas(pt):
    return CPoint(pt.x, pt.y, pt.z)


def to_compas_list(pts):
    return [CPoint(p.x, p.y, p.z) for p in pts]
