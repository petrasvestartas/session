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

For all features together, use the [seven sequential checkpoints](extend-integrated-tutorial.md). Their shared wiring is resolved explicitly and their final source is checked against the maintained viewer.

## Start one lesson

Run from the maintained repository:

```sh
export COURSE_REPO="$PWD"
bash "$COURSE_REPO/docs/serve.sh" build --quiet
python3 docs/extensions.py --prepare "$HOME/viewer-modeling"
cd "$HOME/viewer-modeling/session_viewer"
export REGEN_PROTO=0
cargo check -j4 --lib
```

The initializer copies checkpoint 21 and its kernel into a **new** folder. It does not apply an extension. Follow the lesson's visible code blocks to implement it yourself. Use a separate new folder for another lesson; all patches start from checkpoint 21.

## What is verified

`docs/extensions.py --verify --write` applies each lesson independently in a temporary workspace, proves that its visible snippets produce exactly the same source as its patch, compiles every complete step for WebAssembly, and runs the final native library tests. The measured step status appears beside each check; [verification.json](extensions/verification.json) records the patch hashes.

The [five-round browser record](extensions/README.md) covers commands, all gumball gestures, control edits, nested/graph visibility and resource cleanup.

The pictures in the lessons are captures of the running viewer using the supplied [fixture](extensions/nested.pb), not rendered design mockups. See the caption and [capture record](extensions/screenshots.json) for the browser configuration and operations.

## What these lessons do—and their limits

- Creation: typed points, lines and polylines in world coordinates.
- Trim/extend: normalized parameter intervals on lines and NURBS curves; no cutting-object intersections.
- Explode: polylines into individual lines; no BRep or mesh explosion.
- Panel: nested group and graph-attribute selection/hiding; no reparenting or group transforms.
- Controls: polyline and NURBS control dragging; no mesh vertex or surface-control writes.

Commands are bounded; panel indexes use bounded row vectors and do not own geometry. Undo retains source snapshots, so these limits are not a whole-browser memory guarantee. Geometry commits refuse streamed scenes that the existing rebuild path cannot preserve.

For the solid mesh gumball, follow [its code lesson](extend-gumball-tutorial.md). [Lesson 21](21-editing.md) remains the frozen starting point. The older gumball, command-line and panel design pages are background reading, not the implementation instructions.
