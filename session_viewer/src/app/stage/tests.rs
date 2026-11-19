use super::{deck, font, run, tex};
use std::collections::HashMap;
use std::rc::Rc;

const DECKS: [&str; 11] = [
    "equilibrium",
    "force-density",
    "force-density-matrices",
    "force-density-derivation",
    "force-density-materialization",
    "foppl-spatial-joint",
    "thrust-network_analysis_one_node",
    "thrust-network_analysis_horizontal_equilibrium",
    "thrust-network_analysis_vertical_equilibrium",
    "thrust-network_analysis_horizontal_methods",
    "thrust-network_analysis_geometries",
];

/// The stage fonts read from the crate's assets.
fn fonts() -> font::Fonts {
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/stage/");
    let files = font::FILES
        .iter()
        .map(|f| std::fs::read(format!("{dir}{f}")).expect("stage font"))
        .collect();
    font::Fonts::new(files).expect("fonts parse")
}

/// The index and chapters of an example deck.
fn deck(name: &str) -> (deck::Index, Vec<deck::Chapter>) {
    let dir = format!("{}/examples/stage/{name}/", env!("CARGO_MANIFEST_DIR"));
    let src = std::fs::read_to_string(format!("{dir}index.yaml")).unwrap();
    let index = deck::index(&src).unwrap();
    let chapters = index
        .chapters
        .iter()
        .map(|c| {
            let src = std::fs::read_to_string(format!("{dir}{c}")).unwrap();
            deck::chapter(&src, c).unwrap_or_else(|e| panic!("{e}"))
        })
        .collect();
    (index, chapters)
}

#[test]
fn fraction_rule_sits_on_the_math_axis() {
    let f = fonts();
    let e = tex::typeset(&f, r"\frac{a}{b}", &HashMap::new(), false);
    let rule = e
        .prims
        .iter()
        .find(|p| matches!(p.shape, tex::Shape::Rect { .. }))
        .expect("a fraction rule");
    let tex::Shape::Rect { h, .. } = rule.shape else {
        unreachable!()
    };
    let centre = -(rule.y + h / 2.0);
    assert!(
        (centre - f.c.axis).abs() < 1e-3,
        "rule centre {centre} vs axis {}",
        f.c.axis
    );
    assert!(e.h > 0.8 && e.d > 0.3);
}

#[test]
fn parts_keep_their_tex_as_key() {
    let f = fonts();
    let e = tex::typeset(
        &f,
        r"{{x_k - x_0}}^2 + {{\mathbf{L}^{-1}\,\mathbf{f}}}",
        &HashMap::new(),
        false,
    );
    assert_eq!(e.part(&tex::normalize_key("x_k - x_0")).len(), 5);
    assert_eq!(
        e.part(&tex::normalize_key(r"\mathbf{L}^{-1}\,\mathbf{f}"))
            .len(),
        4
    );
    assert!(e.unknown.is_empty());
}

#[test]
fn unknown_commands_are_reported() {
    let e = tex::typeset(&fonts(), r"\nosuch{x}", &HashMap::new(), false);
    assert_eq!(e.unknown, vec![r"\nosuch".to_string()]);
}

#[test]
fn text_keeps_its_spaces() {
    let f = fonts();
    let one = tex::typeset(&f, r"\text{ab}", &HashMap::new(), false);
    let two = tex::typeset(&f, r"\text{a b}", &HashMap::new(), false);
    assert!(two.w > one.w + 0.2);
}

