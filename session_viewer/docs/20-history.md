# 20 · The document: undo, redo and save

The sheet scene stays visible while document edits gain undo, redo and history-free saving.

![Edits group into transactions and a removal leaves a tombstone to restore from; the cursor moves back and forward through them, and a save purges the whole buffer because history never crosses pb or JSON.](illustrations/history.svg)

## Step 1 · session_rust/src/history.rs

Read this source file from its link; the checkpoint already contains it.

???

    ```rust
    --8<-- "session_rust/src/history.rs"
    ```
    

## Step 2 · session_rust/src/session.rs

Read this source file from its link; the checkpoint already contains it.

???

    ```rust
    --8<-- "session_rust/src/session.rs"
    ```
    

Run `cargo check` in `lessons/20/`.

## Check

Run `trunk serve` in `lessons/20/` and open <http://127.0.0.1:8770/>.

Expected: The sheet scene stays visible while document edits gain undo, redo and history-free saving; status: **the status clears when loading finishes**.

[![Full viewer result for 20 history](screenshots/19-sheets-overview.png)](screenshots/19-sheets-overview.png)

If it fails:

- Undo fails to restore a removed object: its geometry and tree position are not both recorded.
- Saved history reappears: serialization includes the undo stack.

## What changed

```text
lessons/20/session_rust/src/
├── bin/
│   ├── minitest.rs
│   └── pdf_import.rs
├── proto/
│   ├── .gitattributes
│   └── session_proto.rs
├── aabb.rs
├── boolean_polyline.rs
├── brep.rs
├── closest.rs
├── color.rs
├── convex_hull.rs
├── element.rs
├── file_encoders.rs
├── file_obj.rs
├── file_step.rs
├── graph.rs
├── guid_serde.rs
├── history.rs
├── instance_ref.rs
├── intersection.rs
├── io_xyz.rs
├── lib.rs
├── line.rs
├── main.rs
├── matrix.rs
├── mesh.rs
├── mesh_offset.rs
├── nurbscurve.rs
├── nurbsknot.rs
├── nurbssurface.rs
├── nurbssurface_trimmed.rs
├── obb.rs
├── objects.rs
├── pdf.rs
├── plane.rs
├── point.rs
├── pointcloud.rs
├── polyline.rs
├── primitives.rs
├── quaternion.rs
├── remesh_cdt.rs
├── remesh_nurbssurface_adaptive.rs
├── remesh_nurbssurface_grid.rs
├── render_mesh.rs
├── session.rs
├── session_config.rs
├── simple_split.rs
├── spatial_aabbtree.rs
├── spatial_bvh.rs
├── spatial_kdtree.rs
├── spatial_octree.rs
├── spatial_rtree.rs
├── tolerance.rs
├── tree.rs
├── vector.rs
└── xform.rs
```

`+` new in this lesson · `~` changed in this lesson

Every file at this point: `lessons/20/`.

## Next

[21 · Editing: the gumball, the command line and the layers panel](21-editing.md)

## Expected viewer result

The picture is unchanged from lesson 19; undo, redo and save are the new behavior.

[![Full viewer result for 20 history](screenshots/19-sheets-overview.png)](screenshots/19-sheets-overview.png)
