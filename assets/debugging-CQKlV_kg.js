const e={title:"Reading failures",html:`<h1 id="reading-failures">Reading failures<a class="anchor" href="#/course/debugging#reading-failures" aria-label="Link to this section">#</a></h1>
<p>Most of the time you lose goes to an error that already told you the answer. Three habits solve almost every wgpu failure; then the ten this course actually produces.</p>
<h2 id="habit-1-three-declarations-must-agree">Habit 1 · Three declarations must agree<a class="anchor" href="#/course/debugging#habit-1-three-declarations-must-agree" aria-label="Link to this section">#</a></h2>
<p>Almost every validation error in wgpu is the same bug wearing different clothes: <strong>one thing is declared in three places and you changed only two.</strong></p>
<p>When something is wrong, name the thing (a vertex attribute, a binding, a uniform field, a texture format) and check all three. The error names one of them; the bug is usually in another.</p>
<p><img src="/session/docs/course/docs/illustrations/three-declarations.svg" alt="One thing declared in three places: change two and both edges that touch the third disagree. The error names one corner, and the stale declaration is usually a different one." loading="lazy" decoding="async"></p>
<h2 id="habit-2-read-the-error-all-of-it">Habit 2 · Read the error, all of it<a class="anchor" href="#/course/debugging#habit-2-read-the-error-all-of-it" aria-label="Link to this section">#</a></h2>
<p>wgpu&#39;s validation messages are long and they bury the useful line in the middle. Read to the end.</p>
<ul>
<li>In the <strong>browser</strong>, errors arrive asynchronously and print to the devtools console. The viewer also installs <code>on_uncaptured_error</code>, so a GPU error reaches the error panel instead of vanishing (<code>src/engine/gpu/device.rs</code>).</li>
<li>A <strong>Rust panic</strong> in wasm prints a proper stack trace only because <code>console_error_panic_hook::set_once()</code> runs first in <code>lib.rs</code>. Without it you get <code>unreachable executed</code> and nothing else.</li>
<li><strong>Natively</strong> (<code>cargo xtest</code>, the selftest binary) the same errors print to stderr, and naga validates every shader in a unit test — the cheapest place to catch WGSL mistakes.</li>
</ul>
<h2 id="habit-3-bisect-the-frame">Habit 3 · Bisect the frame<a class="anchor" href="#/course/debugging#habit-3-bisect-the-frame" aria-label="Link to this section">#</a></h2>
<p>When the canvas is wrong but nothing errors, cut the frame down until the picture changes:</p>
<ul>
<li>clear to an ugly colour — if you do not see it, the frame is not reaching the screen at all, and nothing after that matters;</li>
<li>draw one object, not the scene;</li>
<li>disable the depth test (<code>DepthMode::Always</code>), then the ink pass, then MSAA;</li>
<li>move the camera to a known view (keys 1–7, <code>C</code>, Space; natively <code>VIEWER_VIEW=top</code>, <code>VIEWER_ORTHO</code>, <code>VIEWER_DISTANCE_SCALE=5</code>).</li>
</ul>
<p>The first change that alters the picture is next to the bug.</p>
<hr>
<h2 id="the-ten-failures">The ten failures<a class="anchor" href="#/course/debugging#the-ten-failures" aria-label="Link to this section">#</a></h2>
<h3 id="1-the-canvas-is-black">1 · The canvas is black<a class="anchor" href="#/course/debugging#1-the-canvas-is-black" aria-label="Link to this section">#</a></h3>
<p>Black is the <em>background clear</em> colour before anything draws, so black means &quot;nothing drew&quot; — many causes, one method. In order:</p>
<ul>
<li>Did the first frame even run? The status line is HTML, not WebGPU — if it is stuck on the loading message, the failure is in the setup chain, not in drawing.</li>
<li>Is the canvas sized? A canvas with zero width configures a zero-sized surface and every draw is clipped away.</li>
<li>Is the geometry in front of the camera? See failure 6.</li>
<li>Is depth clearing right? With reverse-Z the clear value is <code>0.0</code> and the compare is <code>Greater</code>; clear to <code>1.0</code> by habit and every fragment fails the test, silently.</li>
</ul>
<p>Patience first: the first frame in a fresh browser profile compiles every pipeline — on a slow integrated GPU, seconds of black before anything appears.</p>
<h3 id="2-shader-compilation-failure">2 · Shader compilation failure<a class="anchor" href="#/course/debugging#2-shader-compilation-failure" aria-label="Link to this section">#</a></h3>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>error: the type of \`broken\` is expected to be \`u32\`, but got \`bool\`</code></pre></div>
<p>naga tells you the line. The traps that are not typos:</p>
<ul>
<li><strong><code>select(f, t, cond)</code> takes the false value first.</strong> <code>select(a, b, c)</code> is <code>c ? b : a</code>, the opposite of every ternary you have written.</li>
<li>A <code>var</code> without an initializer is zero, not undefined — but a <code>let</code> used before assignment will not compile.</li>
<li>An entry point must return everything its <code>@location</code> declarations promise; a missing field is a compile error, a <em>wrongly typed</em> one is a confusing cast.</li>
</ul>
<p>Catch these without a browser: <code>cargo xtest</code> parses every lane shader with naga.</p>
<h3 id="3-wrong-vertex-layout">3 · Wrong vertex layout<a class="anchor" href="#/course/debugging#3-wrong-vertex-layout" aria-label="Link to this section">#</a></h3>
<p>The symptom is not an error. It is <strong>geometry that looks shredded</strong>: triangles stretched to the horizon, or a mesh that flickers as the camera moves.</p>
<ul>
<li>The <code>array_stride</code> disagrees with <code>size_of::&lt;Vertex&gt;()</code>, so every vertex after the first reads from the middle of its neighbour.</li>
<li>An attribute <code>offset</code> is wrong, so position reads the normal&#39;s bytes.</li>
<li><code>#[repr(C)]</code> is missing, and Rust reordered the fields behind your back.</li>
</ul>
<p>The rule: write the layout from <code>offset_of!</code>, never from a count of bytes in your head.</p>
<h3 id="4-bind-group-layout-mismatch">4 · Bind-group-layout mismatch<a class="anchor" href="#/course/debugging#4-bind-group-layout-mismatch" aria-label="Link to this section">#</a></h3>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>Binding 0 has a different type (Buffer { ty: Uniform, .. }) than the one in the layout (buffer storage)</code></pre></div>
<p>This is Habit 1 in its purest form. The pipeline was compiled against a <em>layout</em>; the draw supplied a <em>group</em>; the shader declared <code>@group</code>/<code>@binding</code>. All three. In this viewer the scene contract (<code>src/shaders/scene.wgsl</code>) declares groups 0 to 2 once instead of in every lane shader.</p>
<h3 id="5-invalid-buffer-usage">5 · Invalid buffer usage<a class="anchor" href="#/course/debugging#5-invalid-buffer-usage" aria-label="Link to this section">#</a></h3>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>Usage flags BufferUsages(VERTEX) of Buffer with 'arena' label do not contain required usage flags BufferUsages(COPY_DST)</code></pre></div>
<p>Usage flags are fixed at creation and wgpu forgives none. If the CPU will ever write into a buffer again, it needs <code>COPY_DST</code> <em>at creation</em>, not at the write. Ask of every buffer: who writes this, and when?</p>
<h3 id="6-the-object-is-behind-the-camera">6 · The object is behind the camera<a class="anchor" href="#/course/debugging#6-the-object-is-behind-the-camera" aria-label="Link to this section">#</a></h3>
<p>Nothing errors; the screen is empty. Test it in this order:</p>
<ul>
<li>print the clip-space position of one known vertex: if <code>w</code> is negative, it is behind the eye;</li>
<li>if <code>x/w</code> or <code>y/w</code> is outside −1..1, it is off screen;</li>
<li>if <code>z/w</code> is outside the depth range, the near or far plane ate it.</li>
</ul>
<p>In this viewer <code>dead_vertex</code> parks a <em>deliberately</em> invisible vertex at <code>(3, 3, 0.5, 1)</code> — this failure used on purpose.</p>
<h3 id="7-wrong-matrix-order">7 · Wrong matrix order<a class="anchor" href="#/course/debugging#7-wrong-matrix-order" aria-label="Link to this section">#</a></h3>
<p>Matrix multiplication does not commute, and the wrong order is wrong <em>plausibly</em>: the object moves when it should spin, or spins around the wrong point.</p>
<ul>
<li>The chain is <code>projection * view * model * position</code>, applied right to left.</li>
<li>WGSL&#39;s <code>m * v</code> is column-vector convention; a matrix built for row vectors comes out transposed.</li>
<li>If the object orbits when the camera should, you have inverted the view matrix (or failed to).</li>
</ul>
<p>Change one factor at a time and watch what moves.</p>
<h3 id="8-surface-resize-problems">8 · Surface resize problems<a class="anchor" href="#/course/debugging#8-surface-resize-problems" aria-label="Link to this section">#</a></h3>
<ul>
<li>The picture is stretched or half the canvas is stale: the surface was not reconfigured after the resize, so its texture is still the old size.</li>
<li>The picture is crisp on one machine and blurry on another: you sized in CSS pixels where physical pixels were needed. <code>devicePixelRatio</code> is the only difference. This viewer reads it in two deliberate places: <code>device_pixel_ratio</code> (capped by <code>?dpr=</code>, used to size the canvas) and <code>surface_per_physical</code> (uncapped, used to convert pointer positions onto the surface actually drawn).</li>
<li>Everything breaks when the window is dragged small: a zero-sized surface is invalid; clamp to at least 1.</li>
</ul>
<p>The depth texture is sized too. A resize that forgets it fails with a mismatch on the next pass.</p>
<h3 id="9-stale-gpu-data">9 · Stale GPU data<a class="anchor" href="#/course/debugging#9-stale-gpu-data" aria-label="Link to this section">#</a></h3>
<p>The scene shows what it showed a moment ago, or half of each.</p>
<ul>
<li>A buffer was written but the frame was not requested: on the web nothing redraws by itself, so every state change must call <code>touch</code>/<code>request_redraw</code>.</li>
<li>A buffer grew and the bind group still points at the old one: a new buffer needs a new bind group.</li>
<li>An asynchronous read (a pick, a ranged fetch) landed after the thing it described was replaced. This viewer stamps such answers with a <code>generation</code> counter and drops the ones that come back late — a pattern worth stealing.</li>
</ul>
<h3 id="10-picking-coordinate-mismatch">10 · Picking coordinate mismatch<a class="anchor" href="#/course/debugging#10-picking-coordinate-mismatch" aria-label="Link to this section">#</a></h3>
<p>The click selects an object slightly up and to the left, or nothing at all. The pointer travels through four coordinate systems, and picking works only when all four agree:</p>
<p><img src="/session/docs/course/docs/illustrations/debugging-01.svg" alt="Diagram: pointer event\\ CSS px, canvas-relative · physical px\\ × devicePixelRatio · pick window\\ small offscreen target · id texture\\ row + sub-id · Scene\\ source identity" loading="lazy" decoding="async"></p>
<ul>
<li>An offset by a constant means the event was page-relative, not canvas-relative.</li>
<li>An offset that grows toward one corner means a missing (or doubled) <code>devicePixelRatio</code>.</li>
<li>Correct on one machine, wrong on a HiDPI laptop: the same bug, revealed.</li>
<li>Nothing is ever hit: the id pass may be rendering with a different camera than the visible pass — they must share the frame&#39;s matrices.</li>
</ul>
<hr>
<h2 id="when-it-is-not-your-code">When it is not your code<a class="anchor" href="#/course/debugging#when-it-is-not-your-code" aria-label="Link to this section">#</a></h2>
<p>A few failures are environmental, and recognising them saves hours:</p>
<ul>
<li><strong><code>navigator.gpu is undefined</code></strong> — WebGPU is off or the page is not on a secure origin. <code>http://127.0.0.1</code> counts as secure; a plain LAN IP does not.</li>
<li><strong>Device lost</strong> — the browser took the GPU away (a driver reset, a laptop lid). The viewer keeps the reason and reports it; recovery means rebuilding the device, not retrying the draw.</li>
<li><strong>It works natively, not in the browser</strong> — different adapter, different limits. Check <code>downlevel</code> limits before blaming the code.</li>
</ul>
<h2 id="getting-back-to-solid-ground">Getting back to solid ground<a class="anchor" href="#/course/debugging#getting-back-to-solid-ground" aria-label="Link to this section">#</a></h2>
<p>Every checkpoint is a complete crate under <code>docs/lessons/&lt;id&gt;/</code>:</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>diff <span class="s">-r</span> <span class="s">docs/lessons/07/src</span> <span class="s">src</span></code></pre></div>
<p>Diff your tree against it. The first file that differs is where your lesson went sideways.</p>
`,toc:[{level:2,id:"habit-1-three-declarations-must-agree",text:"Habit 1 · Three declarations must agree"},{level:2,id:"habit-2-read-the-error-all-of-it",text:"Habit 2 · Read the error, all of it"},{level:2,id:"habit-3-bisect-the-frame",text:"Habit 3 · Bisect the frame"},{level:2,id:"the-ten-failures",text:"The ten failures"},{level:3,id:"1-the-canvas-is-black",text:"1 · The canvas is black"},{level:3,id:"2-shader-compilation-failure",text:"2 · Shader compilation failure"},{level:3,id:"3-wrong-vertex-layout",text:"3 · Wrong vertex layout"},{level:3,id:"4-bind-group-layout-mismatch",text:"4 · Bind-group-layout mismatch"},{level:3,id:"5-invalid-buffer-usage",text:"5 · Invalid buffer usage"},{level:3,id:"6-the-object-is-behind-the-camera",text:"6 · The object is behind the camera"},{level:3,id:"7-wrong-matrix-order",text:"7 · Wrong matrix order"},{level:3,id:"8-surface-resize-problems",text:"8 · Surface resize problems"},{level:3,id:"9-stale-gpu-data",text:"9 · Stale GPU data"},{level:3,id:"10-picking-coordinate-mismatch",text:"10 · Picking coordinate mismatch"},{level:2,id:"when-it-is-not-your-code",text:"When it is not your code"},{level:2,id:"getting-back-to-solid-ground",text:"Getting back to solid ground"}]};export{e as default};