#[test]
fn nested_tex_and_equation_options_keep_their_content() {
    let src = r#"
slides:
  - title: Load
    steps:
      - say: 'What is the force in the cables?'
        do:
          - tex: {tex: 'f_1 = \ ?\qquad f_2 = \ ?', align: left}
      - say: 'The node equilibrium.'
        eq: {tex: '\mathbf{f}_1 + \mathbf{f}_2 + \mathbf{p} = \mathbf{0}', align: left}
      - say: 'A scalar equation with options.'
        eq: 'x = 1'
        align: left
        color: x
      - say: 'A nested follow-up equation.'
        then: {tex: 'x = 2', align: left}
      - say: 'A scalar follow-up equation.'
        then: 'x = 3'
        align: left
"#;
    let chapter = deck::chapter(src, "nested.yaml").unwrap();
    let expected = [
        r"f_1 = \ ?\qquad f_2 = \ ?",
        r"\mathbf{f}_1 + \mathbf{f}_2 + \mathbf{p} = \mathbf{0}",
        "x = 1",
        "x = 2",
        "x = 3",
    ];
    for (step, expected) in chapter.slides[0].steps.iter().zip(expected) {
        let object = match &step.actions.last().unwrap().kind {
            deck::Kind::Add(objects) => &objects[0],
            deck::Kind::Derive { obj, .. } => obj,
            _ => panic!("the step must write a formula"),
        };
        let deck::Content::Tex(actual) = &object.content else {
            panic!("the step must contain TeX");
        };
        assert_eq!(actual.as_str(), expected);
        assert_eq!(object.align.as_deref(), Some("left"));
    }
    let deck::Kind::Add(objects) = &chapter.slides[0].steps[2].actions[0].kind else {
        unreachable!()
    };
    assert_eq!(objects[0].color.as_deref(), Some("x"));
}

#[test]
fn expressions_resolve_points() {
    struct Named;
    impl super::eval::Points for Named {
        fn point(&self, name: &str) -> Option<[f64; 3]> {
            match name {
                "A" => Some([0.0, 0.0, 0.0]),
                "B" => Some([3.0, 4.0, 0.0]),
                _ => None,
            }
        }
    }
    let value = serde_yaml_ng::Value::String("mid(A, B) + [0, 0, 1]".into());
    let pos = deck::pos(&value, "t").unwrap();
    assert_eq!(
        super::eval::position(&pos, &Named).unwrap(),
        [1.5, 2.0, 1.0]
    );
    assert_eq!(super::eval::fill("l = {=len(A,B):.2}", &Named), "l = 5.00");
}

/// Every step at rest: no author errors, no overlapping blocks or labels, every block on the slide.
#[test]
fn decks_rest_cleanly_at_every_step() {
    for name in DECKS {
        rests_cleanly(name);
    }
}

/// Give a stage the pictures its chapters name, read from the example folder.
fn pictures(stage: &mut run::Stage, name: &str) {
    let dir = format!("{}/examples/stage/{name}/", env!("CARGO_MANIFEST_DIR"));
    let chapters = deck::index(&std::fs::read_to_string(format!("{dir}index.yaml")).unwrap())
        .unwrap()
        .chapters;
    for chapter in chapters {
        let src = std::fs::read_to_string(format!("{dir}{chapter}")).unwrap();
        for file in deck::pictures(&src) {
            let bytes = std::fs::read(format!("{dir}{file}")).expect("picture file");
            let picture = super::paint::picture(&bytes).expect("picture decodes");
            let aspect = picture.size[0] as f32 / picture.size[1] as f32;
            let index = stage.pictures.len() as u16;
            stage.pictures.entry(file).or_insert((index, aspect));
        }
    }
}

/// Rebuild every step of a deck and check it at rest.
fn rests_cleanly(name: &str) {
    let (index, chapters) = deck(name);
    let mut stage = run::Stage::new(Rc::new(fonts()), index, chapters);
    pictures(&mut stage, name);
    for c in 0..stage.chapters.len() {
        stage.rebuild(c, None);
        for g in 0..stage.steps() {
            stage.rebuild(c, Some(g));
            stage.frame(0.0);
            stage.frame(0.0);
            let at = format!("{name} chapter {} step {}", c + 1, g + 1);
            assert!(stage.errors.is_empty(), "{at}: {:?}", stage.errors);
            assert!(
                stage.overlaps().is_empty(),
                "{at}: overlaps {:?}",
                stage.overlaps()
            );
            for (id, b) in stage.item_boxes() {
                assert!(
                    b[2] <= run::W + 1.0 && b[3] <= run::H,
                    "{at}: {id} leaves the slide {b:?}"
                );
            }
        }
    }
}

/// Play every chapter forward in real time at 60 frames a second.
#[test]
fn decks_play_through() {
    for name in DECKS {
        plays_through(name);
    }
}

