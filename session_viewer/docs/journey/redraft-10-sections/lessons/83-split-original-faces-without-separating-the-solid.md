# 16.3 · Joined-face topology and exact surface measures

**Goal:** Reuse mesh area for meshes, meshed BReps and Elements. · **Steps:** 4 · **Source:** [app/command/verbs/project_to_plane.rs](../../../../src/app/command/verbs/project_to_plane.rs) · [app/command/verbs/extend.rs](../../../../src/app/command/verbs/extend.rs) · [app/splitting.rs](../../../../src/app/splitting.rs) · [app/command/verbs/measure.rs](../../../../src/app/command/verbs/measure.rs) · [app/command/verbs/trim/parts.rs](../../../../src/app/command/verbs/trim/parts.rs) · [app/command/verbs/explode.rs](../../../../src/app/command/verbs/explode.rs) · [app/walk/frames.rs](../../../../src/app/walk/frames.rs) (`src/app/booleans.rs,src/app/measurements.rs,src/app/recipes.rs`)

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="200" viewBox="0 0 1120 200" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="196" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 16.3 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/split_draft.rs</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/split_face_tests.rs</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">4 Coding Steps</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/app/booleans.rs</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/app/measurements.rs</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Split faces without separ…</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. B-Rep Boolean (A ∪ / ∖ / ∩ B)</text>
  <rect x="618" y="56" width="84" height="68" rx="4" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <rect x="676" y="72" width="84" height="68" rx="4" fill="#fee2e2" stroke="#111111" stroke-width="1.75"/>
  <rect x="676" y="72" width="26" height="52" fill="#fef08a" stroke="#dc2626" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="689" y="102" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">SSI</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Joined-face topology and ex…</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. Split, Trim &amp; Dimensions</text>
  <line x1="896" y1="92" x2="1056" y2="92" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <line x1="896" y1="76" x2="896" y2="108" stroke="#111111" stroke-width="1.75"/>
  <line x1="1056" y1="76" x2="1056" y2="108" stroke="#111111" stroke-width="1.75"/>
  <rect x="932" y="54" width="88" height="24" rx="4" fill="#fef9c3" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="976" y="70" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">124.50 mm</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="976" y="132" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Area / Volume / Mass</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Integrate surface area</text>
</svg>
</div>

## <a id="checkpoint-83-split-original-faces-without-separating-the-solid"></a>1. Split faces without separating the solid

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 1 of 4</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/split_draft.rs, src/spl…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original selected face</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">joined trimmed regions</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/booleans.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">one transaction</text>
</svg>
</div>

**What this code does:** Divide a selected original surface face while preserving joined topology and one-step history.

### 1. `src/split_draft.rs`

Allow original surfaces and validated BRep faces through the shared pending split owner.

**Locate this existing block:**

```rust
        let target = Owner::capture(scene, id).map_err(str::to_owned)?;
        if !crate::split_geometry::is_cutter(&target.source) {
            return Err("Select a curve; surface-face splitting connects next".into());
        }
        let face = crate::split_geometry::face_index(&target.source, face)?;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        let target = Owner::capture(scene, id).map_err(str::to_owned)?;
        let face = crate::split_geometry::face_index(&target.source, face)?;
```

### 2. `src/split_face_tests.rs`

Copy integrated maintained joined-face, original placement, topology, area, archive and history fixtures.

Create the file and type:

