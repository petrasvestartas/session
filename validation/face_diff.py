"""Face-level oracle differ for BRep boolean results vs OCCT ground truth.

Given operands A.step/B.step, an op (cut|common|fuse) and OUR result STEP, this
tool asks the OCCT oracle (validation/step_probe) for the TRUE boolean summary
(`step_probe --<op> A B` -> OP_SOLIDS/OP_FACES/OP_VOLUME/OP_VALID) and diffs our
result against it at FACE level. step_probe cannot dump the true result's faces
(the --cut/--common/--fuse mode prints counts only), so per-face verdicts use
the documented fallback: `--inside` point classification against the EXACT
operand solids. Every face is sampled at UV points (from step_probe's verbose
per-face UV bounds + our own STEP surface evaluator); each sample is offset
+/- eps along the surface normal and classified against A and B; membership in
the TRUE result follows from the boolean formula (cut = A && !B, etc):
  - an OUR-result face is genuine iff one offset lands IN the true result and
    the other OUT (boundary transition); all-non-boundary => EXTRA(interior/
    exterior). Because samples cover the UV bbox while the trimmed face is only
    area/bboxarea of it, verdicts are trim-aware: the measured boundary
    fraction must reach the face's own bbox share, else OVERSHOOT (face keeps
    region the truth cut away). Statistically undecided faces get an adaptive
    second pass at 25 samples;
  - a TRUE face is a region of an operand face where the same transition test
    passes; it is MISSING from ours when no our-face sample cloud covers it;
  - AREA tickets compare our area vs the operand face area for faces the
    sampling says are kept whole (all samples on the true boundary);
  - AGG_AREA checks boundary-area conservation (our exact total vs the sampled
    expectation; >12% flags wrongly kept regions, SUSPECT_KEPT ranks them);
  - the oracle itself is sanity-checked: vol(A)-cut == common == vol(A)+vol(B)
    -fuse; a violation (e.g. chairs z15, where OCCT silently disagrees with
    itself) prints an OCCT-self-inconsistent WARNING.
Probing is multi-scale (eps, eps/4, eps/16): a sample counts as non-boundary
only if every scale agrees, so thin features are not punched through.

Usage:
  python validation/face_diff.py --config z15 --op cut            # chair paths
  python validation/face_diff.py --op cut --a A.step --b B.step --result R.step
Options: --report out.md  --samples N  --grid N  --eps X  --probe exe  --pick
         green|all|<idx> (multi-solid result files: which solid is the result)
"""
import argparse
import math
import os
import re
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
PROBE = os.path.join(HERE, "step_probe", "build", "Release", "step_probe.exe")
if not os.path.exists(PROBE):
    PROBE = os.path.join(HERE, "step_probe", "build", "step_probe")
CHAIRS = os.path.normpath(os.path.join(
    HERE, "..", "session_cpp", "serialization", "boolean_steps", "chairs"))

# ---------------------------------------------------------------- STEP parsing

_NUM = re.compile(r"^[-+]?(?:[0-9]+\.?[0-9]*|\.[0-9]+)(?:[eE][-+]?[0-9]+)?$")


def _tokenize(s):
    toks, i, n = [], 0, len(s)
    while i < n:
        c = s[i]
        if c.isspace():
            i += 1
        elif c in "(),":
            toks.append(c)
            i += 1
        elif c == "'":
            j = i + 1
            while j < n:
                if s[j] == "'" and j + 1 < n and s[j + 1] == "'":
                    j += 2
                elif s[j] == "'":
                    break
                else:
                    j += 1
            toks.append(("STR", s[i + 1:j]))
            i = j + 1
        elif c == "#":
            j = i + 1
            while j < n and s[j].isdigit():
                j += 1
            toks.append(("REF", int(s[i + 1:j])))
            i = j
        elif c == ".":
            j = s.index(".", i + 1)
            toks.append(("ENUM", s[i + 1:j]))
            i = j + 1
        else:
            j = i
            while j < n and s[j] not in "(),' \t" and s[j] != "#":
                j += 1
            t = s[i:j].strip()
            if _NUM.match(t):
                toks.append(("NUM", float(t)))
            else:
                toks.append(("SYM", t))
            i = j
    return toks


def _parse_list(toks, i):
    # toks[i] == '(' ; returns (list, next_index)
    out = []
    i += 1
    while toks[i] != ")":
        if toks[i] == ",":
            i += 1
            continue
        if toks[i] == "(":
            sub, i = _parse_list(toks, i)
            out.append(sub)
        elif isinstance(toks[i], tuple) and toks[i][0] == "SYM" and i + 1 < len(toks) and toks[i + 1] == "(":
            sub, i2 = _parse_list(toks, i + 1)
            out.append((toks[i][1], sub))
            i = i2
        else:
            out.append(toks[i])
            i += 1
    return out, i + 1


class Step:
    def __init__(self, path):
        self.path = path
        text = open(path, "r", errors="replace").read()
        data = text[text.index("DATA;") + 5:]
        self.ent = {}      # id -> (TYPE, args) or ("COMPLEX", [(TYPE,args),..])
        self.order = []
        for stmt in data.split(";"):
            stmt = stmt.strip().replace("\n", " ").replace("\r", " ")
            m = re.match(r"^#(\d+)\s*=\s*(.*)$", stmt, re.S)
            if not m:
                continue
            eid, body = int(m.group(1)), m.group(2).strip()
            toks = _tokenize(body)
            if not toks:
                continue
            if toks[0] == "(":                       # complex instance
                parts, i = [], 1
                while i < len(toks) and toks[i] != ")":
                    if isinstance(toks[i], tuple) and toks[i][0] == "SYM" and toks[i + 1] == "(":
                        args, i2 = _parse_list(toks, i + 1)
                        parts.append((toks[i][1], args))
                        i = i2
                    else:
                        i += 1
                self.ent[eid] = ("COMPLEX", parts)
            elif isinstance(toks[0], tuple) and toks[0][0] == "SYM" and len(toks) > 1 and toks[1] == "(":
                args, _ = _parse_list(toks, 1)
                self.ent[eid] = (toks[0][1], args)
            else:
                continue
            self.order.append(eid)

    def typ(self, eid):
        return self.ent[eid][0]

    def args(self, eid):
        return self.ent[eid][1]

    def refs_of(self, eid):
        out = []

        def walk(a):
            for x in a:
                if isinstance(x, tuple) and x[0] == "REF":
                    out.append(x[1])
                elif isinstance(x, list):
                    walk(x)
                elif isinstance(x, tuple) and len(x) == 2 and isinstance(x[1], list):
                    walk(x[1])
        walk(self.args(eid))
        return out


