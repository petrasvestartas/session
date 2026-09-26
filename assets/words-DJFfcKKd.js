const e={title:"Words before code",html:`<h1 id="words-before-code">Words before code<a class="anchor" href="#/course/words#words-before-code" aria-label="Link to this section">#</a></h1>
<p>Every word the lessons use before they have room to explain it, with the lesson that first needs it and the file or call where it appears.</p>
<p>Nothing here is needed for lesson 00. Do not read this page through: come back when a lesson names a section, and look one word up.</p>
<h2 id="two-sides-and-a-wire">Two sides and a wire<a class="anchor" href="#/course/words#two-sides-and-a-wire" aria-label="Link to this section">#</a></h2>
<p><img src="/session/docs/course/docs/illustrations/cpu-gpu.svg" alt="The CPU side you may read and change at any time; the GPU side you send bytes and commands to and cannot read back casually. Between them is a narrow wire, and almost every mistake in the course is on it." loading="lazy" decoding="async"></p>
<ul>
<li><strong>CPU side</strong>: your Rust structs, the kernel documents, the scene, input events, object ids. You can read and change them at any time.</li>
<li><strong>GPU side</strong>: buffers, textures, bind groups, pipelines, shaders. You cannot read them back casually; you <em>send</em> bytes and commands.</li>
<li><strong>The wire</strong>: <code>queue.write_buffer</code>, <code>create_buffer_init</code>, the vertex layout, the bind-group layout, the shader&#39;s <code>@location</code>/<code>@binding</code> declarations. </li>
<li>Ask of every object: <em>CPU or GPU? Made once or every frame? Who owns it?</em> The answers are the architecture.</li>
</ul>
<h2 id="made-once-lesson-01">Made once (lesson 01)<a class="anchor" href="#/course/words#made-once-lesson-01" aria-label="Link to this section">#</a></h2>
<ul>
<li><strong>instance</strong> — the entry to WebGPU, made from nothing (<code>wgpu::Instance::new</code>). It finds adapters and makes surfaces. <code>Backends::BROWSER_WEBGPU</code> means the browser&#39;s WebGPU only.</li>
<li><strong>adapter</strong> — one GPU as the browser offers it. You ask for one that can draw to your canvas (<code>compatible_surface</code>); nothing is drawn through it.</li>
<li><strong>device</strong> — your open connection to that GPU. Every buffer, texture, shader and pipeline is created through it.</li>
<li><strong>queue</strong> — the device&#39;s inbox: <code>write_buffer</code> and <code>submit</code> go there, and the GPU runs them in order.</li>
<li><strong>surface</strong> — the canvas as wgpu sees it. Each frame it hands out one texture to draw into (<code>get_current_texture</code>); <code>present</code> hands it back to the browser to show.</li>
<li><strong>surface configuration</strong> — what textures the surface hands out: pixel format, size in physical pixels, present mode. Redone on resize (<code>surface.configure</code>), never per frame.</li>
</ul>
<h2 id="memory-01-03-04a">Memory (01, 03, 04a)<a class="anchor" href="#/course/words#memory-01-03-04a" aria-label="Link to this section">#</a></h2>
<ul>
<li><strong>buffer</strong> — bytes on the GPU with fixed <em>usages</em> chosen at creation: <code>UNIFORM</code> (a small block every invocation reads), <code>STORAGE</code> (a big array a shader indexes), <code>VERTEX</code> / <code>INDEX</code> (what a draw pulls), <code>COPY_DST</code> (the CPU may write into it later), <code>COPY_SRC</code> (the GPU may copy out of it). Using a buffer a way it was not created for is a validation error, at the call, not at creation.</li>
<li><strong>uniform buffer</strong> (01) — the camera matrix: 64 bytes, same for every vertex.</li>
<li><strong>storage buffer</strong> (03) — <code>array&lt;Instance&gt;</code>: one row per object, any length, indexed by the shader.</li>
<li><strong>vertex buffer / index buffer</strong> (04a) — fixed-size vertex records, and triangle corner numbers three per triangle so a shared corner is stored once (<code>draw_indexed</code>).</li>
<li><strong>texture</strong> — a grid of texels with a format: the surface&#39;s colour format; <code>Depth32Float</code> (04a, one depth per pixel); <code>Rg16Float</code> (05, the depth-gradient target; lesson 18 widens it to <code>Rgba16Float</code> to carry a triangle address too); <code>Rg32Uint</code> (12, pick ids); <code>R8Unorm</code> (11, glyph coverage; 12, the selection coverage mask; 17, the silhouette masks).</li>
<li><strong>texture view</strong> — the handle a pass draws into or a bind group reads. Passes attach views, not textures.</li>
<li><strong>attachment</strong> — a view a render pass writes: the colour attachment gets fragment colours, the depth attachment remembers the nearest depth per pixel.</li>
<li><strong>multisampling (MSAA)</strong> (04a, used from 05) — N colour and depth samples per pixel so a partly covered edge pixel gets a partial colour; pipelines and every attachment of a pass must share the sample count; the samples are <em>resolved</em> into the 1-sample surface at the end.</li>
</ul>
<h2 id="binding-01-03-04a">Binding (01, 03, 04a)<a class="anchor" href="#/course/words#binding-01-03-04a" aria-label="Link to this section">#</a></h2>
<ul>
<li><strong>bind group layout</strong> — the <em>shape</em> of a set of resources: group N has binding 0 of this kind, binding 1 of that kind, visible to these stages. A pipeline is compiled against shapes.</li>
<li><strong>bind group</strong> — the <em>filled</em> set: this buffer at binding 0, this view at binding 2. Plugged in at draw time with <code>set_bind_group</code>.</li>
<li><strong><code>@group(g) @binding(b)</code></strong> — the shader&#39;s name for one slot; it must agree with the layout and with what the bind group holds.</li>
<li><strong>the scene contract</strong> (04a onward, <code>src/shaders/scene.wgsl</code>) — group 0 the camera matrix, group 1 the per-frame line block, group 2 object rows and translations (plus the physical depth for ink), group 3 the lane&#39;s own. Every lane shader is compiled with that file appended (<code>pipelines::scene_module</code>).</li>
<li><strong>shader stage visibility</strong> — each layout entry lists the stages allowed to read it; a stage reading a binding it may not see fails at pipeline creation.</li>
</ul>
<h2 id="pipeline-01-04a">Pipeline (01, 04a)<a class="anchor" href="#/course/words#pipeline-01-04a" aria-label="Link to this section">#</a></h2>
<ul>
<li><strong>render pipeline</strong> — the frozen recipe for one kind of draw: which shaders, what vertex input, what output formats, blending, depth rule, sample count. Validated once, so every draw is cheap; rebuilt only when the sample count flips (<code>Gpu::retarget</code>).</li>
<li><strong>pipeline layout</strong> — the list of bind-group layouts the pipeline expects, in group order.</li>
<li><strong>entry point</strong> — the shader function a stage runs (<code>vs_main</code>, <code>fs_main</code>). One module can hold several; <code>PipelineDesc::with</code> and <code>vertex</code> pick them.</li>
<li><strong><code>PipelineDesc</code>, <code>build</code></strong> (04a, <code>src/engine/pipelines/mod.rs</code>) — the house form: one base per shader, variants by fragment entry, colour mode and depth mode; <code>build</code> is the only place a pipeline is created.</li>
<li><strong>depth mode</strong> — every compare is reverse-Z: nearer is <strong>greater</strong>. <code>Opaque</code> and <code>OpaqueEqual</code> write and test, <code>ReadOnly</code>/<code>ReadOnlyEqual</code> test only, <code>Always</code> neither, <code>Detached</code> has no depth attachment at all. The <code>*Equal</code> pair compares <code>GreaterEqual</code> so a draw can tie with its own prepass; the others compare <code>Greater</code>.</li>
<li><strong>colour mode</strong> — <code>Opaque</code> overwrites, <code>Blended</code> mixes by alpha, <code>Max</code> keeps the larger value (coverage masks), <code>Nothing</code> writes no colour (a pass run for its side effects).</li>
<li><strong>pipeline-overridable constant</strong> (<code>override SCENE_MSAA</code>, 04b) — a shader constant the Rust side sets at build time, so one WGSL source gives a 1x and a 4x variant.</li>
</ul>
<h2 id="a-frame-01">A frame (01)<a class="anchor" href="#/course/words#a-frame-01" aria-label="Link to this section">#</a></h2>
<ol>
<li>The browser asks for a frame (<code>RedrawRequested</code>); the app decides whether anything changed (<code>needs_frame</code>).</li>
<li><code>surface.get_current_texture()</code> gives this frame&#39;s texture; <code>create_view</code> gives the handle to draw into.</li>
<li><code>device.create_command_encoder()</code> starts an empty command list.</li>
<li><code>encoder.begin_render_pass(...)</code> opens a section aimed at given attachments; <code>LoadOp</code> says clear or keep, <code>StoreOp</code> says keep or drop.</li>
<li><code>set_pipeline</code>, <code>set_bind_group</code>, <code>set_vertex_buffer</code>, <code>draw</code> / <code>draw_indexed</code> record work; nothing runs yet.</li>
<li>The pass ends when it goes out of scope (the inner braces); <code>encoder.finish()</code> closes the list into a command buffer.</li>
<li><code>queue.submit</code> hands the list to the GPU; <code>present</code> shows the texture.</li>
</ol>
<ul>
<li><strong>vertex stage</strong> — runs <code>vs_main</code> once per vertex, must return a clip-space position; <code>@builtin(vertex_index)</code> and <code>@builtin(instance_index)</code> are the two counters a draw hands it.</li>
<li><strong>rasterizer</strong> — finds every pixel inside the triangle and blends the vertex outputs across it (interpolation). <code>@interpolate(flat)</code> takes one vertex&#39;s value unchanged: ids and flags must be flat.</li>
<li><strong>fragment stage</strong> — runs <code>fs_main</code> once per covered pixel (or per sample with <code>@builtin(sample_index)</code>); returns the colour for <code>@location(0)</code>; <code>discard</code> throws the fragment away, writing neither colour nor depth.</li>
<li><strong>clip space</strong> — what the vertex stage outputs: (x, y, z, w). After the divide by w, x and y in −1..1 are on screen and z in 0..1 is the depth; a vertex parked at (3, 3, 0.5, 1) is outside and its triangle vanishes (<code>dead_vertex</code>).</li>
</ul>
<h2 id="coordinates-02">Coordinates (02)<a class="anchor" href="#/course/words#coordinates-02" aria-label="Link to this section">#</a></h2>
<p><code>local (f64, kernel millimetres) → world (placement) → view (eye at the origin, −Z ahead) → clip → NDC (÷ w) → pixels</code></p>
<ul>
<li><strong>model / view / projection</strong> — placement of one object; the camera&#39;s own transform; perspective (divides by distance, converging lines) or orthographic (no divide, parallel lines stay parallel, what a CAD drawing is).</li>
<li><strong>near and far planes, frustum, field of view</strong> — only what lies between near and far is drawn; the frustum is the cut pyramid the screen sees; the vertical field of view is its opening angle.</li>
<li><strong>reverse-Z</strong> — depth 1 at the near plane, 0 at the far plane, clear value 0, compare <code>Greater</code>. It spends float precision where CAD needs it: near the eye.</li>
<li><strong>anchor / rebase</strong> (02, 04a) — the camera works in f64; the GPU rows are f32. Positions are stored relative to an <em>anchor</em> near the camera target and the anchor moves when the camera drifts far, so nothing jitters kilometres from the origin.</li>
<li><strong>CSS pixel vs physical pixel</strong> — page layout counts CSS pixels; a HiDPI screen has 2 (or 1.5, 3) physical pixels per CSS pixel: <code>devicePixelRatio</code>. The canvas draws in physical pixels; pens and markers are sized in CSS pixels and scaled.</li>
<li><strong>aspect ratio</strong> — width over height of the framebuffer; the projection needs it so a circle stays a circle.</li>
</ul>
<h2 id="shader-words-01-04">Shader words (01-04)<a class="anchor" href="#/course/words#shader-words-01-04" aria-label="Link to this section">#</a></h2>
<ul>
<li><strong>WGSL</strong> — the WebGPU Shading Language: small, C-like, no pointers, run in parallel per vertex or per fragment.</li>
<li><strong><code>let</code> / <code>var</code></strong> — immutable value / mutable variable. A <code>var</code> without an initializer is zero.</li>
<li><strong><code>vec3&lt;f32&gt;</code>, <code>vec4&lt;f32&gt;</code>, <code>mat4x4&lt;f32&gt;</code></strong> — vectors and a 4×4 matrix; <code>m * vec4(p, 1.0)</code> moves a point; the trailing 1.0 makes it a point, not a direction.</li>
<li><strong>swizzle</strong> — <code>.xyz</code> / <code>.rgb</code> picks components; <code>.xy</code> of a position is the pixel.</li>
<li><strong><code>select(f, t, cond)</code></strong> — the false value comes <strong>first</strong>: <code>select(a, b, c)</code> is <code>c ? b : a</code>.</li>
<li><strong><code>mix(a, b, t)</code></strong> — <code>a + (b − a) · t</code>.</li>
<li><strong><code>textureLoad</code></strong> — one texel at integer coordinates, no filtering.</li>
<li><strong><code>@builtin(position)</code></strong> — in the vertex stage the clip position you output; in the fragment stage the pixel centre in framebuffer pixels plus depth.</li>
<li><strong>alignment</strong> (03) — a <code>vec4</code> and each matrix column start on 16 bytes, a <code>vec3</code> too although it is 12 bytes wide, and a struct in an array is padded to its largest alignment, which makes the <code>Instance</code> row 96 bytes; <code>_pad</code> fields on the Rust side make the two agree.</li>
</ul>
<h2 id="house-words">House words<a class="anchor" href="#/course/words#house-words" aria-label="Link to this section">#</a></h2>
<ul>
<li><strong>row / instance</strong> (03) — one object on the GPU: a 96-byte record (<code>Instance</code>: model matrix, colour, flags, spacing). A pick returns a row; <code>Scene</code> turns it into a source identity.</li>
<li><strong>lane</strong> (04a) — one drawing family with its own buffers, pipelines and draw calls. The twelve the map names: <code>arena</code> (meshes), <code>faces</code> (17, source faces), <code>segments</code> (strokes), <code>glyphs</code> (markers), <code>cloud</code> + <code>splat</code> + <code>lod</code> (points), <code>text*</code>, <code>surface_outline</code>, <code>backdrop</code> (05), <code>pick</code> (12), <code>triangle_tiles</code> (18). Lanes do not reach into each other&#39;s buffers, except where one lane owns another outright — the arena owns the outline-text lane, the source-face lane and the tile pool. <code>Gpu</code> lists them by hand.</li>
<li><strong>upload</strong> (04a) — the typed rows one file produces, with no wgpu types in them; <code>Gpu::set_scene</code> appends them to the lanes, then the rows are dropped.</li>
<li><strong>walk / producer</strong> (06) — the CPU code that turns one kernel geometry into rows; one producer per geometry family in <code>src/app/walk/</code>.</li>
<li><strong>face pass / physical</strong> (04a, 05) — the first pass: solid faces write colour, depth and a depth-gradient. &quot;Physical&quot; means <em>this is what occludes</em>.</li>
<li><strong>print / print fill</strong> (04a, named again in 06, 17 and 18) — flat filled drawing ink rather than a surface: an imported PDF glyph, a filled region of a sheet. Recognised because the mesh broadcasts a single width of 0 (<code>is_print_fill</code>, <code>FLAG_PRINT</code>). It takes the sheet index runs, keeps its flat colour under the headlight, is never painted red as a back face, and <code>P</code> never x-rays it away — there is no inside to see through.</li>
<li><strong>ink</strong> (04b) — everything that is not a solid face: strokes, markers, lettering, drawn in the second pass, which reads the physical depth and decides visibility per fragment (<code>ink_visibility.wgsl</code>).</li>
<li><strong>pcurve</strong> (07) — a <em>parameter curve</em>: a trimmed face&#39;s boundary in the surface&#39;s own <code>u</code>,<code>v</code> domain, not in XYZ. Lifting it through the surface gives the 3D edge; mapping a shared XYZ edge back onto each face&#39;s pcurve is how two faces agree where it lies.</li>
<li><strong>grid face / constrained face</strong> (07) — which mesher produced a face. A <em>grid</em> face comes from <code>mesh_q</code>, sampling the whole <code>u</code>,<code>v</code> rectangle on a regular grid, so its boundary is an iso line read straight off the <code>u</code>/<code>v</code> attributes. A <em>constrained</em> face comes from <code>mesh_loops</code>, triangulating inside given trim loops, so its boundary nodes carry <code>brep_edge/{edge}/{use}/{sample}</code> tags. </li>
<li><strong>constrained Delaunay</strong> (07) — a triangulation that is Delaunay except that named segments are forced to appear as edges. Here they are the trim loops, so a boundary node is a mesh node, not an approximation of one.</li>
<li><strong>source vs display</strong> — source: the kernel&#39;s face, edge, control point, in f64, with its id. Display: the triangles, node chains and markers made from it. GPU: the packed rows. Selection always names a source thing.</li>
<li><strong>id pass / pick window</strong> (12) — the same draws again into an integer target, only in a small window around the cursor, read back asynchronously; the answer is a row plus a sub-id.</li>
<li><strong>coverage mask / silhouette</strong> (12 for the selected object, 17 for every solid) — an <code>R8Unorm</code> texture marking which pixels a solid (and its edges) covers; a compositor paints the ring just outside it black. <code>O</code> toggles it.</li>
<li><strong>x-ray</strong> (<code>P</code>) — every multi-face solid loses its faces (they are discarded in all entry points), so only edges, vertices and text remain; single faces keep their shading; no silhouettes while it is on.</li>
<li><strong>headlight</strong> (<code>D</code>) — the camera light on shaded faces; off by default, so a face shows its flat colour.</li>
<li><strong>tile lists / finite visibility</strong> (18) — projected triangles binned into screen tiles, so an edge is hidden only by triangles that actually cover it, not by a neighbour&#39;s extended plane.</li>
<li><strong>CSR</strong> (06) — compressed sparse row: one flat array of entries plus a per-owner start index, instead of a vector per owner. The vertex→edge incidence is stored this way (<code>vstart</code>, <code>vinc</code>), so a vertex finds its edges without a heap allocation each.</li>
<li><strong>generation</strong> (12, 13) — a counter on every asynchronous answer (pick, range read); an answer from an older generation is dropped when it lands.</li>
</ul>
<h2 id="rust-idioms-the-code-leans-on">Rust idioms the code leans on<a class="anchor" href="#/course/words#rust-idioms-the-code-leans-on" aria-label="Link to this section">#</a></h2>
<ul>
<li><strong><code>#[repr(C)]</code> + <code>bytemuck::Pod</code></strong> (03) — lay the fields out in order with C&#39;s padding, and promise there are no pointers and no padding bytes, so <code>cast_slice</code> can view the struct as <code>&amp;[u8]</code> for the GPU without copying.</li>
<li><strong><code>Rc&lt;Session&gt;</code> / <code>Weak</code></strong> (12, 16) — a document shared by the scene and whoever decoded it, counted not copied; a <code>Weak</code> remembers it without keeping it alive.</li>
<li><strong><code>Arc</code> on the error callback</strong> (01) — the device keeps the handler and calls it later, so it takes shared ownership.</li>
<li><strong><code>anyhow::Result</code>, <code>?</code>, <code>ensure!</code>, <code>bail!</code></strong> (01) — one catch-all error type; <code>?</code> converts, <code>ensure!</code> checks, <code>bail!</code> returns early.</li>
<li><strong><code>#[cfg(target_arch = &quot;wasm32&quot;)]</code></strong> — code that exists only in the browser build; the crate&#39;s default target is wasm32, tests run natively through <code>cargo xtest</code>.</li>
<li><strong>let-else / let chains</strong> (06) — <code>let Some(x) = y else { continue }</code> and <code>if let A = b &amp;&amp; cond { }</code>: edition 2024 forms the kernel code uses.</li>
<li><strong><code>OnceLock</code> knobs</strong> (06, <code>src/app/knobs.rs</code>) — an environment flag read once per process, always false in the browser. The view knobs, which read <code>?name=</code> on wasm and <code>ENV</code> natively, arrive earlier in <code>src/engine/gpu/view.rs</code> (04a).</li>
</ul>
<h2 id="questions-and-answers">Questions and answers<a class="anchor" href="#/course/words#questions-and-answers" aria-label="Link to this section">#</a></h2>
<p><strong>Which of these is made per pipeline and which per buffer: a bind group layout, a bind group?</strong></p>
<p><em>How to work it out.</em> A layout names <em>kinds</em> of resources at slots — the shape the shader was compiled against; a bind group names <em>actual</em> buffers and views. A pipeline is compiled once; the buffers it draws from change.</p>
<p><em>The answer.</em> The layout is compiled into the pipeline; the bind group is made per set of real resources and plugged in at draw time. One pipeline draws with many bind groups.</p>
<p><strong>With reverse-Z, the depth buffer is cleared to 0 and faces compare <code>Greater</code>. What happens if the clear value were 1?</strong></p>
<p><em>How to work it out.</em> Depth after the divide lies in 0..1, so the largest value any fragment can have is 1. Ask whether <code>1 &gt; 1</code> is ever true.</p>
<p><em>The answer.</em> Nothing passes, ever: the canvas keeps its clear colour. This is the failure behind a black canvas when the depth convention is only half changed.</p>
<p><strong>A vertex shader outputs <code>(3.0, 3.0, 0.5, 1.0)</code> for all three vertices. What reaches the fragment stage?</strong></p>
<p><em>How to work it out.</em> Divide by w: x = y = 3, outside the −1..1 the screen covers. All three vertices are outside the same side of the volume, so the whole triangle is clipped.</p>
<p><em>The answer.</em> Nothing — no fragment entry runs. That is exactly how <code>dead_vertex</code> makes a hidden row disappear from the colour pass and the id pass at once, with one branch in the vertex stage.</p>
<p><strong>Name the three things Rust and WGSL must agree on for one draw, and where each mismatch shows up.</strong></p>
<p><em>How to work it out.</em> Walk the pipeline descriptor field by field and ask which ones restate something the shader also declares.</p>
<p><em>The answer.</em> Group and binding numbers, the entry-point names, and the colour target format. All three surface as a validation error at pipeline creation, never as a Rust compile error — <code>cargo check</code> cannot see inside WGSL, which is why <code>cargo xtest</code> parses every shader with naga.</p>
<p><strong>Where does the source identity of a face live: on the GPU, in <code>Scene</code>, or in the kernel?</strong></p>
<p><em>How to work it out.</em> Ask what each layer holds. The GPU has a row index and a face address — numbers. <code>Scene</code> has the mapping and the retained documents. The kernel <code>Session</code> has the face itself, with its guid.</p>
<p><em>The answer.</em> In the kernel, retained by <code>Scene</code> through <code>Rc</code>. The GPU knows only an object row and a face address; <code>Faces::source</code> turns that pair into its face, and <code>Scene::resolve</code> turns the row into its document and guid. That is why picking never reads a vertex buffer back from the GPU.</p>
`,toc:[{level:2,id:"two-sides-and-a-wire",text:"Two sides and a wire"},{level:2,id:"made-once-lesson-01",text:"Made once (lesson 01)"},{level:2,id:"memory-01-03-04a",text:"Memory (01, 03, 04a)"},{level:2,id:"binding-01-03-04a",text:"Binding (01, 03, 04a)"},{level:2,id:"pipeline-01-04a",text:"Pipeline (01, 04a)"},{level:2,id:"a-frame-01",text:"A frame (01)"},{level:2,id:"coordinates-02",text:"Coordinates (02)"},{level:2,id:"shader-words-01-04",text:"Shader words (01-04)"},{level:2,id:"house-words",text:"House words"},{level:2,id:"rust-idioms-the-code-leans-on",text:"Rust idioms the code leans on"},{level:2,id:"questions-and-answers",text:"Questions and answers"}]};export{e as default};
