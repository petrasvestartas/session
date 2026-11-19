# Horizontal equilibrium with the form diagram fixed, without the averaging loop (numpy and scipy only).
# The force diagram is in the turned frame of compas_tna: every side should be parallel to its bar.
#
# A side (u, v) with target t is parallel when its part across t vanishes: n . (x_v - x_u) = 0, n = t turned by 90.
# These are m linear equations A x = 0 in the 2 n* corner coordinates. Their solutions are all parallel diagrams:
# two of them slide the diagram, the others are the states of self-stress of the form diagram.
import numpy as np
from scipy.linalg import null_space
from scipy.optimize import linprog
from scipy.optimize import lsq_linear
from scipy.optimize import minimize
from scipy.sparse import coo_matrix
from scipy.sparse import identity
from scipy.sparse import vstack
from scipy.sparse.linalg import splu


def targets(xy, edges):
    "Unit vector along every bar of the form diagram."
    uv = np.array([xy[v] - xy[u] for u, v in edges])
    return uv / np.linalg.norm(uv, axis=1)[:, None]


def matrices(n, _edges, t):
    "A: the part of every side across its target. L: its signed length along the target. Both act on [x0 y0 x1 y1 ...]."
    m = len(_edges)
    normal = np.column_stack([-t[:, 1], t[:, 0]])
    rows = np.repeat(np.arange(m), 4)
    cols = np.array([[2 * u, 2 * u + 1, 2 * v, 2 * v + 1] for u, v in _edges]).ravel()
    across = np.column_stack(
        [-normal[:, 0], -normal[:, 1], normal[:, 0], normal[:, 1]]
    ).ravel()
    along = np.column_stack([-t[:, 0], -t[:, 1], t[:, 0], t[:, 1]]).ravel()
    A = coo_matrix((across, (rows, cols)), shape=(m, 2 * n)).tocsr()
    L = coo_matrix((along, (rows, cols)), shape=(m, 2 * n)).tocsr()
    return A, L


def angles(_xy, _edges, t):
    "Angle between every side and its target in degrees: 180 is a flipped side, a bar in tension."
    d = np.array([_xy[v] - _xy[u] for u, v in _edges])
    d /= np.maximum(np.linalg.norm(d, axis=1), 1e-300)[:, None]
    return np.degrees(np.arccos(np.clip((d * t).sum(axis=1), -1.0, 1.0)))


def parallel_direct(xy, edges, _xy, _edges, lmin=None):
    """The parallel force diagram closest to the guess _xy, in one solve.

    Returns the corners, the signed side lengths (the horizontal forces) and the number of free
    states of self-stress. With lmin, every length is kept at or above it: compression only.
    Zero states means no parallel diagram exists for this form diagram.
    """
    t = targets(xy, edges)
    A, L = matrices(len(_xy), _edges, t)
    N = null_space(A.toarray())
    states = N.shape[1] - 2
    x0 = _xy.ravel()
    z0 = N.T @ x0
    z = z0
    LN = L @ N

    if lmin is not None and states > 0 and (LN @ z0).min() < lmin:
        m = LN.shape[0]
        bounds = [(None, None)] * N.shape[1]
        possible = linprog(np.zeros(N.shape[1]), A_ub=-LN, b_ub=-np.full(m, lmin), bounds=bounds, method="highs")
        if possible.status != 0:
            raise ValueError("no parallel diagram in compression exists for this form diagram")
        # closest to the guess with every length >= lmin, solved through its dual: multipliers >= 0
        b = lmin - LN @ z0
        dual = minimize(
            lambda y: (0.5 * ((LN.T @ y) ** 2).sum() - y @ b, LN @ (LN.T @ y) - b),
            np.zeros(m),
            jac=True,
            bounds=[(0.0, None)] * m,
            method="L-BFGS-B",
            options={"maxiter": 20000, "maxfun": 200000, "ftol": 1e-15, "gtol": 1e-10},
        )
        z = z0 + LN.T @ dual.x

    return (N @ z).reshape((-1, 2)), LN @ z, states


