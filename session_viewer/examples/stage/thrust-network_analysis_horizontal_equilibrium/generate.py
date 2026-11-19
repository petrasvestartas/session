# Writes the four chapters of this deck: python3 generate.py (numpy only).
# The solver below is parallelise_nodal of compas_tna, called by horizontal_nodal_numpy with alpha = 100.
# Edit sentences here, not in the yaml files: they are overwritten.
import os

import numpy as np

from horizontal_direct import bounded_densities
from horizontal_direct import heights
from horizontal_direct import self_supporting_plan

HEAD = """# Written by generate.py in this folder; edit the sentences there.
# Open local: http://127.0.0.1:8770/?stage=decks/thrust-network_analysis_horizontal_equilibrium/index.yaml&data=off&chapter={n}&step=1
# Open live: https://petrasvestartas.github.io/session/?stage=thrust-network_analysis_horizontal_equilibrium&chapter={n}&step=1

"""

# form diagram: free nodes 1..4, supports s1..s8 counter-clockwise from the bottom left
FORM = {
    "n1": [1.3, 1.6],
    "n2": [4.9, 2.2],
    "n3": [3.7, 4.8],
    "n4": [2.0, 3.5],
    "s1": [1.2, 0.0],
    "s2": [4.9, 0.0],
    "s3": [6.0, 2.9],
    "s4": [6.0, 4.9],
    "s5": [3.3, 6.0],
    "s6": [2.3, 6.0],
    "s7": [0.0, 4.0],
    "s8": [0.0, 1.4],
}
FREE = ["n1", "n2", "n3", "n4"]
RIM = ["s1", "s2", "s3", "s4", "s5", "s6", "s7", "s8"]
# bar (u, v) and the spaces on its left and right: its side of the force diagram runs left -> right
BARS = [
    ("s1", "n1", "A", "B"),
    ("s2", "n2", "B", "C"),
    ("s8", "n1", "D", "A"),
    ("n1", "n2", "E", "B"),
    ("n1", "n4", "D", "E"),
    ("n2", "s3", "F", "C"),
    ("n2", "n3", "E", "F"),
    ("s7", "n4", "G", "D"),
    ("n4", "n3", "H", "E"),
    ("n4", "s6", "G", "H"),
    ("n3", "s4", "I", "F"),
    ("n3", "s5", "H", "I"),
]
# the corners of every space, for its centroid
SPACES = {
    "A": ["s1", "n1", "s8"],
    "B": ["s1", "s2", "n2", "n1"],
    "C": ["s3", "n2", "s2"],
    "D": ["s8", "n1", "n4", "s7"],
    "E": ["n1", "n2", "n3", "n4"],
    "F": ["n2", "s3", "s4", "n3"],
    "G": ["s7", "n4", "s6"],
    "H": ["n4", "n3", "s5", "s6"],
    "I": ["s5", "n3", "s4"],
}
LETTERS = list(SPACES)
UP = np.array([0.0, 6.5])  # the form diagram is drawn above the force diagram
PUSH = 0.4  # scale of the pushes drawn on the form diagram
TINT = {"n1": "pink", "n2": "blue", "n3": "pink", "n4": "blue"}  # neighbours differ
SHOWN = [2, 3, 5, 10, 20, 50]


def rot90(xy, zdir=1.0):
    out = np.empty_like(xy)
    out[:, 0] = -zdir * xy[:, 1]
    out[:, 1] = +zdir * xy[:, 0]
    return out


def solve(kmax):
    "Every iteration of the force diagram in the turned frame, with the proposals of every corner."
    xy = np.array([FORM[k] for k in FORM])
    index = {k: i for i, k in enumerate(FORM)}
    uv = np.array([xy[index[v]] - xy[index[u]] for u, v, _, _ in BARS])
    length = np.linalg.norm(uv, axis=1)
    targets = uv / length[:, None]

    _xy = np.array([xy[[index[k] for k in SPACES[s]]].mean(axis=0) for s in LETTERS])
    start = _xy.copy()
    _xy = rot90(_xy, +1.0)
    _edges = [(LETTERS.index(a), LETTERS.index(b)) for _, _, a, b in BARS]
    nbrs = {j: [] for j in range(len(_xy))}
    ij_e = {}
    for e, (i, j) in enumerate(_edges):
        nbrs[i].append(j)
        nbrs[j].append(i)
        ij_e[(i, j)] = e

    history, proposals = [_xy.copy()], []
    for k in range(kmax):
        xy0 = _xy.copy()
        l = np.array([np.linalg.norm(xy0[j] - xy0[i]) for i, j in _edges])
        step = {}
        for j in range(len(_xy)):
            step[j] = []
            for i in nbrs[j]:
                if (i, j) in ij_e:
                    e = ij_e[(i, j)]
                    p = xy0[i] + l[e] * targets[e]
                else:
                    e = ij_e[(j, i)]
                    p = xy0[i] - l[e] * targets[e]
                step[j].append((i, e, p))
            _xy[j] = sum(p for _, _, p in step[j]) / len(nbrs[j])
        history.append(_xy.copy())
        proposals.append(step)
    return {
        "xy": xy,
        "index": index,
        "length": length,
        "targets": targets,
        "start": start,
        "edges": _edges,
        "history": history,
        "proposals": proposals,
    }


S = solve(100)
CENTRE = S["start"].mean(axis=0)


def miss(k):
    "Angle between every side and its target after k iterations, in degrees."
    d = np.array([S["history"][k][j] - S["history"][k][i] for i, j in S["edges"]])
    d /= np.linalg.norm(d, axis=1)[:, None]
    return np.degrees(np.arccos(np.clip((d * S["targets"]).sum(axis=1), -1.0, 1.0)))


def turned(k):
    "Iteration k drawn turned: the stored diagram rotated by 90 degrees about its own centre."
    stored = rot90(S["history"][k], -1.0)
    return rot90(stored - CENTRE, +1.0) + CENTRE


def stored(k):
    return rot90(S["history"][k], -1.0)


def place(p):
    "A turned-frame position of the solver, drawn like turned()."
    return rot90(rot90(np.array([p]), -1.0) - CENTRE, +1.0)[0] + CENTRE


def P(p):
    return "[%.3f, %.3f, 0]" % (p[0], p[1])


def points(extra):
    out = ["points:"]
    for k, p in FORM.items():
        out.append("  %s: %s" % (k, P(np.array(p) + UP)))
    for k, p in extra.items():
        out.append("  %s: %s" % (k, P(p)))
    out.append("  TG: [-0.7, %.1f, 0]" % (UP[1] + 5.8))
    out.append("  TF: [-0.7, 5.6, 0]")
    out.append("  LO: [0.0, 0.9, 0]")  # a little inside the drawing: the fit keeps wide margins
    out.append("  HI: [6.0, %.1f, 0]" % (UP[1] + 5.3))
    return out


def corners(xy):
    return {s: xy[i] for i, s in enumerate(LETTERS)}


def form_objects(labels=True):
    out = [
        "{id: rim, polygon: [%s], color: grey, width: 1.2, dash: true}" % ", ".join(RIM)
    ]
    for e, (u, v, _, _) in enumerate(BARS):
        out.append("{id: b%d, line: [%s, %s], color: ink, width: 2.4}" % (e, u, v))
    for s in RIM:
        out.append("{id: %s, point: %s, color: fixed, size: 0.8}" % (s, s))
    for n in FREE:
        label = ", label: '%s'" % n[1] if labels else ""
        out.append("{id: %s, point: %s, color: free%s}" % (n, n, label))
    out.append("{id: tg, label: TG, tex: '\\Gamma', color: accent}")
    return out


def sides(width=3.0):
    return [
        "{id: f%d, line: [%s, %s], color: ink, width: %.1f}" % (e, a, b, width)
        for e, (_, _, a, b) in enumerate(BARS)
    ]


def corner_points(labels=True):
    if labels:
        return [
            "{id: %s, point: %s, color: ink, size: 0.8, label: '%s'}" % (s, s, s)
            for s in LETTERS
        ]
    return ["{id: %s, point: %s, color: ink, size: 0.8}" % (s, s) for s in LETTERS]


