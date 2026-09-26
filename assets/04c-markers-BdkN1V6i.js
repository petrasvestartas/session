const s={title:"04c · Markers",html:`<h1 id="04c-markers">04c · Markers<a class="anchor" href="#/course/04c-markers#04c-markers" aria-label="Link to this section">#</a></h1>
<p>An orange point appears below the triangle and fades smoothly at small sizes.</p>
<p><img src="/session/docs/course/docs/illustrations/markers.svg" alt="A sphere is four template corners pushed out by the pixel radius plus half the feather; a free dot is one equilateral triangle whose incircle is the disc." loading="lazy" decoding="async"></p>
<h2 id="step-1-srcenginegpuglyphsrs">Step 1 · src/engine/gpu/glyphs.rs<a class="anchor" href="#/course/04c-markers#step-1-srcenginegpuglyphsrs" aria-label="Link to this section">#</a></h2>
<p>New file: the lane that draws vertex markers and flat dots, one 48-byte row each.</p>
<p><code>lessons/04c/src/engine/gpu/glyphs.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> super::buffers::{GpuCtx, GrowBuf, ROWS, Template, bind_group};
<span class="k">use</span> super::frame::Binds;
<span class="k">use</span> super::upload::drop_rows;
<span class="k">use</span> <span class="k">crate</span>::engine::pipelines::{
    ColorWrite, DepthMode, Layouts, PipelineDesc, Target, build, ink_module, template_layout,
};
<span class="k">use</span> wgpu::PrimitiveTopology::TriangleList;

<span class="c">/// Shader sources the tests compare against the files.</span>
#[cfg(test)]
<span class="k">pub</span> <span class="k">const</span> SHADERS: &amp;[(&amp;str, &amp;str)] = &amp;[
    (&quot;<span class="s">sphere.wgsl</span>&quot;, include_str!(&quot;<span class="s">../../shaders/sphere.wgsl</span>&quot;)),
    (&quot;<span class="s">glyph.wgsl</span>&quot;, include_str!(&quot;<span class="s">../../shaders/glyph.wgsl</span>&quot;)),
];

<span class="c">/// A dot is a triangle big enough to contain the circle; the fragment shader discards the pixels outside it.</span>
<span class="k">const</span> DOT_VERTS: u32 = <span class="s">3</span>;

<span class="c">/// One marker or dot, 48 bytes, as the shaders read it.</span>
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
<span class="k">pub</span> <span class="k">struct</span> GlyphPoint {
    <span class="k">pub</span> center: [f32; <span class="s">3</span>], <span class="c">// object space; the shader's place() moves it into the scene</span>
    <span class="k">pub</span> radius: f32, <span class="c">// 0 = pen width; &gt; 0 world mm; &lt; 0 screen px</span>
    <span class="k">pub</span> color: [f32; <span class="s">4</span>],
    <span class="k">pub</span> instance_id: u32, <span class="c">// object row</span>
    <span class="k">pub</span> facing: u32, <span class="c">// two 16-bit octahedral normals of the faces around the point</span>
    <span class="k">pub</span> facing_ext: [u32; <span class="s">2</span>], <span class="c">// four more; a cloud dot keeps its point row in [0] instead</span>
}

<span class="k">const</span> _: () = assert!(std::mem::size_of::&lt;GlyphPoint&gt;() == <span class="s">48</span>);

<span class="c">/// Marker and dot rows of one upload.</span>
#[derive(Default)]
<span class="k">pub</span> <span class="k">struct</span> GlyphRows {
    <span class="k">pub</span> spheres: Vec&lt;GlyphPoint&gt;, <span class="c">// vertex markers, shaded</span>
    <span class="k">pub</span> dots: Vec&lt;GlyphPoint&gt;,
}

<span class="k">impl</span> GlyphRows {
    <span class="c">/// Empty both tables and free their memory.</span>
    <span class="k">pub</span> <span class="k">fn</span> drop_rows(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        drop_rows(&amp;<span class="k">mut</span> <span class="k">self</span>.spheres);
        drop_rows(&amp;<span class="k">mut</span> <span class="k">self</span>.dots);
    }
}</code></pre></div>
<p><code>lessons/04c/src/engine/gpu/glyphs.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// One glyph buffer and its bind group.</span>
<span class="k">struct</span> GlyphTable {
    label: &amp;'static str,
    buf: GrowBuf,
    group: wgpu::BindGroup, <span class="c">// group 3</span>
}

<span class="k">impl</span> GlyphTable {
    <span class="k">fn</span> new(ctx: &amp;GpuCtx, l: &amp;Layouts, label: &amp;'static str) -&gt; <span class="k">Self</span> {
        <span class="k">let</span> buf = GrowBuf::new(ctx, label, std::mem::size_of::&lt;GlyphPoint&gt;() <span class="k">as</span> u64, ROWS);
        <span class="k">let</span> group = bind_group(ctx, &amp;l.ink_rows, label, &amp;[&amp;buf.buf]);
        <span class="k">Self</span> { label, buf, group }
    }

    <span class="c">/// A bind group points at one buffer; once GrowBuf replaces it, the old group is stale.</span>
    <span class="k">fn</span> rebind(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, l: &amp;Layouts) {
        <span class="k">self</span>.group = bind_group(ctx, &amp;l.ink_rows, <span class="k">self</span>.label, &amp;[&amp;<span class="k">self</span>.buf.buf]);
    }
}

<span class="k">struct</span> GlyphShaders {
    sphere: wgpu::ShaderModule,
    dot: wgpu::ShaderModule,
}

<span class="k">struct</span> GlyphPipelines {
    sphere: wgpu::RenderPipeline,
    dot: wgpu::RenderPipeline,
    id_sphere: wgpu::RenderPipeline, <span class="c">// pick ids</span>
    id_dot: wgpu::RenderPipeline,
    source_dot: wgpu::RenderPipeline, <span class="c">// pick ids of cloud points: object row and point row</span>
}

<span class="c">/// Markers = small shaded spheres at mesh vertices; dots = flat discs for points.</span>
<span class="k">pub</span> <span class="k">struct</span> GlyphLane {
    spheres: GlyphTable,
    dots: GlyphTable,
    template: Template, <span class="c">// one quad, drawn once per marker</span>
    shaders: GlyphShaders,
    gpu: GlyphPipelines,
}</code></pre></div>
<p><code>lessons/04c/src/engine/gpu/glyphs.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">impl</span> GlyphLane {
    <span class="k">pub</span> <span class="k">fn</span> allocated_bytes(&amp;<span class="k">self</span>) -&gt; u64 {
        <span class="k">self</span>.spheres.buf.buf.size()
            + <span class="k">self</span>.dots.buf.buf.size()
            + <span class="k">self</span>.template.vbo.size()
            + <span class="k">self</span>.template.ibo.size()
    }

    <span class="k">pub</span> <span class="k">fn</span> new(ctx: &amp;GpuCtx, l: &amp;Layouts, target: Target) -&gt; <span class="k">Self</span> {
        <span class="k">let</span> (q_v, q_i) = unit_quad();
        <span class="k">let</span> template = Template::new(ctx, &quot;<span class="s">quad.template</span>&quot;, &amp;q_v, &amp;q_i);
        <span class="k">let</span> shaders = GlyphShaders {
            sphere: ink_module(
                &amp;ctx.device,
                &quot;<span class="s">sphere.shader</span>&quot;,
                include_str!(&quot;<span class="s">../../shaders/sphere.wgsl</span>&quot;),
            ),
            dot: ink_module(
                &amp;ctx.device,
                &quot;<span class="s">glyph.shader</span>&quot;,
                include_str!(&quot;<span class="s">../../shaders/glyph.wgsl</span>&quot;),
            ),
        };
        <span class="k">let</span> gpu = build_pipelines(ctx, l, &amp;shaders, target);
        <span class="k">let</span> spheres = GlyphTable::new(ctx, l, &quot;<span class="s">spheres</span>&quot;);
        <span class="k">let</span> dots = GlyphTable::new(ctx, l, &quot;<span class="s">dots</span>&quot;);
        <span class="k">Self</span> {
            spheres,
            dots,
            template,
            shaders,
            gpu,
        }
    }

    <span class="c">/// Rebuild the pipelines for a new MSAA sample count.</span>
    <span class="k">pub</span> <span class="k">fn</span> retarget(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, l: &amp;Layouts, target: Target) {
        <span class="k">self</span>.gpu = build_pipelines(ctx, l, &amp;<span class="k">self</span>.shaders, target);
    }

    <span class="c">/// Append one upload's rows to both tables.</span>
    <span class="k">pub</span> <span class="k">fn</span> append(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, l: &amp;Layouts, up: &amp;GlyphRows) {
        <span class="k">if</span> <span class="k">self</span>.spheres.buf.append(ctx, &amp;up.spheres) {
            <span class="k">self</span>.spheres.rebind(ctx, l);
        }

        <span class="k">if</span> <span class="k">self</span>.dots.buf.append(ctx, &amp;up.dots) {
            <span class="k">self</span>.dots.rebind(ctx, l);
        }
    }</code></pre></div>
<p><code>lessons/04c/src/engine/gpu/glyphs.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> <span class="k">fn</span> draw_spheres(&amp;<span class="k">self</span>, pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;, b: &amp;Binds) -&gt; u32 {
        <span class="k">self</span>.draw_markers(pass, b, &amp;<span class="k">self</span>.gpu.sphere)
    }

    <span class="k">pub</span> <span class="k">fn</span> draw_dots(&amp;<span class="k">self</span>, pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;, b: &amp;Binds) -&gt; u32 {
        <span class="k">self</span>.draw_dot_table(pass, b, &amp;<span class="k">self</span>.gpu.dot)
    }

    <span class="k">pub</span> <span class="k">fn</span> draw_sphere_ids(&amp;<span class="k">self</span>, pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;, b: &amp;Binds) -&gt; u32 {
        <span class="k">self</span>.draw_markers(pass, b, &amp;<span class="k">self</span>.gpu.id_sphere)
    }

    <span class="k">pub</span> <span class="k">fn</span> draw_dot_ids(&amp;<span class="k">self</span>, pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;, b: &amp;Binds) -&gt; u32 {
        <span class="k">self</span>.draw_dot_table(pass, b, &amp;<span class="k">self</span>.gpu.id_dot)
    }

    <span class="c">/// Pick ids for dots that stand for cloud points.</span>
    <span class="k">pub</span> <span class="k">fn</span> draw_source_ids(&amp;<span class="k">self</span>, pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;, b: &amp;Binds) -&gt; u32 {
        <span class="k">self</span>.draw_dot_table(pass, b, &amp;<span class="k">self</span>.gpu.source_dot)
    }

    <span class="c">/// Draw every marker with \`pipeline\`; returns the draw count.</span>
    <span class="k">fn</span> draw_markers(
        &amp;<span class="k">self</span>,
        pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;,
        b: &amp;Binds,
        pipeline: &amp;wgpu::RenderPipeline,
    ) -&gt; u32 {
        <span class="k">if</span> <span class="k">self</span>.spheres.buf.is_empty() {
            <span class="k">return</span> <span class="s">0</span>;
        }

        pass.set_pipeline(pipeline);
        b.set(pass);
        pass.set_bind_group(<span class="s">3</span>, &amp;<span class="k">self</span>.spheres.group, &amp;[]);
        <span class="k">self</span>.template.bind(pass);
        <span class="c">// instanced: the 6-index quad once per marker row</span>
        pass.draw_indexed(<span class="s">0</span>..<span class="k">self</span>.template.index_count, <span class="s">0</span>, <span class="s">0</span>..<span class="k">self</span>.spheres.buf.len());
        <span class="s">1</span>
    }

    <span class="c">/// Draw every dot with \`pipeline\`; returns the draw count.</span>
    <span class="k">fn</span> draw_dot_table(
        &amp;<span class="k">self</span>,
        pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;,
        b: &amp;Binds,
        pipeline: &amp;wgpu::RenderPipeline,
    ) -&gt; u32 {
        <span class="k">if</span> <span class="k">self</span>.dots.buf.is_empty() {
            <span class="k">return</span> <span class="s">0</span>;
        }

        pass.set_pipeline(pipeline);
        b.set(pass);
        pass.set_bind_group(<span class="s">3</span>, &amp;<span class="k">self</span>.dots.group, &amp;[]);
        <span class="c">// no vertex buffer: vertex_index / 3 is the dot's row</span>
        pass.draw(<span class="s">0</span>..DOT_VERTS * <span class="k">self</span>.dots.buf.len(), <span class="s">0</span>..<span class="s">1</span>);
        <span class="s">1</span>
    }</code></pre></div>
<p><code>lessons/04c/src/engine/gpu/glyphs.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Forget every row; keep the buffers.</span>
    <span class="k">pub</span> <span class="k">fn</span> reset(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">self</span>.spheres.buf.reset();
        <span class="k">self</span>.dots.buf.reset();
    }

    <span class="c">/// Free both buffers.</span>
    <span class="k">pub</span> <span class="k">fn</span> release(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, l: &amp;Layouts) {
        <span class="k">self</span>.spheres.buf.release(ctx);
        <span class="k">self</span>.dots.buf.release(ctx);
        <span class="k">self</span>.spheres.rebind(ctx, l);
        <span class="k">self</span>.dots.rebind(ctx, l);
    }

    <span class="k">pub</span> <span class="k">fn</span> sphere_count(&amp;<span class="k">self</span>) -&gt; u32 {
        <span class="k">self</span>.spheres.buf.len()
    }

    <span class="k">pub</span> <span class="k">fn</span> dot_count(&amp;<span class="k">self</span>) -&gt; u32 {
        <span class="k">self</span>.dots.buf.len()
    }
}</code></pre></div>
<p><code>lessons/04c/src/engine/gpu/glyphs.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">fn</span> build_pipelines(ctx: &amp;GpuCtx, l: &amp;Layouts, s: &amp;GlyphShaders, target: Target) -&gt; GlyphPipelines {
    <span class="k">let</span> groups = [&amp;l.mvp, &amp;l.line, &amp;l.ink_instance, &amp;l.ink_rows];
    <span class="k">let</span> template = [template_layout()];
    <span class="c">// depth Always: the shaders test visibility themselves</span>
    <span class="k">let</span> marker = PipelineDesc::new(&amp;s.sphere, &amp;groups, &amp;template, TriangleList)
        .scene_samples(target.samples)
        .depth(DepthMode::Always);
    <span class="c">// dots: no vertex buffer</span>
    <span class="k">let</span> disc = PipelineDesc::new(&amp;s.dot, &amp;groups, &amp;[], TriangleList)
        .scene_samples(target.samples)
        .depth(DepthMode::Always);
    <span class="k">let</span> dev = &amp;ctx.device;

    GlyphPipelines {
        sphere: build(
            dev,
            target,
            &amp;marker.with(&quot;<span class="s">sphere</span>&quot;, &quot;<span class="s">fs_main</span>&quot;).color(ColorWrite::Blended),
        ),
        dot: build(
            dev,
            target,
            &amp;disc.with(&quot;<span class="s">glyph</span>&quot;, &quot;<span class="s">fs_main</span>&quot;).color(ColorWrite::Blended),
        ),
        id_sphere: build(
            dev,
            Target::ID,
            &amp;marker.with(&quot;<span class="s">sphere.id</span>&quot;, &quot;<span class="s">fs_id</span>&quot;).scene_samples(<span class="s">1</span>), <span class="c">// one sample: ids cannot be averaged</span>
        ),
        id_dot: build(
            dev,
            Target::ID,
            &amp;disc.with(&quot;<span class="s">glyph.id</span>&quot;, &quot;<span class="s">fs_id</span>&quot;).scene_samples(<span class="s">1</span>),
        ),
        source_dot: build(
            dev,
            Target::ID,
            &amp;disc
                .with(&quot;<span class="s">glyph.source</span>&quot;, &quot;<span class="s">fs_source_id</span>&quot;)
                .vertex(&quot;<span class="s">vs_source</span>&quot;)
                .scene_samples(<span class="s">1</span>)
                .depth(DepthMode::OpaqueEqual),
        ),
    }
}

<span class="c">/// A square from -1 to 1; the shader cuts it to a circle.</span>
<span class="k">fn</span> unit_quad() -&gt; (Vec&lt;[f32; 3]&gt;, Vec&lt;u32&gt;) {
    <span class="k">let</span> v = vec![
        [-<span class="s">1</span>.<span class="s">0</span>, -<span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>],
        [<span class="s">1</span>.<span class="s">0</span>, -<span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>],
        [<span class="s">1</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>],
        [-<span class="s">1</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>],
    ];
    <span class="k">let</span> idx = vec![<span class="s">0u32</span>, <span class="s">1</span>, <span class="s">2</span>, <span class="s">0</span>, <span class="s">2</span>, <span class="s">3</span>];
    (v, idx)
}</code></pre></div>
<p>Copy this part from the lesson folder to the path shown.</p>
<p><code>lessons/04c/src/engine/gpu/glyphs.rs</code> · copy the file, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>#[cfg(test)]
<span class="k">mod</span> tests {
    <span class="k">use</span> super::*;
    <span class="k">use</span> <span class="k">crate</span>::engine::gpu::instance::wgsl_fields;

    <span class="c">/// Both shaders declare the same GlyphPoint fields.</span>
    #[test]
    <span class="k">fn</span> glyph_point_mirror() {
        <span class="k">let</span> rust = [
            &quot;<span class="s">center</span>&quot;,
            &quot;<span class="s">radius</span>&quot;,
            &quot;<span class="s">color</span>&quot;,
            &quot;<span class="s">instance_id</span>&quot;,
            &quot;<span class="s">facing</span>&quot;,
            &quot;<span class="s">facing_ext</span>&quot;,
        ];

        <span class="k">for</span> (name, src) <span class="k">in</span> SHADERS {
            assert_eq!(
                wgsl_fields(src, &quot;<span class="s">GlyphPoint</span>&quot;),
                rust,
                &quot;{<span class="s">name</span>}<span class="s">: GlyphPoint fields</span>&quot;
            );
        }

        assert_eq!(std::mem::size_of::&lt;GlyphPoint&gt;(), <span class="s">48</span>);
        assert_eq!(std::mem::offset_of!(GlyphPoint, facing_ext), <span class="s">40</span>);
    }
}</code></pre></div>
<h2 id="step-2-srcshadersspherewgsl">Step 2 · src/shaders/sphere.wgsl<a class="anchor" href="#/course/04c-markers#step-2-srcshadersspherewgsl" aria-label="Link to this section">#</a></h2>
<p>New file: the sphere shader, a round marker with real depth.</p>
<p><code>lessons/04c/src/shaders/sphere.wgsl</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// One marker or dot, 48 bytes; matches GlyphPoint in Rust.</span>
<span class="k">struct</span> GlyphPoint {
    center: <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // world position</span>
    radius: <span class="k">f32</span>,<span class="c"> // 0 = pen width; &gt; 0 world mm; &lt; 0 screen px</span>
    color: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // rgba</span>
    instance_id: <span class="k">u32</span>,<span class="c"> // object row</span>
    facing: <span class="k">u32</span>,<span class="c"> // packed normals of the faces around it</span>
    facing_ext: <span class="k">vec2</span>&lt;<span class="k">u32</span>&gt;,<span class="c"> // more packed normals</span>
};

@group(<span class="s">3</span>) @binding(<span class="s">0</span>) <span class="k">var</span>&lt;<span class="k">storage</span>, <span class="k">read</span>&gt; glyphs: <span class="k">array</span>&lt;GlyphPoint&gt;;<span class="c"> // one row per marker</span>

<span class="c">// Markers shrink when vertices are closer than this many diameters.</span>
<span class="k">const</span> MARKER_MIN_DIAMS: <span class="k">f32</span> = <span class="s">3.0</span>;
<span class="c">// Markers never shrink below this factor.</span>
<span class="k">const</span> TAPER_MIN: <span class="k">f32</span> = <span class="s">0.15</span>;</code></pre></div>
<p><code>lessons/04c/src/shaders/sphere.wgsl</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// World radius that projects to the pen width at this depth.</span>
<span class="k">fn</span> pen_world_radius(clip_w: <span class="k">f32</span>) -&gt; <span class="k">f32</span> {
    <span class="k">if</span> (line.ortho_h &gt; <span class="s">0.0</span>) {
        <span class="k">return</span> line.thickness * line.ortho_h / line.vp_h;
    }

    <span class="k">return</span> line.thickness * clip_w / (line.proj_y * line.vp_h);
}

<span class="c">// A world length in px at depth \`w\`.</span>
<span class="k">fn</span> to_px(world: <span class="k">f32</span>, w: <span class="k">f32</span>) -&gt; <span class="k">f32</span> {
    <span class="k">if</span> (line.ortho_h &gt; <span class="s">0.0</span>) {
        <span class="k">return</span> world * line.vp_h * <span class="s">0.5</span> / line.ortho_h;
    }

    <span class="k">return</span> world * line.proj_y * line.vp_h * <span class="s">0.5</span> / max(w, 1e-<span class="s">6</span>);
}

<span class="c">// What the vertex shader hands the fragment shader.</span>
<span class="k">struct</span> VsOut {
    @builtin(position) pos: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // clip position</span>
    @location(<span class="s">0</span>) color: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // rgba</span>
    @location(<span class="s">1</span>) corner: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // -1..1 across the disc</span>
    @location(<span class="s">2</span>) @interpolate(flat) px: <span class="k">f32</span>,<span class="c"> // disc radius, px</span>
    @location(<span class="s">3</span>) @interpolate(flat) inst_id: <span class="k">u32</span>,<span class="c"> // object row</span>
    @location(<span class="s">4</span>) @interpolate(flat) centre: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // disc center, screen px</span>
    @location(<span class="s">5</span>) @interpolate(flat) depth: <span class="k">f32</span>,<span class="c"> // disc depth, 0..1</span>
};

<span class="c">// A vertex placed off screen, so nothing is drawn.</span>
<span class="k">fn</span> dead_dot() -&gt; VsOut {
    <span class="k">var</span> dead: VsOut;<span class="c"> // all zero; only the position matters</span>
    dead.pos = <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(<span class="s">3.0</span>, <span class="s">3.0</span>, <span class="s">0.5</span>, <span class="s">1.0</span>);
    <span class="k">return</span> dead;
}

<span class="c">// (has face normals, one of them faces the camera).</span>
<span class="k">fn</span> faces_front(g: GlyphPoint, model: <span class="k">mat4x4</span>&lt;<span class="k">f32</span>&gt;, to_eye: <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;) -&gt; <span class="k">vec2</span>&lt;<span class="k">bool</span>&gt; {
    <span class="k">let</span> fwords = <span class="k">array</span>&lt;<span class="k">u32</span>, <span class="s">3</span>&gt;(g.facing, g.facing_ext.x, g.facing_ext.y);
    <span class="k">var</span> known = <span class="s">false</span>;

    <span class="k">for</span> (<span class="k">var</span> w = <span class="s">0u</span>; w &lt; <span class="s">3u</span>; w = w + <span class="s">1u</span>) {
        <span class="k">let</span> fw = fwords[w];

        <span class="k">if</span> (fw == FACING_UNKNOWN) {
            <span class="k">continue</span>;
        }

        known = <span class="s">true</span>;

        <span class="k">for</span> (<span class="k">var</span> h = <span class="s">0u</span>; h &lt; <span class="s">2u</span>; h = h + <span class="s">1u</span>) {
            <span class="k">let</span> n = face_normal(model, oct16_decode((fw &gt;&gt; (<span class="s">16u</span> * h)) &amp; <span class="s">0xffffu</span>));

            <span class="k">if</span> (dot(n, to_eye) &gt; <span class="s">0.0</span>) {
                <span class="k">return</span> <span class="k">vec2</span>&lt;<span class="k">bool</span>&gt;(<span class="s">true</span>, <span class="s">true</span>);</code></pre></div>
<p><code>lessons/04c/src/shaders/sphere.wgsl</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            }
        }
    }

    <span class="k">return</span> <span class="k">vec2</span>&lt;<span class="k">bool</span>&gt;(known, <span class="s">false</span>);
}

@vertex
<span class="c">// Place one corner of a marker's quad.</span>
<span class="k">fn</span> vs_main(@location(<span class="s">0</span>) tmpl: <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;, @builtin(instance_index) gi: <span class="k">u32</span>) -&gt; VsOut {
    <span class="k">let</span> g = glyphs[gi];
    <span class="k">let</span> inst = instances[g.instance_id];

    <span class="k">if</span> ((inst.flags &amp; FLAG_HIDDEN) != <span class="s">0u</span>) {
        <span class="k">return</span> dead_dot();
    }

    <span class="k">let</span> centre = place(g.instance_id, g.center);
    <span class="k">let</span> clip = mvp * <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(centre, <span class="s">1.0</span>);

<span class="c">    // behind the camera</span>
    <span class="k">if</span> (clip.z - clip.w &gt; <span class="s">0.0</span>) {
        <span class="k">return</span> dead_dot();
    }

<span class="c">    // radius: the object's, or the pen</span>
    <span class="k">let</span> r = select(pen_world_radius(clip.w), g.radius, g.radius &gt; <span class="s">0.0</span>);
    <span class="k">var</span> px = to_px(r, clip.w);

<span class="c">    // shrink where vertices crowd</span>
    <span class="k">if</span> (inst.spacing &gt; <span class="s">0.0</span>) {
        <span class="k">let</span> sp_px = to_px(inst.spacing, clip.w);
        px = px * clamp(sp_px / max(MARKER_MIN_DIAMS * <span class="s">2.0</span> * px, 1e-<span class="s">6</span>), TAPER_MIN, <span class="s">1.0</span>);
    }

<span class="c">    // bigger than the screen: skip</span>
    <span class="k">if</span> (px &gt; max(line.frame.x, line.frame.y)) {
        <span class="k">return</span> dead_dot();
    }

    px = max(px, <span class="s">0.5</span>);

    <span class="k">let</span> off = tmpl.xy * (px + <span class="s">0.5</span> * line.feather) * <span class="s">2.0</span> / <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(line.vp_w, line.vp_h) * clip.w;

<span class="c">    // no markers on sampled surfaces</span>
    <span class="k">if</span> ((inst.flags &amp; FLAG_SMOOTH) != <span class="s">0u</span>) {
        <span class="k">return</span> dead_dot();
    }

<span class="c">    // back-facing vertices are skipped, unless inside, open or x-ray</span>
    <span class="k">let</span> inside = (inst.flags &amp; (FLAG_INSIDE | FLAG_OPEN)) != <span class="s">0u</span> || line.opacity &lt;= <span class="s">0.0</span>;

    <span class="k">if</span> (!inside) {
        <span class="k">let</span> kf = faces_front(g, inst.model, toward_eye(centre));

        <span class="k">if</span> (kf.x &amp;&amp; !kf.y) {
            <span class="k">return</span> dead_dot();
        }
    }

    <span class="k">var</span> o: VsOut;
    o.pos = <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(clip.xy + off, clip.z, clip.w);
    <span class="k">var</span> color = g.color * inst.color;

    <span class="k">if</span> ((inst.flags &amp; FLAG_SELECTED) != <span class="s">0u</span>) {
        color = <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(SELECT_COLOR, color.a);
    }

    o.color = color;
    o.corner = tmpl.xy;
    o.px = px;
    o.inst_id = g.instance_id;
<span class="c">    // clip to screen pixels</span>
    o.centre = <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;((clip.x / clip.w * <span class="s">0.5</span> + <span class="s">0.5</span>) * line.vp_w, (<span class="s">0.5</span> - clip.y / clip.w * <span class="s">0.5</span>) * line.vp_h);
    o.depth = clip.z / clip.w;
    <span class="k">return</span> o;
}

<span class="c">// Edge softness in px, never wider than the disc itself.</span>
<span class="k">fn</span> ramp(half_width: <span class="k">f32</span>) -&gt; <span class="k">f32</span> {
    <span class="k">return</span> min(line.feather, <span class="s">2.0</span> * half_width);
}

<span class="c">// How much of this pixel the disc covers, 0..1.</span>
<span class="k">fn</span> coverage(in: VsOut) -&gt; <span class="k">f32</span> {
    <span class="k">let</span> d = length(in.corner) * (in.px + <span class="s">0.5</span> * line.feather);
    <span class="k">let</span> f = ramp(in.px);
    <span class="k">return</span> clamp((in.px + <span class="s">0.5</span> * f - d) / f, <span class="s">0.0</span>, <span class="s">1.0</span>);
}

@fragment
<span class="c">// Color: the disc, faded where geometry hides it.</span>
<span class="k">fn</span> fs_main(in: VsOut, @builtin(sample_index) sample: <span class="k">u32</span>) -&gt; InkColor {
    <span class="k">let</span> alpha = coverage(in);

    <span class="k">if</span> (alpha &lt;= <span class="s">0.0</span> || !ink_disc_visible(in.pos.xy, in.centre, in.depth, sample)) {
        <span class="k">discard</span>;
    }

    <span class="k">return</span> InkColor(<span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(in.color.rgb, in.color.a * alpha));
}

@fragment
<span class="c">// Pick id: object row + 1 and a marker tag.</span>
<span class="k">fn</span> fs_id(in: VsOut) -&gt; @location(<span class="s">0</span>) <span class="k">vec2</span>&lt;<span class="k">u32</span>&gt; {
    <span class="k">if</span> (coverage(in) &lt; <span class="s">0.5</span> || !ink_disc_visible(in.pos.xy, in.centre, in.depth, <span class="s">0u</span>)) {
        <span class="k">discard</span>;
    }

    <span class="k">return</span> <span class="k">vec2</span>&lt;<span class="k">u32</span>&gt;(in.inst_id + <span class="s">1u</span>, DISC_ID_TAG);
}</code></pre></div>
<h2 id="step-3-srcshadersglyphwgsl">Step 3 · src/shaders/glyph.wgsl<a class="anchor" href="#/course/04c-markers#step-3-srcshadersglyphwgsl" aria-label="Link to this section">#</a></h2>
<p>New file: the dot shader: one triangle per dot, cut to a soft-edged disc.</p>
<p><code>lessons/04c/src/shaders/glyph.wgsl</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// One marker or dot, 48 bytes; matches GlyphPoint in Rust.</span>
<span class="k">struct</span> GlyphPoint {
    center: <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;,
    radius: <span class="k">f32</span>,
    color: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,
    instance_id: <span class="k">u32</span>,
    facing: <span class="k">u32</span>,
    facing_ext: <span class="k">vec2</span>&lt;<span class="k">u32</span>&gt;,
};

@group(<span class="s">3</span>) @binding(<span class="s">0</span>) <span class="k">var</span>&lt;<span class="k">storage</span>, <span class="k">read</span>&gt; glyphs: <span class="k">array</span>&lt;GlyphPoint&gt;;</code></pre></div>
<p><code>lessons/04c/src/shaders/glyph.wgsl</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// Corners at radius 2: the triangle's inscribed circle then has radius 1, just holding the disc.</span>
<span class="k">const</span> CORNERS = <span class="k">array</span>&lt;<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;, <span class="s">3</span>&gt;(
    <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>, <span class="s">2.0</span>),
    <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(-<span class="s">1.7320508</span>, -<span class="s">1.0</span>),
    <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">1.7320508</span>, -<span class="s">1.0</span>),
);

<span class="k">struct</span> VsOut {
    @builtin(position) pos: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,
    @location(<span class="s">0</span>) color: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,
    @location(<span class="s">1</span>) corner: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // -1..1 across the disc</span>
    @location(<span class="s">2</span>) @interpolate(linear) px: <span class="k">f32</span>,<span class="c"> // disc radius, px; linear = blended on screen, no perspective correction</span>
    @location(<span class="s">3</span>) @interpolate(linear) fade: <span class="k">f32</span>,<span class="c"> // alpha for discs thinner than a pixel</span>
    @location(<span class="s">4</span>) @interpolate(flat) inst_id: <span class="k">u32</span>,
    @location(<span class="s">5</span>) @interpolate(flat) centre: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // disc center, screen px</span>
    @location(<span class="s">6</span>) @interpolate(flat) depth: <span class="k">f32</span>,<span class="c"> // disc depth, 0..1</span>
    @location(<span class="s">7</span>) @interpolate(flat) point_index: <span class="k">u32</span>,<span class="c"> // row in the glyph table</span>
};

<span class="c">// A vertex placed off screen, so nothing is drawn.</span>
<span class="k">fn</span> dead_dot() -&gt; VsOut {
    <span class="k">var</span> dead: VsOut;<span class="c"> // all zero; only the position matters</span>
    dead.pos = <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(<span class="s">3.0</span>, <span class="s">3.0</span>, <span class="s">0.5</span>, <span class="s">1.0</span>);
    <span class="k">return</span> dead;
}

<span class="c">// Place one corner of a dot's triangle.</span>
<span class="k">fn</span> glyph_vertex(vid: <span class="k">u32</span>) -&gt; VsOut {
    <span class="k">let</span> g = glyphs[vid / <span class="s">3u</span>];<span class="c"> // 3 vertices per dot: vertex 7 is corner 1 of dot 2</span>
    <span class="k">let</span> inst = instances[g.instance_id];

    <span class="k">if</span> ((inst.flags &amp; FLAG_HIDDEN) != <span class="s">0u</span>) {
        <span class="k">return</span> dead_dot();
    }

    <span class="k">let</span> world = place(g.instance_id, g.center);</code></pre></div>
<p><code>lessons/04c/src/shaders/glyph.wgsl</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">let</span> clip = mvp * <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(world, <span class="s">1.0</span>);

<span class="c">    // behind the camera</span>
    <span class="k">if</span> (clip.z - clip.w &gt; <span class="s">0.0</span>) {
        <span class="k">return</span> dead_dot();
    }

<span class="c">    // radius in px: pen width, world mm projected, or fixed px</span>
    <span class="k">var</span> px = line.thickness * <span class="s">0.5</span>;

    <span class="k">if</span> (g.radius &lt; <span class="s">0.0</span>) {
        px = -g.radius;
    } <span class="k">else</span> <span class="k">if</span> (g.radius &gt; <span class="s">0.0</span>) {
        <span class="k">if</span> (line.ortho_h &gt; <span class="s">0.0</span>) {
            px = g.radius * line.vp_h * <span class="s">0.5</span> / line.ortho_h;
        } <span class="k">else</span> {
            px = g.radius * line.proj_y * line.vp_h * <span class="s">0.5</span> / max(clip.w, 1e-<span class="s">6</span>);
        }
    }

<span class="c">    // bigger than the screen: skip</span>
    <span class="k">if</span> (px &gt; max(line.frame.x, line.frame.y)) {
        <span class="k">return</span> dead_dot();
    }

    <span class="k">var</span> fade = <span class="s">1.0</span>;

<span class="c">    // thinner than a pixel: keep half a pixel, fade instead</span>
    <span class="k">if</span> (px &lt; <span class="s">0.5</span>) {
        fade = max(px / <span class="s">0.5</span>, HAIRLINE_MIN_ALPHA);
        px = <span class="s">0.5</span>;
    }

    <span class="k">let</span> corner = CORNERS[vid % <span class="s">3u</span>];
<span class="c">    // px to clip units: 2 / viewport size, times w to undo the perspective divide</span>
    <span class="k">let</span> off = corner * (px + <span class="s">0.5</span> * line.feather) * <span class="s">2.0</span> / <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(line.vp_w, line.vp_h) * clip.w;

    <span class="k">var</span> o: VsOut;
    o.pos = <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(clip.xy + off, clip.z, clip.w);
    <span class="k">var</span> color = g.color * inst.color;

    <span class="k">if</span> ((inst.flags &amp; FLAG_SELECTED) != <span class="s">0u</span>) {
        color = <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(SELECT_COLOR, color.a);
    }

    o.color = color;
    o.corner = corner;
    o.px = px;
    o.fade = fade;
    o.inst_id = g.instance_id;
<span class="c">    // clip to screen pixels</span>
    o.centre = <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;((clip.x / clip.w * <span class="s">0.5</span> + <span class="s">0.5</span>) * line.vp_w, (<span class="s">0.5</span> - clip.y / clip.w * <span class="s">0.5</span>) * line.vp_h);
    o.depth = clip.z / clip.w;
    o.point_index = vid / <span class="s">3u</span>;
    <span class="k">return</span> o;
}

<span class="c">// Edge softness in px, never wider than the disc itself.</span>
<span class="k">fn</span> ramp(half_width: <span class="k">f32</span>) -&gt; <span class="k">f32</span> {
    <span class="k">return</span> min(line.feather, <span class="s">2.0</span> * half_width);
}

<span class="c">// How much of this pixel the disc covers, 0..1.</span>
<span class="k">fn</span> coverage(in: VsOut) -&gt; <span class="k">f32</span> {
    <span class="k">let</span> d = length(in.corner) * (in.px + <span class="s">0.5</span> * line.feather);
    <span class="k">let</span> f = ramp(in.px);
    <span class="k">return</span> clamp((in.px + <span class="s">0.5</span> * f - d) / f, <span class="s">0.0</span>, <span class="s">1.0</span>) * in.fade;
}

@fragment
<span class="c">// Color: the disc, faded where geometry hides it; sample_index makes it run once per MSAA sample.</span>
<span class="k">fn</span> fs_main(in: VsOut, @builtin(sample_index) sample: <span class="k">u32</span>) -&gt; InkColor {
    <span class="k">let</span> alpha = coverage(in);

    <span class="k">if</span> (alpha &lt;= <span class="s">0.0</span> || !ink_disc_visible(in.pos.xy, in.centre, in.depth, sample)) {
        <span class="k">discard</span>;<span class="c"> // this pixel writes nothing</span>
    }

    <span class="k">return</span> InkColor(<span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(in.color.rgb, in.color.a * alpha));
}

@fragment
<span class="c">// Pick id: object row + 1 and a marker tag.</span>
<span class="k">fn</span> fs_id(in: VsOut) -&gt; @location(<span class="s">0</span>) <span class="k">vec2</span>&lt;<span class="k">u32</span>&gt; {
    <span class="k">if</span> (coverage(in) &lt; <span class="s">0.5</span> || !ink_disc_visible(in.pos.xy, in.centre, in.depth, <span class="s">0u</span>)) {
        <span class="k">discard</span>;
    }

    <span class="k">return</span> <span class="k">vec2</span>&lt;<span class="k">u32</span>&gt;(in.inst_id + <span class="s">1u</span>, DISC_ID_TAG | (in.point_index + <span class="s">1u</span>));
}

<span class="c">// Ordinary dots.</span>
@vertex
<span class="k">fn</span> vs_main(@builtin(vertex_index) vid: <span class="k">u32</span>) -&gt; VsOut {
    <span class="k">return</span> glyph_vertex(vid);
}

@vertex
<span class="c">// Dots standing for cloud points; facing_ext.x holds the point row.</span>
<span class="k">fn</span> vs_source(@builtin(vertex_index) vid: <span class="k">u32</span>) -&gt; VsOut {
    <span class="k">var</span> out = glyph_vertex(vid);
    out.point_index = glyphs[vid / <span class="s">3u</span>].facing_ext.x;
    <span class="k">return</span> out;
}

@fragment
<span class="c">// Pick id: object row + 1 and point row + 1.</span>
<span class="k">fn</span> fs_source_id(in: VsOut) -&gt; @location(<span class="s">0</span>) <span class="k">vec2</span>&lt;<span class="k">u32</span>&gt; {
    <span class="k">if</span> (coverage(in) &lt; <span class="s">0.5</span>) {
        <span class="k">discard</span>;
    }

    <span class="k">return</span> <span class="k">vec2</span>&lt;<span class="k">u32</span>&gt;(in.inst_id + <span class="s">1u</span>, in.point_index + <span class="s">1u</span>);
}</code></pre></div>
<p>Run <code>cargo check</code> in <code>lessons/04c/</code>.</p>
<h2 id="step-4-srcenginepipelinesmodrs">Step 4 · src/engine/pipelines/mod.rs<a class="anchor" href="#/course/04c-markers#step-4-srcenginepipelinesmodrs" aria-label="Link to this section">#</a></h2>
<p>Add the marker pipelines.</p>
<p><code>lessons/04c/src/engine/pipelines/mod.rs</code> · edit · type this</p>
<p>Added above</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// The mesh vertex slot: the kernel's interleaved \`RenderVertex\` (pos, normal, colour).</span></code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// One vec3 at location 0: a template position.</span>
<span class="k">const</span> TEMPLATE_ATTRIBS: [wgpu::VertexAttribute; <span class="s">1</span>] = [wgpu::VertexAttribute {
    offset: <span class="s">0</span>,
    shader_location: <span class="s">0</span>,
    format: wgpu::VertexFormat::Float32x3,
}];</code></pre></div>
<p>Added above</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Compile one WGSL source into a module; the caller keeps it and shares it across pipelines.</span></code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Vertex slot 0 for the marker quad: positions only.</span>
<span class="k">pub</span> <span class="k">fn</span> template_layout() -&gt; wgpu::VertexBufferLayout&lt;'static&gt; {
    wgpu::VertexBufferLayout {
        array_stride: <span class="s">12</span>,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &amp;TEMPLATE_ATTRIBS,
    }
}</code></pre></div>
<h2 id="step-5-srcenginepipelineslayoutsrs">Step 5 · src/engine/pipelines/layouts.rs<a class="anchor" href="#/course/04c-markers#step-5-srcenginepipelineslayoutsrs" aria-label="Link to this section">#</a></h2>
<p>Add the marker bind group layout.</p>
<p><code>lessons/04c/src/engine/pipelines/layouts.rs</code> · edit · type this</p>
<p>Added above</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Segment rows retain source-edge IDs and one shared selection uniform.</span></code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Group 3 for markers and dots: one row table.</span>
<span class="k">fn</span> ink_rows_layout(device: &amp;wgpu::Device) -&gt; wgpu::BindGroupLayout {
    device.create_bind_group_layout(&amp;wgpu::BindGroupLayoutDescriptor {
        label: Some(&quot;<span class="s">ink.rows.layout</span>&quot;),
        entries: &amp;[buffer_entry(
            <span class="s">0</span>,
            wgpu::ShaderStages::VERTEX_FRAGMENT,
            wgpu::BufferBindingType::Storage { read_only: <span class="s">true</span> },
        )],
    })
}</code></pre></div>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> ink_instance: wgpu::BindGroupLayout,</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> ink_rows: wgpu::BindGroupLayout, <span class="c">// group 3 for markers and dots</span></code></pre></div>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            ink_instance: ink_instance_layout(device),</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            ink_rows: ink_rows_layout(device),</code></pre></div>
<h2 id="step-6-srcenginegpuuploadrs">Step 6 · src/engine/gpu/upload.rs<a class="anchor" href="#/course/04c-markers#step-6-srcenginegpuuploadrs" aria-label="Link to this section">#</a></h2>
<p>Add markers to the upload.</p>
<p><code>lessons/04c/src/engine/gpu/upload.rs</code> · edit · type this</p>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> super::arena::ArenaRows;</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> super::glyphs::GlyphRows;</code></pre></div>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> seg: SegRows,</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> glyph: GlyphRows, <span class="c">// markers and dots</span></code></pre></div>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            seg: SegRows::default(),</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            glyph: GlyphRows::default(),</code></pre></div>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.seg.drop_rows();</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.glyph.drop_rows();</code></pre></div>
<h2 id="step-7-srcenginegpumodrs">Step 7 · src/engine/gpu/mod.rs<a class="anchor" href="#/course/04c-markers#step-7-srcenginegpumodrs" aria-label="Link to this section">#</a></h2>
<p>Add the marker lane to the GPU owner and the frame.</p>
<p><code>lessons/04c/src/engine/gpu/mod.rs</code> · edit · type this</p>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> <span class="k">mod</span> frame;</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> <span class="k">mod</span> glyphs;</code></pre></div>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> <span class="k">use</span> frame::FrameInput;</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> <span class="k">use</span> glyphs::GlyphPoint;</code></pre></div>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> segments: segments::SegmentLane,</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> glyphs: glyphs::GlyphLane,</code></pre></div>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> segments = segments::SegmentLane::new(&amp;ctx, &amp;layouts, target);</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> glyphs = glyphs::GlyphLane::new(&amp;ctx, &amp;layouts, target);</code></pre></div>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            segments,</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            glyphs,</code></pre></div>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.segments.append(&amp;<span class="k">self</span>.ctx, &amp;<span class="k">self</span>.layouts, &amp;up.seg);</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.glyphs.append(&amp;<span class="k">self</span>.ctx, &amp;<span class="k">self</span>.layouts, &amp;up.glyph);</code></pre></div>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">self</span>.segments.draw_ribbons(&amp;<span class="k">mut</span> pass, &amp;ink);</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">self</span>.glyphs.draw_spheres(&amp;<span class="k">mut</span> pass, &amp;ink);
            <span class="k">self</span>.glyphs.draw_dots(&amp;<span class="k">mut</span> pass, &amp;ink);</code></pre></div>
<h2 id="step-8-srcfixturers">Step 8 · src/fixture.rs<a class="anchor" href="#/course/04c-markers#step-8-srcfixturers" aria-label="Link to this section">#</a></h2>
<p>Copy the test scene: it now has markers.
Copy this file from the lesson folder to the path shown.</p>
<p><code>lessons/04c/src/fixture.rs</code> · edit · copy the file</p>
<p>Replaces the line <code>use crate::engine::gpu::{CylinderSegment, Instance, Objec…</code> in <code>lessons/04b/src/fixture.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> <span class="k">crate</span>::engine::gpu::{CylinderSegment, GlyphPoint, Instance, ObjectRow, Upload};</code></pre></div>
<p>Replaces the line <code>for _ in 0..2 {</code> in <code>lessons/04b/src/fixture.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">for</span> _ <span class="k">in</span> <span class="s">0</span>..<span class="s">3</span> {</code></pre></div>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        });
    }</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    upload.glyph.dots.push(GlyphPoint {
        center: [-<span class="s">1</span>.<span class="s">0</span>, -<span class="s">0</span>.<span class="s">7</span>, <span class="s">0</span>.<span class="s">0</span>],
        radius: <span class="s">0</span>.<span class="s">09</span>,
        color: [<span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">4</span>, <span class="s">0</span>.<span class="s">2</span>, <span class="s">1</span>.<span class="s">0</span>],
        instance_id: <span class="s">2</span>,
        facing: <span class="s">0xffffffff</span>,
        facing_ext: [<span class="s">0xffffffff</span>; <span class="s">2</span>],
    });</code></pre></div>
<h2 id="step-9-srclibrs">Step 9 · src/lib.rs<a class="anchor" href="#/course/04c-markers#step-9-srclibrs" aria-label="Link to this section">#</a></h2>
<p>Report the dot count from the entry point.</p>
<p><code>lessons/04c/src/lib.rs</code> · edit · type this</p>
<p>Replaces the line <code>&quot;meshVertices&quot;:self.gpu.arena.vert_count(),&quot;segments&quot;:sel…</code> in <code>lessons/04b/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            &quot;<span class="s">meshVertices</span>&quot;:<span class="k">self</span>.gpu.arena.vert_count(),&quot;<span class="s">segments</span>&quot;:<span class="k">self</span>.gpu.segments.ribbon_count(),&quot;<span class="s">dots</span>&quot;:<span class="k">self</span>.gpu.glyphs.dot_count()}).to_string())</code></pre></div>
<h2 id="step-10-indexhtml">Step 10 · index.html<a class="anchor" href="#/course/04c-markers#step-10-indexhtml" aria-label="Link to this section">#</a></h2>
<p>Copy the page: the status now reports dots.
Copy this file from the lesson folder to the path shown.</p>
<p><code>lessons/04c/index.html</code> · edit · copy the file</p>
<p>Replaces the line <code>&lt;title&gt;Session checkpoint 04b&lt;/title&gt;</code> in <code>lessons/04b/index.html</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    &lt;title&gt;Session checkpoint 04c&lt;/title&gt;</code></pre></div>
<p>Replaces the line <code>&lt;output id=&quot;status&quot;&gt;Starting checkpoint 04b&lt;/output&gt;</code> in <code>lessons/04b/index.html</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    &lt;output id=&quot;<span class="s">status</span>&quot;&gt;Starting checkpoint 04c&lt;/output&gt;</code></pre></div>
<p>Replaces the line <code>document.getElementById(&#39;status&#39;).textContent = &#39;Checkpoi…</code> in <code>lessons/04b/index.html</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        document.getElementById('status').textContent = 'Checkpoint 04c · ' + state.objects + ' objects · ' + state.width + '×' + state.height;</code></pre></div>
<h2 id="check">Check<a class="anchor" href="#/course/04c-markers#check" aria-label="Link to this section">#</a></h2>
<p>Run <code>trunk serve</code> in <code>lessons/04c/</code> and open <a href="http://127.0.0.1:8770/" target="_blank" rel="noopener">http://127.0.0.1:8770/</a>.</p>
<p>Expected: An orange point appears below the triangle and fades smoothly at small sizes; status: <strong>Checkpoint 04c · 3 objects</strong>.</p>
<p><img src="/session/docs/course/docs/screenshots/04c.png" alt="Checkpoint 04c: vertex markers and free dots drawn from the glyph lane." loading="lazy" decoding="async"></p>
<p>If it fails:</p>
<ul>
<li>Dots vanish while zooming out: the half-pixel floor or alpha fade is missing.</li>
<li>A sphere looks flat: the fragment depth still describes its billboard.</li>
</ul>
<h2 id="what-changed">What changed<a class="anchor" href="#/course/04c-markers#what-changed" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>lessons/04c/src/engine/
├── gpu/
│   ├── arena.rs
│   ├── buffers.rs
│   ├── frame.rs
│   ├── glyphs.rs  +
│   ├── instance.rs
│   ├── mod.rs  ~
│   ├── objects.rs
│   ├── segments.rs
│   ├── targets.rs
│   ├── text_outline.rs
│   ├── upload.rs  ~
│   └── view.rs
├── pipelines/
│   ├── layouts.rs  ~
│   └── mod.rs  ~
└── mod.rs</code></pre></div>
<p><code>+</code> new in this lesson · <code>~</code> changed in this lesson</p>
<p>Data flow: source files → retained scene state → GPU buffers → visible result. Every file at this point: <code>lessons/04c/</code>.</p>
<h2 id="next">Next<a class="anchor" href="#/course/04c-markers#next" aria-label="Link to this section">#</a></h2>
<p><a href="#/course/04d-clouds">04d · Point clouds</a></p>
<h2 id="expected-viewer-result">Expected viewer result<a class="anchor" href="#/course/04c-markers#expected-viewer-result" aria-label="Link to this section">#</a></h2>
<p>Checkpoint 04c: vertex markers and free dots drawn from the glyph lane.</p>
<p><a href="/session/docs/course/docs/screenshots/04c.png"><img src="/session/docs/course/docs/screenshots/04c.png" alt="Full viewer result for 04c markers" loading="lazy" decoding="async"></a></p>
`,toc:[{level:2,id:"step-1-srcenginegpuglyphsrs",text:"Step 1 · src/engine/gpu/glyphs.rs"},{level:2,id:"step-2-srcshadersspherewgsl",text:"Step 2 · src/shaders/sphere.wgsl"},{level:2,id:"step-3-srcshadersglyphwgsl",text:"Step 3 · src/shaders/glyph.wgsl"},{level:2,id:"step-4-srcenginepipelinesmodrs",text:"Step 4 · src/engine/pipelines/mod.rs"},{level:2,id:"step-5-srcenginepipelineslayoutsrs",text:"Step 5 · src/engine/pipelines/layouts.rs"},{level:2,id:"step-6-srcenginegpuuploadrs",text:"Step 6 · src/engine/gpu/upload.rs"},{level:2,id:"step-7-srcenginegpumodrs",text:"Step 7 · src/engine/gpu/mod.rs"},{level:2,id:"step-8-srcfixturers",text:"Step 8 · src/fixture.rs"},{level:2,id:"step-9-srclibrs",text:"Step 9 · src/lib.rs"},{level:2,id:"step-10-indexhtml",text:"Step 10 · index.html"},{level:2,id:"check",text:"Check"},{level:2,id:"what-changed",text:"What changed"},{level:2,id:"next",text:"Next"},{level:2,id:"expected-viewer-result",text:"Expected viewer result"}]};export{s as default};
