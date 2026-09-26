const e={title:"What this is built on",html:`<h1 id="what-this-is-built-on">What this is built on<a class="anchor" href="#/course/references#what-this-is-built-on" aria-label="Link to this section">#</a></h1>
<p>Two kinds of thing belong here: the libraries the viewer links against, checkable in <code>Cargo.toml</code>, and the ideas it borrows, explained here rather than linked away.</p>
<p>Where this viewer uses a known technique, the course draws it rather than linking to someone else&#39;s explanation of it.</p>
<h2 id="the-libraries">The libraries<a class="anchor" href="#/course/references#the-libraries" aria-label="Link to this section">#</a></h2>
<p>Every version below is pinned in <code>session_viewer/Cargo.toml</code> and locked by <code>Cargo.lock</code>, which the course installs unmodified at lesson 00.</p>
<table>
<thead>
<tr>
<th>Crate</th>
<th>Version</th>
<th>What it does here</th>
</tr>
</thead>
<tbody><tr>
<td><code>wgpu</code></td>
<td>29.0</td>
<td>The WebGPU implementation. In the browser a thin layer over the browser&#39;s own WebGPU; natively it reaches Vulkan, Metal or DX12, so the same code runs under <code>cargo xtest</code>.</td>
</tr>
<tr>
<td><code>winit</code></td>
<td>0.30</td>
<td>The window and event loop. On the web it is the canvas and its pointer, keyboard and touch events.</td>
</tr>
<tr>
<td><code>glyphon</code></td>
<td>=0.11.0</td>
<td>Glyph atlas and text renderer on top of wgpu. Pinned exactly: it is the release that matches wgpu 29.</td>
</tr>
<tr>
<td><code>wasm-bindgen</code> 0.2, <code>wasm-bindgen-futures</code> 0.4, <code>web-sys</code> 0.3, <code>js-sys</code> 0.3</td>
<td>The bridge to the browser: the DOM, <code>fetch</code>, <code>EventSource</code>, <code>performance.now</code>.</td>
<td></td>
</tr>
<tr>
<td><code>bytemuck</code></td>
<td>1</td>
<td><code>Pod</code>/<code>Zeroable</code>, which let a <code>#[repr(C)]</code> struct be viewed as bytes for the GPU without a copy.</td>
</tr>
<tr>
<td><code>prost</code></td>
<td>0.14</td>
<td>Protobuf decoding for the <code>.pb</code> documents.</td>
</tr>
<tr>
<td><code>serde</code> 1.0, <code>serde_json</code> 1.0, <code>serde_yaml_ng</code> 0.10, <code>toml</code> =0.8.23</td>
<td>Manifest parsing in three formats with one set of semantics — a test asserts the three agree — plus session JSON validation, sheet side tables and the live source. <code>toml</code> is pinned exactly.</td>
<td></td>
</tr>
<tr>
<td><code>anyhow</code></td>
<td>1.0</td>
<td>One error type at the boundaries, so <code>?</code> composes.</td>
</tr>
<tr>
<td><code>log</code> 0.4, <code>console_log</code> 1.0, <code>console_error_panic_hook</code> 0.1.6</td>
<td>Diagnostics that survive the wasm boundary. Without the panic hook a Rust panic reaches the console as <code>unreachable executed</code> and nothing else.</td>
<td></td>
</tr>
<tr>
<td><code>getrandom</code></td>
<td>0.2 (<code>js</code>)</td>
<td>Randomness in the browser, where the usual system source does not exist.</td>
</tr>
<tr>
<td><code>naga</code></td>
<td>=29.0.4 (<code>wgsl-in</code>)</td>
<td>Parses every shader in the mirror tests, so <code>cargo xtest</code> catches a WGSL mistake without a GPU.</td>
</tr>
<tr>
<td><code>pollster</code></td>
<td>0.4 (native only)</td>
<td>Blocks on the async device setup in the native harness, so the same code path serves the browser and the tests.</td>
</tr>
<tr>
<td><code>session_rust</code></td>
<td>path</td>
<td>The geometry kernel, shared with the C++ and Python implementations. It links wgpu for the shared display type <code>RenderVertex</code> and for the GPU buffers a <code>Mesh</code> caches.</td>
</tr>
</tbody></table>
<p>Trunk builds the page; <code>wasm-bindgen</code> generates the JavaScript that instantiates the module. Lesson 00 draws that chain.</p>
<p><img src="/session/docs/course/docs/illustrations/toolchain.svg" alt="Four tools and four artefacts: cargo produces a .wasm a browser cannot load on its own, wasm-bindgen writes the JavaScript that can, Trunk assembles the page, and the browser runs start()." loading="lazy" decoding="async"></p>
<h2 id="where-the-authority-lives">Where the authority lives<a class="anchor" href="#/course/references#where-the-authority-lives" aria-label="Link to this section">#</a></h2>
<p>Three documents decide what is legal; no tutorial overrides them:</p>
<ul>
<li><strong>The WebGPU specification</strong> (<code>gpuweb.github.io/gpuweb/</code>) — what an adapter, device, pipeline and bind group are, and every validation rule your errors come from.</li>
<li><strong>The WGSL specification</strong> (<code>gpuweb.github.io/gpuweb/wgsl/</code>) — the shading language: types, alignment, entry points, builtins.</li>
<li><strong>The <code>wgpu</code> API documentation</strong> (<code>docs.rs/wgpu</code>) — the Rust shape of all of the above, version by version.</li>
</ul>
<p>These URLs were not fetched while this page was written: treat them as the place to look, not as a citation. Which to open: an error at pipeline creation is a WebGPU rule, an error inside a shader a WGSL rule, a signature that does not match an API question.</p>
<h2 id="the-ideas-and-where-the-course-draws-them">The ideas, and where the course draws them<a class="anchor" href="#/course/references#the-ideas-and-where-the-course-draws-them" aria-label="Link to this section">#</a></h2>
<table>
<thead>
<tr>
<th>Idea</th>
<th>The problem it solves</th>
<th>Drawn in</th>
<th>Built in</th>
</tr>
</thead>
<tbody><tr>
<td><strong>Reverse-Z depth</strong></td>
<td>Float depth crowds its precision at the far plane, where you need it least. Swapping near and far puts it near the eye.</td>
<td><a href="#/course/02-camera">frustum</a>, <a href="#/course/05-visibility">ink-visibility</a></td>
<td><code>camera.rs</code> swaps the planes, every depth-testing <code>DepthMode</code> compares <code>Greater</code> or <code>GreaterEqual</code></td>
</tr>
<tr>
<td><strong>Depth-gradient carried ink</strong></td>
<td>A thick stroke&#39;s fragments sit beside its axis and read the wrong surface&#39;s depth. The surface slope carries the depth from fragment to axis.</td>
<td><a href="#/course/05-visibility">ink-visibility</a></td>
<td><code>shaders/ink_visibility.wgsl</code></td>
</tr>
<tr>
<td><strong>Finite-triangle visibility</strong></td>
<td>A depth plane is infinite; a triangle is not. Binning projected triangles into screen tiles lets an edge be hidden only by geometry that really covers it.</td>
<td><a href="#/course/18-finite-visibility">finite-triangle</a>, <a href="#/course/18-finite-visibility">tiles</a></td>
<td><code>engine/gpu/triangle_tiles.rs</code></td>
</tr>
<tr>
<td><strong>Octahedral normal encoding</strong></td>
<td>A unit vector has two degrees of freedom, so it does not need three floats. Two 8-bit numbers are enough for shading and culling.</td>
<td><a href="#/course/09-normals">normals</a></td>
<td><code>app/walk/encode.rs</code>, <code>shaders/normals.wgsl</code></td>
</tr>
<tr>
<td><strong>Newell&#39;s normal</strong></td>
<td>The cross product of the first two edges inverts on a reflex corner; a sum over all edges cannot.</td>
<td><a href="#/course/06-cad-contract">cad-contract</a></td>
<td><code>app/walk/mesh_topology.rs</code></td>
</tr>
<tr>
<td><strong>Screen-space ribbons</strong></td>
<td>A line with a constant pixel width is not geometry with a width; it is a quad expanded in screen space, with exact coverage rather than a distance ramp.</td>
<td><a href="#/course/04b-strokes">ribbon</a>, <a href="#/course/17-source-presentation">joins</a></td>
<td><code>shaders/ribbon.wgsl</code></td>
</tr>
<tr>
<td><strong>Eye-Dome Lighting</strong></td>
<td>A point cloud with no normals reads flat. Comparing a pixel&#39;s depth with its neighbours&#39; gives shape for free.</td>
<td><a href="#/course/04d-clouds">splat-resolve</a></td>
<td><code>shaders/splat_resolve.wgsl</code></td>
</tr>
<tr>
<td><strong>Octree level of detail</strong></td>
<td>Drawing every point of a large cloud is wasted work when their spacing projects below a pixel.</td>
<td><a href="#/course/04d-clouds">lod</a></td>
<td><code>engine/gpu/lod.rs</code></td>
</tr>
<tr>
<td><strong>Prefix-sum allocation</strong></td>
<td>Per-tile lists with a fixed cap either waste memory or overflow. Counting first, then scanning into offsets, lets dense tiles borrow from sparse ones.</td>
<td><a href="#/course/18-finite-visibility">tiles</a></td>
<td><code>shaders/scan_triangle_tiles.wgsl</code></td>
</tr>
<tr>
<td><strong>Coverage mask silhouettes</strong></td>
<td>An outline drawn as geometry fights the geometry it outlines. A coverage mask plus a dilation is one full-screen pass and always closes.</td>
<td><a href="#/course/17-source-presentation">masks</a></td>
<td><code>engine/gpu/surface_outline.rs</code></td>
</tr>
<tr>
<td><strong>ID-buffer picking</strong></td>
<td>A CPU ray-caster is a second answer to &quot;what is visible here&quot; and will eventually disagree with the picture. Re-rendering ids cannot.</td>
<td><a href="#/course/12-picking">picking</a></td>
<td><code>engine/gpu/pick.rs</code></td>
</tr>
<tr>
<td><strong>Camera rebasing</strong></td>
<td>f32 has about seven digits; CAD coordinates do not fit. Subtracting an anchor near the camera before converting keeps the digits that matter.</td>
<td><a href="#/course/02-camera">spaces</a></td>
<td><code>camera.rs</code>, <code>engine/gpu/objects.rs</code></td>
</tr>
</tbody></table>
<h2 id="the-cad-comparison">The CAD comparison<a class="anchor" href="#/course/references#the-cad-comparison" aria-label="Link to this section">#</a></h2>
<p>This project cites external source code line by line in one place: the boundary-representation contract, because somebody else answered &quot;what should a CAD viewer do with a trimmed face&quot; first, and well. <a href="#/course/cad-design">The CAD design record</a> lists the exact OCCT files and revisions the contract was compared against, with commit-pinned links, and what was taken and what was decided differently.</p>
<h2 id="reading-this-repository-instead">Reading this repository instead<a class="anchor" href="#/course/references#reading-this-repository-instead" aria-label="Link to this section">#</a></h2>
<p>The most reliable reference for this viewer is the viewer. Three places answer most questions faster than a search:</p>
<ul>
<li><a href="#/course/map">The map</a> — where any file sits, and what it is allowed to know.</li>
<li><code>ARCHITECTURE.md</code> — the module graph, the owners table, one frame&#39;s passes in order, and the Rust ↔ WGSL interface tables.</li>
<li><code>cargo xtest</code> — the shader mirror tests. To know whether Rust and WGSL still agree about a struct, run the test that asserts it rather than reading both.</li>
</ul>
`,toc:[{level:2,id:"the-libraries",text:"The libraries"},{level:2,id:"where-the-authority-lives",text:"Where the authority lives"},{level:2,id:"the-ideas-and-where-the-course-draws-them",text:"The ideas, and where the course draws them"},{level:2,id:"the-cad-comparison",text:"The CAD comparison"},{level:2,id:"reading-this-repository-instead",text:"Reading this repository instead"}]};export{e as default};