def needles():
    out = []
    for e, (_, _, a, b) in enumerate(BARS):
        t = 0.42 * S["targets"][e]
        d = "[%.3f, %.3f, 0]" % (t[0], t[1])
        out.append(
            "{id: t%d, arrow: ['mid(%s, %s) - %s', 'mid(%s, %s) + %s'], color: blue, width: 1.5}"
            % (e, a, b, d, a, b, d)
        )
    return out


def moves(xy, dur=1.4):
    out = []
    for i, s in enumerate(LETTERS):
        with_ = ", with: true" if i else ""
        out.append(
            "- move: {point: %s, to: %s, dur: %.1f%s}" % (s, P(xy[i]), dur, with_)
        )
    return out


def add(objects, indent=10):
    pad = " " * indent
    return [pad + "- add:"] + [pad + "    - " + o for o in objects]


def lines(items, indent=10):
    return [" " * indent + i for i in items]


def code(text, indent=10):
    pad = " " * indent
    return [pad + "- code: |"] + [
        pad + "    " + line for line in text.strip("\n").split("\n")
    ]


def pushes(node):
    "The pushes on a free node: (bar, tail on the form, side start, side end), in the order of its polygon."
    xy, index = S["xy"], S["index"]
    force = np.linalg.norm(
        np.array([stored(100)[j] - stored(100)[i] for i, j in S["edges"]]), axis=1
    )
    found = []
    for e, (u, v, a, b) in enumerate(BARS):
        if node not in (u, v):
            continue
        other = u if v == node else v
        away = xy[index[other]] - xy[index[node]]
        tail = xy[index[node]] + PUSH * force[e] * away / np.linalg.norm(away) + UP
        found.append((e, tail, a, b) if v == node else (e, tail, b, a))
    ordered = [found.pop(0)]
    while found:
        nxt = next(f for f in found if f[2] == ordered[-1][3])
        found.remove(nxt)
        ordered.append(nxt)
    return ordered


def push_objects(node):
    return [
        "{id: p%s_%d, arrow: [%s, %s], color: %s, width: 4.2}"
        % (node[1], e, P(tail), node, TINT[node])
        for e, tail, _, _ in pushes(node)
    ]


def push_slides(node, dur=None):
    return [
        "- derive: {from: p%s_%d, id: g%s_%d, arrow: [%s, %s], color: %s, width: 3.2}"
        % (node[1], e, node[1], e, a, b, TINT[node])
        for e, _, a, b in pushes(node)
    ]


def polygon_labels():
    out = []
    for node in FREE:
        a, b, c, d = [p[2] for p in pushes(node)]
        out.append(
            "{id: c%s, label: 'lerp(mid(%s, %s), mid(%s, %s), 0.5)', tex: '%s', color: %s}"
            % (node[1], a, c, b, d, node[1], TINT[node])
        )
    return out


PATTERNS = [
    # name, title, sentence and prose before the solve, sentence and prose after it, which result is drawn
    ("tri", "TRIANGLES", "Triangles: six bars meet at every free node.",
     [],
     "Parallel after {iterations} iterations. With so much freedom a diagram is easy to find.",
     [], "loop"),
    ("quad", "QUADS", "Quads, a little irregular: four bars per node.",
     [],
     "Parallel after {iterations} iterations, every side in compression.", [], "loop"),
    ("quadbad", "QUADS, DISTORTED", "The same quads, strongly distorted.",
     [],
     "But each of them has flipped sides, drawn pink: those bars would be in tension.",
     ["No compression-only diagram exists for this plan. The loop only shrinks to a point."], "direct"),
    ("hex", "HEXAGONS", "Hexagons: three bars per node, so every polygon of $\\Gamma^*$ is a triangle.",
     ["More equations than unknowns, yet one state exists: this plan is regular."],
     "One diagram, fixed up to its scale. No force can be moved elsewhere.", [], "loop"),
    ("hexbad", "HEXAGONS, DISTURBED", "The same hexagons with slightly moved nodes.",
     ["$k = 0$: no parallel diagram exists at all."],
     "The loop stalls at ${miss:.1f}^\\circ$. Pink sides miss by more than $1^\\circ$.",
     ["Here the plan has to move: alpha below 100, or a new drawing."], "loop"),
    ("dome", "DOME PLAN", "A dome plan of hoops and meridians: eight bars meet at the centre.",
     ["By symmetry the first guess is already parallel."], None, [], "loop"),
    ("open", "OPEN EDGE, STRAIGHT", "An open edge: the bottom row has no supports, and its bars lie on one line.",
     [],
     "No compression-only diagram exists. A straight edge cannot turn the thrust that arrives at it.",
     ["Pink sides are flipped: tension."], "direct"),
    ("arch", "OPEN EDGE, ARCHED", "Bend the open edge inwards: it becomes an edge arch.",
     [],
     "Parallel after {iterations} iterations. The fan in $\\Gamma^*$ is the force diagram of that arch.", [], "loop"),
]


COUNT = "- eq: {tex: '2 \\cdot %d = %d\\ \\text{equations} \\qquad %d\\ \\text{unknowns} \\qquad k = %d\\ \\text{free states}', align: left, size: 24}"


def boxed(xy, lo, hi):
    "Points scaled and centred into a box, keeping their proportions."
    xy = np.array(xy, dtype=float)
    a, b = xy.min(axis=0), xy.max(axis=0)
    k = min((hi[0] - lo[0]) / max(b[0] - a[0], 1e-9), (hi[1] - lo[1]) / max(b[1] - a[1], 1e-9))
    return (xy - (a + b) / 2) * k + (np.array(lo) + np.array(hi)) / 2


def chapter5():
    import json

    with open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "patterns.json")) as f:
        data = json.load(f)
    top = ([0.2, UP[1] + 0.1], [5.8, UP[1] + 5.4])
    low = ([0.4, 0.5], [5.6, 5.5])
    extra, scenes = {}, {}
    for name, *_ in PATTERNS:
        d = data[name]
        result = d["direct"] if dict((p[0], p[6]) for p in PATTERNS)[name] == "direct" else d["loop"]
        if np.ptp(np.array(result), axis=0).max() < 1e-6:
            result = d["start"]
        both = boxed(d["start"] + result, *low)
        n = len(d["start"])
        scenes[name] = (boxed(d["xy"], *top), both[:n], both[n:])
        for i in range(n):
            extra["%s%d" % (name, i)] = both[i]
    y = ["title: OTHER PATTERNS", "split: 0.5", "camera: {view: top, fit: [LO, HI]}"]
    y += points(extra)
    y += ["slides:"]
    before = []
    for name, title, say1, text1, say2, text2, which in PATTERNS:
        d = data[name]
        xy, start, result = scenes[name]
        info = dict(d, eq=2 * d["free"], bars=len(d["edges"]))
        bars = ["{id: %sb%d, line: [%s, %s], color: ink, width: 2.0}" % (name, e, P(xy[u]), P(xy[v])) for e, (u, v) in enumerate(d["edges"])]
        sups = ["{id: %ss%d, point: %s, color: fixed, size: 0.6}" % (name, i, P(xy[i])) for i in d["fixed"]]
        sides_ = ["{id: %sf%d, line: [%s%d, %s%d], color: ink, width: 2.0}" % (name, e, name, u, name, v) for e, (u, v) in enumerate(d["_edges"])]
        y += ["  - title: %s" % title, "    steps:"]
        y += ["      - say: '%s'" % say1.format(**info), "        do:"]
        if before:
            y += lines(["- remove: [%s]" % ", ".join(before)])
        else:
            y += add(["{id: tg, label: TG, tex: '\\Gamma', color: accent}", "{id: tf, label: TF, tex: '\\Gamma^*', color: accent}"])
        y += add(bars + sups + sides_)
        y += lines([COUNT % (d["free"], 2 * d["free"], len(d["edges"]), d["states"])])
        y += lines(["- text: '%s'" % t.format(**info) for t in text1])
        before = [o.split(",")[0][5:] for o in bars + sups + sides_]
        if say2 is None:
            continue
        y += ["      - say: '%s'" % say2.format(**info), "        do:"]
        for i in range(len(start)):
            y += lines(["- move: {point: %s%d, to: %s, dur: 1.6%s}" % (name, i, P(result[i]), ", with: true" if i else "")])
        wrong = [e for e, a in enumerate(d["direct_angles"] if which == "direct" else d["loop_angles"]) if a > 1.0]
        y += lines(["- color: {target: %sf%d, color: pink, with: true}" % (name, e) for e in wrong])
        y += lines(["- text: '%s'" % t.format(**info) for t in text2])
    y += ["  - title: WHAT THE PATTERNS SAY", "    steps:"]
    y += ["      - say: 'The turn by $90^\\circ$ always works. Whether a parallel diagram exists is decided by the plan.'", "        do:"]
    y += lines([
        "- eq: {tex: '\\mathbf{C}_i^T\\mathbf{U}\\,\\mathbf{q} = \\mathbf{0} \\quad \\mathbf{C}_i^T\\mathbf{V}\\,\\mathbf{q} = \\mathbf{0} \\qquad k = m - \\text{rank}', align: left, size: 26}",
        "- text: '**3 bars per node:** $k \\le 1$. One diagram, and only for special plans.'",
        "- text: '**4 bars per node:** a few states. Strong distortion leaves none in compression.'",
        "- text: '**Triangles:** many states.  **Open edges:** must be arches in plan.'",
    ])
    return y



