#[test]
fn derived_contact_rgba_survives_display_and_packing_without_changing_the_source(){
 let source=session_rust::Mesh::from_vertices_and_faces(vec![session_rust::Point::new(-1.,-1.,0.),session_rust::Point::new(1.,-1.,0.),session_rust::Point::new(0.,1.,0.)],vec![vec![0,1,2]]);
 let before=source.to_proto();let ordinary=crate::mesh::Mesh::from_kernel(&source).unwrap();assert_eq!(ordinary.alphas(),&[1.;3]);
 let mut render=source.to_render();let positions=render.vertices.iter().map(|v|v.position).collect::<Vec<_>>();
 for vertex in &mut render.vertices{vertex.color=[0.9,0.1,0.1,1./255.];}
 let display=crate::mesh::Mesh::from_render(render).unwrap();assert_eq!(display.alphas(),&[1./255.;3]);
 assert_eq!(display.vertices().iter().map(|v|[v[0],v[1],v[2]]).collect::<Vec<_>>(),positions);
 let bytes=crate::packed_vertex::bytes(&display);assert_eq!(bytes.len(),60);
 for vertex in bytes.chunks_exact(20){assert_eq!(&vertex[16..20],&[230,26,26,1],"current derived contact RGBA");}assert_eq!(source.to_proto(),before);
}
#[test]
fn alpha_columns_are_validated_and_default_display_vertices_stay_opaque(){
 let mesh=crate::mesh::Mesh::new(vec![[0.;6]],vec![]).unwrap();assert_eq!(mesh.alphas(),&[1.]);assert_eq!(crate::packed_vertex::bytes(&mesh)[19],255);
 assert!(mesh.clone().with_alphas(vec![]).is_err());assert!(mesh.clone().with_alphas(vec![f32::NAN]).is_err());assert!(mesh.clone().with_alphas(vec![f32::INFINITY]).is_err());
 let retained=mesh.with_alphas(vec![1./255.]).unwrap();assert_eq!(crate::packed_vertex::bytes(&retained)[19],1);
}
