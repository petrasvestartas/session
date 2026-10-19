# 19.3.8 · Preserve source feature visibility and pixel styles

**Chapter 19 · Integration and original ownership · Section 19.3 · Selecting, copying and presenting originals** · [Contents](contents.md)

**Typing: 17–34 minutes.** [Estimate](typing-load.md).

A feature has source visibility as well as the owner’s Attributes or Interactions switch. Skip invisible features during projection. A global presentation switch cannot revive a source feature marked invisible. Its original Session payload remains available for saving and later regeneration.

## Type

Continue from [Address feature pixels to original CAD owners](96y-address-feature-pixels-to-original-cad-owners.md). [Save or recover your work](recovery.md).

### 1. `src/feature_display.rs`

Respect original feature visibility before generating a display. Reuse marker and path preparation with source colors and current ordinary feature sizes.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::{scene::{Scene, ObjectId}, layer_edit::FeatureTree};
use session_rust::{Element, Session};
use std::rc::Rc;

pub fn append(scene: &mut Scene, id: ObjectId, session: &Session, element: &Element) -> Result<(), &'static str> {
    let model = scene.placement(id).ok_or("Element placement is missing")?;
    let forest = std::mem::take(&mut scene.forest);
    let created = scene.created.take();
    let current = Rc::make_mut(&mut scene.view).current.take();
    let prepared = (|| {
        let mut groups = Vec::new();
        let mut pieces = |indices: &[usize]| -> Result<Vec<FeatureTree>, &'static str> {
            let mut rows = Vec::new();
            for &index in indices {
                let feature = &element.features[index];
                let mut outlines = Vec::new();
                for (part, outline) in feature.outlines.iter().enumerate() {
                    let mut source = outline.clone();
                    source.set_guid(uuid::Uuid::new_v4().to_string());
                    source.name = format!("piece {}", part + 1);
                    source.linecolor = session_rust::Color::black();
                    let piece = scene.insert_path(crate::chain::PreparedChain::polyline(Rc::new(source))?)?;
                    scene.place_following(piece, model.clone())?;
                    if !crate::feature_plan::interaction(feature) {
                        Rc::make_mut(&mut scene.view).hide(&[piece], true);
                    }
                    outlines.push(FeatureTree::Piece(format!("piece {}", part + 1), piece));
                }
                if !outlines.is_empty() {
                    rows.push(FeatureTree::Group(crate::feature_plan::label(feature), outlines));
                }
            }
            Ok(rows)
        };
        let own = pieces(&crate::feature_plan::own(element))?;
        if !own.is_empty() { groups.push(FeatureTree::Group("attributes".into(), own)); }
        let mut hosted = Vec::new();
        for relation in crate::feature_plan::hosted(session, element) {
            let children = pieces(&relation.features)?;
            if !children.is_empty() { hosted.push(FeatureTree::Group(relation.label, children)); }
        }
        if !hosted.is_empty() { groups.push(FeatureTree::Group("interactions".into(), hosted)); }
        Ok(groups)
    })();
    scene.forest = forest;
    scene.created = created;
    Rc::make_mut(&mut scene.view).current = current;
    crate::layer_edit::attach_features(scene, id, prepared?)
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{scene::{Scene, ObjectId}, layer_edit::FeatureTree};
use session_rust::{Element, Session};
use std::rc::Rc;

