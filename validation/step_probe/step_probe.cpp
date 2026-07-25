// Reads a STEP file with OCCT's STRICT importer (the same entry point CAD apps use) and
// reports what an importer would actually see: transferable roots, solids, shells, faces,
// volume, validity. Rhino-proxy gate for our STEP writer: a file with ROOTS 0 imports as
// NOTHING in Rhino no matter how much geometry its DATA section contains.
#include <STEPControl_Reader.hxx>
#include <TopExp_Explorer.hxx>
#include <TopoDS_Shape.hxx>
#include <BRepGProp.hxx>
#include <GProp_GProps.hxx>
#include <BRepCheck_Analyzer.hxx>
#include <BRepClass3d_SolidClassifier.hxx>
#include <gp_Pnt.hxx>
#include <TopoDS.hxx>
#include <BRep_Tool.hxx>
#include <BRepTools.hxx>
#include <Geom_Surface.hxx>
#include <Geom2d_Curve.hxx>
#include <TopoDS_Edge.hxx>
#include <gp_Pnt2d.hxx>
#include <TopoDS_Face.hxx>
#include <Interface_Static.hxx>
#include <STEPControl_Writer.hxx>
#include <BRepCheck.hxx>
#include <BRepCheck_Result.hxx>
#include <iostream>
#include <BRepPrimAPI_MakeBox.hxx>
#include <BRepPrimAPI_MakeTorus.hxx>
#include <BRepAlgoAPI_Cut.hxx>
#include <BRepAlgoAPI_Common.hxx>
#include <BRepAlgoAPI_Fuse.hxx>
#include <BRepBuilderAPI_Transform.hxx>
#include <BRepBuilderAPI_NurbsConvert.hxx>
#include <BRepPrimAPI_MakeSphere.hxx>
#include <cstdlib>
#include <cstdio>
#include <cstring>

