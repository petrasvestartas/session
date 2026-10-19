# 16.4.17 · Collect curve crossings

**Chapter 16 · Geometry editing and measurement · Section 16.4 · Solid volume, extension and Explode** · [Contents](contents.md)

**Typing: 21–42 minutes.** [Estimate](typing-load.md).

Collect kernel curve intersections, filter open endpoints and merge a closed seam. Add shared compact-table helpers for later BRep reconstruction.

## Type

Continue from [Read trim part previews](84zzb-read-trim-part-previews.md). [Save or recover your work](recovery.md).

### 1. `src/trim_parts.rs`

Add this part-building behavior to the existing owner.

<details>
<summary>Locate the existing block</summary>

```rust
/// The parameter range of curve part `index`; a closed curve's last part runs over the seam.
fn interval(curve: &NurbsCurve, cuts: &[f64], index: usize) -> (f64, f64) {
    let (d0, d1) = curve.domain();

    if curve.is_closed() {
        return (cuts[index], cuts[(index + 1) % cuts.len()]);
    }

    let a = if index == 0 { d0 } else { cuts[index - 1] };
    (a, cuts.get(index).copied().unwrap_or(d1))
}

/// The piece of `curve` from `a` to `b`, over the seam when `b <= a`.
fn piece(curve: &NurbsCurve, a: f64, b: f64) -> Option<NurbsCurve> {
    let (d0, d1) = curve.domain();
    let near = (d1 - d0) * 1e-9;
    let trimmed = |a: f64, b: f64| {
        let mut part = curve.clone();
        part.trim(a, b).then_some(part)
    };

    if a < b {
        return trimmed(a, b);
    }

    // over the seam: the tail and the head joined
    let tail = (a < d1 - near).then(|| trimmed(a, d1)).flatten();
    let head = (b > d0 + near).then(|| trimmed(d0, b)).flatten();

    match (tail, head) {
        (Some(tail), Some(head)) if curve.degree() == 1 => {
            let mut points = samples(&tail, 1);
            points.extend(samples(&head, 1).into_iter().skip(1));
            Some(NurbsCurve::create(false, 1, &points))
        }
        (Some(tail), Some(head)) => NurbsCurve::join(&[tail, head], Some(TOLERANCE * 10.0))
            .into_iter()
            .next(),
        (tail, head) => tail.or(head),
    }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/84zzc-collect-curve-crossings-1.rs"
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 16.4.17
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

![Native renderer output for collect curve crossings.](../screenshots/journey/84zza-describe-removable-cad-parts.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [16.4.15 · Describe removable CAD parts](84zza-describe-removable-cad-parts.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original CAD geometry → removable parts → retained source geometry.



Why remove open endpoints from the cut list?

An endpoint creates no additional removable interval.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare cuts at the start, middle and end of an open curve; then repeat on a closed curve.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 16.4.17
npm --prefix ../session_tests run course -- save 16.4.17
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>
