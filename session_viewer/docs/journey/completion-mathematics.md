# Lay out and animate mathematics

Align equations, display matrices and replay a mathematical derivation using the retained production stage implementation.

This cumulative subject follows [Paint shapes and text](completion-painting.md). Its implementation and lesson are reviewed against the recorded Windows Intel WebGPU checks at desktop and upright sizes. The remaining course has separate acceptance.

## Prerequisites and ownership

Keep the device, launcher, frame helper and GPU painter from the first two subjects. This subject owns fonts and metrics, TeX layout, the glyph atlas, column and matrix layout, object state, time, action playback, matching transforms and deck parsing. It introduces `Presentation` and its host-independent mode implementation in `src/app/stage`.

The kernel remains a path dependency. A chapter has one complete schema for formulas, named points, cameras and nets. Parsing that schema requires `geometry`, `view` and `net` even when this chapter uses only formulas. Retain those pure modules now: they calculate values and descriptions without owning a viewer state or GPU rows. The later [scene chapter](stage-scene.md) teaches their camera, geometry and force-density use and installs the adapter; it does not replace this parser or runtime.

### One schema, two hosts

Rust checks every referenced type and method in a compiled module, including branches the example never executes. `deck.rs` expands a net declaration into points, geometry and matrix actions; `run.rs` stores the authored camera and spatial descriptions. Deleting those dependencies would require a second parser or conditional versions of the runtime. The checkpoint instead keeps the complete mathematical model and supplies a host that consumes its 2D painting output. This is an explicit early dependency, not an early installation of the production scene renderer.

`stage/mod.rs` is the module registration boundary. The mathematics assembly declares the core modules and their native tests. The application assembly adds `spatial`, scene `mode` and `application`, then re-exports the loader and `StageApp`. Module declarations make names available; they contain no playback implementation. The assembler checks that earlier declarations survive and rejects function bodies or changed declarations masquerading as registrations. The separate example entry list adds names mapped to start functions, so a later checkpoint can still run the earlier examples.

The host boundary is `Mode<Host>`. `Presentation` implements it for any host without reading that host. Later, `StageApp` implements it for the real viewer state and delegates mathematical navigation to the same presentation. This existing trait and the module/entry declarations are the required registration mechanisms; no new hook is needed inside the completed clock, parser or painter. Predict the result of removing the scene adapter from the early assembly: formulas still paint, but no GPU geometry rows can appear.

## Build

Add the complete stage modules and the mathematics registration, retaining the earlier implementations.

From `session_tests`, assemble a fresh comparison project:

```sh
node --import ./scripts/typescript.mjs scripts/course.ts completion-checkpoint mathematics ../session_viewer/target/learn-mathematics
```

The source-included sections below explain the same stage implementation used by the candidate. The retained source comparison test checks the actual production files. The fixture fills the entire viewport with the stage background before painting. The production painter leaves the scene side transparent, so this host supplies the ground behind the heading too.

The browser constructor opens the first authored step with `chapter_go(0, now)` and embeds four math fonts and the example chapter, so this experiment has no network font race; fetching belongs to the later loading subject.

```rust
--8<-- "session_viewer/examples/course/mathematics.rs:course-mathematics"
```

### Frame and letterbox

Type the coordinate mapping in [paint.rs](code/03a-mathematics-cumulative-18-paint.rs); keep authored positions independent of the browser's dimensions.

```rust
--8<-- "journey/code/03a-mathematics-cumulative-18-paint.rs:frame"
```

The design space is 1600 by 900. An 800 by 800 viewport fits it at half scale, leaving 175 points above and below. `pt` maps authored coordinates into screen points; `slide` reverses that mapping for input. Upright composition gives the scene and column separate mappings. It does not change the formula's authored coordinates. At the top left of the slide, `paint.rs` draws the current slide title when present and falls back to the chapter title when the slide title is empty.

### Font metrics and TeX