/// Play a deck forward until every step has settled.
fn plays_through(name: &str) {
    let (index, chapters) = deck(name);
    let mut stage = run::Stage::new(Rc::new(fonts()), index, chapters);
    pictures(&mut stage, name);
    let mut now = 0.0;
    for c in 0..stage.chapters.len() {
        stage.chapter_go(c, now);
        loop {
            let mut guard = 0;
            while stage.animating() && guard < 2000 {
                now += 16.0;
                stage.tick(now);
                stage.frame(now);
                guard += 1;
            }
            assert!(
                guard < 2000,
                "chapter {} step {:?} never settles",
                c + 1,
                stage.step
            );
            if stage.step.is_some_and(|g| g + 1 >= stage.steps()) {
                break;
            }
            stage.next(now);
        }
        assert!(
            stage.errors.is_empty(),
            "chapter {}: {:?}",
            c + 1,
            stage.errors
        );
    }
}

#[test]
fn multiply_tints_a_row_and_a_column_and_fills_the_entry() {
    let src = "
title: product
slides:
  - title: one entry
    steps:
      - row:
          - {id: A, matrix: [[1, 2]]}
          - {id: B, matrix: [[3], [4]]}
          - '='
          - {id: C, matrix: {rows: [[11]], empty: true}}
      - multiply: {left: A, right: B, into: C, entry: [0, 0]}
";
    let chapter = deck::chapter(src, "product.yaml").unwrap();
    let index = deck::index("name: t\nchapters: [product.yaml]").unwrap();
    let mut stage = run::Stage::new(Rc::new(fonts()), index, vec![chapter]);
    stage.rebuild(0, Some(1));
    stage.frame(0.0);
    assert!(stage.errors.is_empty(), "{:?}", stage.errors);
}

#[test]
fn a_chapter_block_fills_its_holes() {
    let src = "
title: blocks
blocks:
  given:
    - text: '**Given.** $what'
    - matrix: {id: $id, rows: $rows}
    - highlight: [$id]
slides:
  - title: one block
    steps:
      - given: {what: the load, id: p, rows: [[0, 0, -5]]}
";
    let chapter = deck::chapter(src, "blocks.yaml").unwrap();
    assert_eq!(chapter.slides[0].steps[0].actions.len(), 3);
    let index = deck::index("name: t\nchapters: [blocks.yaml]").unwrap();
    let mut stage = run::Stage::new(Rc::new(fonts()), index, vec![chapter]);
    stage.rebuild(0, Some(0));
    stage.frame(0.0);
    assert!(stage.errors.is_empty(), "{:?}", stage.errors);
}

#[test]
fn fill_and_reveal_build_the_given_matrices_from_the_net() {
    let src = "
title: given
net:
  fixed: {n1: [0, 0, 0], n2: [5, 0, 3]}
  free: {n0: [2, 1, 1]}
  cables: {e0: [n0, n1], e1: [n0, n2]}
slides:
  - title: the givens
    steps:
      - show: net
      - fill: xF
      - reveal: C
";
    let chapter = deck::chapter(src, "given.yaml").unwrap();
    let index = deck::index("name: t\nchapters: [given.yaml]").unwrap();
    let mut stage = run::Stage::new(Rc::new(fonts()), index, vec![chapter]);
    for step in 0..3 {
        stage.rebuild(0, Some(step));
        stage.frame(0.0);
        assert!(stage.errors.is_empty(), "step {step}: {:?}", stage.errors);
    }
}

#[test]
fn row_items_center_text_in_fixed_width_slots() {
    let src = "slides: [{title: a, steps: [{row: [{id: left, text: x, width: 4}, {id: right, text: y}]}]}]";
    let chapter = deck::chapter(src, "row.yaml").unwrap();
    let index = deck::index("name: t\nchapters: [row.yaml]").unwrap();
    let mut stage = run::Stage::new(Rc::new(fonts()), index, vec![chapter]);
    stage.rebuild(0, Some(0));
    assert!(stage.errors.is_empty(), "{:?}", stage.errors);
    let row = &stage.objs[0];
    let first_x = row.groups["left"]
        .iter()
        .map(|&i| row.expr.prims[i].x)
        .fold(f32::INFINITY, f32::min);
    let second_x = row.groups["right"]
        .iter()
        .map(|&i| row.expr.prims[i].x)
        .fold(f32::INFINITY, f32::min);
    assert!(first_x > 1.0, "first item is centred: {first_x}");
    assert!(second_x > 4.0, "second item follows the slot: {second_x}");
}

