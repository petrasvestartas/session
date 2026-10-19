fn editor()->viewer_journey::editor::Editor {
    use viewer_journey::{editor::{Editor,Action},commands::{parse,Command}};
    let mut editor=Editor::default();editor.apply(Action::Close).unwrap();
    for line in ["Line w-2,0,0 w2,0,0","Line w0,-2,0 w0,2,0"]{let Command::Editor(action)=parse(line).unwrap()else{panic!("Editor command")};editor.apply(action).unwrap();}
    let ids=editor.scene.lines().iter().map(|row|row.id).collect::<Vec<_>>();editor.select_objects([ids[0]],false);editor.apply(Action::Split).unwrap();editor.apply(Action::SplitPick(Some(ids[1]))).unwrap();editor.apply(Action::Split).unwrap();
    for line in ["View Isometric","Select None","Fit"]{let Command::Editor(action)=parse(line).unwrap()else{panic!("Editor command")};editor.apply(action).unwrap();}editor.select_objects([ids[0]],false);editor
}
fn make_renderer(device:wgpu::Device,queue:wgpu::Queue,format:wgpu::TextureFormat)->Renderer{let e=editor();let mut renderer=Renderer::new(device,queue,format,&e.scene);renderer.select_objects(&e.scene,&e.selected_objects()).unwrap();renderer}
fn draw(renderer:&Renderer,view:&wgpu::TextureView){let e=editor();renderer.draw(view,&e.background,&e.camera.uniform());assert_eq!(e.scene.lines().len(),3);assert!(e.split.is_none());let pieces=e.scene.lines().iter().filter(|r|r.prepared.source.start()[1]==0.&&r.prepared.source.end()[1]==0.).collect::<Vec<_>>();assert_eq!(pieces.len(),2);assert_eq!(pieces[0].prepared.source.end()[0],0.);assert_eq!(pieces[1].prepared.source.start()[0],0.);assert!(pieces.iter().all(|r|r.prepared.source.name.contains("(part")));}
fn verify_pixels(pixels:&[u8]){let black=pixels.chunks_exact(4).filter(|p|p[0]<150&&p[1]<150&&p[2]<150).count();assert!(black>100,"Retained cutter and original sibling piece: {black}");let selected=pixels.chunks_exact(4).filter(|p|p[0]>180&&p[1]>120&&p[2]<100).count();assert!(selected>20,"Selected first original split piece: {selected}");}
