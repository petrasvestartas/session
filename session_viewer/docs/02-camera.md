# 02 · Move a camera through coordinate spaces

**Start:** checkpoint 01. **Finish:** orbit, pan and zoom the triangle using the camera math that will remain in the viewer.

## Four spaces, one deliberate conversion

```text
source/local coordinates → object placement → world coordinates
                                                 ↓ view matrix
                                           camera coordinates
                                                 ↓ projection
                                        clip (x, y, z, w)
                                                 ↓ divide by w
                                      normalized device position
                                                 ↓ viewport + DPR
                                           framebuffer pixels
```

Source geometry uses f64 values because CAD positions may be large while local details are small. GPU records use f32, so the camera and object translations are rebased around a nearby origin before upload. Converting the original huge world coordinate to f32 first would already lose detail; subtracting an origin afterward cannot recover it.

A matrix has a convention, not just sixteen numbers. Read multiplication and upload together: the shared math, Rust arrays and WGSL `mat4x4` must agree about columns and composition order. The lesson's complete camera and math files establish that contract.

## Perspective and orthographic views

Perspective divides by distance through `w`; farther objects appear smaller. Orthographic projection does not shrink objects with distance. The camera retains a target and distance/extent so switching projection can preserve a useful framing.

Depth is reversed in the finished renderer: larger values are nearer. The projection, depth clear and comparison function must change together. Do not copy a matrix from a tutorial using the opposite convention without checking its depth range.

Orbit changes the eye around the target. Pan moves the target in the camera's right/up directions. Zoom changes distance or orthographic extent. Wheel zoom uses the cursor position so the region under the pointer stays useful instead of always moving toward the viewport center.

## Logical size and physical size

CSS controls the displayed canvas size. The framebuffer dimensions are approximately CSS size multiplied by device-pixel ratio. Pointer events arrive in CSS coordinates; GPU pixels use physical coordinates. Apply the conversion exactly once. This will also determine text raster size and picking tolerance later.

Resize can change logical dimensions even when the rounded framebuffer size stays the same. Keep both values. A hidden or zero-sized canvas should not create invalid surface textures or enter a redraw loop.

## Write the files

Follow [Complete file changes for 02](../lessons/02/index.md). Read the camera data and vector/matrix helpers before the browser input adapter. The adapter should translate events into named camera operations, not contain a second camera implementation inside a callback.

Notice `&self` for methods that read the camera and `&mut self` for methods that change it. Rust's references express that ownership contract at each call.

## Checkpoint

```sh
cd "$COURSE_WORK/session_viewer"
cargo check --locked --lib
trunk serve --port 8780
```

At <http://localhost:8780/?data=off&inspect=1>, drag to orbit, hold Shift while dragging to pan, and zoom with the wheel. These are the early teaching shell's bindings; lesson 12 introduces the production right/middle-button controls. Resize the page and repeat. The triangle should remain stable instead of jumping or becoming stretched. The page inspection records both logical and physical sizes.

If dragging moves twice as far on a high-DPI display, inspect the CSS-to-framebuffer conversion before changing camera speed. If a large translated model later jitters, inspect where f64 becomes f32.

**Before continuing:** trace one local point through placement, rebasing, projection and viewport conversion. Continue to [source identity](03-identity.md).
