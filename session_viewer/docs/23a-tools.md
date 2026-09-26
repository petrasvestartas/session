# 23a · Tools that ask for points

Some commands ask before they act: Move wants two points, Trim wants targets, cutters and the parts to remove. Each is a Tool from lesson 23; this lesson writes the transforms, trim and extend with cutters, the selection modes, and the gathering tool that lesson 23c builds its surfaces with.

## Step 1 · src/app/command/tool/cut.rs

New file: a cutter is a curve or a plane; helpers turn objects into cutters and find the part under a click.

`lessons/23a/src/app/command/tool/cut.rs` · type this, new file

```rust
--8<-- "lessons/23a/src/app/command/tool/cut.rs:cutter"
```

## Step 2 · src/app/command/tool/cut.rs

Tests: flat objects give planes, a fence stands on its line, and a scaled mesh is still hit.

`lessons/23a/src/app/command/tool/cut.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/23a/src/app/command/tool/cut.rs:cutter-tests"
```

## Step 3 · src/app/command/verbs/selecting.rs

New file: the rows a select verb may take, and how a result replaces, adds to or removes from the selection.

`lessons/23a/src/app/command/verbs/selecting.rs` · type this, new file

```rust
--8<-- "lessons/23a/src/app/command/verbs/selecting.rs"
```

## Step 4 · src/app/command/verbs/move.rs

Move: a typed offset acts at once; a bare Move starts the Moving tool.

`lessons/23a/src/app/command/verbs/move.rs` · type this, new file

```rust
--8<-- "lessons/23a/src/app/command/verbs/move.rs:move-spec"
```

## Step 5 · src/app/command/verbs/move.rs

Moving implements Tool: its prompts, the preview that follows the cursor, the distance readout and the move itself.

`lessons/23a/src/app/command/verbs/move.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23a/src/app/command/verbs/move.rs:move-tool"
```

## Step 6 · src/app/command/verbs/move.rs

Tests: the selection follows the cursor from the base point.

`lessons/23a/src/app/command/verbs/move.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/23a/src/app/command/verbs/move.rs:move-tests"
```

## Step 7 · src/app/command/verbs/rotate.rs

Rotate: an axis and an angle at once, or a centre, then a typed angle or two reference points.

`lessons/23a/src/app/command/verbs/rotate.rs` · copy the file

```rust
--8<-- "lessons/23a/src/app/command/verbs/rotate.rs"
```

## Step 8 · src/app/command/verbs/scale.rs

Scale: a factor at once, or an origin, then a typed factor or two reference points, in 1D, 2D or 3D.

`lessons/23a/src/app/command/verbs/scale.rs` · copy the file

```rust
--8<-- "lessons/23a/src/app/command/verbs/scale.rs"
```

## Step 9 · src/app/command/verbs/copy.rs

Copy: a base point, then a copy at every target point until Enter, each one undo step.

`lessons/23a/src/app/command/verbs/copy.rs` · copy the file

```rust
--8<-- "lessons/23a/src/app/command/verbs/copy.rs"
```

## Step 10 · src/app/command/verbs/orient_3_points.rs

Orient 3 Points: three reference points onto three target points, moving the selection rigidly.

`lessons/23a/src/app/command/verbs/orient_3_points.rs` · copy the file

```rust
--8<-- "lessons/23a/src/app/command/verbs/orient_3_points.rs"
```

## Step 11 · src/app/command/verbs/trim.rs

Trim: a bare Trim runs the tool; two numbers keep that part of a curve.

`lessons/23a/src/app/command/verbs/trim.rs` · type this, new file

```rust
--8<-- "lessons/23a/src/app/command/verbs/trim.rs:trim-spec"
```

## Step 12 · src/app/command/verbs/trim.rs

The three phases, one target cut into parts, the running trim, and what Trim can cut.

`lessons/23a/src/app/command/verbs/trim.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23a/src/app/command/verbs/trim.rs:trim-state"
```

## Step 13 · src/app/command/verbs/trim.rs

Open `impl Trimming`: toggle targets and cutters, and collect every cutter in the world.

`lessons/23a/src/app/command/verbs/trim.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23a/src/app/command/verbs/trim.rs:trim-pick"
```

## Step 14 · src/app/command/verbs/trim.rs

Cut every target into parts in its own frame, and find the part under a click or the cursor.

