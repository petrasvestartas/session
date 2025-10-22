from typing import List
from .mesh import Mesh
from .point import Point


def write_obj(mesh: Mesh, filepath: str):
    vertices, faces = mesh.to_vertices_and_faces()
    with open(filepath, "w") as f:
        for p in vertices:
            f.write(f"v {p.x} {p.y} {p.z}\n")
        for face in faces:
            if len(face) >= 3:
                idx = " ".join(str(i + 1) for i in face)
                f.write(f"f {idx}\n")


def read_obj(filepath: str) -> Mesh:
    verts: List[Point] = []
    faces: List[List[int]] = []

    with open(filepath, "r") as f:
        for raw in f:
            line = raw.strip()
            if not line or line.startswith("#"):
                continue
            if line.startswith("v "):
                parts = line.split()
                if len(parts) >= 4:
                    x = float(parts[1])
                    y = float(parts[2])
                    z = float(parts[3])
                    verts.append(Point(x, y, z))
            elif line.startswith("f "):
                parts = line.split()[1:]
                face: List[int] = []
                for tok in parts:
                    first = tok.split("/")[0]
                    if not first:
                        continue
                    idx = int(first)
                    if idx > 0:
                        vidx = idx - 1
                    else:
                        vidx = len(verts) + idx
                    face.append(vidx)
                if len(face) >= 3:
                    faces.append(face)

    mesh = Mesh()
    vkeys: List[int] = []
    for p in verts:
        vkeys.append(mesh.add_vertex(p))
    for f in faces:
        vlist = [vkeys[i] for i in f]
        mesh.add_face(vlist)
    return mesh


save_obj = write_obj
load_obj = read_obj
