# 12 · Pick visible source objects and edges

**Start:** checkpoint 11. **Finish:** real pointer input selects visible objects and original edges, with yellow feedback. This stage replaces the early teaching shell with the production winit/State shell.

## The shell transition is explicit

The early chapters called a small direct-canvas tutorial controller. The production application needs owned event callbacks, cancellation, loading messages and demand-driven redraws. `lib.rs::App` now receives events and passes named actions to State.

The complete file list includes all replacements and removals. Do not keep both entry points or two GPU owners. After this lesson the camera, scene and renderer live under the same production ownership used by the final viewer.

## Render identity under the cursor

```text
pointer in CSS pixels → bounded physical-pixel pick window
                                      ↓
                         physical depth + integer ID pass
                                      ↓ asynchronous map
                       GPU row/sub-ID → source identity
                                      ↓
                          State selects → yellow frame
```

The ID texture uses integer values, with no alpha blending or color conversion. It is not a screenshot whose colors are approximately decoded into IDs. The pick pass uses visibility consistent with display toggles, including physical occlusion.

Read only a small circular tolerance window around the pointer. The radius is specified in CSS pixels, so selection remains usable at different DPRs. Nearby candidate ranking and edge precedence happen within that explicit window.

## Asynchronous answers can become stale

GPU readback cannot block the browser thread. The request records which scene, camera/viewport and selection mode it belongs to. A later camera move, hide operation or mode change can invalidate the answer before it arrives.

A mouse press moving at least four CSS pixels becomes a drag; it should not also select the object at release. Focus loss and pointer cancellation retire incomplete gestures. Keep these decisions in input/state methods rather than burying them in event closures.

Plain click selects/toggles an object. Ctrl+click requests original mesh/BRep/NURBS edges. Display tessellation segments of a standalone curve do not become new selectable source edges.

## Yellow feedback is part of success

An ID result alone is insufficient. The object must visibly highlight after the selected flag is uploaded and a frame is requested. The maintained interaction tests inspect actual yellow pixels as well as source identity.

Face selection and the final union silhouette compositor arrive in chapter 17. This checkpoint establishes the row and edge mapping they extend.

## Write the files

Follow [Complete file changes for 12](../lessons/12/index.md). Read `SelectionMode`/`PickMode`, the ID target/readback owner, input translation and State transition in order. Copy the interaction protobuf fixture through the binary-input command.

## Checkpoint

```sh
cd "$COURSE_WORK/session_viewer"
cargo check --locked --lib
trunk serve --port 8780
```

At <http://localhost:8780/?data=off&inspect=1>, click the local fixture's mesh, line, polyline, curve, surface, BRep and cloud. Each must visibly highlight. Ctrl+click an eligible source edge. Drag the camera and click again; a late result must not select the previous view's target.

Use the checkpoint's detailed picking fixture for source-ID assertions. If an object highlights but reports the wrong source GUID, repair the mapping rather than the color shader.

**Before continuing:** explain why a correct ID answer from an obsolete camera is still invalid. Continue to [source controls](13-controls.md).
