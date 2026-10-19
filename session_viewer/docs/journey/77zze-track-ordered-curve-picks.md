# 15.1.39 · Track ordered curve picks

**Chapter 15 · Transforms and surface construction · Section 15.1 · Transforms, extrusions and revolutions** · [Contents](contents.md)

**Typing: 22–44 minutes.** [Estimate](typing-load.md).

Track preselected curve rows and skip answers already typed. Keep curve-pick order while advancing only after the required minimum.

## Type

Continue from [Parse gathering command words](77zzd-parse-gathering-command-words.md). [Save or recover your work](recovery.md).

### 1. `src/gather_input.rs`

Add the next gathering behavior to the existing helpers.

<details>
<summary>Locate the existing block</summary>

```rust
/// A picked curve in world coordinates.
#[derive(Clone, Debug)]
pub struct Picked {
    pub curve: NurbsCurve,           // clamped, domain 0..1
    pub corners: Option<Vec<Point>>, // a line's or polyline's points
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/77zze-track-ordered-curve-picks-1.rs"
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 15.1.39
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

![Native renderer output for track ordered curve picks.](../screenshots/journey/77zza-describe-curve-gathering-answers.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [15.1.35 · Describe curve gathering answers](77zza-describe-curve-gathering-answers.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original curve picks → ordered answers → Session kernel recipe.



Why retain a separate count of typed answers?

Curve picks and other answers occupy different sequences; the count lets progress skip already supplied values.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace two preselected rows through separate rail and profile steps. Identify where progress waits.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 15.1.39
npm --prefix ../session_tests run course -- save 15.1.39
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>
