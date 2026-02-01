import Rhino
import System


def to_rhino(crv):
    points = []
    for i in range(crv.cv_count()):
        cv = crv.get_cv(i)
        points.append(Rhino.Geometry.Point3d(float(cv.x), float(cv.y), float(cv.z)))

    return Rhino.Geometry.Curve.CreateControlPointCurve(points, crv.degree())


def add(obj_or_list, **kwargs):
    if not isinstance(obj_or_list, list):
        obj_or_list = [obj_or_list]
    guids = []
    doc = Rhino.RhinoDoc.ActiveDoc
    for crv in obj_or_list:
        rcrv = to_rhino(crv)
        guid = doc.Objects.AddCurve(rcrv)
        if crv.linecolor is not None:
            attr = doc.Objects.Find(guid).Attributes
            attr.ObjectColor = System.Drawing.Color.FromArgb(crv.linecolor.r, crv.linecolor.g, crv.linecolor.b)
            attr.ColorSource = Rhino.DocObjects.ObjectColorSource.ColorFromObject
            doc.Objects.ModifyAttributes(guid, attr, True)
        guids.append(guid)
    doc.Views.Redraw()
    return guids
