# 08 · Trims, holes and periodic seams

The kernel meshes a trimmed face only inside its trim loops, so a hole stays empty and the walk only has to find each edge on the mesh. Without labels an edge is found along a line of constant u or v, wrapping across a closed surface's seam.

![Left: outer and inner loops select the face in u,v and the hole stays empty. Right: a cylinder's seam is one XYZ curve used at u=0 and u=1.](illustrations/trims-seams.svg)

## Step 1 · src/app/walk/brep_edges.rs

One face's use of an edge, its straight pcurve, and the (u, v) samples the mesher tagged.

`lessons/08/src/app/walk/brep_edges.rs` · read, copied in 06

```rust
--8<-- "lessons/08/src/app/walk/brep_edges.rs:edge-use"
```

## Step 2 · src/app/walk/brep_edges.rs

The vertices along a grid edge in parameter order, wrapping at a seam and closing a loop edge.

`lessons/08/src/app/walk/brep_edges.rs` · read, copied in 06

```rust
--8<-- "lessons/08/src/app/walk/brep_edges.rs:iso-chain"
```

Run `cargo check` in `lessons/08/`.

## Check

Run `cargo xtest --lib brep_edges` in `lessons/08/`: the sphere's seam reaches both poles, the torus seams close, and trim edges follow the face samples.
