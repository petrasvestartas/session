// occt_trace -- runs OCCT's real boolean pipeline and dumps the complete internal
// state after every stage of BOPAlgo_PaveFiller, then the BOPAlgo_BOP result.
//
// Usage:
//   occt_trace --op cut|common|fuse --a <spec> --b <spec> [--name <id>] [--out <file>]
//
// <spec> := <type>[,<key>=<val>]*
//   sphere,r=2.5
//   cylinder,r=1,h=8,center
//   box,dx=4,dy=4,dz=4,center
//   cone,r1=2,r2=0,h=5,center
//   torus,r1=3,r2=1
//   step,file=<path>
// transform keys (applied in this order, about the global origin):
//   center (flag: shift -h/2 along Z for cylinder/cone, or center the box)
//   rotx=<deg> roty=<deg> rotz=<deg> tx= ty= tz=
//
// Record format: one line per record, "TAG key=value key=value ...".
// All floating point printed with %.9g, values below 1e-12 snapped to 0.

#include <BOPAlgo_BOP.hxx>
#include <BOPAlgo_Operation.hxx>
#include <BOPAlgo_PaveFiller.hxx>
#include <BOPDS_CommonBlock.hxx>
#include <BOPDS_Curve.hxx>
#include <BOPDS_DS.hxx>
#include <BOPDS_FaceInfo.hxx>
#include <BOPDS_IndexRange.hxx>
#include <BOPDS_Interf.hxx>
#include <BOPDS_Pave.hxx>
#include <BOPDS_PaveBlock.hxx>
#include <BOPDS_Point.hxx>
#include <BOPDS_ShapeInfo.hxx>
#include <BRepAdaptor_Curve.hxx>
#include <BRepAdaptor_Surface.hxx>
#include <BRepBuilderAPI_Transform.hxx>
#include <BRepCheck_Analyzer.hxx>
#include <BRepGProp.hxx>
#include <BRepPrimAPI_MakeBox.hxx>
#include <BRepPrimAPI_MakeCone.hxx>
#include <BRepPrimAPI_MakeCylinder.hxx>
#include <BRepPrimAPI_MakeSphere.hxx>
#include <BRepPrimAPI_MakeTorus.hxx>
#include <BRep_Tool.hxx>
#include <Bnd_Box.hxx>
#include <GCPnts_AbscissaPoint.hxx>
#include <GProp_GProps.hxx>
#include <Geom2dAdaptor_Curve.hxx>
#include <Geom2d_Curve.hxx>
#include <GeomAdaptor_Curve.hxx>
#include <Geom_Curve.hxx>
#include <IntTools_CommonPrt.hxx>
#include <IntTools_Curve.hxx>
#include <IntTools_Range.hxx>
#include <Message_ProgressRange.hxx>
#include <STEPControl_Reader.hxx>
#include <Standard_Version.hxx>
#include <TColStd_DataMapIteratorOfDataMapOfIntegerInteger.hxx>
#include <TColStd_ListOfInteger.hxx>
#include <TColStd_MapIteratorOfMapOfInteger.hxx>
#include <TopExp.hxx>
#include <TopExp_Explorer.hxx>
#include <TopTools_DataMapOfShapeListOfShape.hxx>
#include <TopTools_IndexedDataMapOfShapeListOfShape.hxx>
#include <TopTools_IndexedMapOfShape.hxx>
#include <TopTools_ListOfShape.hxx>
#include <TopoDS.hxx>
#include <TopoDS_Edge.hxx>
#include <TopoDS_Face.hxx>
#include <TopoDS_Shape.hxx>
#include <gp_Trsf.hxx>

#include <algorithm>
#include <cmath>
#include <cstdio>
#include <cstring>
#include <fstream>
#include <iostream>
#include <map>
#include <sstream>
#include <string>
#include <vector>

// ---------------------------------------------------------------- formatting

static std::string R(double v)
{
  if (!(v == v))
    return "nan";
  if (std::fabs(v) < 1e-12)
    v = 0.0;
  char b[64];
  std::snprintf(b, sizeof(b), "%.9g", v);
  return std::string(b);
}

static std::string P3(const gp_Pnt& p)
{
  return R(p.X()) + "," + R(p.Y()) + "," + R(p.Z());
}

static const char* SurfT(GeomAbs_SurfaceType t)
{
  switch (t)
  {
    case GeomAbs_Plane: return "Plane";
    case GeomAbs_Cylinder: return "Cylinder";
    case GeomAbs_Cone: return "Cone";
    case GeomAbs_Sphere: return "Sphere";
    case GeomAbs_Torus: return "Torus";
    case GeomAbs_BezierSurface: return "Bezier";
    case GeomAbs_BSplineSurface: return "BSpline";
    case GeomAbs_SurfaceOfRevolution: return "Revolution";
    case GeomAbs_SurfaceOfExtrusion: return "Extrusion";
    case GeomAbs_OffsetSurface: return "Offset";
    default: return "Other";
  }
}

static const char* CurveT(GeomAbs_CurveType t)
{
  switch (t)
  {
    case GeomAbs_Line: return "Line";
    case GeomAbs_Circle: return "Circle";
    case GeomAbs_Ellipse: return "Ellipse";
    case GeomAbs_Hyperbola: return "Hyperbola";
    case GeomAbs_Parabola: return "Parabola";
    case GeomAbs_BezierCurve: return "Bezier";
    case GeomAbs_BSplineCurve: return "BSpline";
    case GeomAbs_OffsetCurve: return "OffsetCurve";
    default: return "Other";
  }
}

static const char* ShapeT(TopAbs_ShapeEnum t)
{
  switch (t)
  {
    case TopAbs_COMPOUND: return "COMPOUND";
    case TopAbs_COMPSOLID: return "COMPSOLID";
    case TopAbs_SOLID: return "SOLID";
    case TopAbs_SHELL: return "SHELL";
    case TopAbs_FACE: return "FACE";
    case TopAbs_WIRE: return "WIRE";
    case TopAbs_EDGE: return "EDGE";
    case TopAbs_VERTEX: return "VERTEX";
    default: return "SHAPE";
  }
}

static const char* OriT(TopAbs_Orientation o)
{
  switch (o)
  {
    case TopAbs_FORWARD: return "FWD";
    case TopAbs_REVERSED: return "REV";
    case TopAbs_INTERNAL: return "INT";
    default: return "EXT";
  }
}

static std::string BoxStr(const Bnd_Box& b)
{
  if (b.IsVoid())
    return "void";
  Standard_Real x0, y0, z0, x1, y1, z1;
  b.Get(x0, y0, z0, x1, y1, z1);
  return R(x0) + "," + R(y0) + "," + R(z0) + ";" + R(x1) + "," + R(y1) + "," + R(z1);
}

static std::string IntListStr(const std::vector<int>& v)
{
  if (v.empty())
    return "-";
  std::string s;
  for (size_t i = 0; i < v.size(); ++i)
  {
    if (i)
      s += ",";
    s += std::to_string(v[i]);
  }
  return s;
}

static std::vector<int> MapToSorted(const TColStd_MapOfInteger& m)
{
  std::vector<int> v;
  for (TColStd_MapIteratorOfMapOfInteger it(m); it.More(); it.Next())
    v.push_back(it.Value());
  std::sort(v.begin(), v.end());
  return v;
}