def parallel_loop(xy, edges, _xy, _edges, kmax=100, tol=1e-3, window=5, fixed=0):
    """The loop of horizontal_numpy with a stop at tol degrees and Anderson acceleration.

    Every iteration turns each side onto its target keeping its length, then solves for the corners
    that fit all sides best. window = 0 is the plain loop. Returns the corners and the iterations used.
    """
    t = targets(xy, edges)
    n, m = len(_xy), len(_edges)
    rows = np.repeat(np.arange(m), 2)
    cols = np.array(_edges).ravel()
    C = coo_matrix((np.tile([-1.0, 1.0], m), (rows, cols)), shape=(m, n)).tocsr()
    free = [i for i in range(n) if i != fixed]
    Ct = C.T.tocsr()
    lu = splu(
        (Ct @ C)[free, :][:, free].tocsc() + 0 * identity(len(free), format="csc")
    )
    Cf = (Ct @ C)[free, :][:, [fixed]].toarray()

    def step(x):
        l = np.linalg.norm(C @ x, axis=1)[:, None]
        out = x.copy()
        out[free] = lu.solve((Ct @ (l * t))[free] - Cf @ x[[fixed]])
        return out

    x = _xy.copy()
    X, F = [], []
    for k in range(1, kmax + 1):
        g = step(x)
        if not window:
            x = g
        else:
            X.append(g.ravel())
            F.append((g - x).ravel())
            X, F = X[-window - 1 :], F[-window - 1 :]
            if len(F) > 1:
                dF = np.array([F[i + 1] - F[i] for i in range(len(F) - 1)]).T
                dX = np.array([X[i + 1] - X[i] for i in range(len(X) - 1)]).T
                gamma = np.linalg.lstsq(dF, F[-1], rcond=None)[0]
                x = (X[-1] - dX @ gamma).reshape((-1, 2))
            else:
                x = g
        if angles(x, _edges, t).max() < tol:
            return x, k
    return x, kmax


def equilibrium_matrix(xy, edges, free):
    "A of A q = 0: two rows per free node, one column per bar (Block and Lachauer 2014, eq. 7)."
    m = len(edges)
    rows = np.repeat(np.arange(m), 2)
    cols = np.array(edges).ravel()
    C = coo_matrix((np.tile([-1.0, 1.0], m), (rows, cols)), shape=(m, len(xy))).tocsc()
    uv = C @ xy
    Ci = C[:, free]
    return vstack([Ci.T.multiply(uv[:, 0]), Ci.T.multiply(uv[:, 1])]).tocsr(), C


def bounded_densities(xy, edges, free, qmin=1.0, qmax=50.0):
    """Force densities that fit horizontal equilibrium best, each between qmin and qmax.

    Step 2 of Vouga, Hoebinger, Wallner and Pottmann 2012, kept to the plan: a convex,
    box-constrained least-squares problem that always has a solution. Returns q and the largest
    unbalanced force on a node divided by the sum of the forces meeting there (0 = equilibrium).
    """
    A, C = equilibrium_matrix(xy, edges, free)
    q = lsq_linear(A, np.zeros(A.shape[0]), bounds=(qmin, qmax), tol=1e-14, max_iter=500).x
    r = (A @ q).reshape((2, -1))
    force = q * np.linalg.norm(C @ xy, axis=1)
    return q, (np.linalg.norm(r, axis=0) / (abs(C[:, free]).T @ force)).max()


def self_supporting_plan(xy, edges, fixed, qmin=1.0, qmax=50.0, kmax=300, tol=1e-6):
    """Horizontal equilibrium that may move the plan, after Vouga et al. 2012, split from the heights.

    Repeats: fit bounded force densities to the plan (their step 2), then move the free nodes to
    reduce what is left unbalanced, with a penalty on leaving their place that fades as the error
    falls (their step 3). Returns the plan, q, the iterations used and the error after each.
    """
    xy = np.array(xy, dtype=float)
    free = [i for i in range(len(xy)) if i not in set(fixed) and i in set(np.array(edges).ravel())]
    q, error = bounded_densities(xy, edges, free, qmin, qmax)
    history = [error]
    beta0 = None
    for k in range(1, kmax + 1):
        if error < tol:
            return xy, q, k - 1, history
        _, C = equilibrium_matrix(xy, edges, free)
        D = (C[:, free].T.multiply(q)) @ C
        DN, DF = D[:, free].tocsc(), D[:, fixed]
        if beta0 is None:
            beta0 = 0.1 * DN.diagonal().mean() ** 2
        beta = beta0 * error / history[0]
        lhs = (DN.T @ DN + beta * identity(len(free))).tocsc()
        xy[free] = splu(lhs).solve(-(DN.T @ (DF @ xy[fixed])) + beta * xy[free])
        q, error = bounded_densities(xy, edges, free, qmin, qmax)
        history.append(error)
    return xy, q, kmax, history


def heights(xy, edges, fixed, q, load, zfixed=0.0, r=1.0):
    "Vertical equilibrium: z_N = D_N^-1 (r p - D_F z_F), equation 7.12 of Shell Structures for Architecture."
    free = [i for i in range(len(xy)) if i not in set(fixed)]
    _, C = equilibrium_matrix(np.array(xy, dtype=float), edges, free)
    D = (C[:, free].T.multiply(q)) @ C
    z = np.full(len(xy), float(zfixed))
    z[free] = splu(D[:, free].tocsc()).solve(r * np.full(len(free), float(load)) - D[:, fixed] @ z[fixed])
    return z
