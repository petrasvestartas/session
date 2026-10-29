fn editor()->viewer_journey::editor::Editor {
    use viewer_journey::{editor::{Editor,Action},commands::{parse,Command}};
    let mut editor=Editor::default();editor.apply(Action::Close).unwrap();
    for line in ["CPlane XY","Polyline Rectangle w-0.8,-0.4,0 w-0.1,0.4,0","Polyline Polygon","Sides 5","w0.5,0,0","w0.8,0,0","Fit"] {
        let action=if let Some(action)=editor.tool_line(line) {action.unwrap()}else{let Command::Editor(action)=parse(line).unwrap() else {panic!("Editor command")};action};editor.apply(action).unwrap();
    }editor
}
fn make_renderer(device:wgpu::Device,queue:wgpu::Queue,format:wgpu::TextureFormat)->Renderer {let editor=editor();{let mut renderer=Renderer::new(device,queue,format,&editor.scene);
    let mut math_quality=viewer_journey::math_view::Presentation::default();
    math_quality.drain_matrix(&mut renderer,&editor.scene,&editor.camera.uniform(),[WIDTH,HEIGHT],editor.selected).expect("Native mathematical display must meet the requested physical-pixel quality before capture");
    let math_scene=math_quality.scene(&editor.scene);
    renderer}}
fn draw(renderer:&Renderer,view:&wgpu::TextureView) {let editor=editor();renderer.draw(view,&editor.background,&editor.camera.uniform());assert_eq!(editor.scene.paths().len(),2);assert_eq!(editor.scene.paths()[0].prepared.source.coordinates().len(),5);assert_eq!(editor.scene.paths()[1].prepared.source.coordinates().len(),6);}
fn verify_pixels(pixels:&[u8]) {
    let ink=pixels.chunks_exact(4).filter(|p|p[0]<220&&p[1]<220&&p[2]<220).count();assert!(ink>500,"Antialiased original closed planar boundaries: {ink}");
    let e=editor();let m=e.camera.uniform();
    for row in e.scene.paths() {let points=row.prepared.source.coordinates();for pair in points.windows(2) {
        let p=std::array::from_fn::<_,3,_>(|i|(pair[0][i]+pair[1][i])*0.5);let clip=|row:usize|p.iter().enumerate().fold(m[12+row] as f64,|sum,(i,v)|sum+v*m[i*4+row] as f64);let w=clip(3);let x=((clip(0)/w+1.)*320.).round() as i32;let y=((1.-clip(1)/w)*240.).round() as i32;
        let covered=(-2..=2).any(|dy|(-2..=2).any(|dx|{let (x,y)=(x+dx,y+dy);if !(0..640).contains(&x)||!(0..480).contains(&y){return false;}let i=(y as usize*640+x as usize)*4;pixels[i..i+3].iter().all(|v|*v<220)}));assert!(covered,"Original planar edge midpoint {p:?}");
    }}
}