static std::vector<int> ListToSorted(const TColStd_ListOfInteger& l)
{
  std::vector<int> v;
  for (TColStd_ListIteratorOfListOfInteger it(l); it.More(); it.Next())
    v.push_back(it.Value());
  std::sort(v.begin(), v.end());
  return v;
}

// ------------------------------------------------------------- shape parsing

struct Spec
{
  std::string                   raw;
  std::string                   type;
  std::map<std::string, double> num;
  std::map<std::string, std::string> str;
  bool                          center = false;

  bool has(const char* k) const { return num.count(k) != 0; }
  double get(const char* k, double d) const
  {
    auto it = num.find(k);
    return it == num.end() ? d : it->second;
  }
};

static Spec ParseSpec(const std::string& s)
{
  Spec sp;
  sp.raw = s;
  std::vector<std::string> parts;
  std::string              cur;
  for (char c : s)
  {
    if (c == ',')
    {
      parts.push_back(cur);
      cur.clear();
    }
    else
      cur.push_back(c);
  }
  parts.push_back(cur);
  sp.type = parts.empty() ? "" : parts[0];
  for (size_t i = 1; i < parts.size(); ++i)
  {
    const std::string& p = parts[i];
    size_t             eq = p.find('=');
    if (eq == std::string::npos)
    {
      if (p == "center")
        sp.center = true;
      continue;
    }
    std::string k = p.substr(0, eq), v = p.substr(eq + 1);
    if (k == "file")
      sp.str[k] = v;
    else
      sp.num[k] = std::atof(v.c_str());
  }
  return sp;
}

static TopoDS_Shape ReadStep(const std::string& path)
{
  STEPControl_Reader rd;
  if (rd.ReadFile(path.c_str()) != IFSelect_RetDone)
  {
    std::cerr << "occt_trace: cannot read STEP " << path << "\n";
    std::exit(2);
  }
  rd.TransferRoots();
  return rd.OneShape();
}

static TopoDS_Shape BuildShape(const Spec& sp)
{
  TopoDS_Shape sh;
  double       h = sp.get("h", 1.0);
  double       zshift = 0.0;
  if (sp.type == "sphere")
  {
    sh = BRepPrimAPI_MakeSphere(sp.get("r", 1.0)).Shape();
  }
  else if (sp.type == "cylinder")
  {
    sh = BRepPrimAPI_MakeCylinder(sp.get("r", 1.0), h).Shape();
    if (sp.center)
      zshift = -h / 2.0;
  }
  else if (sp.type == "cone")
  {
    sh = BRepPrimAPI_MakeCone(sp.get("r1", 1.0), sp.get("r2", 0.0), h).Shape();
    if (sp.center)
      zshift = -h / 2.0;
  }
  else if (sp.type == "torus")
  {
    sh = BRepPrimAPI_MakeTorus(sp.get("r1", 2.0), sp.get("r2", 0.5)).Shape();
  }
  else if (sp.type == "box")
  {
    double dx = sp.get("dx", 1.0), dy = sp.get("dy", 1.0), dz = sp.get("dz", 1.0);
    if (sp.center)
      sh = BRepPrimAPI_MakeBox(gp_Pnt(-dx / 2, -dy / 2, -dz / 2), dx, dy, dz).Shape();
    else
      sh = BRepPrimAPI_MakeBox(dx, dy, dz).Shape();
  }
  else if (sp.type == "step")
  {
    auto it = sp.str.find("file");
    sh      = ReadStep(it == sp.str.end() ? "" : it->second);
  }
  else
  {
    std::cerr << "occt_trace: unknown shape type '" << sp.type << "'\n";
    std::exit(2);
  }

  const double D2R = M_PI / 180.0;
  gp_Trsf      T;
  if (zshift != 0.0)
  {
    gp_Trsf t0;
    t0.SetTranslation(gp_Vec(0, 0, zshift));
    T = t0;
  }
  if (sp.has("rotx"))
  {
    gp_Trsf r;
    r.SetRotation(gp_Ax1(gp_Pnt(0, 0, 0), gp_Dir(1, 0, 0)), sp.get("rotx", 0) * D2R);
    T = r * T;
  }
  if (sp.has("roty"))
  {
    gp_Trsf r;
    r.SetRotation(gp_Ax1(gp_Pnt(0, 0, 0), gp_Dir(0, 1, 0)), sp.get("roty", 0) * D2R);
    T = r * T;
  }
  if (sp.has("rotz"))
  {
    gp_Trsf r;
    r.SetRotation(gp_Ax1(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1)), sp.get("rotz", 0) * D2R);
    T = r * T;
  }
  if (sp.has("tx") || sp.has("ty") || sp.has("tz"))
  {
    gp_Trsf t;
    t.SetTranslation(gp_Vec(sp.get("tx", 0), sp.get("ty", 0), sp.get("tz", 0)));
    T = t * T;
  }
  if (T.Form() != gp_Identity)
    sh = BRepBuilderAPI_Transform(sh, T, Standard_True).Shape();
  return sh;
}

// ------------------------------------------------------------- input dumping

