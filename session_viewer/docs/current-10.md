# 10 · Split curves and faces while keeping the shell joined

[Previous](current-9.md) · [Sequence](extend-integrated-tutorial.md) · [Next](current-11.md)

Continue in the same checkpoint workspace. Complete the edits below before compiling.

![Ownership and data flow](illustrations/16-01.svg)

The exact split service is the kernel’s (`session_rust::simple_split`, maintained in its own repository, nothing to type here). Connect source transactions, cutter selection, keyboard and touch confirmation. Keep visible thin curves selectable at pixel boundaries. Curve creates a non-rational NURBS curve from control points (degree up to three). Line and Polyline retain their types after splitting. Existing holes are preserved, all resulting face regions stay in the same BRep and incident boundary edges are updated. No booleans, projection, caps or volume splitting are performed.

### `src/app/command.rs`

**TYPE THIS**

**CURRENT**

```rust
    Scale(f64), // Scale the selection about its own centre.
```

**ADD BELOW**

```rust
    Split,
```

**TYPE THIS**

**CURRENT**

```rust
        "line" => "Line start end · Example: Line 0,0,0 100,0,0",
```

**ADD BELOW**

```rust
        "curve" => "Curve control points… · Example: Curve 0,0,0 50,100,0 100,0,0",
```

**TYPE THIS**

**CURRENT**

```rust
        "explode" => "Select a polyline · Explode creates its individual line segments",
```

**ADD BELOW**

```rust
        "split" => {
            "Select a curve or face · Split · choose cutter curves · Enter confirms · Esc cancels"
        }
```

**TYPE THIS**

**CURRENT**

```rust
        "rotate" | "rot" => Some(2),
        "save" | "open" | "delete" | "del" | "undo" | "redo" | "hide" | "show" | "fit"
        | "escape" | "esc" => Some(0),
        _ => None,
```

**REPLACE WITH**

```rust
        "rotate" | "rot" => Some(2),
        "split" | "save" | "open" | "delete" | "del" | "undo" | "redo" | "hide" | "show"
        | "fit" | "escape" | "esc" => Some(0),
        _ => None,
```

**TYPE THIS**

**CURRENT**

```rust
    match verb.as_str() {
        "point" | "line" | "polyline" | "trim" | "extend" | "explode" => {
            model(&verb, &rest).map(Command::Model)
```

**REPLACE WITH**

```rust
    match verb.as_str() {
        "point" | "line" | "polyline" | "curve" | "trim" | "extend" | "explode" => {
            model(&verb, &rest).map(Command::Model)
```

**TYPE THIS**

**CURRENT**

```rust
        "save" => Ok(Command::Save),
```

**ADD ABOVE**

```rust
        "split" => Ok(Command::Split),
```

**TYPE THIS**

**CURRENT**

```rust
        }
        "point" | "line" | "polyline" => {
            let mut points = Vec::new();
```

**REPLACE WITH**

```rust
        }
        "point" | "line" | "polyline" | "curve" => {
            let mut points = Vec::new();
```

**TYPE THIS**

**CURRENT**

```rust
                ("polyline", 2..) => Ok(Modeling::Polyline(points)),
                _ => Err("point needs one coordinate; line two; polyline at least two".into()),
            }
```

**REPLACE WITH**

```rust
                ("polyline", 2..) => Ok(Modeling::Polyline(points)),
                ("curve", 2..) => Ok(Modeling::Curve(points)),
                _ => {
                    Err("point needs one coordinate; line two; polyline/curve at least two".into())
                }
            }
```

### `src/app/input.rs`

**TYPE THIS**

**CURRENT**

```rust
            Key::Named(NamedKey::Escape) => state.escape_selection(),
```

**ADD BELOW**

```rust
            Key::Named(NamedKey::Enter) => state.confirm_split(),
```

### `src/app/inspection.rs`

**TYPE THIS**

**CURRENT**

```rust
    snapshot["color_count"] = serde_json::json!(state.scene.colors.len());
```

**ADD BELOW**

```rust
    snapshot["split"] = serde_json::json!(state.split_status());
    snapshot["source_faces"] =
        serde_json::json!(parent.and_then(|row| match state.scene.geometry(row)? {
            session_rust::Geometry::BRep(brep) => Some(brep.face_count()),
            session_rust::Geometry::Element(element) => match element.geometry() {
                session_rust::element::ElementGeometry::BRep(brep) => Some(brep.face_count()),
                _ => None,
            },
            _ => None,
        }));
```