```rust
use crate::{
    editor::{Action, Editor},
    scene::Scene,
};
use session_rust::{BRep, Line, Point, Xform};
use std::rc::Rc;
fn editor() -> Editor {
    let mut e = Editor::default();
    e.apply(Action::Close).unwrap();
    e
}
fn cutter(e: &mut Editor, a: &Point, b: &Point) -> crate::scene::ObjectId {
    e.scene
        .insert_line(
            crate::stroke::PreparedLine::new(Rc::new(Line::from_points(a, b))).unwrap(),
        )
        .unwrap()
}
#[test]
fn split_face_keeps_solid_joined_original_area_identity_and_one_undo() {
    let mut e = editor();
    let brep = Rc::new(BRep::create_box(10., 10., 10.));
    let original_area = brep.face_meshes_q(Some((20., 0.005)))[0].area();
    let s = &brep.m_surfaces[0];
    let (a, u, v) = (
        s.get_cv(0, 0).unwrap(),
        s.get_cv(1, 0).unwrap(),
        s.get_cv(0, 1).unwrap(),
    );
    let p = |x: f64, y: f64| {
        Point::new(
            a[0] + x * (u[0] - a[0]) + y * (v[0] - a[0]),
            a[1] + x * (u[1] - a[1]) + y * (v[1] - a[1]),
            a[2] + x * (u[2] - a[2]) + y * (v[2] - a[2]),
        )
    };
    let target = e.scene.insert_cad(brep.clone()).unwrap();
    let cut = cutter(&mut e, &p(0.5, -1.), &p(0.5, 2.));
    let guid = e.scene.row_guid(target).unwrap().to_owned();
    e.select_objects([target], false);
    assert!(e.apply(Action::Split).is_err());
    assert!(e.split.is_none());
    assert!(crate::split_draft::Draft::new(&e.scene, target, Some(99)).is_err());
    e.apply(Action::PickedTarget(Some(
        crate::pick_sources::Target {
            owner: target,
            part: crate::pick_sources::Part::Face(0),
        },
    )))
    .unwrap();
    e.apply(Action::Split).unwrap();
    assert_eq!(e.split.as_ref().unwrap().face, Some(0));
    e.apply(Action::SplitPick(Some(cut)))
        .unwrap();
    e.apply(Action::Split).unwrap();
    let result = e.scene.objects()[0]
        .cad
        .as_ref()
        .unwrap()
        .source
        .clone();
    assert_eq!(result.face_count(), 7);
    assert!(result.is_solid());
    assert_eq!(brep.face_count(), 6);
    assert_eq!(e.scene.row_guid(target), Some(guid.as_str()));
    assert_eq!(e.scene.objects().len(), 1);
    assert_eq!(e.scene.lines().len(), 1);
    let meshes = result.face_meshes_q(Some((20., 0.005)));
    assert!((meshes[0].area() - original_area / 2.).abs() < 1e-6);
    assert!((meshes[6].area() - original_area / 2.).abs() < 1e-6);
    let mut reopened = Scene::empty();
    crate::scene_document::open(
        &mut reopened,
        &crate::scene_archive::save(&e.scene).unwrap(),
    )
    .unwrap();
    assert_eq!(
        reopened.objects()[0]
            .cad
            .as_ref()
            .unwrap()
            .source
            .to_proto(),
        result.to_proto()
    );
    e.apply(Action::Undo).unwrap();
    assert!(Rc::ptr_eq(
        &brep,
        &e.scene.objects()[0]
            .cad
            .as_ref()
            .unwrap()
            .source
    ));
    e.apply(Action::Redo).unwrap();
    assert_eq!(
        e.scene.objects()[0]
            .cad
            .as_ref()
            .unwrap()
            .source
            .face_count(),
        7
    );
}
#[test]
fn placed_original_surface_becomes_one_trimmed_brep_with_the_same_guid_and_exact_archive()
 {
    let mut e = editor();
    let surface = Rc::new(
        crate::patch_geometry::patch(&[
            Point::new(-2., -2., 0.),
            Point::new(2., -2., 0.),
            Point::new(2., 2., 0.),
            Point::new(-2., 2., 0.),
        ])
        .unwrap(),
    );
    let target = e
        .scene
        .insert_surface(surface.clone())
        .unwrap();
    let model = Xform::translation(100., 20., 3.);
    e.scene
        .place_owner(target, model.clone())
        .unwrap();
    let cut = cutter(
        &mut e,
        &Point::new(100., 17., 3.),
        &Point::new(100., 23., 3.),
    );
    let guid = e.scene.row_guid(target).unwrap().to_owned();
    e.select_objects([target], false);
    e.apply(Action::Split).unwrap();
    e.apply(Action::SplitPick(Some(cut)))
        .unwrap();
    e.apply(Action::Split).unwrap();
    let row = &e.scene.objects()[0];
    assert_eq!(row.id, target);
    assert_eq!(row.model.m, model.m);
    assert!(row.surface.is_none());
    let brep = row.cad.as_ref().unwrap().source.clone();
    assert_eq!(brep.face_count(), 2);
    assert_eq!(brep.guid(), surface.guid());
    assert_eq!(e.scene.row_guid(target), Some(guid.as_str()));
    assert_eq!(e.scene.objects().len(), 1);
    let meshes = brep.face_meshes_q(Some((20., 0.005)));
    assert!((meshes.iter().map(|m| m.area()).sum::<f64>() - 16.).abs() < 1e-6);
    let mut reopened = Scene::empty();
    crate::scene_document::open(
        &mut reopened,
        &crate::scene_archive::save(&e.scene).unwrap(),
    )
    .unwrap();
    assert_eq!(
        reopened.objects()[0]
            .cad
            .as_ref()
            .unwrap()
            .source
            .to_proto(),
        brep.to_proto()
    );
    assert_eq!(reopened.objects()[0].model.m, model.m);
    e.apply(Action::Undo).unwrap();
    assert!(Rc::ptr_eq(
        &surface,
        &e.scene.objects()[0]
            .surface
            .as_ref()
            .unwrap()
            .source
    ));
}
#[test]
fn off_surface_cutters_make_no_division_and_preserve_originals_and_redo() {
    let mut e = editor();
    let source = Rc::new(
        crate::patch_geometry::patch(&[
            Point::new(-2., -2., 0.),
            Point::new(2., -2., 0.),
            Point::new(2., 2., 0.),
            Point::new(-2., 2., 0.),
        ])
        .unwrap(),
    );
    let target = e
        .scene
        .insert_surface(source.clone())
        .unwrap();
    let cut = cutter(
        &mut e,
        &Point::new(0., -3., 1.),
        &Point::new(0., 3., 1.),
    );
    e.apply(Action::AddPoint).unwrap();
    e.apply(Action::Undo).unwrap();
    e.select_objects([target], false);
    e.apply(Action::Split).unwrap();
    e.apply(Action::SplitPick(Some(cut)))
        .unwrap();
    e.apply(Action::Split).unwrap();
    assert!(e.tool_status.starts_with("No division"));
    assert!(e.split.is_none());
    assert!(Rc::ptr_eq(
        &source,
        &e.scene.objects()[0]
            .surface
            .as_ref()
            .unwrap()
            .source
    ));
    e.apply(Action::Redo).unwrap();
    assert_eq!(e.scene.points().len(), 1);
}
```