def chapter_geometry():
    "From the pushes on a node to the parallel force diagram, the way compas_tna finds it."
    from scipy.optimize import linprog

    final, first, after = turned(100), turned(0), turned(1)
    at = {s: P(final[i]) for i, s in enumerate(LETTERS)}
    j = LETTERS.index("E")
    votes = S["proposals"][0][j]
    extra = corners(S["start"] + UP)
    arcs, side = {}, {}
    for i, e, p in votes:
        n = LETTERS[i]
        v = place(p)
        side[n] = e
        extra["v" + n] = v
        extra["r" + n] = first[i] + 1.3 * (v - first[i])
        a0 = np.arctan2(*(first[j] - first[i])[::-1])
        a1 = np.arctan2(*(v - first[i])[::-1])
        da = (a1 - a0 + np.pi) % (2 * np.pi) - np.pi
        radius = np.linalg.norm(v - first[i])
        arcs[n] = [first[i] + radius * np.array([np.cos(a0 + da * k / 10), np.sin(a0 + da * k / 10)]) for k in range(11)]
    for i, s in enumerate(LETTERS):
        extra["m" + s] = after[i]

    # another parallel diagram of the same plan: the linear programme of Block 2009, drawn at the same size
    xy, index, m = S["xy"], S["index"], len(BARS)
    C, Cs = np.zeros((m, len(xy))), np.zeros((m, len(LETTERS)))
    for e, (u, v, a, b) in enumerate(BARS):
        C[e, index[u]], C[e, index[v]] = -1.0, 1.0
        Cs[e, LETTERS.index(a)], Cs[e, LETTERS.index(b)] = -1.0, 1.0
    uv = C @ xy
    free = [index[k] for k in FREE]
    A = np.vstack([C[:, free].T * uv[:, 0], C[:, free].T * uv[:, 1]])
    ratio = linprog(np.ones(m), A_eq=A, b_eq=np.zeros(len(A)), bounds=[(1.0 / l, None) for l in S["length"]], method="highs").x
    other = np.linalg.lstsq(Cs, ratio[:, None] * uv, rcond=None)[0]
    thrust = np.linalg.norm(Cs @ S["history"][100], axis=1)
    other = (other - other.mean(axis=0)) * thrust.mean() / np.linalg.norm(Cs @ other, axis=1).mean()
    thrust2 = np.linalg.norm(Cs @ other, axis=1)
    other = other + final.mean(axis=0)

    def slide_in(node, together):
        rows = ["- derive: {from: p%s_%d, id: g%s_%d, arrow: [%s, %s], color: %s, width: 3.2%s}"
                % (node[1], e, node[1], e, at[a], at[b], TINT[node], ", with: true" if together and k else "")
                for k, (e, _, a, b) in enumerate(pushes(node))]
        return rows

    def middle(node):
        return np.mean([final[LETTERS.index(p[2])] for p in pushes(node)], axis=0)

    y = ["title: THE FORCE DIAGRAM", "split: 0.5", "camera: {view: top, fit: [LO, HI]}"]
    y += points(extra)
    y += ["slides:"]

    y += ["  - title: NODES CLOSE POLYGONS", "    steps:"]
    y += ["      - say: 'Four bars push on node $1$.'", "        do:"]
    y += add(form_objects() + push_objects("n1"))
    y += ["      - say: 'Head to tail the pushes close: equilibrium.'", "        do:"]
    y += lines(slide_in("n1", False))
    y += add(["{id: mag, label: %s, tex: '\\times 2.5', color: grey}" % P(middle("n1"))])
    y += lines(["- eq: {tex: '\\sum \\mathbf{F}_{\\mathrm{H}} = \\mathbf{0}', align: left, size: 36}"])
    y += ["      - say: 'Every node closes its own polygon. Neighbours share a side.'", "        do:"]
    y += lines(["- remove: [mag]"])
    for node in ("n2", "n3", "n4"):
        y += add(push_objects(node))
        y += lines(slide_in(node, True))
    y += add(["{id: tf, label: TF, tex: '\\Gamma^*', color: accent}"])
    arrows = ["p%s_%d" % (n[1], p[0]) for n in FREE for p in pushes(n)] + ["g%s_%d" % (n[1], p[0]) for n in FREE for p in pushes(n)]
    truth = (["{id: zf%d, line: [%s, %s], color: ink, width: 3.0}" % (e, at[a], at[b]) for e, (_, _, a, b) in enumerate(BARS)]
             + ["{id: z%s, point: %s, color: ink, size: 0.8, label: '%s'}" % (s, at[s], s) for s in LETTERS]
             + ["{id: c%s, label: %s, tex: '%s', color: %s}" % (n[1], P(middle(n)), n[1], TINT[n]) for n in FREE]
             + ["{id: sp%s, label: %s, tex: '%s', color: grey}" % (s, P(S["start"][i] + UP), s) for i, s in enumerate(LETTERS)])
    y += ["      - say: 'The force diagram $\\Gamma^*$: side $BE$ is parallel to bar $1$–$2$, and its length is the thrust.'", "        do:"]
    y += lines(["- remove: [%s]" % ", ".join(arrows)])
    y += add(truth)
    y += lines(["- color: {target: b3, color: pink}", "- color: {target: zf3, color: pink, with: true}",
                "- eq: {tex: 'F_{\\mathrm{H}} = \\frac{1}{r}\\,l^*', align: left, size: 36}"])

    gone = [o.split(",")[0][5:] for o in truth]
    y += ["  - title: 'compas_tna: A FIRST GUESS'", "    steps:"]
    y += ["      - say: 'Unknown $\\Gamma^*$: guess one corner at the centroid of every space, joined across each bar.'", "        do:"]
    y += lines(["- remove: [%s]" % ", ".join(gone), "- color: {target: b3, color: ink}"])
    y += add(corner_points() + sides(2.6))
    y += code("force = ForceDiagram.from_formdiagram(form)")
    y += ["      - say: 'Set aside and turned by $90^\\circ$. Blue needles: the bars, where every side should point.'", "        do:"]
    y += lines(moves(S["start"], 1.2) + moves(first, 1.4))
    for e, (_, _, a, b) in enumerate(BARS):
        d = "[%.3f, %.3f, 0]" % tuple(0.42 * S["targets"][e])
        y += lines(["- derive: {from: b%d, id: t%d, arrow: ['mid(%s, %s) - %s', 'mid(%s, %s) + %s'], color: blue, width: 1.5%s}"
                    % (e, e, a, b, d, a, b, d, ", with: true" if e else "")])
    y += code("_xy[:] = rot90(_xy, +1.0)    # (x, y) -> (-y, x)")
    y += lines(["- eq: {tex: '\\text{largest miss}\\quad %.1f^\\circ', align: left, size: 30}" % miss(0).max()])

    far = [e for e, (_, _, a, b) in enumerate(BARS) if "E" not in (a, b)]
    dimmed = ["f%d" % e for e in far] + ["t%d" % e for e in far]

    def ask(n):
        return (["- derive: {from: t%d, id: ray%s, line: [%s, r%s], color: blue, width: 1.6, dash: true}" % (side[n], n, n, n)]
                + add(["{id: arc%s, polyline: [%s], color: pink, width: 1.4}" % (n, ", ".join(P(q) for q in arcs[n])),
                       "{id: a%s, arrow: [%s, v%s], color: pink, width: 2.6}" % (n, n, n),
                       "{id: v%s, point: v%s, color: pink, size: 0.7, label: '\\mathbf{p}_%s'}" % (n, n, n)], 0))

    y += ["  - title: ONE ITERATION", "    steps:"]
    y += ["      - say: 'Side $DE$: slide its needle to $D$ and turn the side onto it, keeping its length $l$.'", "        do:"]
    y += lines(["- camera: {fit: [H, B, D, F, vH, vB, vD, vF], dur: 1.2}", "- dim: [%s]" % ", ".join(dimmed)])
    y += lines(ask("D"))
    y += lines(["- eq: {tex: '\\mathbf{p}_D = \\mathbf{x}_D + l\\,\\mathbf{t}', color: pink, align: left, size: 34}"])
    y += ["      - say: 'The other three sides of $E$ do the same.'", "        do:"]
    for n in [LETTERS[i] for i, _, _ in votes if LETTERS[i] != "D"]:
        y += lines(ask(n))
    y += lines(["- eq: {tex: '\\mathbf{p}_i = \\mathbf{x}_i \\pm l_i\\,\\mathbf{t}_i', color: pink, align: left, size: 34}"])
    y += ["      - say: 'Corner $E$ goes to the average of the four.'", "        do:"]
    y += add(["{id: w%s, line: [v%s, mE], color: pink, width: 1.0, dash: true}" % (n, n) for n in side]
             + ["{id: step, arrow: [E, mE], color: ink, width: 3.4}"])
    y += lines(["- eq: {tex: '\\mathbf{x}_E \\leftarrow \\tfrac{1}{4}\\,(\\mathbf{p}_B + \\mathbf{p}_D + \\mathbf{p}_F + \\mathbf{p}_H)', align: left, size: 30}"])
    built = ["step"] + [x + n for n in side for x in ("ray", "arc", "a", "v", "w")]
    y += ["      - say: 'Every corner does this at the same time: one iteration.'", "        do:"]
    y += lines(["- camera: {fit: [A, C, G, I, B, D, F, H], dur: 1.2}", "- remove: [%s]" % ", ".join(built), "- undim: [%s]" % ", ".join(dimmed)])
    y += add(["{id: s%s, arrow: [%s, m%s], color: ink, width: 2.2}" % (s, s, s) for s in LETTERS])
    y += lines(["- wait: 0.8"] + moves(after) + ["- remove: [%s]" % ", ".join("s" + s for s in LETTERS),
                "- eq: {tex: '\\text{largest miss}\\quad %.1f^\\circ \\to %.1f^\\circ', align: left, size: 30}" % (miss(0).max(), miss(1).max())])

    y += ["  - title: REPEAT", "    steps:"]
    y += ["      - say: 'Repeat until every side lies on its needle.'", "        do:"]
    y += lines(["- camera: {fit: [LO, HI], dur: 1.2}",
                "- eq: {tex: 'k = 1:\\quad %.1f^\\circ', align: left, size: 34}" % miss(1).max()])
    for k in SHOWN:
        digits = "%.1f" if k < 10 else ("%.2f" if k < 50 else "%.3f")
        y += lines(moves(turned(k), 0.9) + [("- becomes: 'k = %d:\\quad " + digits + "^\\circ'") % (k, miss(k).max())])

    def thick(prefix, forces):
        return ["{id: %s%d, line: [%s, %s], color: ink, width: %.1f}" % (prefix, e, u, v, 1.0 + 2.2 * forces[e]) for e, (u, v, _, _) in enumerate(BARS)]

    y += ["  - title: READ THE FORCES", "    steps:"]
    y += ["      - say: 'Side length is thrust, drawn as thickness. Divided by bar length: force density.'", "        do:"]
    y += lines(["- remove: [%s]" % ", ".join("t%d" % e for e in range(m))])
    y += add(thick("h", thrust))
    y += lines(["- color: {target: h3, color: pink}", "- color: {target: f3, color: pink, with: true}",
                "- eq: {tex: 'q = \\frac{l^*}{r\\,l}', align: left, size: 34}",
                "- eq: {tex: 'q_{1,2} = \\frac{l^*_{BE}}{l_{1,2}} = \\frac{{=len(B, E):.2}}{%.2f} = {=len(B, E) / %.4f:.2}', align: left, size: 30}" % (S["length"][3], S["length"][3])])
    y += ["      - say: 'Four bars per node: other parallel diagrams exist, with other thrusts.'", "        do:"]
    y += lines(["- remove: [%s]" % ", ".join("h%d" % e for e in range(m))] + moves(other, 1.8))
    y += add(thick("k", thrust2))
    y += lines(["- color: {target: k3, color: pink}"])
    return y


