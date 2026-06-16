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
#include <GeomAPI_Interpolate.hxx>
#include <gp_Ax3.hxx>
#include <gp_Pnt.hxx>
#include <gp_Dir.hxx>
#include <gp_Trsf.hxx>
#include <gp_Ax1.hxx>
#include <TColgp_HArray1OfPnt.hxx>
#include <TColStd_Array1OfReal.hxx>
#include <TColStd_Array1OfInteger.hxx>

#include <fstream>
#include <iostream>
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

int main(int argc, char** argv) {
    if (argc < 3) { std::cerr << "usage: oracle <in> <out>\n"; return 2; }
    std::ifstream in(argv[1]);
    std::ofstream out(argv[2]);
    if (!in || !out) { std::cerr << "io error\n"; return 2; }

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
    else {
        out << "ERROR unknown op " << op << "\n";
    }
    return 0;
}
