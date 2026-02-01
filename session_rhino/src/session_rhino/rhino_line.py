import Rhino
import System


def to_rhino(ln):
    return Rhino.Geometry.Line(
        Rhino.Geometry.Point3d(ln[0], ln[1], ln[2]),
        Rhino.Geometry.Point3d(ln[3], ln[4], ln[5])
    )


def add(obj_or_list):
    if not isinstance(obj_or_list, list):
        obj_or_list = [obj_or_list]
    guids = []
    doc = Rhino.RhinoDoc.ActiveDoc
    for ln in obj_or_list:
        rln = to_rhino(ln)
        guid = doc.Objects.AddLine(rln)
        if ln.linecolor is not None:
            attr = doc.Objects.Find(guid).Attributes
            attr.ObjectColor = System.Drawing.Color.FromArgb(ln.linecolor.r, ln.linecolor.g, ln.linecolor.b)
            attr.ColorSource = Rhino.DocObjects.ObjectColorSource.ColorFromObject
            doc.Objects.ModifyAttributes(guid, attr, True)
        guids.append(guid)
    doc.Views.Redraw()
    return guids