def chapter_matrices():
    "The two matrices of the code, built from the drawing, and the loop as compas_tna writes it."
    final = turned(100)
    m = len(BARS)
    Cs = np.zeros((m, len(LETTERS)), dtype=int)
    for e, (_, _, a, b) in enumerate(BARS):
        Cs[e, LETTERS.index(a)], Cs[e, LETTERS.index(b)] = -1, 1
    L = Cs.T @ Cs

    def rows(M):
        return "[" + ", ".join("[" + ", ".join(str(x) for x in row) + "]" for row in M) + "]"

    def labels(names):
        return "[" + ", ".join("'%s'" % n for n in names) + "]"

    def row(items, size):
        return [" " * 10 + "- row:"] + [" " * 14 + "- " + i for i in items] + [" " * 12 + "size: %d" % size]

    y = ["title: MATRICES AND CODE", "split: 0.42", "camera: {view: top, fit: [LO, HI]}"]
    y += points(corners(final))
    y += ["slides:"]

    y += ["  - title: 'SIDES AND CORNERS: $\\mathbf{C}^*$'", "    steps:"]
    y += ["      - say: 'One row per side: $-1$ at the corner it leaves, $+1$ at the corner it reaches.'", "        do:"]
    y += add(["{id: rim, polygon: [%s], color: grey, width: 1.2, dash: true}" % ", ".join(RIM)]
             + ["{id: g%d, arrow: [%s, %s], color: ink, width: 2.2}" % (e, u, v) for e, (u, v, _, _) in enumerate(BARS)]
             + ["{id: %s, point: %s, color: fixed, size: 0.8}" % (k, k) for k in RIM]
             + ["{id: %s, point: %s, color: free, label: '%s'}" % (k, k, k[1]) for k in FREE]
             + ["{id: tg, label: TG, tex: '\\Gamma', color: accent}", "{id: tf, label: TF, tex: '\\Gamma^*', color: accent}"]
             + ["{id: f%d, arrow: [%s, %s], color: ink, width: 2.2}" % (e, a, b) for e, (_, _, a, b) in enumerate(BARS)]
             + corner_points())
    y += row(["{id: Cs, matrix: {rows: %s, rowlabels: %s, collabels: %s, name: '\\mathbf{C}^*', empty: true}}"
              % (rows(Cs), labels(a + b for _, _, a, b in BARS), labels(LETTERS))], 25)
    for e in range(m):
        y += lines(["- color: {target: g%d, color: pink}" % e, "- color: {target: f%d, color: pink, with: true}" % e,
                    "- show: ['Cs[%d,:]']" % e,
                    "- color: {target: g%d, color: ink, dur: 0.2}" % e, "- color: {target: f%d, color: ink, with: true, dur: 0.2}" % e])
    y += ["      - say: 'Times the corner coordinates it gives every side as a vector; $\\mathbf{C}$ does the same for the bars.'", "        do:"]
    y += code("""
uv = C.dot(xy)               # bars
_uv = _C.dot(_xy)            # sides
targets = normalizerow(uv)   # needles (alpha = 100)
_l = normrow(_uv)            # side lengths
""")

    y += ["  - title: 'THE LOOP IN compas_tna'", "    steps:"]
    y += ["      - say: 'parallelise_nodal: freeze, measure, ask every side, average.'", "        do:"]
    y += code("""
for k in range(kmax):
    xy0 = xy.copy()                     # freeze
    l = normrow(C.dot(xy))              # measure the sides
    for j in range(n):                  # every corner
        xy[j, :] = 0.0
        for i in nbrs:                  # every side at j
            if (i, j) in ij_e:          # side runs i -> j
                xy[j] += xy0[i] + l[e, 0] * targets[e]
            else:                       # side runs j -> i
                xy[j] += xy0[i] - l[e, 0] * targets[e]
        xy[j] /= len(nbrs)              # average

f = _l                                  # side lengths = thrusts
q = f / l                               # force densities
_xy[:] = rot90(_xy, -1.0)               # stored turned back
""")

    j = LETTERS.index("E")
    y += ["  - title: 'ALL CORNERS IN ONE SOLVE'", "    steps:"]
    y += ["      - say: 'The average of corner $E$ written as a row: its $4$ sides on the diagonal, $-1$ for each neighbour.'", "        do:"]
    y += row(["{id: Lm, matrix: {rows: %s, rowlabels: %s, collabels: %s, name: '\\mathbf{C}^{*T}\\mathbf{C}^*'}}" % (rows(L), labels(LETTERS), labels(LETTERS))], 27)
    y += lines(["- color: {target: 'Lm[%d,:]', color: yellow}" % j] + ["- color: {target: f%d, color: pink, with: true}" % e for e in range(m) if Cs[e, j]])
    y += lines(["- eq: {tex: '4\\,\\mathbf{x}_E - \\mathbf{x}_B - \\mathbf{x}_D - \\mathbf{x}_F - \\mathbf{x}_H = \\sum_i \\pm\\, l_i\\,\\mathbf{t}_i', align: left, size: 24}"])
    y += ["      - say: 'horizontal_numpy solves all nine rows at once: the corners that fit every side best to its target.'", "        do:"]
    y += lines(["- eq: {tex: '\\mathbf{C}^{*T}\\mathbf{C}^*\\,\\mathbf{x}^* = \\mathbf{C}^{*T}(\\mathbf{l}^* \\cdot \\mathbf{t})', color: blue, align: left, size: 28}",
                "- eq: {tex: '\\min\\ \\tfrac{1}{2} \\sum_e \\left| (\\mathbf{x}^*_v - \\mathbf{x}^*_u) - l^*_e\\,\\mathbf{t}_e \\right|^2', align: left, size: 26}"])
    y += code("""
_xy = parallelise_sparse(_Ct_C, _Ct.dot(_l * t), _xy, _fixed, "_Ct_C")
""")
    return y


