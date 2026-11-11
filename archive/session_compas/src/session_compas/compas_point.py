from __future__ import annotations

from compas.geometry import Point as CPoint


def to_compas(pt):
    return CPoint(pt[0], pt[1], pt[2])


def to_compas_list(pts):
    return [CPoint(p[0], p[1], p[2]) for p in pts]
