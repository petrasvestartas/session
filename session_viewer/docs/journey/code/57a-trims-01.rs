use std::rc::Rc;
use session_rust::{BRep,NurbsCurve,Point,Primitives};
use session_rust::brep::{BRepRef,BRepOrientation};
use crate::cad_boundary::Prepared;

fn trimmed_wave()->BRep {
    let surface=Primitives::wave_surface(1.,0.5);let mut source=BRep::new();let si=source.add_surface(&surface);let corners=[[0.1,0.1],[0.9,0.1],[0.9,0.9],[0.1,0.9]];
    for uv in corners {source.add_vertex(&surface.point_at(uv[0],uv[1]).unwrap(),0.);}
    let mut edges=Vec::new();
    for side in 0..4 {
        let a=corners[side];let b=corners[(side+1)%4];let dir=usize::from(a[0]==b[0]);let mut curve=surface.iso_curve(dir,a[1-dir]).unwrap();assert!(curve.trim(0.1,0.9));if a[dir]>b[dir] {curve.reverse();}
        let ci=source.add_curve_3d(&curve);let edge=source.add_edge(ci as i32,side as i32,((side+1)%4) as i32);
        let pc=NurbsCurve::create(false,1,&[Point::new(a[0],a[1],0.),Point::new(b[0],b[1],0.)]);let ci=source.add_curve_2d(&pc);source.add_pcurve(edge,si,ci as i32,-1);edges.push(BRepRef::new(edge as i32,BRepOrientation::Forward));
    }
    let wi=source.add_wire(&edges);source.add_face(si as i32,&[BRepRef::new(wi as i32,BRepOrientation::Forward)],0.);source
}

#[test]
fn trimmed_curved_face_retains_uvs_analytic_normals_and_original_source() {
    let source=Rc::new(trimmed_wave());let _=source.guid();let before=source.to_proto();let prepared=Prepared::new(source.clone()).unwrap();let samples=crate::cad_samples::face(&prepared,0).unwrap();assert_eq!(samples.len(),prepared.faces[0].vertex.len());assert!(samples.len()>16);
    for sample in samples {
        assert!(sample.uv.iter().all(|v|*v>=0.1-1e-10&&*v<=0.9+1e-10));assert_eq!(sample.surface,0);let v=&prepared.faces[0].vertex[&sample.key];assert_eq!(sample.point,[v.x,v.y,v.z]);assert!((sample.normal.iter().map(|v|v*v).sum::<f64>()-1.).abs()<1e-10);
    }
    assert!(prepared.chains.iter().all(|c|c.as_ref().is_some_and(|c|c.keys.len()>4)));assert_eq!(source.to_proto(),before);
}

#[test]
fn hole_face_keeps_both_wires_and_never_fills_its_interior() {
    let mut source=BRep::create_block_with_hole(8.,6.,2.,1.);source.m_faces=vec![source.m_faces[5].clone()];source.m_shells.clear();source.m_solids.clear();let source=Rc::new(source);let _=source.guid();let before=source.to_proto();let prepared=Prepared::new(source.clone()).unwrap();assert_eq!(source.m_faces[0].wires.len(),2);let mesh=&prepared.faces[0];assert!(!mesh.face.is_empty());
    for tri in mesh.face.values() {let p=tri.iter().map(|key|&mesh.vertex[key]).collect::<Vec<_>>();let x=p.iter().map(|v|v.x).sum::<f64>()/p.len() as f64;let y=p.iter().map(|v|v.y).sum::<f64>()/p.len() as f64;assert!(x*x+y*y>0.99,"A triangle fills the hole at {x},{y}");}
    assert_eq!(prepared.chains.iter().flatten().count(),5);assert_eq!(source.to_proto(),before);
}
