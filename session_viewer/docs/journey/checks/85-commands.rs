fn editor()->viewer_journey::editor::Editor{
    use viewer_journey::{editor::{Editor,Action},commands::{parse,Command}};
    let mut e=Editor::default();e.apply(Action::Close).unwrap();
    for line in ["CPlane XY","View Top","Polyline w-1,-0.5,0 w-0.2,-0.5,0 w0.3,0.2,0","Arrow w-0.8,0.6,0 w0.8,0.6,0","Text timber","Height 0.25","w-0.65,-0.1,0","Select All","Add Group joint","Select None","Extend Distance 0.8"]{
        let action=if let Some(action)=e.tool_line(line){action.unwrap()}else{let Command::Editor(action)=parse(line).unwrap()else{panic!("Editor command")};action};e.apply(action).unwrap();
    }
    let matrix=e.camera.view_projection().m;let p=[0.8,0.6,0.];let clip=|r|p[0]*matrix[r]+p[1]*matrix[4+r]+p[2]*matrix[8+r]+matrix[12+r];let screen=[(clip(0)/clip(3)) as f32,(clip(1)/clip(3)) as f32];e.apply(Action::ToolScreen{screen,size:[640.,480.],commit:false}).unwrap();e
}
fn make_renderer(device:wgpu::Device,queue:wgpu::Queue,format:wgpu::TextureFormat)->Renderer{
    let e=editor();let mut renderer=Renderer::new(device,queue,format,&e.scene);
    let mut math_quality=viewer_journey::math_view::Presentation::default();
    math_quality.drain_matrix(&mut renderer,&e.scene,&e.camera.uniform(),[WIDTH,HEIGHT],e.selected).expect("Native mathematical display must meet the requested physical-pixel quality before capture");
    let math_scene=math_quality.scene(&e.scene);
    renderer.replace_text_fonts(vec![include_bytes!("../assets/text/NotoSans-Regular.ttf").to_vec(),include_bytes!("../assets/text/NotoSansSymbols2-Regular.ttf").to_vec(),include_bytes!("../assets/text/NotoSansSymbols-Regular.ttf").to_vec()]).unwrap();renderer.set_guides(&e.guide_overlay()).unwrap();renderer
}
fn draw(renderer:&Renderer,view:&wgpu::TextureView){let e=editor();renderer.draw(view,&e.background,&e.camera.uniform());assert_eq!(e.scene.paths().len(),2);assert_eq!(e.scene.texts().len(),1);assert_eq!(e.scene.groups.len(),1);assert_eq!(e.scene.texts()[0].prepared.source.label.text,"timber");let overlay=e.guide_overlay();assert!(!overlay.segments.is_empty());assert_eq!(overlay.label.as_ref().unwrap().1,"+0.800");assert!(renderer.guide_bytes()>0);assert_eq!(renderer.text_error(),None);assert_eq!(renderer.text_stats().unwrap().missing_glyphs,0);}
fn verify_pixels(pixels:&[u8]){let black=pixels.chunks_exact(4).filter(|p|p[0]<120&&p[1]<120&&p[2]<120).count();assert!(black>200,"Actual original headed curves and timber text: {black}");let red=pixels.chunks_exact(4).filter(|p|p[0]>180&&p[1]<160&&p[2]<160&&p[0].saturating_sub(p[1])>60).count();assert!(red>60,"Actual red dashed extension and endpoint: {red}");}
