fn editor()->viewer_journey::editor::Editor {
    use viewer_journey::{editor::{Editor,Action},commands::{parse,Command}};
    let mut editor=Editor::default();editor.apply(Action::Close).unwrap();
    for line in ["CPlane XY","Line w-0.75,-0.4,0 w0.75,-0.4,0","Polyline w-0.6,0,0 w0,0.6,0 w0.6,0,0","Point w0,0.3,0","Fit"] {
        let Command::Editor(action)=parse(line).unwrap() else {panic!("Editor command")};editor.apply(action).unwrap();
    }editor
}
fn make_renderer(device:wgpu::Device,queue:wgpu::Queue,format:wgpu::TextureFormat)->Renderer {let editor=editor();{let mut renderer=Renderer::new(device,queue,format,&editor.scene);
    let mut math_quality=viewer_journey::math_view::Presentation::default();
    math_quality.drain_matrix(&mut renderer,&editor.scene,&editor.camera.uniform(),[WIDTH,HEIGHT],editor.selected).expect("Native mathematical display must meet the requested physical-pixel quality before capture");
    let math_scene=math_quality.scene(&editor.scene);
    renderer}}
fn draw(renderer:&Renderer,view:&wgpu::TextureView) {let editor=editor();renderer.draw(view,&editor.background,&editor.camera.uniform());assert_eq!(editor.scene.lines().len(),1);assert_eq!(editor.scene.paths().len(),1);assert_eq!(editor.scene.points().len(),1);}
fn verify_pixels(pixels:&[u8]) {
    let ink=pixels.chunks_exact(4).filter(|p|p[0]<220&&p[1]<220&&p[2]<220).count();
    assert!(ink>500,"Antialiased original one-pixel drawing strokes: {ink}");
    let m=editor().camera.uniform();
    for p in [[0.,-0.4,0.],[-0.3,0.3,0.],[0.3,0.3,0.],[0.,0.3,0.]] {
        let clip=|row:usize|p.iter().enumerate().fold(m[12+row] as f64,|sum,(i,v)|sum+v*m[i*4+row] as f64);
        let w=clip(3);let x=((clip(0)/w+1.)*320.).round() as i32;let y=((1.-clip(1)/w)*240.).round() as i32;
        let covered=(-2..=2).any(|dy|(-2..=2).any(|dx|{let (x,y)=(x+dx,y+dy);if !(0..640).contains(&x)||!(0..480).contains(&y){return false;}let i=(y as usize*640+x as usize)*4;pixels[i..i+3].iter().all(|v|*v<220)}));
        assert!(covered,"Original drawing coverage around {p:?}");
    }
}
