use crate::cap_gpu_tests::{at,matrix,pixels,renderer,wait};
use std::rc::Rc;

fn source(widths:Vec<f64>)->session_rust::Mesh {
    let mut mesh=session_rust::Mesh::from_vertices_and_faces(vec![session_rust::Point::new(-0.8,-0.8,-0.2),session_rust::Point::new(0.8,-0.8,-0.2),session_rust::Point::new(0.,0.8,-0.2)],vec![vec![0,1,2]]);
    mesh.set_linecolors(vec![],widths);mesh.set_objectcolor(session_rust::Color::new(0.8,0.2,0.1,1.));mesh
}
#[test]
fn only_one_zero_width_classifies_print_and_display_retains_it() {
    for (widths,expected) in [(vec![],false),(vec![0.],true),(vec![-0.],true),(vec![0.,0.],false),(vec![1.],false)] {
        let source=source(widths);let before=source.to_proto();let display=crate::mesh::Mesh::from_kernel(&source).unwrap();
        assert_eq!(display.print_fill(),expected);assert!(!display.print_text());assert_eq!(display.clone().print_fill(),expected);assert_eq!(source.to_proto(),before);
    }
    let mut text=source(vec![0.]);text.name="text".into();assert!(crate::mesh::Mesh::from_kernel(&text).unwrap().print_text());
}
#[test]
fn printed_fill_keeps_ink_and_fresh_queries_unoccluded_without_physical_triangles() {
    let mut scene=crate::scene::Scene::empty();let fill=scene.insert(crate::prepared::PreparedMesh::new(source(vec![0.])).unwrap()).unwrap();
    let mut line=session_rust::Line::new(-0.6,0.,-0.6,0.6,0.,-0.6);line.width=5.;
    let ink=scene.insert_line(crate::stroke::PreparedLine::new(Rc::new(line)).unwrap()).unwrap();
    let original=scene.objects()[0].geometry().unwrap().to_proto();let mut gpu=renderer(&scene);
    for samples in [1,4,1] {
        gpu.set_samples(samples);gpu.set_lighting(true,true);gpu.set_outlines(true);
        let image=pixels(&gpu);assert!(at(&image,160,120)[..3].iter().all(|v|*v<40),"Ink behind a printed fill remains visible");
        assert_eq!(gpu.tile_usage()[3],0,"Printed triangles do not enter physical visibility tables");
        for (position,owner) in [([0.,0.],ink),([0.,-0.3],fill)] {
            gpu.request_pick(position,matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().map(|pixel|pixel[0]),Some(owner.value()));
        }
    }
    assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);
}
