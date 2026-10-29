fn editor() -> viewer_journey::editor::Editor {
    use viewer_journey::{editor::{Action, Editor}, tool::Shape, snap::Kind};
    let mut editor = Editor::default(); editor.apply(Action::Close).unwrap();
    editor.apply(Action::BeginTool(Shape::Line)).unwrap();
    editor.apply(Action::ToolPoint([-0.65, -0.35, 0.])).unwrap();
    editor.apply(Action::ToolPoint([0.65, 0.35, 0.])).unwrap();
    editor.apply(Action::BeginTool(Shape::Line)).unwrap();
    editor.apply(Action::ToolPoint([-0.5, 0.5, 0.])).unwrap();
    let endpoint=session_rust::Point::new(0.65,0.35,0.);
    let clip=editor.camera.view_projection().transform_point(&endpoint);
    editor.apply(Action::ToolScreen { screen:[clip[0] as f32+8./640.,clip[1] as f32],size:[640.,480.],commit:false }).unwrap();
    let draft=editor.draft.as_ref().unwrap();
    assert_eq!(draft.cursor,Some([0.65,0.35,0.])); assert_eq!(draft.snapped,Some(Kind::End));
    assert!(draft.prompt().contains("Snap End")); assert_eq!(editor.scene.lines().len(),1); editor
}

fn make_renderer(device: wgpu::Device, queue: wgpu::Queue, format: wgpu::TextureFormat) -> Renderer {
    let editor=editor(); let mut renderer=Renderer::new(device,queue,format,&editor.scene);
    let mut math_quality=viewer_journey::math_view::Presentation::default();
    math_quality.drain_matrix(&mut renderer,&editor.scene,&editor.camera.uniform(),[WIDTH,HEIGHT],editor.selected).expect("Native mathematical display must meet the requested physical-pixel quality before capture");
    let math_scene=math_quality.scene(&editor.scene);
    renderer.set_preview(editor.draft.as_ref()).unwrap(); assert_eq!(renderer.preview_bytes(),76); renderer
}

fn draw(renderer: &Renderer, view: &wgpu::TextureView) {
    let editor=editor(); renderer.draw(view,&editor.background,&editor.camera.uniform());
}

fn verify_pixels(pixels: &[u8]) {
    let yellow=pixels.chunks_exact(4).filter(|p| p[0]>p[1].saturating_add(10) && p[1]>p[2].saturating_add(50)).count();
    let ink=pixels.chunks_exact(4).filter(|p| p[0].abs_diff(p[1])<8 && p[1].abs_diff(p[2])<8 && p[..3].iter().all(|v| *v<235)).count();
    let editor=editor(); let mid=editor.camera.view_projection().transform_point(&session_rust::Point::new(-0.3,-0.35*0.3/0.65,0.));
    let x=((mid[0]+1.)*WIDTH as f64/2.).round() as i32;
    let y=((1.-mid[1])*HEIGHT as f64/2.).round() as i32;
    let source_sample=(-2..=2).flat_map(|dy| (-2..=2).map(move |dx| (x+dx,y+dy)))
        .filter(|&(x,y)| x>=0 && y>=0 && x<WIDTH as i32 && y<HEIGHT as i32)
        .map(|(x,y)| &pixels[(y as usize*WIDTH as usize+x as usize)*4..][..3])
        .any(|p| p[0].abs_diff(p[1])<8 && p[1].abs_diff(p[2])<8 && p.iter().all(|v| *v<180));
    assert!(yellow>150,"The actual temporary band and snapped cursor draw: {yellow}");
    assert!(ink>100,"The antialiased original black line remains visible: {ink}");
    assert!(source_sample,"Original line ink is present at its projected source midpoint, away from the preview");
}
