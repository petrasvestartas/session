    assert!(changed>50);renderer.request_pick([-0.0625,0.],identity()).unwrap();assert_eq!(finish(&mut renderer).unwrap()[0],id.value());renderer.request_pick([0.125,0.],identity()).unwrap();assert!(finish(&mut renderer).is_none());
}

#[test]
fn actual_selected_silhouette_has_one_union_no_internal_diagonal_and_keeps_source_and_picks() {
    let mut scene=Scene::demo();scene.clear();let mesh=|x:f64|{let mut source=session_rust::Mesh::create_box(0.6,0.8,0.1);source.set_objectcolor(session_rust::Color::new(0.5,0.5,0.5,1.));let mut prepared=PreparedMesh::new(source).unwrap();prepared.model=session_rust::Xform::translation(x,0.,0.5);prepared};let a=scene.insert(mesh(-0.3)).unwrap();let b=scene.insert(mesh(0.3)).unwrap();let sources=scene.objects().iter().map(|row|row.geometry().unwrap().to_proto()).collect::<Vec<_>>();let mut renderer=renderer(&scene);let geometry=renderer.stats();renderer.select_objects(&scene,&[a,b]).unwrap();let first=colour_pixels(&mut renderer);let alpha=renderer.outline_alpha();assert_eq!(alpha.len(),320*240);assert!(alpha.iter().filter(|v|**v>0).count()>300);for y in 90..150 {for x in 90..230 {assert_eq!(alpha[y*320+x],0,"The combined coverage has no outline along the shared object join or tessellation diagonals");}}
    let stats=renderer.outline_usage();assert_eq!(colour_pixels(&mut renderer),first);assert_eq!(renderer.outline_alpha(),alpha);assert_eq!(renderer.outline_usage()[1],stats[1]);assert!(renderer.outline_usage()[2]>stats[2]);assert_eq!(&renderer.stats()[..2],&geometry[..2]);assert_eq!(scene.objects().iter().map(|row|row.geometry().unwrap().to_proto()).collect::<Vec<_>>(),sources);renderer.request_pick([-0.3,0.],identity()).unwrap();assert_eq!(finish(&mut renderer).unwrap()[0],a.value());renderer.request_pick([0.3,0.],identity()).unwrap();assert_eq!(finish(&mut renderer).unwrap()[0],b.value());
    renderer.select_objects(&scene,&[]).unwrap();colour_pixels(&mut renderer);assert!(renderer.outline_alpha().is_empty());assert_eq!(renderer.outline_usage()[0],0);
}

#[test]
fn actual_surface_outline_culls_occluded_selection_and_selected_edge_ink_cannot_narrow_its_alpha() {
    use crate::pick_sources::{Target,Part};
    let mut scene=Scene::demo();scene.clear();let back=scene.insert(triangle(0.6)).unwrap();let front=scene.insert(triangle(0.2)).unwrap();let mut renderer=renderer(&scene);renderer.select_objects(&scene,&[back]).unwrap();colour_pixels(&mut renderer);assert!(renderer.outline_alpha().iter().all(|v|*v==0),"A selected surface wholly behind the current opaque depth adds no silhouette");renderer.select_objects(&scene,&[front]).unwrap();let plain=colour_pixels(&mut renderer);let alpha=renderer.outline_alpha();assert!(alpha.iter().any(|v|*v>128));let edge=scene.objects().iter().find(|row|row.id==front).unwrap().mesh.parts.edges[0];renderer.set_highlight(&scene,Some(Target {owner:front,part:Part::MeshEdge(edge.0,edge.1)})).unwrap();let inked=colour_pixels(&mut renderer);assert_eq!(renderer.outline_alpha(),alpha,"Source edge highlighting cannot change the actual outline width");let mut opaque=0;for ((a,b),coverage) in plain.chunks_exact(4).zip(inked.chunks_exact(4)).zip(&alpha) {if *coverage>=250&&a[..3].iter().all(|c|*c<8) {opaque+=1;assert!(b[..3].iter().all(|c|*c<12),"Yellow source edge ink cannot paint over black outline pixels");}}assert!(opaque>100);scene.clear();renderer.set_scene(&scene,None);colour_pixels(&mut renderer);assert_eq!(renderer.outline_usage()[0],0);
}

#[test]
fn actual_curved_and_flat_silhouettes_keep_the_same_css_width_at_two_densities() {
    let mut widths=Vec::new();
    for curved in [false,true] {
        let mut scene=Scene::demo();scene.clear();let id=if curved {scene.insert_cad(Rc::new(session_rust::BRep::create_cylinder(0.4,0.8))).unwrap()} else {let mut prepared=PreparedMesh::new(session_rust::Mesh::create_box(0.8,0.8,0.2)).unwrap();prepared.model=session_rust::Xform::translation(0.,0.,0.5);scene.insert(prepared).unwrap()};let mut renderer=renderer(&scene);renderer.select_objects(&scene,&[id]).unwrap();let geometry=renderer.stats();let mut css=Vec::new();
        for density in [1.,2.] {renderer.set_density(density);colour_pixels(&mut renderer);let alpha=renderer.outline_alpha();assert!(alpha.iter().any(|v|*v==255));let row=&alpha[120*320..121*320];let width=row[160..].iter().map(|v|*v as f64/255.).sum::<f64>()/density as f64;assert!((0.7..1.7).contains(&width),"Actual outer silhouette CSS coverage remains bounded: {width}");css.push(width);assert_eq!(&renderer.stats()[..2],&geometry[..2]);}
        assert!((css[0]-css[1]).abs()<0.5,"DPI keeps the outline in CSS units: {css:?}");widths.push(css);
    }
    for i in 0..2 {assert!((widths[0][i]-widths[1][i]).abs()<0.25,"A smooth cylinder and flat box use one coverage width: {widths:?}");}
}

#[test]
fn actual_selected_bore_keeps_its_open_center_and_black_inner_contour() {
    let source=Rc::new(session_rust::BRep::create_block_with_hole(1.2,1.2,0.4,0.25));
    let _=source.guid();let original=source.to_proto();let mut scene=Scene::demo();scene.clear();
    let id=scene.insert_cad(source.clone()).unwrap();let mut renderer=renderer(&scene);
    renderer.select_objects(&scene,&[id]).unwrap();let pixels=colour_pixels(&mut renderer);
    let alpha=renderer.outline_alpha();
    for y in 112..128 {for x in 152..168 {assert_eq!(alpha[y*320+x],0,"The bore center stays open");}}
    let mut inner=0;
    for y in 80..160 {for x in 110..210 {
        let i=y*320+x;
        if alpha[i]>=250 {inner+=1;assert!(pixels[i*4..i*4+3].iter().all(|c|*c<12),"The actual inner silhouette is black");}
    }}
    assert!(inner>100,"The real bore boundary has an inner outline: {inner}");
    renderer.request_pick([0.,0.],identity()).unwrap();assert!(finish(&mut renderer).is_none());
    renderer.request_pick([0.4,0.],identity()).unwrap();assert_eq!(finish(&mut renderer).unwrap()[0],id.value());
    assert_eq!(source.to_proto(),original,"Coverage cannot change original topology");
}
