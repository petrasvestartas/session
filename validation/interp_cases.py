"""Shared battery of interpolated-curve test cases (point set + knot style).
style: one of uniform/chord/sqrt/uniform_periodic/chord_periodic/sqrt_periodic
"""
CASES = [
    ("chord_5pt", "chord", [(14,9,0),(21,22,0),(26,10,0),(35,19,0),(41,13,0)]),
    ("chord_7pt", "chord", [(0,0,0),(2,5,0),(6,4,0),(9,9,0),(13,3,0),(17,8,0),(20,0,0)]),
    ("chord_4pt", "chord", [(0,0,0),(3,5,0),(8,2,0),(12,6,0)]),
    ("chord_3pt", "chord", [(0,0,0),(5,8,0),(11,2,0)]),
    ("chord_2pt", "chord", [(0,0,0),(10,4,0)]),
    ("uniform_5pt", "uniform", [(14,9,0),(21,22,0),(26,10,0),(35,19,0),(41,13,0)]),
    ("sqrt_5pt", "sqrt", [(14,9,0),(21,22,0),(26,10,0),(35,19,0),(41,13,0)]),
    ("chord_3d_6pt", "chord", [(0,0,0),(2,5,3),(6,4,-2),(9,9,4),(13,3,1),(17,8,-3)]),
    ("chord_periodic_6pt", "chord_periodic", [(4,20,0),(-2,20,0),(-2,25,0),(-10,28,0),(-10,21,0),(0,15,0)]),
    ("uniform_periodic_6pt", "uniform_periodic", [(4,20,0),(-2,20,0),(-2,25,0),(-10,28,0),(-10,21,0),(0,15,0)]),
]