def chapter_A():
    "The equilibrium matrix A of the algebraic methods, built row by row from the plan."
    xy, index = S["xy"], S["index"]
    m = len(BARS)
    free = [index[k] for k in FREE]
    C = np.zeros((m, len(xy)), dtype=int)
    for e, (u, v, a, b) in enumerate(BARS):
        C[e, index[u]], C[e, index[v]] = -1, 1
    uv = C @ xy
    Ci = C[:, free]
    A = np.vstack([Ci.T * uv[:, 0], Ci.T * uv[:, 1]])
    bars = ["e_{%d}" % e for e in range(m)]

    def num(x):
        return "0" if abs(x) < 1e-9 else "'%.1f'" % x

    def rows(M, fmt=str):
        return "[" + ", ".join("[" + ", ".join(fmt(x) for x in row) + "]" for row in M) + "]"

    def labels(names):
        return "[" + ", ".join("'%s'" % n for n in names) + "]"

    def row(items, size):
        return [" " * 10 + "- row:"] + [" " * 14 + "- " + i for i in items] + [" " * 12 + "size: %d" % size]

    y = ["title: THE EQUILIBRIUM MATRIX", "split: 0.4", "camera: {view: top, fit: [LO, HI]}"]
    y += points(corners(turned(100)))
    y += ["slides:"]
    y += ["  - title: 'BARS AND FREE NODES: $\\mathbf{C}_i$'", "    steps:"]
    y += ["      - say: 'One row per bar, one column per free node: $-1$ where the bar starts, $+1$ where it ends.'", "        do:"]
    y += add(["{id: rim, polygon: [%s], color: grey, width: 1.2, dash: true}" % ", ".join(RIM)]
             + ["{id: g%d, arrow: [%s, %s], color: ink, width: 2.2}" % (e, u, v) for e, (u, v, _, _) in enumerate(BARS)]
             + ["{id: %s, point: %s, color: fixed, size: 0.8}" % (k, k) for k in RIM]
             + ["{id: %s, point: %s, color: free, label: '%s'}" % (k, k, k[1]) for k in FREE]
             + ["{id: tg, label: TG, tex: '\\Gamma', color: accent}", "{id: tf, label: TF, tex: '\\Gamma^*', color: accent}"]
             + sides(2.4) + corner_points())
    y += row(["{id: Ci, matrix: {rows: %s, rowlabels: %s, collabels: ['1', '2', '3', '4'], name: '\\mathbf{C}_i', empty: true}}" % (rows(Ci), labels(bars))], 27)
    for e in range(m):
        y += lines(["- color: {target: g%d, color: pink}" % e, "- show: ['Ci[%d,:]']" % e, "- color: {target: g%d, color: ink, dur: 0.2}" % e])
    y += ["  - title: 'BAR VECTORS: $\\mathbf{u}$ AND $\\mathbf{v}$'", "    steps:"]
    y += ["      - say: 'With all twelve node columns, times the coordinates: every bar as a vector.'", "        do:"]
    y += lines(["- eq: {tex: '\\mathbf{u} = \\mathbf{C}\\,\\mathbf{x} \\qquad \\mathbf{v} = \\mathbf{C}\\,\\mathbf{y}', align: left, size: 28}"])
    y += row(["{id: uu, matrix: {rows: %s, rowlabels: %s, name: '\\mathbf{u}'}}" % (rows(uv[:, [0]], num), labels(bars)), "'\\quad'",
              "{id: vv, matrix: {rows: %s, name: '\\mathbf{v}'}}" % rows(uv[:, [1]], num)], 26)
    node1 = [e for e in range(m) if Ci[e, 0]]
    y += ["  - title: 'ONE NODE, TWO EQUATIONS: $\\mathbf{A}$'", "    steps:"]
    y += ["      - say: 'Node $1$ along $x$: its four bars, each with its $u$ and a sign from $\\mathbf{C}_i$, times the unknown $q$.'", "        do:"]
    y += lines(["- color: {target: g%d, color: pink%s}" % (e, ", with: true" if i else "") for i, e in enumerate(node1)])
    y += lines(["- eq: {tex: '\\mathbf{A} = \\left[\\ \\mathbf{C}_i^T\\mathbf{U}\\ ;\\ \\mathbf{C}_i^T\\mathbf{V}\\ \\right] \\qquad \\mathbf{A}\\,\\mathbf{q} = \\mathbf{0}', align: left, size: 26}"])
    names = ["%d_x" % (i + 1) for i in range(4)] + ["%d_y" % (i + 1) for i in range(4)]
    y += row(["{id: Am, matrix: {rows: %s, rowlabels: %s, collabels: %s, name: '\\mathbf{A}', empty: true}}" % (rows(A, num), labels(names), labels(bars))], 22)
    y += lines(["- show: ['Am[0,:]']"])
    y += ["      - say: 'Along $y$ the same with $v$. Two rows per free node: $8$ equations for $12$ unknown $q$.'", "        do:"]
    y += lines(["- show: ['Am[4,:]']"] + ["- color: {target: g%d, color: ink%s}" % (e, ", with: true" if i else "") for i, e in enumerate(node1)]
               + ["- show: [%s]" % ", ".join("'Am[%d,:]'" % r for r in (1, 2, 3, 5, 6, 7)),
                  "- eq: {tex: 'k = m - \\text{rank}\\,\\mathbf{A} = 12 - 8 = 4\\ \\text{free states}', align: left, size: 26}"])
    return y