def _ref(x):
    return x[1] if isinstance(x, tuple) and x[0] == "REF" else None


def _num(x):
    return x[1] if isinstance(x, tuple) and x[0] == "NUM" else None


# ------------------------------------------------------------- surface models

class BSurf:
    """Non-rational B_SPLINE_SURFACE_WITH_KNOTS (rational complex also handled,
    weights folded in)."""

    def __init__(self, step, eid):
        t = step.ent[eid]
        parts = t[1] if t[0] != "COMPLEX" else None
        if t[0] == "COMPLEX":
            d = dict(t[1])
            base = d.get("B_SPLINE_SURFACE", [])
            knots = d.get("B_SPLINE_SURFACE_WITH_KNOTS", [])
            self.du, self.dv = int(_num(base[0])), int(_num(base[1]))
            grid = base[2]
            um, vm, uk, vk = knots[0], knots[1], knots[2], knots[3]
            rat = d.get("RATIONAL_B_SPLINE_SURFACE")
            self.w = [[_num(x) for x in row] for row in rat[0]] if rat else None
        else:
            a = t[1]
            self.du, self.dv = int(_num(a[1])), int(_num(a[2]))
            grid = a[3]
            um, vm, uk, vk = a[8], a[9], a[10], a[11]
            self.w = None
        self.poles = []
        for row in grid:
            prow = []
            for x in row:
                cp = step.args(_ref(x))
                prow.append(tuple(_num(c) for c in cp[1]))
            self.poles.append(prow)
        self.uk = self._expand(uk, um)
        self.vk = self._expand(vk, vm)
        self.typ = "Geom_BSplineSurface"

    @staticmethod
    def _expand(knots, mults):
        out = []
        for k, m in zip(knots, mults):
            out.extend([_num(k)] * int(_num(m)))
        return out

    def domain(self):
        du, dv = self.du, self.dv
        return (self.uk[du], self.uk[len(self.uk) - du - 1],
                self.vk[dv], self.vk[len(self.vk) - dv - 1])

    @staticmethod
    def _span(kv, deg, t):
        n = len(kv) - deg - 2
        if t >= kv[n + 1]:
            return n
        if t <= kv[deg]:
            return deg
        lo, hi = deg, n + 1
        while hi - lo > 1:
            mid = (lo + hi) // 2
            if t < kv[mid]:
                hi = mid
            else:
                lo = mid
        return lo

    @staticmethod
    def _basis(kv, deg, span, t):
        N = [1.0] + [0.0] * deg
        left = [0.0] * (deg + 1)
        right = [0.0] * (deg + 1)
        for j in range(1, deg + 1):
            left[j] = t - kv[span + 1 - j]
            right[j] = kv[span + j] - t
            saved = 0.0
            for r in range(j):
                den = right[r + 1] + left[j - r]
                tmp = N[r] / den if den != 0 else 0.0
                N[r] = saved + right[r + 1] * tmp
                saved = left[j - r] * tmp
            N[j] = saved
        return N

    def point(self, u, v):
        su = self._span(self.uk, self.du, u)
        sv = self._span(self.vk, self.dv, v)
        Nu = self._basis(self.uk, self.du, su, u)
        Nv = self._basis(self.vk, self.dv, sv, v)
        x = y = z = wsum = 0.0
        for i in range(self.du + 1):
            pi = su - self.du + i
            for j in range(self.dv + 1):
                pj = sv - self.dv + j
                w = Nu[i] * Nv[j] * (self.w[pi][pj] if self.w else 1.0)
                p = self.poles[pi][pj]
                x += w * p[0]
                y += w * p[1]
                z += w * p[2]
                wsum += w
        if self.w:
            x, y, z = x / wsum, y / wsum, z / wsum
        return (x, y, z)