### 3. `src/lib.rs`

Register the copied original-face split fixtures.

**Locate this existing block:**

```rust
pub mod split_draft;

pub mod source_mesh_topology;
#[cfg(test)]
mod split_tests;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod split_draft;

#[cfg(test)]
mod split_tests;

pub mod source_mesh_topology;
#[cfg(test)]
mod split_face_tests;

#[cfg(target_arch = "wasm32")]
```

## <a id="checkpoint-83za-visit-original-mesh-triangles-and-format-measures"></a>2. Visit mesh triangles and format measures

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 2 of 4</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/measure_math.rs, src/li…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original triangles</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">world transform</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/booleans.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">area or signed swept volu…</text>
</svg>
</div>

**What this code does:** Write one visitor for the original mesh triangles in world coordinates.

### 1. `src/measure_math.rs`

Type the whole original mesh measurement helpers using the shared visitor.

Create the file and type:

```rust
use session_rust::{Mesh, Point, Vector, Xform};
/// Visit the triangles the viewer draws for face `face`, in world coordinates: the cached triangulation, else a fan.
pub fn for_each_triangle(
    mesh: &Mesh,
    face: usize,
    place: &Xform,
    visit: impl FnMut([Point; 3]),
) {
    let at = |key: usize| {
        mesh.vertex_point(key)
            .map(|p| p.transformed(place))
    };
    let triangle = |[a, b, c]: [usize; 3]| Some([at(a)?, at(b)?, at(c)?]);

    if let Some(cached) = mesh
        .triangulation
        .get(&face)
        .filter(|tris| !tris.is_empty())
    {
        cached
            .iter()
            .filter_map(|&keys| triangle(keys))
            .for_each(visit);
        return;
    }

    let corners = mesh
        .face
        .get(&face)
        .map_or(&[][..], Vec::as_slice);
    (1..corners.len().saturating_sub(1))
        .filter_map(|i| triangle([corners[0], corners[i], corners[i + 1]]))
        .for_each(visit);
}

/// A value with at most three decimals, no trailing zeros, and no minus on zero.
pub fn to_text(value: f64) -> String {
    // too small for three decimals but not zero
    if value != 0.0 && value.abs() < 0.0005 {
        return format!("{value:.3e}");
    }

    let text = format!("{value:.3}");
    let text = text
        .trim_end_matches('0')
        .trim_end_matches('.');

    match text {
        "-0" => "0".into(),
        text => text.into(),
    }
}

/// `count` followed by `one` or its plural.
pub fn plural(count: usize, one: &str) -> String {
    match count {
        1 => format!("1 {one}"),
        _ => format!("{count} {one}s"),
    }
}

/// ` · N other objects skipped`, or nothing.
pub fn skipped_text(count: usize) -> String {
    match count {
        0 => String::new(),
        _ => format!(
            " · {} skipped",
            plural(count, "other object")
        ),
    }
}
```

