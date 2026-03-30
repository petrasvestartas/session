from compas.datastructures import Mesh as CMesh


def to_compas(mesh):
    vertices = {}
    for vkey in mesh.vertices():
        pt = mesh.vertex_point(vkey)
        vertices[vkey] = [pt.x, pt.y, pt.z]
    faces = []
    for fkey in mesh.faces():
        faces.append(mesh.face_vertices(fkey))
    cmesh = CMesh()
    for vkey, xyz in vertices.items():
        cmesh.add_vertex(key=vkey, x=xyz[0], y=xyz[1], z=xyz[2])
    for face_verts in faces:
        cmesh.add_face(face_verts)
    return cmesh