`lessons/23a/src/app/command/verbs/trim.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23a/src/app/command/verbs/trim.rs:trim-preview"
```

## Step 15 · src/app/command/verbs/trim.rs

Show what is left, restore on cancel, write every trimmed target as one undo step, and close the impl.

`lessons/23a/src/app/command/verbs/trim.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23a/src/app/command/verbs/trim.rs:trim-commit"
```

## Step 16 · src/app/command/verbs/trim.rs

Trimming implements Tool: prompts and buttons per phase, picks, Enter, clicks that remove parts, and the overlay.

`lessons/23a/src/app/command/verbs/trim.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23a/src/app/command/verbs/trim.rs:trim-tool"
```

## Step 17 · src/app/command/verbs/trim.rs

Tests: bare Trim runs the tool, two numbers keep the parametric trim.

`lessons/23a/src/app/command/verbs/trim.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/23a/src/app/command/verbs/trim.rs:trim-tests"
```

## Step 18 · src/app/command/verbs/trim/parts.rs

The cutting itself: curves at their crossings, surfaces and BReps into faces, meshes into two sides, and the commit.

`lessons/23a/src/app/command/verbs/trim/parts.rs` · copy the file

```rust
--8<-- "lessons/23a/src/app/command/verbs/trim/parts.rs"
```

## Step 19 · src/app/command/verbs/extend.rs

Extend: pick boundaries, then click near curve ends; or grow ends by a typed length.

`lessons/23a/src/app/command/verbs/extend.rs` · copy the file

```rust
--8<-- "lessons/23a/src/app/command/verbs/extend.rs"
```

## Step 20 · src/app/command/verbs/extend/reach.rs

Where an end goes: straight along its tangent, or along the curve's shape to a boundary.

`lessons/23a/src/app/command/verbs/extend/reach.rs` · copy the file

```rust
--8<-- "lessons/23a/src/app/command/verbs/extend/reach.rs"
```

## Step 21 · src/app/command/verbs/select_lasso.rs

Select Lasso: arm a tool that takes the next left drag.

`lessons/23a/src/app/command/verbs/select_lasso.rs` · type this, new file

```rust
--8<-- "lessons/23a/src/app/command/verbs/select_lasso.rs:lasso-spec"
```

## Step 22 · src/app/command/verbs/select_lasso.rs

The loop follows the drag, thins itself when long, and selects what lies inside on release.

`lessons/23a/src/app/command/verbs/select_lasso.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23a/src/app/command/verbs/select_lasso.rs:lasso-tool"
```

## Step 23 · src/app/command/verbs/select_lasso.rs

The loop indexed by pixel row for the even-odd test, and a point projected to the screen.

`lessons/23a/src/app/command/verbs/select_lasso.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23a/src/app/command/verbs/select_lasso.rs:lasso-region"
```

## Step 24 · src/app/command/verbs/select_lasso.rs

The rows whose every sample lies inside: curves, surfaces, meshes, BReps and clouds, else their box corners.

`lessons/23a/src/app/command/verbs/select_lasso.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23a/src/app/command/verbs/select_lasso.rs:lasso-inside"
```

## Step 25 · src/app/command/verbs/select_lasso.rs

Tests: the loop, the row index against ray casting, samples of each geometry, and the recorded points.

`lessons/23a/src/app/command/verbs/select_lasso.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/23a/src/app/command/verbs/select_lasso.rs:lasso-tests"
```

## Step 26 · src/app/command/verbs/select_by_name.rs

Select By Name: visible objects whose name contains the text, in any case.

`lessons/23a/src/app/command/verbs/select_by_name.rs` · copy the file

```rust
--8<-- "lessons/23a/src/app/command/verbs/select_by_name.rs"
```

## Step 27 · src/app/command/verbs/select_small.rs

Select Small: visible objects whose box diagonal is shorter than a length.

`lessons/23a/src/app/command/verbs/select_small.rs` · copy the file

```rust
--8<-- "lessons/23a/src/app/command/verbs/select_small.rs"
```

## Step 28 · src/app/command/verbs/object.rs

Object: from now on a click picks whole objects; the first selection-mode verb.

`lessons/23a/src/app/command/verbs/object.rs` · type this, new file

```rust
--8<-- "lessons/23a/src/app/command/verbs/object.rs"
```

## Step 29 · src/app/command/verbs/edge.rs

Edge: a click picks edges.

`lessons/23a/src/app/command/verbs/edge.rs` · copy the file

