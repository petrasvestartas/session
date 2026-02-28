"""Debug script to trace triangulation for the loft test case."""
import sys
sys.path.insert(0, 'session_py/src')

from session_py.mesh import Mesh
from session_py.polyline import Polyline
from session_py.point import Point
from session_py.triangulation_2d import _merge_holes, _ear_clip, _signed_area_2d, triangulate
import numpy as np

bottom = [
    Polyline([
        Point(13.20069, -0.556523, -0.178103),
        Point(12.248787, 0.148384, 0.416685),
        Point(12.673247, 2.119511, 1.167431),
        Point(16.910464, 2.961749, 0.289102),
        Point(15.364327, 0.465135, -0.363618),
        Point(15.953685, -1.032727, -1.203717),
        Point(13.20069, -0.556523, -0.178103),
    ]),
    Polyline([
        Point(14.646845, 0.917382, 0.049546),
        Point(14.636404, 1.36458, 0.251429),
        Point(14.660418, 1.595448, 0.346958),
        Point(15.163581, 1.821395, 0.298639),
        Point(15.422988, 1.014296, -0.136839),
        Point(15.068958, 0.91534, -0.07616),
        Point(15.03918, 0.459713, -0.269899),
        Point(14.771618, 0.635281, -0.112748),
        Point(14.646845, 0.917382, 0.049546),
    ]),
    Polyline([
        Point(13.628016, 0.548716, 0.186877),
        Point(13.116088, 0.844297, 0.469625),
        Point(13.114799, 1.185147, 0.621527),
        Point(13.591866, 1.424645, 0.586947),
        Point(13.884637, 1.32996, 0.458299),
        Point(14.013519, 0.88254, 0.2213),
        Point(13.656275, 0.924872, 0.345738),
        Point(13.628016, 0.548716, 0.186877),
    ]),
]
top = [
    Polyline([
        Point(13.375135, -0.818817, 0.411936),
        Point(12.423233, -0.113909, 1.006724),
        Point(12.847692, 1.857217, 1.75747),
        Point(17.084909, 2.699455, 0.879141),
        Point(15.538772, 0.202841, 0.226421),
        Point(16.12813, -1.295021, -0.613678),
        Point(13.375135, -0.818817, 0.411936),
    ]),
    Polyline([
        Point(14.82129, 0.655088, 0.639585),
        Point(14.810849, 1.102286, 0.841468),
        Point(14.834864, 1.333154, 0.936997),
        Point(15.338026, 1.559101, 0.888678),
        Point(15.597433, 0.752002, 0.4532),
        Point(15.243404, 0.653046, 0.513879),
        Point(15.213626, 0.197419, 0.32014),
        Point(14.946063, 0.372987, 0.477291),
        Point(14.82129, 0.655088, 0.639585),
    ]),
    Polyline([
        Point(13.802461, 0.286422, 0.776916),
        Point(13.290534, 0.582003, 1.059664),
        Point(13.289245, 0.922853, 1.211566),
        Point(13.766312, 1.162351, 1.176986),
        Point(14.059082, 1.067666, 1.048338),
        Point(14.187964, 0.620246, 0.811339),
        Point(13.83072, 0.662578, 0.935777),
        Point(13.802461, 0.286422, 0.776916),
    ]),
]

# Find border_idx and projection
def find_border(polylines):
    border_idx = 0
    max_diag = 0
    for i, pl in enumerate(polylines):
        pts = list(pl.points)
        arr = np.array([[p[0], p[1], p[2]] for p in pts])
        bbox_diag = np.linalg.norm(arr.max(axis=0) - arr.min(axis=0))
        if bbox_diag > max_diag:
            max_diag = bbox_diag
            border_idx = i
    return border_idx

border_idx = find_border(bottom)
border_pl = bottom[border_idx]
origin, xaxis, yaxis, zaxis = border_pl.get_average_plane()

def proj(p):
    dx = p[0] - origin[0]; dy = p[1] - origin[1]; dz = p[2] - origin[2]
    u = dx*xaxis[0]+dy*xaxis[1]+dz*xaxis[2]
    v = dx*yaxis[0]+dy*yaxis[1]+dz*yaxis[2]
    return (u, v)

def get_open(pl):
    pts = list(pl.points)
    if len(pts) > 1:
        f, b = pts[0], pts[-1]
        if abs(f[0]-b[0]) < 1e-12 and abs(f[1]-b[1]) < 1e-12 and abs(f[2]-b[2]) < 1e-12:
            pts = pts[:-1]
    return pts

