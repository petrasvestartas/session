// OCCT ground-truth oracle. Reads a tiny whitespace request from argv[1],
// writes a whitespace result to argv[2]. Geometry is built from primitive specs
// (parameterization-independent) so intersection results can be compared as 3D
// point sets against our kernel. Test-time only.
//
// Request grammar (one token stream):
//   OP <ssi|interpolate>
//   For ssi:
//     SURF <kind> <params...>   (twice)
//       cylinder r h  | sphere r | cone r1 r2 h | torus rmaj rmin | plane (z=0)
//     XF tx ty tz  ax ay az  deg     (optional, after each SURF; rotation about
//                                     axis (ax,ay,az) through origin by deg, then translate)
//     TOL t
//   For interpolate:
//     NPTS n
//     PT x y z            (n lines)
//     PERIODIC <0|1>
//
// Result grammar:
//   ssi:        NCURVES k   then per curve: CURVE m  then m lines "x y z"
//   interpolate:OK  DEG d  NPOLES p  then p lines "x y z w"  KNOTS kc then kc "v m"
//               SAMPLES s then s lines "x y z"
//   errors:     ERROR <message>

#include <BRepBuilderAPI_MakeFace.hxx>
#include <BRepBuilderAPI_Transform.hxx>
#include <BRepAlgoAPI_Section.hxx>
#include <BRep_Tool.hxx>
#include <TopoDS.hxx>
#include <TopoDS_Edge.hxx>
#include <TopoDS_Shape.hxx>
#include <TopExp_Explorer.hxx>
#include <TopTools_HSequenceOfShape.hxx>
#include <ShapeAnalysis_FreeBounds.hxx>
#include <BRepAdaptor_Curve.hxx>
#include <GCPnts_AbscissaPoint.hxx>
#include <Geom_CylindricalSurface.hxx>
#include <Geom_SphericalSurface.hxx>
#include <Geom_ConicalSurface.hxx>
#include <Geom_ToroidalSurface.hxx>
#include <Geom_Plane.hxx>
#include <Geom_Surface.hxx>
#include <Geom_Curve.hxx>
#include <Geom_BSplineCurve.hxx>
#include <Geom_BSplineSurface.hxx>
#include <TColgp_Array2OfPnt.hxx>
#include <TColStd_Array2OfReal.hxx>
#include <GeomAPI_Interpolate.hxx>
#include <GeomAdaptor_Curve.hxx>
#include <GCPnts_UniformAbscissa.hxx>
#include <GCPnts_AbscissaPoint.hxx>
#include <GeomAPI_ProjectPointOnCurve.hxx>
#include <GeomAPI_ExtremaCurveCurve.hxx>
#include <GeomAPI_IntCS.hxx>
#include <Geom_Line.hxx>
#include <gp_Lin.hxx>
#include <BRepPrimAPI_MakeBox.hxx>
#include <BRepPrimAPI_MakeCylinder.hxx>
#include <BRepPrimAPI_MakeSphere.hxx>
#include <BRepPrimAPI_MakeCone.hxx>
#include <BRepPrimAPI_MakeTorus.hxx>
#include <BRepAlgoAPI_Fuse.hxx>
#include <BRepAlgoAPI_Cut.hxx>
#include <BRepAlgoAPI_Common.hxx>
#include <GProp_GProps.hxx>
#include <BRepGProp.hxx>
#include <gp_Ax2.hxx>
#include <TopoDS_Solid.hxx>
#include <BRepClass3d_SolidClassifier.hxx>
#include <BRepMesh_IncrementalMesh.hxx>
#include <Poly_Triangulation.hxx>
#include <TopLoc_Location.hxx>
#include <TopoDS_Face.hxx>
#include <gp_Ax3.hxx>
#include <gp_Pnt.hxx>
#include <gp_Dir.hxx>
#include <gp_Trsf.hxx>
#include <gp_Ax1.hxx>
#include <BRepBuilderAPI_MakeShell.hxx>
#include <BRepBuilderAPI_MakeSolid.hxx>
#include <ShapeFix_Solid.hxx>
#include <TopoDS_Shell.hxx>
#include <TColStd_Array1OfReal.hxx>
#include <TColStd_Array1OfInteger.hxx>
#include <TColgp_HArray1OfPnt.hxx>
#include <TColgp_Array1OfPnt.hxx>
#include <TColStd_Array1OfReal.hxx>
#include <TColStd_Array1OfInteger.hxx>

#include <fstream>
#include <iostream>
#include <iomanip>
#include <chrono>
#include <string>
#include <vector>
#include <cmath>

static const double PI = 3.14159265358979323846;

struct Spec { std::string kind; std::vector<double> p; bool hasXf=false; double tx=0,ty=0,tz=0,ax=0,ay=0,az=1,deg=0; };

static gp_Trsf make_trsf(const Spec& s) {
    gp_Trsf rot, tr;
    if (s.deg != 0.0) {
        gp_Ax1 axis(gp_Pnt(0,0,0), gp_Dir(s.ax, s.ay, s.az));
        rot.SetRotation(axis, s.deg * PI / 180.0);
    }
    tr.SetTranslation(gp_Vec(s.tx, s.ty, s.tz));
    return tr * rot;
}

