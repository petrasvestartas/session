# 15.3.8 · Sample original network curve stations

**Chapter 15 · Transforms and surface construction · Section 15.3 · Lofts, sweeps and Gordon surfaces** · [Contents](contents.md)

**Typing: 23–46 minutes.** [Estimate](typing-load.md).

Write control bounds, tolerant crossing checks and interpolation helpers. Map evenly spaced stations to the original crossing parameters.

## Type

Continue from [Exercise original sections rail groups and exact persistence](79d-exercise-original-sections-rail-groups-and-exact-persistence.md). [Save or recover your work](recovery.md).

### 1. `src/network_geometry.rs`

Type the whole original helpers introduced today.

Create the file and type:

```rust
--8<-- "journey/code/79zza-sample-original-network-curve-stations-1.rs"
```

### 2. `src/lib.rs`

Register the original network-surface owner.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod loft_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/79zza-sample-original-network-curve-stations-2.rs" skip=29
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 15.3.8
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Network helpers can sample stations on the original curves; browser drawing is unchanged.

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for sample original network curve stations.](../screenshots/journey/79zza-sample-original-network-curve-stations.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

<details>
<summary>Code explanation and diagram</summary>



original curve crossings → ordered stations → sampled lofts → fitted surface.



Why retain the original parameters?

Equal stations describe the grid layout; original curve parameters still determine the actual surface points.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare nonuniform crossing intervals and the sample-count cap.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 15.3.8
npm --prefix ../session_tests run course -- save 15.3.8
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

These complete helpers decompose the original network algorithm without introducing a second implementation.



[Full validation scope](release.md).

</details>
