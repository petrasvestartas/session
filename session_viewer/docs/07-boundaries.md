# 07 · Shared boundaries

Two faces that meet must show one edge: the kernel meshes both on the same edge samples, and the walk draws the edge once from those vertices. This lesson reads the chain code lesson 06 copied.

![Before: face A, face B and the ink each chord the same edge differently. After: one canonical chain constrains both meshes and the ink is drawn from those nodes.](illustrations/shared-boundary.svg)

## Step 1 · src/app/walk/brep_edges.rs

The imports, and the sort orders both chain builders use.

`lessons/07/src/app/walk/brep_edges.rs` · read, copied in 06

```rust
--8<-- "lessons/07/src/app/walk/brep_edges.rs:edge-order"
```

## Step 2 · src/app/walk/brep_edges.rs

Read an edge's vertices back, in order, from the labels the kernel mesher left on them.

`lessons/07/src/app/walk/brep_edges.rs` · read, copied in 06

```rust
--8<-- "lessons/07/src/app/walk/brep_edges.rs:constrained-chain"
```

## Step 3 · src/app/walk/brep_edges.rs

One chain per BRep edge, from the first face that carries it, with the face on its other side.

`lessons/07/src/app/walk/brep_edges.rs` · read, copied in 06

```rust
--8<-- "lessons/07/src/app/walk/brep_edges.rs:edge-chains"
```

## Step 4 · src/app/walk/brep_edges.rs

A chain becomes pipes, one per segment, each tagged with its BRep edge for picking.

`lessons/07/src/app/walk/brep_edges.rs` · read, copied in 06

```rust
--8<-- "lessons/07/src/app/walk/brep_edges.rs:edge-pipes"
```

Run `cargo check` in `lessons/07/`.

## Check

Run `cargo xtest --lib brep_edges` in `lessons/07/`: every solid edge has a chain, and remeshing keeps each edge's identity.
