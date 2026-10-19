use std::rc::Rc;
use session_rust::{BRep,NurbsSurface,Point};

#[test]
fn seam_keeps_both_original_pcurves_on_the_same_face() {
    let source=Rc::new(BRep::create_cylinder(2.,3.));let _=source.guid();let before=source.to_proto();let prepared=crate::cad_boundary::Prepared::new(source.clone()).unwrap();let uses=crate::cad_seams::uses(&prepared,2).unwrap();assert_eq!(uses.len(),2);assert_eq!(uses[0].face,uses[1].face);assert_ne!(uses[0].pcurve,uses[1].pcurve);assert_ne!(uses[0].orientation,uses[1].orientation);
    let mesh=&prepared.faces[uses[0].face];let a=uses[0].keys.as_ref().unwrap();let b=uses[1].keys.as_ref().unwrap();assert!(a.len()>=2&&b.len()>=2);
    for key in a {let v=&mesh.vertex[key];assert!(b.iter().any(|key| {let p=&mesh.vertex[key];(v.x-p.x).abs()<1e-9&&(v.y-p.y).abs()<1e-9&&(v.z-p.z).abs()<1e-9}));}
    assert_eq!(source.to_proto(),before);
}

#[test]
fn periodic_surface_grid_keeps_closed_borders_without_drawing_a_seam() {
    let points=(0..10).flat_map(|i| {let angle=std::f64::consts::TAU*(i%8) as f64/8.;(0..2).map(move|j|Point::new(angle.cos(),angle.sin(),j as f64))}).collect::<Vec<_>>();let surface=NurbsSurface::create(true,false,2,1,10,2,&points).unwrap();assert!(surface.is_periodic(0));let before=surface.to_proto();let grid=crate::surface::Grid::new(&surface).unwrap();assert_eq!(grid.boundaries.iter().map(|b|b.edge).collect::<Vec<_>>(),vec![2,3]);
    let [nu,nv]=grid.steps;for j in 0..=nv {let a=&grid.samples[j];let b=&grid.samples[nu*(nv+1)+j];assert!(a.point.iter().zip(b.point).all(|(a,b)|(a-b).abs()<1e-10));assert!(a.normal.iter().zip(b.normal).all(|(a,b)|(a-b).abs()<1e-9));assert_ne!(a.uv[0],b.uv[0]);}
    assert_eq!(surface.to_proto(),before);
}
