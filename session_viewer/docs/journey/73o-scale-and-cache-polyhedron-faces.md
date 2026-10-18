# 14.3.16 · Scale and cache polyhedron faces

**Chapter 14 · Coordinates and geometry tools · Section 14.3 · Planar shapes and regular solids** · [Contents](contents.md)

**Typing: 19–38 minutes.** [Estimate](typing-load.md).

Cache unit polyhedron face outlines, then apply the requested circumradius and the existing placement frame. Supply the original frame validation fixture separately.

## Type

Continue from [Weld solid tessellation once](73n-weld-solid-tessellation-once.md). [Save or recover your work](recovery.md).

### 1. `src/primitive_shape.rs`

Continue the shared solid foundation with this lesson’s functions.

<details>
<summary>Locate the existing block</summary>

```rust
    mesh.name = name.into();
    Geometry::Mesh(Rc::new(mesh))
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/73o-scale-and-cache-polyhedron-faces-1.rs" skip=66
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/primitive_shape.rs`

Supply the existing original frame fixture and reusable builder validation helpers.

<details>
<summary>Locate the existing block</summary>

```text
    let mesh = make(radius / circumradius(&make(1.0)));
    Ok(polyhedron(mesh, option, &part.frame.to_xform(), name))
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/73o-scale-and-cache-polyhedron-faces-2.rs" skip=123
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 14.3.16
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

![Native renderer output for scale and cache polyhedron faces.](../screenshots/journey/73h-match-solid-command-options.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [14.3.9 · Match solid command options](73h-match-solid-command-options.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



Scale and cache polyhedron faces → original f64 kernel coordinates → later temporary preview and accepted geometry.



Why normalize by circumradius instead of scaling by one chosen vertex coordinate?

Circumradius measures the furthest corner from the origin and is independent of axis orientation.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare mesh and BRep output for a small radius. Explain the BRep tolerance check before rebuilding the browser.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 14.3.16
npm --prefix ../session_tests run course -- save 14.3.16
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This codes the shared solid-command foundation before connecting the individual builders. The browser scene is unchanged at this checkpoint.

The supplied original frame fixture checks handedness and placement on Top, Front and Right planes. Native fixtures are author checks; learners build and run the browser.

[Full validation scope](release.md).

</details>