### `src/app/mod.rs`

**TYPE THIS**

**CURRENT**

```rust
pub mod surface_preview;
```

**ADD BELOW**

```rust

pub mod splitting;
```

### `src/app/modeling.rs`

**TYPE THIS**

**CURRENT**

```rust
    Polyline(Vec<[f64; 3]>),
```

**ADD BELOW**

```rust
    Curve(Vec<[f64; 3]>),
```

**TYPE THIS**

**CURRENT**

```rust
            _ => self.edit_geometry(command),
```

**ADD ABOVE**

```rust
            Modeling::Curve(points) => {
                if !(2..=MAX_POINTS).contains(&points.len()) {
                    return Err(format!("curve needs 2–{MAX_POINTS} control points"));
                }

                let points = points
                    .iter()
                    .map(|p| point(*p))
                    .collect::<Result<Vec<_>, _>>()?;
                let curve =
                    session_rust::NurbsCurve::create(false, (points.len() - 1).min(3), &points);
                self.create_geometry(Geometry::NurbsCurve(Rc::new(curve)))
            }
```

**TYPE THIS**

**CURRENT**

```rust
            _ => unreachable!(),
```

**ADD ABOVE**

```rust
            Geometry::NurbsCurve(curve) => {
                session.add_nurbscurve((*curve).clone(), None);
            }
```

### `src/app/scene.rs`

**TYPE THIS**

**CURRENT**

```rust

        if is_planar(&self.tables, &from, &place) {
            mark_sheet(&mut self.tables, &from);
```

**REPLACE WITH**

```rust

        // Typed modeling geometry stays in the 3D workspace even when all its points are coplanar.
        if self.created_doc != Some(self.docs.len()) && is_planar(&self.tables, &from, &place) {
            mark_sheet(&mut self.tables, &from);
```

### `src/app/session_io.rs`

**TYPE THIS**

**CURRENT**

```rust
struct Metadata {
```

**ADD BELOW**

```rust
    #[serde(default)]
    created_doc: Option<usize>,
```

**TYPE THIS**

**CURRENT**

```rust
    let metadata = Metadata {
```

**ADD BELOW**

```rust
        created_doc: scene.created_doc,
```

**TYPE THIS**

**CURRENT**

```rust

    let mut scene = Scene::new();
```

**REPLACE WITH**

```rust

    if metadata
        .created_doc
        .is_some_and(|index| index >= metadata.documents.len())
    {
        return Err("Created document index is outside the inventory".into());
    }

    let mut scene = Scene::new();
    scene.created_doc = metadata.created_doc;
```

**TYPE THIS**

**CURRENT**

```rust
    use session_rust::{Geometry, Mesh, Point};
```

**ADD BELOW**

```rust
    #[test]
    fn created_curves_keep_visible_screen_pens_after_open() {
        let mut scene = Scene::new();
        scene
            .model(&crate::app::modeling::Modeling::Line(
                [-3000., -5000., 200.],
                [-3000., -1000., 200.],
            ))
            .unwrap();
        let restored = open(&save(&scene).unwrap()).unwrap();
        assert_eq!(restored.created_doc, Some(0));
        assert_eq!(restored.tables.seg.ribbons.len(), 1);
        assert_eq!(restored.tables.seg.ribbons[0].radius, 0.);
        assert_eq!(
            restored.tables.obj.rows[0].flags & crate::engine::gpu::Instance::FLAG_SHEET,
            0
        );
    }
```

### `src/app/splitting.rs`

**NEW FILE · TYPE THIS**