#[test]
fn both_grounds_name_the_same_colours() {
    let light = deck::theme_palette(deck::Theme::Light);
    let dark = deck::theme_palette(deck::Theme::Dark);
    let mut a: Vec<_> = light.keys().collect();
    let mut b: Vec<_> = dark.keys().collect();
    a.sort();
    b.sort();
    assert_eq!(a, b);
    assert_eq!(light["bg"], [1.0, 1.0, 1.0, 1.0]);
    assert_eq!(dark["bg"], [0.0, 0.0, 0.0, 1.0]);
    assert_eq!(light["navy"], super::tex::hex("#182c5b").unwrap());
    assert_eq!(light["load"], super::tex::hex("#007bc4").unwrap());
    assert_eq!(light["accent"], light["load"]);
    assert_eq!(light["pos"], super::tex::hex("#ff5c9a").unwrap());
    assert_eq!(light["neg"], super::tex::hex("#4c9bff").unwrap());
    assert_eq!(light["yellow"], super::tex::hex("#ffc400").unwrap());
    assert_eq!(light["free"], light["ink"]);
    assert_eq!(light["fixed"], light["ink"]);
    assert_eq!(light["blue_band"], super::tex::hex("#a9c9ff").unwrap());
}

#[test]
fn a_chapter_colour_can_differ_per_ground() {
    let src = "palette: {accent: {light: blue, dark: '#00ff00'}, mark: pink}\n\
               slides: [{title: a, steps: [{say: x}]}]\n";
    let ch = deck::chapter(src, "c").unwrap();
    let light = deck::chapter_palette(deck::Theme::Light, &ch.overrides);
    let dark = deck::chapter_palette(deck::Theme::Dark, &ch.overrides);
    assert_eq!(light["accent"], light["blue"]);
    assert_eq!(dark["accent"], [0.0, 1.0, 0.0, 1.0]);
    assert_eq!(light["mark"], light["pink"]);
    assert_eq!(dark["mark"], dark["pink"]);
    assert!(
        deck::chapter(
            "palette: {a: nocolour}\nslides: [{title: a, steps: [{say: x}]}]\n",
            "c"
        )
        .is_err()
    );
}

#[test]
fn yellow_matrix_highlights_use_black_numbers_and_accent_labels() {
    let chapter = deck::chapter(
        "slides:\n  - title: m\n    steps:\n      - matrix: {id: M, rows: [[1]], rowlabels: [n_0], collabels: [x]}\n      - color: {target: M, color: yellow}\n",
        "c.yaml",
    )
    .unwrap();
    let index = deck::index("name: Fills\nchapters: [c.yaml]").unwrap();
    let mut stage = run::Stage::new(Rc::new(fonts()), index, vec![chapter]);
    stage.rebuild(0, Some(2));
    stage.tick(10_000.0);
    let matrix = stage.objs.iter().find(|o| o.id == "M").unwrap();
    let entry = matrix.groups["0,0"][1];
    assert_eq!(matrix.colors[&entry].at(10_000.0), stage.palette("ink"));
    for label in &matrix.groups["labels"] {
        assert_eq!(matrix.colors[label].at(10_000.0), stage.palette("accent"));
    }
}

#[test]
fn matching_matrix_symbols_are_colored_together() {
    let (index, chapters) = deck("force-density-matrices");
    let mut stage = run::Stage::new(Rc::new(fonts()), index, chapters);
    stage.rebuild(1, Some(0));
    stage.tick(10_000.0);
    for key in ["CNs/name", "CFs/name"] {
        let matrix = stage
            .objs
            .iter()
            .find(|o| o.groups.contains_key(key))
            .unwrap();
        assert_eq!(
            super::actions::matrix_name_key(&matrix.content, "\\mathbf{C}_N").as_deref(),
            Some("CNs/name")
        );
        let prim = matrix.groups[key][0];
        assert_eq!(
            matrix.colors[&prim].at(10_000.0),
            stage.palette("accent"),
            "{key}"
        );
    }
}

