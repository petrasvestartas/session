#!/usr/bin/env python
# Narrate a viewer lesson with edge-tts (neural voice), per project_lesson_audio_narration.
#   python _tts_read.py 08-mvp.md 08-mvp.mp3 en-US-AvaNeural -6%
# Code blocks are replaced, in document order, by hand-authored DESCRIPTIONS below.
import sys, re, asyncio, edge_tts

SRC  = sys.argv[1] if len(sys.argv) > 1 else "08-mvp.md"
OUT  = sys.argv[2] if len(sys.argv) > 2 else "08-mvp.mp3"
VOICE= sys.argv[3] if len(sys.argv) > 3 else "en-US-AvaNeural"
RATE = sys.argv[4] if len(sys.argv) > 4 else "-6%"

# One plain-English line per fenced ``` block, per lesson, in document order.
import os
DESCRIPTIONS_BY_LESSON = {
    "08-mvp.md": [
        "In short: the clip space position equals projection, times view, times model, times the corner. Those three matrices multiply into one, which we call the m v p matrix.",
        "We touch two files: the shader, triangle w g s l, and gpu rs, where the matrix is built.",
        "In the shader, group zero binding zero is now a four by four matrix called m v p, instead of the old single float aspect. Group one binding zero is still the time value.",
        "The vertex shader now sets the output position to m v p times the corner position. This single matrix multiply replaces the old divide by aspect.",
        "At the top of gpu rs, we bring in Xform, Point, and Vector.",
        "We create the m v p buffer starting at the identity matrix, sixteen floats. Its layout is one uniform read by the vertex stage, and the bind group links it into group zero. Same wiring as before, only the buffer is bigger.",
        "Each frame in clear, after updating time, we build the camera matrix: a perspective projection using the current aspect ratio; a view that places the camera two units back, looking at the origin; and a model that rotates around the up axis by time. We multiply them, projection times view times model, cast the sixteen numbers to f32, and upload the result.",
        "In the render pass we bind group zero to the m v p bind group, and group one to the time bind group, just as before.",
        "To run it, change into session viewer and run trunk serve, then open local host, port eight seven seven zero.",
        "To recap: in chapter seven, group zero was a single float aspect and the shader divided x by it. In chapter eight, group zero is a four by four matrix, rebuilt every frame, and the shader multiplies the position by it, giving real model, camera, and perspective.",
    ],
    "09-projection.md": [
        "We touch two files: gpu rs, for the projection flag and the choice, and lib rs, for the space key.",
        "In the Gpu struct, we add a boolean field called perspective: true for perspective, false for orthographic.",
        "In the returned struct, we initialise perspective to true, so the viewer starts in perspective.",
        "In clear, we pick the projection from the flag: if perspective is true we build a perspective matrix; otherwise an orthographic matrix sized by the aspect ratio. View and model are unchanged.",
        "We add a toggle projection method that simply flips the boolean.",
        "In lib rs we extend the winit imports: add Element State, and bring in Key and Named Key.",
        "We add a keyboard input arm: on a Space key press that is not an auto repeat, we call toggle projection.",
        "To run it, change into session viewer and run trunk serve, then open local host, port eight seven seven zero.",
        "To recap: chapter eight always used a perspective matrix. Chapter nine chooses perspective or orthographic from a flag, and the Space key toggles it.",
    ],
    "10-orbit-camera.md": [
        "The eye position equals the target plus distance, times a direction built from yaw and pitch: cosine pitch times sine yaw, then sine pitch, then cosine pitch times cosine yaw.",
        "We touch two files: gpu rs, for the camera angles and the view, and lib rs, for the mouse.",
        "In the Gpu struct, we add three float fields: yaw, pitch, and distance.",
        "We initialise them to a three-quarter view: yaw zero point six, pitch zero point five, distance three.",
        "We add an orbit method that nudges yaw and pitch by the mouse delta — clamping pitch so it can't flip at the poles — and a zoom method that scales distance.",
        "In clear, we build the eye from yaw, pitch and distance around the target, feed it to look-at-right-handed for the view, and drop the spin: the model is now identity, so the camera moves, not the object.",
        "In lib rs, we extend the winit imports with Mouse Button and Mouse Scroll Delta.",
        "We add two fields to the App struct: orbiting, a boolean, and last cursor, the previous mouse position.",
        "We initialise them when the app is built: orbiting false, last cursor zero zero.",
        "We add three event arms: mouse input sets orbiting true while the right button is held; cursor moved, when orbiting, sends the delta to orbit; mouse wheel sends the scroll amount to zoom.",
        "To run it, change into session viewer and run trunk serve, then open local host, port eight seven seven zero.",
        "To recap: chapter nine had a fixed eye and a spinning model. Chapter ten computes the eye from yaw, pitch and distance — right-drag orbits, scroll zooms — and the model is identity.",
    ],
    "11-pan.md": [
        "The camera's right vector is cosine yaw, zero, minus sine yaw; the up vector is minus sine pitch times sine yaw, then cosine pitch, then minus sine pitch times cosine yaw. Panning adds minus dx times right, plus dy times up, scaled by distance.",
        "We touch two files: gpu rs, for the target and pan, and lib rs, for the Control modifier.",
        "In the Gpu struct, we add a target field: an array of three floats.",
        "We initialise the target to the origin, zero zero zero.",
        "In clear, we build the eye relative to the target instead of the origin, then look from eye to target as before.",
        "We add a pan method: it builds the camera's right and up vectors from yaw and pitch, and shifts the target along them by the mouse delta, scaled by distance — so panning is faster when zoomed out.",
        "In the App struct we add a ctrl flag, initialised false.",
        "We add a modifiers-changed arm that records whether Control is held.",
        "In the cursor-moved arm, while dragging we branch: with Control held we pan, otherwise we orbit.",
        "To run it, change into session viewer and run trunk serve, then open local host, port eight seven seven zero.",
        "To recap: chapter ten orbited a fixed target at the origin; chapter eleven stores the target, and Control plus right-drag moves it along the camera's right and up axes, with the eye following.",
    ],
    "12-depth-buffer.md": [
        "A depth buffer is a second texture, one number per pixel: the depth of the nearest surface drawn so far. Each new fragment is kept only if it's nearer than what's stored, so distance, not draw order, decides what's visible. We clear it to one, the far plane, and keep a fragment when its depth is less than what's stored. The format is depth thirty-two float. There's a precision trick called reverse-Z, but the viewport version of it is rejected by WebGPU, so we'll come back to it later with the camera.",
        "We touch two files: build r s, where the pipeline gains a depth test, and gpu r s, where we make the depth texture, attach it, clear it, and add a second triangle.",
        "In the Gpu struct, we add a depth view field, a w g p u texture view.",
        "We add a helper, create depth view, that makes a depth thirty-two float texture the size of the surface, marked as a render attachment, and returns a view of it.",
        "In new, after configuring the surface, we build the depth view from the device and config, and add it to the returned struct.",
        "In resize, after reconfiguring the surface, we rebuild the depth view so it always matches the new canvas size. Note it uses self dot device and self dot config, because in resize those are fields, not local variables.",
        "In build r s, the pipeline's depth stencil is no longer none. It's a depth thirty-two float state with depth writing on, and the compare function set to Less — keep the nearer fragment, the one with the smaller depth.",
        "In the render pass, inside the clear method, we attach the depth view and clear it to one, the far plane, then store the result.",
        "We replace the single triangle with two triangles at different depths in space: an orange one at z plus zero point three, drawn first and in front, and a blue one at z minus zero point three, drawn second and behind. Remember to point the vertex buffer and the vertex count at the new six-vertex constant.",
        "To run it, change into session viewer and run trunk serve, then open local host, port eight seven seven zero. Orbit around and the orange triangle stays correctly in front of the blue one from every angle. Set the compare function to Always, or drop the depth attachment, and the blue triangle, drawn last, wrongly jumps in front — that's the bug the depth buffer fixes.",
        "To recap: chapter eleven drew one triangle, where the last one drawn always won. Chapter twelve adds a depth texture and a Less depth test, cleared to the far value one, so the nearer fragment wins no matter the draw order.",
    ],
    "13-camera-module.md": [
        "Before, the Gpu struct owned both the graphics card and the camera — yaw, pitch, distance, target, and the orbit, pan, and zoom methods. After, a Camera struct owns all that math and the view projection, the Gpu is pure graphics again, and State owns both and wires them together.",
        "We touch four files: a new camera r s for the Camera struct; lib r s, to declare the module and route input to the camera; state r s, where State owns the camera and builds the view projection; and gpu r s, which loses the camera fields and methods.",
        "The new camera r s holds the camera state and the math, lifted straight out of gpu r s, unchanged. It has orbit, pan, zoom, and toggle projection, plus a view projection method that builds projection times view for a given aspect ratio: perspective or orthographic, then the eye from yaw, pitch and distance, looking at the target.",
        "At the top of gpu r s, the camera math is gone, so we only need Xform now, not Point and Vector.",
        "The clear method no longer computes the matrix; it's handed one. We change its signature to take a view projection reference, and replace the whole camera-matrix block with a single upload of that matrix.",
        "State gains a camera field, built with Camera new. In render, we read the surface aspect ratio, ask the camera for its view projection, and hand it to the GPU's clear.",
        "In lib r s, we declare the new module: mod camera.",
        "The input arms used to call state dot gpu dot orbit and so on. We point them at state dot camera instead — four call sites: toggle projection, pan, orbit, and zoom.",
        "To run it, change into session viewer and run trunk serve, then open local host, port eight seven seven zero. It looks identical to chapter twelve — orbit, pan, zoom, and Space all still work — but gpu r s no longer mentions the camera.",
        "To recap: in chapter twelve the Gpu owned the camera and the graphics card together. In chapter thirteen, the Camera owns the math, State owns the camera and the Gpu and feeds the view projection down, and the Gpu is pure device, surface, and pipelines again.",
    ],
    "14-named-views.md": [
        "The seven canonical views as yaw and pitch angles: Front is yaw zero, pitch zero; Back is yaw one hundred eighty degrees; Left and Right are yaw minus and plus ninety degrees; Top and Bottom are pitch near plus and minus ninety degrees; and Iso is yaw forty-five degrees, pitch thirty-five degrees. The C key resets home.",
        "We touch two files: camera r s, for a View enum plus set view and reset; and lib r s, to bind the digit keys one through seven and the C key.",
        "Above the impl block we add a View enum with the seven views. Inside set view, a match picks the two angles for each view, and the last line switches perspective off — a named view is orthographic. Top and bottom sit a hair under ninety degrees, because dead-on vertical is a gimbal pole we fix later with the quaternion camera.",
        "Reset goes home in one line: it overwrites self with Camera new, restoring the starting angles, distance, target, and perspective.",
        "At the top of lib r s, we bring the View enum into scope.",
        "We widen the keyboard arm from a single Space check into a match on the pressed, non-repeat logical key: Space still toggles projection; digits one through seven snap to the seven views; and C, upper or lower case, resets home.",
        "To run it, change into session viewer and run trunk serve, then open local host, port eight seven seven zero. Tap one through seven to snap through the views — each jumps to orthographic — and press C to fly home.",
        "To recap: chapter thirteen gave the Camera its yaw, pitch, distance and target, driven by hand with orbit, pan and zoom. Chapter fourteen adds named presets — set view snaps the angles and switches to ortho, reset goes home — wired to keys one through seven and C. No new matrix, just known angles.",
    ],
    "16-projection-polish.md": [
        "Three fixes, all inside view projection. In perspective, the near plane is distance times zero point zero zero one and the far plane is distance times one hundred, so the far-to-near ratio stays pinned near one hundred thousand at every zoom. In orthographic, the half-height is distance times the tangent of thirty degrees — exactly what perspective shows at the target plane. And the unit, millimetres or metres, is an enum the camera carries: millimetres scales by zero point zero zero one, metres by one, and that scale is baked into view projection.",
        "We touch three files: camera r s, for a Unit enum plus a unit field and a toggle, the rewritten view projection, and a unit-aware fit; gpu r s, to put the demo triangles into millimetres; and lib r s, for the scene bounds in millimetres and the U key that switches the unit.",
        "Above the impl block we add a Unit enum with two variants, Millimeters and Meters. Each one knows its scale to metres through a to-meters method: millimetres returns zero point zero zero one, metres returns one. Adding centimetres or inches later is a single line. We also add a set unit method, so the API can fix the unit in code when it loads a model — it takes any variant, so it never needs editing when you add more.",
        "We give the Camera struct a unit field, and initialise it to millimetres in new — the default for session geometry. To default to metres instead, change that one value, or call set unit from your API.",
        "In view projection, perspective now uses an adaptive near and far, both proportional to distance, so the depth ratio is constant at every zoom — we have no reverse-Z yet, so a constant ratio is what keeps precision stable. Orthographic sets its half-height to distance times the tangent of thirty degrees, matching what perspective shows at the target, so toggling no longer jumps the apparent size, and because the half-height tracks distance, scrolling now zooms in ortho too. The eye, target and view are unchanged. Finally we read the unit's scale to metres, build a scale matrix from it, and return projection times view times scale — so the geometry is converted to metres before the camera sees it.",
        "Fit follows the unit. The box arrives in the geometry's authoring unit, but the camera's target and distance are in metres, so we multiply the centre and the radius by the very same to-meters scale. Everything else in fit is unchanged.",
        "In gpu r s, we multiply every triangle coordinate by one thousand, so the two triangles are now expressed in millimetres — a roughly one-point-four metre wide scene. After the zero point zero zero one scale, they project to exactly the same size on screen as before.",
        "In lib r s, we scale the scene minimum and maximum by one thousand as well, so the bounds match the millimetre triangles.",
        "To run it, change into session viewer and run trunk serve, then open local host, port eight seven seven zero. Space no longer jumps the size between perspective and ortho, and scrolling finally zooms in ortho. Orbiting far out and back, the orange triangle stays cleanly in front of the blue one at every distance — the adaptive near and far holding depth precision steady. To render a model in metres instead, set Unit Meters in new, or call set unit from your API — nothing else changes.",
        "To recap: chapter fifteen left three loose ends — switching projection jumped the size, ortho ignored zoom, and the world had no real unit. Chapter sixteen fixes all three inside view projection: an adaptive near and far for constant depth precision, an ortho half-height derived from distance for a seamless, zoomable switch, and a Unit enum, millimetres or metres, whose scale to metres the camera bakes in. The unit is fixed in code through the new default or set unit, and fit honours it.",
    ],
    "17-quaternion-camera.md": [
        "The orientation quaternion is the camera's frame. Apply it to the basis vectors and you get the axes directly: right is the orientation applied to one, zero, zero; up is the orientation applied to zero, zero, one; forward is the orientation applied to zero, one, zero. The eye is the target minus distance times forward. No cross products, no degeneracy — a quaternion has no pole.",
        "Yaw rotates the orientation about world up, plus Z — that keeps the horizon level, which makes it a turntable rather than a free trackball. Pitch rotates about the camera's own right. And each named view is a single from-axis-angle rotation, with up coming out correct for free, because a Z rotation leaves plus Z fixed.",
        "We touch a single file: camera r s. The quaternion orientation replaces yaw and pitch, but every public method keeps its signature, so nothing else in the project changes.",
        "We bring in Quaternion. The struct drops yaw and pitch and gains an orientation quaternion and a world up of plus Z, plus two derived fields, position and up, that update position fills in. Notice there is no last right — the quaternion makes it unnecessary. In new, we build an isometric start: yaw forty-five degrees about plus Z, then pitch minus thirty degrees about the tilted right axis, multiply and normalise. We set the fields, then call update position.",
        "Update position is now tiny. The orientation already holds a complete frame, so we just apply it to the basis vectors: forward is orientation times zero, one, zero, and up is orientation times zero, zero, one. We place the eye behind the target along forward, and store up. There is no pole handling, because the quaternion has no pole.",
        "Orbit composes quaternions. Yaw turns about world up, pitch turns about the camera's own right, which is the orientation applied to one, zero, zero. We multiply both onto the current orientation and normalise. No pitch clamp, no pole code — yawing about world up keeps the horizon level by itself.",
        "Pan slides the target along the camera's right and the cached up, scaled by distance, then refreshes the derived frame. Zoom scales distance, clamped, and also refreshes. Both just call update position afterwards.",
        "View projection is unchanged from chapter sixteen except for the eye block: it now reads the cached position and up that update position computed, instead of rebuilding them from yaw and pitch. Look at right handed builds the view, and we still apply the unit scale before returning projection times view times scale.",
        "Each named view sets the orientation to a single rotation. Because up is read straight from the quaternion, every view comes out upright with no special casing — a Z rotation, for Front, Back, Left and Right, leaves plus Z fixed, so up stays plus Z automatically. We switch to orthographic and update the position.",
        "Reset is still one line — new rebuilds the isometric orientation. Fit is the chapter-sixteen logic, but since the camera state is now f64, we drop every cast to f32: we write the target and distance directly, and cast the f32 bounding box to f64 where it enters. And we add a call to update position at the end, because fit writes the target and distance directly.",
        "To run it, change into session viewer and run trunk serve, then open local host, port eight seven seven zero. Orbit straight up and over the top: where chapter fourteen jammed under the pole, the camera now sails over cleanly with no flip — a quaternion simply has no pole. The world is Z up, so press five for Top to look straight down plus Z, and one for Front to look along plus Y. Pan, zoom, fit and reset all behave as before, because their signatures never changed.",
        "To recap: through chapter sixteen the orientation was a yaw and pitch pair with a pitch clamp — a gimbal singularity at the poles. Chapter seventeen stores the orientation as a quaternion that encodes the whole frame; update position reads right, up and forward straight off it, with no pole and no last right. Orbit composes yaw and pitch quaternions, and named views are single rotations, upright for free. It's a Z up turntable, and only camera r s changed.",
    ],
    "18-index-buffer.md": [
        "A draw call has two modes. Plain draw walks the vertex buffer straight through — fine for a throwaway triangle, wasteful when corners are shared. Draw-indexed walks an index buffer instead: the vertices hold the eight unique corners stored once, and the indices are thirty-six small integers that say which corner to use, assembling twelve triangles. The GPU fetches and caches each vertex, so a shared corner's vertex shader runs about once. The pipeline and shader don't change, because indexing happens before the vertex shader.",
        "We touch a single file, gpu r s. The pipeline and the triangle shader are untouched — indexing is invisible to them.",
        "We replace the six triangle vertices with eight cube corners, plus a thirty-six entry index list. The cube is one metre wide expressed in millimetres, with a half-size of five hundred, so it sits nicely at the camera's start distance, and each corner gets a distinct colour so the faces read as gradients. The index list is twelve triangles, two per face, each face's four corners split into two triangles. Winding doesn't matter yet — the pipeline's cull mode is none, so every face draws and the depth test sorts them.",
        "The Gpu struct had a vertex buffer and a vertex count. We swap the count for an index buffer and an index count, the u16 indices into the vertex buffer.",
        "In new, we build the vertex buffer from the cube corners as before, and add the index buffer with index usage, casting the u16 slice to bytes. The index count is thirty-six. We return the index buffer and count in the struct instead of the vertex count.",
        "In clear, after binding the vertex buffer, we bind the index buffer with its format — u-int sixteen, since our indices fit in sixteen bits — and switch draw to draw-indexed. Draw-indexed takes the index range, a base vertex of zero that is added to every index, and the single instance range, zero to one.",
        "To run it, change into session viewer and run trunk serve, then open local host, port eight seven seven zero. A solid colour cube replaces the two triangles. Orbit it with the right mouse, and the quaternion camera sails around while the depth test keeps near faces in front. The digit keys snap to named views, so now you can actually see Top versus Front on a real solid. It's drawn from eight vertices and thirty-six indices, not thirty-six vertices.",
        "To recap: chapter seventeen finished the camera, but the geometry was still six flat triangle vertices. Chapter eighteen adds an index buffer, which separates what the corners are — eight vertices — from how they connect — thirty-six indices. Draw-indexed walks the index list, the shader and pipeline are unchanged, and this is exactly the vertex-buffer plus index-buffer pair the kernel's GpuMesh hands us next.",
    ],
    "19-link-the-kernel.md": [
        "The flow is one bridge. The session-rust Mesh is f64, a half-edge mesh — the modeling truth. Calling mesh dot gpu-mesh, passing the device, flattens it once into f32 render vertices, uploads them, and caches a GpuMesh holding a vertex buffer, an index buffer, and an index count. That GpuMesh is f32 and lives on the GPU, reused every frame and rebuilt only on edit. The pipeline declares the kernel's render-vertex layout — position at zero, normal at one, color at two, stride forty — and we set the vertex buffer, set the index buffer as u-int thirty-two, and draw indexed.",
        "We touch three files: build r s, to point the pipeline at the render-vertex layout; the triangle shader, to read the render vertex's color; and gpu r s, to hold a mesh and draw it. Cargo already lists session rust from the camera chapters, so there's nothing to add there.",
        "The hand-rolled Vertex struct and its bytemuck import are gone — the kernel's render vertex is the format now, and it declares its own layout. We delete the struct, import render vertex, and point the pipeline's vertex buffers at render-vertex layout instead of the old Vertex layout.",
        "Render vertex puts color at location two, a four-component vector, with the normal at location one. We don't shade yet, so the vertex shader just passes color through, but its input must match the new layout. Two lines change: the color input moves to location two as a vec4, and we take its r-g-b when passing it on. A shader may consume a subset of a layout's attributes, so skipping the normal at location one is fine — we pick it up when we add lighting.",
        "In gpu r s we update the imports: drop the local Vertex, and bring in Mesh and Color alongside Xform.",
        "The Gpu struct replaces its three buffer fields — vertex buffer, index buffer, and index count — with a single mesh field.",
        "In new, where the cube and its buffers used to be, we build a one-metre box authored in millimetres with create-box, and set its object color to a blue that's visible on the grey background. A new mesh defaults to object-color mode, and the flattener honors that mode, so the object color shows. Then we return the mesh in the struct instead of the buffer fields.",
        "In clear, we ask the mesh for its GpuMesh. The first call flattens, uploads, and caches; every later frame returns the same cached buffers. We set the pipeline and bind groups as before, bind the GpuMesh's vertex and index buffers — the index buffer as u-int thirty-two now, since kernel indices are u32 — and draw indexed over the index count. The call borrows the mesh mutably and the device shared, which are different fields, so it compiles inside the method.",
        "To run it, change into session viewer and run trunk serve, then open local host, port eight seven seven zero. A blue box appears — but this one is a genuine session-rust Mesh, the same type the kernel builds NURBS, booleans and file-loaded geometry into. Orbit, fit, and named views all work. Swap create-box for cube, a sphere, or any kernel constructor and it just draws, because everything now flows through gpu-mesh. It's a flat solid for now; normals and lighting come in lesson twenty-one, and the normals are already in the buffer at location one.",
        "To recap: chapter eighteen drew a cube from a hand-typed vertex and index buffer. Chapter nineteen gets those buffers from the kernel — mesh dot gpu-mesh flattens the f64 Mesh to f32 once, caches a GpuMesh of vertex buffer, index buffer and index count, and we draw it indexed with u-int thirty-two. The pipeline uses render-vertex layout; there is no cast-slice, no offsets, no as-f32 in the viewer. f64 to compute, f32 to draw.",
    ],
    "20-grid.md": [
        "One render pass, one depth buffer, but two pipelines this frame. The grid pipeline is line-list, with depth-writes off, and it draws first — the floor that never occludes. The triangle pipeline is triangle-list, depth-writes on, and draws second — the solid box, which paints over the grid. Both bind group zero, the camera matrix; the grid skips group one, the time. Drawing the grid first with depth-writes off lays down floor pixels but leaves the depth buffer cleared, so the box drawn next paints over it wherever it covers screen.",
        "We touch four files: a new grid shader; build-grid-pipeline in build r s; a grid field added to Pipelines in mod r s; and a grid vertex buffer plus a draw call in gpu r s.",
        "The grid shader is a stripped-down triangle shader: it reads the camera matrix and a per-vertex color, and passes the color straight through — no time, no pulse. We reuse the kernel's render vertex, so the color sits at location two as r-g-b-a, and we read the r-g-b prefix, exactly like the triangle shader does.",
        "Build-grid-pipeline copies the triangle pipeline and changes exactly three things: the shader, the topology to line-list, and depth-write-enabled to false. It binds only group zero, the camera — no time layout — and reuses the render-vertex layout.",
        "In mod r s we add a grid field to the Pipelines struct and build it. New already receives the aspect layout, so its signature doesn't change.",
        "In gpu r s, first add render vertex to the session-rust import.",
        "Then add two fields to the Gpu struct: a grid buffer, and a grid count.",
        "In new, next to where the mesh is built, generate the lines and upload them. We author in millimetres — the camera's m-v-p already applies the millimetre-to-metre scale — so a plus-or-minus five-thousand-millimetre grid stepping every thousand millimetres is a plus-or-minus five metre floor with one-metre cells. The two lines through the origin are tinted, X red and Y green, so you can read the axes. Each step adds two lines, four vertices; eleven steps give forty-four vertices, which a line-list draws as line segments.",
        "In clear, inside the render pass and before the mesh draw, run the grid pipeline. It binds only group zero, its own vertex buffer, and a plain draw, since lines aren't indexed. The existing mesh draw stays right after it.",
        "To run it, change into session viewer and trunk serve, then open local host port eight seven seven zero. The blue box now sits on a grey grid with red and green axes. Orbit, and the grid anchors the motion — you can tell you're circling a ground plane. The box is centred on the origin, so it straddles the plane, and reads as solid because the grid never writes depth.",
        "To recap: chapter nineteen drew a kernel mesh with one pipeline. Chapter twenty adds a second pipeline, the grid shader, sharing the same pass — line-list topology, depth-writes off, drawn first so the solid box paints over it. Same camera in group zero, no time. Render vertex is reused for the lines, so there's no new vertex type. This is the overlay pattern we'll reuse for edges and gizmos.",
    ],
    "21-mesh-shading.md": [
        "Shading needs a normal per pixel, and there are two ways to get one. Per-vertex normals, stored in the render vertex at location one, give smooth shading — that's the next lesson. A per-face normal, reconstructed in the fragment shader from the screen-space derivatives, gives flat shading — that's this lesson, and it needs no vertex normals at all.",
        "The vertex shader passes the world position through. In the fragment shader, the normal is the normalized cross product of the y-derivative and the x-derivative of that position — the per-face normal. Then the light model sums a hemisphere ambient term, plus a key light and a fill light, each the surface normal's dot with a light direction, clamped at zero. The output is the base color times that lit value.",
        "We touch a single file: the mesh's shader, triangle w g s l. Pass the world position through, and light the fragment — that's the only change. The pipeline, buffers, camera, and grid all stay exactly as they are.",
        "In the shader, the vertex output gains a world-position field at location one. The vertex shader sets it to the incoming position — the box has no per-object transform yet, so model space is world space — while still projecting the position with the m-v-p matrix and passing the color through.",
        "The fragment shader computes the normal first, before any branch, because derivatives must run in uniform control flow. The normal is the normalized cross product of the y-derivative and the x-derivative of the world position. Because WebGPU's framebuffer y points down, that order gives an outward normal; the reversed order would point inward, and nothing would light. We flip the normal for back faces. Then two fixed world-space lights: a key at strength zero point six five and a fill at zero point three, each the clamped dot of the normal with the light direction. Plus a hemisphere ambient that blends from a darker ground to a lighter sky along world plus-Z, the up axis here. The final color is the base color times the summed light.",
        "To run it, change into session viewer and run trunk serve, then open local host, port eight seven seven zero.",
        "To recap: chapter twenty left the box flat — one color, no shape. Chapter twenty-one shades it. The vertex shader passes the world position; the fragment shader builds a per-face normal as the normalized cross of the y and x derivatives — that order for an outward normal under WebGPU's y-down framebuffer — flips it for back faces, and lights it with a hemisphere ambient plus a key and fill term. Base color times lit. One file changed, and no vertex normals needed.",
    ],
}
DESCRIPTIONS = DESCRIPTIONS_BY_LESSON.get(os.path.basename(SRC), [])