```rust
use super::scene::Scene;
use session_rust::simple_split;
use session_rust::{BRep, Geometry, NurbsCurve};
use std::rc::Rc;

pub fn is_cutter(geometry: &Geometry) -> bool {
    matches!(
        geometry,
        Geometry::Line(_) | Geometry::Polyline(_) | Geometry::NurbsCurve(_)
    )
}

pub fn face_index(geometry: &Geometry, selected: Option<usize>) -> Result<Option<usize>, String> {
    match geometry {
        Geometry::Line(_)
        | Geometry::Polyline(_)
        | Geometry::NurbsCurve(_)
        | Geometry::NurbsSurface(_) => Ok(None),
        Geometry::BRep(brep) => brep_face(brep, selected).map(Some),
        Geometry::Element(element) => match element.geometry() {
            session_rust::element::ElementGeometry::BRep(brep) => {
                brep_face(brep, selected).map(Some)
            }
            _ => Err("Split accepts curves and NURBS/BRep faces".into()),
        },
        _ => Err("Split accepts lines, polylines, NURBS curves and surface faces".into()),
    }
}

fn brep_face(brep: &BRep, selected: Option<usize>) -> Result<usize, String> {
    selected
        .or((brep.face_count() == 1).then_some(0))
        .filter(|face| *face < brep.face_count())
        .ok_or_else(|| {
            "Ctrl+Shift-select one BRep face before Split; the solid stays joined".into()
        })
}

fn curve(geometry: &Geometry) -> Result<NurbsCurve, String> {
    match geometry {
        Geometry::Line(line) => Ok(NurbsCurve::create(
            false,
            1,
            &[line.point_at(0.), line.point_at(1.)],
        )),
        Geometry::Polyline(polyline) => Ok(NurbsCurve::create(false, 1, &polyline.get_points())),
        Geometry::NurbsCurve(curve) => Ok((**curve).clone()),
        _ => Err("Choose a line, polyline or NURBS curve as cutter".into()),
    }
}

impl Scene {
    /// Preserve the original object identity, placement and tree node; add curve pieces as siblings.
    pub fn split_rows(
        &mut self,
        target: u32,
        face: Option<usize>,
        cutters: &[u32],
    ) -> Result<usize, String> {
        if !self.streamed.is_empty() || !self.sheets.is_empty() {
            return Err("Splitting requires complete retained source documents".into());
        }

        if cutters.is_empty() || cutters.len() > 64 {
            return Err("Choose 1–64 cutter curves".into());
        }

        if !self.selectable(target) {
            return Err("Unlock the target before splitting".into());
        }

        let (doc, guid) = self.identity_of(target).ok_or("Target no longer exists")?;
        let file = self.docs.get(doc).ok_or("Target has no source document")?;

        if file.display_only {
            return Err("Target is display only".into());
        }

        let back = self
            .placement_of(target)
            .ok_or("Target has no placement")?
            .inverse()
            .ok_or("Target placement is singular")?;
        let mut tools = Vec::new();

        for &row in cutters {
            if row == target || !self.selectable(row) {
                return Err("Choose an unlocked cutter distinct from the target".into());
            }

            let mut cutter = curve(self.geometry(row).ok_or("Cutter no longer exists")?)?;
            let place = self.placement_of(row).ok_or("Cutter has no placement")?;

            if place.inverse().is_none() {
                return Err("Cannot transform the cutter into target coordinates".into());
            }

            cutter.transform(&(&back * &place));
            tools.push(cutter);
        }

        let source = self
            .geometry(target)
            .ok_or("Source geometry is unavailable")?;
        let face = face_index(source, face)?;
        let tolerance = 1e-6;
        let (mut pieces, regions) = match source {
            Geometry::Line(line) => {
                let pieces: Vec<_> = simple_split::split_line_by_curves(line, &tools, tolerance)?
                    .into_iter()
                    .map(|p| Geometry::Line(Rc::new(p)))
                    .collect();
                let count = pieces.len();
                (pieces, count)
            }
            Geometry::Polyline(line) => {
                let pieces: Vec<_> =
                    simple_split::split_polyline_by_curves(line, &tools, tolerance)?
                        .into_iter()
                        .map(|p| Geometry::Polyline(Rc::new(p)))
                        .collect();
                let count = pieces.len();
                (pieces, count)
            }
            Geometry::NurbsCurve(curve) => {
                let pieces: Vec<_> = simple_split::split_curve_by_curves(curve, &tools, tolerance)?
                    .into_iter()
                    .map(|p| Geometry::NurbsCurve(Rc::new(p)))
                    .collect();
                let count = pieces.len();
                (pieces, count)
            }
            Geometry::NurbsSurface(surface) => {
                let mut brep = simple_split::split_surface_by_curves(surface, &tools, tolerance)?;
                brep.name = surface.name.clone();
                let count = brep.face_count();
                (vec![Geometry::BRep(Rc::new(brep))], count)
            }
            Geometry::BRep(brep) => {
                let next = simple_split::split_brep_face_by_curves(
                    brep,
                    face.ok_or("Select a face")?,
                    &tools,
                    tolerance,
                )?;
                let count = next.face_count() - brep.face_count() + 1;
                (vec![Geometry::BRep(Rc::new(next))], count)
            }
            Geometry::Element(element) => {
                let session_rust::element::ElementGeometry::BRep(brep) = element.geometry() else {
                    return Err("Element has no BRep".into());
                };
                let next = simple_split::split_brep_face_by_curves(
                    brep,
                    face.ok_or("Select a face")?,
                    &tools,
                    tolerance,
                )?;
                let count = next.face_count() - brep.face_count() + 1;
                let mut result = (**element).clone();
                result.set_brep_geometry(next);
                (vec![Geometry::Element(Rc::new(result))], count)
            }
            _ => return Err("Unsupported split target".into()),
        };

        if regions < 2 {
            return Ok(1);
        }

        let parent_name = file
            .session
            .tree
            .get_node_by_name(&guid)
            .and_then(|node| node.borrow().parent())
            .map(|node| node.borrow().name.clone());
        let place = file.session.xform(&guid);
        let color = self.colors.get(&(doc, Rc::clone(&guid))).copied();

        if pieces.len() > 1 {
            let name = source.name();

            for (index, piece) in pieces.iter_mut().enumerate() {
                let name = format!("{name} (part {})", index + 1);

                match piece {
                    Geometry::Line(p) => Rc::make_mut(p).name = name,
                    Geometry::Polyline(p) => Rc::make_mut(p).name = name,
                    Geometry::NurbsCurve(p) => Rc::make_mut(p).name = name,
                    _ => unreachable!("only curves create sibling objects"),
                }
            }
        }

        let first = pieces.remove(0);
        let session = Rc::make_mut(&mut self.docs[doc].session);
        // Resolve the parent after copy-on-write, inside the edited document's tree.
        let parent = parent_name.and_then(|name| session.tree.get_node_by_name(&name));
        session.begin("split");
        // All fallible geometry work has completed; these validated pieces have at least two controls.
        let replaced = session.replace(&guid, first);
        debug_assert!(replaced);

        for piece in pieces {
            let node = match piece {
                Geometry::Line(piece) => session.add_line((*piece).clone(), parent.as_ref()),
                Geometry::Polyline(piece) => session
                    .add_polyline((*piece).clone(), parent.as_ref())
                    .expect("valid split polyline"),
                Geometry::NurbsCurve(piece) => session
                    .add_nurbscurve((*piece).clone(), parent.as_ref())
                    .expect("valid split curve"),
                _ => unreachable!("only curves create sibling objects"),
            };
            let id = node.borrow().name.clone();
            session.set_xform(&id, place.clone());

            if let Some(color) = color {
                self.colors.insert((doc, Rc::from(id)), color);
            }
        }

        session.commit();
        self.last_edited = Some(doc);
        Ok(regions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::scene::FileDoc;
    use session_rust::Xform;
    use session_rust::{Line, Point, Session};

    fn add(scene: &mut Scene, session: Rc<Session>, name: &str, place: Xform) {
        scene.add_file(FileDoc {
            name: name.into(),
            session,
            place,
            point_px: 0.,
            display_only: false,
        });
    }

    #[test]
    fn split_preserves_tree_placement_and_other_shared_documents_and_undo() {
        let mut session = Session::new("shared");
        let group = session.add_group("parts");
        session.set_xform(&group.borrow().name, Xform::translation(10., 0., 0.));
        let target = session.add_line(
            Line::from_points(&Point::new(-2., 0., 0.), &Point::new(2., 0., 0.)),
            Some(&group),
        );
        let id = target.borrow().name.clone();
        let shared = Rc::new(session);
        let mut scene = Scene::new();
        add(
            &mut scene,
            Rc::clone(&shared),
            "first",
            Xform::translation(100., 0., 0.),
        );
        add(
            &mut scene,
            Rc::clone(&shared),
            "second",
            Xform::translation(200., 0., 0.),
        );
        let mut cutters = Session::new("cutters");
        cutters.add_line(
            Line::from_points(&Point::new(110., -2., 0.), &Point::new(110., 2., 0.)),
            None,
        );
        add(&mut scene, Rc::new(cutters), "cutters", Xform::identity());
        scene
            .colors
            .insert(scene.identity_of(0).unwrap(), [60, 170, 100]);
        assert_eq!(scene.split_rows(0, None, &[2]).unwrap(), 2);
        let first = &scene.docs[0].session;
        assert_eq!(first.objects.lines.len(), 2);
        assert_eq!(scene.docs[1].session.objects.lines.len(), 1);
        assert_eq!(shared.objects.lines.len(), 1);
        let parent = first.tree.get_node_by_name("parts").unwrap();
        assert_eq!(parent.borrow().children().len(), 2);
        assert_eq!(
            shared
                .tree
                .get_node_by_name("parts")
                .unwrap()
                .borrow()
                .children()
                .len(),
            1
        );
        assert!(first.lookup.contains_key(&id));
        let Geometry::Line(line) = &first.lookup[&id] else {
            panic!()
        };
        assert!(line.point_at(1.).distance(&Point::new(0., 0., 0.), None) < 1e-6);
        assert_eq!(scene.colors.len(), 2);
        assert!(scene.undo());
        assert_eq!(scene.docs[0].session.objects.lines.len(), 1);
        assert!(scene.redo());
        assert_eq!(scene.docs[0].session.objects.lines.len(), 2);
        let bytes = crate::app::session_io::save(&scene).unwrap();
        let restored = crate::app::session_io::open(&bytes).unwrap();
        assert_eq!(restored.docs[0].session.objects.lines.len(), 2);
    }

    #[test]
    fn split_face_keeps_solid_joined_and_invalid_cut_preserves_source() {
        let brep = BRep::create_box(10., 10., 10.);
        let original_area = brep.face_meshes_q(Some((20., 0.005)))[0].area();
        let s = &brep.m_surfaces[0];
        let a = s.get_cv(0, 0).unwrap();
        let u = s.get_cv(1, 0).unwrap();
        let v = s.get_cv(0, 1).unwrap();
        let p = |x: f64, y: f64| {
            Point::new(
                a[0] + x * (u[0] - a[0]) + y * (v[0] - a[0]),
                a[1] + x * (u[1] - a[1]) + y * (v[1] - a[1]),
                a[2] + x * (u[2] - a[2]) + y * (v[2] - a[2]),
            )
        };
        let mut session = Session::new("box");
        let node = session.add_brep(brep, None).unwrap();
        let id = node.borrow().name.clone();
        session.add_line(Line::from_points(&p(0.5, -1.), &p(0.5, 2.)), None);
        let mut scene = Scene::new();
        add(&mut scene, Rc::new(session), "box", Xform::identity());
        let row = (0..scene.object_count() as u32)
            .find(|&row| matches!(scene.geometry(row), Some(Geometry::BRep(_))))
            .unwrap();
        let cutter = (0..scene.object_count() as u32)
            .find(|&row| matches!(scene.geometry(row), Some(Geometry::Line(_))))
            .unwrap();
        assert!(scene.split_rows(row, None, &[cutter]).is_err());
        assert_eq!(scene.split_rows(row, Some(0), &[cutter]).unwrap(), 2);
        let Geometry::BRep(result) = &scene.docs[0].session.lookup[&id] else {
            panic!()
        };
        assert_eq!(result.face_count(), 7);
        assert!(result.is_solid());
        let meshes = result.face_meshes_q(Some((20., 0.005)));
        assert!(
            (meshes[0].area() - original_area / 2.).abs() < 1e-6,
            "first region area: {} of {original_area}",
            meshes[0].area()
        );
        assert!(
            (meshes[6].area() - original_area / 2.).abs() < 1e-6,
            "second region area: {} of {original_area}",
            meshes[6].area()
        );
        assert!(scene.undo());
        let Geometry::BRep(original) = &scene.docs[0].session.lookup[&id] else {
            panic!()
        };
        assert_eq!(original.face_count(), 6);
        assert!(original.is_solid());
    }
}
```