class Analytic:
    """PLANE / CYLINDRICAL / CONICAL / SPHERICAL / TOROIDAL via AXIS2_PLACEMENT_3D,
    OCCT parameterization."""
    KIND = {"PLANE": "Geom_Plane", "CYLINDRICAL_SURFACE": "Geom_CylindricalSurface",
            "CONICAL_SURFACE": "Geom_ConicalSurface", "SPHERICAL_SURFACE": "Geom_SphericalSurface",
            "TOROIDAL_SURFACE": "Geom_ToroidalSurface"}

    def __init__(self, step, eid):
        kind, a = step.ent[eid]
        self.kind = kind
        self.typ = self.KIND[kind]
        loc, ax, refd = None, (0, 0, 1), (1, 0, 0)
        pl = step.args(_ref(a[1]))
        loc = tuple(_num(c) for c in step.args(_ref(pl[1]))[1])
        if _ref(pl[2]) is not None:
            ax = tuple(_num(c) for c in step.args(_ref(pl[2]))[1])
        if len(pl) > 3 and _ref(pl[3]) is not None:
            refd = tuple(_num(c) for c in step.args(_ref(pl[3]))[1])
        z = _norm(ax)
        x = _norm(_sub(refd, _scale(z, _dot(refd, z))))
        y = _cross(z, x)
        self.loc, self.x, self.y, self.z = loc, x, y, z
        self.p = [_num(v) for v in a[2:] if _num(v) is not None]

    def domain(self):
        big = 1e3
        if self.kind == "PLANE":
            return (-big, big, -big, big)
        if self.kind == "CYLINDRICAL_SURFACE":
            return (0, 2 * math.pi, -big, big)
        if self.kind == "CONICAL_SURFACE":
            return (0, 2 * math.pi, -big, big)
        if self.kind == "SPHERICAL_SURFACE":
            return (0, 2 * math.pi, -math.pi / 2, math.pi / 2)
        return (0, 2 * math.pi, 0, 2 * math.pi)

    def point(self, u, v):
        L, X, Y, Z = self.loc, self.x, self.y, self.z
        if self.kind == "PLANE":
            return _add(L, _add(_scale(X, u), _scale(Y, v)))
        if self.kind == "CYLINDRICAL_SURFACE":
            r = self.p[0]
            return _add(L, _add(_scale(X, r * math.cos(u)),
                                _add(_scale(Y, r * math.sin(u)), _scale(Z, v))))
        if self.kind == "CONICAL_SURFACE":
            r, sa = self.p[0], self.p[1]
            rr = r + v * math.sin(sa)
            return _add(L, _add(_scale(X, rr * math.cos(u)),
                                _add(_scale(Y, rr * math.sin(u)), _scale(Z, v * math.cos(sa)))))
        if self.kind == "SPHERICAL_SURFACE":
            r = self.p[0]
            return _add(L, _add(_scale(X, r * math.cos(v) * math.cos(u)),
                                _add(_scale(Y, r * math.cos(v) * math.sin(u)),
                                     _scale(Z, r * math.sin(v)))))
        R, r = self.p[0], self.p[1]
        rr = R + r * math.cos(v)
        return _add(L, _add(_scale(X, rr * math.cos(u)),
                            _add(_scale(Y, rr * math.sin(u)), _scale(Z, r * math.sin(v)))))


def _add(a, b):
    return (a[0] + b[0], a[1] + b[1], a[2] + b[2])


def _sub(a, b):
    return (a[0] - b[0], a[1] - b[1], a[2] - b[2])


def _scale(a, s):
    return (a[0] * s, a[1] * s, a[2] * s)


def _dot(a, b):
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]


def _cross(a, b):
    return (a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0])


def _norm(a):
    l = math.sqrt(_dot(a, a))
    return _scale(a, 1.0 / l) if l > 0 else a


def make_surface(step, eid):
    t = step.typ(eid)
    if t in Analytic.KIND:
        return Analytic(step, eid)
    if t == "B_SPLINE_SURFACE_WITH_KNOTS" or t == "COMPLEX":
        return BSurf(step, eid)
    raise ValueError("unsupported surface %s at #%d in %s" % (t, eid, step.path))


# --------------------------------------------------------- shape-unit extraction

SHELLS = ("CLOSED_SHELL", "OPEN_SHELL")
REPS = ("ADVANCED_BREP_SHAPE_REPRESENTATION", "MANIFOLD_SURFACE_SHAPE_REPRESENTATION",
        "SHAPE_REPRESENTATION", "GEOMETRICALLY_BOUNDED_SURFACE_SHAPE_REPRESENTATION")


def shape_units(step):
    """[(unit_id, kind, [face ids])] in representation order. A unit is one
    MANIFOLD_SOLID_BREP or SHELL_BASED_SURFACE_MODEL (or a bare shell)."""
    units, seen = [], set()

    def faces_of_shell(sid):
        return [_ref(x) for x in step.args(sid)[1]]

    def add_unit(uid):
        if uid in seen:
            return
        t = step.typ(uid)
        if t == "MANIFOLD_SOLID_BREP":
            seen.add(uid)
            units.append((uid, "solid", faces_of_shell(_ref(step.args(uid)[1]))))
        elif t == "SHELL_BASED_SURFACE_MODEL":
            seen.add(uid)
            fl = []
            for sh in step.args(uid)[1]:
                fl.extend(faces_of_shell(_ref(sh)))
            units.append((uid, "shell", fl))
        elif t in SHELLS:
            seen.add(uid)
            units.append((uid, "shell", faces_of_shell(uid)))

    for eid in step.order:
        if step.typ(eid) in REPS:
            for it in step.args(eid)[1]:
                r = _ref(it)
                if r is not None:
                    add_unit(r)
    return units


def face_colors(step):
    """face id -> (r,g,b) via STYLED_ITEM chains (item may be a face or a solid)."""
    out = {}
    for eid in step.order:
        if step.typ(eid) != "STYLED_ITEM":
            continue
        a = step.args(eid)
        target = _ref(a[2])
        rgb = None
        stack = [_ref(x) for x in a[1] if _ref(x) is not None]
        visited = set()
        while stack and rgb is None:
            e = stack.pop()
            if e in visited or e not in step.ent:
                continue
            visited.add(e)
            if step.typ(e) == "COLOUR_RGB":
                c = step.args(e)
                rgb = (_num(c[1]), _num(c[2]), _num(c[3]))
                break
            stack.extend(step.refs_of(e))
        if rgb is None or target is None:
            continue
        if step.typ(target) == "MANIFOLD_SOLID_BREP":
            for f in shape_units(step):
                if f[0] == target:
                    for fid in f[2]:
                        out[fid] = rgb
        else:
            out[target] = rgb
    return out


def pick_result_faces(step, pick):
    units = shape_units(step)
    if not units:
        raise ValueError("no solids/shells found in " + step.path)
    if len(units) == 1 or pick == "all":
        chosen = units
    elif pick == "green":
        cols = face_colors(step)
        chosen = []
        for u in units:
            cs = [cols[f] for f in u[2] if f in cols]
            if cs and all(c[1] > c[0] and c[1] > c[2] for c in cs[:1]):
                chosen.append(u)
        if not chosen:
            chosen = units
    elif pick.isdigit():
        chosen = [units[int(pick)]]
    else:  # auto: prefer green if colored multi-unit, else all
        cols = face_colors(step)
        if cols and len(units) > 1:
            return pick_result_faces(step, "green")
        chosen = units
    faces = []
    for u in chosen:
        faces.extend(u[2])
    return faces, chosen


