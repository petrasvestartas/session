# 23d · Annotate and measure

Text writes a label into the model, Arrowhead puts heads on curves, and Project To Plane flattens a selection. Measure Distance, Length, Area and Volume answer with a number and leave a mark in the scene until the next command.

## Step 1 · registration lines

One line per command in `verbs!`, the `measure` helper module, and the mark kept in `Features`.

`lessons/23d/src/app/command/verbs/mod.rs` · type the line tagged `register:measure`

```rust
--8<-- "lessons/23d/src/app/command/verbs/mod.rs:verbs-modules"
```

`lessons/23d/src/app/command/verbs/mod.rs` · type the lines tagged `register:arrowhead`, `register:text`, `register:project_to_plane`, `register:measure_distance`, `register:length`, `register:area` and `register:volume`

```rust
--8<-- "lessons/23d/src/app/command/verbs/mod.rs:verbs-list"
```

`lessons/23d/src/state/features.rs` · type the two lines tagged `register:measure`

```rust
--8<-- "lessons/23d/src/state/features.rs:features-struct"
```

Copy the other lines tagged `register:annotate` from these files of `lessons/23d/`:

- `src/state/edit.rs`: a new command clears the mark.
- `src/state.rs`: Esc clears the mark.
- `src/app/ui/overlay.rs`: the mark is drawn when no tool draws its own marks.
- `src/app/inspection.rs`: the mark in the inspection snapshot.

## Step 2 · src/app/command/verbs/measure.rs

The mark a measurement leaves, the selected objects with their placement, and fetching released documents back first.

`lessons/23d/src/app/command/verbs/measure.rs` · type this, new file

```rust
--8<-- "lessons/23d/src/app/command/verbs/measure.rs:mark-target"
```

## Step 3 · src/app/command/verbs/measure.rs

The area of a mesh face exactly as drawn, and the signed volume its triangles sweep.

`lessons/23d/src/app/command/verbs/measure.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23d/src/app/command/verbs/measure.rs:face-area"
```

## Step 4 · src/app/command/verbs/measure.rs

Short numbers such as `141.421`, the unit with its power, and plural words for the answers.

`lessons/23d/src/app/command/verbs/measure.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23d/src/app/command/verbs/measure.rs:value-text"
```

## Step 5 · src/app/command/verbs/measure.rs

Set the mark, draw it as a black line with a value chip, and report it to the tests.

`lessons/23d/src/app/command/verbs/measure.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23d/src/app/command/verbs/measure.rs:mark-state"
```

## Step 6 · src/app/command/verbs/measure.rs

Tests: short values, an exact concave face, a warped face as drawn, and a box's swept volume.

`lessons/23d/src/app/command/verbs/measure.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/23d/src/app/command/verbs/measure.rs:measure-tests"
```

## Step 7 · src/app/command/verbs/measure_distance.rs

Measure Distance answers at once for two typed world points, and otherwise asks for them.

`lessons/23d/src/app/command/verbs/measure_distance.rs` · type this, new file

```rust
--8<-- "lessons/23d/src/app/command/verbs/measure_distance.rs:distance-parse"
```

## Step 8 · src/app/command/verbs/measure_distance.rs

Two actions: measure typed points now, or open the picking tool and feed it any typed ones.

`lessons/23d/src/app/command/verbs/measure_distance.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23d/src/app/command/verbs/measure_distance.rs:distance-actions"
```

## Step 9 · src/app/command/verbs/measure_distance.rs

The picking tool: two prompts, a live distance beside the cursor, and the answer after the second point.

`lessons/23d/src/app/command/verbs/measure_distance.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23d/src/app/command/verbs/measure_distance.rs:measuring-tool"
```

## Step 10 · src/app/command/verbs/measure_distance.rs

The answer with its x, y and z steps, marked between the two points.

`lessons/23d/src/app/command/verbs/measure_distance.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23d/src/app/command/verbs/measure_distance.rs:distance-answer"
```

## Step 11 · src/app/command/verbs/measure_distance.rs

Tests: typed points answer, fewer are picked, and the four analysis commands complete in one Enter.

`lessons/23d/src/app/command/verbs/measure_distance.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/23d/src/app/command/verbs/measure_distance.rs:distance-tests"
```

## Step 12 · src/app/command/verbs/length.rs

Length sums the world length of the selected lines, polylines and NURBS curves.

`lessons/23d/src/app/command/verbs/length.rs` · type this, new file

```rust
--8<-- "lessons/23d/src/app/command/verbs/length.rs:length"
```

## Step 13 · src/app/command/verbs/length.rs

Test: each curve kind measures in world units, and a mesh has no length.

`lessons/23d/src/app/command/verbs/length.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/23d/src/app/command/verbs/length.rs:length-tests"
```

## Step 14 · src/app/command/verbs/text.rs

Text takes up to 80 characters and starts at a tidy height a 25th of the view distance.

`lessons/23d/src/app/command/verbs/text.rs` · type this, new file

```rust
--8<-- "lessons/23d/src/app/command/verbs/text.rs:text-parse"
```

## Step 15 · src/app/command/verbs/text.rs

The placing tool: `Height N` changes the letter height, and a click puts the lower-left corner.

`lessons/23d/src/app/command/verbs/text.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23d/src/app/command/verbs/text.rs:placing"
```

## Step 16 · src/app/command/verbs/text.rs

The label standing on the point, the plane axes that read from the camera, and the tidy height.

`lessons/23d/src/app/command/verbs/text.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23d/src/app/command/verbs/text.rs:text-label"
```

## Step 17 · src/app/command/verbs/text.rs

Tests: the words become the text, every standard view reads upright, the line box stands on the point.

`lessons/23d/src/app/command/verbs/text.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/23d/src/app/command/verbs/text.rs:text-tests"
```

## Step 18 · src/app/command/verbs/arrowhead.rs

Arrowhead takes one word, None, Start, End or Both, in any case.

`lessons/23d/src/app/command/verbs/arrowhead.rs` · type this, new file

```rust
--8<-- "lessons/23d/src/app/command/verbs/arrowhead.rs:arrowhead-parse"
```

## Step 19 · src/app/command/verbs/arrowhead.rs

Set the heads on every selected curve that differs, as one undo step.

`lessons/23d/src/app/command/verbs/arrowhead.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23d/src/app/command/verbs/arrowhead.rs:heads"
```

## Step 20 · src/app/command/verbs/arrowhead.rs

A copy of the line, polyline or curve with new heads, and whether they changed.

`lessons/23d/src/app/command/verbs/arrowhead.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23d/src/app/command/verbs/arrowhead.rs:headed"
```

## Step 21 · src/app/command/verbs/arrowhead.rs

Tests: the options parse, only curves take heads, and Undo and Redo take them off and on.

`lessons/23d/src/app/command/verbs/arrowhead.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/23d/src/app/command/verbs/arrowhead.rs:arrowhead-tests"
```

## Step 22 · the other commands

They use the helpers of `measure.rs` and the same undo step; copy these files from `lessons/23d/src/app/command/verbs/`:

- `project_to_plane.rs`: flatten the selection onto the view's plane, XY, YZ, ZX or three picked points.
- `area.rs`: the area of meshes, BReps, NURBS surfaces and elements, or of one selected face.
- `volume.rs`: the volume of closed meshes and solid BReps; open ones are named and refused.

Run `cargo check` in `lessons/23d/`.

## Check

`cargo check` compiles, and `cargo xtest --lib verbs::measure::` passes. `Measure Distance 0,0,0 100,100,0` answers Distance 141.421 in the scene unit, with dx 100, dy 100 and dz 0, and draws a black line with the value until the next command or Esc.
