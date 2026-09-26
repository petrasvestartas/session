const s={title:"18 · Finite-triangle visibility and maintained viewer convergence",html:`<h1 id="18-finite-triangle-visibility-and-maintained-viewer-convergence">18 · Finite-triangle visibility and maintained viewer convergence<a class="anchor" href="#/course/18-finite-visibility#18-finite-triangle-visibility-and-maintained-viewer-convergence" aria-label="Link to this section">#</a></h1>
<p>Concave boundaries and contact seams stay continuous while genuinely covered edges remain hidden.</p>
<p><img src="/session/docs/course/docs/illustrations/finite-triangle.svg" alt="The depth plane continues beyond the finite triangle; only a finite nearer hit can hide the axis." loading="lazy" decoding="async"></p>
<p>Copy each file from the lesson folder to the path shown.</p>
<p>Copy from <code>lessons/18/</code> (tooling this checkpoint needs but the course does not teach):</p>
<ul>
<li><code>lessons/18/examples/mk_triangle_visibility.rs</code></li>
<li><code>lessons/18/tests/README.md</code></li>
<li><code>lessons/18/tests/depth/_gate.sh</code></li>
<li><code>lessons/18/tests/depth/_ink_suite.sh</code></li>
<li><code>lessons/18/tests/depth/_orbit_check.py</code></li>
<li><code>lessons/18/tests/depth/_probe_matrix.py</code></li>
<li><code>lessons/18/tests/depth/_stroke_weight.py</code></li>
<li><code>lessons/18/tests/interaction.cjs</code></li>
<li><code>lessons/18/tests/nameplate-scene.cjs</code></li>
<li><code>lessons/18/tests/selection-overlap.py</code></li>
<li><code>lessons/18/tests/streamed-controls.cjs</code></li>
<li><code>lessons/18/tests/stroke-joins.py</code></li>
<li><code>lessons/18/tests/teapot.cjs</code></li>
<li><code>lessons/18/tests/text-quality.cjs</code></li>
<li><code>lessons/18/tests/triangle-visibility.py</code></li>
<li><code>lessons/18/tests/world-text.cjs</code></li>
</ul>
<h2 id="step-1-srcshadersphysicalwgsl">Step 1 · src/shaders/physical.wgsl<a class="anchor" href="#/course/18-finite-visibility#step-1-srcshadersphysicalwgsl" aria-label="Link to this section">#</a></h2>
<p>Physical outputs store depth information beside face color.</p>
<p><code>lessons/18/src/shaders/physical.wgsl</code> · edit · type this</p>
<p>Replaces the 20 lines from <code>@location(1) gradient: vec2&lt;f32&gt;,</code> in <code>struct PhysicalColor</code> of <code>lessons/17/src/shaders/physical.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    @location(<span class="s">1</span>) gradient: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // depth slope, for the visibility test</span>
}

<span class="c">// Output of a face id fragment: ids and depth slope.</span>
<span class="k">struct</span> PhysicalId {
    @location(<span class="s">0</span>) id: <span class="k">vec2</span>&lt;<span class="k">u32</span>&gt;,<span class="c"> // object row + 1, sub id + 1</span>
    @location(<span class="s">1</span>) gradient: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // depth slope, for the visibility test</span>
}

<span class="c">// Depth slope of this fragment, or the invalid marker.</span>
<span class="k">fn</span> physical_gradient(depth: <span class="k">f32</span>) -&gt; <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt; {
 <span class="k">let</span> scaled=<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(dpdx(depth), dpdy(depth))*PLANE_SCALE;

 <span class="k">if</span> (any(abs(scaled)&gt;=<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(PLANE_INVALID))) {
     <span class="k">return</span> <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(PLANE_INVALID, PLANE_INVALID, <span class="s">0.0</span>, <span class="s">0.0</span>);
 }

 <span class="k">return</span> <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(scaled, <span class="s">0.0</span>, <span class="s">0.0</span>);
}

<span class="c">// Depth slope plus the triangle index, packed into two half floats.</span>
<span class="k">fn</span> physical_triangle(depth: <span class="k">f32</span>, primitive: <span class="k">u32</span>) -&gt; <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt; {
 <span class="k">let</span> words=<span class="k">vec2</span>&lt;<span class="k">u32</span>&gt;(primitive&amp;<span class="s">0x3fffu</span>, primitive&gt;&gt;<span class="s">14u</span>)+<span class="k">vec2</span>&lt;<span class="k">u32</span>&gt;(<span class="s">0x400u</span>);
 <span class="k">let</span> address=unpack2x16float(words.x|(words.y&lt;&lt;<span class="s">16u</span>));
 <span class="k">return</span> <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(physical_gradient(depth).xy, address);</code></pre></div>
<h2 id="step-2-srcshaderstrianglewgsl">Step 2 · src/shaders/triangle.wgsl<a class="anchor" href="#/course/18-finite-visibility#step-2-srcshaderstrianglewgsl" aria-label="Link to this section">#</a></h2>
<p>The mesh shader places vertices and shades visible faces.</p>
<p><code>lessons/18/src/shaders/triangle.wgsl</code> · edit · type this</p>
<p>Added after the <code>@location(7) @interpolate(flat) source_face:…</code> line in <code>struct VsOut</code> of <code>lessons/17/src/shaders/triangle.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    @location(<span class="s">8</span>) @interpolate(flat) primitive: <span class="k">u32</span>,<span class="c"> // triangle index + 1</span>
<span class="c">    // closed solids take the opacity; open sheets stay opaque</span>
    @location(<span class="s">9</span>) @interpolate(flat) closed: <span class="k">u32</span>,
<span class="c">    // x-ray drops the faces of multi-face solids only</span>
    @location(<span class="s">10</span>) @interpolate(flat) xray: <span class="k">u32</span>,
}

<span class="c">// A vertex placed off screen, so nothing is drawn.</span>
<span class="k">fn</span> dead_vertex() -&gt; VsOut {
    <span class="k">var</span> dead: VsOut;<span class="c"> // all zero; only the position matters</span></code></pre></div>
<p>Added after the <code>o.source_face = 0xffffffffu;</code> line in <code>fn transform_vertex</code> of <code>lessons/17/src/shaders/triangle.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    o.closed = select(<span class="s">1u</span>, <span class="s">0u</span>, (inst.flags &amp; FLAG_OPEN) != <span class="s">0u</span>);
    o.xray = select(<span class="s">0u</span>, <span class="s">1u</span>, line.opacity &lt;= <span class="s">0.0</span> &amp;&amp; (inst.flags &amp; (FLAG_PRINT | FLAG_SHEET | FLAG_SINGLE)) == <span class="s">0u</span>);</code></pre></div>
<p>Replaces the 9 lines from <code>@vertex</code> of <code>lessons/17/src/shaders/triangle.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// Vertex \`index\` read from the storage buffers instead of vertex inputs.</span>
<span class="k">fn</span> pull_triangle(index: <span class="k">u32</span>) -&gt; VsOut {
    <span class="k">let</span> vertex = face_indices[index];
    <span class="k">let</span> start = vertex * <span class="s">10u</span>;<span class="c"> // ten floats per vertex: position, normal, color, padding</span>
    <span class="k">let</span> position = <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(face_vertices[start], face_vertices[start+<span class="s">1u</span>], face_vertices[start+<span class="s">2u</span>]);
    <span class="k">let</span> normal = <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(face_vertices[start+<span class="s">3u</span>], face_vertices[start+<span class="s">4u</span>], face_vertices[start+<span class="s">5u</span>]);
    <span class="k">let</span> color = <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(face_vertices[start+<span class="s">6u</span>], face_vertices[start+<span class="s">7u</span>], face_vertices[start+<span class="s">8u</span>]);
    <span class="k">var</span> out = transform_vertex(VsIn(position, normal, color, face_objects[vertex]));
    out.primitive = index/<span class="s">3u</span>+<span class="s">1u</span>;
    <span class="k">return</span> out;
}

@vertex
<span class="c">// Vertices read by index; object ids.</span>
<span class="k">fn</span> vs_triangle(@builtin(vertex_index) index: <span class="k">u32</span>) -&gt; VsOut {
    <span class="k">return</span> pull_triangle(index);
}

@vertex
<span class="c">// Vertices read by index, with the source face and its selection.</span>
<span class="k">fn</span> vs_face(@builtin(vertex_index) index: <span class="k">u32</span>) -&gt; VsOut {
    <span class="k">var</span> out=pull_triangle(index);</code></pre></div>
<p>Replaces the 14 lines from <code>return vec4&lt;f32&gt;(base * shaded, 1.0);</code> in <code>fn shade</code> of <code>lessons/17/src/shaders/triangle.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">let</span> alpha = select(<span class="s">1.0</span>, line.opacity, in.closed != <span class="s">0u</span>);
    <span class="k">return</span> <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(base * shaded, alpha);
}

<span class="c">// Pick id: object row + 1 and the tagged face id.</span>
@fragment
<span class="k">fn</span> fs_id(in: VsOut) -&gt; PhysicalId {
    <span class="k">if</span> (in.xray != <span class="s">0u</span>) {
        <span class="k">discard</span>;
    }

    <span class="k">let</span> sub = select((FACE_TAG | in.source_face) + <span class="s">1u</span>, <span class="s">0u</span>, in.source_face == <span class="s">0xffffffffu</span>);
    <span class="k">return</span> PhysicalId(<span class="k">vec2</span>&lt;<span class="k">u32</span>&gt;(in.inst_id + <span class="s">1u</span>, sub), physical_triangle(in.pos.z, in.primitive));
}

@fragment
<span class="c">// Selected faces into a mask.</span>
<span class="k">fn</span> fs_selection_mask(in: VsOut) -&gt; @location(<span class="s">0</span>) <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt; {
    <span class="k">if</span> (in.selected == <span class="s">0u</span> || in.xray != <span class="s">0u</span>) {</code></pre></div>
<p>Replaces the 7 lines from <code>return PhysicalColor(shade(in, front), physic…</code> in <code>fn fs_main</code> of <code>lessons/17/src/shaders/triangle.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">if</span> (in.xray != <span class="s">0u</span>) {
        <span class="k">discard</span>;
    }

    <span class="k">return</span> PhysicalColor(shade(in, front), physical_triangle(in.pos.z, in.primitive));
}

@fragment
<span class="c">// The selected source face, shaded.</span>
<span class="k">fn</span> fs_face_highlight(in: VsOut, @builtin(front_facing) front: <span class="k">bool</span>) -&gt; @location(<span class="s">0</span>) <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt; {
    <span class="k">if</span> (in.selected == <span class="s">0u</span> || in.xray != <span class="s">0u</span>) {</code></pre></div>
<p>Replaces the 3 lines from <code>return vec4&lt;f32&gt;(1.0);</code> in <code>fn fs_solid_mask</code> of <code>lessons/17/src/shaders/triangle.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">if</span> (in.xray != <span class="s">0u</span>) {
        <span class="k">discard</span>;
    }

    <span class="k">return</span> <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(<span class="s">1.0</span>);
}

<span class="c">// Output into both outline masks at once.</span>
<span class="k">struct</span> MaskPair {
    @location(<span class="s">0</span>) solid: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // every solid's mask</span>
    @location(<span class="s">1</span>) selected: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // the selection's mask</span>
};

@fragment
<span class="c">// Every face into the solid mask; selected ones into both.</span>
<span class="k">fn</span> fs_masks(in: VsOut) -&gt; MaskPair {
    <span class="k">if</span> (in.xray != <span class="s">0u</span>) {
        <span class="k">discard</span>;
    }

    <span class="k">return</span> MaskPair(<span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(<span class="s">1.0</span>), <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(f32(in.selected != <span class="s">0u</span>)));
}</code></pre></div>
<h2 id="step-3-srcshadersbackgroundwgsl">Step 3 · src/shaders/background.wgsl<a class="anchor" href="#/course/18-finite-visibility#step-3-srcshadersbackgroundwgsl" aria-label="Link to this section">#</a></h2>
<p>The background fills uncovered pixels.</p>
<p><code>lessons/18/src/shaders/background.wgsl</code> · edit · type this</p>
<p>Replaces the <code>return PhysicalColor(vec4&lt;f32&gt;(1.0, 1.0, 1.0,…</code> line in <code>fn fs_main</code> of <code>lessons/17/src/shaders/background.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">return</span> PhysicalColor(<span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(<span class="s">1.0</span>, <span class="s">1.0</span>, <span class="s">1.0</span>, <span class="s">1.0</span>), <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>));</code></pre></div>
<h2 id="step-4-srcshadersgridwgsl">Step 4 · src/shaders/grid.wgsl<a class="anchor" href="#/course/18-finite-visibility#step-4-srcshadersgridwgsl" aria-label="Link to this section">#</a></h2>
<p>The grid generates construction lines from vertex indices.</p>
<p><code>lessons/18/src/shaders/grid.wgsl</code> · edit · type this</p>
<p>Replaces the <code>return PhysicalColor(vec4&lt;f32&gt;(in.color, 1.0)…</code> line in <code>fn fs_main</code> of <code>lessons/17/src/shaders/grid.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">return</span> PhysicalColor(<span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(in.color, <span class="s">1.0</span>), <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>));</code></pre></div>
<h2 id="step-5-srcshaderssplatwgsl">Step 5 · src/shaders/splat.wgsl<a class="anchor" href="#/course/18-finite-visibility#step-5-srcshaderssplatwgsl" aria-label="Link to this section">#</a></h2>
<p>Point projection writes the nearest visible cloud samples.</p>
<p><code>lessons/18/src/shaders/splat.wgsl</code> · edit · type this</p>
<p>Replaces the <code>return PhysicalId(vec2&lt;u32&gt;(in.instance + 1u,…</code> line in <code>fn fs_point_id</code> of <code>lessons/17/src/shaders/splat.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">return</span> PhysicalId(<span class="k">vec2</span>&lt;<span class="k">u32</span>&gt;(in.instance + <span class="s">1u</span>, in.row + <span class="s">1u</span>), <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>));</code></pre></div>
<h2 id="step-6-srcshaderssplat_resolvewgsl">Step 6 · src/shaders/splat_resolve.wgsl<a class="anchor" href="#/course/18-finite-visibility#step-6-srcshaderssplat_resolvewgsl" aria-label="Link to this section">#</a></h2>
<p>The resolve writes point color and depth into the scene.</p>
<p><code>lessons/18/src/shaders/splat_resolve.wgsl</code> · edit · type this</p>
<p>Replaces the <code>@location(1) gradient: vec2&lt;f32&gt;,</code> line in <code>struct FsOut</code> of <code>lessons/17/src/shaders/splat_resolve.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    @location(<span class="s">1</span>) gradient: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // depth slope; zero for points</span></code></pre></div>
<p>Replaces the <code>o.gradient = vec2&lt;f32&gt;(0.0);</code> line in <code>fn shade</code> of <code>lessons/17/src/shaders/splat_resolve.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    o.gradient = <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>);</code></pre></div>
<h2 id="step-7-srcshaderstext_outlinewgsl">Step 7 · src/shaders/text_outline.wgsl<a class="anchor" href="#/course/18-finite-visibility#step-7-srcshaderstext_outlinewgsl" aria-label="Link to this section">#</a></h2>
<p>And for the resolve, the pass that writes the scene&#39;s depth for a cloud.</p>
<p><code>lessons/18/src/shaders/text_outline.wgsl</code> · edit · type this</p>
<p>Replaces the <code>return PhysicalId(vec2&lt;u32&gt;(fragment.object+1…</code> line in <code>fn fs_physical_id</code> of <code>lessons/17/src/shaders/text_outline.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code> <span class="k">return</span> PhysicalId(<span class="k">vec2</span>&lt;<span class="k">u32</span>&gt;(fragment.object+<span class="s">1u</span>, <span class="s">0u</span>), <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>));</code></pre></div>
<h2 id="step-8-srcshadersprojected_trianglewgsl">Step 8 · src/shaders/projected_triangle.wgsl<a class="anchor" href="#/course/18-finite-visibility#step-8-srcshadersprojected_trianglewgsl" aria-label="Link to this section">#</a></h2>
<p>Projected triangle helpers test both edge coverage and interpolated depth.</p>
<p><code>lessons/18/src/shaders/projected_triangle.wgsl</code> · 50 lines · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// One triangle in screen space, 96 bytes.</span>
<span class="k">struct</span> ProjectedTriangle {
    edge0: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // xyz: edge line equation; w: reference x</span>
    edge1: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // xyz: edge line equation; w: reference y</span>
    edge2: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // xyz: edge line equation; w: reference depth</span>
    edge3: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // xyz: fourth edge after clipping; w: corner count</span>
    gradient: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // xy: depth slope; z: nearest depth; w: contact radius</span>
    bounds: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // screen box: left, top, right, bottom</span>
};

<span class="c">// (depth, 1) where the triangle covers \`at\`, else (0, 0).</span>
<span class="k">fn</span> projected_triangle_at(triangle: ProjectedTriangle, at: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;) -&gt; <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt; {
<span class="c">    // empty triangle</span>
    <span class="k">if</span> (triangle.edge3.w&lt;<span class="s">3.0</span>) {
        <span class="k">return</span> <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>);
    }

<span class="c">    // 1/256 px slack: the rasterizer's vertex snap</span>
    <span class="k">if</span> (any(at&lt;triangle.bounds.xy-<span class="s">0.00390625</span>) || any(at&gt;triangle.bounds.zw+<span class="s">0.00390625</span>)) {
        <span class="k">return</span> <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>);
    }

<span class="c">    // signed distance to each edge; negative = outside</span>
    <span class="k">let</span> distances = <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(dot(triangle.edge0.xy, at)+triangle.edge0.z,
    dot(triangle.edge1.xy, at)+triangle.edge1.z,
    dot(triangle.edge2.xy, at)+triangle.edge2.z,
    dot(triangle.edge3.xy, at)+triangle.edge3.z);

    <span class="k">if</span> (any(distances&lt;<span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(-<span class="s">0.00390625</span>))) {
        <span class="k">return</span> <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>);
    }

<span class="c">    // depth at \`at\` from the reference point and slope</span>
    <span class="k">let</span> depth = triangle.edge2.w+dot(triangle.gradient.xy, at-<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(triangle.edge0.w, triangle.edge1.w));
    <span class="k">return</span> <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(depth, <span class="s">1.0</span>);
}
<span class="c">// Bound the header and pooled-reference allocations at large framebuffer sizes.</span>

<span class="k">fn</span> visibility_tile_span_of(width: <span class="k">u32</span>, height: <span class="k">u32</span>) -&gt; <span class="k">u32</span> {
    <span class="k">var</span> span = <span class="s">4u</span>;

    <span class="k">while</span> (((width+span-<span class="s">1u</span>)/span)*((height+span-<span class="s">1u</span>)/span)&gt;<span class="s">262144u</span>) {
        span*=<span class="s">2u</span>;
    }

    <span class="k">return</span> span;
}

<span class="c">// Pixels per tile side for this canvas.</span>
<span class="k">fn</span> visibility_tile_span() -&gt; <span class="k">u32</span> {
    <span class="k">return</span> visibility_tile_span_of(u32(line.vp_w), u32(line.vp_h));
}</code></pre></div>
<h2 id="step-9-srcshadersproject_triangleswgsl">Step 9 · src/shaders/project_triangles.wgsl<a class="anchor" href="#/course/18-finite-visibility#step-9-srcshadersproject_triangleswgsl" aria-label="Link to this section">#</a></h2>
<p>The projection pass clips triangles and records their screen bounds.</p>
<p><code>lessons/18/src/shaders/project_triangles.wgsl</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>@group(<span class="s">0</span>) @binding(<span class="s">0</span>) <span class="k">var</span>&lt;<span class="k">uniform</span>&gt; mvp: <span class="k">mat4x4</span>&lt;<span class="k">f32</span>&gt;;<span class="c"> // camera matrix</span>

<span class="c">// The first five fields of LineUniform; this shader needs no more.</span>
<span class="k">struct</span> ProjectLine {
    thickness: <span class="k">f32</span>,<span class="c"> // pen width, px</span>
    proj_y: <span class="k">f32</span>,<span class="c"> // perspective scale factor</span>
    ortho_h: <span class="k">f32</span>,<span class="c"> // ortho half-height; 0 = perspective</span>
    vp_h: <span class="k">f32</span>,<span class="c"> // target height, px</span>
    vp_w: <span class="k">f32</span>,<span class="c"> // target width, px</span>
};

@group(<span class="s">1</span>) @binding(<span class="s">0</span>) <span class="k">var</span>&lt;<span class="k">uniform</span>&gt; line: ProjectLine;<span class="c"> // view settings</span>

<span class="c">// Instance under another name; field order must match the Rust row.</span>
<span class="k">struct</span> ProjectInstance {
    model: <span class="k">mat4x4</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // rotation and scale</span>
    color: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // rgba tint</span>
    flags: <span class="k">u32</span>,
    _pad0: <span class="k">f32</span>,
    spacing: <span class="k">f32</span>,
};

@group(<span class="s">2</span>) @binding(<span class="s">0</span>) <span class="k">var</span>&lt;<span class="k">storage</span>, <span class="k">read</span>&gt; instances: <span class="k">array</span>&lt;ProjectInstance&gt;;<span class="c"> // one row per object</span>
@group(<span class="s">2</span>) @binding(<span class="s">1</span>) <span class="k">var</span>&lt;<span class="k">storage</span>, <span class="k">read</span>&gt; translations: <span class="k">array</span>&lt;<span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;&gt;;<span class="c"> // position per object</span>
@group(<span class="s">3</span>) @binding(<span class="s">0</span>) <span class="k">var</span>&lt;<span class="k">storage</span>, <span class="k">read</span>&gt; physical_vertices: <span class="k">array</span>&lt;<span class="k">f32</span>&gt;;<span class="c"> // mesh vertices, ten floats each</span>
@group(<span class="s">3</span>) @binding(<span class="s">1</span>) <span class="k">var</span>&lt;<span class="k">storage</span>, <span class="k">read</span>&gt; physical_objects: <span class="k">array</span>&lt;<span class="k">u32</span>&gt;;<span class="c"> // object row per vertex</span>
@group(<span class="s">3</span>) @binding(<span class="s">2</span>) <span class="k">var</span>&lt;<span class="k">storage</span>, <span class="k">read</span>&gt; physical_indices: <span class="k">array</span>&lt;<span class="k">u32</span>&gt;;
@group(<span class="s">3</span>) @binding(<span class="s">3</span>) <span class="k">var</span>&lt;<span class="k">storage</span>, <span class="k">read_write</span>&gt; projected: <span class="k">array</span>&lt;ProjectedTriangle&gt;;<span class="c"> // output: one record per triangle</span>
@group(<span class="s">3</span>) @binding(<span class="s">4</span>) <span class="k">var</span>&lt;<span class="k">uniform</span>&gt; live_count: <span class="k">vec4</span>&lt;<span class="k">u32</span>&gt;;<span class="c"> // x = triangle count</span>

<span class="c">// Project one triangle per invocation into \`projected\`.</span>
@compute @workgroup_size(<span class="s">64</span>)
<span class="k">fn</span> cs_main(@builtin(global_invocation_id) id: <span class="k">vec3</span>&lt;<span class="k">u32</span>&gt;) {
    <span class="k">let</span> index = id.x;

    <span class="k">if</span> (index&gt;=live_count.x) {
        <span class="k">return</span>;
    }

    <span class="k">let</span> polygon = project_physical_triangle(index+<span class="s">1u</span>);
    <span class="k">var</span> out: ProjectedTriangle;

<span class="c">    // otherwise the record stays all zero</span>
    <span class="k">if</span> (polygon.count&gt;=<span class="s">3u</span>) {
<span class="c">        // screen box</span>
        <span class="k">var</span> lo = polygon.points[<span class="s">0</span>].xy;
        <span class="k">var</span> hi = lo;

        <span class="k">for</span> (<span class="k">var</span> i = <span class="s">1u</span>;i&lt;polygon.count;i++) {
            lo = min(lo, polygon.points[i].xy);
            hi = max(hi, polygon.points[i].xy);
        }

        <span class="k">let</span> area = physical_polygon_area(polygon);

<span class="c">        // degenerate: empty record</span>
        <span class="k">if</span> (abs(area)&lt;1e-<span class="s">12</span>) {
            projected[index] = out;
            <span class="k">return</span>;
        }

<span class="c">        // one inward line equation per edge</span>
        <span class="k">var</span> equations: <span class="k">array</span>&lt;<span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;, <span class="s">4</span>&gt;;

        <span class="k">for</span> (<span class="k">var</span> i = <span class="s">0u</span>;i&lt;polygon.count;i++) {
            <span class="k">let</span> a = polygon.points[i].xy;
            <span class="k">let</span> edge = polygon.points[(i+<span class="s">1u</span>)%polygon.count].xy-a;
            <span class="k">let</span> normal = sign(area)*<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(-edge.y, edge.x)/length(edge);
            equations[i] = <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(normal, -dot(normal, a), <span class="s">0.0</span>);
        }

        <span class="k">let</span> ab = polygon.points[<span class="s">1</span>]-polygon.points[<span class="s">0</span>];
        <span class="k">let</span> ac = polygon.points[<span class="s">2</span>]-polygon.points[<span class="s">0</span>];
<span class="c">        // depth change per screen pixel</span>
        <span class="k">let</span> gradient = <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(ab.z*ac.y-ac.z*ab.y, ab.x*ac.z-ac.x*ab.z)/area;
        out.edge0 = <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(equations[<span class="s">0</span>].xyz, polygon.points[<span class="s">0</span>].x);
        out.edge1 = <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(equations[<span class="s">1</span>].xyz, polygon.points[<span class="s">0</span>].y);
        out.edge2 = <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(equations[<span class="s">2</span>].xyz, polygon.points[<span class="s">0</span>].z);
        out.edge3 = <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(equations[<span class="s">3</span>].xyz, f32(polygon.count));
<span class="c">        // nearest corner depth</span>
        <span class="k">var</span> nearest = polygon.points[<span class="s">0</span>].z;

        <span class="k">for</span> (<span class="k">var</span> i = <span class="s">1u</span>;i&lt;polygon.count;i++) {
            nearest = max(nearest, polygon.points[i].z);
        }

        out.gradient = <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(gradient, nearest, <span class="s">0.0</span>);
        out.bounds = <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(lo, hi);
    }

    projected[index] = out;
}</code></pre></div>
<p><code>lessons/18/src/shaders/project_triangles.wgsl</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// Shared by tile binning and the visibility test.</span>

<span class="k">struct</span> ProjectedPolygon {
    points: <span class="k">array</span>&lt;<span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;,
    <span class="s">4</span>&gt;,
    count: <span class="k">u32</span>,<span class="c"> // corners; 0 = not drawn</span>
};

<span class="c">// Vertex \`index\` of the mesh in clip space.</span>
<span class="k">fn</span> physical_clip_corner(index: <span class="k">u32</span>) -&gt; <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt; {
    <span class="k">let</span> vertex = physical_indices[index];
<span class="c">    // ten floats per vertex: position, normal, color</span>
    <span class="k">let</span> base = vertex * <span class="s">10u</span>;
    <span class="k">let</span> point = <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(physical_vertices[base], physical_vertices[base+<span class="s">1u</span>], physical_vertices[base+<span class="s">2u</span>]);
    <span class="k">let</span> owner = physical_objects[vertex];
    <span class="k">let</span> world = (instances[owner].model * <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(point, <span class="s">1.0</span>)).xyz + translations[owner].xyz;
    <span class="k">return</span> mvp * <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(world, <span class="s">1.0</span>);
}

<span class="c">// Triangle \`primitive\` (one-based) projected and clipped to the near plane.</span>
<span class="k">fn</span> project_physical_triangle(primitive: <span class="k">u32</span>) -&gt; ProjectedPolygon {
    <span class="k">var</span> polygon: ProjectedPolygon;

    <span class="k">if</span> (primitive == <span class="s">0u</span> || primitive &gt; arrayLength(&amp;physical_indices)/<span class="s">3u</span>) {
        <span class="k">return</span> polygon;
    }

    <span class="k">let</span> base = (primitive-<span class="s">1u</span>)*<span class="s">3u</span>;
    <span class="k">let</span> owner = physical_objects[physical_indices[base]];

<span class="c">    // 2 = FLAG_HIDDEN; hidden objects do not occlude</span>
    <span class="k">if</span> ((instances[owner].flags &amp; <span class="s">2u</span>) != <span class="s">0u</span>) {
        <span class="k">return</span> polygon;
    }

    <span class="k">let</span> input = <span class="k">array</span>&lt;<span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;, <span class="s">3</span>&gt;(physical_clip_corner(base), physical_clip_corner(base+<span class="s">1u</span>), physical_clip_corner(base+<span class="s">2u</span>));
    <span class="k">var</span> clipped: <span class="k">array</span>&lt;<span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;, <span class="s">4</span>&gt;;
    <span class="k">var</span> previous = input[<span class="s">2</span>];
    <span class="k">var</span> previous_distance = previous.w - previous.z;

<span class="c">    // clip each edge against the near plane</span>
    <span class="k">for</span> (<span class="k">var</span> i = <span class="s">0u</span>; i&lt;<span class="s">3u</span>; i++) {
        <span class="k">let</span> current = input[i];
        <span class="k">let</span> distance = current.w - current.z;

        <span class="k">if</span> ((distance &gt; <span class="s">0.0</span> &amp;&amp; previous_distance &lt; <span class="s">0.0</span>) || (distance &lt; <span class="s">0.0</span> &amp;&amp; previous_distance &gt; <span class="s">0.0</span>)) {
            <span class="k">let</span> t = previous_distance / (previous_distance-distance);
            clipped[polygon.count] = mix(previous, current, t);
            polygon.count++;
        }

        <span class="k">if</span> (distance &gt;= <span class="s">0.0</span>) {
            clipped[polygon.count] = current;
            polygon.count++;
        }

        previous = current;
        previous_distance = distance;
    }

<span class="c">    // clip space to screen pixels</span>
    <span class="k">for</span> (<span class="k">var</span> i = <span class="s">0u</span>; i&lt;polygon.count; i++) {
        <span class="k">let</span> clip = clipped[i];

        <span class="k">if</span> (clip.w &lt;= <span class="s">0.0</span>) {
            polygon.count = <span class="s">0u</span>;
            <span class="k">return</span> polygon;
        }

        <span class="k">let</span> ndc = clip.xyz/clip.w;
        polygon.points[i] = <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;((ndc.x*<span class="s">0.5</span>+<span class="s">0.5</span>)*line.vp_w, (<span class="s">0.5</span>-ndc.y*<span class="s">0.5</span>)*line.vp_h, ndc.z);
    }

    <span class="k">return</span> polygon;
}

<span class="c">// 2D cross product.</span>
<span class="k">fn</span> physical_cross(a: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;, b: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;) -&gt; <span class="k">f32</span> {
    <span class="k">return</span> a.x*b.y-a.y*b.x;
}

<span class="c">// Twice the signed area of the first three corners.</span>
<span class="k">fn</span> physical_polygon_area(polygon: ProjectedPolygon) -&gt; <span class="k">f32</span> {
<span class="c">    // zero only for a degenerate polygon</span>
    <span class="k">return</span> physical_cross(polygon.points[<span class="s">1</span>].xy-polygon.points[<span class="s">0</span>].xy, polygon.points[<span class="s">2</span>].xy-polygon.points[<span class="s">0</span>].xy);
}</code></pre></div>
<h2 id="step-10-srcshaderstriangle_tileswgsl">Step 10 · src/shaders/triangle_tiles.wgsl<a class="anchor" href="#/course/18-finite-visibility#step-10-srcshaderstriangle_tileswgsl" aria-label="Link to this section">#</a></h2>
<p>Tile passes count and store projected triangle references.</p>
<p><code>lessons/18/src/shaders/triangle_tiles.wgsl</code> · 113 lines · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>@group(<span class="s">0</span>) @binding(<span class="s">0</span>) <span class="k">var</span>&lt;<span class="k">uniform</span>&gt; mvp: <span class="k">mat4x4</span>&lt;<span class="k">f32</span>&gt;;<span class="c"> // camera matrix</span>

<span class="c">// The first five fields of LineUniform; this shader needs no more.</span>
<span class="k">struct</span> TileLine {
    thickness: <span class="k">f32</span>,<span class="c"> // pen width, px</span>
    proj_y: <span class="k">f32</span>,<span class="c"> // perspective scale factor</span>
    ortho_h: <span class="k">f32</span>,<span class="c"> // ortho half-height; 0 = perspective</span>
    vp_h: <span class="k">f32</span>,<span class="c"> // target height, px</span>
    vp_w: <span class="k">f32</span>,<span class="c"> // target width, px</span>
};

@group(<span class="s">1</span>) @binding(<span class="s">0</span>) <span class="k">var</span>&lt;<span class="k">uniform</span>&gt; line: TileLine;<span class="c"> // view settings</span>
@group(<span class="s">3</span>) @binding(<span class="s">0</span>) <span class="k">var</span>&lt;<span class="k">storage</span>, <span class="k">read</span>&gt; projected: <span class="k">array</span>&lt;ProjectedTriangle&gt;;<span class="c"> // every triangle in screen space</span>

<span class="c">// One tile record: 0 count, 1 list start, 2 write cursor, 3 overflow.</span>
<span class="k">struct</span> TileRecord {
    values: <span class="k">array</span>&lt;<span class="k">atomic</span>&lt;<span class="k">u32</span>&gt;, <span class="s">4</span>&gt;
};

@group(<span class="s">3</span>) @binding(<span class="s">1</span>) <span class="k">var</span>&lt;<span class="k">storage</span>, <span class="k">read_write</span>&gt; tile_records: <span class="k">array</span>&lt;TileRecord&gt;;

<span class="c">// the triangle as the fragment shader sees it</span>
<span class="k">struct</span> TileVertex {
    @builtin(position) clip: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // position in the tile grid</span>
    @location(<span class="s">0</span>) @interpolate(flat) edge0: <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // edge line equation</span>
    @location(<span class="s">1</span>) @interpolate(flat) edge1: <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // edge line equation</span>
    @location(<span class="s">2</span>) @interpolate(flat) edge2: <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // edge line equation</span>
    @location(<span class="s">3</span>) @interpolate(flat) edge3: <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // fourth edge, if clipped</span>
    @location(<span class="s">4</span>) @interpolate(flat) bounds: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // screen box</span>
    @location(<span class="s">5</span>) @interpolate(flat) primitive: <span class="k">u32</span>,<span class="c"> // triangle index + 1</span>
    @location(<span class="s">6</span>) @interpolate(flat) gradient: <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // depth slope and nearest depth</span>
    @location(<span class="s">7</span>) @interpolate(flat) reference: <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // reference point and its depth</span>
};

@vertex
<span class="c">// A quad over the tiles the triangle's box touches; one instance per triangle.</span>
<span class="k">fn</span> vs_main(@builtin(vertex_index) vertex: <span class="k">u32</span>, @builtin(instance_index) instance: <span class="k">u32</span>) -&gt; TileVertex {
    <span class="k">let</span> triangle = projected[instance];
    <span class="k">var</span> out: TileVertex;
    out.clip = <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(<span class="s">2.0</span>, <span class="s">2.0</span>, <span class="s">0.0</span>, <span class="s">1.0</span>);

<span class="c">    // empty triangle: off screen</span>
    <span class="k">if</span> (triangle.edge3.w&lt;<span class="s">3.0</span>) {
        <span class="k">return</span> out;
    }

<span class="c">    // tile grid size</span>
    <span class="k">let</span> size = ceil(<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(line.vp_w, line.vp_h)/f32(visibility_tile_span()));
    <span class="k">let</span> lo = clamp(floor((triangle.bounds.xy-<span class="s">0.00390625</span>)/f32(visibility_tile_span())), <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>), size);
    <span class="k">let</span> hi = clamp(floor((triangle.bounds.zw+<span class="s">0.00390625</span>)/f32(visibility_tile_span()))+<span class="s">1.0</span>, <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>), size);
    <span class="k">let</span> corners = <span class="k">array</span>&lt;<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;, <span class="s">6</span>&gt;(<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>, <span class="s">0.0</span>), <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">1.0</span>, <span class="s">0.0</span>), <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>, <span class="s">1.0</span>), <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>, <span class="s">1.0</span>), <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">1.0</span>, <span class="s">0.0</span>), <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">1.0</span>, <span class="s">1.0</span>));
    <span class="k">let</span> position = mix(lo, hi, corners[vertex])/size;
    out.clip = <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(position.x*<span class="s">2.0</span>-<span class="s">1.0</span>, <span class="s">1.0</span>-position.y*<span class="s">2.0</span>, <span class="s">0.0</span>, <span class="s">1.0</span>);
    out.edge0 = triangle.edge0.xyz;
    out.edge1 = triangle.edge1.xyz;
    out.edge2 = triangle.edge2.xyz;
    out.edge3 = triangle.edge3.xyz;
    out.bounds = triangle.bounds;
    out.primitive = instance+<span class="s">1u</span>;
    out.gradient = triangle.gradient.xyz;
    out.reference = <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(triangle.edge0.w, triangle.edge1.w, triangle.edge2.w);
    <span class="k">return</span> out;
}

<span class="c">// True when a whole tile lies outside one edge.</span>
<span class="k">fn</span> tile_outside(edge: <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;, centre: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;) -&gt; <span class="k">bool</span> {
    <span class="k">return</span> dot(edge.xy, centre)+edge.z+<span class="s">0.5</span>*f32(visibility_tile_span())*(abs(edge.x)+abs(edge.y)) &lt; -<span class="s">0.00390625</span>;
}

<span class="c">// Record index of this fragment's tile; discards tiles the triangle misses.</span>
<span class="k">fn</span> covered_tile(v: TileVertex) -&gt; <span class="k">u32</span> {
    <span class="k">let</span> centre = (floor(v.clip.xy)+<span class="s">0.5</span>)*f32(visibility_tile_span());

    <span class="k">if</span> (tile_outside(v.edge0, centre) || tile_outside(v.edge1, centre) || tile_outside(v.edge2, centre) || tile_outside(v.edge3, centre)) {
        <span class="k">discard</span>;
    }

    <span class="k">let</span> size = <span class="k">vec2</span>&lt;<span class="k">u32</span>&gt;(ceil(<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(line.vp_w, line.vp_h)/f32(visibility_tile_span())));
    <span class="k">let</span> tile = <span class="s">1u</span>+u32(v.clip.y)*size.x+u32(v.clip.x);
    <span class="k">return</span> tile;
}

@fragment
<span class="c">// Count one triangle for the tile.</span>
<span class="k">fn</span> fs_count(v: TileVertex) -&gt; @location(<span class="s">0</span>) <span class="k">f32</span> {
    <span class="k">let</span> tile = covered_tile(v);
    atomicAdd(&amp;tile_records[tile].values[<span class="s">0</span>], <span class="s">1u</span>);
    <span class="k">return</span> <span class="s">0.0</span>;
}

@fragment
<span class="c">// Append (triangle, nearest depth) to the tile's list.</span>
<span class="k">fn</span> fs_fill(v: TileVertex) -&gt; @location(<span class="s">0</span>) <span class="k">f32</span> {
    <span class="k">let</span> tile = covered_tile(v);

<span class="c">    // list overflowed: stop</span>
    <span class="k">if</span> (atomicLoad(&amp;tile_records[tile].values[<span class="s">3</span>])!=<span class="s">0u</span>) {
        <span class="k">return</span> <span class="s">0.0</span>;
    }

    <span class="k">let</span> cursor = atomicAdd(&amp;tile_records[tile].values[<span class="s">2</span>], <span class="s">1u</span>);

<span class="c">    // more writes than counted: mark overflow</span>
    <span class="k">if</span> (cursor&gt;=atomicLoad(&amp;tile_records[tile].values[<span class="s">0</span>])) {
        atomicStore(&amp;tile_records[tile].values[<span class="s">3</span>], <span class="s">1u</span>);
        <span class="k">return</span> <span class="s">0.0</span>;
    }

    <span class="k">let</span> offset = atomicLoad(&amp;tile_records[tile].values[<span class="s">1</span>])+cursor*<span class="s">2u</span>;
    atomicStore(&amp;tile_records[offset/<span class="s">4u</span>].values[offset%<span class="s">4u</span>], v.primitive);
    <span class="k">let</span> centre = (floor(v.clip.xy)+<span class="s">0.5</span>)*f32(visibility_tile_span());
    <span class="k">let</span> slope = abs(v.gradient.x)+abs(v.gradient.y);
<span class="c">    // nearest depth the triangle reaches in this tile</span>
    <span class="k">let</span> nearest = min(v.gradient.z, v.reference.z+dot(v.gradient.xy, centre-v.reference.xy)+<span class="s">0.5</span>*f32(visibility_tile_span())*slope);
<span class="c">    // plus float and vertex-snap tolerance</span>
    <span class="k">let</span> bound = nearest+abs(nearest)*<span class="s">1.9073486e-6</span>+slope*<span class="s">0.00390625</span>;
    atomicStore(&amp;tile_records[(offset+<span class="s">1u</span>)/<span class="s">4u</span>].values[(offset+<span class="s">1u</span>)%<span class="s">4u</span>], <span class="k">bitcast</span>&lt;<span class="k">u32</span>&gt;(bound));
    <span class="k">return</span> <span class="s">0.0</span>;
}</code></pre></div>
<h2 id="step-11-srcshadersscan_triangle_tileswgsl">Step 11 · src/shaders/scan_triangle_tiles.wgsl<a class="anchor" href="#/course/18-finite-visibility#step-11-srcshadersscan_triangle_tileswgsl" aria-label="Link to this section">#</a></h2>
<p>A prefix scan assigns each tile its reference range.</p>
<p><code>lessons/18/src/shaders/scan_triangle_tiles.wgsl</code> · 120 lines · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// The first five fields of LineUniform; this shader needs no more.</span>
<span class="k">struct</span> ScanLine {
    thickness: <span class="k">f32</span>,<span class="c"> // pen width, px</span>
    proj_y: <span class="k">f32</span>,<span class="c"> // perspective scale factor</span>
    ortho_h: <span class="k">f32</span>,<span class="c"> // ortho half-height; 0 = perspective</span>
    vp_h: <span class="k">f32</span>,<span class="c"> // target height, px</span>
    vp_w: <span class="k">f32</span>,<span class="c"> // target width, px</span>
};

@group(<span class="s">0</span>) @binding(<span class="s">0</span>) <span class="k">var</span>&lt;<span class="k">uniform</span>&gt; line: ScanLine;<span class="c"> // view settings</span>

<span class="c">// One 16-byte tile record: count, list start, cursor, overflow.</span>
<span class="k">struct</span> ScanRecord {
    values: <span class="k">array</span>&lt;<span class="k">u32</span>, <span class="s">4</span>&gt;
};

@group(<span class="s">1</span>) @binding(<span class="s">0</span>) <span class="k">var</span>&lt;<span class="k">storage</span>, <span class="k">read_write</span>&gt; records: <span class="k">array</span>&lt;ScanRecord&gt;;<span class="c"> // tile records, then the reference pool</span>
<span class="k">var</span>&lt;<span class="k">workgroup</span>&gt; scan: <span class="k">array</span>&lt;<span class="k">u32</span>, <span class="s">256</span>&gt;;<span class="c"> // scratch for one workgroup's prefix sum</span>

<span class="c">// Tiles in the grid.</span>
<span class="k">fn</span> tile_count() -&gt; <span class="k">u32</span> {
    <span class="k">return</span> u32(ceil(line.vp_w/f32(visibility_tile_span())))*u32(ceil(line.vp_h/f32(visibility_tile_span())));
}

<span class="c">// Workgroups of 256 needed for \`count\` items.</span>
<span class="k">fn</span> blocks(count: <span class="k">u32</span>) -&gt; <span class="k">u32</span> {
    <span class="k">return</span> (count+<span class="s">255u</span>)/<span class="s">256u</span>;
}

<span class="c">// Exclusive prefix sum across the workgroup; saturates at the buffer size.</span>
<span class="k">fn</span> prefix(lane: <span class="k">u32</span>, value: <span class="k">u32</span>) -&gt; <span class="k">u32</span> {
<span class="c">    // an overflowing sum sticks at capacity instead of wrapping</span>
    <span class="k">let</span> capacity = arrayLength(&amp;records)*<span class="s">4u</span>;
    <span class="k">let</span> bounded = min(value, capacity);
    scan[lane] = bounded;
    workgroupBarrier();

    <span class="k">for</span> (<span class="k">var</span> stride = <span class="s">1u</span>;stride&lt;<span class="s">256u</span>;stride*=<span class="s">2u</span>) {
        <span class="k">var</span> previous = <span class="s">0u</span>;

        <span class="k">if</span> (lane&gt;=stride) {
            previous = scan[lane-stride];
        }

        workgroupBarrier();
        scan[lane] = min(scan[lane]+previous, capacity);
        workgroupBarrier();
    }

    <span class="k">return</span> select(scan[lane]-bounded, capacity, scan[lane]==capacity);
}

@compute @workgroup_size(<span class="s">256</span>)
<span class="c">// Pass 1: prefix sum of tile counts inside each block of 256 tiles.</span>
<span class="k">fn</span> scan_tiles(@builtin(global_invocation_id) id: <span class="k">vec3</span>&lt;<span class="k">u32</span>&gt;, @builtin(local_invocation_index) lane: <span class="k">u32</span>, @builtin(workgroup_id) group: <span class="k">vec3</span>&lt;<span class="k">u32</span>&gt;) {
    <span class="k">let</span> count = tile_count();
    <span class="k">var</span> value = <span class="s">0u</span>;

    <span class="k">if</span> (id.x&lt;count) {
<span class="c">        // two words per reference</span>
        value = records[<span class="s">1u</span>+id.x].values[<span class="s">0</span>]*<span class="s">2u</span>;
    }

    <span class="k">let</span> offset = prefix(lane, value);

    <span class="k">if</span> (id.x&lt;count) {
        records[<span class="s">1u</span>+id.x].values[<span class="s">1</span>] = offset;
    }

<span class="c">    // block total, for pass 2</span>
    <span class="k">if</span> (lane==<span class="s">255u</span>) {
        records[<span class="s">1u</span>+count+group.x].values[<span class="s">0</span>] = scan[<span class="s">255</span>];
    }
}

@compute @workgroup_size(<span class="s">256</span>)
<span class="c">// Pass 2: prefix sum of the block totals.</span>
<span class="k">fn</span> scan_blocks(@builtin(global_invocation_id) id: <span class="k">vec3</span>&lt;<span class="k">u32</span>&gt;, @builtin(local_invocation_index) lane: <span class="k">u32</span>, @builtin(workgroup_id) group: <span class="k">vec3</span>&lt;<span class="k">u32</span>&gt;) {
    <span class="k">let</span> count = tile_count();
    <span class="k">let</span> block_count = blocks(count);
    <span class="k">var</span> value = <span class="s">0u</span>;

    <span class="k">if</span> (id.x&lt;block_count) {
        value = records[<span class="s">1u</span>+count+id.x].values[<span class="s">0</span>];
    }

    <span class="k">let</span> offset = prefix(lane, value);

    <span class="k">if</span> (id.x&lt;block_count) {
        records[<span class="s">1u</span>+count+id.x].values[<span class="s">1</span>] = offset;
    }

    <span class="k">if</span> (lane==<span class="s">255u</span>) {
        records[<span class="s">1u</span>+count+block_count+group.x].values[<span class="s">0</span>] = scan[<span class="s">255</span>];
    }
}

@compute @workgroup_size(<span class="s">256</span>)
<span class="c">// Pass 3: each tile's list start = pool start + block offset + tile offset.</span>
<span class="k">fn</span> finish_offsets(@builtin(global_invocation_id) id: <span class="k">vec3</span>&lt;<span class="k">u32</span>&gt;) {
    <span class="k">let</span> count = tile_count();

    <span class="k">if</span> (id.x==<span class="s">0u</span>) {
<span class="c">        // record 0: valid flag</span>
        records[<span class="s">0</span>].values[<span class="s">0</span>] = <span class="s">1u</span>;
    }

    <span class="k">if</span> (id.x&gt;=count) {
        <span class="k">return</span>;
    }

    <span class="k">let</span> block_count = blocks(count);
    <span class="k">let</span> pool = <span class="s">4u</span>*(<span class="s">1u</span>+count+block_count+blocks(block_count));
    <span class="k">let</span> tile = <span class="s">1u</span>+id.x;
    <span class="k">let</span> <span class="k">block</span> = id.x/<span class="s">256u</span>;
    <span class="k">var</span> offset = records[tile].values[<span class="s">1</span>]+records[<span class="s">1u</span>+count+<span class="k">block</span>].values[<span class="s">1</span>];

    <span class="k">for</span> (<span class="k">var</span> i = <span class="s">0u</span>;i&lt;<span class="k">block</span>/<span class="s">256u</span>;i++) {
        offset+=records[<span class="s">1u</span>+count+block_count+i].values[<span class="s">0</span>];
    }

    <span class="k">let</span> start = pool+offset;
    records[tile].values[<span class="s">1</span>] = start;
<span class="c">    // overflow flag when the list runs past the buffer</span>
    records[tile].values[<span class="s">3</span>] = select(<span class="s">0u</span>, <span class="s">1u</span>, start+records[tile].values[<span class="s">0</span>]*<span class="s">2u</span>&gt;arrayLength(&amp;records)*<span class="s">4u</span>);

<span class="c">    // record 0 word 1: words needed; the CPU reads it back</span>
    <span class="k">if</span> (id.x==count-<span class="s">1u</span>) {
        records[<span class="s">0</span>].values[<span class="s">1</span>] = start+records[tile].values[<span class="s">0</span>]*<span class="s">2u</span>;
    }
}</code></pre></div>
<h2 id="step-12-srcenginegputriangle_tilesrs">Step 12 · src/engine/gpu/triangle_tiles.rs<a class="anchor" href="#/course/18-finite-visibility#step-12-srcenginegputriangle_tilesrs" aria-label="Link to this section">#</a></h2>
<p>Triangle tiles limit visibility queries to finite projected geometry.</p>
<p><code>lessons/18/src/engine/gpu/triangle_tiles.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! Splits the screen into tiles and lists which triangles touch each tile, so a pixel tests a few triangles instead of all.</span>
<span class="k">use</span> super::buffers::{GpuCtx, ROWS, bind_group, uniform_buffer, zeroed_buffer};
<span class="k">use</span> super::frame::Binds;
<span class="k">use</span> <span class="k">crate</span>::engine::pipelines::{
    ColorWrite, DepthMode, Layouts, PipelineDesc, Target, build, pipeline_layout,
};
<span class="k">use</span> std::sync::Arc;
<span class="k">use</span> std::sync::atomic::{AtomicU8, Ordering};

<span class="c">/// Bytes per projected triangle record.</span>
<span class="k">pub</span>(super) <span class="k">const</span> PROJECTED_BYTES: u64 = <span class="s">96</span>;

<span class="c">/// Most screen tiles the grid may have.</span>
<span class="k">const</span> MAX_TILES: u32 = <span class="s">262_144</span>;

<span class="c">/// Most triangle references per tile on average.</span>
<span class="k">const</span> REFERENCES_PER_TILE: u64 = <span class="s">32</span>;

<span class="c">/// Words per reference: triangle index and depth.</span>
<span class="k">const</span> REFERENCE_WORDS: u64 = <span class="s">2</span>;

<span class="c">/// Smallest reference pool, in words.</span>
<span class="k">const</span> MIN_POOL_WORDS: u64 = <span class="s">32</span> * <span class="s">1024</span>;

<span class="c">/// The screen tile grid: tile count and pixels per tile.</span>
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
<span class="k">struct</span> TileLayout {
    width: u32, <span class="c">// tiles across</span>
    height: u32, <span class="c">// tiles down</span>
    span: u32, <span class="c">// pixels per tile side</span>
}

<span class="k">impl</span> TileLayout {
    <span class="c">/// Grid for a canvas size; tiles grow until the count fits.</span>
    <span class="k">fn</span> new(size: (u32, u32)) -&gt; <span class="k">Self</span> {
        <span class="k">let</span> <span class="k">mut</span> span = <span class="s">4</span>;

        <span class="k">while</span> size.<span class="s">0</span>.div_ceil(span) <span class="k">as</span> u64 * size.<span class="s">1</span>.div_ceil(span) <span class="k">as</span> u64 &gt; MAX_TILES <span class="k">as</span> u64 {
            span *= <span class="s">2</span>;
        }

        <span class="k">Self</span> {
            width: size.<span class="s">0</span>.div_ceil(span).max(<span class="s">1</span>),
            height: size.<span class="s">1</span>.div_ceil(span).max(<span class="s">1</span>),
            span,
        }
    }

    <span class="c">/// Tiles in the grid.</span>
    <span class="k">fn</span> count(<span class="k">self</span>) -&gt; u32 {
        <span class="k">self</span>.width * <span class="k">self</span>.height
    }

    <span class="c">/// 16-byte records before the pool: one per tile plus scan totals.</span>
    <span class="k">fn</span> header_records(<span class="k">self</span>) -&gt; u64 {
        <span class="k">let</span> blocks = <span class="k">self</span>.count().div_ceil(<span class="s">256</span>);
        <span class="s">1</span> + <span class="k">self</span>.count() <span class="k">as</span> u64 + blocks <span class="k">as</span> u64 + blocks.div_ceil(<span class="s">256</span>) <span class="k">as</span> u64
    }</code></pre></div>
<p><code>lessons/18/src/engine/gpu/triangle_tiles.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Bytes of the tile buffer with a pool of \`pool_words\`.</span>
    <span class="k">fn</span> buffer_bytes(<span class="k">self</span>, pool_words: u64) -&gt; u64 {
        <span class="k">self</span>.header_records() * <span class="s">16</span> + pool_words * <span class="s">4</span>
    }

    <span class="c">/// Largest pool, in words.</span>
    <span class="k">fn</span> max_pool_words(<span class="k">self</span>) -&gt; u64 {
        <span class="k">self</span>.count() <span class="k">as</span> u64 * REFERENCES_PER_TILE * REFERENCE_WORDS
    }

    <span class="c">/// First pool size for \`triangles\`, in words.</span>
    <span class="k">fn</span> initial_pool_words(<span class="k">self</span>, triangles: u32) -&gt; u64 {
        <span class="k">let</span> references = <span class="k">self</span>.count() <span class="k">as</span> u64 * <span class="s">2</span> + u64::from(triangles) * <span class="s">8</span>;
        (references * REFERENCE_WORDS)
            .max(MIN_POOL_WORDS)
            .min(<span class="k">self</span>.max_pool_words())
    }
}

<span class="c">/// Reads back how many words the last scan needed.</span>
<span class="k">struct</span> PoolReport {</code></pre></div>
<p><code>lessons/18/src/engine/gpu/triangle_tiles.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    buffer: wgpu::Buffer, <span class="c">// 16-byte CPU-readable copy</span>
    ready: Arc&lt;AtomicU8&gt;, <span class="c">// 0 waiting, 1 mapped, 2 failed</span>
    copied: bool, <span class="c">// a copy was encoded this frame</span>
    inflight: bool, <span class="c">// the copy is being mapped</span>
}

<span class="k">impl</span> PoolReport {
    <span class="c">/// Create the readback buffer.</span>
    <span class="k">fn</span> new(ctx: &amp;GpuCtx) -&gt; <span class="k">Self</span> {
        <span class="k">Self</span> {
            buffer: ctx.device.create_buffer(&amp;wgpu::BufferDescriptor {
                label: Some(&quot;<span class="s">triangle.tiles.report</span>&quot;),
                size: <span class="s">16</span>,
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
                mapped_at_creation: <span class="s">false</span>,
            }),
            ready: Arc::new(AtomicU8::new(<span class="s">0</span>)),
            copied: <span class="s">false</span>,
            inflight: <span class="s">false</span>,
        }
    }

    <span class="c">/// Copy the first tile record into the readback buffer.</span>
    <span class="k">fn</span> copy(&amp;<span class="k">mut</span> <span class="k">self</span>, encoder: &amp;<span class="k">mut</span> wgpu::CommandEncoder, tiles: &amp;wgpu::Buffer) {
        <span class="k">if</span> <span class="k">self</span>.inflight {
            <span class="k">return</span>;
        }

        encoder.copy_buffer_to_buffer(tiles, <span class="s">0</span>, &amp;<span class="k">self</span>.buffer, <span class="s">0</span>, <span class="s">16</span>);
        <span class="k">self</span>.copied = <span class="s">true</span>;
    }

    <span class="c">/// Start mapping the copy; call once after submit.</span>
    <span class="k">fn</span> map(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">if</span> !<span class="k">self</span>.copied || <span class="k">self</span>.inflight {
            <span class="k">return</span>;
        }

        <span class="k">self</span>.copied = <span class="s">false</span>;
        <span class="k">self</span>.inflight = <span class="s">true</span>;
        <span class="k">let</span> flag = <span class="k">self</span>.ready.clone();
        <span class="k">self</span>.buffer
            .slice(..)
            .map_async(wgpu::MapMode::Read, <span class="k">move</span> |result| {
                flag.store(<span class="k">if</span> result.is_ok() { <span class="s">1</span> } <span class="k">else</span> { <span class="s">2</span> }, Ordering::Release);
            });
    }

    <span class="c">/// Words the last scan needed, once the copy is readable.</span>
    <span class="k">fn</span> poll(&amp;<span class="k">mut</span> <span class="k">self</span>) -&gt; Option&lt;u64&gt; {
        <span class="k">if</span> !<span class="k">self</span>.inflight {
            <span class="k">return</span> None;
        }

        <span class="k">let</span> status = <span class="k">self</span>.ready.load(Ordering::Acquire);

        <span class="k">if</span> status == <span class="s">0</span> {
            <span class="k">return</span> None;
        }

        <span class="k">self</span>.ready.store(<span class="s">0</span>, Ordering::Release);
        <span class="k">self</span>.inflight = <span class="s">false</span>;

        <span class="k">if</span> status != <span class="s">1</span> {
            <span class="k">return</span> None;
        }

        <span class="k">let</span> words = {
            <span class="k">let</span> bytes = <span class="k">self</span>.buffer.slice(..).get_mapped_range();
            u32::from_le_bytes(bytes[<span class="s">4</span>..<span class="s">8</span>].try_into().expect(&quot;<span class="s">report words</span>&quot;))
        };
        <span class="k">self</span>.buffer.unmap();
        Some(u64::from(words))
    }
}

<span class="c">/// Next pool size: at least \`floor\`; doubled when the report overflowed.</span>
<span class="k">fn</span> next_pool_words(
    current: u64,
    floor: u64,
    capacity: u64,
    report: Option&lt;u64&gt;,
    ceiling: u64,
) -&gt; u64 {
    <span class="k">let</span> <span class="k">mut</span> want = current.max(floor);

    <span class="c">// a report at or past capacity means the lists did not fit</span>
    <span class="k">if</span> <span class="k">let</span> Some(needed) = report
        &amp;&amp; needed &gt;= capacity
    {
        want = want.max(current.saturating_mul(<span class="s">2</span>));
    }

    want.min(ceiling)
}

<span class="c">/// What the tile lists were built for; same key = reuse them.</span>
#[derive(Clone, Copy, Debug, PartialEq)]
<span class="k">struct</span> ProjectionKey {
    matrix: [f32; <span class="s">16</span>],</code></pre></div>
<p><code>lessons/18/src/engine/gpu/triangle_tiles.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    objects: u64, <span class="c">// object change count</span>
}

<span class="c">/// The layouts and pipelines of the tile passes.</span>
<span class="k">struct</span> TilePipelines {
    project_layout: wgpu::BindGroupLayout, <span class="c">// vertices, rows, indices, projected, count</span>
    raster_layout: wgpu::BindGroupLayout, <span class="c">// projected, tiles</span>
    scan_layout: wgpu::BindGroupLayout, <span class="c">// tiles</span>
    project: wgpu::ComputePipeline, <span class="c">// triangles to screen space</span>
    count: wgpu::RenderPipeline, <span class="c">// count triangles per tile</span>
    fill: wgpu::RenderPipeline, <span class="c">// write triangle lists per tile</span>
    scans: [wgpu::ComputePipeline; <span class="s">3</span>], <span class="c">// prefix sum of the counts, three levels</span>
}

<span class="c">/// Which triangles cover which screen tiles, rebuilt when the camera moves.</span>
<span class="k">pub</span> <span class="k">struct</span> TriangleTiles {
    <span class="k">pub</span> buffer: wgpu::Buffer, <span class="c">// tile headers and reference pool</span>
    <span class="k">pub</span> projected: wgpu::Buffer, <span class="c">// one screen-space record per triangle</span>
    requested_triangles: u32,
    layout: Option&lt;TileLayout&gt;, <span class="c">// current grid, None when empty</span>
    target: Option&lt;wgpu::TextureView&gt;, <span class="c">// the tile texture, if any</span>
    live_count: wgpu::Buffer, <span class="c">// triangle count, for the shaders</span>
    key: Option&lt;ProjectionKey&gt;, <span class="c">// what the lists were built for</span>
    pipes: TilePipelines,
    pool_words: u64,
    report: PoolReport, <span class="c">// readback of the words needed</span>
}

<span class="k">impl</span> TriangleTiles {
    <span class="c">/// Create with tiny placeholder buffers.</span>
    <span class="k">pub</span> <span class="k">fn</span> new(ctx: &amp;GpuCtx, layouts: &amp;Layouts) -&gt; <span class="k">Self</span> {
        <span class="k">Self</span> {</code></pre></div>
<p><code>lessons/18/src/engine/gpu/triangle_tiles.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            buffer: zeroed_buffer(&amp;ctx.device, &quot;<span class="s">triangle.tiles</span>&quot;, <span class="s">16</span>, ROWS),
            projected: zeroed_buffer(&amp;ctx.device, &quot;<span class="s">triangle.projected</span>&quot;, PROJECTED_BYTES, ROWS),
            requested_triangles: <span class="s">0</span>,
            layout: None,
            target: None,
            live_count: uniform_buffer(&amp;ctx.device, &quot;<span class="s">triangle.project.count</span>&quot;, &amp;[<span class="s">0u32</span>; <span class="s">4</span>]),
            key: None,
            pipes: TilePipelines::new(ctx, layouts),
            pool_words: <span class="s">0</span>,
            report: PoolReport::new(ctx),
        }
    }

    <span class="c">/// Start reading the scan's report; call after submit.</span>
    <span class="k">pub</span> <span class="k">fn</span> map_report(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">self</span>.report.map();
    }

    <span class="c">/// Words in the buffer: headers plus pool.</span>
    <span class="k">fn</span> pool_capacity(&amp;<span class="k">self</span>, layout: TileLayout) -&gt; u64 {
        layout.header_records() * <span class="s">4</span> + <span class="k">self</span>.pool_words
    }

    <span class="c">/// Force a rebuild on the next frame.</span>
    <span class="k">pub</span> <span class="k">fn</span> invalidate(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">self</span>.key = None;
    }

    <span class="c">/// Size the buffers for \`size\` and \`triangles\`; returns true if a buffer moved.</span>
    <span class="k">pub</span> <span class="k">fn</span> prepare(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, size: (u32, u32), triangles: u32) -&gt; bool {
        <span class="k">let</span> limit = ctx.device.limits().max_storage_buffer_binding_size;

        <span class="c">// no triangles, or too many for the device: drop the tables</span>
        <span class="k">if</span> triangles == <span class="s">0</span> || triangles <span class="k">as</span> u64 * PROJECTED_BYTES &gt; limit {
            <span class="k">let</span> changed = <span class="k">self</span>.release_data(ctx);</code></pre></div>
<p><code>lessons/18/src/engine/gpu/triangle_tiles.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">if</span> triangles != <span class="k">self</span>.requested_triangles &amp;&amp; triangles &gt; <span class="s">0</span> {
                log::warn!(
                    &quot;<span class="s">Finite triangle visibility exceeds the device storage limit for </span>{<span class="s">triangles</span>}<span class="s"> triangles; retaining physical depth occlusion</span>&quot;
                );
            }

            <span class="k">self</span>.requested_triangles = triangles;
            <span class="k">return</span> changed;
        }

        <span class="k">let</span> <span class="k">mut</span> changed = <span class="s">false</span>;
        <span class="k">let</span> layout = TileLayout::new(size);
        <span class="c">// only a report for this grid counts</span>
        <span class="k">let</span> report = <span class="k">match</span> <span class="k">self</span>.report.poll() {
            Some(needed) <span class="k">if</span> <span class="k">self</span>.layout == Some(layout) =&gt; Some(needed),
            <span class="c">// a different grid's report is stale</span>
            _ =&gt; None,
        };
        <span class="k">let</span> pool_words = next_pool_words(
            <span class="k">self</span>.pool_words,
            layout.initial_pool_words(triangles),
            <span class="k">self</span>.pool_capacity(layout),
            report,
            layout.max_pool_words(),
        );
        <span class="k">let</span> grow = pool_words &gt; <span class="k">self</span>.pool_words;

        <span class="c">// new triangle count: new projected table</span>
        <span class="k">if</span> triangles != <span class="k">self</span>.requested_triangles || <span class="k">self</span>.layout.is_none() {
            <span class="k">self</span>.projected = zeroed_buffer(
                &amp;ctx.device,
                &quot;<span class="s">triangle.projected</span>&quot;,
                triangles <span class="k">as</span> u64 * PROJECTED_BYTES,
                ROWS,
            );
            ctx.queue.write_buffer(
                &amp;<span class="k">self</span>.live_count,
                <span class="s">0</span>,
                bytemuck::cast_slice(&amp;[triangles, <span class="s">0u32</span>, <span class="s">0</span>, <span class="s">0</span>]),
            );
            <span class="k">self</span>.requested_triangles = triangles;
            <span class="k">self</span>.invalidate();
            changed = <span class="s">true</span>;
        }

        <span class="c">// new grid or bigger pool: new tile buffer</span>
        <span class="k">if</span> <span class="k">self</span>.layout != Some(layout) || grow {
            <span class="k">self</span>.pool_words = pool_words;
            <span class="k">self</span>.buffer = zeroed_buffer(
                &amp;ctx.device,
                &quot;<span class="s">triangle.tiles</span>&quot;,
                layout.buffer_bytes(pool_words).min(limit),
                ROWS,
            );
            <span class="k">self</span>.target = Some(
                ctx.device
                    .create_texture(&amp;wgpu::TextureDescriptor {
                        label: Some(&quot;<span class="s">triangle.tiles.target</span>&quot;), <span class="c">// debug name</span>
                        size: wgpu::Extent3d {
                            width: layout.width, <span class="c">// tiles across</span>
                            height: layout.height, <span class="c">// tiles down</span>
                            depth_or_array_layers: <span class="s">1</span>,
                        },
                        mip_level_count: <span class="s">1</span>,
                        sample_count: <span class="s">1</span>,
                        dimension: wgpu::TextureDimension::D2,
                        format: wgpu::TextureFormat::R8Unorm,
                        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                        view_formats: &amp;[],
                    })
                    .create_view(&amp;Default::default()),
            );
            <span class="k">self</span>.layout = Some(layout);
            <span class="k">self</span>.invalidate();
            changed = <span class="s">true</span>;
        }

        changed
    }

    <span class="c">/// Rebuild the tile lists unless camera and objects are unchanged.</span>
    <span class="k">pub</span>(super) <span class="k">fn</span> encode(
        &amp;<span class="k">mut</span> <span class="k">self</span>,
        ctx: &amp;GpuCtx,
        encoder: &amp;<span class="k">mut</span> wgpu::CommandEncoder,</code></pre></div>
<p><code>lessons/18/src/engine/gpu/triangle_tiles.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        input: TileInput&lt;'_&gt;,
    ) {
        <span class="k">let</span> Some(layout) = <span class="k">self</span>.layout <span class="k">else</span> {
            <span class="k">return</span>;
        };
        <span class="k">let</span> key = ProjectionKey {
            matrix: input.matrix,
            objects: input.objects_revision,
        };

        <span class="k">if</span> <span class="k">self</span>.key == Some(key) {
            <span class="k">return</span>;
        }

        <span class="k">let</span> group = bind_group(
            ctx,
            &amp;<span class="k">self</span>.pipes.project_layout,
            &quot;<span class="s">triangle.project.bindings</span>&quot;,
            &amp;[
                input.geometry[<span class="s">0</span>],
                input.geometry[<span class="s">1</span>],
                input.geometry[<span class="s">2</span>],
                &amp;<span class="k">self</span>.projected,
                &amp;<span class="k">self</span>.live_count,
            ],
        );
        {
            <span class="c">// 1: project every triangle to the screen</span>
            <span class="k">let</span> <span class="k">mut</span> pass = encoder.begin_compute_pass(&amp;wgpu::ComputePassDescriptor {
                label: Some(&quot;<span class="s">triangle.project</span>&quot;),
                timestamp_writes: None,
            });
            pass.set_pipeline(&amp;<span class="k">self</span>.pipes.project);
            pass.set_bind_group(<span class="s">0</span>, input.binds.mvp, &amp;[]);
            pass.set_bind_group(<span class="s">1</span>, input.binds.line, &amp;[]);
            pass.set_bind_group(<span class="s">2</span>, input.binds.instances, &amp;[]);
            pass.set_bind_group(<span class="s">3</span>, &amp;group, &amp;[]);
            pass.dispatch_workgroups(<span class="k">self</span>.requested_triangles.div_ceil(<span class="s">64</span>), <span class="s">1</span>, <span class="s">1</span>);
        }
        <span class="c">// 2: zero the tile headers</span>
        encoder.clear_buffer(&amp;<span class="k">self</span>.buffer, <span class="s">0</span>, Some(layout.header_records() * <span class="s">16</span>));
        <span class="k">let</span> raster = bind_group(
            ctx,
            &amp;<span class="k">self</span>.pipes.raster_layout,
            &quot;<span class="s">triangle.tiles.bindings</span>&quot;,
            &amp;[&amp;<span class="k">self</span>.projected, &amp;<span class="k">self</span>.buffer],
        );
        <span class="c">// 3: count triangles per tile</span>
        <span class="k">self</span>.bin(encoder, input.binds, &amp;raster, &amp;<span class="k">self</span>.pipes.count);
        <span class="k">let</span> scan = bind_group(
            ctx,
            &amp;<span class="k">self</span>.pipes.scan_layout,
            &quot;<span class="s">triangle.scan.bindings</span>&quot;,
            &amp;[&amp;<span class="k">self</span>.buffer],
        );
        <span class="k">let</span> blocks = layout.count().div_ceil(<span class="s">256</span>);

        <span class="c">// 4: prefix sum gives each tile its list offset</span>
        <span class="k">for</span> (index, count) <span class="k">in</span> [blocks, blocks.div_ceil(<span class="s">256</span>), blocks]
            .into_iter()
            .enumerate()
        {
            <span class="k">let</span> <span class="k">mut</span> pass = encoder.begin_compute_pass(&amp;wgpu::ComputePassDescriptor {
                label: Some(&quot;<span class="s">triangle.scan</span>&quot;),
                timestamp_writes: None,
            });
            pass.set_pipeline(&amp;<span class="k">self</span>.pipes.scans[index]);
            pass.set_bind_group(<span class="s">0</span>, input.binds.line, &amp;[]);
            pass.set_bind_group(<span class="s">1</span>, &amp;scan, &amp;[]);
            pass.dispatch_workgroups(count, <span class="s">1</span>, <span class="s">1</span>);
        }

        <span class="c">// 5: write the triangle lists</span>
        <span class="k">self</span>.bin(encoder, input.binds, &amp;raster, &amp;<span class="k">self</span>.pipes.fill);
        <span class="k">self</span>.report.copy(encoder, &amp;<span class="k">self</span>.buffer);
        <span class="k">self</span>.key = Some(key);
    }

    <span class="c">/// Draw every triangle over the tile grid with \`pipeline\`.</span>
    <span class="k">fn</span> bin(</code></pre></div>
<p><code>lessons/18/src/engine/gpu/triangle_tiles.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        &amp;<span class="k">self</span>,
        encoder: &amp;<span class="k">mut</span> wgpu::CommandEncoder,
        binds: &amp;Binds,
        group: &amp;wgpu::BindGroup,
        pipeline: &amp;wgpu::RenderPipeline,
    ) {
        <span class="k">let</span> Some(view) = &amp;<span class="k">self</span>.target <span class="k">else</span> {
            <span class="k">return</span>;
        };
        <span class="k">let</span> <span class="k">mut</span> pass = encoder.begin_render_pass(&amp;wgpu::RenderPassDescriptor {
            label: Some(&quot;<span class="s">triangle.tiles</span>&quot;),
            color_attachments: &amp;[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Discard,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        pass.set_pipeline(pipeline);
        binds.set(&amp;<span class="k">mut</span> pass);
        pass.set_bind_group(<span class="s">3</span>, group, &amp;[]);
        <span class="c">// a quad per triangle, covering its tiles</span>
        pass.draw(<span class="s">0</span>..<span class="s">6</span>, <span class="s">0</span>..<span class="k">self</span>.requested_triangles);
    }

    <span class="c">/// Shrink the buffers back to placeholders; returns true if they were bigger.</span>
    <span class="k">fn</span> release_data(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx) -&gt; bool {
        <span class="k">if</span> <span class="k">self</span>.layout.take().is_none() {
            <span class="k">return</span> <span class="s">false</span>;
        }</code></pre></div>
<p><code>lessons/18/src/engine/gpu/triangle_tiles.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.buffer = zeroed_buffer(&amp;ctx.device, &quot;<span class="s">triangle.tiles</span>&quot;, <span class="s">16</span>, ROWS);
        <span class="k">self</span>.projected = zeroed_buffer(&amp;ctx.device, &quot;<span class="s">triangle.projected</span>&quot;, PROJECTED_BYTES, ROWS);
        <span class="k">self</span>.target = None;
        <span class="k">self</span>.invalidate();
        <span class="s">true</span>
    }

    <span class="c">/// Forget the scene and free the buffers.</span>
    <span class="k">pub</span> <span class="k">fn</span> release(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx) {
        <span class="k">self</span>.release_data(ctx);
        <span class="k">self</span>.requested_triangles = <span class="s">0</span>;
        <span class="k">self</span>.pool_words = <span class="s">0</span>;
        <span class="k">self</span>.invalidate();
    }

    <span class="c">/// Bytes reserved on the GPU: (buffers, textures).</span>
    <span class="k">pub</span> <span class="k">fn</span> allocated_bytes(&amp;<span class="k">self</span>) -&gt; (u64, u64) {
        <span class="k">let</span> pixels = <span class="k">self</span>.layout.map_or(<span class="s">0</span>, TileLayout::count) <span class="k">as</span> u64;
        (
            <span class="k">self</span>.buffer.size()
                + <span class="k">self</span>.projected.size()
                + <span class="k">self</span>.live_count.size()
                + <span class="k">self</span>.report.buffer.size(),
            pixels,
        )
    }
}

<span class="c">/// What \`encode\` needs from the frame and the arena.</span>
<span class="k">pub</span>(super) <span class="k">struct</span> TileInput&lt;'a&gt; {
    <span class="k">pub</span> binds: &amp;'a Binds&lt;'a&gt;, <span class="c">// bind groups 0-2</span>
    <span class="k">pub</span> geometry: [&amp;'a wgpu::Buffer; <span class="s">3</span>], <span class="c">// vertices, object rows, indices</span></code></pre></div>
<p><code>lessons/18/src/engine/gpu/triangle_tiles.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> matrix: [f32; <span class="s">16</span>],
    <span class="k">pub</span> objects_revision: u64, <span class="c">// object change count</span>
}

<span class="c">/// One buffer binding for a layout.</span>
<span class="k">fn</span> entry(
    binding: u32,
    visibility: wgpu::ShaderStages,
    ty: wgpu::BufferBindingType,
) -&gt; wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility,
        ty: wgpu::BindingType::Buffer {
            ty,
            has_dynamic_offset: <span class="s">false</span>,
            min_binding_size: None,
        },
        count: None,
    }
}

<span class="k">impl</span> TilePipelines {
    <span class="c">/// Create the layouts, shaders and pipelines.</span>
    <span class="k">fn</span> new(ctx: &amp;GpuCtx, layouts: &amp;Layouts) -&gt; <span class="k">Self</span> {
        <span class="k">use</span> wgpu::BufferBindingType::{Storage, Uniform};</code></pre></div>
<p><code>lessons/18/src/engine/gpu/triangle_tiles.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">use</span> wgpu::ShaderStages <span class="k">as</span> Stages;
        <span class="k">let</span> device = &amp;ctx.device;
        <span class="k">let</span> project_layout = device.create_bind_group_layout(&amp;wgpu::BindGroupLayoutDescriptor {
            label: Some(&quot;<span class="s">triangle.project.layout</span>&quot;),
            entries: &amp;[
                entry(<span class="s">0</span>, Stages::COMPUTE, Storage { read_only: <span class="s">true</span> }),
                entry(<span class="s">1</span>, Stages::COMPUTE, Storage { read_only: <span class="s">true</span> }),
                entry(<span class="s">2</span>, Stages::COMPUTE, Storage { read_only: <span class="s">true</span> }),
                entry(<span class="s">3</span>, Stages::COMPUTE, Storage { read_only: <span class="s">false</span> }),
                entry(<span class="s">4</span>, Stages::COMPUTE, Uniform),
            ],
        });
        <span class="k">let</span> raster_layout = device.create_bind_group_layout(&amp;wgpu::BindGroupLayoutDescriptor {
            label: Some(&quot;<span class="s">triangle.tiles.layout</span>&quot;),
            entries: &amp;[
                entry(<span class="s">0</span>, Stages::VERTEX, Storage { read_only: <span class="s">true</span> }),
                entry(<span class="s">1</span>, Stages::FRAGMENT, Storage { read_only: <span class="s">false</span> }),
            ],
        });
        <span class="k">let</span> scan_layout = device.create_bind_group_layout(&amp;wgpu::BindGroupLayoutDescriptor {
            label: Some(&quot;<span class="s">triangle.scan.layout</span>&quot;),
            entries: &amp;[entry(<span class="s">0</span>, Stages::COMPUTE, Storage { read_only: <span class="s">false</span> })],
        });
        <span class="k">let</span> project_shader = shader(
            device,
            &quot;<span class="s">triangle.project</span>&quot;,
            include_str!(&quot;<span class="s">../../shaders/project_triangles.wgsl</span>&quot;),
        );
        <span class="k">let</span> project_pipeline_layout = pipeline_layout(
            device,
            &quot;<span class="s">triangle.project</span>&quot;,
            &amp;[
                &amp;layouts.mvp,
                &amp;layouts.line,
                &amp;layouts.instance,
                &amp;project_layout,
            ],
        );
        <span class="k">let</span> project =
            compute_pipeline(device, &amp;project_pipeline_layout, &amp;project_shader, &quot;<span class="s">cs_main</span>&quot;);
        <span class="k">let</span> raster_shader = shader(
            device,
            &quot;<span class="s">triangle.tiles</span>&quot;,
            include_str!(&quot;<span class="s">../../shaders/triangle_tiles.wgsl</span>&quot;),
        );
        <span class="c">// the fragment shader writes buffers, not pixels</span>
        <span class="k">let</span> raster_groups = [
            &amp;layouts.mvp,
            &amp;layouts.line,
            &amp;layouts.instance,
            &amp;raster_layout,
        ];
        <span class="k">let</span> raster = PipelineDesc::new(
            &amp;raster_shader,
            &amp;raster_groups,
            &amp;[],
            wgpu::PrimitiveTopology::TriangleList,
        )
        .depth(DepthMode::Detached)
        .color(ColorWrite::Nothing);
        <span class="k">let</span> tile_target = Target {
            format: wgpu::TextureFormat::R8Unorm,
            samples: <span class="s">1</span>,
        };
        <span class="k">let</span> count = build(device, tile_target, &amp;raster.with(&quot;<span class="s">fs_count</span>&quot;, &quot;<span class="s">fs_count</span>&quot;));
        <span class="k">let</span> fill = build(device, tile_target, &amp;raster.with(&quot;<span class="s">fs_fill</span>&quot;, &quot;<span class="s">fs_fill</span>&quot;));
        <span class="k">let</span> scan_shader = shader(
            device,
            &quot;<span class="s">triangle.scan</span>&quot;,
            include_str!(&quot;<span class="s">../../shaders/scan_triangle_tiles.wgsl</span>&quot;),
        );
        <span class="k">let</span> scan_pipeline_layout =
            pipeline_layout(device, &quot;<span class="s">triangle.scan</span>&quot;, &amp;[&amp;layouts.line, &amp;scan_layout]);
        <span class="k">let</span> scans = [
            compute_pipeline(device, &amp;scan_pipeline_layout, &amp;scan_shader, &quot;<span class="s">scan_tiles</span>&quot;),
            compute_pipeline(device, &amp;scan_pipeline_layout, &amp;scan_shader, &quot;<span class="s">scan_blocks</span>&quot;),
            compute_pipeline(
                device,
                &amp;scan_pipeline_layout,
                &amp;scan_shader,
                &quot;<span class="s">finish_offsets</span>&quot;,
            ),
        ];
        <span class="k">Self</span> {
            project_layout,
            raster_layout,
            scan_layout,
            project,
            count,
            fill,
            scans,
        }
    }
}

<span class="c">/// Compile a shader with the shared projected-triangle code appended.</span>
<span class="k">fn</span> shader(device: &amp;wgpu::Device, label: &amp;str, source: &amp;str) -&gt; wgpu::ShaderModule {
    <span class="k">let</span> source = format!(
        &quot;{<span class="s">source</span>}<span class="s">\\n</span>{}&quot;,
        include_str!(&quot;<span class="s">../../shaders/projected_triangle.wgsl</span>&quot;)
    );
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some(label),
        source: wgpu::ShaderSource::Wgsl(source.into()),
    })
}</code></pre></div>
<p><code>lessons/18/src/engine/gpu/triangle_tiles.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// A compute pipeline for one entry point.</span>
<span class="k">fn</span> compute_pipeline(
    device: &amp;wgpu::Device,
    layout: &amp;wgpu::PipelineLayout,
    shader: &amp;wgpu::ShaderModule,
    entry: &amp;str,
) -&gt; wgpu::ComputePipeline {
    device.create_compute_pipeline(&amp;wgpu::ComputePipelineDescriptor {</code></pre></div>
<p>Copy this part from the lesson folder to the path shown.</p>
<p><code>lessons/18/src/engine/gpu/triangle_tiles.rs</code> · copy the file, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        label: Some(entry),
        layout: Some(layout),
        module: shader,
        entry_point: Some(entry),
        compilation_options: Default::default(),
        cache: None,
    })
}

<span class="c">/// Pool sizing and grid tests.</span>
#[cfg(test)]
<span class="k">mod</span> tests {
    <span class="k">use</span> super::*;

    <span class="c">/// Past or at capacity doubles; below it keeps the pool.</span>
    #[test]
    <span class="k">fn</span> a_saturated_report_grows_the_pool() {
        <span class="k">let</span> capacity = <span class="s">1_000</span>;
        <span class="k">let</span> pool = <span class="s">800</span>;
        assert_eq!(
            next_pool_words(pool, <span class="s">0</span>, capacity, Some(capacity + <span class="s">64</span>), u64::MAX),
            <span class="s">1_600</span>,
            &quot;<span class="s">a report past capacity doubles</span>&quot;
        );
        assert_eq!(
            next_pool_words(pool, <span class="s">0</span>, capacity, Some(capacity), u64::MAX),
            <span class="s">1_600</span>,
            &quot;<span class="s">a pool exactly filled has also run out</span>&quot;
        );
        assert_eq!(
            next_pool_words(pool, <span class="s">0</span>, capacity, Some(capacity - <span class="s">1</span>), u64::MAX),
            pool,
            &quot;<span class="s">a report below capacity measured lists that fitted</span>&quot;
        );
        assert_eq!(
            next_pool_words(pool, <span class="s">0</span>, capacity, None, u64::MAX),
            pool,
            &quot;<span class="s">no report, no change</span>&quot;
        );
    }

    <span class="c">/// Growth stops at the ceiling.</span>
    #[test]
    <span class="k">fn</span> growth_stops_at_the_ceiling() {
        <span class="k">let</span> ceiling = <span class="s">2_048</span>;
        <span class="k">let</span> <span class="k">mut</span> pool = <span class="s">512</span>;

        <span class="k">for</span> _ <span class="k">in</span> <span class="s">0</span>..<span class="s">8</span> {
            pool = next_pool_words(pool, <span class="s">0</span>, pool, Some(pool), ceiling);
        }

        assert_eq!(pool, ceiling);
        assert_eq!(next_pool_words(pool, <span class="s">0</span>, pool, Some(pool), ceiling), ceiling);
    }

    <span class="c">/// Doubling reaches ten times the size in four frames.</span>
    #[test]
    <span class="k">fn</span> doubling_converges_in_a_few_frames() {
        <span class="k">let</span> ceiling = u64::MAX;
        <span class="k">let</span> <span class="k">mut</span> pool = <span class="s">1_000</span>;
        <span class="k">let</span> <span class="k">mut</span> frames = <span class="s">0</span>;

        <span class="k">while</span> pool &lt; <span class="s">10_000</span> {
            pool = next_pool_words(pool, <span class="s">0</span>, pool, Some(pool), ceiling);
            frames += <span class="s">1</span>;
        }

        assert_eq!(frames, <span class="s">4</span>);
    }

    <span class="c">/// The floor wins when the scene grew.</span>
    #[test]
    <span class="k">fn</span> the_initial_pool_is_a_floor() {
        assert_eq!(next_pool_words(<span class="s">100</span>, <span class="s">4_096</span>, <span class="s">100</span>, None, u64::MAX), <span class="s">4_096</span>);
        assert_eq!(next_pool_words(<span class="s">8_192</span>, <span class="s">4_096</span>, <span class="s">8_192</span>, None, u64::MAX), <span class="s">8_192</span>);
    }

    #[test]
    <span class="c">/// Every canvas size gets a grid under the tile and memory limits.</span>
    <span class="k">fn</span> viewport_grid_fits_core_storage_limits_without_fixed_per_tile_caps() {
        <span class="k">for</span> size <span class="k">in</span> [
            (<span class="s">1</span>, <span class="s">1</span>),
            (<span class="s">1400</span>, <span class="s">900</span>),
            (<span class="s">1800</span>, <span class="s">1400</span>),
            (<span class="s">2800</span>, <span class="s">1800</span>),
            (<span class="s">7680</span>, <span class="s">4320</span>),
            (<span class="s">16384</span>, <span class="s">16384</span>),
        ] {
            <span class="k">let</span> layout = TileLayout::new(size);
            assert!(layout.count() &lt;= MAX_TILES);
            assert!(layout.buffer_bytes(layout.max_pool_words()) &lt; <span class="s">128</span> * <span class="s">1024</span> * <span class="s">1024</span>);
            assert!(layout.width * layout.span &gt;= size.<span class="s">0</span> &amp;&amp; layout.height * layout.span &gt;= size.<span class="s">1</span>);
        }

        assert_eq!(TileLayout::new((<span class="s">1800</span>, <span class="s">1400</span>)).span, <span class="s">4</span>);
        assert_eq!(TileLayout::new((<span class="s">2800</span>, <span class="s">1800</span>)).span, <span class="s">8</span>);
    }

    <span class="c">/// A small scene starts with a small pool.</span>
    #[test]
    <span class="k">fn</span> pool_starts_small_and_is_capped() {
        <span class="k">let</span> layout = TileLayout::new((<span class="s">3200</span>, <span class="s">2000</span>));
        <span class="k">let</span> small = layout.initial_pool_words(<span class="s">6_000</span>);
        assert!(small &lt; layout.max_pool_words() / <span class="s">8</span>);
        assert_eq!(layout.initial_pool_words(u32::MAX), layout.max_pool_words());
        assert!(layout.buffer_bytes(small) &lt; <span class="s">4</span> * <span class="s">1024</span> * <span class="s">1024</span>);
        assert_eq!(
            layout.buffer_bytes(layout.max_pool_words()),
            layout.header_records() * <span class="s">16</span> + layout.count() <span class="k">as</span> u64 * REFERENCES_PER_TILE * <span class="s">8</span>
        );
    }

    #[cfg(not(target_arch = &quot;<span class="s">wasm32</span>&quot;))]
    #[test]
    #[ignore = &quot;<span class="s">requires a native GPU adapter</span>&quot;]
    <span class="c">/// Lists are reused until camera, hiding or scene change.</span>
    <span class="k">fn</span> projection_cache_tracks_camera_hidden_state_replacement_and_release() {
        <span class="k">use</span> <span class="k">crate</span>::engine::gpu::{FrameInput, Gpu, ObjectRow, Upload};
        <span class="k">use</span> session_rust::{RenderVertex, Xform};
        <span class="k">let</span> <span class="k">mut</span> gpu = pollster::block_on(Gpu::new_headless(<span class="s">128</span>, <span class="s">128</span>)).unwrap();
        gpu.view.show_grid = <span class="s">false</span>;
        <span class="k">let</span> initial = gpu.arena.tiles.allocated_bytes();
        <span class="c">// placeholders: tiles, one record, count, report</span>
        assert_eq!(initial, (<span class="s">16</span> + PROJECTED_BYTES + <span class="s">16</span> + <span class="s">16</span>, <span class="s">0</span>));
        <span class="k">let</span> <span class="k">mut</span> upload = Upload::default();
        upload.obj.rows.push(ObjectRow::new(Xform::identity(), <span class="s">0</span>));

        <span class="k">for</span> position <span class="k">in</span> [[-<span class="s">0</span>.<span class="s">7</span>, -<span class="s">0</span>.<span class="s">7</span>, <span class="s">0</span>.<span class="s">5</span>], [<span class="s">0</span>.<span class="s">7</span>, -<span class="s">0</span>.<span class="s">7</span>, <span class="s">0</span>.<span class="s">5</span>], [<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">7</span>, <span class="s">0</span>.<span class="s">5</span>]] {
            upload.arena.verts.push(RenderVertex {
                position,
                normal: [<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>],
                color: [<span class="s">0</span>.<span class="s">5</span>; <span class="s">4</span>],
            });
            upload.arena.vids.push(<span class="s">0</span>);
        }

        upload.arena.idx.extend([<span class="s">0</span>, <span class="s">1</span>, <span class="s">2</span>]);
        gpu.set_scene(&amp;upload);
        <span class="k">let</span> <span class="k">mut</span> input = FrameInput {
            view_proj: Xform::identity(),
            clear: wgpu::Color::WHITE,
            now_ms: <span class="s">0</span>.<span class="s">0</span>,
        };
        <span class="k">let</span> visible = gpu.render_offscreen(&amp;input);
        <span class="k">let</span> key = gpu
            .arena
            .tiles
            .key
            .expect(&quot;<span class="s">a populated scene prepares visibility</span>&quot;);
        assert!(gpu.arena.tiles.allocated_bytes().<span class="s">0</span> &gt; initial.<span class="s">0</span>);
        assert_eq!(visible, gpu.render_offscreen(&amp;input));
        assert_eq!(gpu.arena.tiles.key, Some(key));
        gpu.set_selected(<span class="s">0</span>, <span class="s">true</span>);
        gpu.render_offscreen(&amp;input);
        assert_eq!(
            gpu.arena.tiles.key,
            Some(key),
            &quot;<span class="s">highlighting must not reproject source triangles</span>&quot;
        );
        gpu.set_hidden(<span class="s">0</span>, <span class="s">true</span>);
        <span class="k">let</span> hidden = gpu.render_offscreen(&amp;input);
        assert_ne!(hidden, visible);
        assert_ne!(
            gpu.arena.tiles.key,
            Some(key),
            &quot;<span class="s">a hidden occluder must leave the tile lists</span>&quot;
        );
        <span class="k">let</span> hidden_key = gpu.arena.tiles.key;
        gpu.set_hidden(<span class="s">0</span>, <span class="s">false</span>);
        input.view_proj.m[<span class="s">12</span>] = <span class="s">0</span>.<span class="s">1</span>;
        gpu.render_offscreen(&amp;input);
        assert_ne!(gpu.arena.tiles.key, hidden_key);
        gpu.resize(<span class="s">160</span>, <span class="s">96</span>);
        gpu.render_offscreen(&amp;input);
        assert_eq!(gpu.arena.tiles.layout, Some(TileLayout::new((<span class="s">160</span>, <span class="s">96</span>))));
        gpu.reset();
        gpu.set_scene(&amp;upload);
        assert!(
            gpu.arena.tiles.key.is_none(),
            &quot;<span class="s">same-count replacement invalidates projected positions</span>&quot;
        );
        gpu.render_ids_offscreen(&amp;input);
        assert!(
            gpu.arena.tiles.key.is_some(),
            &quot;<span class="s">ID-only rendering prepares its own current geometry</span>&quot;
        );
        gpu.release();
        assert_eq!(gpu.arena.tiles.allocated_bytes(), initial);
        assert!(gpu.arena.tiles.key.is_none());
    }
}</code></pre></div>
<p>Run <code>cargo check</code> in <code>lessons/18/</code>.</p>
<h2 id="step-13-srcshadersink_visibilitywgsl">Step 13 · src/shaders/ink_visibility.wgsl<a class="anchor" href="#/course/18-finite-visibility#step-13-srcshadersink_visibilitywgsl" aria-label="Link to this section">#</a></h2>
<p>The ink test decides which stroke samples are covered by faces.</p>
<p><code>lessons/18/src/shaders/ink_visibility.wgsl</code> · edit · type this</p>
<p>Added after the <code>};</code> line of <code>lessons/17/src/shaders/ink_visibility.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// the stroke as one fragment sees it</span>

<span class="c">// The stroke's center line as one fragment sees it.</span></code></pre></div>
<p>Added after the <code>};</code> line of <code>lessons/17/src/shaders/ink_visibility.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// A texel's depth; outside the viewport is cleared.</span>

<span class="c">// Scene depth at a pixel; 0 outside the screen or where nothing was drawn.</span></code></pre></div>
<p>Added after the <code>}</code> line of <code>lessons/17/src/shaders/ink_visibility.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// allowed error of the fitted plane</span>

<span class="c">// Allowed error of a depth carried \`lever\` pixels along a slope.</span></code></pre></div>
<p>Added after the <code>}</code> line of <code>lessons/17/src/shaders/ink_visibility.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// are this texel and its neighbour one surface</span>

<span class="c">// True when the pixel and its neighbour along \`dir\` are on one surface.</span></code></pre></div>
<p>Added after the <code>}</code> line of <code>lessons/17/src/shaders/ink_visibility.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// the carry's verdict for strokes and discs</span>

<span class="c">// Is the ink visible, given the surface depth carried to its center?</span></code></pre></div>
<p>Added after the <code>}</code> line of <code>lessons/17/src/shaders/ink_visibility.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// one texel step away from the stroke</span>

<span class="c">// One-pixel step away from the stroke, along x or y.</span></code></pre></div>
<p>Added after the <code>}</code> line of <code>lessons/17/src/shaders/ink_visibility.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// the surface under a stroke fragment</span>

<span class="c">// Is a stroke fragment visible under the surface.</span></code></pre></div>
<p>Added after the <code>}</code> line of <code>lessons/17/src/shaders/ink_visibility.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// the surface under a disc fragment</span>

<span class="c">// Is a disc fragment visible under the surface.</span></code></pre></div>
<p>Added after the <code>}</code> line of <code>lessons/17/src/shaders/ink_visibility.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// orthographic: parallel rays</span>

<span class="c">// Direction from a point to the camera; constant in ortho.</span></code></pre></div>
<p>Added after the <code>}</code> line of <code>lessons/17/src/shaders/ink_visibility.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// Test the footprint and its source centre at the same subpixel sample position.</span>

<span class="c">// Is a disc visible: its fragment, and its center pixel too.</span></code></pre></div>
<p>Added after the <code>}</code> line of <code>lessons/17/src/shaders/ink_visibility.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// at a corner the x and y fits may differ</span>

<span class="c">// Is a plane fitted at the center pixel in front of the disc?</span></code></pre></div>
<p><code>lessons/18/src/shaders/ink_visibility.wgsl</code> · edit · type this</p>
<p>Replaces <code>fn ink_visible</code> in <code>lessons/17/src/shaders/ink_visibility.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// Use the physical primitive's own gradient even when it covers only one sample.</span>

<span class="c">// Like ink_axis_visible, but using the stored depth slope when there is one.</span>
<span class="k">fn</span> ink_visible_plane(pixel: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;, axis: InkAxis, sample: <span class="k">u32</span>) -&gt; <span class="k">bool</span> {</code></pre></div>
<p>Added after the <code>}</code> line of <code>lessons/17/src/shaders/ink_visibility.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>@group(<span class="s">2</span>) @binding(<span class="s">6</span>) <span class="k">var</span>&lt;<span class="k">storage</span>, <span class="k">read</span>&gt; projected: <span class="k">array</span>&lt;ProjectedTriangle&gt;;<span class="c"> // every triangle in screen space</span>

<span class="c">// Return the index+1 of the exact opaque triangle that won this depth sample.</span>

<span class="c">// Triangle index + 1 at a pixel, or 0.</span>
<span class="k">fn</span> ink_primitive(pixel: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;, sample: <span class="k">u32</span>) -&gt; <span class="k">u32</span> {
    <span class="k">if</span> (any(pixel &lt; <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>)) || any(pixel &gt;= <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(line.vp_w, line.vp_h))) {
        <span class="k">return</span> <span class="s">0u</span>;
    }

    <span class="k">if</span> (SCENE_MSAA) {
        <span class="k">return</span> ink_decode_primitive(textureLoad(scene_gradient_msaa, <span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;(pixel), i32(sample)).zw);
    }

    <span class="k">return</span> ink_decode_primitive(textureLoad(scene_gradient_single, <span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;(pixel), <span class="s">0</span>).zw);
}

@group(<span class="s">2</span>) @binding(<span class="s">7</span>) <span class="k">var</span>&lt;<span class="k">storage</span>, <span class="k">read</span>&gt; triangle_tiles: <span class="k">array</span>&lt;<span class="k">vec4</span>&lt;<span class="k">u32</span>&gt;&gt;;

<span class="c">/// Is this ink fragment in front of the surface.</span>
<span class="k">fn</span> ink_visible(pixel: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;, axis: InkAxis, sample: <span class="k">u32</span>) -&gt; <span class="k">bool</span> {
    <span class="k">if</span> (ink_visible_plane(pixel, axis, sample)) {
        <span class="k">return</span> <span class="s">true</span>;
    }

<span class="c">    // no tile lists: the plane fit decides</span>
    <span class="k">if</span> (triangle_tiles[<span class="s">0</span>].x==<span class="s">0u</span>) {
        <span class="k">return</span> <span class="s">false</span>;
    }

<span class="c">    // canvas pixel of the center line</span>
    <span class="k">let</span> at = axis.at + line.origin;
<span class="c">    // the triangle under this pixel, tested exactly</span>
    <span class="k">let</span> fringe = ink_primitive(pixel, sample);

    <span class="k">if</span> (fringe==<span class="s">0u</span>) {
        <span class="k">return</span> <span class="s">false</span>;
    }

    <span class="k">let</span> fringe_hit = projected_triangle_at(projected[fringe-<span class="s">1u</span>], at);

    <span class="k">if</span> (fringe_hit.y&gt;<span class="s">0.5</span> &amp;&amp; fringe_hit.x&gt;axis.depth+abs(axis.depth)*DEPTH_REL_TOL) {
        <span class="k">return</span> <span class="s">false</span>;
    }

<span class="c">    // the four triangles around the center line, tested exactly</span>
    <span class="k">let</span> source_base = floor(axis.at-fract(pixel))+fract(pixel);

    <span class="k">for</span> (<span class="k">var</span> i = <span class="s">0u</span>;i&lt;<span class="s">4u</span>;i++) {
        <span class="k">let</span> primitive = ink_primitive(source_base+<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(f32(i&amp;<span class="s">1u</span>), f32(i&gt;&gt;<span class="s">1u</span>)), sample);

        <span class="k">if</span> (primitive==<span class="s">0u</span> || primitive==fringe) {
            <span class="k">continue</span>;
        }

        <span class="k">let</span> hit = projected_triangle_at(projected[primitive-<span class="s">1u</span>], at);

        <span class="k">if</span> (hit.y&gt;<span class="s">0.5</span> &amp;&amp; hit.x&gt;axis.depth+abs(axis.depth)*DEPTH_REL_TOL) {
            <span class="k">return</span> <span class="s">false</span>;
        }
    }

    <span class="k">if</span> (any(at&lt;<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>)) || any(at&gt;=line.frame)) {
        <span class="k">return</span> <span class="s">false</span>;
    }

    <span class="k">let</span> span = f32(visibility_tile_span_of(u32(line.frame.x), u32(line.frame.y)));
    <span class="k">let</span> size = <span class="k">vec2</span>&lt;<span class="k">u32</span>&gt;(ceil(line.frame/span));
    <span class="k">let</span> cell = <span class="k">vec2</span>&lt;<span class="k">u32</span>&gt;(at/span);
    <span class="k">let</span> head = triangle_tiles[<span class="s">1u</span>+cell.y*size.x+cell.x];

<span class="c">    // tile record: x count, y list start, z written, w overflow</span>
    <span class="k">if</span> (head.w!=<span class="s">0u</span> || head.z!=head.x) {
        <span class="k">return</span> <span class="s">false</span>;
    }

    <span class="k">for</span> (<span class="k">var</span> i = <span class="s">0u</span>;i&lt;head.x;i++) {
        <span class="k">let</span> offset = head.y+i*<span class="s">2u</span>;
        <span class="k">let</span> nearest = <span class="k">bitcast</span>&lt;<span class="k">f32</span>&gt;(triangle_tiles[(offset+<span class="s">1u</span>)/<span class="s">4u</span>][(offset+<span class="s">1u</span>)%<span class="s">4u</span>]);

<span class="c">        // triangle no nearer than the ink: skip</span>
        <span class="k">if</span> (nearest&lt;=axis.depth+abs(axis.depth)*DEPTH_REL_TOL) {
            <span class="k">continue</span>;
        }

        <span class="k">let</span> primitive = triangle_tiles[offset/<span class="s">4u</span>][offset%<span class="s">4u</span>];
        <span class="k">let</span> bounds = projected[primitive-<span class="s">1u</span>].bounds;

        <span class="k">if</span> (any(at&lt;bounds.xy-SLOPE_PX) || any(at&gt;bounds.zw+SLOPE_PX)) {
            <span class="k">continue</span>;
        }

        <span class="k">let</span> hit = projected_triangle_at(projected[primitive-<span class="s">1u</span>], at);

        <span class="k">if</span> (hit.y&gt;<span class="s">0.5</span> &amp;&amp; hit.x&gt;axis.depth+abs(axis.depth)*DEPTH_REL_TOL) {
            <span class="k">return</span> <span class="s">false</span>;
        }
    }

    <span class="k">return</span> <span class="s">true</span>;
}

<span class="c">// Triangle index from the two packed half floats, or 0.</span>
<span class="k">fn</span> ink_decode_primitive(encoded: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;) -&gt; <span class="k">u32</span> {
    <span class="k">if</span> (any(encoded==<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>))) {
        <span class="k">return</span> <span class="s">0u</span>;
    }

    <span class="k">let</span> packed = pack2x16float(encoded);
    <span class="k">return</span> ((packed&amp;<span class="s">0xffffu</span>)-<span class="s">0x400u</span>) | (((packed&gt;&gt;<span class="s">16u</span>)-<span class="s">0x400u</span>)&lt;&lt;<span class="s">14u</span>);
}</code></pre></div>
<h2 id="step-14-srcenginegpufacesrs">Step 14 · src/engine/gpu/faces.rs<a class="anchor" href="#/course/18-finite-visibility#step-14-srcenginegpufacesrs" aria-label="Link to this section">#</a></h2>
<p>Face buffers preserve source face addresses alongside triangles.</p>
<p><code>lessons/18/src/engine/gpu/faces.rs</code> · edit · type this</p>
<p>Replaces the <code>use crate::engine::pipelines::{DepthMode, Lay…</code> line of <code>lessons/17/src/engine/gpu/faces.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> <span class="k">crate</span>::engine::pipelines::{ColorWrite, DepthMode, Layouts, PipelineDesc, Target, build};</code></pre></div>
<p>Replaces the 5 lines from <code>layout: wgpu::BindGroupLayout,</code> in <code>struct Faces</code> of <code>lessons/17/src/engine/gpu/faces.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    revision: u64, <span class="c">// bumps on every selection change</span>
    layout: wgpu::BindGroupLayout,
    group: Option&lt;wgpu::BindGroup&gt;, <span class="c">// the face buffers, bound</span>
    pipes: FacePipelines,
}

<span class="c">/// The six face pipelines.</span>
<span class="k">struct</span> FacePipelines {
    physical: wgpu::RenderPipeline, <span class="c">// colored faces</span>
    object_ids: wgpu::RenderPipeline, <span class="c">// object id per pixel</span>
    pick: wgpu::RenderPipeline, <span class="c">// face id per pixel</span>
    highlight: wgpu::RenderPipeline, <span class="c">// selected face in color</span>
    mask: wgpu::RenderPipeline, <span class="c">// selected face into a mask</span>
    masks: wgpu::RenderPipeline, <span class="c">// selected face into both masks</span></code></pre></div>
<p>Replaces the 11 lines from <code>let (pick, highlight, mask) = pipelines(ctx,…</code> in <code>fn new</code> of <code>lessons/17/src/engine/gpu/faces.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> pipes = pipelines(ctx, layouts, shader, target, &amp;layout);
        <span class="k">Self</span> {
            sources: Vec::new(),
            ids: GrowBuf::new(ctx, &quot;<span class="s">source face ids</span>&quot;, <span class="s">4</span>, ROWS),
            selected,
            active: None,
            revision: <span class="s">0</span>,
            layout,
            group: None,
            pipes,</code></pre></div>
<p>Replaces the 2 lines from <code>(self.pick, self.highlight, self.mask) =</code> in <code>fn retarget</code> of <code>lessons/17/src/engine/gpu/faces.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.pipes = pipelines(ctx, layouts, shader, target, &amp;<span class="k">self</span>.layout);</code></pre></div>
<p>Added after the <code>self.active = face;</code> line in <code>fn select</code> of <code>lessons/17/src/engine/gpu/faces.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.revision = <span class="k">self</span>.revision.wrapping_add(<span class="s">1</span>);</code></pre></div>
<p>Replaces <code>fn draw_ids</code> in <code>lessons/17/src/engine/gpu/faces.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Draw the colored faces.</span>
    <span class="k">pub</span> <span class="k">fn</span> draw_physical(&amp;<span class="k">self</span>, pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;, binds: &amp;Binds) -&gt; u32 {
        <span class="k">self</span>.draw(pass, binds, &amp;<span class="k">self</span>.pipes.physical)
    }

    <span class="c">/// Draw object ids.</span>
    <span class="k">pub</span> <span class="k">fn</span> draw_object_ids(&amp;<span class="k">self</span>, pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;, binds: &amp;Binds) -&gt; u32 {
        <span class="k">self</span>.draw(pass, binds, &amp;<span class="k">self</span>.pipes.object_ids)
    }

    <span class="c">/// Draw face ids.</span>
    <span class="k">pub</span> <span class="k">fn</span> draw_ids(&amp;<span class="k">self</span>, pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;, binds: &amp;Binds) -&gt; u32 {
        <span class="k">self</span>.draw(pass, binds, &amp;<span class="k">self</span>.pipes.pick)</code></pre></div>
<p>Replaces the <code>self.draw(pass, binds, &amp;self.highlight)</code> line in <code>fn draw_highlight</code> of <code>lessons/17/src/engine/gpu/faces.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.draw(pass, binds, &amp;<span class="k">self</span>.pipes.highlight)</code></pre></div>
<p>Replaces the <code>self.draw(pass, binds, &amp;self.mask)</code> line in <code>fn draw_mask</code> of <code>lessons/17/src/engine/gpu/faces.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.draw(pass, binds, &amp;<span class="k">self</span>.pipes.mask)
    }

    <span class="c">/// Draw the selected face into both masks.</span>
    <span class="k">pub</span> <span class="k">fn</span> draw_masks(&amp;<span class="k">self</span>, pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;, binds: &amp;Binds) -&gt; u32 {
        <span class="k">if</span> <span class="k">self</span>.active.is_none() {
            <span class="k">return</span> <span class="s">0</span>;
        }

        <span class="k">self</span>.draw(pass, binds, &amp;<span class="k">self</span>.pipes.masks)
    }

    <span class="c">/// Selection change count.</span>
    <span class="k">pub</span> <span class="k">fn</span> revision(&amp;<span class="k">self</span>) -&gt; u64 {
        <span class="k">self</span>.revision</code></pre></div>
<p>Replaces the 5 lines from <code>) -&gt; (</code> of <code>lessons/17/src/engine/gpu/faces.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>) -&gt; FacePipelines {</code></pre></div>
<p>Replaces the <code>(pick, highlight, mask)</code> line in <code>fn pipelines</code> of <code>lessons/17/src/engine/gpu/faces.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">let</span> masks = build(
        &amp;ctx.device,
        Target {
            format: wgpu::TextureFormat::R8Unorm,
            samples: target.samples,
        },
        &amp;base
            .with(&quot;<span class="s">selected face masks</span>&quot;, &quot;<span class="s">fs_masks</span>&quot;)
            .depth(DepthMode::ReadOnlyEqual)
            .masks(),
    );
    <span class="c">// same faces, plain vertex shader</span>
    <span class="k">let</span> object_base = base.vertex(&quot;<span class="s">vs_triangle</span>&quot;);
    <span class="k">let</span> physical = build(
        &amp;ctx.device,
        target,
        &amp;object_base
            .with(&quot;<span class="s">physical triangle</span>&quot;, &quot;<span class="s">fs_main</span>&quot;)
            .color(ColorWrite::Blended)
            .physical(),
    );
    <span class="k">let</span> object_ids = build(
        &amp;ctx.device,
        Target::ID,
        &amp;object_base.with(&quot;<span class="s">object triangle IDs</span>&quot;, &quot;<span class="s">fs_id</span>&quot;).physical(),
    );
    FacePipelines {
        pick,
        highlight,
        mask,
        masks,
        physical,
        object_ids,
    }</code></pre></div>
<h2 id="step-15-srcenginegpuarenars">Step 15 · src/engine/gpu/arena.rs<a class="anchor" href="#/course/18-finite-visibility#step-15-srcenginegpuarenars" aria-label="Link to this section">#</a></h2>
<p>The arena holds mesh vertices and indices across objects.</p>
<p><code>lessons/18/src/engine/gpu/arena.rs</code> · edit · type this</p>
<p>Replaces the 4 lines from <code>}</code> of <code>lessons/17/src/engine/gpu/arena.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    masks: wgpu::RenderPipeline, <span class="c">// both masks in one pass</span>
}

<span class="c">/// All mesh geometry on the GPU, in five growing buffers.</span>
<span class="k">pub</span> <span class="k">struct</span> ArenaLane {
    <span class="k">pub</span> tiles: super::triangle_tiles::TriangleTiles, <span class="c">// screen tiles for visibility tests</span></code></pre></div>
<p>Added after the <code>impl ArenaLane {</code> line of <code>lessons/17/src/engine/gpu/arena.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Recompute which triangles are visible on screen.</span>
    <span class="k">pub</span> <span class="k">fn</span> prepare_visibility(
        &amp;<span class="k">mut</span> <span class="k">self</span>,
        ctx: &amp;GpuCtx,
        encoder: &amp;<span class="k">mut</span> wgpu::CommandEncoder,
        binds: &amp;Binds,
        matrix: [f32; <span class="s">16</span>],
        objects_revision: u64,
    ) {
        <span class="k">self</span>.tiles.encode(
            ctx,
            encoder,
            super::triangle_tiles::TileInput {
                binds,
                geometry: [&amp;<span class="k">self</span>.verts.buf, &amp;<span class="k">self</span>.vids.buf, &amp;<span class="k">self</span>.faces.buf],
                matrix,
                objects_revision,
            },
        );
    }

    <span class="c">/// Bytes reserved on the GPU by this lane.</span></code></pre></div>
<p>Added after the <code>+ self.source_faces.allocated_bytes()</code> line in <code>fn allocated_bytes</code> of <code>lessons/17/src/engine/gpu/arena.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            + <span class="k">self</span>.tiles.allocated_bytes().<span class="s">0</span></code></pre></div>
<p>Added after the <code>source_faces,</code> line in <code>fn new</code> of <code>lessons/17/src/engine/gpu/arena.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            tiles: super::triangle_tiles::TriangleTiles::new(ctx, l),</code></pre></div>
<p>Added after the <code>pub fn append(&amp;mut self, ctx: &amp;GpuCtx, up: &amp;A…</code> line in <code>impl ArenaLane</code> of <code>lessons/17/src/engine/gpu/arena.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.tiles.invalidate();</code></pre></div>
<p>Replaces the <code>self.draw_run(pass, b, &amp;self.pipes.faces, &amp;se…</code> line in <code>fn draw_faces</code> of <code>lessons/17/src/engine/gpu/arena.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.source_faces.draw_physical(pass, b)</code></pre></div>
<p>Replaces the <code>self.draw_run(pass, b, &amp;self.pipes.id_faces,…</code> line in <code>fn draw_face_ids</code> of <code>lessons/17/src/engine/gpu/arena.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.source_faces.draw_object_ids(pass, b)</code></pre></div>
<p>Added after the <code>self.draw_run(pass, b, &amp;self.pipes.solid_mask…</code> line in <code>fn draw_solid_mask</code> of <code>lessons/17/src/engine/gpu/arena.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Both coverage masks from one pass over the faces.</span>
    <span class="k">pub</span> <span class="k">fn</span> draw_masks(&amp;<span class="k">self</span>, pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;, b: &amp;Binds) -&gt; u32 {
        <span class="k">self</span>.draw_run(pass, b, &amp;<span class="k">self</span>.pipes.masks, &amp;<span class="k">self</span>.faces)
    }

    <span class="c">/// Draw object ids of sheet lettering.</span></code></pre></div>
<p>Added after the <code>pub fn reset(&amp;mut self, ctx: &amp;GpuCtx) {</code> line in <code>impl ArenaLane</code> of <code>lessons/17/src/engine/gpu/arena.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.tiles.invalidate();</code></pre></div>
<p>Added after the <code>pub fn release(&amp;mut self, ctx: &amp;GpuCtx) {</code> line in <code>impl ArenaLane</code> of <code>lessons/17/src/engine/gpu/arena.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.tiles.release(ctx);</code></pre></div>
<p>Added after the <code>),</code> line in <code>fn build_pipelines</code> of <code>lessons/17/src/engine/gpu/arena.rs</code></p>
<p>Added after the <code>),</code> line in <code>fn build_pipelines</code> of <code>lessons/17/src/engine/gpu/arena.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        masks: build(
            dev,
            Target {
                format: wgpu::TextureFormat::R8Unorm,
                samples: target.samples,
            },
            &amp;base
                .with(&quot;<span class="s">triangle.masks</span>&quot;, &quot;<span class="s">fs_masks</span>&quot;)
                .depth(<span class="k">crate</span>::engine::pipelines::DepthMode::ReadOnlyEqual)
                .masks(),
        ),</code></pre></div>
<h2 id="step-16-srcenginepipelineslayoutsrs">Step 16 · src/engine/pipelines/layouts.rs<a class="anchor" href="#/course/18-finite-visibility#step-16-srcenginepipelineslayoutsrs" aria-label="Link to this section">#</a></h2>
<p>Bind-group layouts describe the resources shared by the drawing modules.</p>
<p><code>lessons/18/src/engine/pipelines/layouts.rs</code> · edit · type this</p>
<p>Replaces the <code>entries: &amp;[storage_entry(0), storage_entry(1)],</code> line in <code>fn instance_layout</code> of <code>lessons/17/src/engine/pipelines/layouts.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        entries: &amp;[
            buffer_entry(
                <span class="s">0</span>,
                wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::COMPUTE,
                wgpu::BufferBindingType::Storage { read_only: <span class="s">true</span> },
            ),
            buffer_entry(
                <span class="s">1</span>,
                wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::COMPUTE,
                wgpu::BufferBindingType::Storage { read_only: <span class="s">true</span> },
            ),
        ],</code></pre></div>
<p>Added after the <code>scene_gradient(5, true),</code> line in <code>fn ink_instance_layout</code> of <code>lessons/17/src/engine/pipelines/layouts.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            buffer_entry(
                <span class="s">6</span>,
                wgpu::ShaderStages::FRAGMENT,
                wgpu::BufferBindingType::Storage { read_only: <span class="s">true</span> },
            ),
            buffer_entry(
                <span class="s">7</span>,
                wgpu::ShaderStages::FRAGMENT,
                wgpu::BufferBindingType::Storage { read_only: <span class="s">true</span> },
            ),</code></pre></div>
<p>Replaces the 2 lines from <code>mvp: uniform_layout(device, &quot;mvp.layout&quot;, wgp…</code> in <code>fn new</code> of <code>lessons/17/src/engine/pipelines/layouts.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            mvp: uniform_layout(
                device,
                &quot;<span class="s">mvp.layout</span>&quot;,
                wgpu::ShaderStages::VERTEX_FRAGMENT | wgpu::ShaderStages::COMPUTE,
            ),
            line: uniform_layout(
                device,
                &quot;<span class="s">line.layout</span>&quot;,
                wgpu::ShaderStages::VERTEX_FRAGMENT | wgpu::ShaderStages::COMPUTE,
            ),</code></pre></div>
<h2 id="step-17-srcenginepipelinesmodrs">Step 17 · src/engine/pipelines/mod.rs<a class="anchor" href="#/course/18-finite-visibility#step-17-srcenginepipelinesmodrs" aria-label="Link to this section">#</a></h2>
<p>Pipeline descriptions keep color, depth and sample-count choices together.</p>
<p><code>lessons/18/src/engine/pipelines/mod.rs</code> · edit · type this</p>
<p>Added after the <code>pub physical: bool,</code> line in <code>struct PipelineDesc</code> of <code>lessons/17/src/engine/pipelines/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> masks: bool, <span class="c">// writes both outline masks with MAX blending</span></code></pre></div>
<p>Added after the <code>physical: false,</code> line in <code>fn new</code> of <code>lessons/17/src/engine/pipelines/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            masks: <span class="s">false</span>,</code></pre></div>
<p>Added after the <code>self.physical = true;</code> line in <code>fn physical</code> of <code>lessons/17/src/engine/pipelines/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// A copy that writes both outline masks.</span>
    <span class="k">pub</span> <span class="k">fn</span> masks(<span class="k">mut</span> <span class="k">self</span>) -&gt; <span class="k">Self</span> {
        <span class="k">self</span>.masks = <span class="s">true</span>;
        <span class="k">self</span>
    }

    <span class="c">/// A copy with another depth mode.</span></code></pre></div>
<p>Replaces <code>fn module</code> in <code>lessons/17/src/engine/pipelines/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Append the WGSL every shader ends with: normals and physical output.</span>
<span class="k">pub</span> <span class="k">fn</span> shared(source: &amp;str) -&gt; String {
    format!(
        &quot;{<span class="s">source</span>}<span class="s">\\n</span>{}<span class="s">\\n</span>{}&quot;,
        include_str!(&quot;<span class="s">../../shaders/normals.wgsl</span>&quot;),
        include_str!(&quot;<span class="s">../../shaders/physical.wgsl</span>&quot;)
    )
}

<span class="c">/// Compile a shader that declares its own bindings.</span>
<span class="k">pub</span> <span class="k">fn</span> module(device: &amp;wgpu::Device, label: &amp;str, source: &amp;str) -&gt; wgpu::ShaderModule {
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some(label),
        source: wgpu::ShaderSource::Wgsl(shared(source).into()),
    })
}

<span class="c">/// Shared WGSL for ink: the visibility test and projected triangles.</span>
<span class="k">pub</span> <span class="k">const</span> INK: &amp;str = concat!(
    include_str!(&quot;<span class="s">../../shaders/ink_visibility.wgsl</span>&quot;),
    &quot;<span class="s">\\n</span>&quot;,
    include_str!(&quot;<span class="s">../../shaders/projected_triangle.wgsl</span>&quot;)
);

<span class="c">/// Compile an ink shader: scene code plus the ink code.</span>
<span class="k">pub</span> <span class="k">fn</span> ink_module(device: &amp;wgpu::Device, label: &amp;str, source: &amp;str) -&gt; wgpu::ShaderModule {
    scene_module(device, label, &amp;format!(&quot;{<span class="s">source</span>}<span class="s">\\n</span>{<span class="s">INK</span>}&quot;))
}

<span class="c">/// A pipeline layout over \`groups\`, in slot order.</span>
<span class="k">pub</span> <span class="k">fn</span> pipeline_layout(</code></pre></div>
<p>Replaces the 4 lines from <code>if desc.physical {</code> in <code>fn build</code> of <code>lessons/17/src/engine/pipelines/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">// two mask targets with MAX blending</span>
    <span class="k">if</span> desc.masks {
        <span class="k">let</span> max = wgpu::BlendComponent {
            src_factor: wgpu::BlendFactor::One,
            dst_factor: wgpu::BlendFactor::One,
            operation: wgpu::BlendOperation::Max,
        };
        <span class="k">let</span> coverage = Some(wgpu::ColorTargetState {
            format: target.format,
            blend: Some(wgpu::BlendState {
                color: max,
                alpha: max,
            }),
            write_mask: wgpu::ColorWrites::ALL,
        });
        targets = vec![coverage.clone(), coverage];
    }

    <span class="c">// the depth slope target</span>
    <span class="k">if</span> desc.physical {
        targets.push(Some(wgpu::ColorTargetState {
            format: wgpu::TextureFormat::Rgba16Float,
            blend: None,
            <span class="c">// a ReadOnlyEqual pass already wrote it: declare, do not write</span></code></pre></div>
<h2 id="step-18-srcenginegpuobjectsrs">Step 18 · src/engine/gpu/objects.rs<a class="anchor" href="#/course/18-finite-visibility#step-18-srcenginegpuobjectsrs" aria-label="Link to this section">#</a></h2>
<p>The object table stores GPU rows separately from source identity.</p>
<p><code>lessons/18/src/engine/gpu/objects.rs</code> · edit · type this</p>
<p>Replaces <code>struct InstanceTable</code> in <code>lessons/17/src/engine/gpu/objects.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Translation relative to the origin, as the GPU reads it.</span>
<span class="k">fn</span> anchored(t: [f64; <span class="s">3</span>], origin: &amp;Point) -&gt; [f32; <span class="s">4</span>] {
    [
        (t[<span class="s">0</span>] - origin[<span class="s">0</span>]) <span class="k">as</span> f32,
        (t[<span class="s">1</span>] - origin[<span class="s">1</span>]) <span class="k">as</span> f32,
        (t[<span class="s">2</span>] - origin[<span class="s">2</span>]) <span class="k">as</span> f32,
        <span class="s">0</span>.<span class="s">0</span>,
    ]
}

<span class="c">/// The object rows on the GPU and their exact positions on the CPU.</span>
<span class="k">pub</span> <span class="k">struct</span> InstanceTable {
    geometry_revision: u64, <span class="c">// bumps when anything moves or hides</span>
    rows: Vec&lt;Instance&gt;,
    translation: Vec&lt;[f64; 3]&gt;, <span class="c">// exact world position per row</span></code></pre></div>
<p>Replaces the 44 lines from <code>pub targets: &amp;&#39;a Targets,</code> in <code>struct InkScene</code> of <code>lessons/17/src/engine/gpu/objects.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> tiles: &amp;'a super::triangle_tiles::TriangleTiles, <span class="c">// screen tiles for visibility tests</span>
    <span class="k">pub</span> targets: &amp;'a Targets, <span class="c">// depth and gradient textures</span>
}

<span class="c">/// Bind group 2 for ink lanes: rows, depth, gradient, tiles.</span>
<span class="k">fn</span> ink_instance_group(
    ctx: &amp;GpuCtx,
    l: &amp;Layouts,
    label: &amp;str,
    buffers: [&amp;wgpu::Buffer; <span class="s">2</span>],
    depths: [&amp;wgpu::TextureView; <span class="s">2</span>],
    gradients: [&amp;wgpu::TextureView; <span class="s">2</span>],
    tiles: &amp;super::triangle_tiles::TriangleTiles,
) -&gt; wgpu::BindGroup {
    <span class="k">let</span> view = wgpu::BindingResource::TextureView;
    <span class="k">let</span> entries = [
        buffers[<span class="s">0</span>].as_entire_binding(),
        buffers[<span class="s">1</span>].as_entire_binding(),
        view(depths[<span class="s">0</span>]),
        view(depths[<span class="s">1</span>]),
        view(gradients[<span class="s">0</span>]),
        view(gradients[<span class="s">1</span>]),
        tiles.projected.as_entire_binding(),
        tiles.buffer.as_entire_binding(),
    ];
    <span class="k">let</span> entries: Vec&lt;wgpu::BindGroupEntry&gt; = entries
        .into_iter()
        .enumerate()
        .map(|(binding, resource)| wgpu::BindGroupEntry {
            binding: binding <span class="k">as</span> u32,
            resource,
        })
        .collect();
    ctx.device.create_bind_group(&amp;wgpu::BindGroupDescriptor {
        label: Some(label),
        layout: &amp;l.ink_instance,
        entries: &amp;entries,
    })
}

<span class="k">impl</span> InstanceTable {
    <span class="k">pub</span> <span class="k">fn</span> geometry_revision(&amp;<span class="k">self</span>) -&gt; u64 {
        <span class="k">self</span>.geometry_revision
    }</code></pre></div>
<p>Replaces the 3 lines from <code>let ink_group = ink_instance_group(ctx, l, [&amp;…</code> in <code>fn new</code> of <code>lessons/17/src/engine/gpu/objects.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> t = scene.targets;
        <span class="k">let</span> ink_group = ink_instance_group(
            ctx,
            l,
            &quot;<span class="s">ink.instances.bind_group</span>&quot;,
            [&amp;buffer.buf, &amp;translations.buf],
            [&amp;t.depth_single, &amp;t.depth_msaa],
            [&amp;t.gradient_single, &amp;t.gradient_msaa],
            scene.tiles,
        );

        <span class="k">Self</span> {
            geometry_revision: <span class="s">0</span>,</code></pre></div>
<p>Replaces the 2 lines from <code>self.ink_group =</code> in <code>fn rebind_ink</code> of <code>lessons/17/src/engine/gpu/objects.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> t = scene.targets;
        <span class="k">self</span>.ink_group = ink_instance_group(
            ctx,
            l,
            &quot;<span class="s">ink.instances.bind_group</span>&quot;,
            [&amp;<span class="k">self</span>.buffer.buf, &amp;<span class="k">self</span>.translations.buf],
            [&amp;t.depth_single, &amp;t.depth_msaa],
            [&amp;t.gradient_single, &amp;t.gradient_msaa],
            scene.tiles,
        );</code></pre></div>
<p>Replaces the 35 lines from <code>) -&gt; wgpu::BindGroup {</code> in <code>impl InstanceTable</code> of <code>lessons/17/src/engine/gpu/objects.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        tiles: &amp;super::triangle_tiles::TriangleTiles,
    ) -&gt; wgpu::BindGroup {
        ink_instance_group(
            ctx,
            layouts,
            &quot;<span class="s">pick.instances</span>&quot;,
            [&amp;<span class="k">self</span>.buffer.buf, &amp;<span class="k">self</span>.translations.buf],
            depths,
            gradients,
            tiles,
        )
    }

    <span class="c">/// Append one upload's rows.</span>
    <span class="k">pub</span> <span class="k">fn</span> append(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, l: &amp;Layouts, up: &amp;ObjectRows) {
        <span class="k">self</span>.geometry_revision = <span class="k">self</span>.geometry_revision.wrapping_add(<span class="s">1</span>);

        <span class="c">// first upload replaces the placeholder</span></code></pre></div>
<p>Replaces the 14 lines from <code>self.last_origin = Some(origin.clone());</code> in <code>fn rebuild</code> of <code>lessons/17/src/engine/gpu/objects.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.geometry_revision = <span class="k">self</span>.geometry_revision.wrapping_add(<span class="s">1</span>);
        <span class="k">self</span>.last_origin = Some(origin.clone());
        <span class="k">let</span> <span class="k">mut</span> rebased: Vec&lt;[f32; 4]&gt; = Vec::with_capacity(<span class="k">self</span>.rows.len());

        <span class="k">for</span> t <span class="k">in</span> &amp;<span class="k">self</span>.translation {
            rebased.push(anchored(*t, origin));
        }

        rebased.resize(<span class="k">self</span>.rows.len(), [<span class="s">0</span>.<span class="s">0</span>; <span class="s">4</span>]);
        <span class="k">self</span>.translations.write_at(ctx, <span class="s">0</span>, &amp;rebased);</code></pre></div>
<p>Replaces the 3 lines from <code>model[12] = (t[0] - o[0]) as f32;</code> in <code>fn anchored_model</code> of <code>lessons/17/src/engine/gpu/objects.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">let</span> a = anchored(*t, o);
            model[<span class="s">12</span>] = a[<span class="s">0</span>];
            model[<span class="s">13</span>] = a[<span class="s">1</span>];
            model[<span class="s">14</span>] = a[<span class="s">2</span>];</code></pre></div>
<p>Replaces the 5 lines from <code>self.buffer.write_at(ctx, row, std::slice::fr…</code> in <code>fn set_flag</code> of <code>lessons/17/src/engine/gpu/objects.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">if</span> bit &amp; Instance::FLAG_HIDDEN != <span class="s">0</span> {
            <span class="k">self</span>.geometry_revision = <span class="k">self</span>.geometry_revision.wrapping_add(<span class="s">1</span>);
        }

        <span class="k">self</span>.buffer.write_at(ctx, row, std::slice::from_ref(r));
    }

    <span class="c">/// Forget every row; keep the buffers.</span>
    <span class="k">pub</span> <span class="k">fn</span> reset(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">self</span>.geometry_revision = <span class="k">self</span>.geometry_revision.wrapping_add(<span class="s">1</span>);</code></pre></div>
<p>Replaces the 12 lines from <code>let mut r = ObjectRow::new(Xform::translation…</code> in <code>fn world_box_translates</code> of <code>lessons/17/src/engine/gpu/objects.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> <span class="k">mut</span> r = ObjectRow::new(Xform::translation(<span class="s">10</span>.<span class="s">0</span>, <span class="s">20</span>.<span class="s">0</span>, <span class="s">30</span>.<span class="s">0</span>), <span class="s">0</span>);
        r.bounds = AABB::new(<span class="s">0</span>.<span class="s">5</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">5</span>, <span class="s">0</span>.<span class="s">5</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">5</span>);
        <span class="k">let</span> b = world_box(&amp;r);
        assert_eq!(b.min_point(), Point::new(<span class="s">10</span>.<span class="s">0</span>, <span class="s">20</span>.<span class="s">0</span>, <span class="s">30</span>.<span class="s">0</span>));
        assert_eq!(b.max_point(), Point::new(<span class="s">11</span>.<span class="s">0</span>, <span class="s">22</span>.<span class="s">0</span>, <span class="s">33</span>.<span class="s">0</span>));
    }

    <span class="c">/// A row with no local box stays empty, translated or not.</span>
    #[test]
    <span class="k">fn</span> world_box_empty_stays_empty() {
        <span class="k">let</span> r = ObjectRow::new(Xform::translation(<span class="s">10</span>.<span class="s">0</span>, <span class="s">20</span>.<span class="s">0</span>, <span class="s">30</span>.<span class="s">0</span>), <span class="s">0</span>);
        assert!(!world_box(&amp;r).is_valid());
    }

    <span class="c">/// A tiny move far from zero survives only relative to a near origin.</span>
    #[test]
    <span class="k">fn</span> the_anchor_is_what_keeps_a_small_move() {
        <span class="k">let</span> world = <span class="s">1</span>.<span class="s">0</span>e<span class="s">6_f64</span>;
        <span class="k">let</span> step = <span class="s">1</span>.<span class="s">0</span>e-<span class="s">3_f64</span>;
        assert_eq!((world + step) <span class="k">as</span> f32, world <span class="k">as</span> f32);

        <span class="k">let</span> near = Point::new(world - <span class="s">1</span>.<span class="s">0</span>e<span class="s">4</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>);
        <span class="k">let</span> before = anchored([world, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>], &amp;near)[<span class="s">0</span>];
        <span class="k">let</span> after = anchored([world + step, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>], &amp;near)[<span class="s">0</span>];
        assert_ne!(after, before);
    }

    <span class="c">/// A move written to the f32 value is lost at the next rebase.</span>
    #[test]
    <span class="k">fn</span> an_edit_written_past_the_base_does_not_survive_a_rebase() {
        <span class="k">let</span> base = [<span class="s">1</span>.<span class="s">0</span>e<span class="s">4</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>];
        <span class="k">let</span> first = Point::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>);
        <span class="k">let</span> step = <span class="s">0</span>.<span class="s">25_f32</span>;

        <span class="c">// edit the relative value only</span>
        <span class="k">let</span> edited = anchored(base, &amp;first)[<span class="s">0</span>] + step;
        assert_eq!(edited, <span class="s">1</span>.<span class="s">0</span>e<span class="s">4</span> + <span class="s">0</span>.<span class="s">25</span>);

        <span class="c">// the rebase never saw the edit</span>
        <span class="k">let</span> second = Point::new(<span class="s">1</span>.<span class="s">0</span>e<span class="s">3</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>);
        assert_eq!(anchored(base, &amp;second)[<span class="s">0</span>], <span class="s">9</span>.<span class="s">0</span>e<span class="s">3</span>);</code></pre></div>
<h2 id="step-19-srcenginegputargetsrs">Step 19 · src/engine/gpu/targets.rs<a class="anchor" href="#/course/18-finite-visibility#step-19-srcenginegputargetsrs" aria-label="Link to this section">#</a></h2>
<p>Targets own the depth and color attachments for a frame.</p>
<p><code>lessons/18/src/engine/gpu/targets.rs</code> · edit · type this</p>
<p>Replaces the 61 lines from <code>let depth = texture_view(</code> in <code>fn new</code> of <code>lessons/17/src/engine/gpu/targets.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> attachment = |label, size, format, samples| {
            texture_view(
                ctx,
                label,
                &amp;TextureSpec {
                    size,
                    format,
                    samples,
                    usage,
                },
            )
        };
        <span class="k">let</span> depth = attachment(&quot;<span class="s">depth</span>&quot;, size, wgpu::TextureFormat::Depth32Float, samples);
        <span class="k">let</span> msaa = (samples &gt; <span class="s">1</span>).then(|| attachment(&quot;<span class="s">msaa_color</span>&quot;, size, format, samples));

        <span class="c">// shaders bind both sample counts; the unused one is 1x1</span>
        <span class="k">let</span> other_samples = <span class="k">if</span> samples == <span class="s">1</span> { <span class="s">4</span> } <span class="k">else</span> { <span class="s">1</span> };
        <span class="k">let</span> empty_depth = attachment(
            &quot;<span class="s">unused.depth</span>&quot;,
            (<span class="s">1</span>, <span class="s">1</span>),
            wgpu::TextureFormat::Depth32Float,
            other_samples,
        );
        <span class="k">let</span> (depth_single, depth_msaa) = <span class="k">if</span> samples == <span class="s">1</span> {
            (depth.clone(), empty_depth)
        } <span class="k">else</span> {
            (empty_depth, depth.clone())
        };
        <span class="k">let</span> gradient = attachment(
            &quot;<span class="s">physical.gradient</span>&quot;,
            size,
            wgpu::TextureFormat::Rgba16Float,
            samples,
        );
        <span class="k">let</span> empty_gradient = attachment(
            &quot;<span class="s">unused.gradient</span>&quot;,
            (<span class="s">1</span>, <span class="s">1</span>),
            wgpu::TextureFormat::Rgba16Float,
            other_samples,</code></pre></div>
<h2 id="step-20-srcenginegpupickrs">Step 20 · src/engine/gpu/pick.rs<a class="anchor" href="#/course/18-finite-visibility#step-20-srcenginegpupickrs" aria-label="Link to this section">#</a></h2>
<p>Picking reads an object and subobject ID asynchronously.</p>
<p><code>lessons/18/src/engine/gpu/pick.rs</code> · edit · type this</p>
<p>Replaces the <code>(buffer, pixels * 16)</code> line in <code>fn allocated_bytes</code> of <code>lessons/17/src/engine/gpu/pick.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        (buffer, pixels * <span class="s">20</span>)</code></pre></div>
<p>Replaces the <code>format: wgpu::TextureFormat::Rg16Float,</code> line in <code>fn begin_pass</code> of <code>lessons/17/src/engine/gpu/pick.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                    format: wgpu::TextureFormat::Rgba16Float,</code></pre></div>
<h2 id="step-21-srcenginegpuinstancers">Step 21 · src/engine/gpu/instance.rs<a class="anchor" href="#/course/18-finite-visibility#step-21-srcenginegpuinstancers" aria-label="Link to this section">#</a></h2>
<p>Each object row carries placement, color and selection flags for later interaction.</p>
<p><code>lessons/18/src/engine/gpu/instance.rs</code> · edit · type this</p>
<p>Replaces the 18 lines from <code>let source = if source.contains(&quot;-&gt; InkColor&quot;) {</code> in <code>fn shader_validation_and_layouts</code> of <code>lessons/17/src/engine/gpu/instance.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="c">// shaders that use the camera get the shared scene code</span>
            <span class="k">let</span> scene = source.contains(&quot;<span class="s">mvp</span>&quot;) || source.contains(&quot;<span class="s">line.</span>&quot;);
            <span class="k">let</span> <span class="k">mut</span> source = source.to_string();

            <span class="k">if</span> source.contains(&quot;<span class="s">-&gt; InkColor</span>&quot;) {
                source = format!(&quot;{<span class="s">source</span>}<span class="s">\\n</span>{}&quot;, <span class="k">crate</span>::engine::pipelines::INK);
            }

            <span class="k">if</span> scene {
                source = format!(&quot;{<span class="s">source</span>}<span class="s">\\n</span>{}&quot;, <span class="k">crate</span>::engine::pipelines::SCENE);
            }

            <span class="k">let</span> source = <span class="k">crate</span>::engine::pipelines::shared(&amp;source);
            <span class="k">let</span> module = naga::front::wgsl::parse_str(&amp;source)
                .unwrap_or_else(|error| panic!(&quot;{<span class="s">name</span>}<span class="s">: </span>{}&quot;, error.emit_to_string(&amp;source)));
            naga::valid::Validator::new(
                naga::valid::ValidationFlags::all(),
                <span class="c">// allows pack2x16float, as wgpu does</span>
                naga::valid::Capabilities::default()
                    | naga::valid::Capabilities::SHADER_FLOAT16_IN_FLOAT32,</code></pre></div>
<p>Added after the <code>size_of::&lt;StrokeSegment&gt;(),</code> line in <code>fn shader_validation_and_layouts</code> of <code>lessons/17/src/engine/gpu/instance.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                    &quot;<span class="s">ProjectedTriangle</span>&quot; =&gt; (
                        vec![<span class="s">0</span>, <span class="s">16</span>, <span class="s">32</span>, <span class="s">48</span>, <span class="s">64</span>, <span class="s">80</span>],
                        super::super::triangle_tiles::PROJECTED_BYTES <span class="k">as</span> usize,
                    ),</code></pre></div>
<h2 id="step-22-srcenginegpupresentrs">Step 22 · src/engine/gpu/present.rs<a class="anchor" href="#/course/18-finite-visibility#step-22-srcenginegpupresentrs" aria-label="Link to this section">#</a></h2>
<p>Presentation acquires the frame and submits rendering work.</p>
<p><code>lessons/18/src/engine/gpu/present.rs</code> · edit · type this</p>
<p>Added after the <code>self.pick.map();</code> line in <code>fn present</code> of <code>lessons/17/src/engine/gpu/present.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.arena.tiles.map_report();</code></pre></div>
<p>Added after the <code>self.pick.map();</code> line in <code>fn pick_frame</code> of <code>lessons/17/src/engine/gpu/present.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.arena.tiles.map_report();</code></pre></div>
<p>Added after the <code>self.pick.map();</code> line in <code>fn render_offscreen</code> of <code>lessons/17/src/engine/gpu/present.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.arena.tiles.map_report();</code></pre></div>
<h2 id="step-23-srcenginegpusurface_outliners">Step 23 · src/engine/gpu/surface_outline.rs<a class="anchor" href="#/course/18-finite-visibility#step-23-srcenginegpusurface_outliners" aria-label="Link to this section">#</a></h2>
<p>Surface masks add outlines around visible coverage.</p>
<p><code>lessons/18/src/engine/gpu/surface_outline.rs</code> · edit · type this</p>
<p>Added after the <code>const POOL: u32 = 16;</code> line of <code>lessons/17/src/engine/gpu/surface_outline.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// What a mask depends on; same key = reuse the old mask.</span>
#[derive(Clone, Copy, Debug, PartialEq)]
<span class="k">pub</span> <span class="k">struct</span> MaskKey {
    <span class="k">pub</span> mvp: [f32; <span class="s">16</span>], <span class="c">// camera matrix</span>
    <span class="k">pub</span> geometry: u64, <span class="c">// object change count</span>
    <span class="k">pub</span> selection: u64, <span class="c">// selection change count</span>
    <span class="k">pub</span> faces: u64, <span class="c">// face selection change count</span>
    <span class="k">pub</span> size: (u32, u32), <span class="c">// canvas size, px</span>
    <span class="k">pub</span> samples: u32,
    <span class="k">pub</span> edges: bool,
    <span class="k">pub</span> pen: u32, <span class="c">// pen width bits</span>
}

<span class="c">/// Which surfaces the outline goes around.</span></code></pre></div>
<p>Added after the <code>mask: Option&lt;Mask&gt;,</code> line in <code>struct SurfaceOutline</code> of <code>lessons/17/src/engine/gpu/surface_outline.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    valid_for: Option&lt;MaskKey&gt;, <span class="c">// what the mask was drawn for</span></code></pre></div>
<p>Replaces the <code>}</code> line in <code>fn new</code> of <code>lessons/17/src/engine/gpu/surface_outline.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            valid_for: None,
        }
    }

    <span class="c">/// True when the mask was drawn for \`key\`.</span>
    <span class="k">pub</span> <span class="k">fn</span> is_valid(&amp;<span class="k">self</span>, key: &amp;MaskKey) -&gt; bool {
        <span class="k">self</span>.mask.is_some() &amp;&amp; <span class="k">self</span>.valid_for.as_ref() == Some(key)
    }

    <span class="c">/// Record that the mask was drawn for \`key\`.</span>
    <span class="k">pub</span> <span class="k">fn</span> mark_valid(&amp;<span class="k">mut</span> <span class="k">self</span>, key: MaskKey) {
        <span class="k">self</span>.valid_for = Some(key);
    }

    <span class="c">/// The mask as a render target, resolving MSAA into \`resolved\`.</span>
    <span class="k">fn</span> attachment(&amp;<span class="k">self</span>) -&gt; Option&lt;wgpu::RenderPassColorAttachment&lt;'_&gt;&gt; {
        <span class="k">let</span> mask = <span class="k">self</span>.mask.as_ref()?;
        Some(wgpu::RenderPassColorAttachment {
            view: mask.multisampled.as_ref().unwrap_or(&amp;mask.resolved),
            resolve_target: <span class="k">if</span> mask.multisampled.is_some() {
                Some(&amp;mask.resolved)
            } <span class="k">else</span> {
                None
            },
            depth_slice: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                store: wgpu::StoreOp::Store,
            },
        })
    }

    <span class="c">/// Open one pass that writes both masks at once.</span>
    <span class="k">pub</span> <span class="k">fn</span> begin_masks&lt;'a&gt;(
        solid: &amp;'a <span class="k">Self</span>,
        selected: &amp;'a <span class="k">Self</span>,
        encoder: &amp;'a <span class="k">mut</span> wgpu::CommandEncoder,
        targets: &amp;'a Targets,
    ) -&gt; wgpu::RenderPass&lt;'a&gt; {
        encoder.begin_render_pass(&amp;wgpu::RenderPassDescriptor {
            label: Some(&quot;<span class="s">visible coverage masks</span>&quot;),
            color_attachments: &amp;[solid.attachment(), selected.attachment()],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &amp;targets.depth,
                depth_ops: None,
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        })
    }

    <span class="c">/// Remember whether object \`row\` is selected.</span></code></pre></div>
<p>Added after the <code>self.mask = None;</code> line in <code>fn prepare</code> of <code>lessons/17/src/engine/gpu/surface_outline.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">self</span>.valid_for = None;</code></pre></div>
<p>Replaces the 7 lines from <code>}</code> in <code>fn prepare</code> of <code>lessons/17/src/engine/gpu/surface_outline.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">self</span>.valid_for = None;
        }

        <span class="c">// outline width in CSS pixels</span>
        <span class="k">let</span> css_radius = <span class="s">1</span>.<span class="s">6875</span>;
        <span class="c">// at most 12 px: the blur must fit inside one coarse block</span></code></pre></div>
<p>Added after the <code>let mut gpu = pollster::block_on(Gpu::new_hea…</code> line in <code>fn selected_cad_edges_do_not_paint_over_the_black_silhouette</code> of <code>lessons/17/src/engine/gpu/surface_outline.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        gpu.view.show_outlines = <span class="s">true</span>;</code></pre></div>
<p>Added after the <code>let mut gpu = pollster::block_on(Gpu::new_hea…</code> line in <code>fn touching_and_overlapping_solids_have_one_continuous_outline</code> of <code>lessons/17/src/engine/gpu/surface_outline.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        gpu.view.show_outlines = <span class="s">true</span>;</code></pre></div>
<p>Replaces this block of <code>lessons/17/src/engine/gpu/surface_outline.rs</code>:</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> <span class="k">mut</span> gpu = pollster::block_on(Gpu::new_headless(<span class="s">200</span>, <span class="s">200</span>)).unwrap();
        gpu.view.show_grid = <span class="s">false</span>;</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        gpu.view.show_outlines = <span class="s">true</span>;</code></pre></div>
<h2 id="step-24-srcenginegpusegmentsrs">Step 24 · src/engine/gpu/segments.rs<a class="anchor" href="#/course/18-finite-visibility#step-24-srcenginegpusegmentsrs" aria-label="Link to this section">#</a></h2>
<p>The segment buffers store strokes and the object rows they belong to.</p>
<p><code>lessons/18/src/engine/gpu/segments.rs</code> · edit · type this</p>
<p>Added after the <code>id_edge: wgpu::RenderPipeline,</code> line in <code>struct SegPipelines</code> of <code>lessons/17/src/engine/gpu/segments.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    mask_unselected: wgpu::RenderPipeline,
    mask_selected: wgpu::RenderPipeline,
    masks_unselected: wgpu::RenderPipeline,
    masks_selected: wgpu::RenderPipeline,</code></pre></div>
<p>Added after the <code>}</code> line in <code>impl SegmentLane</code> of <code>lessons/17/src/engine/gpu/segments.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Draw every edge into the solid mask.</span>
    <span class="k">pub</span> <span class="k">fn</span> draw_solid_mask(&amp;<span class="k">self</span>, pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;, b: &amp;Binds) -&gt; u32 {
        <span class="k">self</span>.draw_table(pass, b, &amp;<span class="k">self</span>.gpu.mask_unselected, &amp;<span class="k">self</span>.pipes)
            + <span class="k">self</span>.draw_table(pass, b, &amp;<span class="k">self</span>.gpu.mask_selected, &amp;<span class="k">self</span>.pipes)
    }

    <span class="c">/// Draw the selected objects' edges into a mask.</span>
    <span class="k">pub</span> <span class="k">fn</span> draw_selection_mask(&amp;<span class="k">self</span>, pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;, b: &amp;Binds) -&gt; u32 {
        <span class="k">if</span> <span class="k">self</span>.selected_rows.is_empty() {
            <span class="k">return</span> <span class="s">0</span>;
        }

        <span class="k">self</span>.draw_table(pass, b, &amp;<span class="k">self</span>.gpu.mask_selected, &amp;<span class="k">self</span>.pipes)
    }

    <span class="c">/// Draw every edge into both masks at once.</span>
    <span class="k">pub</span> <span class="k">fn</span> draw_masks(&amp;<span class="k">self</span>, pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;, b: &amp;Binds) -&gt; u32 {
        <span class="k">self</span>.draw_table(pass, b, &amp;<span class="k">self</span>.gpu.masks_unselected, &amp;<span class="k">self</span>.pipes)
            + <span class="k">self</span>.draw_table(pass, b, &amp;<span class="k">self</span>.gpu.masks_selected, &amp;<span class="k">self</span>.pipes)
    }

    <span class="c">/// Draw the edges in color.</span></code></pre></div>
<p>Added after the <code>.depth(DepthMode::Always);</code> line in <code>fn build_pipelines</code> of <code>lessons/17/src/engine/gpu/segments.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">// masks are one-channel textures</span>
    <span class="k">let</span> mask = Target {
        format: wgpu::TextureFormat::R8Unorm,
        samples: target.samples,
    };</code></pre></div>
<p>Added after the <code>&amp;quad.with(&quot;edge.id&quot;, &quot;fs_edge_id&quot;).scene_sam…</code> line in <code>fn build_pipelines</code> of <code>lessons/17/src/engine/gpu/segments.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        mask_unselected: build(
            dev,
            mask,
            &amp;quad
                .with(&quot;<span class="s">ribbon.mask</span>&quot;, &quot;<span class="s">fs_mask</span>&quot;)
                .vertex(&quot;<span class="s">vs_unselected</span>&quot;)
                .color(ColorWrite::Max),
        ),
        mask_selected: build(
            dev,
            mask,
            &amp;quad
                .with(&quot;<span class="s">ribbon.mask.selected</span>&quot;, &quot;<span class="s">fs_mask</span>&quot;)
                .vertex(&quot;<span class="s">vs_selected</span>&quot;)
                .color(ColorWrite::Max),
        ),
        masks_unselected: build(
            dev,
            mask,
            &amp;quad
                .with(&quot;<span class="s">ribbon.masks</span>&quot;, &quot;<span class="s">fs_masks</span>&quot;)
                .vertex(&quot;<span class="s">vs_unselected</span>&quot;)
                .masks(),
        ),
        masks_selected: build(
            dev,
            mask,
            &amp;quad
                .with(&quot;<span class="s">ribbon.masks.selected</span>&quot;, &quot;<span class="s">fs_masks_selected</span>&quot;)
                .vertex(&quot;<span class="s">vs_selected</span>&quot;)
                .masks(),
        ),</code></pre></div>
<h2 id="step-25-srcenginegpurenderrs">Step 25 · src/engine/gpu/render.rs<a class="anchor" href="#/course/18-finite-visibility#step-25-srcenginegpurenderrs" aria-label="Link to this section">#</a></h2>
<p>The frame encoder orders face, ink, picking and overlay passes.</p>
<p><code>lessons/18/src/engine/gpu/render.rs</code> · edit · type this</p>
<p>Replaces the 2 lines from \`\` of <code>lessons/17/src/engine/gpu/render.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> super::surface_outline;

<span class="k">impl</span> Gpu {
    <span class="c">/// Compute which triangles cover which screen tiles.</span>
    <span class="k">fn</span> triangle_tile_pass(&amp;<span class="k">mut</span> <span class="k">self</span>, encoder: &amp;<span class="k">mut</span> wgpu::CommandEncoder) {
        <span class="c">// a moved tile buffer needs a new bind group</span>
        <span class="k">if</span> <span class="k">self</span>.arena.tiles.prepare(
            &amp;<span class="k">self</span>.ctx,
            (<span class="k">self</span>.config.width, <span class="k">self</span>.config.height),
            <span class="k">self</span>.arena.face_count() / <span class="s">3</span>,
        ) {
            <span class="k">self</span>.rebind_ink();
        }

        <span class="k">let</span> b = <span class="k">self</span>.frame.binds(&amp;<span class="k">self</span>.objects.group);
        <span class="k">self</span>.arena.prepare_visibility(
            &amp;<span class="k">self</span>.ctx,
            encoder,
            &amp;b,
            <span class="k">self</span>.frame.mvp_f32,
            <span class="k">self</span>.objects.geometry_revision(),
        );
    }

    <span class="c">/// Encode one frame into \`view\`; returns (draws, objects).</span></code></pre></div>
<p>Replaces the 50 lines from <code>self.point_pass(encoder);</code> in <code>fn encode_frame</code> of <code>lessons/17/src/engine/gpu/render.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.triangle_tile_pass(encoder);
        <span class="k">self</span>.point_pass(encoder);

        <span class="c">// pass 1: background, faces and clouds write depth</span>
        <span class="k">let</span> <span class="k">mut</span> draws = {
            <span class="k">let</span> b = <span class="k">self</span>.frame.binds(&amp;<span class="k">self</span>.objects.group);
            <span class="k">let</span> <span class="k">mut</span> pass = <span class="k">self</span>.targets.begin_faces(encoder, view, clear);
            <span class="k">self</span>.face_list(&amp;<span class="k">mut</span> pass, &amp;b)
        };
        <span class="k">let</span> size = (<span class="k">self</span>.config.width, <span class="k">self</span>.config.height);
        <span class="c">// no outlines in x-ray</span>
        <span class="k">let</span> faces =
            <span class="k">self</span>.view.show_outlines &amp;&amp; <span class="k">self</span>.view.opacity &gt; <span class="s">0</span>.<span class="s">0</span> &amp;&amp; <span class="k">self</span>.arena.face_count() &gt; <span class="s">0</span>;
        <span class="k">let</span> selected = <span class="k">self</span>.selection_outline.prepare(
            &amp;<span class="k">self</span>.ctx,
            size,
            <span class="k">self</span>.targets.samples,
            <span class="k">self</span>.logical_size[<span class="s">0</span>],
            faces,
        );
        <span class="k">let</span> solid = <span class="k">self</span>.solid_outline.prepare(
            &amp;<span class="k">self</span>.ctx,
            size,
            <span class="k">self</span>.targets.samples,
            <span class="k">self</span>.logical_size[<span class="s">0</span>],
            faces,
        );
        <span class="c">// redraw the outline masks only when something changed</span>
        <span class="k">let</span> key = surface_outline::MaskKey {
            mvp: <span class="k">self</span>.frame.mvp_f32,
            geometry: <span class="k">self</span>.objects.geometry_revision(),
            selection: <span class="k">self</span>.selection_revision,
            faces: <span class="k">self</span>.arena.source_faces.revision(),
            size,
            samples: <span class="k">self</span>.targets.samples,
            edges: <span class="k">self</span>.view.show_mesh_edges,
            pen: <span class="k">self</span>.view.thickness_px.to_bits(),
        };
        <span class="k">let</span> stale = (solid &amp;&amp; !<span class="k">self</span>.solid_outline.is_valid(&amp;key))
            || (selected &amp;&amp; !<span class="k">self</span>.selection_outline.is_valid(&amp;key));

        <span class="k">if</span> stale {
            <span class="k">let</span> b = <span class="k">self</span>.frame.binds(&amp;<span class="k">self</span>.objects.group);
            <span class="c">// edges widen the mask</span>
            <span class="k">let</span> ink = <span class="k">self</span>.frame.binds(&amp;<span class="k">self</span>.objects.ink_group);
            <span class="k">let</span> edges = <span class="k">self</span>.view.show_mesh_edges;

            <span class="k">if</span> solid &amp;&amp; selected {
                <span class="c">// one pass writes both masks</span>
                <span class="k">let</span> <span class="k">mut</span> pass = surface_outline::SurfaceOutline::begin_masks(
                    &amp;<span class="k">self</span>.solid_outline,
                    &amp;<span class="k">self</span>.selection_outline,
                    encoder,
                    &amp;<span class="k">self</span>.targets,
                );
                draws += <span class="k">self</span>.arena.draw_masks(&amp;<span class="k">mut</span> pass, &amp;b);
                draws += <span class="k">self</span>.arena.source_faces.draw_masks(&amp;<span class="k">mut</span> pass, &amp;b);

                <span class="k">if</span> edges {
                    draws += <span class="k">self</span>.segments.draw_masks(&amp;<span class="k">mut</span> pass, &amp;ink);
                }
            } <span class="k">else</span> <span class="k">if</span> solid {
                <span class="k">let</span> <span class="k">mut</span> pass = <span class="k">self</span>.solid_outline.begin_mask(encoder, &amp;<span class="k">self</span>.targets);
                draws += <span class="k">self</span>.arena.draw_solid_mask(&amp;<span class="k">mut</span> pass, &amp;b);

                <span class="k">if</span> edges {
                    draws += <span class="k">self</span>.segments.draw_solid_mask(&amp;<span class="k">mut</span> pass, &amp;ink);
                }
            } <span class="k">else</span> <span class="k">if</span> selected {
                <span class="k">let</span> <span class="k">mut</span> pass = <span class="k">self</span>.selection_outline.begin_mask(encoder, &amp;<span class="k">self</span>.targets);
                draws += <span class="k">self</span>.arena.draw_selection_mask(&amp;<span class="k">mut</span> pass, &amp;b);
                draws += <span class="k">self</span>.arena.source_faces.draw_mask(&amp;<span class="k">mut</span> pass, &amp;b);

                <span class="k">if</span> edges {
                    draws += <span class="k">self</span>.segments.draw_selection_mask(&amp;<span class="k">mut</span> pass, &amp;ink);
                }
            }

            <span class="k">if</span> solid {
                <span class="k">self</span>.solid_outline.encode_pool(encoder);
                <span class="k">self</span>.solid_outline.mark_valid(key);
            }

            <span class="k">if</span> selected {
                <span class="k">self</span>.selection_outline.encode_pool(encoder);
                <span class="k">self</span>.selection_outline.mark_valid(key);</code></pre></div>
<p>Added after the <code>pub(super) fn id_pass(&amp;mut self, encoder: &amp;mu…</code> line in <code>impl Gpu</code> of <code>lessons/17/src/engine/gpu/render.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.triangle_tile_pass(encoder);</code></pre></div>
<p>Replaces the 6 lines from <code>);</code> in <code>fn scene_list</code> of <code>lessons/17/src/engine/gpu/render.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            &amp;<span class="k">self</span>.arena.tiles,
        );
        <span class="k">let</span> ink = <span class="k">self</span>.frame.pick_binds(&amp;group);</code></pre></div>
<h2 id="step-26-srcenginegpumodrs">Step 26 · src/engine/gpu/mod.rs<a class="anchor" href="#/course/18-finite-visibility#step-26-srcenginegpumodrs" aria-label="Link to this section">#</a></h2>
<p>The GPU owner connects buffers, pipelines and frame resources.</p>
<p><code>lessons/18/src/engine/gpu/mod.rs</code> · edit · type this</p>
<p>Added after the <code>pub mod text_outline;</code> line of <code>lessons/17/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">mod</span> triangle_tiles;</code></pre></div>
<p>Added after the <code>pub solid_outline: surface_outline::SurfaceOu…</code> line in <code>struct Gpu</code> of <code>lessons/17/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> selection_revision: u64, <span class="c">// bumps on every selection change</span></code></pre></div>
<p>Replaces the 5 lines from <code>let frame_textures =</code> in <code>fn allocated_bytes</code> of <code>lessons/17/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="c">// per sample: 4 color + 4 depth + 8 gradient; at 1x no color copy</span>
        <span class="k">let</span> frame_textures = pixels * <span class="k">if</span> samples &gt; <span class="s">1</span> { samples * <span class="s">16</span> } <span class="k">else</span> { <span class="s">12</span> }
            + <span class="k">if</span> samples &gt; <span class="s">1</span> { <span class="s">12</span> } <span class="k">else</span> { <span class="s">48</span> };
        (
            buffers,
            frame_textures
                + <span class="k">self</span>.arena.tiles.allocated_bytes().<span class="s">1</span></code></pre></div>
<p>Replaces the <code>let objects = InstanceTable::new(&amp;ctx, &amp;layou…</code> line in <code>fn build</code> of <code>lessons/17/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> objects = InstanceTable::new(
            &amp;ctx,
            &amp;layouts,
            &amp;InkScene {
                targets: &amp;targets,
                tiles: &amp;arena.tiles,
            },
        );</code></pre></div>
<p>Added after the <code>solid_outline,</code> line in <code>fn build</code> of <code>lessons/17/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            selection_revision: <span class="s">0</span>,</code></pre></div>
<p>Replaces the 5 lines from <code>self.objects.rebind_ink(</code> in <code>fn set_scene</code> of <code>lessons/17/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.rebind_ink();
    }

    <span class="c">/// Rebuild the ink bind group after targets or tiles moved.</span>
    <span class="k">fn</span> rebind_ink(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">self</span>.objects.rebind_ink(
            &amp;<span class="k">self</span>.ctx,
            &amp;<span class="k">self</span>.layouts,
            &amp;InkScene {
                targets: &amp;<span class="k">self</span>.targets,
                tiles: &amp;<span class="k">self</span>.arena.tiles,</code></pre></div>
<p>Replaces the 7 lines from <code>self.objects.rebind_ink(</code> in <code>fn retarget</code> of <code>lessons/17/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">self</span>.rebind_ink();</code></pre></div>
<p>Replaces the 11 lines from <code>self.objects.rebind_ink(</code> in <code>fn release</code> of <code>lessons/17/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.rebind_ink();
    }

    <span class="c">/// Select or deselect object \`row\`.</span>
    <span class="k">pub</span> <span class="k">fn</span> set_selected(&amp;<span class="k">mut</span> <span class="k">self</span>, row: u32, on: bool) {
        <span class="k">self</span>.selection_revision = <span class="k">self</span>.selection_revision.wrapping_add(<span class="s">1</span>);</code></pre></div>
<h2 id="check">Check<a class="anchor" href="#/course/18-finite-visibility#check" aria-label="Link to this section">#</a></h2>
<p>Run <code>trunk serve</code> in <code>lessons/18/</code> and open <a href="http://127.0.0.1:8770/" target="_blank" rel="noopener">http://127.0.0.1:8770/</a>.</p>
<p>Expected: Concave boundaries and contact seams stay continuous while genuinely covered edges remain hidden; status: <strong>the status names the selected object</strong>.</p>
<p><img src="/session/docs/course/docs/screenshots/18-teapot.png" alt="Checkpoint 18 with the supplied teapot: the rim and foot boundaries stay continuous from two camera positions, and the lid-to-body seams stay visible while a neighbouring face passes over their stroke fringe." loading="lazy" decoding="async"></p>
<p>If it fails:</p>
<ul>
<li>A concave boundary breaks: an infinite plane is used outside its triangle.</li>
<li>A stale silhouette remains: the cached mask is not invalidated by geometry or camera changes.</li>
</ul>
<h2 id="what-changed">What changed<a class="anchor" href="#/course/18-finite-visibility#what-changed" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>lessons/18/src/engine/
├── gpu/
│   ├── arena.rs  ~
│   ├── backdrop.rs
│   ├── buffers.rs
│   ├── cloud.rs
│   ├── device.rs
│   ├── faces.rs  ~
│   ├── frame.rs
│   ├── glyphs.rs
│   ├── instance.rs  ~
│   ├── lod.rs
│   ├── mod.rs  ~
│   ├── objects.rs  ~
│   ├── pick.rs  ~
│   ├── present.rs  ~
│   ├── render.rs  ~
│   ├── segments.rs  ~
│   ├── splat.rs
│   ├── surface_outline.rs  ~
│   ├── targets.rs  ~
│   ├── text.rs
│   ├── text_outline.rs
│   ├── text_plane.rs
│   ├── text_plate.rs
│   ├── triangle_tiles.rs  +
│   ├── upload.rs
│   └── view.rs
├── pipelines/
│   ├── layouts.rs  ~
│   └── mod.rs  ~
├── mod.rs
├── performance.rs
└── text.rs</code></pre></div>
<p><code>+</code> new in this lesson · <code>~</code> changed in this lesson</p>
<p>Every file at this point: <code>lessons/18/</code>.</p>
<h2 id="next">Next<a class="anchor" href="#/course/18-finite-visibility#next" aria-label="Link to this section">#</a></h2>
<p><a href="#/course/19-sheets">19 · Sheets: batched drawings with lazy metadata</a></p>
<h2 id="expected-viewer-result">Expected viewer result<a class="anchor" href="#/course/18-finite-visibility#expected-viewer-result" aria-label="Link to this section">#</a></h2>
<p>The teapot&#39;s rim, foot and lid seams draw cleanly.</p>
<p><a href="/session/docs/course/docs/screenshots/18.png"><img src="/session/docs/course/docs/screenshots/18.png" alt="Full viewer result for 18 finite visibility" loading="lazy" decoding="async"></a></p>
`,toc:[{level:2,id:"step-1-srcshadersphysicalwgsl",text:"Step 1 · src/shaders/physical.wgsl"},{level:2,id:"step-2-srcshaderstrianglewgsl",text:"Step 2 · src/shaders/triangle.wgsl"},{level:2,id:"step-3-srcshadersbackgroundwgsl",text:"Step 3 · src/shaders/background.wgsl"},{level:2,id:"step-4-srcshadersgridwgsl",text:"Step 4 · src/shaders/grid.wgsl"},{level:2,id:"step-5-srcshaderssplatwgsl",text:"Step 5 · src/shaders/splat.wgsl"},{level:2,id:"step-6-srcshaderssplat_resolvewgsl",text:"Step 6 · src/shaders/splat_resolve.wgsl"},{level:2,id:"step-7-srcshaderstext_outlinewgsl",text:"Step 7 · src/shaders/text_outline.wgsl"},{level:2,id:"step-8-srcshadersprojected_trianglewgsl",text:"Step 8 · src/shaders/projected_triangle.wgsl"},{level:2,id:"step-9-srcshadersproject_triangleswgsl",text:"Step 9 · src/shaders/project_triangles.wgsl"},{level:2,id:"step-10-srcshaderstriangle_tileswgsl",text:"Step 10 · src/shaders/triangle_tiles.wgsl"},{level:2,id:"step-11-srcshadersscan_triangle_tileswgsl",text:"Step 11 · src/shaders/scan_triangle_tiles.wgsl"},{level:2,id:"step-12-srcenginegputriangle_tilesrs",text:"Step 12 · src/engine/gpu/triangle_tiles.rs"},{level:2,id:"step-13-srcshadersink_visibilitywgsl",text:"Step 13 · src/shaders/ink_visibility.wgsl"},{level:2,id:"step-14-srcenginegpufacesrs",text:"Step 14 · src/engine/gpu/faces.rs"},{level:2,id:"step-15-srcenginegpuarenars",text:"Step 15 · src/engine/gpu/arena.rs"},{level:2,id:"step-16-srcenginepipelineslayoutsrs",text:"Step 16 · src/engine/pipelines/layouts.rs"},{level:2,id:"step-17-srcenginepipelinesmodrs",text:"Step 17 · src/engine/pipelines/mod.rs"},{level:2,id:"step-18-srcenginegpuobjectsrs",text:"Step 18 · src/engine/gpu/objects.rs"},{level:2,id:"step-19-srcenginegputargetsrs",text:"Step 19 · src/engine/gpu/targets.rs"},{level:2,id:"step-20-srcenginegpupickrs",text:"Step 20 · src/engine/gpu/pick.rs"},{level:2,id:"step-21-srcenginegpuinstancers",text:"Step 21 · src/engine/gpu/instance.rs"},{level:2,id:"step-22-srcenginegpupresentrs",text:"Step 22 · src/engine/gpu/present.rs"},{level:2,id:"step-23-srcenginegpusurface_outliners",text:"Step 23 · src/engine/gpu/surface_outline.rs"},{level:2,id:"step-24-srcenginegpusegmentsrs",text:"Step 24 · src/engine/gpu/segments.rs"},{level:2,id:"step-25-srcenginegpurenderrs",text:"Step 25 · src/engine/gpu/render.rs"},{level:2,id:"step-26-srcenginegpumodrs",text:"Step 26 · src/engine/gpu/mod.rs"},{level:2,id:"check",text:"Check"},{level:2,id:"what-changed",text:"What changed"},{level:2,id:"next",text:"Next"},{level:2,id:"expected-viewer-result",text:"Expected viewer result"}]};export{s as default};
