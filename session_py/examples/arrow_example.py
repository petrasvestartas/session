import sys

sys.path.insert(0, "src")

from session_py import Arrow, Line


def main():
    line = Line(0.0, 0.0, 0.0, 10.0, 0.0, 10.0)
    arrow = Arrow(line, 1.0)

    print("=== Cylinder/Pipe Generation Example ===\n")

    v_vertices, v_faces = arrow.mesh.to_vertices_and_faces()

    for vertex in v_vertices:
        print(f"{vertex.x} {vertex.y} {vertex.z}")

    print("Faces:")
    for face in v_faces:
        print(f"{face[0]} {face[1]} {face[2]}")


if __name__ == "__main__":
    main()
