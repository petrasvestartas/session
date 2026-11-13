use super::{deck, font, paint, presentation::Presentation};

fn presentation() -> Presentation {
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/stage/");
    let files = font::FILES
        .iter()
        .map(|name| std::fs::read(format!("{dir}{name}")).unwrap())
        .collect();
    let index = deck::index("name: Presentation\nchapters: [one.yaml]").unwrap();
    let chapter = deck::chapter(
        "slides: [{title: One, steps: [{say: 'A formula', eq: 'x=1'}]}]",
        "one.yaml",
    )
    .unwrap();
    Presentation::new(
        font::Fonts::new(files).unwrap(),
        index,
        vec![chapter],
        vec![
            (
                "wide".into(),
                paint::Picture {
                    size: [2, 1],
                    rgba: vec![255; 8],
                },
            ),
            (
                "square".into(),
                paint::Picture {
                    size: [1, 1],
                    rgba: vec![255; 4],
                },
            ),
        ],
        125.0,
    )
}

#[test]
fn pictures_keep_their_order_and_geometry_without_scene_rows() {
    let p = presentation();
    assert_eq!(p.stage.now, 125.0);
    assert_eq!(p.stage.pictures["wide"], (0, 2.0));
    assert_eq!(p.stage.pictures["square"], (1, 1.0));
    assert_eq!(p.atlas.pictures[0].size, [2, 1]);
    assert_eq!(p.atlas.pictures[1].rgba, vec![255; 4]);
    assert!(p.frame.is_none());
}

#[test]
fn painting_retains_coordinates_and_settles_without_losing_glyphs() {
    let mut p = presentation();
    p.stage.open_at(1, 1, 125.0);
    p.stage.tick(10_000.0);
    let context = egui::Context::default();
    context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
    for size in [[1280.0, 720.0], [390.0, 844.0]] {
        let screen = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(size[0], size[1]));
        p.stage.changed = true;
        let mut more = false;
        let output = context.run_ui(
            egui::RawInput {
                screen_rect: Some(screen),
                ..Default::default()
            },
            |root| {
                more = p.paint(root.painter(), screen, 2.0);
            },
        );
        assert!(more);
        assert!(!p.stage.changed);
        assert!(!output.shapes.is_empty());
        assert!(p.stage.errors.is_empty(), "{:?}", p.stage.errors);
        let frame = p.frame.unwrap();
        let point = [500.0, 300.0];
        let restored = frame.slide(frame.pt(point));
        assert!((restored[0] - point[0]).abs() < 0.001);
        assert!((restored[1] - point[1]).abs() < 0.001);
        let output = context.run_ui(
            egui::RawInput {
                screen_rect: Some(screen),
                ..Default::default()
            },
            |root| {
                more = p.paint(root.painter(), screen, 2.0);
            },
        );
        assert!(!more);
        assert!(!output.shapes.is_empty());
    }
}
