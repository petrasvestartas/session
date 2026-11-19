# Thrust network analysis from scratch: numpy and scipy only, no compas.
# Run it: python3 tna_numpy.py. Every function is one step of the slide decks beside it.
#
#   plan (form diagram)  ->  spaces and C*  ->  first guess of the force diagram
#   ->  horizontal equilibrium (five ways)  ->  force densities q  ->  heights z  ->  bar forces
import numpy as np
from scipy.optimize import linprog
from scipy.optimize import lsq_linear


def connectivity(n, edges):
    "C: one row per bar, one column per node, -1 where the bar starts and +1 where it ends."
    C = np.zeros((len(edges), n))
    for e, (u, v) in enumerate(edges):
        C[e, u], C[e, v] = -1.0, 1.0
    return C


def spaces(xy, edges, fixed):
    """The spaces between the bars: for every bar the space on its left and on its right.

    Walk around every space keeping it on the left: arriving at a node, leave along the next bar
    clockwise. The one walk that runs clockwise is the outside; it is cut at every support, and
    each piece between two supports is a space of its own.
    """
    around = {i: [] for i in range(len(xy))}
    for u, v in edges:
        around[u].append(v)
        around[v].append(u)
    for i, nbrs in around.items():
        nbrs.sort(key=lambda j: np.arctan2(xy[j][1] - xy[i][1], xy[j][0] - xy[i][0]))

    left, corners = {}, []
    for start in [(u, v) for u, v in edges] + [(v, u) for u, v in edges]:
        if start in left:
            continue
        walk, (u, v) = [], start
        while (u, v) not in walk:
            walk.append((u, v))
            w = around[v][around[v].index(u) - 1]
            u, v = v, w
        x, y = xy[[a for a, _ in walk]].T
        if (x * np.roll(y, -1) - np.roll(x, -1) * y).sum() > 0:
            pieces = [walk]
        else:
            cuts = [k for k, (a, b) in enumerate(walk) if b in fixed]
            walk = walk[cuts[0] + 1:] + walk[: cuts[0] + 1]
            pieces, piece = [], []
            for a, b in walk:
                piece.append((a, b))
                if b in fixed:
                    pieces.append(piece)
                    piece = []
        for piece in pieces:
            for half in piece:
                left[half] = len(corners)
            corners.append(sorted({a for a, _ in piece} | {b for _, b in piece}))
    sides = [(left[(u, v)], left[(v, u)]) for u, v in edges]
    return sides, corners


def first_guess(xy, corners):
    "One corner of the force diagram at the centroid of every space, turned by 90 degrees: (x, y) -> (-y, x)."
    c = np.array([xy[nodes].mean(axis=0) for nodes in corners])
    return np.column_stack([-c[:, 1], c[:, 0]])


def targets(xy, edges):
    "Unit vector along every bar: the direction its side must have."
    uv = np.array([xy[v] - xy[u] for u, v in edges])
    return uv / np.linalg.norm(uv, axis=1)[:, None]


def miss(X, sides, t):
    "Largest angle between a side and its bar, in degrees. 180 is a flipped side: tension."
    d = np.array([X[b] - X[a] for a, b in sides])
    d /= np.linalg.norm(d, axis=1)[:, None]
    return np.degrees(np.arccos(np.clip((d * t).sum(axis=1), -1.0, 1.0))).max()


def averaging(X, sides, t, kmax=100, tol=1e-3):
    """Rippmann, Lachauer and Block 2012 (RhinoVAULT); compas_tna parallelise_nodal.

    Every corner moves to the average of what its sides ask: the neighbour plus or minus the
    side's own length along its target.
    """
    X = X.copy()
    for k in range(1, kmax + 1):
        X0 = X.copy()
        l = np.array([np.linalg.norm(X0[b] - X0[a]) for a, b in sides])
        total, count = np.zeros_like(X), np.zeros(len(X))
        for e, (a, b) in enumerate(sides):
            total[b] += X0[a] + l[e] * t[e]
            total[a] += X0[b] - l[e] * t[e]
            count[a] += 1
            count[b] += 1
        X = total / count[:, None]
        if miss(X, sides, t) < tol:
            break
    return X, k