static void DumpOperand(std::ostream& os, int argi, const Spec& sp, const TopoDS_Shape& sh)
{
  TopTools_IndexedMapOfShape mf, me, mv;
  TopExp::MapShapes(sh, TopAbs_FACE, mf);
  TopExp::MapShapes(sh, TopAbs_EDGE, me);
  TopExp::MapShapes(sh, TopAbs_VERTEX, mv);
  TopTools_IndexedMapOfShape msol, mshl;
  TopExp::MapShapes(sh, TopAbs_SOLID, msol);
  TopExp::MapShapes(sh, TopAbs_SHELL, mshl);

  GProp_GProps gv, gs;
  BRepGProp::VolumeProperties(sh, gv);
  BRepGProp::SurfaceProperties(sh, gs);
  BRepCheck_Analyzer an(sh);

  os << "ARG i=" << argi << " spec=" << sp.raw << " type=" << ShapeT(sh.ShapeType())
     << " nsolid=" << msol.Extent() << " nshell=" << mshl.Extent() << " nface=" << mf.Extent()
     << " nedge=" << me.Extent() << " nvert=" << mv.Extent() << " vol=" << R(gv.Mass())
     << " area=" << R(gs.Mass()) << " valid=" << (an.IsValid() ? 1 : 0) << "\n";

  for (int i = 1; i <= mf.Extent(); ++i)
  {
    const TopoDS_Face&   f = TopoDS::Face(mf(i));
    BRepAdaptor_Surface  as(f, Standard_True);
    Standard_Real        u0 = as.FirstUParameter(), u1 = as.LastUParameter();
    Standard_Real        v0 = as.FirstVParameter(), v1 = as.LastVParameter();
    GProp_GProps         g;
    BRepGProp::SurfaceProperties(f, g);
    os << "AFACE a=" << argi << " i=" << i << " surf=" << SurfT(as.GetType()) << " u0=" << R(u0)
       << " u1=" << R(u1) << " v0=" << R(v0) << " v1=" << R(v1)
       << " uper=" << (as.IsUPeriodic() ? 1 : 0) << " vper=" << (as.IsVPeriodic() ? 1 : 0)
       << " uclo=" << (as.IsUClosed() ? 1 : 0) << " vclo=" << (as.IsVClosed() ? 1 : 0)
       << " ori=" << OriT(f.Orientation()) << " tol=" << R(BRep_Tool::Tolerance(f))
       << " area=" << R(g.Mass()) << "\n";
    // edge usage inside this face (seam / degenerated flags need face context)
    TopTools_IndexedMapOfShape fe;
    TopExp::MapShapes(f, TopAbs_EDGE, fe);
    for (int k = 1; k <= fe.Extent(); ++k)
    {
      const TopoDS_Edge& e  = TopoDS::Edge(fe(k));
      int                ei = me.FindIndex(e);
      os << "AFEDGE a=" << argi << " f=" << i << " e=" << ei
         << " seam=" << (BRep_Tool::IsClosed(e, f) ? 1 : 0)
         << " degen=" << (BRep_Tool::Degenerated(e) ? 1 : 0) << " ori=" << OriT(e.Orientation())
         << "\n";
    }
  }

  for (int i = 1; i <= me.Extent(); ++i)
  {
    const TopoDS_Edge& e = TopoDS::Edge(me(i));
    std::string        ct = "Degenerated";
    std::string        t0 = "-", t1 = "-", len = "-";
    if (!BRep_Tool::Degenerated(e))
    {
      BRepAdaptor_Curve ac(e);
      ct  = CurveT(ac.GetType());
      t0  = R(ac.FirstParameter());
      t1  = R(ac.LastParameter());
      len = R(GCPnts_AbscissaPoint::Length(ac));
    }
    else
    {
      Standard_Real f, l;
      BRep_Tool::Range(e, f, l);
      t0 = R(f);
      t1 = R(l);
    }
    TopoDS_Vertex v1, v2;
    TopExp::Vertices(e, v1, v2);
    os << "AEDGE a=" << argi << " i=" << i << " curve=" << ct << " t0=" << t0 << " t1=" << t1
       << " len=" << len << " tol=" << R(BRep_Tool::Tolerance(e))
       << " degen=" << (BRep_Tool::Degenerated(e) ? 1 : 0)
       << " closed=" << (e.Closed() ? 1 : 0) << " v1=" << (v1.IsNull() ? -1 : mv.FindIndex(v1))
       << " v2=" << (v2.IsNull() ? -1 : mv.FindIndex(v2)) << "\n";
  }

  for (int i = 1; i <= mv.Extent(); ++i)
  {
    const TopoDS_Vertex& v = TopoDS::Vertex(mv(i));
    os << "AVERT a=" << argi << " i=" << i << " p=" << P3(BRep_Tool::Pnt(v))
       << " tol=" << R(BRep_Tool::Tolerance(v)) << "\n";
  }
}

// -------------------------------------------------------------- DS utilities

struct PBKey
{
  int         orig, e;
  double      t0, t1;
  Handle(BOPDS_PaveBlock) pb;
};

static bool PBLess(const PBKey& a, const PBKey& b)
{
  if (a.orig != b.orig)
    return a.orig < b.orig;
  if (a.t0 != b.t0)
    return a.t0 < b.t0;
  if (a.t1 != b.t1)
    return a.t1 < b.t1;
  return a.e < b.e;
}

static PBKey MakeKey(const Handle(BOPDS_PaveBlock) & pb)
{
  PBKey k;
  k.pb   = pb;
  k.orig = pb->OriginalEdge();
  k.e    = pb->HasEdge() ? pb->Edge() : -1;
  pb->Range(k.t0, k.t1);
  return k;
}

static std::string PBRef(const Handle(BOPDS_PaveBlock) & pb)
{
  PBKey k = MakeKey(pb);
  return std::to_string(k.orig) + ":" + R(k.t0) + ":" + R(k.t1);
}

// Assign a stable id to every common block, in DS pave-block-pool order.
static std::map<const BOPDS_CommonBlock*, int> CollectCB(BOPDS_DS& ds)
{
  std::map<const BOPDS_CommonBlock*, int> ids;
  int                                     next = 0;
  for (int i = 0; i < ds.NbSourceShapes(); ++i)
  {
    // NB: HasPaveBlocks()/HasFaceInfo() are both just HasReference() -- the
    // reference slot is shared between the pave-block pool and the face-info
    // pool and is discriminated only by the shape type. Always type-check.
    if (ds.ShapeInfo(i).ShapeType() != TopAbs_EDGE || !ds.HasPaveBlocks(i))
      continue;
    const BOPDS_ListOfPaveBlock& lpb = ds.PaveBlocks(i);
    for (BOPDS_ListOfPaveBlock::Iterator it(lpb); it.More(); it.Next())
    {
      if (!ds.IsCommonBlock(it.Value()))
        continue;
      const BOPDS_CommonBlock* cb = ds.CommonBlock(it.Value()).get();
      if (!ids.count(cb))
        ids[cb] = next++;
    }
  }
  return ids;
}

// ------------------------------------------------------------ full DS dumper