# ------------------------------------------------------------------ oracle I/O

def run_probe(args):
    r = subprocess.run([PROBE] + args, capture_output=True, text=True)
    if r.returncode != 0 and "READ_FAIL" in (r.stdout or ""):
        raise RuntimeError("step_probe READ_FAIL: %s" % args)
    return r.stdout


def probe_summary(path):
    out = run_probe([path])
    d = {}
    for k in ("ROOTS", "SOLIDS", "SHELLS", "FACES", "VOLUME", "VALID"):
        m = re.search(r"^%s ([-\d.]+)" % k, out, re.M)
        d[k.lower()] = float(m.group(1)) if k == "VOLUME" else int(m.group(1))
    return d


def probe_faces(path):
    out = run_probe([path, "v"])
    faces = []
    for m in re.finditer(
            r"^FACE (\d+) area ([-\d.eE+]+) orient (\w+) srf (\S+) "
            r"uv \[([-\d.eE+]+),([-\d.eE+]+)\]x\[([-\d.eE+]+),([-\d.eE+]+)\] wires (\d+) edges (\d+)",
            out, re.M):
        faces.append({"i": int(m.group(1)), "area": float(m.group(2)),
                      "orient": m.group(3), "srf": m.group(4),
                      "uv": tuple(float(m.group(k)) for k in range(5, 9)),
                      "wires": int(m.group(9)), "edges": int(m.group(10))})
    return faces


def probe_boolean(op, a, b):
    out = run_probe(["--" + op, a, b])
    d = {}
    for k in ("OP_SOLIDS", "OP_FACES", "OP_VOLUME", "OP_VALID"):
        m = re.search(r"^%s ([-\d.]+)" % k, out, re.M)
        if not m:
            raise RuntimeError("oracle boolean gave no %s:\n%s" % (k, out))
        d[k[3:].lower()] = float(m.group(1)) if "VOL" in k else int(m.group(1))
    return d


def classify(path, pts):
    """IN/OUT/ON per point against the solid in path (chunked --inside)."""
    states = []
    CH = 250
    for i in range(0, len(pts), CH):
        chunk = pts[i:i + CH]
        args = ["--inside", path]
        for p in chunk:
            args.extend(["%.9g" % c for c in p])
        out = run_probe(args)
        got = re.findall(r"-> (IN|OUT|ON)$", out, re.M)
        if len(got) != len(chunk):
            raise RuntimeError("--inside returned %d/%d states for %s" %
                               (len(got), len(chunk), path))
        states.extend(got)
    return states


# ------------------------------------------------------------------- sampling

def tri(a, b, f):
    return a + (b - a) * f


def sample_fracs(n):
    g = max(2, math.ceil(math.sqrt(n)))
    return [((i + 0.5) / g, (j + 0.5) / g) for i in range(g) for j in range(g)][:n]


def face_samples(surf, uv, nsamples):
    u0, u1, v0, v1 = uv
    du, dv = u1 - u0, v1 - v0
    hu, hv = max(du, 1e-9) * 1e-4, max(dv, 1e-9) * 1e-4
    out = []
    for fu, fv in sample_fracs(nsamples):
        u, v = tri(u0, u1, fu), tri(v0, v1, fv)
        p = surf.point(u, v)
        pu = _sub(surf.point(u + hu, v), surf.point(u - hu, v))
        pv = _sub(surf.point(u, v + hv), surf.point(u, v - hv))
        n = _cross(pu, pv)
        nl = math.sqrt(_dot(n, n))
        if nl < 1e-14:                      # degenerate corner: nudge inward
            u, v = tri(u0, u1, 0.5), tri(v0, v1, 0.4)
            p = surf.point(u, v)
            pu = _sub(surf.point(u + hu, v), surf.point(u - hu, v))
            pv = _sub(surf.point(u, v + hv), surf.point(u, v - hv))
            n = _cross(pu, pv)
            nl = math.sqrt(_dot(n, n))
        if nl < 1e-14:
            continue
        out.append((p, _scale(n, 1.0 / nl)))
    return out


def dense_cloud(surf, uv, g):
    """Grid points over the UV bbox + max grid spacing + integrated bbox surface area."""
    u0, u1, v0, v1 = uv
    rows = [[surf.point(tri(u0, u1, i / (g - 1.0)), tri(v0, v1, j / (g - 1.0)))
             for j in range(g)] for i in range(g)]
    pts = [p for row in rows for p in row]
    step_max, area = 0.0, 0.0
    for i in range(g - 1):
        for j in range(g - 1):
            p00, p10, p01, p11 = rows[i][j], rows[i + 1][j], rows[i][j + 1], rows[i + 1][j + 1]
            step_max = max(step_max, math.dist(p00, p10), math.dist(p00, p01))
            d1, d2 = _sub(p11, p00), _sub(p10, p01)
            c = _cross(d1, d2)
            area += 0.5 * math.sqrt(_dot(c, c))
    return pts, step_max, area


def surf_dist(surf, uv, p, g, seed):
    """Distance from p to surface patch over uv-bbox: local 3x3 subdivision
    search seeded at grid cell `seed` (i,j of the dense cloud)."""
    u0, u1, v0, v1 = uv
    cu = tri(u0, u1, seed[0] / (g - 1.0))
    cv = tri(v0, v1, seed[1] / (g - 1.0))
    su, sv = (u1 - u0) / (g - 1.0), (v1 - v0) / (g - 1.0)
    best = math.dist(surf.point(cu, cv), p)
    for _ in range(7):
        bu, bv = cu, cv
        for iu in (-1, 0, 1):
            for iv in (-1, 0, 1):
                u = min(max(cu + iu * su, u0), u1)
                v = min(max(cv + iv * sv, v0), v1)
                d = math.dist(surf.point(u, v), p)
                if d < best:
                    best, bu, bv = d, u, v
        cu, cv, su, sv = bu, bv, su * 0.5, sv * 0.5
    return best


