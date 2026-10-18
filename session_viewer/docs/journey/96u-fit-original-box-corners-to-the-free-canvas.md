# 19.3.3 · Fit original box corners to the free canvas

**Chapter 19 · Integration and original ownership · Section 19.3 · Selecting, copying and presenting originals** · [Contents](contents.md)

**Typing: 16–32 minutes.** [Estimate](typing-load.md).

Reuse the existing camera orientation, panel rectangle and original bounds.

## Type

Continue from [Select and highlight original feature owners](96t-select-and-highlight-original-feature-owners.md). [Save or recover your work](recovery.md).

### 1. `src/camera.rs`

Solve the current projected box-corner distance in the existing camera.

<details>
<summary>Locate the existing block</summary>

```rust
    pub fn fit(&mut self, bounds: &crate::bounds::Bounds) {
        let half_y = (HALF_FOV.tan()*(self.frame[3]-self.frame[1])).atan();
        let half_x = (self.aspect * HALF_FOV.tan()*(self.frame[2]-self.frame[0])).atan();
        let radius = bounds.radius();
        self.radius = if radius > 0.0 { radius } else { 1.0 };
        self.target = bounds.centre();
        let opening = match self.projection {
            Projection::Perspective => half_x.min(half_y).sin(),
            Projection::Orthographic => half_y.tan().min(half_x.tan()),
        };
        self.distance = self.radius * 1.1 / opening;
    }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/96u-fit-original-box-corners-to-the-free-canvas-01.rs"
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/projected_fit_tests.rs`

Supply independent visible-rectangle and thin-part regression witnesses.

Copy this supplied file:

```text
--8<-- "journey/code/96u-fit-original-box-corners-to-the-free-canvas-02.rs"
```

</details>

### 3. `src/lib.rs`

Run the supplied fit witnesses with the cumulative source.

<details>
<summary>Locate the existing block</summary>

```rust
mod feature_owner_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/96u-fit-original-box-corners-to-the-free-canvas-03.rs" skip=25
```

## Run and check

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Thin parts fill the available canvas; all box corners remain inside the rectangle left by the panels.

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for fit original box corners to the free canvas.](../screenshots/journey/96u-fit-original-box-corners-to-the-free-canvas.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

<details>
<summary>Code explanation and diagram</summary>

Project each of the eight corner offsets onto right, up and forward. With forward pointing from the eye toward the target, a corner needs distance at least |x|/tx − z and |y|/ty − z. Every centered box corner has an opposite corner, so the maximum over all eight corners is unchanged when using +z. That symmetric form, followed by a five-percent margin, is the maintained Fit rule.

This upgrades the earlier sphere approximation in place. It introduces no second camera and changes no source geometry. Keep radius as the depth/zoom extent. A point box can move the target without inventing a nonzero extent; invalid bounds leave the pose unchanged. Current unit conversion, reversed depth and GPU rebasing remain separate connected changes.

original bounds → camera-axis offsets → free-rectangle constraints → distance.



Why can a bounding sphere leave a thin CAD part unnecessarily small?

Its radius fits every direction, although the current view needs only the projected box corners. Solve the horizontal and vertical corner constraints in the camera basis, including depth, and use the largest required distance.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Open the imported specimen, enable Layers and select a feature row. Fit its original owner, then compare Front, Top and Isometric with a thin Box. Switch projection and Fit again; preserve original dimensions and selection.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 19.3.3
npm --prefix ../session_tests run course -- save 19.3.3
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This follows camera.rs fit_in’s projected-corner distance constraint and 1.05 margin. The teaching camera still uses its existing source-unit and forward-depth contracts at this checkpoint.

Two supplied numerical regressions cover asymmetric panels, portrait aspect, four named views, both projections, large coordinates, thin parts, invalid bounds and a point box. Browser camera navigation and final CAD actions are checked separately.

Actual Chrome checks original-owner feature selection and thin-part Fit in Front, Top and Isometric, in both projections with panel space. Every original box corner stays inside the free rectangle; original placements and CAD definitions remain unchanged.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 19.3.3
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
