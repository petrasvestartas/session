use crate::{fonts, app::stage::{paint::Frame, tex::{self, Shape}}};
use std::collections::HashMap;

#[test]
fn fraction_has_a_rule_and_ink_above_and_below_baseline() {
    let f = fonts().unwrap();
    let e = tex::typeset(&f, r"\frac{a+b}{c}", &HashMap::new(), false);
    assert!(e.unknown.is_empty());
    assert!(e.prims.iter().any(|p| matches!(p.shape, Shape::Rect { .. })));
    assert!(e.prims.iter().any(|p| matches!(p.shape, Shape::Glyph { .. }) && p.y < 0.0));
    assert!(e.prims.iter().any(|p| matches!(p.shape, Shape::Glyph { .. }) && p.y > 0.0));
    assert!(e.w > 0.0 && e.h > 0.0 && e.d > 0.0);
}

#[test]
fn equals_alignment_survives_different_left_sides() {
    let f = fonts().unwrap();
    let a = tex::typeset(&f, "x=1", &HashMap::new(), false);
    let b = tex::typeset(&f, "x+y=2", &HashMap::new(), false);
    assert!(b.align.unwrap() > a.align.unwrap());
}

#[test]
fn letterbox_round_trip_and_upright_column() {
    let screen = egui::Rect::from_min_size(egui::pos2(10.0, 20.0), egui::vec2(800.0, 800.0));
    let frame = Frame::new(screen, 2.0);
    assert_eq!(frame.k, 0.5);
    assert_eq!(frame.pt([0.0, 0.0]), egui::pos2(10.0, 195.0));
    assert_eq!(frame.slide(frame.pt([120.0, 240.0])), [120.0, 240.0]);
    let upright = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(400.0, 800.0));
    let stacked = Frame::new(upright, 2.0).stacked(upright, 0.5);
    assert!(stacked.is_stacked());
    let column = stacked.side(1000.0);
    assert!(column.pt([1000.0, 0.0]).y > stacked.side(100.0).pt([100.0, 0.0]).y);
}

#[test]
fn unknown_command_is_reported() {
    let f = fonts().unwrap();
    assert!(!tex::typeset(&f, r"\notacommand{x}", &HashMap::new(), false).unknown.is_empty());
}

#[test]
fn lesson_deck_plays_and_previous_reconstructs_the_same_frame() {
    use crate::app::stage::{deck, run::Stage};
    use std::rc::Rc;
    let index = deck::index("name: Mathematics\nchapters: [example.yaml]").unwrap();
    let chapter = deck::chapter(include_str!("../example.yaml"), "example.yaml").unwrap();
    let mut stage = Stage::new(Rc::new(fonts().unwrap()), index, vec![chapter]);
    stage.next(0.0);
    stage.tick(100_000.0);
    let first = format!("{:?}", stage.frame(100_000.0));
    stage.next(100_000.0);
    stage.tick(200_000.0);
    stage.prev(200_000.0);
    assert_eq!(first, format!("{:?}", stage.frame(200_000.0)));
    for step in 1..stage.steps() {
        let now = step as f64 * 100_000.0;
        stage.next(now);
        stage.tick(now + 99_000.0);
        let _ = stage.frame(now + 99_000.0);
        assert!(stage.errors.is_empty(), "{:?}", stage.errors);
        assert!(stage.overlaps().is_empty(), "step {step}: {:?}", stage.overlaps());
    }
}
