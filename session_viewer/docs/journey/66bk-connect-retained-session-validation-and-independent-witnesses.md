# 12.1.14 · Connect retained-session validation and independent witnesses

**Chapter 12 · Files and streaming · Section 12.1 · Scene manifests and range reads** · [Contents](contents.md)

**Typing: 6–12 minutes.** [Estimate](typing-load.md).

Connect the retained-session adapter for meshes, clouds, NURBS curves, surfaces and BReps. Add the supplied independent malformed-count, missing-index and JSON-layout fixtures. The following archive lessons now have one complete validation owner for original input; no geometry is copied or normalized by these checks.

## Type

Continue from [Validate the complete original protobuf session](66bj-validate-the-complete-original-protobuf-session.md). [Save or recover your work](recovery.md).

### 1. `src/source_validate.rs`

Reuse the protobuf validators for already-decoded original kernel objects and register independent malformed-record witnesses.

<details>
<summary>Locate the existing block</summary>

```rust
            element(item)?;
        }
    }

    for entry in &source.xforms {
        xform(entry)?;
    }

    if let Some(tree) = &source.tree
        && let Some(root) = &tree.root
    {
        let mut pending = vec![(root, 0usize)];

        while let Some((node, depth)) = pending.pop() {
            if depth > MAX_DEPTH {
                return Err(TOO_DEEP.into());
            }

            for child in &node.children {
                pending.push((child, depth + 1));
            }
        }
    }

    Ok(())
}
```

</details>

After that block, type:

```rust

/// Check every geometry of a loaded session.
pub fn retained(source: &session_rust::Session) -> Result<(), String> {
    for item in &source.objects.meshes {
        mesh(&item.to_proto())?;
    }

    for item in &source.objects.pointclouds {
        cloud(&item.to_proto())?;
    }

    for item in &source.objects.nurbscurves {
        curve(&item.to_proto())?;
    }

    for item in &source.objects.nurbssurfaces {
        surface(&item.to_proto())?;
    }

    for item in &source.objects.breps {
        brep(&item.to_proto())?;
    }

    Ok(())
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/66bk-connect-retained-session-validation-and-independent-witnesses-01.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/source_validate.rs`

Supply independent hostile-count and original-record rejection fixtures.

<details>
<summary>Locate the existing block</summary>

```text
ined(source: &session_rust::Session) -> Result<(), String> {
    for item in &source.objects.meshes {
        mesh(&item.to_proto())?;
    }

    for item in &source.objects.pointclouds {
        cloud(&item.to_proto())?;
    }

    for item in &source.objects.nurbscurves {
        curve(&item.to_proto())?;
    }

    for item in &source.objects.nurbssurfaces {
        surface(&item.to_proto())?;
    }

    for item in &source.objects.breps {
        brep(&item.to_proto())?;
    }

    Ok(())
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/66bk-connect-retained-session-validation-and-independent-witnesses-02.rs" skip=500
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 12.1.14
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

![Native renderer output for connect retained-session validation and independent witnesses.](../screenshots/journey/65za-read-original-manifest-records-and-placements.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [11.3.5 · Read original manifest records and placements](65za-read-original-manifest-records-and-placements.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original record → bounded scalar/address contract → reusable validator → later archive loader.



Why retain a validator for already-decoded kernel sessions?

Loaded or constructed original records may still need the same checks before being retained or archived. Reuse their protobuf representation and the existing record validators.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Read each supplied malformed-record case and predict its failed contract before running the browser build. In the following archive exercise, import one invalid document and verify that the previous drawing remains intact.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 12.1.14
npm --prefix ../session_tests run course -- save 12.1.14
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The supplied malformed-record fixtures execute against the completed original-source validators; the learner uses the browser build and later import exercises.

This preparation is checked by the following original-source import and archive browser exercises.

[Full validation scope](release.md).

</details>
