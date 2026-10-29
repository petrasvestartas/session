fn editor()->viewer_journey::editor::Editor{
    use viewer_journey::{editor::{Editor,Action},commands::{parse,Command}};
    let mut e=Editor::default();e.apply(Action::Close).unwrap();
    for line in ["CPlane XY","Box w-1,-1,0 2 2 0.4","Select All","BoundingBox","Length BoundingBox","View Isometric","Select None","Fit"]{let Command::Editor(action)=parse(line).unwrap()else{panic!("Editor command")};e.apply(action).unwrap();}
    let id=e.scene.objects().iter().find(|r|r.original.is_some()).unwrap().id;e.select_objects([id],false);let Command::Editor(action)=parse("Measure Distance w-1,0,1 w1,0,1").unwrap()else{panic!("Distance command")};e.apply(action).unwrap();e
}
fn make_renderer(device:wgpu::Device,queue:wgpu::Queue,format:wgpu::TextureFormat)->Renderer{
    let e=editor();let mut renderer=Renderer::new(device,queue,format,&e.scene);
    let mut math_quality=viewer_journey::math_view::Presentation::default();
    math_quality.drain_matrix(&mut renderer,&e.scene,&e.camera.uniform(),[WIDTH,HEIGHT],e.selected).expect("Native mathematical display must meet the requested physical-pixel quality before capture");
    let math_scene=math_quality.scene(&e.scene);renderer.select_objects(&math_scene,&e.selected_objects()).unwrap();
    renderer.replace_text_fonts(vec![include_bytes!("../assets/text/NotoSans-Regular.ttf").to_vec(),include_bytes!("../assets/text/NotoSansSymbols2-Regular.ttf").to_vec(),include_bytes!("../assets/text/NotoSansSymbols-Regular.ttf").to_vec()]).unwrap();renderer.set_measurement(e.mark.as_ref()).unwrap();renderer
}
fn draw(renderer:&Renderer,view:&wgpu::TextureView){let e=editor();renderer.draw(view,&e.background,&e.camera.uniform());assert_eq!(e.scene.objects().len(),2);let row=e.scene.objects().iter().find(|r|r.original.is_some()).unwrap();let Some(session_rust::Geometry::OBB(source))=&row.original else{panic!("Original bounding box")};for(i,expected)in[1.,1.,0.2].into_iter().enumerate(){assert!((source.half_size[i]-expected).abs()<1e-8)}assert!(row.mesh.indices().is_empty());assert_eq!(row.mesh.parts.edges.len(),12);assert_eq!(e.mark.as_ref().unwrap().label,"2 mm");assert!(e.scene.lines().is_empty());assert!(renderer.measurement_bytes()>0);assert_eq!(renderer.text_error(),None);assert_eq!(renderer.text_stats().unwrap().missing_glyphs,0);}
fn verify_pixels(pixels:&[u8]){let faces=pixels.chunks_exact(4).filter(|p|p[0]<235&&p[1]<235&&p[2]<235).count();assert!(faces>1000,"Original solid stays visible inside wire-only bounding box: {faces}");let black=pixels.chunks_exact(4).filter(|p|p[0]<100&&p[1]<100&&p[2]<100).count();assert!(black>100,"Actual unpickable measurement line, endpoint squares and value chip: {black}");let gold=pixels.chunks_exact(4).filter(|p|p[0]>180&&p[1]>120&&p[2]<100).count();assert!(gold>20,"Selected original wire bounding box: {gold}");}