// Build a BOUNDED face matching our kernel's finite surface extents, so the
// section is comparable to our finite-surface SSI.
static TopoDS_Face build_face(const Spec& s) {
    gp_Ax3 ax(gp_Pnt(0,0,0), gp_Dir(0,0,1));
    const double TAU = 2.0 * PI;
    const double tol = 1e-7;
    TopoDS_Face face;
    if (s.kind == "cylinder") {
        Handle(Geom_CylindricalSurface) surf = new Geom_CylindricalSurface(ax, s.p[0]);
        double h = s.p[1];
        face = BRepBuilderAPI_MakeFace(surf, 0.0, TAU, 0.0, h, tol).Face();
    } else if (s.kind == "sphere") {
        Handle(Geom_SphericalSurface) surf = new Geom_SphericalSurface(ax, s.p[0]);
        face = BRepBuilderAPI_MakeFace(surf, 0.0, TAU, -PI/2.0, PI/2.0, tol).Face();
    } else if (s.kind == "cone") {
        double r1 = s.p[0], r2 = s.p[1], h = s.p[2];
        double halfang = std::atan2(r2 - r1, h);
        Handle(Geom_ConicalSurface) surf = new Geom_ConicalSurface(ax, halfang, r1);
        double vlen = h / std::cos(halfang);
        face = BRepBuilderAPI_MakeFace(surf, 0.0, TAU, 0.0, vlen, tol).Face();
    } else if (s.kind == "torus") {
        Handle(Geom_ToroidalSurface) surf = new Geom_ToroidalSurface(ax, s.p[0], s.p[1]);
        face = BRepBuilderAPI_MakeFace(surf, 0.0, TAU, 0.0, TAU, tol).Face();
    } else if (s.kind == "plane") {
        Handle(Geom_Plane) surf = new Geom_Plane(ax);
        double r = (s.p.empty() ? 100.0 : s.p[0]);
        face = BRepBuilderAPI_MakeFace(surf, -r, r, -r, r, tol).Face();
    }
    if (!face.IsNull() && s.hasXf) {
        BRepBuilderAPI_Transform xf(face, make_trsf(s), Standard_True);
        face = TopoDS::Face(xf.Shape());
    }
    return face;
}

// Parse a curve block: DEG d / NPOLES n / POLE x y z w (n) / NKNOTS k / KNOT v m (k).
// Returns a rational Geom_BSplineCurve (non-periodic).
static Handle(Geom_BSplineCurve) read_curve(std::istream& in) {
    std::string kw; int degree = 0, np = 0, nk = 0;
    in >> kw >> degree;  // DEG d
    in >> kw >> np;      // NPOLES n
    TColgp_Array1OfPnt poles(1, np);
    TColStd_Array1OfReal wts(1, np);
    for (int i = 1; i <= np; i++) {
        double x, y, z, w; in >> kw >> x >> y >> z >> w;
        poles.SetValue(i, gp_Pnt(x, y, z));
        wts.SetValue(i, w);
    }
    in >> kw >> nk;      // NKNOTS k
    TColStd_Array1OfReal knots(1, nk);
    TColStd_Array1OfInteger mults(1, nk);
    for (int i = 1; i <= nk; i++) {
        double v; int m; in >> kw >> v >> m;
        knots.SetValue(i, v);
        mults.SetValue(i, m);
    }
    return new Geom_BSplineCurve(poles, wts, knots, mults, degree, Standard_False);
}

static void write_curve(std::ostream& out, const Handle(Geom_BSplineCurve)& c) {
    out << "DEG " << c->Degree() << "\n";
    int npo = c->NbPoles();
    out << "NPOLES " << npo << "\n";
    for (int i = 1; i <= npo; i++) {
        gp_Pnt p = c->Pole(i); double w = c->Weight(i);
        out << p.X() << " " << p.Y() << " " << p.Z() << " " << w << "\n";
    }
    int nkn = c->NbKnots();
    out << "KNOTS " << nkn << "\n";
    for (int i = 1; i <= nkn; i++)
        out << c->Knot(i) << " " << c->Multiplicity(i) << "\n";
    double t0 = c->FirstParameter(), t1 = c->LastParameter();
    const int S = 64;
    out << "SAMPLES " << (S + 1) << "\n";
    for (int k = 0; k <= S; k++) {
        double t = t0 + (t1 - t0) * (double)k / S;
        gp_Pnt p = c->Value(t);
        out << p.X() << " " << p.Y() << " " << p.Z() << "\n";
    }
}

// Build a primitive solid matching the session's conventions:
//   box r sx sy sz   -> dims sx x sy x sz centered at origin
//   cylinder r h     -> radius r, height h, base at z=0 along +Z
//   sphere r         -> radius r centered at origin
static TopoDS_Shape build_solid(const std::string& kind, const std::vector<double>& p) {
    if (kind == "box") {
        gp_Pnt corner(-p[0]/2, -p[1]/2, -p[2]/2);
        return BRepPrimAPI_MakeBox(corner, p[0], p[1], p[2]).Solid();
    } else if (kind == "cylinder") {
        return BRepPrimAPI_MakeCylinder(p[0], p[1]).Solid();
    } else if (kind == "sphere") {
        return BRepPrimAPI_MakeSphere(p[0]).Solid();
    } else if (kind == "cone") {
        // session create_cone(radius, height): base r at z=0, apex at z=height -> R2=0.
        return BRepPrimAPI_MakeCone(p[0], 0.0, p[1]).Solid();
    } else if (kind == "torus") {
        // session create_torus(major, minor), centered at origin, axis +Z.
        return BRepPrimAPI_MakeTorus(p[0], p[1]).Solid();
    }
    return TopoDS_Shape();
}

