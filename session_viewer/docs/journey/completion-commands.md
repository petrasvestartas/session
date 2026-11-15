# Commands, tools and editing

Create a line, move it, and reverse the edit through the same command dock used by the production viewer.

## Prerequisites and ownership

Complete the dock, decoding, application lifecycle and document/history chapters. This chapter belongs to the existing `application` checkpoint: it owns `app/command`, `state/edit.rs`, `state/drawing.rs` and `state/tool.rs`. The dock continues to own text entry, focus, history display and completion presentation. Commands add behavior through the production registry rather than replacing that finished UI.

Continue with [construction planes, snapping and modeling](completion-modeling.md) in this same application checkpoint.

## Build

Copy the application command adapters and their registrations, then trace one typed line through parsing, execution and row synchronization.

### Separate spelling from execution

```rust
--8<-- "session_viewer/src/app/command/mod.rs:command-contract"
```

`Spec` describes names, aliases, help, options and parsing. A parser returns `Result<Box<dyn Action>, String>`: either an owned executable action or a user-facing explanation. The `Action` trait specifies execution and whether a command requires selection or preserves an active draft/split. Parsing is independent of the mutable viewer state; execution receives it explicitly.

The command matcher compares names without spaces or case and chooses the longest matching name. It returns the consumed word count, leaving arguments intact. This prevents a short verb from stealing the prefix of a longer command. `canonical` normalizes the displayed verb and option while preserving argument text. `number` rejects non-finite values before geometry or transforms receive them.

The `verbs!` macro declares modules and collects their `SPEC` values into `REGISTRY`. Each entry implements `Verb`; ordinary commands expose a `Spec`, while drawing commands additionally expose a `Draw`. A command therefore needs its implementation and a registry entry, with no new dispatch branch in the dock.

### Execute through the application host

```rust
--8<-- "session_viewer/src/state/edit.rs:command-dispatch"
```

`State::run_command` canonicalizes the line, calls `command_action`, and remembers a repeatable command only after success. `run_echoed` is the path for commands started by mouse or keyboard: it uses the same execution path and sends the answer to status/history.

The host first lets the active drawing operation consume coordinates, options or Enter. Otherwise it parses an action. A bare verb with a short option list can open a chooser instead of failing immediately. Commands cancel incompatible drafts and pending splits according to their trait methods. An action requiring selection starts an object-selection prompt when no object is selected; it is not silently executed against the whole document.

### Drawing is a checked construction

`Draw` supplies the permitted point count, a constructor, labels and buttons. A fixed count, such as a line's two endpoints, can finish automatically. A variable count, such as a polyline, uses Enter to finish. Typed and picked coordinates converge on the same constructor.

The line constructor rejects coincident endpoints. Shared drawing validation rejects non-finite coordinates and magnitudes beyond the supported bound. `Create::run` asks the scene to model the geometry, synchronizes history, then selects the created object. A rejected constructor must leave both the document and its row count unchanged.

### Reuse construction rules across verbs

The Point, Line and Arrow files each declare a `Draw` specification and delegate point parsing to it. Point needs one location and clones it into owned geometry. Line needs two distinct endpoints and rejects length at or below 1e-12. Arrow reuses that same line constructor, then sets the end-arrowhead flag. The final geometry is wrapped in `Rc` for the application model; each verb contributes its own construction rule without duplicating coordinate parsing or the input loop.

Polyline and Curve accept a variable number of points up to the shared modeling limit. Polyline joins them in order and offers rectangle/polygon options through the tool path. Curve treats them as NURBS control points, with degree capped at three; it does not promise interpolation through every input point. A repeated first point with at least four entries selects closed construction and removes the duplicate before building. Closing therefore changes the construction rule instead of merely drawing a straight closing segment over an open spline.

Close acts on the active drawing: it adds the first point again and finishes through `close_drawing`. Its action preserves the draft until execution so normal command cancellation cannot erase the points first. This is why Close is available beside Polyline and Curve. It does not close a loaded document; document replacement is exercised through Open.

`Choices` presents a bare command's options as buttons, shortening the visible labels while retaining the complete command lines. It asks for no points and yields when the chosen command runs; Enter cancels. `tool/options.rs` handles multiword option matching by removing spaces and case, returning both the matched option index and consumed word count. Empty Enter actions and Escape are excluded from ordinary option matching, so continuation controls cannot be mistaken for geometric arguments.

The command tests compare registry completions with sorted, unique canonical names and require readable names/options. They also compare parsed actions for aliases and canonical spelling. These checks protect the registry as the single source of discoverable commands: adding a verb should make it available to parsing and completion together. They complement execution tests; a successfully parsed action does not prove that its geometry edit or undo behavior works.

### Delegate each command to the state it owns