### 2. `src/lib.rs`

Compile this shared original measurement module.

**Locate this existing block:**

```rust
mod split_tests;

pub mod source_mesh_topology;
#[cfg(test)]
mod split_face_tests;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod split_tests;

#[cfg(test)]
mod split_face_tests;

pub mod measure_math;
pub mod source_mesh_topology;

#[cfg(target_arch = "wasm32")]
```

## <a id="checkpoint-83zb-compute-original-mesh-measures-and-curve-length"></a>3. Compute mesh measures and curve length

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 3 of 4</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/measure_math.rs, src/me…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original geometry</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">placed source</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/booleans.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">measure or refusal</text>
</svg>
</div>

**What this code does:** Reuse the triangle visitor for face area, mesh area and signed swept volume.

### 1. `src/measure_math.rs`

Reuse the original triangle visitor for mesh area and signed swept volume. Supply only the unchanged original mesh measurement fixtures.

**Locate this existing block:**

```rust
}

/// A value with at most three decimals, no trailing zeros, and no minus on zero.
pub fn to_text(value: f64) -> String {
    // too small for three decimals but not zero
    if value != 0.0 && value.abs() < 0.0005 {
        return format!("{value:.3e}");
    }

    let text = format!("{value:.3}");
    let text = text
        .trim_end_matches('0')
        .trim_end_matches('.');

    match text {
        "-0" => "0".into(),
        text => text.into(),
    }
}

/// `count` followed by `one` or its plural.
pub fn plural(count: usize, one: &str) -> String {
    match count {
        1 => format!("1 {one}"),
        _ => format!("{count} {one}s"),
    }
}

/// ` · N other objects skipped`, or nothing.
pub fn skipped_text(count: usize) -> String {
    match count {
        0 => String::new(),
        _ => format!(
            " · {} skipped",
            plural(count, "other object")
        ),
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

/// Area of one mesh face in world units; a fan that folds back (a concave polygon) sums with signs, so it stays exact.
pub fn compute_face_area(mesh: &Mesh, face: usize, place: &Xform) -> f64 {
    let cached = mesh
        .triangulation
        .get(&face)
        .is_some_and(|tris| !tris.is_empty());
    let mut summed = Vector::new(0.0, 0.0, 0.0); // signed fan normals
    let mut unsigned = 0.0; // triangle areas, doubled

    for_each_triangle(mesh, face, place, |[a, b, c]| {
        let cross = (&b - &a).cross(&(&c - &a));
        unsigned += cross.magnitude();
        summed += cross;
    });

    if cached || unsigned - summed.magnitude() <= 1e-12 * unsigned {
        return unsigned * 0.5;
    }

    // a warped face whose fan never folds back is the drawn area
    let mut folded = false;

    for_each_triangle(mesh, face, place, |[a, b, c]| {
        folded |= (&b - &a).cross(&(&c - &a)).dot(&summed) < 0.0;
    });

    if !folded {
        return unsigned * 0.5;
    }

    summed.magnitude() * 0.5
}

/// Area of a mesh in world units: every face, or only `face`.
pub fn compute_mesh_area(mesh: &Mesh, place: &Xform, face: Option<usize>) -> f64 {
    match face {
        Some(face) => compute_face_area(mesh, face, place),
        None => mesh
            .faces()
            .into_iter()
            .map(|face| compute_face_area(mesh, face, place))
            .sum(),
    }
}

/// Six times the signed volume the mesh's triangles sweep from `origin`, in world units.
pub fn compute_swept(mesh: &Mesh, place: &Xform, origin: &Point) -> f64 {
    let mut total = 0.0;

    for face in mesh.faces() {
        for_each_triangle(mesh, face, place, |[a, b, c]| {
            total += (&a - origin).dot(&(&b - origin).cross(&(&c - origin)));
        });
    }

    total
}

/// A value with at most three decimals, no trailing zeros, and no minus on zero.
pub fn to_text(value: f64) -> String {
    // too small for three decimals but not zero
    if value != 0.0 && value.abs() < 0.0005 {
        return format!("{value:.3e}");
    }

    let text = format!("{value:.3}");
    let text = text
        .trim_end_matches('0')
        .trim_end_matches('.');

    match text {
        "-0" => "0".into(),
        text => text.into(),
    }
}

/// `count` followed by `one` or its plural.
pub fn plural(count: usize, one: &str) -> String {
    match count {
        1 => format!("1 {one}"),
        _ => format!("{count} {one}s"),
    }
}

/// ` · N other objects skipped`, or nothing.
pub fn skipped_text(count: usize) -> String {
    match count {
        0 => String::new(),
        _ => format!(
            " · {} skipped",
            plural(count, "other object")
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn values_print_short() {
        assert_eq!(to_text(141.42135623730951), "141.421");
        assert_eq!(to_text(100.0), "100");
        assert_eq!(to_text(-0.0), "0");
        assert_eq!(to_text(-0.0001 * 0.0), "0");
        assert_eq!(to_text(1.6000000000000003), "1.6");
        assert_eq!(to_text(-40.0), "-40");
        assert_eq!(to_text(0.0004), "4.000e-4");
        assert_eq!(to_text(-0.0002), "-2.000e-4");
        assert_eq!(plural(1, "curve"), "1 curve");
        assert_eq!(plural(3, "curve"), "3 curves");
        assert_eq!(skipped_text(0), "");
        assert_eq!(skipped_text(2), " · 2 other objects skipped");
    }

    /// An L-shaped hexagon: its fan overlaps, the signed sum does not.
    #[test]
    fn a_concave_face_measures_exactly() {
        let ring = [
            (2.0, 1.0),
            (1.0, 1.0),
            (1.0, 2.0),
            (0.0, 2.0),
            (0.0, 0.0),
            (2.0, 0.0),
        ];
        let points: Vec<Point> = ring
            .iter()
            .map(|&(x, y)| Point::new(x, y, 0.0))
            .collect();
        let fan = Mesh::from_vertices_and_faces(points.clone(), vec![(0..6).collect()]);
        let face = fan.faces()[0];
        assert!(
            (compute_face_area(&fan, face, &Xform::identity()) - 3.0).abs() < 1e-12
        );
        assert!(
            fan.area() > 3.5,
            "the kernel's unsigned fan over-counts"
        );
        let cached = Mesh::from_polylines(vec![points], None);
        assert!(
            (compute_mesh_area(&cached, &Xform::identity(), None) - 3.0).abs() < 1e-12
        );
        let moved = Xform::translation(-9.0, 3.0, 7.0);
        assert!((compute_mesh_area(&cached, &moved, None) - 3.0).abs() < 1e-12);
        let scaled = Xform::scale_xyz(2.0, 2.0, 2.0);
        assert!((compute_mesh_area(&cached, &scaled, None) - 12.0).abs() < 1e-12);
    }

    /// A warped quad measures its two drawn triangles, not their flat shadow.
    #[test]
    fn a_warped_face_measures_as_drawn() {
        let points = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 1.0),
            Point::new(0.0, 1.0, 0.0),
        ];
        let quad = Mesh::from_vertices_and_faces(points, vec![vec![0, 1, 2, 3]]);
        let area = compute_mesh_area(&quad, &Xform::identity(), None);
        assert!((area - 2.0_f64.sqrt()).abs() < 1e-12);
    }

    #[test]
    fn a_box_sweeps_six_times_its_volume() {
        let mesh = Mesh::create_box(2.0, 2.0, 0.4);
        let origin = Point::new(5.0, -3.0, 1.0);
        let swept = compute_swept(&mesh, &Xform::identity(), &origin).abs() / 6.0;
        assert!((swept - 1.6).abs() < 1e-9);
    }
```

