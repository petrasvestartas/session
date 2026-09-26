const s={title:"04b · Strokes",html:`<h1 id="04b-strokes">04b · Strokes<a class="anchor" href="#/course/04b-strokes#04b-strokes" aria-label="Link to this section">#</a></h1>
<p>A yellow polyline appears beside the blue triangle and keeps its width while zooming.</p>
<p><img src="/session/docs/course/docs/illustrations/ribbon.svg" alt="Six vertices place a camera-facing quad around the projected axis, and band_area integrates one pixel box against the capsule so coverage is an area rather than a distance ramp." loading="lazy" decoding="async"></p>
<h2 id="step-1-srcenginegpusegmentsrs">Step 1 · src/engine/gpu/segments.rs<a class="anchor" href="#/course/04b-strokes#step-1-srcenginegpusegmentsrs" aria-label="Link to this section">#</a></h2>
<p>New file: stroke segments and the object rows they belong to.</p>
<p><code>lessons/04b/src/engine/gpu/segments.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> super::buffers::{GpuCtx, GrowBuf, ROWS, bind_group, uniform_buffer};
<span class="k">use</span> super::frame::Binds;
<span class="k">use</span> super::upload::drop_rows;
<span class="k">use</span> <span class="k">crate</span>::engine::pipelines::{
    ColorWrite, DepthMode, Layouts, PipelineDesc, Target, build, ink_module,
};
<span class="k">use</span> wgpu::PrimitiveTopology::TriangleList;

<span class="c">/// Shader sources the tests compare against the files.</span>
#[cfg(test)]
<span class="k">pub</span> <span class="k">const</span> SHADERS: &amp;[(&amp;str, &amp;str)] = &amp;[(&quot;<span class="s">ribbon.wgsl</span>&quot;, include_str!(&quot;<span class="s">../../shaders/ribbon.wgsl</span>&quot;))];

<span class="c">/// GPUs only draw triangles, so a line becomes a thin rectangle: two triangles the shader builds from the two endpoints.</span>
<span class="k">const</span> RIBBON_VERTS: u32 = <span class="s">6</span>;

<span class="c">/// One line segment, 40 bytes, as the shaders read it.</span>
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
<span class="k">pub</span> <span class="k">struct</span> CylinderSegment {
    <span class="k">pub</span> p0: [f32; <span class="s">3</span>], <span class="c">// start point</span>
    <span class="k">pub</span> radius: f32, <span class="c">// 0 = pen width; &gt; 0 = world mm</span>
    <span class="k">pub</span> p1: [f32; <span class="s">3</span>], <span class="c">// end point</span>
    <span class="k">pub</span> instance_id: u32, <span class="c">// object row</span>
    <span class="k">pub</span> color: u32, <span class="c">// packed rgba, red in the low byte</span>
    <span class="k">pub</span> facing: u32, <span class="c">// packed normals of the two faces beside it</span>
}

<span class="k">const</span> _: () = assert!(std::mem::size_of::&lt;CylinderSegment&gt;() == <span class="s">40</span>);

<span class="c">/// Segment rows of one upload.</span>
#[derive(Default)]
<span class="k">pub</span> <span class="k">struct</span> SegRows {
    <span class="k">pub</span> pipes: Vec&lt;CylinderSegment&gt;, <span class="c">// mesh and solid edges</span>
    <span class="k">pub</span> pipe_ids: Vec&lt;u32&gt;, <span class="c">// source edge per pipe, or u32::MAX</span>
    <span class="k">pub</span> ribbons: Vec&lt;CylinderSegment&gt;, <span class="c">// standalone lines and curves</span>
}

<span class="k">impl</span> SegRows {
    <span class="c">/// Empty every table and free its memory.</span>
    <span class="k">pub</span> <span class="k">fn</span> drop_rows(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        drop_rows(&amp;<span class="k">mut</span> <span class="k">self</span>.pipes);
        drop_rows(&amp;<span class="k">mut</span> <span class="k">self</span>.pipe_ids);
        drop_rows(&amp;<span class="k">mut</span> <span class="k">self</span>.ribbons);
    }
}</code></pre></div>
<p><code>lessons/04b/src/engine/gpu/segments.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// One segment buffer, its ids and their bind group.</span>
<span class="k">struct</span> SegTable {
    label: &amp;'static str, <span class="c">// name shown in GPU errors</span>
    buf: GrowBuf, <span class="c">// StrokeSegment rows</span>
    ids: GrowBuf, <span class="c">// source id per row</span>
    group: wgpu::BindGroup, <span class="c">// group 3: rows, ids, selected edge</span>
}

<span class="k">impl</span> SegTable {
    <span class="c">/// An empty table and its bind group.</span>
    <span class="k">fn</span> new(ctx: &amp;GpuCtx, l: &amp;Layouts, label: &amp;'static str, selection: &amp;wgpu::Buffer) -&gt; <span class="k">Self</span> {
        <span class="k">let</span> buf = GrowBuf::new(
            ctx,
            label,
            std::mem::size_of::&lt;CylinderSegment&gt;() <span class="k">as</span> u64,
            ROWS,
        );
        <span class="k">let</span> ids = GrowBuf::new(ctx, &quot;<span class="s">segment.sources</span>&quot;, <span class="s">4</span>, ROWS);
        <span class="k">let</span> group = bind_group(
            ctx,
            &amp;l.segment_rows,
            label,
            &amp;[&amp;buf.buf, &amp;ids.buf, selection],
        );
        <span class="k">Self</span> {
            label,
            buf,
            ids,
            group,
        }
    }

    <span class="c">/// Rebuild the bind group after a buffer moved.</span>
    <span class="k">fn</span> rebind(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, l: &amp;Layouts, selection: &amp;wgpu::Buffer) {
        <span class="k">self</span>.group = bind_group(
            ctx,
            &amp;l.segment_rows,
            <span class="k">self</span>.label,
            &amp;[&amp;<span class="k">self</span>.buf.buf, &amp;<span class="k">self</span>.ids.buf, selection],
        );
    }
}

<span class="c">/// The nine segment pipelines.</span>
<span class="k">struct</span> SegPipelines {
    ribbon: wgpu::RenderPipeline, <span class="c">// plain lines in color</span>
    id_ribbon: wgpu::RenderPipeline, <span class="c">// object ids</span>
    id_edge: wgpu::RenderPipeline, <span class="c">// source edge ids</span>
}

<span class="c">/// Lines on the GPU: edges as pipes, curves as ribbons.</span>
<span class="k">pub</span> <span class="k">struct</span> SegmentLane {
    pipes: SegTable, <span class="c">// mesh and solid edges</span>
    ribbons: SegTable, <span class="c">// standalone lines and curves</span>
    shader: wgpu::ShaderModule, <span class="c">// ribbon shader</span>
    gpu: SegPipelines, <span class="c">// pipelines</span>
    selection: wgpu::Buffer, <span class="c">// selected edge (object, edge), read by shaders</span>
}</code></pre></div>
<p><code>lessons/04b/src/engine/gpu/segments.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">impl</span> SegmentLane {
    <span class="c">/// Bytes reserved on the GPU by this lane.</span>
    <span class="k">pub</span> <span class="k">fn</span> allocated_bytes(&amp;<span class="k">self</span>) -&gt; u64 {
        <span class="k">self</span>.pipes.buf.buf.size()
            + <span class="k">self</span>.pipes.ids.buf.size()
            + <span class="k">self</span>.ribbons.buf.buf.size()
            + <span class="k">self</span>.ribbons.ids.buf.size()
            + <span class="k">self</span>.selection.size()
    }

    <span class="c">/// Create the lane: shader, pipelines, empty tables.</span>
    <span class="k">pub</span> <span class="k">fn</span> new(ctx: &amp;GpuCtx, l: &amp;Layouts, target: Target) -&gt; <span class="k">Self</span> {
        <span class="k">let</span> shader = ink_module(
            &amp;ctx.device,
            &quot;<span class="s">ribbon.shader</span>&quot;,
            include_str!(&quot;<span class="s">../../shaders/ribbon.wgsl</span>&quot;),
        );
        <span class="k">let</span> gpu = build_pipelines(ctx, l, &amp;shader, target);
        <span class="k">let</span> selection = uniform_buffer(&amp;ctx.device, &quot;<span class="s">edge.selection</span>&quot;, &amp;[u32::MAX; <span class="s">4</span>]);
        <span class="k">let</span> pipes = SegTable::new(ctx, l, &quot;<span class="s">pipes</span>&quot;, &amp;selection);
        <span class="k">let</span> ribbons = SegTable::new(ctx, l, &quot;<span class="s">ribbons</span>&quot;, &amp;selection);
        <span class="k">Self</span> {
            pipes,
            ribbons,
            shader,
            gpu,
            selection,
        }
    }

    <span class="c">/// Rebuild the pipelines for a new MSAA sample count.</span>
    <span class="k">pub</span> <span class="k">fn</span> retarget(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, l: &amp;Layouts, target: Target) {
        <span class="k">self</span>.gpu = build_pipelines(ctx, l, &amp;<span class="k">self</span>.shader, target);
    }

    <span class="c">/// Append one upload's rows to both tables.</span>
    <span class="k">pub</span> <span class="k">fn</span> append(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, l: &amp;Layouts, up: &amp;SegRows) {
        <span class="c">// pad missing ids with u32::MAX</span>
        <span class="k">let</span> <span class="k">mut</span> ids = up.pipe_ids.clone();
        ids.resize(up.pipes.len(), u32::MAX);
        <span class="k">let</span> pipes_changed = <span class="k">self</span>.pipes.buf.append(ctx, &amp;up.pipes);

        <span class="k">if</span> <span class="k">self</span>.pipes.ids.append(ctx, &amp;ids) || pipes_changed {
            <span class="k">self</span>.pipes.rebind(ctx, l, &amp;<span class="k">self</span>.selection);
        }

        <span class="k">let</span> ribbons_changed = <span class="k">self</span>.ribbons.buf.append(ctx, &amp;up.ribbons);

        <span class="k">if</span> <span class="k">self</span>
            .ribbons
            .ids
            .append(ctx, &amp;vec![u32::MAX; up.ribbons.len()])
            || ribbons_changed
        {
            <span class="k">self</span>.ribbons.rebind(ctx, l, &amp;<span class="k">self</span>.selection);
        }
    }

    <span class="c">/// Highlight one edge; no re-upload.</span>
    <span class="k">pub</span> <span class="k">fn</span> set_edge(&amp;<span class="k">self</span>, ctx: &amp;GpuCtx, edge: Option&lt;(u32, u32)&gt;) {
        <span class="k">let</span> (parent, edge) = edge.unwrap_or((u32::MAX, u32::MAX));
        ctx.queue.write_buffer(
            &amp;<span class="k">self</span>.selection,
            <span class="s">0</span>,
            bytemuck::cast_slice(&amp;[parent, edge, <span class="s">0</span>, <span class="s">0</span>]),
        );
    }</code></pre></div>
<p><code>lessons/04b/src/engine/gpu/segments.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Draw the edges in color.</span>
    <span class="k">pub</span> <span class="k">fn</span> draw_pipes(&amp;<span class="k">self</span>, pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;, b: &amp;Binds) -&gt; u32 {
        <span class="k">self</span>.draw_table(pass, b, &amp;<span class="k">self</span>.gpu.ribbon, &amp;<span class="k">self</span>.pipes)
    }

    <span class="c">/// Draw the lines and curves in color.</span>
    <span class="k">pub</span> <span class="k">fn</span> draw_ribbons(&amp;<span class="k">self</span>, pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;, b: &amp;Binds) -&gt; u32 {
        <span class="k">self</span>.draw_table(pass, b, &amp;<span class="k">self</span>.gpu.ribbon, &amp;<span class="k">self</span>.ribbons)
    }

    <span class="c">/// Draw edge object ids.</span>
    <span class="k">pub</span> <span class="k">fn</span> draw_pipe_ids(&amp;<span class="k">self</span>, pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;, b: &amp;Binds) -&gt; u32 {
        <span class="k">self</span>.draw_table(pass, b, &amp;<span class="k">self</span>.gpu.id_ribbon, &amp;<span class="k">self</span>.pipes)
    }

    <span class="c">/// Draw source edge ids.</span>
    <span class="k">pub</span> <span class="k">fn</span> draw_edge_ids(&amp;<span class="k">self</span>, pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;, b: &amp;Binds) -&gt; u32 {
        <span class="k">self</span>.draw_table(pass, b, &amp;<span class="k">self</span>.gpu.id_edge, &amp;<span class="k">self</span>.pipes)
    }

    <span class="c">/// Draw line object ids.</span>
    <span class="k">pub</span> <span class="k">fn</span> draw_ribbon_ids(&amp;<span class="k">self</span>, pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;, b: &amp;Binds) -&gt; u32 {
        <span class="k">self</span>.draw_table(pass, b, &amp;<span class="k">self</span>.gpu.id_ribbon, &amp;<span class="k">self</span>.ribbons)
    }

    <span class="c">/// Draw one table with \`pipeline\`; returns the draw count.</span>
    <span class="k">fn</span> draw_table(
        &amp;<span class="k">self</span>,
        pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;,
        b: &amp;Binds,
        pipeline: &amp;wgpu::RenderPipeline,
        table: &amp;SegTable,
    ) -&gt; u32 {
        <span class="k">if</span> table.buf.is_empty() {
            <span class="k">return</span> <span class="s">0</span>;
        }

        pass.set_pipeline(pipeline);
        b.set(pass);
        pass.set_bind_group(<span class="s">3</span>, &amp;table.group, &amp;[]);
        <span class="c">// six vertices per segment, placed by the shader</span>
        pass.draw(<span class="s">0</span>..RIBBON_VERTS * table.buf.len(), <span class="s">0</span>..<span class="s">1</span>);
        <span class="s">1</span>
    }

    <span class="c">/// Forget every row; keep the buffers.</span>
    <span class="k">pub</span> <span class="k">fn</span> reset(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">self</span>.pipes.buf.reset();
        <span class="k">self</span>.pipes.ids.reset();
        <span class="k">self</span>.ribbons.buf.reset();
        <span class="k">self</span>.ribbons.ids.reset();
    }

    <span class="c">/// Forget every row and free the buffers.</span>
    <span class="k">pub</span> <span class="k">fn</span> release(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, l: &amp;Layouts) {
        <span class="k">self</span>.pipes.buf.release(ctx);
        <span class="k">self</span>.pipes.ids.release(ctx);
        <span class="k">self</span>.ribbons.buf.release(ctx);
        <span class="k">self</span>.ribbons.ids.release(ctx);
        <span class="k">self</span>.pipes.rebind(ctx, l, &amp;<span class="k">self</span>.selection);
        <span class="k">self</span>.ribbons.rebind(ctx, l, &amp;<span class="k">self</span>.selection);
    }</code></pre></div>
<p><code>lessons/04b/src/engine/gpu/segments.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Pipe rows on the GPU.</span>
    <span class="k">pub</span> <span class="k">fn</span> pipe_count(&amp;<span class="k">self</span>) -&gt; u32 {
        <span class="k">self</span>.pipes.buf.len()
    }

    <span class="c">/// Ribbon rows on the GPU.</span>
    <span class="k">pub</span> <span class="k">fn</span> ribbon_count(&amp;<span class="k">self</span>) -&gt; u32 {
        <span class="k">self</span>.ribbons.buf.len()
    }
}</code></pre></div>
<p><code>lessons/04b/src/engine/gpu/segments.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Build the nine segment pipelines.</span>
<span class="k">fn</span> build_pipelines(
    ctx: &amp;GpuCtx,
    l: &amp;Layouts,
    shader: &amp;wgpu::ShaderModule,
    target: Target,
) -&gt; SegPipelines {
    <span class="k">let</span> groups = [&amp;l.mvp, &amp;l.line, &amp;l.ink_instance, &amp;l.segment_rows];
    <span class="c">// no vertex buffer, always drawn; the shader tests depth</span>
    <span class="k">let</span> quad = PipelineDesc::new(shader, &amp;groups, &amp;[], TriangleList)
        .scene_samples(target.samples)
        .depth(DepthMode::Always);
    <span class="k">let</span> dev = &amp;ctx.device;

    SegPipelines {
        ribbon: build(
            dev,
            target,
            &amp;quad.with(&quot;<span class="s">ribbon</span>&quot;, &quot;<span class="s">fs_main</span>&quot;).color(ColorWrite::Blended),
        ),
        id_ribbon: build(
            dev,
            Target::ID,
            &amp;quad.with(&quot;<span class="s">ribbon.id</span>&quot;, &quot;<span class="s">fs_id</span>&quot;).scene_samples(<span class="s">1</span>),
        ),</code></pre></div>
<p>Copy this part from the lesson folder to the path shown.</p>
<p><code>lessons/04b/src/engine/gpu/segments.rs</code> · copy the file, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        id_edge: build(
            dev,
            Target::ID,
            &amp;quad.with(&quot;<span class="s">edge.id</span>&quot;, &quot;<span class="s">fs_edge_id</span>&quot;).scene_samples(<span class="s">1</span>),
        ),
    }
}

#[cfg(test)]
<span class="k">mod</span> tests {
    <span class="k">use</span> super::*;
    <span class="k">use</span> <span class="k">crate</span>::engine::gpu::instance::wgsl_fields;

    <span class="c">/// The shader declares StrokeSegment with the Rust fields.</span>
    #[test]
    <span class="k">fn</span> cylinder_segment_mirror() {
        <span class="k">let</span> rust = [
            &quot;<span class="s">p0x</span>&quot;,
            &quot;<span class="s">p0y</span>&quot;,
            &quot;<span class="s">p0z</span>&quot;,
            &quot;<span class="s">radius</span>&quot;,
            &quot;<span class="s">p1x</span>&quot;,
            &quot;<span class="s">p1y</span>&quot;,
            &quot;<span class="s">p1z</span>&quot;,
            &quot;<span class="s">instance_id</span>&quot;,
            &quot;<span class="s">color</span>&quot;,
            &quot;<span class="s">facing</span>&quot;,
        ];

        <span class="k">for</span> (name, src) <span class="k">in</span> SHADERS {
            assert_eq!(
                wgsl_fields(src, &quot;<span class="s">CylinderSegment</span>&quot;),
                rust,
                &quot;{<span class="s">name</span>}<span class="s">: CylinderSegment fields</span>&quot;
            );
        }

        assert_eq!(std::mem::size_of::&lt;CylinderSegment&gt;(), <span class="s">40</span>);
        assert_eq!(std::mem::offset_of!(CylinderSegment, facing), <span class="s">36</span>);
    }
}</code></pre></div>
<p>The test reads a shader struct&#39;s field names with a small helper; it lives beside <code>Instance</code>.</p>
<p><code>lessons/04b/src/engine/gpu/instance.rs</code> · copy the file, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Field names of a WGSL struct, in order.</span>
#[cfg(test)]
<span class="k">pub</span>(<span class="k">crate</span>) <span class="k">fn</span> wgsl_fields(src: &amp;str, struct_name: &amp;str) -&gt; Vec&lt;String&gt; {
    <span class="k">let</span> at = src
        .find(&amp;format!(&quot;<span class="s">struct </span>{<span class="s">struct_name</span>}&quot;))
        .expect(&quot;<span class="s">struct declared in the shader</span>&quot;);
    <span class="k">let</span> rest = &amp;src[at..];
    <span class="k">let</span> open = rest.find('<span class="s">{</span>').expect(&quot;<span class="s">struct body opens</span>&quot;);
    <span class="k">let</span> close = rest.find('<span class="s">}</span>').expect(&quot;<span class="s">struct body closes</span>&quot;);

    rest[open + <span class="s">1</span>..close]
        .lines()
        .map(|l| l.split(&quot;<span class="s">//</span>&quot;).next().unwrap_or(&quot;&quot;))
        .flat_map(|l| l.split('<span class="s">,</span>'))
        .map(|f| f.split('<span class="s">:</span>').next().unwrap_or(&quot;&quot;).trim())
        .filter(|n| !n.is_empty())
        .map(str::to_owned)
        .collect()
}</code></pre></div>
<h2 id="step-2-srcshadersink_visibilitywgsl">Step 2 · src/shaders/ink_visibility.wgsl<a class="anchor" href="#/course/04b-strokes#step-2-srcshadersink_visibilitywgsl" aria-label="Link to this section">#</a></h2>
<p>New file: the test that hides a stroke sample behind a face.</p>
<p><code>lessons/04b/src/shaders/ink_visibility.wgsl</code> · 51 lines · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>@group(<span class="s">2</span>) @binding(<span class="s">2</span>) <span class="k">var</span> scene_depth_single: texture_depth_2d;<span class="c"> // scene depth at 1x</span>
@group(<span class="s">2</span>) @binding(<span class="s">3</span>) <span class="k">var</span> scene_depth_msaa: texture_depth_multisampled_2d;<span class="c"> // scene depth at 4x</span>
override SCENE_MSAA: <span class="k">bool</span> = <span class="s">false</span>;<span class="c"> // which depth texture is live</span>

<span class="c">// Output of an ink fragment.</span>
<span class="k">struct</span> InkColor {
    @location(<span class="s">0</span>) color: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // rgba</span>
}

<span class="c">// The stroke's center line as one fragment sees it.</span>
<span class="k">struct</span> InkAxis {
    at:<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;,
    depth:<span class="k">f32</span>,
    along:<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;,
    slope:<span class="k">f32</span>,
}

<span class="k">fn</span> ink_depth(pixel:<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;, sample:<span class="k">u32</span>) -&gt; <span class="k">f32</span> {
    <span class="k">if</span> (any(pixel &lt; <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>)) || any(pixel &gt;= <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(line.vp_w, line.vp_h))) {
        <span class="k">return</span> <span class="s">0.0</span>;
    }

    <span class="k">if</span> (SCENE_MSAA) {
        <span class="k">return</span> textureLoad(scene_depth_msaa, <span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;(pixel), i32(sample));
    }

    <span class="k">return</span> textureLoad(scene_depth_single, <span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;(pixel), <span class="s">0</span>);
}

<span class="k">fn</span> ink_visible(pixel:<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;, axis:InkAxis, sample:<span class="k">u32</span>) -&gt; <span class="k">bool</span> {
    <span class="k">return</span> ink_depth(pixel, sample) &lt;= axis.depth;
}

<span class="k">fn</span> ink_disc_fragment_visible(pixel:<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;, centre:<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;, depth:<span class="k">f32</span>, sample:<span class="k">u32</span>) -&gt; <span class="k">bool</span> {
    <span class="k">return</span> ink_depth(pixel, sample) &lt;= depth;
}

<span class="k">fn</span> ink_disc_visible(pixel:<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;, centre:<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;, depth:<span class="k">f32</span>, sample:<span class="k">u32</span>) -&gt; <span class="k">bool</span> {
    <span class="k">return</span> ink_disc_fragment_visible(pixel, centre, depth, sample);
}

<span class="k">fn</span> ink_disc_source_hidden(pixel:<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;, centre:<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;, depth:<span class="k">f32</span>, sample:<span class="k">u32</span>) -&gt; <span class="k">bool</span> {
    <span class="k">return</span> ink_depth(pixel, sample) &gt; depth;
}

<span class="c">// Direction from a point to the camera; constant in ortho.</span>
<span class="k">fn</span> toward_eye(point: <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;) -&gt; <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt; {
    <span class="k">if</span> (line.ortho_h &gt; <span class="s">0.0</span>) {
        <span class="k">return</span> <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(mvp[<span class="s">0</span>].z, mvp[<span class="s">1</span>].z, mvp[<span class="s">2</span>].z);
    }

    <span class="k">return</span> <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(line.eye_x, line.eye_y, line.eye_z) - point;
}</code></pre></div>
<h2 id="step-3-srcshadersribbonwgsl">Step 3 · src/shaders/ribbon.wgsl<a class="anchor" href="#/course/04b-strokes#step-3-srcshadersribbonwgsl" aria-label="Link to this section">#</a></h2>
<p>New file: the stroke shader, a segment expanded into a screen ribbon.</p>
<p><code>lessons/04b/src/shaders/ribbon.wgsl</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">struct</span> CylinderSegment {
    p0x: <span class="k">f32</span>,<span class="c"> // start point x</span>
    p0y: <span class="k">f32</span>,<span class="c"> // start point y</span>
    p0z: <span class="k">f32</span>,<span class="c"> // start point z</span>
    radius: <span class="k">f32</span>,<span class="c"> // 0 = pen; &gt; 0 world mm; &lt; 0 pen multiplier</span>
    p1x: <span class="k">f32</span>,<span class="c"> // end point x</span>
    p1y: <span class="k">f32</span>,<span class="c"> // end point y</span>
    p1z: <span class="k">f32</span>,<span class="c"> // end point z</span>
    instance_id: <span class="k">u32</span>,<span class="c"> // object row</span>
    color: <span class="k">u32</span>,<span class="c"> // packed rgba</span>
    facing: <span class="k">u32</span>,<span class="c"> // packed normals of the two faces beside it</span>
}

@group(<span class="s">3</span>) @binding(<span class="s">0</span>) <span class="k">var</span>&lt;<span class="k">storage</span>, <span class="k">read</span>&gt; segments: <span class="k">array</span>&lt;CylinderSegment&gt;;
@group(<span class="s">3</span>) @binding(<span class="s">1</span>) <span class="k">var</span>&lt;<span class="k">storage</span>, <span class="k">read</span>&gt; source_edges: <span class="k">array</span>&lt;<span class="k">u32</span>&gt;;<span class="c"> // source edge per segment</span>
@group(<span class="s">3</span>) @binding(<span class="s">2</span>) <span class="k">var</span>&lt;<span class="k">uniform</span>&gt; edge_selection: <span class="k">vec4</span>&lt;<span class="k">u32</span>&gt;;<span class="c"> // selected (object, edge)</span></code></pre></div>
<p><code>lessons/04b/src/shaders/ribbon.wgsl</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// A wire shorter than this many pen widths thins.</span>
<span class="k">const</span> WIRE_MIN_PENS: <span class="k">f32</span> = <span class="s">3.0</span>;
<span class="k">const</span> TAPER_MIN: <span class="k">f32</span> = <span class="s">0.15</span>;

<span class="c">// True when either face beside the edge faces the camera.</span>
<span class="k">fn</span> edge_faces_camera(facing: <span class="k">u32</span>, n0: <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;, n1: <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;, to_eye: <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;) -&gt; <span class="k">bool</span> {
    <span class="k">if</span> (facing == FACING_UNKNOWN) {
        <span class="k">return</span> <span class="s">true</span>;</code></pre></div>
<p><code>lessons/04b/src/shaders/ribbon.wgsl</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    }

    <span class="k">return</span> dot(n0, to_eye) &gt; <span class="s">0.0</span> || dot(n1, to_eye) &gt; <span class="s">0.0</span>;
}

<span class="c">// Half width in px at depth \`w\`, from the radius field.</span>
<span class="k">fn</span> half_width_px(radius: <span class="k">f32</span>, w: <span class="k">f32</span>) -&gt; <span class="k">f32</span> {
    <span class="k">if</span> (radius &gt; <span class="s">0.0</span>) {
        <span class="k">if</span> (line.ortho_h &gt; <span class="s">0.0</span>) {
            <span class="k">return</span> radius * line.vp_h * <span class="s">0.5</span> / line.ortho_h;
        }

        <span class="k">return</span> radius * line.proj_y * line.vp_h * <span class="s">0.5</span> / w;
    }

    <span class="k">return</span> line.thickness * <span class="s">0.5</span>;
}

<span class="c">// Half a pixel's diagonal: how far a pixel reaches from its center.</span>
<span class="k">const</span> FILTER_REACH: <span class="k">f32</span> = <span class="s">0.70711</span>;

<span class="c">// Fraction of a pixel square lying below signed distance \`t\` from a line.</span>
<span class="k">fn</span> box_cdf(t: <span class="k">f32</span>, hi: <span class="k">f32</span>, lo: <span class="k">f32</span>, m: <span class="k">f32</span>, q: <span class="k">f32</span>) -&gt; <span class="k">f32</span> {
    <span class="k">let</span> s = clamp(t, -hi, hi);
    <span class="k">let</span> e = hi - abs(s);
    <span class="k">let</span> tail = select(e * e / q, <span class="s">1.0</span> - e * e / q, s &gt; <span class="s">0.0</span>);
    <span class="k">return</span> select(tail, <span class="s">0.5</span> + s / m, abs(s) &lt;= lo);
}

<span class="c">// Exact area of this pixel within \`hw\` of the line, \`d\` away, direction \`g\`.</span>
<span class="k">fn</span> band_area(d: <span class="k">f32</span>, hw: <span class="k">f32</span>, g: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;) -&gt; <span class="k">f32</span> {
    <span class="k">let</span> a = abs(g.x);
    <span class="k">let</span> b = abs(g.y);
    <span class="k">let</span> hi = <span class="s">0.5</span> * (a + b);
    <span class="k">let</span> lo = <span class="s">0.5</span> * abs(a - b);
    <span class="k">let</span> m = max(max(a, b), 1e-<span class="s">6</span>);
    <span class="k">let</span> q = max(<span class="s">2.0</span> * a * b, 1e-<span class="s">6</span>);
    <span class="k">return</span> box_cdf(hw - d, hi, lo, m, q) + box_cdf(hw + d, hi, lo, m, q) - <span class="s">1.0</span>;
}

<span class="c">// Never thinner than one pixel.</span>
<span class="k">fn</span> floor_hairline(px: <span class="k">f32</span>) -&gt; <span class="k">f32</span> {
    <span class="k">return</span> max(px, <span class="s">0.5</span>);
}

<span class="c">// Alpha for a line thinner than a pixel.</span>
<span class="k">fn</span> hairline_fade(px: <span class="k">f32</span>) -&gt; <span class="k">f32</span> {
    <span class="k">if</span> (px &lt; <span class="s">0.5</span>) {
        <span class="k">return</span> max(px / <span class="s">0.5</span>, HAIRLINE_MIN_ALPHA);
    }

    <span class="k">return</span> <span class="s">1.0</span>;
}

<span class="c">// What the vertex shader hands the fragment shader.</span>
<span class="k">struct</span> VsOut {
    @builtin(position) pos: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // clip position</span>
    @location(<span class="s">0</span>) color: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // rgba</span>
    @location(<span class="s">1</span>) @interpolate(linear) p: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // this corner, screen px</span>
    @location(<span class="s">2</span>) @interpolate(flat) a: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // start point, screen px</span>
    @location(<span class="s">3</span>) @interpolate(flat) b: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // end point, screen px</span>
    @location(<span class="s">4</span>) @interpolate(flat) hw0: <span class="k">f32</span>,<span class="c"> // half width at the start, px</span>
    @location(<span class="s">5</span>) @interpolate(flat) hw1: <span class="k">f32</span>,<span class="c"> // half width at the end, px</span></code></pre></div>
<p><code>lessons/04b/src/shaders/ribbon.wgsl</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    @location(<span class="s">6</span>) @interpolate(flat) solid: <span class="k">f32</span>,<span class="c"> // 1 = a mesh edge, never fades</span>
    @location(<span class="s">7</span>) @interpolate(flat) inst_id: <span class="k">u32</span>,<span class="c"> // object row</span>
    @location(<span class="s">8</span>) @interpolate(flat) segment_index: <span class="k">u32</span>,<span class="c"> // segment row</span>
    @location(<span class="s">9</span>) @interpolate(flat) end_depth: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // depth at start and end</span>
    @location(<span class="s">10</span>) @interpolate(flat) source_edge: <span class="k">u32</span>,<span class="c"> // source edge, or none</span>
};

<span class="c">// (half width, alpha) at fraction \`h\` along the segment.</span>
<span class="k">fn</span> resolve_width(in: VsOut, h: <span class="k">f32</span>) -&gt; <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt; {
    <span class="k">let</span> raw = mix(in.hw0, in.hw1, h);
    <span class="k">return</span> <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(floor_hairline(raw), select(hairline_fade(raw), <span class="s">1.0</span>, in.solid &gt; <span class="s">0.5</span>));
}

<span class="k">fn</span> density_taper(facing: <span class="k">u32</span>, len_px: <span class="k">f32</span>, px: <span class="k">f32</span>) -&gt; <span class="k">f32</span> {
    <span class="k">if</span> (facing == FACING_UNKNOWN) {
        <span class="k">return</span> <span class="s">1.0</span>;
    }

    <span class="k">let</span> room = WIRE_MIN_PENS * <span class="s">2.0</span> * max(px, 1e-<span class="s">6</span>);
    <span class="k">return</span> clamp(len_px / room, TAPER_MIN, <span class="s">1.0</span>);
}

<span class="c">// A vertex placed off screen, so nothing is drawn.</span>
<span class="k">fn</span> dead_vertex() -&gt; VsOut {
    <span class="k">var</span> dead: VsOut;<span class="c"> // all zero; only the position matters</span>
    dead.pos = <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(<span class="s">3.0</span>, <span class="s">3.0</span>, <span class="s">0.5</span>, <span class="s">1.0</span>);
    <span class="k">return</span> dead;
}

<span class="c">// Quad corner of vertex \`k\` of 6: 0 start-, 1 start+, 2 end-, 3 end+.</span>
<span class="k">fn</span> corner_of(k: <span class="k">u32</span>) -&gt; <span class="k">u32</span> {
    <span class="k">if</span> (k == <span class="s">0u</span>) {
        <span class="k">return</span> <span class="s">0u</span>;
    }

    <span class="k">if</span> (k == <span class="s">1u</span>) {
        <span class="k">return</span> <span class="s">1u</span>;
    }

    <span class="k">if</span> (k == <span class="s">2u</span> || k == <span class="s">3u</span>) {
        <span class="k">return</span> <span class="s">2u</span>;
    }

    <span class="k">if</span> (k == <span class="s">4u</span>) {
        <span class="k">return</span> <span class="s">1u</span>;
    }

    <span class="k">return</span> <span class="s">3u</span>;
}

@vertex
<span class="k">fn</span> vs_main(@builtin(vertex_index) vid: <span class="k">u32</span>) -&gt; VsOut {
    <span class="k">let</span> iid = vid / <span class="s">6u</span>;
    <span class="k">let</span> corner = corner_of(vid % <span class="s">6u</span>);
    <span class="k">let</span> seg = segments[iid];
    <span class="k">let</span> inst = instances[seg.instance_id];

    <span class="k">if</span> ((inst.flags &amp; FLAG_HIDDEN) != <span class="s">0u</span>) {
        <span class="k">return</span> dead_vertex();</code></pre></div>
<p><code>lessons/04b/src/shaders/ribbon.wgsl</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    }

    <span class="k">if</span> (!neighbor_visible(seg)) {
        <span class="k">return</span> dead_vertex();
    }

    <span class="k">let</span> model = inst.model;

    <span class="k">let</span> w0 = place(seg.instance_id, <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(seg.p0x, seg.p0y, seg.p0z));
    <span class="k">let</span> w1 = place(seg.instance_id, <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(seg.p1x, seg.p1y, seg.p1z));

    <span class="k">let</span> c0 = mvp * <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(w0, <span class="s">1.0</span>);
    <span class="k">let</span> c1 = mvp * <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(w1, <span class="s">1.0</span>);
    <span class="k">let</span> at_end1 = corner &gt;= <span class="s">2u</span>;
    <span class="k">let</span> side = select(-<span class="s">1.0</span>, <span class="s">1.0</span>, (corner &amp; <span class="s">1u</span>) == <span class="s">1u</span>);

<span class="c">    // clip to the near plane before dividing by w</span>
    <span class="k">let</span> f0 = c0.z - c0.w;
    <span class="k">let</span> f1 = c1.z - c1.w;

    <span class="k">if</span> (f0 &gt; <span class="s">0.0</span> &amp;&amp; f1 &gt; <span class="s">0.0</span>) {
        <span class="k">return</span> dead_vertex();
    }

    <span class="k">let</span> e0 = select(c0, mix(c0, c1, f0 / (f0 - f1)), f0 &gt; <span class="s">0.0</span>);
    <span class="k">let</span> e1 = select(c1, mix(c1, c0, f1 / (f1 - f0)), f1 &gt; <span class="s">0.0</span>);
    <span class="k">let</span> clip = select(e0, e1, at_end1);

<span class="c">    // both ends in screen pixels</span>
    <span class="k">let</span> vp = <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(line.vp_w, line.vp_h);
    <span class="k">let</span> s0 = (e0.xy / e0.w * <span class="s">0.5</span> + <span class="s">0.5</span>) * vp;
    <span class="k">let</span> s1 = (e1.xy / e1.w * <span class="s">0.5</span> + <span class="s">0.5</span>) * vp;
    <span class="k">let</span> d = s1 - s0;
    <span class="k">let</span> len = length(d);
    <span class="k">let</span> dir = select(<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">1.0</span>, <span class="s">0.0</span>), d / len, len &gt; 1e-<span class="s">6</span>);
    <span class="k">let</span> n = <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(-dir.y, dir.x);

<span class="c">    // Both ends keep their width on screen.</span>
    <span class="k">let</span> raw0 = half_width_px(seg.radius, e0.w);
    <span class="k">let</span> raw1 = half_width_px(seg.radius, e1.w);
    <span class="k">let</span> px = floor_hairline(select(raw0, raw1, at_end1));
    <span class="k">let</span> crowd = density_taper(seg.facing, len, px);
<span class="c">    // corner: sideways by the width, outward by the filter reach</span>
    <span class="k">let</span> along = select(-<span class="s">1.0</span>, <span class="s">1.0</span>, at_end1);
    <span class="k">let</span> p = select(s0, s1, at_end1) + (n * side + dir * along) * (px + FILTER_REACH);

    <span class="k">var</span> o: VsOut;
    <span class="k">let</span> ndc = (p / vp - <span class="s">0.5</span>) * <span class="s">2.0</span>;
    o.pos = <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(ndc * clip.w, clip.z, clip.w);
    <span class="k">var</span> color = unpack4x8unorm(seg.color) * inst.color;

    <span class="k">if</span> ((inst.flags &amp; FLAG_SELECTED) != <span class="s">0u</span> || (edge_selection.x == seg.instance_id &amp;&amp; edge_selection.y != <span class="s">0xffffffffu</span> &amp;&amp; edge_selection.y == source_edges[iid])) {
        color = <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(SELECT_COLOR, color.a);
    }

    o.color = color;
    o.p = p;</code></pre></div>
<p><code>lessons/04b/src/shaders/ribbon.wgsl</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    o.a = s0;
    o.b = s1;
    o.hw0 = raw0 * crowd;
    o.hw1 = raw1 * crowd;
    o.solid = select(<span class="s">0.0</span>, <span class="s">1.0</span>, seg.facing != FACING_UNKNOWN);
    o.inst_id = seg.instance_id;
    o.segment_index = iid;
    o.source_edge = source_edges[iid];
    o.end_depth = <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(e0.z / e0.w, e1.z / e1.w);
    <span class="k">return</span> o;
}

<span class="c">// How much of this pixel the stroke covers, 0..1, times the hairline alpha.</span>
<span class="k">fn</span> coverage(in: VsOut) -&gt; <span class="k">f32</span> {
<span class="c">    // distance from the pixel to the segment</span>
    <span class="k">let</span> pa = in.p - in.a;
    <span class="k">let</span> ba = in.b - in.a;
    <span class="k">let</span> h = clamp(dot(pa, ba) / max(dot(ba, ba), 1e-<span class="s">6</span>), <span class="s">0.0</span>, <span class="s">1.0</span>);
    <span class="k">let</span> v = pa - ba * h;
    <span class="k">let</span> d = length(v);
    <span class="k">let</span> hf = resolve_width(in, h);
    <span class="k">let</span> g = select(<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">1.0</span>, <span class="s">0.0</span>), v / d, d &gt; 1e-<span class="s">6</span>);
    <span class="k">return</span> clamp(band_area(d, hf.x, g), <span class="s">0.0</span>, <span class="s">1.0</span>) * hf.y;
}

<span class="c">// The stroke's center line as this fragment sees it.</span>
<span class="k">fn</span> ink_axis(in: VsOut) -&gt; InkAxis {
    <span class="k">let</span> ba = in.b - in.a;
    <span class="k">let</span> len2 = max(dot(ba, ba), 1e-<span class="s">6</span>);
    <span class="k">let</span> h = clamp(dot(in.p - in.a, ba) / len2, <span class="s">0.0</span>, <span class="s">1.0</span>);
    <span class="k">let</span> at = in.a + ba * h;
    <span class="k">let</span> len = sqrt(len2);
    <span class="k">let</span> along = select(<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">1.0</span>, <span class="s">0.0</span>), <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(ba.x, -ba.y) / len, len &gt; 1e-<span class="s">3</span>);
    <span class="k">let</span> slope = (in.end_depth.y - in.end_depth.x) / max(len, 1e-<span class="s">3</span>);
    <span class="k">return</span> InkAxis(<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(at.x, line.vp_h - at.y), mix(in.end_depth.x, in.end_depth.y, h), along, slope);
}

@fragment
<span class="c">// Color: the stroke, faded where geometry hides it.</span>
<span class="k">fn</span> fs_main(in: VsOut, @builtin(sample_index) sample: <span class="k">u32</span>) -&gt; InkColor {
    <span class="k">let</span> alpha = coverage(in);

    <span class="k">if</span> (alpha &lt;= <span class="s">0.0</span> || !ink_visible(in.pos.xy, ink_axis(in), sample)) {
        <span class="k">discard</span>;
    }

    <span class="k">return</span> InkColor(<span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(in.color.rgb, in.color.a * alpha));
}

<span class="c">// Visible stroke coverage into an outline mask.</span>
@fragment
<span class="k">fn</span> fs_mask(in: VsOut, @builtin(sample_index) sample: <span class="k">u32</span>) -&gt; @location(<span class="s">0</span>) <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt; {
    <span class="k">let</span> alpha = coverage(in);

    <span class="k">if</span> (alpha &lt;= <span class="s">0.0</span> || !ink_visible(in.pos.xy, ink_axis(in), sample)) {
        <span class="k">discard</span>;
    }

    <span class="k">return</span> <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(alpha);
}

<span class="c">// Output into both outline masks at once.</span>
<span class="k">struct</span> MaskPair {
    @location(<span class="s">0</span>) solid: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // every solid's mask</span>
    @location(<span class="s">1</span>) selected: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // the selection's mask</span>
};

<span class="c">// Unselected stroke: into the solid mask only.</span>
@fragment
<span class="k">fn</span> fs_masks(in: VsOut, @builtin(sample_index) sample: <span class="k">u32</span>) -&gt; MaskPair {
    <span class="k">let</span> alpha = coverage(in);

    <span class="k">if</span> (alpha &lt;= <span class="s">0.0</span> || !ink_visible(in.pos.xy, ink_axis(in), sample)) {
        <span class="k">discard</span>;
    }

    <span class="k">return</span> MaskPair(<span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(alpha), <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>));
}

@fragment
<span class="c">// Selected stroke: into both masks.</span>
<span class="k">fn</span> fs_masks_selected(in: VsOut, @builtin(sample_index) sample: <span class="k">u32</span>) -&gt; MaskPair {
    <span class="k">let</span> alpha = coverage(in);

    <span class="k">if</span> (alpha &lt;= <span class="s">0.0</span> || !ink_visible(in.pos.xy, ink_axis(in), sample)) {
        <span class="k">discard</span>;
    }

    <span class="k">return</span> MaskPair(<span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(alpha), <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(alpha));
}

<span class="c">// Bit that marks a pick id as a stroke.</span>
<span class="k">const</span> SEGMENT_BIT: <span class="k">u32</span> = <span class="s">0x80000000u</span>;

@fragment
<span class="c">// Pick id: object row + 1 and tagged segment row + 1.</span>
<span class="k">fn</span> fs_id(in: VsOut) -&gt; @location(<span class="s">0</span>) <span class="k">vec2</span>&lt;<span class="k">u32</span>&gt; {
    <span class="k">if</span> (coverage(in) &lt; <span class="s">0.5</span> || !ink_visible(in.pos.xy, ink_axis(in), <span class="s">0u</span>)) {
        <span class="k">discard</span>;
    }

    <span class="k">return</span> <span class="k">vec2</span>&lt;<span class="k">u32</span>&gt;(in.inst_id + <span class="s">1u</span>, (in.segment_index + <span class="s">1u</span>) | SEGMENT_BIT);
}

<span class="c">// Pick id for edge picks; segments without a source edge are skipped.</span>
@fragment
<span class="k">fn</span> fs_edge_id(in: VsOut) -&gt; @location(<span class="s">0</span>) <span class="k">vec2</span>&lt;<span class="k">u32</span>&gt; {
    <span class="k">if</span> (in.source_edge == <span class="s">0xffffffffu</span> || coverage(in) &lt; <span class="s">0.5</span> || !ink_visible(in.pos.xy, ink_axis(in), <span class="s">0u</span>)) {
        <span class="k">discard</span>;
    }

<span class="c">    // same tag as SEGMENT_BIT</span>
    <span class="k">return</span> <span class="k">vec2</span>&lt;<span class="k">u32</span>&gt;(in.inst_id + <span class="s">1u</span>, (in.segment_index + <span class="s">1u</span>) | <span class="s">0x80000000u</span>);
}</code></pre></div>
<p>Run <code>cargo check</code> in <code>lessons/04b/</code>.</p>
<h2 id="step-4-srcenginepipelinesmodrs">Step 4 · src/engine/pipelines/mod.rs<a class="anchor" href="#/course/04b-strokes#step-4-srcenginepipelinesmodrs" aria-label="Link to this section">#</a></h2>
<p>Add the stroke pipeline.</p>
<p><code>lessons/04b/src/engine/pipelines/mod.rs</code> · edit · type this</p>
<p>Added above</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// The pipeline layout for \`groups\`, in slot order.</span></code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Compile an ink shader: scene code plus the ink code.</span>
<span class="k">pub</span> <span class="k">fn</span> ink_module(device: &amp;wgpu::Device, label: &amp;str, source: &amp;str) -&gt; wgpu::ShaderModule {
    <span class="k">let</span> source = format!(
        &quot;{}<span class="s">\\n</span>{}&quot;,
        source,
        include_str!(&quot;<span class="s">../../shaders/ink_visibility.wgsl</span>&quot;)
    );
    module(device, label, &amp;source)
}</code></pre></div>
<h2 id="step-5-srcenginepipelineslayoutsrs">Step 5 · src/engine/pipelines/layouts.rs<a class="anchor" href="#/course/04b-strokes#step-5-srcenginepipelineslayoutsrs" aria-label="Link to this section">#</a></h2>
<p>Add the stroke bind group layout.</p>
<p><code>lessons/04b/src/engine/pipelines/layouts.rs</code> · edit · type this</p>
<p>Replaces the <code>struct Layouts</code> lines in <code>lessons/04a/src/engine/pipelines/layouts.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Group 3 for lines: rows, source edge ids, selected edge.</span>
<span class="k">fn</span> segment_rows_layout(device: &amp;wgpu::Device) -&gt; wgpu::BindGroupLayout {
    device.create_bind_group_layout(&amp;wgpu::BindGroupLayoutDescriptor {
        label: Some(&quot;<span class="s">segment.rows.layout</span>&quot;),
        entries: &amp;[
            storage_entry(<span class="s">0</span>),
            storage_entry(<span class="s">1</span>),
            buffer_entry(
                <span class="s">2</span>,
                wgpu::ShaderStages::VERTEX,
                wgpu::BufferBindingType::Uniform,
            ),
        ],
    })
}

<span class="c">/// The bind group layouts every lane shares.</span>
<span class="k">pub</span> <span class="k">struct</span> Layouts {
    <span class="k">pub</span> mvp: wgpu::BindGroupLayout, <span class="c">// group 0: camera matrix</span>
    <span class="k">pub</span> line: wgpu::BindGroupLayout, <span class="c">// group 1: pen and view settings</span>
    <span class="k">pub</span> instance: wgpu::BindGroupLayout, <span class="c">// group 2: object rows</span>
    <span class="k">pub</span> ink_instance: wgpu::BindGroupLayout, <span class="c">// group 2 for ink, with depth textures</span>
    <span class="k">pub</span> segment_rows: wgpu::BindGroupLayout, <span class="c">// group 3 for lines</span></code></pre></div>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            ink_instance: ink_instance_layout(device),</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            segment_rows: segment_rows_layout(device),</code></pre></div>
<h2 id="step-6-srcenginegpuuploadrs">Step 6 · src/engine/gpu/upload.rs<a class="anchor" href="#/course/04b-strokes#step-6-srcenginegpuuploadrs" aria-label="Link to this section">#</a></h2>
<p>Add strokes to the upload.</p>
<p><code>lessons/04b/src/engine/gpu/upload.rs</code> · edit · type this</p>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> super::objects::ObjectRows;</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> super::segments::SegRows;</code></pre></div>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> arena: ArenaRows,</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> seg: SegRows, <span class="c">// lines</span></code></pre></div>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            arena: ArenaRows::default(),</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            seg: SegRows::default(),</code></pre></div>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.arena.drop_rows();</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.seg.drop_rows();</code></pre></div>
<h2 id="step-7-srcenginegpumodrs">Step 7 · src/engine/gpu/mod.rs<a class="anchor" href="#/course/04b-strokes#step-7-srcenginegpumodrs" aria-label="Link to this section">#</a></h2>
<p>Add the stroke lane to the GPU owner and the frame.</p>
<p><code>lessons/04b/src/engine/gpu/mod.rs</code> · edit · type this</p>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> <span class="k">mod</span> objects;</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> <span class="k">mod</span> segments;</code></pre></div>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> <span class="k">use</span> objects::{ObjectRow, Rebase};</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> <span class="k">use</span> segments::CylinderSegment;</code></pre></div>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> arena: arena::ArenaLane,</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> segments: segments::SegmentLane,</code></pre></div>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> arena = arena::ArenaLane::new(&amp;ctx, &amp;layouts, target);</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> segments = segments::SegmentLane::new(&amp;ctx, &amp;layouts, target);</code></pre></div>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            arena,</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            segments,</code></pre></div>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.arena.append(&amp;<span class="k">self</span>.ctx, &amp;up.arena);</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.segments.append(&amp;<span class="k">self</span>.ctx, &amp;<span class="k">self</span>.layouts, &amp;up.seg);</code></pre></div>
<p>Replaces the line <code>self.arena.draw_print(&amp;mut pass, &amp;basic);</code> in <code>lessons/04a/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">let</span> ink = frame::Binds {
                mvp: &amp;<span class="k">self</span>.frame.mvp_group,
                line: &amp;<span class="k">self</span>.frame.line_group,
                instances: &amp;<span class="k">self</span>.objects.ink_group,
            };
            <span class="k">self</span>.arena.draw_print(&amp;<span class="k">mut</span> pass, &amp;basic);
            <span class="k">self</span>.segments.draw_pipes(&amp;<span class="k">mut</span> pass, &amp;ink);
            <span class="k">self</span>.segments.draw_ribbons(&amp;<span class="k">mut</span> pass, &amp;ink);</code></pre></div>
<h2 id="step-8-srcfixturers">Step 8 · src/fixture.rs<a class="anchor" href="#/course/04b-strokes#step-8-srcfixturers" aria-label="Link to this section">#</a></h2>
<p>Copy the test scene: it now has strokes.
Copy this file from the lesson folder to the path shown.</p>
<p><code>lessons/04b/src/fixture.rs</code> · edit · copy the file</p>
<p>Replaces the line <code>use crate::engine::gpu::{Instance, ObjectRow, Upload};</code> in <code>lessons/04a/src/fixture.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> <span class="k">crate</span>::engine::gpu::{CylinderSegment, Instance, ObjectRow, Upload};</code></pre></div>
<p>Replaces the line <code>for _ in 0..1 {</code> in <code>lessons/04a/src/fixture.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">for</span> _ <span class="k">in</span> <span class="s">0</span>..<span class="s">2</span> {</code></pre></div>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    upload.arena.idx.extend([<span class="s">0</span>, <span class="s">1</span>, <span class="s">2</span>]);</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">let</span> chain = [
        [<span class="s">0</span>.<span class="s">3</span>, <span class="s">0</span>.<span class="s">2</span>, <span class="s">0</span>.<span class="s">0</span>],
        [<span class="s">0</span>.<span class="s">7</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>],
        [<span class="s">1</span>.<span class="s">2</span>, <span class="s">0</span>.<span class="s">4</span>, <span class="s">0</span>.<span class="s">0</span>],
        [<span class="s">1</span>.<span class="s">7</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>],
    ];

    <span class="k">for</span> pair <span class="k">in</span> chain.windows(<span class="s">2</span>) {
        upload.seg.ribbons.push(CylinderSegment {
            p0: pair[<span class="s">0</span>],
            p1: pair[<span class="s">1</span>],
            radius: <span class="s">0</span>.<span class="s">0</span>,
            instance_id: <span class="s">1</span>,
            color: <span class="s">0xff55ccff</span>,
            facing: <span class="s">0xffffffff</span>,
        });
    }</code></pre></div>
<h2 id="step-9-srclibrs">Step 9 · src/lib.rs<a class="anchor" href="#/course/04b-strokes#step-9-srclibrs" aria-label="Link to this section">#</a></h2>
<p>Report the stroke count from the entry point.</p>
<p><code>lessons/04b/src/lib.rs</code> · edit · type this</p>
<p>Replaces the line <code>&quot;meshVertices&quot;:self.gpu.arena.vert_count()}).to_string())</code> in <code>lessons/04a/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            &quot;<span class="s">meshVertices</span>&quot;:<span class="k">self</span>.gpu.arena.vert_count(),&quot;<span class="s">segments</span>&quot;:<span class="k">self</span>.gpu.segments.ribbon_count()}).to_string())</code></pre></div>
<h2 id="step-10-indexhtml">Step 10 · index.html<a class="anchor" href="#/course/04b-strokes#step-10-indexhtml" aria-label="Link to this section">#</a></h2>
<p>Copy the page: the status now reports segments.
Copy this file from the lesson folder to the path shown.</p>
<p><code>lessons/04b/index.html</code> · edit · copy the file</p>
<p>Replaces the line <code>&lt;title&gt;Session checkpoint 04a&lt;/title&gt;</code> in <code>lessons/04a/index.html</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    &lt;title&gt;Session checkpoint 04b&lt;/title&gt;</code></pre></div>
<p>Replaces the line <code>&lt;output id=&quot;status&quot;&gt;Starting checkpoint 04a&lt;/output&gt;</code> in <code>lessons/04a/index.html</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    &lt;output id=&quot;<span class="s">status</span>&quot;&gt;Starting checkpoint 04b&lt;/output&gt;</code></pre></div>
<p>Replaces the line <code>document.getElementById(&#39;status&#39;).textContent = &#39;Checkpoi…</code> in <code>lessons/04a/index.html</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        document.getElementById('status').textContent = 'Checkpoint 04b · ' + state.objects + ' objects · ' + state.width + '×' + state.height;</code></pre></div>
<h2 id="check">Check<a class="anchor" href="#/course/04b-strokes#check" aria-label="Link to this section">#</a></h2>
<p>Run <code>trunk serve</code> in <code>lessons/04b/</code> and open <a href="http://127.0.0.1:8770/" target="_blank" rel="noopener">http://127.0.0.1:8770/</a>.</p>
<p>Expected: A yellow polyline appears beside the blue triangle and keeps its width while zooming; status: <strong>Checkpoint 04b · 2 objects</strong>.</p>
<p><img src="/session/docs/course/docs/screenshots/04b.png" alt="Checkpoint 04b: strokes expanded on the GPU into screen-space ribbons beside the mesh." loading="lazy" decoding="async"></p>
<p>If it fails:</p>
<ul>
<li>Lines change width with distance: the offset is applied before the perspective divide.</li>
<li>A line disappears near the camera: a segment endpoint crosses the near plane without clipping.</li>
</ul>
<h2 id="what-changed">What changed<a class="anchor" href="#/course/04b-strokes#what-changed" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>lessons/04b/src/engine/
├── gpu/
│   ├── arena.rs
│   ├── buffers.rs
│   ├── frame.rs
│   ├── instance.rs
│   ├── mod.rs  ~
│   ├── objects.rs
│   ├── segments.rs  +
│   ├── targets.rs
│   ├── text_outline.rs
│   ├── upload.rs  ~
│   └── view.rs
├── pipelines/
│   ├── layouts.rs  ~
│   └── mod.rs  ~
└── mod.rs</code></pre></div>
<p><code>+</code> new in this lesson · <code>~</code> changed in this lesson</p>
<p>Data flow: source files → retained scene state → GPU buffers → visible result. Every file at this point: <code>lessons/04b/</code>.</p>
<h2 id="next">Next<a class="anchor" href="#/course/04b-strokes#next" aria-label="Link to this section">#</a></h2>
<p><a href="#/course/04c-markers">04c · Markers</a></p>
<h2 id="expected-viewer-result">Expected viewer result<a class="anchor" href="#/course/04b-strokes#expected-viewer-result" aria-label="Link to this section">#</a></h2>
<p>Checkpoint 04b: strokes expanded on the GPU into screen-space ribbons beside the mesh.</p>
<p><a href="/session/docs/course/docs/screenshots/04b.png"><img src="/session/docs/course/docs/screenshots/04b.png" alt="Full viewer result for 04b strokes" loading="lazy" decoding="async"></a></p>
`,toc:[{level:2,id:"step-1-srcenginegpusegmentsrs",text:"Step 1 · src/engine/gpu/segments.rs"},{level:2,id:"step-2-srcshadersink_visibilitywgsl",text:"Step 2 · src/shaders/ink_visibility.wgsl"},{level:2,id:"step-3-srcshadersribbonwgsl",text:"Step 3 · src/shaders/ribbon.wgsl"},{level:2,id:"step-4-srcenginepipelinesmodrs",text:"Step 4 · src/engine/pipelines/mod.rs"},{level:2,id:"step-5-srcenginepipelineslayoutsrs",text:"Step 5 · src/engine/pipelines/layouts.rs"},{level:2,id:"step-6-srcenginegpuuploadrs",text:"Step 6 · src/engine/gpu/upload.rs"},{level:2,id:"step-7-srcenginegpumodrs",text:"Step 7 · src/engine/gpu/mod.rs"},{level:2,id:"step-8-srcfixturers",text:"Step 8 · src/fixture.rs"},{level:2,id:"step-9-srclibrs",text:"Step 9 · src/lib.rs"},{level:2,id:"step-10-indexhtml",text:"Step 10 · index.html"},{level:2,id:"check",text:"Check"},{level:2,id:"what-changed",text:"What changed"},{level:2,id:"next",text:"Next"},{level:2,id:"expected-viewer-result",text:"Expected viewer result"}]};export{s as default};
