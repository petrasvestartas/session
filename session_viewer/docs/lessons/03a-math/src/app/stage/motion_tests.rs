use super::{morph::match_prims, tex, tween::{Tween, smooth}};
use std::collections::{HashMap, HashSet};

#[test]
fn a_clock_starts_and_finishes_at_exact_values() {
    let mut value = Tween::fixed(0.0f32);
    value.set(10.0, 100.0, 200.0);
    assert_eq!(value.at(50.0), 0.0);
    assert_eq!(value.at(100.0), 0.0);
    assert_eq!(value.at(200.0), 5.0);
    assert_eq!(value.at(300.0), 10.0);
    assert!(!value.busy(300.0));
    assert_eq!(smooth(-1.0), 0.0);
    assert_eq!(smooth(2.0), 1.0);
}

#[test]
fn retargeting_starts_at_the_current_value_without_jumping() {
    let mut value = Tween::fixed(0.0f32);
    value.set(10.0, 0.0, 100.0);
    value.set(20.0, 50.0, 100.0);
    assert_eq!(value.at(50.0), 5.0);
    assert_eq!(value.at(150.0), 20.0);
    value.retarget(3.0, 200.0, 100.0, false);
    assert_eq!(value.at(200.0), 3.0);
    assert!(!value.busy(200.0));
}

#[test]
fn equal_glyphs_pair_once_and_new_symbols_are_unpaired() {
    let fonts = crate::fonts().unwrap();
    let a = tex::typeset(&fonts, "x+x=1", &HashMap::new(), false);
    let b = tex::typeset(&fonts, "x+x+y=1", &HashMap::new(), false);
    let from: Vec<_> = (0..a.prims.len()).collect();
    let to: Vec<_> = (0..b.prims.len()).collect();
    let (pairs, outs, ins) = match_prims(&a, &from, &b, &to, &[], &HashMap::new(), &HashMap::new());
    assert!(outs.is_empty());
    assert_eq!(pairs.len(), a.prims.len());
    assert_eq!(ins.len(), 2);
    let unique: HashSet<_> = pairs.iter().map(|&(_, j)| j).collect();
    assert_eq!(unique.len(), pairs.len());
    for (i, j) in pairs {
        assert_eq!(a.prims[i].shape.ident(), b.prims[j].shape.ident());
    }
}

#[test]
fn explicit_parts_survive_reordering() {
    let fonts = crate::fonts().unwrap();
    let a = tex::typeset(&fonts, "{{x}}+{{y}}", &HashMap::new(), false);
    let b = tex::typeset(&fonts, "{{y}}+{{x}}", &HashMap::new(), false);
    let from: Vec<_> = (0..a.prims.len()).collect();
    let to: Vec<_> = (0..b.prims.len()).collect();
    let (pairs, outs, ins) = match_prims(&a, &from, &b, &to, &[], &HashMap::new(), &HashMap::new());
    assert_eq!(pairs.len(), 3);
    assert!(outs.is_empty() && ins.is_empty());
    let x = (a.part("x")[0], b.part("x")[0]);
    assert!(pairs.contains(&x));
    assert!(a.prims[x.0].x < b.prims[x.1].x);
}
