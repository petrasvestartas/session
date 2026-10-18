fn editor()->viewer_journey::editor::Editor {
    use viewer_journey::{editor::{Editor,Action},commands::{parse,Command}};
    let mut editor=Editor::default();editor.apply(Action::Close).unwrap();
    for line in ["Line w0,0,1 w1,0,1","Select All","Project To Plane XY","Extend -0.5 1.5","Curve w2,0,1 w2.5,1,2 w3,0,1","Select All","Project To Plane XY","View Isometric","Select None","Fit"] {
        let Command::Editor(action)=parse(line).unwrap() else{panic!("Editor command")};editor.apply(action).unwrap();
    }editor
}
fn make_renderer(device:wgpu::Device,queue:wgpu::Queue,format:wgpu::TextureFormat)->Renderer {let editor=editor();Renderer::new(device,queue,format,&editor.scene)}
fn draw(renderer:&Renderer,view:&wgpu::TextureView) {
    let editor=editor();renderer.draw(view,&editor.background,&editor.camera.uniform());
    assert_eq!(editor.scene.lines().len(),1);assert_eq!(editor.scene.paths().len(),1);
    let source=&editor.scene.lines()[0].prepared.source;assert_eq!([source.start()[0],source.end()[0]],[-0.5,1.5]);assert_eq!([source.start()[2],source.end()[2]],[0.,0.]);
    let viewer_journey::chain::Source::Curve(curve,_)=&editor.scene.paths()[0].prepared.source else{panic!("Original curve")};assert_eq!(curve.cv_count(),3);for i in 0..3{assert_eq!(curve.cv(i).unwrap()[2],0.)}
}
fn verify_pixels(pixels:&[u8]) {
    let ink=pixels.chunks_exact(4).filter(|p|p[0]<200&&p[1]<200&&p[2]<200).count();assert!(ink>100,"Actual original line and projected curve: {ink}");
    let e=editor();for id in [e.scene.lines()[0].id,e.scene.paths()[0].id]{let bounds=e.scene.selected_bounds(id).unwrap();let c=bounds.centre();let p=e.camera.view_projection().transform_point(&session_rust::Point::new(c[0],c[1],c[2]));let(x,y)=(((p[0]+1.)*320.)as i32,((1.-p[1])*240.)as i32);let mut covered=0;for yy in (y-35).max(0)..(y+35).min(480){for xx in (x-45).max(0)..(x+45).min(640){let i=(yy*640+xx)as usize*4;if pixels[i..i+3].iter().all(|v|*v<220){covered+=1}}}assert!(covered>15,"Each original edit has independent visible coverage: {covered}");}
}
