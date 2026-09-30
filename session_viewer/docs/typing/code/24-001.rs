
/// One Enter on a partial name runs the verb, or opens its options when it needs one.
#[test]
fn one_enter_runs_a_partial_verb_or_opens_its_options() {
    assert_eq!(accept("Del"), ("Delete".into(), true));
    assert_eq!(accept("clo"), ("Close".into(), true));
    assert_eq!(accept(" clo"), ("Close".into(), true));
    assert_eq!(accept("cur"), ("Curve".into(), true));
    assert_eq!(accept("Sn"), ("Snap ".into(), false));
    assert_eq!(accept("Snap ne"), ("Snap Near".into(), true));
    assert_eq!(accept("Snap pe"), ("Snap Perp".into(), true));
}

/// Snap turns snapping on or off, or toggles one kind.
#[test]
fn snap_takes_a_switch_or_a_kind() {
    assert_eq!(
        parsed("Snap Off"),
        Ok("Snap { on: Some(false), mode: 0 }".into())
    );
    assert_eq!(parsed("snap"), Ok("Snap { on: None, mode: 0 }".into()));
    assert_eq!(parsed("Snap near"), Ok("Snap { on: None, mode: 2 }".into()));
    assert_eq!(
        parsed("Snap Center"),
        Ok("Snap { on: None, mode: 8 }".into())
    );
    assert!(parsed("Snap sideways").is_err());
    assert_eq!(parsed("Close"), Ok("Close".into()));
    assert!(parsed("Close 3").is_err());
}

/// The shown spelling replaces what was typed, for history lines.
#[test]
fn canonical_spells_the_command_as_shown() {
    assert_eq!(
        canonical("clippingplane xy 0,0,1"),
        "Clipping Plane XY 0,0,1"
    );
    assert_eq!(
        canonical("clipping plane fill hatch"),
        "Clipping Plane Fill Hatch"
    );
    assert_eq!(canonical("  snap   near "), "Snap Near");
    assert_eq!(canonical("m 10 0 0"), "Move 10 0 0");
    assert_eq!(canonical("0,0,0"), "0,0,0");
    assert_eq!(canonical(""), "");
}