Copy [font.rs](code/03a-mathematics-cumulative-8-font.rs) and [tex.rs](code/03a-mathematics-cumulative-23-tex.rs). Read `Fonts::new`: dividing font units by units-per-em gives layout units independent of raster size.

```rust
--8<-- "journey/code/03a-mathematics-cumulative-8-font.rs:metrics"
```

Ink bounds and horizontal advance describe different things. Advancing by ink width would collapse deliberate spaces around glyphs. The font's OpenType MATH table supplies fraction gaps, script offsets and delimiter construction. A tall delimiter can use a larger glyph or an assembly of extender pieces.

Trace the lexer before following the layout: commands, groups, scripts and raw text must be distinct tokens before dimensions are measured.

```rust
--8<-- "journey/code/03a-mathematics-cumulative-23-tex.rs:lexer"
```

A fraction has separate numerator and denominator boxes. Layout positions them around the math axis and emits a rule between them. A square root needs a radical and an overbar; it is not a Unicode character pasted before a string.

```rust
--8<-- "journey/code/03a-mathematics-cumulative-23-tex.rs:typeset"
```

`Expr` carries positioned primitives, width, height, depth, an optional equals-sign offset and named parts. It also records unknown commands. Prose permits `$...$` mathematics; code uses the monospace face. These content forms share the painter rather than creating another rendering lane.

### Glyph atlas and painting

Type the atlas implementation; cache a raster by font, glyph ID and pixel size.

```rust
--8<-- "journey/code/03a-mathematics-cumulative-18-paint.rs:atlas"
```

Swash converts a glyph ID into coverage pixels. A cached space consumes no texture area. Bearings locate the raster relative to its baseline; one-pixel gutters separate atlas entries. Layout remains in em units, the object's size converts em to slide units, and the frame plus display density determines raster pixels.

Read `prim`, `Out::flush` and `paint` in the same file. Glyphs accumulate into one egui mesh until another shape must preserve drawing order. Rounded matrix cells, paths and pictures use that same ordered output. The atlas and painter are the full stage implementation, including raster-size quantization and atlas reset handling.

The viewer's scene text lane cannot draw these font glyph IDs. Mathematics therefore uses the font's MATH layout and egui glyph quads. The later 3D picture still uses the real scene and camera.

### Keep the presentation together

The mathematics constructor owns one production `Presentation`. The fixture forwards keys to it, ticks its stage clock, and paints through the retained egui renderer. No viewer scene is required. The generic `Mode<Host>` implementation is also retained for the later production application.

The production [presentation owner](../../src/app/stage/presentation.rs) keeps the running deck, its glyph and picture atlas, and the last painted coordinate mapping together:

```rust
--8<-- "journey/code/03a-mathematics-cumulative-19-presentation.rs:stage-presentation"
```

Its constructor assigns each named picture an atlas index and an aspect ratio in the same pass. Its `paint` method stores the frame used for drawing, clears the previous overflow flag, paints, and returns whether animation, changed state or a newly overflowing atlas requires another frame. It clears `changed` only after calculating that result. The outer application owns the clock tick, camera projection, event routing and GPU submission.

Keeping this owner independent of scene rows lets the later `StageApp` contain the same presentation alongside `Scene3` and pointer state. It does not replace the parser, runtime or painter. Predict what would happen if `changed` were cleared before computing the return value: a static invalidation would no longer request another frame. The presentation regression tests check this transition, picture indices and desktop/upright coordinate round trips.

The [regression fixture](../../src/app/stage/presentation_tests.rs) creates a one-step formula deck and two pictures. It checks that picture IDs still address the correct aspect ratios and pixels. Painting twice must retain glyph output while the repaint request settles from true to false; mapping a slide point to screen coordinates and back must recover it in both viewport shapes. These are CPU egui checks, not GPU or device acceptance.

The supplied presentation tests exercise the same owner. Preserve the standalone checkpoint and its receipts as historical evidence; this cumulative checkpoint needs its own native and browser acceptance.