Snap can toggle the master object-snapping switch or one named mode bit; its toolbar follows the master enabled state. Grid Snap instead controls construction-plane grid fallback and accepts a positive spacing that also enables it. Grid Snap preserves an active draft so spacing can change during drawing. The two adapters update the shared snapping state; they do not calculate intersection candidates or move already committed points.

Layers controls panel visibility through feedback state, refreshes its rows, and expands the hierarchy when All is requested. Report requests a diagnostic report download through the same feedback boundary and returns a status message. These commands expose UI services without reaching into DOM elements from the parser; their action result alone does not prove that a browser download completed.

Undo and Redo ask the scene history to move one step, report an error when no step exists, then call `after_history` to reconcile the application. Delete delegates selected-row removal to `delete_selection` and declares that it requires selection. The verbs do not manipulate GPU buffers directly; the document/history path determines which rows must change. A selection requirement is explicit in the action contract, not inferred from the command name.

Hide marks selected rows hidden, Show restores hidden rows, and Fit asks the camera to frame the selection or the whole scene when nothing is selected. These adapters call `hide_selected`, `show_all` and `fit_selected_or_all` respectively. Their zero-argument specifications keep completion and parsing consistent. Hiding changes visibility rather than deleting source geometry, while fitting changes the camera rather than moving the objects.

Escape declares that it preserves the active draft long enough to cancel it itself. If a draft exists, it calls `cancel_drawing` and keeps the selection; otherwise it clears selection through `escape_selection`. This order prevents the generic command-dispatch cancellation from making Escape incorrectly behave as though no tool were running. The same action also accepts the short alias Esc.

Arctic and Outline parse the shared On/Off contract and delegate to rendering state. Arctic uses `set_arctic`, which enables outlines when shading turns on; Outline changes its own flag. Opacity parses a finite number, rejects values outside zero through one and calls the host setter. These are view controls, so changing them does not replace the selected geometry or create a modeling result.

Element Attributes and Element Interactions share On/Off parsing but call different host paths. Attributes controls features owned by the element; Interactions controls contacts, joints and cuts contributed by other elements. The interaction path can require walking the element again to change the drawn feature geometry. The earlier walker classification supplies this distinction; the commands only expose it through the registry and return the resulting state to the dock.

Arrowhead parses None, Start, End or Both and requires selected curves. Execution checks locked rows, clones supported lines/polylines/NURBS curves with the requested flag and collects only changed geometries. One replacement transaction handles the edits, followed by row synchronization. A selection containing no supported curve produces an error; already matching heads produce zero changes. Head appearance is persistent curve state, distinct from the temporary reveal value stored in a GPU object row.

### Select through a common eligibility rule

Object, Edge and Face clear the current selection before changing `selection_tool`, so an old subobject selection cannot be reinterpreted as a different kind. Controls delegates to `enable_controls`, which supplies the control-point mode and its display setup. These zero-argument adapters choose what subsequent picks mean; they do not manufacture a hit or bypass the renderer's pick result.

`selecting::candidates` returns ascending rows that are selectable, not explicitly hidden and not hidden by the GPU object state. `apply` centralizes replacement, addition and removal; removal can binary-search the sorted candidate list. Region selection indexes loop crossings by pixel-row centres for an even-odd interior test. Window and crossing modes distinguish complete sampled containment from contact with the region. Geometry sampling has explicit budgets, so region selection is not an unlimited exact intersection oracle.

Select All uses every eligible row. Select By Name joins its remaining words, removes matching surrounding quotes and performs a case-insensitive substring match. Select Small requires a positive length and compares it strictly with valid world-bound diagonals. The name and size filters retain the existing selection when no match exists and report that fact. Shared eligibility keeps hidden or locked objects from reappearing merely because a selection was made by text instead of a click.

Select Lasso opens a pointer-driven tool that records a screen-space loop. Points must be at least two device pixels apart; reaching 4,096 points removes alternating samples and increases spacing instead of truncating the end of the gesture. On release the region query applies replacement, Shift-add or Ctrl-remove semantics. The overlay and selection share the collected loop, while cancelling leaves no partially committed geometry.

### Preview without committing on every pointer event

```rust
--8<-- "session_viewer/src/app/command/verbs/move.rs:command-move"
```

A `Draft` owns collected points, the construction plane, the current tool and saved row placements. A `Tool` supplies prompts, options, a cursor preview and a response when another point is placed. `Next::More`, `Next::Repeat` and `Next::Done` make the continuation explicit.

Interactive Move first collects a base point, then previews the translation to the cursor. The preview changes displayed placements; it does not append an undo transaction on every mouse movement. The second accepted point applies one transform through `State::apply`. Cancelling restores saved placements, clears the draft and keeps the selection.

`with_tool` temporarily takes the draft and tool out of `State` before invoking the callback. This gives the callback mutable access to the host without borrowing the tool field and the containing state at the same time. The host restores ownership afterwards and interprets the returned continuation.