#[test]
fn matrix_cells_use_vivid_sign_fills() {
    let chapter = deck::chapter(
        "slides: [{title: m, steps: [{matrix: {id: M, rows: [[1, -1, 0]]}}]}]",
        "c.yaml",
    )
    .unwrap();
    let index = deck::index("name: Fills\nchapters: [c.yaml]").unwrap();
    let mut stage = run::Stage::new(Rc::new(fonts()), index, vec![chapter]);
    stage.rebuild(0, Some(1));
    let (positive, negative, neutral) = (
        stage.palette("pos"),
        stage.palette("neg"),
        stage.palette("cell"),
    );
    let matrix = stage.objs.iter().find(|o| o.id == "M").unwrap();
    let fills: Vec<_> = matrix
        .expr
        .prims
        .iter()
        .filter(|p| matches!(p.shape, tex::Shape::Rect { .. }))
        .map(|p| p.color)
        .collect();
    assert_eq!(fills, vec![Some(positive), Some(negative), Some(neutral)]);
}

#[test]
fn the_ground_swaps_at_the_same_step() {
    let (index, chapters) = deck("force-density");
    let mut stage = run::Stage::new(Rc::new(fonts()), index, chapters);
    pictures(&mut stage, "force-density");
    assert_eq!(stage.theme, deck::Theme::Light);
    stage.rebuild(0, Some(2));
    let before = (stage.chapter, stage.step, stage.objs.len());
    stage.set_theme(deck::Theme::Dark, 0.0);
    assert_eq!(stage.palette("bg"), [0.0, 0.0, 0.0, 1.0]);
    assert_eq!((stage.chapter, stage.step, stage.objs.len()), before);
    stage.set_theme(deck::Theme::Light, 0.0);
    assert_eq!(stage.palette("bg"), [1.0, 1.0, 1.0, 1.0]);
}

#[test]
fn labels_have_halo_outlines_and_stay_above_drawings_and_clear_of_segments() {
    let (ow, oh, pad, bytes) = super::paint::dilate_alpha(&[255], 1, 1, 4.5);
    assert_eq!((ow, oh, pad), (13, 13, 6));
    assert_eq!(bytes[6 * ow + 6], 255);
    assert_eq!(bytes[6 * ow + 10], 255);
    assert!(bytes[6 * ow + 11] > 100);
    assert_eq!(bytes[0], 0);

    let src = "
slides:
  - title: order
    steps:
      - add:
          - {id: note, tex: 'F_{\\mathrm{H},li}', at: [960, 260]}
          - {id: line, drawing: [{arrow: [[920, 260], [1060, 260]], color: ink}], at: [0, 0]}
";
    let ch = deck::chapter(src, "order.yaml").unwrap();
    let idx = deck::index("name: t\nchapters: [order.yaml]").unwrap();
    let mut stage = run::Stage::new(Rc::new(fonts()), idx, vec![ch]);
    stage.rebuild(0, Some(0));
    let draws = stage.frame(0.0);
    let first_path = draws
        .iter()
        .position(|d| matches!(d, run::Draw::Path { .. }))
        .unwrap();
    let last_label = draws
        .iter()
        .rposition(|d| matches!(d, run::Draw::Label(_)))
        .unwrap();
    assert!(
        first_path < last_label,
        "2D drawings must paint before diagram labels so text stays on top"
    );

    let (index, chapters) = deck("thrust-network_analysis_one_node");
    let mut stage = run::Stage::new(Rc::new(fonts()), index, chapters);
    for c in 0..stage.chapters.len() {
        for g in 0..stage.steps() {
            stage.rebuild(c, Some(g));
            stage.frame(0.0);
            let view = stage.view(0.0);
            for i in 0..stage.objs.len() {
                let o = &stage.objs[i];
                let Some(att) = o.attach.as_deref() else {
                    continue;
                };
                let prims = stage.prims_of(i, 0.0);
                let Some(b) = stage.bbox(&prims, None) else {
                    continue;
                };
                let Some(li) = stage.objs.iter().position(|x| x.id == att) else {
                    continue;
                };
                if !matches!(stage.objs[li].content, deck::Content::Line { .. }) {
                    continue;
                }
                let pts = stage.geo_points(&stage.objs[li], 0.0);
                let (p0, p1) = (view.project(pts[0]), view.project(pts[1]));
                let hit = super::geometry::clip_len(p0, p1, b);
                assert!(
                    hit <= 0.5,
                    "chapter {} step {}: label {} crosses its segment {} by {hit:.1}px ({b:?})",
                    c + 1,
                    g + 1,
                    o.id,
                    att
                );
            }
        }
    }
}
