# 06 · Preserve CAD faces in display data

**Start:** checkpoint 05. **Finish:** the CAD fixture passes through source geometry, face meshes and stable boundary records.

## What a BRep contributes

A boundary representation separates topology from surface geometry. A face refers to a supporting surface and oriented boundary uses. An edge has a source identity and can belong to multiple faces. A NURBS surface maps parameters `(u, v)` to a 3D point; a trim curve describes the region of that parameter domain that belongs to the face.

```text
BRep face + surface + trim uses
              ↓ shared geometry producer
positions / triangles / UVs / normals / boundary provenance
              ↓ viewer preparation
display rows carrying parent + source face/edge identities
```

The viewer needs more than an anonymous triangle soup. A face's source index must survive triangulation. A rendered boundary chain must identify the original edge. Shading vertices can be duplicated at a crease without creating new CAD vertices.

## Read the mesh contract before the mesher

The shared mesh records carry positions and indexed triangles. UV values connect samples back to the supporting surface. Boundary metadata connects original loop samples and inserted interval samples to their source use. These fields let the consumer reuse actual face mesh nodes for ink.

The GPU vertex layout is a separate packed contract. Geometry remains f64 until preparation converts it to the object's local display representation. Boundary endpoints and triangle positions must use the same conversion; independent rounding can reintroduce a gap even if source coordinates matched.

Normals also have a source meaning. A valid analytic surface normal can describe a curved interior better than an average of coarse triangles. A singular derivative needs a real fallback; an arbitrary default vector must not masquerade as a valid derivative.

## Shared code is a dependency with a contract

This lesson changes the supplied Rust geometry producer and its independent C++/Python equivalents. The Rust version is the one used by this browser build. The other implementations demonstrate parity of the geometry API and are provided in full for verification.

Keep rendering-only decisions in the viewer. CSS widths, projected visibility tiles and GPU buffer packing are not changes to the CAD file format or geometry kernel.

## Write the files

Follow [Complete file changes for 06](../lessons/06/index.md). Source paths beginning `session_rust/`, `session_cpp/` or `session_py/` are siblings of `session_viewer`, beneath `$COURSE_WORK`. Do not create them inside the viewer's `src` directory.

Type the Rust mesh/face contract and follow how the producer fills it. Copy the independent parity files and fixture tools. Then wire the viewer consumer. The complete file pages include all required imports and module exports.

## Checkpoint

```sh
cd "$COURSE_WORK/session_viewer"
cargo check --locked --lib
trunk serve --port 8780
```

The local CAD fixture at <http://localhost:8780/?data=off&inspect=1> should show a substantial shaded model, with boundaries separate from fill. The browser check deliberately tests shaded pixels rather than assuming every CAD fixture is brightly colored.

Inspect a boundary chain's source ID and its referenced mesh nodes. If the chain is unavailable, record it as unavailable; do not invent an edge ID from a triangle's index.

**Before continuing:** explain how one topological edge can have multiple oriented face uses. Continue to [coherent boundaries](07-boundaries.md).