### 2. `src/measure_length.rs`

Type the complete original helpers introduced today.

Create the file and type:

```rust
use session_rust::{Geometry, Xform};
/// World length of a line, polyline or NURBS curve placed by `place`; None for other kinds.
pub fn compute_length(geometry: &Geometry, place: &Xform) -> Option<f64> {
    match geometry {
        Geometry::Line(line) => Some(line.transformed(place).length()),
        Geometry::Polyline(polyline) => Some(polyline.transformed(place).length()),
        Geometry::NurbsCurve(curve) => Some(curve.transformed(place).length(None)),
        _ => None,
    }
}
```

### 3. `src/lib.rs`

Register this original measurement owner.

**Locate this existing block:**

```rust
pub mod measure_math;
pub mod source_mesh_topology;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod measure_math;

pub mod measure_length;
pub mod source_mesh_topology;
```

### 4. `src/measure_length.rs`

Supply only the unchanged original validation fixtures.

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use session_rust::{Line, Mesh, NurbsCurve, Point, Polyline};
    use std::rc::Rc;

    #[test]
    fn curves_measure_in_world_units() {
        let identity = Xform::identity();
        let line = Geometry::Line(Rc::new(Line::new(
            0.0, 0.0, 0.0, 3.0, 4.0, 0.0,
        )));
        assert_eq!(compute_length(&line, &identity), Some(5.0));
        let polyline = Geometry::Polyline(Rc::new(Polyline::new(vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(60.0, 0.0, 0.0),
            Point::new(60.0, 60.0, 0.0),
        ])));
        let place =
            &Xform::translation(20.0, 0.0, 0.0) * &Xform::scale_xyz(1.5, 1.5, 1.5);
        let placed = compute_length(&polyline, &place).unwrap();
        assert!((placed - 180.0).abs() < 1e-9);
        let points = [
            Point::new(8.0, 3.0, 0.0),
            Point::new(9.0, 4.0, 0.0),
            Point::new(10.0, 3.0, 0.0),
        ];
        let curve = Geometry::NurbsCurve(Rc::new(NurbsCurve::create(
            false, 2, &points,
        )));
        let exact = (2.0 * 2.0_f64.sqrt() + 2.0 * 1.0_f64.asinh()) / 2.0;
        assert!((compute_length(&curve, &identity).unwrap() - exact).abs() < 1e-9);
        let mesh = Geometry::Mesh(Rc::new(Mesh::create_box(1.0, 1.0, 1.0)));
        assert_eq!(compute_length(&mesh, &identity), None);
    }
}
```

### 5. `src/cad_orient.rs`

Supply only original orientation validation fixtures against the already taught shared owner.

**Locate this existing block:**

```rust
    sign
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    sign
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cad_boundary::QUALITY;
    use crate::cad_edges::edge_chains;
    use session_rust::brep::brep_reverse;

    /// Flip sign of every face of `b`.
    fn signs_of(b: &BRep) -> Vec<f64> {
        let fms = b.face_meshes_q(Some(QUALITY));
        let chains = edge_chains(b, &fms);
        face_signs(b, &fms, &chains)
    }

    /// Reverse the first two face uses.
    fn flipped(mut b: BRep) -> BRep {
        for face in b.m_shells[0].faces.iter_mut().take(2) {
            face.orientation = brep_reverse(face.orientation);
        }

        b
    }

    /// Kernel solids need no face flipped.
    #[test]
    fn kernel_solids_keep_their_normals() {
        for b in [
            BRep::create_cylinder(150.0, 400.0),
            BRep::create_cone(150.0, 400.0),
            BRep::create_box(400.0, 300.0, 250.0),
            BRep::create_block_with_hole(500.0, 300.0, 200.0, 80.0),
        ] {
            assert!(
                signs_of(&b).iter().all(|&s| s == 1.0),
                "{}",
                b.name
            );
        }
    }

    /// An open shell has no face flipped.
    #[test]
    fn open_shell_preserves_authored_orientation() {
        let mut b = flipped(BRep::create_cylinder(150.0, 400.0));
        b.m_solids.clear();
        assert!(signs_of(&b).iter().all(|&sign| sign == 1.0));
    }

    /// A flipped face keeps the same outward normal.
    #[test]
    fn flipped_uses_change_no_outward_normal() {
        let ok = BRep::create_cylinder(150.0, 400.0);
        let fl = flipped(BRep::create_cylinder(150.0, 400.0));
        let (fms_ok, fms_fl) = (
            ok.face_meshes_q(Some(QUALITY)),
            fl.face_meshes_q(Some(QUALITY)),
        );
        let signs = face_signs(&fl, &fms_fl, &edge_chains(&fl, &fms_fl));
        assert_eq!(signs[0], -1.0);
        assert_eq!(signs[1], -1.0);
        assert_eq!(signs[2], 1.0);

        for fi in 0..3 {
            for (key, vd) in fms_ok[fi].vertex.iter() {
                let n_ok = vd.normal().unwrap();
                let n_fl = fms_fl[fi].vertex[key].normal().unwrap();
                assert_eq!(
                    [
                        n_fl[0] * signs[fi],
                        n_fl[1] * signs[fi],
                        n_fl[2] * signs[fi]
                    ],
                    n_ok
                );
            }
        }
    }

    /// A flipped cylinder draws the same pipes.
    #[test]
    fn flipped_uses_change_no_pipe() {
        let mut pipes = Vec::new();
        for b in [
            BRep::create_cylinder(150., 400.),
            flipped(BRep::create_cylinder(150., 400.)),
        ] {
            let meshes = b.face_meshes_q(Some(QUALITY));
            let chains = edge_chains(&b, &meshes);
            pipes.push(
                chains
                    .iter()
                    .map(|c| {
                        c.as_ref().map(|c| {
                            c.keys
                                .iter()
                                .map(|key| {
                                    let p = &meshes[c.face].vertex[key];
                                    [p.x, p.y, p.z]
                                })
                                .collect::<Vec<_>>()
                        })
                    })
                    .collect::<Vec<_>>(),
            );
        }
        assert_eq!(pipes[0], pipes[1]);
    }
}
```

## <a id="checkpoint-83ze-integrate-original-surface-area"></a>4. Integrate surface area

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 4 of 4</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/measure_area.rs, src/li…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original geometry</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">placed source</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/booleans.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">measure or refusal</text>
</svg>
</div>

**What this code does:** Reuse mesh area for meshes, meshed BReps and Elements.

### 1. `src/measure_area.rs`

Type the complete original helpers introduced today.

Create the file and type:

```rust
use crate::measure_math::compute_mesh_area;
use session_rust::element::ElementGeometry;
use session_rust::{BRep, Geometry, NurbsSurface, Xform};
/// World area of a surface kind placed by `place`, all faces or only `face`; None for other kinds.
pub fn compute_area(
    geometry: &Geometry,
    place: &Xform,
    face: Option<usize>,
) -> Option<f64> {
    match geometry {
        Geometry::Mesh(mesh) => Some(compute_mesh_area(mesh, place, face)),
        Geometry::BRep(brep) => Some(compute_brep_area(brep, place, face)),
        Geometry::NurbsSurface(surface) => Some(compute_surface_area(
            &surface.transformed(place),
        )),
        Geometry::Element(element) => match element.geometry() {
            ElementGeometry::Mesh(mesh) => Some(compute_mesh_area(mesh, place, face)),
            ElementGeometry::BRep(brep) => Some(compute_brep_area(brep, place, face)),
            ElementGeometry::None => None,
        },
        _ => None,
    }
}

