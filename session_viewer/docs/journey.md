# Build a viewer you can explain

Start with a picture you understand, then grow it into the viewer we already use. You will type the implementation yourself. We will revisit earlier code when a new responsibility appears, explain the change and keep a working checkpoint.

**Current release: 39 cumulative lessons, about 87–151 active study hours.** Installation is extra. These estimates include reading, typing and experiments. Check the [release evidence](journey/release.md) before starting. The complete feature course is still being written.

The [complete draft lesson checklist](journey/roadmap.md) has **101 proposed slots: 29 current checkpoints and 72 later slots planned**. This is a teaching plan, not a fixed final count or percentage of engineering work.

**Command-line revision:** [Use the viewer’s real command dock](journey/command-line.md) from the early lessons. Features are exercised by typing commands; later lessons retain canvas picking and gestures.

The white canvas fills the browser window from lesson 01. The runtime shows your drawing and the command dock; lesson titles and explanations stay in this documentation.

**One-hour typing target:** the [typing-load audit](journey/typing-load.md) separates actual code changes from reading and experiments. Seventeen existing checkpoints still need splitting; working builds alone do not make those long sections finished lessons.

## Start small, keep the destination

| Lesson | Time | Working result |
| --- | --- | --- |
| [01 · A page that Rust can reach](journey/01-canvas.md) | 1–2 hours | Fill the browser window with a canvas and let Rust signal that it has started. |
| [02 · Ask the GPU to paint](journey/02-clear.md) | 2–3 hours | Paint the whole canvas white with a real GPU command. |
| [03 · Give the GPU three corners](journey/03-triangle.md) | 1–2 hours | Draw a pink triangle on the white background. |
| [03a · Draw our command line](journey/03a-panel.md) | 4–6 hours | Draw the production command panel and its Noto text over the triangle, using the same GPU. |
| [03b · Give the command line its memory](journey/03b-state.md) | 3–5 hours | The text field and history have one owner. |
| [03c · Draw completion and history](journey/03c-layout.md) | 8–12 hours | Lay out the production command dock from its model. |
| [03d · Type into the real command dock](journey/03d-input.md) | 5–8 hours | Send browser events to the dock and submit Help. |
| [04 · Make a choice change the picture](journey/04-input.md) | 1–2 hours | Use a command to switch backgrounds without changing the triangle. |
| [05 · Let Rust supply the corners](journey/05-vertices.md) | 1–2 hours | Draw a rectangle from six positions stored in a GPU buffer. |
| [06 · Share a corner between triangles](journey/06-indices.md) | 1–2 hours | Draw a diamond from four positions and six small index numbers. |
| [07 · Send one view setting to every corner](journey/07-uniforms.md) | 2–3 hours | Scale and shift the diamond without changing its stored positions. |
| [08 · Move the view, keep the geometry](journey/08-camera.md) | 2–3 hours | Pan, zoom and reset a flat view through camera state. |
| [09 · Let one matrix describe the view](journey/09-matrices.md) | 2–4 hours | Rotate the flat view using a matrix, ready for the third dimension. |
| [10 · Keep the nearest surface](journey/10-depth.md) | 2–4 hours | Draw two overlapping triangles in depth, keeping the nearer one visible even when it is drawn first. |
| [11 · Give the scene an owner](journey/11-scene.md) | 3–5 hours | Add and remove a mesh through scene data while reusing the renderer and camera. |
| [12 · Name objects without depending on their row](journey/12-identity.md) | 3–5 hours | Select and delete objects using stable identities, then highlight the selected object. |
| [13 · Ask which object is under the pointer](journey/13-picking.md) | 2–4 hours | Click a visible triangle to select its stable object ID, including after camera movement. |
| [14 · Make document changes reversible](journey/14-history.md) | 2–4 hours | Undo and redo adding or deleting an object without changing the camera or losing identity. |
| [15 · Look through a perspective camera](journey/15-perspective.md) | 4–7 hours | View the scene in perspective and select surfaces with a ray that agrees with the camera. |
| [16 · Walk around the model](journey/16-orbit.md) | 2–4 hours | Orbit and tilt a perspective camera while keeping its target in place. |
| [17 · Bring a solid into the scene](journey/17-solid.md) | 3–5 hours | Create a kernel box, convert it to display data, and add it as one undoable scene object. |
| [18 · Read the shape through light](journey/18-light.md) | 1–2 hours | Shade the box faces according to their direction, using the same mesh and renderer. |
| [19 · Give every action the same route](journey/19-actions.md) | 3–5 hours | Move document actions into a browser-independent editor while keeping picking, undo and drawing working. |
| [20 · Keep a changing window in proportion](journey/20-resize.md) | 3–5 hours | Resize the drawing buffer, depth attachment and camera together, including on dense displays. |
| [21 · Remember a press until it ends](journey/21-gestures.md) | 3–5 hours | Orbit with a right drag, pick with a left click, and stop safely when the pointer or window loses focus. |
| [22 · Keep navigation on the mouse and commands in the dock](journey/22-shortcuts.md) | 2–4 hours | Normalize wheel input and keep every keyboard feature command in the command dock. |
| [23 · Keep the document behind the picture](journey/23-import.md) | 5–8 hours | Import a real mesh-session file, keep its source identity, and undo the whole import as one action. |
| [24 · Find the whole scene](journey/24-fit.md) | 3–5 hours | Frame all current objects without rotating them or changing the document. |
| [25 · Choose how depth changes size](journey/25-projection.md) | 3–5 hours | Switch between perspective and orthographic views while keeping fitting, zoom and picking coherent. |
| [26 · Frame one object without changing its size](journey/26-selected.md) | 1–2 hours | Run Fit Selected while keeping geometry, selection and document history unchanged. |
| [27 · Give each object a placement](journey/27-placement.md) | 1–2 hours | Keep local mesh coordinates and an independent object placement. |
| [27a · Ask geometry questions in world coordinates](journey/27a-world.md) | 1–2 hours | Use placement for scene bounds, selected bounds and ray picking. |
| [27b · Apply object placement on the GPU](journey/27b-model.md) | 1–2 hours | Send a separate model matrix for each draw while retaining local vertex buffers. |
| [27c · Move a placed object with a typed offset](journey/27c-move.md) | 1–2 hours | Translate selected geometry through the existing command and history route. |
| [27d · Prove placement and history agree](journey/27d-history.md) | 1–2 hours | Check that Move changes world placement, preserves local geometry, and remains one reversible transaction. |
| [28 · Prepare a display from an owned source mesh](journey/28-record.md) | 1–2 hours | Give source geometry a shared owner and prepare its display before committing an object. |
| [28a · Retain the imported mesh behind each row](journey/28a-imported.md) | 1–2 hours | Insert imported source geometry and its prepared display together. |
| [28b · Give generated objects the same source owner](journey/28b-generated.md) | 1–2 hours | Build demo triangles and Example Box from kernel geometry; derive all displays on insertion. |
| [28c · Prove source ownership survives editing](journey/28c-ownership.md) | 1–2 hours | Connect local source geometry, display caches, provenance and document history. |

