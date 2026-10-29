fn editor()->viewer_journey::editor::Editor {
    use viewer_journey::{editor::{Editor,Action},commands::{parse,Command}};
    let mut editor=Editor::default();editor.apply(Action::Close).unwrap();
    for line in ["Line w0,1,0 w2,1,1","Line w2,0,0 w2,2,2","Line w2,0,0 w0,0,0","Line w1,0,0 w1,2,1","Line w0,2,0 w2,2,2","Line w0,0,0 w0,2,0","Select All","Nurbs Surface Network","Nurbs Surface 4 Points w3,0,0 w4,0,0 w4,1,0.3 w3,1,0","View Isometric","Select None","Fit"] {
        let Command::Editor(action)=parse(line).unwrap() else{panic!("Editor command")};editor.apply(action).unwrap();
    }editor
}
fn make_renderer(device:wgpu::Device,queue:wgpu::Queue,format:wgpu::TextureFormat)->Renderer {let editor=editor();{let mut renderer=Renderer::new(device,queue,format,&editor.scene);assert_eq!(renderer.stats()[0],2);
    let mut math_quality=viewer_journey::math_view::Presentation::default();
    math_quality.drain_matrix(&mut renderer,&editor.scene,&editor.camera.uniform(),[WIDTH,HEIGHT],editor.selected).expect("Native mathematical display must meet the requested physical-pixel quality before capture");
    let math_scene=math_quality.scene(&editor.scene);
    renderer}}
fn draw(renderer:&Renderer,view:&wgpu::TextureView) {let editor=editor();let uploads=renderer.stats()[0];renderer.draw(view,&editor.background,&editor.camera.uniform());assert_eq!(renderer.stats()[0],uploads);assert_eq!(editor.scene.lines().len(),6);let rows=editor.scene.objects();assert_eq!(rows.len(),2);assert!(rows.iter().all(|row|row.surface.as_ref().unwrap().source.is_valid()));assert_eq!(rows[1].surface.as_ref().unwrap().source.degree(0),1);assert_eq!(renderer.usage()[0],2);assert_eq!(renderer.usage()[2],2);}
fn verify_pixels(pixels:&[u8]) {let ink=pixels.chunks_exact(4).filter(|p|p[0]<220&&p[1]<220&&p[2]<220).count();assert!(ink>1000,"Actual network and corner patch: {ink}");let black=pixels.chunks_exact(4).filter(|p|p[0]<100&&p[1]<100&&p[2]<100).count();assert!(black>75,"Actual original network rulings and domain boundaries: {black}");let e=editor();for row in e.scene.objects(){let bounds=e.scene.selected_bounds(row.id).unwrap();let c=bounds.centre();let p=e.camera.view_projection().transform_point(&session_rust::Point::new(c[0],c[1],c[2]));let(x,y)=(((p[0]+1.)*320.)as i32,((1.-p[1])*240.)as i32);let mut covered=0;for yy in (y-25).max(0)..(y+25).min(480){for xx in (x-25).max(0)..(x+25).min(640){let i=(yy*640+xx)as usize*4;if pixels[i..i+3].iter().all(|v|*v<250){covered+=1}}}assert!(covered>100,"Each original surface has independent visible coverage: {covered}");}}