### Register content without owning the application

A mode receives input and produces egui output; it does not acquire another surface or create another GPU device. Read [the application contract](../../src/app/mode.rs). `Host` is a Rust type parameter: the early mathematics mode works with an empty host, while the later scene mode receives the viewer state. The mathematics implementation never accesses the host, so adding a camera later does not require changing its painter.

```rust
--8<-- "journey/code/03a-mathematics-cumulative-11-math_mode.rs:math-mode"
```

Read `Loaded::open`: loading produces a mode only after its inputs are ready, with an explicit starting time. The host is borrowed mutably for work that needs application resources.

```rust
--8<-- "journey/code/03a-mathematics-cumulative-2-mode.rs:application-mode"
```

The mode advances the stage clock before entering egui's UI pass. The browser driver supplies event timestamps in milliseconds and egui time in seconds; confusing these units changes animation duration. Pointer events return false here because scene manipulation belongs to the later scene adapter. `FullOutput` carries texture and shape changes back to the GPU owner for submission.

`Mode<Host>` uses a trait object so the application stores one active content implementation without knowing its concrete type. `Frame` carries the logical canvas rectangle, canvas dimensions, pixel density and current time; these are observations supplied by the application, not a second clock. The background is four normalized colour channels, so this interface does not require a GPU device.

The frame callback ticks the existing clock, runs egui once and returns both the output and repaint request. The application uploads textures and presents. Key input calls the presentation's navigation implementation; the scene adapter uses that same implementation. Pointer handling remains the later scene adapter's responsibility. The loaded-mode factory consumes its payload exactly once, using ownership rather than a second copy of the deck and fonts.

Predict whether replacing the empty host with a scene requires changing this implementation: it does not, because none of these operations reads host data. The scene adapter instead owns a separate callback that applies its camera before painting and synchronizes geometry afterwards. This interface passes the native mode probe and full application native and WebAssembly compilation. The cumulative constructor uploads the returned egui textures, records its draw pass and presents. A key uses the browser event timestamp, so an animation starts at the time of the input even after a long idle interval. Resizing reconfigures the canvas and marks the presentation dirty.

### Column and equals alignment

Copy [layout.rs](code/03a-mathematics-cumulative-10-layout.rs) and [object.rs](code/03a-mathematics-cumulative-17-object.rs). An object keeps its content, colour, size, position, fit, opacity and named primitive groups. Layout measures blocks before positioning them.

Follow `relayout`: stacking determines vertical positions; fitting determines a common usable scale; equals alignment offsets each expression by its own `Expr::align`. Left-aligning `x=1` and `x+y=2` cannot align their equals signs because their left sides have different widths. Rows lay out formulas, pictures and matrices together, and an item with an explicit `width` in em centres its primitives inside that fixed slot.

`Presentation::paint` builds the current frame and applies the stage split before passing that mapping to the painter. The column layout keeps authored positions independent of the viewport.

`object.rs::Obj` separates immutable layout from animated state. An `Rc<Expr>` shares positioned primitives, while `pos`, `dim` and per-primitive colours hold tweens. `hidden` retains primitive indices rather than deleting glyphs, so matrix row addresses remain valid when entries disappear. `size * fit` converts the expression's em units to its displayed scale. Named points are sampled through `PointSet` at one time; a point object cannot recursively resolve itself through its own name.

### Matrices and addresses

Copy [matrix.rs](code/03a-mathematics-cumulative-12-matrix.rs). Measure entries before choosing cell width and height; centre each entry using its full height and depth.

```rust
--8<-- "journey/code/03a-mathematics-cumulative-12-matrix.rs:cells"
```

Each matrix cell has a rounded fill and zero or more glyph primitives. Groups such as `0,1`, `0,:`, `:,1`, `cells` and `entries` retain primitive indices. A later action can address a row without guessing coordinates. Entry ink is chosen for contrast against its cell fill.

