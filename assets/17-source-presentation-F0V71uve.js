const s={title:"17 · Source faces, text objects and one silhouette",html:`<h1 id="17-source-faces-text-objects-and-one-silhouette">17 · Source faces, text objects and one silhouette<a class="anchor" href="#/course/17-source-presentation#17-source-faces-text-objects-and-one-silhouette" aria-label="Link to this section">#</a></h1>
<p>Source faces and scene text become selectable, and optional outlines surround visible solids.</p>
<p><img src="/session/docs/course/docs/illustrations/pick-window.svg" alt="A pick renders a 19 x 19 attachment: a 13 x 13 readback window inside a three-texel halo, with origin and frame carrying the canvas into it." loading="lazy" decoding="async"></p>
<h2 id="step-1-srcenginegpufacesrs">Step 1 · src/engine/gpu/faces.rs<a class="anchor" href="#/course/17-source-presentation#step-1-srcenginegpufacesrs" aria-label="Link to this section">#</a></h2>
<p>Face buffers preserve source face addresses alongside triangles.</p>
<p><code>lessons/17/src/engine/gpu/faces.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! Face geometry kept apart from edges, because a face needs shading and normals while an edge needs neither.</span>
<span class="k">use</span> super::buffers::{GpuCtx, GrowBuf, ROWS};
<span class="k">use</span> super::frame::Binds;
<span class="k">use</span> <span class="k">crate</span>::engine::pipelines::{DepthMode, Layouts, PipelineDesc, Target, build};

<span class="c">/// Bit that marks a pick id as a face.</span>
<span class="k">pub</span> <span class="k">const</span> FACE_TAG: u32 = <span class="s">0x2000_0000</span>;

<span class="c">/// Which object and face a triangle came from.</span>
#[derive(Clone, Copy)]
<span class="k">pub</span> <span class="k">struct</span> FaceSource {
    <span class="k">pub</span> parent: u32, <span class="c">// object row</span>
    <span class="k">pub</span> face: usize, <span class="c">// face index in that object</span>
}

<span class="c">/// Solid faces: their source ids, the selected one, and the pipelines.</span>
<span class="k">pub</span> <span class="k">struct</span> Faces {
    <span class="k">pub</span> sources: Vec&lt;FaceSource&gt;,
    ids: GrowBuf, <span class="c">// face id per triangle</span>
    selected: wgpu::Buffer, <span class="c">// selected face id, read by shaders</span>
    active: Option&lt;u32&gt;, <span class="c">// selected face id, if any</span>
    layout: wgpu::BindGroupLayout,
    group: Option&lt;wgpu::BindGroup&gt;, <span class="c">// the face buffers, bound</span>
    pick: wgpu::RenderPipeline, <span class="c">// face id per pixel</span>
    highlight: wgpu::RenderPipeline, <span class="c">// selected face in color</span>
    mask: wgpu::RenderPipeline, <span class="c">// selected face into a mask</span>
}</code></pre></div>
<p><code>lessons/17/src/engine/gpu/faces.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">impl</span> Faces {
    <span class="c">/// Create the layout, the selection buffer and the pipelines.</span>
    <span class="k">pub</span> <span class="k">fn</span> new(
        ctx: &amp;GpuCtx,
        layouts: &amp;Layouts,
        shader: &amp;wgpu::ShaderModule,
        target: Target,
    ) -&gt; <span class="k">Self</span> {
        <span class="c">// bindings 0-3 are storage buffers, 4 is the selection</span>
        <span class="k">let</span> entries: Vec&lt;_&gt; = (<span class="s">0</span>..<span class="s">5</span>)
            .map(|binding| wgpu::BindGroupLayoutEntry {
                binding,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: <span class="k">if</span> binding == <span class="s">4</span> {
                        wgpu::BufferBindingType::Uniform
                    } <span class="k">else</span> {
                        wgpu::BufferBindingType::Storage { read_only: <span class="s">true</span> }
                    },
                    has_dynamic_offset: <span class="s">false</span>,
                    min_binding_size: None,
                },
                count: None,
            })
            .collect();
        <span class="k">let</span> layout = ctx
            .device
            .create_bind_group_layout(&amp;wgpu::BindGroupLayoutDescriptor {
                label: Some(&quot;<span class="s">source faces</span>&quot;),
                entries: &amp;entries,
            });
        <span class="c">// 16 bytes: selected face id plus padding</span>
        <span class="k">let</span> selected = super::buffers::zeroed_buffer(
            &amp;ctx.device,
            &quot;<span class="s">selected source face</span>&quot;,
            <span class="s">16</span>,
            wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        );
        <span class="k">let</span> (pick, highlight, mask) = pipelines(ctx, layouts, shader, target, &amp;layout);
        <span class="k">Self</span> {
            sources: Vec::new(),
            ids: GrowBuf::new(ctx, &quot;<span class="s">source face ids</span>&quot;, <span class="s">4</span>, ROWS),
            selected,
            active: None,
            layout,
            group: None,
            pick,
            highlight,
            mask,
        }
    }</code></pre></div>
<p><code>lessons/17/src/engine/gpu/faces.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Rebuild the pipelines for a new MSAA sample count.</span>
    <span class="k">pub</span> <span class="k">fn</span> retarget(
        &amp;<span class="k">mut</span> <span class="k">self</span>,
        ctx: &amp;GpuCtx,
        layouts: &amp;Layouts,
        shader: &amp;wgpu::ShaderModule,
        target: Target,
    ) {
        (<span class="k">self</span>.pick, <span class="k">self</span>.highlight, <span class="k">self</span>.mask) =
            pipelines(ctx, layouts, shader, target, &amp;<span class="k">self</span>.layout);
    }

    <span class="c">/// Append one upload's faces and rebuild the bind group.</span>
    <span class="k">pub</span> <span class="k">fn</span> append(
        &amp;<span class="k">mut</span> <span class="k">self</span>,
        ctx: &amp;GpuCtx,
        up: &amp;super::arena::ArenaRows,
        buffers: [&amp;wgpu::Buffer; <span class="s">3</span>],
    ) {
        <span class="c">// faces before this upload</span>
        <span class="k">let</span> base = <span class="k">self</span>.sources.len() <span class="k">as</span> u32;
        <span class="k">self</span>.sources.extend_from_slice(&amp;up.face_sources);
        <span class="c">// one face id per triangle, u32::MAX = none</span>
        <span class="k">let</span> ids: Vec&lt;u32&gt; = (<span class="s">0</span>..up.idx.len() / <span class="s">3</span>)
            .map(|i| <span class="k">match</span> up.face_ids.get(i) {
                Some(&amp;id) <span class="k">if</span> id != u32::MAX =&gt; base + id,
                _ =&gt; u32::MAX,
            })
            .collect();
        <span class="k">self</span>.ids.append(ctx, &amp;ids);
        <span class="k">let</span> buffers = [
            buffers[<span class="s">0</span>],
            buffers[<span class="s">1</span>],
            buffers[<span class="s">2</span>],
            &amp;<span class="k">self</span>.ids.buf,
            &amp;<span class="k">self</span>.selected,
        ];
        <span class="k">let</span> entries: Vec&lt;_&gt; = buffers
            .iter()
            .enumerate()
            .map(|(binding, buffer)| wgpu::BindGroupEntry {
                binding: binding <span class="k">as</span> u32,
                resource: buffer.as_entire_binding(),
            })
            .collect();
        <span class="k">self</span>.group = Some(ctx.device.create_bind_group(&amp;wgpu::BindGroupDescriptor {
            label: Some(&quot;<span class="s">source faces</span>&quot;),
            layout: &amp;<span class="k">self</span>.layout,
            entries: &amp;entries,
        }));
    }</code></pre></div>
<p><code>lessons/17/src/engine/gpu/faces.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Face behind a pick id, if it belongs to object \`row\`.</span>
    <span class="k">pub</span> <span class="k">fn</span> source(&amp;<span class="k">self</span>, row: u32, sub: u32) -&gt; Option&lt;(u32, FaceSource)&gt; {
        <span class="c">// top three bits say what kind of pick</span>
        <span class="k">if</span> sub &amp; <span class="s">0xe000_0000</span> != FACE_TAG {
            <span class="k">return</span> None;
        }

        <span class="k">let</span> address = sub &amp; !FACE_TAG;
        <span class="k">let</span> source = *<span class="k">self</span>.sources.get(address <span class="k">as</span> usize)?;
        (source.parent == row).then_some((address, source))
    }

    <span class="c">/// Select a face; None clears the selection.</span>
    <span class="k">pub</span> <span class="k">fn</span> select(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, face: Option&lt;u32&gt;) {
        <span class="k">self</span>.active = face;
        ctx.queue.write_buffer(
            &amp;<span class="k">self</span>.selected,
            <span class="s">0</span>,
            bytemuck::cast_slice(&amp;[face.unwrap_or(u32::MAX), <span class="s">0</span>, <span class="s">0</span>, <span class="s">0</span>]),
        );
    }

    <span class="c">/// Draw face ids.</span>
    <span class="k">pub</span> <span class="k">fn</span> draw_ids(&amp;<span class="k">self</span>, pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;, binds: &amp;Binds) -&gt; u32 {
        <span class="k">self</span>.draw(pass, binds, &amp;<span class="k">self</span>.pick)
    }

    <span class="c">/// Draw the selected face highlighted.</span>
    <span class="k">pub</span> <span class="k">fn</span> draw_highlight(&amp;<span class="k">self</span>, pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;, binds: &amp;Binds) -&gt; u32 {
        <span class="k">if</span> <span class="k">self</span>.active.is_none() {
            <span class="k">return</span> <span class="s">0</span>;
        }

        <span class="k">self</span>.draw(pass, binds, &amp;<span class="k">self</span>.highlight)
    }

    <span class="c">/// Draw the selected face into the selection mask.</span>
    <span class="k">pub</span> <span class="k">fn</span> draw_mask(&amp;<span class="k">self</span>, pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;, binds: &amp;Binds) -&gt; u32 {
        <span class="k">if</span> <span class="k">self</span>.active.is_none() {
            <span class="k">return</span> <span class="s">0</span>;
        }

        <span class="k">self</span>.draw(pass, binds, &amp;<span class="k">self</span>.mask)
    }</code></pre></div>
<p><code>lessons/17/src/engine/gpu/faces.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Draw every face with \`pipeline\`; returns the draw count.</span>
    <span class="k">fn</span> draw(
        &amp;<span class="k">self</span>,
        pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;,
        binds: &amp;Binds,
        pipeline: &amp;wgpu::RenderPipeline,
    ) -&gt; u32 {
        <span class="k">let</span> Some(group) = &amp;<span class="k">self</span>.group <span class="k">else</span> {
            <span class="k">return</span> <span class="s">0</span>;
        };

        <span class="k">if</span> <span class="k">self</span>.ids.is_empty() {
            <span class="k">return</span> <span class="s">0</span>;
        }

        pass.set_pipeline(pipeline);
        binds.set(pass);
        pass.set_bind_group(<span class="s">3</span>, group, &amp;[]);
        <span class="c">// three vertices per triangle, read by index in the shader</span>
        pass.draw(<span class="s">0</span>..<span class="k">self</span>.ids.len() * <span class="s">3</span>, <span class="s">0</span>..<span class="s">1</span>);
        <span class="s">1</span>
    }

    <span class="c">/// Forget every face; keep the buffers.</span>
    <span class="k">pub</span> <span class="k">fn</span> reset(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx) {
        <span class="k">self</span>.select(ctx, None);
        <span class="k">self</span>.ids.reset();
        <span class="k">self</span>.sources.clear();
        <span class="k">self</span>.group = None;
    }

    <span class="c">/// Forget every face and free the buffers.</span>
    <span class="k">pub</span> <span class="k">fn</span> release(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx) {
        <span class="k">self</span>.reset(ctx);
        <span class="k">self</span>.ids.release(ctx);
        <span class="k">self</span>.sources.shrink_to_fit();
    }

    <span class="c">/// Bytes reserved on the GPU by this struct.</span>
    <span class="k">pub</span> <span class="k">fn</span> allocated_bytes(&amp;<span class="k">self</span>) -&gt; u64 {
        <span class="k">self</span>.ids.buf.size() + <span class="k">self</span>.selected.size()
    }
}</code></pre></div>
<p><code>lessons/17/src/engine/gpu/faces.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Build the six face pipelines.</span>
<span class="k">fn</span> pipelines(
    ctx: &amp;GpuCtx,
    layouts: &amp;Layouts,
    shader: &amp;wgpu::ShaderModule,
    target: Target,
    layout: &amp;wgpu::BindGroupLayout,
) -&gt; (
    wgpu::RenderPipeline,
    wgpu::RenderPipeline,
    wgpu::RenderPipeline,
) {
    <span class="k">let</span> groups = [&amp;layouts.mvp, &amp;layouts.line, &amp;layouts.instance, layout];
    <span class="c">// no vertex buffers: the shader reads vertices by index</span>
    <span class="k">let</span> base = PipelineDesc::new(shader, &amp;groups, &amp;[], wgpu::PrimitiveTopology::TriangleList)
        .vertex(&quot;<span class="s">vs_face</span>&quot;);
    <span class="k">let</span> pick = build(
        &amp;ctx.device,
        Target::ID,
        &amp;base.with(&quot;<span class="s">source face IDs</span>&quot;, &quot;<span class="s">fs_id</span>&quot;).physical(),
    );
    <span class="k">let</span> highlight = build(
        &amp;ctx.device,
        target,
        &amp;base
            .with(&quot;<span class="s">selected face</span>&quot;, &quot;<span class="s">fs_face_highlight</span>&quot;)
            .depth(DepthMode::ReadOnlyEqual),
    );
    <span class="k">let</span> mask = build(
        &amp;ctx.device,
        Target {
            format: wgpu::TextureFormat::R8Unorm,
            samples: target.samples,
        },
        &amp;base
            .with(&quot;<span class="s">selected face mask</span>&quot;, &quot;<span class="s">fs_selection_mask</span>&quot;)
            .depth(DepthMode::ReadOnlyEqual),
    );
    (pick, highlight, mask)
}</code></pre></div>
<h2 id="step-2-srcshaderstrianglewgsl">Step 2 · src/shaders/triangle.wgsl<a class="anchor" href="#/course/17-source-presentation#step-2-srcshaderstrianglewgsl" aria-label="Link to this section">#</a></h2>
<p>The mesh shader places vertices and shades visible faces.</p>
<p><code>lessons/17/src/shaders/triangle.wgsl</code> · edit · type this</p>
<p>Added after the <code>@location(6) @interpolate(flat) selected: u32,</code> line in <code>struct VsOut</code> of <code>lessons/16/src/shaders/triangle.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    @location(<span class="s">7</span>) @interpolate(flat) source_face: <span class="k">u32</span>,</code></pre></div>
<p>Replaces the 6 lines from <code>return dead;</code> in <code>fn dead_vertex</code> of <code>lessons/16/src/shaders/triangle.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    dead.source_face = <span class="s">0xffffffffu</span>;
    <span class="k">return</span> dead;
}

<span class="c">// Place and color one vertex.</span>
<span class="k">fn</span> transform_vertex(in: VsIn) -&gt; VsOut {</code></pre></div>
<p>Replaces the <code>return o;</code> line in <code>fn vs_main</code> of <code>lessons/16/src/shaders/triangle.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    o.source_face = <span class="s">0xffffffffu</span>;
    <span class="k">return</span> o;
}

@vertex
<span class="c">// Vertices from the vertex buffers.</span>
<span class="k">fn</span> vs_main(in: VsIn) -&gt; VsOut {
    <span class="k">return</span> transform_vertex(in);
}

@group(<span class="s">3</span>) @binding(<span class="s">0</span>) <span class="k">var</span>&lt;<span class="k">storage</span>, <span class="k">read</span>&gt; face_vertices: <span class="k">array</span>&lt;<span class="k">f32</span>&gt;;<span class="c"> // mesh vertices, ten floats each</span>

<span class="c">// Bit that marks a pick id as a face; matches FACE_TAG in Rust.</span>
<span class="k">const</span> FACE_TAG: <span class="k">u32</span> = <span class="s">0x20000000u</span>;
@group(<span class="s">3</span>) @binding(<span class="s">1</span>) <span class="k">var</span>&lt;<span class="k">storage</span>, <span class="k">read</span>&gt; face_objects: <span class="k">array</span>&lt;<span class="k">u32</span>&gt;;<span class="c"> // object row per vertex</span>
@group(<span class="s">3</span>) @binding(<span class="s">2</span>) <span class="k">var</span>&lt;<span class="k">storage</span>, <span class="k">read</span>&gt; face_indices: <span class="k">array</span>&lt;<span class="k">u32</span>&gt;;
@group(<span class="s">3</span>) @binding(<span class="s">3</span>) <span class="k">var</span>&lt;<span class="k">storage</span>, <span class="k">read</span>&gt; source_faces: <span class="k">array</span>&lt;<span class="k">u32</span>&gt;;
@group(<span class="s">3</span>) @binding(<span class="s">4</span>) <span class="k">var</span>&lt;<span class="k">uniform</span>&gt; selected_face: <span class="k">vec4</span>&lt;<span class="k">u32</span>&gt;;

@vertex
<span class="c">// Vertices read by index, with the source face and its selection.</span>
<span class="k">fn</span> vs_face(@builtin(vertex_index) index: <span class="k">u32</span>) -&gt; VsOut {
    <span class="k">let</span> vertex = face_indices[index];
    <span class="k">let</span> start = vertex * <span class="s">10u</span>;<span class="c"> // ten floats per vertex: position, normal, color, padding</span>
    <span class="k">let</span> position = <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(face_vertices[start], face_vertices[start+<span class="s">1u</span>], face_vertices[start+<span class="s">2u</span>]);
    <span class="k">let</span> normal = <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(face_vertices[start+<span class="s">3u</span>], face_vertices[start+<span class="s">4u</span>], face_vertices[start+<span class="s">5u</span>]);
    <span class="k">let</span> color = <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(face_vertices[start+<span class="s">6u</span>], face_vertices[start+<span class="s">7u</span>], face_vertices[start+<span class="s">8u</span>]);
    <span class="k">var</span> out = transform_vertex(VsIn(position, normal, color, face_objects[vertex]));
    out.source_face = source_faces[index/<span class="s">3u</span>];
    out.selected = select(<span class="s">0u</span>, <span class="s">1u</span>, out.source_face != <span class="s">0xffffffffu</span> &amp;&amp; out.source_face == selected_face.x);

    <span class="k">if</span> (out.selected != <span class="s">0u</span>) {
        out.color = SELECT_COLOR;
    }

    <span class="k">return</span> out;
}

<span class="c">// Direction toward the camera; constant in ortho.</span></code></pre></div>
<p>Replaces the <code>return PhysicalId(vec2&lt;u32&gt;(in.inst_id + 1u,…</code> line in <code>fn fs_id</code> of <code>lessons/16/src/shaders/triangle.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">let</span> sub = select((<span class="s">0x20000000u</span> | in.source_face) + <span class="s">1u</span>, <span class="s">0u</span>, in.source_face == <span class="s">0xffffffffu</span>);
    <span class="k">return</span> PhysicalId(<span class="k">vec2</span>&lt;<span class="k">u32</span>&gt;(in.inst_id + <span class="s">1u</span>, sub), physical_gradient(in.pos.z));</code></pre></div>
<p>Added after the <code>}</code> line of <code>lessons/16/src/shaders/triangle.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>@fragment
<span class="c">// The selected source face, shaded.</span>
<span class="k">fn</span> fs_face_highlight(in: VsOut, @builtin(front_facing) front: <span class="k">bool</span>) -&gt; @location(<span class="s">0</span>) <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt; {
    <span class="k">if</span> (in.selected == <span class="s">0u</span>) {
        <span class="k">discard</span>;
    }

    <span class="k">return</span> shade(in, front);
}

<span class="c">// Every face into the solid mask.</span>
@fragment
<span class="k">fn</span> fs_solid_mask(in: VsOut) -&gt; @location(<span class="s">0</span>) <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt; {
    <span class="k">return</span> <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(<span class="s">1.0</span>);
}</code></pre></div>
<p>Run <code>cargo check</code> in <code>lessons/17/</code>.</p>
<h2 id="step-3-srcenginegpuarenars">Step 3 · src/engine/gpu/arena.rs<a class="anchor" href="#/course/17-source-presentation#step-3-srcenginegpuarenars" aria-label="Link to this section">#</a></h2>
<p>The arena holds mesh vertices and indices across objects.</p>
<p><code>lessons/17/src/engine/gpu/arena.rs</code> · edit · type this</p>
<p>Added after the <code>pub idx_text: Vec&lt;u32&gt;,</code> line in <code>struct ArenaRows</code> of <code>lessons/16/src/engine/gpu/arena.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> face_ids: Vec&lt;u32&gt;, <span class="c">// source face of each solid triangle</span>
    <span class="k">pub</span> face_sources: Vec&lt;super::faces::FaceSource&gt;,</code></pre></div>
<p>Added after the <code>drop_rows(&amp;mut self.idx_text);</code> line in <code>fn drop_rows</code> of <code>lessons/16/src/engine/gpu/arena.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        drop_rows(&amp;<span class="k">mut</span> <span class="k">self</span>.face_ids);
        drop_rows(&amp;<span class="k">mut</span> <span class="k">self</span>.face_sources);</code></pre></div>
<p>Added after the <code>selection_mask: wgpu::RenderPipeline,</code> line in <code>struct ArenaPipelines</code> of <code>lessons/16/src/engine/gpu/arena.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    solid_mask: wgpu::RenderPipeline, <span class="c">// marks every solid face</span></code></pre></div>
<p>Added after the <code>outline_text: OutlineTextLane,</code> line in <code>struct ArenaLane</code> of <code>lessons/16/src/engine/gpu/arena.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> source_faces: super::faces::Faces, <span class="c">// solid faces with their source ids</span></code></pre></div>
<p>Added after the <code>+ self.text.buf.size()</code> line in <code>fn allocated_bytes</code> of <code>lessons/16/src/engine/gpu/arena.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            + <span class="k">self</span>.source_faces.allocated_bytes()</code></pre></div>
<p>Added after the <code>let pipes = build_pipelines(ctx, l, &amp;shader,…</code> line in <code>fn new</code> of <code>lessons/16/src/engine/gpu/arena.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> source_faces = super::faces::Faces::new(ctx, l, &amp;shader, target);
        <span class="k">Self</span> {
            source_faces,
            verts: GrowBuf::new(
                ctx,
                &quot;<span class="s">arena.vbo</span>&quot;,
                std::mem::size_of::&lt;RenderVertex&gt;() <span class="k">as</span> u64,
                VERTS | wgpu::BufferUsages::STORAGE,
            ),
            vids: GrowBuf::new(ctx, &quot;<span class="s">arena.vids</span>&quot;, <span class="s">4</span>, VERTS | wgpu::BufferUsages::STORAGE),
            faces: GrowBuf::new(ctx, &quot;<span class="s">arena.ibo</span>&quot;, <span class="s">4</span>, INDICES | wgpu::BufferUsages::STORAGE),</code></pre></div>
<p>Added after the <code>self.outline_text.retarget(ctx, l, target);</code> line in <code>fn retarget</code> of <code>lessons/16/src/engine/gpu/arena.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.source_faces.retarget(ctx, l, &amp;<span class="k">self</span>.shader, target);</code></pre></div>
<p>Added after the <code>self.text.append(ctx, &amp;up.idx_text);</code> line in <code>fn append</code> of <code>lessons/16/src/engine/gpu/arena.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.source_faces
            .append(ctx, up, [&amp;<span class="k">self</span>.verts.buf, &amp;<span class="k">self</span>.vids.buf, &amp;<span class="k">self</span>.faces.buf]);</code></pre></div>
<p>Added after the <code>.draw_physical_ids(pass, b, &amp;self.outline_buf…</code> line in <code>fn draw_face_ids</code> of <code>lessons/16/src/engine/gpu/arena.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Draw face ids of faces and sheet fills.</span>
    <span class="k">pub</span> <span class="k">fn</span> draw_component_ids(&amp;<span class="k">self</span>, pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;, b: &amp;Binds) -&gt; u32 {
        <span class="k">self</span>.source_faces.draw_ids(pass, b)
            + <span class="k">self</span>
                .outline_text
                .draw_physical_ids(pass, b, &amp;<span class="k">self</span>.outline_buffers(&amp;<span class="k">self</span>.print))
    }

    <span class="c">/// Draw every solid face into a mask.</span>
    <span class="k">pub</span> <span class="k">fn</span> draw_solid_mask(&amp;<span class="k">self</span>, pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;, b: &amp;Binds) -&gt; u32 {
        <span class="k">self</span>.draw_run(pass, b, &amp;<span class="k">self</span>.pipes.solid_mask, &amp;<span class="k">self</span>.faces)
    }

    <span class="c">/// Draw object ids of sheet lettering.</span></code></pre></div>
<h2 id="step-4-srcappwalkmeshrs">Step 4 · src/app/walk/mesh.rs<a class="anchor" href="#/course/17-source-presentation#step-4-srcappwalkmeshrs" aria-label="Link to this section">#</a></h2>
<p>The mesh walk uploads vertices, indices and source face IDs.</p>
<p><code>lessons/17/src/app/walk/mesh.rs</code> · edit · type this</p>
<p>Delete the <code>use session_rust::RenderVertex;</code> line of <code>lessons/16/src/app/walk/mesh.rs</code>.</p>
<p>Added after the <code>}</code> line in <code>fn walk_mesh</code> of <code>lessons/16/src/app/walk/mesh.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">if</span> !(o.sheet_lanes &amp;&amp; print) {
        <span class="c">// a smooth surface is one source face</span>
        append_face_ids(arena, m, cx.row, o.smooth, rm.indices.len() / <span class="s">3</span>);
    }</code></pre></div>
<p>Replaces <code>fn positions</code> in <code>lessons/16/src/app/walk/mesh.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// One source face address per triangle.</span>
<span class="k">fn</span> append_face_ids(
    arena: &amp;<span class="k">mut</span> ArenaRows,
    mesh: &amp;Mesh,
    parent: u32,
    surface: bool,
    triangles: usize,
) {
    <span class="k">use</span> <span class="k">crate</span>::engine::gpu::faces::FaceSource;

    <span class="c">// a surface is one face</span>
    <span class="k">if</span> surface {
        <span class="k">let</span> address = arena.face_sources.len() <span class="k">as</span> u32;
        arena.face_sources.push(FaceSource { parent, face: <span class="s">0</span> });
        arena
            .face_ids
            .extend(std::iter::repeat_n(address, triangles));
        <span class="k">return</span>;
    }

    <span class="k">let</span> start = arena.face_ids.len();
    <span class="k">let</span> <span class="k">mut</span> keys: Vec&lt;_&gt; = mesh.face.keys().copied().collect();
    keys.sort_unstable();

    <span class="k">for</span> face <span class="k">in</span> keys {
        <span class="k">let</span> address = arena.face_sources.len() <span class="k">as</span> u32;
        arena.face_sources.push(FaceSource { parent, face });
        <span class="k">let</span> valid =
            |triangle: &amp;[usize; <span class="s">3</span>]| triangle.iter().all(|key| mesh.vertex.contains_key(key));
        <span class="c">// triangles of this face: cached, else a fan</span>
        <span class="k">let</span> count = <span class="k">if</span> <span class="k">let</span> Some(cached) = mesh
            .triangulation
            .get(&amp;face)
            .filter(|tris| !tris.is_empty())
        {
            cached.iter().filter(|tri| valid(tri)).count()
        } <span class="k">else</span> {
            <span class="k">let</span> corners = &amp;mesh.face[&amp;face];
            (<span class="s">1</span>..corners.len().saturating_sub(<span class="s">1</span>))
                .filter(|&amp;i| valid(&amp;[corners[<span class="s">0</span>], corners[i], corners[i + <span class="s">1</span>]]))
                .count()
        };
        arena.face_ids.extend(std::iter::repeat_n(address, count));
    }

    assert_eq!(
        arena.face_ids.len() - start,
        triangles,
        &quot;<span class="s">source face IDs must match the kernel triangle stream</span>&quot;
    );</code></pre></div>
<h2 id="step-5-srcappwalkbreprs">Step 5 · src/app/walk/brep.rs<a class="anchor" href="#/course/17-source-presentation#step-5-srcappwalkbreprs" aria-label="Link to this section">#</a></h2>
<p>The geometry walk uploads shaded faces and their source boundary edges.</p>
<p><code>lessons/17/src/app/walk/brep.rs</code> · edit · type this</p>
<p>Replaces <code>fn push_face</code> in <code>lessons/16/src/app/walk/brep.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Append one face mesh to the arena.</span>
<span class="k">fn</span> push_face(arena: &amp;<span class="k">mut</span> ArenaRows, rm: &amp;RenderMesh, cx: &amp;WalkCx, solid: &amp;<span class="k">mut</span> Solid, face: usize) {
    <span class="c">// one source face address for every triangle</span>
    <span class="k">let</span> address = arena.face_sources.len() <span class="k">as</span> u32;
    arena
        .face_sources
        .push(<span class="k">crate</span>::engine::gpu::faces::FaceSource {
            parent: cx.row,
            face,
        });
    arena
        .face_ids
        .extend(std::iter::repeat_n(address, rm.indices.len() / <span class="s">3</span>));</code></pre></div>
<p>Replaces the <code>push_face(arena, &amp;rm, cx, &amp;mut solid);</code> line in <code>fn walk_brep</code> of <code>lessons/16/src/app/walk/brep.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        push_face(arena, &amp;rm, cx, &amp;<span class="k">mut</span> solid, fi);</code></pre></div>
<h2 id="step-6-srcappselectionrs">Step 6 · src/app/selection.rs<a class="anchor" href="#/course/17-source-presentation#step-6-srcappselectionrs" aria-label="Link to this section">#</a></h2>
<p>Selection keeps original edge, face and control IDs under their parent object.</p>
<p><code>lessons/17/src/app/selection.rs</code> · edit · type this</p>
<p>Added after the <code>},</code> line in <code>enum SelectionMode</code> of <code>lessons/16/src/app/selection.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    Face {
        parent: u32, <span class="c">// object row</span>
        face: usize,
    },</code></pre></div>
<p>Replaces the <code>Self::Edge { parent, .. } | Self::Controls {…</code> line in <code>fn parent</code> of <code>lessons/16/src/app/selection.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">Self</span>::Edge { parent, .. }
            | <span class="k">Self</span>::Face { parent, .. }
            | <span class="k">Self</span>::Controls { parent, .. } =&gt; Some(*parent),</code></pre></div>
<h2 id="step-7-srcenginegpupickrs">Step 7 · src/engine/gpu/pick.rs<a class="anchor" href="#/course/17-source-presentation#step-7-srcenginegpupickrs" aria-label="Link to this section">#</a></h2>
<p>Picking reads an object and subobject ID asynchronously.</p>
<p><code>lessons/17/src/engine/gpu/pick.rs</code> · edit · type this</p>
<p>Added after the <code>use super::buffers::GpuCtx;</code> line of <code>lessons/16/src/engine/gpu/pick.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> super::frame::PickView;</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Textures the id pass draws into, sized to the pick window.</span></code></pre></div>
<p>Added after the <code>pub const PICK_RADIUS: u32 = 6;</code> line of <code>lessons/16/src/engine/gpu/pick.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Extra pixels drawn around the window, for the visibility test.</span>
<span class="k">pub</span> <span class="k">const</span> PICK_HALO: u32 = <span class="s">3</span>;

<span class="c">/// Largest tolerance, framebuffer pixels.</span></code></pre></div>
<p>Added after the <code>Edge,</code> line in <code>enum PickMode</code> of <code>lessons/16/src/engine/gpu/pick.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    Component, <span class="c">// edges first, else the visible face</span></code></pre></div>
<p>Added after the <code>}</code> line in <code>impl Window</code> of <code>lessons/16/src/engine/gpu/pick.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// The window plus its halo, as the pass draws it.</span>
    <span class="k">pub</span> <span class="k">fn</span> view(&amp;<span class="k">self</span>, size: (u32, u32)) -&gt; PickView {
        <span class="k">let</span> size = (size.<span class="s">0</span>.max(<span class="s">1</span>), size.<span class="s">1</span>.max(<span class="s">1</span>));
        <span class="k">let</span> x = <span class="k">self</span>.x.saturating_sub(PICK_HALO);
        <span class="k">let</span> y = <span class="k">self</span>.y.saturating_sub(PICK_HALO);
        <span class="k">let</span> right = (<span class="k">self</span>.x + <span class="k">self</span>.w + PICK_HALO).min(size.<span class="s">0</span>);
        <span class="k">let</span> bottom = (<span class="k">self</span>.y + <span class="k">self</span>.h + PICK_HALO).min(size.<span class="s">1</span>);
        PickView {
            x,
            y,
            w: (right - x).max(<span class="s">1</span>),
            h: (bottom - y).max(<span class="s">1</span>),
        }
    }
}

<span class="c">/// Bytes per row of the readback buffer, 256-aligned as wgpu requires.</span></code></pre></div>
<p>Added after the <code>targets: Option&lt;IdTargets&gt;,</code> line in <code>struct Picker</code> of <code>lessons/16/src/engine/gpu/pick.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    view: PickView, <span class="c">// where the textures sit in the canvas</span></code></pre></div>
<p>Added after the <code>targets: None,</code> line in <code>fn new</code> of <code>lessons/16/src/engine/gpu/pick.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            view: PickView::whole((<span class="s">1</span>, <span class="s">1</span>)),</code></pre></div>
<p>Added after the <code>}</code> line in <code>impl Picker</code> of <code>lessons/16/src/engine/gpu/pick.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Area the id pass draws: the window plus halo, or the whole canvas.</span>
    <span class="k">pub</span> <span class="k">fn</span> view_for(&amp;<span class="k">self</span>, at: Option&lt;(u32, u32)&gt;, size: (u32, u32)) -&gt; PickView {
        <span class="k">match</span> at {
            Some(at) =&gt; <span class="k">self</span>.window(at, size).view(size),
            None =&gt; PickView::whole(size),
        }
    }

    <span class="c">/// True while a pick is requested or in flight.</span></code></pre></div>
<p>Replaces the 2 lines from <code>size: (u32, u32),</code> in <code>fn begin_pass</code> of <code>lessons/16/src/engine/gpu/pick.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Open the id pass over textures sized to \`view\`, cleared.</span>
    <span class="k">pub</span> <span class="k">fn</span> begin_pass&lt;'a&gt;(
        &amp;'a <span class="k">mut</span> <span class="k">self</span>,
        ctx: &amp;GpuCtx,
        encoder: &amp;'a <span class="k">mut</span> wgpu::CommandEncoder,
        view: PickView,
    ) -&gt; wgpu::RenderPass&lt;'a&gt; {
        <span class="k">let</span> size = (view.w, view.h);
        <span class="k">self</span>.view = view;

        <span class="c">// remake the textures when the size changed</span></code></pre></div>
<p>Replaces the 3 lines from <code>) {</code> in <code>impl Picker</code> of <code>lessons/16/src/engine/gpu/pick.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Copy the window around \`at\` into the readback buffer.</span>
    <span class="k">pub</span> <span class="k">fn</span> copy_window(
        &amp;<span class="k">mut</span> <span class="k">self</span>,
        ctx: &amp;GpuCtx,
        encoder: &amp;<span class="k">mut</span> wgpu::CommandEncoder,
        at: (u32, u32),
        size: (u32, u32),
    ) {
        <span class="k">let</span> Some(t) = &amp;<span class="k">self</span>.targets <span class="k">else</span> { <span class="k">return</span> };
        <span class="k">let</span> win = <span class="k">self</span>.window(at, size);</code></pre></div>
<p>Replaces the 2 lines from <code>x: win.x,</code> in <code>fn copy_window</code> of <code>lessons/16/src/engine/gpu/pick.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                    x: win.x.saturating_sub(<span class="k">self</span>.view.x),
                    y: win.y.saturating_sub(<span class="k">self</span>.view.y),</code></pre></div>
<p>Replaces the <code>let key = (sub == 0, distance, object, sub);</code> line in <code>fn nearest_hit</code> of <code>lessons/16/src/engine/gpu/pick.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">let</span> face = sub == <span class="s">0</span> || sub.wrapping_sub(<span class="s">1</span>) &amp; <span class="s">0xe000_0000</span> == super::faces::FACE_TAG;
            <span class="c">// smallest tuple wins: ink first, then nearest</span>
            <span class="k">let</span> key = (face, distance, object, sub);</code></pre></div>
<h2 id="step-8-srcappinputrs">Step 8 · src/app/input.rs<a class="anchor" href="#/course/17-source-presentation#step-8-srcappinputrs" aria-label="Link to this section">#</a></h2>
<p>Input routes gestures and keyboard actions to State.</p>
<p><code>lessons/17/src/app/input.rs</code> · edit · type this</p>
<p>Added after the <code>use crate::camera::View;</code> line of <code>lessons/16/src/app/input.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> winit::event::{ElementState, MouseButton, MouseScrollDelta, TouchPhase, WindowEvent};</code></pre></div>
<p>Added after the <code>ctrl: bool,</code> line in <code>struct Input</code> of <code>lessons/16/src/app/input.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    shift: bool,</code></pre></div>
<p>Added after the <code>ctrl: false,</code> line in <code>fn new</code> of <code>lessons/16/src/app/input.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            shift: <span class="s">false</span>,</code></pre></div>
<p>Added after the <code>}</code> line in <code>fn key</code> of <code>lessons/16/src/app/input.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            Key::Character(&quot;<span class="s">o</span>&quot; | &quot;<span class="s">O</span>&quot;) =&gt; {
                state.gpu.view.show_outlines = !state.gpu.view.show_outlines
            }</code></pre></div>
<p>Added after the <code>self.orbiting = *btn == ElementState::Pressed;</code> line in <code>fn mouse</code> of <code>lessons/16/src/app/input.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                state.interacting = <span class="k">self</span>.orbiting || <span class="k">self</span>.panning;</code></pre></div>
<p>Added after the <code>self.panning = *btn == ElementState::Pressed;</code> line in <code>fn mouse</code> of <code>lessons/16/src/app/input.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                state.interacting = <span class="k">self</span>.orbiting || <span class="k">self</span>.panning;</code></pre></div>
<h2 id="step-9-srcstaters">Step 9 · src/state.rs<a class="anchor" href="#/course/17-source-presentation#step-9-srcstaters" aria-label="Link to this section">#</a></h2>
<p>State coordinates input, selection and frame requests.</p>
<p><code>lessons/17/src/state.rs</code> · edit · type this</p>
<p>Replaces the 2 lines from <code>use crate::engine::text::{TextLabel, TextPlac…</code> of <code>lessons/16/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">mod</span> cloud_query;
<span class="k">mod</span> text;</code></pre></div>
<p>Added after the <code>pub needs_frame: bool,</code> line in <code>struct State</code> of <code>lessons/16/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> interacting: bool, <span class="c">// a drag or pinch is in progress</span></code></pre></div>
<p>Added after the <code>needs_frame: true,</code> line in <code>fn new</code> of <code>lessons/16/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            interacting: <span class="s">false</span>,</code></pre></div>
<p>Replaces the <code>self.scene.texts = texts;</code> line in <code>fn set_texts</code> of <code>lessons/16/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.scene.set_texts(texts, &amp;<span class="k">mut</span> <span class="k">self</span>.gpu);</code></pre></div>
<p>Added after the <code>self.selection = SelectionMode::Object;</code> line in <code>fn clear</code> of <code>lessons/16/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.gpu.arena.source_faces.select(&amp;<span class="k">self</span>.gpu.ctx, None);</code></pre></div>
<p>Replaces the <code>self.request_selection(x, y, false);</code> line in <code>fn request_pick</code> of <code>lessons/16/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.request_selection(x, y, <span class="s">false</span>, <span class="s">false</span>);</code></pre></div>
<p>Added after the <code>self.selection = SelectionMode::Object;</code> line in <code>fn select</code> of <code>lessons/16/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.gpu.arena.source_faces.select(&amp;<span class="k">self</span>.gpu.ctx, None);</code></pre></div>
<h2 id="step-10-srcenginetextrs">Step 10 · src/engine/text.rs<a class="anchor" href="#/course/17-source-presentation#step-10-srcenginetextrs" aria-label="Link to this section">#</a></h2>
<p>Text layout retains shaped glyph positions for rendering.</p>
<p><code>lessons/17/src/engine/text.rs</code> · edit · type this</p>
<p>Replaces the 4 lines from <code>#[derive(Clone, Debug, PartialEq)]</code> of <code>lessons/16/src/engine/text.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Authored text is selectable; annotations are not.</span>
#[derive(Clone, Copy, Debug, PartialEq)]
<span class="k">pub</span> <span class="k">struct</span> TextObject {
    <span class="k">pub</span> row: u32,
    <span class="k">pub</span> selected: bool,
}

<span class="c">/// One source label, sizes in CSS pixels.</span>
#[derive(Clone, Debug, PartialEq)]
<span class="k">pub</span> <span class="k">struct</span> TextLabel {
    <span class="k">pub</span> id: u32, <span class="c">// unique per label</span>
    <span class="k">pub</span> object: Option&lt;TextObject&gt;, <span class="c">// owning object, if any</span></code></pre></div>
<p>Added after the <code>pub clip: Option&lt;[f32; 4]&gt;,</code> line in <code>struct TextLabel</code> of <code>lessons/16/src/engine/text.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">impl</span> TextLabel {
    <span class="c">/// Ink color: black when selected, else the label's own.</span>
    <span class="k">pub</span> <span class="k">fn</span> ink_color(&amp;<span class="k">self</span>) -&gt; [u8; <span class="s">4</span>] {
        <span class="k">if</span> <span class="k">self</span>.object.is_some_and(|object| object.selected) {
            [<span class="s">0</span>, <span class="s">0</span>, <span class="s">0</span>, <span class="s">255</span>]
        } <span class="k">else</span> {
            <span class="k">self</span>.color
        }
    }
}

<span class="c">/// A label with its shaped glyphs.</span></code></pre></div>
<p>Added after the <code>TextLabel {</code> line in <code>fn label</code> of <code>lessons/16/src/engine/text.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            object: None,</code></pre></div>
<h2 id="step-11-srcappmanifestrs">Step 11 · src/app/manifest.rs<a class="anchor" href="#/course/17-source-presentation#step-11-srcappmanifestrs" aria-label="Link to this section">#</a></h2>
<p>The manifest describes scene files and their placements.</p>
<p><code>lessons/17/src/app/manifest.rs</code> · edit · type this</p>
<p>Added after the <code>pub height: f64,</code> line in <code>struct TextItem</code> of <code>lessons/16/src/app/manifest.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    #[serde(default)]
    <span class="k">pub</span> camera_facing: bool, <span class="c">// always face the camera</span></code></pre></div>
<h2 id="step-12-srcappscene_textrs">Step 12 · src/app/scene_text.rs<a class="anchor" href="#/course/17-source-presentation#step-12-srcappscene_textrs" aria-label="Link to this section">#</a></h2>
<p>Scene text assigns labels their own source rows.</p>
<p><code>lessons/17/src/app/scene_text.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! Text objects belonging to the document itself, not to the interface around it.</span>
<span class="k">use</span> super::Scene;
<span class="k">use</span> <span class="k">crate</span>::engine::gpu::Gpu;
<span class="k">use</span> <span class="k">crate</span>::engine::text::{TextLabel, TextObject, TextPlacement};
<span class="k">use</span> session_rust::Xform;

<span class="c">/// One text object and its scene row.</span>
<span class="k">pub</span> <span class="k">struct</span> SceneText {
    <span class="k">pub</span> label: TextLabel, <span class="c">// what to draw</span>
    <span class="k">pub</span> row: u32,
    <span class="k">pub</span>(super) key: String,
    <span class="k">pub</span>(super) active: bool,</code></pre></div>
<p><code>lessons/17/src/app/scene_text.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>}

<span class="k">impl</span> Scene {
    <span class="c">/// Replace the manifest's texts, keeping their rows.</span>
    <span class="k">pub</span> <span class="k">fn</span> set_texts(&amp;<span class="k">mut</span> <span class="k">self</span>, texts: Vec&lt;super::super::manifest::TextItem&gt;, gpu: &amp;<span class="k">mut</span> Gpu) {
        <span class="c">// retire the old manifest texts</span>
        <span class="k">for</span> text <span class="k">in</span> &amp;<span class="k">mut</span> <span class="k">self</span>.texts {
            <span class="k">if</span> text.key.starts_with(&quot;<span class="s">manifest-text/</span>&quot;) {
                text.active = <span class="s">false</span>;
            }
        }

        <span class="k">for</span> (index, text) <span class="k">in</span> texts.into_iter().enumerate() {
            <span class="k">let</span> placement = <span class="k">if</span> text.camera_facing {
                TextPlacement::WorldBillboard {
                    world: text.at,
                    world_height: text.height,
                }
            } <span class="k">else</span> {
                TextPlacement::WorldPlane {
                    world: text.at,
                    right: text.right,
                    up: text.up,
                    world_height: text.height,
                }
            };
            <span class="k">self</span>.register_text(
                format!(&quot;<span class="s">manifest-text/</span>{<span class="s">index</span>}&quot;),
                TextLabel {
                    id: <span class="s">0</span>,
                    object: None,
                    text: text.text,
                    font_size: <span class="s">18</span>.<span class="s">0</span>,
                    line_height: <span class="s">26</span>.<span class="s">0</span>,
                    color: [<span class="s">255</span>; <span class="s">4</span>],
                    placement,
                    clip: None,
                },
                <span class="s">true</span>,
            );
        }

        <span class="k">self</span>.upload_to(gpu);
        <span class="k">self</span>.restore_text_visibility(gpu);
    }

    <span class="c">/// Add a document title as a text object.</span>
    <span class="k">pub</span> <span class="k">fn</span> set_document_title(&amp;<span class="k">mut</span> <span class="k">self</span>, label: TextLabel, gpu: &amp;<span class="k">mut</span> Gpu) {
        <span class="k">self</span>.register_text(
            format!(&quot;<span class="s">document-title/</span>{}&quot;, <span class="k">self</span>.docs.len() - <span class="s">1</span>),
            label,
            <span class="s">true</span>,
        );
        <span class="k">self</span>.upload_to(gpu);
        <span class="k">self</span>.restore_text_visibility(gpu);</code></pre></div>
<p><code>lessons/17/src/app/scene_text.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    }

    <span class="c">/// Reuse the row for this key, else append one.</span>
    <span class="k">pub</span>(super) <span class="k">fn</span> register_text(&amp;<span class="k">mut</span> <span class="k">self</span>, key: String, <span class="k">mut</span> label: TextLabel, active: bool) {
        <span class="k">for</span> text <span class="k">in</span> &amp;<span class="k">mut</span> <span class="k">self</span>.texts {
            <span class="k">if</span> text.key == key {
                label.id = text.row + <span class="s">1</span>;
                label.object = Some(TextObject {
                    row: text.row,
                    selected: <span class="s">false</span>,
                });
                text.label = label;
                text.active = active;
                <span class="k">return</span>;
            }
        }

        <span class="k">let</span> row = <span class="k">self</span>.push_row(usize::MAX, &amp;key, Xform::identity(), <span class="s">0</span>);
        label.id = row + <span class="s">1</span>;</code></pre></div>
<p><code>lessons/17/src/app/scene_text.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        label.object = Some(TextObject {
            row,
            selected: <span class="s">false</span>,
        });
        <span class="k">self</span>.texts.push(SceneText {
            label,
            row,
            key,
            active,
        });
    }

    <span class="c">/// Hide inactive and hidden texts on the GPU.</span>
    <span class="k">pub</span>(super) <span class="k">fn</span> restore_text_visibility(&amp;<span class="k">self</span>, gpu: &amp;<span class="k">mut</span> Gpu) {
        <span class="k">for</span> text <span class="k">in</span> &amp;<span class="k">self</span>.texts {
            <span class="k">let</span> hidden = <span class="k">self</span>
                .identity_of(text.row)
                .is_some_and(|id| <span class="k">self</span>.hidden.contains(&amp;id));
            gpu.set_hidden(text.row, !text.active || hidden);
        }
    }

    <span class="c">/// The active text on \`row\`.</span>
    #[expect(
        clippy::manual_find,
        reason = &quot;<span class="s">Scene identity lookup deliberately uses explicit control flow</span>&quot;
    )]
    <span class="k">pub</span> <span class="k">fn</span> text_at(&amp;<span class="k">self</span>, row: u32) -&gt; Option&lt;&amp;SceneText&gt; {
        <span class="k">for</span> text <span class="k">in</span> &amp;<span class="k">self</span>.texts {
            <span class="k">if</span> text.active &amp;&amp; text.row == row {
                <span class="k">return</span> Some(text);
            }
        }

        None
    }

    <span class="c">/// Every visible text label, with its selection flag.</span>
    <span class="k">pub</span> <span class="k">fn</span> visible_texts(&amp;<span class="k">self</span>) -&gt; Vec&lt;TextLabel&gt; {
        <span class="k">let</span> <span class="k">mut</span> labels = Vec::new();

        <span class="k">for</span> text <span class="k">in</span> &amp;<span class="k">self</span>.texts {
            <span class="k">if</span> !text.active
                || <span class="k">self</span>
                    .identity_of(text.row)
                    .is_some_and(|id| <span class="k">self</span>.hidden.contains(&amp;id))
            {
                <span class="k">continue</span>;
            }

            <span class="k">let</span> <span class="k">mut</span> label = text.label.clone();
            label.object = Some(TextObject {
                row: text.row,
                selected: <span class="k">self</span>.selected == Some(text.row),
            });
            labels.push(label);
        }

        labels
    }
}</code></pre></div>
<h2 id="step-13-srcappsceners">Step 13 · src/app/scene.rs<a class="anchor" href="#/course/17-source-presentation#step-13-srcappsceners" aria-label="Link to this section">#</a></h2>
<p>The scene owns source documents and maps their identities to GPU rows.</p>
<p><code>lessons/17/src/app/scene.rs</code> · edit · type this</p>
<p>Added at the top of <code>lessons/16/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! The document the viewer shows: the objects, the rows they occupy on the GPU, and what is currently selected.</span>
#[path = &quot;<span class="s">scene_text.rs</span>&quot;]
<span class="k">mod</span> text;
<span class="k">pub</span> <span class="k">use</span> text::SceneText;</code></pre></div>
<p>Replaces the <code>pub texts: Vec&lt;super::manifest::TextItem&gt;,</code> line in <code>struct Scene</code> of <code>lessons/16/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> texts: Vec&lt;SceneText&gt;,</code></pre></div>
<p>Added after the <code>let docs = std::mem::take(&amp;mut self.docs);</code> line in <code>fn rebuild</code> of <code>lessons/16/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> texts = std::mem::take(&amp;<span class="k">mut</span> <span class="k">self</span>.texts);</code></pre></div>
<p>Replaces the 10 lines from <code>self.add_file(FileDoc {</code> in <code>fn rebuild</code> of <code>lessons/16/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">self</span>.add_file(d);
        }

        <span class="k">for</span> text <span class="k">in</span> texts {
            <span class="k">self</span>.register_text(text.key, text.label, text.active);
        }

        <span class="k">self</span>.upload_to(gpu);
        <span class="k">self</span>.restore_text_visibility(gpu);</code></pre></div>
<p>Added after the <code>pub fn object_name(&amp;self, row: u32) -&gt; &amp;str {</code> line in <code>impl Scene</code> of <code>lessons/16/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">if</span> <span class="k">let</span> Some(text) = <span class="k">self</span>.text_at(row) {
            <span class="k">return</span> &amp;text.label.text;
        }</code></pre></div>
<h2 id="step-14-srcstatetextrs">Step 14 · src/state/text.rs<a class="anchor" href="#/course/17-source-presentation#step-14-srcstatetextrs" aria-label="Link to this section">#</a></h2>
<p>State updates annotations when selection changes.</p>
<p><code>lessons/17/src/state/text.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> super::State;
<span class="k">use</span> <span class="k">crate</span>::app::selection::SelectionMode;
<span class="k">use</span> <span class="k">crate</span>::engine::text::{TextLabel, TextPlacement};
<span class="k">use</span> session_rust::AABB;

<span class="k">impl</span> State {
    <span class="c">/// Put the document's name above its CAD objects.</span>
    <span class="k">pub</span>(super) <span class="k">fn</span> annotate_document(&amp;<span class="k">mut</span> <span class="k">self</span>, first_row: usize) {
        <span class="k">let</span> <span class="k">mut</span> bounds = AABB::empty();

        <span class="c">// the box around the new BReps and surfaces</span>
        <span class="k">for</span> index <span class="k">in</span> first_row..<span class="k">self</span>.scene.object_count() {
            <span class="k">let</span> row = index <span class="k">as</span> u32;
            <span class="k">if</span> matches!(
                <span class="k">self</span>.scene.geometry(row),
                Some(session_rust::Geometry::BRep(_) | session_rust::Geometry::NurbsSurface(_))
            ) &amp;&amp; <span class="k">let</span> Some(object) = <span class="k">self</span>.gpu.objects.row_bounds(row)
            {
                bounds.union_with(&amp;object);
            }
        }

        <span class="k">if</span> !bounds.is_valid() {
            <span class="k">return</span>;
        }

        <span class="k">let</span> Some(document) = <span class="k">self</span>.scene.docs.last() <span class="k">else</span> {
            <span class="k">return</span>;
        };
        <span class="k">let</span> <span class="k">mut</span> anchor = label_center(&amp;bounds);
        anchor[<span class="s">2</span>] = bounds.cz + bounds.hz + bounds.diagonal() * <span class="s">0</span>.<span class="s">08</span>; <span class="c">// a little above the top</span>
        <span class="k">let</span> label = nameplate(<span class="s">1</span>, document.name.clone(), anchor);
        <span class="k">self</span>.scene.set_document_title(label, &amp;<span class="k">mut</span> <span class="k">self</span>.gpu);
    }</code></pre></div>
<p><code>lessons/17/src/state/text.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Send the scene texts plus the selection's name label to the GPU.</span>
    <span class="k">pub</span>(super) <span class="k">fn</span> update_label(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">let</span> <span class="k">mut</span> labels = <span class="k">self</span>.scene.visible_texts();

        <span class="c">// the name of the selected object, at its center</span>
        <span class="k">if</span> <span class="k">self</span>.show_selected_names
            &amp;&amp; !matches!(<span class="k">self</span>.selection, SelectionMode::Controls { .. })
            &amp;&amp; <span class="k">let</span> Some(row) = <span class="k">self</span>.scene.selected
            &amp;&amp; <span class="k">let</span> Some(bounds) = <span class="k">self</span>.gpu.objects.row_bounds(row)
        {
            labels.push(nameplate(
                <span class="s">0</span>,
                <span class="k">self</span>.scene.object_name(row).to_string(),
                label_center(&amp;bounds),
            ));
        }

        <span class="k">if</span> <span class="k">let</span> Err(error) = <span class="k">self</span>.gpu.text.set_labels(labels) {
            <span class="k">self</span>.status(&amp;format!(&quot;<span class="s">Text: </span>{<span class="s">error</span>}&quot;));
        }

        <span class="k">self</span>.include_text_bounds();
    }</code></pre></div>
<p><code>lessons/17/src/state/text.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Grow the scene box and each text row's box around the shaped text.</span>
    <span class="k">pub</span>(super) <span class="k">fn</span> include_text_bounds(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">for</span> run <span class="k">in</span> &amp;<span class="k">self</span>.gpu.text.document.runs {
            <span class="c">// the text position and size in the scene</span>
            <span class="k">let</span> (world, right, up, world_height) = <span class="k">match</span> run.label.placement {
                TextPlacement::WorldPlane {
                    world,
                    right,
                    up,
                    world_height,
                } =&gt; (world, right, up, world_height),
                TextPlacement::WorldBillboard {
                    world,
                    world_height,
                } =&gt; (world, [<span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>], [<span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>], world_height),
                TextPlacement::Nameplate { world, .. } | TextPlacement::Anchor { world, .. } =&gt; (
                    world,
                    [<span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>],
                    [<span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>],
                    <span class="k">self</span>.gpu.bounds.diagonal().max(<span class="s">1</span>.<span class="s">0</span>) * <span class="s">0</span>.<span class="s">002</span>,
                ),
                TextPlacement::Screen { .. } =&gt; <span class="k">continue</span>,
            };

            <span class="k">if</span> run.label.object.is_none() {
                <span class="k">continue</span>;
            }

            <span class="k">let</span> unit = world_height / f64::from(run.label.font_size); <span class="c">// scene units per font pixel</span>
            <span class="k">let</span> <span class="k">mut</span> width = <span class="s">0</span>.<span class="s">0f64</span>;
            <span class="k">let</span> <span class="k">mut</span> height = <span class="s">0</span>.<span class="s">0f64</span>;

            <span class="c">// the shaped text size</span>
            <span class="k">for</span> line <span class="k">in</span> run.buffer.layout_runs() {
                width = width.max(f64::from(line.line_w) * unit);
                height = height.max(f64::from(line.line_top + line.line_height) * unit);
            }

            <span class="k">let</span> vertical_padding = world_height * <span class="s">2</span>.<span class="s">0</span> / <span class="s">9</span>.<span class="s">0</span>;
            <span class="k">let</span> horizontal_padding = height * <span class="s">0</span>.<span class="s">5</span> + vertical_padding;
            <span class="k">let</span> <span class="k">mut</span> bounds = AABB::empty();

            <span class="c">// the four corners of the padded text</span>
            <span class="k">for</span> x <span class="k">in</span> [-horizontal_padding, width + horizontal_padding] {
                <span class="k">for</span> y <span class="k">in</span> [-vertical_padding, height + vertical_padding] {
                    <span class="k">let</span> <span class="k">mut</span> point = [<span class="s">0</span>.<span class="s">0f32</span>; <span class="s">3</span>];

                    <span class="k">for</span> axis <span class="k">in</span> <span class="s">0</span>..<span class="s">3</span> {
                        point[axis] = (world[axis] + right[axis] * x - up[axis] * y) <span class="k">as</span> f32;
                    }

                    <span class="k">if</span> point.into_iter().all(f32::is_finite) {
                        <span class="k">if</span> matches!(
                            run.label.placement,
                            TextPlacement::WorldPlane { .. } | TextPlacement::WorldBillboard { .. }
                        ) {
                            <span class="k">self</span>.gpu.bounds.union_with_point(
                                f64::from(point[<span class="s">0</span>]),
                                f64::from(point[<span class="s">1</span>]),
                                f64::from(point[<span class="s">2</span>]),
                            );
                        }

                        bounds.union_with_point(
                            f64::from(point[<span class="s">0</span>]),
                            f64::from(point[<span class="s">1</span>]),
                            f64::from(point[<span class="s">2</span>]),
                        );
                    }
                }
            }

            <span class="k">if</span> <span class="k">let</span> Some(object) = run.label.object {
                <span class="k">self</span>.gpu.objects.set_text_bounds(object.row, bounds);
            }
        }
    }
}</code></pre></div>
<p><code>lessons/17/src/state/text.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// The center of a box.</span>
<span class="k">fn</span> label_center(bounds: &amp;AABB) -&gt; [f64; <span class="s">3</span>] {
    [bounds.cx, bounds.cy, bounds.cz]
}

<span class="c">/// A white-on-black label; id 0 is the smaller selection name.</span>
<span class="k">fn</span> nameplate(id: u32, text: String, world: [f64; <span class="s">3</span>]) -&gt; TextLabel {
    <span class="k">let</span> scale = <span class="k">if</span> id == <span class="s">0</span> { <span class="s">0</span>.<span class="s">75</span> } <span class="k">else</span> { <span class="s">1</span>.<span class="s">0</span> };
    <span class="k">let</span> line_height = <span class="s">26</span>.<span class="s">0</span> * scale;
    <span class="k">let</span> vertical_padding = <span class="s">4</span>.<span class="s">0</span> * scale;
    <span class="c">// room for the rounded ends</span>
    <span class="k">let</span> horizontal_padding = line_height * <span class="s">0</span>.<span class="s">5</span> + vertical_padding;
    TextLabel {
        object: None,
        id,
        text,
        font_size: <span class="s">18</span>.<span class="s">0</span> * scale,
        line_height,
        color: [<span class="s">255</span>; <span class="s">4</span>],
        placement: TextPlacement::Nameplate {
            world,
            padding: [horizontal_padding, vertical_padding],
            rounded: <span class="s">true</span>,
        },
        clip: None,
    }
}</code></pre></div>
<h2 id="step-15-srcstatecloud_queryrs">Step 15 · src/state/cloud_query.rs<a class="anchor" href="#/course/17-source-presentation#step-15-srcstatecloud_queryrs" aria-label="Link to this section">#</a></h2>
<p>State coordinates asynchronous source-point queries.</p>
<p><code>lessons/17/src/state/cloud_query.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> super::State;
#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">use</span> super::{
    ControlId, FACING_UNKNOWN, GlyphPoint, GlyphRows, Pick, PickMode, SelectionMode,
    render_position,
};

<span class="k">impl</span> State {
    <span class="c">/// True while a cloud query waits for the GPU pick.</span>
    <span class="k">pub</span>(super) <span class="k">fn</span> cloud_query_awaiting_gpu(&amp;<span class="k">self</span>) -&gt; bool {
        <span class="k">match</span> &amp;<span class="k">self</span>.cloud_query {
            Some(query) =&gt; query.awaiting_gpu,
            None =&gt; <span class="s">false</span>,
        }
    }

    <span class="c">/// The streamed cloud slot of row \`parent\`.</span>
    <span class="k">pub</span>(super) <span class="k">fn</span> streamed_slot(&amp;<span class="k">self</span>, parent: u32) -&gt; Option&lt;usize&gt; {
        <span class="k">for</span> (slot, cloud) <span class="k">in</span> <span class="k">self</span>.scene.streamed.iter().enumerate() {
            <span class="k">if</span> cloud.row == parent {
                <span class="k">return</span> Some(slot);
            }
        }

        None
    }

    <span class="c">/// Drop the cloud query in flight.</span>
    <span class="k">pub</span>(super) <span class="k">fn</span> cancel_cloud_query(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">if</span> <span class="k">self</span>.cloud_query.take().is_some() {
            <span class="k">self</span>.gpu.pick.cancel();
            <span class="k">self</span>.upload_controls();
            <span class="k">self</span>.status(&quot;<span class="s">Point query cancelled because the view or selection changed</span>&quot;);
        }
    }</code></pre></div>
<p><code>lessons/17/src/state/cloud_query.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Start a click query on a streamed cloud's source points.</span>
    #[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
    <span class="k">pub</span>(super) <span class="k">fn</span> start_cloud_query(&amp;<span class="k">mut</span> <span class="k">self</span>, x: u32, y: u32) -&gt; bool {
        <span class="k">use</span> <span class="k">crate</span>::app::cloud_query::{Query, QueryView};
        <span class="c">// only in cloud control mode</span>
        <span class="k">let</span> SelectionMode::Controls {
            parent,
            cloud: <span class="s">true</span>,
            ..
        } = <span class="k">self</span>.selection
        <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="k">let</span> Some(slot) = <span class="k">self</span>.streamed_slot(parent) <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="k">self</span>.gpu.pick.cancel();
        <span class="k">self</span>.query_generation = <span class="k">self</span>.query_generation.wrapping_add(<span class="s">1</span>); <span class="c">// new query id</span>
        <span class="k">let</span> cloud = &amp;<span class="k">self</span>.scene.streamed[slot];
        <span class="c">// the matrix that puts a source point on screen</span>
        <span class="k">let</span> projection = <span class="k">self</span>
            .camera
            .view_proj_anchored(<span class="k">self</span>.aspect(), &amp;session_rust::Point::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>));
        <span class="k">let</span> scale = f64::from(<span class="k">self</span>.gpu.config.width) / <span class="k">self</span>.logical_size()[<span class="s">0</span>]; <span class="c">// device pixels per CSS pixel</span>
        <span class="k">let</span> view = QueryView {
            matrix: &amp;projection * &amp;cloud.place,
            size: [
                f64::from(<span class="k">self</span>.gpu.config.width),
                f64::from(<span class="k">self</span>.gpu.config.height),
            ],
            at: [x, y], <span class="c">// the click</span>
            radius: (<span class="k">self</span>.selection_radius_css * scale).ceil().clamp(<span class="s">1</span>.<span class="s">0</span>, <span class="s">128</span>.<span class="s">0</span>) + <span class="s">3</span>.<span class="s">5</span> * scale, <span class="c">// click tolerance plus a dot</span>
        };
        <span class="k">self</span>.cloud_query = Some(Query::new(<span class="k">self</span>.query_generation, cloud, view));
        <span class="k">self</span>.gpu.pick.start_source_query();
        <span class="k">self</span>.advance_cloud_query();
        <span class="s">true</span>
    }

    <span class="c">/// Fetch the next page of source points, or finish with the best one.</span>
    #[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
    <span class="k">fn</span> advance_cloud_query(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">let</span> Some(query) = <span class="k">self</span>.cloud_query.as_mut() <span class="k">else</span> {
            <span class="k">return</span>;
        };
        query.awaiting_gpu = <span class="s">false</span>;
        query.candidates.clear();

        <span class="c">// more pages to check</span>
        <span class="k">if</span> <span class="k">let</span> Some(page) = query.next_page() {
            <span class="k">let</span> progress = format!(
                &quot;<span class="s">Checking source points: </span>{}<span class="s"> / </span>{}<span class="s"> (display LOD remains bounded)</span>&quot;,
                query.checked, query.total
            );
            <span class="k">crate</span>::app::cloud_query::fetch_page(query, page);
            <span class="k">self</span>.status(&amp;progress);
        } <span class="k">else</span> <span class="k">if</span> <span class="k">let</span> Some(best) = query.best {
            <span class="c">// all pages checked: ask for the winner's source id</span>
            <span class="k">crate</span>::app::cloud_query::resolve_id(query, best);
            <span class="k">self</span>.status(&quot;<span class="s">All eligible source points checked; resolving original point ID…</span>&quot;);
        } <span class="k">else</span> {
            <span class="k">self</span>.cloud_query = None;
            <span class="k">self</span>.gpu.pick.cancel();
            <span class="k">self</span>.status(&quot;<span class="s">No visible source point in the selection window</span>&quot;);
        }

        <span class="k">self</span>.upload_controls();
    }</code></pre></div>
<p><code>lessons/17/src/state/cloud_query.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// One page of source points arrived: draw them for the GPU to pick.</span>
    #[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
    <span class="k">pub</span> <span class="k">fn</span> cloud_query_batch(&amp;<span class="k">mut</span> <span class="k">self</span>, batch: <span class="k">crate</span>::app::cloud_query::Batch) {
        <span class="k">let</span> Some(query) = <span class="k">self</span>.cloud_query.as_mut() <span class="k">else</span> {
            <span class="k">return</span>;
        };

        <span class="c">// an old query's answer</span>
        <span class="k">if</span> query.id != batch.query || query.cancelled.get() {
            <span class="k">return</span>;
        }

        <span class="k">let</span> (candidates, revision) = <span class="k">match</span> batch.result {
            Ok(result) =&gt; result,
            Err(error) =&gt; {
                <span class="k">self</span>.cloud_query = None;
                <span class="k">self</span>.gpu.pick.cancel();
                <span class="k">self</span>.upload_controls();
                <span class="k">self</span>.status(&amp;format!(&quot;<span class="s">Point query failed: </span>{<span class="s">error</span>}&quot;));
                <span class="k">return</span>;
            }
        };
        query.checked += batch.count;
        query.revision = revision;

        <span class="c">// nothing near the click on this page</span>
        <span class="k">if</span> candidates.is_empty() {
            <span class="k">self</span>.advance_cloud_query();
            <span class="k">return</span>;
        }

        query.candidates = candidates;
        query.awaiting_gpu = <span class="s">true</span>;
        <span class="k">let</span> parent = query.parent;
        <span class="k">let</span> at = query.view.at;
        <span class="k">let</span> scale = f64::from(<span class="k">self</span>.gpu.config.width) / <span class="k">self</span>.logical_size()[<span class="s">0</span>];
        <span class="k">let</span> query = <span class="k">self</span>.cloud_query.as_ref().unwrap();
        <span class="k">let</span> <span class="k">mut</span> glyphs = GlyphRows::default();

        <span class="c">// one dot per candidate</span>
        <span class="k">for</span> candidate <span class="k">in</span> &amp;query.candidates {
            glyphs.dots.push(GlyphPoint {
                center: render_position(candidate.position),
                radius: -<span class="s">3</span>.<span class="s">5</span> * scale <span class="k">as</span> f32, <span class="c">// pixel size</span>
                color: [<span class="s">0</span>.<span class="s">15</span>, <span class="s">0</span>.<span class="s">35</span>, <span class="s">0</span>.<span class="s">9</span>, <span class="s">1</span>.<span class="s">0</span>], <span class="c">// blue</span>
                instance_id: parent,
                facing: FACING_UNKNOWN,
                facing_ext: [candidate.local, FACING_UNKNOWN], <span class="c">// carries the point's row</span>
            });
        }

        <span class="k">self</span>.gpu.controls.reset();
        <span class="k">self</span>.gpu
            .controls
            .append(&amp;<span class="k">self</span>.gpu.ctx, &amp;<span class="k">self</span>.gpu.layouts, &amp;glyphs);
        <span class="c">// a pick frame over the dots, never shown</span>
        <span class="k">self</span>.requested = PickMode::Controls {
            parent,
            cloud: <span class="s">false</span>,
        };
        <span class="k">self</span>.gpu
            .pick
            .configure(<span class="k">self</span>.requested, <span class="k">self</span>.selection_radius_css, scale);
        <span class="k">self</span>.gpu.pick.request(at[<span class="s">0</span>], at[<span class="s">1</span>]);
        <span class="k">self</span>.needs_frame = <span class="s">true</span>;
    }</code></pre></div>
<p><code>lessons/17/src/state/cloud_query.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// The GPU picked one of this page's dots: keep it, go on.</span>
    #[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
    <span class="k">pub</span>(super) <span class="k">fn</span> apply_cloud_query_pick(&amp;<span class="k">mut</span> <span class="k">self</span>, pick: Option&lt;Pick&gt;) {
        <span class="k">let</span> Some(query) = <span class="k">self</span>.cloud_query.as_mut() <span class="k">else</span> {
            <span class="k">return</span>;
        };
        <span class="c">// the nearest so far, across pages</span>
        query.best = <span class="k">match</span> pick {
            Some(pick) <span class="k">if</span> pick.row == query.parent &amp;&amp; pick.sub &lt; query.fields.count =&gt; {
                Some(pick.sub)
            }
            _ =&gt; None,
        };
        <span class="k">self</span>.advance_cloud_query();
    }

    <span class="c">/// The winner's source id and position arrived: select it.</span>
    #[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
    <span class="k">pub</span> <span class="k">fn</span> cloud_query_resolved(&amp;<span class="k">mut</span> <span class="k">self</span>, resolved: <span class="k">crate</span>::app::cloud_query::Resolved) {
        <span class="k">let</span> Some(query) = <span class="k">self</span>.cloud_query.as_ref() <span class="k">else</span> {
            <span class="k">return</span>;
        };

        <span class="c">// an old query's answer</span>
        <span class="k">if</span> query.id != resolved.query || query.cancelled.get() {
            <span class="k">return</span>;
        }

        <span class="k">let</span> query = <span class="k">self</span>.cloud_query.take().unwrap();
        <span class="k">let</span> (source, position) = <span class="k">match</span> resolved.result {
            Ok(result) =&gt; result,
            Err(error) =&gt; {
                <span class="k">self</span>.gpu.pick.cancel();
                <span class="k">self</span>.upload_controls();
                <span class="k">self</span>.status(&amp;format!(&quot;<span class="s">Point query failed: </span>{<span class="s">error</span>}&quot;));
                <span class="k">return</span>;
            }
        };
        <span class="k">let</span> Some(best) = query.best <span class="k">else</span> { <span class="k">return</span> };
        <span class="k">let</span> id = ControlId::Point(source);
        <span class="k">self</span>.selection = SelectionMode::Controls {
            parent: query.parent,
            selected: Some(id),
            cloud: <span class="s">true</span>,
        };
        <span class="k">self</span>.controls.points = vec![<span class="k">crate</span>::app::selection::Control { id, position }]; <span class="c">// the one dot shown</span>
        <span class="k">self</span>.gpu.splat.set_point(None);
        <span class="k">self</span>.upload_controls();
        <span class="k">self</span>.status(&amp;format!(
            &quot;<span class="s">Selected source point </span>{<span class="s">source</span>}<span class="s"> (row </span>{}<span class="s">); all </span>{}<span class="s"> eligible points checked</span>&quot;,
            best, query.total
        ));
        <span class="k">self</span>.touch();
    }
}</code></pre></div>
<h2 id="step-16-srcstaters">Step 16 · src/state.rs<a class="anchor" href="#/course/17-source-presentation#step-16-srcstaters" aria-label="Link to this section">#</a></h2>
<p>State coordinates input, selection and frame requests.</p>
<p><code>lessons/17/src/state.rs</code> · edit · type this</p>
<p>Added after the <code>self.gpu.set_hidden(row, true);</code> line in <code>fn hide_selected</code> of <code>lessons/16/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.update_label();</code></pre></div>
<p>Added after the <code>self.scene.hidden.clear();</code> line in <code>fn show_all</code> of <code>lessons/16/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.update_label();</code></pre></div>
<p>Replaces the <code>PickMode::Edge =&gt; {</code> line in <code>fn apply_pick</code> of <code>lessons/16/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            PickMode::Edge | PickMode::Component =&gt; {
                <span class="c">// an edge hit wins over a face hit</span></code></pre></div>
<p>Added after the <code>self.status(&amp;format!(&quot;Edge {edge} selected&quot;));</code> line in <code>fn apply_pick</code> of <code>lessons/16/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                } <span class="k">else</span> <span class="k">if</span> <span class="k">self</span>.requested == PickMode::Component
                    &amp;&amp; <span class="k">let</span> Some(pick) = pick
                    &amp;&amp; <span class="k">let</span> Some((address, source)) =
                        <span class="k">self</span>.gpu.arena.source_faces.source(pick.row, pick.sub)
                {
                    <span class="k">self</span>.select(Some(source.parent));
                    <span class="k">self</span>.gpu.set_selected(source.parent, <span class="s">false</span>);
                    <span class="k">self</span>.gpu.selection_outline.set_selected(source.parent, <span class="s">true</span>);
                    <span class="k">self</span>.selection = SelectionMode::Face {
                        parent: source.parent,
                        face: source.face,
                    };
                    <span class="k">self</span>.gpu
                        .arena
                        .source_faces
                        .select(&amp;<span class="k">self</span>.gpu.ctx, Some(address));
                    <span class="k">self</span>.status(&amp;format!(&quot;<span class="s">Face </span>{}<span class="s"> selected</span>&quot;, source.face));</code></pre></div>
<p>Added after the <code>if let Some(message) = failure {</code> line in <code>fn render</code> of <code>lessons/16/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="c">// a lost device reloads the page</span>
            #[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
            <span class="k">if</span> <span class="k">crate</span>::app::route::recover_from_device_loss(&amp;message) {
                <span class="k">self</span>.needs_frame = <span class="s">false</span>;
                <span class="k">return</span>;
            }</code></pre></div>
<p>Added after the <code>self.last_frame_ms = now_ms;</code> line in <code>fn render</code> of <code>lessons/16/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">self</span>.gpu.performance.interacting = <span class="k">self</span>.interacting;
            <span class="k">let</span> drawn = <span class="k">self</span>.gpu.present(&amp;input); <span class="c">// encode time, None when the frame was dropped</span>

            <span class="c">// slow frames: drop to device scale 1 and no antialiasing</span>
            <span class="k">if</span> <span class="k">self</span>.gpu.performance.take_slow_interaction()
                &amp;&amp; <span class="k">crate</span>::engine::gpu::view::device_pixel_ratio() &gt; <span class="s">1</span>.<span class="s">0</span>
            {
                <span class="k">crate</span>::engine::gpu::view::reduce_for_slow_frames();
                log::warn!(
                    &quot;<span class="s">slow interaction frames; rendering at device scale 1 without antialiasing</span>&quot;
                );
                <span class="k">self</span>.status(&quot;<span class="s">Slow frames: rendering at device scale 1 without antialiasing</span>&quot;);
            }</code></pre></div>
<p>Added after the <code>}</code> line in <code>impl State</code> of <code>lessons/16/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Ask what is under a pixel: an object, an edge (Ctrl) or a face (Ctrl+Shift).</span>
    <span class="k">pub</span> <span class="k">fn</span> request_selection(&amp;<span class="k">mut</span> <span class="k">self</span>, x: u32, y: u32, edge: bool, face: bool) {</code></pre></div>
<p>Replaces the <code>let mode = if edge {</code> line in <code>fn request_selection</code> of <code>lessons/16/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> mode = <span class="k">if</span> face {
            PickMode::Component
        } <span class="k">else</span> <span class="k">if</span> edge {</code></pre></div>
<p>Added after the <code>self.selection.enable_controls(Some(parent),…</code> line in <code>fn enable_controls</code> of <code>lessons/16/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.gpu.arena.source_faces.select(&amp;<span class="k">self</span>.gpu.ctx, None);</code></pre></div>
<h2 id="step-17-srcenginegpuobjectsrs">Step 17 · src/engine/gpu/objects.rs<a class="anchor" href="#/course/17-source-presentation#step-17-srcenginegpuobjectsrs" aria-label="Link to this section">#</a></h2>
<p>The object table stores GPU rows separately from source identity.</p>
<p><code>lessons/17/src/engine/gpu/objects.rs</code> · edit · type this</p>
<p>Added after the <code>}</code> line in <code>impl InstanceTable</code> of <code>lessons/16/src/engine/gpu/objects.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Set the world box of a text row.</span>
    <span class="k">pub</span> <span class="k">fn</span> set_text_bounds(&amp;<span class="k">mut</span> <span class="k">self</span>, row: u32, bounds: AABB) {
        <span class="k">if</span> <span class="k">let</span> Some(target) = <span class="k">self</span>.world_bounds.get_mut(row <span class="k">as</span> usize) {
            *target = bounds;
        }
    }

    <span class="c">/// Scene origin in f64.</span></code></pre></div>
<h2 id="step-18-srcenginegputext_platers">Step 18 · src/engine/gpu/text_plate.rs<a class="anchor" href="#/course/17-source-presentation#step-18-srcenginegputext_platers" aria-label="Link to this section">#</a></h2>
<p>Text plates draw a backing around shaped labels.</p>
<p><code>lessons/17/src/engine/gpu/text_plate.rs</code> · edit · type this</p>
<p>Replaces the 78 lines from <code>}</code> of <code>lessons/16/src/engine/gpu/text_plate.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span>(super) depth: Option&lt;f32&gt;, <span class="c">// scene depth, or None for an overlay</span>
    <span class="k">pub</span>(super) object: Option&lt;<span class="k">crate</span>::engine::text::TextObject&gt;, <span class="c">// object it belongs to, for picks</span>
}

<span class="c">/// Every plate of the frame, in one vertex buffer.</span>
<span class="k">pub</span>(super) <span class="k">struct</span> Plates {
    vertices: GrowBuf, <span class="c">// six per plate: two triangles</span>
    pipeline: wgpu::RenderPipeline,
    id_pipeline: wgpu::RenderPipeline, <span class="c">// object ids</span>
    physical_vertices: u32, <span class="c">// vertices of depth-tested plates; overlays follow</span>
}

<span class="k">impl</span> Plates {
    <span class="c">/// Starts empty; the buffer grows with the first plates.</span>
    <span class="k">pub</span>(super) <span class="k">fn</span> new(ctx: &amp;GpuCtx, target: Target) -&gt; <span class="k">Self</span> {
        <span class="k">Self</span> {
            <span class="c">// 40 bytes per vertex; must match the shader and \`pipeline\`</span>
            vertices: GrowBuf::new(ctx, &quot;<span class="s">text.plates</span>&quot;, <span class="s">40</span>, VERTS),
            pipeline: pipeline(ctx, target, <span class="s">false</span>),
            id_pipeline: pipeline(ctx, Target::ID, <span class="s">true</span>),
            physical_vertices: <span class="s">0</span>,
        }
    }

    <span class="c">/// Rebuild the color pipeline for a new MSAA sample count.</span>
    <span class="k">pub</span>(super) <span class="k">fn</span> retarget(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, target: Target) {
        <span class="k">self</span>.pipeline = pipeline(ctx, target, <span class="s">false</span>);
    }

    <span class="c">/// Two triangles per plate, cut to its clip box.</span>
    <span class="k">pub</span>(super) <span class="k">fn</span> prepare(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, rectangles: &amp;[Rectangle], size: [u32; <span class="s">2</span>]) {
        <span class="k">self</span>.vertices.reset();
        <span class="k">let</span> <span class="k">mut</span> vertices = Vec::with_capacity(rectangles.len() * <span class="s">6</span>);
        <span class="k">self</span>.physical_vertices = <span class="s">0</span>;

        <span class="k">for</span> overlay <span class="k">in</span> [<span class="s">false</span>, <span class="s">true</span>] {
            <span class="k">for</span> rectangle <span class="k">in</span> rectangles {
                <span class="k">if</span> rectangle.depth.is_none() != overlay {
                    <span class="k">continue</span>;
                }

                <span class="c">// measure before cutting, so a cut plate keeps its true corners</span>
                <span class="k">let</span> [left, top, right, bottom] = rectangle.bounds;
                <span class="k">let</span> half = [(right - left) * <span class="s">0</span>.<span class="s">5</span>, (bottom - top) * <span class="s">0</span>.<span class="s">5</span>];
                <span class="k">let</span> center = [(left + right) * <span class="s">0</span>.<span class="s">5</span>, (top + bottom) * <span class="s">0</span>.<span class="s">5</span>];
                <span class="k">let</span> radius = <span class="k">if</span> rectangle.rounded {
                    half[<span class="s">0</span>].min(half[<span class="s">1</span>]).max(<span class="s">0</span>.<span class="s">0</span>)
                } <span class="k">else</span> {
                    <span class="s">0</span>.<span class="s">0</span>
                };
                <span class="k">let</span> left = left.max(rectangle.clip[<span class="s">0</span>]);
                <span class="k">let</span> top = top.max(rectangle.clip[<span class="s">1</span>]);
                <span class="k">let</span> right = right.min(rectangle.clip[<span class="s">2</span>]);
                <span class="k">let</span> bottom = bottom.min(rectangle.clip[<span class="s">3</span>]);

                <span class="k">if</span> right &lt;= left || bottom &lt;= top {
                    <span class="k">continue</span>;
                }

                <span class="c">// pixels to clip space: x 0..width becomes -1..1, and y flips because clip y points up</span>
                <span class="k">for</span> [x, y] <span class="k">in</span> [
                    [left, top],
                    [left, bottom],
                    [right, bottom],
                    [left, top],
                    [right, bottom],
                    [right, top],
                ] {
                    vertices.push([
                        <span class="s">2</span>.<span class="s">0</span> * x / size[<span class="s">0</span>] <span class="k">as</span> f32 - <span class="s">1</span>.<span class="s">0</span>,
                        <span class="s">1</span>.<span class="s">0</span> - <span class="s">2</span>.<span class="s">0</span> * y / size[<span class="s">1</span>] <span class="k">as</span> f32,
                        rectangle.depth.unwrap_or(<span class="s">1</span>.<span class="s">0</span>),
                        x - center[<span class="s">0</span>],
                        y - center[<span class="s">1</span>],
                        half[<span class="s">0</span>],
                        half[<span class="s">1</span>],
                        radius,
                        f32::from_bits(rectangle.object.map_or(<span class="s">0</span>, |object| object.row + <span class="s">1</span>)),
                        <span class="k">if</span> rectangle.object.is_some_and(|object| object.selected) {
                            <span class="s">1</span>.<span class="s">0</span>
                        } <span class="k">else</span> {
                            <span class="s">0</span>.<span class="s">0</span>
                        },
                    ]);
                }
            }

            <span class="k">if</span> !overlay {
                <span class="k">self</span>.physical_vertices = vertices.len() <span class="k">as</span> u32;
            }
        }

        <span class="k">self</span>.vertices.append(ctx, &amp;vertices);
    }

    <span class="c">/// Draw the depth-tested plates, or the overlay ones; returns the draw count.</span>
    <span class="k">pub</span>(super) <span class="k">fn</span> draw(&amp;<span class="k">self</span>, pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;, overlay: bool) -&gt; u32 {
        <span class="k">let</span> range = <span class="k">if</span> overlay {
            <span class="k">self</span>.physical_vertices..<span class="k">self</span>.vertices.len()
        } <span class="k">else</span> {
            <span class="s">0</span>..<span class="k">self</span>.physical_vertices
        };

        <span class="k">if</span> range.is_empty() {
            <span class="k">return</span> <span class="s">0</span>;
        }

        pass.set_pipeline(&amp;<span class="k">self</span>.pipeline);
        pass.set_vertex_buffer(<span class="s">0</span>, <span class="k">self</span>.vertices.buf.slice(..));
        pass.draw(range, <span class="s">0</span>..<span class="s">1</span>);
        <span class="s">1</span>
    }

    <span class="c">/// Draw every plate's object id; group 0 is the pick transform.</span>
    <span class="k">pub</span>(super) <span class="k">fn</span> draw_ids(
        &amp;<span class="k">self</span>,
        pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;,
        pick_transform: &amp;wgpu::BindGroup,
    ) -&gt; u32 {
        <span class="k">if</span> <span class="k">self</span>.vertices.is_empty() {
            <span class="k">return</span> <span class="s">0</span>;
        }

        pass.set_pipeline(&amp;<span class="k">self</span>.id_pipeline);
        pass.set_bind_group(<span class="s">0</span>, pick_transform, &amp;[]);
        pass.set_vertex_buffer(<span class="s">0</span>, <span class="k">self</span>.vertices.buf.slice(..));</code></pre></div>
<p>Added after the <code>self.vertices.reset();</code> line in <code>impl Plates</code> of <code>lessons/16/src/engine/gpu/text_plate.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.physical_vertices = <span class="s">0</span>;</code></pre></div>
<p>Replaces <code>fn pipeline</code> in <code>lessons/16/src/engine/gpu/text_plate.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Build the color or id pipeline; depth is read, not written.</span>
<span class="k">fn</span> pipeline(ctx: &amp;GpuCtx, target: Target, ids: bool) -&gt; wgpu::RenderPipeline {
    <span class="k">let</span> shader = ctx
        .device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(&quot;<span class="s">text plate shader</span>&quot;),
            source: wgpu::ShaderSource::Wgsl(include_str!(&quot;<span class="s">../../shaders/text_plate.wgsl</span>&quot;).into()),
        });
    <span class="k">let</span> pick = <span class="k">crate</span>::engine::gpu::frame::pick_transform_layout(ctx);
    <span class="k">let</span> id_layout = ctx
        .device
        .create_pipeline_layout(&amp;wgpu::PipelineLayoutDescriptor {
            label: Some(&quot;<span class="s">text plate ids</span>&quot;),
            bind_group_layouts: &amp;[Some(&amp;pick)],
            immediate_size: <span class="s">0</span>,
        });
    ctx.device
        .create_render_pipeline(&amp;wgpu::RenderPipelineDescriptor {
            label: Some(&quot;<span class="s">text plates</span>&quot;),
            layout: <span class="k">if</span> ids { Some(&amp;id_layout) } <span class="k">else</span> { None },
            vertex: wgpu::VertexState {
                module: &amp;shader,
                entry_point: Some(<span class="k">if</span> ids { &quot;<span class="s">vs_id</span>&quot; } <span class="k">else</span> { &quot;<span class="s">vs_main</span>&quot; }),
                buffers: &amp;[wgpu::VertexBufferLayout {
                    array_stride: <span class="s">40</span>,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &amp;wgpu::vertex_attr_array![<span class="s">0</span> =&gt; Float32x3, <span class="s">1</span> =&gt; Float32x2, <span class="s">2</span> =&gt; Float32x2, <span class="s">3</span> =&gt; Float32, <span class="s">4</span> =&gt; Uint32, <span class="s">5</span> =&gt; Float32],
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &amp;shader,
                entry_point: Some(<span class="k">if</span> ids { &quot;<span class="s">fs_id</span>&quot; } <span class="k">else</span> { &quot;<span class="s">fs_main</span>&quot; }),
                targets: &amp;[Some(wgpu::ColorTargetState {
                    format: target.format,
                    blend: <span class="k">if</span> ids { None } <span class="k">else</span> { Some(wgpu::BlendState::ALPHA_BLENDING) },</code></pre></div>
<p>Replaces the <code>depth_compare: Some(wgpu::CompareFunction::Al…</code> line in <code>fn pipeline</code> of <code>lessons/16/src/engine/gpu/text_plate.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                depth_compare: Some(wgpu::CompareFunction::GreaterEqual),</code></pre></div>
<h2 id="step-19-srcshaderstext_platewgsl">Step 19 · src/shaders/text_plate.wgsl<a class="anchor" href="#/course/17-source-presentation#step-19-srcshaderstext_platewgsl" aria-label="Link to this section">#</a></h2>
<p>Text plates draw rounded backing shapes behind labels.</p>
<p><code>lessons/17/src/shaders/text_plate.wgsl</code> · edit · type this</p>
<p>Replaces the 23 lines from <code>}</code> of <code>lessons/16/src/shaders/text_plate.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    @location(<span class="s">3</span>) @interpolate(flat) object: <span class="k">u32</span>,<span class="c"> // object row + 1, or 0</span>
    @location(<span class="s">4</span>) @interpolate(flat) selected: <span class="k">f32</span>,<span class="c"> // 1 when selected</span>
}

<span class="c">// Canvas clip space to the pick window; id pass only.</span>
@group(<span class="s">0</span>) @binding(<span class="s">0</span>) <span class="k">var</span>&lt;<span class="k">uniform</span>&gt; pick: <span class="k">mat4x4</span>&lt;<span class="k">f32</span>&gt;;

<span class="c">// Pack the vertex attributes.</span>
<span class="k">fn</span> plate_vertex(position: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;, local: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;, half_size: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;, radius: <span class="k">f32</span>,
                object: <span class="k">u32</span>, selected: <span class="k">f32</span>) -&gt; PlateVertex {
    <span class="k">var</span> out: PlateVertex;
    out.position = position;
    out.local = local;
    out.half_size = half_size;
    out.radius = radius;
    out.object = object;
    out.selected = selected;
    <span class="k">return</span> out;
}

@vertex
<span class="c">// Vertices arrive already in clip space.</span>
<span class="k">fn</span> vs_main(@location(<span class="s">0</span>) position: <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;, @location(<span class="s">1</span>) local: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;,
           @location(<span class="s">2</span>) half_size: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;, @location(<span class="s">3</span>) radius: <span class="k">f32</span>, @location(<span class="s">4</span>) object: <span class="k">u32</span>,
           @location(<span class="s">5</span>) selected: <span class="k">f32</span>) -&gt; PlateVertex {
    <span class="k">return</span> plate_vertex(<span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(position, <span class="s">1.0</span>), local, half_size, radius, object, selected);
}

@vertex
<span class="c">// The same, mapped into the pick window.</span>
<span class="k">fn</span> vs_id(@location(<span class="s">0</span>) position: <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;, @location(<span class="s">1</span>) local: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;,
         @location(<span class="s">2</span>) half_size: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;, @location(<span class="s">3</span>) radius: <span class="k">f32</span>, @location(<span class="s">4</span>) object: <span class="k">u32</span>,
         @location(<span class="s">5</span>) selected: <span class="k">f32</span>) -&gt; PlateVertex {
    <span class="k">return</span> plate_vertex(pick * <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(position, <span class="s">1.0</span>), local, half_size, radius, object, selected);
}

<span class="c">// Signed distance to the rounded plate edge, px.</span>
<span class="k">fn</span> plate_distance(in: PlateVertex) -&gt; <span class="k">f32</span> {
    <span class="k">let</span> radius = min(in.radius, min(in.half_size.x, in.half_size.y));
    <span class="k">let</span> q = abs(in.local) - in.half_size + <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(radius);
    <span class="k">return</span> length(max(q, <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>))) + min(max(q.x, q.y), <span class="s">0.0</span>) - radius;
}

@fragment
<span class="c">// Black plate, yellow when selected, soft edge.</span>
<span class="k">fn</span> fs_main(in: PlateVertex) -&gt; @location(<span class="s">0</span>) <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt; {
    <span class="k">let</span> distance = plate_distance(in);
    <span class="k">let</span> coverage = clamp(<span class="s">0.5</span> - distance / max(fwidth(distance), <span class="s">0.001</span>), <span class="s">0.0</span>, <span class="s">1.0</span>);
    <span class="k">return</span> <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(in.selected, in.selected, <span class="s">0.0</span>, coverage);
}

@fragment
<span class="c">// Object id inside the plate.</span>
<span class="k">fn</span> fs_id(in: PlateVertex) -&gt; @location(<span class="s">0</span>) <span class="k">vec2</span>&lt;<span class="k">u32</span>&gt; {
<span class="c">    // object is already row + 1; 0 = no object</span>
    <span class="k">if</span> (in.object == <span class="s">0u</span> || plate_distance(in) &gt; <span class="s">0.0</span>) {
        <span class="k">discard</span>;
    }

    <span class="k">return</span> <span class="k">vec2</span>&lt;<span class="k">u32</span>&gt;(in.object, <span class="s">0u</span>);</code></pre></div>
<h2 id="step-20-srcenginegputext_planers">Step 20 · src/engine/gpu/text_plane.rs<a class="anchor" href="#/course/17-source-presentation#step-20-srcenginegputext_planers" aria-label="Link to this section">#</a></h2>
<p>Plane text projects labels through their scene placement.</p>
<p><code>lessons/17/src/engine/gpu/text_plane.rs</code> · edit · type this</p>
<p>Added after the <code>pipeline: wgpu::RenderPipeline,</code> line in <code>struct CachedPlane</code> of <code>lessons/16/src/engine/gpu/text_plane.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    id_pipeline: wgpu::RenderPipeline, <span class="c">// object ids</span></code></pre></div>
<p>Replaces the 8 lines from <code>let pipeline = pipeline(ctx, target, &amp;layout);</code> in <code>impl Planes</code> of <code>lessons/16/src/engine/gpu/text_plane.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> id_pipeline = pipeline(ctx, Target::ID, &amp;layout, <span class="s">true</span>);
        <span class="k">let</span> pipeline = pipeline(ctx, target, &amp;layout, <span class="s">false</span>);
        <span class="k">Self</span> {
            cached: Vec::new(),
            draws: Vec::new(),
            vertices: GrowBuf::new(ctx, &quot;<span class="s">world text vertices</span>&quot;, <span class="s">64</span>, VERTS),
            layout,
            sampler,
            pipeline,
            id_pipeline,</code></pre></div>
<p>Replaces the <code>self.pipeline = pipeline(ctx, target, &amp;self.l…</code> line in <code>impl Planes</code> of <code>lessons/16/src/engine/gpu/text_plane.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.pipeline = pipeline(ctx, target, &amp;<span class="k">self</span>.layout, <span class="s">false</span>);</code></pre></div>
<p>Replaces the <code>pass.set_pipeline(&amp;self.pipeline);</code> line in <code>impl Planes</code> of <code>lessons/16/src/engine/gpu/text_plane.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.draw_run(pass, &amp;<span class="k">self</span>.pipeline)
    }

    <span class="c">/// Draw every label's object id; group 1 is the pick transform.</span>
    <span class="k">pub</span>(super) <span class="k">fn</span> draw_ids(
        &amp;<span class="k">self</span>,
        pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;,
        pick_transform: &amp;wgpu::BindGroup,
    ) -&gt; u32 {
        pass.set_bind_group(<span class="s">1</span>, pick_transform, &amp;[]);
        <span class="k">self</span>.draw_run(pass, &amp;<span class="k">self</span>.id_pipeline)
    }

    <span class="c">/// One quad per label with \`pipeline\`; returns the draw count.</span>
    <span class="k">fn</span> draw_run(&amp;<span class="k">self</span>, pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;, pipeline: &amp;wgpu::RenderPipeline) -&gt; u32 {
        <span class="k">if</span> <span class="k">self</span>.draws.is_empty() {
            <span class="k">return</span> <span class="s">0</span>;
        }

        pass.set_pipeline(pipeline);</code></pre></div>
<p>Replaces the 4 lines from <code>bounds[0] -= 2;</code> in <code>fn rasterize</code> of <code>lessons/16/src/engine/gpu/text_plane.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">// padding around the glyphs</span>
    <span class="k">let</span> vertical_padding = (run.label.font_size * (<span class="s">2</span>.<span class="s">0</span> / <span class="s">9</span>.<span class="s">0</span>) * scale).ceil() <span class="k">as</span> i32;
    <span class="k">let</span> horizontal_padding = (bounds[<span class="s">3</span>] - bounds[<span class="s">1</span>] + <span class="s">2</span> * vertical_padding + <span class="s">1</span>) / <span class="s">2</span>;
    bounds[<span class="s">0</span>] -= horizontal_padding;
    bounds[<span class="s">1</span>] -= vertical_padding;
    bounds[<span class="s">2</span>] += horizontal_padding;
    bounds[<span class="s">3</span>] += vertical_padding;</code></pre></div>
<p>Replaces the <code>vertices: &amp;mut Vec&lt;[f32; 14]&gt;,</code> line in <code>fn append_quad</code> of <code>lessons/16/src/engine/gpu/text_plane.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    vertices: &amp;<span class="k">mut</span> Vec&lt;[f32; 16]&gt;,</code></pre></div>
<p>Replaces the <code>let value = f32::from(label.color[index]) / 2…</code> line in <code>fn append_quad</code> of <code>lessons/16/src/engine/gpu/text_plane.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> value = f32::from(label.ink_color()[index]) / <span class="s">255</span>.<span class="s">0</span>;</code></pre></div>
<p>Replaces the 26 lines from <code>clip[0], clip[1], clip[2], clip[3], u, v, col…</code> in <code>fn append_quad</code> of <code>lessons/16/src/engine/gpu/text_plane.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            clip[<span class="s">0</span>],
            clip[<span class="s">1</span>],
            clip[<span class="s">2</span>],
            clip[<span class="s">3</span>],
            u,
            v,
            color[<span class="s">0</span>],
            color[<span class="s">1</span>],
            color[<span class="s">2</span>],
            color[<span class="s">3</span>],
            bounds[<span class="s">0</span>],
            bounds[<span class="s">1</span>],
            bounds[<span class="s">2</span>],
            bounds[<span class="s">3</span>],
            f32::from_bits(label.object.map_or(<span class="s">0</span>, |object| object.row + <span class="s">1</span>)),
            <span class="k">if</span> label.object.is_some_and(|object| object.selected) {
                <span class="s">1</span>.<span class="s">0</span>
            } <span class="k">else</span> {
                <span class="s">0</span>.<span class="s">0</span>
            },
        ]);
    }
}

<span class="c">/// Build the color or id pipeline.</span>
<span class="k">fn</span> pipeline(
    ctx: &amp;GpuCtx,
    target: Target,
    layout: &amp;wgpu::BindGroupLayout,
    ids: bool,
) -&gt; wgpu::RenderPipeline {
    <span class="k">let</span> shader = ctx
        .device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(&quot;<span class="s">world text shader</span>&quot;),
            source: wgpu::ShaderSource::Wgsl(include_str!(&quot;<span class="s">../../shaders/text_plane.wgsl</span>&quot;).into()),
        });
    <span class="k">let</span> pick = <span class="k">crate</span>::engine::gpu::frame::pick_transform_layout(ctx);
    <span class="k">let</span> colour_groups = [Some(layout)];
    <span class="k">let</span> id_groups = [Some(layout), Some(&amp;pick)];
    <span class="k">let</span> pipeline_layout = ctx
        .device
        .create_pipeline_layout(&amp;wgpu::PipelineLayoutDescriptor {
            label: Some(&quot;<span class="s">world text</span>&quot;),
            bind_group_layouts: <span class="k">if</span> ids { &amp;id_groups } <span class="k">else</span> { &amp;colour_groups },
            immediate_size: <span class="s">0</span>,
        });
    ctx.device.create_render_pipeline(&amp;wgpu::RenderPipelineDescriptor {
        label: Some(&quot;<span class="s">world text</span>&quot;), layout: Some(&amp;pipeline_layout),
        vertex: wgpu::VertexState { module: &amp;shader, entry_point: Some(<span class="k">if</span> ids { &quot;<span class="s">vs_id</span>&quot; } <span class="k">else</span> { &quot;<span class="s">vs_main</span>&quot; }), buffers: &amp;[wgpu::VertexBufferLayout { array_stride: <span class="s">64</span>, step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &amp;wgpu::vertex_attr_array![<span class="s">0</span> =&gt; Float32x4, <span class="s">1</span> =&gt; Float32x2, <span class="s">2</span> =&gt; Float32x4, <span class="s">3</span> =&gt; Float32x4, <span class="s">4</span> =&gt; Uint32, <span class="s">5</span> =&gt; Float32] }], compilation_options: Default::default() },
        fragment: Some(wgpu::FragmentState { module: &amp;shader, entry_point: Some(<span class="k">if</span> ids { &quot;<span class="s">fs_id</span>&quot; } <span class="k">else</span> { &quot;<span class="s">fs_main</span>&quot; }), targets: &amp;[Some(wgpu::ColorTargetState { format: target.format, blend: <span class="k">if</span> ids { None } <span class="k">else</span> { Some(wgpu::BlendState::ALPHA_BLENDING) }, write_mask: wgpu::ColorWrites::ALL })], compilation_options: Default::default() }),</code></pre></div>
<p>Added after the <code>let mut label = TextLabel {</code> line in <code>fn fixed_plane_obeys_solid_depth_orientation_cache_and_release</code> of <code>lessons/16/src/engine/gpu/text_plane.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            object: None,</code></pre></div>
<p>Added after the <code>assert_eq!(gpu.text.stats.world_plane_rasteri…</code> line in <code>fn fixed_plane_obeys_solid_depth_orientation_cache_and_release</code> of <code>lessons/16/src/engine/gpu/text_plane.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        label.object = Some(<span class="k">crate</span>::engine::text::TextObject {
            row: <span class="s">0</span>,
            selected: <span class="s">true</span>,
        });
        gpu.text.set_labels(vec![label.clone()]).unwrap();
        <span class="k">let</span> selected = gpu.render_offscreen(&amp;input);
        <span class="k">let</span> yellow = selected
            .chunks_exact(<span class="s">4</span>)
            .filter(|pixel| pixel[<span class="s">0</span>] &gt; <span class="s">240</span> &amp;&amp; pixel[<span class="s">1</span>] &gt; <span class="s">240</span> &amp;&amp; pixel[<span class="s">2</span>] &lt; <span class="s">8</span>)
            .count();
        <span class="k">let</span> black_ink = selected
            .chunks_exact(<span class="s">4</span>)
            .filter(|pixel| pixel[<span class="s">0</span>] &lt; <span class="s">8</span> &amp;&amp; pixel[<span class="s">1</span>] &lt; <span class="s">8</span> &amp;&amp; pixel[<span class="s">2</span>] &lt; <span class="s">8</span>)
            .count();
        assert!(
            yellow &gt; <span class="s">500</span> &amp;&amp; black_ink &gt; <span class="s">40</span>,
            &quot;<span class="s">selected fixed text has a yellow backing and black glyphs</span>&quot;
        );
        assert_eq!(
            gpu.text.stats.world_plane_rasterizations, rasterizations,
            &quot;<span class="s">selection reuses the coverage texture</span>&quot;
        );
        label.object = None;
        gpu.text.set_labels(vec![label.clone()]).unwrap();
        assert_eq!(
            front,
            gpu.render_offscreen(&amp;input),
            &quot;<span class="s">deselect restores the original text colors</span>&quot;
        );</code></pre></div>
<h2 id="step-21-srcshaderstext_planewgsl">Step 21 · src/shaders/text_plane.wgsl<a class="anchor" href="#/course/17-source-presentation#step-21-srcshaderstext_planewgsl" aria-label="Link to this section">#</a></h2>
<p>Plane labels project shaped glyphs onto their scene plane.</p>
<p><code>lessons/17/src/shaders/text_plane.wgsl</code> · edit · type this</p>
<p>Replaces the 12 lines from <code>}</code> of <code>lessons/16/src/shaders/text_plane.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    @location(<span class="s">3</span>) @interpolate(flat) object: <span class="k">u32</span>,<span class="c"> // object row + 1, or 0</span>
    @location(<span class="s">4</span>) @interpolate(flat) selected: <span class="k">f32</span>,<span class="c"> // 1 when selected</span>
}

@group(<span class="s">0</span>) @binding(<span class="s">0</span>) <span class="k">var</span> coverage_texture: texture_2d&lt;<span class="k">f32</span>&gt;;<span class="c"> // Unorm: byte 255 reads as 1.0</span>
@group(<span class="s">0</span>) @binding(<span class="s">1</span>) <span class="k">var</span> coverage_sampler: sampler;

<span class="c">// Canvas clip space to the pick window; id pass only.</span>
@group(<span class="s">1</span>) @binding(<span class="s">0</span>) <span class="k">var</span>&lt;<span class="k">uniform</span>&gt; pick: <span class="k">mat4x4</span>&lt;<span class="k">f32</span>&gt;;

<span class="c">// Pack the vertex attributes.</span>
<span class="k">fn</span> plane_vertex(position: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;, uv: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;, color: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;, clip: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,
                object: <span class="k">u32</span>, selected: <span class="k">f32</span>) -&gt; Vertex {
    <span class="k">var</span> out: Vertex;
    out.position = position; out.uv = uv; out.color = color; out.clip = clip;
    out.object = object; out.selected = selected;
    <span class="k">return</span> out;
}

@vertex
<span class="c">// Vertices arrive already in clip space.</span>
<span class="k">fn</span> vs_main(@location(<span class="s">0</span>) position: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;, @location(<span class="s">1</span>) uv: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;,
           @location(<span class="s">2</span>) color: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;, @location(<span class="s">3</span>) clip: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,
           @location(<span class="s">4</span>) object: <span class="k">u32</span>, @location(<span class="s">5</span>) selected: <span class="k">f32</span>) -&gt; Vertex {
    <span class="k">return</span> plane_vertex(position, uv, color, clip, object, selected);
}

@vertex
<span class="c">// The same, mapped into the pick window.</span>
<span class="k">fn</span> vs_id(@location(<span class="s">0</span>) position: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;, @location(<span class="s">1</span>) uv: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;,
         @location(<span class="s">2</span>) color: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;, @location(<span class="s">3</span>) clip: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,
         @location(<span class="s">4</span>) object: <span class="k">u32</span>, @location(<span class="s">5</span>) selected: <span class="k">f32</span>) -&gt; Vertex {
    <span class="k">return</span> plane_vertex(pick * position, uv, color, clip, object, selected);
}

<span class="c">// Signed distance to the rounded plate edge, in texture pixels.</span>
<span class="k">fn</span> plate_distance(uv: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;) -&gt; <span class="k">f32</span> {
    <span class="k">let</span> half_size = <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(textureDimensions(coverage_texture)) * <span class="s">0.5</span>;
    <span class="k">let</span> radius = min(half_size.x, half_size.y);
    <span class="k">let</span> q = abs((uv * <span class="s">2.0</span> - <span class="s">1.0</span>) * half_size) - half_size + radius;
    <span class="k">return</span> length(max(q, <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>))) + min(max(q.x, q.y), <span class="s">0.0</span>) - radius;
}

<span class="c">// Glyph color over a black plate, yellow when selected.</span></code></pre></div>
<p>Replaces the 3 lines from <code>return vec4&lt;f32&gt;(in.color.rgb * coverage, in.…</code> in <code>fn fs_main</code> of <code>lessons/16/src/shaders/text_plane.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">let</span> distance = plate_distance(in.uv);
    <span class="k">let</span> plate_coverage = clamp(<span class="s">0.5</span> - distance / max(fwidth(distance), <span class="s">0.001</span>), <span class="s">0.0</span>, <span class="s">1.0</span>);
    <span class="k">let</span> background = <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(in.selected, in.selected, <span class="s">0.0</span>);
    <span class="k">return</span> <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(mix(background, in.color.rgb, coverage), in.color.a * plate_coverage);
}

@fragment
<span class="c">// Object id inside the plate.</span>
<span class="k">fn</span> fs_id(in: Vertex) -&gt; @location(<span class="s">0</span>) <span class="k">vec2</span>&lt;<span class="k">u32</span>&gt; {
    <span class="k">if</span> (in.object == <span class="s">0u</span> || in.position.x &lt; in.clip.x || in.position.y &lt; in.clip.y || in.position.x &gt;= in.clip.z || in.position.y &gt;= in.clip.w) {
        <span class="k">discard</span>;
    }

    <span class="k">if</span> (plate_distance(in.uv) &gt; <span class="s">0.0</span>) {
        <span class="k">discard</span>;
    }

    <span class="k">return</span> <span class="k">vec2</span>&lt;<span class="k">u32</span>&gt;(in.object, <span class="s">0u</span>);</code></pre></div>
<h2 id="step-22-srcenginegputextrs">Step 22 · src/engine/gpu/text.rs<a class="anchor" href="#/course/17-source-presentation#step-22-srcenginegputextrs" aria-label="Link to this section">#</a></h2>
<p>The GPU text owner coordinates glyphs and label backgrounds.</p>
<p><code>lessons/17/src/engine/gpu/text.rs</code> · edit · type this</p>
<p>Replaces the <code>if let Some(rectangle) = center_nameplate(run…</code> line in <code>fn prepare</code> of <code>lessons/16/src/engine/gpu/text.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">if</span> <span class="k">let</span> Some(rectangle) = text_rectangle(run, &amp;<span class="k">mut</span> placed, frame, scale) {</code></pre></div>
<p>Replaces the 4 lines from <code>run.label.color[0],</code> in <code>fn prepare</code> of <code>lessons/16/src/engine/gpu/text.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                    run.label.ink_color()[<span class="s">0</span>],
                    run.label.ink_color()[<span class="s">1</span>],
                    run.label.ink_color()[<span class="s">2</span>],
                    run.label.ink_color()[<span class="s">3</span>],</code></pre></div>
<p>Replaces <code>fn draw</code> in <code>lessons/16/src/engine/gpu/text.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Draw object ids of planes and plates.</span>
    <span class="k">pub</span> <span class="k">fn</span> draw_ids(
        &amp;<span class="k">self</span>,
        pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;,
        pick_transform: &amp;wgpu::BindGroup,
    ) -&gt; u32 {
        <span class="k">self</span>.planes.draw_ids(pass, pick_transform) + <span class="k">self</span>.plates.draw_ids(pass, pick_transform)
    }

    <span class="c">/// Draw planes, anchored plates and text, then overlay plates and text.</span>
    <span class="k">pub</span> <span class="k">fn</span> draw(&amp;<span class="k">self</span>, pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;) -&gt; u32 {
        <span class="k">let</span> <span class="k">mut</span> draws = <span class="k">self</span>.planes.draw(pass);
        draws += <span class="k">self</span>.plates.draw(pass, <span class="s">false</span>);</code></pre></div>
<p>Replaces the <code>draws += self.plates.draw(pass);</code> line in <code>fn draw</code> of <code>lessons/16/src/engine/gpu/text.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        draws += <span class="k">self</span>.plates.draw(pass, <span class="s">true</span>);</code></pre></div>
<p>Replaces <code>fn center_nameplate</code> in <code>lessons/16/src/engine/gpu/text.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// The background rectangle of a label, if it has one.</span>
<span class="k">fn</span> text_rectangle(
    run: &amp;TextRun,
    placed: &amp;<span class="k">mut</span> PlacedText,
    frame: &amp;TextFrame,
    scale: f32,
) -&gt; Option&lt;plate::Rectangle&gt; {
    <span class="k">let</span> (<span class="k">mut</span> padding, rounded, centered) = <span class="k">match</span> run.label.placement {
        TextPlacement::Nameplate {
            padding, rounded, ..
        } =&gt; (padding, rounded, <span class="s">true</span>),
        _ <span class="k">if</span> run.label.object.is_some() =&gt; ([<span class="s">0</span>.<span class="s">0</span>, run.label.font_size * <span class="s">2</span>.<span class="s">0</span> / <span class="s">9</span>.<span class="s">0</span>], <span class="s">true</span>, <span class="s">false</span>),
        _ =&gt; <span class="k">return</span> None,</code></pre></div>
<p>Replaces the 2 lines from <code>placed.left -= width * scale * 0.5;</code> in <code>fn center_nameplate</code> of <code>lessons/16/src/engine/gpu/text.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">if</span> rounded {
        <span class="c">// round caps need a full half-height at each end</span>
        padding[<span class="s">0</span>] = padding[<span class="s">0</span>].max((bottom - top) * <span class="s">0</span>.<span class="s">5</span> + padding[<span class="s">1</span>]);
    }

    <span class="k">let</span> raster = placed.scale;

    <span class="c">// nameplates sit centered on their anchor</span>
    <span class="k">if</span> centered {
        placed.left -= width * raster * <span class="s">0</span>.<span class="s">5</span>;
        placed.top -= (top + bottom) * raster * <span class="s">0</span>.<span class="s">5</span>;
    }</code></pre></div>
<p><code>lessons/17/src/engine/gpu/text.rs</code> · edit · type this</p>
<p>Replaces the 7 lines from <code>placed.left - padding[0] * scale,</code> in <code>fn center_nameplate</code> of <code>lessons/16/src/engine/gpu/text.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            placed.left - padding[<span class="s">0</span>] * raster,
            placed.top + (top - padding[<span class="s">1</span>]) * raster,
            placed.left + (width + padding[<span class="s">0</span>]) * raster,
            placed.top + (bottom + padding[<span class="s">1</span>]) * raster,
        ],
        clip: bounds,
        rounded,
        depth: placed.depth,
        object: run.label.object,</code></pre></div>
<p>Added after the <code>let label = TextLabel {</code> line in <code>fn logical_to_physical_scale_is_applied_once</code> of <code>lessons/16/src/engine/gpu/text.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                object: None,</code></pre></div>
<p>Added after the <code>let mut label = TextLabel {</code> line in <code>fn scene_anchor_preserves_rebased_depth_and_culls_near_plane</code> of <code>lessons/16/src/engine/gpu/text.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            object: None,</code></pre></div>
<p>Added after the <code>let mut label = TextLabel {</code> line in <code>fn nameplate_center_padding_clip_and_scale_share_one_coordinate_system</code> of <code>lessons/16/src/engine/gpu/text.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            object: None,</code></pre></div>
<p>Replaces the <code>let rectangle = center_nameplate(run, &amp;mut pl…</code> line in <code>fn nameplate_center_padding_clip_and_scale_share_one_coordinate_system</code> of <code>lessons/16/src/engine/gpu/text.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">let</span> rectangle = text_rectangle(run, &amp;<span class="k">mut</span> placed, &amp;frame, scale <span class="k">as</span> f32)</code></pre></div>
<p>Replaces the 3 lines from <code>center_nameplate(run, &amp;mut placed, &amp;frame, 2.0)</code> in <code>fn nameplate_center_padding_clip_and_scale_share_one_coordinate_system</code> of <code>lessons/16/src/engine/gpu/text.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            text_rectangle(run, &amp;<span class="k">mut</span> placed, &amp;frame, <span class="s">2</span>.<span class="s">0</span>).unwrap().clip,</code></pre></div>
<p>Added after the <code>let label = TextLabel {</code> line in <code>fn nameplate_has_black_background_white_ink_centering_and_clean_release</code> of <code>lessons/16/src/engine/gpu/text.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            object: None,</code></pre></div>
<p>Replaces the <code>gpu.text.set_labels(vec![label]).unwrap();</code> line in <code>fn nameplate_has_black_background_white_ink_centering_and_clean_release</code> of <code>lessons/16/src/engine/gpu/text.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        gpu.text.set_labels(vec![label.clone()]).unwrap();</code></pre></div>
<p>Added after the <code>);</code> line in <code>fn nameplate_has_black_background_white_ink_centering_and_clean_release</code> of <code>lessons/16/src/engine/gpu/text.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> <span class="k">mut</span> selected = label.clone();
        selected.object = Some(<span class="k">crate</span>::engine::text::TextObject {
            row: <span class="s">0</span>,
            selected: <span class="s">true</span>,
        });
        gpu.text.set_labels(vec![selected]).unwrap();
        <span class="k">let</span> selected_pixels = gpu.render_offscreen(&amp;input);
        <span class="k">let</span> yellow = selected_pixels
            .chunks_exact(<span class="s">4</span>)
            .filter(|pixel| pixel[<span class="s">0</span>] &gt; <span class="s">240</span> &amp;&amp; pixel[<span class="s">1</span>] &gt; <span class="s">240</span> &amp;&amp; pixel[<span class="s">2</span>] &lt; <span class="s">8</span>)
            .count();
        <span class="k">let</span> black_ink = selected_pixels
            .chunks_exact(<span class="s">4</span>)
            .filter(|pixel| pixel[<span class="s">0</span>] &lt; <span class="s">8</span> &amp;&amp; pixel[<span class="s">1</span>] &lt; <span class="s">8</span> &amp;&amp; pixel[<span class="s">2</span>] &lt; <span class="s">8</span>)
            .count();
        assert!(
            yellow &gt; <span class="s">600</span> &amp;&amp; black_ink &gt; <span class="s">40</span>,
            &quot;<span class="s">selected source text fills the plate yellow and glyphs black</span>&quot;
        );
        assert_eq!(
            white_pixels(&amp;selected_pixels),
            <span class="s">0</span>,
            &quot;<span class="s">selected source glyphs are no longer white</span>&quot;
        );
        gpu.text.set_labels(vec![label]).unwrap();
        assert_eq!(
            pixels,
            gpu.render_offscreen(&amp;input),
            &quot;<span class="s">unselected annotation colors are restored exactly</span>&quot;
        );</code></pre></div>
<p>Replaces the <code>assert_eq!(gpu.text.stats.nameplate_capacity_…</code> line in <code>fn nameplate_has_black_background_white_ink_centering_and_clean_release</code> of <code>lessons/16/src/engine/gpu/text.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        assert_eq!(gpu.text.stats.nameplate_capacity_bytes, <span class="s">40</span>);</code></pre></div>
<p>Added after the <code>let mut label = TextLabel {</code> line in <code>fn actual_glyph_coverage_obeys_depth_clip_motion_and_release</code> of <code>lessons/16/src/engine/gpu/text.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            object: None,</code></pre></div>
<p>Replaces this block of <code>lessons/16/src/engine/gpu/text.rs</code>:</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> <span class="k">mut</span> label = TextLabel {
            id: <span class="s">1</span>,</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            object: None,</code></pre></div>
<h2 id="step-23-srcappinspectionrs">Step 23 · src/app/inspection.rs<a class="anchor" href="#/course/17-source-presentation#step-23-srcappinspectionrs" aria-label="Link to this section">#</a></h2>
<p>Inspection reports retained resources and source information.</p>
<p><code>lessons/17/src/app/inspection.rs</code> · edit · type this</p>
<p>Added after the <code>&quot;samples&quot;: state.gpu.targets.samples,</code> line in <code>fn publish</code> of <code>lessons/16/src/app/inspection.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        &quot;<span class="s">outlines</span>&quot;: state.gpu.view.show_outlines,</code></pre></div>
<h2 id="step-24-srcenginegpusurface_outliners">Step 24 · src/engine/gpu/surface_outline.rs<a class="anchor" href="#/course/17-source-presentation#step-24-srcenginegpusurface_outliners" aria-label="Link to this section">#</a></h2>
<p>Surface masks add outlines around visible coverage.</p>
<p><code>lessons/17/src/engine/gpu/surface_outline.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! The silhouette: the outline where a curved surface turns away from the viewer, which no edge in the model describes.</span>
<span class="k">use</span> std::collections::HashSet;

<span class="k">use</span> super::buffers::GpuCtx;
<span class="k">use</span> super::targets::{Targets, TextureSpec, texture_view};
<span class="k">use</span> <span class="k">crate</span>::engine::pipelines::{ColorWrite, DepthMode, PipelineDesc, Target, build, module};

<span class="c">/// One coverage mask: where the surfaces are on screen.</span>
<span class="k">struct</span> Mask {
    resolved: wgpu::TextureView, <span class="c">// coverage at 1x</span>
    multisampled: Option&lt;wgpu::TextureView&gt;, <span class="c">// MSAA coverage, if on</span>
    group: wgpu::BindGroup, <span class="c">// mask, radius and coarse mask, for the compositor</span>
    coarse: wgpu::TextureView, <span class="c">// max per \`POOL\` block, lets the compositor skip</span>
    coarse_size: (u32, u32),
    pool_group: wgpu::BindGroup, <span class="c">// the mask, for the coarse pass</span>
    size: (u32, u32), <span class="c">// mask size, px</span>
    samples: u32,
}

<span class="c">/// Mask pixels per coarse pixel; must match the shader.</span>
<span class="k">const</span> POOL: u32 = <span class="s">16</span>;

<span class="c">/// Which surfaces the outline goes around.</span>
#[derive(Clone, Copy, PartialEq)]
<span class="k">pub</span> <span class="k">enum</span> OutlineKind {
    Selected,
    AllSolids,
}</code></pre></div>
<p><code>lessons/17/src/engine/gpu/surface_outline.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Draws a black outline around surfaces from a coverage mask.</span>
<span class="k">pub</span> <span class="k">struct</span> SurfaceOutline {
    kind: OutlineKind, <span class="c">// which surfaces</span>
    selected: HashSet&lt;u32&gt;, <span class="c">// selected object rows</span>
    layout: wgpu::BindGroupLayout, <span class="c">// mask, radius, coarse mask</span>
    pool_layout: wgpu::BindGroupLayout, <span class="c">// one mask</span>
    uniform: wgpu::Buffer, <span class="c">// radius in px and a selected flag</span>
    pipeline: wgpu::RenderPipeline, <span class="c">// draws the outline</span>
    pool_pipeline: wgpu::RenderPipeline, <span class="c">// shrinks the mask to blocks</span>
    mask: Option&lt;Mask&gt;, <span class="c">// current mask textures</span>
}

<span class="k">impl</span> SurfaceOutline {
    <span class="c">/// Create the layouts and pipelines; textures come with the first frame.</span>
    <span class="k">pub</span> <span class="k">fn</span> new(ctx: &amp;GpuCtx, target: Target, kind: OutlineKind) -&gt; <span class="k">Self</span> {
        <span class="k">let</span> layout = ctx
            .device
            .create_bind_group_layout(&amp;wgpu::BindGroupLayoutDescriptor {
                label: Some(&quot;<span class="s">selection outline</span>&quot;),
                entries: &amp;[
                    wgpu::BindGroupLayoutEntry {
                        binding: <span class="s">0</span>,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: <span class="s">false</span> },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: <span class="s">false</span>,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: <span class="s">1</span>,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: <span class="s">false</span>,
                            min_binding_size: wgpu::BufferSize::new(<span class="s">16</span>),
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: <span class="s">2</span>,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: <span class="s">false</span> },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: <span class="s">false</span>,
                        },
                        count: None,
                    },
                ],
            });
        <span class="k">let</span> pool_layout = ctx
            .device
            .create_bind_group_layout(&amp;wgpu::BindGroupLayoutDescriptor {
                label: Some(&quot;<span class="s">selection outline pool</span>&quot;),
                entries: &amp;[wgpu::BindGroupLayoutEntry {
                    binding: <span class="s">0</span>,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: <span class="s">false</span> },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: <span class="s">false</span>,
                    },
                    count: None,
                }],
            });
        <span class="k">let</span> uniform = ctx.device.create_buffer(&amp;wgpu::BufferDescriptor {
            label: Some(&quot;<span class="s">selection outline radius</span>&quot;),
            size: <span class="s">16</span>,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: <span class="s">false</span>,
        });
        <span class="k">let</span> pipeline = pipeline(ctx, &amp;layout, target);
        <span class="k">let</span> pool_pipeline = pool_pipeline(ctx, &amp;pool_layout);
        <span class="k">Self</span> {
            kind,
            selected: HashSet::new(),
            layout,
            pool_layout,
            uniform,
            pipeline,
            pool_pipeline,
            mask: None,
        }
    }</code></pre></div>
<p><code>lessons/17/src/engine/gpu/surface_outline.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Remember whether object \`row\` is selected.</span>
    <span class="k">pub</span> <span class="k">fn</span> set_selected(&amp;<span class="k">mut</span> <span class="k">self</span>, row: u32, selected: bool) {
        <span class="k">if</span> selected {
            <span class="k">self</span>.selected.insert(row);
        } <span class="k">else</span> {
            <span class="k">self</span>.selected.remove(&amp;row);
        }

        <span class="c">// nothing selected: free the mask</span>
        <span class="k">if</span> <span class="k">self</span>.kind == OutlineKind::Selected &amp;&amp; <span class="k">self</span>.selected.is_empty() {
            <span class="k">self</span>.mask = None;
        }
    }

    <span class="c">/// Forget the selection and drop the mask.</span>
    <span class="k">pub</span> <span class="k">fn</span> reset(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">self</span>.selected.clear();
        <span class="k">self</span>.mask = None;
    }

    <span class="c">/// Rebuild the pipeline for a new MSAA sample count.</span>
    <span class="k">pub</span> <span class="k">fn</span> retarget(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, target: Target) {
        <span class="k">self</span>.pipeline = pipeline(ctx, &amp;<span class="k">self</span>.layout, target);
        <span class="k">self</span>.mask = None;
    }</code></pre></div>
<p><code>lessons/17/src/engine/gpu/surface_outline.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Make sure the mask textures exist; returns false when no outline is due.</span>
    <span class="k">pub</span> <span class="k">fn</span> prepare(
        &amp;<span class="k">mut</span> <span class="k">self</span>,
        ctx: &amp;GpuCtx,
        size: (u32, u32),
        samples: u32,
        css_width: f64,
        faces: bool,
    ) -&gt; bool {
        <span class="k">if</span> (<span class="k">self</span>.kind == OutlineKind::Selected &amp;&amp; <span class="k">self</span>.selected.is_empty()) || !faces {
            <span class="k">self</span>.mask = None;
            <span class="k">return</span> <span class="s">false</span>;
        }

        <span class="c">// remake the textures when size or samples changed</span>
        <span class="k">let</span> changed = <span class="k">match</span> &amp;<span class="k">self</span>.mask {
            Some(mask) =&gt; mask.size != size || mask.samples != samples,
            None =&gt; <span class="s">true</span>,
        };

        <span class="k">if</span> changed {
            <span class="k">let</span> usage =
                wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING;
            <span class="k">let</span> spec = TextureSpec {
                size,
                format: wgpu::TextureFormat::R8Unorm,
                samples: <span class="s">1</span>,
                usage,
            };
            <span class="k">let</span> resolved = texture_view(ctx, &quot;<span class="s">selection coverage</span>&quot;, &amp;spec);
            <span class="k">let</span> multisampled = <span class="k">if</span> samples &gt; <span class="s">1</span> {
                Some(texture_view(
                    ctx,
                    &quot;<span class="s">selection coverage MSAA</span>&quot;,
                    &amp;TextureSpec { samples, ..spec },
                ))
            } <span class="k">else</span> {
                None
            };
            <span class="k">let</span> coarse_size = (size.<span class="s">0</span>.div_ceil(POOL).max(<span class="s">1</span>), size.<span class="s">1</span>.div_ceil(POOL).max(<span class="s">1</span>));
            <span class="k">let</span> coarse = texture_view(
                ctx,
                &quot;<span class="s">selection coverage coarse</span>&quot;,
                &amp;TextureSpec {
                    size: coarse_size,
                    ..spec
                },
            );
            <span class="k">let</span> group = ctx.device.create_bind_group(&amp;wgpu::BindGroupDescriptor {
                label: Some(&quot;<span class="s">selection outline</span>&quot;),
                layout: &amp;<span class="k">self</span>.layout,
                entries: &amp;[
                    wgpu::BindGroupEntry {
                        binding: <span class="s">0</span>,
                        resource: wgpu::BindingResource::TextureView(&amp;resolved),
                    },
                    wgpu::BindGroupEntry {
                        binding: <span class="s">1</span>,
                        resource: <span class="k">self</span>.uniform.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: <span class="s">2</span>,
                        resource: wgpu::BindingResource::TextureView(&amp;coarse),
                    },
                ],
            });
            <span class="k">let</span> pool_group = ctx.device.create_bind_group(&amp;wgpu::BindGroupDescriptor {
                label: Some(&quot;<span class="s">selection outline pool</span>&quot;),
                layout: &amp;<span class="k">self</span>.pool_layout,
                entries: &amp;[wgpu::BindGroupEntry {
                    binding: <span class="s">0</span>,
                    resource: wgpu::BindingResource::TextureView(&amp;resolved),
                }],
            });
            <span class="k">self</span>.mask = Some(Mask {
                resolved,
                multisampled,
                group,
                coarse,
                coarse_size,
                pool_group,
                size,
                samples,
            });
        }</code></pre></div>
<p><code>lessons/17/src/engine/gpu/surface_outline.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> css_radius = <span class="k">if</span> <span class="k">self</span>.kind == OutlineKind::AllSolids {
            <span class="s">2</span>.<span class="s">25</span>
        } <span class="k">else</span> {
            <span class="s">3</span>.<span class="s">375</span>
        };
        <span class="c">// at most 12 px: the blur must fit inside one coarse block</span>
        <span class="k">let</span> radius = (css_radius * f64::from(size.<span class="s">0</span>) / css_width.max(<span class="s">1</span>.<span class="s">0</span>)).clamp(<span class="s">1</span>.<span class="s">0</span>, <span class="s">12</span>.<span class="s">0</span>) <span class="k">as</span> f32;
        ctx.queue.write_buffer(
            &amp;<span class="k">self</span>.uniform,
            <span class="s">0</span>,
            bytemuck::cast_slice(&amp;[
                radius,
                <span class="k">if</span> <span class="k">self</span>.kind == OutlineKind::Selected {
                    <span class="s">1</span>.<span class="s">0</span>
                } <span class="k">else</span> {
                    <span class="s">0</span>.<span class="s">0</span>
                },
                <span class="s">0</span>.<span class="s">0</span>,
                <span class="s">0</span>.<span class="s">0</span>,
            ]),
        );
        <span class="s">true</span>
    }</code></pre></div>
<p><code>lessons/17/src/engine/gpu/surface_outline.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Open the pass that draws this mask, cleared, against the scene depth.</span>
    <span class="k">pub</span> <span class="k">fn</span> begin_mask&lt;'a&gt;(
        &amp;'a <span class="k">self</span>,
        encoder: &amp;'a <span class="k">mut</span> wgpu::CommandEncoder,
        targets: &amp;'a Targets,
    ) -&gt; wgpu::RenderPass&lt;'a&gt; {
        <span class="k">let</span> mask = <span class="k">self</span>
            .mask
            .as_ref()
            .expect(&quot;<span class="s">prepare enabled selected coverage</span>&quot;);
        encoder.begin_render_pass(&amp;wgpu::RenderPassDescriptor {
            label: Some(&quot;<span class="s">visible selection coverage</span>&quot;),
            color_attachments: &amp;[Some(wgpu::RenderPassColorAttachment {
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
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &amp;targets.depth,
                depth_ops: None,
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        })
    }</code></pre></div>
<p><code>lessons/17/src/engine/gpu/surface_outline.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Shrink the mask to its block maxima.</span>
    <span class="k">pub</span> <span class="k">fn</span> encode_pool(&amp;<span class="k">self</span>, encoder: &amp;<span class="k">mut</span> wgpu::CommandEncoder) {
        <span class="k">let</span> Some(mask) = <span class="k">self</span>.mask.as_ref() <span class="k">else</span> {
            <span class="k">return</span>;
        };
        <span class="k">let</span> <span class="k">mut</span> pass = encoder.begin_render_pass(&amp;wgpu::RenderPassDescriptor {
            label: Some(&quot;<span class="s">selection coverage pool</span>&quot;),
            color_attachments: &amp;[Some(wgpu::RenderPassColorAttachment {
                view: &amp;mask.coarse,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        pass.set_pipeline(&amp;<span class="k">self</span>.pool_pipeline);
        pass.set_bind_group(<span class="s">0</span>, &amp;mask.pool_group, &amp;[]);
        pass.draw(<span class="s">0</span>..<span class="s">3</span>, <span class="s">0</span>..<span class="s">1</span>);
    }</code></pre></div>
<p><code>lessons/17/src/engine/gpu/surface_outline.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Draw both outlines in one fullscreen pass; returns the draw count.</span>
    <span class="k">pub</span> <span class="k">fn</span> draw_combined(&amp;<span class="k">self</span>, selected: &amp;<span class="k">Self</span>, pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;) -&gt; u32 {
        <span class="k">let</span> Some(normal) = <span class="k">self</span>.mask.as_ref().or(selected.mask.as_ref()) <span class="k">else</span> {
            <span class="k">return</span> <span class="s">0</span>;
        };
        <span class="k">let</span> selection = selected.mask.as_ref().unwrap_or(normal);
        pass.set_pipeline(&amp;<span class="k">self</span>.pipeline);
        pass.set_bind_group(<span class="s">0</span>, &amp;normal.group, &amp;[]);
        pass.set_bind_group(<span class="s">1</span>, &amp;selection.group, &amp;[]);
        pass.draw(<span class="s">0</span>..<span class="s">3</span>, <span class="s">0</span>..<span class="s">1</span>);
        <span class="s">1</span>
    }

    <span class="c">/// Bytes reserved on the GPU: (buffers, textures).</span>
    <span class="k">pub</span> <span class="k">fn</span> allocated_bytes(&amp;<span class="k">self</span>) -&gt; (u64, u64) {
        <span class="k">let</span> textures = <span class="k">match</span> &amp;<span class="k">self</span>.mask {
            Some(mask) =&gt; {
                <span class="k">let</span> samples = <span class="k">if</span> mask.samples &gt; <span class="s">1</span> {
                    u64::from(mask.samples) + <span class="s">1</span>
                } <span class="k">else</span> {
                    <span class="s">1</span>
                };
                u64::from(mask.size.<span class="s">0</span>) * u64::from(mask.size.<span class="s">1</span>) * samples
                    + u64::from(mask.coarse_size.<span class="s">0</span>) * u64::from(mask.coarse_size.<span class="s">1</span>)
            }
            None =&gt; <span class="s">0</span>,
        };
        (<span class="k">self</span>.uniform.size(), textures)
    }
}</code></pre></div>
<p><code>lessons/17/src/engine/gpu/surface_outline.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Outline pipeline: fullscreen, blended over the scene.</span>
<span class="k">fn</span> pipeline(ctx: &amp;GpuCtx, layout: &amp;wgpu::BindGroupLayout, target: Target) -&gt; wgpu::RenderPipeline {
    <span class="k">let</span> shader = module(
        &amp;ctx.device,
        &quot;<span class="s">selection outline</span>&quot;,
        include_str!(&quot;<span class="s">../../shaders/surface_outline.wgsl</span>&quot;),
    );
    <span class="k">let</span> groups = [layout, layout];
    <span class="k">let</span> desc = PipelineDesc::new(&amp;shader, &amp;groups, &amp;[], wgpu::PrimitiveTopology::TriangleList)
        .with(&quot;<span class="s">black selection outline</span>&quot;, &quot;<span class="s">fs_main</span>&quot;)
        .depth(DepthMode::Always)
        .color(ColorWrite::Blended);
    build(&amp;ctx.device, target, &amp;desc)
}

<span class="c">/// Pool pipeline: fullscreen into the coarse texture.</span>
<span class="k">fn</span> pool_pipeline(ctx: &amp;GpuCtx, layout: &amp;wgpu::BindGroupLayout) -&gt; wgpu::RenderPipeline {
    <span class="k">let</span> shader = module(
        &amp;ctx.device,
        &quot;<span class="s">selection outline pool</span>&quot;,
        include_str!(&quot;<span class="s">../../shaders/surface_outline.wgsl</span>&quot;),
    );
    <span class="k">let</span> groups = [layout];
    <span class="k">let</span> desc = PipelineDesc::new(&amp;shader, &amp;groups, &amp;[], wgpu::PrimitiveTopology::TriangleList)
        .with(&quot;<span class="s">selection outline pool</span>&quot;, &quot;<span class="s">fs_pool</span>&quot;)
        .depth(DepthMode::Detached);
    <span class="k">let</span> target = Target {
        format: wgpu::TextureFormat::R8Unorm,
        samples: <span class="s">1</span>,
    };
    build(&amp;ctx.device, target, &amp;desc)</code></pre></div>
<p>Copy this part from the lesson folder to the path shown.</p>
<p><code>lessons/17/src/engine/gpu/surface_outline.rs</code> · copy the file, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>}

#[cfg(all(test, not(target_arch = &quot;<span class="s">wasm32</span>&quot;)))]
<span class="k">mod</span> tests {
    <span class="k">use</span> <span class="k">crate</span>::engine::gpu::{FrameInput, Gpu, ObjectRow, Upload};
    <span class="k">use</span> session_rust::{RenderVertex, Xform};

    #[test]
    #[ignore = &quot;<span class="s">requires a native GPU adapter</span>&quot;]
    <span class="c">/// Selected edges never thin the black outline.</span>
    <span class="k">fn</span> selected_cad_edges_do_not_paint_over_the_black_silhouette() {
        <span class="k">use</span> <span class="k">crate</span>::app::scene::{FileDoc, Scene};
        <span class="k">use</span> <span class="k">crate</span>::camera::Camera;
        <span class="k">use</span> session_rust::{BRep, Session};
        <span class="k">use</span> std::rc::Rc;

        <span class="k">let</span> <span class="k">mut</span> gpu = pollster::block_on(Gpu::new_headless(<span class="s">480</span>, <span class="s">480</span>)).unwrap();
        gpu.view.show_grid = <span class="s">false</span>;
        gpu.view.markers = <span class="s">false</span>;

        <span class="k">for</span> shape <span class="k">in</span> [
            BRep::create_cone(<span class="s">150</span>.<span class="s">0</span>, <span class="s">400</span>.<span class="s">0</span>),
            BRep::create_cylinder(<span class="s">150</span>.<span class="s">0</span>, <span class="s">400</span>.<span class="s">0</span>),
        ] {
            gpu.reset();
            <span class="k">let</span> <span class="k">mut</span> source = Session::new(&quot;<span class="s">selected silhouette regression</span>&quot;);
            source.add_brep(shape, None);
            <span class="k">let</span> <span class="k">mut</span> scene = Scene::new();
            scene.add_file(FileDoc {
                name: &quot;<span class="s">solid</span>&quot;.into(),
                session: Rc::new(source),
                place: Xform::identity(),
                point_px: <span class="s">0</span>.<span class="s">0</span>,
                display_only: <span class="s">false</span>,
            });
            scene.upload_to(&amp;<span class="k">mut</span> gpu);
            gpu.set_selected(<span class="s">0</span>, <span class="s">true</span>);

            <span class="k">for</span> samples <span class="k">in</span> [<span class="s">1</span>, <span class="s">4</span>] {
                <span class="k">for</span> dpr <span class="k">in</span> [<span class="s">1</span>.<span class="s">0</span>, <span class="s">2</span>.<span class="s">0</span>] {
                    gpu.logical_size = [<span class="s">480</span>.<span class="s">0</span> / dpr; <span class="s">2</span>];
                    gpu.view.msaa_forced = Some(samples);
                    gpu.resize(<span class="s">480</span>, <span class="s">480</span>);

                    <span class="k">for</span> orbit <span class="k">in</span> [(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), (<span class="s">95</span>.<span class="s">0</span>, -<span class="s">65</span>.<span class="s">0</span>), (-<span class="s">80</span>.<span class="s">0</span>, <span class="s">130</span>.<span class="s">0</span>)] {
                        <span class="k">let</span> <span class="k">mut</span> camera = Camera::new();
                        camera.fit(&amp;gpu.bounds, <span class="s">1</span>.<span class="s">0</span>);
                        camera.orbit(orbit.<span class="s">0</span>, orbit.<span class="s">1</span>);
                        <span class="k">let</span> rebase =
                            gpu.rebase_anchor(&amp;camera.origin(), camera.distance_world(), <span class="s">0</span>.<span class="s">0</span>);
                        <span class="k">let</span> input = FrameInput {
                            view_proj: camera.view_proj_anchored(<span class="s">1</span>.<span class="s">0</span>, &amp;rebase.anchor),
                            clear: wgpu::Color::WHITE,
                            now_ms: <span class="s">0</span>.<span class="s">0</span>,
                        };
                        gpu.view.show_mesh_edges = <span class="s">false</span>;
                        <span class="k">let</span> silhouette = gpu.render_offscreen(&amp;input);
                        gpu.view.show_mesh_edges = <span class="s">true</span>;
                        <span class="k">let</span> edged = gpu.render_offscreen(&amp;input);
                        <span class="k">let</span> <span class="k">mut</span> black = <span class="s">0</span>;

                        <span class="k">for</span> (plain, inked) <span class="k">in</span> silhouette.chunks_exact(<span class="s">4</span>).zip(edged.chunks_exact(<span class="s">4</span>))
                        {
                            <span class="k">if</span> plain[..<span class="s">3</span>].iter().all(|channel| *channel &lt; <span class="s">8</span>) {
                                black += <span class="s">1</span>;
                                assert!(
                                    inked[..<span class="s">3</span>].iter().all(|channel| *channel &lt; <span class="s">12</span>),
                                    &quot;<span class="s">yellow CAD strokes must not narrow the black border: </span>{<span class="s">plain:?</span>}<span class="s"> -&gt; </span>{<span class="s">inked:?</span>}<span class="s">; samples=</span>{<span class="s">samples</span>}<span class="s">, DPR=</span>{<span class="s">dpr</span>}<span class="s">, orbit=</span>{<span class="s">orbit:?</span>}&quot;
                                );
                            }
                        }

                        assert!(
                            black &gt; <span class="s">500</span>,
                            &quot;<span class="s">the perspective silhouette must remain visible</span>&quot;
                        );
                    }
                }
            }
        }
    }

    #[test]
    #[ignore = &quot;<span class="s">requires a native GPU adapter</span>&quot;]
    <span class="c">/// Two touching solids get one outline, none between them.</span>
    <span class="k">fn</span> touching_and_overlapping_solids_have_one_continuous_outline() {
        <span class="k">let</span> <span class="k">mut</span> gpu = pollster::block_on(Gpu::new_headless(<span class="s">200</span>, <span class="s">200</span>)).unwrap();
        gpu.view.show_grid = <span class="s">false</span>;
        gpu.view.lit = <span class="s">false</span>;
        <span class="k">let</span> input = FrameInput {
            view_proj: Xform::identity(),
            clear: wgpu::Color::WHITE,
            now_ms: <span class="s">0</span>.<span class="s">0</span>,
        };

        <span class="k">for</span> samples <span class="k">in</span> [<span class="s">1</span>, <span class="s">4</span>] {
            gpu.view.msaa_forced = Some(samples);
            gpu.resize(<span class="s">200</span>, <span class="s">200</span>);

            <span class="k">for</span> overlap <span class="k">in</span> [<span class="s">false</span>, <span class="s">true</span>] {
                gpu.reset();
                <span class="k">let</span> <span class="k">mut</span> upload = Upload::default();
                quad(&amp;<span class="k">mut</span> upload, <span class="s">0</span>.<span class="s">4</span>, <span class="s">0</span>.<span class="s">5</span>);
                quad(&amp;<span class="k">mut</span> upload, <span class="s">0</span>.<span class="s">4</span>, <span class="s">0</span>.<span class="s">6</span>);

                <span class="k">for</span> (i, vertex) <span class="k">in</span> upload.arena.verts.iter_mut().enumerate() {
                    vertex.position[<span class="s">0</span>] += <span class="k">if</span> i &lt; <span class="s">4</span> {
                        -<span class="s">0</span>.<span class="s">4</span>
                    } <span class="k">else</span> <span class="k">if</span> overlap {
                        <span class="s">0</span>.<span class="s">2</span>
                    } <span class="k">else</span> {
                        <span class="s">0</span>.<span class="s">4</span>
                    };
                }

                gpu.set_scene(&amp;upload);
                <span class="k">let</span> outlined = gpu.render_offscreen(&amp;input);
                gpu.view.show_outlines = <span class="s">false</span>;
                <span class="k">let</span> plain = gpu.render_offscreen(&amp;input);
                <span class="k">let</span> ids = gpu.render_ids_offscreen(&amp;input);
                gpu.view.show_outlines = <span class="s">true</span>;
                assert_eq!(ids, gpu.render_ids_offscreen(&amp;input));

                <span class="k">for</span> y <span class="k">in</span> <span class="s">65</span>..<span class="s">135</span> {
                    <span class="k">for</span> x <span class="k">in</span> <span class="s">45</span>..<span class="s">150</span> {
                        <span class="k">let</span> at = (y * <span class="s">200</span> + x) * <span class="s">4</span>;
                        assert_eq!(
                            &amp;outlined[at..at + <span class="s">4</span>],
                            &amp;plain[at..at + <span class="s">4</span>],
                            &quot;<span class="s">no outline inside the combined silhouette, including object joins</span>&quot;
                        );
                    }
                }

                assert_ne!(
                    outlined, plain,
                    &quot;<span class="s">the combined outside silhouette must still be outlined</span>&quot;
                );
            }
        }
    }

    <span class="c">/// Add one square object to the upload.</span>
    <span class="k">fn</span> quad(upload: &amp;<span class="k">mut</span> Upload, extent: f32, depth: f32) {
        <span class="k">let</span> row = upload.obj.rows.len() <span class="k">as</span> u32;
        <span class="k">let</span> first = upload.arena.verts.len() <span class="k">as</span> u32;
        upload.obj.rows.push(ObjectRow::new(Xform::identity(), <span class="s">0</span>));

        <span class="k">for</span> [x, y] <span class="k">in</span> [
            [-extent, -extent],
            [extent, -extent],
            [extent, extent],
            [-extent, extent],
        ] {
            upload.arena.verts.push(RenderVertex {
                position: [x, y, depth],
                normal: [<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>],
                color: [<span class="s">0</span>.<span class="s">3</span>, <span class="s">0</span>.<span class="s">5</span>, <span class="s">0</span>.<span class="s">7</span>, <span class="s">1</span>.<span class="s">0</span>],
            });
            upload.arena.vids.push(row);
        }

        upload
            .arena
            .idx
            .extend([<span class="s">0</span>, <span class="s">1</span>, <span class="s">2</span>, <span class="s">0</span>, <span class="s">2</span>, <span class="s">3</span>].map(|i| first + i));
    }

    #[test]
    #[ignore = &quot;<span class="s">requires a native GPU adapter</span>&quot;]
    <span class="c">/// A hidden selection has no outline; clearing it frees the mask.</span>
    <span class="k">fn</span> selected_silhouette_is_black_visible_only_and_releases_coverage() {
        <span class="k">let</span> <span class="k">mut</span> gpu = pollster::block_on(Gpu::new_headless(<span class="s">200</span>, <span class="s">200</span>)).unwrap();
        gpu.view.show_grid = <span class="s">false</span>;
        gpu.view.lit = <span class="s">false</span>;
        <span class="k">let</span> <span class="k">mut</span> upload = Upload::default();
        quad(&amp;<span class="k">mut</span> upload, <span class="s">0</span>.<span class="s">5</span>, <span class="s">0</span>.<span class="s">5</span>);
        quad(&amp;<span class="k">mut</span> upload, <span class="s">0</span>.<span class="s">8</span>, <span class="s">0</span>.<span class="s">7</span>);
        gpu.set_scene(&amp;upload);
        <span class="k">let</span> input = FrameInput {
            view_proj: Xform::identity(),
            clear: wgpu::Color::WHITE,
            now_ms: <span class="s">0</span>.<span class="s">0</span>,
        };

        <span class="k">for</span> samples <span class="k">in</span> [<span class="s">1</span>, <span class="s">4</span>, <span class="s">1</span>] {
            gpu.view.msaa_forced = Some(samples);
            gpu.resize(<span class="s">200</span>, <span class="s">200</span>);
            <span class="k">let</span> hidden = gpu.render_offscreen(&amp;input);
            gpu.set_selected(<span class="s">0</span>, <span class="s">true</span>);
            assert_eq!(
                hidden,
                gpu.render_offscreen(&amp;input),
                &quot;<span class="s">an entirely occluded selection has no outline or yellow pixels</span>&quot;
            );
            gpu.set_hidden(<span class="s">1</span>, <span class="s">true</span>);
            <span class="k">let</span> selected = gpu.render_offscreen(&amp;input);
            <span class="c">// with only the selected mask the picture is the same</span>
            gpu.solid_outline.kind = super::OutlineKind::Selected;
            assert_eq!(
                selected,
                gpu.render_offscreen(&amp;input),
                &quot;<span class="s">selection must not receive a second overlapping outline</span>&quot;
            );
            gpu.solid_outline.kind = super::OutlineKind::AllSolids;
            <span class="k">let</span> black = selected
                .chunks_exact(<span class="s">4</span>)
                .filter(|p| p[..<span class="s">3</span>].iter().all(|c| *c &lt; <span class="s">8</span>))
                .count();
            <span class="k">let</span> yellow = selected
                .chunks_exact(<span class="s">4</span>)
                .filter(|p| p[<span class="s">0</span>] &gt; <span class="s">240</span> &amp;&amp; p[<span class="s">1</span>] &gt; <span class="s">240</span> &amp;&amp; p[<span class="s">2</span>] &lt; <span class="s">8</span>)
                .count();
            assert!(
                black &gt; <span class="s">350</span>,
                &quot;<span class="s">selected silhouette must have a continuous black border: </span>{<span class="s">black</span>}&quot;
            );
            assert!(
                yellow &gt; <span class="s">9500</span>,
                &quot;<span class="s">the original selection fill stays yellow: </span>{<span class="s">yellow</span>}&quot;
            );
            <span class="k">let</span> selected_ids = gpu.render_ids_offscreen(&amp;input);
            <span class="k">let</span> capacity = gpu.selection_outline.allocated_bytes().<span class="s">1</span>;
            assert_eq!(
                capacity,
                <span class="s">200</span> * <span class="s">200</span> * <span class="k">if</span> samples &gt; <span class="s">1</span> { <span class="s">5</span> } <span class="k">else</span> { <span class="s">1</span> } + <span class="s">13</span> * <span class="s">13</span>
            );
            gpu.set_selected(<span class="s">0</span>, <span class="s">false</span>);
            assert_eq!(
                gpu.selection_outline.allocated_bytes().<span class="s">1</span>,
                <span class="s">0</span>,
                &quot;<span class="s">clearing selection releases coverage immediately</span>&quot;
            );
            <span class="k">let</span> plain = gpu.render_offscreen(&amp;input);
            assert_eq!(
                selected_ids,
                gpu.render_ids_offscreen(&amp;input),
                &quot;<span class="s">a visual border must not add pickable geometry</span>&quot;
            );
            <span class="k">let</span> plain_black = plain
                .chunks_exact(<span class="s">4</span>)
                .filter(|p| p[..<span class="s">3</span>].iter().all(|c| *c &lt; <span class="s">8</span>))
                .count();
            assert!(
                plain_black &gt; <span class="s">0</span> &amp;&amp; plain_black &lt; black,
                &quot;<span class="s">ordinary and selected outlines both stay visible</span>&quot;
            );
            gpu.view.show_outlines = <span class="s">false</span>;
            <span class="k">let</span> disabled = gpu.render_offscreen(&amp;input);
            assert!(
                disabled
                    .chunks_exact(<span class="s">4</span>)
                    .all(|p| p[<span class="s">0</span>] &gt; <span class="s">8</span> || p[<span class="s">1</span>] &gt; <span class="s">8</span> || p[<span class="s">2</span>] &gt; <span class="s">8</span>)
            );
            assert_eq!(
                selected_ids,
                gpu.render_ids_offscreen(&amp;input),
                &quot;<span class="s">outline toggling never changes source picking</span>&quot;
            );
            gpu.view.show_outlines = <span class="s">true</span>;
            gpu.set_hidden(<span class="s">1</span>, <span class="s">false</span>);
        }

        gpu.release();
        assert_eq!(gpu.selection_outline.allocated_bytes(), (<span class="s">16</span>, <span class="s">0</span>));
    }
}</code></pre></div>
<h2 id="step-25-srcshaderssurface_outlinewgsl">Step 25 · src/shaders/surface_outline.wgsl<a class="anchor" href="#/course/17-source-presentation#step-25-srcshaderssurface_outlinewgsl" aria-label="Link to this section">#</a></h2>
<p>The outline shader expands visible coverage into a narrow border.</p>
<p><code>lessons/17/src/shaders/surface_outline.wgsl</code> · 109 lines · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>@group(<span class="s">0</span>) @binding(<span class="s">0</span>) <span class="k">var</span> mask: texture_2d&lt;<span class="k">f32</span>&gt;;<span class="c"> // coverage of every solid</span>
@group(<span class="s">0</span>) @binding(<span class="s">1</span>) <span class="k">var</span>&lt;<span class="k">uniform</span>&gt; radius: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;;<span class="c"> // x: outline width, px; y: 1 for the selection mask</span>
@group(<span class="s">0</span>) @binding(<span class="s">2</span>) <span class="k">var</span> coarse: texture_2d&lt;<span class="k">f32</span>&gt;;<span class="c"> // max of each 16x16 block of the mask</span>
@group(<span class="s">1</span>) @binding(<span class="s">0</span>) <span class="k">var</span> selected_mask: texture_2d&lt;<span class="k">f32</span>&gt;;
@group(<span class="s">1</span>) @binding(<span class="s">1</span>) <span class="k">var</span>&lt;<span class="k">uniform</span>&gt; selected_radius: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;;
@group(<span class="s">1</span>) @binding(<span class="s">2</span>) <span class="k">var</span> selected_coarse: texture_2d&lt;<span class="k">f32</span>&gt;;

<span class="c">// Mask pixels per coarse pixel; must match the Rust POOL.</span>
<span class="k">const</span> POOL: <span class="k">i32</span> = <span class="s">16</span>;

<span class="c">// True when any of the nine coarse blocks around \`p\` has coverage.</span>
<span class="k">fn</span> near_any_coverage(coarse_mask: texture_2d&lt;<span class="k">f32</span>&gt;, p: <span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;) -&gt; <span class="k">bool</span> {
    <span class="k">let</span> size = <span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;(textureDimensions(coarse_mask));
    <span class="k">let</span> c = p / POOL;

    <span class="k">for</span> (<span class="k">var</span> y = -<span class="s">1</span>; y &lt;= <span class="s">1</span>; y++) {
        <span class="k">for</span> (<span class="k">var</span> x = -<span class="s">1</span>; x &lt;= <span class="s">1</span>; x++) {
            <span class="k">let</span> q = c + <span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;(x, y);

            <span class="k">if</span> (any(q &lt; <span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;(<span class="s">0</span>)) || any(q &gt;= size)) {
                <span class="k">continue</span>;
            }

            <span class="k">if</span> (textureLoad(coarse_mask, q, <span class="s">0</span>).r &gt; <span class="s">0.0</span>) {
                <span class="k">return</span> <span class="s">true</span>;
            }
        }
    }

    <span class="k">return</span> <span class="s">false</span>;
}

@vertex
<span class="c">// Fullscreen triangle.</span>
<span class="k">fn</span> vs_main(@builtin(vertex_index) i: <span class="k">u32</span>) -&gt; @builtin(position) <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt; {
    <span class="k">let</span> xy = <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(f32((i &lt;&lt; <span class="s">1u</span>) &amp; <span class="s">2u</span>), f32(i &amp; <span class="s">2u</span>));
    <span class="k">return</span> <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(xy * <span class="s">2.0</span> - <span class="s">1.0</span>, <span class="s">0.0</span>, <span class="s">1.0</span>);
}

<span class="c">// Outline strength at a pixel: covered within \`r\` but not itself covered.</span>
<span class="k">fn</span> boundary_coverage(coverage_mask: texture_2d&lt;<span class="k">f32</span>&gt;, coarse_mask: texture_2d&lt;<span class="k">f32</span>&gt;, r: <span class="k">f32</span>, position: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;) -&gt; <span class="k">f32</span> {
    <span class="k">let</span> size = <span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;(textureDimensions(coverage_mask));
    <span class="k">let</span> p = <span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;(position);

<span class="c">    // far from any surface: nothing to do</span>
    <span class="k">if</span> (!near_any_coverage(coarse_mask, p)) {
        <span class="k">return</span> <span class="s">0.0</span>;
    }

    <span class="k">let</span> center = textureLoad(coverage_mask, p, <span class="s">0</span>).r;

<span class="c">    // inside the surface: no outline</span>
    <span class="k">if</span> (center &gt;= <span class="s">0.999</span>) {
        <span class="k">return</span> <span class="s">0.0</span>;
    }

    <span class="k">let</span> extent = i32(ceil(r + <span class="s">0.5</span>));
    <span class="k">var</span> coverage = <span class="s">0.0</span>;

<span class="c">    // widest coverage within radius \`r\`</span>
    <span class="k">for</span> (<span class="k">var</span> y = -extent; y &lt;= extent; y++) {
        <span class="k">for</span> (<span class="k">var</span> x = -extent; x &lt;= extent; x++) {
            <span class="k">let</span> q = p + <span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;(x, y);

            <span class="k">if</span> (any(q &lt; <span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;(<span class="s">0</span>)) || any(q &gt;= size)) {
                <span class="k">continue</span>;
            }

            <span class="k">let</span> distance = length(<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(f32(x), f32(y)));

            <span class="k">if</span> (distance &gt;= r + <span class="s">0.5</span>) {
                <span class="k">continue</span>;
            }

            <span class="k">let</span> weight = <span class="s">1.0</span> - smoothstep(r - <span class="s">0.5</span>, r + <span class="s">0.5</span>, distance);
            coverage = max(coverage, textureLoad(coverage_mask, q, <span class="s">0</span>).r * weight);
        }
    }

    <span class="k">return</span> coverage * (<span class="s">1.0</span> - center);
}

@fragment
<span class="c">// Black outline with the coverage as alpha, from both masks.</span>
<span class="k">fn</span> fs_main(@builtin(position) position: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;) -&gt; @location(<span class="s">0</span>) <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt; {
    <span class="k">var</span> coverage = boundary_coverage(mask, coarse, radius.x, position.xy);

<span class="c">    // a second, different mask is bound</span>
    <span class="k">if</span> (radius.y != selected_radius.y) {
        coverage = max(coverage, boundary_coverage(selected_mask, selected_coarse, selected_radius.x, position.xy));
    }

    <span class="k">return</span> <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>, <span class="s">0.0</span>, <span class="s">0.0</span>, coverage);
}

<span class="c">// One coarse pixel: the maximum of its 16x16 block of the mask.</span>
@fragment
<span class="k">fn</span> fs_pool(@builtin(position) position: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;) -&gt; @location(<span class="s">0</span>) <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt; {
    <span class="k">let</span> size = <span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;(textureDimensions(mask));
    <span class="k">let</span> base = <span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;(position.xy) * POOL;
    <span class="k">var</span> highest = <span class="s">0.0</span>;

    <span class="k">for</span> (<span class="k">var</span> y = <span class="s">0</span>; y &lt; POOL; y++) {
        <span class="k">for</span> (<span class="k">var</span> x = <span class="s">0</span>; x &lt; POOL; x++) {
            <span class="k">let</span> q = base + <span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;(x, y);

            <span class="k">if</span> (any(q &gt;= size)) {
                <span class="k">continue</span>;
            }

            highest = max(highest, textureLoad(mask, q, <span class="s">0</span>).r);
        }
    }

    <span class="k">return</span> <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(highest, <span class="s">0.0</span>, <span class="s">0.0</span>, <span class="s">1.0</span>);
}</code></pre></div>
<h2 id="step-26-srcenginegpuarenars">Step 26 · src/engine/gpu/arena.rs<a class="anchor" href="#/course/17-source-presentation#step-26-srcenginegpuarenars" aria-label="Link to this section">#</a></h2>
<p>The arena holds mesh vertices and indices across objects.</p>
<p><code>lessons/17/src/engine/gpu/arena.rs</code> · edit · type this</p>
<p>Added after the <code>}</code> line in <code>impl ArenaLane</code> of <code>lessons/16/src/engine/gpu/arena.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Forget every row; capacity stays.</span>
    <span class="k">pub</span> <span class="k">fn</span> reset(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx) {
        <span class="k">self</span>.source_faces.reset(ctx);</code></pre></div>
<p>Added after the <code>pub fn release(&amp;mut self, ctx: &amp;GpuCtx) {</code> line in <code>impl ArenaLane</code> of <code>lessons/16/src/engine/gpu/arena.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.source_faces.release(ctx);</code></pre></div>
<p>Added after the <code>),</code> line in <code>fn build_pipelines</code> of <code>lessons/16/src/engine/gpu/arena.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="c">// masks write to a one-channel texture at the scene depth</span>
        solid_mask: build(
            dev,
            Target {
                format: wgpu::TextureFormat::R8Unorm,
                samples: target.samples,
            },
            &amp;base
                .with(&quot;<span class="s">triangle.solid_mask</span>&quot;, &quot;<span class="s">fs_solid_mask</span>&quot;)
                .depth(<span class="k">crate</span>::engine::pipelines::DepthMode::ReadOnlyEqual),
        ),</code></pre></div>
<h2 id="step-27-srcenginegpuviewrs">Step 27 · src/engine/gpu/view.rs<a class="anchor" href="#/course/17-source-presentation#step-27-srcenginegpuviewrs" aria-label="Link to this section">#</a></h2>
<p>View settings control display features without changing source geometry.</p>
<p><code>lessons/17/src/engine/gpu/view.rs</code> · edit · type this</p>
<p>Added after the <code>pub show_mesh_edges: bool,</code> line in <code>struct View</code> of <code>lessons/16/src/engine/gpu/view.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> show_outlines: bool, <span class="c">// black outlines around surfaces, \`O\`</span>
    <span class="k">pub</span> markers: bool, <span class="c">// vertex markers on mesh edges</span>
    <span class="k">pub</span> cloud_size: f32, <span class="c">// point size scale, \`[\` and \`]\`</span>
    <span class="k">pub</span> edl_strength: f32, <span class="c">// 0 = off</span>
    <span class="k">pub</span> lod_px: f32, <span class="c">// LOD = level of detail: skip cloud points smaller than this, px; 0 = draw all</span>
    <span class="k">pub</span> thickness_px: f32, <span class="c">// line width, CSS px</span></code></pre></div>
<p>Replaces the 5 lines from <code>markers: knob(&quot;BENCH_NO_MARKERS&quot;, &quot;nomarkers&quot;…</code> in <code>fn from_env</code> of <code>lessons/16/src/engine/gpu/view.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            show_outlines: knob(&quot;<span class="s">VIEWER_OUTLINES</span>&quot;, &quot;<span class="s">outlines</span>&quot;).is_some(),
            markers: knob(&quot;<span class="s">BENCH_NO_MARKERS</span>&quot;, &quot;<span class="s">nomarkers</span>&quot;).is_none(),
            cloud_size: knob_f32(&quot;<span class="s">VIEWER_CLOUD_SCALE</span>&quot;, &quot;<span class="s">cloud</span>&quot;, <span class="s">1</span>.<span class="s">0</span>),
            edl_strength: knob_f32(&quot;<span class="s">VIEWER_EDL</span>&quot;, &quot;<span class="s">edl</span>&quot;, <span class="s">0</span>.<span class="s">25</span>),
            lod_px: knob_f32(&quot;<span class="s">VIEWER_LOD</span>&quot;, &quot;<span class="s">lod</span>&quot;, <span class="s">0</span>.<span class="s">0</span>),
            thickness_px: knob_f32(&quot;<span class="s">VIEWER_THICKNESS</span>&quot;, &quot;<span class="s">thickness</span>&quot;, <span class="s">1</span>.<span class="s">0</span>).max(<span class="s">0</span>.<span class="s">1</span>),</code></pre></div>
<p>Added after the <code>}</code> line in <code>fn from_env</code> of <code>lessons/16/src/engine/gpu/view.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Framebuffer pixels per CSS pixel, capped by \`?dpr=\`.</span>
<span class="k">pub</span> <span class="k">fn</span> device_pixel_ratio() -&gt; f64 {
    #[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
    {
        <span class="k">let</span> ratio = web_sys::window()
            .map(|window| window.device_pixel_ratio())
            .filter(|ratio| *ratio &gt; <span class="s">0</span>.<span class="s">0</span>)
            .unwrap_or(<span class="s">1</span>.<span class="s">0</span>);
        <span class="k">let</span> cap = f64::from(knob_f32(&quot;<span class="s">VIEWER_DPR</span>&quot;, &quot;<span class="s">dpr</span>&quot;, <span class="s">0</span>.<span class="s">0</span>));
        <span class="k">let</span> ratio = <span class="k">if</span> cap &gt;= <span class="s">0</span>.<span class="s">5</span> { ratio.min(cap) } <span class="k">else</span> { ratio };

        <span class="k">if</span> reduced() { ratio.min(<span class="s">1</span>.<span class="s">0</span>) } <span class="k">else</span> { ratio }
    }
    #[cfg(not(target_arch = &quot;<span class="s">wasm32</span>&quot;))]
    {
        <span class="s">1</span>.<span class="s">0</span>
    }
}

<span class="c">/// True once the page dropped to device scale 1 without MSAA.</span>
<span class="k">static</span> REDUCED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(<span class="s">false</span>);

<span class="c">/// render at device scale 1</span>
<span class="k">pub</span> <span class="k">fn</span> reduce_for_slow_frames() {
    REDUCED.store(<span class="s">true</span>, std::sync::atomic::Ordering::Relaxed);
}

<span class="c">/// True once \`reduce\` was called.</span>
<span class="k">pub</span> <span class="k">fn</span> reduced() -&gt; bool {
    REDUCED.load(std::sync::atomic::Ordering::Relaxed)
}

<span class="c">/// Canvas pixels per browser pixel; below 1 when \`?dpr=\` caps it.</span>
<span class="k">pub</span> <span class="k">fn</span> surface_per_physical() -&gt; f64 {
    #[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
    {
        <span class="k">let</span> browser = web_sys::window()
            .map(|window| window.device_pixel_ratio())
            .filter(|ratio| *ratio &gt; <span class="s">0</span>.<span class="s">0</span>)
            .unwrap_or(<span class="s">1</span>.<span class="s">0</span>);
        device_pixel_ratio() / browser
    }
    #[cfg(not(target_arch = &quot;<span class="s">wasm32</span>&quot;))]
    {
        <span class="s">1</span>.<span class="s">0</span>
    }
}

<span class="c">/// One setting's text: \`?query=\` in the browser, \`ENV\` natively.</span></code></pre></div>
<h2 id="step-28-srcappinputrs">Step 28 · src/app/input.rs<a class="anchor" href="#/course/17-source-presentation#step-28-srcappinputrs" aria-label="Link to this section">#</a></h2>
<p>Input routes gestures and keyboard actions to State.</p>
<p><code>lessons/17/src/app/input.rs</code> · edit · type this</p>
<p>Added after the <code>WindowEvent::CursorMoved { position, .. } =&gt; {</code> line in <code>fn mouse</code> of <code>lessons/16/src/app/input.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                <span class="k">let</span> scale = <span class="k">crate</span>::engine::gpu::view::surface_per_physical(); <span class="c">// window to canvas pixels</span>
                <span class="k">let</span> position =
                    winit::dpi::PhysicalPosition::new(position.x * scale, position.y * scale);</code></pre></div>
<h2 id="step-29-srcappinspectionrs">Step 29 · src/app/inspection.rs<a class="anchor" href="#/course/17-source-presentation#step-29-srcappinspectionrs" aria-label="Link to this section">#</a></h2>
<p>Inspection reports retained resources and source information.</p>
<p><code>lessons/17/src/app/inspection.rs</code> · edit · type this</p>
<p>Added after the <code>&quot;id&quot;: run.label.id,</code> line in <code>fn text_labels</code> of <code>lessons/16/src/app/inspection.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            &quot;<span class="s">object</span>&quot;: run.label.object.map(|object| object.row),
            &quot;<span class="s">text</span>&quot;: run.label.text,
            &quot;<span class="s">font_size</span>&quot;: run.label.font_size,
            &quot;<span class="s">line_height</span>&quot;: run.label.line_height,
            &quot;<span class="s">color</span>&quot;: run.label.ink_color(),
            &quot;<span class="s">placement</span>&quot;: kind,
            &quot;<span class="s">world</span>&quot;: world,
            &quot;<span class="s">world_height</span>&quot;: <span class="k">match</span> run.label.placement {
                TextPlacement::WorldBillboard { world_height, .. } =&gt; Some(world_height),
                _ =&gt; None,
            },</code></pre></div>
<h2 id="step-30-srcappwalkcurvesrs">Step 30 · src/app/walk/curves.rs<a class="anchor" href="#/course/17-source-presentation#step-30-srcappwalkcurvesrs" aria-label="Link to this section">#</a></h2>
<p>Curve sampling builds connected strokes from source geometry.</p>
<p><code>lessons/17/src/app/walk/curves.rs</code> · edit · type this</p>
<p>Added after the <code>pub(super) fn push_polyline(seg: &amp;mut SegRows…</code> line of <code>lessons/16/src/app/walk/curves.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">let</span> first = seg.ribbons.len() <span class="k">as</span> u32;</code></pre></div>
<p>Added after the <code>}</code> line of <code>lessons/16/src/app/walk/curves.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    seg.ribbon_chains.push(first..seg.ribbons.len() <span class="k">as</span> u32); <span class="c">// one joined stroke</span></code></pre></div>
<h2 id="step-31-srcappwalkbrep_edgesrs">Step 31 · src/app/walk/brep_edges.rs<a class="anchor" href="#/course/17-source-presentation#step-31-srcappwalkbrep_edgesrs" aria-label="Link to this section">#</a></h2>
<p>Boundary chains share samples between adjacent faces.</p>
<p><code>lessons/17/src/app/walk/brep_edges.rs</code> · edit · type this</p>
<p>Added after the <code>let fm = &amp;ep.fms[chain.face];</code> line in <code>fn push_edge_pipes</code> of <code>lessons/16/src/app/walk/brep_edges.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">let</span> first = seg.pipes.len() <span class="k">as</span> u32;</code></pre></div>
<p>Added after the <code>}</code> line in <code>fn push_edge_pipes</code> of <code>lessons/16/src/app/walk/brep_edges.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    seg.pipe_chains.push(first..seg.pipes.len() <span class="k">as</span> u32); <span class="c">// one joined stroke</span></code></pre></div>
<h2 id="step-32-srcenginegpusegmentsrs">Step 32 · src/engine/gpu/segments.rs<a class="anchor" href="#/course/17-source-presentation#step-32-srcenginegpusegmentsrs" aria-label="Link to this section">#</a></h2>
<p>The segment buffers store strokes and the object rows they belong to.</p>
<p><code>lessons/17/src/engine/gpu/segments.rs</code> · edit · type this</p>
<p>Added after the <code>};</code> line of <code>lessons/16/src/engine/gpu/segments.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> std::collections::HashSet;</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// One line segment, 40 bytes, as the shaders read it.</span></code></pre></div>
<p>Added after the <code>pub pipe_ids: Vec&lt;u32&gt;,</code> line in <code>struct SegRows</code> of <code>lessons/16/src/engine/gpu/segments.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> pipe_chains: Vec&lt;std::ops::Range&lt;u32&gt;&gt;, <span class="c">// runs of pipes that form one curve</span>
    <span class="k">pub</span> ribbon_chains: Vec&lt;std::ops::Range&lt;u32&gt;&gt;, <span class="c">// runs of ribbons that form one curve</span></code></pre></div>
<p>Replaces the 2 lines from <code>drop_rows(&amp;mut self.ribbons);</code> in <code>fn drop_rows</code> of <code>lessons/16/src/engine/gpu/segments.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        drop_rows(&amp;<span class="k">mut</span> <span class="k">self</span>.pipe_chains);
        drop_rows(&amp;<span class="k">mut</span> <span class="k">self</span>.ribbon_chains);
        drop_rows(&amp;<span class="k">mut</span> <span class="k">self</span>.ribbons);
    }
}

<span class="c">/// A segment with its neighbours, so joints are drawn once.</span>
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
<span class="k">pub</span>(super) <span class="k">struct</span> StrokeSegment {
    <span class="k">pub</span>(super) segment: CylinderSegment,
    <span class="k">pub</span>(super) previous: u32, <span class="c">// row of the segment before it, or u32::MAX</span>
    <span class="k">pub</span>(super) next: u32, <span class="c">// row of the segment after it, or u32::MAX</span>
}

<span class="c">/// Link neighbouring segments inside each chain.</span>
<span class="k">fn</span> joined_rows(
    rows: &amp;[CylinderSegment],
    chains: &amp;[std::ops::Range&lt;u32&gt;],
    base: u32,
) -&gt; Vec&lt;StrokeSegment&gt; {
    <span class="k">let</span> <span class="k">mut</span> result = Vec::with_capacity(rows.len());

    <span class="k">for</span> segment <span class="k">in</span> rows {
        result.push(StrokeSegment {
            segment: *segment,
            previous: u32::MAX,
            next: u32::MAX,
        });
    }

    <span class="k">for</span> chain <span class="k">in</span> chains {
        <span class="k">if</span> chain.end &gt; rows.len() <span class="k">as</span> u32 || chain.end.saturating_sub(chain.start) &lt; <span class="s">2</span> {
            <span class="k">continue</span>;
        }

        <span class="k">for</span> index <span class="k">in</span> chain.clone() {
            <span class="c">// a closed chain wraps around</span>
            <span class="k">let</span> next = <span class="k">if</span> index + <span class="s">1</span> == chain.end {
                chain.start
            } <span class="k">else</span> {
                index + <span class="s">1</span>
            };
            <span class="k">let</span> a = &amp;rows[index <span class="k">as</span> usize];
            <span class="k">let</span> b = &amp;rows[next <span class="k">as</span> usize];

            <span class="c">// join only where the ends meet and look the same</span>
            <span class="k">if</span> a.p1 == b.p0
                &amp;&amp; a.instance_id == b.instance_id
                &amp;&amp; a.color == b.color
                &amp;&amp; a.radius == b.radius
            {
                result[index <span class="k">as</span> usize].next = next + base;
                result[next <span class="k">as</span> usize].previous = index + base;
            }
        }
    }

    result
}

<span class="c">/// One segment buffer, its ids and their bind group.</span></code></pre></div>
<p>Replaces the <code>std::mem::size_of::&lt;CylinderSegment&gt;() as u64,</code> line in <code>fn new</code> of <code>lessons/16/src/engine/gpu/segments.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            std::mem::size_of::&lt;StrokeSegment&gt;() <span class="k">as</span> u64,</code></pre></div>
<p><code>lessons/17/src/engine/gpu/segments.rs</code> · edit · type this</p>
<p>Added after the <code>ribbon: wgpu::RenderPipeline,</code> line in <code>struct SegPipelines</code> of <code>lessons/16/src/engine/gpu/segments.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    unselected: wgpu::RenderPipeline, <span class="c">// unselected objects' lines</span>
    selected: wgpu::RenderPipeline, <span class="c">// selected objects' lines</span></code></pre></div>
<p>Added after the <code>selection: wgpu::Buffer,</code> line in <code>struct SegmentLane</code> of <code>lessons/16/src/engine/gpu/segments.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    selected_rows: HashSet&lt;u32&gt;,
    selected_edge: bool,</code></pre></div>
<p>Added after the <code>selection,</code> line in <code>fn new</code> of <code>lessons/16/src/engine/gpu/segments.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            selected_rows: HashSet::new(),
            selected_edge: <span class="s">false</span>,</code></pre></div>
<p>Replaces the 7 lines from <code>let pipes_changed = self.pipes.buf.append(ctx…</code> in <code>fn append</code> of <code>lessons/16/src/engine/gpu/segments.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> pipes = joined_rows(&amp;up.pipes, &amp;up.pipe_chains, <span class="k">self</span>.pipes.buf.len());
        <span class="k">let</span> pipes_changed = <span class="k">self</span>.pipes.buf.append(ctx, &amp;pipes);

        <span class="k">if</span> <span class="k">self</span>.pipes.ids.append(ctx, &amp;ids) || pipes_changed {
            <span class="k">self</span>.pipes.rebind(ctx, l, &amp;<span class="k">self</span>.selection);
        }

        <span class="k">let</span> ribbons = joined_rows(&amp;up.ribbons, &amp;up.ribbon_chains, <span class="k">self</span>.ribbons.buf.len());
        <span class="k">let</span> ribbons_changed = <span class="k">self</span>.ribbons.buf.append(ctx, &amp;ribbons);</code></pre></div>
<p>Replaces <code>fn set_edge</code> in <code>lessons/16/src/engine/gpu/segments.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Select one source edge (object, edge); None clears it.</span>
    <span class="k">pub</span> <span class="k">fn</span> set_edge(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, edge: Option&lt;(u32, u32)&gt;) {
        <span class="k">self</span>.selected_edge = edge.is_some();</code></pre></div>
<p>Added after the <code>);</code> line in <code>fn set_edge</code> of <code>lessons/16/src/engine/gpu/segments.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Remember whether object \`row\` is selected.</span>
    <span class="k">pub</span> <span class="k">fn</span> set_selected(&amp;<span class="k">mut</span> <span class="k">self</span>, row: u32, selected: bool) {
        <span class="k">if</span> selected {
            <span class="k">self</span>.selected_rows.insert(row);
        } <span class="k">else</span> {
            <span class="k">self</span>.selected_rows.remove(&amp;row);
        }
    }

    <span class="c">/// Draw the unselected objects' lines.</span>
    <span class="k">pub</span> <span class="k">fn</span> draw_unselected(
        &amp;<span class="k">self</span>,
        pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;,
        b: &amp;Binds,
        pipes: bool,
        ribbons: bool,
    ) -&gt; u32 {
        <span class="k">let</span> <span class="k">mut</span> draws = <span class="s">0</span>;

        <span class="k">if</span> pipes {
            draws += <span class="k">self</span>.draw_table(pass, b, &amp;<span class="k">self</span>.gpu.unselected, &amp;<span class="k">self</span>.pipes);
        }

        <span class="k">if</span> ribbons {
            draws += <span class="k">self</span>.draw_table(pass, b, &amp;<span class="k">self</span>.gpu.unselected, &amp;<span class="k">self</span>.ribbons);
        }

        draws
    }

    <span class="c">/// Draw the selected objects' lines.</span>
    <span class="k">pub</span> <span class="k">fn</span> draw_selected(
        &amp;<span class="k">self</span>,
        pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;,
        b: &amp;Binds,
        pipes: bool,
        ribbons: bool,
    ) -&gt; u32 {
        <span class="k">if</span> <span class="k">self</span>.selected_rows.is_empty() &amp;&amp; !<span class="k">self</span>.selected_edge {
            <span class="k">return</span> <span class="s">0</span>;
        }

        <span class="k">let</span> <span class="k">mut</span> draws = <span class="s">0</span>;

        <span class="k">if</span> pipes {
            draws += <span class="k">self</span>.draw_table(pass, b, &amp;<span class="k">self</span>.gpu.selected, &amp;<span class="k">self</span>.pipes);
        }

        <span class="k">if</span> ribbons {
            draws += <span class="k">self</span>.draw_table(pass, b, &amp;<span class="k">self</span>.gpu.selected, &amp;<span class="k">self</span>.ribbons);
        }

        draws
    }

    <span class="c">/// Draw the edges in color.</span></code></pre></div>
<p>Added after the <code>pub fn reset(&amp;mut self) {</code> line in <code>impl SegmentLane</code> of <code>lessons/16/src/engine/gpu/segments.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.selected_rows.clear();
        <span class="k">self</span>.selected_edge = <span class="s">false</span>;</code></pre></div>
<p>Added after the <code>pub fn release(&amp;mut self, ctx: &amp;GpuCtx, l: &amp;L…</code> line in <code>impl SegmentLane</code> of <code>lessons/16/src/engine/gpu/segments.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.selected_rows.clear();
        <span class="k">self</span>.selected_edge = <span class="s">false</span>;</code></pre></div>
<p><code>lessons/17/src/engine/gpu/segments.rs</code> · edit · type this</p>
<p>Added after the <code>SegPipelines {</code> line in <code>fn build_pipelines</code> of <code>lessons/16/src/engine/gpu/segments.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        unselected: build(
            dev,
            target,
            &amp;quad
                .with(&quot;<span class="s">ribbon.unselected</span>&quot;, &quot;<span class="s">fs_main</span>&quot;)
                .vertex(&quot;<span class="s">vs_unselected</span>&quot;)
                .color(ColorWrite::Blended),
        ),
        selected: build(
            dev,
            target,
            &amp;quad
                .with(&quot;<span class="s">ribbon.selected</span>&quot;, &quot;<span class="s">fs_main</span>&quot;)
                .vertex(&quot;<span class="s">vs_selected</span>&quot;)
                .color(ColorWrite::Blended),
        ),</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// The shader declares StrokeSegment with the Rust fields.</span></code></pre></div>
<p>Replaces the 11 lines from <code>];</code> in <code>fn cylinder_segment_mirror</code> of <code>lessons/16/src/engine/gpu/segments.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            &quot;<span class="s">previous</span>&quot;,
            &quot;<span class="s">next</span>&quot;,
        ];

        <span class="k">for</span> (name, src) <span class="k">in</span> SHADERS {
            assert_eq!(
                wgsl_fields(src, &quot;<span class="s">StrokeSegment</span>&quot;),
                rust,
                &quot;{<span class="s">name</span>}<span class="s">: StrokeSegment fields</span>&quot;
            );
        }

        assert_eq!(std::mem::size_of::&lt;CylinderSegment&gt;(), <span class="s">40</span>);
        assert_eq!(std::mem::size_of::&lt;StrokeSegment&gt;(), <span class="s">48</span>);</code></pre></div>
<h2 id="step-33-srcenginegpuinstancers">Step 33 · src/engine/gpu/instance.rs<a class="anchor" href="#/course/17-source-presentation#step-33-srcenginegpuinstancers" aria-label="Link to this section">#</a></h2>
<p>Each object row carries placement, color and selection flags for later interaction.</p>
<p><code>lessons/17/src/engine/gpu/instance.rs</code> · edit · type this</p>
<p>Replaces the <code>use crate::engine::gpu::segments::CylinderSeg…</code> line in <code>fn shader_validation_and_layouts</code> of <code>lessons/16/src/engine/gpu/instance.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">use</span> <span class="k">crate</span>::engine::gpu::segments::{CylinderSegment, StrokeSegment};</code></pre></div>
<p>Replaces the <code>&quot;CylinderSegment&quot; =&gt; (</code> line in <code>fn shader_validation_and_layouts</code> of <code>lessons/16/src/engine/gpu/instance.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                    &quot;<span class="s">StrokeSegment</span>&quot; =&gt; (</code></pre></div>
<p>Replaces the 2 lines from <code>],</code> in <code>fn shader_validation_and_layouts</code> of <code>lessons/16/src/engine/gpu/instance.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                            offset_of!(StrokeSegment, previous),
                            offset_of!(StrokeSegment, next),
                        ],
                        size_of::&lt;StrokeSegment&gt;(),</code></pre></div>
<p>Added after the <code>offset_of!(LineUniform, backface),</code> line in <code>fn shader_validation_and_layouts</code> of <code>lessons/16/src/engine/gpu/instance.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                            offset_of!(LineUniform, origin),
                            offset_of!(LineUniform, frame),
                            offset_of!(LineUniform, opacity),</code></pre></div>
<p>Replaces the 25 lines from <code>];</code> in <code>fn line_uniform_mirror</code> of <code>lessons/16/src/engine/gpu/instance.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            &quot;<span class="s">origin</span>&quot;,
            &quot;<span class="s">frame</span>&quot;,
            &quot;<span class="s">opacity</span>&quot;,
        ];
        assert_eq!(
            wgsl_fields(SCENE, &quot;<span class="s">LineUniform</span>&quot;),
            rust,
            &quot;<span class="s">LineUniform fields</span>&quot;
        );
        assert_eq!(std::mem::size_of::&lt;LineUniform&gt;(), <span class="s">80</span>);
    }

    <span class="c">/// Translations live in the shared code; no shader redeclares the structs.</span>
    #[test]
    <span class="k">fn</span> translations_mirror() {
        <span class="k">let</span> binding = &quot;<span class="s">@group(2) @binding(1) var&lt;storage, read&gt; translations: array&lt;vec4&lt;f32&gt;&gt;;</span>&quot;;
        assert!(SCENE.contains(binding), &quot;<span class="s">translations binding</span>&quot;);
        assert!(SCENE.contains(&quot;<span class="s">fn place(</span>&quot;), &quot;<span class="s">the place() helper</span>&quot;);

        <span class="k">for</span> (name, src) <span class="k">in</span> lane_shaders() {
            assert!(
                !src.contains(&quot;<span class="s">struct Instance</span>&quot;),
                &quot;{<span class="s">name</span>}<span class="s">: redeclares Instance</span>&quot;
            );
            assert!(
                !src.contains(&quot;<span class="s">struct LineUniform</span>&quot;),
                &quot;{<span class="s">name</span>}<span class="s">: redeclares LineUniform</span>&quot;
            );</code></pre></div>
<h2 id="step-34-srcshadersribbonwgsl">Step 34 · src/shaders/ribbon.wgsl<a class="anchor" href="#/course/17-source-presentation#step-34-srcshadersribbonwgsl" aria-label="Link to this section">#</a></h2>
<p>The stroke shader expands segments into screen-space ribbons.</p>
<p><code>lessons/17/src/shaders/ribbon.wgsl</code> · edit · type this</p>
<p>Replaces <code>struct CylinderSegment</code> in <code>lessons/16/src/shaders/ribbon.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// One line segment with its neighbours, 48 bytes; matches StrokeSegment in Rust.</span>
<span class="k">struct</span> StrokeSegment {</code></pre></div>
<p>Replaces the 3 lines from <code>}</code> of <code>lessons/16/src/shaders/ribbon.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    previous: <span class="k">u32</span>,<span class="c"> // row of the segment before it, or none</span>
    next: <span class="k">u32</span>,<span class="c"> // row of the segment after it, or none</span>
}

@group(<span class="s">3</span>) @binding(<span class="s">0</span>) <span class="k">var</span>&lt;<span class="k">storage</span>, <span class="k">read</span>&gt; segments: <span class="k">array</span>&lt;StrokeSegment&gt;;</code></pre></div>
<p>Added after the <code>@location(10) @interpolate(flat) source_edge:…</code> line in <code>struct VsOut</code> of <code>lessons/16/src/shaders/ribbon.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    @location(<span class="s">11</span>) @interpolate(flat) start_join: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // cut plane at the start joint: normal, point</span>
    @location(<span class="s">12</span>) @interpolate(flat) end_join: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,</code></pre></div>
<p><code>lessons/17/src/shaders/ribbon.wgsl</code> · edit · type this</p>
<p>Replaces the 6 lines from <code>@vertex</code> of <code>lessons/16/src/shaders/ribbon.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// True when the segment is drawn: a face beside it faces the camera.</span>
<span class="k">fn</span> neighbor_visible(seg: StrokeSegment) -&gt; <span class="k">bool</span> {
    <span class="k">let</span> inst = instances[seg.instance_id];

    <span class="k">if</span> ((inst.flags &amp; (FLAG_INSIDE | FLAG_OPEN)) != <span class="s">0u</span> || seg.facing == FACING_UNKNOWN || line.opacity &lt;= <span class="s">0.0</span>) {
        <span class="k">return</span> <span class="s">true</span>;
    }

    <span class="k">let</span> p0 = place(seg.instance_id, <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(seg.p0x, seg.p0y, seg.p0z));
    <span class="k">let</span> p1 = place(seg.instance_id, <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(seg.p1x, seg.p1y, seg.p1z));
    <span class="k">let</span> n0 = face_normal(inst.model, oct16_decode(seg.facing &amp; <span class="s">0xffffu</span>));
    <span class="k">let</span> n1 = face_normal(inst.model, oct16_decode(seg.facing &gt;&gt; <span class="s">16u</span>));
    <span class="k">return</span> edge_faces_camera(seg.facing, n0, n1, toward_eye((p0+p1)*<span class="s">0.5</span>));
}

<span class="c">// Cut plane between two joined segments: (normal, point), or zero for no joint.</span>
<span class="k">fn</span> join_plane(before: <span class="k">u32</span>, after: <span class="k">u32</span>) -&gt; <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt; {
    <span class="k">if</span> (before == <span class="s">0xffffffffu</span> || after == <span class="s">0xffffffffu</span> || before &gt;= arrayLength(&amp;segments) || after &gt;= arrayLength(&amp;segments)) {
        <span class="k">return</span> <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>);
    }

    <span class="k">let</span> a = segments[before];
    <span class="k">let</span> b = segments[after];

    <span class="k">if</span> (!neighbor_visible(a) || !neighbor_visible(b)) {
        <span class="k">return</span> <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>);
    }

    <span class="k">let</span> c0 = mvp * <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(place(a.instance_id, <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(a.p0x, a.p0y, a.p0z)), <span class="s">1.0</span>);
    <span class="k">let</span> c1 = mvp * <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(place(a.instance_id, <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(a.p1x, a.p1y, a.p1z)), <span class="s">1.0</span>);
    <span class="k">let</span> c2 = mvp * <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(place(b.instance_id, <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(b.p1x, b.p1y, b.p1z)), <span class="s">1.0</span>);

    <span class="k">if</span> (c0.w &lt;= <span class="s">0.0</span> || c1.w &lt;= <span class="s">0.0</span> || c2.w &lt;= <span class="s">0.0</span> || c0.z &gt; c0.w || c1.z &gt; c1.w || c2.z &gt; c2.w) {
        <span class="k">return</span> <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>);
    }

    <span class="k">let</span> vp = <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(line.vp_w, line.vp_h);
    <span class="k">let</span> p0 = (c0.xy/c0.w*<span class="s">0.5</span>+<span class="s">0.5</span>)*vp;
    <span class="k">let</span> p1 = (c1.xy/c1.w*<span class="s">0.5</span>+<span class="s">0.5</span>)*vp;
    <span class="k">let</span> p2 = (c2.xy/c2.w*<span class="s">0.5</span>+<span class="s">0.5</span>)*vp;
    <span class="k">let</span> d0 = p1-p0;
    <span class="k">let</span> d1 = p2-p1;

    <span class="k">if</span> (dot(d0, d0) &lt; 1e-<span class="s">8</span> || dot(d1, d1) &lt; 1e-<span class="s">8</span>) {
        <span class="k">return</span> <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>);
    }

<span class="c">    // bisector of the two directions</span>
    <span class="k">let</span> normal = normalize(d0)+normalize(d1);

    <span class="k">if</span> (dot(normal, normal) &lt; 1e-<span class="s">8</span>) {
        <span class="k">return</span> <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>);
    }

    <span class="k">return</span> <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(normalize(normal), p1);
}

<span class="c">// One quad corner of segment \`vid / 6\`; layer 0 all, 1 unselected, 2 selected.</span>
<span class="k">fn</span> stroke_vertex(vid: <span class="k">u32</span>, layer: <span class="k">u32</span>) -&gt; VsOut {
    <span class="k">let</span> iid = vid / <span class="s">6u</span>;
    <span class="k">let</span> corner = corner_of(vid % <span class="s">6u</span>);
    <span class="k">let</span> seg = segments[iid];
    <span class="k">let</span> inst = instances[seg.instance_id];
<span class="c">    // selected object, or the selected source edge</span>
    <span class="k">let</span> selected = (inst.flags &amp; FLAG_SELECTED) != <span class="s">0u</span> ||
        (edge_selection.x == seg.instance_id &amp;&amp; edge_selection.y != <span class="s">0xffffffffu</span> &amp;&amp;
         edge_selection.y == source_edges[iid]);

    <span class="k">if</span> ((layer == <span class="s">1u</span> &amp;&amp; selected) || (layer == <span class="s">2u</span> &amp;&amp; !selected)) {
        <span class="k">return</span> dead_vertex();
    }</code></pre></div>
<p>Replaces the 5 lines from <code>let raw0 = half_width_px(seg.radius, e0.w);</code> in <code>fn vs_main</code> of <code>lessons/16/src/shaders/ribbon.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">    // half widths at both ends; a selected stroke is at least a full pen wide</span>
    <span class="k">let</span> raw0 = max(half_width_px(seg.radius, e0.w), select(<span class="s">0.0</span>, line.thickness, selected));
    <span class="k">let</span> raw1 = max(half_width_px(seg.radius, e1.w), select(<span class="s">0.0</span>, line.thickness, selected));
    <span class="k">let</span> px = floor_hairline(select(raw0, raw1, at_end1));
<span class="c">    // CAD boundary segments are samples of one curve, not independent mesh wires.</span>
    <span class="k">let</span> cad_boundary = (inst.flags &amp; FLAG_SMOOTH) != <span class="s">0u</span> &amp;&amp; source_edges[iid] != <span class="s">0xffffffffu</span>;
    <span class="k">let</span> crowd = select(density_taper(seg.facing, len, px), <span class="s">1.0</span>, cad_boundary || selected);
<span class="c">    // corner: sideways by the width, outward by the filter reach</span></code></pre></div>
<p><code>lessons/17/src/shaders/ribbon.wgsl</code> · edit · type this</p>
<p>Replaces the <code>if ((inst.flags &amp; FLAG_SELECTED) != 0u || (ed…</code> line in <code>fn vs_main</code> of <code>lessons/16/src/shaders/ribbon.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">if</span> (selected) {</code></pre></div>
<p>Replaces the 5 lines from <code>return o;</code> in <code>fn vs_main</code> of <code>lessons/16/src/shaders/ribbon.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    o.start_join = join_plane(seg.previous, iid);
    o.end_join = join_plane(iid, seg.next);
    <span class="k">return</span> o;
}

@vertex
<span class="k">fn</span> vs_main(@builtin(vertex_index) vid: <span class="k">u32</span>) -&gt; VsOut {
    <span class="k">return</span> stroke_vertex(vid, <span class="s">0u</span>);
}

@vertex
<span class="k">fn</span> vs_unselected(@builtin(vertex_index) vid: <span class="k">u32</span>) -&gt; VsOut {
    <span class="k">return</span> stroke_vertex(vid, <span class="s">1u</span>);
}

@vertex
<span class="k">fn</span> vs_selected(@builtin(vertex_index) vid: <span class="k">u32</span>) -&gt; VsOut {
    <span class="k">return</span> stroke_vertex(vid, <span class="s">2u</span>);
}

<span class="c">// How much of this pixel the stroke covers, 0..1, times the hairline alpha.</span>
<span class="k">fn</span> coverage(in: VsOut) -&gt; <span class="k">f32</span> {
    <span class="k">let</span> pixel = <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(in.pos.x, line.vp_h-in.pos.y);

<span class="c">    // past the start joint: the previous segment draws it</span>
    <span class="k">if</span> (dot(pixel-in.start_join.zw, in.start_join.xy) &lt; <span class="s">0.0</span>) {
        <span class="k">return</span> <span class="s">0.0</span>;
    }

<span class="c">    // past the end joint: the next segment draws it</span>
    <span class="k">if</span> (any(in.end_join.xy != <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>)) &amp;&amp; dot(pixel-in.end_join.zw, in.end_join.xy) &gt;= <span class="s">0.0</span>) {
        <span class="k">return</span> <span class="s">0.0</span>;
    }

<span class="c">    // distance from the pixel to the segment</span></code></pre></div>
<p>Copy each file from the lesson folder to the path shown.</p>
<p>Copy from <code>lessons/17/</code> (tooling this checkpoint needs but the course does not teach):</p>
<ul>
<li><code>lessons/17/examples/mk_mixed_solids.rs</code></li>
<li><code>lessons/17/examples/mk_selection_overlap.rs</code></li>
<li><code>lessons/17/examples/mk_stroke_joins.rs</code></li>
<li><code>lessons/17/src/selftest.rs</code></li>
<li><code>lessons/17/src/text_quality.rs</code></li>
<li><code>lessons/17/tests/interaction.cjs</code></li>
<li><code>lessons/17/tests/selection-overlap.py</code></li>
<li><code>lessons/17/tests/stroke-joins.py</code></li>
<li><code>lessons/17/tests/world-text.cjs</code></li>
</ul>
<h2 id="step-35-srcshaderssplatwgsl">Step 35 · src/shaders/splat.wgsl<a class="anchor" href="#/course/17-source-presentation#step-35-srcshaderssplatwgsl" aria-label="Link to this section">#</a></h2>
<p>Point projection writes the nearest visible cloud samples.</p>
<p><code>lessons/17/src/shaders/splat.wgsl</code> · edit · type this</p>
<p>Added after the <code>edl: f32,</code> line in <code>struct CloudUniform</code> of <code>lessons/16/src/shaders/splat.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    _pad0: <span class="k">f32</span>,<span class="c"> // padding</span>
    _pad1: <span class="k">f32</span>,<span class="c"> // padding</span>
    origin: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // top-left of this target in the canvas, px</span>
    frame: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // canvas size, px</span></code></pre></div>
<p>Replaces the 3 lines from <code>s.r = clamp(bitcast&lt;f32&gt;(table[base + 23u]) *…</code> in <code>fn project</code> of <code>lessons/16/src/shaders/splat.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">    // canvas pixel, shifted into this target</span>
    s.r = clamp(<span class="k">bitcast</span>&lt;<span class="k">f32</span>&gt;(table[base + <span class="s">23u</span>]) * cloud.frame.y / clip.w, r_min, <span class="s">8.0</span>);
    <span class="k">let</span> x = (ndc.x * <span class="s">0.5</span> + <span class="s">0.5</span>) * cloud.frame.x - cloud.origin.x;
    <span class="k">let</span> y = (<span class="s">0.5</span> - ndc.y * <span class="s">0.5</span>) * cloud.frame.y - cloud.origin.y;</code></pre></div>
<h2 id="step-36-srcshaderssplat_resolvewgsl">Step 36 · src/shaders/splat_resolve.wgsl<a class="anchor" href="#/course/17-source-presentation#step-36-srcshaderssplat_resolvewgsl" aria-label="Link to this section">#</a></h2>
<p>The resolve writes point color and depth into the scene.</p>
<p><code>lessons/17/src/shaders/splat_resolve.wgsl</code> · edit · type this</p>
<p>Added after the <code>edl: f32,</code> line in <code>struct CloudUniform</code> of <code>lessons/16/src/shaders/splat_resolve.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    _pad0: <span class="k">f32</span>,<span class="c"> // padding</span>
    _pad1: <span class="k">f32</span>,<span class="c"> // padding</span>
    origin: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // top-left of this target in the canvas, px</span>
    frame: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // canvas size, px</span></code></pre></div>
<h2 id="step-37-srcappinputrs">Step 37 · src/app/input.rs<a class="anchor" href="#/course/17-source-presentation#step-37-srcappinputrs" aria-label="Link to this section">#</a></h2>
<p>Input routes gestures and keyboard actions to State.</p>
<p><code>lessons/17/src/app/input.rs</code> · edit · type this</p>
<p>Replaces the 7 lines from <code>false</code> in <code>fn mouse</code> of <code>lessons/16/src/app/input.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                <span class="k">self</span>.shift = mods.state().shift_key();
                <span class="s">false</span>
            }
            WindowEvent::Focused(<span class="s">false</span>) =&gt; {
                <span class="k">self</span>.cancel();
                state.interacting = <span class="s">false</span>;
                <span class="s">true</span>
            }
            WindowEvent::Touch(t) =&gt; {
                <span class="k">let</span> scale = <span class="k">crate</span>::engine::gpu::view::surface_per_physical();
                <span class="k">let</span> t = &amp;winit::event::Touch {
                    location: winit::dpi::PhysicalPosition::new(
                        t.location.x * scale,
                        t.location.y * scale,
                    ),
                    ..*t
                };
                state.interacting = matches!(t.phase, TouchPhase::Started | TouchPhase::Moved);

                <span class="c">// otherwise the fingers move the camera</span></code></pre></div>
<p>Added after the <code>Act::Tap(at) =&gt; {</code> line in <code>fn mouse</code> of <code>lessons/16/src/app/input.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                        state.request_selection(at.<span class="s">0</span> <span class="k">as</span> u32, at.<span class="s">1</span> <span class="k">as</span> u32, <span class="s">false</span>, <span class="s">false</span>);</code></pre></div>
<p>Added after the <code>self.ctrl = false;</code> line in <code>fn cancel</code> of <code>lessons/16/src/app/input.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.shift = <span class="s">false</span>;</code></pre></div>
<p>Added after the <code>self.ctrl,</code> line in <code>fn left</code> of <code>lessons/16/src/app/input.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                    <span class="k">self</span>.ctrl &amp;&amp; <span class="k">self</span>.shift,</code></pre></div>
<p>Replaces the 12 lines from <code>#[cfg(target_arch = &quot;wasm32&quot;)]</code> of <code>lessons/16/src/app/input.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Physical pixels per CSS pixel.</span>
<span class="k">fn</span> device_pixel_ratio() -&gt; f64 {
    <span class="k">crate</span>::engine::gpu::view::device_pixel_ratio()</code></pre></div>
<h2 id="step-38-srclibrs">Step 38 · src/lib.rs<a class="anchor" href="#/course/17-source-presentation#step-38-srclibrs" aria-label="Link to this section">#</a></h2>
<p>The crate entry point connects the camera, scene and GPU owners.</p>
<p><code>lessons/17/src/lib.rs</code> · edit · type this</p>
<p>Replaces the 2 lines from <code>let win = web_sys::window()?;</code> in <code>fn desired_canvas_size</code> of <code>lessons/16/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// The canvas size in device pixels, \`None\` when zero.</span>
#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">fn</span> desired_canvas_size() -&gt; Option&lt;(u32, u32)&gt; {
    <span class="k">let</span> dpr = engine::gpu::view::device_pixel_ratio();</code></pre></div>
<h2 id="step-39-srcenginegputargetsrs">Step 39 · src/engine/gpu/targets.rs<a class="anchor" href="#/course/17-source-presentation#step-39-srcenginegputargetsrs" aria-label="Link to this section">#</a></h2>
<p>Targets own the depth and color attachments for a frame.</p>
<p><code>lessons/17/src/engine/gpu/targets.rs</code> · edit · type this</p>
<p>Added after the <code>const MSAA_PIXELS_DISCRETE: u32 = 9_000_000;</code> line of <code>lessons/16/src/engine/gpu/targets.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Device scale from which MSAA is off unless forced.</span>
<span class="k">const</span> MSAA_MAX_PIXEL_SCALE: f32 = <span class="s">2</span>.<span class="s">0</span>;

<span class="c">/// Pixels an integrated GPU may draw at 4x MSAA.</span></code></pre></div>
<p>Replaces <code>fn samples_for</code> in <code>lessons/16/src/engine/gpu/targets.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Sample count: 4x only with solids, within budget, below device scale 2.</span>
    <span class="k">pub</span> <span class="k">fn</span> samples_for(
        solid: bool,
        pixels: u32,
        forced: Option&lt;u32&gt;,
        budget: Option&lt;u32&gt;,
        pixel_scale: f32,
    ) -&gt; u32 {
        <span class="k">if</span> <span class="k">let</span> Some(s) = forced {
            <span class="k">return</span> <span class="k">if</span> s == <span class="s">4</span> { <span class="s">4</span> } <span class="k">else</span> { <span class="s">1</span> };
        }

        <span class="k">if</span> super::view::reduced() || pixel_scale &gt;= MSAA_MAX_PIXEL_SCALE {
            <span class="k">return</span> <span class="s">1</span>;
        }</code></pre></div>
<p>Replaces the 4 lines from <code>Targets::samples_for(true, 3840 * 2160, Some(…</code> in <code>fn msaa_follows_the_adapter</code> of <code>lessons/16/src/engine/gpu/targets.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            Targets::samples_for(<span class="s">true</span>, <span class="s">3840</span> * <span class="s">2160</span>, None, discrete, <span class="s">1</span>.<span class="s">0</span>),
            <span class="s">4</span>
        );
        assert_eq!(
            Targets::samples_for(<span class="s">true</span>, <span class="s">3840</span> * <span class="s">2160</span>, None, shared, <span class="s">1</span>.<span class="s">0</span>),
            <span class="s">1</span>
        );
        assert_eq!(
            Targets::samples_for(<span class="s">true</span>, <span class="s">1920</span> * <span class="s">1080</span>, None, shared, <span class="s">1</span>.<span class="s">0</span>),
            <span class="s">4</span>
        );
        assert_eq!(Targets::samples_for(<span class="s">true</span>, <span class="s">1</span>, None, software, <span class="s">1</span>.<span class="s">0</span>), <span class="s">1</span>);
        assert_eq!(Targets::samples_for(<span class="s">false</span>, <span class="s">1</span>, None, discrete, <span class="s">1</span>.<span class="s">0</span>), <span class="s">1</span>);
        assert_eq!(
            Targets::samples_for(<span class="s">true</span>, <span class="s">3840</span> * <span class="s">2160</span>, Some(<span class="s">1</span>), discrete, <span class="s">1</span>.<span class="s">0</span>),
            <span class="s">1</span>
        );
        assert_eq!(
            Targets::samples_for(<span class="s">false</span>, u32::MAX, Some(<span class="s">4</span>), software, <span class="s">1</span>.<span class="s">0</span>),
            <span class="s">4</span>
        );
        <span class="c">// device scale 2: 1x unless forced</span>
        assert_eq!(
            Targets::samples_for(<span class="s">true</span>, <span class="s">1920</span> * <span class="s">1080</span>, None, discrete, <span class="s">2</span>.<span class="s">0</span>),
            <span class="s">1</span>
        );
        assert_eq!(
            Targets::samples_for(<span class="s">true</span>, <span class="s">1920</span> * <span class="s">1080</span>, None, discrete, <span class="s">1</span>.<span class="s">5</span>),
            <span class="s">4</span>
        );
        assert_eq!(
            Targets::samples_for(<span class="s">true</span>, <span class="s">1920</span> * <span class="s">1080</span>, Some(<span class="s">4</span>), discrete, <span class="s">2</span>.<span class="s">0</span>),
            <span class="s">4</span>
        );</code></pre></div>
<p>Replaces the 3 lines from <code>assert_eq!(Targets::samples_for(true, 2560 *…</code> in <code>fn the_browser_arm_is_not_the_integrated_one</code> of <code>lessons/16/src/engine/gpu/targets.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        assert_eq!(
            Targets::samples_for(<span class="s">true</span>, <span class="s">2560</span> * <span class="s">1440</span>, None, browser, <span class="s">1</span>.<span class="s">0</span>),
            <span class="s">4</span>
        );
        assert_eq!(
            Targets::samples_for(<span class="s">true</span>, <span class="s">2560</span> * <span class="s">1440</span>, None, shared, <span class="s">1</span>.<span class="s">0</span>),
            <span class="s">1</span>
        );
        assert_eq!(
            Targets::samples_for(<span class="s">true</span>, <span class="s">3840</span> * <span class="s">2160</span>, None, browser, <span class="s">1</span>.<span class="s">0</span>),
            <span class="s">1</span>
        );</code></pre></div>
<h2 id="step-40-srcapprouters">Step 40 · src/app/route.rs<a class="anchor" href="#/course/17-source-presentation#step-40-srcapprouters" aria-label="Link to this section">#</a></h2>
<p>Route helpers read viewer options from the page URL.</p>
<p><code>lessons/17/src/app/route.rs</code> · edit · type this</p>
<p>Added after the <code>}</code> line of <code>lessons/16/src/app/route.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Reload once at reduced quality after a lost GPU device.</span>
#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">pub</span> <span class="k">fn</span> recover_from_device_loss(message: &amp;str) -&gt; bool {
    <span class="k">if</span> !message.contains(&quot;<span class="s">device lost</span>&quot;) || query(&quot;<span class="s">recovered</span>&quot;).is_some() {
        <span class="k">return</span> <span class="s">false</span>;
    }

    <span class="k">let</span> Some(window) = web_sys::window() <span class="k">else</span> {
        <span class="k">return</span> <span class="s">false</span>;
    };
    <span class="k">let</span> location = window.location();
    <span class="k">let</span> Ok(search) = location.search() <span class="k">else</span> {
        <span class="k">return</span> <span class="s">false</span>;
    };
    <span class="k">let</span> kept: Vec&lt;&amp;str&gt; = search
        .strip_prefix('<span class="s">?</span>')
        .unwrap_or(&quot;&quot;)
        .split('<span class="s">&amp;</span>')
        .filter(|pair| {
            !pair.is_empty()
                &amp;&amp; ![&quot;<span class="s">dpr</span>&quot;, &quot;<span class="s">msaa</span>&quot;, &quot;<span class="s">recovered</span>&quot;]
                    .iter()
                    .any(|name| pair.starts_with(&amp;format!(&quot;{<span class="s">name</span>}<span class="s">=</span>&quot;)) || *pair == *name)
        })
        .collect();
    <span class="k">let</span> <span class="k">mut</span> query = kept.join(&quot;<span class="s">&amp;</span>&quot;);

    <span class="k">if</span> !query.is_empty() {
        query.push('<span class="s">&amp;</span>');
    }

    query.push_str(&quot;<span class="s">dpr=1&amp;msaa=1&amp;recovered=1</span>&quot;);
    <span class="k">let</span> Ok(hash) = location.hash() <span class="k">else</span> {
        <span class="k">return</span> <span class="s">false</span>;
    };
    <span class="k">let</span> Ok(path) = location.pathname() <span class="k">else</span> {
        <span class="k">return</span> <span class="s">false</span>;
    };
    log::warn!(&quot;{<span class="s">message</span>}<span class="s">; reloading at device scale 1 without antialiasing</span>&quot;);
    location.replace(&amp;format!(&quot;{<span class="s">path</span>}<span class="s">?</span>{<span class="s">query</span>}{<span class="s">hash</span>}&quot;)).is_ok()
}

<span class="c">/// The recovery notice, if this page reloaded after a device loss.</span>
#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">pub</span> <span class="k">fn</span> recovered_notice() -&gt; Option&lt;&amp;'static str&gt; {
    query(&quot;<span class="s">recovered</span>&quot;).map(|_| {
        &quot;<span class="s">The GPU ran out of memory at full resolution: drawing at device scale 1 without antialiasing</span>&quot;
    })
}</code></pre></div>
<h2 id="step-41-srcappfeedbackrs">Step 41 · src/app/feedback.rs<a class="anchor" href="#/course/17-source-presentation#step-41-srcappfeedbackrs" aria-label="Link to this section">#</a></h2>
<p>Feedback publishes status and panel information from the same application state.</p>
<p><code>lessons/17/src/app/feedback.rs</code> · edit · type this</p>
<p>Added after the <code>pub fn status(message: &amp;str) {</code> line of <code>lessons/16/src/app/feedback.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">// an empty message shows the reload notice, if any</span>
    #[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
    <span class="k">let</span> message = <span class="k">if</span> message.is_empty() {
        super::route::recovered_notice().unwrap_or(message)
    } <span class="k">else</span> {
        message
    };</code></pre></div>
<h2 id="step-42-srcstaters">Step 42 · src/state.rs<a class="anchor" href="#/course/17-source-presentation#step-42-srcstaters" aria-label="Link to this section">#</a></h2>
<p>State coordinates input, selection and frame requests.</p>
<p><code>lessons/17/src/state.rs</code> · edit · type this</p>
<p>Delete <code>fn cloud_query_awaiting_gpu</code> from <code>lessons/16/src/state.rs</code>.</p>
<p>Delete <code>fn label_center</code> from <code>lessons/16/src/state.rs</code>.</p>
<h2 id="step-43-srcenginegpuselection_outliners">Step 43 · src/engine/gpu/selection_outline.rs<a class="anchor" href="#/course/17-source-presentation#step-43-srcenginegpuselection_outliners" aria-label="Link to this section">#</a></h2>
<p>Remove this file; its replacement is now part of the rendering modules.</p>
<p>Delete <code>src/engine/gpu/selection_outline.rs</code> (it exists in <code>lessons/16/</code>, not in <code>lessons/17/</code>).</p>
<h2 id="step-44-srcshadersselection_outlinewgsl">Step 44 · src/shaders/selection_outline.wgsl<a class="anchor" href="#/course/17-source-presentation#step-44-srcshadersselection_outlinewgsl" aria-label="Link to this section">#</a></h2>
<p>Remove this file; its replacement is now part of the rendering modules.</p>
<p>Delete <code>src/shaders/selection_outline.wgsl</code> (it exists in <code>lessons/16/</code>, not in <code>lessons/17/</code>).</p>
<h2 id="step-45-srcenginegpumodrs">Step 45 · src/engine/gpu/mod.rs<a class="anchor" href="#/course/17-source-presentation#step-45-srcenginegpumodrs" aria-label="Link to this section">#</a></h2>
<p>The GPU owner connects buffers, pipelines and frame resources.</p>
<p><code>lessons/17/src/engine/gpu/mod.rs</code> · edit · type this</p>
<p>Added after the <code>pub mod device;</code> line of <code>lessons/16/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> <span class="k">mod</span> faces;</code></pre></div>
<p>Added after the <code>pub mod splat;</code> line in <code>mod splat</code> of <code>lessons/16/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> <span class="k">mod</span> surface_outline;</code></pre></div>
<p>Replaces the <code>pub selection_outline: selection_outline::Sel…</code> line in <code>struct Gpu</code> of <code>lessons/16/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> selection_outline: surface_outline::SurfaceOutline,
    <span class="k">pub</span> solid_outline: surface_outline::SurfaceOutline,</code></pre></div>
<p>Replaces the <code>let (outline_buffers, outline_textures) = sel…</code> line in <code>fn allocated_bytes</code> of <code>lessons/16/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> (<span class="k">mut</span> outline_buffers, <span class="k">mut</span> outline_textures) = <span class="k">self</span>.selection_outline.allocated_bytes();
        <span class="k">let</span> (buffers, textures) = <span class="k">self</span>.solid_outline.allocated_bytes();
        outline_buffers += buffers;
        outline_textures += textures;</code></pre></div>
<p>Replaces the <code>let selection_outline = selection_outline::Se…</code> line in <code>fn build</code> of <code>lessons/16/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> selection_outline = surface_outline::SurfaceOutline::new(
            &amp;ctx,
            target,
            surface_outline::OutlineKind::Selected,
        );
        <span class="k">let</span> solid_outline = surface_outline::SurfaceOutline::new(
            &amp;ctx,
            target,
            surface_outline::OutlineKind::AllSolids,
        );</code></pre></div>
<p>Added after the <code>selection_outline,</code> line in <code>fn build</code> of <code>lessons/16/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            solid_outline,</code></pre></div>
<p>Added after the <code>self.selection_outline.retarget(&amp;self.ctx, ta…</code> line in <code>fn retarget</code> of <code>lessons/16/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">self</span>.solid_outline.retarget(&amp;<span class="k">self</span>.ctx, target);</code></pre></div>
<p>Added after the <code>self.msaa_budget(),</code> line in <code>fn msaa_now</code> of <code>lessons/16/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">self</span>.config.width <span class="k">as</span> f32 / <span class="k">self</span>.logical_size[<span class="s">0</span>].max(<span class="s">1</span>.<span class="s">0</span>) <span class="k">as</span> f32,</code></pre></div>
<p>Replaces the 7 lines from <code>self.arena.reset();</code> in <code>fn reset</code> of <code>lessons/16/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.arena.reset(&amp;<span class="k">self</span>.ctx);
        <span class="k">self</span>.segments.reset();
        <span class="k">self</span>.glyphs.reset();
        <span class="k">self</span>.controls.reset();
        <span class="k">self</span>.control_net.reset();
        <span class="k">self</span>.text.reset();
        <span class="k">self</span>.selection_outline.reset();
        <span class="k">self</span>.solid_outline.reset();</code></pre></div>
<p>Replaces this block of <code>lessons/16/src/engine/gpu/mod.rs</code>:</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.selection_outline.reset();
        <span class="k">self</span>.pick.cancel();</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.solid_outline.reset();</code></pre></div>
<p>Added after the <code>pub fn set_selected(&amp;mut self, row: u32, on:…</code> line in <code>impl Gpu</code> of <code>lessons/16/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.segments.set_selected(row, on);</code></pre></div>
<h2 id="step-46-srcenginegpurenderrs">Step 46 · src/engine/gpu/render.rs<a class="anchor" href="#/course/17-source-presentation#step-46-srcenginegpurenderrs" aria-label="Link to this section">#</a></h2>
<p>The frame encoder orders face, ink, picking and overlay passes.</p>
<p><code>lessons/17/src/engine/gpu/render.rs</code> · edit · type this</p>
<p>Replaces the 9 lines from <code>self.arena.face_count() &gt; 0,</code> in <code>fn encode_frame</code> of <code>lessons/16/src/engine/gpu/render.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">self</span>.view.show_outlines &amp;&amp; <span class="k">self</span>.arena.face_count() &gt; <span class="s">0</span>,
        ) {
            <span class="k">let</span> b = Binds {
                mvp: &amp;<span class="k">self</span>.frame.mvp_group, <span class="c">// the camera matrix</span>
                line: &amp;<span class="k">self</span>.frame.line_group, <span class="c">// pen settings</span>
                instances: &amp;<span class="k">self</span>.objects.group,
            };
            {
                <span class="k">let</span> <span class="k">mut</span> pass = <span class="k">self</span>.selection_outline.begin_mask(encoder, &amp;<span class="k">self</span>.targets);
                draws += <span class="k">self</span>.arena.draw_selection_mask(&amp;<span class="k">mut</span> pass, &amp;b);
                draws += <span class="k">self</span>.arena.source_faces.draw_mask(&amp;<span class="k">mut</span> pass, &amp;b);
            }
            <span class="k">self</span>.selection_outline.encode_pool(encoder);
        }

        <span class="k">if</span> <span class="k">self</span>.solid_outline.prepare(
            &amp;<span class="k">self</span>.ctx,
            (<span class="k">self</span>.config.width, <span class="k">self</span>.config.height),
            <span class="k">self</span>.targets.samples,
            <span class="k">self</span>.logical_size[<span class="s">0</span>],
            <span class="k">self</span>.view.show_outlines &amp;&amp; <span class="k">self</span>.arena.face_count() &gt; <span class="s">0</span>,
        ) {
            <span class="k">let</span> b = Binds {
                mvp: &amp;<span class="k">self</span>.frame.mvp_group, <span class="c">// the camera matrix</span>
                line: &amp;<span class="k">self</span>.frame.line_group, <span class="c">// pen settings</span>
                instances: &amp;<span class="k">self</span>.objects.group,
            };
            {
                <span class="k">let</span> <span class="k">mut</span> pass = <span class="k">self</span>.solid_outline.begin_mask(encoder, &amp;<span class="k">self</span>.targets);
                draws += <span class="k">self</span>.arena.draw_solid_mask(&amp;<span class="k">mut</span> pass, &amp;b);
            }
            <span class="k">self</span>.solid_outline.encode_pool(encoder);</code></pre></div>
<p>Replaces the 19 lines from <code>let basic = Binds {</code> in <code>fn scene_list</code> of <code>lessons/16/src/engine/gpu/render.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> basic = <span class="k">self</span>.frame.binds(&amp;<span class="k">self</span>.objects.group);
        <span class="k">let</span> b = <span class="k">self</span>.frame.binds(&amp;<span class="k">self</span>.objects.ink_group);
        <span class="k">let</span> <span class="k">mut</span> draws = <span class="k">self</span>.arena.source_faces.draw_highlight(pass, &amp;basic);
        draws += <span class="k">self</span>.arena.draw_print(pass, &amp;basic);
        draws += <span class="k">self</span>
            .segments
            .draw_unselected(pass, &amp;b, v.show_mesh_edges, v.show_lines);
        <span class="c">// selected mesh edges now, its curves after the outline</span>
        draws += <span class="k">self</span>
            .segments
            .draw_selected(pass, &amp;b, v.show_mesh_edges, <span class="s">false</span>);
        draws += <span class="k">self</span>
            .solid_outline
            .draw_combined(&amp;<span class="k">self</span>.selection_outline, pass);
        <span class="c">// selected curves over the outline</span>
        draws += <span class="k">self</span>.segments.draw_selected(pass, &amp;b, <span class="s">false</span>, v.show_lines);</code></pre></div>
<p>Delete the 59 lines from <code>let window = match at {</code> in <code>fn scene_list</code> of <code>lessons/16/src/engine/gpu/render.rs</code>.</p>
<p>Replaces the 59 lines from <code>let window = match at {</code> in <code>fn scene_list</code> of <code>lessons/16/src/engine/gpu/render.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="c">// draw only the window around the cursor, plus its halo</span>
        <span class="k">let</span> window = at.map(|position| <span class="k">self</span>.pick.window(position, size));
        <span class="k">let</span> view = <span class="k">self</span>.pick.view_for(at, size);
        <span class="k">self</span>.frame.write_pick(&amp;<span class="k">self</span>.ctx, view, size);
        <span class="c">// the window inside the drawn area</span>
        <span class="k">let</span> inner = window.map(|window| {
            (
                window.x.saturating_sub(view.x),
                window.y.saturating_sub(view.y),
                window.w.min(view.w),
                window.h.min(view.h),
            )
        });
        <span class="k">let</span> basic = <span class="k">self</span>.frame.pick_binds(&amp;<span class="k">self</span>.objects.group);

        <span class="c">// source point query: faces and clouds, then the source dots</span>
        <span class="k">if</span> <span class="k">self</span>.pick.source_query() {
            <span class="k">if</span> !<span class="k">self</span>.pick.source_initialized() {
                <span class="k">let</span> <span class="k">mut</span> pass = <span class="k">self</span>.pick.begin_pass(&amp;<span class="k">self</span>.ctx, encoder, view);

                <span class="k">if</span> <span class="k">let</span> Some((x, y, w, h)) = inner {
                    pass.set_scissor_rect(x, y, w, h);
                }

                <span class="k">self</span>.arena.draw_face_ids(&amp;<span class="k">mut</span> pass, &amp;basic);
                <span class="k">self</span>.splat.draw_ids(&amp;<span class="k">mut</span> pass, &amp;<span class="k">self</span>.frame.pick_cloud_group);
            }

            {
                <span class="k">let</span> <span class="k">mut</span> pass = <span class="k">self</span>.pick.begin_source(encoder);

                <span class="k">if</span> <span class="k">let</span> Some((x, y, w, h)) = inner {
                    pass.set_scissor_rect(x, y, w, h);
                }

                <span class="k">let</span> source = <span class="k">self</span>.frame.pick_binds(&amp;<span class="k">self</span>.objects.ink_group);
                <span class="k">self</span>.controls.draw_source_ids(&amp;<span class="k">mut</span> pass, &amp;source);
            }

            <span class="k">if</span> <span class="k">let</span> Some(at) = at {
                <span class="k">self</span>.pick.copy_window(&amp;<span class="k">self</span>.ctx, encoder, at, size);
            }

            <span class="k">return</span>;
        }

        {
            <span class="c">// faces and clouds over the whole area, halo included</span>
            <span class="k">let</span> <span class="k">mut</span> pass = <span class="k">self</span>.pick.begin_pass(&amp;<span class="k">self</span>.ctx, encoder, view);

            <span class="k">if</span> mode == PickMode::Component {
                <span class="k">self</span>.arena.draw_component_ids(&amp;<span class="k">mut</span> pass, &amp;basic);
            } <span class="k">else</span> {
                <span class="k">self</span>.arena.draw_face_ids(&amp;<span class="k">mut</span> pass, &amp;basic);
            }

            <span class="k">self</span>.splat.draw_ids(&amp;<span class="k">mut</span> pass, &amp;<span class="k">self</span>.frame.pick_cloud_group);</code></pre></div>
<p>Replaces the 14 lines from <code>mvp: &amp;self.frame.mvp_group,</code> in <code>fn scene_list</code> of <code>lessons/16/src/engine/gpu/render.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            mvp: &amp;<span class="k">self</span>.frame.pick_mvp_group,
            line: &amp;<span class="k">self</span>.frame.pick_line_group, <span class="c">// pen settings for picking</span>
            instances: &amp;group, <span class="c">// per-object rows</span>
        };
        {
            <span class="k">let</span> <span class="k">mut</span> pass = <span class="k">self</span>.pick.begin_ink(encoder);

            <span class="k">if</span> <span class="k">let</span> Some((x, y, w, h)) = inner {
                pass.set_scissor_rect(x, y, w, h);
            }

            <span class="c">// which ink may answer depends on the mode</span>
            <span class="k">match</span> mode {
                PickMode::Edge | PickMode::Component =&gt; {</code></pre></div>
<p>Replaces the 4 lines from <code>}</code> in <code>fn scene_list</code> of <code>lessons/16/src/engine/gpu/render.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="c">// text ids in every mode</span>
            <span class="k">self</span>.text
                .draw_ids(&amp;<span class="k">mut</span> pass, &amp;<span class="k">self</span>.frame.pick_transform_group);
        }

        <span class="k">if</span> <span class="k">let</span> Some(at) = at {
            <span class="k">self</span>.pick.copy_window(&amp;<span class="k">self</span>.ctx, encoder, at, size);</code></pre></div>
<h2 id="check">Check<a class="anchor" href="#/course/17-source-presentation#check" aria-label="Link to this section">#</a></h2>
<p>Run <code>trunk serve</code> in <code>lessons/17/</code> and open <a href="http://127.0.0.1:8770/" target="_blank" rel="noopener">http://127.0.0.1:8770/</a>.</p>
<p>Expected: Source faces and scene text become selectable, and optional outlines surround visible solids; status: <strong>Face N selected</strong>.</p>
<p><img src="/session/docs/course/docs/screenshots/17-face-silhouette.png" alt="Checkpoint 17. Left: Ctrl + Shift + click inside the mesh selects one source face, the rest of the object stays grey. Middle: the selected BRep with its silhouette after pressing O, one black border of uniform width around the yellow fill. Right: without the silhouette only the yellow strokes remain." loading="lazy" decoding="async"></p>
<p>If it fails:</p>
<ul>
<li>A face click selects an edge: edges intentionally win where both are hit.</li>
<li>Text stays visible after hiding: its row is missing from the scene visibility path.</li>
<li>Joints show dark dots: connected segments overlap instead of sharing a join partition.</li>
</ul>
<h2 id="what-changed">What changed<a class="anchor" href="#/course/17-source-presentation#what-changed" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>lessons/17/src/
├── app/
│   ├── inspection/
│   │   └── source_memory.rs
│   ├── walk/
│   │   ├── bounds.rs
│   │   ├── brep.rs  ~
│   │   ├── brep_edges.rs  ~
│   │   ├── brep_orient.rs
│   │   ├── cloud.rs
│   │   ├── curves.rs  ~
│   │   ├── encode.rs
│   │   ├── frames.rs
│   │   ├── mesh.rs  ~
│   │   ├── mesh_ink.rs
│   │   ├── mesh_topology.rs
│   │   ├── mod.rs
│   │   └── points.rs
│   ├── cloud_query.rs
│   ├── decode.rs
│   ├── feedback.rs  ~
│   ├── fetch.rs
│   ├── input.rs  ~
│   ├── inspection.rs  ~
│   ├── knobs.rs
│   ├── live.rs
│   ├── loader.rs
│   ├── manifest.rs  ~
│   ├── mod.rs
│   ├── route.rs  ~
│   ├── scene.rs  ~
│   ├── scene_text.rs  +
│   ├── selection.rs  ~
│   ├── stream.rs
│   ├── touch.rs
│   └── validate.rs
├── engine/
│   ├── gpu/
│   │   ├── arena.rs  ~
│   │   ├── backdrop.rs
│   │   ├── buffers.rs
│   │   ├── cloud.rs
│   │   ├── device.rs
│   │   ├── faces.rs  +
│   │   ├── frame.rs
│   │   ├── glyphs.rs
│   │   ├── instance.rs  ~
│   │   ├── lod.rs
│   │   ├── mod.rs  ~
│   │   ├── objects.rs  ~
│   │   ├── pick.rs  ~
│   │   ├── present.rs
│   │   ├── render.rs  ~
│   │   ├── segments.rs  ~
│   │   ├── splat.rs
│   │   ├── surface_outline.rs  +
│   │   ├── targets.rs  ~
│   │   ├── text.rs  ~
│   │   ├── text_outline.rs
│   │   ├── text_plane.rs  ~
│   │   ├── text_plate.rs  ~
│   │   ├── upload.rs
│   │   └── view.rs  ~
│   ├── pipelines/
│   │   ├── layouts.rs
│   │   └── mod.rs
│   ├── mod.rs
│   ├── performance.rs
│   └── text.rs  ~
├── shaders/
│   ├── background.wgsl
│   ├── glyph.wgsl
│   ├── grid.wgsl
│   ├── ink_visibility.wgsl
│   ├── normals.wgsl
│   ├── physical.wgsl
│   ├── ribbon.wgsl  ~
│   ├── scene.wgsl
│   ├── sphere.wgsl
│   ├── splat.wgsl  ~
│   ├── splat_resolve.wgsl  ~
│   ├── surface_outline.wgsl  +
│   ├── text_outline.wgsl
│   ├── text_plane.wgsl  ~
│   ├── text_plate.wgsl  ~
│   └── triangle.wgsl  ~
├── state/
│   ├── cloud_query.rs  +
│   └── text.rs  +
├── camera.rs
├── lib.rs  ~
└── state.rs  ~</code></pre></div>
<p><code>+</code> new in this lesson · <code>~</code> changed in this lesson</p>
<p>Every file at this point: <code>lessons/17/</code>.</p>
<h2 id="next">Next<a class="anchor" href="#/course/17-source-presentation#next" aria-label="Link to this section">#</a></h2>
<p><a href="#/course/18-finite-visibility">18 · Finite-triangle visibility and maintained viewer convergence</a></p>
<h2 id="expected-viewer-result">Expected viewer result<a class="anchor" href="#/course/17-source-presentation#expected-viewer-result" aria-label="Link to this section">#</a></h2>
<p>Select a face, read the authored text, and see silhouettes on the teapot.</p>
<p><a href="/session/docs/course/docs/screenshots/17.png"><img src="/session/docs/course/docs/screenshots/17.png" alt="Full viewer result for 17 source presentation" loading="lazy" decoding="async"></a></p>
`,toc:[{level:2,id:"step-1-srcenginegpufacesrs",text:"Step 1 · src/engine/gpu/faces.rs"},{level:2,id:"step-2-srcshaderstrianglewgsl",text:"Step 2 · src/shaders/triangle.wgsl"},{level:2,id:"step-3-srcenginegpuarenars",text:"Step 3 · src/engine/gpu/arena.rs"},{level:2,id:"step-4-srcappwalkmeshrs",text:"Step 4 · src/app/walk/mesh.rs"},{level:2,id:"step-5-srcappwalkbreprs",text:"Step 5 · src/app/walk/brep.rs"},{level:2,id:"step-6-srcappselectionrs",text:"Step 6 · src/app/selection.rs"},{level:2,id:"step-7-srcenginegpupickrs",text:"Step 7 · src/engine/gpu/pick.rs"},{level:2,id:"step-8-srcappinputrs",text:"Step 8 · src/app/input.rs"},{level:2,id:"step-9-srcstaters",text:"Step 9 · src/state.rs"},{level:2,id:"step-10-srcenginetextrs",text:"Step 10 · src/engine/text.rs"},{level:2,id:"step-11-srcappmanifestrs",text:"Step 11 · src/app/manifest.rs"},{level:2,id:"step-12-srcappscene_textrs",text:"Step 12 · src/app/scene_text.rs"},{level:2,id:"step-13-srcappsceners",text:"Step 13 · src/app/scene.rs"},{level:2,id:"step-14-srcstatetextrs",text:"Step 14 · src/state/text.rs"},{level:2,id:"step-15-srcstatecloud_queryrs",text:"Step 15 · src/state/cloud_query.rs"},{level:2,id:"step-16-srcstaters",text:"Step 16 · src/state.rs"},{level:2,id:"step-17-srcenginegpuobjectsrs",text:"Step 17 · src/engine/gpu/objects.rs"},{level:2,id:"step-18-srcenginegputext_platers",text:"Step 18 · src/engine/gpu/text_plate.rs"},{level:2,id:"step-19-srcshaderstext_platewgsl",text:"Step 19 · src/shaders/text_plate.wgsl"},{level:2,id:"step-20-srcenginegputext_planers",text:"Step 20 · src/engine/gpu/text_plane.rs"},{level:2,id:"step-21-srcshaderstext_planewgsl",text:"Step 21 · src/shaders/text_plane.wgsl"},{level:2,id:"step-22-srcenginegputextrs",text:"Step 22 · src/engine/gpu/text.rs"},{level:2,id:"step-23-srcappinspectionrs",text:"Step 23 · src/app/inspection.rs"},{level:2,id:"step-24-srcenginegpusurface_outliners",text:"Step 24 · src/engine/gpu/surface_outline.rs"},{level:2,id:"step-25-srcshaderssurface_outlinewgsl",text:"Step 25 · src/shaders/surface_outline.wgsl"},{level:2,id:"step-26-srcenginegpuarenars",text:"Step 26 · src/engine/gpu/arena.rs"},{level:2,id:"step-27-srcenginegpuviewrs",text:"Step 27 · src/engine/gpu/view.rs"},{level:2,id:"step-28-srcappinputrs",text:"Step 28 · src/app/input.rs"},{level:2,id:"step-29-srcappinspectionrs",text:"Step 29 · src/app/inspection.rs"},{level:2,id:"step-30-srcappwalkcurvesrs",text:"Step 30 · src/app/walk/curves.rs"},{level:2,id:"step-31-srcappwalkbrep_edgesrs",text:"Step 31 · src/app/walk/brep_edges.rs"},{level:2,id:"step-32-srcenginegpusegmentsrs",text:"Step 32 · src/engine/gpu/segments.rs"},{level:2,id:"step-33-srcenginegpuinstancers",text:"Step 33 · src/engine/gpu/instance.rs"},{level:2,id:"step-34-srcshadersribbonwgsl",text:"Step 34 · src/shaders/ribbon.wgsl"},{level:2,id:"step-35-srcshaderssplatwgsl",text:"Step 35 · src/shaders/splat.wgsl"},{level:2,id:"step-36-srcshaderssplat_resolvewgsl",text:"Step 36 · src/shaders/splat_resolve.wgsl"},{level:2,id:"step-37-srcappinputrs",text:"Step 37 · src/app/input.rs"},{level:2,id:"step-38-srclibrs",text:"Step 38 · src/lib.rs"},{level:2,id:"step-39-srcenginegputargetsrs",text:"Step 39 · src/engine/gpu/targets.rs"},{level:2,id:"step-40-srcapprouters",text:"Step 40 · src/app/route.rs"},{level:2,id:"step-41-srcappfeedbackrs",text:"Step 41 · src/app/feedback.rs"},{level:2,id:"step-42-srcstaters",text:"Step 42 · src/state.rs"},{level:2,id:"step-43-srcenginegpuselection_outliners",text:"Step 43 · src/engine/gpu/selection_outline.rs"},{level:2,id:"step-44-srcshadersselection_outlinewgsl",text:"Step 44 · src/shaders/selection_outline.wgsl"},{level:2,id:"step-45-srcenginegpumodrs",text:"Step 45 · src/engine/gpu/mod.rs"},{level:2,id:"step-46-srcenginegpurenderrs",text:"Step 46 · src/engine/gpu/render.rs"},{level:2,id:"check",text:"Check"},{level:2,id:"what-changed",text:"What changed"},{level:2,id:"next",text:"Next"},{level:2,id:"expected-viewer-result",text:"Expected viewer result"}]};export{s as default};