### `src/app/ui.rs`

**TYPE THIS**

**CURRENT**

```rust
                    model.command_open = false;
                    response.surrender_focus();
                    crate::app::feedback::focus_canvas();
```

**REPLACE WITH**

```rust
                    model.command_open = false;
                    model.focus_command = false;
                    response.surrender_focus();
                    close.surrender_focus();
                    crate::app::feedback::focus_canvas();
```

**TYPE THIS**

**CURRENT**

```rust
    ("Undo", "Undo the last edit", "undo"),
```

**ADD ABOVE**

```rust
    (
        "Split",
        "Split selected curve or face with cutter curves",
        "split",
    ),
```

**TYPE THIS**

**CURRENT**

```rust
                        if command.contains(' ') {
```

**ADD ABOVE**

```rust
                        response.surrender_focus();
```

### `src/shaders/ribbon.wgsl`

**TYPE THIS**

**CURRENT**

```wgsl
@fragment
fn fs_id(in: VsOut) -> @location(0) vec2<u32> {
    if (coverage(in) < 0.5 || !ink_visible(in.pos.xy, ink_axis(in), 0u)) {
        discard;
```

**REPLACE WITH**

```wgsl
@fragment
// Keep visible hairlines pickable even when their coverage is shared across adjacent pixels.
fn fs_id(in: VsOut) -> @location(0) vec2<u32> {
    if (coverage(in) <= 0.0 || !ink_visible(in.pos.xy, ink_axis(in), 0u)) {
        discard;
```

