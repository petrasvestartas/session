pub mod geometry; // register:geometry
pub mod measure; // register:measure
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
    split,                   // register:split
    save,                    // register:save
    open,                    // register:open
    delete,                  // register:delete
    undo,                    // register:undo
    redo,                    // register:redo
    hide,                    // register:hide
    show,                    // register:show
    fit,                     // register:fit
    escape,                  // register:escape
    layers,                  // register:layers
    arrowhead,               // register:arrowhead
    snap,                    // register:snap
    arctic,                  // register:arctic
    outline,                 // register:outline
    object,                  // register:object
    edge,                    // register:edge
    face,                    // register:face
    controls,                // register:controls
    select_lasso,            // register:select_lasso
    select_by_name,          // register:select_by_name
    select_small,            // register:select_small
    clipping_plane,          // register:clipping_plane
    r#box,                   // register:box
    sphere,                  // register:sphere
    cylinder,                // register:cylinder
    cone,                    // register:cone
    pyramid,                 // register:pyramid
    torus,                   // register:torus
    block_with_hole,         // register:block_with_hole
    tetrahedron,             // register:tetrahedron
    octahedron,              // register:octahedron
    dodecahedron,            // register:dodecahedron
    icosahedron,             // register:icosahedron
    quad_sphere,             // register:quad_sphere
    capsule,                 // register:capsule
    nurbs_curve_circle,      // register:nurbs_curve_circle
    nurbs_curve_ellipse,     // register:nurbs_curve_ellipse
    nurbs_curve_arc,         // register:nurbs_curve_arc
    nurbs_curve_parabola,    // register:nurbs_curve_parabola
    loft,                    // register:loft
    extrude,                 // register:extrude
    nurbs_surface_loft,      // register:nurbs_surface_loft
    nurbs_surface_network,   // register:nurbs_surface_network
    nurbs_surface_revolve,   // register:nurbs_surface_revolve
    nurbs_surface_4_points,  // register:nurbs_surface_4_points
    nurbs_surface_sweep1,    // register:nurbs_surface_sweep1
    nurbs_surface_sweep2,    // register:nurbs_surface_sweep2
    text,                    // register:text
    project_to_plane,        // register:project_to_plane
    measure_distance,        // register:measure_distance
    length,                  // register:length
    area,                    // register:area
    volume,                  // register:volume
    add_group,               // register:add_group
    add_edge,                // register:add_edge
}
