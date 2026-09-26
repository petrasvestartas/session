const s={title:"05 · Depth and visible ink",html:`<h1 id="05-depth-and-visible-ink">05 · Depth and visible ink<a class="anchor" href="#/course/05-visibility#05-depth-and-visible-ink" aria-label="Link to this section">#</a></h1>
<p>A grey box shows its visible red edges and black corners while its faces hide the far edges.</p>
<p><img src="/session/docs/course/docs/illustrations/ink-visibility.svg" alt="Reversed depth, and why a thick stroke must transfer the surface depth to its axis before comparing." loading="lazy" decoding="async"></p>
<h2 id="step-1-srcshadersphysicalwgsl">Step 1 · src/shaders/physical.wgsl<a class="anchor" href="#/course/05-visibility#step-1-srcshadersphysicalwgsl" aria-label="Link to this section">#</a></h2>
<p>New file: the physical outputs, depth beside colour.</p>
<p><code>lessons/05/src/shaders/physical.wgsl</code> · 22 lines · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// Depth slope is stored times this.</span>
<span class="k">const</span> PLANE_SCALE: <span class="k">f32</span> = <span class="s">65536.0</span>;
<span class="c">// Slope value that means &quot;no plane&quot;.</span>
<span class="k">const</span> PLANE_INVALID: <span class="k">f32</span> = <span class="s">65504.0</span>;

<span class="c">// Output of a face fragment: color and depth slope.</span>
<span class="k">struct</span> PhysicalColor {
    @location(<span class="s">0</span>) color: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // rgba</span>
    @location(<span class="s">1</span>) gradient: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;,
};

<span class="c">// Output of a face id fragment: ids and depth slope.</span>
<span class="k">struct</span> PhysicalId {
 @location(<span class="s">0</span>) id: <span class="k">vec2</span>&lt;<span class="k">u32</span>&gt;,<span class="c"> // object row + 1, sub id + 1</span>
 @location(<span class="s">1</span>) gradient: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;,
};

<span class="k">fn</span> physical_gradient(depth: <span class="k">f32</span>) -&gt; <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt; {
    <span class="k">let</span> scaled = <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(dpdx(depth), dpdy(depth)) * PLANE_SCALE;

    <span class="k">if</span> (any(abs(scaled) &gt;= <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(PLANE_INVALID))) {
        <span class="k">return</span> <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(PLANE_INVALID);
    }

    <span class="k">return</span> scaled;
}</code></pre></div>
<h2 id="step-2-srcshadersbackgroundwgsl">Step 2 · src/shaders/background.wgsl<a class="anchor" href="#/course/05-visibility#step-2-srcshadersbackgroundwgsl" aria-label="Link to this section">#</a></h2>
<p>New file: the background shader, one fullscreen triangle.</p>
<p><code>lessons/05/src/shaders/background.wgsl</code> · 21 lines · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// One vertex of the fullscreen triangle.</span>
<span class="k">struct</span> VsOut {
    @builtin(position) pos: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // clip position</span>
}

<span class="c">// One triangle that covers the whole screen.</span>
<span class="k">const</span> CORNERS = <span class="k">array</span>&lt;<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;, <span class="s">3</span>&gt;(
    <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(-<span class="s">1.0</span>, -<span class="s">1.0</span>),
    <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">3.0</span>, -<span class="s">1.0</span>),
    <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(-<span class="s">1.0</span>, <span class="s">3.0</span>),
);

@vertex
<span class="c">// Place the three corners.</span>
<span class="k">fn</span> vs_main(@builtin(vertex_index) vid: <span class="k">u32</span>) -&gt; VsOut {
    <span class="k">var</span> o: VsOut;
    o.pos = <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(CORNERS[vid], <span class="s">1.0</span>, <span class="s">1.0</span>);
    <span class="k">return</span> o;
}

@fragment
<span class="c">// White background; slightly grey when SSAO is on.</span>
<span class="k">fn</span> fs_main(in: VsOut) -&gt; PhysicalColor {
    <span class="k">return</span> PhysicalColor(<span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(<span class="s">1.0</span>, <span class="s">1.0</span>, <span class="s">1.0</span>, <span class="s">1.0</span>), <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>));
}</code></pre></div>
<h2 id="step-3-srcshadersgridwgsl">Step 3 · src/shaders/grid.wgsl<a class="anchor" href="#/course/05-visibility#step-3-srcshadersgridwgsl" aria-label="Link to this section">#</a></h2>
<p>New file: the grid shader, lines from vertex indices.</p>
<p><code>lessons/05/src/shaders/grid.wgsl</code> · 56 lines · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">const</span> STEP: <span class="k">f32</span> = <span class="s">1000.0</span>;<span class="c"> // mm per cell</span>
<span class="k">const</span> HALF: <span class="k">f32</span> = <span class="s">5000.0</span>;<span class="c"> // grid reaches 5 m each way</span>
<span class="k">const</span> N: <span class="k">u32</span> = <span class="s">5u</span>;<span class="c"> // cells from the center to the edge</span>
<span class="k">const</span> PER_DIR: <span class="k">u32</span> = <span class="s">22u</span>;<span class="c"> // vertices per direction: 11 lines, 2 ends</span>
<span class="k">const</span> FLOOR: <span class="k">u32</span> = <span class="s">44u</span>;<span class="c"> // floor vertices; axis vertices follow</span>

<span class="c">// Floor line color.</span>
<span class="k">const</span> GREY: <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt; = <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.55</span>, <span class="s">0.55</span>, <span class="s">0.55</span>);
<span class="c">// X axis color.</span>
<span class="k">const</span> RED: <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt; = <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.85</span>, <span class="s">0.30</span>, <span class="s">0.30</span>);
<span class="c">// Y axis color.</span>
<span class="k">const</span> GREEN: <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt; = <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.30</span>, <span class="s">0.70</span>, <span class="s">0.30</span>);
<span class="c">// Z axis color.</span>
<span class="k">const</span> BLUE: <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt; = <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.30</span>, <span class="s">0.45</span>, <span class="s">0.85</span>);

<span class="c">// One grid vertex.</span>
<span class="k">struct</span> VsOut {
    @builtin(position) pos: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // clip position</span>
    @location(<span class="s">0</span>) color: <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // line color</span>
}

@vertex
<span class="c">// Place one endpoint of a floor line or an axis from its index.</span>
<span class="k">fn</span> vs_main(@builtin(vertex_index) vid: <span class="k">u32</span>) -&gt; VsOut {
<span class="c">    // odd index = far end of the line</span>
    <span class="k">let</span> far = (vid % <span class="s">2u</span>) == <span class="s">1u</span>;
    <span class="k">var</span> wp: <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;;
    <span class="k">var</span> c: <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;;

<span class="c">    // floor lines: 11 along x, then 11 along y</span>
    <span class="k">if</span> (vid &lt; FLOOR) {
        <span class="k">let</span> dir = vid / PER_DIR;
        <span class="k">let</span> k = (vid % PER_DIR) / <span class="s">2u</span>;
        <span class="k">let</span> t = (f32(k) - f32(N)) * STEP;
        <span class="k">let</span> end = select(-HALF, HALF, far);
        wp = select(<span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(end, t, <span class="s">0.0</span>), <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(t, end, <span class="s">0.0</span>), dir == <span class="s">1u</span>);
        c = GREY;
    } <span class="k">else</span> {
<span class="c">        // axes: x, y, then a short z</span>
        <span class="k">let</span> axis = (vid - FLOOR) / <span class="s">2u</span>;

        <span class="k">if</span> (axis == <span class="s">0u</span>) {
            wp = <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(select(<span class="s">0.0</span>, HALF, far), <span class="s">0.0</span>, <span class="s">0.0</span>);
            c = RED;
        } <span class="k">else</span> <span class="k">if</span> (axis == <span class="s">1u</span>) {
            wp = <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>, select(<span class="s">0.0</span>, HALF, far), <span class="s">0.0</span>);
            c = GREEN;
        } <span class="k">else</span> {
<span class="c">            // z axis is one cell long</span>
            wp = <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>, <span class="s">0.0</span>, select(<span class="s">0.0</span>, STEP, far));
            c = BLUE;
        }
    }

    <span class="k">var</span> o: VsOut;
<span class="c">    // world to clip, relative to the scene origin</span>
    o.pos = mvp * <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(wp - line.anchor, <span class="s">1.0</span>);
    o.color = c;
    <span class="k">return</span> o;
}

@fragment
<span class="c">// Flat line color.</span>
<span class="k">fn</span> fs_main(in: VsOut) -&gt; PhysicalColor {
    <span class="k">return</span> PhysicalColor(<span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(in.color, <span class="s">1.0</span>), <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>));
}</code></pre></div>
<h2 id="step-4-srcenginegpubackdroprs">Step 4 · src/engine/gpu/backdrop.rs<a class="anchor" href="#/course/05-visibility#step-4-srcenginegpubackdroprs" aria-label="Link to this section">#</a></h2>
<p>New file: the backdrop lane, background and grid.</p>
<p><code>lessons/05/src/engine/gpu/backdrop.rs</code> · 111 lines · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> super::buffers::GpuCtx;
<span class="k">use</span> super::frame::Binds;
<span class="k">use</span> <span class="k">crate</span>::engine::pipelines::{
    DepthMode, Layouts, PipelineDesc, Target, build, module, scene_module,
};
<span class="k">use</span> wgpu::PrimitiveTopology::{LineList, TriangleList};

<span class="c">/// Shader sources the tests compare against the files.</span>
#[cfg(test)]
<span class="k">pub</span> <span class="k">const</span> SHADERS: &amp;[(&amp;str, &amp;str)] = &amp;[
    (&quot;<span class="s">grid.wgsl</span>&quot;, include_str!(&quot;<span class="s">../../shaders/grid.wgsl</span>&quot;)),
    (
        &quot;<span class="s">background.wgsl</span>&quot;,
        include_str!(&quot;<span class="s">../../shaders/background.wgsl</span>&quot;),
    ),
];

<span class="c">/// Grid vertex count: 44 floor lines plus 6 axis lines.</span>
<span class="k">const</span> GRID_VERTS: u32 = <span class="s">50</span>;

<span class="c">/// Drawn before everything else with depth writing off, so every later object covers it.</span>
<span class="k">pub</span> <span class="k">struct</span> BackdropLane {
    background_shader: wgpu::ShaderModule, <span class="c">// fullscreen background shader</span>
    grid_shader: wgpu::ShaderModule, <span class="c">// floor grid shader</span>
    background: wgpu::RenderPipeline, <span class="c">// background pipeline</span>
    grid: wgpu::RenderPipeline, <span class="c">// grid pipeline</span>
}

<span class="k">impl</span> BackdropLane {
    <span class="c">/// Compile both shaders and build the pipelines.</span>
    <span class="k">pub</span> <span class="k">fn</span> new(ctx: &amp;GpuCtx, l: &amp;Layouts, target: Target) -&gt; <span class="k">Self</span> {
        <span class="k">let</span> background_shader = module(
            &amp;ctx.device,
            &quot;<span class="s">background.shader</span>&quot;,
            include_str!(&quot;<span class="s">../../shaders/background.wgsl</span>&quot;),
        );
        <span class="k">let</span> grid_shader = scene_module(
            &amp;ctx.device,
            &quot;<span class="s">grid.shader</span>&quot;,
            include_str!(&quot;<span class="s">../../shaders/grid.wgsl</span>&quot;),
        );
        <span class="k">let</span> background = build_background(ctx, &amp;background_shader, target);
        <span class="k">let</span> grid = build_grid(ctx, l, &amp;grid_shader, target);

        <span class="k">Self</span> {
            background_shader,
            grid_shader,
            background,
            grid,
        }
    }

    <span class="c">/// Rebuild both pipelines for a new sample count.</span>
    <span class="k">pub</span> <span class="k">fn</span> retarget(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, l: &amp;Layouts, target: Target) {
        <span class="k">self</span>.background = build_background(ctx, &amp;<span class="k">self</span>.background_shader, target);
        <span class="k">self</span>.grid = build_grid(ctx, l, &amp;<span class="k">self</span>.grid_shader, target);
    }

    <span class="c">/// The background: one fullscreen triangle.</span>
    <span class="k">pub</span> <span class="k">fn</span> draw_background(&amp;<span class="k">self</span>, pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;) -&gt; u32 {
        pass.set_pipeline(&amp;<span class="k">self</span>.background);
        <span class="c">// three vertices, the shader places them</span>
        pass.draw(<span class="s">0</span>..<span class="s">3</span>, <span class="s">0</span>..<span class="s">1</span>);
        <span class="s">1</span>
    }

    <span class="c">/// Draw the floor grid lines.</span>
    <span class="k">pub</span> <span class="k">fn</span> draw_grid(&amp;<span class="k">self</span>, pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;, b: &amp;Binds) -&gt; u32 {
        pass.set_pipeline(&amp;<span class="k">self</span>.grid);
        pass.set_bind_group(<span class="s">0</span>, b.mvp, &amp;[]);
        pass.set_bind_group(<span class="s">1</span>, b.line, &amp;[]);
        pass.draw(<span class="s">0</span>..GRID_VERTS, <span class="s">0</span>..<span class="s">1</span>);
        <span class="s">1</span>
    }
}

<span class="c">/// Background pipeline: always passes the depth test.</span>
<span class="k">fn</span> build_background(
    ctx: &amp;GpuCtx,
    shader: &amp;wgpu::ShaderModule,
    target: Target,
) -&gt; wgpu::RenderPipeline {
    <span class="k">let</span> base = PipelineDesc::new(shader, &amp;[], &amp;[], TriangleList);
    build(
        &amp;ctx.device,
        target,
        &amp;base
            .with(&quot;<span class="s">background</span>&quot;, &quot;<span class="s">fs_main</span>&quot;)
            .depth(DepthMode::Always)
            .physical(),
    )
}

<span class="c">/// Grid pipeline: lines behind geometry are hidden.</span>
<span class="k">fn</span> build_grid(
    ctx: &amp;GpuCtx,
    l: &amp;Layouts,
    shader: &amp;wgpu::ShaderModule,
    target: Target,
) -&gt; wgpu::RenderPipeline {
    <span class="k">let</span> groups = [&amp;l.mvp, &amp;l.line];
    <span class="k">let</span> base = PipelineDesc::new(shader, &amp;groups, &amp;[], LineList);
    build(
        &amp;ctx.device,
        target,
        &amp;base
            .with(&quot;<span class="s">grid</span>&quot;, &quot;<span class="s">fs_main</span>&quot;)
            .depth(DepthMode::ReadOnly)
            .physical(),
    )
}</code></pre></div>
<p>Run <code>cargo check</code> in <code>lessons/05/</code>.</p>
<h2 id="step-5-srcshadersink_visibilitywgsl">Step 5 · src/shaders/ink_visibility.wgsl<a class="anchor" href="#/course/05-visibility#step-5-srcshadersink_visibilitywgsl" aria-label="Link to this section">#</a></h2>
<p>Replace the ink test: fit the surface under a sample and compare depth.</p>
<p><code>lessons/05/src/shaders/ink_visibility.wgsl</code> · type this, replace the whole file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>@group(<span class="s">2</span>) @binding(<span class="s">2</span>) <span class="k">var</span> scene_depth_single: texture_depth_2d;<span class="c"> // scene depth at 1x</span>
@group(<span class="s">2</span>) @binding(<span class="s">3</span>) <span class="k">var</span> scene_depth_msaa: texture_depth_multisampled_2d;<span class="c"> // scene depth at 4x</span>

override SCENE_MSAA: <span class="k">bool</span> = <span class="s">false</span>;<span class="c"> // which depth texture is live</span>
@group(<span class="s">2</span>) @binding(<span class="s">4</span>) <span class="k">var</span> scene_gradient_single: texture_2d&lt;<span class="k">f32</span>&gt;;<span class="c"> // depth slope and triangle id at 1x</span>
@group(<span class="s">2</span>) @binding(<span class="s">5</span>) <span class="k">var</span> scene_gradient_msaa: texture_multisampled_2d&lt;<span class="k">f32</span>&gt;;<span class="c"> // the same at 4x</span>

<span class="c">// Depth tolerance as a fraction of the depth.</span>
<span class="k">const</span> DEPTH_REL_TOL: <span class="k">f32</span> = <span class="s">1.9073486e-6</span>;

<span class="c">// Vertex snapping error, px: 1/256.</span>
<span class="k">const</span> SLOPE_PX: <span class="k">f32</span> = <span class="s">0.00390625</span>;

<span class="c">// How much two slopes may differ and still be one surface.</span>
<span class="k">const</span> KINK: <span class="k">f32</span> = <span class="s">0.03125</span>;

<span class="c">// Output of an ink fragment.</span>
<span class="k">struct</span> InkColor {
    @location(<span class="s">0</span>) color: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // rgba</span>
};

<span class="c">// The stroke's center line as one fragment sees it.</span>
<span class="k">struct</span> InkAxis {
    at: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // nearest point on the center line, screen px</span>
    depth: <span class="k">f32</span>,<span class="c"> // its depth</span>
    along: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // stroke direction on screen, unit</span>
    slope: <span class="k">f32</span>,<span class="c"> // depth change per pixel along it</span>
};</code></pre></div>
<p><code>lessons/05/src/shaders/ink_visibility.wgsl</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// Scene depth at a pixel; 0 outside the screen or where nothing was drawn.</span>
<span class="k">fn</span> ink_depth(pixel: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;, sample: <span class="k">u32</span>) -&gt; <span class="k">f32</span> {
    <span class="k">if</span> (any(pixel &lt; <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>)) || any(pixel &gt;= <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(line.vp_w, line.vp_h))) {
        <span class="k">return</span> <span class="s">0.0</span>;
    }

    <span class="k">let</span> at = <span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;(pixel);

    <span class="k">if</span> (SCENE_MSAA) {
        <span class="k">return</span> textureLoad(scene_depth_msaa, at, i32(sample));
    }

    <span class="k">return</span> textureLoad(scene_depth_single, at, <span class="s">0</span>);
}

<span class="c">// Allowed error of a depth carried \`lever\` pixels along a slope.</span>
<span class="k">fn</span> ink_tolerance(depth: <span class="k">f32</span>, slope: <span class="k">f32</span>, lever: <span class="k">f32</span>) -&gt; <span class="k">f32</span> {
    <span class="k">return</span> abs(depth) * DEPTH_REL_TOL + abs(slope) * SLOPE_PX * (<span class="s">1.0</span> + lever);
}

<span class="c">// True when the pixel and its neighbour along \`dir\` are on one surface.</span>
<span class="k">fn</span> ink_pair_planar(pixel: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;, dir: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;, z: <span class="k">f32</span>, sample: <span class="k">u32</span>) -&gt; <span class="k">bool</span> {
    <span class="k">let</span> z_side = ink_depth(pixel + dir, sample);

<span class="c">    // no neighbour: no pair</span>
    <span class="k">if</span> (z_side == <span class="s">0.0</span>) {
        <span class="k">return</span> <span class="s">false</span>;
    }

    <span class="k">let</span> z_far = ink_depth(pixel + dir * <span class="s">2.0</span>, sample);

<span class="c">    // surface ends: the pair is all there is</span>
    <span class="k">if</span> (z_far == <span class="s">0.0</span>) {
        <span class="k">return</span> <span class="s">true</span>;
    }

<span class="c">    // the two slopes must nearly agree</span>
    <span class="k">let</span> g = z_side - z;
    <span class="k">let</span> g_far = z_far - z_side;
    <span class="k">return</span> abs(g_far - g) &lt;= abs(z) * DEPTH_REL_TOL + KINK * (abs(g) + abs(g_far));
}

<span class="c">// Is the ink visible, given the surface depth carried to its center?</span>
<span class="k">fn</span> ink_carry_visible(predicted: <span class="k">f32</span>, z: <span class="k">f32</span>, depth: <span class="k">f32</span>, tolerance: <span class="k">f32</span>) -&gt; <span class="k">bool</span> {
<span class="c">    // pixel nearer than the ink: only its own surface may show it</span>
    <span class="k">if</span> (z &gt; depth + abs(depth) * DEPTH_REL_TOL) {
        <span class="k">return</span> abs(predicted - depth) &lt;= tolerance;
    }

    <span class="k">return</span> predicted &lt;= depth + tolerance;
}

<span class="c">// One-pixel step away from the stroke, along x or y.</span>
<span class="k">fn</span> ink_step(pixel: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;, axis: InkAxis) -&gt; <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt; {
    <span class="k">let</span> perp = <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(-axis.along.y, axis.along.x);
    <span class="k">var</span> step = <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(sign(perp.x), <span class="s">0.0</span>);

    <span class="k">if</span> (abs(perp.y) &gt; abs(perp.x)) {
        step = <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>, sign(perp.y));
    }

    <span class="k">return</span> select(step, -step, dot(pixel - axis.at, step) &lt; <span class="s">0.0</span>);
}</code></pre></div>
<p><code>lessons/05/src/shaders/ink_visibility.wgsl</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// Is a stroke fragment visible: fit the surface under it and carry it to the center line.</span>
<span class="k">fn</span> ink_axis_visible(pixel: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;, axis: InkAxis, sample: <span class="k">u32</span>) -&gt; <span class="k">bool</span> {
    <span class="k">let</span> z = ink_depth(pixel, sample);

    <span class="k">if</span> (z == <span class="s">0.0</span>) {
        <span class="k">return</span> <span class="s">true</span>;
    }

    <span class="k">let</span> step = ink_step(pixel, axis);
<span class="c">    // fit away from the stroke, else toward it, else compare raw depth</span>
    <span class="k">var</span> side = step;

    <span class="k">if</span> (!ink_pair_planar(pixel, side, z, sample)) {
        side = -step;

        <span class="k">if</span> (!ink_pair_planar(pixel, side, z, sample)) {
            <span class="k">return</span> z &lt;= axis.depth + abs(axis.depth) * DEPTH_REL_TOL;
        }
    }

    <span class="k">let</span> z_side = ink_depth(pixel + side, sample);
<span class="c">    // offset to the center line as a * along + b * side</span>
    <span class="k">let</span> e = axis.at - pixel;
    <span class="k">let</span> det = axis.along.x * side.y - axis.along.y * side.x;
    <span class="k">let</span> a = (e.x * side.y - e.y * side.x) / det;
    <span class="k">let</span> b = (axis.along.x * e.y - axis.along.y * e.x) / det;
    <span class="k">let</span> g = z_side - z;
<span class="c">    // surface depth carried to the center line</span>
    <span class="k">let</span> predicted = z + a * axis.slope + b * g;
    <span class="k">return</span> ink_carry_visible(predicted, z, axis.depth, ink_tolerance(axis.depth, abs(g) + abs(axis.slope), abs(b)));
}</code></pre></div>
<p><code>lessons/05/src/shaders/ink_visibility.wgsl</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// Is a disc fragment visible: fit the surface under it and carry it to the disc center.</span>
<span class="k">fn</span> ink_disc_fragment_visible(pixel: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;, centre: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;, depth: <span class="k">f32</span>, sample: <span class="k">u32</span>) -&gt; <span class="k">bool</span> {
    <span class="k">let</span> z = ink_depth(pixel, sample);

    <span class="k">if</span> (z == <span class="s">0.0</span>) {
        <span class="k">return</span> <span class="s">true</span>;
    }

    <span class="k">let</span> d = pixel - centre;
<span class="c">    // fit along x and y, away from the center first</span>
    <span class="k">var</span> along_x = <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(select(<span class="s">1.0</span>, -<span class="s">1.0</span>, d.x &lt; <span class="s">0.0</span>), <span class="s">0.0</span>);
    <span class="k">var</span> along_y = <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>, select(<span class="s">1.0</span>, -<span class="s">1.0</span>, d.y &lt; <span class="s">0.0</span>));

    <span class="k">if</span> (!ink_pair_planar(pixel, along_x, z, sample)) {
        along_x = -along_x;

        <span class="k">if</span> (!ink_pair_planar(pixel, along_x, z, sample)) {
            <span class="k">return</span> z &lt;= depth + abs(depth) * DEPTH_REL_TOL;
        }
    }

    <span class="k">if</span> (!ink_pair_planar(pixel, along_y, z, sample)) {
        along_y = -along_y;

        <span class="k">if</span> (!ink_pair_planar(pixel, along_y, z, sample)) {
            <span class="k">return</span> z &lt;= depth + abs(depth) * DEPTH_REL_TOL;
        }
    }

<span class="c">    // slopes per pixel</span>
    <span class="k">let</span> gx = (ink_depth(pixel + along_x, sample) - z) * along_x.x;
    <span class="k">let</span> gy = (ink_depth(pixel + along_y, sample) - z) * along_y.y;
    <span class="k">let</span> predicted = z - d.x * gx - d.y * gy;
    <span class="k">return</span> ink_carry_visible(predicted, z, depth, ink_tolerance(depth, abs(gx) + abs(gy), abs(d.x) + abs(d.y)));
}

<span class="c">// Direction from a point to the camera; constant in ortho.</span>
<span class="k">fn</span> toward_eye(point: <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;) -&gt; <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt; {
    <span class="k">if</span> (line.ortho_h &gt; <span class="s">0.0</span>) {
        <span class="k">return</span> <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(mvp[<span class="s">0</span>].z, mvp[<span class="s">1</span>].z, mvp[<span class="s">2</span>].z);
    }

    <span class="k">return</span> <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(line.eye_x, line.eye_y, line.eye_z) - point;
}

<span class="c">// Is a disc visible: its fragment, and its center pixel too.</span>
<span class="k">fn</span> ink_disc_visible(pixel: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;, centre: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;, depth: <span class="k">f32</span>, sample: <span class="k">u32</span>) -&gt; <span class="k">bool</span> {
    <span class="k">if</span> (!ink_disc_fragment_visible(pixel, centre, depth, sample)) {
        <span class="k">return</span> <span class="s">false</span>;
    }

    <span class="k">let</span> source_pixel = floor(centre) + fract(pixel);
    <span class="k">return</span> !ink_disc_source_hidden(source_pixel, centre, depth, sample);
}</code></pre></div>
<p><code>lessons/05/src/shaders/ink_visibility.wgsl</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// Is a plane fitted at the center pixel in front of the disc?</span>
<span class="k">fn</span> ink_disc_source_hidden(pixel: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;, centre: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;, depth: <span class="k">f32</span>, sample: <span class="k">u32</span>) -&gt; <span class="k">bool</span> {
    <span class="k">let</span> z = ink_depth(pixel, sample);

    <span class="k">if</span> (z == <span class="s">0.0</span>) {
        <span class="k">return</span> <span class="s">false</span>;
    }

    <span class="k">let</span> d = pixel - centre;

<span class="c">    // try every quadrant</span>
    <span class="k">for</span> (<span class="k">var</span> x = <span class="s">0u</span>; x &lt; <span class="s">2u</span>; x++) {
        <span class="k">let</span> dx = <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(select(<span class="s">1.0</span>, -<span class="s">1.0</span>, x == <span class="s">1u</span>), <span class="s">0.0</span>);

        <span class="k">if</span> (!ink_pair_planar(pixel, dx, z, sample)) {
            <span class="k">continue</span>;
        }

        <span class="k">let</span> gx = (ink_depth(pixel + dx, sample) - z) * dx.x;

        <span class="k">for</span> (<span class="k">var</span> y = <span class="s">0u</span>; y &lt; <span class="s">2u</span>; y++) {
            <span class="k">let</span> dy = <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>, select(<span class="s">1.0</span>, -<span class="s">1.0</span>, y == <span class="s">1u</span>));

            <span class="k">if</span> (!ink_pair_planar(pixel, dy, z, sample)) {
                <span class="k">continue</span>;
            }

            <span class="k">let</span> gy = (ink_depth(pixel + dy, sample) - z) * dy.y;
<span class="c">            // the diagonal pixel must agree with the plane too</span>
            <span class="k">let</span> diagonal = ink_depth(pixel + dx + dy, sample);
            <span class="k">let</span> expected = z + gx * dx.x + gy * dy.y;

            <span class="k">if</span> (diagonal == <span class="s">0.0</span> || abs(diagonal - expected) &gt; ink_tolerance(z, abs(gx) + abs(gy), <span class="s">2.0</span>)) {
                <span class="k">continue</span>;
            }

            <span class="k">let</span> predicted = z - d.x * gx - d.y * gy;

            <span class="k">if</span> (predicted &gt; depth + ink_tolerance(depth, abs(gx) + abs(gy), abs(d.x) + abs(d.y))) {
                <span class="k">return</span> <span class="s">true</span>;
            }
        }
    }

    <span class="k">return</span> <span class="s">false</span>;
}

<span class="c">// Use the primitive's own gradient.</span>
<span class="k">fn</span> ink_visible(pixel: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;, axis: InkAxis, sample: <span class="k">u32</span>) -&gt; <span class="k">bool</span> {
    <span class="k">let</span> z = ink_depth(pixel, sample);

    <span class="k">if</span> (z == <span class="s">0.0</span>) {
        <span class="k">return</span> <span class="s">true</span>;
    }

    <span class="k">var</span> encoded = <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>);

    <span class="k">if</span> (SCENE_MSAA) {
        encoded = textureLoad(scene_gradient_msaa, <span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;(pixel), i32(sample)).xy;
    }
    <span class="k">else</span> {
        encoded = textureLoad(scene_gradient_single, <span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;(pixel), <span class="s">0</span>).xy;
    }

<span class="c">    // no stored slope: fit one</span>
    <span class="k">if</span> (any(abs(encoded) &gt;= <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(PLANE_INVALID))) {
        <span class="k">return</span> ink_axis_visible(pixel, axis, sample);
    }

    <span class="k">let</span> gradient = encoded / PLANE_SCALE;
    <span class="k">let</span> delta = axis.at - pixel;
    <span class="k">let</span> predicted = z + dot(gradient, delta);
    <span class="k">return</span> ink_carry_visible(predicted, z, axis.depth, ink_tolerance(axis.depth, abs(gradient.x)+abs(gradient.y), abs(delta.x)+abs(delta.y)));
}</code></pre></div>
<h2 id="step-6-srcshaderstrianglewgsl">Step 6 · src/shaders/triangle.wgsl<a class="anchor" href="#/course/05-visibility#step-6-srcshaderstrianglewgsl" aria-label="Link to this section">#</a></h2>
<p>The mesh shader writes physical depth and reads it back for ink.</p>
<p><code>lessons/05/src/shaders/triangle.wgsl</code> · edit · type this</p>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    @location(<span class="s">5</span>) @interpolate(flat) mirrored: <span class="k">u32</span>,</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    @location(<span class="s">6</span>) @interpolate(flat) selected: <span class="k">u32</span>,<span class="c"> // nonzero when selected</span></code></pre></div>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    dead.mirrored = <span class="s">0u</span>;</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    dead.selected = <span class="s">0u</span>;</code></pre></div>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    o.inst_id = in.inst_id;</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    o.selected = inst.flags &amp; FLAG_SELECTED;</code></pre></div>
<p>Replaces the <code>fn fs_id</code> lines in <code>lessons/04d/src/shaders/triangle.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">fn</span> fs_id(in: VsOut) -&gt; PhysicalId {
    <span class="k">return</span> PhysicalId(<span class="k">vec2</span>&lt;<span class="k">u32</span>&gt;(in.inst_id + <span class="s">1u</span>, <span class="s">0u</span>), physical_gradient(in.pos.z));
}

@fragment
<span class="c">// Selected faces into a mask.</span>
<span class="k">fn</span> fs_selection_mask(in: VsOut) -&gt; @location(<span class="s">0</span>) <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt; {
    <span class="k">if</span> (in.selected == <span class="s">0u</span>) {
        <span class="k">discard</span>;
    }

    <span class="k">return</span> <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(<span class="s">1.0</span>);
}

@fragment
<span class="c">// Shaded face color and depth slope.</span>
<span class="k">fn</span> fs_main(in: VsOut, @builtin(front_facing) front: <span class="k">bool</span>) -&gt; PhysicalColor {
    <span class="k">return</span> PhysicalColor(shade(in, front), physical_gradient(in.pos.z));</code></pre></div>
<h2 id="step-7-srcshaderssplatwgsl">Step 7 · src/shaders/splat.wgsl<a class="anchor" href="#/course/05-visibility#step-7-srcshaderssplatwgsl" aria-label="Link to this section">#</a></h2>
<p>The splat shader writes physical depth too.</p>
<p><code>lessons/05/src/shaders/splat.wgsl</code> · edit · type this</p>
<p>Replaces the <code>fn fs_point_id</code> lines in <code>lessons/04d/src/shaders/splat.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">fn</span> fs_point_id(in: PointOut) -&gt; PhysicalId {
    <span class="k">if</span> (outside(in)) {
        <span class="k">discard</span>;
    }

    <span class="k">return</span> PhysicalId(<span class="k">vec2</span>&lt;<span class="k">u32</span>&gt;(in.instance + <span class="s">1u</span>, in.row + <span class="s">1u</span>), <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>));</code></pre></div>
<h2 id="step-8-srcshaderssplat_resolvewgsl">Step 8 · src/shaders/splat_resolve.wgsl<a class="anchor" href="#/course/05-visibility#step-8-srcshaderssplat_resolvewgsl" aria-label="Link to this section">#</a></h2>
<p>The resolve writes point depth into the scene.</p>
<p><code>lessons/05/src/shaders/splat_resolve.wgsl</code> · edit · type this</p>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    @location(<span class="s">0</span>) color: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    @location(<span class="s">1</span>) gradient: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;,</code></pre></div>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    o.depth = d;</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    o.gradient = <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>);</code></pre></div>
<h2 id="step-9-srcshaderstext_outlinewgsl">Step 9 · src/shaders/text_outline.wgsl<a class="anchor" href="#/course/05-visibility#step-9-srcshaderstext_outlinewgsl" aria-label="Link to this section">#</a></h2>
<p>Outline text takes the same depth test.</p>
<p><code>lessons/05/src/shaders/text_outline.wgsl</code> · edit · type this</p>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// Keep the exact source object row available to the shared identity pass.</span></code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// Object id, writing depth slope too.</span>
@fragment
<span class="k">fn</span> fs_physical_id(fragment: Fragment) -&gt; PhysicalId {
 <span class="k">return</span> PhysicalId(<span class="k">vec2</span>&lt;<span class="k">u32</span>&gt;(fragment.object+<span class="s">1u</span>, <span class="s">0u</span>), <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>));
}</code></pre></div>
<h2 id="step-10-srcenginegputargetsrs">Step 10 · src/engine/gpu/targets.rs<a class="anchor" href="#/course/05-visibility#step-10-srcenginegputargetsrs" aria-label="Link to this section">#</a></h2>
<p>Targets gain the depth and gradient textures at 1x and 4x.</p>
<p><code>lessons/05/src/engine/gpu/targets.rs</code> · edit · type this</p>
<p>Added above</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// The attachments of the frame's render pass and the sample count they were made at.</span></code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Pixels a discrete GPU may draw at 4x MSAA.</span>
<span class="k">const</span> MSAA_PIXELS_DISCRETE: u32 = <span class="s">9_000_000</span>;

<span class="c">/// Pixels an integrated GPU may draw at 4x MSAA.</span>
<span class="k">const</span> MSAA_PIXELS_SHARED: u32 = <span class="s">2_500_000</span>;

<span class="c">/// Pixels at 4x when the GPU type is unknown; every browser lands here.</span>
<span class="k">const</span> MSAA_PIXELS_UNKNOWN: u32 = <span class="s">4_200_000</span>;</code></pre></div>
<p>Added after the <code>depth_msaa</code> field of <code>struct Targets</code> in <code>lessons/04d/src/engine/gpu/targets.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> gradient: wgpu::TextureView,
    <span class="k">pub</span> gradient_single: wgpu::TextureView, <span class="c">// gradient at 1x, or a placeholder</span>
    <span class="k">pub</span> gradient_msaa: wgpu::TextureView, <span class="c">// gradient at 4x, or a placeholder</span></code></pre></div>
<p>Replaces the line <code>Self {</code> in <code>lessons/04d/src/engine/gpu/targets.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> gradient = texture_view(
            ctx,
            &quot;<span class="s">physical.gradient</span>&quot;,
            &amp;TextureSpec {
                size,
                format: wgpu::TextureFormat::Rg16Float,
                samples,
                usage,
            },
        );
        <span class="k">let</span> empty_gradient = texture_view(
            ctx,
            &quot;<span class="s">unused.gradient</span>&quot;,
            &amp;TextureSpec {
                size: (<span class="s">1</span>, <span class="s">1</span>),
                format: wgpu::TextureFormat::Rg16Float,
                samples: other_samples,
                usage,
            },
        );
        <span class="k">let</span> (gradient_single, gradient_msaa) = <span class="k">if</span> samples == <span class="s">1</span> {
            (gradient.clone(), empty_gradient)
        } <span class="k">else</span> {
            (empty_gradient, gradient.clone())
        };
        <span class="k">Self</span> {
            gradient,
            gradient_single,
            gradient_msaa,</code></pre></div>
<p>Replaces the <code>fn begin_faces</code> lines in <code>lessons/04d/src/engine/gpu/targets.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Pixels this GPU type may draw at 4x; None = never.</span>
    <span class="k">pub</span> <span class="k">fn</span> msaa_budget(gpu: wgpu::DeviceType) -&gt; Option&lt;u32&gt; {
        <span class="k">match</span> gpu {
            wgpu::DeviceType::DiscreteGpu =&gt; Some(MSAA_PIXELS_DISCRETE),
            wgpu::DeviceType::IntegratedGpu | wgpu::DeviceType::VirtualGpu =&gt; {
                Some(MSAA_PIXELS_SHARED)
            }
            wgpu::DeviceType::Cpu =&gt; None,
            wgpu::DeviceType::Other =&gt; Some(MSAA_PIXELS_UNKNOWN),
        }
    }

    <span class="c">/// 4x samples only when solids are drawn.</span>
    <span class="k">pub</span> <span class="k">fn</span> samples_for(solid: bool, pixels: u32, forced: Option&lt;u32&gt;, budget: Option&lt;u32&gt;) -&gt; u32 {
        <span class="k">if</span> <span class="k">let</span> Some(s) = forced {
            <span class="k">return</span> <span class="k">if</span> s == <span class="s">4</span> { <span class="s">4</span> } <span class="k">else</span> { <span class="s">1</span> };
        }

        <span class="k">match</span> budget {
            Some(max) <span class="k">if</span> solid &amp;&amp; pixels &lt;= max =&gt; <span class="s">4</span>,
            _ =&gt; <span class="s">1</span>,
        }
    }</code></pre></div>
<p>Replaces the lines from <code>color_attachments: &amp;[Some(wgpu::RenderPassColorAttachment {</code> to <code>})],</code> in <code>lessons/04d/src/engine/gpu/targets.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            color_attachments: &amp;[
                Some(wgpu::RenderPassColorAttachment {
                    view: target,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(clear),
                        store: wgpu::StoreOp::Store,
                    },
                }),
                Some(wgpu::RenderPassColorAttachment {
                    view: &amp;<span class="k">self</span>.gradient,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                }),
            ],</code></pre></div>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    texture(ctx, label, spec).create_view(&amp;wgpu::TextureViewDescriptor::default())
}</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>#[cfg(test)]
<span class="k">mod</span> tests {
    <span class="k">use</span> super::*;

    <span class="c">/// The budget follows the GPU type, not the pixel count alone.</span>
    #[test]
    <span class="k">fn</span> msaa_follows_the_adapter() {
        <span class="k">let</span> discrete = Targets::msaa_budget(wgpu::DeviceType::DiscreteGpu);
        <span class="k">let</span> shared = Targets::msaa_budget(wgpu::DeviceType::IntegratedGpu);
        <span class="k">let</span> software = Targets::msaa_budget(wgpu::DeviceType::Cpu);
        assert_eq!(Targets::samples_for(<span class="s">true</span>, <span class="s">3840</span> * <span class="s">2160</span>, None, discrete), <span class="s">4</span>);
        assert_eq!(Targets::samples_for(<span class="s">true</span>, <span class="s">3840</span> * <span class="s">2160</span>, None, shared), <span class="s">1</span>);
        assert_eq!(Targets::samples_for(<span class="s">true</span>, <span class="s">1920</span> * <span class="s">1080</span>, None, shared), <span class="s">4</span>);
        assert_eq!(Targets::samples_for(<span class="s">true</span>, <span class="s">1</span>, None, software), <span class="s">1</span>);
        assert_eq!(Targets::samples_for(<span class="s">false</span>, <span class="s">1</span>, None, discrete), <span class="s">1</span>);
        assert_eq!(
            Targets::samples_for(<span class="s">true</span>, <span class="s">3840</span> * <span class="s">2160</span>, Some(<span class="s">1</span>), discrete),
            <span class="s">1</span>
        );
        assert_eq!(Targets::samples_for(<span class="s">false</span>, u32::MAX, Some(<span class="s">4</span>), software), <span class="s">4</span>);
    }

    <span class="c">/// The unknown-GPU budget keeps 4x at 2560x1440 but not at 4K.</span>
    #[test]
    <span class="k">fn</span> the_browser_arm_is_not_the_integrated_one() {
        <span class="k">let</span> browser = Targets::msaa_budget(wgpu::DeviceType::Other);
        <span class="k">let</span> shared = Targets::msaa_budget(wgpu::DeviceType::IntegratedGpu);
        assert_eq!(Targets::samples_for(<span class="s">true</span>, <span class="s">2560</span> * <span class="s">1440</span>, None, browser), <span class="s">4</span>);
        assert_eq!(Targets::samples_for(<span class="s">true</span>, <span class="s">2560</span> * <span class="s">1440</span>, None, shared), <span class="s">1</span>);
        assert_eq!(Targets::samples_for(<span class="s">true</span>, <span class="s">3840</span> * <span class="s">2160</span>, None, browser), <span class="s">1</span>);
    }
}</code></pre></div>
<h2 id="step-11-srcenginepipelinesmodrs">Step 11 · src/engine/pipelines/mod.rs<a class="anchor" href="#/course/05-visibility#step-11-srcenginepipelinesmodrs" aria-label="Link to this section">#</a></h2>
<p>Pipelines gain the physical target and the sample count.</p>
<p><code>lessons/05/src/engine/pipelines/mod.rs</code> · edit · type this</p>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> scene_samples: Option&lt;u32&gt;,</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> physical: bool, <span class="c">// also writes the depth slope target</span></code></pre></div>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            scene_samples: None,</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            physical: <span class="s">false</span>,</code></pre></div>
<p>Added above</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// The same desc with another depth mode.</span></code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// A copy that also writes the depth slope target.</span>
    <span class="k">pub</span> <span class="k">fn</span> physical(<span class="k">mut</span> <span class="k">self</span>) -&gt; <span class="k">Self</span> {
        <span class="k">self</span>.physical = <span class="s">true</span>;
        <span class="k">self</span>
    }</code></pre></div>
<p>Replaces the line <code>let source = format!(&quot;{}\\n{}&quot;, source, include_str!(&quot;../.…</code> in <code>lessons/04d/src/engine/pipelines/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">let</span> source = format!(
        &quot;{}<span class="s">\\n</span>{}<span class="s">\\n</span>{}&quot;,
        source,
        include_str!(&quot;<span class="s">../../shaders/normals.wgsl</span>&quot;),
        include_str!(&quot;<span class="s">../../shaders/physical.wgsl</span>&quot;)
    );</code></pre></div>
<p>Replaces the line <code>let targets = [Some(wgpu::ColorTargetState {</code> in <code>lessons/04d/src/engine/pipelines/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">let</span> <span class="k">mut</span> targets = vec![Some(wgpu::ColorTargetState {
        format: target.format,
        blend,
        write_mask,
    })];

    <span class="c">// the depth slope target</span>
    <span class="k">if</span> desc.physical {
        targets.push(Some(wgpu::ColorTargetState {
            format: wgpu::TextureFormat::Rg16Float,
            blend: None,
            <span class="c">// a ReadOnlyEqual pass already wrote it: declare, do not write</span>
            write_mask: <span class="k">if</span> desc.depth == DepthMode::ReadOnlyEqual {
                wgpu::ColorWrites::empty()
            } <span class="k">else</span> {
                wgpu::ColorWrites::ALL
            },
        }));
    }</code></pre></div>
<h2 id="step-12-srcenginepipelineslayoutsrs">Step 12 · src/engine/pipelines/layouts.rs<a class="anchor" href="#/course/05-visibility#step-12-srcenginepipelineslayoutsrs" aria-label="Link to this section">#</a></h2>
<p>Layouts gain the depth and gradient bindings.</p>
<p><code>lessons/05/src/engine/pipelines/layouts.rs</code> · edit · type this</p>
<p>Added before <code>fn ink_instance_layout</code> in <code>lessons/04d/src/engine/pipelines/layouts.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// A float texture binding for the fragment stage, unfiltered.</span>
<span class="k">fn</span> scene_gradient(binding: u32, multisampled: bool) -&gt; wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::FRAGMENT,
        ty: wgpu::BindingType::Texture {
            sample_type: wgpu::TextureSampleType::Float { filterable: <span class="s">false</span> },
            view_dimension: wgpu::TextureViewDimension::D2,
            multisampled,
        },
        count: None,
    }
}</code></pre></div>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            scene_depth(<span class="s">3</span>, <span class="s">true</span>),</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            scene_gradient(<span class="s">4</span>, <span class="s">false</span>),
            scene_gradient(<span class="s">5</span>, <span class="s">true</span>),</code></pre></div>
<h2 id="step-13-srcenginegpuobjectsrs">Step 13 · src/engine/gpu/objects.rs<a class="anchor" href="#/course/05-visibility#step-13-srcenginegpuobjectsrs" aria-label="Link to this section">#</a></h2>
<p>The object table binds the new textures.</p>
<p><code>lessons/05/src/engine/gpu/objects.rs</code> · edit · type this</p>
<p>Added after the <code>binding: 3</code> entry in <code>lessons/04d/src/engine/gpu/objects.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            wgpu::BindGroupEntry {
                binding: <span class="s">4</span>,
                resource: wgpu::BindingResource::TextureView(&amp;targets.gradient_single),
            },
            wgpu::BindGroupEntry {
                binding: <span class="s">5</span>,
                resource: wgpu::BindingResource::TextureView(&amp;targets.gradient_msaa),
            },</code></pre></div>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        depths: [&amp;wgpu::TextureView; <span class="s">2</span>],</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        gradients: [&amp;wgpu::TextureView; <span class="s">2</span>],</code></pre></div>
<p>Added above</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            ],</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                wgpu::BindGroupEntry {
                    binding: <span class="s">4</span>,
                    resource: wgpu::BindingResource::TextureView(gradients[<span class="s">0</span>]),
                },
                wgpu::BindGroupEntry {
                    binding: <span class="s">5</span>,
                    resource: wgpu::BindingResource::TextureView(gradients[<span class="s">1</span>]),
                },</code></pre></div>
<h2 id="step-14-srcenginegpuarenars">Step 14 · src/engine/gpu/arena.rs<a class="anchor" href="#/course/05-visibility#step-14-srcenginegpuarenars" aria-label="Link to this section">#</a></h2>
<p>The arena draws a physical pass before the colour pass.</p>
<p><code>lessons/05/src/engine/gpu/arena.rs</code> · edit · type this</p>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    id_faces: wgpu::RenderPipeline,</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    selection_mask: wgpu::RenderPipeline, <span class="c">// marks selected faces</span></code></pre></div>
<p>Added above</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Sheet fills: same vertex table, depth write off, so a page's exactly coplanar regions</span></code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Draw the selected faces into a mask.</span>
    <span class="k">pub</span> <span class="k">fn</span> draw_selection_mask(&amp;<span class="k">self</span>, pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;, b: &amp;Binds) -&gt; u32 {
        <span class="k">self</span>.draw_run(pass, b, &amp;<span class="k">self</span>.pipes.selection_mask, &amp;<span class="k">self</span>.faces)
    }</code></pre></div>
<p>Replaces the line <code>.draw_ids(pass, b, &amp;self.outline_buffers(&amp;self.print))</code> in <code>lessons/04d/src/engine/gpu/arena.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                .draw_physical_ids(pass, b, &amp;<span class="k">self</span>.outline_buffers(&amp;<span class="k">self</span>.print))</code></pre></div>
<p>Replaces the lines from <code>faces: build(dev, target, &amp;base.with(&quot;triangle&quot;, &quot;fs_main…</code> to <code>id_faces: build(dev, Target::ID, &amp;base.with(&quot;triangle.id&quot;…</code> in <code>lessons/04d/src/engine/gpu/arena.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        faces: build(dev, target, &amp;base.with(&quot;<span class="s">triangle</span>&quot;, &quot;<span class="s">fs_main</span>&quot;).physical()),
        id_faces: build(
            dev,
            Target::ID,
            &amp;base.with(&quot;<span class="s">triangle.id</span>&quot;, &quot;<span class="s">fs_id</span>&quot;).physical(),
        ),
        selection_mask: build(
            dev,
            Target {
                format: wgpu::TextureFormat::R8Unorm,
                samples: target.samples,
            },
            &amp;base
                .with(&quot;<span class="s">triangle.selection_mask</span>&quot;, &quot;<span class="s">fs_selection_mask</span>&quot;)
                .depth(<span class="k">crate</span>::engine::pipelines::DepthMode::ReadOnlyEqual),
        ),</code></pre></div>
<h2 id="step-15-srcenginegpusplatrs">Step 15 · src/engine/gpu/splat.rs<a class="anchor" href="#/course/05-visibility#step-15-srcenginegpusplatrs" aria-label="Link to this section">#</a></h2>
<p>The splat pass runs against the physical depth.</p>
<p><code>lessons/05/src/engine/gpu/splat.rs</code> · edit · type this</p>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        .vertex(&quot;<span class="s">vs_point</span>&quot;);</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">let</span> desc = <span class="k">if</span> v.target == Target::ID {
        desc.physical()
    } <span class="k">else</span> {
        desc
    };</code></pre></div>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        .with(&quot;<span class="s">splat.resolve</span>&quot;, &quot;<span class="s">fs_main</span>&quot;)</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        .physical()</code></pre></div>
<h2 id="step-16-srcenginegputext_outliners">Step 16 · src/engine/gpu/text_outline.rs<a class="anchor" href="#/course/05-visibility#step-16-srcenginegputext_outliners" aria-label="Link to this section">#</a></h2>
<p>Outline text draws in both passes.</p>
<p><code>lessons/05/src/engine/gpu/text_outline.rs</code> · edit · type this</p>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    id: wgpu::RenderPipeline,</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    physical_id: wgpu::RenderPipeline, <span class="c">// object ids with depth and gradient</span></code></pre></div>
<p>Replaces the lines from <code>let (color, id) = pipelines(ctx, layouts, &amp;shader, target);</code> to <code>(self.color, self.id) = pipelines(ctx, layouts, &amp;self.sha…</code> in <code>lessons/04d/src/engine/gpu/text_outline.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> (color, id, physical_id) = pipelines(ctx, layouts, &amp;shader, target);
        <span class="k">Self</span> {
            shader,
            color,
            id,
            physical_id,
        }
    }

    <span class="c">/// A pipeline is built for one MSAA sample count, so a new count needs new pipelines.</span>
    <span class="k">pub</span> <span class="k">fn</span> retarget(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, layouts: &amp;Layouts, target: Target) {
        (<span class="k">self</span>.color, <span class="k">self</span>.id, <span class="k">self</span>.physical_id) = pipelines(ctx, layouts, &amp;<span class="k">self</span>.shader, target);</code></pre></div>
<p>Added above</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Preserve the existing exact object IDs and sheet depth comparison in the picking pass.</span></code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Draw object ids, writing depth and gradient too.</span>
    <span class="k">pub</span> <span class="k">fn</span> draw_physical_ids(
        &amp;<span class="k">self</span>,
        pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;,
        binds: &amp;Binds,
        buffers: &amp;OutlineBuffers&lt;'_&gt;,
    ) -&gt; u32 {
        pass.set_pipeline(&amp;<span class="k">self</span>.physical_id);
        draw(pass, binds, buffers)
    }</code></pre></div>
<p>Replaces the line <code>) -&gt; (wgpu::RenderPipeline, wgpu::RenderPipeline) {</code> in <code>lessons/04d/src/engine/gpu/text_outline.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>) -&gt; (
    wgpu::RenderPipeline,
    wgpu::RenderPipeline,
    wgpu::RenderPipeline,
) {</code></pre></div>
<p>Replaces the line <code>(color, id)</code> in <code>lessons/04d/src/engine/gpu/text_outline.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">let</span> physical_id = build(
        &amp;ctx.device,
        Target::ID,
        &amp;base
            .with(&quot;<span class="s">text-outline.physical_id</span>&quot;, &quot;<span class="s">fs_physical_id</span>&quot;)
            .physical()
            .depth(DepthMode::ReadOnlyEqual),
    );
    (color, id, physical_id)</code></pre></div>
<h2 id="step-17-srcenginegpumodrs">Step 17 · src/engine/gpu/mod.rs<a class="anchor" href="#/course/05-visibility#step-17-srcenginegpumodrs" aria-label="Link to this section">#</a></h2>
<p>The GPU owner runs the physical pass, then every lane over it.</p>
<p><code>lessons/05/src/engine/gpu/mod.rs</code> · edit · type this</p>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> <span class="k">mod</span> arena;</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> <span class="k">mod</span> backdrop;</code></pre></div>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> <span class="k">use</span> segments::CylinderSegment;</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> targets::Targets;</code></pre></div>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> objects: objects::InstanceTable,</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> backdrop: backdrop::BackdropLane,</code></pre></div>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> objects = objects::InstanceTable::new(&amp;ctx, &amp;layouts, &amp;InkScene { targets: &amp;targets });</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> backdrop = backdrop::BackdropLane::new(&amp;ctx, &amp;layouts, target);</code></pre></div>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            objects,</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            backdrop,</code></pre></div>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.bounds.union_with(&amp;up.bounds);</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.retarget(<span class="s">false</span>);</code></pre></div>
<p>Replaces the lines from <code>self.targets = targets::Targets::new(</code> to <code>);</code> in <code>lessons/04d/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.retarget(<span class="s">true</span>);
        <span class="k">self</span>.splat.resize();
    }

    <span class="c">/// Current color format and sample count.</span>
    <span class="k">fn</span> target(&amp;<span class="k">self</span>) -&gt; Target {
        Target {
            format: <span class="k">self</span>.config.format,
            samples: <span class="k">self</span>.targets.samples,
        }
    }

    <span class="c">/// Remake targets and pipelines when the sample count changes.</span>
    <span class="k">fn</span> retarget(&amp;<span class="k">mut</span> <span class="k">self</span>, resized: bool) {
        <span class="k">let</span> samples = <span class="k">self</span>.msaa_now();
        <span class="k">let</span> flip = samples != <span class="k">self</span>.targets.samples;

        <span class="k">if</span> flip || resized {
            <span class="k">self</span>.targets = Targets::new(
                &amp;<span class="k">self</span>.ctx,
                (<span class="k">self</span>.config.width, <span class="k">self</span>.config.height),
                <span class="k">self</span>.config.format,
                samples,
            );
            <span class="k">self</span>.objects.rebind_ink(
                &amp;<span class="k">self</span>.ctx,
                &amp;<span class="k">self</span>.layouts,
                &amp;InkScene {
                    targets: &amp;<span class="k">self</span>.targets,
                },
            );
        }

        <span class="k">if</span> flip {
            <span class="k">let</span> target = <span class="k">self</span>.target();
            <span class="k">self</span>.backdrop.retarget(&amp;<span class="k">self</span>.ctx, &amp;<span class="k">self</span>.layouts, target);
            <span class="k">self</span>.arena.retarget(&amp;<span class="k">self</span>.ctx, &amp;<span class="k">self</span>.layouts, target);
            <span class="k">self</span>.segments.retarget(&amp;<span class="k">self</span>.ctx, &amp;<span class="k">self</span>.layouts, target);
            <span class="k">self</span>.glyphs.retarget(&amp;<span class="k">self</span>.ctx, &amp;<span class="k">self</span>.layouts, target);
            <span class="k">self</span>.splat.retarget(&amp;<span class="k">self</span>.ctx, &amp;<span class="k">self</span>.layouts, target);
            log::info!(&quot;<span class="s">msaa: </span>{}<span class="s">x</span>&quot;, samples);
        }
    }

    <span class="c">/// Pixels this GPU can afford at 4x MSAA.</span>
    <span class="k">pub</span> <span class="k">fn</span> msaa_budget(&amp;<span class="k">self</span>) -&gt; Option&lt;u32&gt; {
        Targets::msaa_budget(<span class="k">self</span>.device_type)
    }

    <span class="c">/// MSAA samples for the current scene: 4x only with solid geometry.</span>
    <span class="k">fn</span> msaa_now(&amp;<span class="k">self</span>) -&gt; u32 {
        <span class="k">let</span> solid = <span class="k">self</span>.arena.face_count() &gt; <span class="s">0</span>
            || <span class="k">self</span>.arena.sheet_count() &gt; <span class="s">0</span>
            || <span class="k">self</span>.segments.pipe_count() &gt; <span class="s">0</span>
            || <span class="k">self</span>.glyphs.sphere_count() &gt; <span class="s">0</span>;
        Targets::samples_for(
            solid,
            <span class="k">self</span>.config.width * <span class="k">self</span>.config.height,
            <span class="k">self</span>.view.msaa_forced,
            <span class="k">self</span>.msaa_budget(),
        )
    }</code></pre></div>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">let</span> <span class="k">mut</span> pass = <span class="k">self</span>.targets.begin_faces(&amp;<span class="k">mut</span> encoder, &amp;target, input.clear);</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">self</span>.backdrop.draw_background(&amp;<span class="k">mut</span> pass);

            <span class="k">if</span> <span class="k">self</span>.view.show_grid {
                <span class="k">self</span>.backdrop.draw_grid(&amp;<span class="k">mut</span> pass, &amp;basic);
            }</code></pre></div>
<h2 id="step-18-srcfixturers">Step 18 · src/fixture.rs<a class="anchor" href="#/course/05-visibility#step-18-srcfixturers" aria-label="Link to this section">#</a></h2>
<p>Copy the test scene: a box with edges behind faces.
Copy this file from the lesson folder to the path shown.</p>
<p><code>lessons/05/src/fixture.rs</code> · edit · copy the file</p>
<p>Replaces the lines from <code>use crate::engine::gpu::{</code> to <code>facing_ext: [0xffffffff; 2],</code> in <code>lessons/04d/src/fixture.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> <span class="k">crate</span>::engine::gpu::{CylinderSegment, GlyphPoint, ObjectRow, Upload};
<span class="k">use</span> session_rust::AABB;
<span class="k">use</span> session_rust::Point;
<span class="k">use</span> session_rust::{RenderVertex, Xform};

<span class="c">/// Small test scenes built in code, no loading.</span>
<span class="k">pub</span> <span class="k">fn</span> scene() -&gt; Upload {
    <span class="k">if</span> <span class="k">crate</span>::app::route::query(&quot;<span class="s">fixture</span>&quot;).as_deref() == Some(&quot;<span class="s">floor</span>&quot;) {
        floor()
    } <span class="k">else</span> {
        grey_box()
    }
}

<span class="c">/// A box: six faces, twelve red edges, eight markers.</span>
<span class="k">fn</span> grey_box() -&gt; Upload {
    <span class="k">let</span> <span class="k">mut</span> upload = Upload::default();
    <span class="k">let</span> points = [
        [-<span class="s">0</span>.<span class="s">5</span>, -<span class="s">0</span>.<span class="s">5</span>, -<span class="s">0</span>.<span class="s">5</span>],
        [<span class="s">0</span>.<span class="s">5</span>, -<span class="s">0</span>.<span class="s">5</span>, -<span class="s">0</span>.<span class="s">5</span>],
        [<span class="s">0</span>.<span class="s">5</span>, <span class="s">0</span>.<span class="s">5</span>, -<span class="s">0</span>.<span class="s">5</span>],
        [-<span class="s">0</span>.<span class="s">5</span>, <span class="s">0</span>.<span class="s">5</span>, -<span class="s">0</span>.<span class="s">5</span>],
        [-<span class="s">0</span>.<span class="s">5</span>, -<span class="s">0</span>.<span class="s">5</span>, <span class="s">0</span>.<span class="s">5</span>],
        [<span class="s">0</span>.<span class="s">5</span>, -<span class="s">0</span>.<span class="s">5</span>, <span class="s">0</span>.<span class="s">5</span>],
        [<span class="s">0</span>.<span class="s">5</span>, <span class="s">0</span>.<span class="s">5</span>, <span class="s">0</span>.<span class="s">5</span>],
        [-<span class="s">0</span>.<span class="s">5</span>, <span class="s">0</span>.<span class="s">5</span>, <span class="s">0</span>.<span class="s">5</span>],
    ];
    upload.bounds = AABB::from_points(
        &amp;[Point::new(-<span class="s">0</span>.<span class="s">5</span>, -<span class="s">0</span>.<span class="s">5</span>, -<span class="s">0</span>.<span class="s">5</span>), Point::new(<span class="s">0</span>.<span class="s">5</span>, <span class="s">0</span>.<span class="s">5</span>, <span class="s">0</span>.<span class="s">5</span>)],
        <span class="s">0</span>.<span class="s">0</span>,
    );
    row(&amp;<span class="k">mut</span> upload, <span class="s">true</span>);
    <span class="k">let</span> faces = [
        ([<span class="s">0</span>, <span class="s">3</span>, <span class="s">2</span>, <span class="s">1</span>], [<span class="s">0</span>., <span class="s">0</span>., -<span class="s">1</span>.]),
        ([<span class="s">4</span>, <span class="s">5</span>, <span class="s">6</span>, <span class="s">7</span>], [<span class="s">0</span>., <span class="s">0</span>., <span class="s">1</span>.]),
        ([<span class="s">0</span>, <span class="s">1</span>, <span class="s">5</span>, <span class="s">4</span>], [<span class="s">0</span>., -<span class="s">1</span>., <span class="s">0</span>.]),
        ([<span class="s">3</span>, <span class="s">7</span>, <span class="s">6</span>, <span class="s">2</span>], [<span class="s">0</span>., <span class="s">1</span>., <span class="s">0</span>.]),
        ([<span class="s">0</span>, <span class="s">4</span>, <span class="s">7</span>, <span class="s">3</span>], [-<span class="s">1</span>., <span class="s">0</span>., <span class="s">0</span>.]),
        ([<span class="s">1</span>, <span class="s">2</span>, <span class="s">6</span>, <span class="s">5</span>], [<span class="s">1</span>., <span class="s">0</span>., <span class="s">0</span>.]),
    ];

    <span class="k">for</span> (corners, normal) <span class="k">in</span> faces {
        <span class="k">let</span> <span class="k">mut</span> positions = [[<span class="s">0</span>.<span class="s">0</span>; <span class="s">3</span>]; <span class="s">4</span>];

        <span class="k">for</span> (slot, corner) <span class="k">in</span> corners.into_iter().enumerate() {
            positions[slot] = points[corner];
        }

        quad(&amp;<span class="k">mut</span> upload, <span class="s">0</span>, positions, normal);
    }

    <span class="k">for</span> [first, last] <span class="k">in</span> [
        [<span class="s">0</span>, <span class="s">1</span>],
        [<span class="s">1</span>, <span class="s">2</span>],
        [<span class="s">2</span>, <span class="s">3</span>],
        [<span class="s">3</span>, <span class="s">0</span>],
        [<span class="s">4</span>, <span class="s">5</span>],
        [<span class="s">5</span>, <span class="s">6</span>],
        [<span class="s">6</span>, <span class="s">7</span>],
        [<span class="s">7</span>, <span class="s">4</span>],
        [<span class="s">0</span>, <span class="s">4</span>],
        [<span class="s">1</span>, <span class="s">5</span>],
        [<span class="s">2</span>, <span class="s">6</span>],
        [<span class="s">3</span>, <span class="s">7</span>],
    ] {
        stroke(&amp;<span class="k">mut</span> upload, <span class="s">0</span>, points[first], points[last], <span class="s">0xff0000ff</span>);
    }

    <span class="k">for</span> point <span class="k">in</span> points {
        marker(&amp;<span class="k">mut</span> upload, <span class="s">0</span>, point, [<span class="s">0</span>., <span class="s">0</span>., <span class="s">0</span>., <span class="s">1</span>.]);
    }

    upload
}

<span class="c">/// Magenta ink 4 mm under a sloping floor.</span>
<span class="k">fn</span> floor() -&gt; Upload {
    <span class="k">let</span> <span class="k">mut</span> upload = Upload::default();
    upload.bounds = AABB::from_points(
        &amp;[Point::new(-<span class="s">2</span>.<span class="s">0</span>, -<span class="s">1</span>.<span class="s">5</span>, -<span class="s">0</span>.<span class="s">504</span>), Point::new(<span class="s">2</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">5</span>, <span class="s">0</span>.<span class="s">5</span>)],
        <span class="s">0</span>.<span class="s">0</span>,
    );
    row(&amp;<span class="k">mut</span> upload, <span class="s">true</span>);
    row(&amp;<span class="k">mut</span> upload, <span class="s">false</span>);
    quad(
        &amp;<span class="k">mut</span> upload,
        <span class="s">0</span>,
        [
            [-<span class="s">2</span>., -<span class="s">1</span>.<span class="s">5</span>, -<span class="s">0</span>.<span class="s">5</span>],
            [<span class="s">2</span>., -<span class="s">1</span>.<span class="s">5</span>, <span class="s">0</span>.<span class="s">5</span>],
            [<span class="s">2</span>., <span class="s">1</span>.<span class="s">5</span>, <span class="s">0</span>.<span class="s">5</span>],
            [-<span class="s">2</span>., <span class="s">1</span>.<span class="s">5</span>, -<span class="s">0</span>.<span class="s">5</span>],
        ],
        [-<span class="s">0</span>.<span class="s">24253562</span>, <span class="s">0</span>., <span class="s">0</span>.<span class="s">9701425</span>],
    );

    <span class="k">for</span> y <span class="k">in</span> [-<span class="s">0</span>.<span class="s">6</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">6</span>] {
        stroke(&amp;<span class="k">mut</span> upload, <span class="s">1</span>, [-<span class="s">1</span>., y, -<span class="s">0</span>.<span class="s">254</span>], [<span class="s">1</span>., y, <span class="s">0</span>.<span class="s">246</span>], <span class="s">0xffff00ff</span>);
        marker(&amp;<span class="k">mut</span> upload, <span class="s">1</span>, [<span class="s">0</span>., y, -<span class="s">0</span>.<span class="s">004</span>], [<span class="s">1</span>., <span class="s">0</span>., <span class="s">1</span>., <span class="s">1</span>.]);
    }

    stroke(
        &amp;<span class="k">mut</span> upload,
        <span class="s">0</span>,
        [-<span class="s">1</span>.<span class="s">3</span>, -<span class="s">1</span>., -<span class="s">0</span>.<span class="s">325</span>],
        [<span class="s">1</span>.<span class="s">3</span>, -<span class="s">1</span>., <span class="s">0</span>.<span class="s">325</span>],
        <span class="s">0xffff0000</span>,
    );
    upload
}

<span class="c">/// One entry per object; index = row.</span>
<span class="k">fn</span> row(upload: &amp;<span class="k">mut</span> Upload, faces: bool) {
    <span class="k">let</span> <span class="k">mut</span> row = ObjectRow::new(Xform::identity(), <span class="s">0</span>);
    row.bounds = upload.bounds;
    row.faces = faces;
    upload.obj.rows.push(row);
}

<span class="c">/// Two triangles on the four stroke corners.</span>
<span class="k">fn</span> quad(upload: &amp;<span class="k">mut</span> Upload, row: u32, points: [[f32; <span class="s">3</span>]; <span class="s">4</span>], normal: [f32; <span class="s">3</span>]) {
    <span class="k">let</span> first = upload.arena.verts.len() <span class="k">as</span> u32;

    <span class="k">for</span> position <span class="k">in</span> points {
        upload.arena.verts.push(RenderVertex {
            position,
            normal,
            color: [<span class="s">0</span>.<span class="s">6</span>, <span class="s">0</span>.<span class="s">6</span>, <span class="s">0</span>.<span class="s">6</span>, <span class="s">1</span>.],
        });
        upload.arena.vids.push(row);
    }

    <span class="k">for</span> index <span class="k">in</span> [<span class="s">0</span>, <span class="s">1</span>, <span class="s">2</span>, <span class="s">0</span>, <span class="s">2</span>, <span class="s">3</span>] {
        upload.arena.idx.push(first + index);
    }
}

<span class="c">/// Screen-space ribbon extrusion consumes unchanged source endpoints.</span>
<span class="k">fn</span> stroke(upload: &amp;<span class="k">mut</span> Upload, row: u32, p0: [f32; <span class="s">3</span>], p1: [f32; <span class="s">3</span>], color: u32) {
    upload.seg.ribbons.push(CylinderSegment {
        p0,
        p1,
        radius: <span class="s">0</span>.,
        instance_id: row,
        color,
        facing: u32::MAX,
    });
}

<span class="c">/// Markers test depth the same way as strokes.</span>
<span class="k">fn</span> marker(upload: &amp;<span class="k">mut</span> Upload, row: u32, center: [f32; <span class="s">3</span>], color: [f32; <span class="s">4</span>]) {
    upload.glyph.dots.push(GlyphPoint {
        center,
        radius: <span class="s">0</span>.<span class="s">018</span>,
        color,
        instance_id: row,
        facing: u32::MAX,
        facing_ext: [u32::MAX; <span class="s">2</span>],</code></pre></div>
<h2 id="step-19-srclibrs">Step 19 · src/lib.rs<a class="anchor" href="#/course/05-visibility#step-19-srclibrs" aria-label="Link to this section">#</a></h2>
<p>The entry point reports the sample count.</p>
<p><code>lessons/05/src/lib.rs</code> · edit · type this</p>
<p>Replaces the line <code>camera.set_view(camera::View::Top);</code> in <code>lessons/04d/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        camera.set_view(camera::View::Iso);
        camera.fit(&amp;gpu.bounds, <span class="s">1</span>.<span class="s">5</span>);

        <span class="k">if</span> app::route::query(&quot;<span class="s">top</span>&quot;).is_some() {
            camera.set_view(camera::View::Top);
        }

        camera.perspective = app::route::query(&quot;<span class="s">perspective</span>&quot;).is_some();

        <span class="k">if</span> <span class="k">let</span> Some(distance) = app::route::query(&quot;<span class="s">distance</span>&quot;).and_then(parse_distance) {
            camera.distance *= distance;
            camera.update_position();
        }

        gpu.view.show_grid = <span class="s">false</span>;</code></pre></div>
<p>Replaces the line <code>Ok(serde_json::json!({&quot;stage&quot;:4,&quot;objects&quot;:self.gpu.object…</code> in <code>lessons/04d/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        Ok(serde_json::json!({&quot;<span class="s">stage</span>&quot;:<span class="s">5</span>,&quot;<span class="s">objects</span>&quot;:<span class="k">self</span>.gpu.objects.len(),&quot;<span class="s">width</span>&quot;:w,&quot;<span class="s">height</span>&quot;:h,&quot;<span class="s">scale</span>&quot;:scale,&quot;<span class="s">drawn</span>&quot;:<span class="s">true</span>,&quot;<span class="s">samples</span>&quot;:<span class="k">self</span>.gpu.targets.samples,</code></pre></div>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    JsValue::from_str(&amp;error.to_string())
}</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Clamp the test distance to its range.</span>
<span class="k">fn</span> parse_distance(value: String) -&gt; Option&lt;f64&gt; {
    <span class="k">let</span> value = value.parse::&lt;f64&gt;().ok()?;
    (value.is_finite() &amp;&amp; (<span class="s">1</span>.<span class="s">0</span>..=<span class="s">16</span>.<span class="s">0</span>).contains(&amp;value)).then_some(value)
}</code></pre></div>
<h2 id="step-20-indexhtml">Step 20 · index.html<a class="anchor" href="#/course/05-visibility#step-20-indexhtml" aria-label="Link to this section">#</a></h2>
<p>Copy the page: the status says checkpoint 05.
Copy this file from the lesson folder to the path shown.</p>
<p><code>lessons/05/index.html</code> · edit · copy the file</p>
<p>Replaces the line <code>&lt;title&gt;Session checkpoint 04&lt;/title&gt;</code> in <code>lessons/04d/index.html</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    &lt;title&gt;Session checkpoint 05&lt;/title&gt;</code></pre></div>
<p>Replaces the line <code>&lt;output id=&quot;status&quot;&gt;Starting checkpoint 04&lt;/output&gt;</code> in <code>lessons/04d/index.html</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    &lt;output id=&quot;<span class="s">status</span>&quot;&gt;Starting checkpoint 05&lt;/output&gt;</code></pre></div>
<p>Replaces the line <code>document.getElementById(&#39;status&#39;).textContent = &#39;Checkpoi…</code> in <code>lessons/04d/index.html</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        document.getElementById('status').textContent = 'Checkpoint 05 · ' + state.objects + ' objects · ' + state.width + '×' + state.height;</code></pre></div>
<h2 id="check">Check<a class="anchor" href="#/course/05-visibility#check" aria-label="Link to this section">#</a></h2>
<p>Run <code>trunk serve</code> in <code>lessons/05/</code> and open <a href="http://127.0.0.1:8770/" target="_blank" rel="noopener">http://127.0.0.1:8770/</a>.</p>
<p>Expected: A grey box shows its visible red edges and black corners while its faces hide the far edges; status: <strong>Checkpoint 05 · 1 objects</strong>.</p>
<p><img src="/session/docs/course/docs/screenshots/05.png" alt="Checkpoint 05: hidden lines stay hidden while visible strokes and corners stay readable, over the white backdrop." loading="lazy" decoding="async"></p>
<p>If it fails:</p>
<ul>
<li>Every edge disappears: the depth clear and comparison disagree with reversed depth.</li>
<li>Hidden edges show through: the face pipeline omits its physical depth gradient.</li>
</ul>
<h2 id="what-changed">What changed<a class="anchor" href="#/course/05-visibility#what-changed" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>lessons/05/src/
├── app/
│   ├── mod.rs
│   └── route.rs
├── engine/
│   ├── gpu/
│   │   ├── arena.rs  ~
│   │   ├── backdrop.rs  +
│   │   ├── buffers.rs
│   │   ├── cloud.rs
│   │   ├── frame.rs
│   │   ├── glyphs.rs
│   │   ├── instance.rs  ~
│   │   ├── lod.rs
│   │   ├── mod.rs  ~
│   │   ├── objects.rs  ~
│   │   ├── segments.rs
│   │   ├── splat.rs  ~
│   │   ├── targets.rs  ~
│   │   ├── text_outline.rs  ~
│   │   ├── upload.rs
│   │   └── view.rs
│   ├── pipelines/
│   │   ├── layouts.rs  ~
│   │   └── mod.rs  ~
│   └── mod.rs
├── shaders/
│   ├── background.wgsl  +
│   ├── glyph.wgsl
│   ├── grid.wgsl  +
│   ├── ink_visibility.wgsl  ~
│   ├── normals.wgsl
│   ├── physical.wgsl  +
│   ├── ribbon.wgsl
│   ├── scene.wgsl
│   ├── sphere.wgsl
│   ├── splat.wgsl  ~
│   ├── splat_resolve.wgsl  ~
│   ├── text_outline.wgsl  ~
│   └── triangle.wgsl  ~
├── camera.rs
├── fixture.rs  ~
└── lib.rs  ~</code></pre></div>
<p><code>+</code> new in this lesson · <code>~</code> changed in this lesson</p>
<p>Data flow: source files → retained scene state → GPU buffers → visible result. Every file at this point: <code>lessons/05/</code>.</p>
<h2 id="next">Next<a class="anchor" href="#/course/05-visibility#next" aria-label="Link to this section">#</a></h2>
<p><a href="#/course/06-cad-contract">06 · CAD face rules</a></p>
<h2 id="expected-viewer-result">Expected viewer result<a class="anchor" href="#/course/05-visibility#expected-viewer-result" aria-label="Link to this section">#</a></h2>
<p>Checkpoint 05: hidden lines stay hidden while visible strokes and corners stay readable, over the white backdrop.</p>
<p><a href="/session/docs/course/docs/screenshots/05.png"><img src="/session/docs/course/docs/screenshots/05.png" alt="Full viewer result for 05 visibility" loading="lazy" decoding="async"></a></p>
`,toc:[{level:2,id:"step-1-srcshadersphysicalwgsl",text:"Step 1 · src/shaders/physical.wgsl"},{level:2,id:"step-2-srcshadersbackgroundwgsl",text:"Step 2 · src/shaders/background.wgsl"},{level:2,id:"step-3-srcshadersgridwgsl",text:"Step 3 · src/shaders/grid.wgsl"},{level:2,id:"step-4-srcenginegpubackdroprs",text:"Step 4 · src/engine/gpu/backdrop.rs"},{level:2,id:"step-5-srcshadersink_visibilitywgsl",text:"Step 5 · src/shaders/ink_visibility.wgsl"},{level:2,id:"step-6-srcshaderstrianglewgsl",text:"Step 6 · src/shaders/triangle.wgsl"},{level:2,id:"step-7-srcshaderssplatwgsl",text:"Step 7 · src/shaders/splat.wgsl"},{level:2,id:"step-8-srcshaderssplat_resolvewgsl",text:"Step 8 · src/shaders/splat_resolve.wgsl"},{level:2,id:"step-9-srcshaderstext_outlinewgsl",text:"Step 9 · src/shaders/text_outline.wgsl"},{level:2,id:"step-10-srcenginegputargetsrs",text:"Step 10 · src/engine/gpu/targets.rs"},{level:2,id:"step-11-srcenginepipelinesmodrs",text:"Step 11 · src/engine/pipelines/mod.rs"},{level:2,id:"step-12-srcenginepipelineslayoutsrs",text:"Step 12 · src/engine/pipelines/layouts.rs"},{level:2,id:"step-13-srcenginegpuobjectsrs",text:"Step 13 · src/engine/gpu/objects.rs"},{level:2,id:"step-14-srcenginegpuarenars",text:"Step 14 · src/engine/gpu/arena.rs"},{level:2,id:"step-15-srcenginegpusplatrs",text:"Step 15 · src/engine/gpu/splat.rs"},{level:2,id:"step-16-srcenginegputext_outliners",text:"Step 16 · src/engine/gpu/text_outline.rs"},{level:2,id:"step-17-srcenginegpumodrs",text:"Step 17 · src/engine/gpu/mod.rs"},{level:2,id:"step-18-srcfixturers",text:"Step 18 · src/fixture.rs"},{level:2,id:"step-19-srclibrs",text:"Step 19 · src/lib.rs"},{level:2,id:"step-20-indexhtml",text:"Step 20 · index.html"},{level:2,id:"check",text:"Check"},{level:2,id:"what-changed",text:"What changed"},{level:2,id:"next",text:"Next"},{level:2,id:"expected-viewer-result",text:"Expected viewer result"}]};export{s as default};