The opening lessons separate browser events, the command dock, application state and GPU drawing. Before adding a camera or editable objects, follow this path without the listing: **typed command → state change → drawing commands → picture**.

Each lesson has one visible goal, a diagram, exact code changes, an experiment and a question. Do not rush through code you cannot connect to that goal. [Rust foundations](foundations.md) are available when a language idea needs more practice; you do not have to complete eight separate exercises before seeing a canvas.

## The destination

The final project must retain the current viewer’s features and crafted appearance. The small triangle is the first milestone. It is not the final product, and the later courses must continue the same handwritten project.

| Course | Destination | Availability |
| --- | --- | --- |
| 1 · A small viewer you understand | Mesh, camera, input and object identity. | Available through lit solids, orbit, pointer gestures, focused shortcuts, resizing, mesh import, scene fitting and projection choices; advanced picking still pending. |
| 2 · Geometry that reads clearly | Curves, points, CAD faces, boundaries, normals and text. | Pending conversion. |
| 3 · Documents that stay reliable | Loading, streaming, instancing, sheets and resource lifetimes. | Pending conversion. |
| 4 · Editing that can be undone | Selection, gestures, snapping, gumball, panels and transactions. | Basic selection, undo/redo and fallible transactions available; advanced tools pending. |
| 5 · The modeling toolbox | Every existing command, shape, surface, annotation and splitting tool. | Pending conversion. |
| 6 · The rendering quality we have now | Finite visibility, clipping caps, outlines, ambient occlusion and transparency. | Pending conversion. |
| 7 · Prove the complete viewer | Behavior, visual quality, responsiveness and resource checks. | Pending conversion. |

[The feature and quality contract](journey/destination.md) maps all 43 existing sections to these courses. A source inventory records all 271 current implementation/test files and the command implementations. No feature is considered delivered merely because it appears in this table.

The [complete implementation reference](reference-course.md) remains available during conversion. Its large sections have not become short lessons by being regrouped. They use a different starting point; do not jump from the current last lesson into reference section 00 and overwrite your project. The continuous bridge is still work to be done.

## Keep your place

Before an experiment, save your files. After it, write three lines: what changed; which values crossed a file boundary; what still puzzles me. You should be able to return tomorrow without reconstructing the whole lesson in your head.

[Checkpoint and recovery instructions](journey/recovery.md) preserve your own work. A corrected lesson should arrive as a focused change with a reason and a verification step, not a demand to restart. Releases keep their dependency lock and source fingerprints; the documentation labels unverified behavior.

[Prepare your computer](README.md#prepare-your-computer) · [Begin lesson 01](journey/01-canvas.md) · [Release evidence](journey/release.md)
