# Route input and select geometry

Orbit, select and edit the scene while keeping dock typing and panel gestures separate from scene interaction.

## Prerequisites and ownership

Complete the camera, renderer, dock, application, document and command chapters. This application chapter teaches `app/input.rs`, `touch.rs`, `keys.rs`, `gesture/`, `selection.rs`, `ui/pointer.rs` and the selection adapters in `state.rs` and `state/features.rs`. The renderer retains its completed GPU picking implementation; the application interprets those asynchronous answers.

## Build

Register the application gestures and route panel, keyboard, pointer and pick events through the existing host.

### A pointer belongs to one interaction

The panel adapter receives an event before normal scene input. `ui/pointer.rs` uses the current pointer position and actual popup layers, not the previous frame's hover state. A press in a panel captures its drag. Releasing a scene drag over a panel drops that edit rather than applying it through the UI. Wheel and touch events also respect the scene rectangle.

The command dock's completed focus policy determines when text entry owns keyboard input. A keyboard shortcut intended for text must not also select scene objects. Scene shortcuts run through `keys.rs`, which maps keys and modifiers to the same command behavior used by the dock. Runtime focus and repeat checks happen before this dispatch.

Normal mouse navigation uses right drag to orbit, middle drag to pan and the wheel to zoom. A right press that remains a click can finish a draft or repeat a command; a right drag must not repeat that command on release. `Input` therefore retains both press position and whether it became a drag.

### Choose a gesture before changing geometry

```rust
--8<-- "session_viewer/src/app/gesture/mod.rs:input-gestures"
```

The registry tries control handles, gizmo handles, object drags and box selection in that order. A handle may take the initial press. An ordinary object drag or selection rectangle starts only after the pointer moves beyond the click threshold. This lets a small hand movement remain a click while deliberate movement starts editing.

Each gesture supplies press/start, drag and release callbacks. Once chosen, it remains responsible for the interaction. Adding a gesture means adding its module and registry entry, not reopening every event branch. The `control`, `gizmo`, `object` and `box_select` adapters delegate to their corresponding state operations, which own saved placements, preview and commit/cancel behavior.

Input thresholds are in CSS pixels, while window events and GPU picking can use different pixel spaces. The adapter converts window coordinates to surface pixels and accounts for device scale when comparing movement or handle reach. A finger gets a wider handle reach than a mouse; multiplying an already converted position by the device ratio again would move the pick away from the visible pointer.

### Give each adapter a precise release policy

The control and gizmo adapters can claim the initial press, but a click has different meanings. Control release cancels the gesture when movement stayed a click, restoring the point. Gizmo release instead opens its numeric input on a click. A true drag commits through each corresponding state operation. Keeping these release callbacks in their adapters prevents a shared mouse handler from confusing click-to-edit with drag-to-transform.

The object and box-selection adapters have no initial-press callback. They become eligible only after the host recognizes a plain mouse drag. Object dragging delegates its delayed pick and movement to state; a miss can leave the selection rectangle path responsible. Box selection delegates start, update and completion without implementing geometry edits. A finger touching an ordinary object therefore does not acquire this mouse-drag adapter merely because it lands on geometry.

`keys.rs` routes Escape and Enter to the active state continuation and maps canvas Delete, F, H and S to commands. Ctrl or Cmd selects history shortcuts: Z undoes, Y or Shift-Z redoes, and A selects all. The host must first decide whether text owns the keys; the shortcut mapper itself does not know which field is focused. Reusing command dispatch gives keyboard and dock actions the same validation and history behavior.

### Touch has its own state

```rust
--8<-- "session_viewer/src/app/touch.rs:input-touch-camera"
```

The ordinary viewer tracks touches by browser ID. One finger orbits. Two fingers pan by their midpoint and zoom by the change in separation. When finger count changes, the next event records a fresh span instead of applying a jump from the previous gesture. Each pinch ratio is clamped to bound an unusually large event.

A tap must satisfy both distance and duration limits. A nearby second tap can fit the view. The input host separately tracks a finger engaged with an editing tool; camera gestures must not simultaneously move that tool. Cancellation clears the gesture and pending tap. Browser pointer cancellation also invalidates the host's gesture state and wheel interaction.

Stage input is a different mode: quick horizontal swipes step through the deck and a column tap advances it. Its scene camera uses the stage's own held-touch policy. These policies belong to the stage adapter described in [the scene chapter](stage-scene.md), not an additional rewrite of normal viewer touch handling.

### A pick answer is not immediate

The GPU pick pass identifies geometry near a cursor and copies its result to a readback buffer. Mapping that buffer completes asynchronously. `Pick::poll` distinguishes no answer yet from a completed background hit: its return type is an option containing another option.

Each request or cancellation advances a generation. The submitted copy remembers its generation; an answer from before a newer request, camera change or cancellation is discarded. The host can continue rendering while waiting, without letting a delayed answer select an object under an obsolete cursor.

`state/features.rs::TAKE_PICK` offers a completed result to the active drag, tool, split and cloud-query adapters before ordinary selection. The first consumer handles it. `State::apply_pick` then interprets unconsumed results using object identity and the current selection mode. A non-pickable decoration may be visible without participating in this path.

### Selection identifies what an edit means

```rust
--8<-- "session_viewer/src/app/selection.rs:input-selection"
```

`SelectionMode` distinguishes a whole object from an edge, face or control on a parent row. `ControlId` preserves the geometric address: a vertex, curve control, surface control pair or cloud point. These are not interchangeable with GPU buffer offsets.

`Controls` derives finite control positions and connection lines from the kernel geometry. Cloud controls remain GPU-resident instead of expanding every point into a CPU vector. Escape can return from a sub-selection to its parent. Editing uses the mode to choose a whole-object transform or a deformation of geometry inside that object.

