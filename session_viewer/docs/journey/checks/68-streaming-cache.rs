fn editor()->viewer_journey::editor::Editor {
    use viewer_journey::{editor::{Editor,Action},pick_sources::{Target,Part}};
    let mut editor=Editor::default();editor.apply(Action::Close).unwrap();editor.apply(Action::AddCad).unwrap();let cad=editor.scene.objects()[0].id;editor.scene.translate(cad,[-2.,0.,0.]).unwrap();editor.apply(Action::AddSurface).unwrap();
    let source=viewer_journey::range_example::prepared();let owner=editor.scene.insert_stream(source.clone(),session_rust::Xform::translation(2.,0.,0.),12.).unwrap();assert_eq!(source.source.fields.count,4);assert_eq!(source.position(u32::MAX),Some([-0.5,0.5,0.3]));assert!(viewer_journey::scene_archive::save(&editor.scene).unwrap_err().contains("complete source"));
    editor.apply(Action::Fit).unwrap();editor.apply(Action::Isometric).unwrap();editor.apply(Action::Picked(Some(owner))).unwrap();editor.apply(Action::Controls(true)).unwrap();editor.apply(Action::PickedTarget(Some(Target {owner,part:Part::CloudPoint(u32::MAX)}))).unwrap();editor
}
fn make_renderer(device:wgpu::Device,queue:wgpu::Queue,format:wgpu::TextureFormat)->Renderer {
    let editor=editor();let mut renderer=Renderer::new(device,queue,format,&editor.scene);
    let mut math_quality=viewer_journey::math_view::Presentation::default();
    math_quality.drain_matrix(&mut renderer,&editor.scene,&editor.camera.uniform(),[WIDTH,HEIGHT],editor.selected).expect("Native mathematical display must meet the requested physical-pixel quality before capture");
    let math_scene=math_quality.scene(&editor.scene);renderer.cloud_camera(&editor.camera);renderer.set_highlight(&math_scene,editor.selection()).unwrap();assert!(renderer.highlight_bytes()>0);renderer
}
fn draw(renderer:&Renderer,view:&wgpu::TextureView) {
    let editor=editor();renderer.cloud_camera(&editor.camera);renderer.draw(view,&editor.background,&editor.camera.uniform());assert_eq!(renderer.cloud_usage()[0],4);assert_eq!(renderer.cloud_error(),None);assert!(editor.selection().unwrap().describe().contains("4294967295"));
}
fn verify_pixels(pixels:&[u8]) {
    let red=pixels.chunks_exact(4).filter(|p|p[0]>160&&p[1]<80&&p[2]<80).count();let blue=pixels.chunks_exact(4).filter(|p|p[2]>160&&p[0]<80&&p[1]<80).count();let yellow=pixels.chunks_exact(4).filter(|p|p[0]>180&&p[1]>120&&p[2]<80).count();let dark=pixels.chunks_exact(4).filter(|p|p[0]<65&&p[1]<65&&p[2]<65).count();assert!(red>20,"Original coarse page colours: {red}");assert!(blue>20,"Original detailed page colours: {blue}");assert!(yellow>10,"Full-u32 original point marker: {yellow}");assert!(dark>100,"Original CAD and surface boundaries: {dark}");
}
