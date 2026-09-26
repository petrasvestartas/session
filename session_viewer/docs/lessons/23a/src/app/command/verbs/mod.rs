pub mod geometry; // register:geometry
pub mod selecting; // register:selecting

/// Declare each verb's module and list its SPEC in REGISTRY, so a verb is one file plus one line.
macro_rules! verbs {
    ($($name:ident),* $(,)?) => {
        $(pub mod $name;)*

        /// Every verb the command line knows.
        pub const REGISTRY: &[&dyn super::Verb] = &[
            $(&$name::SPEC,)*
            #[cfg(test)]
            &geometry::tests::SPEC, // Wedge, a drawing verb for tests
        ];
    };
}

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
