fn editor()->viewer_journey::editor::Editor {
    use viewer_journey::{editor::{Editor,Action},commands::{parse,Command}};
    let mut editor=Editor::default();editor.apply(Action::Close).unwrap();
    for line in ["CPlane XY","Cylinder w-2,0,0 0.45 1","Cone w-0.7,0,0 0.45 1","Sphere w0.7,0,0.5 0.5","Torus w2,0,0.5 0.5 0.15","View Isometric","Fit"] {
        let Command::Editor(action)=parse(line).unwrap() else{panic!("Editor command")};editor.apply(action).unwrap();
    }editor
}
fn make_renderer(device:wgpu::Device,queue:wgpu::Queue,format:wgpu::TextureFormat)->Renderer {let editor=editor();Renderer::new(device,queue,format,&editor.scene)}
fn draw(renderer:&Renderer,view:&wgpu::TextureView) {let editor=editor();renderer.draw(view,&editor.background,&editor.camera.uniform());assert_eq!(editor.scene.objects().len(),4);assert_eq!(editor.scene.objects().iter().map(|row|row.cad.as_ref().unwrap().source.face_count()).collect::<Vec<_>>(),vec![3,2,1,1]);assert!(renderer.stats()[0]>=4);}
fn verify_pixels(pixels:&[u8]) {
    let ink=pixels.chunks_exact(4).filter(|p|p[0]<220&&p[1]<220&&p[2]<220).count();assert!(ink>1000,"Actual original round faces and boundaries: {ink}");
    let black=pixels.chunks_exact(4).filter(|p|p[0]<100&&p[1]<100&&p[2]<100).count();assert!(black>75,"Actual one-pixel round boundaries: {black}");
    let m=editor().camera.uniform();
    for p in [[-2.,0.,0.5],[-0.7,0.,0.33],[0.7,0.,0.5],[2.,0.5,0.5]] {
        let clip=|row:usize|p.iter().enumerate().fold(m[12+row] as f64,|sum,(i,v)|sum+v*m[i*4+row] as f64);let w=clip(3);let x=((clip(0)/w+1.)*320.).round() as i32;let y=((1.-clip(1)/w)*240.).round() as i32;
        let covered=(-8..=8).any(|dy|(-8..=8).any(|dx|{let(x,y)=(x+dx,y+dy);if !(0..640).contains(&x)||!(0..480).contains(&y){return false;}let i=(y as usize*640+x as usize)*4;pixels[i..i+3].iter().all(|v|*v<250)}));assert!(covered,"Independent original round object around {p:?}");
    }
}