// SHAPE nurbs <rat> <deg_u> <deg_v> <ncu> <ncv>
//   (ncu+deg_u-1) u-knots, then (ncv+deg_v-1) v-knots  (ON convention: cv+order-2),
//   then ncu*ncv CV lines "x y z w" (euclidean position + weight, u-major).
// Builds the session kernel's freeform surface VERBATIM as a Geom_BSplineSurface, then a
// closed solid (MakeShell + MakeSolid + ShapeFix) -- the reference for freeform booleans
// that no primitive spec can express.
static TopoDS_Shape build_nurbs_solid(std::istream& in) {
    int rat, du, dv, ncu, ncv;
    in >> rat >> du >> dv >> ncu >> ncv;
    int nku = ncu + du - 1, nkv = ncv + dv - 1;
    std::vector<double> ku(nku), kv(nkv);
    for (auto& k : ku) in >> k;
    for (auto& k : kv) in >> k;
    TColgp_Array2OfPnt poles(1, ncu, 1, ncv);
    TColStd_Array2OfReal weights(1, ncu, 1, ncv);
    for (int i = 1; i <= ncu; ++i)
        for (int j = 1; j <= ncv; ++j) {
            double x, y, z, w; in >> x >> y >> z >> w;
            poles(i, j) = gp_Pnt(x, y, z);
            weights(i, j) = w;
        }
    // ON knots (cv+order-2, clamped end multiplicity = degree) -> full clamped vector
    // (duplicate first/last) -> OCCT distinct knots + multiplicities.
    auto to_occ = [](const std::vector<double>& on, std::vector<double>& dk, std::vector<int>& dm) {
        std::vector<double> full;
        full.push_back(on.front());
        full.insert(full.end(), on.begin(), on.end());
        full.push_back(on.back());
        for (double k : full) {
            if (dk.empty() || k - dk.back() > 1e-12) { dk.push_back(k); dm.push_back(1); }
            else dm.back()++;
        }
    };
    std::vector<double> uk, vk; std::vector<int> um, vm;
    to_occ(ku, uk, um); to_occ(kv, vk, vm);
    TColStd_Array1OfReal UK(1, (int)uk.size()), VK(1, (int)vk.size());
    TColStd_Array1OfInteger UM(1, (int)um.size()), VM(1, (int)vm.size());
    for (int i = 0; i < (int)uk.size(); ++i) { UK(i+1) = uk[i]; UM(i+1) = um[i]; }
    for (int i = 0; i < (int)vk.size(); ++i) { VK(i+1) = vk[i]; VM(i+1) = vm[i]; }
    try {
        Handle(Geom_BSplineSurface) surf = new Geom_BSplineSurface(
            poles, weights, UK, VK, UM, VM, du, dv, Standard_False, Standard_False);
        TopoDS_Shell shell = BRepBuilderAPI_MakeShell(surf).Shell();
        TopoDS_Solid solid = BRepBuilderAPI_MakeSolid(shell).Solid();
        ShapeFix_Solid fix(solid);
        fix.Perform();
        return fix.Solid();
    } catch (...) { return TopoDS_Shape(); }
}

static void solid_props(std::ostream& out, const TopoDS_Shape& s) {
    // Adaptive Gauss with Eps=1e-9: the default fixed-order overload integrates OCCT's own
    // APPROXIMATED section pcurves and is only ~2e-6 accurate on curved-section solids (box x
    // torus common: 15.4811080 vs true 15.4810801, +1.8e-6) -- too coarse to referee a 1e-6
    // volume gate. The adaptive overload refines until each face's relative error <= Eps.
    GProp_GProps vp; BRepGProp::VolumeProperties(s, vp, 1e-9);
    GProp_GProps sp; BRepGProp::SurfaceProperties(s, sp);
    int nf = 0;
    for (TopExp_Explorer ex(s, TopAbs_FACE); ex.More(); ex.Next()) nf++;
    gp_Pnt c = vp.CentreOfMass();
    out << "VOLUME " << vp.Mass() << "\n";
    out << "AREA " << sp.Mass() << "\n";
    out << "NFACES " << nf << "\n";
    out << "CENTROID " << c.X() << " " << c.Y() << " " << c.Z() << "\n";
}

// Read "SHAPE <kind> <params...> XF tx ty tz ax ay az deg" (XF mandatory; identity = all 0
// except az=1). Returns the (optionally transformed) solid.
static TopoDS_Shape read_solid_shape(std::istream& in) {
    std::string kw, kind; in >> kw >> kind;  // SHAPE kind
    std::vector<double> p;
    if (kind == "box") { p.resize(3); in >> p[0] >> p[1] >> p[2]; }
    else if (kind == "cylinder") { p.resize(2); in >> p[0] >> p[1]; }
    else if (kind == "sphere") { p.resize(1); in >> p[0]; }
    else if (kind == "cone") { p.resize(2); in >> p[0] >> p[1]; }
    else if (kind == "torus") { p.resize(2); in >> p[0] >> p[1]; }
    TopoDS_Shape shp = (kind == "nurbs") ? build_nurbs_solid(in) : build_solid(kind, p);
    std::string xfkw; Spec sp; sp.hasXf = true;
    in >> xfkw >> sp.tx >> sp.ty >> sp.tz >> sp.ax >> sp.ay >> sp.az >> sp.deg;  // XF ...
    if (!shp.IsNull() && (sp.tx || sp.ty || sp.tz || sp.deg)) {
        BRepBuilderAPI_Transform xf(shp, make_trsf(sp), Standard_True);
        shp = xf.Shape();
    }
    return shp;
}