order = [border_idx] + [i for i in range(len(bottom)) if i != border_idx]
all_bot = []; poly_infos = []
for oi, idx in enumerate(order):
    bpts = get_open(bottom[idx]); tpts = get_open(top[idx])
    n = min(len(bpts), len(tpts)); bpts = bpts[:n]; tpts = tpts[:n]
    pts2d = [proj(p) for p in bpts]
    area = sum(pts2d[i][0]*pts2d[(i+1)%n][1] - pts2d[(i+1)%n][0]*pts2d[i][1] for i in range(n)) * 0.5
    if (oi == 0 and area < 0) or (oi != 0 and area > 0):
        bpts = bpts[::-1]
    poly_infos.append((len(all_bot), n))
    all_bot.extend(bpts)

off0, n0 = poly_infos[0]
border_2d = [Point(proj(p)[0], proj(p)[1], 0) for p in all_bot[off0:off0+n0]]
holes_2d = [[Point(proj(p)[0], proj(p)[1], 0) for p in all_bot[off:off+cnt]] for off, cnt in poly_infos[1:]]

print(f"border_2d: {n0} pts, holes: {[len(h) for h in holes_2d]}")
print(f"Total vertices: {n0 + sum(len(h) for h in holes_2d)}")

# Build coords and indices (as triangulate does internally)
coords = []
bn = n0
boundary_indices = list(range(bn))
for i in range(bn):
    coords.append(border_2d[i][0])
    coords.append(border_2d[i][1])
bcoords = [c for i in range(bn) for c in [coords[i*2], coords[i*2+1]]]
if _signed_area_2d(bcoords) < 0:
    boundary_indices.reverse()

hole_indices_list = []
for hole in holes_2d:
    hn = len(hole)
    start = len(coords) // 2
    hidx = list(range(start, start + hn))
    for i in range(hn):
        coords.append(hole[i][0])
        coords.append(hole[i][1])
    hcoords = [c for i in range(hn) for c in [coords[hidx[i]*2], coords[hidx[i]*2+1]]]
    if _signed_area_2d(hcoords) > 0:
        hidx.reverse()
    hole_indices_list.append(hidx)

merged = _merge_holes(coords, boundary_indices, hole_indices_list)
print(f"Merged polygon ({len(merged)} pts): {merged}")

# Build constrained set
constrained = set()
for i in range(bn):
    a, b = boundary_indices[i], boundary_indices[(i+1) % bn]
    constrained.add((a, b)); constrained.add((b, a))
for hidx in hole_indices_list:
    hn = len(hidx)
    for i in range(hn):
        a, b = hidx[i], hidx[(i+1) % hn]
        constrained.add((a, b)); constrained.add((b, a))

print(f"\nConstrained edges: {len(constrained)}")

# Try WITHOUT constrained (original algorithm)
print("\n=== WITHOUT constrained check ===")
tris_no_c = _ear_clip(coords, merged, None, None)
print(f"Triangles: {len(tris_no_c)}")
he_set = set()
dup = []
for t in tris_no_c:
    a, b, c = t
    for u, v in [(a,b),(b,c),(c,a)]:
        if (u, v) in he_set:
            dup.append((u, v, t))
        he_set.add((u, v))
print(f"Duplicate directed halfedges: {len(dup)}")
for d in dup:
    print(f"  dup: ({d[0]},{d[1]}) in tri {d[2]}")

# Try WITH constrained (new algorithm)
print("\n=== WITH constrained check ===")
tris_c = _ear_clip(coords, merged, None, constrained)
print(f"Triangles: {len(tris_c)}")
for t in tris_c:
    print(f"  {t}")

# Show which ears were blocked by constrained check
print("\n=== Blocked ears analysis ===")
# Re-run but show which diagonals are blocked
from session_py.triangulation_2d import _segments_intersect

# Check problematic ear: the one with diagonal crossing a constrained edge
print("\nChecking all possible ears in the merged polygon:")
n_merged = len(merged)
poly_coords = []
for i in range(n_merged):
    poly_coords.append(coords[merged[i]*2])
    poly_coords.append(coords[merged[i]*2+1])
if _signed_area_2d(poly_coords) < 0:
    test_indices = merged[::-1]
else:
    test_indices = list(merged)

blocked_by_constrained = []
for i in range(len(test_indices)):
    p = (i - 1) % len(test_indices)
    nx = (i + 1) % len(test_indices)
    a, c = test_indices[p], test_indices[nx]
    ax, ay = coords[a*2], coords[a*2+1]
    cx, cy = coords[c*2], coords[c*2+1]
    for u, v in constrained:
        if u == a or u == c or v == a or v == c:
            continue
        if _segments_intersect(ax, ay, cx, cy, coords[u*2], coords[u*2+1], coords[v*2], coords[v*2+1]):
            blocked_by_constrained.append((a, test_indices[i], c, u, v))
            break

print(f"Ears blocked by constrained: {len(blocked_by_constrained)}")
for b in blocked_by_constrained:
    a, mid, c, u, v = b
    print(f"  ear({a},{mid},{c}): diagonal ({a},{c}) crosses constrained ({u},{v})")