static void DumpDS(std::ostream& os, BOPDS_DS& ds, const char* tag)
{
  os << "DS tag=" << tag << " nbshapes=" << ds.NbShapes() << " nbsource=" << ds.NbSourceShapes()
     << " nbranges=" << ds.NbRanges() << "\n";
  for (int i = 0; i < ds.NbRanges(); ++i)
  {
    const BOPDS_IndexRange& r = ds.Range(i);
    os << "RANGE tag=" << tag << " i=" << i << " first=" << r.First() << " last=" << r.Last()
       << "\n";
  }

  // shape info
  for (int i = 0; i < ds.NbShapes(); ++i)
  {
    const BOPDS_ShapeInfo& si = ds.ShapeInfo(i);
    std::vector<int>       sub = ListToSorted(si.SubShapes());
    os << "SI tag=" << tag << " i=" << i << " type=" << ShapeT(si.ShapeType())
       << " rank=" << ds.Rank(i) << " new=" << (ds.IsNewShape(i) ? 1 : 0)
       << " ref=" << si.Reference() << " flag=" << si.Flag()
       << " brep=" << (si.HasBRep() ? 1 : 0) << " interf=" << (si.IsInterfering() ? 1 : 0)
       << " nsub=" << sub.size() << " sub=" << IntListStr(sub) << " box=" << BoxStr(si.Box())
       << "\n";
  }

  // vertices with geometry
  for (int i = 0; i < ds.NbShapes(); ++i)
  {
    const BOPDS_ShapeInfo& si = ds.ShapeInfo(i);
    if (si.ShapeType() != TopAbs_VERTEX || si.Shape().IsNull())
      continue;
    const TopoDS_Vertex& v = TopoDS::Vertex(si.Shape());
    int                  sd = -1;
    Standard_Integer     isd;
    if (ds.HasShapeSD(i, isd))
      sd = isd;
    os << "DSVERT tag=" << tag << " i=" << i << " p=" << P3(BRep_Tool::Pnt(v))
       << " tol=" << R(BRep_Tool::Tolerance(v)) << " new=" << (ds.IsNewShape(i) ? 1 : 0)
       << " sd=" << sd << "\n";
  }

  // same-domain map
  {
    std::vector<std::pair<int, int>> sd;
    for (TColStd_DataMapIteratorOfDataMapOfIntegerInteger it(ds.ShapesSD()); it.More(); it.Next())
      sd.emplace_back(it.Key(), it.Value());
    std::sort(sd.begin(), sd.end());
    for (auto& p : sd)
      os << "SD tag=" << tag << " i=" << p.first << " sd=" << p.second << "\n";
  }

  // edges: paves + pave blocks
  std::map<const BOPDS_CommonBlock*, int> cbids = CollectCB(ds);
  for (int i = 0; i < ds.NbShapes(); ++i)
  {
    if (ds.ShapeInfo(i).ShapeType() != TopAbs_EDGE)
      continue;
    if (!ds.HasPaveBlocks(i))
      continue;
    BOPDS_ListOfPave lp;
    ds.Paves(i, lp);
    int k = 0;
    for (BOPDS_ListOfPave::Iterator it(lp); it.More(); it.Next(), ++k)
      os << "PAVE tag=" << tag << " e=" << i << " k=" << k << " t=" << R(it.Value().Parameter())
         << " v=" << it.Value().Index() << "\n";

    std::vector<PBKey> keys;
    for (BOPDS_ListOfPaveBlock::Iterator it(ds.PaveBlocks(i)); it.More(); it.Next())
      keys.push_back(MakeKey(it.Value()));
    std::sort(keys.begin(), keys.end(), PBLess);
    for (size_t j = 0; j < keys.size(); ++j)
    {
      const Handle(BOPDS_PaveBlock)& pb = keys[j].pb;
      Standard_Integer               i1, i2;
      pb->Indices(i1, i2);
      bool          hascb = ds.IsCommonBlock(pb) == Standard_True;
      int           cbid  = -1;
      if (hascb)
        cbid = cbids[ds.CommonBlock(pb).get()];
      std::string sr = "-";
      if (pb->HasShrunkData())
      {
        Standard_Real    s1, s2;
        Bnd_Box          bx;
        Standard_Boolean spl;
        pb->ShrunkData(s1, s2, bx, spl);
        sr = R(s1) + ":" + R(s2);
      }
      std::string etol = "-";
      if (pb->HasEdge() && pb->Edge() < ds.NbShapes()
          && ds.ShapeInfo(pb->Edge()).ShapeType() == TopAbs_EDGE
          && !ds.Shape(pb->Edge()).IsNull())
        etol = R(BRep_Tool::Tolerance(TopoDS::Edge(ds.Shape(pb->Edge()))));
      os << "PB tag=" << tag << " e=" << i << " k=" << j << " orig=" << pb->OriginalEdge()
         << " t0=" << R(keys[j].t0) << " t1=" << R(keys[j].t1) << " v1=" << i1 << " v2=" << i2
         << " edge=" << keys[j].e << " etol=" << etol << " cb=" << cbid
         << " split=" << (pb->IsSplitEdge() ? 1 : 0)
         << " splittable=" << (pb->IsSplittable() ? 1 : 0) << " shrunk=" << sr << "\n";
    }
  }

  // common blocks
  {
    std::vector<std::pair<int, const BOPDS_CommonBlock*>> ord;
    for (auto& p : cbids)
      ord.emplace_back(p.second, p.first);
    std::sort(ord.begin(), ord.end());
    for (auto& p : ord)
    {
      const BOPDS_CommonBlock* cb = p.second;
      std::vector<PBKey>       keys;
      for (BOPDS_ListOfPaveBlock::Iterator it(cb->PaveBlocks()); it.More(); it.Next())
        keys.push_back(MakeKey(it.Value()));
      std::sort(keys.begin(), keys.end(), PBLess);
      std::string pbs;
      for (size_t j = 0; j < keys.size(); ++j)
      {
        if (j)
          pbs += "|";
        pbs += PBRef(keys[j].pb);
      }
      std::vector<int> fs = ListToSorted(cb->Faces());
      os << "CB tag=" << tag << " id=" << p.first << " tol=" << R(cb->Tolerance())
         << " edge=" << const_cast<BOPDS_CommonBlock*>(cb)->Edge() << " npb=" << keys.size()
         << " pbs=" << (pbs.empty() ? "-" : pbs) << " nfaces=" << fs.size()
         << " faces=" << IntListStr(fs) << "\n";
    }
  }

  // face info
  for (int i = 0; i < ds.NbShapes(); ++i)
  {
    if (ds.ShapeInfo(i).ShapeType() != TopAbs_FACE)
      continue;
    if (!ds.HasFaceInfo(i))
      continue;
    const BOPDS_FaceInfo& fi = ds.FaceInfo(i);
    struct SetRef
    {
      const char*                        name;
      const BOPDS_IndexedMapOfPaveBlock* pbs;
      const TColStd_MapOfInteger*        vs;
    };
    SetRef sets[3] = {{"In", &fi.PaveBlocksIn(), &fi.VerticesIn()},
                      {"On", &fi.PaveBlocksOn(), &fi.VerticesOn()},
                      {"Sc", &fi.PaveBlocksSc(), &fi.VerticesSc()}};
    os << "FI tag=" << tag << " f=" << i << " nIn=" << fi.PaveBlocksIn().Extent()
       << " nOn=" << fi.PaveBlocksOn().Extent() << " nSc=" << fi.PaveBlocksSc().Extent()
       << " vIn=" << IntListStr(MapToSorted(fi.VerticesIn()))
       << " vOn=" << IntListStr(MapToSorted(fi.VerticesOn()))
       << " vSc=" << IntListStr(MapToSorted(fi.VerticesSc())) << "\n";
    for (int s = 0; s < 3; ++s)
    {
      std::vector<PBKey> keys;
      for (int j = 1; j <= sets[s].pbs->Extent(); ++j)
        keys.push_back(MakeKey(sets[s].pbs->FindKey(j)));
      std::sort(keys.begin(), keys.end(), PBLess);
      for (size_t j = 0; j < keys.size(); ++j)
        os << "FIPB tag=" << tag << " f=" << i << " set=" << sets[s].name << " k=" << j
           << " orig=" << keys[j].orig << " t0=" << R(keys[j].t0) << " t1=" << R(keys[j].t1)
           << " edge=" << keys[j].e << "\n";
    }
  }

  // interferences
  {
    std::vector<std::string> lines;
    for (int i = 0; i < ds.InterfVV().Length(); ++i)
    {
      const BOPDS_InterfVV& x = ds.InterfVV()(i);
      Standard_Integer      a, b;
      x.Indices(a, b);
      lines.push_back("IVV tag=" + std::string(tag) + " i1=" + std::to_string(a)
                      + " i2=" + std::to_string(b) + " new=" + std::to_string(x.IndexNew()));
    }
    std::sort(lines.begin(), lines.end());
    for (auto& l : lines)
      os << l << "\n";
  }
  {
    std::vector<std::string> lines;
    for (int i = 0; i < ds.InterfVE().Length(); ++i)
    {
      const BOPDS_InterfVE& x = ds.InterfVE()(i);
      Standard_Integer      a, b;
      x.Indices(a, b);
      lines.push_back("IVE tag=" + std::string(tag) + " i1=" + std::to_string(a)
                      + " i2=" + std::to_string(b) + " t=" + R(x.Parameter())
                      + " new=" + std::to_string(x.IndexNew()));
    }
    std::sort(lines.begin(), lines.end());
    for (auto& l : lines)
      os << l << "\n";
  }
  {
    std::vector<std::string> lines;
    for (int i = 0; i < ds.InterfVF().Length(); ++i)
    {
      const BOPDS_InterfVF& x = ds.InterfVF()(i);
      Standard_Integer      a, b;
      x.Indices(a, b);
      Standard_Real u, v;
      x.UV(u, v);
      lines.push_back("IVF tag=" + std::string(tag) + " i1=" + std::to_string(a)
                      + " i2=" + std::to_string(b) + " u=" + R(u) + " v=" + R(v)
                      + " new=" + std::to_string(x.IndexNew()));
    }
    std::sort(lines.begin(), lines.end());
    for (auto& l : lines)
      os << l << "\n";
  }
  {
    std::vector<std::string> lines;
    for (int i = 0; i < ds.InterfEE().Length(); ++i)
    {
      const BOPDS_InterfEE& x = ds.InterfEE()(i);
      Standard_Integer      a, b;
      x.Indices(a, b);
      const IntTools_CommonPrt& cp = x.CommonPart();
      Standard_Real             r1f = 0, r1l = 0;
      cp.Range1(r1f, r1l);
      std::string r2;
      for (int q = 1; q <= cp.Ranges2().Length(); ++q)
      {
        if (q > 1)
          r2 += "|";
        r2 += R(cp.Ranges2()(q).First()) + ":" + R(cp.Ranges2()(q).Last());
      }
      lines.push_back("IEE tag=" + std::string(tag) + " i1=" + std::to_string(a)
                      + " i2=" + std::to_string(b) + " ctype=" + ShapeT(cp.Type()) + " r1="
                      + R(r1f) + ":" + R(r1l) + " r2=" + (r2.empty() ? "-" : r2)
                      + " new=" + std::to_string(x.IndexNew()));
    }
    std::sort(lines.begin(), lines.end());
    for (auto& l : lines)
      os << l << "\n";
  }
  {
    std::vector<std::string> lines;
    for (int i = 0; i < ds.InterfEF().Length(); ++i)
    {
      const BOPDS_InterfEF& x = ds.InterfEF()(i);
      Standard_Integer      a, b;
      x.Indices(a, b);
      const IntTools_CommonPrt& cp = x.CommonPart();
      Standard_Real             r1f = 0, r1l = 0;
      cp.Range1(r1f, r1l);
      lines.push_back("IEF tag=" + std::string(tag) + " i1=" + std::to_string(a)
                      + " i2=" + std::to_string(b) + " ctype=" + ShapeT(cp.Type()) + " r1="
                      + R(r1f) + ":" + R(r1l) + " new=" + std::to_string(x.IndexNew()));
    }
    std::sort(lines.begin(), lines.end());
    for (auto& l : lines)
      os << l << "\n";
  }

  // FF: section curves and points
  {
    struct FFRec
    {
      int         a, b;
      std::string body;
    };
    std::vector<FFRec> recs;
    for (int i = 0; i < ds.InterfFF().Length(); ++i)
    {
      const BOPDS_InterfFF& x = ds.InterfFF()(i);
      Standard_Integer      a, b;
      x.Indices(a, b);
      std::ostringstream ss;
      ss << "IFF tag=" << tag << " i1=" << a << " i2=" << b
         << " tangent=" << (x.TangentFaces() ? 1 : 0) << " ncurves=" << x.Curves().Length()
         << " npoints=" << x.Points().Length() << "\n";

      for (int c = 0; c < x.Curves().Length(); ++c)
      {
        const BOPDS_Curve&    bc = x.Curves()(c);
        const IntTools_Curve& ic = bc.Curve();
        Standard_Real         f = 0, l = 0;
        gp_Pnt                pf, pl;
        bool                  hb = ic.HasBounds() == Standard_True;
        if (hb)
          ic.Bounds(f, l, pf, pl);
        std::string len = "-";
        std::string dyn = "null";
        if (!ic.Curve().IsNull())
        {
          dyn = ic.Curve()->DynamicType()->Name();
          if (hb)
          {
            GeomAdaptor_Curve ga(ic.Curve());
            len = R(GCPnts_AbscissaPoint::Length(ga, f, l));
          }
        }
        std::vector<PBKey> keys;
        for (BOPDS_ListOfPaveBlock::Iterator it(bc.PaveBlocks()); it.More(); it.Next())
          keys.push_back(MakeKey(it.Value()));
        std::sort(keys.begin(), keys.end(), PBLess);

        ss << "SEC tag=" << tag << " f1=" << a << " f2=" << b << " c=" << c
           << " type=" << CurveT(ic.Type()) << " geom=" << dyn << " t0=" << R(f) << " t1=" << R(l)
           << " len=" << len << " tol=" << R(bc.Tolerance())
           << " tantol=" << R(ic.TangentialTolerance()) << " p0=" << (hb ? P3(pf) : "-")
           << " p1=" << (hb ? P3(pl) : "-") << " c2d1=" << (ic.FirstCurve2d().IsNull() ? 0 : 1)
           << " c2d2=" << (ic.SecondCurve2d().IsNull() ? 0 : 1) << " npb=" << keys.size()
           << " box=" << BoxStr(bc.Box()) << "\n";

        // 2D footprints on both faces: reveals seam crossing (u outside [0,2pi])
        for (int w = 0; w < 2; ++w)
        {
          Handle(Geom2d_Curve) c2 = w == 0 ? ic.FirstCurve2d() : ic.SecondCurve2d();
          if (c2.IsNull() || !hb)
            continue;
          Standard_Real umin = 1e100, umax = -1e100, vmin = 1e100, vmax = -1e100;
          const int     N = 20;
          std::string   samples;
          for (int q = 0; q <= N; ++q)
          {
            Standard_Real t = f + (l - f) * q / double(N);
            if (t < c2->FirstParameter())
              t = c2->FirstParameter();
            if (t > c2->LastParameter())
              t = c2->LastParameter();
            gp_Pnt2d p = c2->Value(t);
            umin       = std::min(umin, p.X());
            umax       = std::max(umax, p.X());
            vmin       = std::min(vmin, p.Y());
            vmax       = std::max(vmax, p.Y());
            if (q == 0 || q == N / 2 || q == N)
              samples += (samples.empty() ? "" : "|") + R(p.X()) + ":" + R(p.Y());
          }
          ss << "SEC2D tag=" << tag << " f1=" << a << " f2=" << b << " c=" << c
             << " face=" << (w == 0 ? a : b) << " umin=" << R(umin) << " umax=" << R(umax)
             << " vmin=" << R(vmin) << " vmax=" << R(vmax) << " s=" << samples << "\n";
        }

        for (size_t j = 0; j < keys.size(); ++j)
        {
          const Handle(BOPDS_PaveBlock)& pb = keys[j].pb;
          Standard_Integer               i1, i2;
          pb->Indices(i1, i2);
          std::string etol = "-";
          if (pb->HasEdge() && pb->Edge() >= 0 && pb->Edge() < ds.NbShapes()
              && !ds.Shape(pb->Edge()).IsNull()
              && ds.ShapeInfo(pb->Edge()).ShapeType() == TopAbs_EDGE)
            etol = R(BRep_Tool::Tolerance(TopoDS::Edge(ds.Shape(pb->Edge()))));
          ss << "SECPB tag=" << tag << " f1=" << a << " f2=" << b << " c=" << c << " k=" << j
             << " t0=" << R(keys[j].t0) << " t1=" << R(keys[j].t1) << " v1=" << i1 << " v2=" << i2
             << " edge=" << keys[j].e << " etol=" << etol << "\n";
        }
        std::vector<int> tv = ListToSorted(bc.TechnoVertices());
        if (!tv.empty())
          ss << "SECTV tag=" << tag << " f1=" << a << " f2=" << b << " c=" << c
             << " v=" << IntListStr(tv) << "\n";
      }
      for (int p = 0; p < x.Points().Length(); ++p)
      {
        const BOPDS_Point& bp = x.Points()(p);
        ss << "FFP tag=" << tag << " f1=" << a << " f2=" << b << " p=" << p
           << " xyz=" << P3(bp.Pnt()) << " uv1=" << R(bp.Pnt2D1().X()) << ":"
           << R(bp.Pnt2D1().Y()) << " uv2=" << R(bp.Pnt2D2().X()) << ":" << R(bp.Pnt2D2().Y())
           << " v=" << bp.Index() << "\n";
      }
      recs.push_back({a, b, ss.str()});
    }
    std::sort(recs.begin(), recs.end(), [](const FFRec& p, const FFRec& q) {
      return p.a != q.a ? p.a < q.a : p.b < q.b;
    });
    for (auto& r : recs)
      os << r.body;
  }
}

