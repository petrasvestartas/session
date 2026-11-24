use super::{CommandHost, HeadlessHost, REGISTRY, name_of};
use crate::app::selection::SelectionTool;
use std::collections::HashSet;

struct Recorder<'a> {
    host: &'a mut HeadlessHost,
    tested: &'a mut HashSet<&'static str>,
}

impl Recorder<'_> {
    fn run(&mut self, line: &str) -> String {
        let canon = name_of(line);
        if !canon.is_empty() {
            self.tested.insert(canon);
        }
        self.host
            .run_command(line)
            .unwrap_or_else(|e| panic!("command `{line}` failed: {e}"))
    }

    fn run_follow(&mut self, line: &str) -> String {
        self.host
            .run_command(line)
            .unwrap_or_else(|e| panic!("follow-up `{line}` failed: {e}"))
    }
}

#[test]
fn headless_host_executes_every_registered_verb_without_gpu_or_window() {
    let mut host = HeadlessHost::new();
    let mut tested: HashSet<&'static str> = HashSet::new();

    {
        let mut r = Recorder {
            host: &mut host,
            tested: &mut tested,
        };

        // 1. Drawing & curve verbs (point, line, arrow, polyline, curve, close, wedge)
        r.run("Point 0,0,0");
        assert_eq!(r.host.scene.object_count(), 1);
        let l1_msg = r.run("Line 0,0,0 100,0,0");
        assert!(l1_msg.to_ascii_lowercase().contains("line"), "{l1_msg}");
        r.run("Arrow 0,20,0 100,20,0");
        r.run("Curve 0,40,0 50,80,0 100,40,0");
        // Open polyline closed with `Close`
        r.run("Polyline");
        r.run_follow("0,0,0");
        r.run_follow("40,0,0");
        r.run_follow("40,40,0");
        let close_msg = r.run("Close");
        assert!(
            close_msg.to_ascii_lowercase().contains("polyline"),
            "{close_msg}"
        );
        // Test verb `Wedge`
        r.run("Wedge 0,0,0 20,0,0 0,20,0");

        // 2. NURBS curve primitives
        r.run("Nurbs Curve Circle 0,0,0 15");
        r.run("Nurbs Curve Ellipse 0,0,0 25 12");
        r.run("Nurbs Curve Arc 0,0,0 15 90");
        r.run("Nurbs Curve Parabola 0,0,0 80 30");

        // 3. Solid / mesh primitives (all 13)
        r.run("Box 0,0,0 40 30 20");
        r.run("Sphere 60,0,0 10");
        r.run("Cylinder 90,0,0 8 25");
        r.run("Cone 120,0,0 8 25");
        r.run("Pyramid 150,0,0 12 20");
        r.run("Torus 190,0,0 16 4");
        r.run("Block With Hole 240,0,0 40 30 20 5");
        r.run("Tetrahedron 0,80,0 10");
        r.run("Octahedron 30,80,0 10");
        r.run("Dodecahedron 60,80,0 10");
        r.run("Icosahedron 90,80,0 10");
        r.run("Quad Sphere 120,80,0 10");
        r.run("Capsule 150,80,0 6 24");

        // 4. Measurements on solids & curves
        // Capsule is currently selected -> Volume, Area, Bounding Box, Length Bounding Box
        let vol = r.run("Volume");
        assert!(vol.contains("Volume"), "{vol}");
        let area = r.run("Area");
        assert!(area.contains("Area"), "{area}");
        let bbox = r.run("Bounding Box");
        assert!(bbox.to_ascii_lowercase().contains("bounding"), "{bbox}");
        let lbbox = r.run("Length Bounding Box");
        assert!(!lbbox.is_empty());

        // Select a line and run Length, Arrowhead, Extend; trim a NURBS curve
        r.run("Line -50,0,0 50,0,0");
        let line_row = r.host.scene.selected.expect("line selected");
        let len_msg = r.run("Length");
        assert!(len_msg.contains("100"), "{len_msg}");
        r.run("Arrowhead End");
        r.run("Extend Distance 10");
        r.run("Curve -50,10,0 0,30,0 50,10,0");
        let trim_msg = r.run("Trim 0.1 0.9");
        assert!(
            trim_msg.contains("updated") || trim_msg.to_ascii_lowercase().contains("trim"),
            "{trim_msg}"
        );

        // Measure Distance with two typed points
        r.run("Measure Distance");
        r.run_follow("0,0,0");
        let dist_msg = r.run_follow("30,40,0");
        assert!(dist_msg.contains("50"), "{dist_msg}");

        // 5. Transforms (Move, Rotate, Scale, Copy, Orient 3 Points, Project To Plane)
        r.host.select(Some(line_row));
        r.run("Move 10,5,0");
        r.run("Rotate z 45");
        r.run("Scale 2");
        let before_copy = r.host.scene.object_count();
        r.run("Copy 5,5,0");
        assert_eq!(r.host.scene.object_count(), before_copy + 1);
        r.run("Orient 3 Points 0,0,0 10,0,0 0,10,0 0,0,5 10,0,5 0,10,5");
        r.run("Project To Plane XY");

        // 6. Split & Explode
        r.run("Line 0,-50,0 0,50,0");
        let target_line = r.host.scene.selected.unwrap();
        r.run("Line -50,0,0 50,0,0");
        let cutter_line = r.host.scene.selected.unwrap();
        r.host.select(Some(target_line));
        r.run("Split");
        r.host.pick_split_cutter(cutter_line);
        let split_done = r.run("Split");
        assert!(
            split_done.to_ascii_lowercase().contains("split"),
            "{split_done}"
        );

        r.run("Polyline 0,0,0 20,0,0 20,20,0");
        let explode_msg = r.run("Explode");
        assert!(
            explode_msg.to_ascii_lowercase().contains("exploded"),
            "{explode_msg}"
        );

        // 7. Surfacing verbs (Loft, Extrude, Nurbs Surface Loft/Network/Revolve/4 Points/Sweep1/Sweep2)
        r.run("Nurbs Surface 4 Points 0,0,0 20,0,0 20,20,5 0,20,0");

        r.run("Line 0,200,0 40,200,0");
        let sec_a = r.host.scene.selected.unwrap();
        r.run("Line 0,240,10 40,240,10");
        let sec_b = r.host.scene.selected.unwrap();

        r.host.select_rows(vec![sec_a, sec_b], false);
        r.run("Loft Open");

        r.host.select_rows(vec![sec_a, sec_b], false);
        r.run("Nurbs Surface Loft Smooth");

        r.host.select(Some(sec_a));
        r.run("Extrude 25");

        r.host.select(Some(sec_a));
        r.run("Nurbs Surface Revolve 0,200,0 1,200,0 90");

        // Sweep1: rail + profile
        r.run("Line 0,300,0 0,360,0");
        let rail1 = r.host.scene.selected.unwrap();
        r.run("Line -10,300,0 10,300,0");
        let prof1 = r.host.scene.selected.unwrap();
        r.host.select_rows(vec![rail1, prof1], false);
        r.run("Nurbs Surface Sweep1");

        // Sweep2: rail1 + rail2 + shape
        r.run("Line -10,400,0 -10,460,0");
        let s2_r1 = r.host.scene.selected.unwrap();
        r.run("Line 10,400,0 10,460,0");
        let s2_r2 = r.host.scene.selected.unwrap();
        r.run("Line -10,400,0 10,400,0");
        let s2_shape = r.host.scene.selected.unwrap();
        r.host.select_rows(vec![s2_r1, s2_r2, s2_shape], false);
        r.run("Nurbs Surface Sweep2");

        // Network: 2 U curves crossing 2 V curves
        r.run("Line 0,500,0 40,500,0");
        let nu0 = r.host.scene.selected.unwrap();
        r.run("Line 0,540,0 40,540,0");
        let nu1 = r.host.scene.selected.unwrap();
        r.run("Line 0,500,0 0,540,0");
        let nv0 = r.host.scene.selected.unwrap();
        r.run("Line 40,500,0 40,540,0");
        let nv1 = r.host.scene.selected.unwrap();
        r.host.select_rows(vec![nu0, nu1, nv0, nv1], false);
        r.run("Nurbs Surface Network");

        // 8. Layers, Groups, Graph Edges, Text
        r.run("Layers All");
        assert!(r.host.display.layers_open && r.host.display.layers_expanded);
        r.host.select_rows(vec![nu0, nu1], false);
        r.run("Add Group Truss");
        r.host.select_rows(vec![nu0, nu1], false);
        r.run("Add Edge");

        r.run("Text Stamp");
        r.run_follow("0,0,0");

        // 9. Selection modes & selection queries
        r.run("Edge");
        assert_eq!(r.host.selection.tool, SelectionTool::Edge);
        r.run("Face");
        assert_eq!(r.host.selection.tool, SelectionTool::Face);
        r.run("Object");
        assert_eq!(r.host.selection.tool, SelectionTool::Object);
        r.host.select(Some(nu0));
        r.run("Controls");
        r.run("Escape");

        r.run("Select All");
        assert!(!r.host.selected_rows().is_empty());
        r.run("Select Small 15");
        r.run("Select By Name line");
        r.run("Select Lasso");
        r.run("Escape");

        // 10. Visibility, Undo/Redo, Delete
        r.run("Select All");
        r.run("Hide");
        r.run("Show");
        r.host.select(Some(nu0));
        r.run("Delete");
        r.run("Undo");
        r.run("Redo");

        // 11. View, Display, Snap, CPlane, Clipping Plane
        r.run("Fit");
        r.run("View Isometric");
        r.run("Arctic On");
        assert!(r.host.display.ssao);
        r.run("Arctic Off");
        r.run("Outline Off");
        assert!(!r.host.display.show_outlines);
        r.run("Outline On");
        r.run("Opacity 0.6");
        assert!((r.host.display.opacity - 0.6).abs() < 1e-5);
        r.run("Element Attributes On");
        r.run("Element Interactions Off");
        r.run("Snap Off");
        assert!(!r.host.snap.enabled);
        r.run("Snap On");
        r.run("Grid Snap 50");
        assert!((r.host.snap.grid_step - 50.0).abs() < 1e-9);
        r.run("Construction Plane XZ");
        r.run("Construction Plane View");

        r.run("Clipping Plane XY 0,0,10");
        r.run("Clipping Plane Flip");
        r.run("Clipping Plane Fill Solid");
        r.run("Clipping Plane Off");
        r.run("Clipping Plane On");

        // 12. Session I/O, M5 verbs (Alpha, Lock, Gumball, Properties, Wireframe, Cull, Export, Import) & Report
        r.run("Properties On");
        assert!(r.host.display.properties_open);
        r.run("Properties Off");
        assert!(!r.host.display.properties_open);

        r.run("View Wireframe On");
        assert!(r.host.display.wireframe);
        r.run("View Shaded");
        assert!(!r.host.display.wireframe);
        r.run("View Cull Backfaces On");
        assert!(r.host.display.cull_backfaces);
        r.run("View Cull Backfaces Off");

        r.run("Box 0,0,0 10 10 10");
        let m5_box = r.host.scene.selected.unwrap();
        r.run("Alpha 0.4");
        r.run("Gumball Object");
        r.run("Gumball Pivot 5,5,5");
        r.run("Gumball Reset");

        let exp_msg = r.run("Export");
        assert!(exp_msg.contains("OBJ"), "{exp_msg}");
        r.run("Import");

        r.host.select(Some(m5_box));
        r.run("Lock");
        assert!(!r.host.scene.selectable(m5_box));
        r.run("Lock Unlock");
        assert!(r.host.scene.selectable(m5_box));

        r.host.camera.orbit(25.0, -15.0);
        r.run("Arctic On");
        let saved_pose = r.host.camera.pose();
        let save_msg = r.run("Save");
        assert!(save_msg.contains("Saved"), "{save_msg}");
        let bytes = r.host.saved_bytes.clone().expect("saved .session bytes");
        let reopened =
            crate::app::session_io::open_with_state(&bytes).expect("open saved .session");
        assert!(!reopened.scene.docs.is_empty());
        assert_eq!(reopened.view.as_ref().map(|v| v.arctic), Some(true));
        let mut restored_cam = crate::camera::Camera::new();
        reopened.camera.unwrap().apply_to(&mut restored_cam);
        assert_eq!(restored_cam.pose(), saved_pose);
        r.run("Arctic Off");

        r.run("Open");
        assert_eq!(r.host.open_prompts, 2);
        r.run("Report");
        assert_eq!(r.host.reports_requested, 1);
    }

    let missing: Vec<&'static str> = REGISTRY
        .iter()
        .map(|v| v.spec().names[0])
        .filter(|name| !tested.contains(name))
        .collect();
    assert!(
        missing.is_empty(),
        "every registered verb must be exercised on HeadlessHost; missing ({}/{}): {missing:?}",
        missing.len(),
        REGISTRY.len()
    );
}
