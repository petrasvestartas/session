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
#include <Message.hxx>
#include <Message_PrinterOStream.hxx>
#include <Message_Messenger.hxx>
#include <BRepCheck.hxx>
#include <BRepCheck_Result.hxx>
#include <TopExp.hxx>
#include <TopTools_IndexedDataMapOfShapeListOfShape.hxx>
#include <BRepPrimAPI_MakeCylinder.hxx>
#include <BRepPrimAPI_MakeCone.hxx>
#include <BRepBuilderAPI_MakePolygon.hxx>
#include <BRepBuilderAPI_MakeFace.hxx>
#include <BRepBuilderAPI_MakeSolid.hxx>
#include <BRepBuilderAPI_Sewing.hxx>
#include <TopoDS_Shell.hxx>
#include <gp_Ax2.hxx>
#include <gp_Dir.hxx>
#include <vector>
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
    // Operand writer for the rotated-primitive corpus tier:
    //   --prim box   out.step sx sy sz          (corner at -s/2, centred)
    //   --prim sph   out.step r
    //   --prim cyl   out.step r h               (axis +z, base at -h/2)
    //   --prim cone  out.step r h               (axis +z, base at -h/2)
    //   --prim tor   out.step R r
    if (std::strcmp(argv[1], "--prim") == 0 && argc >= 5) {
        const char* kind = argv[2];
        const char* out = argv[3];
        double p[4] = {0, 0, 0, 0};
        for (int i = 4; i < argc && i - 4 < 4; ++i) p[i - 4] = std::atof(argv[i]);
        TopoDS_Shape s;
        if (std::strcmp(kind, "box") == 0)
            s = BRepPrimAPI_MakeBox(gp_Pnt(-p[0] / 2, -p[1] / 2, -p[2] / 2),
                                    p[0], p[1], p[2]).Solid();
        else if (std::strcmp(kind, "sph") == 0)
            s = BRepPrimAPI_MakeSphere(p[0]).Solid();
        else if (std::strcmp(kind, "cyl") == 0)
            s = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -p[1] / 2), gp_Dir(0, 0, 1)),
                                         p[0], p[1]).Solid();
        else if (std::strcmp(kind, "cone") == 0)
            s = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(0, 0, -p[1] / 2), gp_Dir(0, 0, 1)),
                                     p[0], 0.0, p[1]).Solid();
        else if (std::strcmp(kind, "tor") == 0)
            s = BRepPrimAPI_MakeTorus(p[0], p[1]).Solid();
        else { std::printf("BAD_KIND %s\n", kind); return 2; }
        STEPControl_Writer sw;
        sw.Transfer(s, STEPControl_AsIs);
        sw.Write(out);
        std::printf("PRIM_WRITTEN %s\n", kind);
        return 0;
    }
    // Polyhedron writer: --poly in.txt out.step, where in.txt holds
    //   V x y z            (one per vertex, 0-based index order)
    //   F i j k [...]      (one per planar face, CCW seen from outside)
    // Planar-faced, non-axis-aligned solids (platonics) are the geometry class the
    // chairs represent, and the kernel has no BRep constructors for them.
    if (std::strcmp(argv[1], "--poly") == 0 && argc >= 4) {
        std::FILE* f = std::fopen(argv[2], "r");
        if (!f) { std::printf("READ_FAIL %s\n", argv[2]); return 1; }
        std::vector<gp_Pnt> vs;
        std::vector<std::vector<int>> fs;
        char line[4096];
        while (std::fgets(line, sizeof line, f)) {
            if (line[0] == 'V') {
                double x, y, z;
                if (std::sscanf(line + 1, "%lf %lf %lf", &x, &y, &z) == 3)
                    vs.push_back(gp_Pnt(x, y, z));
            } else if (line[0] == 'F') {
                std::vector<int> idx;
                const char* c = line + 1;
                int v = 0, n = 0;
                while (std::sscanf(c, "%d%n", &v, &n) == 1) { idx.push_back(v); c += n; }
                if (idx.size() >= 3) fs.push_back(idx);
            }
        }
        std::fclose(f);
        BRepBuilderAPI_Sewing sew(1e-7);
        for (auto& idx : fs) {
            BRepBuilderAPI_MakePolygon poly;
            for (int i : idx) {
                if (i < 0 || i >= (int)vs.size()) { std::printf("BAD_INDEX %d\n", i); return 2; }
                poly.Add(vs[i]);
            }
            poly.Close();
            BRepBuilderAPI_MakeFace mf(poly.Wire(), Standard_True);
            if (!mf.IsDone()) { std::printf("FACE_FAIL\n"); return 1; }
            sew.Add(mf.Face());
        }
        sew.Perform();
        TopoDS_Shape sh = sew.SewedShape();
        TopoDS_Shape solid = sh;
        for (TopExp_Explorer e(sh, TopAbs_SHELL); e.More(); e.Next()) {
            BRepBuilderAPI_MakeSolid ms(TopoDS::Shell(e.Current()));
            if (ms.IsDone()) solid = ms.Solid();
            break;
        }
        GProp_GProps vp; BRepGProp::VolumeProperties(solid, vp, 1e-9);
        if (vp.Mass() < 0) solid.Reverse();
        STEPControl_Writer sw;
        sw.Transfer(solid, STEPControl_AsIs);
        sw.Write(argv[3]);
        std::printf("POLY_WRITTEN verts %zu faces %zu volume %.9f\n",
                    vs.size(), fs.size(), std::abs(vp.Mass()));
        return 0;
    }
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
    if (st != IFSelect_RetDone) {
        std::printf("READ_FAIL\n");
        if (argc > 2 && std::strcmp(argv[2], "--diag") == 0) {
            Message::DefaultMessenger()->AddPrinter(new Message_PrinterOStream());
            STEPControl_Reader r2;
            r2.ReadFile(argv[1]);
        }
        return 1;
    }
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
    if (argc > 2 && std::strcmp(argv[2], "-n") == 0) {   // naked edges: <2 face parents
        TopTools_IndexedDataMapOfShapeListOfShape m;
        TopExp::MapShapesAndAncestors(s, TopAbs_EDGE, TopAbs_FACE, m);
        int naked = 0, seam = 0, shared = 0, degen = 0, nonman = 0;
        for (int i = 1; i <= m.Extent(); ++i) {
            const TopoDS_Edge& e = TopoDS::Edge(m.FindKey(i));
            if (BRep_Tool::Degenerated(e)) { ++degen; continue; }
            int nf = m.FindFromIndex(i).Extent();
            if (nf > 2) { ++nonman; ++shared; continue; }   // non-manifold: >2 faces
            if (nf >= 2) { ++shared; continue; }
            // A seam edge has ONE face parent but occurs twice inside that face.
            int occ = 0;
            if (nf == 1) {
                const TopoDS_Shape& f = m.FindFromIndex(i).First();
                for (TopExp_Explorer ee(f, TopAbs_EDGE); ee.More(); ee.Next())
                    if (ee.Current().IsSame(e)) ++occ;
            }
            if (occ >= 2) ++seam; else ++naked;
        }
        int nclosed = 0, nopen = 0;
        for (TopExp_Explorer e(s, TopAbs_SHELL); e.More(); e.Next())
            (BRep_Tool::IsClosed(e.Current()) ? nclosed : nopen)++;
        std::printf("EDGES %d\nNAKED %d\nSEAM %d\nSHARED %d\nDEGEN %d\nNONMANIFOLD %d\n"
                    "SHELLS_CLOSED %d\nSHELLS_OPEN %d\n",
                    m.Extent(), naked, seam, shared, degen, nonman, nclosed, nopen);
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
