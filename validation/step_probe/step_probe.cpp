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
#include <TopoDS_Face.hxx>
#include <Interface_Static.hxx>
#include <cstdlib>
#include <cstdio>

int main(int argc, char** argv) {
    if (argc < 2) { std::printf("usage: step_probe <file.step>\n"); return 2; }
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
    if (argc > 2) {   // verbose: per-face area + orientation + surface type + UV bounds
        int i = 0;
        for (TopExp_Explorer e(s, TopAbs_FACE); e.More(); e.Next(), ++i) {
            GProp_GProps fp; BRepGProp::SurfaceProperties(e.Current(), fp);
            TopoDS_Face f = TopoDS::Face(e.Current());
            Handle(Geom_Surface) surf = BRep_Tool::Surface(f);
            double u0, u1, v0, v1;
            BRepTools::UVBounds(f, u0, u1, v0, v1);
            std::printf("FACE %d area %.6f orient %s srf %s uv [%.4f,%.4f]x[%.4f,%.4f]\n",
                        i, fp.Mass(),
                        e.Current().Orientation() == TopAbs_REVERSED ? "REV" : "FWD",
                        surf.IsNull() ? "null" : surf->DynamicType()->Name(),
                        u0, u1, v0, v1);
        }
    }
    return 0;
}
