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
#include <cstdio>
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
    if (argc < 3) { std::printf("usage: brep2step <in.brep> <out.step> | brep2step --prim <outdir>\n"); return 2; }
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
