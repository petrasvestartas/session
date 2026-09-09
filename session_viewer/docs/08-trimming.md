# 08 · Respect holes and periodic seams

**Start:** checkpoint 07. **Finish:** trimmed surfaces, natural boundaries and repeated seam uses retain correct geometry and source mappings.

## A parameter rectangle is not always a face

An untrimmed NURBS surface has a natural parameter domain. A trimmed face uses outer and inner loops to select part of that domain. Triangulating the whole rectangular grid and merely drawing a hole curve on top does not create a hole; the fill must exclude the trimmed-out region.

```text
surface domain + outer loop + inner loops
                     ↓ constrained UV triangulation
             triangles inside the allowed region
                     ↓ evaluate/lift with boundary XYZ constraints
                 trimmed 3D face and source chains
```

The constrained mesher inserts trim segments as constraints, classifies allowed regions and preserves boundary provenance. The viewer consumes the resulting face triangles and chains; it does not infer holes from screen-space linework.

## Periodic surfaces repeat coordinates

A cylinder's seam can be the same 3D curve used on two sides of the parameter domain. Those uses may have opposite orientation and different UV coordinates. Keep both face-use meanings even when their XYZ samples coincide.

A collapsed pole is another special case: multiple parameter samples can map to one point. A zero-length geometric edge should not become a long invalid stroke. Its topology can still exist in source data. Distinguish unavailable display length from missing source identity.

Natural boundaries belong to untrimmed surface limits. Trimmed loops belong to actual trim uses. Do not invent visible trim curves for a region the source never defined.

## Keep identity independent of triangle order

Reordering triangles for storage or rebuilding a constrained mesh must not change which source edge is selected. Boundary intervals and original edge tables supply that identity. Approximate nearest-position matching cannot distinguish repeated seam uses reliably.

The same logic applies to F10: control points come from the original surface/curve definition, not from every display subdivision inserted by the mesher.

## Write the files

Follow [Complete file changes for 08](../lessons/08/index.md). This stage updates the surface/boundary consumer using the producer contract already established in lesson 07. Read the handling of natural boundaries, trims and periodic uses beside the returned chain IDs.

## Checkpoint

```sh
cd "$COURSE_WORK/session_viewer"
cargo check --locked --lib
trunk serve --port 8780
```

At <http://localhost:8780/?data=off&inspect=1>, inspect the trimmed fixture from above and obliquely. The hole must remain empty in the fill, not merely outlined. Boundaries should remain attached while orbiting. A seam can be visible ink even though the surface is geometrically smooth across it.

If a periodic boundary crosses the wrong part of the surface, inspect UV branch choice and oriented use mapping before changing stroke depth.

**Before continuing:** explain why a seam and a sharp shading crease are different concepts. Continue to [normals and shading](09-normals.md).
