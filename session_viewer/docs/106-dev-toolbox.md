# 106 Developer toolbox — selftest, surfaced errors, CI

> **Big picture.** *Phase 14.* The kernel has minitests in three languages; the viewer, until now,
> had scattered `#[cfg(test)]`s and the browser console. This lesson closes the workflow gap the
> plan review flagged: a **headless selftest** you run in seconds without a browser, **GPU errors
> that report in the viewer's own CLI** instead of dying silently in F12, a black-screen checklist
> for the worst debugging hour, and a CI recipe. This is the lesson that makes the other 82
> maintainable.

## Files we touch

```
examples/selftest.rs    # EXISTS — the PPM render harness (src/selftest.rs); ADD the checks below
Cargo.toml + src/lib.rs # the two visibility edits that let an example link the crate
src/engine/gpu/mod.rs   # error scopes around pipeline/shader creation → CLI log, not console
.github/workflows/viewer.yml   # NEW — build + selftest on push
```

## Step 0 — two visibility edits (examples can't see the crate yet)

Both are required, and both are one line:

1. **`Cargo.toml`** — find `crate-type = ["cdylib"]` and make it
   `crate-type = ["cdylib", "rlib"]`. A `cdylib` is a finished wasm artifact; examples and
   integration tests link the *library*, which only an `rlib` provides.
2. **`src/lib.rs`** — find `mod app; // App layer for file loading` and make it `pub mod app;`.
   The example says `use session_viewer::app::scene::Scene;`, and a private module ends that
   conversation at the front door.

