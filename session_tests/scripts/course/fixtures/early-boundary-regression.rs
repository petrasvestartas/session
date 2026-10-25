use std::rc::Rc;
use viewer_journey::{cad_boundary, cad_display, editor::{Action, Editor}, pick_sources::{Part, Sources}};

fn main() {
    let mut boundary=cad_boundary::Prepared::new(Rc::new(session_rust::BRep::create_cylinder(1.,2.))).unwrap();
    for face in &mut boundary.faces { for vertex in face.vertex.values_mut() { vertex.x+=0.03; } }
    let mut closed=0;
    let mut refined=0;
    for chain in boundary.chains.iter().flatten() {
        let (points,spans)=boundary.display(chain.edge).unwrap();
        assert_eq!(points.len(),spans.len()+1);
        assert!(spans.iter().all(|span|span+1<chain.keys.len()));
        assert!(spans.windows(2).all(|p|p[0]<=p[1]));
        for p in &points { assert!((p[0]*p[0]+p[1]*p[1]-1.).abs()<1e-9,"Authored cylindrical edge, not displaced face chord: {p:?}"); }
        if points.len()>chain.keys.len() { refined+=1; }
        if boundary.source.m_curves_3d[boundary.source.m_edges[chain.edge].curve_3d_index as usize].is_closed() {
            closed+=1;
            let a=points.first().unwrap();let b=points.last().unwrap();
            assert!((0..3).map(|i|(a[i]-b[i]).powi(2)).sum::<f64>()<1e-18);
            assert!(points.windows(2).all(|p|(0..3).map(|i|(p[0][i]-p[1][i]).powi(2)).sum::<f64>()>1e-18));
        }
    }
    assert!(closed>0,"The fixture exercises a closed original edge");
    assert!(refined>0,"The exact display subdivides a coarse curved mesh chain");
    let mut editor=Editor::default();editor.apply(Action::Close).unwrap();editor.apply(Action::AddCad).unwrap();
    let row=&editor.scene.objects()[0];let cad=row.cad.as_ref().unwrap();
    let sources=Sources::new(&editor.scene).unwrap();
    assert_eq!(cad.edge_ids.len(),row.mesh.parts.endpoints.len());
    assert_eq!(sources.mesh_ink.len(),cad.edge_ids.len());
    for (target,edge) in sources.edges.iter().zip(&cad.edge_ids) {assert_eq!(target.owner,row.id);assert_eq!(target.part,Part::BRepEdge(*edge));}
    let (_,mesh)=cad_display::prepare(Rc::new(cad_display::example())).unwrap();
    assert_eq!(mesh.display.parts.edges.len(),mesh.display.parts.endpoints.len());
    println!("PASS exact authored cylinder, closed seam, chain spans, and original CAD owner addresses");
}
