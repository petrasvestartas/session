use std::rc::Rc;
use session_rust::BRep;
use crate::cad_boundary::Prepared;

#[test]
fn shared_box_edges_follow_original_face_samples_and_preserve_brep_source() {
    let source=Rc::new(BRep::create_box(4.,3.,2.));let _=source.guid();let before=source.to_proto();let prepared=Prepared::new(source.clone()).unwrap();
    assert!(Rc::ptr_eq(&prepared.source,&source));assert_eq!(prepared.faces.len(),6);assert_eq!(prepared.chains.len(),12);
    for (edge,chain) in prepared.chains.iter().enumerate() {
        let chain=chain.as_ref().unwrap();assert_eq!(chain.edge,edge);assert!(chain.other.is_some());let points=prepared.points(edge).unwrap();assert!(points.len()>=2);
        let other=&prepared.faces[chain.other.unwrap()];
        for p in points {assert!(other.vertex.values().any(|v|[v.x,v.y,v.z].iter().zip(p).all(|(a,b)|(a-b).abs()<1e-10)),"Shared edge {edge} follows both incident faces");}
    }
    assert_eq!(source.to_proto(),before);let weak=Rc::downgrade(&source);drop(source);assert!(weak.upgrade().is_some());drop(prepared);assert!(weak.upgrade().is_none());
}

#[test]
fn circular_boundary_samples_are_shared_by_cap_and_wall() {
    let source=Rc::new(BRep::create_cylinder(2.,3.));let _=source.guid();let before=source.to_proto();let prepared=Prepared::new(source.clone()).unwrap();
    for edge in [0,1] {
        let chain=prepared.chains[edge].as_ref().unwrap();let other=&prepared.faces[chain.other.unwrap()];let points=prepared.points(edge).unwrap();assert!(points.len()>16);
        for p in points {assert!(other.vertex.values().any(|v|[v.x,v.y,v.z].iter().zip(p).all(|(a,b)|(a-b).abs()<1e-9)),"Circle {edge} follows both incident faces");}
    }
    assert_eq!(source.to_proto(),before);
}

#[test]
fn invalid_topology_and_control_storage_fail_before_meshing() {
    let mut source=BRep::create_box(4.,3.,2.);source.m_faces[0].surface_index=-1;assert!(Prepared::new(Rc::new(source)).is_err());
    let mut source=BRep::create_box(4.,3.,2.);source.m_surfaces[0].m_cv.pop();assert!(Prepared::new(Rc::new(source)).is_err());
    let mut source=BRep::create_box(4.,3.,2.);source.m_curves_3d[0].m_cv.pop();assert!(Prepared::new(Rc::new(source)).is_err());
    let mut source=BRep::create_box(4.,3.,2.);source.m_curves_2d[0].m_nurbsknot.reverse();source.m_curves_2d[0].m_nurbsknot[0]=f64::NAN;assert!(Prepared::new(Rc::new(source)).is_err());
}

fn adjacent()->BRep {
use session_rust::brep::BRepOrientation;
use session_rust::brep::BRepRef;
use session_rust::BRep;
use session_rust::NurbsCurve;
use session_rust::NurbsSurface;
use session_rust::Point;

let mut b = BRep::new();
let mut surfaces = Vec::new();

for face in 0..2 {
    let mut points = Vec::new();

    for i in 0..3 {
        for j in 0..2 {
            let z = if i != 1 {
                0.0
            } else if j == 0 || face == 0 {
                0.5
            } else {
                4.0
            };
            points.push(Point::new(
                i as f64 * 0.5,
                j as f64 * if face == 0 { 1.0 } else { -1.0 },
                z,
            ));
        }
    }

    let surface = NurbsSurface::create(false, false, 2, 1, 3, 2, &points).unwrap();
    let si = b.add_surface(&surface);
    let corners = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
    let mut vertices = Vec::new();

    for corner in &corners {
        vertices.push(b.add_vertex(&surface.point_at(corner[0], corner[1]).unwrap(), 0.0));
    }

    let mut edges = Vec::new();

    for side in 0..4 {
        let a = corners[side];
        let z = corners[(side + 1) % 4];
        let mut edge = 0;

        if side != 0 || face != 1 {
            let dir = if a[0] != z[0] { 0 } else { 1 };
            let mut curve = surface.iso_curve(dir, a[1 - dir]).unwrap();

            if a[dir] > z[dir] {
                curve.reverse();
            }

            let ci = b.add_curve_3d(&curve);
            edge = b.add_edge(
                ci as i32,
                vertices[side] as i32,
                vertices[(side + 1) % 4] as i32,
            );
        }

        let pc = NurbsCurve::create(
            false,
            1,
            &[Point::new(a[0], a[1], 0.0), Point::new(z[0], z[1], 0.0)],
        );
        let ci = b.add_curve_2d(&pc);
        b.add_pcurve(edge, si, ci as i32, -1);
        edges.push(BRepRef::new(edge as i32, BRepOrientation::Forward));
    }

    let wi = b.add_wire(&edges);
    b.add_face(
        si as i32,
        &[BRepRef::new(wi as i32, BRepOrientation::Forward)],
        1e-8,
    );
    surfaces.push(surface);
}

b
}

#[test]
fn unequal_independent_grids_become_a_coherent_curved_shared_boundary() {
    use session_rust::remesh_nurbssurface_grid::RemeshNurbsSurfaceGrid;
    let source=Rc::new(adjacent());let _=source.guid();let before=source.to_proto();
    let points=|mesh:&session_rust::Mesh| {let mut p=mesh.vertex.values().filter(|v|v.attributes.get("v")==Some(&0.)).map(|v|[v.x,v.y,v.z]).collect::<Vec<_>>();p.sort_by(|a,b|a[0].total_cmp(&b[0]));p.dedup();p};
    let a=RemeshNurbsSurfaceGrid::from_u_v_q(&source.m_surfaces[0],0,0,20.,0.005);let b=RemeshNurbsSurfaceGrid::from_u_v_q(&source.m_surfaces[1],0,0,20.,0.005);assert_ne!(points(&a).len(),points(&b).len());
    let prepared=Prepared::new(source.clone()).unwrap();let a=points(&prepared.faces[0]);let b=points(&prepared.faces[1]);assert_eq!(a,b);assert!(a.len()>7);assert_eq!(source.to_proto(),before);
    let normal=|surface:&session_rust::NurbsSurface|surface.normal_at(0.5,0.);assert_ne!(normal(&source.m_surfaces[0]).to_f32(),normal(&source.m_surfaces[1]).to_f32());
}