pub fn append(scene: &mut Scene, id: ObjectId, session: &Session, element: &Element) -> Result<(), &'static str> {
    let model = scene.placement(id).ok_or("Element placement is missing")?;
    let forest = std::mem::take(&mut scene.forest);
    let created = scene.created.take();
    let current = Rc::make_mut(&mut scene.view).current.take();
    let prepared = (|| {
        let mut groups = Vec::new();
        let mut pieces = |indices: &[usize]| -> Result<Vec<FeatureTree>, &'static str> {
            let mut rows = Vec::new();
            for &index in indices {
                let feature = &element.features[index];
                if !feature.visible { continue; }
                let mut outlines = Vec::new();
                for (part, outline) in feature.outlines.iter().enumerate() {
                    let point = outline.get_point(0).filter(|p| outline.coords.chunks_exact(3)
                        .all(|q| [q[0] as f32, q[1] as f32, q[2] as f32] == p.to_f32()));
                    let piece = if let Some(mut point) = point {
                        point.set_guid(uuid::Uuid::new_v4().to_string());
                        point.name = format!("piece {}", part + 1);
                        point.pointcolor = outline.linecolor.clone();
                        point.width = 12.0;
                        point.is_visible = true;
                        point.is_locked = false;
                        let piece = scene.insert_point(crate::marker::PreparedPoint::new(Rc::new(point))?)?;
                        scene.place_point(piece, model.clone())?;
                        piece
                    } else {
                        let mut source = outline.clone();
                        source.set_guid(uuid::Uuid::new_v4().to_string());
                        source.name = format!("piece {}", part + 1);
                        source.width = 2.0;
                        source.is_visible = true;
                        source.is_locked = false;
                        source.arrowhead = session_rust::Arrowhead::NONE;
                        let piece = scene.insert_path(crate::chain::PreparedChain::polyline(Rc::new(source))?)?;
                        scene.place_following(piece, model.clone())?;
                        piece
                    };
                    if !crate::feature_plan::interaction(feature) {
                        Rc::make_mut(&mut scene.view).hide(&[piece], true);
                    }
                    outlines.push(FeatureTree::Piece(format!("piece {}", part + 1), piece));
                }
                if !outlines.is_empty() {
                    rows.push(FeatureTree::Group(crate::feature_plan::label(feature), outlines));
                }
            }
            Ok(rows)
        };
        let own = pieces(&crate::feature_plan::own(element))?;
        if !own.is_empty() { groups.push(FeatureTree::Group("attributes".into(), own)); }
        let mut hosted = Vec::new();
        for relation in crate::feature_plan::hosted(session, element) {
            let children = pieces(&relation.features)?;
            if !children.is_empty() { hosted.push(FeatureTree::Group(relation.label, children)); }
        }
        if !hosted.is_empty() { groups.push(FeatureTree::Group("interactions".into(), hosted)); }
        Ok(groups)
    })();
    scene.forest = forest;
    scene.created = created;
    Rc::make_mut(&mut scene.view).current = current;
    crate::layer_edit::attach_features(scene, id, prepared?)
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96z-preserve-source-feature-visibility-and-pixel-styles-01.rs"
```

```text
--8<-- "journey/code/96z-preserve-source-feature-visibility-and-pixel-styles-02.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/feature_style_tests.rs`

Supply original-data style, visibility, placement and ownership witnesses.

Copy this supplied file:

```text
--8<-- "journey/code/96z-preserve-source-feature-visibility-and-pixel-styles-03.rs"
```

</details>

### 3. `src/lib.rs`

Register the independent source-style fixture.

<details>
<summary>Locate the existing block</summary>

```rust
mod feature_pick_tests;
```

</details>

After that block, type:

```rust
#[cfg(test)]
mod feature_style_tests;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96z-preserve-source-feature-visibility-and-pixel-styles-04.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/pick_gpu_tests.rs`

Supply a real GPU marker ownership and CSS-diameter witness at DPR one and two.

<details>
<summary>Locate the existing block</summary>

```text
#[test]
fn actual_integer_pick_preserves_high_ids_depth_and_locked_occlusion()
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96z-preserve-source-feature-visibility-and-pixel-styles-05.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 19.3.8
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Visible ordinary feature outlines retain their source colors and use the current screen-size conventions without altering original Session data.

### Recorded visual result

**Previous native render — current verification pending.**

This supporting frame has no instances; use the supplied original Session specimen for the browser exercise.

![Native renderer output for preserve source feature visibility and pixel styles.](../screenshots/journey/96u-fit-original-box-corners-to-the-free-canvas.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [19.3.3 · Fit original box corners to the free canvas](96u-fit-original-box-corners-to-the-free-canvas.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Use the existing marker pipeline for an outline whose coordinates coincide after conversion to the renderer’s f32 format. Preserve the original outline color and set a twelve-pixel diameter on the temporary display point. Other outlines use two-pixel strokes without arrowheads. Clone only display inputs: original widths, colors, arrowheads and visibility remain unchanged.

Both paths and markers already resolve to their original owner through the integer picking table. Reuse that route; do not add another shader or readback. Contact polygon fills are developed separately.

A feature outline is display input, not a separately editable original. Its standalone visible and locked flags do not override the feature flag or the original owner. Set those flags only on the temporary display clone; preserve them unchanged in the original payload.

source feature visibility → coincident-point classification → existing marker/path display → original owner.



Why must a coincident outline become a marker before path preparation?

The kernel permits coincident feature endpoints to describe a point. Path preparation removes duplicate vertices and requires a segment; the existing marker pipeline represents this source point with a stable screen diameter.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Import an Element with a colored axis, an invisible cut and coincident drill endpoints. Enable Attributes: the axis appears with a two-pixel stroke, the drill remains a twelve-pixel disc, and the invisible cut stays absent. Change zoom and pixel density; predict which dimensions remain fixed in CSS pixels. Save and inspect the original feature widths and arrowheads.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 19.3.8
npm --prefix ../session_tests run course -- save 19.3.8
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Current app/walk/mod.rs filters feature.visible before the owner switch, uses two-pixel ordinary feature lines and twelve-pixel coincident point markers. This bridge uses the already-developed path and marker pipelines. Contact fill and its facing/depth treatment remain separate.

Actual browser acceptance for the new feature-style fixture is pending.

[Full validation scope](release.md).

</details>
