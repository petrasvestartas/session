// --8<-- [start:verbs-modules]
pub mod geometry; // shared code of the drawing verbs, not a verb itself; register:geometry
pub mod selecting; // register:selecting
// --8<-- [end:verbs-modules]

// --8<-- [start:verbs-macro]
// One list of names becomes both the `pub mod` lines and the REGISTRY array.
/// Declare each verb's module and list its SPEC in REGISTRY, so a verb is one file plus one line.
macro_rules! verbs {
    ($($name:ident),* $(,)?) => { // `$name:ident` matches one name; `$(...),*` repeats for each comma-separated name
        $(pub mod $name;)* // `pub mod point;` for every name, so Rust compiles verbs/point.rs

        /// Every verb the command line knows.
        pub const REGISTRY: &[&dyn super::Verb] = &[ // `&dyn Verb`: each entry points at a different type, a Spec or a Draw, through the one trait
            $(&$name::SPEC,)*
            #[cfg(test)]
            &geometry::tests::SPEC, // Wedge exists only in test builds, proving a verb needs no other file
        ];
    };
}
// --8<-- [end:verbs-macro]

// --8<-- [start:verbs-list]
// Each line below is one verb; its tag tells docs/cut.py which lesson adds it. `r#move`: `move` is a Rust keyword, `r#` makes it a plain name.
verbs! {
    point,                   // register:point
    line,                    // register:line
    arrow,                   // register:arrow
    polyline,                // register:polyline
    curve,                   // register:curve
    close,                   // register:close
    trim,                    // register:trim
    extend,                  // register:extend
    explode,                 // register:explode
    r#move,                  // register:move
    rotate,                  // register:rotate
    scale,                   // register:scale
    copy,                    // register:copy
    orient_3_points,         // register:orient_3_points
    save,                    // register:save
    open,                    // register:open
    delete,                  // register:delete
    undo,                    // register:undo
    redo,                    // register:redo
    hide,                    // register:hide
    show,                    // register:show
    fit,                     // register:fit
    escape,                  // register:escape
    object,                  // register:object
    edge,                    // register:edge
    face,                    // register:face
    controls,                // register:controls
    select_lasso,            // register:select_lasso
    select_by_name,          // register:select_by_name
    select_small,            // register:select_small
    clipping_plane,          // register:clipping_plane
}
// --8<-- [end:verbs-list]
