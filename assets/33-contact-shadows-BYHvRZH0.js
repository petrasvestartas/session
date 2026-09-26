const s={title:"33 · Contact shadows that follow object size",html:`<h1 id="33-contact-shadows-that-follow-object-size">33 · Contact shadows that follow object size<a class="anchor" href="#/course/33-contact-shadows#33-contact-shadows-that-follow-object-size" aria-label="Link to this section">#</a></h1>
<p>Each object carries its own ambient-occlusion radius, so a small part and a large one both get a soft contact shadow that fits.</p>
<h2 id="step-1-srcenginegpuinstancers">Step 1 · src/engine/gpu/instance.rs<a class="anchor" href="#/course/33-contact-shadows#step-1-srcenginegpuinstancers" aria-label="Link to this section">#</a></h2>
<p>Each object row carries a contact radius where the padding word used to be.</p>
<p><code>lessons/33/src/engine/gpu/instance.rs</code> · edit · type this</p>
<p>Replaces the line <code>pub _pad0: f32,</code> in <code>lessons/32/src/engine/gpu/instance.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> ao_radius: f32, <span class="c">// SSAO contact radius, world units</span></code></pre></div>
<p>Replaces the line <code>_pad0: 0.0,</code> in <code>lessons/32/src/engine/gpu/instance.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            ao_radius: <span class="s">0</span>.<span class="s">0</span>,</code></pre></div>
<p>Replaces the line <code>let rust = [&quot;model&quot;, &quot;color&quot;, &quot;flags&quot;, &quot;_pad0&quot;, &quot;spacing&quot;…</code> in <code>lessons/32/src/engine/gpu/instance.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> rust = [
            &quot;<span class="s">model</span>&quot;,
            &quot;<span class="s">color</span>&quot;,
            &quot;<span class="s">flags</span>&quot;,
            &quot;<span class="s">ao_radius</span>&quot;,
            &quot;<span class="s">spacing</span>&quot;,
            &quot;<span class="s">edge_color</span>&quot;,
        ];</code></pre></div>
<h2 id="step-2-srcenginegpuobjectsrs">Step 2 · src/engine/gpu/objects.rs<a class="anchor" href="#/course/33-contact-shadows#step-2-srcenginegpuobjectsrs" aria-label="Link to this section">#</a></h2>
<p>The radius is 5% of the object&#39;s diagonal, so it survives a move and scales with a resize.</p>
<p><code>lessons/33/src/engine/gpu/objects.rs</code> · edit · type this</p>
<p>Added after the line <code>r.bounds.transformed(&amp;r.place)</code> in <code>lessons/32/src/engine/gpu/objects.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>}

<span class="c">/// A small object needs a small contact shadow, so each object gets its own radius: 5% of its box diagonal.</span>
<span class="k">fn</span> ambient_radius(bounds: &amp;AABB) -&gt; f32 {
    <span class="k">if</span> !bounds.is_valid() {
        <span class="k">return</span> <span class="s">0</span>.<span class="s">0</span>;
    }
    (<span class="s">0</span>.<span class="s">05</span> * bounds.hx.hypot(bounds.hy).hypot(bounds.hz)).max(<span class="s">0</span>.<span class="s">01</span>) <span class="k">as</span> f32</code></pre></div>
<p>Replaces the line <code>_pad0: 0.0,</code> in <code>lessons/32/src/engine/gpu/objects.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                ao_radius: ambient_radius(&amp;world),</code></pre></div>
<p>Added after the line <code>instance.model = model;</code> in <code>lessons/32/src/engine/gpu/objects.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        instance.ao_radius = ambient_radius(&amp;world);</code></pre></div>
<p>Added after the line <code>#[test]</code> in <code>lessons/32/src/engine/gpu/objects.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    #[test]
    <span class="c">/// The SSAO radius scales with the box, not with where it is.</span>
    <span class="k">fn</span> contact_radius_follows_object_size_not_position() {
        <span class="k">let</span> small = AABB::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">50</span>.<span class="s">0</span>, <span class="s">50</span>.<span class="s">0</span>, <span class="s">50</span>.<span class="s">0</span>);
        <span class="k">let</span> moved = small.transformed(&amp;Xform::translation(<span class="s">1</span>.<span class="s">0</span>e<span class="s">6</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>));
        <span class="k">let</span> large = small.transformed(&amp;Xform::scale_xyz(<span class="s">100</span>.<span class="s">0</span>, <span class="s">100</span>.<span class="s">0</span>, <span class="s">100</span>.<span class="s">0</span>));
        assert_eq!(ambient_radius(&amp;small), ambient_radius(&amp;moved));
        assert!((ambient_radius(&amp;large) / ambient_radius(&amp;small) - <span class="s">100</span>.<span class="s">0</span>).abs() &lt; <span class="s">1</span>e-<span class="s">4</span>);
        assert_eq!(ambient_radius(&amp;AABB::empty()), <span class="s">0</span>.<span class="s">0</span>);
    }

    <span class="c">/// A translated local box lands at the translated world position.</span></code></pre></div>
<p>Added after the line <code>assert_eq!(gpu.objects.len(), 2);</code> in <code>lessons/32/src/engine/gpu/objects.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> radius = gpu.objects.row(<span class="s">0</span>).unwrap().ao_radius;

        <span class="k">let</span> moved = Xform::translation(<span class="s">0</span>.<span class="s">0</span>, <span class="s">50</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>);
        assert!(gpu.objects.set_placement(&amp;gpu.ctx, <span class="s">0</span>, &amp;moved));
        assert_eq!(gpu.objects.row(<span class="s">0</span>).unwrap().ao_radius, radius);</code></pre></div>
<p>Added after the line <code>assert_eq!(second.min_point()[1], -1.0, &quot;the neighbour di…</code> in <code>lessons/32/src/engine/gpu/objects.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        gpu.objects
            .set_placement(&amp;gpu.ctx, <span class="s">0</span>, &amp;Xform::scale_xyz(<span class="s">2</span>.<span class="s">0</span>, <span class="s">2</span>.<span class="s">0</span>, <span class="s">2</span>.<span class="s">0</span>));
        assert_eq!(gpu.objects.row(<span class="s">0</span>).unwrap().ao_radius, <span class="s">2</span>.<span class="s">0</span> * radius);
        assert_eq!(gpu.objects.row(<span class="s">1</span>).unwrap().ao_radius, radius);</code></pre></div>
<h2 id="step-3-srcenginegpurenderrs">Step 3 · src/engine/gpu/render.rs<a class="anchor" href="#/course/33-contact-shadows#step-3-srcenginegpurenderrs" aria-label="Link to this section">#</a></h2>
<p>The projected-triangle tiles are handed to the ambient pass.</p>
<p><code>lessons/33/src/engine/gpu/render.rs</code> · edit · type this</p>
<p>Added after the line <code>&amp;self.targets,</code> in <code>lessons/32/src/engine/gpu/render.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                &amp;<span class="k">self</span>.arena.tiles.projected,</code></pre></div>
<h2 id="step-4-srcenginegpussaors">Step 4 · src/engine/gpu/ssao.rs<a class="anchor" href="#/course/33-contact-shadows#step-4-srcenginegpussaors" aria-label="Link to this section">#</a></h2>
<p>Replace the whole file: the ambient pass samples a hemisphere per pixel and blends it with a contact term.</p>
<p><code>lessons/33/src/engine/gpu/ssao.rs</code> · replace the whole file · type this</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> super::{buffers::GpuCtx, targets::Targets};
<span class="k">use</span> <span class="k">crate</span>::engine::pipelines::{ColorWrite, DepthMode, PipelineDesc, Target, build, module};

<span class="c">/// Screen-space ambient occlusion: soft shadows where surfaces meet.</span>
<span class="k">pub</span> <span class="k">struct</span> Ssao {
    target: Target, <span class="c">// scene color format and samples</span>
    layout: wgpu::BindGroupLayout, <span class="c">// depth, uniform, gradient, triangles</span>
    raw: wgpu::RenderPipeline, <span class="c">// computes occlusion per pixel</span>
    composite: wgpu::RenderPipeline, <span class="c">// darkens the scene with it</span>
    filter: [wgpu::RenderPipeline; <span class="s">2</span>], <span class="c">// blur in x, then in y</span>
    filtered: wgpu::Texture, <span class="c">// half-blurred occlusion</span>
    filtered_view: wgpu::TextureView, <span class="c">// view of it</span>
    filtered_group: wgpu::BindGroup, <span class="c">// binds it</span>
    inverse: wgpu::Buffer, <span class="c">// inverse camera, camera, ground, size</span>
    texture: wgpu::Texture, <span class="c">// occlusion per pixel</span>
    view: wgpu::TextureView, <span class="c">// view of it</span>
    sampled: wgpu::BindGroup, <span class="c">// binds it</span>
    size: (u32, u32), <span class="c">// occlusion texture size, px</span>
    cached: Option&lt;([f32; 36], u64)&gt;, <span class="c">// uniform and geometry the occlusion was computed for</span>
    receiver_bounds: Option&lt;(u64, session_rust::AABB, f32)&gt;, <span class="c">// geometry revision, box of shadow receivers, largest radius</span>
}

<span class="k">impl</span> Ssao {
    <span class="c">/// Ground height and contact radius from the visible solids.</span>
    <span class="k">pub</span> <span class="k">fn</span> receiver(&amp;<span class="k">mut</span> <span class="k">self</span>, objects: &amp;super::objects::InstanceTable) -&gt; [f32; <span class="s">2</span>] {
        <span class="k">let</span> revision = objects.geometry_revision();
        <span class="c">// recompute when the objects changed</span>
        <span class="k">if</span> <span class="k">self</span>
            .receiver_bounds
            .as_ref()
            .is_none_or(|(r, _, _)| *r != revision)
        {
            <span class="k">let</span> <span class="k">mut</span> bounds = session_rust::AABB::empty();
            <span class="k">let</span> <span class="k">mut</span> radius = <span class="s">0</span>.<span class="s">01_f32</span>;
            <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..objects.len() {
                <span class="k">let</span> flags = objects.row(i).unwrap().flags;
                <span class="k">if</span> flags &amp; super::Instance::FLAG_HAS_FACES != <span class="s">0</span>
                    &amp;&amp; flags &amp; (super::Instance::FLAG_HIDDEN | super::Instance::FLAG_SHEET) == <span class="s">0</span>
                    &amp;&amp; <span class="k">let</span> Some(b) = objects.row_bounds(i)
                {
                    bounds.union_with(&amp;b);
                    radius = radius.max(objects.row(i).unwrap().ao_radius);
                }
            }
            <span class="k">self</span>.receiver_bounds = Some((revision, bounds, radius));
        }
        <span class="k">let</span> b = &amp;<span class="k">self</span>.receiver_bounds.as_ref().unwrap().<span class="s">1</span>;
        <span class="k">let</span> radius = <span class="k">self</span>.receiver_bounds.as_ref().unwrap().<span class="s">2</span>;
        <span class="c">// bottom of the box, relative to the scene origin</span>
        [(b.cz - b.hz - objects.anchor()[<span class="s">2</span>]) <span class="k">as</span> f32, radius]
    }

    <span class="c">/// Create the pipelines and textures for a \`full\`-sized canvas.</span>
    <span class="k">pub</span> <span class="k">fn</span> new(ctx: &amp;GpuCtx, target: Target, full: (u32, u32)) -&gt; <span class="k">Self</span> {
        <span class="c">// group 0: depth, uniform, gradient, projected triangles</span>
        <span class="k">let</span> layout = ctx
            .device
            .create_bind_group_layout(&amp;wgpu::BindGroupLayoutDescriptor {
                label: Some(&quot;<span class="s">ambient depth</span>&quot;),
                entries: &amp;[
                    wgpu::BindGroupLayoutEntry {
                        binding: <span class="s">0</span>,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Depth,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: target.samples &gt; <span class="s">1</span>,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: <span class="s">1</span>,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: <span class="s">false</span>,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: <span class="s">2</span>,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: <span class="s">false</span> },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: target.samples &gt; <span class="s">1</span>,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: <span class="s">3</span>,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: <span class="s">true</span> },
                            has_dynamic_offset: <span class="s">false</span>,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            });
        <span class="c">// group 1: one occlusion texture</span>
        <span class="k">let</span> sample_layout = ctx
            .device
            .create_bind_group_layout(&amp;wgpu::BindGroupLayoutDescriptor {
                label: Some(&quot;<span class="s">ambient reconstruction</span>&quot;),
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
        <span class="k">let</span> shader = module(&amp;ctx.device, &quot;<span class="s">ambient</span>&quot;, &amp;shader_source(target.samples));
        <span class="c">// occlusion into a one-channel half-float texture</span>
        <span class="k">let</span> raw = build(
            &amp;ctx.device,
            Target {
                format: wgpu::TextureFormat::R16Float,
                samples: <span class="s">1</span>,
            },
            &amp;PipelineDesc::new(
                &amp;shader,
                &amp;[&amp;layout],
                &amp;[],
                wgpu::PrimitiveTopology::TriangleList,
            )
            .with(&quot;<span class="s">ambient hemisphere</span>&quot;, &quot;<span class="s">fs_main</span>&quot;)
            .depth(DepthMode::Detached),
        );
        <span class="c">// two blur passes, one per axis</span>
        <span class="k">let</span> filter = [&quot;<span class="s">fs_filter_x</span>&quot;, &quot;<span class="s">fs_filter_y</span>&quot;].map(|entry| {
            build(
                &amp;ctx.device,
                Target {
                    format: wgpu::TextureFormat::R16Float,
                    samples: <span class="s">1</span>,
                },
                &amp;PipelineDesc::new(
                    &amp;shader,
                    &amp;[&amp;layout, &amp;sample_layout],
                    &amp;[],
                    wgpu::PrimitiveTopology::TriangleList,
                )
                .with(&quot;<span class="s">ambient denoise</span>&quot;, entry)
                .depth(DepthMode::Detached),
            )
        });
        <span class="c">// multiply the scene by the occlusion</span>
        <span class="k">let</span> composite = build(
            &amp;ctx.device,
            target,
            &amp;PipelineDesc::new(
                &amp;shader,
                &amp;[&amp;layout, &amp;sample_layout],
                &amp;[],
                wgpu::PrimitiveTopology::TriangleList,
            )
            .with(&quot;<span class="s">ambient reconstruction</span>&quot;, &quot;<span class="s">fs_composite</span>&quot;)
            .depth(DepthMode::Detached)
            .color(ColorWrite::Blended),
        );
        <span class="c">// 36 floats: inverse camera, camera, ground, size</span>
        <span class="k">let</span> inverse = ctx.device.create_buffer(&amp;wgpu::BufferDescriptor {
            label: Some(&quot;<span class="s">ambient inverse and ground</span>&quot;),
            size: <span class="s">144</span>,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: <span class="s">false</span>,
        });
        <span class="c">// occlusion at most 1920 px wide</span>
        <span class="k">let</span> scale = <span class="s">1</span>.<span class="s">0_f64</span>.min(<span class="s">1920</span>.<span class="s">0</span> / f64::from(full.<span class="s">0</span>.max(full.<span class="s">1</span>).max(<span class="s">1</span>)));
        <span class="k">let</span> size = (
            (f64::from(full.<span class="s">0</span>) * scale).ceil().max(<span class="s">1</span>.<span class="s">0</span>) <span class="k">as</span> u32,
            (f64::from(full.<span class="s">1</span>) * scale).ceil().max(<span class="s">1</span>.<span class="s">0</span>) <span class="k">as</span> u32,
        );
        <span class="k">let</span> descriptor = wgpu::TextureDescriptor {
            label: Some(&quot;<span class="s">ambient half-float display pixels</span>&quot;),
            size: wgpu::Extent3d {
                width: size.<span class="s">0</span>,
                height: size.<span class="s">1</span>,
                depth_or_array_layers: <span class="s">1</span>,
            },
            mip_level_count: <span class="s">1</span>,
            sample_count: <span class="s">1</span>,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R16Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &amp;[],
        };
        <span class="k">let</span> texture = ctx.device.create_texture(&amp;descriptor);
        <span class="k">let</span> filtered = ctx.device.create_texture(&amp;descriptor);
        <span class="k">let</span> filtered_view = filtered.create_view(&amp;Default::default());
        <span class="k">let</span> filtered_group = ctx.device.create_bind_group(&amp;wgpu::BindGroupDescriptor {
            label: Some(&quot;<span class="s">ambient filtered texture</span>&quot;),
            layout: &amp;sample_layout,
            entries: &amp;[wgpu::BindGroupEntry {
                binding: <span class="s">0</span>,
                resource: wgpu::BindingResource::TextureView(&amp;filtered_view),
            }],
        });
        <span class="k">let</span> view = texture.create_view(&amp;Default::default());
        <span class="k">let</span> sampled = ctx.device.create_bind_group(&amp;wgpu::BindGroupDescriptor {
            label: Some(&quot;<span class="s">ambient texture</span>&quot;),
            layout: &amp;sample_layout,
            entries: &amp;[wgpu::BindGroupEntry {
                binding: <span class="s">0</span>,
                resource: wgpu::BindingResource::TextureView(&amp;view),
            }],
        });
        <span class="k">Self</span> {
            target,
            layout,
            raw,
            composite,
            filter,
            filtered,
            filtered_view,
            filtered_group,
            inverse,
            texture,
            view,
            sampled,
            size,
            cached: None,
            receiver_bounds: None,
        }
    }

    <span class="c">/// Bytes of the two occlusion textures.</span>
    <span class="k">pub</span> <span class="k">fn</span> texture_bytes(&amp;<span class="k">self</span>) -&gt; u64 {
        <span class="s">4</span> * u64::from(<span class="k">self</span>.size.<span class="s">0</span>) * u64::from(<span class="k">self</span>.size.<span class="s">1</span>)
    }

    <span class="c">/// Compute occlusion if needed, then darken the scene; returns the draw count.</span>
    #[allow(clippy::too_many_arguments)]
    <span class="k">pub</span> <span class="k">fn</span> draw(
        &amp;<span class="k">mut</span> <span class="k">self</span>,
        ctx: &amp;GpuCtx,
        target: Target,
        targets: &amp;Targets,
        projected: &amp;wgpu::Buffer,
        encoder: &amp;<span class="k">mut</span> wgpu::CommandEncoder,
        view: &amp;wgpu::TextureView,
        mvp: [f32; <span class="s">16</span>],
        ground: [f32; <span class="s">2</span>],
        revision: u64,
    ) -&gt; u32 {
        <span class="k">let</span> size = (
            targets.depth.texture().width(),
            targets.depth.texture().height(),
        );
        <span class="c">// remake everything when format or size changed</span>
        <span class="k">if</span> <span class="k">self</span>.target != target
            || <span class="k">self</span>
                .cached
                .is_some_and(|(key, _)| key[<span class="s">34</span>] != size.<span class="s">0</span> <span class="k">as</span> f32 || key[<span class="s">35</span>] != size.<span class="s">1</span> <span class="k">as</span> f32)
        {
            *<span class="k">self</span> = <span class="k">Self</span>::new(ctx, target, size);
        }
        <span class="c">// clip space back to view space</span>
        <span class="k">let</span> Some(inverse) = inverse_projection(mvp) <span class="k">else</span> {
            <span class="k">return</span> <span class="s">0</span>;
        };
        <span class="k">let</span> <span class="k">mut</span> uniform = [<span class="s">0</span>.<span class="s">0</span>; <span class="s">36</span>];
        uniform[..<span class="s">16</span>].copy_from_slice(&amp;inverse);
        uniform[<span class="s">16</span>..<span class="s">32</span>].copy_from_slice(&amp;mvp);
        uniform[<span class="s">32</span>..].copy_from_slice(&amp;[ground[<span class="s">0</span>], ground[<span class="s">1</span>], size.<span class="s">0</span> <span class="k">as</span> f32, size.<span class="s">1</span> <span class="k">as</span> f32]);
        ctx.queue
            .write_buffer(&amp;<span class="k">self</span>.inverse, <span class="s">0</span>, bytemuck::cast_slice(&amp;uniform));
        <span class="c">// this frame's depth and gradient</span>
        <span class="k">let</span> group = ctx.device.create_bind_group(&amp;wgpu::BindGroupDescriptor {
            label: Some(&quot;<span class="s">ambient</span>&quot;),
            layout: &amp;<span class="k">self</span>.layout,
            entries: &amp;[
                wgpu::BindGroupEntry {
                    binding: <span class="s">0</span>,
                    resource: wgpu::BindingResource::TextureView(&amp;targets.depth),
                },
                wgpu::BindGroupEntry {
                    binding: <span class="s">1</span>,
                    resource: <span class="k">self</span>.inverse.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: <span class="s">2</span>,
                    resource: wgpu::BindingResource::TextureView(&amp;targets.gradient),
                },
                wgpu::BindGroupEntry {
                    binding: <span class="s">3</span>,
                    resource: projected.as_entire_binding(),
                },
            ],
        });
        <span class="k">let</span> <span class="k">mut</span> draws = <span class="s">1</span>;
        <span class="c">// recompute only when camera or geometry moved</span>
        <span class="k">if</span> <span class="k">self</span>.cached != Some((uniform, revision)) {
            <span class="k">let</span> <span class="k">mut</span> pass = encoder.begin_render_pass(&amp;wgpu::RenderPassDescriptor {
                label: Some(&quot;<span class="s">ambient horizons</span>&quot;),
                color_attachments: &amp;[Some(wgpu::RenderPassColorAttachment {
                    view: &amp;<span class="k">self</span>.view,
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
            pass.set_pipeline(&amp;<span class="k">self</span>.raw);
            pass.set_bind_group(<span class="s">0</span>, &amp;group, &amp;[]);
            pass.draw(<span class="s">0</span>..<span class="s">3</span>, <span class="s">0</span>..<span class="s">1</span>);
            drop(pass);
            <span class="c">// blur x into filtered, then y back into view</span>
            <span class="k">for</span> (pipeline, output, input) <span class="k">in</span> [
                (&amp;<span class="k">self</span>.filter[<span class="s">0</span>], &amp;<span class="k">self</span>.filtered_view, &amp;<span class="k">self</span>.sampled),
                (&amp;<span class="k">self</span>.filter[<span class="s">1</span>], &amp;<span class="k">self</span>.view, &amp;<span class="k">self</span>.filtered_group),
            ] {
                <span class="k">let</span> <span class="k">mut</span> pass = encoder.begin_render_pass(&amp;wgpu::RenderPassDescriptor {
                    label: Some(&quot;<span class="s">ambient denoise</span>&quot;),
                    color_attachments: &amp;[Some(wgpu::RenderPassColorAttachment {
                        view: output,
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
                pass.set_pipeline(pipeline);
                pass.set_bind_group(<span class="s">0</span>, &amp;group, &amp;[]);
                pass.set_bind_group(<span class="s">1</span>, input, &amp;[]);
                pass.draw(<span class="s">0</span>..<span class="s">3</span>, <span class="s">0</span>..<span class="s">1</span>);
            }
            <span class="k">self</span>.cached = Some((uniform, revision));
            draws += <span class="s">3</span>;
        }
        <span class="c">// darken the scene color</span>
        <span class="k">let</span> <span class="k">mut</span> pass = encoder.begin_render_pass(&amp;wgpu::RenderPassDescriptor {
            label: Some(&quot;<span class="s">ambient composite</span>&quot;),
            color_attachments: &amp;[Some(wgpu::RenderPassColorAttachment {
                view: targets.msaa.as_deref().unwrap_or(view),
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        pass.set_pipeline(&amp;<span class="k">self</span>.composite);
        pass.set_bind_group(<span class="s">0</span>, &amp;group, &amp;[]);
        pass.set_bind_group(<span class="s">1</span>, &amp;<span class="k">self</span>.sampled, &amp;[]);
        pass.draw(<span class="s">0</span>..<span class="s">3</span>, <span class="s">0</span>..<span class="s">1</span>);
        draws
    }
}

<span class="c">/// Free the textures now, not when the browser collects them.</span>
<span class="k">impl</span> Drop <span class="k">for</span> Ssao {
    <span class="c">/// Remove the DOM listener.</span>
    <span class="k">fn</span> drop(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">self</span>.texture.destroy();
        <span class="k">self</span>.filtered.destroy();
    }
}

<span class="c">/// The shader text, rewritten for multisampled depth when needed.</span>
<span class="k">fn</span> shader_source(samples: u32) -&gt; String {
    <span class="k">let</span> source = include_str!(&quot;<span class="s">../../shaders/ssao.wgsl</span>&quot;);
    <span class="k">if</span> samples &gt; <span class="s">1</span> {
        source
            .replace(&quot;<span class="s">texture_depth_2d</span>&quot;, &quot;<span class="s">texture_depth_multisampled_2d</span>&quot;)
            .replace(
                &quot;<span class="s">fn fs_composite(@builtin(position) pixel: vec4&lt;f32&gt;)</span>&quot;,
                &quot;<span class="s">fn fs_composite(@builtin(position) pixel: vec4&lt;f32&gt;, @builtin(sample_index) sample: u32)</span>&quot;,
            )
            .replace(&quot;<span class="s">reconstruct(pixel.xy,vec2&lt;i32&gt;(0),0)</span>&quot;, &quot;<span class="s">reconstruct(pixel.xy,vec2&lt;i32&gt;(0),i32(sample))</span>&quot;)
            .replace(
                &quot;<span class="s">physical: texture_2d&lt;f32&gt;</span>&quot;,
                &quot;<span class="s">physical: texture_multisampled_2d&lt;f32&gt;</span>&quot;,
            )
    } <span class="k">else</span> {
        source.to_owned()
    }
}

<span class="c">/// Invert the camera matrix; rows are scaled first to keep precision.</span>
<span class="k">fn</span> inverse_projection(matrix: [f32; <span class="s">16</span>]) -&gt; Option&lt;[f32; 16]&gt; {
    <span class="k">let</span> <span class="k">mut</span> normalized = matrix.map(f64::from);
    <span class="k">let</span> scales: [f64; <span class="s">4</span>] = std::array::from_fn(|row| {
        (<span class="s">0</span>..<span class="s">4</span>)
            .map(|col| normalized[col * <span class="s">4</span> + row].abs())
            .fold(<span class="s">0</span>.<span class="s">0</span>, f64::max)
    });
    <span class="k">if</span> scales.iter().any(|s| !s.is_finite() || *s == <span class="s">0</span>.<span class="s">0</span>) {
        <span class="k">return</span> None;
    }
    <span class="k">for</span> row <span class="k">in</span> <span class="s">0</span>..<span class="s">4</span> {
        <span class="k">for</span> col <span class="k">in</span> <span class="s">0</span>..<span class="s">4</span> {
            normalized[col * <span class="s">4</span> + row] /= scales[row];
        }
    }
    <span class="k">let</span> <span class="k">mut</span> inverse = session_rust::Xform::from_matrix(normalized).inverse()?.m;
    <span class="k">for</span> col <span class="k">in</span> <span class="s">0</span>..<span class="s">4</span> {
        <span class="k">for</span> row <span class="k">in</span> <span class="s">0</span>..<span class="s">4</span> {
            inverse[col * <span class="s">4</span> + row] /= scales[col];
        }
    }
    Some(inverse.map(|v| v <span class="k">as</span> f32))
}

#[cfg(test)]
<span class="k">mod</span> tests {
    #[cfg(not(target_arch = &quot;<span class="s">wasm32</span>&quot;))]
    #[test]
    #[ignore = &quot;<span class="s">requires a native GPU adapter</span>&quot;]
    <span class="c">/// Contact darkens pixels, far ground stays bright, memory returns.</span>
    <span class="k">fn</span> occlusion_darkens_contact_and_releases_its_small_uniform() {
        <span class="k">use</span> <span class="k">crate</span>::app::scene::{FileDoc, Scene};
        <span class="k">use</span> <span class="k">crate</span>::camera::Camera;
        <span class="k">use</span> <span class="k">crate</span>::engine::gpu::{FrameInput, Gpu};
        <span class="k">use</span> session_rust::{BRep, Session, Xform};
        <span class="k">use</span> std::rc::Rc;
        <span class="k">let</span> <span class="k">mut</span> gpu = pollster::block_on(Gpu::new_headless(<span class="s">256</span>, <span class="s">256</span>)).unwrap();
        gpu.view.show_grid = <span class="s">false</span>;
        <span class="k">let</span> <span class="k">mut</span> source = Session::new(&quot;<span class="s">contact</span>&quot;);
        source.add_brep(BRep::create_box(<span class="s">100</span>.<span class="s">0</span>, <span class="s">100</span>.<span class="s">0</span>, <span class="s">10</span>.<span class="s">0</span>), None);
        <span class="k">let</span> tower = source
            .add_brep(BRep::create_box(<span class="s">30</span>.<span class="s">0</span>, <span class="s">30</span>.<span class="s">0</span>, <span class="s">80</span>.<span class="s">0</span>), None)
            .unwrap();
        source.set_xform(&amp;tower.borrow().name, Xform::translation(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">35</span>.<span class="s">0</span>));
        <span class="k">let</span> raised = source
            .add_brep(BRep::create_box(<span class="s">40</span>.<span class="s">0</span>, <span class="s">40</span>.<span class="s">0</span>, <span class="s">10</span>.<span class="s">0</span>), None)
            .unwrap();
        source.set_xform(&amp;raised.borrow().name, Xform::translation(<span class="s">110</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">12</span>.<span class="s">0</span>));
        <span class="k">if</span> std::env::var_os(&quot;<span class="s">VIEWER_AO_CAPTURE</span>&quot;).is_some() {
            std::fs::create_dir_all(&quot;<span class="s">target/review</span>&quot;).unwrap();
            std::fs::write(&quot;<span class="s">target/review/ambient-contact.pb</span>&quot;, source.pb_dumps()).unwrap();
        }
        <span class="k">let</span> <span class="k">mut</span> scene = Scene::new();
        scene.add_file(FileDoc {
            name: &quot;<span class="s">contact</span>&quot;.into(),
            session: Rc::new(source),
            place: Xform::identity(),
            point_px: <span class="s">0</span>.<span class="s">0</span>,
            display_only: <span class="s">false</span>,
        });
        scene.upload_to(&amp;<span class="k">mut</span> gpu);
        <span class="k">let</span> <span class="k">mut</span> camera = Camera::new();
        camera.fit(&amp;gpu.bounds, <span class="s">1</span>.<span class="s">0</span>);
        <span class="k">let</span> rebase = gpu.rebase_anchor(&amp;camera.origin(), camera.distance_world(), <span class="s">0</span>.<span class="s">0</span>);
        <span class="k">for</span> (samples, perspective) <span class="k">in</span> [(<span class="s">1</span>, <span class="s">true</span>), (<span class="s">4</span>, <span class="s">true</span>), (<span class="s">1</span>, <span class="s">false</span>), (<span class="s">4</span>, <span class="s">false</span>)] {
            camera.perspective = perspective;
            <span class="k">let</span> input = FrameInput {
                view_proj: camera.view_proj_anchored(<span class="s">1</span>.<span class="s">0</span>, &amp;rebase.anchor),
                clear: wgpu::Color::WHITE,
                now_ms: <span class="s">0</span>.<span class="s">0</span>,
            };
            gpu.view.msaa_forced = Some(samples);
            gpu.resize(<span class="s">256</span>, <span class="s">256</span>);
            gpu.view.ssao = <span class="s">false</span>;
            <span class="k">let</span> plain = gpu.render_offscreen(&amp;input);
            <span class="k">let</span> memory = gpu.allocated_bytes();
            gpu.view.ssao = <span class="s">true</span>;
            <span class="k">let</span> shaded = gpu.render_offscreen(&amp;input);
            <span class="k">if</span> std::env::var_os(&quot;<span class="s">VIEWER_AO_CAPTURE</span>&quot;).is_some() {
                std::fs::write(format!(&quot;<span class="s">target/review/ambient-</span>{<span class="s">samples</span>}<span class="s">x-off.rgba</span>&quot;), &amp;plain)
                    .unwrap();
                std::fs::write(format!(&quot;<span class="s">target/review/ambient-</span>{<span class="s">samples</span>}<span class="s">x-on.rgba</span>&quot;), &amp;shaded)
                    .unwrap();
            }
            <span class="k">let</span> ground_shadow = plain
                .chunks_exact(<span class="s">4</span>)
                .zip(shaded.chunks_exact(<span class="s">4</span>))
                .filter(|(a, b)| {
                    a[<span class="s">0</span>] == <span class="s">255</span> &amp;&amp; a[<span class="s">1</span>] == <span class="s">255</span> &amp;&amp; a[<span class="s">2</span>] == <span class="s">255</span> &amp;&amp; b[<span class="s">0</span>] &lt; shaded[<span class="s">0</span>].saturating_sub(<span class="s">3</span>)
                })
                .count();
            assert!(
                ground_shadow &gt; <span class="s">20</span>,
                &quot;<span class="s">virtual ground receives soft shadows: </span>{<span class="s">ground_shadow</span>}&quot;
            );
            <span class="k">let</span> <span class="k">mut</span> distant_ground = <span class="s">0</span>;
            <span class="k">for</span> (i, (a, b)) <span class="k">in</span> plain
                .chunks_exact(<span class="s">4</span>)
                .zip(shaded.chunks_exact(<span class="s">4</span>))
                .enumerate()
            {
                <span class="k">if</span> a[..<span class="s">3</span>] != [<span class="s">255</span>, <span class="s">255</span>, <span class="s">255</span>] {
                    <span class="k">continue</span>;
                }
                <span class="k">let</span> (origin, direction) = camera
                    .ray(
                        ((i % <span class="s">256</span>) <span class="k">as</span> f64 + <span class="s">0</span>.<span class="s">5</span>, (i / <span class="s">256</span>) <span class="k">as</span> f64 + <span class="s">0</span>.<span class="s">5</span>),
                        (<span class="s">256</span>.<span class="s">0</span>, <span class="s">256</span>.<span class="s">0</span>),
                    )
                    .unwrap();
                <span class="k">if</span> direction[<span class="s">2</span>] &gt;= <span class="s">0</span>.<span class="s">0</span> {
                    <span class="k">continue</span>;
                }
                <span class="k">let</span> t = (-<span class="s">5</span>.<span class="s">0</span> - origin[<span class="s">2</span>]) / direction[<span class="s">2</span>];
                <span class="k">let</span> x = origin[<span class="s">0</span>] + t * direction[<span class="s">0</span>];
                <span class="k">let</span> y = origin[<span class="s">1</span>] + t * direction[<span class="s">1</span>];
                <span class="k">let</span> distance = (x.abs() - <span class="s">50</span>.<span class="s">0</span>).max(<span class="s">0</span>.<span class="s">0</span>).hypot((y.abs() - <span class="s">50</span>.<span class="s">0</span>).max(<span class="s">0</span>.<span class="s">0</span>));
                <span class="k">if</span> t &gt; <span class="s">0</span>.<span class="s">0</span> &amp;&amp; distance &gt; <span class="s">25</span>.<span class="s">0</span> {
                    distant_ground += <span class="s">1</span>;
                    assert!(
                        b[<span class="s">0</span>] &gt;= shaded[<span class="s">0</span>].saturating_sub(<span class="s">2</span>),
                        &quot;<span class="s">ground halo away from base at (</span>{<span class="s">x</span>}<span class="s">, </span>{<span class="s">y</span>}<span class="s">), </span>{<span class="s">samples</span>}<span class="s">x, perspective=</span>{<span class="s">perspective</span>}<span class="s">: </span>{}&quot;,
                        b[<span class="s">0</span>]
                    );
                }
            }
            assert!(
                distant_ground &gt; <span class="s">1000</span>,
                &quot;<span class="s">check exposed ground around raised geometry</span>&quot;
            );
            <span class="k">let</span> darkened = plain
                .chunks_exact(<span class="s">4</span>)
                .zip(shaded.chunks_exact(<span class="s">4</span>))
                .filter(|(a, b)| a[<span class="s">0</span>] &gt; b[<span class="s">0</span>].saturating_add(<span class="s">2</span>))
                .count();
            assert!(
                darkened &gt; <span class="s">20</span>,
                &quot;<span class="s">contact occlusion changes pixels at </span>{<span class="s">samples</span>}<span class="s">x: </span>{<span class="s">darkened</span>}&quot;
            );
            assert_eq!(
                gpu.allocated_bytes(),
                (memory.<span class="s">0</span> + <span class="s">144</span>, memory.<span class="s">1</span> + <span class="s">4</span> * <span class="s">256</span> * <span class="s">256</span>)
            );
            gpu.view.ssao = <span class="s">false</span>;
            assert_eq!(gpu.render_offscreen(&amp;input), plain);
            assert_eq!(gpu.allocated_bytes(), memory);
        }
    }

    #[test]
    <span class="c">/// The shader compiles at 1x and 4x.</span>
    <span class="k">fn</span> shader_validates_for_both_depth_sample_counts() {
        <span class="k">for</span> samples <span class="k">in</span> [<span class="s">1</span>, <span class="s">4</span>] {
            <span class="k">let</span> module = naga::front::wgsl::parse_str(&amp;super::shader_source(samples)).unwrap();
            naga::valid::Validator::new(
                naga::valid::ValidationFlags::all(),
                naga::valid::Capabilities::all(),
            )
            .validate(&amp;module)
            .unwrap();
        }
    }
}</code></pre></div>
<h2 id="step-5-srcshadersscenewgsl">Step 5 · src/shaders/scene.wgsl<a class="anchor" href="#/course/33-contact-shadows#step-5-srcshadersscenewgsl" aria-label="Link to this section">#</a></h2>
<p>The shared scene layout names the new radius field.</p>
<p><code>lessons/33/src/shaders/scene.wgsl</code> · edit · type this</p>
<p>Replaces the line <code>_pad0: f32,</code> in <code>lessons/32/src/shaders/scene.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    ao_radius: <span class="k">f32</span>,<span class="c"> // SSAO contact radius, world units</span></code></pre></div>
<h2 id="step-6-srcshadersproject_triangleswgsl">Step 6 · src/shaders/project_triangles.wgsl<a class="anchor" href="#/course/33-contact-shadows#step-6-srcshadersproject_triangleswgsl" aria-label="Link to this section">#</a></h2>
<p>The projection pass writes each triangle&#39;s contact radius next to its depth slope.</p>
<p><code>lessons/33/src/shaders/project_triangles.wgsl</code> · edit · type this</p>
<p>Replaces the line <code>_pad0: f32,</code> in <code>lessons/32/src/shaders/project_triangles.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    ao_radius: <span class="k">f32</span>,<span class="c"> // SSAO contact radius</span></code></pre></div>
<p>Replaces the line <code>out.gradient = vec4&lt;f32&gt;(gradient, nearest, 0.0);</code> in <code>lessons/32/src/shaders/project_triangles.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> row = physical_objects[physical_indices[index*<span class="s">3u</span>]];
        out.gradient = <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(gradient, nearest, instances[row].ao_radius);</code></pre></div>
<h2 id="step-7-srcshadersssaowgsl">Step 7 · src/shaders/ssao.wgsl<a class="anchor" href="#/course/33-contact-shadows#step-7-srcshadersssaowgsl" aria-label="Link to this section">#</a></h2>
<p>Replace the whole file: the shader reconstructs positions from depth and softens the contact by each object&#39;s radius.</p>
<p><code>lessons/33/src/shaders/ssao.wgsl</code> · replace the whole file · type this</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// Uniform: camera matrices, ground height, radius, canvas size.</span>
<span class="k">struct</span> Ambient {
    inverse_mvp: <span class="k">mat4x4</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // clip space back to world</span>
    mvp: <span class="k">mat4x4</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // camera matrix</span>
    params: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // ground z, largest contact radius, canvas width, height</span>
};
@group(<span class="s">0</span>) @binding(<span class="s">0</span>) <span class="k">var</span> depth: texture_depth_2d;<span class="c"> // scene depth</span>
@group(<span class="s">0</span>) @binding(<span class="s">1</span>) <span class="k">var</span>&lt;<span class="k">uniform</span>&gt; ambient: Ambient;<span class="c"> // settings</span>
@group(<span class="s">0</span>) @binding(<span class="s">2</span>) <span class="k">var</span> physical: texture_2d&lt;<span class="k">f32</span>&gt;;<span class="c"> // depth slope and triangle id per pixel</span>
<span class="c">// Projected triangles, six vec4 each; word 4.w is the contact radius.</span>
@group(<span class="s">0</span>) @binding(<span class="s">3</span>) <span class="k">var</span>&lt;<span class="k">storage</span>, <span class="k">read</span>&gt; triangles: <span class="k">array</span>&lt;<span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;&gt;;
@group(<span class="s">1</span>) @binding(<span class="s">0</span>) <span class="k">var</span> occlusion: texture_2d&lt;<span class="k">f32</span>&gt;;<span class="c"> // occlusion from the previous pass</span>

<span class="c">// Fullscreen triangle.</span>
@vertex <span class="k">fn</span> vs_main(@builtin(vertex_index) i: <span class="k">u32</span>) -&gt; @builtin(position) <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt; {
    <span class="k">let</span> p = <span class="k">array</span>&lt;<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;, <span class="s">3</span>&gt;(<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(-<span class="s">1.0</span>,-<span class="s">1.0</span>), <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">3.0</span>,-<span class="s">1.0</span>), <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(-<span class="s">1.0</span>,<span class="s">3.0</span>));
    <span class="k">return</span> <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(p[i],<span class="s">0.0</span>,<span class="s">1.0</span>);
}
<span class="c">// World position of a pixel at depth \`z\`.</span>
<span class="k">fn</span> world(pixel: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;, z: <span class="k">f32</span>) -&gt; <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt; {
    <span class="k">let</span> uv = pixel / ambient.params.zw;
    <span class="k">let</span> p = ambient.inverse_mvp * <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(uv.x*<span class="s">2.0</span>-<span class="s">1.0</span>, <span class="s">1.0</span>-uv.y*<span class="s">2.0</span>, z, <span class="s">1.0</span>);
    <span class="k">return</span> p.xyz / p.w;
}
<span class="c">// Scene depth at a pixel, clamped to the canvas.</span>
<span class="k">fn</span> depth_sample(pixel: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;, sample: <span class="k">i32</span>) -&gt; <span class="k">f32</span> {
    <span class="k">let</span> xy = clamp(<span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;(pixel),<span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;(<span class="s">0</span>),<span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;(ambient.params.zw)-<span class="s">1</span>);
    <span class="k">return</span> textureLoad(depth, xy, sample);
}
<span class="c">// Scene depth at sample 0.</span>
<span class="k">fn</span> depth_at(pixel: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;) -&gt; <span class="k">f32</span> { <span class="k">return</span> depth_sample(pixel,<span class="s">0</span>); }
<span class="c">// Contact radius of the object under a pixel, or 0.</span>
<span class="k">fn</span> contact_radius(pixel: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;, sample: <span class="k">i32</span>) -&gt; <span class="k">f32</span> {
    <span class="k">let</span> xy = clamp(<span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;(pixel),<span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;(<span class="s">0</span>),<span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;(ambient.params.zw)-<span class="s">1</span>);
    <span class="k">let</span> packed = pack2x16float(textureLoad(physical,xy,sample).zw);
    <span class="k">let</span> words = <span class="k">vec2</span>&lt;<span class="k">u32</span>&gt;(packed&amp;<span class="s">0xffffu</span>,packed&gt;&gt;<span class="s">16u</span>);
    <span class="k">if</span> any(words&lt;<span class="k">vec2</span>&lt;<span class="k">u32</span>&gt;(<span class="s">0x400u</span>)) { <span class="k">return</span> <span class="s">0.0</span>; }
    <span class="k">let</span> address = words-<span class="k">vec2</span>&lt;<span class="k">u32</span>&gt;(<span class="s">0x400u</span>);
    <span class="k">let</span> primitive = address.x|(address.y&lt;&lt;<span class="s">14u</span>);
    <span class="k">if</span> primitive==<span class="s">0u</span> || primitive&gt;arrayLength(&amp;triangles)/<span class="s">6u</span> { <span class="k">return</span> <span class="s">0.0</span>; }
    <span class="k">return</span> triangles[(primitive-<span class="s">1u</span>)*<span class="s">6u</span>+<span class="s">4u</span>].w;
}
<span class="c">// world point under a pixel; w = 0 for none</span>
<span class="k">fn</span> surface(pixel: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;, sample: <span class="k">i32</span>) -&gt; <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt; {
    <span class="k">let</span> z = depth_sample(pixel,sample);
    <span class="k">if</span> z &gt; <span class="s">0.0</span> { <span class="k">return</span> <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(world(pixel,z),<span class="s">1.0</span>); }
    <span class="k">let</span> near = world(pixel,<span class="s">1.0</span>);
    <span class="k">let</span> direction = world(pixel,<span class="s">0.5</span>)-near;
    <span class="k">if</span> direction.z &gt;= -1e-<span class="s">7</span> || near.z &lt; ambient.params.x { <span class="k">return</span> <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>); }
    <span class="k">let</span> t = (ambient.params.x-near.z)/direction.z;
    <span class="k">return</span> <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(near+direction*t,<span class="s">1.0</span>);
}
<span class="c">// Surface normal at a pixel, from the stored slope or neighbouring pixels.</span>
<span class="k">fn</span> normal_at(pixel: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;, p: <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;, sample: <span class="k">i32</span>) -&gt; <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt; {
    <span class="k">let</span> z = depth_sample(pixel,sample);
    <span class="k">if</span> z == <span class="s">0.0</span> { <span class="k">return</span> <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>,<span class="s">0.0</span>,<span class="s">1.0</span>); }
    <span class="k">let</span> encoded = textureLoad(physical,<span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;(pixel),sample).xy;
<span class="c">    // stored slope available</span>
    <span class="k">if</span> all(abs(encoded)&lt;<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">65504.0</span>)) {
        <span class="k">let</span> gradient = encoded/<span class="s">65536.0</span>;
        <span class="k">let</span> dx = world(pixel+<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">1.0</span>,<span class="s">0.0</span>),z+gradient.x)-p;
        <span class="k">let</span> dy = world(pixel+<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>,<span class="s">1.0</span>),z+gradient.y)-p;
        <span class="k">let</span> cross_n = cross(dx,dy);
        <span class="k">if</span> dot(cross_n,cross_n)&gt;1e-<span class="s">24</span> {
            <span class="k">let</span> n = normalize(cross_n);
            <span class="k">return</span> select(-n,n,dot(n,world(pixel,<span class="s">1.0</span>)-p)&gt;<span class="s">0.0</span>);
        }
    }
    <span class="k">let</span> a = surface(pixel+<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">1.0</span>,<span class="s">0.0</span>),sample);
    <span class="k">let</span> b = surface(pixel-<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">1.0</span>,<span class="s">0.0</span>),sample);
    <span class="k">let</span> c = surface(pixel+<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>,<span class="s">1.0</span>),sample);
    <span class="k">let</span> d = surface(pixel-<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>,<span class="s">1.0</span>),sample);
<span class="c">    // nearest neighbour per axis, so edges do not mix surfaces</span>
    <span class="k">let</span> dx = select(p-b.xyz,a.xyz-p,a.w&gt;<span class="s">0.0</span> &amp;&amp; (b.w==<span class="s">0.0</span> || distance(a.xyz,p)&lt;distance(b.xyz,p)));
    <span class="k">let</span> dy = select(p-d.xyz,c.xyz-p,c.w&gt;<span class="s">0.0</span> &amp;&amp; (d.w==<span class="s">0.0</span> || distance(c.xyz,p)&lt;distance(d.xyz,p)));
    <span class="k">let</span> cross_n = cross(dx,dy);
    <span class="k">let</span> n = cross_n / max(length(cross_n),1e-<span class="s">12</span>);
    <span class="k">return</span> select(-n,n,dot(n,world(pixel,<span class="s">1.0</span>)-p)&gt;<span class="s">0.0</span>);
}
<span class="c">// Occlusion texture size: the canvas, at most 1920 px wide.</span>
<span class="k">fn</span> ao_size() -&gt; <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt; {
    <span class="k">let</span> scale = min(<span class="s">1.0</span>,<span class="s">1920.0</span>/max(ambient.params.z,ambient.params.w));
    <span class="k">return</span> ceil(ambient.params.zw*scale);
}
<span class="c">// Shadow on the virtual ground from geometry just above it, 0..0.65.</span>
<span class="k">fn</span> ground_contact(at: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;, p: <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;, noise: <span class="k">f32</span>) -&gt; <span class="k">f32</span> {
    <span class="k">let</span> clip = ambient.mvp*<span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(p,<span class="s">1.0</span>);
    <span class="k">let</span> pixel_world = distance(world(at+<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">1.0</span>,<span class="s">0.0</span>),clip.z/clip.w),p);
    <span class="k">let</span> search_radius = ambient.params.y*<span class="s">6.0</span>;
    <span class="k">let</span> radius_px = min(search_radius/max(pixel_world,1e-<span class="s">6</span>),<span class="s">64.0</span>);
    <span class="k">var</span> sum = <span class="s">0.0</span>;
<span class="c">    // 16 directions, 8 steps each</span>
    <span class="k">for</span> (<span class="k">var</span> direction=<span class="s">0u</span>; direction&lt;<span class="s">16u</span>; direction++) {
        <span class="k">let</span> angle = (f32(direction)+noise)*<span class="s">0.39269908</span>;
        <span class="k">let</span> axis = <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(cos(angle),sin(angle));
        <span class="k">let</span> jitter = fract(noise+f32(direction)*<span class="s">0.618034</span>);
        <span class="k">var</span> horizon = <span class="s">0.0</span>;
        <span class="k">for</span> (<span class="k">var</span> step=<span class="s">0u</span>; step&lt;<span class="s">8u</span>; step++) {
            <span class="k">let</span> fraction = (f32(step)+<span class="s">0.5</span>+jitter*<span class="s">0.5</span>)/<span class="s">8.0</span>;
            <span class="k">let</span> qxy = floor(at+axis*max(<span class="s">1.5</span>,radius_px*fraction*fraction))+<span class="s">0.5</span>;
            <span class="k">if</span> any(qxy&lt;<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>)) || any(qxy&gt;=ambient.params.zw) { <span class="k">continue</span>; }
            <span class="k">let</span> z = depth_at(qxy);
            <span class="k">if</span> z==<span class="s">0.0</span> { <span class="k">continue</span>; }
            <span class="k">let</span> radius = contact_radius(qxy,<span class="s">0</span>)*<span class="s">6.0</span>;
            <span class="k">if</span> radius&lt;=<span class="s">0.0</span> { <span class="k">continue</span>; }
            <span class="k">let</span> delta = world(qxy,z)-p;
            <span class="k">let</span> span = length(delta);
            <span class="k">let</span> height = delta.z;
            <span class="k">if</span> height&lt;=<span class="s">0.0</span> || height&gt;=radius { <span class="k">continue</span>; }
            <span class="k">let</span> elevation = max(height/max(span,1e-<span class="s">6</span>)-<span class="s">0.04</span>,<span class="s">0.0</span>);
            <span class="k">let</span> radial = <span class="s">1.0</span>-smoothstep(radius*<span class="s">0.1</span>,radius,length(delta.xy));
            <span class="k">let</span> vertical = <span class="s">1.0</span>-smoothstep(<span class="s">0.0</span>,radius,height);
            <span class="k">let</span> weight = radial*vertical*vertical;
            horizon = max(horizon,elevation*weight);
        }
        sum += horizon;
    }
    <span class="k">return</span> clamp(sum/<span class="s">16.0</span>*<span class="s">2.6</span>,<span class="s">0.0</span>,<span class="s">0.65</span>);
}
<span class="c">// Occlusion at one pixel: ground shadow, or hemisphere samples on geometry.</span>
@fragment <span class="k">fn</span> fs_main(@builtin(position) pixel: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;) -&gt; @location(<span class="s">0</span>) <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt; {
    <span class="k">let</span> at = floor(pixel.xy/ao_size()*ambient.params.zw)+<span class="s">0.5</span>;
    <span class="k">let</span> center = surface(at,<span class="s">0</span>);
    <span class="k">if</span> center.w == <span class="s">0.0</span> { <span class="k">return</span> <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>); }
    <span class="k">let</span> p = center.xyz;
    <span class="k">let</span> n = normal_at(at,p,<span class="s">0</span>);
    <span class="k">let</span> ground = depth_at(at)==<span class="s">0.0</span>;
<span class="c">    // per-pixel noise so the sample pattern does not tile</span>
    <span class="k">let</span> noise = fract(<span class="s">52.9829189</span>*fract(dot(floor(pixel.xy),<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.06711056</span>,<span class="s">0.00583715</span>))));
    <span class="k">if</span> ground { <span class="k">return</span> <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(ground_contact(at,p,noise),<span class="s">0.0</span>,<span class="s">0.0</span>,<span class="s">1.0</span>); }
    <span class="k">let</span> local_radius = contact_radius(at,<span class="s">0</span>);
    <span class="k">if</span> local_radius&lt;=<span class="s">0.0</span> { <span class="k">return</span> <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>); }
<span class="c">    // tangent frame around the normal</span>
    <span class="k">let</span> up = select(<span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>,<span class="s">1.0</span>,<span class="s">0.0</span>),<span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(<span class="s">1.0</span>,<span class="s">0.0</span>,<span class="s">0.0</span>),abs(n.y)&gt;<span class="s">0.9</span>);
    <span class="k">let</span> tangent = normalize(cross(up,n));
    <span class="k">let</span> bitangent = cross(n,tangent);
    <span class="k">let</span> view = normalize(world(at,<span class="s">1.0</span>)-p);
    <span class="k">let</span> bias = local_radius*<span class="s">0.025</span>*(<span class="s">1.0</span>+<span class="s">3.0</span>*(<span class="s">1.0</span>-max(dot(n,view),<span class="s">0.0</span>)));
    <span class="k">var</span> near_occ = <span class="s">0.0</span>;
    <span class="k">var</span> far_occ = <span class="s">0.0</span>;
<span class="c">    // 64 near samples, then 64 far ones</span>
    <span class="k">for</span> (<span class="k">var</span> i=<span class="s">0u</span>; i&lt;<span class="s">128u</span>; i++) {
        <span class="k">let</span> j = f32(i%<span class="s">64u</span>);
        <span class="k">let</span> far = i&gt;=<span class="s">64u</span>;
        <span class="k">let</span> radius = local_radius*select(<span class="s">1.0</span>,<span class="s">8.0</span>,far);
        <span class="k">let</span> angle = j*<span class="s">2.39996323</span>+noise*<span class="s">6.2831853</span>;
        <span class="k">let</span> z = <span class="s">0.2</span>+<span class="s">0.8</span>*(j+<span class="s">0.5</span>)/<span class="s">64.0</span>;
        <span class="k">let</span> xy = sqrt(<span class="s">1.0</span>-z*z);
        <span class="k">let</span> direction = tangent*(cos(angle)*xy)+bitangent*(sin(angle)*xy)+n*z;
<span class="c">        // sample distance, shuffled against the angle</span>
        <span class="k">let</span> fraction = (f32((i*<span class="s">23u</span>)%<span class="s">64u</span>)+<span class="s">0.5</span>)/<span class="s">64.0</span>;
        <span class="k">let</span> distance = radius*mix(select(<span class="s">0.1</span>,<span class="s">0.25</span>,far),<span class="s">1.0</span>,fraction*fraction);
        <span class="k">let</span> probe = p+direction*distance;
        <span class="k">let</span> clip = ambient.mvp*<span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(probe,<span class="s">1.0</span>);
        <span class="k">if</span> clip.w&lt;=<span class="s">0.0</span> { <span class="k">continue</span>; }
        <span class="k">let</span> uv = <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(clip.x/clip.w*<span class="s">0.5</span>+<span class="s">0.5</span>,<span class="s">0.5</span>-clip.y/clip.w*<span class="s">0.5</span>);
        <span class="k">if</span> any(uv&lt;<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>)) || any(uv&gt;=<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">1.0</span>)) { <span class="k">continue</span>; }
        <span class="k">let</span> qxy = floor(uv*ambient.params.zw)+<span class="s">0.5</span>;
        <span class="k">let</span> qz = depth_at(qxy);
        <span class="k">if</span> qz==<span class="s">0.0</span> || all(qxy==at) { <span class="k">continue</span>; }
        <span class="k">let</span> q = world(qxy,qz);
        <span class="k">let</span> delta = q-p;
<span class="c">        // blocked when the hit is in front of the probe and above the surface</span>
        <span class="k">let</span> ray = normalize(world(qxy,<span class="s">1.0</span>)-probe);
        <span class="k">let</span> blocked = dot(q-probe,ray)&gt;max(bias,distance*<span class="s">0.015</span>)
            &amp;&amp; dot(delta,n)&gt;length(delta)*<span class="s">0.07</span>+bias;
        <span class="k">let</span> weight = <span class="s">1.0</span>-smoothstep(radius*<span class="s">0.5</span>,radius*<span class="s">2.0</span>,length(delta));
        <span class="k">if</span> blocked {
            <span class="k">if</span> far { far_occ+=weight; } <span class="k">else</span> { near_occ+=weight; }
        }
    }
    <span class="k">return</span> <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(clamp(max(near_occ/<span class="s">64.0</span>,far_occ/<span class="s">64.0</span>*<span class="s">0.75</span>)*<span class="s">1.08</span>,<span class="s">0.0</span>,<span class="s">0.65</span>),<span class="s">0.0</span>,<span class="s">0.0</span>,<span class="s">1.0</span>);
}
<span class="c">// Blur along x, keeping edges.</span>
@fragment <span class="k">fn</span> fs_filter_x(@builtin(position) pixel: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;) -&gt; @location(<span class="s">0</span>) <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt; {
    <span class="k">return</span> <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(reconstruct(floor(pixel.xy/ao_size()*ambient.params.zw)+<span class="s">0.5</span>,<span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;(<span class="s">1</span>,<span class="s">0</span>),<span class="s">0</span>),<span class="s">0.0</span>,<span class="s">0.0</span>,<span class="s">1.0</span>);
}
<span class="c">// Blur along y, keeping edges.</span>
@fragment <span class="k">fn</span> fs_filter_y(@builtin(position) pixel: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;) -&gt; @location(<span class="s">0</span>) <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt; {
    <span class="k">return</span> <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(reconstruct(floor(pixel.xy/ao_size()*ambient.params.zw)+<span class="s">0.5</span>,<span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;(<span class="s">0</span>,<span class="s">1</span>),<span class="s">0</span>),<span class="s">0.0</span>,<span class="s">0.0</span>,<span class="s">1.0</span>);
}
<span class="c">// Darken the scene: black with the occlusion as alpha.</span>
@fragment <span class="k">fn</span> fs_composite(@builtin(position) pixel: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;) -&gt; @location(<span class="s">0</span>) <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt; {
    <span class="k">return</span> <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>,<span class="s">0.0</span>,<span class="s">0.0</span>,reconstruct(pixel.xy,<span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;(<span class="s">0</span>),<span class="s">0</span>));
}
<span class="c">// Occlusion at a pixel, blurred over neighbours on the same surface.</span>
<span class="k">fn</span> reconstruct(pixel: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;, axis: <span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;, depth_index: <span class="k">i32</span>) -&gt; <span class="k">f32</span> {
    <span class="k">let</span> center = surface(pixel,depth_index);
    <span class="k">if</span> center.w == <span class="s">0.0</span> { <span class="k">return</span> <span class="s">0.0</span>; }
    <span class="k">let</span> n = normal_at(pixel,center.xyz,depth_index);
    <span class="k">let</span> geometry = depth_sample(pixel,depth_index)&gt;<span class="s">0.0</span>;
    <span class="k">let</span> local_radius = select(ambient.params.y,contact_radius(pixel,depth_index),geometry);
    <span class="k">let</span> dims = <span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;(textureDimensions(occlusion));
    <span class="k">let</span> source = pixel/ambient.params.zw*<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(dims)-<span class="s">0.5</span>;
    <span class="k">let</span> filtering = any(axis!=<span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;(<span class="s">0</span>));
    <span class="k">let</span> base = <span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;(floor(source+<span class="s">0.5</span>));
    <span class="k">var</span> sum = <span class="s">0.0</span>;
    <span class="k">var</span> weight = <span class="s">0.0</span>;
<span class="c">    // 3x3 block, or 13 along one axis</span>
    <span class="k">for</span> (<span class="k">var</span> i=<span class="s">0</span>; i&lt;select(<span class="s">9</span>,<span class="s">13</span>,filtering); i++) {
        <span class="k">let</span> offset = select(<span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;(i%<span class="s">3</span>-<span class="s">1</span>,i/<span class="s">3</span>-<span class="s">1</span>),axis*(i-<span class="s">6</span>),filtering);
        <span class="k">let</span> q = clamp(base+offset,<span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;(<span class="s">0</span>),dims-<span class="s">1</span>);
        <span class="k">let</span> full = floor((<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(q)+<span class="s">0.5</span>)/<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(dims)*ambient.params.zw)+<span class="s">0.5</span>;
        <span class="k">let</span> sample = surface(full,<span class="s">0</span>);
        <span class="k">let</span> delta = sample.xyz-center.xyz;
        <span class="k">let</span> separation = abs(dot(delta,n));
        <span class="k">let</span> tolerance = max(local_radius*<span class="s">0.024</span>,length(delta)*<span class="s">0.08</span>);
        <span class="k">let</span> distance = <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(q)-source;
        <span class="k">let</span> spatial = exp(-dot(distance,distance)*select(<span class="s">1.5</span>,<span class="s">0.04</span>,filtering));
        <span class="k">let</span> ground_weight = select(
            <span class="s">1.0</span>-smoothstep(<span class="s">0.0</span>,local_radius,length(delta.xy)),<span class="s">1.0</span>,geometry);
        <span class="k">let</span> w = spatial*exp(-separation/max(tolerance,1e-<span class="s">6</span>))*ground_weight;
        <span class="k">let</span> valid = select(<span class="s">0.0</span>,w,sample.w&gt;<span class="s">0.0</span> &amp;&amp; ((depth_at(full)&gt;<span class="s">0.0</span>) == geometry));
        sum += textureLoad(occlusion,q,<span class="s">0</span>).r*valid;
        weight += valid;
    }
    <span class="k">return</span> sum/max(weight,1e-<span class="s">6</span>);
}</code></pre></div>
<h2 id="check">Check<a class="anchor" href="#/course/33-contact-shadows#check" aria-label="Link to this section">#</a></h2>
<p>Run <code>trunk serve</code> in <code>lessons/33/</code> and open <a href="http://127.0.0.1:8770/" target="_blank" rel="noopener">http://127.0.0.1:8770/</a>.</p>
<p>Load a scene with several solids of different sizes; each one sits in a soft shadow proportional to itself, and moving one keeps its shadow.</p>
<h2 id="next">Next<a class="anchor" href="#/course/33-contact-shadows#next" aria-label="Link to this section">#</a></h2>
<p><a href="#/course/34-polyline-options">34 · Polyline options, ordered input and a remembered view</a></p>
`,toc:[{level:2,id:"step-1-srcenginegpuinstancers",text:"Step 1 · src/engine/gpu/instance.rs"},{level:2,id:"step-2-srcenginegpuobjectsrs",text:"Step 2 · src/engine/gpu/objects.rs"},{level:2,id:"step-3-srcenginegpurenderrs",text:"Step 3 · src/engine/gpu/render.rs"},{level:2,id:"step-4-srcenginegpussaors",text:"Step 4 · src/engine/gpu/ssao.rs"},{level:2,id:"step-5-srcshadersscenewgsl",text:"Step 5 · src/shaders/scene.wgsl"},{level:2,id:"step-6-srcshadersproject_triangleswgsl",text:"Step 6 · src/shaders/project_triangles.wgsl"},{level:2,id:"step-7-srcshadersssaowgsl",text:"Step 7 · src/shaders/ssao.wgsl"},{level:2,id:"check",text:"Check"},{level:2,id:"next",text:"Next"}]};export{s as default};
