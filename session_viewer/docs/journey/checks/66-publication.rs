fn editor()->viewer_journey::editor::Editor {
    use viewer_journey::{editor::{Editor,Action},pick_sources::{Target,Part}};
    let mut editor=Editor::default();editor.apply(Action::Close).unwrap();editor.apply(Action::AddCad).unwrap();let cad=editor.scene.objects()[0].id;editor.scene.translate(cad,[-2.,0.,0.]).unwrap();
    editor.apply(Action::AddSurface).unwrap();editor.apply(Action::AddCloud).unwrap();let cloud=editor.scene.clouds()[0].id;editor.scene.translate(cloud,[2.,0.,0.]).unwrap();
    let original=editor.scene.clouds()[0].prepared.source.to_proto();let bytes=viewer_journey::scene_archive::save(&editor.scene).unwrap();viewer_journey::scene_document::open(&mut editor.scene,&bytes).unwrap();
    assert_eq!(editor.scene.clouds()[0].prepared.source.to_proto(),original);assert_eq!(editor.scene.clouds()[0].model.m[12],2.);assert_eq!(editor.scene.objects().iter().filter(|row|row.cad.is_some()).count(),1);assert_eq!(editor.scene.objects().iter().filter(|row|row.surface.is_some()).count(),1);
    editor.apply(Action::Fit).unwrap();editor.apply(Action::Isometric).unwrap();let owner=editor.scene.clouds()[0].id;editor.apply(Action::Picked(Some(owner))).unwrap();editor.apply(Action::Controls(true)).unwrap();editor.apply(Action::PickedTarget(Some(Target {owner,part:Part::CloudPoint(840)}))).unwrap();editor
}
fn make_renderer(device:wgpu::Device,queue:wgpu::Queue,format:wgpu::TextureFormat)->Renderer {
    let editor=editor();let mut renderer=Renderer::new(device,queue,format,&editor.scene);renderer.cloud_camera(&editor.camera);renderer.set_highlight(&editor.scene,editor.selection()).unwrap();assert!(renderer.highlight_bytes()>0);renderer
}
fn draw(renderer:&Renderer,view:&wgpu::TextureView) {
    let editor=editor();renderer.cloud_camera(&editor.camera);renderer.draw(view,&editor.background,&editor.camera.uniform());assert_eq!(renderer.cloud_usage()[0],1681);assert_eq!(renderer.cloud_error(),None);assert!(editor.selection().unwrap().describe().contains("original point 840"));
}
fn verify_pixels(pixels:&[u8]) {
    let colored=pixels.chunks_exact(4).filter(|p|p[2]>p[0].saturating_add(20)&&p[2]>p[1].saturating_add(10)).count();
    let yellow=pixels.chunks_exact(4).filter(|p|p[0]>180&&p[1]>120&&p[2]<80).count();
    let dark=pixels.chunks_exact(4).filter(|p|p[0]<65&&p[1]<65&&p[2]<65).count();
    assert!(colored>500,"Reopened original CAD/surface/cloud colours: {colored}");assert!(yellow>10,"Local original point marker after reopening: {yellow}");assert!(dark>100,"Original black boundaries remain visible: {dark}");
    let editor=editor();let view=editor.camera.view_projection();
    let boundary_visible=|row:&viewer_journey::scene::Object| {let mut visible=std::collections::HashSet::new();for (edge,pair) in row.mesh.parts.edges.iter().zip(&row.mesh.parts.endpoints) {
        let middle=std::array::from_fn::<_,3,_>(|i|(pair[0][i]+pair[1][i])*0.5);let world=row.model.transform_point(&session_rust::Point::new(middle[0],middle[1],middle[2]));let p=view.transform_point(&world);let x=((p[0]+1.)*320.).round() as i32;let y=((1.-p[1])*240.).round() as i32;
        if (-2..=2).any(|dy|(-2..=2).any(|dx| {let (x,y)=(x+dx,y+dy);if !(0..640).contains(&x)||!(0..480).contains(&y) {return false;}let at=(y as usize*640+x as usize)*4;pixels[at..at+3].iter().all(|v|*v<65)})) {visible.insert(*edge);}
    }visible.len()};
    let cad=editor.scene.objects().iter().find(|row|row.cad.is_some()).unwrap();let surface=editor.scene.objects().iter().find(|row|row.surface.is_some()).unwrap();
    assert!(boundary_visible(cad)>=3,"Black boundaries belong to the reopened CAD body");assert!(boundary_visible(surface)>=1,"Black boundaries belong to the reopened parametric surface");
}
