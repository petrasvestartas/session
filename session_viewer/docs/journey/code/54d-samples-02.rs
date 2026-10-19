use crate::rect_select::{Rectangle,screen,found};

#[test]
fn direction_and_segment_clipping_distinguish_window_from_crossing() {
    let window=Rectangle::new([40.,40.],[60.,60.]).unwrap();let crossing=Rectangle::new([60.,40.],[40.,60.]).unwrap();assert!(!window.crossing&&crossing.crossing);assert_eq!(window.bounds,crossing.bounds);
    assert!(crossing.crosses([0.,50.],[100.,50.]));assert!(!crossing.contains([0.,50.])&&!crossing.contains([100.,50.]));assert!(!crossing.crosses([0.,20.],[100.,20.]));assert!(crossing.crosses([40.,0.],[40.,100.]));
    assert!(Rectangle::new([0.,0.],[0.,3.]).is_none());assert!(Rectangle::new([f64::NAN,0.],[3.,3.]).is_none());assert!(!crossing.crosses([f64::INFINITY,0.],[1.,2.]));
}

#[test]
fn projection_uses_css_sizes_and_rejects_behind_eye_and_invalid_dimensions() {
    let identity=session_rust::Xform::identity().m;assert_eq!(screen(&identity,[0.,0.,0.],[640.,480.]),Some([320.,240.]));assert_eq!(screen(&identity,[0.5,0.5,0.],[640.,480.]),Some([480.,120.]));
    let mut behind=identity;behind[15]=-1.;assert!(screen(&behind,[0.;3],[640.,480.]).is_none());assert!(screen(&identity,[0.;3],[0.,480.]).is_none());assert!(screen(&identity,[f64::NAN,0.,0.],[640.,480.]).is_none());
}

#[test]
fn mixed_precise_sources_are_selected_by_projected_geometry_and_current_eligibility() {
    let mut e=crate::editor::Editor::default();e.apply(crate::editor::Action::Close).unwrap();
    let line=e.scene.insert_line(crate::stroke::PreparedLine::new(std::rc::Rc::new(session_rust::Line::new(-0.8,0.,0.,0.8,0.,0.))).unwrap()).unwrap();
    let point=e.scene.insert_point(crate::marker::PreparedPoint::new(std::rc::Rc::new(session_rust::Point::new(0.,0.,0.))).unwrap()).unwrap();let identity=session_rust::Xform::identity().m;
    let window=Rectangle::new([90.,90.],[110.,110.]).unwrap();let crossing=Rectangle::new([110.,90.],[90.,110.]).unwrap();assert_eq!(found(&e.scene,&identity,[200.,200.],window),vec![point]);assert_eq!(found(&e.scene,&identity,[200.,200.],crossing),vec![line,point]);
    std::rc::Rc::make_mut(&mut e.scene.view).lock(&[point],true);assert_eq!(found(&e.scene,&identity,[200.,200.],crossing),vec![line]);std::rc::Rc::make_mut(&mut e.scene.view).hide(&[line],true);assert!(found(&e.scene,&identity,[200.,200.],crossing).is_empty());
    assert_eq!(e.scene.lines()[0].prepared.source.start()[0],-0.8);assert_eq!(e.scene.points()[0].prepared.source[0],0.);
}