static void StageSummary(std::ostream& os, BOPDS_DS& ds, const char* stage)
{
  int npb = 0, ncb = 0, nfi = 0;
  for (int i = 0; i < ds.NbSourceShapes(); ++i)
  {
    if (ds.ShapeInfo(i).ShapeType() == TopAbs_EDGE && ds.HasPaveBlocks(i))
      npb += ds.PaveBlocks(i).Extent();
    if (ds.ShapeInfo(i).ShapeType() == TopAbs_FACE && ds.HasFaceInfo(i))
      ++nfi;
  }
  ncb          = (int)CollectCB(ds).size();
  int nffc = 0, nffp = 0, ntan = 0;
  for (int i = 0; i < ds.InterfFF().Length(); ++i)
  {
    nffc += ds.InterfFF()(i).Curves().Length();
    nffp += ds.InterfFF()(i).Points().Length();
    if (ds.InterfFF()(i).TangentFaces())
      ++ntan;
  }
  os << "STAGE name=" << stage << " nbshapes=" << ds.NbShapes()
     << " nbsource=" << ds.NbSourceShapes() << " nsd=" << ds.ShapesSD().Extent()
     << " npb=" << npb << " ncb=" << ncb << " nfaceinfo=" << nfi
     << " VV=" << ds.InterfVV().Length() << " VE=" << ds.InterfVE().Length()
     << " VF=" << ds.InterfVF().Length() << " EE=" << ds.InterfEE().Length()
     << " EF=" << ds.InterfEF().Length() << " FF=" << ds.InterfFF().Length()
     << " ffcurves=" << nffc << " ffpoints=" << nffp << " fftangent=" << ntan << "\n";
}

