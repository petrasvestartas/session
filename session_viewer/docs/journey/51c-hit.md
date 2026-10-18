# 7.6.4 · Pick handles in a stable priority order

**Chapter 7 · Drawing, snapping and transform tools · Section 7.6 · Move, rotate and scale handles** · [Contents](contents.md)

**Typing: 18–35 minutes.** [Estimate](typing-load.md).

Test the hub, scale balls, arms and rotation arcs through the current world ray. Keep a CSS grab aperture and reject axes pointing almost straight at the eye.

## Type

Continue from [Keep handle state at one original-world pivot](51b-pivot.md). [Save or recover your work](recovery.md).

### 1. `src/gizmo.rs`

Resolve stable handle priority and the same CSS aperture used by drawing.

<details>
<summary>Locate the existing block</summary>

```rust
        }
    }

}

fn typed_factor(value: f64) -> f64 {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/51c-hit-01.rs"
```

## Run and check

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.



### Recorded visual result

**Previous native render — current verification pending.**

The earlier two gold boxes and temporary Move band remain.

![Native renderer output for pick handles in a stable priority order.](../screenshots/journey/50d-ownership.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [7.5.5 · Cancel previews without consuming document history](50d-ownership.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



world ray and CSS scale → hub → balls → arms → negative-quadrant arcs.

![Pick handles in a stable priority order](../illustrations/journey-51c.svg)

Why test scale balls before the arms beneath them?

A ball lies on its arm. Without priority the arm captures the click, so the learner cannot start the scale operation.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Query a ray through a ball and then one through the arrow tip. They must return different handle operations at every zoom.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 7.6.4
npm --prefix ../session_tests run course -- save 7.6.4
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This preparation retains the earlier Move input. Current handle math and drawing are introduced in dependency order; selection and pointer input are connected later in this subject.

The earlier temporary Move remains while handle infrastructure is introduced.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 7.6.4
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