**TYPE THIS**

**CURRENT**

```wgsl
fn fs_edge_id(in: VsOut) -> @location(0) vec2<u32> {
    if (in.source_edge == 0xffffffffu || coverage(in) < 0.5 || !ink_visible(in.pos.xy, ink_axis(in), 0u)) {
        discard;
```

**REPLACE WITH**

```wgsl
fn fs_edge_id(in: VsOut) -> @location(0) vec2<u32> {
    if (in.source_edge == 0xffffffffu || coverage(in) <= 0.0 || !ink_visible(in.pos.xy, ink_axis(in), 0u)) {
        discard;
```

### `src/state.rs`

**TYPE THIS**

**CURRENT**

```rust
mod sheet_query;
```

**ADD BELOW**

```rust
mod splitting;
```

**TYPE THIS**

**CURRENT**

```rust
    hierarchy: crate::app::hierarchy::Hierarchy,
```

**ADD BELOW**

```rust
    pending_split: Option<splitting::Pending>,
```

**TYPE THIS**

**CURRENT**

```rust
            hierarchy: Default::default(),
```

**ADD BELOW**

```rust
            pending_split: None,
```

**TYPE THIS**

**CURRENT**

```rust
    pub fn clear(&mut self) {
```

**ADD BELOW**

```rust
        self.cancel_split();
```

