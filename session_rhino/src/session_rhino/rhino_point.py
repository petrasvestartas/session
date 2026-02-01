import Rhino
import System


def to_rhino(pt):
    return Rhino.Geometry.Point3d(pt.x, pt.y, pt.z)


def add(obj_or_list):
    if not isinstance(obj_or_list, list):
        obj_or_list = [obj_or_list]
    guids = []
    doc = Rhino.RhinoDoc.ActiveDoc
    for pt in obj_or_list:
        rpt = to_rhino(pt)
        guid = doc.Objects.AddPoint(rpt)
        if pt.pointcolor is not None:
            attr = doc.Objects.Find(guid).Attributes
            attr.ObjectColor = System.Drawing.Color.FromArgb(pt.pointcolor.r, pt.pointcolor.g, pt.pointcolor.b)
            attr.ColorSource = Rhino.DocObjects.ObjectColorSource.ColorFromObject
            doc.Objects.ModifyAttributes(guid, attr, True)
        guids.append(guid)
    doc.Views.Redraw()
    return guids
