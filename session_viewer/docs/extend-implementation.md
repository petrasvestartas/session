# Implement the editing extensions

These are **code lessons**, starting from checkpoint 21. Choose one. Each shows every new file and every exact **CURRENT → REPLACE WITH** or **ADD BELOW** edit. No step asks you to find and copy an unspecified method.

| Lesson | What you write | Steps |
|---|---|---|
| [Create, trim, extend and explode](extend-modeling-tutorial.md) | Geometry transactions, input parser and command dispatch | 2 complete compile checkpoints |
| [Nested session and graph panel](extend-panels-tutorial.md) | Tree/graph index, selection and visibility actions, DOM wiring | 3 complete compile checkpoints |
| [Solid gumball](extend-gumball-tutorial.md) | Cylinders, cone tips, flat colors, antialiasing and resource lifetime | 1 complete compile checkpoint |
| [Placed control dragging](extend-controls-tutorial.md) | World/local conversion, pointer preview, commit and cancellation | 3 complete compile checkpoints |
| [egui interface](extend-ui-tutorial.md) | Input routing, layers window, command field and final GPU pass | 1 complete compile checkpoint |

For using the maintained viewer, follow the [command-line walkthrough with screenshots](command-line-walkthrough.md). Its interface uses egui, like the archive. The independent modeling and panel lessons retain checkpoint 21's DOM interface; the optional egui lesson replaces it.

For all features together, use the [ten sequential checkpoints](extend-integrated-tutorial.md). Their shared wiring is resolved explicitly and their final source is checked against the maintained viewer.

## Start one lesson

Run from the maintained repository:

```sh
cp -r docs/lessons/21 docs/lessons/my-modeling
cd docs/lessons/my-modeling
cargo check -j4 --lib
```

The copy is checkpoint 21 with no extension applied. Follow the lesson's visible code blocks to implement it yourself; the finished crates (`docs/lessons/modeling-1/`, `panels-1/` … `ui-1/`) are there to diff against. Use a separate copy for another lesson; every extension starts from checkpoint 21.

## What is verified

Every step of every extension lesson is a complete crate under `docs/lessons/<chain>-<n>/`; `cargo check` inside it proves the step compiles, and `cargo xtest` in the last step of a chain runs its native library tests.

The pictures in the lessons are captures of the running viewer using the supplied [fixture](extensions/nested.pb), not rendered design mockups; each caption names the browser configuration and operations.

## What these lessons do—and their limits

- Creation: typed points, lines and polylines in world coordinates.
- Trim/extend: normalized parameter intervals on lines and NURBS curves; no cutting-object intersections.
- Explode: polylines into individual lines; no BRep or mesh explosion.
- Panel: nested group and graph-attribute selection/hiding; no reparenting or group transforms.
- Independent controls lesson: polyline and NURBS control dragging. [Checkpoint 8](current-8.md) adds mesh vertices/edges/faces, NURBS surface controls/boundaries and compatible BRep edits, plus docked UI, touch interaction and Save/Open.

Commands are bounded; panel indexes use bounded row vectors and do not own geometry. Undo retains source snapshots, so these limits are not a whole-browser memory guarantee. Geometry commits refuse streamed scenes that the existing rebuild path cannot preserve.

For the solid mesh gumball, follow [its code lesson](extend-gumball-tutorial.md). [Lesson 21](21-editing.md) remains the frozen starting point. The older gumball, command-line and panel design pages are background reading, not the implementation instructions.

## Expected viewer result

The completed viewer has a command dock across the bottom, one right-hand layer tree with bulbs, selection locks and color swatches, and a left toolbar. Split keeps both face regions in the joined shell. The selected region has its gumball; Save/Open retains the edited geometry and layer settings. See the [phone layout](screenshots/extensions-workspace-current-phone.png).

[![Full viewer result for extend implementation](screenshots/extensions-workspace-current-desktop.png)](screenshots/extensions-workspace-current-desktop.png)