**TYPE THIS**

**CURRENT**

```rust
    pub fn select(&mut self, row: Option<u32>) {
```

**ADD BELOW**

```rust
        self.cancel_split();
```

**TYPE THIS**

**CURRENT**

```rust
    fn apply_pick(&mut self, pick: Option<Pick>) {
```

**ADD BELOW**

```rust
        if self.pending_split.is_some() {
            if let Some(pick) = pick {
                self.pick_split_cutter(pick.row);
            }

            return;
        }
```

**TYPE THIS**

**CURRENT**

```rust
    pub fn request_selection(&mut self, x: u32, y: u32, edge: bool, face: bool) {
        let face = face || self.selection_tool == crate::app::selection::SelectionTool::Face;
        let edge = edge || self.selection_tool == crate::app::selection::SelectionTool::Edge;
        self.cancel_cloud_query();
        self.gpu.pick.cancel();

        #[cfg(target_arch = "wasm32")]
        if !edge && self.start_cloud_query(x, y) {
            return;
        }

        let mode = if face {
            PickMode::Component
```

**REPLACE WITH**

```rust
    pub fn request_selection(&mut self, x: u32, y: u32, edge: bool, face: bool) {
        let splitting = self.pending_split.is_some();
        let face = !splitting
            && (face || self.selection_tool == crate::app::selection::SelectionTool::Face);
        let edge = !splitting
            && (edge || self.selection_tool == crate::app::selection::SelectionTool::Edge);
        self.cancel_cloud_query();
        self.gpu.pick.cancel();

        #[cfg(target_arch = "wasm32")]
        if !splitting && !edge && self.start_cloud_query(x, y) {
            return;
        }

        let mode = if splitting {
            PickMode::Object
        } else if face {
            PickMode::Component
```