def chapter6():
    "The same net solved four ways, side by side, in the turned frame (sides parallel to bars)."
    from scipy.optimize import linprog

    xy, index, t, length = S["xy"], S["index"], S["targets"], S["length"]
    m, n = len(BARS), len(LETTERS)
    start = S["history"][0]
    free = [index[k] for k in FREE]
    C = np.zeros((m, len(xy)))
    Cs = np.zeros((m, n))
    for e, (u, v, a, b) in enumerate(BARS):
        C[e, index[u]], C[e, index[v]] = -1.0, 1.0
        Cs[e, LETTERS.index(a)], Cs[e, LETTERS.index(b)] = -1.0, 1.0
    uv = C @ xy
    A = np.vstack([C[:, free].T * uv[:, 0], C[:, free].T * uv[:, 1]])

    def drawn(q):
        "Corners of the diagram whose sides are q times the bars: least squares, centred."
        X = np.linalg.lstsq(Cs, q[:, None] * uv, rcond=None)[0]
        return X - X.mean(axis=0)

    def worst(X):
        d = Cs @ X
        d /= np.linalg.norm(d, axis=1)[:, None]
        return np.degrees(np.arccos(np.clip((d * t).sum(axis=1), -1.0, 1.0))).max()

    # 1. Block 2009: a linear programme over the length ratios
    lp = linprog(np.ones(m), A_eq=A, b_eq=np.zeros(len(A)), bounds=[(1.0 / l, None) for l in length], method="highs")
    q_lp = lp.x

    # 2. Block and Lachauer 2014: independent force densities from the reduced row echelon form
    R, pivots, row = A.copy(), [], 0
    for col in range(m):
        if row == len(R):
            break
        k = row + np.abs(R[row:, col]).argmax()
        if abs(R[k, col]) < 1e-9:
            continue
        R[[row, k]] = R[[k, row]]
        R[row] /= R[row, col]
        for other in range(len(R)):
            if other != row:
                R[other] -= R[other, col] * R[row]
        pivots.append(col)
        row += 1
    indep = [e for e in range(m) if e not in pivots]
    q_al = np.zeros(m)
    q_al[indep] = ((Cs @ start) * t).sum(axis=1)[indep] / length[indep]
    q_al[pivots] = -np.linalg.solve(A[:, pivots], A[:, indep] @ q_al[indep])
    assert q_al.min() > 0 and np.abs(A @ q_lp).max() < 1e-8

    # 3. Rippmann 2012: averaging, already in S. 4. least squares, one corner held
    ls, X = [start.copy()], start.copy()
    L = Cs.T @ Cs
    for k in range(50):
        l = np.linalg.norm(Cs @ X, axis=1)[:, None]
        rhs = Cs.T @ (l * t)
        X = X.copy()
        X[1:] = np.linalg.solve(L[1:, 1:], rhs[1:] - L[1:, [0]] @ X[[0]])
        ls.append(X)

    scale = 0.74
    centre = start.mean(axis=0)
    where = {"a": np.array([2.3, 7.0]), "b": np.array([7.3, 7.0]), "e": np.array([12.3, 7.0]), "c": np.array([2.3, 2.3]), "d": np.array([7.3, 2.3])}
    q_bl, _ = bounded_densities(xy, [(index[u], index[v]) for u, v, _, _ in BARS], free)
    size = np.linalg.norm(Cs @ start, axis=1).mean()

    def panel(key, X, own=False):
        X = np.array(X)
        if own:
            X = (X - X.mean(axis=0)) * size / np.linalg.norm(Cs @ X, axis=1).mean()
        else:
            X = X - centre
        return X * scale + where[key]

    fixed = {"a": panel("a", drawn(q_lp), True), "b": panel("b", drawn(q_al), True), "e": panel("e", drawn(q_bl), True)}
    extra = {}
    for key in "abe":
        for i in range(n):
            extra["q%s%d" % (key, i)] = fixed[key][i]
    for key in "cd":
        for i in range(n):
            extra["q%s%d" % (key, i)] = panel(key, start)[i]
    for key, c in where.items():
        extra["T" + key] = c + np.array([-2.2, 2.35])
    form = (xy - np.array([3.0, 3.0])) * 0.62 + np.array([12.3, 2.2])

    y = ["title: FIVE METHODS SIDE BY SIDE", "split: 0.62", "camera: {view: top, fit: [M0, M1]}", "points:"]
    y += ["  %s: %s" % (k, P(v)) for k, v in extra.items()]
    y += ["  M0: [0.8, 1.2, 0]", "  M1: [13.8, 8.6, 0]", "  TG: [10.2, 4.3, 0]", "slides:"]

    def diagram(key, needle):
        out = ["{id: %sf%d, line: [q%s%d, q%s%d], color: ink, width: 2.4}" % (key, e, key, LETTERS.index(a), key, LETTERS.index(b)) for e, (_, _, a, b) in enumerate(BARS)]
        if needle:
            for e, (_, _, a, b) in enumerate(BARS):
                d = "[%.3f, %.3f, 0]" % tuple(0.3 * t[e])
                ends = "mid(q%s%d, q%s%d)" % (key, LETTERS.index(a), key, LETTERS.index(b))
                out.append("{id: %st%d, arrow: ['%s - %s', '%s + %s'], color: blue, width: 1.2}" % (key, e, ends, d, ends, d))
        return out

    def go(key, X):
        return ["- move: {point: q%s%d, to: %s, dur: 1.2, with: true}" % (key, i, P(panel(key, X)[i])) for i in range(n)]

    def tag(key, text):
        return "- morph: {target: t%s, label: T%s, tex: '%s', dur: 0.4, with: true}" % (key, key, text)

    names = {"a": "\\text{1 · linear programme}", "b": "\\text{2 · independent } q", "c": "\\text{3 · averaging}", "d": "\\text{4 · least squares}", "e": "\\text{5 · bounded fit}"}
    y += ["  - title: ONE PLAN, FIVE SOLVERS", "    steps:"]
    y += ["      - say: 'The same plan and the same targets. Methods 3 and 4 start from the first guess; 1, 2 and 5 need none.'", "        do:"]
    y += add(["{id: gb%d, line: [%s, %s], color: ink, width: 2.0}" % (e, P(form[index[u]]), P(form[index[v]])) for e, (u, v, _, _) in enumerate(BARS)]
             + ["{id: gs%s, point: %s, color: fixed, size: 0.6}" % (k, P(form[index[k]])) for k in RIM]
             + ["{id: tg, label: TG, tex: '\\Gamma', color: accent}"]
             + ["{id: t%s, label: T%s, tex: '%s', color: accent}" % (k, k, names[k]) for k in "abe"]
             + ["{id: t%s, label: T%s, tex: '%s\\quad k = 0:\\ %.1f^\\circ', color: accent}" % (k, k, names[k], worst(start)) for k in "cd"]
             + diagram("c", True) + diagram("d", True))

    y += ["  - title: 1 · LINEAR PROGRAMME (BLOCK 2009)", "    steps:"]
    y += ["      - say: 'Unknowns: one ratio $t_e = l^*_e / l_e$ per bar. Every node must close; every ratio stays positive.'", "        do:"]
    y += lines(["- eq: {tex: '\\min \\sum_e t_e', align: left, size: 28}",
                "- eq: {tex: '\\mathbf{C}_i^T\\mathbf{U}\\,\\mathbf{t} = \\mathbf{0} \\qquad \\mathbf{C}_i^T\\mathbf{V}\\,\\mathbf{t} = \\mathbf{0}', align: left, size: 28}",
                "- eq: {tex: 't_e \\ge d / l_e', align: left, size: 28}"])
    y += ["      - say: 'One solve gives all twelve ratios. Each side is its bar times its ratio, so the diagram can be drawn at once.'", "        do:"]
    y += add(diagram("a", False))
    y += lines([tag("a", names["a"] + "\\quad %.1f^\\circ" % abs(worst(drawn(q_lp)))),
                "- eq: {tex: '\\mathbf{C}^*\\mathbf{x}^* = \\mathbf{U}\\,\\mathbf{t} \\qquad \\mathbf{C}^*\\mathbf{y}^* = \\mathbf{V}\\,\\mathbf{t}', align: left, size: 28}"])

    y += ["  - title: 2 · INDEPENDENT FORCE DENSITIES (BLOCK AND LACHAUER 2014)", "    steps:"]
    y += ["      - say: 'Row reduction of the equilibrium matrix finds $k = %d$ bars whose force can be chosen freely, drawn pink.'" % len(indep), "        do:"]
    y += lines(["- color: {target: gb%d, color: pink%s}" % (e, ", with: true" if i else "") for i, e in enumerate(indep)])
    y += lines(["- eq: {tex: '\\mathbf{A}\\,\\mathbf{q} = \\mathbf{0} \\qquad k = m - \\text{rank}\\,\\mathbf{A} = %d - %d = %d', align: left, size: 28}" % (m, len(pivots), len(indep))])
    y += ["      - say: 'Choose those four; the other eight follow from one small solve, and the diagram is drawn from $q$.'", "        do:"]
    y += add(diagram("b", False))
    y += lines([tag("b", names["b"] + "\\quad %.1f^\\circ" % abs(worst(drawn(q_al)))),
                "- eq: {tex: '\\mathbf{q}_{dep} = -\\mathbf{A}_{dep}^{-1}\\,\\mathbf{A}_{indep}\\,\\mathbf{q}_{indep}', align: left, size: 28}"])

    y += ["  - title: 5 · BOUNDED FIT (VOUGA, HÖBINGER, WALLNER, POTTMANN 2012)", "    steps:"]
    y += ["      - say: 'Fit all force densities at once to the node equations, each kept between two bounds. No first guess, no rank.'", "        do:"]
    y += lines(["- color: {target: gb%d, color: ink%s}" % (e, ", with: true" if i else "") for i, e in enumerate(indep)])
    y += lines(["- eq: {tex: '\\min_{\\mathbf{q}}\\ \\left| \\mathbf{A}\\,\\mathbf{q} \\right|^2 \\qquad q_{min} \\le q_e \\le q_{max}', align: left, size: 28}"])
    y += ["      - say: 'Here the minimum is zero, so the plan stands and the diagram is drawn from $q$. If it is not zero, the nodes of the plan are moved.'", "        do:"]
    y += add(diagram("e", False))
    y += lines([tag("e", names["e"] + "\\quad %.1f^\\circ" % abs(worst(drawn(q_bl))))])

    y += ["  - title: 3 AND 4 · THE TWO LOOPS", "    steps:"]
    y += ["      - say: 'Method 3 moves each corner to the average of its wishes. Method 4 solves for all corners at once.'", "        do:"]
    y += lines(["- eq: {tex: '3:\\quad \\mathbf{x}_j \\leftarrow \\frac{1}{n_j} \\sum_i (\\mathbf{x}_i \\pm l_i\\,\\mathbf{t}_i)', align: left, size: 26}",
                "- eq: {tex: '4:\\quad \\mathbf{C}^{*T}\\mathbf{C}^*\\,\\mathbf{x}^* = \\mathbf{C}^{*T}(\\mathbf{l}^* \\cdot \\mathbf{t})', align: left, size: 26}"])
    for k in [1, 2, 3, 5, 10, 20, 50]:
        y += ["      - say: 'Iteration $%d$.'" % k, "        do:"]
        rows = go("c", S["history"][k]) + go("d", ls[k])
        rows[0] = rows[0].replace(", with: true", "")
        y += lines(rows)
        fmt = "%.1f" if k < 10 else "%.3f"
        y += lines([tag("c", names["c"] + ("\\quad k = %d:\\ " + fmt + "^\\circ") % (k, worst(S["history"][k]))),
                    tag("d", names["d"] + ("\\quad k = %d:\\ " + fmt + "^\\circ") % (k, worst(ls[k])))])

    q_c = np.linalg.norm(Cs @ S["history"][100], axis=1) / length
    q_d = np.linalg.norm(Cs @ ls[50], axis=1) / length
    y += ["  - title: FIVE ANSWERS", "    steps:"]
    y += ["      - say: 'All five diagrams are parallel, and they differ: each method picks another of the possible force distributions.'", "        do:"]
    y += lines(["- eq: {tex: 'q_{1,2}:\\quad %.2f \\qquad %.2f \\qquad %.2f \\qquad %.2f \\qquad %.2f', align: left, size: 28}" % (q_lp[3] / q_lp.mean(), q_al[3] / q_al.mean(), q_c[3] / q_c.mean(), q_d[3] / q_d.mean(), q_bl[3] / q_bl.mean()),
                "- text: 'Force density of bar $1$–$2$ in methods 1 to 5, each divided by its mean so the scales compare.'",
                "- text: '1, 2 and 5 are one solve each. 3 and 4 need a first guess and many steps. 3, 4 and 5 can also move the plan.'"])
    return y


