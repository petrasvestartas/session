# 16 · Resource accounting

The scene stays visible while the inspection snapshot reports retained source memory.

![Scene owns documents through Rc; the cache keeps Weak identities and a payload figure, reuses it while the pointers match, walks once when a document is replaced, and never keeps a dropped document alive.](illustrations/source-cache.svg)

Copy each file from the lesson folder to the path shown.

Copy from `lessons/16/` (tooling this checkpoint needs but the course does not teach):

- `lessons/16/.gitignore`
- `lessons/16/assets/pb/.gitkeep`
- `lessons/16/examples/add_lod.rs`
- `lessons/16/examples/cad_boundary_audit.rs`
- `lessons/16/examples/cad_fixture.rs`
- `lessons/16/examples/census_plates.rs`
- `lessons/16/examples/check_cad_fixture.rs`
- `lessons/16/examples/check_determinism.rs`
- `lessons/16/examples/check_hidden_line_lifecycle.rs`
- `lessons/16/examples/interaction_fixture.rs`
- `lessons/16/examples/mk_brep_probe.rs`
- `lessons/16/examples/mk_cylinder_hidden_probe.rs`
- `lessons/16/examples/mk_hidden_line_probe.rs`
- `lessons/16/examples/mk_joint_probe.rs`
- `lessons/16/examples/mk_mixed_solids.rs`
- `lessons/16/examples/mk_plate_outline.rs`
- `lessons/16/examples/mk_shade_probe.rs`
- `lessons/16/examples/mk_teapot.rs`
- `lessons/16/examples/selftest.rs`
- `lessons/16/src/selftest.rs`
- `lessons/16/src/selftest/lifecycle.rs`
- `lessons/16/tests/README.md`
- `lessons/16/tests/cad-boundary-plot.py`
- `lessons/16/tests/cad-quality.py`
- `lessons/16/tests/depth/_closeup_box.py`
- `lessons/16/tests/depth/_count_colors.py`
- `lessons/16/tests/depth/_gate.sh`
- `lessons/16/tests/depth/_hidden_line_matrix.py`
- `lessons/16/tests/depth/_ink_suite.sh`
- `lessons/16/tests/depth/_orbit_check.py`
- `lessons/16/tests/depth/_probe_matrix.py`
- `lessons/16/tests/depth/_shade_scanline.py`
- `lessons/16/tests/depth/_stroke_weight.py`
- `lessons/16/tests/format.py`
- `lessons/16/tests/interaction.cjs`
- `lessons/16/tests/nameplate-scene.cjs`
- `lessons/16/tests/nameplate.cjs`
- `lessons/16/tests/teapot.cjs`
- `lessons/16/tests/text-quality.cjs`
- `lessons/16/tests/world-text.cjs`

## Step 1 · Cargo.toml

Copy this file from the lesson folder to the path shown.

`lessons/16/Cargo.toml` · edit · copy the file

Replaces the 16 lines from `getrandom = { version = "0.2", features = ["j…` of `lessons/15/Cargo.toml`

```toml
--8<-- "lessons/16/Cargo.toml:step-1"
```

## Step 2 · src/app/inspection/source_memory.rs

The source cache counts retained payload once per shared identity.

`lessons/16/src/app/inspection/source_memory.rs` · type this, new file, start with these lines

```rust
--8<-- "lessons/16/src/app/inspection/source_memory.rs:step-2a"
```

`lessons/16/src/app/inspection/source_memory.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/16/src/app/inspection/source_memory.rs:step-2b"
```

`lessons/16/src/app/inspection/source_memory.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/16/src/app/inspection/source_memory.rs:step-2c"
```

Copy this part from the lesson folder to the path shown.

`lessons/16/src/app/inspection/source_memory.rs` · copy the file, append at the end of the file

```rust
--8<-- "lessons/16/src/app/inspection/source_memory.rs:step-2d"
```

Copy this part from the lesson folder to the path shown.

`lessons/16/src/app/inspection/source_memory.rs` · copy the file, append at the end of the file

```rust
--8<-- "lessons/16/src/app/inspection/source_memory.rs:step-2e"
```

Run `cargo check` in `lessons/16/`.

## Step 3 · src/app/inspection.rs

Inspection reports retained resources and source information.

`lessons/16/src/app/inspection.rs` · edit · type this

Added after the `use crate::State;` line of `lessons/15/src/app/inspection.rs`

```rust
--8<-- "lessons/16/src/app/inspection.rs:step-3a"
```

Added after the `let (buffers, textures) = state.gpu.allocated…` line in `fn publish` of `lessons/15/src/app/inspection.rs`

```rust
--8<-- "lessons/16/src/app/inspection.rs:step-3b"
```

Added after the `"samples": state.gpu.targets.samples,` line in `fn publish` of `lessons/15/src/app/inspection.rs`

```rust
--8<-- "lessons/16/src/app/inspection.rs:step-3c"
```

## Check

Run `trunk serve` in `lessons/16/` and open <http://127.0.0.1:8770/>.

Expected: The scene stays visible while the inspection snapshot reports retained source memory; status: **the status clears when loading finishes**.

![Checkpoint 16: the scene is unchanged; the new figures live in the inspection snapshot above.](screenshots/16.png)

If it fails:

- Payload bytes double for shared files: shared source values are counted more than once.
- The scan count grows each frame: accounting is not cached by document identity.

## What changed

```text
lessons/16/src/app/
├── inspection/
│   └── source_memory.rs  +
├── walk/
│   ├── bounds.rs
│   ├── brep.rs
│   ├── brep_edges.rs
│   ├── brep_orient.rs
│   ├── cloud.rs
│   ├── curves.rs
│   ├── encode.rs
│   ├── frames.rs
│   ├── mesh.rs
│   ├── mesh_ink.rs
│   ├── mesh_topology.rs
│   ├── mod.rs
│   └── points.rs
├── cloud_query.rs
├── decode.rs
├── feedback.rs
├── fetch.rs
├── input.rs
├── inspection.rs  ~
├── knobs.rs
├── live.rs
├── loader.rs
├── manifest.rs
├── mod.rs
├── route.rs
├── scene.rs
├── selection.rs
├── stream.rs
├── touch.rs
└── validate.rs
```

`+` new in this lesson · `~` changed in this lesson

Every file at this point: `lessons/16/`.

## Next

[17 · Faces, text objects and silhouettes](17-source-presentation.md): source-face selection, selectable authored text and one black outline.

## Expected viewer result

Checkpoint 16: the scene is unchanged; the new figures live in the inspection snapshot above.

[![Full viewer result for 16 accounting](screenshots/16.png)](screenshots/16.png)