And one target reality: `.cargo/config.toml` pins **every** cargo command to
`wasm32-unknown-unknown` (that's why the code has no `cfg` gates), so anything headless must
override it explicitly — `--target x86_64-unknown-linux-gnu` on every `cargo run --example` /
`cargo test` below, exactly as 52's BVH test already does. The selftest exercises `Scene`,
picking, and the tables — never fetch/parse, which are browser APIs. If the native build trips
over `persistence.rs` regardless (it's web-only by nature), the honest fix is to say so in the
one place gates are allowed now: in `app/mod.rs`, make it
`#[cfg(target_arch = "wasm32")] pub mod persistence;` — the wasm build is unchanged, the native
build skips the module nothing native can call anyway. And that is the **one** permitted cfg gate
in the viewer — everything else stays target-blind, because the wasm pin's whole point is that
browser APIs fail at *compile* time. If you find yourself reaching for a second `cfg`, the design —
not the target — is what's wrong.

## Step 1 — the headless selftest: `examples/selftest.rs`

Everything below the GPU is plain Rust — `Scene`, picking math, reconcile, the BVH — and runs
natively in milliseconds. The harness isn't new: `src/selftest.rs` already renders a scene headless
through `Gpu::new_headless` (the shader-look harness — it wants a native GPU), and
`examples/selftest.rs` wraps it. What this lesson adds to that example is the **GPU-free assertion
block** below — run it first, before any render, so a broken invariant fails in milliseconds without
touching a device. It asserts the invariants the lessons established:

```rust
//! Headless viewer selftest: no browser, no GPU.
//!   cargo run --example selftest --target x86_64-unknown-linux-gnu
//! Exit 0 = all pass. Run it before every push; CI runs it on every push.

use session_viewer::app::scene::Scene;
use session_rust::{Session, Xform};

fn check(name: &str, ok: bool, fails: &mut u32) {
    println!("{} {}", if ok { "PASS" } else { "FAIL" }, name);
    if !ok { *fails += 1; }
}

fn main() {
    let mut fails = 0;

    // 1. the mesh-heavy fixture loads with the counts 34b documented
    let bytes = std::fs::read("../session_data/floor_model.pb").expect("fixture");
    let session = Session::pb_loads(&bytes).unwrap_or_default();
    check("floor_model: 491 objects", session.lookup.len() == 491, &mut fails);

    // 2. Scene invariants: rows bijective, boxes finite, BVH == brute force (52's parity)
    let mut scene = Scene::new();
    scene.add_file("floor".into(), session, Xform::identity());
    check("rows bijective", scene.rows_are_bijective(), &mut fails);
    check("bvh parity", scene.bvh_matches_brute_force_sample(), &mut fails);

    // 3. a known ray hits a known object (55, no GPU needed — picking is CPU by design):
    // aim straight down at the first row's box center (54's Ray, 53's world_aabb)
    let g0 = scene.order[0].clone();
    let (lo, hi) = scene.world_aabb(&g0);
    let c = [(lo[0] + hi[0]) * 0.5, (lo[1] + hi[1]) * 0.5, (lo[2] + hi[2]) * 0.5];
    let ray = session_viewer::engine::pick::Ray {
        origin: session_rust::Point::new(c[0], c[1], hi[2] + 1000.0),
        dir: session_rust::Vector::new(0.0, 0.0, -1.0),
    };
    check("pick hits", scene.pick_ray(&ray, 1.0).is_some(), &mut fails);

    // 4. copy identity (92): a duplicate MINTS a fresh guid — the Rc-clone trap stays dead
    check("copy identity", copies_get_fresh_guids(&mut scene), &mut fails);

    // 5. reconcile buckets (49): 1 changed / rest unchanged on a perturbed copy
    check("reconcile diff", reconcile_one_changed(&mut scene), &mut fails);

    std::process::exit(if fails == 0 { 0 } else { 1 });
}

// The two helpers are small and local — keep them in the example, not in Scene:

/// 92's identity trap, as a check: clone one object; the copy's guid must DIFFER.
fn copies_get_fresh_guids(scene: &mut Scene) -> bool {
    let g0 = scene.order[0].clone();
    scene.selected.clear();
    scene.selected.insert(0);          // selected is row-keyed (58); row 0 IS order[0]
    let snaps = scene.clone_selection();
    snaps.len() == 1 && snaps[0].geom.guid().to_string() != g0
}

/// 46's diff, as a check: perturb ONE object in a re-parsed copy → exactly one `changed`.
fn reconcile_one_changed(scene: &mut Scene) -> bool {
    let bytes = std::fs::read("../session_data/floor_model.pb").expect("fixture");
    let mut fresh = Session::pb_loads(&bytes).unwrap_or_default();
    // move one object: a new world xform for one guid changes its content hash (46's rule)
    let g0 = scene.order[0].clone();
    fresh.set_xform(&g0, Xform::translation(1.0, 0.0, 0.0));
    let diff = scene.reconcile(&fresh);
    diff.added.is_empty() && diff.removed.is_empty() && diff.changed == vec![g0]
}
```

The deep reason this works: **picking was CPU-side by necessity** (47 — WebGPU has no sync
readback), and the Scene/Gpu split (35) kept the document logic GPU-free. Two decisions made for
other reasons; testability fell out — Step 0's two lines were the only thing missing.

**Fixture paths, native vs browser.** The selftest runs native, so it reads the source-tree fixture
directly (`../session_data/floor_model.pb`, relative to the crate dir — that file exists). The
browser can't do that: trunk serves only the assets root, and a manifest item resolves against it —
so 84's `bunny.obj` manifest line means *copy the kernel fixture into `assets/` first*; a manifest
path will never reach `session_rust/session_data/`. Same fixture, two doors — don't confuse them.

## Step 2 — GPU errors surface in the CLI: `src/engine/gpu/mod.rs`

wgpu reports validation errors asynchronously — by default to the browser console, which the person
staring at a black canvas doesn't think to open. Two hooks route them into the viewer itself:

```rust
    // once, in Gpu::new — the catch-all for anything not scoped below:
    device.on_uncaptured_error(std::sync::Arc::new(|e| {
        // route into a static queue that State drains into the CLI log each frame
        crate::engine::gpu::push_gpu_error(format!("wgpu: {e}"));
    }));
```

```rust
    // around every pipeline build (04's builders) — names the culprit:
    device.push_error_scope(wgpu::ErrorFilter::Validation);
    let pipeline = build_cylinder_pipeline(/* … */);
    let scope = device.pop_error_scope();
    wasm_bindgen_futures::spawn_local(async move {
        if let Some(e) = scope.await {
            push_gpu_error(format!("pipeline 'cylinder': {e}"));
        }
    });
```

(`push_gpu_error` appends to a `static` mutex'd `Vec<String>`; `State` drains it into `ui.log` once
per frame — single-threaded wasm makes this trivially safe. The payoff: a bad WGSL edit now prints
`pipeline 'cylinder': …shader error…` **in the app**, with the pipeline named.)

## Step 3 — the black-screen checklist

Pin this order; it resolves ~all "nothing draws" sessions in minutes:

1. **CLI log / console** — with Step 2, shader and pipeline errors name themselves. Nothing there?
2. **Perf HUD draw count** (28) — `0 draws` means the frame never ran (init failure, panicked
   promise — check the console for the panic hook); `6 draws` but black means the draws land wrong.
3. **Counts** — HUD objects vs expected (the loader logs what loaded). `0 objects` = load/parse
   problem, not rendering.
4. **Camera** — press `F` (fit). Half of all "black screens" are a camera parked inside an object
   or a million units away. Then Space (projection) — an ortho `h` of 0 shows nothing.
5. **Clear color test** — change 03's clear to red. Red screen = swapchain fine, geometry path
   suspect; still black = surface/canvas sizing (05's DPR block).
6. **Bisect the passes** — comment draws out in `clear()` back-to-front; the pass whose removal
   changes nothing is the broken one. (This is why `clear()` stays a readable list of draw blocks.)

## Step 4 — CI: `.github/workflows/viewer.yml`

```yaml
name: viewer
on: [push]
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with: { submodules: recursive }
      - uses: dtolnay/rust-toolchain@stable
        with: { targets: wasm32-unknown-unknown }
      - run: cargo install trunk --locked
      # headless invariants, seconds — the explicit --target overrides .cargo/config.toml's wasm pin
      - run: cargo run --example selftest --target x86_64-unknown-linux-gnu
        working-directory: session_viewer
      - run: trunk build --release               # the wasm actually builds
        working-directory: session_viewer
```

Selftest first (fast, most informative), release build second (catches wasm-only breakage —
`std::fs` in browser paths, missing web-sys features; the native selftest can't see those). No
headless-browser GPU testing — WebGPU in CI containers is still fragile; the CPU selftest plus a
compiling wasm covers what's coverable.

## Step 5 — verify

- `cargo run --example selftest --target x86_64-unknown-linux-gnu` → PASS lines, exit 0, no
  browser anywhere. Break something real — swap one of `clone_selection`'s `duplicate()` arms for
  a bare `Rc::clone` (92's trap) → `copy identity` FAILs (the "copy"'s guid equals its source's)
  and the exit code is 1. Restore the arm.
- Put a typo in `cylinder.wgsl`, `trunk serve` → the **CLI log** reads
  `pipeline 'cylinder': Shader validation error…` — no F12 required. Fix it, error gone.
- Walk the checklist once deliberately (park the camera at 1e9, "debug" it back) so the order is
  muscle memory before a real emergency.

## Recap

```
Ch 87: the viewer answers questions about the model.
Ch 88: THE WORKFLOW. Two visibility lines first: crate-type gains "rlib" (examples can't link a
       cdylib) and `mod app` goes pub — and every headless command carries
       --target x86_64-unknown-linux-gnu, because .cargo/config.toml pins wasm for everything
       (persistence is web-only; if native chokes on it, gate it #[cfg(target_arch = "wasm32")]
       and say so). Headless selftest (examples/selftest.rs): fixture counts, Scene::new +
       add_file invariants, BVH parity, a CPU pick, copy identity (92's duplicate-vs-Rc-clone
       trap), reconcile buckets — possible because picking was CPU-by-necessity (55) and Scene was
       GPU-free by design (35); seconds, exit-coded, no browser. GPU errors: on_uncaptured_error +
       per-pipeline error scopes → a static queue → the CLI log, with the pipeline NAMED. The
       black-screen checklist: log → draw count → object count → camera(F!) → clear-color →
       bisect passes. CI: selftest + trunk build --release; no headless-GPU testing (fragile),
       and honestly so.
```

Edited: `examples/selftest.rs` (NEW), `Cargo.toml` (`+ "rlib"`), `src/lib.rs` (`pub mod app`),
`engine/gpu/mod.rs` (error scopes + queue), `.github/workflows/viewer.yml` (NEW).

## Next

`107-web-polish.md` — the last mile: a progress bar for the 17.5 MB stress file, and a wasm binary
worth shipping.