text = open(SRC, encoding="utf-8").read()

# Split out fenced code blocks, replacing each with its description (in order).
parts = re.split(r"```.*?\n.*?```", text, flags=re.S)
blocks = re.findall(r"```.*?```", text, flags=re.S)
print(f"[tts] {len(blocks)} code blocks found, {len(DESCRIPTIONS)} descriptions provided")
out = []
for i, seg in enumerate(parts):
    out.append(seg)
    if i < len(blocks):
        out.append("\n\n" + (DESCRIPTIONS[i] if i < len(DESCRIPTIONS) else "See the code in the lesson.") + "\n\n")
text = "".join(out)

SYM = {"×":" times ","÷":" divided by ","→":" then ","←":" ","–":", ","—":", ","•":", ",
       "§":"section ","&":" and ","≈":" about ","≥":" at least ","°":" degrees ","·":" times "}

clean = []
for line in text.splitlines():
    s = line.rstrip()
    if not s.strip():
        clean.append("")                      # paragraph break
        continue
    if re.match(r"^\s*\|?\s*-{3,}", s):        # --- / table separators
        continue
    if s.lstrip().startswith("|"):             # table rows (none expected)
        continue
    s = re.sub(r"^#+\s*", "", s)               # heading hashes
    s = re.sub(r"^\s*>\s?", "", s)             # blockquote markers
    s = re.sub(r"^\s*[-*]\s+", "", s)          # list bullets
    s = s.replace("`", "")                     # keep inline-code text, drop backticks
    s = re.sub(r"\*\*(.+?)\*\*", r"\1", s)     # bold
    s = s.replace("_", " ").replace("::", " ").replace("/", " ")
    for k, v in SYM.items():
        s = s.replace(k, v)
    clean.append(s.strip())

# Reflow: join consecutive non-empty lines into one paragraph (pauses on sentence ends).
paras, buf = [], []
for s in clean:
    if s == "":
        if buf: paras.append(" ".join(buf)); buf = []
    else:
        buf.append(s)
if buf: paras.append(" ".join(buf))
speech = "\n\n".join(paras)

async def main():
    await edge_tts.Communicate(speech, VOICE, rate=RATE).save(OUT)
    print(f"[tts] wrote {OUT}")

asyncio.run(main())
