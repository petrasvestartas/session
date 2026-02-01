import Rhino


def to_rhino(mesh):
    rmesh = Rhino.Geometry.Mesh()
    verts, faces = mesh.to_vertices_and_faces()
    for v in verts:
        rmesh.Vertices.Add(v[0], v[1], v[2])
    for f in faces:
        if len(f) == 3:
            rmesh.Faces.AddFace(f[0], f[1], f[2])
        elif len(f) == 4:
            rmesh.Faces.AddFace(f[0], f[1], f[2], f[3])
    rmesh.Normals.ComputeNormals()
    rmesh.Compact()
    return rmesh


def add(obj_or_list):
    if not isinstance(obj_or_list, list):
        obj_or_list = [obj_or_list]
    guids = []
    doc = Rhino.RhinoDoc.ActiveDoc
    for mesh in obj_or_list:
        rmesh = to_rhino(mesh)
        guids.append(doc.Objects.AddMesh(rmesh))
    doc.Views.Redraw()
    return guids