Typed `Move 0 1000 0` bypasses point collection and directly applies the checked offset. A whole-object transform updates the document and descendants, synchronizes rows, refreshes the gizmo and invalidates the image. A selected face, edge or control point instead follows the subobject deformation path. Locked objects are rejected before either operation.

### Change drawing and viewing context

`Construction Plane` selects a fixed XY, XZ or YZ frame, accepts three noncollinear points, or returns to View mode. A bare command opens the choices, while a typed triple uses the same frame constructor as picked input. Setting a plane preserves the active drawing draft, allowing subsequent coordinates and snapping to use the new plane without discarding accepted points. This changes input context; it does not project existing geometry.

`View` parses camera directions, projection and display choices into one `Change` enum. Projection changes refresh bounds and use framed camera conversion; point and plane sizes require positive finite values. Other cases delegate to the rendering flags or the host setters for Xray, labels and visibility. The action preserves both a drawing draft and pending split, so inspecting the model from another view does not cancel the operation being constructed.

`Clipping Plane` distinguishes creation modes from global On/Off, selected-plane Flip and Hatch/Solid fill. Typed coordinates must be finite and within the supported magnitude, with exactly the point count required by the selected mode. Creation sizes the visible plane rectangle to the scene and delegates its document insertion to state. On/Off and fill preserve an active draft; Flip requires selection. The command does not implement the cap renderer or silently turn a plane into a geometry cut.

`app/clipping.rs` constructs clipping planes from a normal direction, three points or a named world-plane orientation. The normal points toward the removed side. Converting a placed plane to the GPU half-space crosses its transformed in-plane axes, which handles nonuniform placement without incorrectly transforming a normal as an ordinary vector. The kept-side normal is negated and its offset comes from the placed origin. Degenerate rectangles are rejected; hatch direction derives from the in-plane axes, keeping the pattern attached to the plane.

`state/clipping.rs` gathers each clipping plane from its current GPU row placement, so preview dragging moves the cut immediately. Hiding its rectangle does not disable its cut; the host reports that distinction when a plane becomes hidden. The first active cut verifies previously walked mesh solid orientation, sharing the result for instances of one geometry. Creation enforces the maximum plane count and uses document history, whereas On/Off and fill change rendering state. Flipping replaces the selected plane geometry and synchronizes its rows.

### Exercise the command field frame by frame

The command-field tests feed egui press/release events and individual text characters through real frames. Typing `Orient 3 Points` one character at a time must retain its spaces rather than prematurely accepting a completion. Option browsing begins from the typed value: Down after `Snap Off` selects `Snap End`. Up and Down cycle options, Left and Right leave the command unchanged, and Enter either runs the accepted choice or leaves the expanded text according to the command contract. This tests UI semantics without claiming browser IME or phone keyboard acceptance.

## Run and observe

Run the `application` checkpoint's `production.html` entry with the recorded comparison fixture. Enter `Line 0,0,0 3000,0,0`, then `Fit`. A new line appears and is selected. Expand the new Created layer to see its hierarchy row; new layers start collapsed. Enter `Move 0 1000 0`, `Undo`, then `Redo`; the line alternates between its original and translated position. Existing comparison geometry stays in place.

Enter `Line 0,0,0 0,0,0`. The dock reports that endpoints must differ and adds no object. Select `my_line` in the expanded Created layer; undo/redo does not promise to retain selection. Enter `Move` to display the base-point prompt, then `Escape` to cancel it. The committed geometry remains unchanged.

![A typed Move changes the selected line. The command history and hierarchy assertions support this step; this image does not prove every editing gesture.](../screenshots/completion/typed-move.png)

A typed Move changes the selected line. The command history and hierarchy assertions support this step; this image does not prove every editing gesture.

## Predict and try

Predict the undo count after moving the cursor through several positions during interactive Move and then cancelling. Run `Move`, choose a base point, move the cursor, and press Escape before choosing the target. No transform transaction should be committed: preview placements are restored. Repeat and accept the target; one Undo should restore the original placement regardless of how many cursor events preceded it.

## Question and answer

Why does the dock keep a command specification separate from its action?

Completion and help need stable spelling and options before execution is possible. An action carries parsed values and receives mutable state only when the host runs it. The same registered command can therefore be offered by completion, typed in the dock or triggered by another input without duplicating editing logic.

## Validation

The cumulative application passes 171 native command tests. Fourteen browser captures cover this chapter's creation, move, undo/redo, invalid construction and cancellation sequence at desktop and upright sizes, with command history and hierarchy assertions. The upright open-panel view remains narrow and crops scene labels; close Layers for a full-width inspection. These checks do not establish interactive drag-preview cancellation or every modeling operation.
