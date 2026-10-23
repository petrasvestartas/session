#[test]
fn fully_hidden_finite_ink_defers_colour_and_restores_fresh_owner_queries_without_uploads() {
    for samples in [1,4] {
        let mut scene=Scene::empty();
        let mut point=session_rust::Point::new(0.,0.,-0.5);
        point.width=12.;point.pointcolor=session_rust::Color::new(1.,0.,0.,1.);
        let id=scene.insert_point(crate::marker::PreparedPoint::new(std::rc::Rc::new(point)).unwrap()).unwrap();
        let original=scene.points()[0].prepared.source.to_proto();
        let mut r=renderer(&scene);r.set_samples(samples);
        r.markers.ranges=Default::default();r.markers.ranges.push(POINTS,0,1);
        r.markers.ranges.mask=0;
        let usage=r.point_usage();let source_ids=r.sources.marker_ids.clone();
        assert!(!r.markers.finite_colour_ready());
        let hidden=pixels(&r);
        assert!(!r.markers.finite_colour_ready(),"Hidden ranges must not compile a colour pipeline");
        r.request_pick([0.,0.],matrix()).unwrap();wait(&r);
        assert!(r.take_pick(matrix()).unwrap().unwrap().is_none());
        assert!(!r.markers.finite_colour_ready(),"An independent owner query must not compile colour");
        r.markers.ranges.mask=POINTS;
        let visible=pixels(&r);
        assert!(r.markers.finite_colour_ready());assert_ne!(at(&visible,160,120),at(&hidden,160,120));
        r.request_pick([0.,0.],matrix()).unwrap();wait(&r);
        assert_eq!(r.take_pick(matrix()).unwrap().unwrap().map(|v|v[0]),Some(id.value()));
        r.markers.ranges.mask=0;
        assert!(pixels(&r)==hidden);
        r.request_pick([0.,0.],matrix()).unwrap();wait(&r);
        assert!(r.take_pick(matrix()).unwrap().unwrap().is_none());
        r.markers.ranges.mask=POINTS;
        assert!(pixels(&r)==visible);
        r.request_pick([0.,0.],matrix()).unwrap();wait(&r);
        assert_eq!(r.take_pick(matrix()).unwrap().unwrap().map(|v|v[0]),Some(id.value()));
        assert_eq!(r.point_usage(),usage);assert_eq!(r.sources.marker_ids,source_ids);
        assert_eq!(scene.points()[0].prepared.source.to_proto(),original);
    }
}

#[test]
fn actual_filtered_instances_keep_absolute_colour_and_original_query_addresses(){