/// World area of a BRep's faces as the viewer meshes them; planar faces are exact.
fn compute_brep_area(brep: &BRep, place: &Xform, face: Option<usize>) -> f64 {
    let meshes = brep.face_meshes_q(Some(crate::cad_boundary::QUALITY));
    meshes
        .iter()
        .enumerate()
        .filter(|(index, _)| face.is_none_or(|face| face == *index))
        .map(|(_, mesh)| compute_mesh_area(mesh, place, None))
        .sum()
}

const GAUSS: [(f64, f64); 8] = [
    (-0.9602898564975363, 0.1012285362903763),
    (-0.7966664774136267, 0.2223810344533745),
    (-0.5255324099163290, 0.3137066458778873),
    (-0.1834346424956498, 0.3626837833783620),
    (0.1834346424956498, 0.3626837833783620),
    (0.5255324099163290, 0.3137066458778873),
    (0.7966664774136267, 0.2223810344533745),
    (0.9602898564975363, 0.1012285362903763),
]; // 8-point Gauss-Legendre nodes and weights on [-1, 1]

/// Area of a NURBS surface: |Su x Sv| integrated per knot span with 8 x 8 Gauss points.
fn compute_surface_area(surface: &NurbsSurface) -> f64 {
    let us = surface.get_span_vector(0);
    let vs = surface.get_span_vector(1);
    let mut total = 0.0;

    for u in us.windows(2) {
        let (uh, um) = ((u[1] - u[0]) * 0.5, (u[1] + u[0]) * 0.5); // half width, middle

        for v in vs.windows(2) {
            let (vh, vm) = ((v[1] - v[0]) * 0.5, (v[1] + v[0]) * 0.5);

            for (su, wu) in GAUSS {
                for (sv, wv) in GAUSS {
                    let derivatives = surface.evaluate(um + uh * su, vm + vh * sv, 1); // [S, Sv, Su]

                    if let [_, dv, du] = derivatives.as_slice() {
                        total += wu * wv * uh * vh * du.cross(dv).magnitude();
                    }
                }
            }
        }
    }

    total
}
```

### 2. `src/lib.rs`

Register surface area after curve length; reuse the earlier orientation owner.

**Locate this existing block:**

```rust
pub mod measure_length;
pub mod source_mesh_topology;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod measure_length;

