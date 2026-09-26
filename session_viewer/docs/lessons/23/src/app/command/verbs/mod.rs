pub mod geometry; // register:geometry

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
    explode,                 // register:explode
    save,                    // register:save
    open,                    // register:open
    delete,                  // register:delete
    undo,                    // register:undo
    redo,                    // register:redo
    hide,                    // register:hide
    show,                    // register:show
    fit,                     // register:fit
    escape,                  // register:escape
    clipping_plane,          // register:clipping_plane
}