### `src/state/edit.rs`

**TYPE THIS**

**CURRENT**

```rust
    /// patched. The selection is dropped because the row it named may not exist any more.
    fn after_history(&mut self) {
        self.hierarchy.open.clear();
```

**REPLACE WITH**

```rust
    /// patched. The selection is dropped because the row it named may not exist any more.
    pub(super) fn after_history(&mut self) {
        self.hierarchy.open.clear();
```

**TYPE THIS**

**CURRENT**

```rust
        if matches!(command, Command::Delete | Command::Undo | Command::Redo)
```

**ADD ABOVE**

```rust
        if command != Command::Split {
            self.cancel_split();
        }
```

**TYPE THIS**

**CURRENT**

```rust
        match command {
```

**ADD BELOW**

```rust
            Command::Split => self.split_command(),
```

**TYPE THIS**

**CURRENT**

```rust
                    command,
                    Modeling::Point(_) | Modeling::Line(..) | Modeling::Polyline(_)
                );
```

**REPLACE WITH**

```rust
                    command,
                    Modeling::Point(_)
                        | Modeling::Line(..)
                        | Modeling::Polyline(_)
                        | Modeling::Curve(_)
                );
```

**TYPE THIS**

**CURRENT**

```rust
                        Modeling::Line(..) => "line",
```

**ADD BELOW**

```rust
                        Modeling::Curve(_) => "NURBS curve",
```

### `src/state/panel.rs`

**TYPE THIS**

**CURRENT**

```rust
                    let rows = self.hierarchy.targets(index);
                    self.select(None);
```

**REPLACE WITH**

```rust
                    let rows = self.hierarchy.targets(index);

                    if self.pending_split.is_some() {
                        for row in rows {
                            self.pick_split_cutter(row);
                        }

                        return;
                    }

                    self.select(None);
```

### `src/state/splitting.rs`

**NEW FILE · TYPE THIS**

```rust
use super::State;
use crate::app::{feedback, splitting};

pub(super) struct Pending {
    pub target: u32,
    pub face: Option<usize>,
    pub cutters: Vec<u32>,
}

impl State {
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn split_status(&self) -> Option<(u32, Option<usize>, &[u32])> {
        self.pending_split
            .as_ref()
            .map(|p| (p.target, p.face, p.cutters.as_slice()))
    }

    pub(super) fn split_command(&mut self) -> Result<String, String> {
        if self.pending_split.is_some() {
            return self.finish_split();
        }

        let row = self
            .scene
            .selected
            .ok_or("Select a curve or Ctrl+Shift-select a face, then run Split")?;
        let selected = match self.selection {
            crate::app::selection::SelectionMode::Face { face, .. } => Some(face),
            _ => None,
        };
        let face = splitting::face_index(
            self.scene.geometry(row).ok_or("Source unavailable")?,
            selected,
        )?;
        self.pending_split = Some(Pending {
            target: row,
            face,
            cutters: vec![],
        });
        self.place_gizmo(None);
        feedback::command_line(false);
        feedback::focus_canvas();
        Ok("Split: select cutter lines, polylines or curves, then press Enter or tap Split again. Esc cancels. Faces require on-surface cutters.".into())
    }

    pub(super) fn cancel_split(&mut self) {
        if let Some(pending) = self.pending_split.take() {
            for row in pending.cutters {
                self.gpu.set_selected(row, false);
            }
        }
    }

    pub(super) fn pick_split_cutter(&mut self, row: u32) {
        let Some(pending) = self.pending_split.as_mut() else {
            return;
        };

        if row == pending.target
            || !self.scene.selectable(row)
            || !self.scene.geometry(row).is_some_and(splitting::is_cutter)
        {
            self.status(
                "Choose an unlocked line, polyline or NURBS curve distinct from the target",
            );
            return;
        }

        if let Some(at) = pending.cutters.iter().position(|item| *item == row) {
            pending.cutters.remove(at);
            self.gpu.set_selected(row, false);
        } else if pending.cutters.len() < 64 {
            pending.cutters.push(row);
            self.gpu.set_selected(row, true);
        }

        let count = pending.cutters.len();
        self.status(&format!(
            "Split: {count} cutter curves selected. Enter or Split confirms; Esc cancels."
        ));
        self.touch();
    }

    pub fn confirm_split(&mut self) {
        if self.pending_split.is_some() {
            let message = self.finish_split().unwrap_or_else(|error| error);
            self.status(&message);
            self.touch();
        }
    }

    fn finish_split(&mut self) -> Result<String, String> {
        let pending = self.pending_split.take().ok_or("Start Split first")?;

        if pending.cutters.is_empty() {
            self.pending_split = Some(pending);
            return Err("Select at least one cutter curve, then press Enter".into());
        }

        for &row in &pending.cutters {
            self.gpu.set_selected(row, false);
        }

        let identity = self.scene.identity_of(pending.target);
        let result = self
            .scene
            .split_rows(pending.target, pending.face, &pending.cutters);

        match result {
            Ok(regions) if regions > 1 => {
                self.after_history();
                let row = identity.and_then(|id| {
                    (0..self.scene.object_count() as u32)
                        .find(|&row| self.scene.identity_of(row).as_ref() == Some(&id))
                });
                self.select(row);
                Ok(format!(
                    "Split into {regions} regions. The BRep stays joined; Undo restores the original. Cutters are retained."
                ))
            }
            Ok(_) => {
                self.place_gizmo(self.scene.selected);
                Ok("No division: cutters must cross the curve or lie on the selected face.".into())
            }
            Err(error) => {
                self.place_gizmo(self.scene.selected);
                Err(error)
            }
        }
    }
}
```

