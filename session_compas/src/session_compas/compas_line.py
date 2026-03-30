from compas.geometry import Line as CLine


def to_compas(ln):
    return CLine([ln[0], ln[1], ln[2]], [ln[3], ln[4], ln[5]])
