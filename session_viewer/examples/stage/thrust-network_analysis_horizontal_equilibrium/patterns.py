# Writes patterns.json for chapter 5: seven form diagrams, their first guess and what the solvers find.
# Needs compas_tna for the diagrams (python3 patterns.py); generate.py then only reads the json.
import json
import os

import numpy as np
from compas.datastructures import Mesh
from compas_tna.diagrams import ForceDiagram
from compas_tna.diagrams import FormDiagram

from horizontal_direct import angles
from horizontal_direct import parallel_direct
from horizontal_direct import parallel_loop
from horizontal_direct import targets


def rot90(xy, zdir=1.0):
    return np.column_stack([-zdir * xy[:, 1], zdir * xy[:, 0]])


def grid(n, tri=False):
    mesh = Mesh.from_meshgrid(dx=n, nx=n)
    if tri:
        mesh.quads_to_triangles()
    return mesh


def hexes(n):
    polygons = []
    for r in range(n):
        for c in range(n):
            cx, cy = 1.5 * c, np.sqrt(3) * (r + 0.5 * (c % 2))
            polygons.append([[cx + np.cos(a), cy + np.sin(a), 0] for a in np.arange(6) * np.pi / 3])
    return Mesh.from_polygons(polygons)


def dome(rings, spokes):
    vertices, faces = [[0, 0, 0]], []
    for r in range(1, rings + 1):
        for s in range(spokes):
            a = 2 * np.pi * s / spokes
            vertices.append([r * np.cos(a), r * np.sin(a), 0])

    def at(r, s):
        return 1 + (r - 1) * spokes + s % spokes

    for s in range(spokes):
        faces.append([0, at(1, s), at(1, s + 1)])
    for r in range(1, rings):
        for s in range(spokes):
            faces.append([at(r, s), at(r + 1, s), at(r + 1, s + 1), at(r, s + 1)])
    return Mesh.from_vertices_and_faces(vertices, faces)


def edge(amplitude):
    "A 5 x 5 grid whose bottom edge is free: straight, or bent inwards as an arch."
    mesh = grid(5)
    for v in mesh.vertices():
        x, y = mesh.vertex_attributes(v, "xy")
        if y < 1e-6 and 0 < x < 5:
            mesh.vertex_attribute(v, "y", amplitude * np.sin(np.pi * x / 5))
    return mesh


def not_bottom(p):
    return p[1] > 0.99 or p[0] < 1e-6 or p[0] > 5 - 1e-6


# name: mesh, jitter of the free nodes, seed, which boundary vertices are supports
CASES = {
    "tri": (grid(4, tri=True), 0.18, 3, None),
    "quad": (grid(5), 0.15, 3, None),
    "quadbad": (grid(5), 0.42, 5, None),
    "hex": (hexes(4), 0.0, 1, None),
    "hexbad": (hexes(4), 0.15, 1, None),
    "dome": (dome(3, 8), 0.0, 2, None),
    "open": (edge(0.0), 0.0, 1, not_bottom),
    "arch": (edge(0.55), 0.0, 1, not_bottom),
}

out = {}
for name, (mesh, jitter, seed, supported) in CASES.items():
    rng = np.random.default_rng(seed)
    form = FormDiagram.from_mesh(mesh)
    boundary = set(form.vertices_on_boundary())
    for v in form.vertices():
        if v not in boundary:
            x, y = form.vertex_attributes(v, "xy")
            form.vertex_attributes(v, "xy", [x + rng.uniform(-jitter, jitter), y + rng.uniform(-jitter, jitter)])
    supports = [v for v in boundary if supported is None or supported(form.vertex_attributes(v, "xy"))]
    form.vertices_attribute("is_support", True, keys=supports)
    form.update_boundaries()
    force = ForceDiagram.from_formdiagram(form)

    k_i, _k_i = form.vertex_index(), force.vertex_index()
    xy = np.array(form.vertices_attributes("xy"), dtype=float)
    edges = [(k_i[u], k_i[v]) for u, v in form.edges_where(_is_edge=True)]
    _xy = rot90(np.array(force.vertices_attributes("xy"), dtype=float))
    _edges = [(_k_i[u], _k_i[v]) for u, v in force.ordered_edges(form)]
    fixed = [k_i[v] for v in form.supports()]
    t = targets(xy, edges)

    loop, iterations = parallel_loop(xy, edges, _xy, _edges, kmax=3000, tol=1e-3)
    direct, lengths, states = parallel_direct(xy, edges, _xy, _edges)
    compression = False
    if states:
        try:
            direct, lengths, _ = parallel_direct(xy, edges, _xy, _edges, lmin=0.15 * np.median(np.abs(lengths)))
            compression = True
        except ValueError:
            pass
    used = sorted(set(np.array(edges).ravel()) - set(fixed))
    out[name] = {
        "xy": xy.round(4).tolist(), "edges": edges, "fixed": fixed, "_edges": _edges,
        "start": rot90(_xy, -1.0).round(4).tolist(),
        "loop": rot90(loop, -1.0).round(4).tolist(), "iterations": iterations,
        "miss0": float(angles(_xy, _edges, t).max()), "miss": float(angles(loop, _edges, t).max()),
        "loop_angles": angles(loop, _edges, t).round(3).tolist(),
        "direct": rot90(direct, -1.0).round(4).tolist(), "direct_angles": angles(direct, _edges, t).round(3).tolist(),
        "states": int(states), "compression": compression, "free": len(used),
        "valence": sorted(set(int(d) for d in np.bincount(np.array(edges).ravel())[used])),
    }
    print(name, {k: out[name][k] for k in ("free", "valence", "states", "compression", "iterations", "miss0", "miss")})

with open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "patterns.json"), "w") as f:
    json.dump(out, f)
