# 09 · Shade curved interiors and preserve sharp faces

**Start:** checkpoint 08. **Finish:** curved faces shade smoothly while planar face boundaries, C0 creases and singular poles remain well defined.

## Geometry and shading use different continuity

A coarse mesh approximates a curved surface with planar triangles. Interpolated surface normals can make lighting vary smoothly across those triangles. That is appropriate within one smooth face. Averaging normals across unrelated BRep faces makes a planar polyhedron appear curved and is incorrect.

```text
surface derivatives → valid analytic normal ───────┐
                  singular derivative → fallback ├─ face-local shading vertices
                    C0 knot / sharp face → split ┘
                                     ↓ instance normal transform
                               normalized fragment normal
```

At a smooth interior, evaluate derivatives and normalize their cross product. At a singularity, first check whether those derivatives actually define a valid normal. A default +Z sentinel must not be treated as evidence that a cone apex or sphere pole has a valid analytic derivative.

## Separate faces and crease sides

BRep face meshes retain their own normals. At a true C0 knot crossing within a surface, use one-sided values and split shading vertices along the crease. Those copies can occupy identical XYZ positions while carrying different normals. They retain their source/boundary provenance.

A periodic seam is not necessarily a crease. Repeated parameter coordinates alone do not justify a lighting discontinuity. Conversely, welding by XYZ position alone can erase a real sharp edge.

## Transform normals correctly

Positions use the instance model matrix. Normals require its inverse transpose, followed by normalization. Nonuniform scaling makes the difference visible: multiplying a normal like a position can tilt it away from the transformed surface.

The shader uses the cofactor form and handles determinant sign. Mirrored instances also affect front/back interpretation. Singular transforms return a zero-normal sentinel and the shading path uses a finite face fallback rather than propagating NaNs.

Normalize the interpolated normal in the fragment shader. Interpolation does not preserve unit length, and lighting intensity should not depend on that accidental length.

## Write the files

Follow [Complete file changes for 09](../lessons/09/index.md). Read shared analytic/fallback normal generation first, then the shader normal transform and triangle lighting. Copy binary fixtures using the exact command on the file list before checking the stage.

The full Rust/C++/Python changes include tests for singular normals, C0 splits and matching geometry. The [CAD design record](cad-design.md) explains quality bounds and source-informed choices.

## Checkpoint

```sh
cd "$COURSE_WORK/session_viewer"
cargo check --locked --lib
trunk serve --port 8780
```

Open these exact fixture views:

- [Sphere](http://localhost:8780/?cad=sphere): smooth interior shading.
- [Cylinder](http://localhost:8780/?cad=cylinder): smooth side and separate cap normals.
- [Crease](http://localhost:8780/?cad=crease): one-sided shading across a C0 fold.
- [Affine copies](http://localhost:8780/?cad=cylinder&affine=1): transformed instances.
- [Fill only](http://localhost:8780/?cad=cylinder&fill=1): inspect lighting without boundary ink.

The same selector accepts `hole`, `trimmed` and `torus`. Add `&perspective=1` for the teaching shell's perspective view. This checkpoint has no unlit toggle; do not look for the later production lighting shortcut here. A visually subtle crease under one light is not proof that its normals are shared correctly; the one-sided normal assertions check that directly.

**Before continuing:** explain why identical position coordinates do not imply identical normals. Continue to [text layout](10-text-layout.md).