// ------------------------------------------------------- traced pave filler

class TracePF : public BOPAlgo_PaveFiller
{
public:
  std::ostream* os = nullptr;

  void FullDump(const char* tag) { DumpDS(*os, *myDS, tag); }

protected:
  void PerformVV(const Message_ProgressRange& r) Standard_OVERRIDE
  {
    BOPAlgo_PaveFiller::PerformVV(r);
    StageSummary(*os, *myDS, "after_VV");
  }
  void PerformVE(const Message_ProgressRange& r) Standard_OVERRIDE
  {
    BOPAlgo_PaveFiller::PerformVE(r);
    StageSummary(*os, *myDS, "after_VE");
  }
  void PerformVF(const Message_ProgressRange& r) Standard_OVERRIDE
  {
    BOPAlgo_PaveFiller::PerformVF(r);
    StageSummary(*os, *myDS, "after_VF");
  }
  void PerformEE(const Message_ProgressRange& r) Standard_OVERRIDE
  {
    BOPAlgo_PaveFiller::PerformEE(r);
    StageSummary(*os, *myDS, "after_EE");
  }
  void PerformEF(const Message_ProgressRange& r) Standard_OVERRIDE
  {
    BOPAlgo_PaveFiller::PerformEF(r);
    StageSummary(*os, *myDS, "after_EF");
  }
  void PerformFF(const Message_ProgressRange& r) Standard_OVERRIDE
  {
    BOPAlgo_PaveFiller::PerformFF(r);
    StageSummary(*os, *myDS, "after_FF");
    DumpDS(*os, *myDS, "afterFF");
  }
  void Init(const Message_ProgressRange& r) Standard_OVERRIDE
  {
    BOPAlgo_PaveFiller::Init(r);
    StageSummary(*os, *myDS, "after_Init");
  }
};

// -------------------------------------------------------------- result dump

struct ResultCounts
{
  int    nsolid = 0, nshell = 0, nface = 0, nedge = 0, nvert = 0, nnaked = 0, ndegen = 0;
  double vol = 0, area = 0;
  int    valid = 0;
};

