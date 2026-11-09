import threading
import webbrowser
from math import sqrt
from pathlib import Path

from session_py import Arrowhead
from session_py import Color
from session_py import Line
from session_py import Matrix
from session_py import Point
from session_py import Polyline
from session_py import Session
from session_py import Text
from session_py import Xform
from session_py.viewer import MatrixEquation
from session_py.viewer import serve

FREE = Color(220 / 255, 75 / 255, 141 / 255)
FIXED = Color(98 / 255, 160 / 255, 1.0)
LOAD = Color(74 / 255, 203 / 255, 106 / 255)
INK = Color(232 / 255, 237 / 255, 243 / 255)
GREY = Color(123 / 255, 132 / 255, 146 / 255)
PALE = Color(37 / 255, 43 / 255, 53 / 255)
X = Color(239 / 255, 119 / 255, 112 / 255)
Y = Color(98 / 255, 204 / 255, 124 / 255)
Z = Color(114 / 255, 164 / 255, 239 / 255)
BARS = [
    Color(180 / 255, 147 / 255, 232 / 255),
    Color(242 / 255, 173 / 255, 46 / 255),
    Color(54 / 255, 201 / 255, 184 / 255),
    Color(207 / 255, 139 / 255, 94 / 255),
]
NODES = [[3, 3, 0.25], [0, 0, 0], [5, 0, 3], [0, 7, 3], [7, 5, 0]]
EDGES = [(0, 1), (0, 2), (0, 3), (0, 4)]
CONNECTION = [[1, -1, 0, 0, 0], [1, 0, -1, 0, 0], [1, 0, 0, -1, 0], [1, 0, 0, 0, -1]]
LENGTHS = [4.254, 4.535, 5.704, 4.478]
TITLES = [
    ("1 · Equilibrium", "Four cable forces and the external load form a closed polygon."),
    ("2 · Given", "The fixed coordinates and every force density are known."),
    ("3 · Connectivity", "One row per cable: +1 at n₀, −1 at its fixed neighbour."),
    ("4 · Direction cosines", "One cable, one box, three coordinate projections."),
    ("5 · Cable e₁", "The x-component of one cable."),
    ("6 · Cable e₀", "A negative x-component points the other way."),
    ("7 · Cable e₂", "Another cable, another signed x-component."),
    ("8 · Cable e₃", "The last x-component."),
    ("9 · Equilibrium along x", "Only the four signed x-projections matter here."),
    ("10 · Force density", "qᵢ = fᵢ / lᵢ turns the nonlinear balance into a linear one."),
    ("11 · Equal force densities", "Two equal force densities put n₀ at the midpoint."),
    ("12 · A stronger neighbour", "Increasing q₂ from 1 to 3 pulls n₀ from 5 to 7.5."),
    ("13 · An external load", "With p = 4 and equal force densities, n₀ moves to 7."),
    ("14 · Length is a function of n₀", "Moving n₀ changes each cable length."),
    ("15 · The slope of a length", "Only the motion along a cable changes its length."),
    ("16 · The Jacobian row", "Four direction cosines collect the four length slopes."),
    ("17 · Diagonal matrices", "Coordinate differences divided by lengths give the same row."),
    ("18 · Equilibrium with J", "Row times column collects the four x-components."),
    ("19 · Lengths cancel", "Force over length replaces L⁻¹f by q."),
    ("20 · Coordinates from C", "Each row subtracts a fixed coordinate from the free one."),
    ("21 · Free-node contribution", "Each row of Cₙ copies x₀ into the free-node part of a coordinate difference."),
    ("22 · Fixed-node contribution", "Each row of C𝒇 selects and negates one fixed x-coordinate."),
    ("23 · Add the contributions", "Add matching entries to obtain every x-coordinate difference."),
    ("24 · One linear equation", "Known terms move to the right-hand side."),
    ("25 · Solve three axes", "One division per axis gives n₀ = (3, 3, 0.25)."),
    ("26 · A small cable net", "A collects the force-density weights around every free node."),
    ("27 · Node a", "One matrix row is one free node's equilibrium; follow its cables and its x-balance."),
    ("28 · Node b", "The same row-by-row construction, now at node b."),
    ("29 · Node c", "The interior node connects to all four surrounding free nodes."),
    ("30 · Node d", "The same equilibrium row at node d."),
    ("31 · Node e", "The last free-node row completes the five by five system."),
    ("32 · Three coordinate systems", "The same A acts independently on x, y, and z; their right-hand sides differ."),
    ("33 · Relaxed coordinates", "Each coordinate row pulses the free node it places in the relaxed net."),
    ("34 · Verify node c", "The four highlighted cables produce one matrix row, checked independently for x, y, and z."),
    ("35 · One local rule, repeated", "Every interior node balances four unit-weight cables by averaging its four neighbors."),
    ("36 · Recover cable forces", "Read q and length from each highlighted cable, then recover its force."),
]
NETWORK = {0, 1, 2, 3, 4, 5, 6, 7, 9, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24}


def grid_net(size, lift, load):
    xyz = [[i, j, 0.0] for j in range(size) for i in range(size)]
    fixed = [0, size - 1, size * (size - 1), size * size - 1]
    free = [index for index in range(size * size) if index not in fixed]
    xyz[size - 1][2] = lift
    xyz[size * (size - 1)][2] = lift
    edges = []
    for index in range(size * size):
        if index % size < size - 1:
            edges.append((index, index + 1))
        if index < size * (size - 1):
            edges.append((index, index + size))
    densities = []
    for a, b in edges:
        boundary = (xyz[a][0] == xyz[b][0] and xyz[a][0] in (0, size - 1)) or (xyz[a][1] == xyz[b][1] and xyz[a][1] in (0, size - 1))
        densities.append(8 if boundary else 1)
    index_of = {node: index for index, node in enumerate(free)}
    laplacian = [[0.0 for _ in free] for _ in free]
    for (a, b), q in zip(edges, densities):
        for node, other in ((a, b), (b, a)):
            if node not in index_of:
                continue
            row = index_of[node]
            laplacian[row][row] += q
            if other in index_of:
                laplacian[row][index_of[other]] -= q
    solved = [point[:] for point in xyz]
    for axis in range(3):
        rows = []
        for row, node in enumerate(free):
            rhs = load if axis == 2 else 0.0
            for (a, b), q in zip(edges, densities):
                if a == node and b in fixed:
                    rhs += q * xyz[b][axis]
                elif b == node and a in fixed:
                    rhs += q * xyz[a][axis]
            rows.append(laplacian[row][:] + [rhs])
        for col in range(len(free)):
            pivot = max(range(col, len(free)), key=lambda row: abs(rows[row][col]))
            rows[col], rows[pivot] = rows[pivot], rows[col]
            for row in range(len(free)):
                if row == col:
                    continue
                factor = rows[row][col] / rows[col][col]
                for k in range(col, len(free) + 1):
                    rows[row][k] -= factor * rows[col][k]
        for row, node in enumerate(free):
            solved[node][axis] = rows[row][-1] / rows[row][row]
    return xyz, solved, fixed, free, edges, densities, laplacian


screen = Xform()
screen.m[5] = -1.0
screen.m[13] = 760.0
deck = []

