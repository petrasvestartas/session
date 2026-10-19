fn editor()->viewer_journey::editor::Editor{
    use viewer_journey::{editor::{Editor,Action},commands::{parse,Command}};
    let mut e=Editor::default();e.apply(Action::Close).unwrap();
    for line in ["Nurbs Surface 4 Points w-2,-2,0 w2,-2,0 w2,2,0 w-2,2,0","Line w0,-3,0 w0,3,0"]{let Command::Editor(action)=parse(line).unwrap()else{panic!("Editor command")};e.apply(action).unwrap();}
    let target=e.scene.objects()[0].id;let cutter=e.scene.lines()[0].id;e.select_objects([target],false);e.apply(Action::Split).unwrap();e.apply(Action::SplitPick(Some(cutter))).unwrap();e.apply(Action::Split).unwrap();
    for line in ["View Isometric","Select None","Fit"]{let Command::Editor(action)=parse(line).unwrap()else{panic!("Editor command")};e.apply(action).unwrap();}e
}
fn make_renderer(device:wgpu::Device,queue:wgpu::Queue,format:wgpu::TextureFormat)->Renderer{let e=editor();Renderer::new(device,queue,format,&e.scene)}
fn draw(renderer:&Renderer,view:&wgpu::TextureView){let e=editor();renderer.draw(view,&e.background,&e.camera.uniform());assert_eq!(e.scene.objects().len(),1);assert_eq!(e.scene.lines().len(),1);let row=&e.scene.objects()[0];assert!(row.surface.is_none());let source=&row.cad.as_ref().unwrap().source;assert_eq!(source.face_count(),2);assert_eq!(row.mesh.parts.faces.len(),2);assert_eq!(renderer.stats()[0],1);}
fn verify_pixels(pixels:&[u8]){let face=pixels.chunks_exact(4).filter(|p|p[0]<230&&p[1]<230&&p[2]<230).count();assert!(face>1000,"Actual original trimmed face regions: {face}");let black=pixels.chunks_exact(4).filter(|p|p[0]<120&&p[1]<120&&p[2]<120).count();assert!(black>100,"Original trimmed boundary and retained cutter: {black}");}
