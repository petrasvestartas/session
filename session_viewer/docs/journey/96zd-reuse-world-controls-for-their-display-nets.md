# 19.4.4 · Reuse world controls for their display nets

**Chapter 19 · Integration and original ownership · Section 19.4 · Editing original controls** · [Contents](contents.md)

**Typing: 24–48 minutes.** [Estimate](typing-load.md).

A curve or surface control net connects the collector’s point indices.

## Type

Continue from [Share the current control-point style](96zc-share-the-current-control-point-style.md). [Save or recover your work](recovery.md).

### 1. `src/scene.rs`

Project one control owner for both point discs and net links.

<details>
<summary>Locate the existing block</summary>

```rust
    pub fn control_markers(&self) -> Vec<ControlMarker> {
        let mut markers = Vec::new();
        for row in self.paths.iter().filter(|row| row.controls_visible && self.visible(row.id)) {
            for control in row.prepared.source.controls() {
                let p = control.position().expect("Validated original control");
                let p = row.model.transform_point(&session_rust::Point::new(p[0], p[1], p[2]));
                markers.push(ControlMarker::new(row.id, crate::pick_sources::Part::Control(control.index), p.to_f32()));
            }
        }
        for row in self.objects.iter().filter(|row| self.control_owners.contains(&row.id) && self.visible(row.id)) {
            let Some(source) = crate::source_edit::original(self, row.id) else { continue };
            for control in crate::source_controls::Controls::from_geometry(&source).points {
                let local = session_rust::Point::new(control.position[0], control.position[1], control.position[2]);
                let center = row.model.transform_point(&local).to_f32();
                if center.iter().any(|v| !v.is_finite()) { continue; }
                markers.push(ControlMarker::new(row.id, crate::pick_sources::Part::SourceControl(control.id), center));
            }
        }
        markers
    }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/96zd-reuse-world-controls-for-their-display-nets-1.rs"
```

### 2. `src/pick_sources.rs`

Reuse stroke drawing and address each net only to its original parent.

<details>
<summary>Locate the existing block</summary>

```rust
        for row in scene.control_markers() {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
        for (owner, stroke) in scene.control_nets() {
            sources.mesh_ink.push(stroke);
            sources.mesh_ids.push([owner.value(), 0]);
        }
        for row in scene.control_markers() {
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zd-reuse-world-controls-for-their-display-nets-2.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/pick_gpu_tests.rs`

Supply real net integer-pick, endpoint, source identity and Off checks at both densities.

<details>
<summary>Locate the existing block</summary>

```text
    renderer.set_highlight(&scene,Some(target)).unwrap();let selected=colour_pixels(&mut renderer);
        assert_eq!(&selected[center..center+4],&[255,255,0,255]);
        renderer.set_pick_mode(crate::pick_sources::Mode::Control);renderer.request_pick([-0.5,-0.5],identity()).unwrap();let pixel=finish(&mut renderer).unwrap();assert_eq!(renderer.resolve_pick(&scene,pixel),Some(target));
    }
    assert!(crate::source_edit::same(&source,&crate::source_edit::original(&scene,owner).unwrap()));
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zd-reuse-world-controls-for-their-display-nets-3.rs" skip=500
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/pick_sources_tests.rs`

Keep twelve original mesh edge addresses distinct from two parent-only control links.

<details>
<summary>Locate the existing block</summary>

```text
assert_eq!(sources.mesh_ink.len(), 12);
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zd-reuse-world-controls-for-their-display-nets-4.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 19.4.4
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.



### Recorded visual result

**Previous native render — current verification pending.**

This supporting frame has no instances; use the supplied original Session specimen for the browser exercise.

![Native renderer output for reuse world controls for their display nets.](../screenshots/journey/96u-fit-original-box-corners-to-the-free-canvas.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [19.3.3 · Fit original box corners to the free canvas](96u-fit-original-box-corners-to-the-free-canvas.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Build one shared control-source projection: retain original family keys and legacy curve addressing, transform the copied control positions into world space in f64, and leave the original geometry untouched. Both marker and net conversion consume this projection. Reuse the existing stroke lane for thin control links with the maintained packed color 0xffcc8866. Each link belongs to its original parent and has subpart tag zero; it must not invent a CAD edge address. Original mesh controls have no extra net links. Visibility and Controls Off remove the derived presentation together.

source feature visibility → coincident-point classification → existing marker/path display → original owner.



Why do control links carry an original parent but no edge tag?

They explain the source’s control structure. They are not original mesh edges or BRep boundaries, so object picking may find the parent while edge and control picking must not invent a new subpart.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Enable controls on Example Curve and Example Surface. Compare each link endpoint with its control disc. Switch object, edge and control picking; the link can address the original object but only the discs address control keys.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 19.4.4
npm --prefix ../session_tests run course -- save 19.4.4
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Reuse the maintained control collector and net color with existing point/stroke owners. Selected-owner command semantics, editing, current depth/finite visibility and packed rendering remain separate required bridges.

Original-control browser acceptance is pending.

[Full validation scope](release.md).

</details>