// Tessellate a shape and emit all triangulation node positions as boundary samples,
// for a representation-independent boundary Hausdorff comparison.
static void write_boundary_samples(std::ostream& out, const TopoDS_Shape& s, double defl) {
    BRepMesh_IncrementalMesh mesher(s, defl, Standard_False, 0.5, Standard_True);
    std::vector<gp_Pnt> pts;
    for (TopExp_Explorer ex(s, TopAbs_FACE); ex.More(); ex.Next()) {
        TopoDS_Face f = TopoDS::Face(ex.Current());
        TopLoc_Location loc;
        Handle(Poly_Triangulation) tri = BRep_Tool::Triangulation(f, loc);
        if (tri.IsNull()) continue;
        const gp_Trsf& trsf = loc.Transformation();
        for (int i = 1; i <= tri->NbNodes(); i++) {
            gp_Pnt p = tri->Node(i).Transformed(trsf);
            pts.push_back(p);
        }
    }
    out << "SAMPLES " << pts.size() << "\n";
    for (const auto& p : pts) out << p.X() << " " << p.Y() << " " << p.Z() << "\n";
}

static TopoDS_Shape run_boolean(const std::string& mode, const TopoDS_Shape& a, const TopoDS_Shape& b) {
    if (mode == "fuse") return BRepAlgoAPI_Fuse(a, b).Shape();
    if (mode == "cut") return BRepAlgoAPI_Cut(a, b).Shape();
    if (mode == "common") return BRepAlgoAPI_Common(a, b).Shape();
    return TopoDS_Shape();
}

// Parse a surface block: DEGU du DEGV dv / NU nu NV nv / POLE i j x y z w (nu*nv) /
// NKU ku / KU v m (ku) / NKV kv / KV v m (kv). Returns a rational Geom_BSplineSurface.
static Handle(Geom_BSplineSurface) read_surface(std::istream& in) {
    std::string kw; int du = 0, dv = 0, nu = 0, nv = 0;
    in >> kw >> du >> kw >> dv;       // DEGU du DEGV dv
    in >> kw >> nu >> kw >> nv;       // NU nu NV nv
    TColgp_Array2OfPnt poles(1, nu, 1, nv);
    TColStd_Array2OfReal wts(1, nu, 1, nv);
    for (int t = 0; t < nu * nv; t++) {
        int i, j; double x, y, z, w; in >> kw >> i >> j >> x >> y >> z >> w;
        poles.SetValue(i + 1, j + 1, gp_Pnt(x, y, z));
        wts.SetValue(i + 1, j + 1, w);
    }
    int ku = 0, kv = 0;
    in >> kw >> ku;
    TColStd_Array1OfReal uk(1, ku); TColStd_Array1OfInteger um(1, ku);
    for (int i = 1; i <= ku; i++) { double v; int m; in >> kw >> v >> m; uk.SetValue(i, v); um.SetValue(i, m); }
    in >> kw >> kv;
    TColStd_Array1OfReal vk(1, kv); TColStd_Array1OfInteger vm(1, kv);
    for (int i = 1; i <= kv; i++) { double v; int m; in >> kw >> v >> m; vk.SetValue(i, v); vm.SetValue(i, m); }
    return new Geom_BSplineSurface(poles, wts, uk, vk, um, vm, du, dv, Standard_False, Standard_False);
}