def pattern_data():
    import json

    with open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "patterns.json")) as f:
        return json.load(f)


def stored_diagram(xy, d, q):
    "Force diagram of force densities q as compas_tna stores it: sides perpendicular to bars."
    Cs = np.zeros((len(d["_edges"]), len(d["start"])))
    for e, (a, b) in enumerate(d["_edges"]):
        Cs[e, a], Cs[e, b] = -1.0, 1.0
    uv = np.array([xy[v] - xy[u] for u, v in d["edges"]])
    X = np.linalg.lstsq(Cs, q[:, None] * uv, rcond=None)[0]
    return np.column_stack([X[:, 1], -X[:, 0]])


def chapter_repair():
    "The plans that failed, repaired by moving their nodes: Vouga et al. 2012, kept to the plan."
    data = pattern_data()
    top = ([0.2, UP[1] + 0.1], [5.8, UP[1] + 5.4])
    low = ([0.4, 0.5], [5.6, 5.5])
    cases = [("quadbad", "QUADS, DISTORTED"), ("hexbad", "HEXAGONS, DISTURBED"), ("open", "OPEN EDGE, STRAIGHT")]
    extra, scenes = {}, {}
    for name, _ in cases:
        d = data[name]
        xy0 = np.array(d["xy"])
        xy1, q, its, hist = self_supporting_plan(xy0, d["edges"], d["fixed"])
        bar = np.median(np.linalg.norm(np.array([xy0[v] - xy0[u] for u, v in d["edges"]]), axis=1))
        both = boxed(np.vstack([xy0, xy1]), *top)
        scenes[name] = (both[: len(xy0)], both[len(xy0):], boxed(stored_diagram(xy1, d, q), *low), its, hist, np.abs(xy1 - xy0).max() / bar)
        for i in range(len(xy0)):
            extra["%s%d" % (name, i)] = both[i]
    y = ["title: MOVING THE PLAN", "split: 0.5", "camera: {view: top, fit: [LO, HI]}"]
    y += points(extra)
    y += ["slides:"]
    before = []
    for name, title in cases:
        d = data[name]
        p0, p1, force, its, hist, moved = scenes[name]
        ghost = ["{id: %sg%d, line: [%s, %s], color: grey, width: 1.0, dash: true}" % (name, e, P(p0[u]), P(p0[v])) for e, (u, v) in enumerate(d["edges"])]
        bars = ["{id: %sb%d, line: [%s%d, %s%d], color: ink, width: 2.0}" % (name, e, name, u, name, v) for e, (u, v) in enumerate(d["edges"])]
        sups = ["{id: %ss%d, point: %s, color: fixed, size: 0.6}" % (name, i, P(p0[i])) for i in d["fixed"]]
        sides_ = ["{id: %sf%d, line: [%s, %s], color: ink, width: 2.0}" % (name, e, P(force[a]), P(force[b])) for e, (a, b) in enumerate(d["_edges"])]
        y += ["  - title: %s" % title, "    steps:"]
        y += ["      - say: 'Fit force densities between two bounds to this plan. The best fit leaves $%.0f$ percent of a node force unbalanced.'" % (100 * hist[0]), "        do:"]
        if before:
            y += lines(["- remove: [%s]" % ", ".join(before)])
        else:
            y += add(["{id: tg, label: TG, tex: '\\Gamma', color: accent}", "{id: tf, label: TF, tex: '\\Gamma^*', color: accent}"])
        y += add(ghost + bars + sups)
        y += lines(["- eq: {tex: '\\min_{\\mathbf{q}}\\ \\left| \\mathbf{A}\\,\\mathbf{q} \\right|^2 \\qquad q_{min} \\le q_e \\le q_{max}', align: left, size: 28}"])
        y += ["      - say: 'Move the free nodes to reduce what is unbalanced, staying close to where they were. Fit again.'", "        do:"]
        moving = [i for i in range(len(p0)) if np.abs(p1[i] - p0[i]).max() > 1e-9]
        y += lines(["- move: {point: %s%d, to: %s, dur: 1.6%s}" % (name, i, P(p1[i]), ", with: true" if n else "") for n, i in enumerate(moving)])
        y += lines(["- eq: {tex: '\\min_{\\mathbf{x}_N}\\ \\left| \\mathbf{D}_N\\,\\mathbf{x}_N + \\mathbf{D}_F\\,\\mathbf{x}_F \\right|^2 + \\beta\\,\\left| \\mathbf{x}_N - \\mathbf{x}_N^0 \\right|^2', align: left, size: 26}",
                    "- eq: {tex: '%d\\ \\text{iteration%s} \\qquad \\text{largest move: } %.2f\\ \\text{bar lengths}', align: left, size: 24}" % (its, "" if its == 1 else "s", moved)])
        y += ["      - say: 'Nothing is left unbalanced: this plan has a compression-only force diagram.'", "        do:"]
        y += add(sides_)
        before = [o.split(",")[0][5:] for o in ghost + bars + sups + sides_]
    y += ["  - title: WHAT IT COSTS", "    steps:"]
    y += ["      - say: 'The price is the plan itself: it is no longer the one that was drawn. The dashed lines show how far it went.'", "        do:"]
    y += lines(["- text: 'Vouga, Höbinger, Wallner and Pottmann 2012 move plan and heights together to stay near a given surface. Here only the plan step is used; the heights follow in the next chapter.'"])
    return y


