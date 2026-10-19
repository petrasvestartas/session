# 15.1.41 · Prepare placed kernel curves

**Chapter 15 · Transforms and surface construction · Section 15.1 · Transforms, extrusions and revolutions** · [Contents](contents.md)

**Typing: 17–34 minutes.** [Estimate](typing-load.md).

Convert lines, polylines and NURBS curves to placed kernel curves. Remove repeated polyline corners, clamp periodic input and normalize the domain for shared surfacing recipes.

## Type

Continue from [Advance and cancel curve picks](77zzf-advance-and-cancel-curve-picks.md). [Save or recover your work](recovery.md).

### 1. `src/gather_input.rs`

Add the next gathering behavior to the existing helpers.

<details>
<summary>Locate the existing block</summary>

```rust
/// "1 curve" or "3 curves".
pub fn count(count: usize, noun: &str) -> String {
    match count {
        1 => format!("1 {noun}"),
        _ => format!("{count} {noun}s"),
    }
}

pub const SAMPLES: usize = 48; // points per curve for normals and previews
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/77zzg-prepare-placed-kernel-curves-1.rs"
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/gather_input.rs`

Supply the original gathering-input validation fixtures separately.

<details>
<summary>Locate the existing block</summary>

```text
pub const SAMPLES: usize = 48; // points per curve for normals and previews
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/77zzg-prepare-placed-kernel-curves-2.rs" skip=76
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 15.1.41
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

![Native renderer output for prepare placed kernel curves.](../screenshots/journey/77zza-describe-curve-gathering-answers.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [15.1.35 · Describe curve gathering answers](77zza-describe-curve-gathering-answers.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original curve picks → ordered answers → Session kernel recipe.



Why normalize every gathered curve to the same domain?

Loft and sweep need compatible input conventions even when the original curves use different parameter ranges.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare a placed polyline with a periodic NURBS curve. Trace their distinct preparation paths and shared validity checks.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 15.1.41
npm --prefix ../session_tests run course -- save 15.1.41
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

These helpers prepare later interactive gathering commands. The browser scene stays unchanged until those commands are registered.



[Full validation scope](release.md).

</details>
