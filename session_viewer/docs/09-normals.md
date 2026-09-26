# 09 · Normals and shading

Every face of a solid must point outward, and every edge pipe must know the normals on both sides, or back edges show and shading flips. This lesson reads the code lesson 06 copied for that.

![Analytic normal or finite fallback at a pole; two shading normals at a C0 crease; the cofactor transform keeps a normal perpendicular under nonuniform scale.](illustrations/normals.svg)

## Step 1 · src/app/walk/brep_orient.rs

Which way a face walks from one vertex to the next, read from its halfedges.

`lessons/09/src/app/walk/brep_orient.rs` · read, copied in 06

```rust
--8<-- "lessons/09/src/app/walk/brep_orient.rs:halfedges"
```

## Step 2 · src/app/walk/brep_orient.rs

Find the same vertex, and the same next vertex, on the neighbouring face's mesh.

`lessons/09/src/app/walk/brep_orient.rs` · read, copied in 06

```rust
--8<-- "lessons/09/src/app/walk/brep_orient.rs:vertex-search"
```

## Step 3 · src/app/walk/brep_orient.rs

Two faces agree when they walk their shared edge in opposite directions; each face's signed volume.

`lessons/09/src/app/walk/brep_orient.rs` · read, copied in 06

```rust
--8<-- "lessons/09/src/app/walk/brep_orient.rs:face-agreement"
```

## Step 4 · src/app/walk/brep_orient.rs

Give every face +1 or -1 so neighbours agree and the whole solid encloses a positive volume.

`lessons/09/src/app/walk/brep_orient.rs` · read, copied in 06

```rust
--8<-- "lessons/09/src/app/walk/brep_orient.rs:face-signs"
```

## Step 5 · src/app/walk/brep_edges.rs

Index every triangle of a face mesh by its edge's end positions, keeping its normal.

`lessons/09/src/app/walk/brep_edges.rs` · read, copied in 06

```rust
--8<-- "lessons/09/src/app/walk/brep_edges.rs:facets"
```

## Step 6 · src/app/walk/brep_edges.rs

A pipe's facing word: the triangle normal on each side, turned outward by its face's sign.

`lessons/09/src/app/walk/brep_edges.rs` · read, copied in 06

```rust
--8<-- "lessons/09/src/app/walk/brep_edges.rs:edge-pen"
```

Run `cargo check` in `lessons/09/`.

## Check

Run `cargo xtest --lib brep_orient` in `lessons/09/`: kernel solids keep their normals, and a cylinder with two flipped faces draws the same pipes.
