# 12.1.12 · Validate BRep Element payloads and shared definitions

**Chapter 12 · Files and streaming · Section 12.1 · Scene manifests and range reads** · [Contents](contents.md)

**Typing: 20–40 minutes.** [Estimate](typing-load.md).

Validate BRep curve, surface and vertex records using the same existing geometry contracts. Decode an Element’s embedded Mesh or BRep payload according to its declared type and preserve decoding errors. Finally traverse shared definition records and bound their count together with instance references.

## Type

Continue from [Validate cloud columns and octree addresses](66bh-validate-cloud-columns-and-octree-addresses.md). [Save or recover your work](recovery.md).

### 1. `src/source_validate.rs`

Check BRep records and encoded Element geometry, then validate every shared definition without expanding instances into copied geometry.

<details>
<summary>Locate the existing block</summary>

```rust
            if source.lod_first[node] < 0
                || source.lod_count[node] < 0
                || source.lod_first[node] as u64 + source.lod_count[node] as u64 > points as u64
            {
                return Err("octree range exceeds its source points".into());
            }
        }

        for child in &source.lod_children {
            if *child < -1 || *child >= nodes as i32 {
                return Err("octree references an absent child".into());
            }
        }
    }

    Ok(())
}
```

</details>

After that block, type:

```rust

/// Check one protobuf BRep.
pub fn brep(source: &proto::BRep) -> Result<(), String> {
    for item in &source.curves_2d {
        curve(item)?;
    }

    for item in &source.curves_3d {
        curve(item)?;
    }

    for item in &source.surfaces {
        surface(item)?;
    }

    for vertex in &source.vertices {
        if let Some(point) = &vertex.point {
            finite(&[point.x, point.y, point.z], "BRep vertex")?;
        }
    }

    Ok(())
}

/// Check one protobuf element's mesh or BRep.
pub fn element(source: &proto::Element) -> Result<(), String> {
    match source.geometry_type.as_str() {
        "Mesh" => {
            mesh(&proto::Mesh::decode(source.geometry_data.as_slice()).map_err(decode_error)?)
        }
        "BRep" => {
            brep(&proto::BRep::decode(source.geometry_data.as_slice()).map_err(decode_error)?)
        }
        _ => Ok(()),
    }
}

/// Check every definition record, and the definitions with `instances` within the cap.
pub fn definitions(d: &proto::Objects, instances: usize) -> Result<(), String> {
    let count = d.points.len()
        + d.lines.len()
        + d.polylines.len()
        + d.meshes.len()
        + d.pointclouds.len()
        + d.nurbscurves.len()
        + d.nurbssurfaces.len()
        + d.breps.len()
        + d.elements.len();

    if count + instances > MAX_OBJECTS {
        return Err(TOO_MANY.into());
    }

    for item in &d.points {
        point(item)?;
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

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/66bi-validate-brep-element-payloads-and-shared-definitions-01.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 12.1.12
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

![Native renderer output for validate brep element payloads and shared definitions.](../screenshots/journey/65za-read-original-manifest-records-and-placements.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [11.3.5 · Read original manifest records and placements](65za-read-original-manifest-records-and-placements.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original record → bounded scalar/address contract → reusable validator → later archive loader.



Why validate shared definitions without expanding their instances?

The source stores a definition once and separate instance references. Expanding them would change ownership and multiply work before validation.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace an Element declaring Mesh with malformed payload bytes, then a shared BRep definition with two references. Identify the checks that run once and the count that includes the references.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 12.1.12
npm --prefix ../session_tests run course -- save 12.1.12
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



This preparation is checked by the following original-source import and archive browser exercises.

[Full validation scope](release.md).

</details>