static ResultCounts DumpResult(std::ostream&                             os,
                               const TopoDS_Shape&                       res,
                               const TopTools_DataMapOfShapeListOfShape& images,
                               const TopoDS_Shape&                       a,
                               const TopoDS_Shape&                       b)
{
  ResultCounts rc;
  if (res.IsNull())
  {
    os << "RES null=1\n";
    return rc;
  }
  TopTools_IndexedMapOfShape rf, re, rv, rsh, rso;
  TopExp::MapShapes(res, TopAbs_FACE, rf);
  TopExp::MapShapes(res, TopAbs_EDGE, re);
  TopExp::MapShapes(res, TopAbs_VERTEX, rv);
  TopExp::MapShapes(res, TopAbs_SHELL, rsh);
  TopExp::MapShapes(res, TopAbs_SOLID, rso);

  GProp_GProps gv, gs;
  BRepGProp::VolumeProperties(res, gv);
  BRepGProp::SurfaceProperties(res, gs);
  BRepCheck_Analyzer an(res);

  TopTools_IndexedDataMapOfShapeListOfShape ef;
  TopExp::MapShapesAndAncestors(res, TopAbs_EDGE, TopAbs_FACE, ef);
  int naked = 0, ndeg = 0;
  for (int i = 1; i <= re.Extent(); ++i)
  {
    const TopoDS_Edge& e = TopoDS::Edge(re(i));
    if (BRep_Tool::Degenerated(e))
    {
      ++ndeg;
      continue;
    }
    int idx = ef.FindIndex(e);
    if (idx && ef(idx).Extent() < 2)
      ++naked;
  }

  rc.nsolid = rso.Extent();
  rc.nshell = rsh.Extent();
  rc.nface  = rf.Extent();
  rc.nedge  = re.Extent();
  rc.nvert  = rv.Extent();
  rc.nnaked = naked;
  rc.ndegen = ndeg;
  rc.vol    = gv.Mass();
  rc.area   = gs.Mass();
  rc.valid  = an.IsValid() ? 1 : 0;

  os << "RES type=" << ShapeT(res.ShapeType()) << " nsolid=" << rc.nsolid
     << " nshell=" << rc.nshell << " nface=" << rc.nface << " nedge=" << rc.nedge
     << " nvert=" << rc.nvert << " naked=" << naked << " ndegen=" << ndeg << " vol=" << R(rc.vol)
     << " area=" << R(rc.area) << " valid=" << rc.valid << "\n";

  for (int i = 1; i <= rso.Extent(); ++i)
  {
    GProp_GProps g;
    BRepGProp::VolumeProperties(rso(i), g);
    TopTools_IndexedMapOfShape sf;
    TopExp::MapShapes(rso(i), TopAbs_FACE, sf);
    os << "RESSOLID i=" << i << " nface=" << sf.Extent() << " vol=" << R(g.Mass())
       << " ori=" << OriT(rso(i).Orientation()) << "\n";
  }
  for (int i = 1; i <= rsh.Extent(); ++i)
  {
    TopTools_IndexedMapOfShape sf;
    TopExp::MapShapes(rsh(i), TopAbs_FACE, sf);
    std::vector<int> fids;
    for (int k = 1; k <= sf.Extent(); ++k)
      fids.push_back(rf.FindIndex(sf(k)));
    std::sort(fids.begin(), fids.end());
    os << "RESSHELL i=" << i << " nface=" << sf.Extent()
       << " closed=" << (BRep_Tool::IsClosed(rsh(i)) ? 1 : 0) << " faces=" << IntListStr(fids)
       << "\n";
  }
  for (int i = 1; i <= rf.Extent(); ++i)
  {
    const TopoDS_Face&  f = TopoDS::Face(rf(i));
    BRepAdaptor_Surface as(f, Standard_True);
    GProp_GProps        g;
    BRepGProp::SurfaceProperties(f, g);
    TopTools_IndexedMapOfShape fw, fe;
    TopExp::MapShapes(f, TopAbs_WIRE, fw);
    TopExp::MapShapes(f, TopAbs_EDGE, fe);
    int nseam = 0;
    for (int k = 1; k <= fe.Extent(); ++k)
      if (BRep_Tool::IsClosed(TopoDS::Edge(fe(k)), f))
        ++nseam;
    os << "RESFACE i=" << i << " surf=" << SurfT(as.GetType()) << " u0=" << R(as.FirstUParameter())
       << " u1=" << R(as.LastUParameter()) << " v0=" << R(as.FirstVParameter())
       << " v1=" << R(as.LastVParameter()) << " ori=" << OriT(f.Orientation())
       << " tol=" << R(BRep_Tool::Tolerance(f)) << " area=" << R(g.Mass())
       << " nwire=" << fw.Extent() << " nedge=" << fe.Extent() << " nseam=" << nseam << "\n";
    // oriented edge occurrences: a seam edge shows up twice, once per orientation
    int w = 0;
    for (TopExp_Explorer xw(f, TopAbs_WIRE); xw.More(); xw.Next(), ++w)
    {
      for (TopExp_Explorer xe(xw.Current(), TopAbs_EDGE); xe.More(); xe.Next())
      {
        const TopoDS_Edge& e = TopoDS::Edge(xe.Current());
        Standard_Real      cf = 0, cl = 0;
        Handle(Geom2d_Curve) pc = BRep_Tool::CurveOnSurface(e, f, cf, cl);
        std::string          uv0 = "-", uv1 = "-";
        if (!pc.IsNull())
        {
          gp_Pnt2d p0 = pc->Value(cf), p1 = pc->Value(cl);
          uv0         = R(p0.X()) + ":" + R(p0.Y());
          uv1         = R(p1.X()) + ":" + R(p1.Y());
        }
        os << "RESFEDGE f=" << i << " w=" << w << " e=" << re.FindIndex(e)
           << " ori=" << OriT(e.Orientation())
           << " seam=" << (BRep_Tool::IsClosed(e, f) ? 1 : 0)
           << " degen=" << (BRep_Tool::Degenerated(e) ? 1 : 0) << " pc=" << (pc.IsNull() ? 0 : 1)
           << " uv0=" << uv0 << " uv1=" << uv1 << "\n";
      }
    }
  }
  for (int i = 1; i <= re.Extent(); ++i)
  {
    const TopoDS_Edge& e  = TopoDS::Edge(re(i));
    std::string        ct = "Degenerated", t0 = "-", t1 = "-", len = "-";
    if (!BRep_Tool::Degenerated(e))
    {
      BRepAdaptor_Curve ac(e);
      ct  = CurveT(ac.GetType());
      t0  = R(ac.FirstParameter());
      t1  = R(ac.LastParameter());
      len = R(GCPnts_AbscissaPoint::Length(ac));
    }
    int idx = ef.FindIndex(e);
    os << "RESEDGE i=" << i << " curve=" << ct << " t0=" << t0 << " t1=" << t1 << " len=" << len
       << " tol=" << R(BRep_Tool::Tolerance(e))
       << " degen=" << (BRep_Tool::Degenerated(e) ? 1 : 0)
       << " nface=" << (idx ? ef(idx).Extent() : 0) << "\n";
  }
  for (int i = 1; i <= rv.Extent(); ++i)
    os << "RESVERT i=" << i << " p=" << P3(BRep_Tool::Pnt(TopoDS::Vertex(rv(i))))
       << " tol=" << R(BRep_Tool::Tolerance(TopoDS::Vertex(rv(i)))) << "\n";

  // face images: which result faces came from which input face
  const TopoDS_Shape* args[2] = {&a, &b};
  for (int ai = 0; ai < 2; ++ai)
  {
    TopTools_IndexedMapOfShape mf;
    TopExp::MapShapes(*args[ai], TopAbs_FACE, mf);
    for (int i = 1; i <= mf.Extent(); ++i)
    {
      const TopoDS_Shape& f = mf(i);
      std::vector<int>    outs;
      int                 nimg = 0;
      if (images.IsBound(f))
      {
        const TopTools_ListOfShape& li = images.Find(f);
        nimg                           = li.Extent();
        for (TopTools_ListOfShape::Iterator it(li); it.More(); it.Next())
        {
          int k = rf.FindIndex(it.Value());
          outs.push_back(k);  // 0 == image face not present in the result
        }
      }
      else
      {
        int k = rf.FindIndex(f);
        if (k)
          outs.push_back(k);
      }
      std::sort(outs.begin(), outs.end());
      int kept = 0;
      for (int k : outs)
        if (k)
          ++kept;
      os << "IMGFACE a=" << ai << " f=" << i << " split=" << (images.IsBound(f) ? 1 : 0)
         << " nimg=" << nimg << " nkept=" << kept << " out=" << IntListStr(outs) << "\n";
    }
    TopTools_IndexedMapOfShape mev;
    TopExp::MapShapes(*args[ai], TopAbs_EDGE, mev);
    for (int i = 1; i <= mev.Extent(); ++i)
    {
      const TopoDS_Shape& e = mev(i);
      if (!images.IsBound(e))
        continue;
      const TopTools_ListOfShape& li = images.Find(e);
      std::vector<int>            outs;
      for (TopTools_ListOfShape::Iterator it(li); it.More(); it.Next())
        outs.push_back(re.FindIndex(it.Value()));
      std::sort(outs.begin(), outs.end());
      os << "IMGEDGE a=" << ai << " e=" << i << " nimg=" << li.Extent()
         << " out=" << IntListStr(outs) << "\n";
    }
  }
  return rc;
}