for topic, (title, caption) in enumerate(TITLES):
    title = title.replace(" · ", " ")
    scene = Session(f"{title}\n{caption}")
    nodes = []
    cables = []
    force_labels = []
    matrix = None
    matrices = []
    matrix_at = (1200, 250)

    if topic in NETWORK:
        selected = {0, 1, 2, 3, 4, 5, 6, 7, 13}
        for index, at in enumerate(NODES):
            if topic in (3, 4, 5, 6, 7) and index not in (0, (1, 0, 2, 3)[topic - 4] + 1):
                continue
            point = Point(*at)
            point.name = f"n{index}"
            point.width = 12.0
            point.pointcolor = FREE if index == 0 else FIXED
            scene.add_point(point, None)
            scene.graph.set_vertex_attribute(point.guid, "spatial", 1.0)
            nodes.append(point)
            if topic in (0, 9) and index > 0:
                continue
            if topic in (3, 4, 5, 6, 7) and index != 0:
                continue
            offset = (0.3, 0.0, 0.32) if index == 0 else (0.2, -0.18, 0.22)
            label = Text.from_tex(f"n_{index}", Point(*[at[axis] + offset[axis] for axis in range(3)]), True)
            label.name = f"n{index}-label"
            label.height = 28.0
            label.color = point.pointcolor
            scene.add_text(label)

        node_by_index = {int(point.name[1:]): point for point in nodes}
        for index, (start, end) in enumerate(EDGES):
            if topic in (3, 4, 5, 6, 7) and index != (1, 0, 2, 3)[topic - 4]:
                continue
            line = Line.from_points(node_by_index[start], node_by_index[end])
            line.name = f"e{index}"
            line.width = 6.0 if topic == 3 else 5.0
            line.linecolor = BARS[index] if topic == 0 else INK
            line.arrowhead = Arrowhead.END
            scene.add_line(line, None)
            scene.graph.set_vertex_attribute(line.guid, "spatial", 1.0)
            cables.append(line)
            middle = [(NODES[start][axis] + NODES[end][axis]) * 0.5 for axis in range(3)]
            if topic == 0:
                label = Text.from_tex(f"f_{index}", Point(middle[0], middle[1], middle[2] + 0.3), True)
                label.name = f"e{index}-label"
                label.height = 24.0
                label.color = BARS[index]
                scene.add_text(label)
                force_labels.append(label)
            elif topic in (1, 2, 3, 4, 5, 6, 7):
                label = Text.from_tex(f"e_{index}", Point(middle[0], middle[1], middle[2] + 0.25), True)
                label.name = f"e{index}-label"
                label.height = 22.0
                label.color = INK
                scene.add_text(label)

        for axis in (0, 1):
            other = 1 - axis
            for value in range(-1, 9):
                start = [0.0, 0.0, 0.17]
                end = [0.0, 0.0, 0.17]
                start[axis], end[axis] = float(value), float(value)
                start[other], end[other] = -1.0, 8.0
                line = Line.from_points(Point(*start), Point(*end))
                line.name = f"grid/{axis}/{value}"
                line.width = 0.9
                line.linecolor = Color(0.32, 0.42, 0.52, 0.55)
                scene.add_line(line, None)
                scene.graph.set_vertex_attribute(line.guid, "spatial", 1.0)
                scene.graph.set_vertex_attribute(line.guid, "pickable", 0.0)

        if topic in (0, 1, 2, 13):
            end = (NODES[0][0], NODES[0][1], NODES[0][2] - 2.1)
            load = Line.from_points(node_by_index[0], Point(*end))
            load.name = "p"
            load.width = 4.0
            load.linecolor = LOAD
            load.arrowhead = Arrowhead.END
            scene.add_line(load, None)
            scene.graph.set_vertex_attribute(load.guid, "spatial", 1.0)
            label = Text.from_tex("p", Point(end[0] + 0.65, end[1] - 0.45, end[2] - 0.1), True)
            label.name = "p-label"
            label.height = 24.0
            label.color = LOAD
            scene.add_text(label)
            cables.append(load)
            force_labels.append(label)

    if topic == 0:
        vectors = []
        scale = 94 * 0.45
        for _, neighbour in EDGES:
            dx, dy, dz = [NODES[neighbour][axis] - NODES[0][axis] for axis in range(3)]
            vectors.append(((0.866025403784 * dx - 0.5 * dy) * scale, -(0.25 * dx + 0.433012701892 * dy + 0.866025403784 * dz) * scale))
        vectors.append((0.0, 0.866025403784 * 5 * scale))
        points = [(0.0, 0.0)]
        for vector in vectors:
            points.append((points[-1][0] + vector[0], points[-1][1] + vector[1]))
        xs, ys = [point[0] for point in points], [point[1] for point in points]
        origin = (1240 - (min(xs) + max(xs)) / 2, 360 - (min(ys) + max(ys)) / 2)
        points = [(origin[0] + point[0], origin[1] + point[1]) for point in points]
        for index, (start, end) in enumerate(zip(points, points[1:])):
            color = BARS[index] if index < 4 else LOAD
            line = Line.from_points(Point(*start), Point(*end))
            line.name = f"polygon-{index}"
            line.width = 5.0
            line.linecolor = color
            line.arrowhead = Arrowhead.END
            scene.add_line(line, None)
            scene.set_xform(line.guid, screen)
            scene.add_relationship(cables[index].guid, line.guid, "morph")
            scene.graph.set_edge_attribute((cables[index].guid, line.guid), "duration_ms", 1500.0)
            scene.graph.set_edge_attribute((cables[index].guid, line.guid), "delay_ms", index * 1600.0)
            dx, dy = end[0] - start[0], end[1] - start[1]
            length = sqrt(dx * dx + dy * dy)
            middle = ((start[0] + end[0]) * 0.5, (start[1] + end[1]) * 0.5)
            label = Text.from_tex(rf"\mathbf{{f}}_{index}" if index < 4 else r"\mathbf{p}", Point(middle[0] - dy / length * 28, middle[1] + dx / length * 28, 0.0))
            label.name = f"polygon-label-{index}"
            label.height = 30.0
            label.color = color
            scene.add_text(label)
            scene.add_relationship(force_labels[index].guid, label.guid, "morph")
            scene.graph.set_edge_attribute((force_labels[index].guid, label.guid), "duration_ms", 1500.0)
            scene.graph.set_edge_attribute((force_labels[index].guid, label.guid), "delay_ms", index * 1600.0)
        equation = Text.from_tex(r"\sum_{i=0}^{3}\mathbf{f}_i+\mathbf{p}=\mathbf{0}", Point(1240, 650, 0.0))
        equation.name = "equilibrium-sum"
        equation.height = 34.0
        equation.color = INK
        scene.add_text(equation)
        scene.graph.set_vertex_attribute(equation.guid, "grow", 900.0)
        scene.graph.set_vertex_attribute(equation.guid, "delay_ms", 7600.0)

    if topic == 1:
        for index, at in enumerate(NODES[1:]):
            start, end = NODES[0], at
            fraction = 0.45 if index == 0 else 0.68
            offset = (-0.65, 0, 0.15) if index == 0 else (-0.1, 0.3, 0.15)
            position = [start[axis] + fraction * (end[axis] - start[axis]) + offset[axis] for axis in range(3)]
            label = Text.from_tex(f"q_{index}=1", Point(*position), True)
            label.name = f"q-{index}"
            label.height = 22.0
            label.color = INK
            scene.add_text(label)

    if topic == 2:
        matrix = Matrix.from_rows(CONNECTION)
        matrix.name = "C"
        matrix.cell = 80.0
        matrix.mincolor = PALE
        matrix.maxcolor = PALE
        matrix.zerocolor = PALE
        matrix.textcolor = INK
        matrix.linecolor = INK
        scene.add_matrix(matrix, None)
        scene.set_xform(matrix.guid, screen * Xform.translation(*matrix_at, 0))
        scene.add_relationship(nodes[0].guid, matrix.guid, "highlight")
        for row, values in enumerate(CONNECTION):
            scene.add_relationship(cables[row].guid, matrix.guid, f"highlight:{row}")
            for col, value in enumerate(values):
                label = Text.from_tex(f"{value:g}", Point(matrix_at[0] + col * 80, matrix_at[1] + row * 80, 0.0))
                label.name = f"C[{row},{col}]"
                label.color = Color.white() if value else INK
                scene.add_text(label)
                if value:
                    scene.graph.set_vertex_attribute(matrix.guid, f"color{row * len(values) + col}", 1.0)
        title_text = Text.from_tex(r"\mathbf{C}", Point(1100, 190, 0.0))
        title_text.name = "C-title"
        title_text.height = 32.0
        title_text.color = INK
        scene.add_text(title_text)

    if topic in (3, 4, 5, 6, 7):
        edge = 1 if topic == 3 else (1, 0, 2, 3)[topic - 4]
        _, neighbour = EDGES[edge]
        a = tuple(NODES[0])
        b = tuple(NODES[neighbour])
        difference = [b[axis] - a[axis] for axis in range(3)]
        dx, dy, dz = difference
        x = (a[0] + dx, a[1], a[2])
        y = (a[0], a[1] + dy, a[2])
        z = (a[0], a[1], a[2] + dz)
        xy = (a[0] + dx, a[1] + dy, a[2])
        xz = (a[0] + dx, a[1], a[2] + dz)
        yz = (a[0], a[1] + dy, a[2] + dz)
        box = [(a, y), (a, z), (z, yz), (y, yz), (x, xy), (x, xz), (z, xz), (y, xy), (xy, b), (xz, b), (yz, b)]
        for index, (start, end) in enumerate(box):
            line = Polyline([Point(*start), Point(*end)])
            line.name = f"box-{edge}-{index}"
            line.width = 1.4
            line.linecolor = Color(INK.r, INK.g, INK.b, 0.6)
            line.dash = [0.12, 0.12]
            scene.add_polyline(line, None)
            scene.graph.set_vertex_attribute(line.guid, "spatial", 1.0)
            scene.graph.set_vertex_attribute(line.guid, "grow", 650.0)
        parts = [(a, x), (x, xy), (xy, b)]
        for index, axis in enumerate("xyz" if topic == 3 else "x"):
            start, end = parts["xyz".index(axis)]
            line = Line.from_points(Point(*start), Point(*end))
            line.name = f"box-{edge}-{axis}"
            line.width = 5.0
            line.linecolor = (X, Y, Z)[index]
            line.arrowhead = Arrowhead.END
            scene.add_line(line, None)
            scene.graph.set_vertex_attribute(line.guid, "spatial", 1.0)
            scene.graph.set_vertex_attribute(line.guid, "grow", 1200.0)
            label = Text.from_tex(f"f_{axis}", Point(*end), True)
            label.name = f"component-{axis}"
            label.height = 24.0
            label.color = (X, Y, Z)[index]
            scene.add_text(label)
        equations = []
        if topic == 3:
            for index, (axis, greek) in enumerate([("x", "alpha"), ("y", "beta"), ("z", "gamma")]):
                equations.append((f"projection-{axis}", rf"f_{axis}=f\cos\{greek}=f\dfrac{{{axis}_2-{axis}_0}}{{l}}", (1240, 180 + index * 125), (X, Y, Z)[index]))
            equations.append(("cosine", r"\cos\alpha=\dfrac{5-3}{4.535}=0.441", (1240, 570), INK))
        else:
            i = edge
            equations.append((f"fx-value-{i}", rf"f_{{x,{i}}}=f_{i}\dfrac{{x_{neighbour}-x_0}}{{l_{i}}}", (1240, 350), X))
        for order, (name, tex, at, color) in enumerate(equations):
            label = Text.from_tex(tex, Point(*at))
            label.name = name
            label.height = 30.0
            label.color = color
            scene.add_text(label)
            scene.graph.set_vertex_attribute(label.guid, "grow", 800.0)
            scene.graph.set_vertex_attribute(label.guid, "delay_ms", order * 700.0)

    if topic == 8:
        axis = Polyline([Point(-1, -2.5, 0), Point(11, -2.5, 0)])
        axis.name = "x-axis"
        axis.width = 1.5
        axis.linecolor = X
        axis.dash = [0.08, 0.12]
        scene.add_polyline(axis, None)
        scene.graph.set_vertex_attribute(axis.guid, "spatial", 1.0)
        label = Text.from_tex("x", Point(11, -2.2, 0), True)
        label.name = "x-axis-label"
        label.height = 24.0
        label.color = X
        scene.add_text(label)
        position = (3.0, -2.5, 0.0)
        for index, dx in enumerate((2, 4, -3, -3)):
            start = (position[0], position[1] + 0.65 * (index + 1), position[2])
            end = (start[0] + dx, start[1], start[2])
            line = Line.from_points(Point(*start), Point(*end))
            line.name = f"projection-{index}"
            line.width = 5.0
            line.linecolor = X
            line.arrowhead = Arrowhead.END
            scene.add_line(line, None)
            scene.graph.set_vertex_attribute(line.guid, "spatial", 1.0)
            scene.graph.set_vertex_attribute(line.guid, "grow", 1600.0)
            scene.graph.set_vertex_attribute(line.guid, "delay_ms", index * 600.0)
            label = Text.from_tex(f"{dx:+d}", Point(start[0], start[1] + 0.25, start[2] + 0.15), True)
            label.name = f"projection-label-{index}"
            label.height = 24.0
            label.color = X
            scene.add_text(label)
            position = (position[0] + dx, position[1], position[2])
        for name, tex, at in [
            ("balance-x", r"\sum_i f_i\dfrac{x_i-x_0}{l_i}+p_x=0", (1240, 280)),
            ("balance-x-values", r"2+4-3-3+0=0", (1240, 400)),
        ]:
            label = Text.from_tex(tex, Point(*at))
            label.name = name
            label.height = 32.0
            label.color = X
            scene.add_text(label)

    if topic == 9:
        values = [[round(length, 2), round(length, 2), 1] for length in LENGTHS]
        matrix = Matrix.from_rows(values)
        matrix.name = "D"
        matrix.cell = 90.0
        matrix.mincolor = PALE
        matrix.maxcolor = PALE
        matrix.zerocolor = PALE
        matrix.textcolor = INK
        matrix.linecolor = INK
        matrix_at = (1240, 140)
        scene.add_matrix(matrix, None)
        scene.set_xform(matrix.guid, screen * Xform.translation(*matrix_at, 0))
        for row in range(4):
            scene.add_relationship(cables[row].guid, matrix.guid, f"highlight:{row}")
            for col in range(3):
                value = values[row][col]
                label = Text.from_tex(f"{value:g}", Point(1240 + col * 90, 140 + row * 90, 0.0))
                label.name = f"D[{row},{col}]"
                label.color = Color.white() if col == 2 else INK
                scene.add_text(label)
                if col == 2:
                    scene.graph.set_vertex_attribute(matrix.guid, f"color{row * 3 + col}", 1.0)
        title_text = Text.from_tex(r"\mathbf{D}", Point(1100, 100, 0.0))
        title_text.name = "D-title"
        title_text.height = 32.0
        title_text.color = INK
        scene.add_text(title_text)
        cancel_source = Text.from_tex(r"\sum_i f_i\dfrac{x_i-x_0}{l_i}+p_x=0", Point(1240, 530, 0.0))
        cancel_source.name = "cancel-source"
        cancel_source.height = 30.0
        cancel_source.color = INK
        scene.add_text(cancel_source)
        cancel = Text.from_tex(r"\sum_i q_i(x_i-x_0)+p_x=0", Point(1240, 610, 0.0))
        cancel.name = "cancel"
        cancel.height = 30.0
        cancel.color = INK
        scene.add_text(cancel)
        scene.add_relationship(cancel_source.guid, cancel.guid, "morph")
        length = Text.from_tex(
            r"l_i=\sqrt{(x_i-x_0)^2+(y_i-y_0)^2+(z_i-z_0)^2}",
            Point(1240, 700, 0.0),
        )
        length.name = "length"
        length.height = 30.0
        length.color = INK
        scene.add_text(length)

    if topic in (10, 11, 12):
        q2 = 3 if topic == 11 else 1
        external = 4 if topic == 12 else 0
        axis = Polyline([Point(-1, -2.5, 0), Point(11, -2.5, 0)])
        axis.name = "line-axis"
        axis.width = 1.5
        axis.linecolor = X
        axis.dash = [0.08, 0.12]
        scene.add_polyline(axis, None)
        scene.graph.set_vertex_attribute(axis.guid, "spatial", 1.0)
        fixed_nodes = []
        for name, x in (("line-n1", 0), ("line-n2", 10)):
            point = Point(x, -2.5, 0)
            point.name = name
            point.width = 16.0
            point.pointcolor = FIXED
            scene.add_point(point, None)
            scene.graph.set_vertex_attribute(point.guid, "spatial", 1.0)
            fixed_nodes.append(point)
        free = Point(5, -2.5, 0)
        free.name = "line-n0"
        free.width = 18.0
        free.pointcolor = FREE
        scene.add_point(free, None)
        scene.graph.set_vertex_attribute(free.guid, "spatial", 1.0)
        for name, start, end in (("line-cable1", fixed_nodes[0], free), ("line-cable2", free, fixed_nodes[1])):
            line = Line.from_points(start, end)
            line.name = name
            line.width = 3.0
            line.linecolor = INK
            scene.add_line(line, None)
            scene.graph.set_vertex_attribute(line.guid, "spatial", 1.0)
            if name == "line-cable1":
                cable1 = line
            else:
                cable2 = line
        for name, end in (("line-q1", (4.2, -2.5, 0)), ("line-q2", (5.8, -2.5, 0))):
            line = Line.from_points(free, Point(*end))
            line.name = name
            line.width = 4.0
            line.linecolor = INK
            line.arrowhead = Arrowhead.END
            scene.add_line(line, None)
            scene.graph.set_vertex_attribute(line.guid, "spatial", 1.0)
            if name == "line-q1":
                force1 = line
            else:
                force2 = line
        labels = [
            ("line-n1-label", "n_1", (0, -3.2, 0), FIXED),
            ("line-n2-label", "n_2", (10, -3.2, 0), FIXED),
            ("line-n0-label", "n_0", (5, -3.2, 0), FREE),
            ("line-x1", "x_1=0", (0, -1.4, 0), INK),
            ("line-x2", "x_2=10", (10, -1.4, 0), INK),
            ("line-x0", "x_0=5", (5, -4.1, 0), FREE),
            ("line-q1-label", "q_1=1", (2.5, -1.4, 0), INK),
            ("line-q2-label", "q_2=1", (7.5, -1.4, 0), INK),
        ]
        for name, tex, position, color in labels:
            label = Text.from_tex(tex, Point(*position), True)
            label.name = name
            label.height = 24.0
            label.color = color
            scene.add_text(label)
            if name == "line-n0-label":
                free_label = label
            elif name == "line-x0":
                x0 = label
            elif name == "line-q1-label":
                q1source = label
            elif name == "line-q2-label":
                q2source = label
        if external:
            load = Line.from_points(free, Point(3.8, -2.5, 0))
            load.name = "line-p"
            load.width = 4.0
            load.linecolor = LOAD
            load.arrowhead = Arrowhead.END
            scene.add_line(load, None)
            scene.graph.set_vertex_attribute(load.guid, "spatial", 1.0)
        source = Text.from_tex(r"x_0=\dfrac{q_1x_1+q_2x_2+p}{q_1+q_2}", Point(1240, 360, 0.0))
        source.name = "weighted"
        source.height = 34.0
        source.color = INK
        scene.add_text(source)
        if topic == 10:
            solved = Text.from_tex(r"x_0=\dfrac{1\cdot0+1\cdot10+0}{1+1}=5", Point(1240, 360, 0.0))
            solved.name = "equal-solved"
            solved.height = 34.0
            solved.color = FREE
            scene.add_text(solved)
            scene.add_relationship(source.guid, solved.guid, "morph")
            scene.graph.set_edge_attribute((source.guid, solved.guid), "duration_ms", 1400.0)
        if topic == 11:
            q2solved = Text.from_tex("q_2=3", Point(8.75, -1.4, 0), True)
            q2solved.name = "line-q2-solved"
            q2solved.height = 24.0
            q2solved.color = INK
            scene.add_text(q2solved)
            x0solved = Text.from_tex("x_0=7.5", Point(7.5, -4.1, 0), True)
            x0solved.name = "line-x0-solved"
            x0solved.height = 24.0
            x0solved.color = FREE
            scene.add_text(x0solved)
            for original, target in ((x0, x0solved), (q2source, q2solved)):
                scene.add_relationship(original.guid, target.guid, "morph")
                scene.graph.set_edge_attribute((original.guid, target.guid), "duration_ms", 1200.0)
                scene.graph.set_edge_attribute((original.guid, target.guid), "delay_ms", 400.0)
            scene.animate_xform(force2.guid, Xform.scale_uniform(Point(3.75, -2.5, 0), 3), 3200, 400)
            scene.animate_xform(cable1.guid, Xform.scale_uniform(Point(0, -2.5, 0), 1.5), 3200, 400)
            scene.animate_xform(cable2.guid, Xform.scale_uniform(Point(10, -2.5, 0), 0.5), 3200, 400)
            for target in (force1, free, free_label):
                scene.animate_xform(target.guid, Xform.translation(2.5, 0, 0), 3200, 400)
            scene.animate_xform(q1source.guid, Xform.translation(1.25, 0, 0), 3200, 400)
            solved = Text.from_tex(r"x_0=\dfrac{1\cdot0+3\cdot10+0}{1+3}=7.5", Point(1240, 360, 0.0))
            solved.name = "case-1-solved"
            solved.height = 34.0
            solved.color = FREE
            scene.add_text(solved)
            scene.add_relationship(source.guid, solved.guid, "morph")
            scene.graph.set_edge_attribute((source.guid, solved.guid), "duration_ms", 1400.0)
        if topic == 12:
            for target in (free, free_label, force1, force2, load):
                scene.animate_xform(target.guid, Xform.translation(2.0, 0, 0), 3200, 400)
            for target in (q1source, q2source):
                scene.animate_xform(target.guid, Xform.translation(1.0, 0, 0), 3200, 400)
            scene.animate_xform(cable1.guid, Xform.scale_uniform(Point(0, -2.5, 0), 1.4), 3200, 400)
            scene.animate_xform(cable2.guid, Xform.scale_uniform(Point(10, -2.5, 0), 0.6), 3200, 400)
            x0solved = Text.from_tex("x_0=7", Point(7, -4.1, 0), True)
            x0solved.name = "line-x0-solved"
            x0solved.height = 24.0
            x0solved.color = FREE
            scene.add_text(x0solved)
            scene.add_relationship(x0.guid, x0solved.guid, "morph")
            scene.graph.set_edge_attribute((x0.guid, x0solved.guid), "duration_ms", 1200.0)
            scene.graph.set_edge_attribute((x0.guid, x0solved.guid), "delay_ms", 400.0)
            solved = Text.from_tex(r"x_0=\dfrac{1\cdot0+1\cdot10+4}{1+1}=7", Point(1240, 360, 0.0))
            solved.name = "case-2-solved"
            solved.height = 34.0
            solved.color = FREE
            scene.add_text(solved)
            scene.add_relationship(source.guid, solved.guid, "morph")
            scene.graph.set_edge_attribute((source.guid, solved.guid), "duration_ms", 1400.0)

    if topic in (10, 11, 12):
        balance = Text.from_tex(r"q_1(x_1-x_0)+q_2(x_2-x_0)+p=0", Point(1240, 190, 0.0))
        balance.name = "line-balance"
        balance.height = 30.0
        balance.color = INK
        scene.add_text(balance)
        values = {
            10: r"l_1=l_2=5\qquad f_1=q_1l_1=5,\quad f_2=q_2l_2=5",
            11: r"l_1=7.5,\ l_2=2.5\qquad f_1=f_2=7.5",
            12: r"l_1=7,\ l_2=3\qquad f_1=1\cdot7=7,\quad f_2=1\cdot3=3",
        }[topic]
        result = Text.from_tex(values, Point(1240, 545, 0.0))
        result.name = "line-lengths-forces"
        result.height = 28.0
        result.color = INK
        scene.add_text(result)

    if topic == 13:
        moved = Point(3, 3.7, 0.85)
        moved.name = "n0-moved"
        moved.width = 12.0
        moved.pointcolor = FREE
        scene.add_point(moved, None)
        scene.graph.set_vertex_attribute(moved.guid, "spatial", 1.0)
        scene.graph.set_vertex_attribute(moved.guid, "pulse", 1800.0)
        label = Text.from_tex("n_0'", Point(3.3, 3.7, 1.0), True)
        label.name = "n0-moved-label"
        label.height = 28.0
        label.color = FREE
        scene.add_text(label)
        matrix = Matrix.from_rows([[value] for value in LENGTHS])
        matrix.name = "L"
        matrix.cell = 90.0
        matrix.mincolor = PALE
        matrix.maxcolor = PALE
        matrix.zerocolor = PALE
        matrix.textcolor = INK
        matrix.linecolor = INK
        matrix_at = (1240, 180)
        scene.add_matrix(matrix, None)
        scene.set_xform(matrix.guid, screen * Xform.translation(*matrix_at, 0))
        scene.add_relationship(moved.guid, matrix.guid, "highlight:0")
        for index, value in enumerate(LENGTHS):
            scene.graph.set_vertex_attribute(matrix.guid, f"color{index}", 1.0)
            label = Text.from_tex(f"{value:g}", Point(1240, 180 + index * 90, 0.0))
            label.name = f"L[{index},0]"
            label.color = Color.white()
            scene.add_text(label)
        title_text = Text.from_tex(r"\mathbf{L}", Point(1140, 110, 0.0))
        title_text.name = "L-title"
        title_text.height = 32.0
        title_text.color = INK
        scene.add_text(title_text)
        label = Text.from_tex(r"\mathbf{l}=\mathbf{l}(x_0,y_0,z_0)", Point(1240, 590, 0.0))
        label.name = "length-function"
        label.height = 32.0
        label.color = INK
        scene.add_text(label)

    if topic == 14:
        push = Line.from_points(nodes[0], Point(4.1, 3, 0.25))
        push.name = "dx"
        push.width = 5.0
        push.linecolor = X
        push.arrowhead = Arrowhead.END
        scene.add_line(push, None)
        scene.graph.set_vertex_attribute(push.guid, "spatial", 1.0)
        scene.graph.set_vertex_attribute(push.guid, "grow", 1600.0)
        label = Text.from_tex(r"\mathrm{d}x", Point(4.1, 3, 0.6), True)
        label.name = "dx-label"
        label.height = 26.0
        label.color = X
        scene.add_text(label)
        for order, (name, tex, at) in enumerate([
            ("length-differential", r"\mathrm{d}l_i=\mathrm{d}x\cos\alpha_i", (1240, 260)),
            ("length-slope", r"\dfrac{\partial l_i}{\partial x_0}=\dfrac{x_0-x_i}{l_i}", (1240, 430)),
        ]):
            label = Text.from_tex(tex, Point(*at))
            label.name = name
            label.height = 34.0
            label.color = X if order == 0 else INK
            scene.add_text(label)
            scene.graph.set_vertex_attribute(label.guid, "grow", 900.0)
            scene.graph.set_vertex_attribute(label.guid, "delay_ms", order * 1100.0)

    if topic == 15:
        slopes = [(NODES[0][0] - NODES[index][0]) / LENGTHS[row] for row, index in enumerate((1, 2, 3, 4))]
        matrix = Matrix.from_rows([slopes])
        matrix.name = "Jx"
        matrix.cell = 110.0
        matrix.mincolor = PALE
        matrix.maxcolor = PALE
        matrix.zerocolor = PALE
        matrix.textcolor = INK
        matrix.linecolor = INK
        matrix_at = (1050, 280)
        scene.add_matrix(matrix, None)
        scene.set_xform(matrix.guid, screen * Xform.translation(*matrix_at, 0))
        for index, value in enumerate(slopes):
            scene.add_relationship(cables[index].guid, matrix.guid, f"highlight:0,{index}")
            scene.graph.set_vertex_attribute(matrix.guid, f"color{index}", 1.0)
            label = Text.from_tex(f"{value:.3f}", Point(1050 + index * 110, 280, 0.0))
            label.name = f"Jx[0,{index}]"
            label.color = Color.white()
            scene.add_text(label)
        for name, tex, at in [
            ("Jx-title", r"\mathbf{J}_x=\dfrac{\partial\mathbf{l}}{\partial x_0}", (1240, 150)),
            ("Jx-example", r"\dfrac{\partial l_1}{\partial x_0}=\dfrac{3-5}{4.535}=-0.441", (1240, 500)),
        ]:
            label = Text.from_tex(tex, Point(*at))
            label.name = name
            label.height = 32.0
            label.color = INK
            scene.add_text(label)

    if topic == 16:
        differences = [NODES[0][0] - NODES[index][0] for index in (1, 2, 3, 4)]
        for name, values, at in [
            ("U", [[differences[i] if i == j else 0 for j in range(4)] for i in range(4)], (900, 160)),
            ("Linv", [[1 / LENGTHS[i] if i == j else 0 for j in range(4)] for i in range(4)], (1300, 160)),
        ]:
            diagonal = Matrix.from_rows(values)
            diagonal.name = name
            diagonal.cell = 65.0
            diagonal.mincolor = PALE
            diagonal.maxcolor = PALE
            diagonal.zerocolor = PALE
            diagonal.textcolor = INK
            diagonal.linecolor = INK
            scene.add_matrix(diagonal, None)
            scene.set_xform(diagonal.guid, screen * Xform.translation(*at, 0))
            matrices.append((diagonal, at, scene))
            for row in range(4):
                scene.add_relationship(cables[row].guid, diagonal.guid, f"highlight:{row}")
                for col in range(4):
                    if values[row][col]:
                        scene.graph.set_vertex_attribute(diagonal.guid, f"color{row * 4 + col}", 1.0)
            title_text = Text.from_tex(r"\mathbf{U}" if name == "U" else r"\mathbf{L}^{-1}", Point(at[0] + 90, at[1] - 80, 0.0))
            title_text.name = f"{name}-title"
            title_text.height = 30.0
            title_text.color = INK
            scene.add_text(title_text)
        label = Text.from_tex(r"\mathbf{J}_x^{T}=\mathbf{C}_N^{T}\mathbf{U}\mathbf{L}^{-1}", Point(1240, 570, 0.0))
        label.name = "diagonal-product"
        label.height = 34.0
        label.color = INK
        scene.add_text(label)

    if topic == 17:
        for order, (name, tex, at, color) in enumerate([
            ("j-equilibrium", r"-\mathbf{J}_x\mathbf{f}+p_x=0", (1240, 180), INK),
            ("j-products", r"(-0.706)(4.25)+(0.441)(4.53)", (1240, 310), X),
            ("j-products-2", r"+(-0.526)(5.71)+(0.893)(4.48)+0=0", (1240, 390), X),
            ("j-sum", r"-3+2-3+4+0=0", (1240, 540), INK),
        ]):
            label = Text.from_tex(tex, Point(*at))
            label.name = name
            label.height = 32.0
            label.color = color
            scene.add_text(label)
            scene.graph.set_vertex_attribute(label.guid, "grow", 850.0)
            scene.graph.set_vertex_attribute(label.guid, "delay_ms", order * 900.0)

    if topic == 18:
        equation = MatrixEquation(scene, 1240, 320)
        stages = [
            ("q-start", r"-\mathbf{J}_x\mathbf{f}+p_x=0", "q-start-note", r"\text{Start with the equilibrium row from the previous slide.}"),
            ("q-jacobian", r"-\mathbf{C}_N^T\mathbf{U}\mathbf{L}^{-1}\mathbf{f}+p_x=0", "q-jacobian-note", r"\mathbf{J}_x=\mathbf{C}_N^T\mathbf{U}\mathbf{L}^{-1}\quad\text{substitute the Jacobian row.}"),
            ("q-force-density", r"-\mathbf{C}_N^T\mathbf{U}\mathbf{q}+p_x=0", "q-definition", r"\mathbf{q}=\mathbf{L}^{-1}\mathbf{f}\quad\longrightarrow\quad\mathbf{L}^{-1}\mathbf{f}=\mathbf{q}"),
            ("q-equilibrium", r"-\mathbf{C}_N^T\mathbf{Q}\mathbf{u}+p_x=0", "q-commute", r"\mathbf{U}\mathbf{q}=\mathbf{Q}\mathbf{u}\quad\text{diagonal matrices commute.}"),
        ]
        previous = None
        for order, (name, tex, note_name, note_tex) in enumerate(stages):
            label = Text.from_tex(tex, Point(1240, 290))
            label.name = name
            label.height = 34.0
            label.color = INK
            scene.add_text(label)
            note = Text.from_tex(note_tex, Point(1240, 460))
            note.name = note_name
            note.height = 28.0
            note.color = X
            scene.add_text(note)
            if previous is not None:
                equation.morph(previous[0], label, 1400.0, order * 2100.0)
                equation.morph(previous[1], note, 1100.0, order * 2100.0 + 300.0)
            previous = (label, note)
        final = Text.from_tex(r"f_i=q_i l_i\quad\longrightarrow\quad f_i/l_i=q_i\quad\text{no lengths remain.}", Point(1240, 460))
        final.name = "q-no-length"
        final.height = 28.0
        final.color = X
        scene.add_text(final)
        equation.morph(previous[1], final, 1200.0, 8800.0)

    if topic == 19:
        equation = MatrixEquation(scene, 900, 285, cell=52, gap=24)
        item = equation.matrix("Cx-C", CONNECTION, r"\mathbf{C}")
        equation.symbol(r"\times", "Cx-times")
        x = equation.matrix("Cx-x", [[NODES[index][0]] for index in range(5)], r"\mathbf{x}")
        equation.symbol("=", "Cx-equals")
        u = equation.matrix("Cx-u", [[NODES[0][0] - NODES[index][0]] for index in (1, 2, 3, 4)], r"\mathbf{u}")
        for row in range(4):
            equation.product(item, x, u, row, 0, (1240, 470 + row * 55), row * 2800)
            scene.add_relationship(cables[row].guid, item.guid, f"highlight:{row}")

    if topic == 20:
        equation = MatrixEquation(scene, 1030, 285, cell=58, gap=30)
        cn = equation.matrix("CN", [[1] for _ in range(4)], r"\mathbf{C}_N")
        equation.symbol(r"\times", "CN-times")
        xn = equation.matrix("xN", [[3]], r"x_N=x_0")
        equation.symbol("=", "CN-equals")
        product = equation.matrix("CNxN", [[3] for _ in range(4)], r"\mathbf{C}_Nx_N")
        for row in range(4):
            equation.product(cn, xn, product, row, 0, (1240, 470 + row * 55), row * 1100)

    if topic == 21:
        equation = MatrixEquation(scene, 920, 285, cell=52, gap=24)
        cf = equation.matrix("CF", [[-1 if i == j else 0 for j in range(4)] for i in range(4)], r"\mathbf{C}_F")
        equation.symbol(r"\times", "CF-times")
        xf = equation.matrix("xF", [[0], [5], [0], [7]], r"\mathbf{x}_F")
        equation.symbol("=", "CF-equals")
        product = equation.matrix("CFxF", [[0], [-5], [0], [-7]], r"\mathbf{C}_F\mathbf{x}_F")
        for row in range(4):
            equation.product(cf, xf, product, row, 0, (1240, 470 + row * 55), row * 2400)

    if topic == 22:
        equation = MatrixEquation(scene, 1030, 285, cell=58, gap=30)
        known = equation.matrix("CNxN", [[3] for _ in range(4)], r"\mathbf{C}_Nx_N")
        equation.symbol("+", "u-plus")
        fixed = equation.matrix("CFxF", [[0], [-5], [0], [-7]], r"\mathbf{C}_F\mathbf{x}_F")
        equation.symbol("=", "u-equals")
        u = equation.matrix("u", [[3], [-2], [3], [-4]], r"\mathbf{u}")
        for row in range(4):
            equation.sum(known, fixed, u, row, 0, (1240, 470 + row * 55), row * 1500)
        label = Text.from_tex(r"\mathbf{u}=\mathbf{C}_N x_0+\mathbf{C}_F\mathbf{x}_F", Point(1240, 105))
        label.name = "known-unknown"
        label.height = 32.0
        label.color = INK
        scene.add_text(label)

    if topic == 23:
        for order, (name, tex, at, color) in enumerate([
            ("linear-general", r"\mathbf{C}_N^{T}\mathbf{Q}\mathbf{C}_N x_0=p_x-\mathbf{C}_N^{T}\mathbf{Q}\mathbf{C}_F\mathbf{x}_F", (1240, 160), INK),
            ("linear-known", r"4x_0=0-(-12)=12", (1240, 370), FIXED),
            ("linear-solved", r"x_0=3", (1240, 540), FREE),
        ]):
            label = Text.from_tex(tex, Point(*at))
            label.name = name
            label.height = 32.0 if order == 0 else 38.0
            label.color = color
            scene.add_text(label)
            scene.graph.set_vertex_attribute(label.guid, "grow", 900.0)
            scene.graph.set_vertex_attribute(label.guid, "delay_ms", order * 1200.0)
        scene.graph.set_vertex_attribute(nodes[0].guid, "pulse", 2400.0)
        scene.graph.set_vertex_attribute(nodes[0].guid, "delay_ms", 2600.0)

    if topic == 24:
        for order, (name, tex, at, color) in enumerate([
            ("solve-x", r"4x_0=12\quad\longrightarrow\quad x_0=3", (1240, 180), X),
            ("solve-y", r"4y_0=12\quad\longrightarrow\quad y_0=3", (1240, 330), Y),
            ("solve-z", r"4z_0=1\quad\longrightarrow\quad z_0=0.25", (1240, 480), Z),
            ("solve-forces", r"\mathbf{f}=\mathbf{L}\mathbf{q}", (1240, 630), INK),
        ]):
            label = Text.from_tex(tex, Point(*at))
            label.name = name
            label.height = 32.0
            label.color = color
            scene.add_text(label)
            scene.graph.set_vertex_attribute(label.guid, "grow", 900.0)
            scene.graph.set_vertex_attribute(label.guid, "delay_ms", order * 1200.0)
        scene.graph.set_vertex_attribute(nodes[0].guid, "pulse", 2000.0)

    if 25 <= topic <= 35:
        size = 3 if topic <= 33 else 7
        incremental = 26 <= topic <= 33 or topic == 35
        xyz, solved, fixed, free, edges, densities, laplacian = grid_net(size, 1.2 if size == 3 else 2.6, -0.3 if size == 3 else -0.12)
        positions = solved
        large_names = {}
        if topic == 34:
            center = 3 * size + 3
            large_names[center] = "n_0"
            for a, b in edges:
                if center in (a, b):
                    large_names[b if a == center else a] = f"n_{len(large_names)}"
        net_nodes = []
        for index, at in enumerate(positions):
            point = Point(*at)
            point.name = f"net-node-{index}"
            point.guid = point.name
            point.width = 10.0 if index in fixed else 6.0
            point.pointcolor = FIXED if index in fixed else FREE
            if not incremental:
                scene.add_point(point, None)
                scene.graph.set_vertex_attribute(point.guid, "spatial", 1.0)
                scene.graph.set_vertex_attribute(point.guid, "grow", 1200.0)
                scene.graph.set_vertex_attribute(point.guid, "delay_ms", (index % size) * 90.0)
                if index == 0 and topic in (25, 34):
                    scene.graph.set_vertex_attribute(point.guid, "preserve_spatial", 1.0)
            net_nodes.append(point)
            if not incremental and (size == 3 or index in fixed or index in large_names):
                label_text = chr(65 + fixed.index(index)) if index in fixed else chr(97 + free.index(index)) if size == 3 else large_names[index]
                label = Text.from_tex(label_text, Point(at[0] + 0.12, at[1] + 0.12, at[2] + 0.1), True)
                label.name = f"net-label-{index}"
                label.height = 24.0
                label.color = point.pointcolor
                scene.add_text(label)
        net_edges = []
        for index, (a, b) in enumerate(edges):
            line = Line.from_points(net_nodes[a], net_nodes[b])
            line.name = f"net-edge-{index}"
            line.guid = line.name
            line.width = 4.5 if densities[index] == 8 else 2.5
            line.linecolor = FIXED if densities[index] == 8 else INK
            if not incremental:
                scene.add_line(line, None)
                scene.graph.set_vertex_attribute(line.guid, "spatial", 1.0)
                scene.graph.set_vertex_attribute(line.guid, "grow", 1300.0)
                scene.graph.set_vertex_attribute(line.guid, "delay_ms", index * (60 if size == 3 else 15))
            net_edges.append(line)
        if not incremental:
            for axis in (0, 1):
                other = 1 - axis
                for value in range(size):
                    start = [0.0, 0.0, 0.0]
                    end = [0.0, 0.0, 0.0]
                    start[axis], end[axis] = float(value), float(value)
                    start[other], end[other] = -0.5, size - 0.5
                    line = Line.from_points(Point(*start), Point(*end))
                    line.name = f"grid/net/{axis}/{value}"
                    line.width = 0.8
                    line.linecolor = Color(0.32, 0.42, 0.52, 0.4)
                    scene.add_line(line, None)
                    scene.graph.set_vertex_attribute(line.guid, "spatial", 1.0)
                    scene.graph.set_vertex_attribute(line.guid, "pickable", 0.0)
        rhs = []
        for node in free:
            row = []
            for axis in range(3):
                value = -0.3 if axis == 2 else 0.0
                for (a, b), q in zip(edges, densities):
                    if a == node and b in fixed:
                        value += q * xyz[b][axis]
                    elif b == node and a in fixed:
                        value += q * xyz[a][axis]
                row.append(value)
            rhs.append(row)
        coordinates = [[solved[node][axis] for axis in range(3)] for node in free]
        if topic == 25:
            equation = MatrixEquation(scene, 1050, 310, cell=52)
            matrix = equation.matrix("net-stiffness", laplacian, r"\mathbf{A}=\mathbf{C}_N^T\mathbf{Q}\mathbf{C}_N")
            for row in range(len(free)):
                scene.add_relationship(net_nodes[free[row]].guid, matrix.guid, f"highlight:{row}")
            for edge, (a, b) in zip(net_edges, edges):
                if a in free and b in free:
                    scene.add_relationship(edge.guid, matrix.guid, f"highlight:{free.index(a)},{free.index(b)}")
            for name, tex, at in [
                ("net-diagonal", r"A_{ii}=\sum_j q_{ij}\quad\text{sum of weights at node }i", (1240, 510)),
                ("net-off-diagonal", r"A_{ij}=-q_{ij}\quad\text{minus the weight of cable }ij", (1240, 580)),
                ("net-system", r"\mathbf{A}\mathbf{X}_N=\mathbf{B}", (1240, 650)),
            ]:
                label = Text.from_tex(tex, Point(*at))
                label.name = name
                label.height = 28.0
                label.color = INK
                scene.add_text(label)
        if 26 <= topic <= 30:
            row = topic - 26
            node = free[row]
            letter = chr(97 + row)
            equation = MatrixEquation(scene, 1030, 260, cell=42, gap=20)
            matrix = equation.matrix("net-stiffness", laplacian, r"\mathbf{A}")
            equation.symbol(r"\times", "node-times")
            x = equation.matrix("node-solution", [[value[0]] for value in coordinates], r"\mathbf{x}_N")
            equation.symbol("=", "node-equals")
            b = equation.matrix("node-rhs", [[value[0]] for value in rhs], r"\mathbf{b}_x")
            scene.add_relationship(net_nodes[node].guid, matrix.guid, f"highlight:{row}")
            scene.graph.set_vertex_attribute(net_nodes[node].guid, "highlight", 1.0)
            for index, (edge, (start, end)) in enumerate(zip(net_edges, edges)):
                if node in (start, end):
                    scene.graph.set_vertex_attribute(edge.guid, "highlight", 1.0)
                    middle = [(solved[start][axis] + solved[end][axis]) * 0.5 for axis in range(3)]
                    label = Text.from_tex(f"q={densities[index]}", Point(middle[0], middle[1], middle[2] + 0.22), True)
                    label.name = f"node-q-{index}"
                    label.height = 22.0
                    label.color = FIXED if densities[index] == 8 else INK
                    scene.add_text(label)
            at = solved[node]
            ground_y = at[1] - 0.45
            arrow = Line.from_points(Point(0, ground_y, 0), Point(at[0], ground_y, 0))
            arrow.name = f"node-x-arrow-{letter}"
            arrow.width = 3.0
            arrow.linecolor = X
            arrow.arrowhead = Arrowhead.END
            scene.add_line(arrow, None)
            scene.graph.set_vertex_attribute(arrow.guid, "spatial", 1.0)
            projection = Polyline([Point(at[0], ground_y, 0), Point(*at)])
            projection.name = f"node-x-projection-{letter}"
            projection.width = 1.2
            projection.linecolor = Color(X.r, X.g, X.b, 0.65)
            projection.dash = [0.08, 0.08]
            scene.add_polyline(projection, None)
            scene.graph.set_vertex_attribute(projection.guid, "spatial", 1.0)
            scene.graph.set_vertex_attribute(projection.guid, "pickable", 0.0)
            label = Text.from_tex(f"x_{letter}", Point(at[0] * 0.5, ground_y, 0.25), True)
            label.name = f"node-x-{letter}"
            label.height = 22.0
            label.color = X
            scene.add_text(label)
            formulas = [
                r"17x_a-x_c=16",
                r"17x_b-x_c=0",
                r"-x_a-x_b+4x_c-x_d-x_e=0",
                r"-x_c+17x_d=32",
                r"-x_c+17x_e=16",
            ]
            entries = [
                (f"node-general-{letter}", rf"\sum_j q_{{{letter}j}}(x_j-x_{letter})=p_{{x,{letter}}}", (1240, 435), 0.0),
                (f"node-symbolic-{letter}", formulas[row], (1240, 505), 700.0),
            ]
            for name, tex, at, delay in entries:
                label = Text.from_tex(tex, Point(*at))
                label.name = name
                label.height = 28.0
                label.color = X
                scene.add_text(label)
                scene.graph.set_vertex_attribute(label.guid, "grow", 700.0)
                scene.graph.set_vertex_attribute(label.guid, "delay_ms", delay)
            note = equation.product(matrix, x, b, row, 0, (1240, 575), 1500.0)
            note.height = 25.0 if row == 2 else 28.0
            label = Text.from_tex(rf"x_{letter}={coordinates[row][0]:.2f}", Point(1240, 660))
            label.name = f"node-solved-{letter}"
            label.height = 32.0
            label.color = FREE
            scene.add_text(label)
            scene.graph.set_vertex_attribute(label.guid, "grow", 700.0)
            scene.graph.set_vertex_attribute(label.guid, "delay_ms", 5500.0)
        if topic == 31:
            equation = MatrixEquation(scene, 990, 300, cell=58, gap=95)
            vectors = []
            for axis, name in enumerate("xyz"):
                values = [[row[axis]] for row in rhs]
                item = equation.matrix(f"net-rhs-{name}", values, rf"\mathbf{{b}}_{name}")
                vectors.append(item)
                for row in range(len(free)):
                    scene.add_relationship(net_nodes[free[row]].guid, item.guid, f"highlight:{row}")
            for axis, (name, tex) in enumerate([
                ("x", r"\mathbf{A}\mathbf{x}_N=\mathbf{b}_x"),
                ("y", r"\mathbf{A}\mathbf{y}_N=\mathbf{b}_y"),
                ("z", r"\mathbf{A}\mathbf{z}_N=\mathbf{b}_z"),
            ]):
                label = Text.from_tex(tex, Point(1240, 500 + axis * 70))
                label.name = f"net-system-{name}"
                label.height = 34.0
                label.color = (X, Y, Z)[axis]
                scene.add_text(label)
        if topic == 32:
            equation = MatrixEquation(scene, 1070, 300, cell=58)
            matrix = equation.matrix("net-solution", coordinates, r"\mathbf{X}_N")
            previous = None
            for row, node in enumerate(free):
                letter = chr(97 + row)
                scene.add_relationship(net_nodes[node].guid, matrix.guid, f"highlight:{row}")
                scene.graph.set_vertex_attribute(net_nodes[node].guid, "pulse", 900.0)
                scene.graph.set_vertex_attribute(net_nodes[node].guid, "delay_ms", row * 1100.0 + 300.0)
                values = ", ".join(f"{value:.2f}" for value in coordinates[row])
                label = Text.from_tex(rf"{letter}:\quad({values})", Point(1240, 530))
                label.name = f"coordinate-{letter}"
                label.height = 30.0
                label.color = FREE
                scene.add_text(label)
                if previous is None:
                    scene.graph.set_vertex_attribute(label.guid, "grow", 500.0)
                    scene.graph.set_vertex_attribute(label.guid, "delay_ms", row * 1100.0)
                else:
                    equation.morph(previous, label, 750.0, row * 1100.0)
                edge = (label.guid, matrix.guid)
                scene.add_relationship(*edge, f"flash:{row}")
                scene.graph.set_edge_attribute(edge, "duration_ms", 800.0)
                scene.graph.set_edge_attribute(edge, "delay_ms", row * 1100.0)
                previous = label
            label = Text.from_tex(r"\mathbf{X}_N=\mathbf{A}^{-1}\mathbf{B}\quad\text{each row places one free node.}", Point(1240, 650))
            label.name = "net-solved-system"
            label.height = 28.0
            label.color = INK
            scene.add_text(label)
        if topic == 33:
            equation = MatrixEquation(scene, 900, 275, cell=38, gap=20)
            a = equation.matrix("net-stiffness", laplacian, r"\mathbf{A}")
            equation.symbol(r"\times", "net-times")
            x = equation.matrix("net-solution", coordinates, r"\mathbf{X}_N")
            equation.symbol("=", "net-equals")
            b = equation.matrix("net-rhs", rhs, r"\mathbf{B}")
            for col in range(3):
                equation.product(a, x, b, 2, col, (1240, 490 + col * 75), col * 3300)
            for row in range(len(free)):
                for item in (a, x, b):
                    scene.add_relationship(net_nodes[free[row]].guid, item.guid, f"highlight:{row}")
            center = free[2]
            scene.graph.set_vertex_attribute(net_nodes[center].guid, "highlight", 1.0)
            for index, (edge, (start, end)) in enumerate(zip(net_edges, edges)):
                if center in (start, end):
                    scene.graph.set_vertex_attribute(edge.guid, "highlight", 1.0)
                    middle = [(solved[start][axis] + solved[end][axis]) * 0.5 for axis in range(3)]
                    label = Text.from_tex(f"q={densities[index]}", Point(middle[0], middle[1], middle[2] + 0.22), True)
                    label.name = f"verify-q-{index}"
                    label.height = 22.0
                    label.color = FIXED if densities[index] == 8 else INK
                    scene.add_text(label)
        if topic == 34:
            center = 3 * size + 3
            scene.graph.set_vertex_attribute(net_nodes[center].guid, "highlight", 1.0)
            for edge, (start, end) in zip(net_edges, edges):
                if center in (start, end):
                    scene.graph.set_vertex_attribute(edge.guid, "highlight", 1.0)
                    middle = [(solved[start][axis] + solved[end][axis]) * 0.5 for axis in range(3)]
                    label = Text.from_tex("q=1", Point(middle[0], middle[1], middle[2] + 0.22), True)
                    label.name = f"large-q-{start}-{end}"
                    label.height = 22.0
                    label.color = INK
                    scene.add_text(label)
            for order, (name, tex, at, color) in enumerate([
                ("large-local-rule", r"\text{At every interior node: four cables, each with }q=1.", (1240, 130), INK),
                ("large-x", r"4x_0-x_1-x_2-x_3-x_4=0", (1240, 245), X),
                ("large-y", r"4y_0-y_1-y_2-y_3-y_4=0", (1240, 340), Y),
                ("large-z", r"4z_0-z_1-z_2-z_3-z_4=-0.12", (1240, 435), Z),
                ("large-average", r"\mathbf{r}_0=\dfrac{\mathbf{r}_1+\mathbf{r}_2+\mathbf{r}_3+\mathbf{r}_4}{4}", (1240, 565), INK),
                ("large-z-average", r"z_0=\dfrac{z_1+z_2+z_3+z_4}{4}+0.03", (1240, 655), Z),
            ]):
                label = Text.from_tex(tex, Point(*at))
                label.name = name
                label.height = 29.0
                label.color = color
                scene.add_text(label)
                scene.graph.set_vertex_attribute(label.guid, "grow", 700.0)
                scene.graph.set_vertex_attribute(label.guid, "delay_ms", order * 700.0)
        if topic == 35:
            inner = next(index for index, (a, b) in enumerate(edges) if a == 3 * size + 3 and b == 3 * size + 4)
            boundary = next(index for index, (a, b) in enumerate(edges) if a == 1 and b == 2)
            a, b = edges[inner]
            difference = [solved[b][axis] - solved[a][axis] for axis in range(3)]
            length = sqrt(sum(value * value for value in difference))
            boundary_length = sqrt(sum((solved[edges[boundary][1]][axis] - solved[edges[boundary][0]][axis]) ** 2 for axis in range(3)))
            scene.graph.set_vertex_attribute(net_edges[inner].guid, "highlight", 1.0)
            scene.graph.set_vertex_attribute(net_edges[boundary].guid, "pulse", 1800.0)
            scene.graph.set_vertex_attribute(net_edges[boundary].guid, "delay_ms", 4100.0)
            start, end = edges[boundary]
            middle = [(solved[start][axis] + solved[end][axis]) * 0.5 for axis in range(3)]
            label = Text.from_tex("q=8", Point(middle[0], middle[1], middle[2] + 0.25), True)
            label.name = f"force-q-{boundary}"
            label.height = 23.0
            label.color = FIXED
            scene.add_text(label)
            for order, (name, tex, at) in enumerate([
                ("large-difference", rf"[u,v,w]^T=[{difference[0]:.2f},{difference[1]:.2f},{difference[2]:.2f}]^T", (1240, 145)),
                ("large-length", rf"l=\sqrt{{u^2+v^2+w^2}}={length:.2f}", (1240, 270)),
                ("large-force", rf"f=q\times l=1\times{length:.2f}={length:.2f}\ \mathrm{{kN}}", (1240, 395)),
                ("large-force-example", rf"\text{{boundary cable: }}q=8\quad f=8\times{boundary_length:.2f}={8 * boundary_length:.2f}\ \mathrm{{kN}}", (1240, 535)),
            ]):
                label = Text.from_tex(tex, Point(*at))
                label.name = name
                label.height = 32.0
                label.color = INK
                scene.add_text(label)
                source = net_edges[boundary] if name == "large-force-example" else net_edges[inner]
                scene.add_relationship(source.guid, label.guid, "highlight")
                scene.graph.set_vertex_attribute(label.guid, "grow", 800.0)
                scene.graph.set_vertex_attribute(label.guid, "delay_ms", 800.0 + order * 1100.0)

    if 26 <= topic <= 33 or topic == 35:
        deck[-1].append(scene)
    else:
        deck.append([scene])


if __name__ == "__main__":
    running = serve(deck, directory=Path(__file__).with_name("generated"))
    webbrowser.open(running.url)
    threading.Event().wait()