Named net matrices select a square-pattern representation above 14 rows or columns. Authored matrices select it explicitly with `pattern: true`. Zero entries remain empty; nonzero entries show structure. This is a different reading task from inspecting individual values, so it does not squeeze hundreds of unreadable numbers into the same rectangle.

### Clock, actions and drawing

Copy [tween.rs](code/03a-mathematics-cumulative-24-tween.rs), [actions.rs](code/03a-mathematics-cumulative-3-actions.rs) and [draw.rs](code/03a-mathematics-cumulative-6-draw.rs). Sample motion from time and endpoints rather than repeatedly adding a distance per frame.

```rust
--8<-- "journey/code/03a-mathematics-cumulative-24-tween.rs:clock"
```

A transition starting at 100 ms with duration 200 ms reaches its midpoint at 200 ms and its endpoint at 300 ms. Easing changes the speed along that interval. Retargeting starts from the sampled current value, so an interruption does not jump back to an old starting value.

`actions.rs` updates object state and schedules effects. `draw.rs` converts that state into primitives, including fade-out snapshots and rounded boxes. Removing an object must retain enough drawing data to finish its disappearance. The runtime advances pending actions and retires completed animations:

```rust
--8<-- "journey/code/03a-mathematics-cumulative-22-run.rs:clock"
```

A thing moves only when the movement is the idea. Use a matching transform for a rewritten equation. Use appearance for a newly introduced object. Colour changes can identify an addressed matrix row without moving it elsewhere.

`actions.rs::apply` performs the same state transition during animation and replay. The `animate` flag controls whether effects enter the animation queue; it does not skip adding or removing the object. Adding a group staggers its appearance inside one duration and lays out the column once the objects exist, while a box action attaches a flush `-1.2`-unit inset outline to its target. Removing an object also removes its attached label and dependent boxes. That ownership rule prevents an orphaned annotation after the formula it describes has gone.

`draw.rs::prims_of` samples positions and colours, converts expression primitives into slide pixels and makes hidden entries transparent. It neutralizes a matrix cell when all its entry primitives are hidden, preventing the cell colour from revealing an answer prematurely. `fade` scales alpha for both primitive and path drawings. Rounded box outlines use a five-unit corner radius clamped to half the available width and height, so a small box cannot turn inside out. These operations produce drawing data; the atlas and egui painter still own rasterization.

### Kernel motion values

The clock supplies a normalized, eased parameter. The kernel supplies the value at that parameter. Keeping these responsibilities separate lets one clock move different kinds of objects without adding a second timing system.

Read these live kernel implementations and their tests; the mathematics checkpoint already depends on this kernel. The later scene subject connects their results to scene rows.

- [Vector::lerp](../../../session_rust/src/vector.rs): At parameter `0.5`, each component is halfway between its endpoints. Between `(2, 0, 0)` and `(0, 2, 0)`, the result is `(1, 1, 0)`. This does not preserve vector length. Use it for component motion, not as a rotation algorithm.

- [Color::lerp](../../../session_rust/src/color.rs): Red, green, blue and alpha are blended component by component; the result keeps the first colour's name. This is interpolation of stored channels, with no perceptual colour-space conversion.

- [Line::lerp](../../../session_rust/src/line.rs): The start moves toward the other start and the end toward the other end. The result copies the first line's other properties. Swapping one line's endpoints changes the motion and can collapse the midpoint.

- [Polyline::resampled](../../../session_rust/src/polyline.rs): A requested count of points follows equal arc-length intervals, retaining the first and last point. Counts below two, or inputs with fewer than two points, return a duplicate. Equal spacing need not retain every original corner.

- [Polyline::morph](../../../session_rust/src/polyline.rs): Both inputs are resampled to their larger point count, then corresponding points are blended. Correspondence follows traversal order. Even at parameter zero the result may be the resampled first path, rather than its original vertex list.

