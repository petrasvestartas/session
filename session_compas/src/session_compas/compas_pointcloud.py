from compas.geometry import Pointcloud as CPointcloud


def to_compas(pc):
    pts = []
    for p in pc.get_points():
        pts.append([p[0], p[1], p[2]])
    return CPointcloud(pts)
