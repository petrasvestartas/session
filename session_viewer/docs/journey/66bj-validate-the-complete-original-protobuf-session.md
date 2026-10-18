# 12.1.13 · Validate the complete original protobuf session

**Chapter 12 · Files and streaming · Section 12.1 · Scene manifests and range reads** · [Contents](contents.md)

**Typing: 18–36 minutes.** [Estimate](typing-load.md).

Count original geometry records and apply the already-written per-type validators. Validate every placement. Walk the original tree with an explicit depth stack, allowing depth sixty-four and rejecting a deeper child. This orchestration adds budgets and traversal, not another implementation of mesh, curve or cloud checks.

## Type

Continue from [Validate BRep Element payloads and shared definitions](66bi-validate-brep-element-payloads-and-shared-definitions.md). [Save or recover your work](recovery.md).

### 1. `src/source_validate.rs`

Apply the existing per-record validators and enforce total original-object and hierarchy-depth budgets before allocation.

<details>
<summary>Locate the existing block</summary>

```rust
    }

    for item in &d.lines {
        line(item)?;
    }

    for item in &d.polylines {
        polyline(item)?;
    }

    for item in &d.meshes {
        mesh(item)?;
    }

    for item in &d.pointclouds {
        cloud(item)?;
    }

    for item in &d.nurbscurves {
        curve(item)?;
    }

    for item in &d.nurbssurfaces {
        surface(item)?;
    }

    for item in &d.breps {
        brep(item)?;
    }

    for item in &d.elements {
        element(item)?;
    }

    Ok(())
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/66bj-validate-the-complete-original-protobuf-session-01.rs" skip=500
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 12.1.13
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

The original cloud and local point marker remain unchanged.

![Native renderer output for validate the complete original protobuf session.](../screenshots/journey/65za-read-original-manifest-records-and-placements.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [11.3.5 · Read original manifest records and placements](65za-read-original-manifest-records-and-placements.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original record → bounded scalar/address contract → reusable validator → later archive loader.



Why does whole-session validation happen before kernel construction?

Declared records, transforms and hierarchy must pass their budgets and record checks before constructors can allocate from them.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Place a valid mesh and an invalid placement in one source session. Predict whether partial geometry should appear after import. Then compare a tree reaching depth sixty-four with one additional child.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 12.1.13
npm --prefix ../session_tests run course -- save 12.1.13
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



Compile the cumulative WebAssembly viewer. Independent malformed-record fixtures are registered with the completed validator module.

This preparation is checked by the following original-source import and archive browser exercises.

[Full validation scope](release.md).

</details>
