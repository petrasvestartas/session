from compas.datastructures import Mesh as CMesh


def to_compas(srf, nu=30, nv=30):
    du = srf.domain(0)
    dv = srf.domain(1)
    verts = []
    for i in range(nu + 1):
        u = du[0] + (du[1] - du[0]) * i / nu
        for j in range(nv + 1):
            v = dv[0] + (dv[1] - dv[0]) * j / nv
            p = srf.point_at(u, v)
            verts.append([p.x, p.y, p.z])
    cmesh = CMesh()
    for xyz in verts:
        cmesh.add_vertex(x=xyz[0], y=xyz[1], z=xyz[2])
    for i in range(nu):
        for j in range(nv):
            a = i * (nv + 1) + j
            b = a + 1
            c = a + nv + 2
            d = a + nv + 1
            cmesh.add_face([a, b, c, d])
    return cmesh