- [Xform::interpolate](../../../session_rust/src/xform.rs): Translation and axis scales blend linearly; rotation follows a normalized quaternion blend, using spherical interpolation away from nearly equal rotations. This interpolates a translation/rotation/scale decomposition, not all sixteen matrix entries. Do not assume arbitrary shear is preserved.

The vector, colour and line implementations do not clamp the parameter. The stage clock clamps time before easing, so an expired animation stays at its endpoint. `geometry.rs::geo_points` uses the kernel's point, line and polyline operations after that time calculation. The stage's floating-point RGBA tween remains a separate small value implementation; it does not call `Color::lerp`.

Open **Lerp** in the Vector, Color and Line kernel test pages, **Morph** in Polyline (which also exercises resampling), and **Interpolate** in Xform. Read the live assertions before running the corresponding kernel tests. A discoverable test name alone does not establish that those tests ran in this checkout.

Predict the midpoint of a line and the same line with reversed endpoints: both ends meet at the centre. Compare that with rotating the line's placement by half a turn using `Xform::interpolate`: at the halfway orientation, its length stays unchanged. Choose the operation according to what the movement means.

### Steps, slides and stepping backward

Copy [run.rs](code/03a-mathematics-cumulative-22-run.rs). A chapter owns the shared scene and its slides; each slide contains steps. The runtime flattens those steps for navigation while retaining slide boundaries.

```rust
--8<-- "journey/code/03a-mathematics-cumulative-22-run.rs:navigation"
```

Advancing first finishes any running step. Going backward rebuilds the chapter and replays actions without animation up to the requested position. It does not invert every mutation. Replay restores removed objects, colours and transforms using the same action semantics as forward navigation.

### Matching transform

Copy [morph.rs](code/03a-mathematics-cumulative-14-morph.rs). Match explicit named parts first, then pair equal symbols without using either primitive twice.

```rust
--8<-- "journey/code/03a-mathematics-cumulative-14-morph.rs:matching"
```

Matched primitives interpolate their positions and sizes. Unmatched old primitives fade out; unmatched new primitives fade in. Longest-common-subsequence matching preserves the order of repeated symbols. In `x+y` becoming `x+y+z`, the existing symbols retain their identity while the new term appears.

### Deck data and errors

Copy [deck.rs](code/03a-mathematics-cumulative-5-deck.rs), [eval.rs](code/03a-mathematics-cumulative-7-eval.rs) and [check.rs](code/03a-mathematics-cumulative-4-check.rs), together with the supplied [geometry](code/03a-mathematics-cumulative-9-geometry.rs), [view](code/03a-mathematics-cumulative-25-view.rs) and [net](code/03a-mathematics-cumulative-16-net.rs) modules.

```rust
--8<-- "journey/code/03a-mathematics-cumulative-5-deck.rs:schema"
```

An index names chapter files and defaults to the dark ground when `theme` is omitted. The chapter parser expands short keys and author blocks into actions, parses optional fixed-width row slots in em, and merges step-level appearance and alignment options onto both scalar and nested `eq`, `then` and `tex` formula definitions through `object_with_options` without discarding their TeX text. It carries the chapter name into errors; playback adds the step context. Unknown names and unsupported TeX must be reported rather than silently dropping the requested content. The overlap check uses laid-out ink bounds, not string lengths.

`eval.rs` reads expressions with recursive descent: `expr` handles addition, `term` handles multiplication, and `factor` handles literals, names, vectors and grouping. The `Val` enum distinguishes a scalar from a three-component vector. Addition requires matching kinds; multiplication permits scaling a vector by a scalar but rejects multiplying two points. `Points` supplies named values without coupling evaluation to a GPU scene. Predict `(a+b)/2` for two named points, then try `a+2`: the first is their midpoint, while the second reports incompatible kinds.

