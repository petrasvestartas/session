// Tier-4/Tier-2 STEP corpus generator.
//   brep2step <in.brep> <out.step>   BRepTools::Read -> STEPControl_Writer (OCCT test models)
//   brep2step --prim <outdir>        OCCT-authored primitives matching our kernel's reference
//                                    dimensions (box 4x4x4 centered, sphere r=2.5, cylinder
//                                    r=1.5 h=6 base z=0, cone r=2 h=4 base z=0 apex up,
//                                    torus R=2 r=0.8) -> occt_prim_<name>.step
// Prints SOLIDS/FACES/VOLUME/VALID of the shape it wrote (writer-side truth; the reader-side
// truth line comes from step_probe on the emitted file).
#include <BRep_Builder.hxx>
#include <BRepTools.hxx>
#include <TopoDS_Shape.hxx>
#include <TopExp_Explorer.hxx>
#include <BRepGProp.hxx>
#include <GProp_GProps.hxx>
#include <BRepCheck_Analyzer.hxx>
#include <STEPControl_Writer.hxx>
#include <BRepPrimAPI_MakeBox.hxx>
#include <BRepPrimAPI_MakeSphere.hxx>
#include <BRepPrimAPI_MakeCylinder.hxx>
#include <BRepPrimAPI_MakeCone.hxx>
#include <BRepPrimAPI_MakeTorus.hxx>
#include <gp_Pnt.hxx>
#include <gp_Trsf.hxx>
#include <gp_Vec.hxx>
#include <Bnd_Box.hxx>
#include <BRepBndLib.hxx>
#include <BRepBuilderAPI_Transform.hxx>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <string>

static void report(const char* name, const TopoDS_Shape& s) {
    int nsolid = 0, nface = 0;
    for (TopExp_Explorer e(s, TopAbs_SOLID); e.More(); e.Next()) nsolid++;
    for (TopExp_Explorer e(s, TopAbs_FACE); e.More(); e.Next()) nface++;
    double vol = 0.0;
    if (nface > 0) { GProp_GProps vp; BRepGProp::VolumeProperties(s, vp, 1e-9); vol = vp.Mass(); }
    int valid = (nface > 0) ? (BRepCheck_Analyzer(s).IsValid() ? 1 : 0) : 0;
    std::printf("%s SOLIDS %d FACES %d VOLUME %.9f VALID %d\n", name, nsolid, nface, vol, valid);
}

static int write_step(const TopoDS_Shape& s, const std::string& path) {
    STEPControl_Writer sw;
    sw.Transfer(s, STEPControl_AsIs);
    return sw.Write(path.c_str()) == IFSelect_RetDone ? 0 : 1;
}

int main(int argc, char** argv) {
    if (argc < 3) { std::printf("usage: brep2step <in.brep> <out.step> | brep2step --prim <outdir> | brep2step --pair <a.brep> <b.brep> <outA.step> <outB.step> fx fy fz\n"); return 2; }
    if (std::strcmp(argv[1], "--pair") == 0) {
        if (argc < 9) { std::printf("usage: brep2step --pair <a.brep> <b.brep> <outA.step> <outB.step> fx fy fz\n"); return 2; }
        TopoDS_Shape a, bb;
        BRep_Builder bld;
        if (!BRepTools::Read(a, argv[2], bld)) { std::printf("BREP_READ_FAIL A\n"); return 1; }
        if (!BRepTools::Read(bb, argv[3], bld)) { std::printf("BREP_READ_FAIL B\n"); return 1; }
        Bnd_Box ba, bbx;
        BRepBndLib::Add(a, ba);
        BRepBndLib::Add(bb, bbx);
        double ax0, ay0, az0, ax1, ay1, az1, bx0, by0, bz0, bx1, by1, bz1;
        ba.Get(ax0, ay0, az0, ax1, ay1, az1);
        bbx.Get(bx0, by0, bz0, bx1, by1, bz1);
        double fx = std::atof(argv[6]), fy = std::atof(argv[7]), fz = std::atof(argv[8]);
        gp_Vec mv(0.5 * (ax0 + ax1) - 0.5 * (bx0 + bx1) + fx * (ax1 - ax0),
                  0.5 * (ay0 + ay1) - 0.5 * (by0 + by1) + fy * (ay1 - ay0),
                  0.5 * (az0 + az1) - 0.5 * (bz0 + bz1) + fz * (az1 - az0));
        gp_Trsf t;
        t.SetTranslation(mv);
        TopoDS_Shape bmoved = BRepBuilderAPI_Transform(bb, t, Standard_True).Shape();
        if (write_step(a, argv[4])) { std::printf("STEP_WRITE_FAIL A\n"); return 1; }
        if (write_step(bmoved, argv[5])) { std::printf("STEP_WRITE_FAIL B\n"); return 1; }
        std::printf("MOVE %.9f %.9f %.9f\n", mv.X(), mv.Y(), mv.Z());
        report("A", a);
        report("B", bmoved);
        return 0;
    }
    if (std::strcmp(argv[1], "--prim") == 0) {
        std::string d = argv[2];
        struct P { const char* name; TopoDS_Shape s; } prims[] = {
            {"box",      BRepPrimAPI_MakeBox(gp_Pnt(-2, -2, -2), 4.0, 4.0, 4.0).Solid()},
            {"sphere",   BRepPrimAPI_MakeSphere(2.5).Solid()},
            {"cylinder", BRepPrimAPI_MakeCylinder(1.5, 6.0).Solid()},
            {"cone",     BRepPrimAPI_MakeCone(2.0, 0.0, 4.0).Solid()},
            {"torus",    BRepPrimAPI_MakeTorus(2.0, 0.8).Solid()},
        };
        for (auto& p : prims) {
            std::string out = d + "/occt_prim_" + p.name + ".step";
            if (write_step(p.s, out)) { std::printf("%s WRITE_FAIL\n", p.name); return 1; }
            report(p.name, p.s);
        }
        return 0;
    }
    TopoDS_Shape s;
    BRep_Builder b;
    if (!BRepTools::Read(s, argv[1], b)) { std::printf("BREP_READ_FAIL\n"); return 1; }
    if (write_step(s, argv[2])) { std::printf("STEP_WRITE_FAIL\n"); return 1; }
    report(argv[1], s);
    return 0;
}