def chapter_vaults():
    "Every pattern lifted to its thrust network by the height solve."
    data = pattern_data()
    order = [("tri", "Triangles."), ("quad", "Quads."), ("quadbad", "Distorted quads, on their repaired plan."), ("hex", "Hexagons."),
             ("hexbad", "Disturbed hexagons, on their repaired plan."), ("dome", "The dome plan."),
             ("open", "The straight open edge, repaired: it has become an arch."), ("arch", "The arched open edge.")]
    extra, scenes = {}, {}
    for name, _ in order:
        d = data[name]
        xy, q, _, _ = self_supporting_plan(np.array(d["xy"]), d["edges"], d["fixed"])
        z = heights(xy, d["edges"], d["fixed"], q, 1.0)
        flat = boxed(xy, [-3.0, -3.0], [3.0, 3.0])
        used = sorted(set(np.array(d["edges"]).ravel()))
        scenes[name] = (flat, 2.2 * z / z.max(), used)
        for i in used:
            extra["%s%d" % (name, i)] = flat[i]
    y = ["title: THE VAULTS", "split: 0.56", "camera: {yaw: -118, pitch: 28, fit: [V0, V1]}", "points:"]
    y += ["  %s: [%.3f, %.3f, 0]" % (k, v[0], v[1]) for k, v in extra.items()]
    y += ["  V0: [-3.0, -3.0, 0]", "  V1: [3.0, 3.0, 2.2]", "slides:", "  - title: FROM PLAN TO THRUST NETWORK", "    steps:"]
    before = []
    for n, (name, say) in enumerate(order):
        d = data[name]
        flat, z, used = scenes[name]
        plan = ["{id: %sg%d, line: [%s, %s], color: grey, width: 1.0}" % (name, e, P(flat[u]), P(flat[v])) for e, (u, v) in enumerate(d["edges"])]
        bars = ["{id: %sb%d, line: [%s%d, %s%d], color: ink, width: 2.2}" % (name, e, name, u, name, v) for e, (u, v) in enumerate(d["edges"])]
        sups = ["{id: %ss%d, point: %s, color: fixed, size: 0.6}" % (name, i, P(flat[i])) for i in d["fixed"]]
        y += ["      - say: '%s'" % say, "        do:"]
        if before:
            y += lines(["- remove: [%s]" % ", ".join(before)])
        y += add(plan + bars + sups)
        lifted = [i for i in used if z[i] > 1e-9]
        y += lines(["- move: {point: %s%d, to: [%.3f, %.3f, %.3f], dur: 1.8%s}" % (name, i, flat[i][0], flat[i][1], z[i], ", with: true" if k else "") for k, i in enumerate(lifted)])
        if not n:
            y += lines(["- eq: {tex: '\\mathbf{z}_N = \\mathbf{D}_N^{-1}\\,(r\\,\\mathbf{p} - \\mathbf{D}_F\\,\\mathbf{z}_F)', align: left, size: 30}",
                        "- text: 'The same load on every free node; $r$ is set so that all vaults are equally high.'"])
        before = [o.split(",")[0][5:] for o in plan + bars + sups]
    return y


def split(rows):
    "A sentence written as 'say || more': the second part becomes the first line of prose below it."
    out, more = [], None
    for row in rows:
        if "- say: '" in row and " || " in row:
            row, more = row.split(" || ")
            row, more = row + "'", more[:-1]
        if more and row.strip() == "do:":
            out.append(row)
            row, more = " " * 10 + "- text: '%s'" % more, None
        if row.strip().startswith("- text: '"):
            row = row.replace("- text: '", "- {text: '", 1) + ", align: left}"
        out.append(row)
    return out


here = os.path.dirname(os.path.abspath(__file__))
DECKS = [
    ("thrust-network_analysis_horizontal_equilibrium", "Thrust network analysis · horizontal equilibrium",
     "How compas_tna computes horizontal equilibrium: the force diagram is made parallel to the form diagram.",
     [("01-force-diagram.yaml", chapter_geometry), ("02-matrices-and-code.yaml", chapter_matrices)]),
    ("thrust-network_analysis_horizontal_methods", "Thrust network analysis · five ways to the force diagram",
     "Five published methods for horizontal equilibrium, solved side by side on the same plan.",
     [("01-equilibrium-matrix.yaml", chapter_A), ("02-methods.yaml", chapter6)]),
    ("thrust-network_analysis_geometries", "Thrust network analysis · plans that stand",
     "Which plans have a compression-only force diagram, how a failing plan is repaired, and the vaults they give.",
     [("01-patterns.yaml", chapter5), ("02-moving-the-plan.yaml", chapter_repair), ("03-vaults.yaml", chapter_vaults)]),
]
for deck, title, about, chapters in DECKS:
    folder = os.path.join(os.path.dirname(here), deck)
    os.makedirs(folder, exist_ok=True)
    head = HEAD.replace("thrust-network_analysis_horizontal_equilibrium", deck).replace("in this folder", "in ../thrust-network_analysis_horizontal_equilibrium")
    with open(os.path.join(folder, "index.yaml"), "w") as f:
        f.write("# Open live: https://petrasvestartas.github.io/session/?stage=%s\n\n# %s\n# The chapters are written by generate.py in ../thrust-network_analysis_horizontal_equilibrium.\nname: %s\nchapters:\n" % (deck, about, title))
        f.write("".join("  - %s\n" % name for name, _ in chapters))
    for n, (name, chapter) in enumerate(chapters, 1):
        with open(os.path.join(folder, name), "w") as f:
            f.write(head.format(n=n) + "\n".join(split(chapter())) + "\n")
print("largest miss:", ", ".join("k=%d %.4f" % (k, miss(k).max()) for k in [0, 1] + SHOWN))