### Bridge the phone keyboard without losing composition

`CommandAgent` owns the hidden browser input and its event-listener closures. Input events send the complete text through the event-loop proxy; editing keys send explicit key messages. Composition start/end tracks whether an IME is building a word, so the host can avoid replacing that in-progress input. Touch or pen taps focus the hidden field within the browser gesture when a text target needs the keyboard, while taps outside fields and popups blur it. A raised-keyboard flag prevents the same tap from immediately undoing that focus. Drop removes all installed listeners; retaining the closures only for the agent lifetime prevents callbacks from outliving their Rust owner.

### Retain input state until the interaction ends

`Input` distinguishes a quick pointer wobble from deliberate dragging: a plain mouse press starts a drag after more than ten CSS pixels, or more than four pixels held for at least 200 milliseconds. A chosen gesture keeps ownership through release. Running pointer tools take priority over ordinary selection; right-button clicks can finish or repeat commands only when they did not become navigation drags. Touch editing tracks the owning finger ID. A second finger cancels that edit and suppresses further actions until every finger lifts, then resets camera-touch state. Lost focus and browser pointer cancellation clear the held state instead of committing an abandoned edit.

Selection separates a parent object row from the stable address of its selected edge, face or control. `Controls` filters nonfinite positions and creates links only between available neighboring controls, so a missing control does not shift the original geometric indices. BRep controls include vertex, curve and surface addresses; surface links connect both grid directions. Cloud points stay on the GPU instead of expanding into CPU controls. Escape returns to object mode while retaining the parent identity for the caller.

The phone adapter routes whole-text messages to the last registered open panel field before the command dock. Whole replacement avoids duplicating composed input; key messages become one press and release. When the dock is closed, newly typed characters return to viewport dispatch. Following a field synchronizes externally changed text, but skips inline completion suffixes and active IME composition. Closing the last field lowers the keyboard unless the command line owns it. This adapter decides text ownership without modifying geometry or the browser listener lifetime.

`ui/overlay.rs` converts device-pixel preview coordinates to egui points before painting drawing lines, filled regions, dashed strokes and square point marks. The label chip measures its text galley, adds padding and draws centered white text on black. These temporary overlays visualize the active tool or measurement; they are not persistent document objects or GPU-pickable geometry. Keeping the conversion at this boundary avoids applying the display scale twice.

### Keep selection requests tied to their initiating gesture

A box-selection gesture stores both corners in device pixels. Left-to-right means a window that contains whole objects; right-to-left means a crossing that touches them, drawn with a dashed outline. Starting is refused while drawing or splitting owns input. Release applies Ctrl removal before Shift addition and reports both matched and resulting selected counts. The overlay visualizes the pending region; selection changes only when the gesture finishes.

The cloud-query state adapter starts only in streamed-cloud control mode and assigns each request a generation. It combines camera projection with cloud placement and converts the CSS selection aperture into device pixels. Source pages produce temporary GPU-pick candidates while displayed LOD remains bounded; after eligible pages are checked, the winner is resolved to its original source ID. View or selection changes cancel the query. Batches with another generation or a cancelled token cannot replace the current selection.

Picking a sheet entity highlights its edge within the parent row rather than selecting the whole sheet; picking the same entity again clears selection. A new generation identifies its optional metadata request. Matching responses cache the side table and expose the entity name and kind; stale or cancelled responses are ignored. A missing metadata URL still permits selection by entity ID, while a metadata error is reported without inventing a name.

## Run and observe

Run the cumulative application with the comparison fixture. Drag with the right button to orbit, use the wheel to zoom, and enter `Fit` to restore the framing. Open `Layers All` and click an object's row: its selection highlight and gizmo should appear. Type a command in the dock and verify that its letters do not trigger scene shortcuts.

At an upright 390-pixel viewport, use `Layers Off` to recover the scene width. The open panel has a 240-pixel minimum and can crop document nameplates in the remaining region. This is current production behavior, not proof of a fully adapted phone layout. Test ordinary touch navigation separately from the stage's swipe/tap navigation.

![An actual feature-pixel click resolves the original Element, pick-first. This is the controlled front-view fixture, not a general picking-mode or physical-device qualification.](../screenshots/completion/feature-pixel-selected.png)

An actual feature-pixel click resolves the original Element, pick-first. This is the controlled front-view fixture, not a general picking-mode or physical-device qualification.

![Deleting that original removes its visible feature and makes the same pixel unpickable. The second original remains; allocated ribbon storage is retained for Undo.](../screenshots/completion/feature-pixel-deleted.png)

Deleting that original removes its visible feature and makes the same pixel unpickable. The second original remains; allocated ribbon storage is retained for Undo.

## Predict and try

Predict what happens when you press a handle, drag it, then cancel before releasing. Its preview should return to the saved placement, without an undo transaction. Compare that with accepting the drag and undoing once. Then type into the dock while the object remains selected: input ownership should prevent the same key from acting on both the text and the scene.

## Question and answer

Why must a completed GPU pick carry a generation?

Readback finishes later than the event that requested it. The camera, pointer or scene may have changed in between. Comparing the submitted generation with the current generation prevents an old result from selecting geometry according to a view the user no longer sees.

## Validation

Current cumulative browser evidence covers normal zoom and command focus, hierarchy-row selection, and stage mouse/reset/zoom/click/key navigation with emulated upright swipe/tap input. Hierarchy selection does not test GPU screen-pick readback. Normal-viewer multitouch, cancelled editing gestures and physical-device interaction remain separate checks; stage touch success must not be reported as normal-viewer touch acceptance.
