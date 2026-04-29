"""Debug BRep loaded from protobuf — run standalone (no Rhino)."""
import sys
sys.path.insert(0, r"C:\brg\code_rust\session\session_py\src")

from session_py.brep import BRep

PB = r"C:\brg\code_rust\session\serialization\schoring_body_end_0_0.pb"

brep = BRep.pb_load(PB)
print("=== BRep: faces={} edges={} verts={} ===".format(
    brep.face_count(), brep.edge_count(), brep.vertex_count()))

# ── Vertices ──────────────────────────────────────────────────────────────────
print("\n── vertices ({}) ──".format(len(brep.m_vertices)))
for i, v in enumerate(brep.m_vertices):
    print("  v[{}] ({:.3f}, {:.3f}, {:.3f})".format(i, v.x, v.y, v.z))

# ── Surfaces ──────────────────────────────────────────────────────────────────
print("\n── surfaces ({}) ──".format(len(brep.m_surfaces)))
for i, srf in enumerate(brep.m_surfaces):
    ku = srf.get_nurbsknots(0).tolist()
    kv = srf.get_nurbsknots(1).tolist()
    print("  srf[{}] order=({},{}) cvs=({},{}) rational={} planar={}".format(
        i, srf.order(0), srf.order(1),
        srf.cv_count(0), srf.cv_count(1),
        srf.is_rational(), srf.is_planar()))
    print("    knots_u={} knots_v={}".format(
        ["{:.3f}".format(k) for k in ku],
        ["{:.3f}".format(k) for k in kv]))
    if srf.is_rational():
        print("    weights: ", end="")
        wts = []
        for ci in range(srf.cv_count(0)):
            for cj in range(srf.cv_count(1)):
                ok, wx, wy, wz, w = srf.get_cv_4d(ci, cj)
                wts.append("{:.4f}".format(w))
        print(wts)

# ── 3D Curves ─────────────────────────────────────────────────────────────────
print("\n── 3D curves ({}) ──".format(len(brep.m_curves_3d)))
for i, crv in enumerate(brep.m_curves_3d):
    p0 = crv.get_cv(0)
    pn = crv.get_cv(crv.cv_count() - 1)
    print("  crv3d[{}] order={} cvs={} rational={} "
          "start=({:.2f},{:.2f},{:.2f}) end=({:.2f},{:.2f},{:.2f})".format(
        i, crv.order(), crv.cv_count(), crv.is_rational(),
        p0.x, p0.y, p0.z, pn.x, pn.y, pn.z))

# ── 2D Curves ─────────────────────────────────────────────────────────────────
print("\n── 2D curves ({}) ──".format(len(brep.m_curves_2d)))
for i, crv in enumerate(brep.m_curves_2d):
    p0 = crv.get_cv(0)
    pn = crv.get_cv(crv.cv_count() - 1)
    print("  crv2d[{}] order={} cvs={} "
          "start=({:.3f},{:.3f}) end=({:.3f},{:.3f})".format(
        i, crv.order(), crv.cv_count(),
        p0.x, p0.y, pn.x, pn.y))

# ── Topology Edges ────────────────────────────────────────────────────────────
print("\n── topology edges ({}) ──".format(len(brep.m_topology_edges)))
for i, e in enumerate(brep.m_topology_edges):
    sv = brep.m_topology_vertices[e.start_vertex].point_index if e.start_vertex >= 0 else -1
    ev = brep.m_topology_vertices[e.end_vertex].point_index if e.end_vertex >= 0 else -1
    print("  edge[{}] crv3d={} sv={} ev={} trims={}".format(
        i, e.curve_3d_index, e.start_vertex, e.end_vertex,
        e.trim_indices))

# ── Faces ─────────────────────────────────────────────────────────────────────
print("\n── faces ({}) ──".format(len(brep.m_faces)))
for fi, face in enumerate(brep.m_faces):
    srf = brep.m_surfaces[face.surface_index]
    ku = srf.get_nurbsknots(0).tolist()
    kv = srf.get_nurbsknots(1).tolist()
    u_domain = (ku[0], ku[-1])
    v_domain = (kv[0], kv[-1])
    print("  face[{}] srf={} reversed={} loops={} domain_u=({:.3f},{:.3f}) domain_v=({:.3f},{:.3f})".format(
        fi, face.surface_index, face.reversed,
        face.loop_indices, u_domain[0], u_domain[1], v_domain[0], v_domain[1]))
    for li in face.loop_indices:
        loop = brep.m_loops[li]
        print("    loop[{}] type={} trims={}".format(li, loop.type, loop.trim_indices))
        for ti in loop.trim_indices:
            trim = brep.m_trims[ti]
            crv2d = brep.m_curves_2d[trim.curve_2d_index]
            u_vals = [crv2d.get_cv(k).x for k in range(crv2d.cv_count())]
            v_vals = [crv2d.get_cv(k).y for k in range(crv2d.cv_count())]
            in_domain = (min(u_vals) >= u_domain[0] - 0.001 and
                         max(u_vals) <= u_domain[1] + 0.001 and
                         min(v_vals) >= v_domain[0] - 0.001 and
                         max(v_vals) <= v_domain[1] + 0.001)
            print("      trim[{}] edge={} rev={} crv2d={} u=[{:.3f},{:.3f}] v=[{:.3f},{:.3f}] in_domain={}".format(
                ti, trim.edge_index, trim.reversed,
                trim.curve_2d_index,
                min(u_vals), max(u_vals), min(v_vals), max(v_vals),
                in_domain))