pub mod measure_area;
pub mod source_mesh_topology;
```

### 3. `src/measure_area.rs`

Supply only the unchanged original validation fixtures.

**Locate this existing block:**

```rust
    total
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use session_rust::{Element, Line, Mesh, Point, Primitives};
    use std::f64::consts::PI;
    use std::rc::Rc;

    fn near(value: f64, expected: f64, relative: f64) -> bool {
        (value - expected).abs() <= relative * expected.abs().max(1.0)
    }

    #[test]
    fn meshes_and_breps_measure_in_world_units() {
        let identity = Xform::identity();
        let mesh = Mesh::create_box(2.0, 2.0, 0.4);
        let boxed = Geometry::Mesh(Rc::new(mesh.clone()));
        assert!(near(
            compute_area(&boxed, &identity, None).unwrap(),
            11.2,
            1e-9
        ));
        let moved = Xform::translation(-9.0, 3.0, 0.0);
        assert!(near(
            compute_area(&boxed, &moved, None).unwrap(),
            11.2,
            1e-9
        ));
        let scaled = Xform::scale_xyz(2.0, 2.0, 2.0);
        assert!(near(
            compute_area(&boxed, &scaled, None).unwrap(),
            44.8,
            1e-9
        ));
        let faces: Vec<f64> = mesh
            .faces()
            .into_iter()
            .map(|face| compute_area(&boxed, &identity, Some(face)).unwrap())
            .collect();
        assert!(
            faces
                .iter()
                .any(|area| near(*area, 4.0, 1e-9))
        );
        assert!(
            faces
                .iter()
                .any(|area| near(*area, 0.8, 1e-9))
        );

        let brep = BRep::create_box(2.0, 2.0, 0.4);
        let solid = Geometry::BRep(Rc::new(brep.clone()));
        assert!(near(
            compute_area(&solid, &identity, None).unwrap(),
            11.2,
            1e-9
        ));
        let each: Vec<f64> = (0..brep.m_faces.len())
            .map(|face| compute_area(&solid, &identity, Some(face)).unwrap())
            .collect();
        assert!(near(each.iter().sum(), 11.2, 1e-9));
        assert!(
            each.iter()
                .all(|area| near(*area, 4.0, 1e-9) || near(*area, 0.8, 1e-9))
        );

        let cylinder = Geometry::BRep(Rc::new(BRep::create_cylinder(150.0, 400.0)));
        let exact = 2.0 * PI * 150.0 * 400.0 + 2.0 * PI * 150.0 * 150.0;
        assert!(near(
            compute_area(&cylinder, &identity, None).unwrap(),
            exact,
            5e-3
        ));

        let element = Geometry::Element(Rc::new(Element::from_mesh(mesh, "beam")));
        assert!(near(
            compute_area(&element, &identity, None).unwrap(),
            11.2,
            1e-9
        ));
        let line = Geometry::Line(Rc::new(Line::new(
            0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
        )));
        assert_eq!(compute_area(&line, &identity, None), None);
    }

    #[test]
    fn nurbs_surfaces_integrate_exactly() {
        let identity = Xform::identity();
        let cylinder = Primitives::cylinder_surface(0.0, 0.0, 0.0, 2.0, 5.0);
        let side = Geometry::NurbsSurface(Rc::new(cylinder));
        assert!(near(
            compute_area(&side, &identity, None).unwrap(),
            20.0 * PI,
            1e-7
        ));
        let scaled = Xform::scale_xyz(2.0, 2.0, 2.0);
        assert!(near(
            compute_area(&side, &scaled, None).unwrap(),
            80.0 * PI,
            1e-7
        ));
        let sphere = Geometry::NurbsSurface(Rc::new(Primitives::sphere_surface(
            0.0, 0.0, 0.0, 3.0,
        )));
        assert!(near(
            compute_area(&sphere, &identity, None).unwrap(),
            36.0 * PI,
            1e-7
        ));
        let corners = [
            Point::new(-1.0, -1.0, 0.0),
            Point::new(-1.0, 1.0, 0.0),
            Point::new(1.0, -1.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
        ];
        let flat = NurbsSurface::create(false, false, 1, 1, 2, 2, &corners).unwrap();
        let flat = Geometry::NurbsSurface(Rc::new(flat));
        assert!(near(
            compute_area(&flat, &identity, None).unwrap(),
            4.0,
            1e-12
        ));
    }
}
```

## Build & Run

![Recorded visual output for Lesson 16.3 (83ze-integrate-original-surface-area)](../../../screenshots/journey/83ze-integrate-original-surface-area.png)

```sh
npm --prefix ../session_tests run course -- dependencies 16.3
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 16.3
npm --prefix ../session_tests run course -- save 16.3
```

Continue with [16.4 · Measurements, temporary marks and retained CAD pieces](../../consolidation/84-measure-original-curves-surfaces-and-closed-solids.md).