int main(int argc, char** argv) {
    if (argc < 2) { std::printf("usage: step_probe <file.step>\n"); return 2; }
    // Reference writer: OCCT's own box(4,4,4,centered) cut torus(2,0.8) to STEP -- the
    // ground-truth entity structure to diff our writer against.
    if (std::strcmp(argv[1], "--make-ref") == 0 && argc >= 3) {
        TopoDS_Shape a, b;
        if (argc >= 4 && std::strcmp(argv[3], "bsphere") == 0) {
            TopoDS_Shape sp = BRepPrimAPI_MakeSphere(2.5).Solid();
            a = BRepBuilderAPI_NurbsConvert(sp, true).Shape();
            STEPControl_Writer sw;
            sw.Transfer(a, STEPControl_AsIs);
            sw.Write(argv[2]);
            std::printf("REF_WRITTEN\n");
            return 0;
        }
        if (argc >= 4 && std::strcmp(argv[3], "tortor") == 0) {
            a = BRepPrimAPI_MakeTorus(2.0, 0.8).Solid();
            gp_Trsf t; t.SetTranslation(gp_Vec(2, 0, 0));
            b = BRepBuilderAPI_Transform(BRepPrimAPI_MakeTorus(2.0, 0.8).Solid(), t, true).Shape();
        } else {
            a = BRepPrimAPI_MakeBox(gp_Pnt(-2, -2, -2), 4.0, 4.0, 4.0).Solid();
            b = BRepPrimAPI_MakeTorus(2.0, 0.8).Solid();
        }
        TopoDS_Shape rr = BRepAlgoAPI_Cut(a, b).Shape();
        STEPControl_Writer sw;
        sw.Transfer(rr, STEPControl_AsIs);
        sw.Write(argv[2]);
        std::printf("REF_WRITTEN\n");
        return 0;
    }
    // Point classification oracle: --inside file.step x y z [x y z ...] -> IN/OUT/ON per
    // point, using OCCT's exact solid classifier (truth for our winding/angle debates).
    if (std::strcmp(argv[1], "--inside") == 0 && argc >= 6) {
        STEPControl_Reader r2;
        if (r2.ReadFile(argv[2]) != IFSelect_RetDone) { std::printf("READ_FAIL\n"); return 1; }
        r2.TransferRoots();
        TopoDS_Shape s2 = r2.OneShape();
        BRepClass3d_SolidClassifier c2(s2);
        for (int k = 3; k + 2 < argc; k += 3) {
            gp_Pnt p(std::atof(argv[k]), std::atof(argv[k+1]), std::atof(argv[k+2]));
            c2.Perform(p, 1e-7);
            std::printf("PT %s %s %s -> %s\n", argv[k], argv[k+1], argv[k+2],
                        c2.State() == TopAbs_IN ? "IN" : c2.State() == TopAbs_OUT ? "OUT" : "ON");
        }
        return 0;
    }
    // Boolean oracle on IMPORTED files: --cut A.step B.step reads both with the strict
    // importer, runs OCCT's cut, and reports the result -- the truth reference for our
    // imported-brep boolean campaign (oracle.exe only builds primitives).
    if ((std::strcmp(argv[1], "--cut") == 0 || std::strcmp(argv[1], "--common") == 0
         || std::strcmp(argv[1], "--fuse") == 0) && argc >= 4) {
        STEPControl_Reader ra, rb;
        if (ra.ReadFile(argv[2]) != IFSelect_RetDone) { std::printf("READ_FAIL A\n"); return 1; }
        if (rb.ReadFile(argv[3]) != IFSelect_RetDone) { std::printf("READ_FAIL B\n"); return 1; }
        ra.TransferRoots(); rb.TransferRoots();
        TopoDS_Shape rr;
        if (std::strcmp(argv[1], "--common") == 0)
            rr = BRepAlgoAPI_Common(ra.OneShape(), rb.OneShape()).Shape();
        else if (std::strcmp(argv[1], "--fuse") == 0)
            rr = BRepAlgoAPI_Fuse(ra.OneShape(), rb.OneShape()).Shape();
        else
            rr = BRepAlgoAPI_Cut(ra.OneShape(), rb.OneShape()).Shape();
        int nsolid = 0, nface = 0;
        for (TopExp_Explorer e(rr, TopAbs_SOLID); e.More(); e.Next()) nsolid++;
        for (TopExp_Explorer e(rr, TopAbs_FACE); e.More(); e.Next()) nface++;
        double vol = 0.0;
        if (nface > 0) { GProp_GProps vp; BRepGProp::VolumeProperties(rr, vp, 1e-9); vol = vp.Mass(); }
        std::printf("OP_SOLIDS %d\nOP_FACES %d\nOP_VOLUME %.9f\nOP_VALID %d\n",
                    nsolid, nface, vol, nface > 0 ? (BRepCheck_Analyzer(rr).IsValid() ? 1 : 0) : 0);
        return 0;
    }
    if (const char* m = std::getenv("PROBE_SCMODE"))
        Interface_Static::SetIVal("read.surfacecurve.mode", std::atoi(m));
    STEPControl_Reader r;
    IFSelect_ReturnStatus st = r.ReadFile(argv[1]);
    if (st != IFSelect_RetDone) { std::printf("READ_FAIL\n"); return 1; }
    int nroots = r.TransferRoots();
    TopoDS_Shape s = r.OneShape();
    int nsolid = 0, nshell = 0, nface = 0;
    for (TopExp_Explorer e(s, TopAbs_SOLID); e.More(); e.Next()) nsolid++;
    for (TopExp_Explorer e(s, TopAbs_SHELL); e.More(); e.Next()) nshell++;
    for (TopExp_Explorer e(s, TopAbs_FACE); e.More(); e.Next()) nface++;
    double vol = 0.0;
    if (nface > 0) { GProp_GProps vp; BRepGProp::VolumeProperties(s, vp, 1e-9); vol = vp.Mass(); }
    int valid = (nface > 0) ? (BRepCheck_Analyzer(s).IsValid() ? 1 : 0) : 0;
    const char* cls = "N/A";
    if (nsolid > 0) {
        BRepClass3d_SolidClassifier c(s);
        c.Perform(gp_Pnt(0, 0, 0), 1e-7);
        cls = c.State() == TopAbs_IN ? "IN" : c.State() == TopAbs_OUT ? "OUT" : "ON";
    }
    std::printf("ROOTS %d\nSOLIDS %d\nSHELLS %d\nFACES %d\nVOLUME %.9f\nVALID %d\nORIGIN %s\n",
                nroots, nsolid, nshell, nface, vol, valid, cls);
    if (argc > 2 && std::strcmp(argv[2], "-w") == 0) {   // per-face wires: edge pcurve ranges
        int i = 0;
        for (TopExp_Explorer e(s, TopAbs_FACE); e.More(); e.Next(), ++i) {
            TopoDS_Face f = TopoDS::Face(e.Current());
            std::printf("FACE %d\n", i);
            int wi = 0;
            for (TopExp_Explorer w2(f, TopAbs_WIRE); w2.More(); w2.Next(), ++wi) {
                std::printf(" WIRE %d\n", wi);
                for (TopExp_Explorer ee(w2.Current(), TopAbs_EDGE); ee.More(); ee.Next()) {
                    TopoDS_Edge ed = TopoDS::Edge(ee.Current());
                    double f1, l1;
                    Handle(Geom2d_Curve) pc = BRep_Tool::CurveOnSurface(ed, f, f1, l1);
                    if (pc.IsNull()) { std::printf("  EDGE no-pcurve\n"); continue; }
                    gp_Pnt2d a = pc->Value(f1), b = pc->Value(l1);
                    std::printf("  EDGE %s [%g,%g] uv (%.4f,%.4f)->(%.4f,%.4f) or=%s\n",
                                pc->DynamicType()->Name(), f1, l1,
                                a.X(), a.Y(), b.X(), b.Y(),
                                ed.Orientation() == TopAbs_REVERSED ? "R" : "F");
                }
            }
        }
        return 0;
    }
    if (argc > 2 && std::strcmp(argv[2], "-c") == 0) {   // BRepCheck: per-subshape failures
        BRepCheck_Analyzer an(s);
        const struct { TopAbs_ShapeEnum t; const char* n; } kinds[] = {
            {TopAbs_SOLID, "SOLID"}, {TopAbs_SHELL, "SHELL"}, {TopAbs_FACE, "FACE"},
            {TopAbs_WIRE, "WIRE"}, {TopAbs_EDGE, "EDGE"}, {TopAbs_VERTEX, "VERTEX"}};
        for (auto& k : kinds) {
            int i = 0;
            for (TopExp_Explorer e(s, k.t); e.More(); e.Next(), ++i) {
                Handle(BRepCheck_Result) res = an.Result(e.Current());
                if (res.IsNull()) continue;
                for (const BRepCheck_Status& st : res->Status()) {
                    if (st == BRepCheck_NoError) continue;
                    std::printf("%s %d: ", k.n, i);
                    BRepCheck::Print(st, std::cout);
                }
            }
        }
        return 0;
    }
    if (argc > 2) {   // verbose: per-face area + orientation + surface type + UV bounds
        int i = 0;
        for (TopExp_Explorer e(s, TopAbs_FACE); e.More(); e.Next(), ++i) {
            GProp_GProps fp; BRepGProp::SurfaceProperties(e.Current(), fp);
            TopoDS_Face f = TopoDS::Face(e.Current());
            Handle(Geom_Surface) surf = BRep_Tool::Surface(f);
            double u0, u1, v0, v1;
            BRepTools::UVBounds(f, u0, u1, v0, v1);
            int nw = 0, ne2 = 0;
            for (TopExp_Explorer w(f, TopAbs_WIRE); w.More(); w.Next()) nw++;
            for (TopExp_Explorer w(f, TopAbs_EDGE); w.More(); w.Next()) ne2++;
            std::printf("FACE %d area %.6f orient %s srf %s uv [%.4f,%.4f]x[%.4f,%.4f] wires %d edges %d\n",
                        i, fp.Mass(),
                        e.Current().Orientation() == TopAbs_REVERSED ? "REV" : "FWD",
                        surf.IsNull() ? "null" : surf->DynamicType()->Name(),
                        u0, u1, v0, v1, nw, ne2);
        }
    }
    return 0;
}