def least_squares(X, sides, t, kmax=100, tol=1e-3):
    """Van Mele et al. 2014; compas_tna horizontal_numpy: all corners at once, corner 0 held.

    Solves C*^T C* x* = C*^T (l* t) in every iteration; the matrix never changes.
    """
    X = X.copy()
    Cs = connectivity(len(X), sides)
    L = Cs.T @ Cs
    for k in range(1, kmax + 1):
        l = np.linalg.norm(Cs @ X, axis=1)[:, None]
        rhs = Cs.T @ (l * t)
        X[1:] = np.linalg.solve(L[1:, 1:], rhs[1:] - L[1:, [0]] @ X[[0]])
        if miss(X, sides, t) < tol:
            break
    return X, k


def equilibrium_matrix(xy, edges, free):
    "A of A q = 0: for every free node one row along x and one along y (Block and Lachauer 2014)."
    C = connectivity(len(xy), edges)
    uv = C @ xy
    return np.vstack([C[:, free].T * uv[:, 0], C[:, free].T * uv[:, 1]])


def independent(A, tol=1e-9):
    "Row reduction of A: the columns without a pivot are the bars whose force can be chosen."
    R, pivots, row = A.copy(), [], 0
    for col in range(R.shape[1]):
        if row == len(R):
            break
        k = row + np.abs(R[row:, col]).argmax()
        if abs(R[k, col]) < tol:
            continue
        R[[row, k]] = R[[k, row]]
        R[row] /= R[row, col]
        for other in range(len(R)):
            if other != row:
                R[other] -= R[other, col] * R[row]
        pivots.append(col)
        row += 1
    return [e for e in range(R.shape[1]) if e not in pivots], pivots


def densities_from_independent(A, q_indep):
    "Block and Lachauer 2014: q_dep = -A_dep^-1 A_indep q_indep."
    indep, dep = independent(A)
    q = np.zeros(A.shape[1])
    q[indep] = q_indep
    q[dep] = np.linalg.lstsq(A[:, dep], -A[:, indep] @ q[indep], rcond=None)[0]
    return q


def densities_lp(A, lengths, d=1.0):
    "Block 2009: minimise the sum of the ratios t = l*/l with every node closed and every side at least d long."
    res = linprog(np.ones(A.shape[1]), A_eq=A, b_eq=np.zeros(len(A)), bounds=[(d / l, None) for l in lengths], method="highs")
    if res.status != 0:
        raise ValueError("no compression-only force diagram exists for this plan")
    return res.x


def densities_bounded(A, qmin=1.0, qmax=50.0):
    "Vouga, Hoebinger, Wallner and Pottmann 2012, step 2: the best fit with every q between two bounds."
    q = lsq_linear(A, np.zeros(len(A)), bounds=(qmin, qmax), tol=1e-14).x
    return q, np.abs(A @ q).max()


def diagram(xy, edges, sides, q):
    "The force diagram of force densities q: every side is its bar times q (least squares, centred)."
    Cs = connectivity(max(max(s) for s in sides) + 1, sides)
    uv = np.array([xy[v] - xy[u] for u, v in edges])
    X = np.linalg.lstsq(Cs, q[:, None] * uv, rcond=None)[0]
    return X - X.mean(axis=0)


def densities(xy, edges, X, sides, r=1.0):
    "q = l* / (r l): length of the side over length of the bar, divided by the scale (7.10)."
    l = np.array([np.linalg.norm(xy[v] - xy[u]) for u, v in edges])
    ls = np.array([np.linalg.norm(X[b] - X[a]) for a, b in sides])
    return ls / l / r


def heights(xy, edges, free, fixed, q, p, zF=0.0, r=1.0):
    "Vertical equilibrium (7.12): z_N = D_N^-1 (r p - D_F z_F), with D = C_N^T diag(q) C."
    C = connectivity(len(xy), edges)
    D = C[:, free].T @ (q[:, None] * C)
    z = np.full(len(xy), float(zF))
    z[free] = np.linalg.solve(D[:, free], r * p - D[:, fixed] @ z[fixed])
    return z