// --------------------------------------------------------------------- main

int main(int argc, char** argv)
{
  std::string opS = "cut", aS, bS, outS, nameS = "case";
  for (int i = 1; i < argc; ++i)
  {
    std::string k = argv[i];
    auto        nxt = [&]() -> std::string { return (i + 1 < argc) ? argv[++i] : std::string(); };
    if (k == "--op")
      opS = nxt();
    else if (k == "--a")
      aS = nxt();
    else if (k == "--b")
      bS = nxt();
    else if (k == "--out")
      outS = nxt();
    else if (k == "--name")
      nameS = nxt();
    else
    {
      std::cerr << "occt_trace: unknown option " << k << "\n";
      return 2;
    }
  }
  if (aS.empty() || bS.empty())
  {
    std::cerr << "usage: occt_trace --op cut|common|fuse --a <spec> --b <spec> [--name id] "
                 "[--out file]\n";
    return 2;
  }

  std::ofstream fout;
  if (!outS.empty())
    fout.open(outS.c_str());
  std::ostream& os = outS.empty() ? std::cout : fout;
  os.setf(std::ios::fixed, std::ios::floatfield);
  os.unsetf(std::ios::floatfield);

  BOPAlgo_Operation op = BOPAlgo_CUT;
  if (opS == "common")
    op = BOPAlgo_COMMON;
  else if (opS == "fuse")
    op = BOPAlgo_FUSE;
  else if (opS == "cut")
    op = BOPAlgo_CUT;
  else
  {
    std::cerr << "occt_trace: unknown op " << opS << "\n";
    return 2;
  }

  Spec         spa = ParseSpec(aS), spb = ParseSpec(bS);
  TopoDS_Shape sa = BuildShape(spa), sb = BuildShape(spb);

  os << "TRACE v=1 name=" << nameS << " op=" << opS << " occt=" << OCC_VERSION_STRING_EXT << "\n";
  os << "SPEC a=" << spa.raw << " b=" << spb.raw << "\n";
  DumpOperand(os, 0, spa, sa);
  DumpOperand(os, 1, spb, sb);

  TopTools_ListOfShape la, lb, lall;
  la.Append(sa);
  lb.Append(sb);
  lall.Append(sa);
  lall.Append(sb);

  TracePF pf;
  pf.os = &os;
  pf.SetArguments(lall);
  pf.SetRunParallel(Standard_False);
  pf.SetUseOBB(Standard_False);
  pf.Perform();

  os << "PF errors=" << (pf.HasErrors() ? 1 : 0) << " warnings=" << (pf.HasWarnings() ? 1 : 0)
     << "\n";
  if (pf.HasErrors())
  {
    std::ostringstream e;
    pf.DumpErrors(e);
    std::string s = e.str();
    for (auto& c : s)
      if (c == '\n')
        c = ';';
    os << "PFERR " << s << "\n";
  }
  if (pf.HasWarnings())
  {
    std::ostringstream e;
    pf.DumpWarnings(e);
    std::string s = e.str();
    for (auto& c : s)
      if (c == '\n')
        c = ';';
    os << "PFWARN " << s << "\n";
  }

  StageSummary(os, *pf.PDS(), "final");
  pf.FullDump("final");

  BOPAlgo_BOP bop;
  bop.SetArguments(la);
  bop.SetTools(lb);
  bop.SetOperation(op);
  bop.SetRunParallel(Standard_False);
  bop.SetUseOBB(Standard_False);
  bop.PerformWithFiller(pf);

  os << "BOP errors=" << (bop.HasErrors() ? 1 : 0) << " warnings=" << (bop.HasWarnings() ? 1 : 0)
     << "\n";
  if (bop.HasErrors())
  {
    std::ostringstream e;
    bop.DumpErrors(e);
    std::string s = e.str();
    for (auto& c : s)
      if (c == '\n')
        c = ';';
    os << "BOPERR " << s << "\n";
  }
  if (bop.HasWarnings())
  {
    std::ostringstream e;
    bop.DumpWarnings(e);
    std::string s = e.str();
    for (auto& c : s)
      if (c == '\n')
        c = ';';
    os << "BOPWARN " << s << "\n";
  }

  ResultCounts rc;
  if (!bop.HasErrors())
    rc = DumpResult(os, bop.Shape(), bop.Images(), sa, sb);

  // ---- summary
  BOPDS_DS& ds  = *pf.PDS();
  int       npb = 0, nfi = 0;
  for (int i = 0; i < ds.NbSourceShapes(); ++i)
  {
    if (ds.ShapeInfo(i).ShapeType() == TopAbs_EDGE && ds.HasPaveBlocks(i))
      npb += ds.PaveBlocks(i).Extent();
    if (ds.ShapeInfo(i).ShapeType() == TopAbs_FACE && ds.HasFaceInfo(i))
      ++nfi;
  }
  int ncb = (int)CollectCB(ds).size();
  int nffc = 0, nffp = 0, ntan = 0, nsecpb = 0;
  for (int i = 0; i < ds.InterfFF().Length(); ++i)
  {
    const BOPDS_InterfFF& x = ds.InterfFF()(i);
    nffc += x.Curves().Length();
    nffp += x.Points().Length();
    if (x.TangentFaces())
      ++ntan;
    for (int c = 0; c < x.Curves().Length(); ++c)
      nsecpb += x.Curves()(c).PaveBlocks().Extent();
  }
  int nnewv = 0;
  for (int i = ds.NbSourceShapes(); i < ds.NbShapes(); ++i)
    if (ds.ShapeInfo(i).ShapeType() == TopAbs_VERTEX)
      ++nnewv;

  os << "SUMMARY name=" << nameS << " op=" << opS << " dsshapes=" << ds.NbShapes()
     << " dssource=" << ds.NbSourceShapes() << " newverts=" << nnewv
     << " sd=" << ds.ShapesSD().Extent() << " pb=" << npb << " cb=" << ncb << " faceinfo=" << nfi
     << " VV=" << ds.InterfVV().Length() << " VE=" << ds.InterfVE().Length()
     << " VF=" << ds.InterfVF().Length() << " EE=" << ds.InterfEE().Length()
     << " EF=" << ds.InterfEF().Length() << " FF=" << ds.InterfFF().Length()
     << " seccurves=" << nffc << " secpoints=" << nffp << " fftangent=" << ntan
     << " secpb=" << nsecpb << " res_solid=" << rc.nsolid << " res_shell=" << rc.nshell
     << " res_face=" << rc.nface << " res_edge=" << rc.nedge << " res_vert=" << rc.nvert
     << " res_naked=" << rc.nnaked << " res_degen=" << rc.ndegen << " res_vol=" << R(rc.vol)
     << " res_area=" << R(rc.area) << " res_valid=" << rc.valid
     << " pf_err=" << (pf.HasErrors() ? 1 : 0) << " bop_err=" << (bop.HasErrors() ? 1 : 0) << "\n";
  return 0;
}