### Check step 10

**Verified:** the complete step compiles for WebAssembly.

```bash
cargo check -j4 --lib
```


## Answers and next action

**How are trims handled?** A face has an outer trim and optional hole trims in surface parameter space. Cutter curves must lie on that surface within tolerance. The kernel partitions this trimmed region, preserves every region and hole, and updates shared boundary edges in neighboring faces. It rejects ambiguous overlaps and unsupported seam cases before replacing the source.

**Does the solid stay joined?** Yes. Select a face with Ctrl+Shift or the Face toolbar, run Split, then choose cutter curves and press Enter or tap Split again. A multi-face BRep needs an explicit face. The operation returns the owning BRep with its shell and solid relationships retained. It subdivides faces, without producing separate volumes.

**What is saved?** The original source is replaced in one undoable transaction. Curve pieces inherit the original placement, parent and color; cutters remain. Save/Open serializes the complete edited session.

**Run now**, in the same learning workspace:

```bash
cargo check -j4 --lib
trunk serve --port 8780
```

Expected compiler result: `Finished` with no errors. Open <http://localhost:8780/?data=off&inspect=1>. For the pictured shell, download [split.pb](extensions/split.pb) into assets/extension-split.pb and [split.yaml](extensions/split.yaml) into assets/extension-split.yaml. Open http://localhost:8780/?scene=extension-split.yaml&data=off&inspect=1, create Line 0,-150,20 0,150,20, select the top face and split it with that line. For the curve-only example, create Line -100,40,0 100,40,0 and Line 0,-60,0 0,140,0. Clear the selection and Fit, select the first line, press Split, pick the second line and press Enter or Split again. You should see two target pieces and the retained cutter. Undo restores the original; Redo and Save/Open retain the pieces. On the example shell, split its top face with a line lying on that face and confirm that both regions remain joined.

Stop the server with **Ctrl+C** before editing the next checkpoint. Then open [Separate face and edge colors and keep large-object dragging live](current-11.md) and apply its blocks in order.

[Previous](current-9.md) · [Sequence](extend-integrated-tutorial.md) · [Next](current-11.md)

## Expected viewer result

The shell has been divided into two face regions by an on-surface line. The cutter remains and the selected half is highlighted. The shell stays joined, and the editable session stores the updated source geometry. Use Undo to restore the original or Save to keep this result. See the [phone layout](screenshots/extensions-split-phone.png).

[![Full viewer result for current 10](screenshots/extensions-split-face.png)](screenshots/extensions-split-face.png)