def forces(xy, z, edges, q):
    "Force in every bar (7.14): force density times length in space."
    xyz = np.column_stack([xy, z])
    return q * np.array([np.linalg.norm(xyz[v] - xyz[u]) for u, v in edges])


if __name__ == "__main__":
    np.set_printoptions(precision=3, suppress=True)

    # the plan of the horizontal-equilibrium deck: free nodes 0..3, supports 4..11
    xy = np.array([[1.3, 1.6], [4.9, 2.2], [3.7, 4.8], [2.0, 3.5],
                   [1.2, 0.0], [4.9, 0.0], [6.0, 2.9], [6.0, 4.9], [3.3, 6.0], [2.3, 6.0], [0.0, 4.0], [0.0, 1.4]])
    edges = [(4, 0), (5, 1), (11, 0), (0, 1), (0, 3), (1, 6), (1, 2), (10, 3), (3, 2), (3, 9), (2, 7), (2, 8)]
    free, fixed = [0, 1, 2, 3], list(range(4, 12))

    sides, corners = spaces(xy, edges, fixed)
    t = targets(xy, edges)
    X0 = first_guess(xy, corners)
    print("spaces:", len(corners), " sides:", len(sides), " first guess misses by %.1f degrees" % miss(X0, sides, t))

    A = equilibrium_matrix(xy, edges, free)
    indep, dep = independent(A)
    lengths = np.linalg.norm(connectivity(len(xy), edges) @ xy, axis=1)
    print("A is %d x %d, rank %d: %d bars can be chosen freely" % (A.shape + (len(dep), len(indep))))

    Xa, ka = averaging(X0, sides, t)
    Xl, kl = least_squares(X0, sides, t)
    q_lp = densities_lp(A, lengths)
    q_in = densities_from_independent(A, densities(xy, edges, X0, sides)[indep])
    q_bf, left = densities_bounded(A)
    answers = {
        "averaging (%d iterations)" % ka: densities(xy, edges, Xa, sides),
        "least squares (%d iterations)" % kl: densities(xy, edges, Xl, sides),
        "linear programme": q_lp,
        "independent q": q_in,
        "bounded fit": q_bf,
    }
    for name, q in answers.items():
        X = diagram(xy, edges, sides, q)
        print("%-30s unbalanced %.1e  miss %.4f deg  q of bar 0-1 over mean %.2f" % (name, np.abs(A @ q).max(), miss(X, sides, t), q[3] / q.mean()))

    q = answers["averaging (%d iterations)" % ka]
    z = heights(xy, edges, free, fixed, q, p=np.ones(len(free)))
    print("heights of the free nodes:", z[free], " largest bar force: %.2f" % forces(xy, z, edges, q).max())

    # the nine-node network of Figure 7.9 of Shell Structures for Architecture, with the book's t
    xy = np.array([[0, 0], [1, 0], [0, 1], [-1, 0], [0, -1], [1.8, 0], [0, 1.8], [-1.8, 0], [0, -1.8]], dtype=float)
    edges = [(0, 1), (0, 2), (0, 3), (0, 4), (1, 2), (2, 3), (3, 4), (1, 4), (1, 5), (2, 6), (3, 7), (4, 8)]
    free, fixed = [0, 1, 2, 3, 4], [5, 6, 7, 8]
    q = np.array([2, 2, 2, 2, 1, 1, 1, 1, 5, 5, 5, 5], dtype=float)
    print("\nFigure 7.9: unbalanced %.1e" % np.abs(equilibrium_matrix(xy, edges, free) @ q).max())
    z = heights(xy, edges, free, fixed, q, p=np.ones(5))
    print("z_N =", z[free], " (book: 0.375, 0.25, 0.25, 0.25, 0.25)")
    print("s   =", forces(xy, z, edges, q), " (book: 2.016, 1.414, 4.191)")
