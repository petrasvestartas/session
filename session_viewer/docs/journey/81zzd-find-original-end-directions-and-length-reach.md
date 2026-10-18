# 16.1.5 · Find original end directions and length reach

**Chapter 16 · Geometry editing and measurement · Section 16.1 · Projection and Split inputs** · [Contents](contents.md)

**Typing: 26–52 minutes.** [Estimate](typing-load.md).

Find outward directions for supported original curve families. Search domain growth until the gained curve has the requested physical length.

## Type

Continue from [Grow original domains and extract the tail](81zzc-grow-original-domains-and-extract-the-tail.md). [Save or recover your work](recovery.md).

### 1. `src/extend_reach.rs`

Extend this original owner with today’s complete helper.

<details>
<summary>Locate the existing block</summary>

```rust
/// The curve with its domain grown by `s` at `end`.
fn grow(curve: &NurbsCurve, end: End, s: f64) -> Option<NurbsCurve> {
    let (d0, d1) = curve.domain();
    let mut grown = curve.clone();
    let done = match end {
        End::Start => grown.extend(d0 - s, d1),
        End::End => grown.extend(d0, d1 + s),
    };
    done.then_some(grown)
}

/// The part of `grown` beyond the old domain.
fn beyond(curve: &NurbsCurve, grown: &NurbsCurve, end: End) -> Option<NurbsCurve> {
    let (d0, d1) = curve.domain();
    let (g0, g1) = grown.domain();
    let mut piece = grown.clone();
    let done = match end {
        End::Start => piece.trim(g0, d0),
        End::End => piece.trim(d1, g1),
    };
    done.then_some(piece)
}

/// The added piece as points, from the old end.
fn tail(curve: &NurbsCurve, grown: &NurbsCurve, end: End) -> Vec<Point> {
    let Some(piece) = beyond(curve, grown, end) else {
        return Vec::new();
    };
    let mut points = crate::source_cut::samples(&piece, 24);

    if end == End::Start {
        points.reverse();
    }

    points
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/81zzd-find-original-end-directions-and-length-reach-1.rs"
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 16.1.5
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Extension helpers can derive original endpoint directions and length reach.

**Visual review: issue found.** This inherited network reference has faint or interrupted original input curves. Its aggregate pixel checks do not establish curve continuity or contrast. These are input curves, not a full NURBS isocurve display; visual-quality acceptance is incomplete.

[Visual review findings](visual-review.md).

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for find original end directions and length reach.](../screenshots/journey/80za-read-original-projection-planes.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [15.4.6 · Read original projection planes](80za-read-original-projection-planes.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original cutter → object frame → physical reach → edited curve.



Why search by physical length?

A NURBS parameter step does not represent a fixed distance. Evaluate the gained curve length instead.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace a curved end with changing parameter speed and a zero tangent.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 16.1.5
npm --prefix ../session_tests run course -- save 16.1.5
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

These original cutter and reach helpers prepare cancellable editing of the original curve.



[Full validation scope](release.md).

</details>
