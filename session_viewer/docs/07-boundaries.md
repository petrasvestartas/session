# 07 · Make boundaries belong to the face mesh

**Start:** checkpoint 06. **Finish:** adjoining faces use one canonical boundary polygon and preserve its original edge identity.

## Sharing endpoints is insufficient

Two faces can agree on an edge's endpoints and still approximate the curve with different intermediate chords. Independently sampling a NURBS curve for ink creates a third approximation. Those representations can overlap, separate or hide one another as the camera moves.

```text
original BRep edge
       ↓ choose/refine one canonical XYZ chain
       ├─ recover parameters on face A's actual pcurve → constrain face A mesh
       └─ recover parameters on face B's actual pcurve → constrain face B mesh
                              ↓
                return ordered mesh-node chains
                              ↓
                use those same nodes for boundary ink
```

The canonical chain fixes display geometry correspondence. It does not by itself decide screen-space visibility; the later finite-triangle pass addresses that separate problem.

## Follow the producer in order

First collect each edge's oriented face uses. Choose the shared XYZ samples once and retain original caller samples exactly. Refine curved boundary intervals before refining interior triangles: interior points moved onto a curved surface can otherwise crowd a coarse boundary chord and bury it.

Next map each shared 3D sample onto the incident face's actual pcurve. A pcurve is a curve in surface parameter space. A periodic surface can have multiple parameter coordinates for the same XYZ location, so unconstrained surface inversion may choose the wrong branch. The producer validates the lifted point and uses the bounded search on that specific pcurve when necessary.

Then give the constrained mesher the trim polygons, matching XYZ positions and interior seeds through `TrimLoops`. The mesher must retain boundary constraints while inserting interior points. After triangulation, return ordered node chains with provenance so the viewer can draw exactly those mesh edges.

## Provenance survives refinement

Original samples retain `boundary/{loop}/{sample}` metadata. Inserted samples retain `boundary_interval/{loop}/{segment}` fractions. An interval fraction belongs to the input polygon segment; it is **not automatically a NURBS curve parameter**. The BRep producer translates this information back to its original edge table and face uses.

When an adjoining grid has different boundary sampling, rebuild that face with the canonical polygon and its prior interior UV seeds. Do not leave both incompatible grids in the display. Sorting seeds makes the result independent of incidental map iteration order.

## Readable failures are part of the contract

Invalid trims, lost required provenance or an unconstrained C0 crossing cannot produce a trustworthy boundary mapping. The producer rejects those cases rather than manufacturing a selectable topology edge. An explicit analytic fallback is a display fallback and must not pretend to have source provenance.

The [CAD design record](cad-design.md) explains the OCCT source comparison and the bounded refinement policy. This implementation is Session's own kernel, not an OCCT runtime.

## Write the files

Follow [Complete file changes for 07](../lessons/07/index.md). Read `TrimLoops`, the constrained mesher, then BRep shared-edge assembly and the viewer's chain consumer. Copy the independent parity implementations and regression fixtures. All full replacements are provided; there is no hidden block to recover from a patch.

## Checkpoint

```sh
cd "$COURSE_WORK/session_viewer"
cargo check --locked --lib
trunk serve --port 8780
```

At <http://localhost:8780/?data=off&inspect=1>, inspect shared boundaries and holes while orbiting. A boundary should remain attached to its face; internal tessellation diagonals should not appear as invented CAD edges.

For debugging, compare source XYZ chains first, then converted f32 endpoints, then visibility. Increasing subdivisions without identifying which contract differs can make a model heavier while leaving the same defect.

**Before continuing:** explain why both faces must use newly refined shared samples. Continue to [trims and seams](08-trimming.md).
