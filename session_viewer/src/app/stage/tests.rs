use super::{deck, font, run, tex};
use std::collections::HashMap;
use std::rc::Rc;

const DECKS: [&str; 2] = ["force-density", "force-density-derivations"];

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
