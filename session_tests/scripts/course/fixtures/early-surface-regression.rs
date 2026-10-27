use std::rc::Rc;
use viewer_journey::surface::{Grid,Prepared};
fn main() {
    let source=session_rust::BRep::create_cylinder(1.,2.).m_surfaces.remove(0);
    let _=source.guid();let before=source.to_proto();let grid=Grid::new(&source).unwrap();let [nu,nv]=grid.steps;
    let face_vertices=(nu+1)*(nv+1);
    assert_eq!(grid.indices.len(),nu*nv*6);assert!(grid.indices.iter().all(|i|(*i as usize)<face_vertices));
    assert!(grid.samples.len()>face_vertices,"Curved borders add exact samples without retessellating faces");
    assert_eq!(grid.boundaries.iter().map(|b|b.edge).collect::<Vec<_>>(),vec![2,3]);
    for boundary in &grid.boundaries {
        for pair in &boundary.segments {
            for &index in pair {let p=grid.samples[index as usize].point;assert!((p[0]*p[0]+p[1]*p[1]-1.).abs()<1e-9);}
        }
        assert!(boundary.segments.windows(2).all(|p|p[0][1]==p[1][0]));
    }
    let prepared=Prepared::new(Rc::new(source)).unwrap();let mesh=prepared.mesh().unwrap();
    assert_eq!(mesh.display.indices().len(),grid.indices.len());
    assert_eq!(prepared.edge_ids().count(),mesh.display.parts.endpoints.len());
    assert!(prepared.edge_ids().all(|id|id==2||id==3));
    for ends in &mesh.display.parts.endpoints {for p in ends {assert!((p[0]*p[0]+p[1]*p[1]-1.).abs()<1e-9);}}
    assert_eq!(prepared.source.to_proto(),before);
    println!("PASS exact rational surface borders, unchanged face topology, joined source edge IDs and source archive");
}
