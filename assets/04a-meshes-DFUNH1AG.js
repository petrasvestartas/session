const s={title:"04a · Meshes on the GPU",html:`<h1 id="04a-meshes-on-the-gpu">04a · Meshes on the GPU<a class="anchor" href="#/course/04a-meshes#04a-meshes-on-the-gpu" aria-label="Link to this section">#</a></h1>
<p>A blue triangle draws from mesh buffers while the camera still orbits and zooms.</p>
<p><img src="/session/docs/course/docs/illustrations/arena.svg" alt="One growable arena holds every mesh's vertices and a parallel table gives every vertex its object row; a mesh is a range of indices, and a draw binds both vertex buffers, binds one index run and calls draw_indexed." loading="lazy" decoding="async"></p>
<h2 id="step-1-srcenginegpubuffersrs">Step 1 · src/engine/gpu/buffers.rs<a class="anchor" href="#/course/04a-meshes#step-1-srcenginegpubuffersrs" aria-label="Link to this section">#</a></h2>
<p>New file: buffer usage flags, a GPU buffer that grows by half when full, and small buffer helpers.</p>
<p><code>lessons/04a/src/engine/gpu/buffers.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> bytemuck::Pod; <span class="c">// Pod = plain old data: no pointers, so its bytes can go straight to the GPU</span>
<span class="k">use</span> wgpu::util::DeviceExt;

<span class="c">/// Device and queue travel together; one borrow of this passes both.</span>
<span class="k">pub</span> <span class="k">struct</span> GpuCtx {
    <span class="k">pub</span> device: wgpu::Device,
    <span class="k">pub</span> queue: wgpu::Queue,
}

<span class="c">/// Row buffers: COPY_DST so rows can be written, COPY_SRC so growing can copy them on the GPU.</span>
<span class="k">pub</span> <span class="k">const</span> ROWS: wgpu::BufferUsages = wgpu::BufferUsages::STORAGE
    .union(wgpu::BufferUsages::COPY_DST)
    .union(wgpu::BufferUsages::COPY_SRC);

<span class="c">/// A vertex buffer holds the corner positions; the GPU walks it once per drawn vertex.</span>
<span class="k">pub</span> <span class="k">const</span> VERTS: wgpu::BufferUsages = wgpu::BufferUsages::VERTEX
    .union(wgpu::BufferUsages::COPY_DST)
    .union(wgpu::BufferUsages::COPY_SRC);

<span class="c">/// An index buffer lists the corners of each triangle, so a cube stores 8 corners for its 12 triangles, not 36.</span>
<span class="k">pub</span> <span class="k">const</span> INDICES: wgpu::BufferUsages = wgpu::BufferUsages::INDEX
    .union(wgpu::BufferUsages::COPY_DST)
    .union(wgpu::BufferUsages::COPY_SRC);

<span class="c">/// A GPU buffer cannot grow in place: growing allocates a bigger one and copies, so each growth adds half to keep that rare.</span>
<span class="k">pub</span> <span class="k">struct</span> GrowBuf {
    <span class="k">pub</span> buf: wgpu::Buffer,
    len: u32, <span class="c">// rows in use</span>
    cap: u64, <span class="c">// rows allocated</span>
    stride: u64, <span class="c">// bytes per row</span>
    usage: wgpu::BufferUsages,
    label: &amp;'static str, <span class="c">// name shown in GPU errors</span>
}</code></pre></div>
<p><code>lessons/04a/src/engine/gpu/buffers.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">impl</span> GrowBuf {
    <span class="c">/// Room for one row, since a zero-size buffer cannot be bound.</span>
    <span class="k">pub</span> <span class="k">fn</span> new(ctx: &amp;GpuCtx, label: &amp;'static str, stride: u64, usage: wgpu::BufferUsages) -&gt; <span class="k">Self</span> {
        <span class="k">let</span> buf = zeroed_buffer(&amp;ctx.device, label, stride, usage);

        <span class="k">Self</span> {
            buf,
            len: <span class="s">0</span>,
            cap: <span class="s">1</span>,
            stride,
            usage,
            label,
        }
    }

    <span class="c">/// true = the buffer was replaced, so every bind group that points at it must be rebuilt.</span>
    <span class="k">pub</span> <span class="k">fn</span> append&lt;T: Pod&gt;(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, data: &amp;[T]) -&gt; bool {
        debug_assert_eq!(std::mem::size_of::&lt;T&gt;() <span class="k">as</span> u64, <span class="k">self</span>.stride); <span class="c">// checked in debug builds only, free in release</span>

        <span class="k">if</span> data.is_empty() {
            <span class="k">return</span> <span class="s">false</span>;
        }

        <span class="c">// rows needed after the append</span>
        <span class="k">let</span> need = <span class="k">self</span>.len <span class="k">as</span> u64 + data.len() <span class="k">as</span> u64;
        <span class="k">let</span> grew = need &gt; <span class="k">self</span>.cap;

        <span class="k">if</span> grew {
            <span class="c">// grow by at least half</span>
            <span class="k">self</span>.grow(ctx, need.max(<span class="k">self</span>.cap * <span class="s">3</span> / <span class="s">2</span>));
        }

        <span class="c">// write the new rows after the existing ones</span>
        ctx.queue.write_buffer(
            &amp;<span class="k">self</span>.buf,
            <span class="k">self</span>.len <span class="k">as</span> u64 * <span class="k">self</span>.stride,
            bytemuck::cast_slice(data),
        );
        <span class="k">self</span>.len += data.len() <span class="k">as</span> u32;
        grew
    }

    <span class="c">/// Move to a bigger buffer, copying the rows in use.</span>
    <span class="k">fn</span> grow(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, new_cap: u64) {
        <span class="k">let</span> nb = zeroed_buffer(&amp;ctx.device, <span class="k">self</span>.label, new_cap * <span class="k">self</span>.stride, <span class="k">self</span>.usage);

        <span class="k">if</span> <span class="k">self</span>.len &gt; <span class="s">0</span> {
            <span class="c">// copy old rows on the GPU, no round trip</span>
            <span class="k">let</span> <span class="k">mut</span> enc = ctx.device.create_command_encoder(&amp;Default::default());
            enc.copy_buffer_to_buffer(&amp;<span class="k">self</span>.buf, <span class="s">0</span>, &amp;nb, <span class="s">0</span>, <span class="k">self</span>.len <span class="k">as</span> u64 * <span class="k">self</span>.stride);
            ctx.queue.submit([enc.finish()]);
        }

        <span class="k">self</span>.buf = nb;
        <span class="k">self</span>.cap = new_cap;
    }

    <span class="c">/// Overwrite existing rows starting at \`at\`.</span>
    <span class="k">pub</span> <span class="k">fn</span> write_at&lt;T: Pod&gt;(&amp;<span class="k">self</span>, ctx: &amp;GpuCtx, at: u32, data: &amp;[T]) {
        debug_assert!(at <span class="k">as</span> u64 + data.len() <span class="k">as</span> u64 &lt;= <span class="k">self</span>.cap);
        ctx.queue.write_buffer(
            &amp;<span class="k">self</span>.buf,
            at <span class="k">as</span> u64 * <span class="k">self</span>.stride,
            bytemuck::cast_slice(data),
        );
    }</code></pre></div>
<p><code>lessons/04a/src/engine/gpu/buffers.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Forget the rows; keep the buffer.</span>
    <span class="k">pub</span> <span class="k">fn</span> reset(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">self</span>.len = <span class="s">0</span>;
    }

    <span class="c">/// Forget the rows and free the buffer.</span>
    <span class="k">pub</span> <span class="k">fn</span> release(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx) {
        <span class="k">self</span>.buf = zeroed_buffer(&amp;ctx.device, <span class="k">self</span>.label, <span class="k">self</span>.stride, <span class="k">self</span>.usage);
        <span class="k">self</span>.len = <span class="s">0</span>;
        <span class="k">self</span>.cap = <span class="s">1</span>;
    }

    <span class="k">pub</span> <span class="k">fn</span> len(&amp;<span class="k">self</span>) -&gt; u32 {
        <span class="k">self</span>.len
    }

    <span class="k">pub</span> <span class="k">fn</span> is_empty(&amp;<span class="k">self</span>) -&gt; bool {
        <span class="k">self</span>.len == <span class="s">0</span>
    }
}</code></pre></div>
<p><code>lessons/04a/src/engine/gpu/buffers.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// A small mesh drawn many times, once per instance.</span>
<span class="k">pub</span> <span class="k">struct</span> Template {
    <span class="k">pub</span> vbo: wgpu::Buffer, <span class="c">// vbo = vertex buffer object</span>
    <span class="k">pub</span> ibo: wgpu::Buffer, <span class="c">// ibo = index buffer object</span>
    <span class="k">pub</span> index_count: u32,
}

<span class="k">impl</span> Template {
    <span class="c">/// Upload the mesh once.</span>
    <span class="k">pub</span> <span class="k">fn</span> new(ctx: &amp;GpuCtx, label: &amp;str, verts: &amp;[[f32; <span class="s">3</span>]], idx: &amp;[u32]) -&gt; <span class="k">Self</span> {
        <span class="k">let</span> vbo = ctx
            .device
            .create_buffer_init(&amp;wgpu::util::BufferInitDescriptor {
                label: Some(&amp;format!(&quot;{<span class="s">label</span>}<span class="s">.vbo</span>&quot;)),
                contents: bytemuck::cast_slice(verts),
                usage: wgpu::BufferUsages::VERTEX,
            });
        <span class="k">let</span> ibo = ctx
            .device
            .create_buffer_init(&amp;wgpu::util::BufferInitDescriptor {
                label: Some(&amp;format!(&quot;{<span class="s">label</span>}<span class="s">.ibo</span>&quot;)),
                contents: bytemuck::cast_slice(idx),
                usage: wgpu::BufferUsages::INDEX,
            });

        <span class="k">Self</span> {
            vbo,
            ibo,
            index_count: idx.len() <span class="k">as</span> u32,
        }
    }

    <span class="c">/// Vertex slot 0 = the first entry in the pipeline's list of vertex buffers.</span>
    <span class="k">pub</span> <span class="k">fn</span> bind(&amp;<span class="k">self</span>, pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;) {
        pass.set_vertex_buffer(<span class="s">0</span>, <span class="k">self</span>.vbo.slice(..));
        pass.set_index_buffer(<span class="k">self</span>.ibo.slice(..), wgpu::IndexFormat::Uint32);
    }
}

<span class="c">/// WebGPU zero-fills every new buffer, so the zeros cost no upload.</span>
<span class="k">pub</span> <span class="k">fn</span> zeroed_buffer(
    device: &amp;wgpu::Device,
    label: &amp;str,
    size: u64,
    usage: wgpu::BufferUsages,
) -&gt; wgpu::Buffer {
    device.create_buffer(&amp;wgpu::BufferDescriptor {
        label: Some(label),
        size,
        usage,
        mapped_at_creation: <span class="s">false</span>,
    })
}

<span class="c">/// A small buffer holding one \`T\` for shaders to read.</span>
<span class="k">pub</span> <span class="k">fn</span> uniform_buffer&lt;T: Pod&gt;(device: &amp;wgpu::Device, label: &amp;str, value: &amp;T) -&gt; wgpu::Buffer {
    device.create_buffer_init(&amp;wgpu::util::BufferInitDescriptor {
        label: Some(label),
        contents: bytemuck::bytes_of(value),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    })
}</code></pre></div>
<p><code>lessons/04a/src/engine/gpu/buffers.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// A bind group with \`buffers\` at bindings 0, 1, 2…</span>
<span class="k">pub</span> <span class="k">fn</span> bind_group(
    ctx: &amp;GpuCtx,
    layout: &amp;wgpu::BindGroupLayout,
    label: &amp;str,
    buffers: &amp;[&amp;wgpu::Buffer],
) -&gt; wgpu::BindGroup {
    <span class="k">let</span> <span class="k">mut</span> entries: Vec&lt;wgpu::BindGroupEntry&gt; = Vec::with_capacity(buffers.len());

    <span class="k">for</span> (i, b) <span class="k">in</span> buffers.iter().enumerate() {
        entries.push(wgpu::BindGroupEntry {
            binding: i <span class="k">as</span> u32,
            resource: b.as_entire_binding(),
        });
    }

    ctx.device.create_bind_group(&amp;wgpu::BindGroupDescriptor {
        label: Some(label),
        layout,
        entries: &amp;entries,
    })
}</code></pre></div>
<h2 id="step-2-srcenginepipelineslayoutsrs">Step 2 · src/engine/pipelines/layouts.rs<a class="anchor" href="#/course/04a-meshes#step-2-srcenginepipelineslayoutsrs" aria-label="Link to this section">#</a></h2>
<p>New file: the bind group layouts every pipeline shares.</p>
<p><code>lessons/04a/src/engine/pipelines/layouts.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// One buffer binding, visible to \`stages\`.</span>
<span class="k">fn</span> buffer_entry(
    binding: u32,
    stages: wgpu::ShaderStages,
    ty: wgpu::BufferBindingType,
) -&gt; wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: stages,
        ty: wgpu::BindingType::Buffer {
            ty,
            has_dynamic_offset: <span class="s">false</span>,
            min_binding_size: None,
        },
        count: None,
    }
}

<span class="c">/// A read-only storage buffer at \`binding\`, for the vertex stage.</span>
<span class="k">fn</span> storage_entry(binding: u32) -&gt; wgpu::BindGroupLayoutEntry {
    buffer_entry(
        binding,
        wgpu::ShaderStages::VERTEX,
        wgpu::BufferBindingType::Storage { read_only: <span class="s">true</span> },
    )
}

<span class="c">/// A layout with one uniform buffer at binding 0.</span>
<span class="k">fn</span> uniform_layout(
    device: &amp;wgpu::Device,
    label: &amp;str,
    stages: wgpu::ShaderStages,
) -&gt; wgpu::BindGroupLayout {
    device.create_bind_group_layout(&amp;wgpu::BindGroupLayoutDescriptor {
        label: Some(label),
        entries: &amp;[buffer_entry(<span class="s">0</span>, stages, wgpu::BufferBindingType::Uniform)],
    })
}

<span class="c">/// Group 2: object rows at binding 0, translations at 1.</span>
<span class="k">fn</span> instance_layout(device: &amp;wgpu::Device) -&gt; wgpu::BindGroupLayout {
    device.create_bind_group_layout(&amp;wgpu::BindGroupLayoutDescriptor {
        label: Some(&quot;<span class="s">instance.layout</span>&quot;),
        entries: &amp;[storage_entry(<span class="s">0</span>), storage_entry(<span class="s">1</span>)],
    })
}</code></pre></div>
<p><code>lessons/04a/src/engine/pipelines/layouts.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// A depth texture binding for the fragment stage.</span>
<span class="k">fn</span> scene_depth(binding: u32, multisampled: bool) -&gt; wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::FRAGMENT,
        ty: wgpu::BindingType::Texture {
            sample_type: wgpu::TextureSampleType::Depth,
            view_dimension: wgpu::TextureViewDimension::D2,
            multisampled,
        },
        count: None,
    }
}

<span class="c">/// Group 2 for ink: rows, translations, depth at 1x and 4x, gradient at 1x and 4x, triangles, tiles.</span>
<span class="k">fn</span> ink_instance_layout(device: &amp;wgpu::Device) -&gt; wgpu::BindGroupLayout {
    device.create_bind_group_layout(&amp;wgpu::BindGroupLayoutDescriptor {
        label: Some(&quot;<span class="s">ink.instance.layout</span>&quot;),
        entries: &amp;[
            buffer_entry(
                <span class="s">0</span>,
                wgpu::ShaderStages::VERTEX_FRAGMENT,
                wgpu::BufferBindingType::Storage { read_only: <span class="s">true</span> },
            ),
            buffer_entry(
                <span class="s">1</span>,
                wgpu::ShaderStages::VERTEX_FRAGMENT,
                wgpu::BufferBindingType::Storage { read_only: <span class="s">true</span> },
            ),
            scene_depth(<span class="s">2</span>, <span class="s">false</span>),
            scene_depth(<span class="s">3</span>, <span class="s">true</span>),
        ],
    })
}

<span class="c">/// The bind group layouts every lane shares.</span>
<span class="k">pub</span> <span class="k">struct</span> Layouts {
    <span class="k">pub</span> mvp: wgpu::BindGroupLayout, <span class="c">// group 0: camera matrix</span>
    <span class="k">pub</span> line: wgpu::BindGroupLayout, <span class="c">// group 1: pen and view settings</span>
    <span class="k">pub</span> instance: wgpu::BindGroupLayout, <span class="c">// group 2: object rows</span>
    <span class="k">pub</span> ink_instance: wgpu::BindGroupLayout, <span class="c">// group 2 for ink, with depth textures</span>
}

<span class="k">impl</span> Layouts {
    <span class="c">/// Build every layout once.</span>
    <span class="k">pub</span> <span class="k">fn</span> new(device: &amp;wgpu::Device) -&gt; <span class="k">Self</span> {
        <span class="k">Self</span> {
            mvp: uniform_layout(device, &quot;<span class="s">mvp.layout</span>&quot;, wgpu::ShaderStages::VERTEX_FRAGMENT),
            line: uniform_layout(device, &quot;<span class="s">line.layout</span>&quot;, wgpu::ShaderStages::VERTEX_FRAGMENT),
            instance: instance_layout(device),
            ink_instance: ink_instance_layout(device),
        }
    }
}</code></pre></div>
<h2 id="step-3-srcenginepipelinesmodrs">Step 3 · src/engine/pipelines/mod.rs<a class="anchor" href="#/course/04a-meshes#step-3-srcenginepipelinesmodrs" aria-label="Link to this section">#</a></h2>
<p>New file: one builder for every render pipeline.</p>
<p><code>lessons/04a/src/engine/pipelines/mod.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> <span class="k">mod</span> layouts;

<span class="k">pub</span> <span class="k">use</span> layouts::Layouts;

<span class="k">use</span> session_rust::RenderVertex;

<span class="c">/// Where a pipeline draws: color format and MSAA sample count.</span>
#[derive(Clone, Copy, PartialEq, Eq)]
<span class="k">pub</span> <span class="k">struct</span> Target {
    <span class="k">pub</span> format: wgpu::TextureFormat, <span class="c">// color format</span>
    <span class="k">pub</span> samples: u32, <span class="c">// MSAA samples</span>
}

<span class="k">impl</span> Target {
    <span class="c">/// The id pass target: two u32 per pixel, no MSAA.</span>
    <span class="k">pub</span> <span class="k">const</span> ID: Target = Target {
        format: wgpu::TextureFormat::Rg32Uint,
        samples: <span class="s">1</span>,
    };
}

<span class="c">/// How a pipeline uses depth; reverse-Z, so nearer is greater.</span>
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
<span class="k">pub</span> <span class="k">enum</span> DepthMode {
    Opaque, <span class="c">// write, nearer wins</span>
    OpaqueEqual, <span class="c">// write, nearer or equal wins</span>
    ReadOnly, <span class="c">// test only, nearer wins</span>
    ReadOnlyEqual, <span class="c">// test only, nearer or equal wins</span>
    Always, <span class="c">// no test, no write</span>
    Detached, <span class="c">// no depth attachment at all</span>
}

<span class="k">impl</span> DepthMode {
    <span class="c">/// The (write, compare) pair wgpu wants.</span>
    <span class="k">fn</span> state(<span class="k">self</span>) -&gt; (bool, wgpu::CompareFunction) {
        <span class="k">match</span> <span class="k">self</span> {
            DepthMode::Opaque =&gt; (<span class="s">true</span>, wgpu::CompareFunction::Greater),
            DepthMode::OpaqueEqual =&gt; (<span class="s">true</span>, wgpu::CompareFunction::GreaterEqual),
            DepthMode::ReadOnly =&gt; (<span class="s">false</span>, wgpu::CompareFunction::Greater),
            DepthMode::ReadOnlyEqual =&gt; (<span class="s">false</span>, wgpu::CompareFunction::GreaterEqual),
            DepthMode::Always | DepthMode::Detached =&gt; (<span class="s">false</span>, wgpu::CompareFunction::Always),
        }
    }
}

<span class="c">/// How a pipeline writes color.</span>
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
<span class="k">pub</span> <span class="k">enum</span> ColorWrite {</code></pre></div>
<p><code>lessons/04a/src/engine/pipelines/mod.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    Opaque, <span class="c">// overwrite</span>
    Blended, <span class="c">// alpha blend</span>
    Max, <span class="c">// keep the larger value; for masks</span>
    Nothing, <span class="c">// write nothing; the fragment shader has side effects</span>
}

<span class="k">impl</span> ColorWrite {
    <span class="c">/// The (blend, write mask) pair wgpu wants.</span>
    <span class="k">fn</span> state(<span class="k">self</span>) -&gt; (Option&lt;wgpu::BlendState&gt;, wgpu::ColorWrites) {
        <span class="k">match</span> <span class="k">self</span> {
            ColorWrite::Opaque =&gt; (None, wgpu::ColorWrites::ALL),
            ColorWrite::Blended =&gt; (
                Some(wgpu::BlendState::ALPHA_BLENDING),
                wgpu::ColorWrites::ALL,
            ),
            ColorWrite::Nothing =&gt; (None, wgpu::ColorWrites::empty()),
            ColorWrite::Max =&gt; {
                <span class="k">let</span> max = wgpu::BlendComponent {
                    src_factor: wgpu::BlendFactor::One,
                    dst_factor: wgpu::BlendFactor::One,
                    operation: wgpu::BlendOperation::Max,
                };
                (
                    Some(wgpu::BlendState {
                        color: max,
                        alpha: max,
                    }),
                    wgpu::ColorWrites::ALL,
                )
            }
        }
    }
}

<span class="c">/// Everything \`build\` needs for one render pipeline.</span>
#[derive(Clone)]
<span class="k">pub</span> <span class="k">struct</span> PipelineDesc&lt;'a&gt; {
    <span class="k">pub</span> label: &amp;'a str, <span class="c">// name shown in GPU errors</span>
    <span class="k">pub</span> shader: &amp;'a wgpu::ShaderModule, <span class="c">// compiled shader</span>
    <span class="k">pub</span> vs: &amp;'a str, <span class="c">// vertex entry point</span>
    <span class="k">pub</span> fs: &amp;'a str, <span class="c">// fragment entry point</span>
    <span class="k">pub</span> groups: &amp;'a [&amp;'a wgpu::BindGroupLayout], <span class="c">// bind group layouts, in slot order</span>
    <span class="k">pub</span> vertex_buffers: &amp;'a [wgpu::VertexBufferLayout&lt;'a&gt;], <span class="c">// vertex buffer layouts</span>
    <span class="k">pub</span> topology: wgpu::PrimitiveTopology, <span class="c">// triangles or lines</span>
    <span class="k">pub</span> color: ColorWrite, <span class="c">// how color is written</span>
    <span class="k">pub</span> depth: DepthMode, <span class="c">// how depth is used</span>
    <span class="k">pub</span> scene_samples: Option&lt;u32&gt;, <span class="c">// sets SCENE_MSAA in the shader</span>
}

<span class="k">impl</span>&lt;'a&gt; PipelineDesc&lt;'a&gt; {
    <span class="c">/// A base: \`vs_main\`, \`fs_main\`, opaque color, opaque depth.</span>
    <span class="k">pub</span> <span class="k">fn</span> new(
        shader: &amp;'a wgpu::ShaderModule,</code></pre></div>
<p><code>lessons/04a/src/engine/pipelines/mod.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        groups: &amp;'a [&amp;'a wgpu::BindGroupLayout],
        vertex_buffers: &amp;'a [wgpu::VertexBufferLayout&lt;'a&gt;],
        topology: wgpu::PrimitiveTopology,
    ) -&gt; <span class="k">Self</span> {
        <span class="k">Self</span> {
            label: &quot;&quot;,
            shader,
            vs: &quot;<span class="s">vs_main</span>&quot;,
            fs: &quot;<span class="s">fs_main</span>&quot;,
            groups,
            vertex_buffers,
            topology,
            color: ColorWrite::Opaque,
            depth: DepthMode::Opaque,
            scene_samples: None,
        }
    }

    <span class="c">/// A copy with another label and fragment entry point.</span>
    <span class="k">pub</span> <span class="k">fn</span> with(&amp;<span class="k">self</span>, label: &amp;'a str, fs: &amp;'a str) -&gt; <span class="k">Self</span> {
        <span class="k">let</span> <span class="k">mut</span> d = <span class="k">self</span>.clone();
        d.label = label;
        d.fs = fs;
        d
    }

    <span class="c">/// A copy with another vertex entry point.</span>
    <span class="k">pub</span> <span class="k">fn</span> vertex(<span class="k">mut</span> <span class="k">self</span>, vs: &amp;'a str) -&gt; <span class="k">Self</span> {
        <span class="k">self</span>.vs = vs;
        <span class="k">self</span>
    }

    <span class="c">/// A copy with another color mode.</span>
    <span class="k">pub</span> <span class="k">fn</span> color(<span class="k">mut</span> <span class="k">self</span>, color: ColorWrite) -&gt; <span class="k">Self</span> {
        <span class="k">self</span>.color = color;
        <span class="k">self</span>
    }

    <span class="c">/// A copy that reads the scene depth at \`samples\`.</span>
    <span class="k">pub</span> <span class="k">fn</span> scene_samples(<span class="k">mut</span> <span class="k">self</span>, samples: u32) -&gt; <span class="k">Self</span> {
        <span class="k">self</span>.scene_samples = Some(samples);
        <span class="k">self</span>
    }

    <span class="c">/// A copy with another depth mode.</span>
    <span class="k">pub</span> <span class="k">fn</span> depth(<span class="k">mut</span> <span class="k">self</span>, depth: DepthMode) -&gt; <span class="k">Self</span> {
        <span class="k">self</span>.depth = depth;
        <span class="k">self</span>
    }
}

<span class="c">/// Shared WGSL: groups 0-2, Instance, LineUniform, flags, \`place\`.</span>
<span class="k">pub</span> <span class="k">const</span> SCENE: &amp;str = include_str!(&quot;<span class="s">../../shaders/scene.wgsl</span>&quot;);

<span class="c">/// Compile a shader with the shared scene code appended.</span>
<span class="k">pub</span> <span class="k">fn</span> scene_module(device: &amp;wgpu::Device, label: &amp;str, source: &amp;str) -&gt; wgpu::ShaderModule {
    module(device, label, &amp;format!(&quot;{<span class="s">source</span>}<span class="s">\\n</span>{<span class="s">SCENE</span>}&quot;))
}

<span class="c">/// One u32 at location 3: the object row.</span>
<span class="k">const</span> INSTANCE_ID_ATTRIBS: [wgpu::VertexAttribute; <span class="s">1</span>] = [wgpu::VertexAttribute {
    offset: <span class="s">0</span>,
    shader_location: <span class="s">3</span>,
    format: wgpu::VertexFormat::Uint32,
}];</code></pre></div>
<p><code>lessons/04a/src/engine/pipelines/mod.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Vertex slot 0: the kernel's RenderVertex (position, normal, color).</span>
<span class="k">pub</span> <span class="k">fn</span> vertex_layout() -&gt; wgpu::VertexBufferLayout&lt;'static&gt; {
    RenderVertex::layout()
}

<span class="c">/// Vertex slot 1: one object row per vertex.</span>
<span class="k">pub</span> <span class="k">fn</span> instance_id_layout() -&gt; wgpu::VertexBufferLayout&lt;'static&gt; {
    wgpu::VertexBufferLayout {
        array_stride: <span class="s">4</span>,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &amp;INSTANCE_ID_ATTRIBS,
    }
}

<span class="c">/// Compile a shader that declares its own bindings.</span>
<span class="k">pub</span> <span class="k">fn</span> module(device: &amp;wgpu::Device, label: &amp;str, source: &amp;str) -&gt; wgpu::ShaderModule {
    <span class="k">let</span> source = format!(&quot;{}<span class="s">\\n</span>{}&quot;, source, include_str!(&quot;<span class="s">../../shaders/normals.wgsl</span>&quot;));
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some(label),
        source: wgpu::ShaderSource::Wgsl(source.into()),
    })
}</code></pre></div>
<p><code>lessons/04a/src/engine/pipelines/mod.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// The pipeline layout for \`groups\`, in slot order.</span>
<span class="k">fn</span> pipeline_layout(
    device: &amp;wgpu::Device,
    label: &amp;str,
    groups: &amp;[&amp;wgpu::BindGroupLayout],
) -&gt; wgpu::PipelineLayout {
    <span class="k">let</span> <span class="k">mut</span> slots: Vec&lt;Option&lt;&amp;wgpu::BindGroupLayout&gt;&gt; = Vec::with_capacity(groups.len());

    <span class="k">for</span> g <span class="k">in</span> groups {
        slots.push(Some(*g));
    }

    device.create_pipeline_layout(&amp;wgpu::PipelineLayoutDescriptor {
        label: Some(label),
        bind_group_layouts: &amp;slots,
        immediate_size: <span class="s">0</span>,
    })
}

<span class="c">/// Build one render pipeline; Depth32Float, no culling, fill mode.</span>
<span class="k">pub</span> <span class="k">fn</span> build(device: &amp;wgpu::Device, target: Target, desc: &amp;PipelineDesc) -&gt; wgpu::RenderPipeline {
    <span class="k">let</span> layout = pipeline_layout(device, desc.label, desc.groups);
    <span class="k">let</span> (depth_write, depth_compare) = desc.depth.state();
    <span class="k">let</span> (blend, write_mask) = desc.color.state();
    <span class="k">let</span> targets = [Some(wgpu::ColorTargetState {
        format: target.format,
        blend,
        write_mask,
    })];
    <span class="k">let</span> <span class="k">mut</span> constants = Vec::new();</code></pre></div>
<p><code>lessons/04a/src/engine/pipelines/mod.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">// shader constant: which depth texture is live</span>
    <span class="k">if</span> <span class="k">let</span> Some(samples) = desc.scene_samples {
        constants.push((&quot;<span class="s">SCENE_MSAA</span>&quot;, f64::from(samples &gt; <span class="s">1</span>)));
    }

    device.create_render_pipeline(&amp;wgpu::RenderPipelineDescriptor {
        label: Some(desc.label),
        layout: Some(&amp;layout),
        vertex: wgpu::VertexState {
            module: desc.shader,
            entry_point: Some(desc.vs),
            buffers: desc.vertex_buffers,
            compilation_options: wgpu::PipelineCompilationOptions {
                constants: &amp;constants,
                ..Default::default()
            },
        },
        fragment: Some(wgpu::FragmentState {
            module: desc.shader,
            entry_point: Some(desc.fs),
            targets: &amp;targets,
            compilation_options: wgpu::PipelineCompilationOptions {
                constants: &amp;constants,
                ..Default::default()
            },
        }),
        primitive: wgpu::PrimitiveState {
            topology: desc.topology,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: <span class="s">false</span>,
            conservative: <span class="s">false</span>,
        },
        depth_stencil: (desc.depth != DepthMode::Detached).then_some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: Some(depth_write),
            depth_compare: Some(depth_compare),
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState {
            count: target.samples,
            mask: !<span class="s">0</span>,
            alpha_to_coverage_enabled: <span class="s">false</span>,
        },
        multiview_mask: None,
        cache: None,
    })
}</code></pre></div>
<h2 id="step-4-srcshadersscenewgsl">Step 4 · src/shaders/scene.wgsl<a class="anchor" href="#/course/04a-meshes#step-4-srcshadersscenewgsl" aria-label="Link to this section">#</a></h2>
<p>New file: the bindings, structs and helpers every scene shader starts with.</p>
<p><code>lessons/04a/src/shaders/scene.wgsl</code> · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>@group(<span class="s">0</span>) @binding(<span class="s">0</span>) <span class="k">var</span>&lt;<span class="k">uniform</span>&gt; mvp: <span class="k">mat4x4</span>&lt;<span class="k">f32</span>&gt;;
@group(<span class="s">1</span>) @binding(<span class="s">0</span>) <span class="k">var</span>&lt;<span class="k">uniform</span>&gt; line: LineUniform;

<span class="c">// One object row, 96 bytes; matches Instance in Rust.</span>
<span class="k">struct</span> Instance {
    model: <span class="k">mat4x4</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // rotation and scale; translation is separate</span>
    color: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,
    flags: <span class="k">u32</span>,
    _pad0: <span class="k">f32</span>,
    spacing: <span class="k">f32</span>,<span class="c"> // vertex spacing, world units; WGSL pads the struct to 96 bytes by itself</span>
};

@group(<span class="s">2</span>) @binding(<span class="s">0</span>) <span class="k">var</span>&lt;<span class="k">storage</span>, <span class="k">read</span>&gt; instances: <span class="k">array</span>&lt;Instance&gt;;
@group(<span class="s">2</span>) @binding(<span class="s">1</span>) <span class="k">var</span>&lt;<span class="k">storage</span>, <span class="k">read</span>&gt; translations: <span class="k">array</span>&lt;<span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;&gt;;<span class="c"> // xyz = position minus the anchor; w is padding</span>

<span class="c">// Pen and view settings, 80 bytes; matches LineUniform in Rust.</span>
<span class="k">struct</span> LineUniform {
    thickness: <span class="k">f32</span>,
    proj_y: <span class="k">f32</span>,
    ortho_h: <span class="k">f32</span>,
    vp_h: <span class="k">f32</span>,
    vp_w: <span class="k">f32</span>,
    eye_x: <span class="k">f32</span>,<span class="c"> // split in three: a vec3 aligns to 16 bytes and would shift every field after it</span>
    eye_y: <span class="k">f32</span>,
    eye_z: <span class="k">f32</span>,
    anchor: <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;,
    feather: <span class="k">f32</span>,
    lit: <span class="k">f32</span>,
    backface: <span class="k">f32</span>,
    origin: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;,
    frame: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;,
    opacity: <span class="k">f32</span>,
};

<span class="c">// Object flag bits; match Instance::FLAG_* in Rust.</span>
<span class="k">const</span> FLAG_SELECTED: <span class="k">u32</span> = <span class="s">1u</span>;<span class="c"> // selected: drawn tinted</span>
<span class="k">const</span> FLAG_HIDDEN: <span class="k">u32</span> = <span class="s">2u</span>;<span class="c"> // hidden: its vertices are moved outside clip space</span>
<span class="k">const</span> FLAG_INSIDE: <span class="k">u32</span> = <span class="s">4u</span>;<span class="c"> // camera is inside the object</span>
<span class="k">const</span> FLAG_PRINT: <span class="k">u32</span> = <span class="s">8u</span>;<span class="c"> // sheet fill: flat color</span>
<span class="k">const</span> FLAG_OPEN: <span class="k">u32</span> = <span class="s">16u</span>;<span class="c"> // open mesh: no back-face culling</span>
<span class="k">const</span> FLAG_SHEET: <span class="k">u32</span> = <span class="s">32u</span>;<span class="c"> // part of a drawing sheet</span>
<span class="k">const</span> FLAG_SMOOTH: <span class="k">u32</span> = <span class="s">64u</span>;<span class="c"> // sampled surface: vertices are samples</span>
<span class="k">const</span> FLAG_SINGLE: <span class="k">u32</span> = <span class="s">128u</span>;<span class="c"> // single face: stays shaded in x-ray</span>

<span class="c">// No face normals known: always drawn.</span>
<span class="k">const</span> FACING_UNKNOWN: <span class="k">u32</span> = <span class="s">0xffffffffu</span>;

<span class="c">// Bit that marks a pick id as a marker.</span>
<span class="k">const</span> DISC_ID_TAG: <span class="k">u32</span> = <span class="s">0x40000000u</span>;
<span class="k">const</span> SELECT_COLOR: <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt; = <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(<span class="s">1.0</span>, <span class="s">1.0</span>, <span class="s">0.0</span>);
<span class="k">const</span> MM_TO_M: <span class="k">f32</span> = <span class="s">0.001</span>;
<span class="c">// Thinnest lines never fade below this alpha.</span>
<span class="k">const</span> HAIRLINE_MIN_ALPHA: <span class="k">f32</span> = <span class="s">0.5</span>;

<span class="c">// Object space to scene space: rotate and scale by the row's matrix, then move.</span>
<span class="k">fn</span> place(i: <span class="k">u32</span>, p: <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;) -&gt; <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt; {
    <span class="k">return</span> (instances[i].model * <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(p, <span class="s">1.0</span>)).xyz + translations[i].xyz;
}</code></pre></div>
<h2 id="step-5-srcshadersnormalswgsl">Step 5 · src/shaders/normals.wgsl<a class="anchor" href="#/course/04a-meshes#step-5-srcshadersnormalswgsl" aria-label="Link to this section">#</a></h2>
<p>New file: two helpers that turn a normal with its object, and one that unpacks a 2-byte normal.</p>
<p><code>lessons/04a/src/shaders/normals.wgsl</code> · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// Returns zero, not NaN, when the matrix flattens the normal to nothing.</span>
<span class="k">fn</span> transform_normal(model: <span class="k">mat3x3</span>&lt;<span class="k">f32</span>&gt;, normal: <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;) -&gt; <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt; {
    <span class="k">let</span> transformed = model * normal;

    <span class="k">if</span> (dot(transformed, transformed) == <span class="s">0.0</span>) {
        <span class="k">return</span> <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>);
    }

    <span class="k">return</span> normalize(transformed);
}

<span class="c">// Only the upper-left 3x3 (rotation and scale): a direction must not be moved.</span>
<span class="k">fn</span> face_normal(model: <span class="k">mat4x4</span>&lt;<span class="k">f32</span>&gt;, normal: <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;) -&gt; <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt; {
    <span class="k">return</span> transform_normal(<span class="k">mat3x3</span>&lt;<span class="k">f32</span>&gt;(model[<span class="s">0</span>].xyz, model[<span class="s">1</span>].xyz, model[<span class="s">2</span>].xyz), normal);
}

<span class="c">// Octahedral encoding: a unit normal packed into 2 bytes instead of 12.</span>
<span class="k">fn</span> oct16_decode(p: <span class="k">u32</span>) -&gt; <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt; {
    <span class="k">let</span> e = <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(f32(i32(p &lt;&lt; <span class="s">24u</span>) &gt;&gt; <span class="s">24u</span>) / <span class="s">127.0</span>, f32(i32(p &lt;&lt; <span class="s">16u</span>) &gt;&gt; <span class="s">24u</span>) / <span class="s">127.0</span>);<span class="c"> // shifting an i32 right keeps the sign: each byte becomes -128..127</span>
    <span class="k">var</span> n = <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(e, <span class="s">1.0</span> - abs(e.x) - abs(e.y));

<span class="c">    // fold the lower half back</span>
    <span class="k">if</span> (n.z &lt; <span class="s">0.0</span>) {
        <span class="k">let</span> s = <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(select(<span class="s">1.0</span>, -<span class="s">1.0</span>, n.x &lt; <span class="s">0.0</span>), select(<span class="s">1.0</span>, -<span class="s">1.0</span>, n.y &lt; <span class="s">0.0</span>));
        n = <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;((<span class="s">1.0</span> - abs(n.y)) * s.x, (<span class="s">1.0</span> - abs(n.x)) * s.y, n.z);
    }

    <span class="k">return</span> normalize(n);
}</code></pre></div>
<p>Run <code>cargo check</code> in <code>lessons/04a/</code>.</p>
<h2 id="step-6-srcenginegputargetsrs">Step 6 · src/engine/gpu/targets.rs<a class="anchor" href="#/course/04a-meshes#step-6-srcenginegputargetsrs" aria-label="Link to this section">#</a></h2>
<p>New file: the colour and depth textures of one frame.</p>
<p><code>lessons/04a/src/engine/gpu/targets.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> super::buffers::GpuCtx;

<span class="c">/// The depth texture remembers how far away each pixel already is, which is what lets a nearer triangle cover a farther one.</span>
<span class="k">pub</span> <span class="k">struct</span> Targets {
    <span class="k">pub</span> depth: wgpu::TextureView,
    <span class="k">pub</span> msaa: Option&lt;wgpu::TextureView&gt;,
    <span class="k">pub</span> depth_single: wgpu::TextureView, <span class="c">// depth at 1x, or a 1x1 placeholder</span>
    <span class="k">pub</span> depth_msaa: wgpu::TextureView, <span class="c">// depth at 4x, or a 1x1 placeholder</span>
    <span class="k">pub</span> samples: u32, <span class="c">// MSAA draws each pixel 4 times at slightly different spots and averages them, to soften edges</span>
}

<span class="k">impl</span> Targets {
    <span class="c">/// Create the textures for \`size\` at \`samples\`.</span>
    <span class="k">pub</span> <span class="k">fn</span> new(ctx: &amp;GpuCtx, size: (u32, u32), format: wgpu::TextureFormat, samples: u32) -&gt; <span class="k">Self</span> {
        <span class="k">let</span> usage = wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING;
        <span class="k">let</span> depth = texture_view(
            ctx,
            &quot;<span class="s">depth</span>&quot;,
            &amp;TextureSpec {
                size,
                format: wgpu::TextureFormat::Depth32Float,
                samples,
                usage,
            },
        );
        <span class="k">let</span> msaa = <span class="k">if</span> samples &gt; <span class="s">1</span> {
            Some(texture_view(
                ctx,
                &quot;<span class="s">msaa_color</span>&quot;,
                &amp;TextureSpec {
                    size,
                    format,
                    samples,
                    usage,
                },
            ))
        } <span class="k">else</span> {
            None
        };

        <span class="c">// shaders bind both sample counts; the unused one is 1x1</span>
        <span class="k">let</span> other_samples = <span class="k">if</span> samples == <span class="s">1</span> { <span class="s">4</span> } <span class="k">else</span> { <span class="s">1</span> };
        <span class="k">let</span> empty_depth = texture_view(
            ctx,
            &quot;<span class="s">unused.depth</span>&quot;,
            &amp;TextureSpec {
                size: (<span class="s">1</span>, <span class="s">1</span>),
                format: wgpu::TextureFormat::Depth32Float,
                samples: other_samples,
                usage,
            },
        );
        <span class="k">let</span> (depth_single, depth_msaa) = <span class="k">if</span> samples == <span class="s">1</span> {
            (depth.clone(), empty_depth)
        } <span class="k">else</span> {
            (empty_depth, depth.clone())
        };
        <span class="k">Self</span> {
            depth,
            msaa,
            depth_single,
            depth_msaa,
            samples,
        }
    }</code></pre></div>
<p><code>lessons/04a/src/engine/gpu/targets.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Open the face pass: color and depth cleared.</span>
    <span class="k">pub</span> <span class="k">fn</span> begin_faces&lt;'a&gt;(
        &amp;'a <span class="k">self</span>,
        encoder: &amp;'a <span class="k">mut</span> wgpu::CommandEncoder,
        view: &amp;'a wgpu::TextureView,
        clear: wgpu::Color,
    ) -&gt; wgpu::RenderPass&lt;'a&gt; {
        <span class="k">let</span> target = <span class="k">self</span>.msaa.as_ref().unwrap_or(view);
        encoder.begin_render_pass(&amp;wgpu::RenderPassDescriptor {
            label: Some(&quot;<span class="s">physical face pass</span>&quot;),
            color_attachments: &amp;[Some(wgpu::RenderPassColorAttachment {
                view: target,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(clear),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &amp;<span class="k">self</span>.depth,
                depth_ops: Some(wgpu::Operations {
                    <span class="c">// reverse-Z: 0 is the far plane</span>
                    load: wgpu::LoadOp::Clear(<span class="s">0</span>.<span class="s">0</span>),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        })
    }

    <span class="c">/// Open the ink pass over the faces; depth is read, not written.</span>
    <span class="k">pub</span> <span class="k">fn</span> begin_ink&lt;'a&gt;(
        &amp;'a <span class="k">self</span>,
        encoder: &amp;'a <span class="k">mut</span> wgpu::CommandEncoder,
        view: &amp;'a wgpu::TextureView,
    ) -&gt; wgpu::RenderPass&lt;'a&gt; {
        <span class="k">let</span> (target, resolve) = <span class="k">match</span> &amp;<span class="k">self</span>.msaa {
            Some(msaa) =&gt; (msaa, Some(view)),
            None =&gt; (view, None),
        };
        encoder.begin_render_pass(&amp;wgpu::RenderPassDescriptor {
            label: Some(&quot;<span class="s">visible ink pass</span>&quot;),
            color_attachments: &amp;[Some(wgpu::RenderPassColorAttachment {
                view: target,
                resolve_target: resolve,
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &amp;<span class="k">self</span>.depth,
                depth_ops: None,
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        })
    }
}</code></pre></div>
<p><code>lessons/04a/src/engine/gpu/targets.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Settings for one 2D texture.</span>
<span class="k">pub</span> <span class="k">struct</span> TextureSpec {
    <span class="k">pub</span> size: (u32, u32), <span class="c">// width and height, px</span>
    <span class="k">pub</span> format: wgpu::TextureFormat, <span class="c">// pixel format</span>
    <span class="k">pub</span> samples: u32, <span class="c">// MSAA samples</span>
    <span class="k">pub</span> usage: wgpu::TextureUsages, <span class="c">// how the GPU may use it</span>
}

<span class="c">/// Create a 2D texture from \`spec\`.</span>
<span class="k">pub</span> <span class="k">fn</span> texture(ctx: &amp;GpuCtx, label: &amp;str, spec: &amp;TextureSpec) -&gt; wgpu::Texture {
    ctx.device.create_texture(&amp;wgpu::TextureDescriptor {
        label: Some(label),
        size: wgpu::Extent3d {
            width: spec.size.<span class="s">0</span>.max(<span class="s">1</span>),
            height: spec.size.<span class="s">1</span>.max(<span class="s">1</span>),
            depth_or_array_layers: <span class="s">1</span>,
        },
        mip_level_count: <span class="s">1</span>,
        sample_count: spec.samples,
        dimension: wgpu::TextureDimension::D2,
        format: spec.format,
        usage: spec.usage,
        view_formats: &amp;[],
    })
}

<span class="c">/// A texture's default view; wgpu keeps the texture alive.</span>
<span class="k">pub</span> <span class="k">fn</span> texture_view(ctx: &amp;GpuCtx, label: &amp;str, spec: &amp;TextureSpec) -&gt; wgpu::TextureView {
    texture(ctx, label, spec).create_view(&amp;wgpu::TextureViewDescriptor::default())
}</code></pre></div>
<h2 id="step-7-srcenginegpuframers">Step 7 · src/engine/gpu/frame.rs<a class="anchor" href="#/course/04a-meshes#step-7-srcenginegpuframers" aria-label="Link to this section">#</a></h2>
<p>New file: what the app hands the renderer each frame, the uniforms it writes, and the three bind groups every draw sets first.</p>
<p><code>lessons/04a/src/engine/gpu/frame.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> super::buffers::{GpuCtx, bind_group, uniform_buffer};
<span class="k">use</span> super::view::View;
<span class="k">use</span> <span class="k">crate</span>::camera::FOVY_DEG;
<span class="k">use</span> <span class="k">crate</span>::engine::pipelines::Layouts;
<span class="k">use</span> session_rust::Xform;

<span class="c">/// Everything that changes per frame; the renderer keeps no camera or clock of its own.</span>
<span class="k">pub</span> <span class="k">struct</span> FrameInput {
    <span class="k">pub</span> view_proj: Xform,
    <span class="k">pub</span> clear: wgpu::Color,
    <span class="k">pub</span> now_ms: f64, <span class="c">// browser clock, milliseconds</span>
}

<span class="c">/// \`'a\`: this struct must not outlive the View it borrows.</span>
<span class="k">pub</span> <span class="k">struct</span> FrameCx&lt;'a&gt; {
    <span class="k">pub</span> view: &amp;'a View,
    <span class="k">pub</span> anchor: [f32; <span class="s">3</span>],
    <span class="k">pub</span> size: (u32, u32), <span class="c">// device pixels, not CSS pixels</span>
    <span class="k">pub</span> pixel_scale: f32,
}

<span class="c">/// Group 0 = camera matrix, group 1 = pen and view settings, group 2 = one row per object.</span>
<span class="k">pub</span> <span class="k">struct</span> Binds&lt;'a&gt; {
    <span class="k">pub</span> mvp: &amp;'a wgpu::BindGroup,
    <span class="k">pub</span> line: &amp;'a wgpu::BindGroup,
    <span class="k">pub</span> instances: &amp;'a wgpu::BindGroup,
}

<span class="c">// \`'_\` = whatever lifetime this Binds was made with; the method has no reason to name it.</span>
<span class="k">impl</span> Binds&lt;'_&gt; {
    <span class="c">/// Every pipeline expects the same three groups, so each draw sets them in one call.</span>
    <span class="k">pub</span> <span class="k">fn</span> set(&amp;<span class="k">self</span>, pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;) {
        pass.set_bind_group(<span class="s">0</span>, <span class="k">self</span>.mvp, &amp;[]);
        pass.set_bind_group(<span class="s">1</span>, <span class="k">self</span>.line, &amp;[]);
        pass.set_bind_group(<span class="s">2</span>, <span class="k">self</span>.instances, &amp;[]);
    }
}</code></pre></div>
<p><code>lessons/04a/src/engine/gpu/frame.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Pen and view settings every shader reads, 80 bytes.</span>
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
<span class="k">pub</span> <span class="k">struct</span> LineUniform {
    <span class="k">pub</span> thickness: f32, <span class="c">// line width, device pixels</span>
    <span class="k">pub</span> proj_y: f32, <span class="c">// perspective: clip units per mm at distance 1</span>
    <span class="k">pub</span> ortho_h: f32, <span class="c">// half the view height when orthographic; 0 = perspective</span>
    <span class="k">pub</span> vp_h: f32, <span class="c">// vp = viewport: the render target's size in pixels</span>
    <span class="k">pub</span> vp_w: f32,
    <span class="k">pub</span> eye: [f32; <span class="s">3</span>],
    <span class="k">pub</span> anchor: [f32; <span class="s">3</span>],
    <span class="k">pub</span> feather: f32, <span class="c">// soft edge in px: lines fade out instead of stair-stepping</span>
    <span class="k">pub</span> lit: f32, <span class="c">// 0 flat, 1 lit, 2 lit + ambient occlusion</span>
    <span class="k">pub</span> backface: f32, <span class="c">// 1 = paint back faces red</span>
    <span class="k">pub</span> origin: [f32; <span class="s">2</span>], <span class="c">// where this render target sits in the canvas, px; non-zero only in a pick</span>
    <span class="k">pub</span> frame: [f32; <span class="s">2</span>], <span class="c">// canvas size, px</span>
    <span class="k">pub</span> opacity: f32, <span class="c">// mesh face alpha, 1 = opaque</span>
    <span class="k">pub</span> _pad: f32, <span class="c">// uniform sizes go in 16-byte steps: 76 becomes 80</span>
}

<span class="c">// Checked at compile time: a moved field fails the build instead of drawing garbage.</span>
<span class="k">const</span> _: () = {
    assert!(std::mem::size_of::&lt;LineUniform&gt;() == <span class="s">80</span>);
    assert!(std::mem::offset_of!(LineUniform, lit) == <span class="s">48</span>);
    assert!(std::mem::offset_of!(LineUniform, backface) == <span class="s">52</span>);
    assert!(std::mem::offset_of!(LineUniform, origin) == <span class="s">56</span>);
    assert!(std::mem::offset_of!(LineUniform, frame) == <span class="s">64</span>);
    assert!(std::mem::offset_of!(LineUniform, opacity) == <span class="s">72</span>);
};

<span class="c">/// Point cloud settings, 48 bytes.</span>
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
<span class="k">pub</span> <span class="k">struct</span> CloudUniform {
    <span class="k">pub</span> size: f32, <span class="c">// point size scale; the CPU applies it, shaders do not</span>
    <span class="k">pub</span> vp_w: f32,
    <span class="k">pub</span> vp_h: f32,
    <span class="k">pub</span> edl: f32, <span class="c">// EDL = eye-dome lighting: darkens depth jumps so a cloud reads as solid; 0 = off</span>
    <span class="k">pub</span> _pad0: f32,
    <span class="k">pub</span> _pad1: f32,
    <span class="k">pub</span> origin: [f32; <span class="s">2</span>],
    <span class="k">pub</span> frame: [f32; <span class="s">2</span>],
    <span class="k">pub</span> _pad: [f32; <span class="s">2</span>],
}

<span class="k">const</span> _: () = {
    assert!(std::mem::size_of::&lt;CloudUniform&gt;() == <span class="s">48</span>);
    assert!(std::mem::offset_of!(CloudUniform, origin) == <span class="s">24</span>);
    assert!(std::mem::offset_of!(CloudUniform, frame) == <span class="s">32</span>);
};</code></pre></div>
<p><code>lessons/04a/src/engine/gpu/frame.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// A pick renders only a small window around the cursor, 13 x 13 px by default, not the whole canvas.</span>
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
<span class="k">pub</span> <span class="k">struct</span> PickView {
    <span class="k">pub</span> x: u32, <span class="c">// canvas pixels from the top-left</span>
    <span class="k">pub</span> y: u32,
    <span class="k">pub</span> w: u32,
    <span class="k">pub</span> h: u32,
}

<span class="k">impl</span> PickView {
    <span class="c">/// .max(1): a 0 x 0 canvas would divide by zero later.</span>
    <span class="k">pub</span> <span class="k">fn</span> whole(size: (u32, u32)) -&gt; <span class="k">Self</span> {
        <span class="k">Self</span> {
            x: <span class="s">0</span>,
            y: <span class="s">0</span>,
            w: size.<span class="s">0</span>.max(<span class="s">1</span>),
            h: size.<span class="s">1</span>.max(<span class="s">1</span>),
        }
    }

    <span class="c">/// Stretch clip space so this small window fills -1..1, the pick target's whole area.</span>
    <span class="k">pub</span> <span class="k">fn</span> clip_transform(&amp;<span class="k">self</span>, frame: (u32, u32)) -&gt; [f32; <span class="s">16</span>] {
        <span class="k">let</span> (fw, fh) = (frame.<span class="s">0</span>.max(<span class="s">1</span>) <span class="k">as</span> f32, frame.<span class="s">1</span>.max(<span class="s">1</span>) <span class="k">as</span> f32);
        <span class="k">let</span> (w, h) = (<span class="k">self</span>.w.max(<span class="s">1</span>) <span class="k">as</span> f32, <span class="k">self</span>.h.max(<span class="s">1</span>) <span class="k">as</span> f32);
        <span class="k">let</span> sx = fw / w;
        <span class="k">let</span> sy = fh / h;
        <span class="k">let</span> tx = sx - <span class="s">2</span>.<span class="s">0</span> * <span class="k">self</span>.x <span class="k">as</span> f32 / w - <span class="s">1</span>.<span class="s">0</span>;
        <span class="k">let</span> ty = <span class="s">1</span>.<span class="s">0</span> - (fh - <span class="s">2</span>.<span class="s">0</span> * <span class="k">self</span>.y <span class="k">as</span> f32) / h;
        [
            sx, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="c">//</span>
            <span class="s">0</span>.<span class="s">0</span>, sy, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="c">//</span>
            <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="c">//</span>
            tx, ty, <span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>,
        ]
    }
}

<span class="k">pub</span> <span class="k">fn</span> pick_transform_layout(ctx: &amp;GpuCtx) -&gt; wgpu::BindGroupLayout {
    ctx.device
        .create_bind_group_layout(&amp;wgpu::BindGroupLayoutDescriptor {
            label: Some(&quot;<span class="s">pick.transform.layout</span>&quot;),
            entries: &amp;[wgpu::BindGroupLayoutEntry {
                binding: <span class="s">0</span>,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: <span class="s">false</span>,
                    min_binding_size: None,
                },
                count: None,
            }],
        })
}

<span class="c">/// Column-major, like WGSL: out = left * right.</span>
<span class="k">fn</span> mat4_mul(left: &amp;[f32; <span class="s">16</span>], right: &amp;[f32; <span class="s">16</span>]) -&gt; [f32; <span class="s">16</span>] {
    <span class="k">let</span> <span class="k">mut</span> out = [<span class="s">0</span>.<span class="s">0</span>; <span class="s">16</span>];

    <span class="k">for</span> col <span class="k">in</span> <span class="s">0</span>..<span class="s">4</span> {
        <span class="k">for</span> row <span class="k">in</span> <span class="s">0</span>..<span class="s">4</span> {
            out[col * <span class="s">4</span> + row] = (<span class="s">0</span>..<span class="s">4</span>).map(|k| left[k * <span class="s">4</span> + row] * right[col * <span class="s">4</span> + k]).sum();
        }
    }

    out
}</code></pre></div>
<p><code>lessons/04a/src/engine/gpu/frame.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Two copies of every uniform: one for the frame, one for the pick window, so a pick never disturbs the picture.</span>
<span class="k">pub</span> <span class="k">struct</span> FrameUniforms {
    mvp_buffer: wgpu::Buffer,
    line_buffer: wgpu::Buffer,
    cloud_buffer: wgpu::Buffer,
    <span class="k">pub</span> mvp_group: wgpu::BindGroup,
    <span class="k">pub</span> line_group: wgpu::BindGroup,
    <span class="k">pub</span> cloud_group: wgpu::BindGroup, <span class="c">// the point lane's group 1</span>
    pick_mvp_buffer: wgpu::Buffer, <span class="c">// the same three, for the pick window</span>
    pick_line_buffer: wgpu::Buffer,
    pick_cloud_buffer: wgpu::Buffer,
    pick_transform_buffer: wgpu::Buffer, <span class="c">// canvas-to-window matrix</span>
    <span class="k">pub</span> pick_mvp_group: wgpu::BindGroup,
    <span class="k">pub</span> pick_line_group: wgpu::BindGroup,
    <span class="k">pub</span> pick_cloud_group: wgpu::BindGroup,
    <span class="k">pub</span> pick_transform_group: wgpu::BindGroup, <span class="c">// text lanes' pick group</span>
    line: LineUniform, <span class="c">// kept to derive the pick copy</span>
    cloud: CloudUniform,
    <span class="k">pub</span> mvp_f32: [f32; <span class="s">16</span>],
    <span class="k">pub</span> ortho_h: f32,
    <span class="k">pub</span> eye: [f32; <span class="s">3</span>],
}</code></pre></div>
<p><code>lessons/04a/src/engine/gpu/frame.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">impl</span> FrameUniforms {
    <span class="c">/// \`'a\` on both inputs: the Binds may outlive neither self nor instances.</span>
    <span class="k">pub</span> <span class="k">fn</span> binds&lt;'a&gt;(&amp;'a <span class="k">self</span>, instances: &amp;'a wgpu::BindGroup) -&gt; Binds&lt;'a&gt; {
        Binds {
            mvp: &amp;<span class="k">self</span>.mvp_group,
            line: &amp;<span class="k">self</span>.line_group,
            instances,
        }
    }

    <span class="k">pub</span> <span class="k">fn</span> pick_binds&lt;'a&gt;(&amp;'a <span class="k">self</span>, instances: &amp;'a wgpu::BindGroup) -&gt; Binds&lt;'a&gt; {
        Binds {
            mvp: &amp;<span class="k">self</span>.pick_mvp_group,
            line: &amp;<span class="k">self</span>.pick_line_group,
            instances,
        }
    }

    <span class="c">/// For the memory counters: 448 bytes in all.</span>
    <span class="k">pub</span> <span class="k">fn</span> allocated_bytes(&amp;<span class="k">self</span>) -&gt; u64 {
        <span class="k">self</span>.mvp_buffer.size()
            + <span class="k">self</span>.line_buffer.size()
            + <span class="k">self</span>.cloud_buffer.size()
            + <span class="k">self</span>.pick_mvp_buffer.size()
            + <span class="k">self</span>.pick_line_buffer.size()
            + <span class="k">self</span>.pick_cloud_buffer.size()
            + <span class="k">self</span>.pick_transform_buffer.size()
    }

    <span class="k">pub</span> <span class="k">fn</span> new(ctx: &amp;GpuCtx, l: &amp;Layouts, size: (u32, u32)) -&gt; <span class="k">Self</span> {
        <span class="k">let</span> mvp_buffer = uniform_buffer(&amp;ctx.device, &quot;<span class="s">mvp.buffer</span>&quot;, &amp;Xform::identity().to_f32());
        <span class="k">let</span> line = LineUniform {
            thickness: <span class="s">2</span>.<span class="s">0</span>,
            proj_y: <span class="s">1</span>.<span class="s">0</span>,
            ortho_h: <span class="s">0</span>.<span class="s">0</span>,
            vp_h: size.<span class="s">1</span> <span class="k">as</span> f32,
            vp_w: size.<span class="s">0</span> <span class="k">as</span> f32,
            eye: [<span class="s">0</span>.<span class="s">0</span>; <span class="s">3</span>],
            anchor: [<span class="s">0</span>.<span class="s">0</span>; <span class="s">3</span>],
            feather: <span class="s">1</span>.<span class="s">5</span>,
            lit: <span class="s">0</span>.<span class="s">0</span>,
            backface: <span class="s">0</span>.<span class="s">0</span>,
            origin: [<span class="s">0</span>.<span class="s">0</span>; <span class="s">2</span>],
            frame: [size.<span class="s">0</span> <span class="k">as</span> f32, size.<span class="s">1</span> <span class="k">as</span> f32],
            opacity: <span class="s">1</span>.<span class="s">0</span>,
            _pad: <span class="s">0</span>.<span class="s">0</span>,</code></pre></div>
<p><code>lessons/04a/src/engine/gpu/frame.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        };
        <span class="k">let</span> line_buffer = uniform_buffer(&amp;ctx.device, &quot;<span class="s">line.buffer</span>&quot;, &amp;line);
        <span class="k">let</span> cloud = CloudUniform {
            size: <span class="s">1</span>.<span class="s">0</span>,
            vp_w: size.<span class="s">0</span> <span class="k">as</span> f32,
            vp_h: size.<span class="s">1</span> <span class="k">as</span> f32,
            edl: <span class="s">0</span>.<span class="s">0</span>,
            _pad0: <span class="s">0</span>.<span class="s">0</span>,
            _pad1: <span class="s">0</span>.<span class="s">0</span>,
            origin: [<span class="s">0</span>.<span class="s">0</span>; <span class="s">2</span>],
            frame: [size.<span class="s">0</span> <span class="k">as</span> f32, size.<span class="s">1</span> <span class="k">as</span> f32],
            _pad: [<span class="s">0</span>.<span class="s">0</span>; <span class="s">2</span>],
        };
        <span class="k">let</span> cloud_buffer = uniform_buffer(&amp;ctx.device, &quot;<span class="s">cloud.buffer</span>&quot;, &amp;cloud);
        <span class="k">let</span> identity = Xform::identity().to_f32();
        <span class="k">let</span> pick_mvp_buffer = uniform_buffer(&amp;ctx.device, &quot;<span class="s">pick.mvp.buffer</span>&quot;, &amp;identity);
        <span class="k">let</span> pick_line_buffer = uniform_buffer(&amp;ctx.device, &quot;<span class="s">pick.line.buffer</span>&quot;, &amp;line);
        <span class="k">let</span> pick_cloud_buffer = uniform_buffer(&amp;ctx.device, &quot;<span class="s">pick.cloud.buffer</span>&quot;, &amp;cloud);
        <span class="k">let</span> pick_transform_buffer = uniform_buffer(&amp;ctx.device, &quot;<span class="s">pick.transform.buffer</span>&quot;, &amp;identity);

        <span class="k">let</span> mvp_group = bind_group(ctx, &amp;l.mvp, &quot;<span class="s">mvp.bind_group</span>&quot;, &amp;[&amp;mvp_buffer]);
        <span class="k">let</span> line_group = bind_group(ctx, &amp;l.line, &quot;<span class="s">line.bind_group</span>&quot;, &amp;[&amp;line_buffer]);
        <span class="k">let</span> cloud_group = bind_group(ctx, &amp;l.line, &quot;<span class="s">cloud.bind_group</span>&quot;, &amp;[&amp;cloud_buffer]); <span class="c">// same layout: one uniform buffer</span>
        <span class="k">let</span> pick_mvp_group = bind_group(ctx, &amp;l.mvp, &quot;<span class="s">pick.mvp.bind_group</span>&quot;, &amp;[&amp;pick_mvp_buffer]);
        <span class="k">let</span> pick_line_group =
            bind_group(ctx, &amp;l.line, &quot;<span class="s">pick.line.bind_group</span>&quot;, &amp;[&amp;pick_line_buffer]);
        <span class="k">let</span> pick_cloud_group =
            bind_group(ctx, &amp;l.line, &quot;<span class="s">pick.cloud.bind_group</span>&quot;, &amp;[&amp;pick_cloud_buffer]);
        <span class="k">let</span> pick_transform_group = bind_group(
            ctx,
            &amp;pick_transform_layout(ctx),
            &quot;<span class="s">pick.transform.bind_group</span>&quot;,
            &amp;[&amp;pick_transform_buffer],
        );

        <span class="k">Self</span> {
            mvp_buffer,
            line_buffer,
            cloud_buffer,
            mvp_group,
            line_group,
            cloud_group,
            pick_mvp_buffer,
            pick_line_buffer,
            pick_cloud_buffer,
            pick_transform_buffer,
            pick_mvp_group,
            pick_line_group,
            pick_cloud_group,
            pick_transform_group,
            line,
            cloud,
            mvp_f32: [<span class="s">0</span>.<span class="s">0</span>; <span class="s">16</span>],
            ortho_h: <span class="s">0</span>.<span class="s">0</span>,
            eye: [<span class="s">0</span>.<span class="s">0</span>; <span class="s">3</span>],
        }
    }

    <span class="c">/// Called once per frame before any draw; the copies land on the GPU at the next submit.</span>
    <span class="k">pub</span> <span class="k">fn</span> write(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, input: &amp;FrameInput, cx: &amp;FrameCx) {
        <span class="k">self</span>.mvp_f32 = input.view_proj.to_f32();
        <span class="k">self</span>.ortho_h = input.view_proj.ortho_half_height() <span class="k">as</span> f32;
        <span class="k">let</span> eye = input.view_proj.eye();
        <span class="k">self</span>.eye = [eye[<span class="s">0</span>] <span class="k">as</span> f32, eye[<span class="s">1</span>] <span class="k">as</span> f32, eye[<span class="s">2</span>] <span class="k">as</span> f32];
        ctx.queue
            .write_buffer(&amp;<span class="k">self</span>.mvp_buffer, <span class="s">0</span>, bytemuck::cast_slice(&amp;<span class="k">self</span>.mvp_f32));

        <span class="k">let</span> line = LineUniform {
            thickness: cx.view.thickness_px * cx.pixel_scale,
            feather: cx.view.feather_px,
            <span class="c">// world mm to clip units at distance 1</span>
            proj_y: <span class="s">1</span>.<span class="s">0</span> / (FOVY_DEG <span class="k">as</span> f32 * <span class="s">0</span>.<span class="s">5</span>).to_radians().tan() * <span class="s">0</span>.<span class="s">001</span>,
            ortho_h: <span class="k">self</span>.ortho_h,
            vp_h: cx.size.<span class="s">1</span> <span class="k">as</span> f32,
            vp_w: cx.size.<span class="s">0</span> <span class="k">as</span> f32,
            eye: <span class="k">self</span>.eye,
            anchor: cx.anchor,</code></pre></div>
<p><code>lessons/04a/src/engine/gpu/frame.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            lit: f32::from(cx.view.lit),
            backface: f32::from(cx.view.backface),
            origin: [<span class="s">0</span>.<span class="s">0</span>; <span class="s">2</span>],
            frame: [cx.size.<span class="s">0</span> <span class="k">as</span> f32, cx.size.<span class="s">1</span> <span class="k">as</span> f32],
            opacity: cx.view.opacity,
            _pad: <span class="s">0</span>.<span class="s">0</span>,
        };
        ctx.queue
            .write_buffer(&amp;<span class="k">self</span>.line_buffer, <span class="s">0</span>, bytemuck::bytes_of(&amp;line));
        <span class="k">self</span>.line = line;

        <span class="k">let</span> cloud = CloudUniform {
            size: cx.view.cloud_size,
            vp_w: cx.size.<span class="s">0</span> <span class="k">as</span> f32,
            vp_h: cx.size.<span class="s">1</span> <span class="k">as</span> f32,
            edl: cx.view.edl_strength,
            _pad0: <span class="s">0</span>.<span class="s">0</span>,
            _pad1: <span class="s">0</span>.<span class="s">0</span>,</code></pre></div>
<p><code>lessons/04a/src/engine/gpu/frame.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            origin: [<span class="s">0</span>.<span class="s">0</span>; <span class="s">2</span>],
            frame: [cx.size.<span class="s">0</span> <span class="k">as</span> f32, cx.size.<span class="s">1</span> <span class="k">as</span> f32],
            _pad: [<span class="s">0</span>.<span class="s">0</span>; <span class="s">2</span>],
        };
        ctx.queue
            .write_buffer(&amp;<span class="k">self</span>.cloud_buffer, <span class="s">0</span>, bytemuck::bytes_of(&amp;cloud));
        <span class="k">self</span>.cloud = cloud;
    }

    <span class="c">/// Write the same settings for the pick window; call after \`write\`.</span>
    <span class="k">pub</span> <span class="k">fn</span> write_pick(&amp;<span class="k">self</span>, ctx: &amp;GpuCtx, view: PickView, frame: (u32, u32)) {
        <span class="k">let</span> transform = view.clip_transform(frame);
        <span class="k">let</span> mvp = mat4_mul(&amp;transform, &amp;<span class="k">self</span>.mvp_f32);
        ctx.queue
            .write_buffer(&amp;<span class="k">self</span>.pick_mvp_buffer, <span class="s">0</span>, bytemuck::cast_slice(&amp;mvp));
        ctx.queue.write_buffer(
            &amp;<span class="k">self</span>.pick_transform_buffer,
            <span class="s">0</span>,
            bytemuck::cast_slice(&amp;transform),
        );
        <span class="c">// keeps pixel sizes equal in the smaller window</span>
        <span class="k">let</span> ratio = frame.<span class="s">1</span>.max(<span class="s">1</span>) <span class="k">as</span> f32 / view.h.max(<span class="s">1</span>) <span class="k">as</span> f32;
        <span class="k">let</span> line = LineUniform {
            proj_y: <span class="k">self</span>.line.proj_y * ratio,
            ortho_h: <span class="k">self</span>.line.ortho_h / ratio,
            vp_w: view.w <span class="k">as</span> f32,
            vp_h: view.h <span class="k">as</span> f32,
            origin: [view.x <span class="k">as</span> f32, view.y <span class="k">as</span> f32],
            ..<span class="k">self</span>.line
        };
        ctx.queue
            .write_buffer(&amp;<span class="k">self</span>.pick_line_buffer, <span class="s">0</span>, bytemuck::bytes_of(&amp;line));
        <span class="k">let</span> cloud = CloudUniform {
            vp_w: view.w <span class="k">as</span> f32,
            vp_h: view.h <span class="k">as</span> f32,
            origin: [view.x <span class="k">as</span> f32, view.y <span class="k">as</span> f32],
            ..<span class="k">self</span>.cloud
        };
        ctx.queue
            .write_buffer(&amp;<span class="k">self</span>.pick_cloud_buffer, <span class="s">0</span>, bytemuck::bytes_of(&amp;cloud));
    }
}</code></pre></div>
<p><code>lessons/04a/src/engine/gpu/frame.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>#[cfg(test)]
<span class="k">mod</span> tests {
    <span class="k">use</span> super::*;

    <span class="c">/// Whole canvas maps to identity; a window maps its corners to ±1.</span>
    #[test]
    <span class="k">fn</span> pick_view_clip_transform() {
        <span class="k">let</span> whole = PickView::whole((<span class="s">1600</span>, <span class="s">1000</span>)).clip_transform((<span class="s">1600</span>, <span class="s">1000</span>));
        assert_eq!(whole, Xform::identity().to_f32());
        <span class="k">let</span> view = PickView {
            x: <span class="s">100</span>,
            y: <span class="s">250</span>,
            w: <span class="s">19</span>,
            h: <span class="s">19</span>,
        };
        <span class="k">let</span> t = view.clip_transform((<span class="s">1600</span>, <span class="s">1000</span>));
        <span class="c">// canvas pixel to clip space</span>
        <span class="k">let</span> ndc = |px: f32, py: f32| [px / <span class="s">800</span>.<span class="s">0</span> - <span class="s">1</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span> - py / <span class="s">500</span>.<span class="s">0</span>];</code></pre></div>
<p>Copy this part from the lesson folder to the path shown.</p>
<p><code>lessons/04a/src/engine/gpu/frame.rs</code> · copy the file, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> apply = |ndc: [f32; <span class="s">2</span>]| [t[<span class="s">0</span>] * ndc[<span class="s">0</span>] + t[<span class="s">12</span>], t[<span class="s">5</span>] * ndc[<span class="s">1</span>] + t[<span class="s">13</span>]];
        <span class="k">let</span> left_top = apply(ndc(<span class="s">100</span>.<span class="s">0</span>, <span class="s">250</span>.<span class="s">0</span>));
        <span class="k">let</span> right_bottom = apply(ndc(<span class="s">119</span>.<span class="s">0</span>, <span class="s">269</span>.<span class="s">0</span>));
        assert!((left_top[<span class="s">0</span>] + <span class="s">1</span>.<span class="s">0</span>).abs() &lt; <span class="s">1</span>e-<span class="s">4</span> &amp;&amp; (left_top[<span class="s">1</span>] - <span class="s">1</span>.<span class="s">0</span>).abs() &lt; <span class="s">1</span>e-<span class="s">4</span>);
        assert!((right_bottom[<span class="s">0</span>] - <span class="s">1</span>.<span class="s">0</span>).abs() &lt; <span class="s">1</span>e-<span class="s">4</span> &amp;&amp; (right_bottom[<span class="s">1</span>] + <span class="s">1</span>.<span class="s">0</span>).abs() &lt; <span class="s">1</span>e-<span class="s">4</span>);
    }
}</code></pre></div>
<h2 id="step-8-srcenginegpuviewrs">Step 8 · src/engine/gpu/view.rs<a class="anchor" href="#/course/04a-meshes#step-8-srcenginegpuviewrs" aria-label="Link to this section">#</a></h2>
<p>New file: the display settings, each read once from the page URL or an env variable.
Copy this file from the lesson folder to the path shown.</p>
<p><code>lessons/04a/src/engine/gpu/view.rs</code> · copy the file, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Display settings; each starts from the page URL, e.g. \`?msaa=4\`, or natively from an env variable.</span>
<span class="k">pub</span> <span class="k">struct</span> View {
    <span class="k">pub</span> show_grid: bool,
    <span class="k">pub</span> show_points: bool, <span class="c">// point markers, \`Q\`</span>
    <span class="k">pub</span> show_lines: bool, <span class="c">// lines and curves, \`W\`</span>
    <span class="k">pub</span> show_mesh_edges: bool, <span class="c">// mesh edges and their vertex markers, \`E\`</span>
    <span class="k">pub</span> markers: bool, <span class="c">// vertex markers on mesh edges</span>
    <span class="k">pub</span> cloud_size: f32, <span class="c">// point size scale, \`[\` and \`]\`</span>
    <span class="k">pub</span> edl_strength: f32, <span class="c">// 0 = off</span>
    <span class="k">pub</span> lod_px: f32, <span class="c">// LOD = level of detail: skip cloud points smaller than this, px; 0 = draw all</span>
    <span class="k">pub</span> thickness_px: f32, <span class="c">// line width, CSS px</span>
    <span class="k">pub</span> feather_px: f32, <span class="c">// edge softness of dots, px</span>
    <span class="k">pub</span> lit: bool, <span class="c">// headlight on mesh faces, \`D\`</span>
    <span class="k">pub</span> backface: bool, <span class="c">// back faces painted red, \`B\`</span>
    <span class="k">pub</span> opacity: f32, <span class="c">// face alpha; 0 = x-ray, \`P\` toggles</span>
    <span class="k">pub</span> msaa_forced: Option&lt;u32&gt;, <span class="c">// 4 forces 4x MSAA, other values 1x</span>
    <span class="k">pub</span> perf: bool, <span class="c">// draw every frame and show timing</span>
    <span class="k">pub</span> spin: bool, <span class="c">// orbit a little every frame</span>
}

<span class="k">impl</span> View {
    <span class="c">/// Read every setting once at start.</span>
    <span class="k">pub</span> <span class="k">fn</span> from_env() -&gt; <span class="k">Self</span> {
        <span class="k">Self</span> {
            show_grid: knob(&quot;<span class="s">VIEWER_NO_GRID</span>&quot;, &quot;<span class="s">nogrid</span>&quot;).is_none(),
            show_points: <span class="s">true</span>,
            show_lines: <span class="s">true</span>,
            show_mesh_edges: <span class="s">true</span>,
            markers: knob(&quot;<span class="s">BENCH_NO_MARKERS</span>&quot;, &quot;<span class="s">nomarkers</span>&quot;).is_none(),
            cloud_size: knob_f32(&quot;<span class="s">VIEWER_CLOUD_SCALE</span>&quot;, &quot;<span class="s">cloud</span>&quot;, <span class="s">1</span>.<span class="s">0</span>),
            edl_strength: knob_f32(&quot;<span class="s">VIEWER_EDL</span>&quot;, &quot;<span class="s">edl</span>&quot;, <span class="s">0</span>.<span class="s">25</span>),
            lod_px: knob_f32(&quot;<span class="s">VIEWER_LOD</span>&quot;, &quot;<span class="s">lod</span>&quot;, <span class="s">0</span>.<span class="s">0</span>),
            thickness_px: knob_f32(&quot;<span class="s">VIEWER_THICKNESS</span>&quot;, &quot;<span class="s">thickness</span>&quot;, <span class="s">1</span>.<span class="s">5</span>).max(<span class="s">0</span>.<span class="s">1</span>),
            feather_px: knob_f32(&quot;<span class="s">VIEWER_AA</span>&quot;, &quot;<span class="s">aa</span>&quot;, <span class="s">1</span>.<span class="s">0</span>).clamp(<span class="s">0</span>.<span class="s">5</span>, <span class="s">4</span>.<span class="s">0</span>),
            lit: knob(&quot;<span class="s">VIEWER_LIT</span>&quot;, &quot;<span class="s">lit</span>&quot;).is_some(),
            backface: knob(&quot;<span class="s">VIEWER_BACKFACE</span>&quot;, &quot;<span class="s">backface</span>&quot;).is_some(),
            opacity: knob_f32(&quot;<span class="s">VIEWER_OPACITY</span>&quot;, &quot;<span class="s">opacity</span>&quot;, <span class="s">1</span>.<span class="s">0</span>).clamp(<span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>),
            msaa_forced: knob_u32(&quot;<span class="s">VIEWER_MSAA</span>&quot;, &quot;<span class="s">msaa</span>&quot;),
            perf: knob(&quot;<span class="s">VIEWER_PERF</span>&quot;, &quot;<span class="s">perf</span>&quot;).is_some(),
            spin: knob(&quot;<span class="s">VIEWER_SPIN</span>&quot;, &quot;<span class="s">spin</span>&quot;).is_some(),
        }
    }
}

<span class="c">/// One setting's text: \`?query=\` in the browser, \`ENV\` natively.</span>
<span class="k">pub</span> <span class="k">fn</span> knob(env: &amp;str, query: &amp;str) -&gt; Option&lt;String&gt; {
    <span class="c">// cfg: only one of these two blocks is compiled, picked by the build target.</span>
    #[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
    {
        <span class="k">let</span> _ = env;
        <span class="k">crate</span>::app::route::query(query)
    }
    #[cfg(not(target_arch = &quot;<span class="s">wasm32</span>&quot;))]
    {
        <span class="k">let</span> _ = query;
        std::env::var(env).ok()
    }
}

<span class="c">/// A float setting, or \`default\`.</span>
<span class="k">fn</span> knob_f32(env: &amp;str, query: &amp;str, default: f32) -&gt; f32 {
    <span class="c">// let-else: bind raw, or leave the function when there is nothing to bind.</span>
    <span class="k">let</span> Some(raw) = knob(env, query) <span class="k">else</span> {
        <span class="k">return</span> default;
    };

    <span class="k">match</span> raw.parse::&lt;f32&gt;() {
        Ok(value) <span class="k">if</span> value.is_finite() =&gt; value, <span class="c">// a match guard: &quot;inf&quot; and &quot;NaN&quot; parse too, but are refused</span>
        _ =&gt; default,
    }
}

<span class="c">/// An integer setting, or None.</span>
<span class="k">fn</span> knob_u32(env: &amp;str, query: &amp;str) -&gt; Option&lt;u32&gt; {
    knob(env, query)?.parse().ok() <span class="c">// \`?\` on an Option returns None early, as it returns Err on a Result</span>
}</code></pre></div>
<h2 id="step-9-srcappmodrs">Step 9 · src/app/mod.rs<a class="anchor" href="#/course/04a-meshes#step-9-srcappmodrs" aria-label="Link to this section">#</a></h2>
<p>New file: the app module, one line for now.</p>
<p><code>lessons/04a/src/app/mod.rs</code> · 1 lines · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> <span class="k">mod</span> route;</code></pre></div>
<h2 id="step-10-srcapprouters">Step 10 · src/app/route.rs<a class="anchor" href="#/course/04a-meshes#step-10-srcapprouters" aria-label="Link to this section">#</a></h2>
<p>New file: read one value from the page URL.</p>
<p><code>lessons/04a/src/app/route.rs</code> · 14 lines · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// The first URL query value for \`key\`.</span>
<span class="k">pub</span> <span class="k">fn</span> query(key: &amp;str) -&gt; Option&lt;String&gt; {
    <span class="k">let</span> search = web_sys::window()?.location().search().ok()?;

    <span class="k">for</span> pair <span class="k">in</span> search.trim_start_matches('<span class="s">?</span>').split('<span class="s">&amp;</span>') {
        <span class="k">let</span> (name, value) = pair.split_once('<span class="s">=</span>').unwrap_or((pair, &quot;&quot;));

        <span class="k">if</span> name == key {
            <span class="k">return</span> Some(js_sys::decode_uri_component(value).ok()?.into());
        }
    }

    None
}</code></pre></div>
<h2 id="step-11-srcenginegpuobjectsrs">Step 11 · src/engine/gpu/objects.rs<a class="anchor" href="#/course/04a-meshes#step-11-srcenginegpuobjectsrs" aria-label="Link to this section">#</a></h2>
<p>New file: the object table on the GPU.</p>
<p><code>lessons/04a/src/engine/gpu/objects.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> super::buffers::{GpuCtx, GrowBuf, ROWS, bind_group};
<span class="k">use</span> super::instance::Instance;
<span class="k">use</span> super::targets::Targets;
<span class="k">use</span> <span class="k">crate</span>::engine::pipelines::Layouts;
<span class="k">use</span> session_rust::{AABB, Point, Xform};

<span class="c">/// Smallest camera drift, world units, that moves the anchor.</span>
<span class="k">const</span> REANCHOR_MIN: f64 = <span class="s">1</span>.<span class="s">0</span>e<span class="s">3</span>;

<span class="c">/// Largest camera drift, world units, before the anchor must move.</span>
<span class="k">const</span> REANCHOR_MAX: f64 = <span class="s">1</span>.<span class="s">0</span>e<span class="s">5</span>;

<span class="c">/// Least time between two anchor moves, ms.</span>
<span class="k">const</span> REANCHOR_THROTTLE_MS: f64 = <span class="s">200</span>.<span class="s">0</span>;

<span class="c">/// One object row as the CPU builds it.</span>
#[derive(Clone)]
<span class="k">pub</span> <span class="k">struct</span> ObjectRow {
    <span class="k">pub</span> place: Xform, <span class="c">// world placement</span>
    <span class="k">pub</span> color: [f32; <span class="s">4</span>], <span class="c">// rgba tint</span></code></pre></div>
<p><code>lessons/04a/src/engine/gpu/objects.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> flags: u32, <span class="c">// Instance::FLAG_* bits</span>
    <span class="k">pub</span> bounds: AABB, <span class="c">// box in the object's own space</span>
    <span class="k">pub</span> spacing: f32, <span class="c">// vertex spacing, or point size for clouds</span>
    <span class="k">pub</span> faces: bool, <span class="c">// true when the object drew faces</span>
}

<span class="k">impl</span> ObjectRow {
    <span class="c">/// A row with a placement and flags, everything else empty.</span>
    <span class="k">pub</span> <span class="k">fn</span> new(place: Xform, flags: u32) -&gt; <span class="k">Self</span> {
        <span class="k">Self</span> {
            place,
            color: [<span class="s">1</span>.<span class="s">0</span>; <span class="s">4</span>],
            flags,
            bounds: AABB::empty(),
            spacing: <span class="s">0</span>.<span class="s">0</span>,
            faces: <span class="s">false</span>,
        }
    }
}

<span class="c">/// Object rows of one upload.</span>
#[derive(Default)]
<span class="k">pub</span> <span class="k">struct</span> ObjectRows {
    <span class="k">pub</span> rows: Vec&lt;ObjectRow&gt;, <span class="c">// one per object</span>
}

<span class="c">/// Result of a \`rebase_anchor\` call.</span>
<span class="k">pub</span> <span class="k">struct</span> Rebase {
    <span class="k">pub</span> anchor: Point, <span class="c">// current scene origin</span>
    <span class="k">pub</span> moved: bool, <span class="c">// true when the table was rebuilt now</span>
    <span class="k">pub</span> pending: bool, <span class="c">// true when a rebuild waits on the throttle</span>
}

<span class="c">/// A row with faces and its world box, for the inside test.</span>
<span class="k">struct</span> BoundedRow {
    row: u32, <span class="c">// object row</span></code></pre></div>
<p><code>lessons/04a/src/engine/gpu/objects.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    lo: [f64; <span class="s">3</span>], <span class="c">// box minimum</span>
    hi: [f64; <span class="s">3</span>], <span class="c">// box maximum</span>
}

<span class="c">/// The row's box in world space.</span>
<span class="k">fn</span> world_box(r: &amp;ObjectRow) -&gt; AABB {
    r.bounds.transformed(&amp;r.place)
}

<span class="c">/// The object rows on the GPU and their exact positions on the CPU.</span>
<span class="k">pub</span> <span class="k">struct</span> InstanceTable {
    rows: Vec&lt;Instance&gt;, <span class="c">// the rows, as uploaded</span>
    translation: Vec&lt;[f64; 3]&gt;, <span class="c">// exact world position per row</span>
    bounded: Vec&lt;BoundedRow&gt;, <span class="c">// rows with faces, for the inside test</span>
    world_bounds: Vec&lt;AABB&gt;, <span class="c">// box per row in world space</span>
    last_origin: Option&lt;Point&gt;, <span class="c">// origin the GPU positions are measured from</span>
    buffer: GrowBuf, <span class="c">// Instance rows on the GPU</span>
    translations: GrowBuf, <span class="c">// positions minus the scene origin, on the GPU</span>
    last_rebase_ms: f64, <span class="c">// when the origin last moved</span>
    <span class="k">pub</span> group: wgpu::BindGroup, <span class="c">// group 2: rows and translations</span>
    <span class="k">pub</span> ink_group: wgpu::BindGroup, <span class="c">// group 2 for ink, with depth textures</span>
}

<span class="c">/// Bind group 2: rows at binding 0, translations at 1.</span>
<span class="k">fn</span> instance_group(
    ctx: &amp;GpuCtx,
    l: &amp;Layouts,
    rows: &amp;wgpu::Buffer,
    translations: &amp;wgpu::Buffer,
) -&gt; wgpu::BindGroup {
    bind_group(
        ctx,
        &amp;l.instance,
        &quot;<span class="s">instances.bind_group</span>&quot;,
        &amp;[rows, translations],
    )
}

<span class="c">/// Textures and tiles the ink bind group reads.</span>
<span class="k">pub</span> <span class="k">struct</span> InkScene&lt;'a&gt; {
    <span class="k">pub</span> targets: &amp;'a Targets, <span class="c">// depth and gradient textures</span>
}

<span class="c">/// Bind group 2 for ink lanes: rows, depth, gradient, tiles.</span>
<span class="k">fn</span> ink_instance_group(
    ctx: &amp;GpuCtx,
    l: &amp;Layouts,
    buffers: [&amp;wgpu::Buffer; <span class="s">2</span>],
    scene: &amp;InkScene,</code></pre></div>
<p><code>lessons/04a/src/engine/gpu/objects.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>) -&gt; wgpu::BindGroup {
    <span class="k">let</span> targets = scene.targets;
    ctx.device.create_bind_group(&amp;wgpu::BindGroupDescriptor {
        label: Some(&quot;<span class="s">ink.instances.bind_group</span>&quot;),
        layout: &amp;l.ink_instance,
        entries: &amp;[
            wgpu::BindGroupEntry {
                binding: <span class="s">0</span>,
                resource: buffers[<span class="s">0</span>].as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: <span class="s">1</span>,
                resource: buffers[<span class="s">1</span>].as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: <span class="s">2</span>,
                resource: wgpu::BindingResource::TextureView(&amp;targets.depth_single),
            },
            wgpu::BindGroupEntry {
                binding: <span class="s">3</span>,
                resource: wgpu::BindingResource::TextureView(&amp;targets.depth_msaa),
            },
        ],
    })
}

<span class="k">impl</span> InstanceTable {
    <span class="k">pub</span> <span class="k">fn</span> allocated_bytes(&amp;<span class="k">self</span>) -&gt; u64 {
        <span class="k">self</span>.buffer.buf.size() + <span class="k">self</span>.translations.buf.size()
    }

    <span class="k">pub</span> <span class="k">fn</span> new(ctx: &amp;GpuCtx, l: &amp;Layouts, scene: &amp;InkScene) -&gt; <span class="k">Self</span> {
        <span class="k">let</span> buffer = GrowBuf::new(
            ctx,
            &quot;<span class="s">instance.buffer</span>&quot;,
            std::mem::size_of::&lt;Instance&gt;() <span class="k">as</span> u64,
            ROWS,
        );
        <span class="k">let</span> translations = GrowBuf::new(ctx, &quot;<span class="s">instance.translations</span>&quot;, <span class="s">16</span>, ROWS);
        <span class="k">let</span> group = instance_group(ctx, l, &amp;buffer.buf, &amp;translations.buf);
        <span class="k">let</span> ink_group = ink_instance_group(ctx, l, [&amp;buffer.buf, &amp;translations.buf], scene);

        <span class="k">Self</span> {
            rows: vec![Instance::placeholder()],
            translation: Vec::new(),
            bounded: Vec::new(),
            world_bounds: Vec::new(),
            last_origin: None,
            buffer,
            translations,
            last_rebase_ms: <span class="s">0</span>.<span class="s">0</span>,
            group,
            ink_group,
        }
    }

    <span class="c">/// Rebuild the ink bind group.</span>
    <span class="k">pub</span> <span class="k">fn</span> rebind_ink(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, l: &amp;Layouts, scene: &amp;InkScene) {
        <span class="k">self</span>.ink_group =
            ink_instance_group(ctx, l, [&amp;<span class="k">self</span>.buffer.buf, &amp;<span class="k">self</span>.translations.buf], scene);
    }

    <span class="c">/// An ink bind group over the pick pass's own depth textures.</span>
    <span class="k">pub</span> <span class="k">fn</span> pick_group(
        &amp;<span class="k">self</span>,
        ctx: &amp;GpuCtx,
        layouts: &amp;Layouts,
        depths: [&amp;wgpu::TextureView; <span class="s">2</span>],
    ) -&gt; wgpu::BindGroup {</code></pre></div>
<p><code>lessons/04a/src/engine/gpu/objects.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        ctx.device.create_bind_group(&amp;wgpu::BindGroupDescriptor {
            label: Some(&quot;<span class="s">pick.instances</span>&quot;),
            layout: &amp;layouts.ink_instance,
            entries: &amp;[
                wgpu::BindGroupEntry {
                    binding: <span class="s">0</span>,
                    resource: <span class="k">self</span>.buffer.buf.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: <span class="s">1</span>,
                    resource: <span class="k">self</span>.translations.buf.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: <span class="s">2</span>,
                    resource: wgpu::BindingResource::TextureView(depths[<span class="s">0</span>]),
                },
                wgpu::BindGroupEntry {
                    binding: <span class="s">3</span>,
                    resource: wgpu::BindingResource::TextureView(depths[<span class="s">1</span>]),
                },
            ],
        })
    }

    <span class="c">/// Append one upload's rows.</span>
    <span class="k">pub</span> <span class="k">fn</span> append(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, l: &amp;Layouts, up: &amp;ObjectRows) {
        <span class="c">// first upload replaces the placeholder</span>
        <span class="k">if</span> <span class="k">self</span>.translation.is_empty() {
            <span class="k">self</span>.rows.clear();
            <span class="k">self</span>.world_bounds.clear();
            <span class="k">self</span>.buffer.reset();
            <span class="k">self</span>.translations.reset();
        }

        <span class="c">// row number of the first new object</span>
        <span class="k">let</span> base = <span class="k">self</span>.translation.len() <span class="k">as</span> u32;
        <span class="k">self</span>.rows.reserve(up.rows.len());
        <span class="k">self</span>.translation.reserve(up.rows.len());
        <span class="k">self</span>.world_bounds.reserve(up.rows.len());

        <span class="k">for</span> (i, r) <span class="k">in</span> up.rows.iter().enumerate() {
            <span class="k">let</span> world = world_box(r);

            <span class="c">// rows with faces join the inside test</span>
            <span class="k">if</span> r.faces &amp;&amp; world.is_valid() {
                <span class="k">let</span> lo = [
                    world.min_point()[<span class="s">0</span>],
                    world.min_point()[<span class="s">1</span>],
                    world.min_point()[<span class="s">2</span>],
                ];
                <span class="k">let</span> hi = [
                    world.max_point()[<span class="s">0</span>],
                    world.max_point()[<span class="s">1</span>],
                    world.max_point()[<span class="s">2</span>],
                ];
                <span class="k">self</span>.bounded.push(BoundedRow {
                    row: base + i <span class="k">as</span> u32,
                    lo,
                    hi,
                });
            }

            <span class="k">self</span>.world_bounds.push(<span class="k">if</span> world.is_valid() {
                world
            } <span class="k">else</span> {
                AABB::empty()
            });
            <span class="k">self</span>.translation
                .push([r.place.m[<span class="s">12</span>], r.place.m[<span class="s">13</span>], r.place.m[<span class="s">14</span>]]);
            <span class="c">// matrix without translation</span>
            <span class="k">let</span> <span class="k">mut</span> model = r.place.to_f32();
            model[<span class="s">12</span>] = <span class="s">0</span>.<span class="s">0</span>;
            model[<span class="s">13</span>] = <span class="s">0</span>.<span class="s">0</span>;
            model[<span class="s">14</span>] = <span class="s">0</span>.<span class="s">0</span>;</code></pre></div>
<p><code>lessons/04a/src/engine/gpu/objects.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">self</span>.rows.push(Instance {
                model,
                color: r.color,
                flags: r.flags,
                _pad0: <span class="s">0</span>.<span class="s">0</span>,
                spacing: r.spacing,
                _pad: <span class="s">0</span>,
            });
        }

        <span class="k">if</span> <span class="k">self</span>.rows.is_empty() {
            <span class="k">self</span>.rows.push(Instance::placeholder());
        }

        <span class="c">// rows not yet on the GPU</span>
        <span class="k">let</span> fresh = &amp;<span class="k">self</span>.rows[<span class="k">self</span>.buffer.len() <span class="k">as</span> usize..];

        <span class="k">if</span> fresh.is_empty() {
            <span class="k">return</span>;
        }

        <span class="c">// translations are filled by the next rebase</span>
        <span class="k">let</span> zeros = vec![[<span class="s">0</span>.<span class="s">0f32</span>; <span class="s">4</span>]; fresh.len()];
        <span class="k">let</span> grew = <span class="k">self</span>.buffer.append(ctx, fresh);

        <span class="k">if</span> <span class="k">self</span>.translations.append(ctx, &amp;zeros) || grew {
            <span class="k">self</span>.group = instance_group(ctx, l, &amp;<span class="k">self</span>.buffer.buf, &amp;<span class="k">self</span>.translations.buf);
        }

        <span class="k">self</span>.last_origin = None;
    }

    <span class="c">/// Move the scene origin when the camera drifted far enough.</span>
    <span class="k">pub</span> <span class="k">fn</span> rebase_anchor(
        &amp;<span class="k">mut</span> <span class="k">self</span>,
        ctx: &amp;GpuCtx,
        origin: &amp;Point,
        view_dist: f64,
        now: f64,
    ) -&gt; Rebase {
        <span class="c">// a quarter of the view distance, clamped</span>
        <span class="k">let</span> thresh = (view_dist * <span class="s">0</span>.<span class="s">25</span>).clamp(REANCHOR_MIN, REANCHOR_MAX);
        <span class="c">// drifted past the threshold?</span>
        <span class="k">let</span> need = <span class="k">match</span> &amp;<span class="k">self</span>.last_origin {
            None =&gt; <span class="s">true</span>,
            Some(a) =&gt; {
                <span class="k">let</span> d = [a[<span class="s">0</span>] - origin[<span class="s">0</span>], a[<span class="s">1</span>] - origin[<span class="s">1</span>], a[<span class="s">2</span>] - origin[<span class="s">2</span>]];
                (d[<span class="s">0</span>] * d[<span class="s">0</span>] + d[<span class="s">1</span>] * d[<span class="s">1</span>] + d[<span class="s">2</span>] * d[<span class="s">2</span>]).sqrt() &gt; thresh
            }
        };
        <span class="c">// rebuild now unless throttled</span>
        <span class="k">let</span> moved = need
            &amp;&amp; (<span class="k">self</span>.last_origin.is_none() || now - <span class="k">self</span>.last_rebase_ms &gt; REANCHOR_THROTTLE_MS);

        <span class="k">if</span> moved {
            <span class="k">self</span>.rebuild(ctx, origin);
            <span class="k">self</span>.last_rebase_ms = now;
        }

        Rebase {
            <span class="c">// set by rebuild above or earlier</span>
            anchor: <span class="k">self</span>.last_origin.clone().unwrap(),
            moved,
            pending: need &amp;&amp; !moved,
        }
    }</code></pre></div>
<p><code>lessons/04a/src/engine/gpu/objects.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Recompute every relative translation for a new origin.</span>
    <span class="k">fn</span> rebuild(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, origin: &amp;Point) {
        <span class="k">self</span>.last_origin = Some(origin.clone());
        <span class="k">let</span> <span class="k">mut</span> anchored: Vec&lt;[f32; 4]&gt; = Vec::with_capacity(<span class="k">self</span>.rows.len());

        <span class="k">for</span> t <span class="k">in</span> &amp;<span class="k">self</span>.translation {
            anchored.push([
                (t[<span class="s">0</span>] - origin[<span class="s">0</span>]) <span class="k">as</span> f32,
                (t[<span class="s">1</span>] - origin[<span class="s">1</span>]) <span class="k">as</span> f32,
                (t[<span class="s">2</span>] - origin[<span class="s">2</span>]) <span class="k">as</span> f32,
                <span class="s">0</span>.<span class="s">0</span>,
            ]);
        }

        anchored.resize(<span class="k">self</span>.rows.len(), [<span class="s">0</span>.<span class="s">0</span>; <span class="s">4</span>]);
        <span class="k">self</span>.translations.write_at(ctx, <span class="s">0</span>, &amp;anchored);
    }

    <span class="c">/// Row \`i\`'s full matrix as the shader composes it.</span>
    <span class="k">pub</span> <span class="k">fn</span> anchored_model(&amp;<span class="k">self</span>, i: u32) -&gt; Option&lt;[f32; 16]&gt; {
        <span class="k">let</span> <span class="k">mut</span> model = <span class="k">self</span>.rows.get(i <span class="k">as</span> usize)?.model;

        <span class="k">if</span> <span class="k">let</span> (Some(t), Some(o)) = (<span class="k">self</span>.translation.get(i <span class="k">as</span> usize), &amp;<span class="k">self</span>.last_origin) {
            model[<span class="s">12</span>] = (t[<span class="s">0</span>] - o[<span class="s">0</span>]) <span class="k">as</span> f32;
            model[<span class="s">13</span>] = (t[<span class="s">1</span>] - o[<span class="s">1</span>]) <span class="k">as</span> f32;
            model[<span class="s">14</span>] = (t[<span class="s">2</span>] - o[<span class="s">2</span>]) <span class="k">as</span> f32;
        }

        Some(model)
    }

    <span class="c">/// Set FLAG_INSIDE on rows whose box contains the eye.</span>
    <span class="k">pub</span> <span class="k">fn</span> update_inside(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, eye: [f32; <span class="s">3</span>], scene: &amp;AABB) {
        <span class="k">if</span> <span class="k">self</span>.bounded.is_empty() {
            <span class="k">return</span>;
        }

        <span class="k">let</span> Some(origin) = <span class="k">self</span>.last_origin.clone() <span class="k">else</span> {
            <span class="k">return</span>;
        };
        <span class="c">// eye in world space</span>
        <span class="k">let</span> ew = [
            origin[<span class="s">0</span>] + eye[<span class="s">0</span>] <span class="k">as</span> f64,
            origin[<span class="s">1</span>] + eye[<span class="s">1</span>] <span class="k">as</span> f64,
            origin[<span class="s">2</span>] + eye[<span class="s">2</span>] <span class="k">as</span> f64,
        ];
        <span class="k">let</span> in_scene = scene.contains(&amp;Point::new(ew[<span class="s">0</span>], ew[<span class="s">1</span>], ew[<span class="s">2</span>]));

        <span class="k">for</span> b <span class="k">in</span> &amp;<span class="k">self</span>.bounded {
            <span class="k">let</span> <span class="k">mut</span> inside = in_scene;

            <span class="k">if</span> inside {
                <span class="k">for</span> (coordinate, (low, high)) <span class="k">in</span> ew.iter().zip(b.lo.iter().zip(&amp;b.hi)) {
                    <span class="k">if</span> !(coordinate &gt;= low &amp;&amp; coordinate &lt;= high) {
                        inside = <span class="s">false</span>;
                        <span class="k">break</span>;</code></pre></div>
<p><code>lessons/04a/src/engine/gpu/objects.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                    }
                }
            }

            <span class="k">let</span> Some(row) = <span class="k">self</span>.rows.get_mut(b.row <span class="k">as</span> usize) <span class="k">else</span> {
                <span class="k">continue</span>;
            };

            <span class="c">// write only when the answer changed</span>
            <span class="k">if</span> (row.flags &amp; Instance::FLAG_INSIDE != <span class="s">0</span>) == inside {
                <span class="k">continue</span>;
            }

            row.flags ^= Instance::FLAG_INSIDE;
            <span class="k">self</span>.buffer.write_at(ctx, b.row, std::slice::from_ref(row));
        }
    }

    <span class="c">/// Set or clear one flag bit on one row.</span>
    <span class="k">pub</span> <span class="k">fn</span> set_flag(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, row: u32, bit: u32, on: bool) {
        <span class="k">let</span> Some(r) = <span class="k">self</span>.rows.get_mut(row <span class="k">as</span> usize) <span class="k">else</span> {
            <span class="k">return</span>;
        };
        <span class="k">let</span> was = r.flags &amp; bit != <span class="s">0</span>;

        <span class="k">if</span> was == on {
            <span class="k">return</span>;
        }

        r.flags ^= bit;
        <span class="k">self</span>.buffer.write_at(ctx, row, std::slice::from_ref(r));
    }

    <span class="c">/// Forget every row; keep the buffers.</span>
    <span class="k">pub</span> <span class="k">fn</span> reset(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">self</span>.rows.clear();
        <span class="k">self</span>.translation.clear();
        <span class="k">self</span>.bounded.clear();
        <span class="k">self</span>.world_bounds.clear();
        <span class="k">self</span>.buffer.reset();
        <span class="k">self</span>.translations.reset();
        <span class="k">self</span>.last_origin = None;
    }

    <span class="c">/// Forget every row and free the memory.</span>
    <span class="k">pub</span> <span class="k">fn</span> release(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, l: &amp;Layouts) {
        <span class="k">self</span>.reset();
        <span class="k">self</span>.rows.shrink_to_fit();
        <span class="k">self</span>.translation.shrink_to_fit();
        <span class="k">self</span>.bounded.shrink_to_fit();
        <span class="k">self</span>.world_bounds.shrink_to_fit();
        <span class="k">self</span>.rows.push(Instance::placeholder());
        <span class="k">self</span>.buffer.release(ctx);
        <span class="k">self</span>.translations.release(ctx);
        <span class="k">self</span>.group = instance_group(ctx, l, &amp;<span class="k">self</span>.buffer.buf, &amp;<span class="k">self</span>.translations.buf);
    }

    <span class="c">/// Row \`i\` as uploaded.</span>
    <span class="k">pub</span> <span class="k">fn</span> row(&amp;<span class="k">self</span>, i: u32) -&gt; Option&lt;&amp;Instance&gt; {
        <span class="k">self</span>.rows.get(i <span class="k">as</span> usize)
    }</code></pre></div>
<p>Copy this part from the lesson folder to the path shown.</p>
<p><code>lessons/04a/src/engine/gpu/objects.rs</code> · copy the file, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Row count.</span>
    <span class="k">pub</span> <span class="k">fn</span> len(&amp;<span class="k">self</span>) -&gt; u32 {
        <span class="k">self</span>.rows.len() <span class="k">as</span> u32
    }

    <span class="c">/// World box of a row, None when it has none.</span>
    <span class="k">pub</span> <span class="k">fn</span> row_bounds(&amp;<span class="k">self</span>, row: u32) -&gt; Option&lt;AABB&gt; {
        <span class="k">let</span> b = *<span class="k">self</span>.world_bounds.get(row <span class="k">as</span> usize)?;
        b.is_valid().then_some(b)
    }

    <span class="c">/// Scene origin in f64.</span>
    <span class="k">pub</span> <span class="k">fn</span> anchor(&amp;<span class="k">self</span>) -&gt; [f64; <span class="s">3</span>] {
        <span class="k">match</span> &amp;<span class="k">self</span>.last_origin {
            Some(point) =&gt; [point[<span class="s">0</span>], point[<span class="s">1</span>], point[<span class="s">2</span>]],
            None =&gt; [<span class="s">0</span>.<span class="s">0</span>; <span class="s">3</span>],
        }
    }

    <span class="c">/// Scene origin in f32; zero before the first frame.</span>
    <span class="k">pub</span> <span class="k">fn</span> anchor_f32(&amp;<span class="k">self</span>) -&gt; [f32; <span class="s">3</span>] {
        <span class="k">match</span> &amp;<span class="k">self</span>.last_origin {
            Some(origin) =&gt; [origin[<span class="s">0</span>] <span class="k">as</span> f32, origin[<span class="s">1</span>] <span class="k">as</span> f32, origin[<span class="s">2</span>] <span class="k">as</span> f32],
            None =&gt; [<span class="s">0</span>.<span class="s">0</span>; <span class="s">3</span>],
        }
    }
}

#[cfg(test)]
<span class="k">mod</span> tests {
    <span class="k">use</span> super::*;
    <span class="k">use</span> session_rust::Xform;

    <span class="c">/// A translated local box lands at the translated world position.</span>
    #[test]
    <span class="k">fn</span> world_box_translates() {
        <span class="k">let</span> <span class="k">mut</span> r = ObjectRow::new(Xform::translation(<span class="s">10</span>.<span class="s">0</span>, <span class="s">20</span>.<span class="s">0</span>, <span class="s">30</span>.<span class="s">0</span>), <span class="s">0</span>);
        r.bounds = AABB::from_points(&amp;[Point::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), Point::new(<span class="s">1</span>.<span class="s">0</span>, <span class="s">2</span>.<span class="s">0</span>, <span class="s">3</span>.<span class="s">0</span>)], <span class="s">0</span>.<span class="s">0</span>);
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
}</code></pre></div>
<p>Run <code>cargo check</code> in <code>lessons/04a/</code>.</p>
<h2 id="step-12-srcshaderstrianglewgsl">Step 12 · src/shaders/triangle.wgsl<a class="anchor" href="#/course/04a-meshes#step-12-srcshaderstrianglewgsl" aria-label="Link to this section">#</a></h2>
<p>New file: the mesh shader, position and flat colour per face.</p>
<p><code>lessons/04a/src/shaders/triangle.wgsl</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// Red for faces seen from behind.</span>
<span class="k">const</span> BACKFACE_COLOR: <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt; = <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.80</span>, <span class="s">0.05</span>, <span class="s">0.05</span>);

<span class="c">// One mesh vertex from the vertex buffers.</span>
<span class="k">struct</span> VsIn {
    @location(<span class="s">0</span>) position: <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // object-space position</span>
    @location(<span class="s">1</span>) normal: <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // normal, or zero</span>
    @location(<span class="s">2</span>) color: <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // rgb</span>
    @location(<span class="s">3</span>) inst_id: <span class="k">u32</span>,<span class="c"> // object row</span>
}

<span class="c">// What the vertex shader hands the fragment shader.</span>
<span class="k">struct</span> VsOut {
    @builtin(position) pos: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // clip position</span>
    @location(<span class="s">0</span>) color: <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // rgb, yellow when selected</span>
    @location(<span class="s">1</span>) world_pos: <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // scene-space position</span>
    @location(<span class="s">2</span>) normal: <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // scene-space normal</span>
    @location(<span class="s">3</span>) print: <span class="k">f32</span>,<span class="c"> // 1 for a sheet fill</span>
    @location(<span class="s">4</span>) @interpolate(flat) inst_id: <span class="k">u32</span>,<span class="c"> // object row</span>
    @location(<span class="s">5</span>) @interpolate(flat) mirrored: <span class="k">u32</span>,<span class="c"> // 1 when the object matrix flips handedness</span>
}</code></pre></div>
<p><code>lessons/04a/src/shaders/triangle.wgsl</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// A vertex placed off screen, so nothing is drawn.</span>
<span class="k">fn</span> dead_vertex() -&gt; VsOut {
    <span class="k">var</span> dead: VsOut;<span class="c"> // all zero; only the position matters</span>
    dead.pos = <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(<span class="s">3.0</span>, <span class="s">3.0</span>, <span class="s">0.5</span>, <span class="s">1.0</span>);
    dead.color = <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>);
    dead.world_pos = <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>);
    dead.normal = <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>);
    dead.print = <span class="s">0.0</span>;
    dead.inst_id = <span class="s">0u</span>;
    dead.mirrored = <span class="s">0u</span>;
    <span class="k">return</span> dead;
}

@vertex
<span class="c">// Vertices from the vertex buffers.</span>
<span class="k">fn</span> vs_main(in: VsIn) -&gt; VsOut {
    <span class="k">let</span> inst = instances[in.inst_id];

    <span class="k">if</span> ((inst.flags &amp; FLAG_HIDDEN) != <span class="s">0u</span>) {
        <span class="k">return</span> dead_vertex();
    }

    <span class="k">let</span> world = place(in.inst_id, in.position);
    <span class="k">let</span> clip = mvp * <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(world, <span class="s">1.0</span>);
    <span class="k">var</span> o: VsOut;
    o.pos = clip;
    <span class="k">var</span> color = in.color.rgb * inst.color.rgb;

    <span class="k">if</span> ((inst.flags &amp; FLAG_SELECTED) != <span class="s">0u</span>) {
        color = SELECT_COLOR;
    }

    o.color = color;
    o.world_pos = world;
    o.normal = face_normal(inst.model, in.normal);
<span class="c">    // negative determinant: winding is flipped</span>
    o.mirrored = select(<span class="s">0u</span>, <span class="s">1u</span>, dot(inst.model[<span class="s">0</span>].xyz, cross(inst.model[<span class="s">1</span>].xyz, inst.model[<span class="s">2</span>].xyz)) &lt; <span class="s">0.0</span>);
    o.print = select(<span class="s">0.0</span>, <span class="s">1.0</span>, (inst.flags &amp; FLAG_PRINT) != <span class="s">0u</span>);
    o.inst_id = in.inst_id;
    <span class="k">return</span> o;
}</code></pre></div>
<p><code>lessons/04a/src/shaders/triangle.wgsl</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// Direction toward the camera; constant in ortho.</span>
<span class="k">fn</span> view_dir(world_pos: <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;) -&gt; <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt; {
    <span class="k">if</span> (line.ortho_h &gt; <span class="s">0.0</span>) {
        <span class="k">return</span> normalize(<span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(mvp[<span class="s">0</span>].z, mvp[<span class="s">1</span>].z, mvp[<span class="s">2</span>].z));
    }

    <span class="k">return</span> normalize(<span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(line.eye_x, line.eye_y, line.eye_z) - world_pos);
}

<span class="c">// Face color with lighting, opacity and back-face red.</span>
<span class="k">fn</span> shade(in: VsOut, raster_front: <span class="k">bool</span>) -&gt; <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt; {
    <span class="k">let</span> front = raster_front != (in.mirrored != <span class="s">0u</span>);
<span class="c">    // flat normal from screen derivatives, when the mesh has none</span>
    <span class="k">let</span> flat_n = cross(dpdy(in.world_pos), dpdx(in.world_pos));
    <span class="k">var</span> n = <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>, <span class="s">0.0</span>, <span class="s">1.0</span>);

    <span class="k">if</span> (dot(in.normal, in.normal) &gt; 1e-<span class="s">12</span>) {
        n = normalize(in.normal);
    } <span class="k">else</span> <span class="k">if</span> (dot(flat_n, flat_n) &gt; 1e-<span class="s">24</span>) {
        n = normalize(flat_n);
    }

<span class="c">    // headlight: the lamp rides the camera, a little above it</span>
    <span class="k">let</span> v = view_dir(in.world_pos);

<span class="c">    // turn the normal toward the eye</span>
    <span class="k">if</span> (dot(n, v) &lt; <span class="s">0.0</span>) {
        n = -n;
    }

    <span class="k">let</span> l = normalize(v + <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>, <span class="s">0.0</span>, <span class="s">0.35</span>));
    <span class="k">let</span> h = normalize(l + v);
<span class="c">    // wrapped diffuse: no hard terminator</span>
    <span class="k">let</span> wrap = clamp((dot(n, l) + <span class="s">0.5</span>) / <span class="s">1.5</span>, <span class="s">0.0</span>, <span class="s">1.0</span>);
<span class="c">    // soft highlight</span>
    <span class="k">let</span> spec = pow(max(dot(n, h), <span class="s">0.0</span>), <span class="s">32.0</span>) * <span class="s">0.15</span>;
    <span class="k">let</span> lit = min(<span class="s">0.40</span> + <span class="s">0.55</span> * wrap + spec, <span class="s">1.0</span>);

<span class="c">    // back faces red when asked; sheet fills never</span>
    <span class="k">let</span> backface = !front &amp;&amp; in.print &lt;= <span class="s">0.5</span> &amp;&amp; line.backface &gt; <span class="s">0.5</span>;
    <span class="k">let</span> base = select(in.color, BACKFACE_COLOR, backface);
    <span class="k">let</span> shaded = select(<span class="s">1.0</span>, lit, line.lit &gt; <span class="s">0.5</span> &amp;&amp; in.print &lt;= <span class="s">0.5</span>);
    <span class="k">return</span> <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(base * shaded, <span class="s">1.0</span>);
}</code></pre></div>
<p><code>lessons/04a/src/shaders/triangle.wgsl</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// The id pass: (object row + 1, 0).</span>
@fragment
<span class="k">fn</span> fs_id(in: VsOut) -&gt; @location(<span class="s">0</span>) <span class="k">vec2</span>&lt;<span class="k">u32</span>&gt; {
    <span class="k">return</span> <span class="k">vec2</span>&lt;<span class="k">u32</span>&gt;(in.inst_id + <span class="s">1u</span>, <span class="s">0u</span>);
}

@fragment
<span class="k">fn</span> fs_main(in: VsOut, @builtin(front_facing) front: <span class="k">bool</span>) -&gt; @location(<span class="s">0</span>) <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt; {
    <span class="k">return</span> shade(in, front);
}</code></pre></div>
<h2 id="step-13-srcenginegpuarenars">Step 13 · src/engine/gpu/arena.rs<a class="anchor" href="#/course/04a-meshes#step-13-srcenginegpuarenars" aria-label="Link to this section">#</a></h2>
<p>New file: the mesh arena, vertices and indices of every mesh.</p>
<p><code>lessons/04a/src/engine/gpu/arena.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> super::buffers::{GpuCtx, GrowBuf, INDICES, VERTS};
<span class="k">use</span> super::frame::Binds;
<span class="k">use</span> super::text_outline::{OutlineBuffers, OutlineTextLane};
<span class="k">use</span> super::upload::drop_rows;
<span class="k">use</span> <span class="k">crate</span>::engine::pipelines::{
    Layouts, PipelineDesc, Target, build, instance_id_layout, scene_module, vertex_layout,
};
<span class="k">use</span> session_rust::RenderVertex;
<span class="k">use</span> wgpu::PrimitiveTopology::TriangleList;

<span class="c">/// Shader sources the tests compare against the files.</span>
#[cfg(test)]
<span class="k">pub</span> <span class="k">const</span> SHADERS: &amp;[(&amp;str, &amp;str)] = &amp;[
    (&quot;<span class="s">triangle.wgsl</span>&quot;, include_str!(&quot;<span class="s">../../shaders/triangle.wgsl</span>&quot;)),
    (
        &quot;<span class="s">text_outline.wgsl</span>&quot;,
        include_str!(&quot;<span class="s">../../shaders/text_outline.wgsl</span>&quot;),
    ),
];

<span class="c">/// Mesh rows of one upload, ready for the GPU.</span>
#[derive(Default)]
<span class="k">pub</span> <span class="k">struct</span> ArenaRows {
    <span class="k">pub</span> verts: Vec&lt;RenderVertex&gt;, <span class="c">// one vertex per row</span>
    <span class="k">pub</span> vids: Vec&lt;u32&gt;, <span class="c">// object row of each vertex</span>
    <span class="k">pub</span> idx: Vec&lt;u32&gt;, <span class="c">// triangle indices of solid faces</span>
    <span class="k">pub</span> idx_print: Vec&lt;u32&gt;, <span class="c">// triangle indices of sheet fills</span>
    <span class="k">pub</span> idx_text: Vec&lt;u32&gt;, <span class="c">// triangle indices of sheet lettering</span>
}

<span class="k">impl</span> ArenaRows {
    <span class="c">/// Empty every table and free its memory.</span>
    <span class="k">pub</span> <span class="k">fn</span> drop_rows(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        drop_rows(&amp;<span class="k">mut</span> <span class="k">self</span>.verts);
        drop_rows(&amp;<span class="k">mut</span> <span class="k">self</span>.vids);
        drop_rows(&amp;<span class="k">mut</span> <span class="k">self</span>.idx);
        drop_rows(&amp;<span class="k">mut</span> <span class="k">self</span>.idx_print);
        drop_rows(&amp;<span class="k">mut</span> <span class="k">self</span>.idx_text);
    }
}

<span class="c">/// Pipelines that draw solid faces as masks.</span>
<span class="k">struct</span> ArenaPipelines {
    faces: wgpu::RenderPipeline,
    id_faces: wgpu::RenderPipeline,
}

<span class="c">/// All mesh geometry on the GPU, in five growing buffers.</span>
<span class="k">pub</span> <span class="k">struct</span> ArenaLane {
    verts: GrowBuf, <span class="c">// vertex buffer</span>
    vids: GrowBuf, <span class="c">// object row per vertex</span>
    faces: GrowBuf, <span class="c">// solid face indices</span>
    print: GrowBuf, <span class="c">// sheet fill indices</span>
    text: GrowBuf, <span class="c">// sheet lettering indices</span>
    shader: wgpu::ShaderModule, <span class="c">// triangle shader</span>
    pipes: ArenaPipelines, <span class="c">// mask pipelines</span>
    outline_text: OutlineTextLane, <span class="c">// draws sheet fills and lettering</span>
}</code></pre></div>
<p><code>lessons/04a/src/engine/gpu/arena.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">impl</span> ArenaLane {
    <span class="c">/// Bytes reserved on the GPU by this lane.</span>
    <span class="k">pub</span> <span class="k">fn</span> allocated_bytes(&amp;<span class="k">self</span>) -&gt; u64 {
        <span class="k">self</span>.verts.buf.size()
            + <span class="k">self</span>.vids.buf.size()
            + <span class="k">self</span>.faces.buf.size()
            + <span class="k">self</span>.print.buf.size()
            + <span class="k">self</span>.text.buf.size()
    }

    <span class="c">/// Create the lane with empty buffers.</span>
    <span class="k">pub</span> <span class="k">fn</span> new(ctx: &amp;GpuCtx, l: &amp;Layouts, target: Target) -&gt; <span class="k">Self</span> {
        <span class="k">let</span> shader = scene_module(
            &amp;ctx.device,
            &quot;<span class="s">triangle.shader</span>&quot;,
            include_str!(&quot;<span class="s">../../shaders/triangle.wgsl</span>&quot;),
        );
        <span class="k">let</span> pipes = build_pipelines(ctx, l, &amp;shader, target);

        <span class="k">Self</span> {
            verts: GrowBuf::new(
                ctx,
                &quot;<span class="s">arena.vbo</span>&quot;,
                std::mem::size_of::&lt;RenderVertex&gt;() <span class="k">as</span> u64,
                VERTS,
            ),
            vids: GrowBuf::new(ctx, &quot;<span class="s">arena.vids</span>&quot;, <span class="s">4</span>, VERTS),
            faces: GrowBuf::new(ctx, &quot;<span class="s">arena.ibo</span>&quot;, <span class="s">4</span>, INDICES),
            print: GrowBuf::new(ctx, &quot;<span class="s">arena.ibo.print</span>&quot;, <span class="s">4</span>, INDICES),
            text: GrowBuf::new(ctx, &quot;<span class="s">arena.ibo.text</span>&quot;, <span class="s">4</span>, INDICES),
            shader,
            pipes,
            outline_text: OutlineTextLane::new(ctx, l, target),
        }
    }

    <span class="c">/// Rebuild the pipelines for a new MSAA sample count.</span>
    <span class="k">pub</span> <span class="k">fn</span> retarget(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, l: &amp;Layouts, target: Target) {
        <span class="k">self</span>.pipes = build_pipelines(ctx, l, &amp;<span class="k">self</span>.shader, target);
        <span class="k">self</span>.outline_text.retarget(ctx, l, target);
    }

    <span class="c">/// Append one upload's rows to every buffer.</span>
    <span class="k">pub</span> <span class="k">fn</span> append(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, up: &amp;ArenaRows) {
        <span class="k">self</span>.verts.append(ctx, &amp;up.verts);
        <span class="k">self</span>.vids.append(ctx, &amp;up.vids);
        <span class="k">self</span>.faces.append(ctx, &amp;up.idx);
        <span class="k">self</span>.print.append(ctx, &amp;up.idx_print);
        <span class="k">self</span>.text.append(ctx, &amp;up.idx_text);
    }</code></pre></div>
<p><code>lessons/04a/src/engine/gpu/arena.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Draw the solid faces.</span>
    <span class="k">pub</span> <span class="k">fn</span> draw_faces(&amp;<span class="k">self</span>, pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;, b: &amp;Binds) -&gt; u32 {
        <span class="k">self</span>.draw_run(pass, b, &amp;<span class="k">self</span>.pipes.faces, &amp;<span class="k">self</span>.faces)
    }

    <span class="c">/// Draw sheet fills.</span>
    <span class="k">pub</span> <span class="k">fn</span> draw_print(&amp;<span class="k">self</span>, pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;, b: &amp;Binds) -&gt; u32 {
        <span class="k">self</span>.outline_text
            .draw(pass, b, &amp;<span class="k">self</span>.outline_buffers(&amp;<span class="k">self</span>.print))
    }

    <span class="c">/// Draw sheet lettering.</span>
    <span class="k">pub</span> <span class="k">fn</span> draw_text(&amp;<span class="k">self</span>, pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;, b: &amp;Binds) -&gt; u32 {
        <span class="k">self</span>.outline_text
            .draw(pass, b, &amp;<span class="k">self</span>.outline_buffers(&amp;<span class="k">self</span>.text))
    }

    <span class="c">/// Draw object ids of faces and sheet fills.</span>
    <span class="k">pub</span> <span class="k">fn</span> draw_face_ids(&amp;<span class="k">self</span>, pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;, b: &amp;Binds) -&gt; u32 {
        <span class="k">self</span>.draw_run(pass, b, &amp;<span class="k">self</span>.pipes.id_faces, &amp;<span class="k">self</span>.faces)
            + <span class="k">self</span>
                .outline_text
                .draw_ids(pass, b, &amp;<span class="k">self</span>.outline_buffers(&amp;<span class="k">self</span>.print))
    }

    <span class="c">/// Draw object ids of sheet lettering.</span>
    <span class="k">pub</span> <span class="k">fn</span> draw_text_ids(&amp;<span class="k">self</span>, pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;, b: &amp;Binds) -&gt; u32 {
        <span class="k">self</span>.outline_text
            .draw_ids(pass, b, &amp;<span class="k">self</span>.outline_buffers(&amp;<span class="k">self</span>.text))
    }

    <span class="c">/// Bundle the buffers one outline draw needs.</span>
    <span class="k">fn</span> outline_buffers&lt;'a&gt;(&amp;'a <span class="k">self</span>, indices: &amp;'a GrowBuf) -&gt; OutlineBuffers&lt;'a&gt; {
        OutlineBuffers {
            vertices: &amp;<span class="k">self</span>.verts,
            objects: &amp;<span class="k">self</span>.vids,
            indices,
        }
    }

    <span class="c">/// Index count of sheet fills and lettering together.</span>
    <span class="k">pub</span> <span class="k">fn</span> sheet_count(&amp;<span class="k">self</span>) -&gt; u32 {
        <span class="k">self</span>.text.len().saturating_add(<span class="k">self</span>.print.len())
    }</code></pre></div>
<p><code>lessons/04a/src/engine/gpu/arena.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Draw one index buffer with \`pipeline\`; returns the draw count.</span>
    <span class="k">fn</span> draw_run(
        &amp;<span class="k">self</span>,
        pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;,
        b: &amp;Binds,
        pipeline: &amp;wgpu::RenderPipeline,
        run: &amp;GrowBuf,
    ) -&gt; u32 {
        <span class="k">if</span> run.is_empty() {
            <span class="k">return</span> <span class="s">0</span>;
        }

        pass.set_pipeline(pipeline);
        b.set(pass);
        pass.set_vertex_buffer(<span class="s">0</span>, <span class="k">self</span>.verts.buf.slice(..));
        pass.set_vertex_buffer(<span class="s">1</span>, <span class="k">self</span>.vids.buf.slice(..));
        pass.set_index_buffer(run.buf.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(<span class="s">0</span>..run.len(), <span class="s">0</span>, <span class="s">0</span>..<span class="s">1</span>);
        <span class="s">1</span>
    }

    <span class="c">/// Forget every row; capacity stays.</span>
    <span class="k">pub</span> <span class="k">fn</span> reset(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">self</span>.verts.reset();
        <span class="k">self</span>.vids.reset();
        <span class="k">self</span>.faces.reset();
        <span class="k">self</span>.print.reset();
        <span class="k">self</span>.text.reset();
    }

    <span class="c">/// Free every buffer.</span>
    <span class="k">pub</span> <span class="k">fn</span> release(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx) {
        <span class="k">self</span>.verts.release(ctx);
        <span class="k">self</span>.vids.release(ctx);
        <span class="k">self</span>.faces.release(ctx);
        <span class="k">self</span>.print.release(ctx);
        <span class="k">self</span>.text.release(ctx);
    }

    <span class="c">/// Vertices on the GPU.</span>
    <span class="k">pub</span> <span class="k">fn</span> vert_count(&amp;<span class="k">self</span>) -&gt; u32 {
        <span class="k">self</span>.verts.len()
    }

    <span class="c">/// Index count of the solid faces.</span>
    <span class="k">pub</span> <span class="k">fn</span> face_count(&amp;<span class="k">self</span>) -&gt; u32 {
        <span class="k">self</span>.faces.len()
    }
}

<span class="c">/// Build the three mask pipelines for \`target\`.</span>
<span class="k">fn</span> build_pipelines(
    ctx: &amp;GpuCtx,
    l: &amp;Layouts,
    shader: &amp;wgpu::ShaderModule,
    target: Target,
) -&gt; ArenaPipelines {
    <span class="c">// bind groups every mask pipeline uses</span>
    <span class="k">let</span> groups = [&amp;l.mvp, &amp;l.line, &amp;l.instance];
    <span class="c">// vertex buffer 0: vertices, 1: object rows</span>
    <span class="k">let</span> buffers = [vertex_layout(), instance_id_layout()];
    <span class="k">let</span> base = PipelineDesc::new(shader, &amp;groups, &amp;buffers, TriangleList);
    <span class="k">let</span> dev = &amp;ctx.device;

    ArenaPipelines {
        faces: build(dev, target, &amp;base.with(&quot;<span class="s">triangle</span>&quot;, &quot;<span class="s">fs_main</span>&quot;)),
        id_faces: build(dev, Target::ID, &amp;base.with(&quot;<span class="s">triangle.id</span>&quot;, &quot;<span class="s">fs_id</span>&quot;)),
    }
}</code></pre></div>
<h2 id="step-14-srcshaderstext_outlinewgsl">Step 14 · src/shaders/text_outline.wgsl<a class="anchor" href="#/course/04a-meshes#step-14-srcshaderstext_outlinewgsl" aria-label="Link to this section">#</a></h2>
<p>New file: the shader for sheet fills and lettering: placed, flat-coloured, yellow when selected.
Copy this file from the lesson folder to the path shown.</p>
<p><code>lessons/04a/src/shaders/text_outline.wgsl</code> · copy the file, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// One mesh vertex, as the arena stores it.</span>
<span class="k">struct</span> Vertex {
    @location(<span class="s">0</span>) position: <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;,
    @location(<span class="s">2</span>) color: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // location 1, the normal, is not read: text is unlit</span>
    @location(<span class="s">3</span>) object: <span class="k">u32</span>,<span class="c"> // which row of instances[] this vertex belongs to</span>
}

<span class="c">// What the vertex shader hands the fragment shader.</span>
<span class="k">struct</span> Fragment {
    @builtin(position) position: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,
    @location(<span class="s">0</span>) color: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,
    @location(<span class="s">1</span>) @interpolate(flat) object: <span class="k">u32</span>,<span class="c"> // flat: an integer cannot be blended, so one vertex's value is used</span>
}

<span class="c">// Place the vertex; color is flat, yellow when selected.</span>
@vertex
<span class="k">fn</span> vs_main(vertex: Vertex) -&gt; Fragment {
    <span class="k">let</span> instance = instances[vertex.object];
    <span class="k">var</span> out: Fragment;
    out.object = vertex.object;

<span class="c">    // hidden: x = 3 lies outside clip space, so nothing is drawn</span>
    <span class="k">if</span> (instance.flags &amp; FLAG_HIDDEN) != <span class="s">0u</span> {
        out.position = <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(<span class="s">3.0</span>, <span class="s">3.0</span>, <span class="s">0.5</span>, <span class="s">1.0</span>);
        out.color = <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>);
        <span class="k">return</span> out;
    }

    <span class="k">let</span> world = place(vertex.object, vertex.position);
    out.position = mvp * <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(world, <span class="s">1.0</span>);
    <span class="k">let</span> selected = (instance.flags &amp; FLAG_SELECTED) != <span class="s">0u</span>;
<span class="c">    // select(a, b, c) = if c then b else a</span>
    out.color = <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(select(vertex.color.rgb * instance.color.rgb,
        SELECT_COLOR, selected), vertex.color.a * instance.color.a);
    <span class="k">return</span> out;
}

@fragment
<span class="k">fn</span> fs_main(fragment: Fragment) -&gt; @location(<span class="s">0</span>) <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt; {
    <span class="k">return</span> fragment.color;
}

@fragment
<span class="c">// Picking: row + 1 into an integer target; 0 means nothing was hit.</span>
<span class="k">fn</span> fs_id(fragment: Fragment) -&gt; @location(<span class="s">0</span>) <span class="k">vec2</span>&lt;<span class="k">u32</span>&gt; {
    <span class="k">return</span> <span class="k">vec2</span>&lt;<span class="k">u32</span>&gt;(fragment.object + <span class="s">1u</span>, <span class="s">0u</span>);
}</code></pre></div>
<h2 id="step-15-srcenginegputext_outliners">Step 15 · src/engine/gpu/text_outline.rs<a class="anchor" href="#/course/04a-meshes#step-15-srcenginegputext_outliners" aria-label="Link to this section">#</a></h2>
<p>New file: the lane that draws sheet fills and lettering, in colour and as pick ids.</p>
<p><code>lessons/04a/src/engine/gpu/text_outline.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> super::buffers::{GpuCtx, GrowBuf};
<span class="k">use</span> super::frame::Binds;
<span class="k">use</span> <span class="k">crate</span>::engine::pipelines::{
    ColorWrite, DepthMode, Layouts, PipelineDesc, Target, build, instance_id_layout, scene_module,
    vertex_layout,
};

<span class="c">/// The three mesh buffers one sheet draw reads, borrowed from the arena.</span>
<span class="k">pub</span> <span class="k">struct</span> OutlineBuffers&lt;'a&gt; {
    <span class="k">pub</span> vertices: &amp;'a GrowBuf,
    <span class="k">pub</span> objects: &amp;'a GrowBuf, <span class="c">// object row per vertex</span>
    <span class="k">pub</span> indices: &amp;'a GrowBuf,
}

<span class="c">/// Lane = one kind of geometry with its own shader and pipelines; this one draws sheet fills and lettering, unlit.</span>
<span class="k">pub</span> <span class="k">struct</span> OutlineTextLane {
    shader: wgpu::ShaderModule, <span class="c">// kept, so retarget rebuilds without compiling again</span>
    color: wgpu::RenderPipeline,
    id: wgpu::RenderPipeline, <span class="c">// writes object ids for picking</span>
}

<span class="k">impl</span> OutlineTextLane {
    <span class="k">pub</span> <span class="k">fn</span> new(ctx: &amp;GpuCtx, layouts: &amp;Layouts, target: Target) -&gt; <span class="k">Self</span> {
        <span class="k">let</span> shader = scene_module(
            &amp;ctx.device,
            &quot;<span class="s">text-outline.shader</span>&quot;,
            include_str!(&quot;<span class="s">../../shaders/text_outline.wgsl</span>&quot;),
        );
        <span class="k">let</span> (color, id) = pipelines(ctx, layouts, &amp;shader, target);
        <span class="k">Self</span> { shader, color, id }
    }

    <span class="c">/// A pipeline is built for one MSAA sample count, so a new count needs new pipelines.</span>
    <span class="k">pub</span> <span class="k">fn</span> retarget(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, layouts: &amp;Layouts, target: Target) {
        (<span class="k">self</span>.color, <span class="k">self</span>.id) = pipelines(ctx, layouts, &amp;<span class="k">self</span>.shader, target);
    }

    <span class="k">pub</span> <span class="k">fn</span> draw(
        &amp;<span class="k">self</span>,
        pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;,
        binds: &amp;Binds,
        buffers: &amp;OutlineBuffers&lt;'_&gt;,
    ) -&gt; u32 {
        pass.set_pipeline(&amp;<span class="k">self</span>.color);
        draw(pass, binds, buffers)
    }</code></pre></div>
<p><code>lessons/04a/src/engine/gpu/text_outline.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> <span class="k">fn</span> draw_ids(
        &amp;<span class="k">self</span>,
        pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;,
        binds: &amp;Binds,
        buffers: &amp;OutlineBuffers&lt;'_&gt;,
    ) -&gt; u32 {
        pass.set_pipeline(&amp;<span class="k">self</span>.id);
        draw(pass, binds, buffers)
    }
}</code></pre></div>
<p><code>lessons/04a/src/engine/gpu/text_outline.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Returns the number of draw calls, for the frame statistics.</span>
<span class="k">fn</span> draw(pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;, binds: &amp;Binds, buffers: &amp;OutlineBuffers&lt;'_&gt;) -&gt; u32 {
    <span class="k">if</span> buffers.indices.is_empty() {
        <span class="k">return</span> <span class="s">0</span>;
    }

    binds.set(pass);
    pass.set_vertex_buffer(<span class="s">0</span>, buffers.vertices.buf.slice(..));
    pass.set_vertex_buffer(<span class="s">1</span>, buffers.objects.buf.slice(..)); <span class="c">// slot 1: each vertex's object row</span>
    pass.set_index_buffer(buffers.indices.buf.slice(..), wgpu::IndexFormat::Uint32);
    pass.draw_indexed(<span class="s">0</span>..buffers.indices.len(), <span class="s">0</span>, <span class="s">0</span>..<span class="s">1</span>); <span class="c">// all indices, base vertex 0, one instance</span>
    <span class="s">1</span>
}

<span class="c">/// The two pipelines differ only in fragment entry point and target.</span>
<span class="k">fn</span> pipelines(
    ctx: &amp;GpuCtx,
    layouts: &amp;Layouts,
    shader: &amp;wgpu::ShaderModule,
    target: Target,
) -&gt; (wgpu::RenderPipeline, wgpu::RenderPipeline) {
    <span class="k">let</span> groups = [&amp;layouts.mvp, &amp;layouts.line, &amp;layouts.instance];
    <span class="k">let</span> vertices = [vertex_layout(), instance_id_layout()];
    <span class="k">let</span> base = PipelineDesc::new(
        shader,
        &amp;groups,
        &amp;vertices,
        wgpu::PrimitiveTopology::TriangleList,
    );
    <span class="k">let</span> color = build(
        &amp;ctx.device,
        target,
        &amp;base
            .with(&quot;<span class="s">text-outline</span>&quot;, &quot;<span class="s">fs_main</span>&quot;)
            .color(ColorWrite::Blended)
            .depth(DepthMode::ReadOnly),
    );
    <span class="k">let</span> id = build(
        &amp;ctx.device,
        Target::ID,
        &amp;base
            .with(&quot;<span class="s">text-outline.id</span>&quot;, &quot;<span class="s">fs_id</span>&quot;)
            .depth(DepthMode::ReadOnlyEqual),
    );
    (color, id)
}</code></pre></div>
<h2 id="step-16-srcenginegpuuploadrs">Step 16 · src/engine/gpu/upload.rs<a class="anchor" href="#/course/04a-meshes#step-16-srcenginegpuuploadrs" aria-label="Link to this section">#</a></h2>
<p>New file: everything one scene uploads, collected before the GPU sees it.</p>
<p><code>lessons/04a/src/engine/gpu/upload.rs</code> · 37 lines · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> super::arena::ArenaRows;
<span class="k">use</span> super::objects::ObjectRows;
<span class="k">use</span> session_rust::AABB;

<span class="c">/// Every lane's rows for one file, ready to upload.</span>
<span class="k">pub</span> <span class="k">struct</span> Upload {
    <span class="k">pub</span> obj: ObjectRows, <span class="c">// object rows</span>
    <span class="k">pub</span> arena: ArenaRows, <span class="c">// meshes</span>
    <span class="k">pub</span> bounds: AABB, <span class="c">// world box of this upload</span>
}

<span class="k">impl</span> Default <span class="k">for</span> Upload {
    <span class="c">/// Every lane empty.</span>
    <span class="k">fn</span> default() -&gt; <span class="k">Self</span> {
        <span class="k">Self</span> {
            obj: ObjectRows::default(),
            arena: ArenaRows::default(),
            bounds: AABB::empty(),
        }
    }
}

<span class="k">impl</span> Upload {
    <span class="c">/// Free the rows once the GPU holds them.</span>
    <span class="k">pub</span> <span class="k">fn</span> drop_uploaded(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        drop_rows(&amp;<span class="k">mut</span> <span class="k">self</span>.obj.rows);
        <span class="k">self</span>.arena.drop_rows();
        <span class="k">self</span>.bounds = AABB::empty();
    }
}

<span class="c">/// Empty a list and free its memory.</span>
<span class="k">pub</span> <span class="k">fn</span> drop_rows&lt;T&gt;(v: &amp;<span class="k">mut</span> Vec&lt;T&gt;) {
    v.clear();
    v.shrink_to_fit();
}</code></pre></div>
<h2 id="step-17-srcfixturers">Step 17 · src/fixture.rs<a class="anchor" href="#/course/04a-meshes#step-17-srcfixturers" aria-label="Link to this section">#</a></h2>
<p>New file: a small test scene built in code.
Copy this file from the lesson folder to the path shown.</p>
<p><code>lessons/04a/src/fixture.rs</code> · 35 lines · copy the file, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> <span class="k">crate</span>::engine::gpu::{Instance, ObjectRow, Upload};
<span class="k">use</span> session_rust::AABB;
<span class="k">use</span> session_rust::Point;
<span class="k">use</span> session_rust::{RenderVertex, Xform};

<span class="c">/// Small test scenes built in code, no loading.</span>
<span class="k">pub</span> <span class="k">fn</span> scene() -&gt; Upload {
    <span class="k">let</span> <span class="k">mut</span> upload = Upload::default();
    upload.bounds = AABB::from_points(
        &amp;[Point::new(-<span class="s">2</span>.<span class="s">0</span>, -<span class="s">1</span>.<span class="s">3</span>, -<span class="s">0</span>.<span class="s">1</span>), Point::new(<span class="s">2</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">3</span>, <span class="s">0</span>.<span class="s">1</span>)],
        <span class="s">0</span>.<span class="s">0</span>,
    );

    <span class="k">for</span> _ <span class="k">in</span> <span class="s">0</span>..<span class="s">1</span> {
        upload
            .obj
            .rows
            .push(ObjectRow::new(Xform::identity(), Instance::FLAG_OPEN));
    }

    upload.obj.rows[<span class="s">0</span>].faces = <span class="s">true</span>;
    upload.obj.rows[<span class="s">0</span>].bounds = upload.bounds;

    <span class="k">for</span> position <span class="k">in</span> [[-<span class="s">1</span>.<span class="s">7</span>, <span class="s">0</span>.<span class="s">1</span>, <span class="s">0</span>.<span class="s">0</span>], [-<span class="s">0</span>.<span class="s">3</span>, <span class="s">0</span>.<span class="s">1</span>, <span class="s">0</span>.<span class="s">0</span>], [-<span class="s">1</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">1</span>, <span class="s">0</span>.<span class="s">0</span>]] {
        upload.arena.verts.push(RenderVertex {
            position,
            normal: [<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>],
            color: [<span class="s">0</span>.<span class="s">2</span>, <span class="s">0</span>.<span class="s">7</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>],
        });
        upload.arena.vids.push(<span class="s">0</span>);
    }

    upload.arena.idx.extend([<span class="s">0</span>, <span class="s">1</span>, <span class="s">2</span>]);
    upload
}</code></pre></div>
<h2 id="step-18-srcenginegpumodrs">Step 18 · src/engine/gpu/mod.rs<a class="anchor" href="#/course/04a-meshes#step-18-srcenginegpumodrs" aria-label="Link to this section">#</a></h2>
<p>New file: the GPU owner, one field per drawing lane.</p>
<p><code>lessons/04a/src/engine/gpu/mod.rs</code> · type this, replace the whole file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> session_rust::AABB;
<span class="k">pub</span> <span class="k">mod</span> arena;
<span class="k">pub</span> <span class="k">mod</span> buffers;
<span class="k">pub</span> <span class="k">mod</span> frame;
<span class="k">pub</span> <span class="k">mod</span> instance;
<span class="k">pub</span> <span class="k">mod</span> objects;
<span class="k">pub</span> <span class="k">mod</span> targets;
<span class="k">pub</span> <span class="k">mod</span> text_outline;
<span class="k">pub</span> <span class="k">mod</span> upload;
<span class="k">pub</span> <span class="k">mod</span> view;
<span class="k">use</span> <span class="k">crate</span>::engine::pipelines::{Layouts, Target};
<span class="k">use</span> buffers::GpuCtx;
<span class="k">pub</span> <span class="k">use</span> frame::FrameInput;
<span class="k">pub</span> <span class="k">use</span> instance::Instance;
<span class="k">use</span> objects::InkScene;
<span class="k">pub</span> <span class="k">use</span> objects::{ObjectRow, Rebase};
<span class="k">pub</span> <span class="k">use</span> upload::Upload;

<span class="c">/// Everything on the GPU: the device, the frame and one field per lane.</span>
<span class="k">pub</span> <span class="k">struct</span> Gpu {
    <span class="k">pub</span> surface: wgpu::Surface&lt;'static&gt;,
    <span class="k">pub</span> ctx: GpuCtx, <span class="c">// device and queue</span>
    <span class="k">pub</span> config: wgpu::SurfaceConfiguration, <span class="c">// canvas size and format</span>
    <span class="k">pub</span> layouts: Layouts, <span class="c">// shared bind group layouts</span>
    <span class="k">pub</span> frame: frame::FrameUniforms,
    <span class="k">pub</span> targets: targets::Targets,
    <span class="k">pub</span> view: view::View,
    <span class="k">pub</span> objects: objects::InstanceTable,
    <span class="k">pub</span> arena: arena::ArenaLane,
    <span class="k">pub</span> bounds: AABB, <span class="c">// world box of everything uploaded</span>
    <span class="k">pub</span> logical_size: [f64; <span class="s">2</span>], <span class="c">// canvas size in CSS pixels</span>
    <span class="k">pub</span> device_type: wgpu::DeviceType,
}

<span class="k">impl</span> Gpu {</code></pre></div>
<p><code>lessons/04a/src/engine/gpu/mod.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Open WebGPU on the canvas.</span>
    #[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
    <span class="k">pub</span> <span class="k">async</span> <span class="k">fn</span> new(canvas: web_sys::HtmlCanvasElement) -&gt; anyhow::Result&lt;<span class="k">Self</span>&gt; {
        <span class="k">let</span> instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::BROWSER_WEBGPU,
            flags: Default::default(),
            memory_budget_thresholds: Default::default(),
            backend_options: Default::default(),
            display: None,
        });
        <span class="k">let</span> surface = instance.create_surface(wgpu::SurfaceTarget::Canvas(canvas))?;
        <span class="k">let</span> adapter = instance
            .request_adapter(&amp;wgpu::RequestAdapterOptions {
                compatible_surface: Some(&amp;surface),
                ..Default::default()
            })
            .<span class="k">await</span>?;
        <span class="k">let</span> device_type = adapter.get_info().device_type;
        <span class="k">let</span> (device, queue) = adapter
            .request_device(&amp;wgpu::DeviceDescriptor::default())
            .<span class="k">await</span>?;
        device.on_uncaptured_error(std::sync::Arc::new(gpu_error));
        <span class="k">let</span> caps = surface.get_capabilities(&amp;adapter);
        <span class="k">let</span> config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: caps.formats[<span class="s">0</span>],
            width: <span class="s">1</span>,
            height: <span class="s">1</span>,
            present_mode: caps.present_modes[<span class="s">0</span>],
            alpha_mode: caps.alpha_modes[<span class="s">0</span>],
            view_formats: vec![],
            desired_maximum_frame_latency: <span class="s">2</span>,
        };
        <span class="k">let</span> ctx = GpuCtx { device, queue };
        <span class="c">// start without MSAA; retarget flips it later</span>
        <span class="k">let</span> target = Target {
            format: config.format,
            samples: <span class="s">1</span>,
        };
        <span class="k">let</span> layouts = Layouts::new(&amp;ctx.device);
        <span class="k">let</span> frame = frame::FrameUniforms::new(&amp;ctx, &amp;layouts, (<span class="s">1</span>, <span class="s">1</span>));
        <span class="k">let</span> targets = targets::Targets::new(&amp;ctx, (<span class="s">1</span>, <span class="s">1</span>), config.format, <span class="s">1</span>);
        <span class="k">let</span> objects = objects::InstanceTable::new(&amp;ctx, &amp;layouts, &amp;InkScene { targets: &amp;targets });
        <span class="k">let</span> arena = arena::ArenaLane::new(&amp;ctx, &amp;layouts, target);
        Ok(<span class="k">Self</span> {
            surface,
            ctx,
            config,
            layouts,
            frame,
            targets,
            view: view::View::from_env(),
            objects,
            arena,
            bounds: AABB::empty(),
            logical_size: [<span class="s">1</span>.<span class="s">0</span>; <span class="s">2</span>],
            device_type,
        })
    }

    <span class="c">/// Append one upload to every lane.</span>
    <span class="k">pub</span> <span class="k">fn</span> set_scene(&amp;<span class="k">mut</span> <span class="k">self</span>, up: &amp;Upload) {
        <span class="k">self</span>.objects.append(&amp;<span class="k">self</span>.ctx, &amp;<span class="k">self</span>.layouts, &amp;up.obj);
        <span class="k">self</span>.arena.append(&amp;<span class="k">self</span>.ctx, &amp;up.arena);
        <span class="k">self</span>.bounds.union_with(&amp;up.bounds);
    }</code></pre></div>
<p><code>lessons/04a/src/engine/gpu/mod.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Resize the canvas and every texture that follows it.</span>
    <span class="k">pub</span> <span class="k">fn</span> resize(&amp;<span class="k">mut</span> <span class="k">self</span>, width: u32, height: u32) {
        <span class="k">self</span>.config.width = width.max(<span class="s">1</span>);
        <span class="k">self</span>.config.height = height.max(<span class="s">1</span>);
        <span class="k">self</span>.surface.configure(&amp;<span class="k">self</span>.ctx.device, &amp;<span class="k">self</span>.config);
        <span class="k">self</span>.targets = targets::Targets::new(
            &amp;<span class="k">self</span>.ctx,
            (<span class="k">self</span>.config.width, <span class="k">self</span>.config.height),
            <span class="k">self</span>.config.format,
            <span class="s">1</span>,
        );
        <span class="k">self</span>.objects.rebind_ink(
            &amp;<span class="k">self</span>.ctx,
            &amp;<span class="k">self</span>.layouts,
            &amp;InkScene {
                targets: &amp;<span class="k">self</span>.targets,
            },
        );
    }

    <span class="c">/// Positions as f32 offsets from one f64 anchor.</span>
    <span class="k">pub</span> <span class="k">fn</span> rebase_anchor(
        &amp;<span class="k">mut</span> <span class="k">self</span>,
        origin: &amp;session_rust::Point,
        distance: f64,
        now: f64,
    ) -&gt; Rebase {
        <span class="k">self</span>.objects.rebase_anchor(&amp;<span class="k">self</span>.ctx, origin, distance, now)
    }

    <span class="c">/// Upload the camera and pen scale for this frame.</span>
    <span class="k">pub</span> <span class="k">fn</span> write_frame_uniforms(&amp;<span class="k">mut</span> <span class="k">self</span>, input: &amp;FrameInput) {
        <span class="k">self</span>.frame.write(
            &amp;<span class="k">self</span>.ctx,
            input,
            &amp;frame::FrameCx {
                view: &amp;<span class="k">self</span>.view,
                anchor: <span class="k">self</span>.objects.anchor_f32(),
                size: (<span class="k">self</span>.config.width, <span class="k">self</span>.config.height),
                pixel_scale: <span class="k">self</span>.config.width <span class="k">as</span> f32 / <span class="k">self</span>.logical_size[<span class="s">0</span>] <span class="k">as</span> f32,
            },
        );
        <span class="k">self</span>.objects
            .update_inside(&amp;<span class="k">self</span>.ctx, <span class="k">self</span>.frame.eye, &amp;<span class="k">self</span>.bounds);
    }</code></pre></div>
<p><code>lessons/04a/src/engine/gpu/mod.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Faces first, then ink over their depth.</span>
    <span class="k">pub</span> <span class="k">fn</span> render(&amp;<span class="k">mut</span> <span class="k">self</span>, input: &amp;FrameInput) -&gt; anyhow::Result&lt;()&gt; {
        <span class="k">self</span>.write_frame_uniforms(input);
        <span class="k">let</span> output = <span class="k">match</span> <span class="k">self</span>.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(output)
            | wgpu::CurrentSurfaceTexture::Suboptimal(output) =&gt; output,
            error =&gt; anyhow::bail!(&quot;<span class="s">surface unavailable: </span>{<span class="s">error:?</span>}&quot;),
        };
        <span class="k">let</span> target = output.texture.create_view(&amp;Default::default());
        <span class="k">let</span> <span class="k">mut</span> encoder = <span class="k">self</span>.ctx.device.create_command_encoder(&amp;Default::default());
        <span class="k">let</span> basic = frame::Binds {
            mvp: &amp;<span class="k">self</span>.frame.mvp_group,
            line: &amp;<span class="k">self</span>.frame.line_group,
            instances: &amp;<span class="k">self</span>.objects.group,
        };
        {
            <span class="k">let</span> <span class="k">mut</span> pass = <span class="k">self</span>.targets.begin_faces(&amp;<span class="k">mut</span> encoder, &amp;target, input.clear);
            <span class="k">self</span>.arena.draw_faces(&amp;<span class="k">mut</span> pass, &amp;basic);
        }
        {
            <span class="k">let</span> <span class="k">mut</span> pass = <span class="k">self</span>.targets.begin_ink(&amp;<span class="k">mut</span> encoder, &amp;target);
            <span class="k">self</span>.arena.draw_print(&amp;<span class="k">mut</span> pass, &amp;basic);
            <span class="k">self</span>.arena.draw_text(&amp;<span class="k">mut</span> pass, &amp;basic);
        }
        <span class="k">self</span>.ctx.queue.submit([encoder.finish()]);
        output.present();
        Ok(())
    }
}

<span class="c">/// Make validation errors fail the checkpoint visibly.</span>
<span class="k">fn</span> gpu_error(error: wgpu::Error) {
    panic!(&quot;<span class="s">tutorial WebGPU error: </span>{<span class="s">error</span>}&quot;);
}</code></pre></div>
<h2 id="step-19-srcenginemodrs">Step 19 · src/engine/mod.rs<a class="anchor" href="#/course/04a-meshes#step-19-srcenginemodrs" aria-label="Link to this section">#</a></h2>
<p>Put the gpu and pipelines folders into the build.</p>
<p><code>lessons/04a/src/engine/mod.rs</code> · edit · type this</p>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> <span class="k">mod</span> gpu;</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> <span class="k">mod</span> pipelines;</code></pre></div>
<h2 id="step-20-srclibrs">Step 20 · src/lib.rs<a class="anchor" href="#/course/04a-meshes#step-20-srclibrs" aria-label="Link to this section">#</a></h2>
<p>Replace the entry point: it now owns a camera and the GPU.</p>
<p><code>lessons/04a/src/lib.rs</code> · type this, replace the whole file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> <span class="k">mod</span> app;
<span class="k">pub</span> <span class="k">mod</span> camera;
<span class="k">pub</span> <span class="k">mod</span> engine;
<span class="k">pub</span> <span class="k">mod</span> fixture;
<span class="k">use</span> engine::gpu::{FrameInput, Gpu};
<span class="k">use</span> wasm_bindgen::prelude::*; <span class="c">// Rust/WASM toolchain</span>

<span class="c">/// Everything needed to draw one frame: the browser owns the canvas, this struct owns the GPU.</span>
#[wasm_bindgen]
<span class="k">pub</span> <span class="k">struct</span> Tutorial {
    canvas: web_sys::HtmlCanvasElement,
    gpu: Gpu,
    camera: camera::Camera, <span class="c">// orbit, pan and zoom state</span>
    scale: f64,
}

#[wasm_bindgen] <span class="c">// Everything in this block is exported to JavaScript</span>
<span class="k">impl</span> Tutorial {
    <span class="c">/// Negotiate a presentation compatible browser adapter and build the first pipeline.</span>
    #[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
    <span class="k">pub</span> <span class="k">async</span> <span class="k">fn</span> create(canvas: web_sys::HtmlCanvasElement) -&gt; Result&lt;Tutorial, JsValue&gt; {
        console_error_panic_hook::set_once(); <span class="c">// Better error messages in the console</span>
        <span class="k">let</span> <span class="k">mut</span> gpu = Gpu::new(canvas.clone()).<span class="k">await</span>.map_err(js_error)?;
        <span class="k">let</span> <span class="k">mut</span> upload = fixture::scene();
        gpu.set_scene(&amp;upload);
        upload.drop_uploaded();
        <span class="c">// fit() picks the distance where this box fills the view, so the triangle is framed at startup.</span>
        <span class="k">let</span> <span class="k">mut</span> camera = camera::Camera::new();
        camera.unit = camera::Unit::Meters;
        camera.set_view(camera::View::Top);
        camera.fit(&amp;gpu.bounds, <span class="s">1</span>.<span class="s">5</span>);
        <span class="c">// Create an instance of the struct, Ok is needed to return also the error message Err(...)</span>
        Ok(<span class="k">Self</span> {
            canvas,
            gpu,
            camera,
            scale: <span class="s">1</span>.<span class="s">0</span>,
        })
    }

    <span class="c">/// Apply one camera gesture. The first GPU checkpoint intentionally has no camera yet.</span>
    <span class="k">pub</span> <span class="k">fn</span> drag(&amp;<span class="k">mut</span> <span class="k">self</span>, dx: f32, dy: f32, pan: bool) {
        <span class="k">if</span> pan {
            <span class="k">self</span>.camera.pan(dx, dy)
        } <span class="k">else</span> {
            <span class="k">self</span>.camera.orbit(dx, dy)
        }
    }

    <span class="c">/// Apply one cursor-centered camera zoom when the camera checkpoint is installed.</span>
    <span class="k">pub</span> <span class="k">fn</span> zoom(&amp;<span class="k">mut</span> <span class="k">self</span>, delta: f32, x: f64, y: f64) {
        <span class="k">self</span>.camera.zoom_at(
            delta,
            (x * <span class="k">self</span>.scale, y * <span class="k">self</span>.scale),
            (<span class="k">self</span>.gpu.config.width <span class="k">as</span> f64, <span class="k">self</span>.gpu.config.height <span class="k">as</span> f64),
        );
    }</code></pre></div>
<p><code>lessons/04a/src/lib.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Clear and draw one frame at full device-pixel resolution.</span>
    <span class="k">pub</span> <span class="k">fn</span> render(&amp;<span class="k">mut</span> <span class="k">self</span>, width: u32, height: u32, scale: f64) -&gt; Result&lt;String, JsValue&gt; {
        <span class="k">if</span> !scale.is_finite() || scale &lt;= <span class="s">0</span>.<span class="s">0</span> {
            <span class="k">return</span> Err(JsValue::from_str(&quot;<span class="s">invalid scale</span>&quot;));
        }

        <span class="k">self</span>.scale = scale;
        <span class="k">self</span>.gpu.logical_size = [width.max(<span class="s">1</span>) <span class="k">as</span> f64, height.max(<span class="s">1</span>) <span class="k">as</span> f64];
        <span class="k">let</span> w = (width.max(<span class="s">1</span>) <span class="k">as</span> f64 * scale).round() <span class="k">as</span> u32;
        <span class="k">let</span> h = (height.max(<span class="s">1</span>) <span class="k">as</span> f64 * scale).round() <span class="k">as</span> u32;

        <span class="k">if</span> <span class="k">self</span>.canvas.width() != w || <span class="k">self</span>.canvas.height() != h || <span class="k">self</span>.gpu.config.width == <span class="s">1</span> {
            <span class="k">self</span>.canvas.set_width(w);
            <span class="k">self</span>.canvas.set_height(h);
            <span class="k">self</span>.gpu.resize(w, h);
        }

        <span class="k">let</span> now = web_sys::window().and_then(window_time).unwrap_or(<span class="s">0</span>.<span class="s">0</span>);
        <span class="k">let</span> rebase =
            <span class="k">self</span>.gpu
                .rebase_anchor(&amp;<span class="k">self</span>.camera.origin(), <span class="k">self</span>.camera.distance_world(), now);
        <span class="k">let</span> input = FrameInput {
            view_proj: <span class="k">self</span>
                .camera
                .view_proj_anchored(w <span class="k">as</span> f64 / h <span class="k">as</span> f64, &amp;rebase.anchor),
            clear: wgpu::Color {
                r: <span class="s">0</span>.<span class="s">025</span>,
                g: <span class="s">0</span>.<span class="s">035</span>,
                b: <span class="s">0</span>.<span class="s">055</span>,
                a: <span class="s">1</span>.<span class="s">0</span>,
            },
            now_ms: now,
        };
        <span class="k">self</span>.gpu.render(&amp;input).map_err(js_error)?;
        Ok(serde_json::json!({&quot;<span class="s">stage</span>&quot;:<span class="s">4</span>,&quot;<span class="s">objects</span>&quot;:<span class="k">self</span>.gpu.objects.len(),&quot;<span class="s">width</span>&quot;:w,&quot;<span class="s">height</span>&quot;:h,&quot;<span class="s">scale</span>&quot;:scale,&quot;<span class="s">drawn</span>&quot;:<span class="s">true</span>,
            &quot;<span class="s">meshVertices</span>&quot;:<span class="k">self</span>.gpu.arena.vert_count()}).to_string())
    }
}

<span class="c">/// Frame time from the browser clock.</span>
<span class="k">fn</span> window_time(window: web_sys::Window) -&gt; Option&lt;f64&gt; {
    Some(window.performance()?.now())
}

<span class="c">/// Keep initialization/render errors visible at the browser boundary.</span>
<span class="k">fn</span> js_error(error: <span class="k">impl</span> std::fmt::Display) -&gt; JsValue {
    JsValue::from_str(&amp;error.to_string())
}</code></pre></div>
<h2 id="step-21-srcsceners">Step 21 · src/scene.rs<a class="anchor" href="#/course/04a-meshes#step-21-srcsceners" aria-label="Link to this section">#</a></h2>
<p>Delete the old scene file.</p>
<p>Delete <code>src/scene.rs</code> (it exists in <code>lessons/03/</code>, not in <code>lessons/04a/</code>).</p>
<h2 id="step-22-srcshadersfirstwgsl">Step 22 · src/shaders/first.wgsl<a class="anchor" href="#/course/04a-meshes#step-22-srcshadersfirstwgsl" aria-label="Link to this section">#</a></h2>
<p>Delete the first triangle shader.</p>
<p>Delete <code>src/shaders/first.wgsl</code> (it exists in <code>lessons/03/</code>, not in <code>lessons/04a/</code>).</p>
<h2 id="step-23-indexhtml">Step 23 · index.html<a class="anchor" href="#/course/04a-meshes#step-23-indexhtml" aria-label="Link to this section">#</a></h2>
<p>Copy the page: the status now reports mesh vertices.
Copy this file from the lesson folder to the path shown.</p>
<p><code>lessons/04a/index.html</code> · edit · copy the file</p>
<p>Replaces the line <code>&lt;title&gt;Session checkpoint 03&lt;/title&gt;</code> in <code>lessons/03/index.html</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    &lt;title&gt;Session checkpoint 04a&lt;/title&gt;</code></pre></div>
<p>Replaces the line <code>&lt;output id=&quot;status&quot;&gt;Starting checkpoint 03&lt;/output&gt;</code> in <code>lessons/03/index.html</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    &lt;output id=&quot;<span class="s">status</span>&quot;&gt;Starting checkpoint 04a&lt;/output&gt;</code></pre></div>
<p>Replaces the line <code>document.getElementById(&#39;status&#39;).textContent = &#39;Checkpoi…</code> in <code>lessons/03/index.html</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        document.getElementById('status').textContent = 'Checkpoint 04a · ' + state.objects + ' objects · ' + state.width + '×' + state.height;</code></pre></div>
<p>Run <code>cargo check</code> in <code>lessons/04a/</code>.</p>
<h2 id="check">Check<a class="anchor" href="#/course/04a-meshes#check" aria-label="Link to this section">#</a></h2>
<p>Run <code>trunk serve</code> in <code>lessons/04a/</code> and open <a href="http://127.0.0.1:8770/" target="_blank" rel="noopener">http://127.0.0.1:8770/</a>.</p>
<p>Expected: A blue triangle draws from mesh buffers while the camera still orbits and zooms; status: <strong>Checkpoint 04a · 1 objects</strong>.</p>
<p><img src="/session/docs/course/docs/screenshots/04a.png" alt="Checkpoint 04a: the first mesh drawn from arena buffers through the object table." loading="lazy" decoding="async"></p>
<p>If it fails:</p>
<ul>
<li>The canvas stays empty: the surface is not configured or the arena has no rows.</li>
<li>Geometry is scrambled: the vertex stride or object row layout differs from the shader.</li>
</ul>
<h2 id="what-changed">What changed<a class="anchor" href="#/course/04a-meshes#what-changed" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>lessons/04a/src/
├── app/
│   ├── mod.rs  +
│   └── route.rs  +
├── engine/
│   ├── gpu/
│   │   ├── arena.rs  +
│   │   ├── buffers.rs  +
│   │   ├── frame.rs  +
│   │   ├── instance.rs  ~
│   │   ├── mod.rs  ~
│   │   ├── objects.rs  +
│   │   ├── targets.rs  +
│   │   ├── text_outline.rs  +
│   │   ├── upload.rs  +
│   │   └── view.rs  +
│   ├── pipelines/
│   │   ├── layouts.rs  +
│   │   └── mod.rs  +
│   └── mod.rs  ~
├── shaders/
│   ├── normals.wgsl  +
│   ├── scene.wgsl  +
│   ├── text_outline.wgsl  +
│   └── triangle.wgsl  +
├── camera.rs
├── fixture.rs  +
└── lib.rs  ~</code></pre></div>
<p><code>+</code> new in this lesson · <code>~</code> changed in this lesson</p>
<p>Data flow: source files → retained scene state → GPU buffers → visible result. Every file at this point: <code>lessons/04a/</code>.</p>
<h2 id="next">Next<a class="anchor" href="#/course/04a-meshes#next" aria-label="Link to this section">#</a></h2>
<p><a href="#/course/04b-strokes">04b · Strokes</a>: the segment drawing module, <code>ribbon.wgsl</code>, and the shared ink visibility rule.</p>
<h2 id="expected-viewer-result">Expected viewer result<a class="anchor" href="#/course/04a-meshes#expected-viewer-result" aria-label="Link to this section">#</a></h2>
<p>Checkpoint 04a: the first mesh drawn from arena buffers through the object table.</p>
<p><a href="/session/docs/course/docs/screenshots/04a.png"><img src="/session/docs/course/docs/screenshots/04a.png" alt="Full viewer result for 04a meshes" loading="lazy" decoding="async"></a></p>
`,toc:[{level:2,id:"step-1-srcenginegpubuffersrs",text:"Step 1 · src/engine/gpu/buffers.rs"},{level:2,id:"step-2-srcenginepipelineslayoutsrs",text:"Step 2 · src/engine/pipelines/layouts.rs"},{level:2,id:"step-3-srcenginepipelinesmodrs",text:"Step 3 · src/engine/pipelines/mod.rs"},{level:2,id:"step-4-srcshadersscenewgsl",text:"Step 4 · src/shaders/scene.wgsl"},{level:2,id:"step-5-srcshadersnormalswgsl",text:"Step 5 · src/shaders/normals.wgsl"},{level:2,id:"step-6-srcenginegputargetsrs",text:"Step 6 · src/engine/gpu/targets.rs"},{level:2,id:"step-7-srcenginegpuframers",text:"Step 7 · src/engine/gpu/frame.rs"},{level:2,id:"step-8-srcenginegpuviewrs",text:"Step 8 · src/engine/gpu/view.rs"},{level:2,id:"step-9-srcappmodrs",text:"Step 9 · src/app/mod.rs"},{level:2,id:"step-10-srcapprouters",text:"Step 10 · src/app/route.rs"},{level:2,id:"step-11-srcenginegpuobjectsrs",text:"Step 11 · src/engine/gpu/objects.rs"},{level:2,id:"step-12-srcshaderstrianglewgsl",text:"Step 12 · src/shaders/triangle.wgsl"},{level:2,id:"step-13-srcenginegpuarenars",text:"Step 13 · src/engine/gpu/arena.rs"},{level:2,id:"step-14-srcshaderstext_outlinewgsl",text:"Step 14 · src/shaders/text_outline.wgsl"},{level:2,id:"step-15-srcenginegputext_outliners",text:"Step 15 · src/engine/gpu/text_outline.rs"},{level:2,id:"step-16-srcenginegpuuploadrs",text:"Step 16 · src/engine/gpu/upload.rs"},{level:2,id:"step-17-srcfixturers",text:"Step 17 · src/fixture.rs"},{level:2,id:"step-18-srcenginegpumodrs",text:"Step 18 · src/engine/gpu/mod.rs"},{level:2,id:"step-19-srcenginemodrs",text:"Step 19 · src/engine/mod.rs"},{level:2,id:"step-20-srclibrs",text:"Step 20 · src/lib.rs"},{level:2,id:"step-21-srcsceners",text:"Step 21 · src/scene.rs"},{level:2,id:"step-22-srcshadersfirstwgsl",text:"Step 22 · src/shaders/first.wgsl"},{level:2,id:"step-23-indexhtml",text:"Step 23 · index.html"},{level:2,id:"check",text:"Check"},{level:2,id:"what-changed",text:"What changed"},{level:2,id:"next",text:"Next"},{level:2,id:"expected-viewer-result",text:"Expected viewer result"}]};export{s as default};