```rust
--8<-- "lessons/23a/src/app/command/verbs/edge.rs"
```

## Step 30 · src/app/command/verbs/face.rs

Face: a click picks faces.

`lessons/23a/src/app/command/verbs/face.rs` · copy the file

```rust
--8<-- "lessons/23a/src/app/command/verbs/face.rs"
```

## Step 31 · src/app/command/verbs/controls.rs

Controls: a click picks control points.

`lessons/23a/src/app/command/verbs/controls.rs` · copy the file

```rust
--8<-- "lessons/23a/src/app/command/verbs/controls.rs"
```

## Step 32 · src/app/command/tool/gather.rs

New file: the questions a gathering command asks, the answers, and what it gathered.

`lessons/23a/src/app/command/tool/gather.rs` · type this, new file

```rust
--8<-- "lessons/23a/src/app/command/tool/gather.rs:gather-steps"
```

## Step 33 · src/app/command/tool/gather.rs

A Recipe describes a command as data; typed words pick its options and answer its steps.

`lessons/23a/src/app/command/tool/gather.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23a/src/app/command/tool/gather.rs:gather-recipe"
```

## Step 34 · src/app/command/tool/gather.rs

Progress: the step waiting, the curves picked per step, and what Enter does.

`lessons/23a/src/app/command/tool/gather.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23a/src/app/command/tool/gather.rs:gather-progress"
```

## Step 35 · src/app/command/tool/gather.rs

Gather: the action that starts the tool with the typed answers.

`lessons/23a/src/app/command/tool/gather.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23a/src/app/command/tool/gather.rs:gather-start"
```

## Step 36 · src/app/command/tool/gather.rs

Gathering: the running tool's state, building once every step is answered, and the axis a Distance slides on.

`lessons/23a/src/app/command/tool/gather.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23a/src/app/command/tool/gather.rs:gathering"
```

## Step 37 · src/app/command/tool/gather.rs

Gathering implements Tool: prompts, option words, number boxes, curve picks, points and the sliding Distance.

`lessons/23a/src/app/command/tool/gather.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23a/src/app/command/tool/gather.rs:gathering-tool"
```

## Step 38 · src/app/command/tool/gather.rs

Tests: pick order, Enter below the minimum, preselection, answers in step order and defaults.

`lessons/23a/src/app/command/tool/gather.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/23a/src/app/command/tool/gather.rs:gather-tests"
```

## Step 39 · src/app/command/tool/gather.rs

A picked line, polyline or curve moved into the world, and a count in words.

`lessons/23a/src/app/command/tool/gather.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23a/src/app/command/tool/gather.rs:gather-picked"
```

## Step 40 · src/app/ui/overlay.rs

Draw the running tool's strokes, squares and label over the scene.

`lessons/23a/src/app/ui/overlay.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/23a/src/app/ui/overlay.rs:tool-marks"
```

## Step 41 · registration lines

Copy the lines tagged with a lesson 23a tag from these files of `lessons/23a/`:

- `src/app/command/verbs/mod.rs`: the `selecting` module, and `trim`, `extend`, `move`, `rotate`, `scale`, `copy`, `orient_3_points`, `object`, `edge`, `face`, `controls`, `select_lasso`, `select_by_name` and `select_small` in the `verbs!` list.
- `src/app/command/tool.rs`: the `cut` and `gather` modules.
- `src/app/input.rs`: a left press, drag and release go to the running tool first (`register:tools`).
- `src/app/keys.rs`: Enter runs the command being drawn.
- `src/state.rs`, `src/state/edit.rs`, `src/state/features.rs`: a tool's pick, cancel and hidden gumball (`register:tools`).
- `src/app/ui/mod.rs`, `src/app/inspection.rs`: the tool's marks drawn each frame, and its `tool` snapshot.

## Step 42 · tests

Copy `tests/transforms.cjs` and `tests/trim-extend.cjs` from `lessons/23a/`: browser checks of the transform tools and of trim and extend with cutters.

Run `cargo check` in `lessons/23a/`.

## Check

`cargo check` compiles. In `trunk serve`, select an object and type `Move`: pick a base point and the object follows the cursor until the second click. Draw two crossing lines, type `Trim`, pick one, Enter, pick the other, Enter: the part under the cursor turns red, and a click removes it as one undo step. `Select Lasso` selects what a dragged loop surrounds.
