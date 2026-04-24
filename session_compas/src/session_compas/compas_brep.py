from . import compas_mesh


def to_compas(brep, fallback_mesh=None):
    """Tessellate a session_py BRep to a session_py Mesh, then convert to compas Mesh.

    If tessellation yields an empty mesh (a known gap in NurbsSurfaceTrimmed.mesh()
    for some STEP-derived BReps) and `fallback_mesh` is provided, render that instead.
    """
    mesh = brep.mesh()
    if (len(mesh.vertex) == 0 or len(mesh.face) == 0) and fallback_mesh is not None:
        mesh = fallback_mesh
    return compas_mesh.to_compas(mesh)
