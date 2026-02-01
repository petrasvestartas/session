import Rhino
import System


def to_rhino(pl):
    origin = Rhino.Geometry.Point3d(pl.origin.x, pl.origin.y, pl.origin.z)
    x_axis = Rhino.Geometry.Vector3d(pl.x_axis[0], pl.x_axis[1], pl.x_axis[2])
    y_axis = Rhino.Geometry.Vector3d(pl.y_axis[0], pl.y_axis[1], pl.y_axis[2])
    return Rhino.Geometry.Plane(origin, x_axis, y_axis)


def add(obj_or_list, scale=1.0):
    if not isinstance(obj_or_list, list):
        obj_or_list = [obj_or_list]
    guids = []
    doc = Rhino.RhinoDoc.ActiveDoc
    for pl in obj_or_list:
        rpl = to_rhino(pl)
        s = scale * 0.5
        c0 = rpl.PointAt(-s, -s)
        c1 = rpl.PointAt(s, -s)
        c2 = rpl.PointAt(s, s)
        c3 = rpl.PointAt(-s, s)
        mesh = Rhino.Geometry.Mesh()
        mesh.Vertices.Add(c0)
        mesh.Vertices.Add(c1)
        mesh.Vertices.Add(c2)
        mesh.Vertices.Add(c3)
        mesh.Faces.AddFace(0, 1, 2, 3)
        mesh.Normals.ComputeNormals()
        guids.append(doc.Objects.AddMesh(mesh))

        o = rpl.Origin
        ax_len = scale * 0.5
        x_end = Rhino.Geometry.Point3d(o.X + rpl.XAxis.X * ax_len, o.Y + rpl.XAxis.Y * ax_len, o.Z + rpl.XAxis.Z * ax_len)
        y_end = Rhino.Geometry.Point3d(o.X + rpl.YAxis.X * ax_len, o.Y + rpl.YAxis.Y * ax_len, o.Z + rpl.YAxis.Z * ax_len)
        z_end = Rhino.Geometry.Point3d(o.X + rpl.ZAxis.X * ax_len, o.Y + rpl.ZAxis.Y * ax_len, o.Z + rpl.ZAxis.Z * ax_len)

        for end, color in [(x_end, System.Drawing.Color.Red), (y_end, System.Drawing.Color.Green), (z_end, System.Drawing.Color.Blue)]:
            guid = doc.Objects.AddLine(Rhino.Geometry.Line(o, end))
            attr = doc.Objects.Find(guid).Attributes
            attr.ObjectColor = color
            attr.ColorSource = Rhino.DocObjects.ObjectColorSource.ColorFromObject
            doc.Objects.ModifyAttributes(guid, attr, True)
            guids.append(guid)

        plane_guids = guids[-4:]
        doc.Groups.Add(plane_guids)

    doc.Views.Redraw()
    return guids