`check.rs` measures actual drawing primitives. A glyph box uses its bearing, height and depth; rectangles and pictures use their dimensions; strokes use their endpoints. Invisible primitives do not enlarge an ordinary object box, but an explicitly requested subset can be measured while hidden. `overlaps` compares panel blocks and labels and reports pairs whose intersection exceeds one slide unit in both directions. It does not prove scene depth visibility or all-frame animation separation. Run it after layout and after the intended clock sample, rather than estimating overlap from text length.

Type the lesson's [example.yaml](code/03a-mathematics-cumulative-27-example.yaml). Its two slides exercise equals alignment, matching, a matrix and a row address.

```yaml
--8<-- "journey/code/03a-mathematics-cumulative-27-example.yaml:deck"
```

The next scene subject uses the same schema and runtime. It adds the adapter that turns `Spatial` descriptions into kernel geometry and GPU rows; it does not replace the 2D implementation.

## Run and observe

Serve the assembled project through the workspace storage guard and build slot, using the shared Cargo cache and `trunk serve --port 8780`. Open `http://127.0.0.1:8780/?tutorial=mathematics`.

Expect two equations aligned at their equals signs. Right arrow advances through a matching rewrite, a matrix, and its coloured first row. Left arrow reconstructs the previous state; Up and Down change slides; `T` changes the theme. The scene region stays empty. Resizing changes the letterbox and pixel density without changing the authored step. Reload with `?tutorial=painting` or `?tutorial=frame` to run the retained preceding experiments.

## Predict and try

Change the second equation's left side from `x+y` to `x+y+z`. Predict which edge moves: the equals sign should stay fixed while the left edge moves left. Restore the fixture, advance to the coloured matrix row and step backward. Predict whether replay restores the earlier colour as well as its position.

## Question and answer

Why replay previous steps instead of reversing each animation?

The action stream constructs every state, including removed objects and named groups. Replaying it restores that information; reversing motion alone cannot recover an object that was removed.

### Test the model and its host separately

`tests.rs` uses the real fonts and shipped chapter files. Its formula and layout tests distinguish math-axis fraction placement, named parts, spaces, fixed-width row-slot centring, nested and scalar `tex`/`eq`/`then` option merging, and unsupported commands. Deck checks replay actions at rest and during playback, inspect layout errors, and exercise matrix filling and dark-default theme swaps at the same step. These assertions test the interpretation of a deck; they do not initialize a browser GPU or establish physical-phone interaction.

`presentation_tests.rs` supplies pictures and a minimal chapter to the same `Presentation` constructor. It checks picture order and aspect ratios, reversible frame coordinates at desktop/upright sizes, and a second settled egui frame that still contains shapes without requesting another repaint. The mode test uses an integer host and verifies it stays unchanged as background state transitions from the default black ground to white on `t` and steps backward. That last assertion demonstrates why mathematical playback can finish before scene integration exists.

## Validation

The 41-file partial project passes its locked wasm Cargo check. The cumulative tooling tests retain every earlier implementation unchanged and check that actual production source is used. The complete viewer also passes native and default/tutorial wasm typechecks after the dependency extraction.

The previous [standalone mathematics evidence](../lessons/03a-math/full-browser-evidence.json) and [earlier cumulative evidence](release/cumulative-mathematics-registration-windows-20261114.json) retain their own source scope. They do not establish acceptance for this new assembly. The linked bundle passes all four visible steps, exact backward reconstruction, theme round trips and slide navigation at desktop and upright dimensions; the native candidate suite passes 21 tests with one GPU-dependent test ignored, as re-verified after upstream dark-default ground, flush box-highlight, fixed-width row-slot and deck updates in [the stage reconciliation receipt](release/completion-stage-reconciliation-windows-20261118.json).

The [current interaction receipt](release/completion-partials-windows-20261115.json) records Chrome 157 and the Intel Gen-12LP adapter. Upright viewport emulation is not a physical-phone test, and these checks do not accept the remaining cumulative course.
