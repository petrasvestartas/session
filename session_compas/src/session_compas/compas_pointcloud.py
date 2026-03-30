from compas.geometry import Pointcloud as CPointcloud


def to_compas(pc):
    pts = []
    for i in range(pc.count()):
        p = pc.point(i)
        pts.append([p.x, p.y, p.z])
    return CPointcloud(pts)