int main(int argc, char** argv) {
    if (argc < 3) { std::cerr << "usage: oracle <in> <out>\n"; return 2; }
    std::ifstream in(argv[1]);
    std::ofstream out(argv[2]);
    if (!in || !out) { std::cerr << "io error\n"; return 2; }
    out << std::setprecision(16);

    std::string tok, op;
    in >> tok >> op; // OP <op>

    if (op == "ssi") {
        std::vector<Spec> specs;
        double tol = 1e-6;
        std::string kw;
        while (in >> kw) {
            if (kw == "SURF") {
                Spec s; in >> s.kind;
                int n = 0;
                if (s.kind == "cylinder" || s.kind == "torus") n = 2;
                else if (s.kind == "sphere") n = 1;
                else if (s.kind == "cone") n = 3;
                else if (s.kind == "plane") n = 0;
                for (int i = 0; i < n; i++) { double v; in >> v; s.p.push_back(v); }
                specs.push_back(s);
            } else if (kw == "XF") {
                Spec& s = specs.back(); s.hasXf = true;
                in >> s.tx >> s.ty >> s.tz >> s.ax >> s.ay >> s.az >> s.deg;
            } else if (kw == "TOL") {
                in >> tol;
            }
        }
        if (specs.size() < 2) { out << "ERROR need two surfaces\n"; return 0; }
        TopoDS_Face fa = build_face(specs[0]);
        TopoDS_Face fb = build_face(specs[1]);
        if (fa.IsNull() || fb.IsNull()) { out << "ERROR bad surface\n"; return 0; }

        // Bounded section, then join edges into wires = logical intersection curves.
        BRepAlgoAPI_Section sec(fa, fb, Standard_False);
        sec.ComputePCurveOn1(Standard_False);
        sec.Approximation(Standard_False); // keep exact analytic section curves
        sec.Build();
        if (!sec.IsDone()) { out << "ERROR section not done\n"; return 0; }

        Handle(TopTools_HSequenceOfShape) edges = new TopTools_HSequenceOfShape();
        for (TopExp_Explorer ex(sec.Shape(), TopAbs_EDGE); ex.More(); ex.Next())
            edges->Append(ex.Current());

        Handle(TopTools_HSequenceOfShape) wires = new TopTools_HSequenceOfShape();
        ShapeAnalysis_FreeBounds::ConnectEdgesToWires(edges, tol, Standard_False, wires);

        out << "NCURVES " << wires->Length() << "\n";
        for (int wi = 1; wi <= wires->Length(); wi++) {
            // Walk the wire's edges, sample each by arc length, concatenate.
            std::vector<gp_Pnt> pts;
            for (TopExp_Explorer ex(wires->Value(wi), TopAbs_EDGE); ex.More(); ex.Next()) {
                TopoDS_Edge e = TopoDS::Edge(ex.Current());
                BRepAdaptor_Curve ac(e);
                int seg = 200;
                for (int k = 0; k <= seg; k++) {
                    double t = ac.FirstParameter() + (ac.LastParameter() - ac.FirstParameter()) * (double)k / seg;
                    pts.push_back(ac.Value(t));
                }
            }
            out << "CURVE " << pts.size() << "\n";
            for (auto& p : pts) out << p.X() << " " << p.Y() << " " << p.Z() << "\n";
        }
    }
    else if (op == "interpolate") {
        int n = 0; std::string kw;
        in >> kw >> n; // NPTS n
        Handle(TColgp_HArray1OfPnt) pts = new TColgp_HArray1OfPnt(1, n);
        for (int i = 1; i <= n; i++) {
            std::string p; double x, y, z; in >> p >> x >> y >> z; // PT x y z
            pts->SetValue(i, gp_Pnt(x, y, z));
        }
        int periodic = 0;
        if (in >> kw >> periodic) {} // PERIODIC 0/1
        try {
            GeomAPI_Interpolate interp(pts, periodic != 0, 1e-7);
            interp.Perform();
            if (!interp.IsDone()) { out << "ERROR interp not done\n"; return 0; }
            Handle(Geom_BSplineCurve) c = interp.Curve();
            out << "OK\n";
            out << "DEG " << c->Degree() << "\n";
            int np = c->NbPoles();
            out << "NPOLES " << np << "\n";
            for (int i = 1; i <= np; i++) {
                gp_Pnt p = c->Pole(i);
                double w = c->Weight(i);
                out << p.X() << " " << p.Y() << " " << p.Z() << " " << w << "\n";
            }
            int nk = c->NbKnots();
            out << "KNOTS " << nk << "\n";
            for (int i = 1; i <= nk; i++)
                out << c->Knot(i) << " " << c->Multiplicity(i) << "\n";
            double t0 = c->FirstParameter(), t1 = c->LastParameter();
            const int S = 64;
            out << "SAMPLES " << (S + 1) << "\n";
            for (int k = 0; k <= S; k++) {
                double t = t0 + (t1 - t0) * (double)k / S;
                gp_Pnt p = c->Value(t);
                out << p.X() << " " << p.Y() << " " << p.Z() << "\n";
            }
        } catch (...) { out << "ERROR interp exception\n"; return 0; }
    }
    else if (op == "curve_eval") {
        // OP curve_eval / DEG d / NPOLES n / POLE x y z w (n) / NKNOTS k / KNOT v m (k)
        int degree = 0, np = 0, nk = 0; std::string kw;
        in >> kw >> degree;       // DEG d
        in >> kw >> np;           // NPOLES n
        TColgp_Array1OfPnt poles(1, np);
        TColStd_Array1OfReal wts(1, np);
        for (int i = 1; i <= np; i++) {
            double x, y, z, w; in >> kw >> x >> y >> z >> w; // POLE x y z w
            poles.SetValue(i, gp_Pnt(x, y, z));
            wts.SetValue(i, w);
        }
        in >> kw >> nk;           // NKNOTS k
        TColStd_Array1OfReal knots(1, nk);
        TColStd_Array1OfInteger mults(1, nk);
        for (int i = 1; i <= nk; i++) {
            double v; int m; in >> kw >> v >> m; // KNOT v m
            knots.SetValue(i, v);
            mults.SetValue(i, m);
        }
        try {
            Handle(Geom_BSplineCurve) c = new Geom_BSplineCurve(poles, wts, knots, mults, degree, Standard_False);
            out << "OK\n";
            out << "DEG " << c->Degree() << "\n";
            int npo = c->NbPoles();
            out << "NPOLES " << npo << "\n";
            for (int i = 1; i <= npo; i++) {
                gp_Pnt p = c->Pole(i);
                double w = c->Weight(i);
                out << p.X() << " " << p.Y() << " " << p.Z() << " " << w << "\n";
            }
            int nkn = c->NbKnots();
            out << "KNOTS " << nkn << "\n";
            for (int i = 1; i <= nkn; i++)
                out << c->Knot(i) << " " << c->Multiplicity(i) << "\n";
            double t0 = c->FirstParameter(), t1 = c->LastParameter();
            const int S = 64;
            out << "SAMPLES " << (S + 1) << "\n";
            for (int k = 0; k <= S; k++) {
                double t = t0 + (t1 - t0) * (double)k / S;
                gp_Pnt p = c->Value(t);
                out << p.X() << " " << p.Y() << " " << p.Z() << "\n";
            }
        } catch (...) { out << "ERROR curve_eval exception\n"; return 0; }
    }
    else if (op == "curve_length") {
        Handle(Geom_BSplineCurve) c = read_curve(in);
        GeomAdaptor_Curve ad(c);
        out << "OK\n";
        // Tight tolerance so OCCT integrates to the true length (its default tol is loose).
        out << "LENGTH " << GCPnts_AbscissaPoint::Length(ad, 1e-11) << "\n";
    }
    else if (op == "curve_divide") {
        // <curve block> / N count
        Handle(Geom_BSplineCurve) c = read_curve(in);
        std::string kw; int count = 0; in >> kw >> count; // N count
        GeomAdaptor_Curve ad(c);
        double L = GCPnts_AbscissaPoint::Length(ad);
        // compas divide_by_count: params = [a] + abscissa(length=L/count) + [b]
        GCPnts_UniformAbscissa ua(ad, count + 1);  // count+1 points -> count segments
        out << "OK\n";
        std::vector<double> params;
        double a = c->FirstParameter(), b = c->LastParameter();
        if (ua.IsDone()) {
            for (int i = 1; i <= ua.NbPoints(); i++) params.push_back(ua.Parameter(i));
        }
        // Ensure endpoints present (compas brackets with domain a,b)
        out << "LENGTH " << L << "\n";
        out << "NPARAMS " << params.size() << "\n";
        for (double t : params) {
            gp_Pnt p = c->Value(t);
            out << t << " " << p.X() << " " << p.Y() << " " << p.Z() << "\n";
        }
        (void)a; (void)b;
    }
    else if (op == "curve_closest") {
        // <curve block> / PT x y z
        Handle(Geom_BSplineCurve) c = read_curve(in);
        std::string kw; double x, y, z; in >> kw >> x >> y >> z; // PT x y z
        GeomAPI_ProjectPointOnCurve proj(gp_Pnt(x, y, z), c);
        out << "OK\n";
        if (proj.NbPoints() > 0) {
            double t = proj.LowerDistanceParameter();
            gp_Pnt p = proj.NearestPoint();
            out << "CLOSEST " << p.X() << " " << p.Y() << " " << p.Z()
                << " PARAM " << t << " DIST " << proj.LowerDistance() << "\n";
        } else {
            out << "CLOSEST none\n";
        }
    }
    else if (op == "curve_extrema") {
        // <curve block A> / <curve block B>
        Handle(Geom_BSplineCurve) ca = read_curve(in);
        Handle(Geom_BSplineCurve) cb = read_curve(in);
        GeomAPI_ExtremaCurveCurve ex(ca, cb);
        out << "OK\n";
        if (ex.NbExtrema() > 0) {
            double u = 0, v = 0; ex.LowerDistanceParameters(u, v);
            gp_Pnt pa, pb; ex.NearestPoints(pa, pb);
            out << "U " << u << " V " << v << " DIST " << ex.LowerDistance() << "\n";
            out << "PA " << pa.X() << " " << pa.Y() << " " << pa.Z() << "\n";
            out << "PB " << pb.X() << " " << pb.Y() << " " << pb.Z() << "\n";
        } else {
            out << "none\n";
        }
    }
    else if (op == "curve_segment") {
        // <curve block> / UV u v
        Handle(Geom_BSplineCurve) c = read_curve(in);
        std::string kw; double u, v; in >> kw >> u >> v; // UV u v
        Handle(Geom_BSplineCurve) seg = Handle(Geom_BSplineCurve)::DownCast(c->Copy());
        seg->Segment(u, v);
        out << "OK\n";
        write_curve(out, seg);
    }
    else if (op == "surface_eval") {
        // OP surface_eval / DEGU du DEGV dv / NU nu NV nv /
        //   POLE i j x y z w (nu*nv) / NKU ku / KU v m (ku) / NKV kv / KV v m (kv)
        std::string kw; int du = 0, dv = 0, nu = 0, nv = 0;
        in >> kw >> du >> kw >> dv;      // DEGU du DEGV dv
        in >> kw >> nu >> kw >> nv;      // NU nu NV nv
        TColgp_Array2OfPnt poles(1, nu, 1, nv);
        TColStd_Array2OfReal wts(1, nu, 1, nv);
        for (int t = 0; t < nu * nv; t++) {
            int i, j; double x, y, z, w; in >> kw >> i >> j >> x >> y >> z >> w; // POLE i j x y z w
            poles.SetValue(i + 1, j + 1, gp_Pnt(x, y, z));
            wts.SetValue(i + 1, j + 1, w);
        }
        int ku = 0, kv = 0;
        in >> kw >> ku;                  // NKU ku
        TColStd_Array1OfReal uk(1, ku); TColStd_Array1OfInteger um(1, ku);
        for (int i = 1; i <= ku; i++) { double v; int m; in >> kw >> v >> m; uk.SetValue(i, v); um.SetValue(i, m); }
        in >> kw >> kv;                  // NKV kv
        TColStd_Array1OfReal vk(1, kv); TColStd_Array1OfInteger vm(1, kv);
        for (int i = 1; i <= kv; i++) { double v; int m; in >> kw >> v >> m; vk.SetValue(i, v); vm.SetValue(i, m); }
        try {
            Handle(Geom_BSplineSurface) s = new Geom_BSplineSurface(
                poles, wts, uk, vk, um, vm, du, dv, Standard_False, Standard_False);
            out << "OK\n";
            out << "DEGU " << s->UDegree() << " DEGV " << s->VDegree() << "\n";
            out << "NU " << s->NbUPoles() << " NV " << s->NbVPoles() << "\n";
            for (int i = 1; i <= s->NbUPoles(); i++)
                for (int j = 1; j <= s->NbVPoles(); j++) {
                    gp_Pnt p = s->Pole(i, j); double w = s->Weight(i, j);
                    out << "POLE " << (i-1) << " " << (j-1) << " "
                        << p.X() << " " << p.Y() << " " << p.Z() << " " << w << "\n";
                }
            double u0, u1, v0, v1; s->Bounds(u0, u1, v0, v1);
            out << "DOMAIN " << u0 << " " << u1 << " " << v0 << " " << v1 << "\n";
            const int S = 24;
            out << "SAMPLES " << (S + 1) * (S + 1) << "\n";
            for (int a = 0; a <= S; a++)
                for (int b = 0; b <= S; b++) {
                    double u = u0 + (u1 - u0) * a / S;
                    double v = v0 + (v1 - v0) * b / S;
                    gp_Pnt p = s->Value(u, v);
                    out << u << " " << v << " " << p.X() << " " << p.Y() << " " << p.Z() << "\n";
                }
        } catch (...) { out << "ERROR surface_eval exception\n"; return 0; }
    }
    else if (op == "surface_frame") {
        // <surface block> / UV u v  -> point, D1u, D1v, normal at (u,v)
        Handle(Geom_BSplineSurface) s = read_surface(in);
        std::string kw; double u, v; in >> kw >> u >> v;  // UV u v
        gp_Pnt p; gp_Vec d1u, d1v;
        s->D1(u, v, p, d1u, d1v);
        gp_Vec n = d1u.Crossed(d1v);
        out << "OK\n";
        out << "POINT " << p.X() << " " << p.Y() << " " << p.Z() << "\n";
        out << "DU " << d1u.X() << " " << d1u.Y() << " " << d1u.Z() << "\n";
        out << "DV " << d1v.X() << " " << d1v.Y() << " " << d1v.Z() << "\n";
        double nm = n.Magnitude();
        if (nm > 1e-30) n.Divide(nm);
        out << "NORMAL " << n.X() << " " << n.Y() << " " << n.Z() << "\n";
    }
    else if (op == "surface_line") {
        // <surface block> / LINE px py pz dx dy dz  -> intersection points
        Handle(Geom_BSplineSurface) s = read_surface(in);
        std::string kw; double px, py, pz, dx, dy, dz;
        in >> kw >> px >> py >> pz >> dx >> dy >> dz;  // LINE p d
        gp_Lin lin(gp_Pnt(px, py, pz), gp_Dir(dx, dy, dz));
        Handle(Geom_Line) gl = new Geom_Line(lin);
        GeomAPI_IntCS intcs(gl, s);
        out << "OK\n";
        if (!intcs.IsDone()) { out << "NPTS 0\n"; return 0; }
        int n = intcs.NbPoints();
        out << "NPTS " << n << "\n";
        for (int i = 1; i <= n; i++) {
            gp_Pnt p = intcs.Point(i);
            out << p.X() << " " << p.Y() << " " << p.Z() << "\n";
        }
    }
    else if (op == "solid") {
        // OP solid / SHAPE <kind> <params...>
        std::string kw, kind; in >> kw >> kind;
        std::vector<double> p;
        if (kind == "box") { p.resize(3); in >> p[0] >> p[1] >> p[2]; }
        else if (kind == "cylinder") { p.resize(2); in >> p[0] >> p[1]; }
        else if (kind == "sphere") { p.resize(1); in >> p[0]; }
        else if (kind == "cone") { p.resize(2); in >> p[0] >> p[1]; }
        else if (kind == "torus") { p.resize(2); in >> p[0] >> p[1]; }
        TopoDS_Shape s = build_solid(kind, p);
        if (s.IsNull()) { out << "ERROR bad solid\n"; return 0; }
        out << "OK\n";
        solid_props(out, s);
    }
    else if (op == "classify") {
        // OP classify / SHAPE <kind> <params> XF tx ty tz ax ay az deg / NPTS n / PT x y z (n)
        std::string kw, kind; in >> kw >> kind;  // SHAPE kind
        std::vector<double> p;
        if (kind == "box") { p.resize(3); in >> p[0] >> p[1] >> p[2]; }
        else if (kind == "cylinder") { p.resize(2); in >> p[0] >> p[1]; }
        else if (kind == "sphere") { p.resize(1); in >> p[0]; }
        else if (kind == "cone") { p.resize(2); in >> p[0] >> p[1]; }
        else if (kind == "torus") { p.resize(2); in >> p[0] >> p[1]; }
        TopoDS_Shape s = build_solid(kind, p);
        std::string xfkw; Spec sp; sp.hasXf = true;
        in >> xfkw >> sp.tx >> sp.ty >> sp.tz >> sp.ax >> sp.ay >> sp.az >> sp.deg;  // XF ...
        if (!s.IsNull() && (sp.tx || sp.ty || sp.tz || sp.deg)) {
            BRepBuilderAPI_Transform xf(s, make_trsf(sp), Standard_True);
            s = xf.Shape();
        }
        if (s.IsNull()) { out << "ERROR bad classify solid\n"; return 0; }
        int n = 0; in >> kw >> n;  // NPTS n
        out << "OK\n";
        out << "STATES " << n << "\n";
        BRepClass3d_SolidClassifier cls(s);
        for (int i = 0; i < n; i++) {
            double x, y, z; in >> kw >> x >> y >> z;  // PT x y z
            cls.Perform(gp_Pnt(x, y, z), 1e-7);
            TopAbs_State st = cls.State();
            out << (st == TopAbs_IN ? "IN" : st == TopAbs_OUT ? "OUT" : st == TopAbs_ON ? "ON" : "UNK") << "\n";
        }
    }
    else if (op == "boolean_bench") {
        // OP boolean_bench / MODE <mode> / N <iters> / SHAPE ... / SHAPE ...
        std::string kw, mode; in >> kw >> mode;       // MODE
        int iters = 0; in >> kw >> iters;             // N iters
        TopoDS_Shape a = read_solid_shape(in);
        TopoDS_Shape b = read_solid_shape(in);
        if (a.IsNull() || b.IsNull()) { out << "ERROR bad bench operands\n"; return 0; }
        // Warm up once (first run pays one-time init).
        try { run_boolean(mode, a, b); } catch (...) {}
        auto t0 = std::chrono::high_resolution_clock::now();
        for (int i = 0; i < iters; i++) {
            try { TopoDS_Shape r = run_boolean(mode, a, b); (void)r; } catch (...) {}
        }
        auto t1 = std::chrono::high_resolution_clock::now();
        double us = std::chrono::duration<double, std::micro>(t1 - t0).count() / (iters > 0 ? iters : 1);
        out << "OK\n";
        out << "ITERS " << iters << "\n";
        out << "AVG_US " << us << "\n";
    }
    else if (op == "boolean") {
        // OP boolean / MODE <fuse|cut|common> / SHAPE <kind> <params> [XF ...] (A) /
        //   SHAPE <kind> <params> [XF ...] (B).   XF = tx ty tz ax ay az deg.
        std::string kw, mode; in >> kw >> mode;  // MODE <mode>
        // Grammar: SHAPE <kind> <params...> XF tx ty tz ax ay az deg  (XF mandatory;
        // identity transform = "XF 0 0 0 0 0 1 0").
        auto read_shape = [&](TopoDS_Shape& shp) {
            std::string k2, kind; in >> k2 >> kind;  // SHAPE kind
            std::vector<double> p;
            if (kind == "box") { p.resize(3); in >> p[0] >> p[1] >> p[2]; }
            else if (kind == "cylinder") { p.resize(2); in >> p[0] >> p[1]; }
            else if (kind == "sphere") { p.resize(1); in >> p[0]; }
            else if (kind == "cone") { p.resize(2); in >> p[0] >> p[1]; }
            else if (kind == "torus") { p.resize(2); in >> p[0] >> p[1]; }
            shp = (kind == "nurbs") ? build_nurbs_solid(in) : build_solid(kind, p);
            std::string xfkw; Spec sp; sp.hasXf = true;
            in >> xfkw >> sp.tx >> sp.ty >> sp.tz >> sp.ax >> sp.ay >> sp.az >> sp.deg;  // XF ...
            if (!shp.IsNull() && (sp.tx || sp.ty || sp.tz || sp.deg)) {
                BRepBuilderAPI_Transform xf(shp, make_trsf(sp), Standard_True);
                shp = xf.Shape();
            }
        };
        TopoDS_Shape a, b; read_shape(a); read_shape(b);
        if (a.IsNull() || b.IsNull()) { out << "ERROR bad boolean operands\n"; return 0; }
        TopoDS_Shape r;
        try {
            if (mode == "fuse") r = BRepAlgoAPI_Fuse(a, b).Shape();
            else if (mode == "cut") r = BRepAlgoAPI_Cut(a, b).Shape();
            else if (mode == "common") r = BRepAlgoAPI_Common(a, b).Shape();
            else { out << "ERROR bad boolean mode\n"; return 0; }
        } catch (...) { out << "ERROR boolean exception\n"; return 0; }
        out << "OK\n";
        solid_props(out, r);
        write_boundary_samples(out, r, 0.1);  // boundary samples for Hausdorff comparison
    }
    else {
        out << "ERROR unknown op " << op << "\n";
    }
    return 0;
}