def state3(s):
    return {"IN": 1, "OUT": 0, "ON": None}[s]


def in_true(op, a, b):
    """Kleene 3-valued boolean membership: 1/0/None."""
    if op == "cut":
        nb = None if b is None else 1 - b
        if a == 0 or nb == 0:
            return 0
        if a == 1 and nb == 1:
            return 1
        return None
    if op == "common":
        if a == 0 or b == 0:
            return 0
        if a == 1 and b == 1:
            return 1
        return None
    if a == 1 or b == 1:
        return 1
    if a == 0 and b == 0:
        return 0
    return None


# ------------------------------------------------------------------------ main

def main():
    global PROBE
    ap = argparse.ArgumentParser(description="Face-level oracle differ (OCCT truth vs our boolean result)")
    ap.add_argument("--config", help="chair rot config, e.g. z15 (resolves standard paths)")
    ap.add_argument("--op", required=True, choices=["cut", "common", "fuse"])
    ap.add_argument("--a", help="operand A STEP")
    ap.add_argument("--b", help="operand B STEP")
    ap.add_argument("--result", help="our result STEP")
    ap.add_argument("--report", help="markdown report path (default validation/face_diff_<tag>_<op>.md)")
    ap.add_argument("--probe", default=PROBE, help="step_probe.exe path")
    ap.add_argument("--samples", type=int, default=9, help="UV samples per face")
    ap.add_argument("--grid", type=int, default=11, help="dense coverage grid per face")
    ap.add_argument("--eps", type=float, default=0.0, help="normal probe offset (0 = 5e-3*diag)")
    ap.add_argument("--pick", default="auto", help="result-solid pick for multi-solid files: auto|green|all|<idx>")
    args = ap.parse_args()
    PROBE = args.probe

    if args.config:
        A = os.path.join(CHAIRS, "chair0.stp")
        B = os.path.join(CHAIRS, "rot", "B_%s.step" % args.config)
        R = os.path.join(CHAIRS, "rot", "res_%s_%s.step" % (args.config, args.op))
        tag = args.config
    else:
        if not (args.a and args.b and args.result):
            ap.error("need --config or all of --a/--b/--result")
        A, B, R = args.a, args.b, args.result
        tag = os.path.splitext(os.path.basename(args.result))[0]
    for p in (A, B, R):
        if not os.path.exists(p):
            sys.exit("missing file: " + p)
    if not os.path.exists(PROBE):
        sys.exit("step_probe.exe not found: " + PROBE)
    report_path = args.report or os.path.join(HERE, "face_diff_%s_%s.md" % (tag, args.op))

    # 1) oracle truth + summaries + inclusion-exclusion sanity of the oracle
    # itself (vol(A)-cut == common == vol(A)+vol(B)-fuse; OCCT silently fails on
    # tangent-heavy configs with OP_VALID still 1 -- e.g. chairs z15)
    truth = probe_boolean(args.op, A, B)
    ours_sum = probe_summary(R)
    degenerate = truth["valid"] == 0 or abs(truth["volume"]) < 0.01
    sumA, sumB = probe_summary(A), probe_summary(B)
    v3 = {op: (truth if op == args.op else probe_boolean(op, A, B))["volume"]
          for op in ("cut", "common", "fuse")}
    c_cut = sumA["volume"] - v3["cut"]
    c_fuse = sumA["volume"] + sumB["volume"] - v3["fuse"]
    ietol = max(0.02 * abs(sumA["volume"]), 0.05)
    ok_cut = abs(c_cut - v3["common"]) <= ietol
    ok_fuse = abs(c_fuse - v3["common"]) <= ietol
    oracle_inconsistent = not (ok_cut and ok_fuse)
    # Which cell is the liar (majority vote of the two identities)? When it is the op
    # under test, the identity-derived volume replaces OCCT's as the VOLUME gate --
    # OCCT's own number is known wrong there (chairs z15 fuse).
    odd = ("" if not oracle_inconsistent else
           "fuse" if ok_cut else "cut" if ok_fuse else
           "common" if abs(c_cut - c_fuse) <= ietol else "unresolved")
    derived_vol = None
    if odd == args.op:
        derived_vol = {"cut": sumA["volume"] - v3["common"],
                       "fuse": sumA["volume"] + sumB["volume"] - v3["common"],
                       "common": 0.5 * (c_cut + c_fuse)}[odd]

    # 2) face tables (probe) + geometry (our STEP parser)
    pf_ours, pf_a, pf_b = probe_faces(R), probe_faces(A), probe_faces(B)
    st_r, st_a, st_b = Step(R), Step(A), Step(B)
    faces_r, units_r = pick_result_faces(st_r, args.pick)
    faces_a, _ = pick_result_faces(st_a, "all")
    faces_b, _ = pick_result_faces(st_b, "all")
    notes = []
    all_units_r = shape_units(st_r)
    subset = len(all_units_r) > len(units_r)
    if subset:
        notes.append("result file has %d shape units; diffing %d picked (--pick %s); "
                     "whole-file volume/solid/face-count tickets suppressed" %
                     (len(all_units_r), len(units_r), args.pick))
        faces_full = [fid for u in all_units_r for fid in u[2]]
        chosen_set = set(faces_r)
        faces_r = faces_full            # bind probe order against ALL faces
    else:
        chosen_set = None

    def bind(pf, step, fids, label):
        if len(pf) != len(fids):
            sys.exit("%s: probe reports %d faces but STEP parse found %d" %
                     (label, len(pf), len(fids)))
        out = []
        for rec, fid in zip(pf, fids):
            surf_id = _ref(step.args(fid)[2])
            surf = make_surface(step, surf_id)
            if isinstance(surf, BSurf):
                d = surf.domain()
                u0, u1, v0, v1 = rec["uv"]
                if u0 < d[0] - 1e-3 or u1 > d[1] + 1e-3 or v0 < d[2] - 1e-3 or v1 > d[3] + 1e-3:
                    sys.exit("%s FACE %d: probe uv %s outside surface domain %s "
                             "(face-order mismatch)" % (label, rec["i"], rec["uv"], d))
            out.append({"pf": rec, "surf": surf, "fid": fid})
        return out

    F_ours = bind(pf_ours, st_r, faces_r, "ours")
    if chosen_set is not None:
        F_ours = [f for f in F_ours if f["fid"] in chosen_set]
    F_a = bind(pf_a, st_a, faces_a, "A")
    F_b = bind(pf_b, st_b, faces_b, "B")

    # 3) sampling (probe samples + dense cloud/bbox-area per face)
    for F in (F_ours, F_a, F_b):
        for f in F:
            f["samples"] = face_samples(f["surf"], f["pf"]["uv"], args.samples)
            f["cloud"], f["stepmax"], f["bboxarea"] = dense_cloud(
                f["surf"], f["pf"]["uv"], args.grid)
    allp = [p for F in (F_ours, F_a, F_b) for f in F for (p, n) in f["samples"]]
    bb0 = [min(p[i] for p in allp) for i in range(3)]
    bb1 = [max(p[i] for p in allp) for i in range(3)]
    diag = math.dist(bb0, bb1)
    eps = args.eps or max(5e-3 * diag, 1e-4)

    for F in (F_ours, F_a, F_b):
        for f in F:
            f["sres"] = [None] * len(f["samples"])   # 'b'/'i'/'o'/'?' per sample
            f["str_a"] = [False] * len(f["samples"])
            f["str_b"] = [False] * len(f["samples"])

    def resolve_samples(slots):
        """Multi-scale classification: a sample is non-boundary only if every
        scale agrees (large eps punches through thin features)."""
        pending = slots
        for scale in (1.0, 0.25, 0.0625):
            if not pending:
                break
            probes = []
            for (f, si) in pending:
                p, n = f["samples"][si]
                probes.append(_add(p, _scale(n, eps * scale)))
                probes.append(_sub(p, _scale(n, eps * scale)))
            stA = [state3(s) for s in classify(A, probes)]
            stB = [state3(s) for s in classify(B, probes)]
            nxt = []
            for j, (f, si) in enumerate(pending):
                k = 2 * j
                tp = in_true(args.op, stA[k], stB[k])
                tm = in_true(args.op, stA[k + 1], stB[k + 1])
                # overwrite: finest evaluated scale wins (coarse eps straddles a
                # wide tangent band beyond the trim and over-counts on-source)
                f["str_a"][si] = (stA[k] is not None and stA[k + 1] is not None
                                  and stA[k] != stA[k + 1])
                f["str_b"][si] = (stB[k] is not None and stB[k + 1] is not None
                                  and stB[k] != stB[k + 1])
                if tp is None or tm is None:
                    f["sres"][si] = "?"
                    nxt.append((f, si))
                elif tp != tm:
                    f["sres"][si] = "b"          # boundary: settled, stop retrying
                else:
                    f["sres"][si] = "i" if tp == 1 else "o"
                    nxt.append((f, si))
            pending = nxt

    resolve_samples([(f, si) for F in (F_ours, F_a, F_b)
                     for f in F for si in range(len(f["samples"]))])

    def face_verdict(f, final):
        rs = f["sres"]
        nb, ni, no = rs.count("b"), rs.count("i"), rs.count("o")
        namb = rs.count("?")
        sa, sb = sum(f["str_a"]), sum(f["str_b"])
        f["ni_str"] = sum(1 for si, r in enumerate(rs)
                          if r == "i" and (f["str_a"][si] or f["str_b"][si]))
        f["no_str"] = sum(1 for si, r in enumerate(rs)
                          if r == "o" and (f["str_a"][si] or f["str_b"][si]))
        f.update(nb=nb, ni=ni, no=no, namb=namb,
                 src=("A" if sa > sb else "B" if sb > sa else "A+B" if sa else "?"))
        valid = nb + ni + no
        # Trim-aware consistency: samples cover the UV bbox, but the face is only
        # area/bboxarea of it. Measured true-boundary fraction (mf) must reach the
        # face's own bbox share (ef) or the face overshoots the true region.
        ef = min(1.0, f["pf"]["area"] / f["bboxarea"]) if f["bboxarea"] > 0 else 1.0
        mf = nb / valid if valid else 0.0
        f["ef"], f["mf"] = ef, mf
        if valid == 0:
            return "UNKNOWN"
        if nb == 0:
            if not final and ef * valid < 3:
                return "UNDECIDED"       # heavily-trimmed face: samples may all miss it
            return "EXTRA_INTERIOR" if ni >= no else "EXTRA_EXTERIOR"
        sig = math.sqrt(max(ef * (1 - ef), 0.04) / valid)
        delta = ef - mf
        if delta > 2.0 * sig + 0.05:
            return "OVERSHOOT"
        if not final and delta > 1.0 * sig + 0.05:
            return "UNDECIDED"
        return "OK"

    for f in F_ours:
        f["verdict"] = face_verdict(f, final=False)
    for F in (F_a, F_b):
        for f in F:
            face_verdict(f, final=True)   # fills nb/ni/no => true-boundary fraction

    # adaptive second pass: refine undecided faces with 25 fresh samples
    und = [f for f in F_ours if f["verdict"] == "UNDECIDED"]
    if und:
        slots = []
        for f in und:
            extra = face_samples(f["surf"], f["pf"]["uv"], 25)
            base = len(f["samples"])
            f["samples"] += extra
            f["sres"] += [None] * len(extra)
            f["str_a"] += [False] * len(extra)
            f["str_b"] += [False] * len(extra)
            slots += [(f, base + k) for k in range(len(extra))]
        resolve_samples(slots)
        for f in und:
            f["verdict"] = face_verdict(f, final=True)

    # 4) MISSING: true-boundary regions of operand faces uncovered by our cloud
    clouds = [(f["cloud"], (0.6 * f["stepmax"] + 2 * eps) ** 2, f) for f in F_ours]

    def covered(p):
        for pts, tol2, _f in clouds:
            for q in pts:
                dx = p[0] - q[0]
                dy = p[1] - q[1]
                dz = p[2] - q[2]
                if dx * dx + dy * dy + dz * dz <= tol2:
                    return True
        return False

    missing = []
    for label, F in (("A", F_a), ("B", F_b)):
        for f in F:
            tb = [f["samples"][si][0] for si, r in enumerate(f["sres"]) if r == "b"]
            if not tb:
                continue
            ncov = sum(1 for p in tb if covered(p))
            f["tb"] = len(tb)
            f["cov"] = ncov
            if ncov * 2 < len(tb):
                missing.append((label, f))

    # 5) AREA: our faces vs confidently whole-kept operand faces (paired by
    # proximity). Whole-kept = face fills its UV bbox (ef>=0.9) and every sample
    # is true boundary -- only then is the operand trim area the expected area.
    op_clouds = {}
    for label, F in (("A", F_a), ("B", F_b)):
        for f in F:
            valid = f["nb"] + f["ni"] + f["no"]
            ef = min(1.0, f["pf"]["area"] / f["bboxarea"]) if f["bboxarea"] > 0 else 1.0
            if valid >= 5 and f["nb"] == valid and ef >= 0.9:
                op_clouds[(label, f["pf"]["i"])] = (f["cloud"], f)
    area_tickets = []
    used = {}
    for f in F_ours:
        if f["verdict"] != "OK":
            continue
        best, bestd, bestseed = None, None, None
        smp = f["samples"][:args.samples]
        for key, (pts, of) in op_clouds.items():
            d, seed = 0.0, None
            for (p, _n) in smp:
                dmin, kmin = None, 0
                for k, q in enumerate(pts):
                    dd = (p[0] - q[0]) ** 2 + (p[1] - q[1]) ** 2 + (p[2] - q[2]) ** 2
                    if dmin is None or dd < dmin:
                        dmin, kmin = dd, k
                d += dmin
                seed = kmin
            if bestd is None or d < bestd:
                bestd, best, bestseed = d, key, seed
        if best is None:
            continue
        # confirm with true point-to-surface distance (discriminates opposite
        # thin walls that the coarse cloud distance confuses)
        _pts, of = op_clouds[best]
        g = args.grid
        dref = 0.0
        for (p, _n) in smp:
            dmin, kmin = None, 0
            for k, q in enumerate(_pts):
                dd = (p[0] - q[0]) ** 2 + (p[1] - q[1]) ** 2 + (p[2] - q[2]) ** 2
                if dmin is None or dd < dmin:
                    dmin, kmin = dd, k
            dref += surf_dist(of["surf"], of["pf"]["uv"], p, g, (kmin // g, kmin % g))
        if dref / len(smp) < 2 * eps:
            used.setdefault(best, 0.0)
            used[best] += f["pf"]["area"]
    for key, our_area in used.items():
        _pts, of = op_clouds[key]
        ta = of["pf"]["area"]
        rel = abs(our_area - ta) / ta if ta else 0.0
        if rel > 0.01:
            area_tickets.append((key, of, our_area, rel))

    # 6) report
    def centroid(f):
        ps = [p for (p, n) in f["samples"]]
        return tuple(sum(c[i] for c in ps) / len(ps) for i in range(3))

    L = []
    L.append("# face_diff %s %s" % (tag, args.op))
    L.append("")
    L.append("A: `%s`  " % A)
    L.append("B: `%s`  " % B)
    L.append("ours: `%s`  " % R)
    L.append("eps %.4g, %d samples/face, grid %d" % (eps, args.samples, args.grid))
    L.append("")
    L.append("| side | solids | shells | faces | volume | OCCT valid |")
    L.append("|---|---|---|---|---|---|")
    L.append("| TRUE (OCCT %s) | %d | - | %d | %.6f | %d |" %
             (args.op, truth["solids"], truth["faces"], truth["volume"], truth["valid"]))
    L.append("| OURS | %d | %d | %d | %.6f | %d |" %
             (ours_sum["solids"], ours_sum["shells"], ours_sum["faces"],
              ours_sum["volume"], ours_sum["valid"]))
    if degenerate:
        L.append("")
        L.append("**WARNING: oracle result degenerate (OP_VALID=0 or |vol|<0.01) -- verdicts untrusted.**")
    if oracle_inconsistent:
        L.append("")
        L.append("**WARNING: OCCT self-inconsistent on this config** -- vol identities "
                 "imply common=%.4f (from cut), %.4f (from fuse), but OCCT common=%.4f. "
                 "The oracle truth itself is unreliable here; treat VOLUME/COUNT/AGG "
                 "tickets as indicative only. Odd cell (majority vote): %s."
                 % (c_cut, c_fuse, v3["common"], odd or "-"))
        if derived_vol is not None:
            L.append("")
            L.append("**The op under test IS the odd cell** -- the VOLUME ticket below "
                     "gates on the identity-derived volume %.6f, not on OCCT's %.6f."
                     % (derived_vol, truth["volume"]))
    for n in notes:
        L.append("")
        L.append("NOTE: " + n)

    tickets = []
    if subset:
        if len(F_ours) != truth["faces"]:
            tickets.append(("COUNT", "-", "-", "-", "-",
                            "picked-solid faces %d vs OCCT %d (%+d)" %
                            (len(F_ours), truth["faces"], len(F_ours) - truth["faces"])))
    elif ours_sum["faces"] != truth["faces"]:
        tickets.append(("COUNT", "-", "-", "-", "-",
                        "our faces %d vs OCCT %d (%+d)" %
                        (ours_sum["faces"], truth["faces"], ours_sum["faces"] - truth["faces"])))
    if not subset and ours_sum["solids"] != truth["solids"]:
        tickets.append(("SOLIDITY", "-", "-", "-", "-",
                        "our solids %d vs OCCT %d (shells %d, valid %d)" %
                        (ours_sum["solids"], truth["solids"], ours_sum["shells"], ours_sum["valid"])))
    gate_vol = derived_vol if derived_vol is not None else truth["volume"]
    if not subset and gate_vol:
        vrel = abs(ours_sum["volume"] - gate_vol) / abs(gate_vol)
        if vrel > 0.01:
            tickets.append(("VOLUME", "-", "-", "-", "-",
                            "our vol %.4f vs %s %.4f (rel %.2e)" %
                            (ours_sum["volume"],
                             "identity-derived" if derived_vol is not None else "OCCT",
                             gate_vol, vrel)))
    for f in F_ours:
        if f["verdict"] in ("EXTRA_INTERIOR", "EXTRA_EXTERIOR", "OVERSHOOT", "UNKNOWN"):
            c = centroid(f)
            extra = ""
            if "ef" in f:
                extra = " bnd-frac %.2f vs face/bbox %.2f" % (f["mf"], f["ef"])
            tickets.append((f["verdict"], "ours FACE %d" % f["pf"]["i"],
                            f["pf"]["srf"].replace("Geom_", ""), "%.4f" % f["pf"]["area"],
                            "(%.3f,%.3f,%.3f)" % c,
                            "samples bnd/int/ext/amb %d/%d/%d/%d src %s%s" %
                            (f["nb"], f["ni"], f["no"], f["namb"], f["src"], extra)))
    for label, f in missing:
        c = centroid(f)
        frac = f["nb"] / max(1, f["nb"] + f["ni"] + f["no"])
        tickets.append(("MISSING", "%s FACE %d" % (label, f["pf"]["i"]),
                        f["pf"]["srf"].replace("Geom_", ""), "%.4f" % f["pf"]["area"],
                        "(%.3f,%.3f,%.3f)" % c,
                        "true-boundary samples %d, covered by ours %d (exp.area ~%.3f)" %
                        (f["tb"], f["cov"], frac * f["pf"]["area"])))
    for key, of, our_area, rel in area_tickets:
        c = centroid(of)
        tickets.append(("AREA", "%s FACE %d" % (key[0], key[1]),
                        of["pf"]["srf"].replace("Geom_", ""), "%.4f" % of["pf"]["area"],
                        "(%.3f,%.3f,%.3f)" % c,
                        "ours %.4f vs operand %.4f (rel %.1f%%)" % (our_area, of["pf"]["area"], rel * 100)))

    # aggregate boundary-area conservation: expected true area = sum over operand
    # faces of (kept fraction among ON-SOURCE samples) x exact trim area; our
    # total is exact (OCCT face areas). Inflation => wrongly kept regions (e.g.
    # fuse interior walls); rank suspects by on-source non-boundary sample mass.
    exp_area = 0.0
    for F in (F_a, F_b):
        for of in F:
            n_on = of["nb"] + of["ni_str"] + of["no_str"]
            if n_on:
                exp_area += of["pf"]["area"] * of["nb"] / n_on
    our_total = sum(f["pf"]["area"] for f in F_ours)
    if exp_area > 1e-9:
        agg_rel = (our_total - exp_area) / exp_area
        if abs(agg_rel) > 0.12:           # estimator carries ~5-8% tangent-band bias
            tickets.append(("AGG_AREA", "-", "-", "-", "-",
                            "our total face area %.3f vs expected true boundary %.3f "
                            "(rel %+.1f%%, estimator bias ~8%%)" %
                            (our_total, exp_area, agg_rel * 100)))
            if agg_rel > 0:
                sus = [((f.get("ef", 1.0) - f.get("mf", 0.0)) * f["bboxarea"], f)
                       for f in F_ours if f.get("ef", 1.0) - f.get("mf", 0.0) > 0.1]
                sus = [s for s in sus if s[0] > 0.02 * our_total]
                sus.sort(key=lambda s: -s[0])
                for est, f in sus[:10]:
                    c = centroid(f)
                    tickets.append(("SUSPECT_KEPT", "ours FACE %d" % f["pf"]["i"],
                                    f["pf"]["srf"].replace("Geom_", ""),
                                    "%.4f" % f["pf"]["area"], "(%.3f,%.3f,%.3f)" % c,
                                    "face/bbox %.2f vs bnd-frac %.2f, on-src non-bnd "
                                    "int/ext %d/%d (~%.2f area kept beyond truth?)" %
                                    (f["ef"], f["mf"], f["ni_str"], f["no_str"], est)))

    L.append("")
    L.append("## Defect tickets (%d)" % len(tickets))
    L.append("")
    if tickets:
        L.append("| # | kind | face | srf | area | centroid | detail |")
        L.append("|---|---|---|---|---|---|---|")
        for i, t in enumerate(tickets):
            L.append("| %d | %s | %s | %s | %s | %s | %s |" % ((i,) + t))
    else:
        L.append("No defects: every our-face verifies as true-result boundary, all true "
                 "regions covered, counts/volume match.")

    nok = sum(1 for f in F_ours if f["verdict"] == "OK")
    L.append("")
    L.append("## Our-face verdicts (%d OK / %d faces)" % (nok, len(F_ours)))
    L.append("")
    L.append("| face | verdict | srf | area | bnd/int/ext/amb | bnd-frac | face/bbox | src |")
    L.append("|---|---|---|---|---|---|---|---|")
    for f in F_ours:
        L.append("| %d | %s | %s | %.4f | %d/%d/%d/%d | %.2f | %.2f | %s |" %
                 (f["pf"]["i"], f["verdict"], f["pf"]["srf"].replace("Geom_", ""),
                  f["pf"]["area"], f["nb"], f["ni"], f["no"], f["namb"],
                  f.get("mf", 0.0), f.get("ef", 1.0), f["src"]))

    text = "\n".join(L) + "\n"
    print(text)
    open(report_path, "w").write(text)
    print("report: %s" % report_path, file=sys.stderr)


if __name__ == "__main__":
    main()
