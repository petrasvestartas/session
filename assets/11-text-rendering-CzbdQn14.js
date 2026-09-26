const s={title:"11 · Text rendering",html:`<h1 id="11-text-rendering">11 · Text rendering<a class="anchor" href="#/course/11-text-rendering#11-text-rendering" aria-label="Link to this section">#</a></h1>
<p>Screen-sized nameplates and a foreshortened scene label appear above the model.</p>
<p><img src="/session/docs/course/docs/illustrations/text-placement.svg" alt="Five placements of one shaped line, and the same label rasterized once per device scale." loading="lazy" decoding="async"></p>
<p>Copy each file from the lesson folder to the path shown.</p>
<p>Copy from <code>lessons/11/</code> (tooling this checkpoint needs but the course does not teach):</p>
<ul>
<li><code>lessons/11/src/text_quality.rs</code></li>
</ul>
<h2 id="step-1-srcenginegputext_platers">Step 1 · src/engine/gpu/text_plate.rs<a class="anchor" href="#/course/11-text-rendering#step-1-srcenginegputext_platers" aria-label="Link to this section">#</a></h2>
<p>New file: the plate behind a nameplate, two triangles per plate, cut to the label&#39;s clip box.</p>
<p><code>lessons/11/src/engine/gpu/text_plate.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! Plates: the box drawn behind a label, with no texture: the shader works out the rounded shape per pixel.</span>
<span class="k">use</span> super::super::buffers::{GpuCtx, GrowBuf, VERTS};
<span class="k">use</span> <span class="k">crate</span>::engine::pipelines::Target;

<span class="c">/// One plate, in framebuffer pixels.</span>
<span class="k">pub</span>(super) <span class="k">struct</span> Rectangle {
    <span class="k">pub</span>(super) bounds: [f32; <span class="s">4</span>], <span class="c">// left, top, right, bottom</span>
    <span class="k">pub</span>(super) clip: [f32; <span class="s">4</span>], <span class="c">// the label's clip box</span>
    <span class="k">pub</span>(super) rounded: bool, <span class="c">// corner radius = half the height: a pill</span>
}

<span class="c">/// Every plate of the frame, in one vertex buffer.</span>
<span class="k">pub</span>(super) <span class="k">struct</span> Plates {
    vertices: GrowBuf, <span class="c">// six per plate: two triangles</span>
    pipeline: wgpu::RenderPipeline,
}

<span class="k">impl</span> Plates {
    <span class="c">/// Starts empty; the buffer grows with the first plates.</span>
    <span class="k">pub</span>(super) <span class="k">fn</span> new(ctx: &amp;GpuCtx, target: Target) -&gt; <span class="k">Self</span> {
        <span class="k">Self</span> {
            <span class="c">// 7 floats = 28 bytes: clip position 2, offset from the center 2, half size 2, radius 1</span>
            vertices: GrowBuf::new(ctx, &quot;<span class="s">text.plates</span>&quot;, <span class="s">28</span>, VERTS),
            pipeline: pipeline(ctx, target),
        }
    }

    <span class="c">/// Rebuild the color pipeline for a new MSAA sample count.</span>
    <span class="k">pub</span>(super) <span class="k">fn</span> retarget(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, target: Target) {
        <span class="k">self</span>.pipeline = pipeline(ctx, target);
    }

    <span class="c">/// Two triangles per plate, cut to its clip box.</span>
    <span class="k">pub</span>(super) <span class="k">fn</span> prepare(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, rectangles: &amp;[Rectangle], size: [u32; <span class="s">2</span>]) {
        <span class="k">self</span>.vertices.reset();
        <span class="k">let</span> <span class="k">mut</span> vertices = Vec::with_capacity(rectangles.len() * <span class="s">6</span>);

        <span class="k">for</span> rectangle <span class="k">in</span> rectangles {
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
                    x - center[<span class="s">0</span>],
                    y - center[<span class="s">1</span>],
                    half[<span class="s">0</span>],
                    half[<span class="s">1</span>],
                    radius,
                ]);
            }
        }

        <span class="k">self</span>.vertices.append(ctx, &amp;vertices);
    }</code></pre></div>
<p><code>lessons/11/src/engine/gpu/text_plate.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Draw before the overlay text, so the glyphs land on top.</span>
    <span class="k">pub</span>(super) <span class="k">fn</span> draw(&amp;<span class="k">self</span>, pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;) -&gt; u32 {
        <span class="k">if</span> <span class="k">self</span>.vertices.is_empty() {
            <span class="k">return</span> <span class="s">0</span>;
        }

        pass.set_pipeline(&amp;<span class="k">self</span>.pipeline);
        pass.set_vertex_buffer(<span class="s">0</span>, <span class="k">self</span>.vertices.buf.slice(..));
        pass.draw(<span class="s">0</span>..<span class="k">self</span>.vertices.len(), <span class="s">0</span>..<span class="s">1</span>);
        <span class="s">1</span>
    }

    <span class="c">/// Forget every rectangle.</span>
    <span class="k">pub</span>(super) <span class="k">fn</span> reset(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">self</span>.vertices.reset();
    }

    <span class="c">/// Forget every rectangle and free the buffer.</span>
    <span class="k">pub</span>(super) <span class="k">fn</span> release(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx) {
        <span class="k">self</span>.vertices.release(ctx);
    }

    <span class="c">/// Bytes reserved by the vertex buffer.</span>
    <span class="k">pub</span>(super) <span class="k">fn</span> allocated_bytes(&amp;<span class="k">self</span>) -&gt; u64 {
        <span class="k">self</span>.vertices.buf.size()
    }
}</code></pre></div>
<p><code>lessons/11/src/engine/gpu/text_plate.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Depth test Always: a plate is never hidden by the scene.</span>
<span class="k">fn</span> pipeline(ctx: &amp;GpuCtx, target: Target) -&gt; wgpu::RenderPipeline {
    <span class="k">let</span> shader = ctx
        .device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(&quot;<span class="s">text plate shader</span>&quot;),
            source: wgpu::ShaderSource::Wgsl(include_str!(&quot;<span class="s">../../shaders/text_plate.wgsl</span>&quot;).into()),
        });
    ctx.device
        .create_render_pipeline(&amp;wgpu::RenderPipelineDescriptor {
            label: Some(&quot;<span class="s">text plates</span>&quot;),
            layout: None,
            vertex: wgpu::VertexState {
                module: &amp;shader,
                entry_point: Some(&quot;<span class="s">vs_main</span>&quot;),
                buffers: &amp;[wgpu::VertexBufferLayout {
                    array_stride: <span class="s">28</span>,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &amp;wgpu::vertex_attr_array![<span class="s">0</span> =&gt; Float32x2, <span class="s">1</span> =&gt; Float32x2, <span class="s">2</span> =&gt; Float32x2, <span class="s">3</span> =&gt; Float32],
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &amp;shader,
                entry_point: Some(&quot;<span class="s">fs_main</span>&quot;),
                targets: &amp;[Some(wgpu::ColorTargetState {
                    format: target.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING), <span class="c">// alpha = edge coverage: a smooth rim</span>
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: Default::default(),
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: Some(<span class="s">false</span>),
                depth_compare: Some(wgpu::CompareFunction::Always),
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: target.samples,
                ..Default::default()
            },
            multiview_mask: None,
            cache: None,
        })
}</code></pre></div>
<h2 id="step-2-srcshaderstext_platewgsl">Step 2 · src/shaders/text_plate.wgsl<a class="anchor" href="#/course/11-text-rendering#step-2-srcshaderstext_platewgsl" aria-label="Link to this section">#</a></h2>
<p>New shader: a rounded-box distance gives each plate a soft edge one pixel wide at any size.</p>
<p><code>lessons/11/src/shaders/text_plate.wgsl</code> · 28 lines · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// What the vertex shader hands the fragment shader.</span>
<span class="k">struct</span> PlateVertex {
    @builtin(position) position: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,
    @location(<span class="s">0</span>) local: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // pixel offset from the plate center</span>
    @location(<span class="s">1</span>) half_size: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // px</span>
    @location(<span class="s">2</span>) radius: <span class="k">f32</span>,<span class="c"> // corner radius, px</span>
}

@vertex
<span class="k">fn</span> vs_main(@location(<span class="s">0</span>) position: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;, @location(<span class="s">1</span>) local: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;,
           @location(<span class="s">2</span>) half_size: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;, @location(<span class="s">3</span>) radius: <span class="k">f32</span>) -&gt; PlateVertex {
    <span class="k">var</span> out: PlateVertex;
    out.position = <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(position, <span class="s">0.0</span>, <span class="s">1.0</span>);<span class="c"> // the CPU already placed the corners in clip space</span>
    out.local = local;
    out.half_size = half_size;
    out.radius = radius;
    <span class="k">return</span> out;
}

<span class="c">// Black; alpha from the signed distance to the rounded box (negative inside, px), faded over one pixel.</span>
@fragment
<span class="k">fn</span> fs_main(in: PlateVertex) -&gt; @location(<span class="s">0</span>) <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt; {
    <span class="k">let</span> radius = min(in.radius, min(in.half_size.x, in.half_size.y));
    <span class="k">let</span> q = abs(in.local) - in.half_size + <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(radius);
    <span class="k">let</span> distance = length(max(q, <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>))) + min(max(q.x, q.y), <span class="s">0.0</span>) - radius;
    <span class="k">let</span> coverage = clamp(<span class="s">0.5</span> - distance / max(fwidth(distance), <span class="s">0.001</span>), <span class="s">0.0</span>, <span class="s">1.0</span>);<span class="c"> // fwidth: change per pixel</span>
    <span class="k">return</span> <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>, <span class="s">0.0</span>, <span class="s">0.0</span>, coverage);
}</code></pre></div>
<h2 id="step-3-srcenginegputext_planers">Step 3 · src/engine/gpu/text_plane.rs<a class="anchor" href="#/course/11-text-rendering#step-3-srcenginegputext_planers" aria-label="Link to this section">#</a></h2>
<p>New file: text lying on a world plane, rasterized once into its own texture and drawn as one quad.</p>
<p><code>lessons/11/src/engine/gpu/text_plane.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! Text lying on a plane in the scene, turning and shrinking with the model like writing on paper.</span>
<span class="k">use</span> super::super::buffers::{GpuCtx, GrowBuf, VERTS};
<span class="k">use</span> super::TextFrame;
<span class="k">use</span> <span class="k">crate</span>::engine::pipelines::Target;
<span class="k">use</span> <span class="k">crate</span>::engine::text::{TextDocument, TextLabel, TextPlacement, TextRun};
<span class="k">use</span> glyphon::{FontSystem, SwashCache, SwashContent};

<span class="c">/// All plane textures together, one byte per texel: two labels at the 4096 x 4096 limit fill it.</span>
<span class="k">const</span> TEXTURE_BUDGET: u64 = <span class="s">32</span> * <span class="s">1024</span> * <span class="s">1024</span>;

<span class="c">/// One label rasterized into its own texture, reused until its text, font or needed sharpness changes.</span>
<span class="k">struct</span> CachedPlane {
    label: TextLabel,
    font_revision: u64,
    em_pixels: u32, <span class="c">// raster sharpness: pixels per em, where 1 em = the font size</span>
    size: [u32; <span class="s">2</span>], <span class="c">// texels</span>
    extent: [f32; <span class="s">4</span>], <span class="c">// the texture's box in font units (the label's own pixels): left, top, right, bottom</span>
    _texture: wgpu::Texture, <span class="c">// \`_\` = never read, only owned: dropping the entry frees the texture</span>
    bind: wgpu::BindGroup,
}

<span class="c">/// Every plane label: cached textures, and this frame's quads.</span>
<span class="k">pub</span>(super) <span class="k">struct</span> Planes {
    cached: Vec&lt;CachedPlane&gt;,
    draws: Vec&lt;(usize, u32)&gt;, <span class="c">// (cache index, first vertex) per label drawn this frame</span>
    vertices: GrowBuf, <span class="c">// six per label: two triangles</span>
    layout: wgpu::BindGroupLayout, <span class="c">// slot 0 texture, slot 1 sampler</span>
    sampler: wgpu::Sampler,
    pipeline: wgpu::RenderPipeline,
    target: Target,
    <span class="k">pub</span>(super) rasterizations: u64, <span class="c">// textures made so far, for the stats</span>
}

<span class="k">impl</span> Planes {</code></pre></div>
<p><code>lessons/11/src/engine/gpu/text_plane.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// The layout and sampler are made once and shared by every label's texture.</span>
    <span class="k">pub</span>(super) <span class="k">fn</span> new(ctx: &amp;GpuCtx, target: Target) -&gt; <span class="k">Self</span> {
        <span class="k">let</span> layout = ctx
            .device
            .create_bind_group_layout(&amp;wgpu::BindGroupLayoutDescriptor {
                label: Some(&quot;<span class="s">world text texture</span>&quot;),
                entries: &amp;[
                    wgpu::BindGroupLayoutEntry {
                        binding: <span class="s">0</span>,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: <span class="s">true</span> }, <span class="c">// Linear needs this</span>
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: <span class="s">false</span>,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: <span class="s">1</span>,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });
        <span class="c">// a sampler reads between texels; Linear blends the nearest four, so a scaled label stays smooth</span>
        <span class="k">let</span> sampler = ctx.device.create_sampler(&amp;wgpu::SamplerDescriptor {
            label: Some(&quot;<span class="s">world text coverage</span>&quot;),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        <span class="k">let</span> pipeline = pipeline(ctx, target, &amp;layout);
        <span class="k">Self</span> {
            cached: Vec::new(),
            draws: Vec::new(),
            vertices: GrowBuf::new(ctx, &quot;<span class="s">world text vertices</span>&quot;, <span class="s">56</span>, VERTS), <span class="c">// 14 floats: clip position 4, uv 2, color 4, clip box 4</span>
            layout,
            sampler,
            pipeline,
            target,
            rasterizations: <span class="s">0</span>,
        }
    }

    <span class="c">/// Rebuild the color pipeline for a new MSAA sample count.</span>
    <span class="k">pub</span>(super) <span class="k">fn</span> retarget(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, target: Target) {
        <span class="k">self</span>.target = target;
        <span class="k">self</span>.pipeline = pipeline(ctx, target, &amp;<span class="k">self</span>.layout);
    }</code></pre></div>
<p><code>lessons/11/src/engine/gpu/text_plane.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Rasterize new, changed or too blurry labels, then place every label's quad.</span>
    <span class="k">pub</span>(super) <span class="k">fn</span> prepare(
        &amp;<span class="k">mut</span> <span class="k">self</span>,
        ctx: &amp;GpuCtx,
        document: &amp;<span class="k">mut</span> TextDocument,
        raster: &amp;<span class="k">mut</span> SwashCache,
        frame: &amp;TextFrame,
    ) -&gt; anyhow::Result&lt;()&gt; {
        <span class="k">self</span>.draws.clear();
        <span class="k">self</span>.vertices.reset();
        <span class="k">let</span> <span class="k">mut</span> retained = Vec::new();

        <span class="c">// keep textures whose label still exists</span>
        <span class="k">for</span> cached <span class="k">in</span> <span class="k">self</span>.cached.drain(..) {
            <span class="k">for</span> run <span class="k">in</span> &amp;document.runs {
                <span class="k">if</span> run.label.id == cached.label.id
                    &amp;&amp; matches!(run.label.placement, TextPlacement::WorldPlane { .. })
                {
                    retained.push(cached);
                    <span class="k">break</span>;
                }
            }
        }

        <span class="k">self</span>.cached = retained;
        <span class="k">let</span> <span class="k">mut</span> vertices = Vec::new();

        <span class="k">for</span> run <span class="k">in</span> &amp;document.runs {
            <span class="k">if</span> !matches!(run.label.placement, TextPlacement::WorldPlane { .. })
                || run.label.text.is_empty()
            {
                <span class="k">continue</span>;
            }
            <span class="c">// sharpness the label needs at the current zoom</span>
            <span class="k">let</span> em_pixels = raster_em(&amp;run.label, frame);
            <span class="k">let</span> <span class="k">mut</span> index = None;

            <span class="k">for</span> (at, cached) <span class="k">in</span> <span class="k">self</span>.cached.iter().enumerate() {
                <span class="k">if</span> cached.label.id == run.label.id {
                    index = Some(at);
                    <span class="k">break</span>;
                }
            }

            <span class="c">// new label, or text, font or resolution changed</span>
            <span class="k">let</span> rebuild = <span class="k">match</span> index {
                Some(at) =&gt; !same_raster(
                    &amp;<span class="k">self</span>.cached[at],
                    &amp;run.label,
                    document.font_revision,
                    em_pixels,
                ),
                None =&gt; <span class="s">true</span>,
            };

            <span class="k">if</span> rebuild {
                <span class="k">let</span> (pixels, size, extent) =
                    rasterize(run, &amp;<span class="k">mut</span> document.fonts, raster, em_pixels)?;
                <span class="c">// bytes of every texture once this one is added</span>
                <span class="k">let</span> <span class="k">mut</span> allocated = u64::from(size[<span class="s">0</span>]) * u64::from(size[<span class="s">1</span>]);

                <span class="k">for</span> (at, cached) <span class="k">in</span> <span class="k">self</span>.cached.iter().enumerate() {
                    <span class="k">if</span> Some(at) != index {
                        allocated += u64::from(cached.size[<span class="s">0</span>]) * u64::from(cached.size[<span class="s">1</span>]);
                    }
                }

                anyhow::ensure!(
                    allocated &lt;= TEXTURE_BUDGET,
                    &quot;<span class="s">world text coverage exceeds 32 MiB budget</span>&quot;
                );</code></pre></div>
<p><code>lessons/11/src/engine/gpu/text_plane.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                <span class="k">let</span> texture = ctx.device.create_texture(&amp;wgpu::TextureDescriptor {
                    label: Some(&quot;<span class="s">world text coverage</span>&quot;),
                    size: wgpu::Extent3d {
                        width: size[<span class="s">0</span>],
                        height: size[<span class="s">1</span>],
                        depth_or_array_layers: <span class="s">1</span>,
                    },
                    mip_level_count: <span class="s">1</span>,
                    sample_count: <span class="s">1</span>,
                    dimension: wgpu::TextureDimension::D2,
                    format: wgpu::TextureFormat::R8Unorm, <span class="c">// one byte of coverage; color comes from the vertex</span>
                    usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST, <span class="c">// COPY_DST lets write_texture fill it</span>
                    view_formats: &amp;[],
                });
                ctx.queue.write_texture(
                    texture.as_image_copy(),
                    &amp;pixels,
                    wgpu::TexelCopyBufferLayout {
                        offset: <span class="s">0</span>,
                        bytes_per_row: Some(size[<span class="s">0</span>]),
                        rows_per_image: Some(size[<span class="s">1</span>]),
                    },
                    texture.size(),
                );
                <span class="k">let</span> view = texture.create_view(&amp;Default::default());
                <span class="k">let</span> bind = ctx.device.create_bind_group(&amp;wgpu::BindGroupDescriptor {
                    label: Some(&quot;<span class="s">world text coverage</span>&quot;),
                    layout: &amp;<span class="k">self</span>.layout,
                    entries: &amp;[
                        wgpu::BindGroupEntry {
                            binding: <span class="s">0</span>,
                            resource: wgpu::BindingResource::TextureView(&amp;view),
                        },
                        wgpu::BindGroupEntry {
                            binding: <span class="s">1</span>,
                            resource: wgpu::BindingResource::Sampler(&amp;<span class="k">self</span>.sampler),
                        },
                    ],
                });
                <span class="k">let</span> cached = CachedPlane {
                    label: run.label.clone(),
                    font_revision: document.font_revision,
                    em_pixels,
                    size,
                    extent,
                    _texture: texture,
                    bind,
                };

                <span class="k">match</span> index {
                    Some(at) =&gt; <span class="k">self</span>.cached[at] = cached,
                    None =&gt; {
                        index = Some(<span class="k">self</span>.cached.len());
                        <span class="k">self</span>.cached.push(cached);
                    }
                }

                <span class="k">self</span>.rasterizations += <span class="s">1</span>;

                <span class="c">// past 4096 glyph bitmaps, start the CPU cache over instead of growing forever</span>
                <span class="k">if</span> raster.image_cache.len() &gt; <span class="s">4096</span> {
                    *raster = SwashCache::new();
                }
            }

            <span class="k">let</span> index = index.expect(&quot;<span class="s">a prepared world plane has a cache entry</span>&quot;);
            <span class="k">let</span> start = vertices.len() <span class="k">as</span> u32;
            append_quad(
                &amp;<span class="k">mut</span> vertices,
                &amp;run.label,
                <span class="k">self</span>.cached[index].extent,
                frame,
                <span class="k">self</span>.target.format.is_srgb(),
            );
            <span class="k">self</span>.draws.push((index, start));
        }

        <span class="k">self</span>.vertices.append(ctx, &amp;vertices);
        Ok(())
    }</code></pre></div>
<p><code>lessons/11/src/engine/gpu/text_plane.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// One draw per label: each has its own texture, so each needs its own bind group.</span>
    <span class="k">pub</span>(super) <span class="k">fn</span> draw(&amp;<span class="k">self</span>, pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;) -&gt; u32 {
        <span class="k">if</span> <span class="k">self</span>.draws.is_empty() {
            <span class="k">return</span> <span class="s">0</span>;
        }

        pass.set_pipeline(&amp;<span class="k">self</span>.pipeline);
        pass.set_vertex_buffer(<span class="s">0</span>, <span class="k">self</span>.vertices.buf.slice(..));

        <span class="k">for</span> &amp;(index, start) <span class="k">in</span> &amp;<span class="k">self</span>.draws {
            pass.set_bind_group(<span class="s">0</span>, &amp;<span class="k">self</span>.cached[index].bind, &amp;[]);
            pass.draw(start..start + <span class="s">6</span>, <span class="s">0</span>..<span class="s">1</span>);
        }

        <span class="k">self</span>.draws.len() <span class="k">as</span> u32
    }

    <span class="c">/// Forget every label and free the textures.</span>
    <span class="k">pub</span>(super) <span class="k">fn</span> reset(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">self</span>.cached.clear();
        <span class="k">self</span>.draws.clear();
        <span class="k">self</span>.vertices.reset();
    }

    <span class="c">/// Forget every label and free the buffers too.</span>
    <span class="k">pub</span>(super) <span class="k">fn</span> release(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx) {
        <span class="k">self</span>.reset();
        <span class="k">self</span>.vertices.release(ctx);
    }

    <span class="c">/// Bytes reserved by the vertex buffer.</span>
    <span class="k">pub</span>(super) <span class="k">fn</span> buffer_bytes(&amp;<span class="k">self</span>) -&gt; u64 {
        <span class="k">self</span>.vertices.buf.size()
    }

    <span class="c">/// Bytes of every label texture.</span>
    <span class="k">pub</span>(super) <span class="k">fn</span> texture_bytes(&amp;<span class="k">self</span>) -&gt; u64 {
        <span class="k">let</span> <span class="k">mut</span> total = <span class="s">0</span>;

        <span class="k">for</span> cached <span class="k">in</span> &amp;<span class="k">self</span>.cached {
            total += u64::from(cached.size[<span class="s">0</span>]) * u64::from(cached.size[<span class="s">1</span>]);
        }

        total
    }
}</code></pre></div>
<p><code>lessons/11/src/engine/gpu/text_plane.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// True when the cached texture can still draw the label.</span>
<span class="k">fn</span> same_raster(cached: &amp;CachedPlane, label: &amp;TextLabel, revision: u64, em_pixels: u32) -&gt; bool {
    cached.font_revision == revision
        &amp;&amp; cached.label.text == label.text
        &amp;&amp; cached.label.font_size == label.font_size
        &amp;&amp; cached.label.line_height == label.line_height
        &amp;&amp; cached.em_pixels &gt;= em_pixels <span class="c">// sharper than needed is fine, blurrier is not</span>
}

<span class="c">/// World point to clip space; in perspective w grows with distance, and dividing by it makes far things small.</span>
<span class="k">fn</span> project(world: [f64; <span class="s">3</span>], frame: &amp;TextFrame) -&gt; [f32; <span class="s">4</span>] {
    <span class="k">let</span> point = [
        (world[<span class="s">0</span>] - frame.origin[<span class="s">0</span>]) <span class="k">as</span> f32,
        (world[<span class="s">1</span>] - frame.origin[<span class="s">1</span>]) <span class="k">as</span> f32,
        (world[<span class="s">2</span>] - frame.origin[<span class="s">2</span>]) <span class="k">as</span> f32,
        <span class="s">1</span>.<span class="s">0</span>,
    ];
    <span class="k">let</span> <span class="k">mut</span> clip = [<span class="s">0</span>.<span class="s">0</span>; <span class="s">4</span>];

    <span class="k">for</span> (row, out) <span class="k">in</span> clip.iter_mut().enumerate() {
        <span class="k">for</span> (column, value) <span class="k">in</span> point.iter().enumerate() {
            *out += frame.mvp[column * <span class="s">4</span> + row] * value; <span class="c">// column-major: each column's 4 numbers are adjacent</span>
        }
    }

    clip
}

<span class="c">/// Pixels per em to rasterize at: 32, 64, 128 or 256, so zooming from 40 to 50 px reuses the 64 texture.</span>
<span class="k">fn</span> raster_em(label: &amp;TextLabel, frame: &amp;TextFrame) -&gt; u32 {
    <span class="k">let</span> TextPlacement::WorldPlane {
        world,
        right,
        up,
        world_height,
        ..
    } = label.placement
    <span class="k">else</span> {
        <span class="k">return</span> <span class="s">32</span>;
    };
    <span class="k">let</span> a = project(world, frame);

    <span class="c">// behind the eye</span>
    <span class="k">if</span> a[<span class="s">3</span>] &lt;= <span class="s">0</span>.<span class="s">0</span> {
        <span class="k">return</span> <span class="s">32</span>;
    }

    <span class="k">let</span> <span class="k">mut</span> projected_em = <span class="s">0</span>.<span class="s">0f32</span>;

    <span class="c">// em size on screen along each plane axis</span>
    <span class="k">for</span> direction <span class="k">in</span> [right, up] {
        <span class="k">let</span> <span class="k">mut</span> end = world;

        <span class="k">for</span> axis <span class="k">in</span> <span class="s">0</span>..<span class="s">3</span> {
            end[axis] += direction[axis] * world_height;
        }

        <span class="k">let</span> b = project(end, frame);

        <span class="k">if</span> b[<span class="s">3</span>] &lt;= <span class="s">0</span>.<span class="s">0</span> {
            <span class="k">continue</span>;
        }

        <span class="c">// clip space is 2 wide, so one clip unit is half the framebuffer</span>
        <span class="k">let</span> x = (a[<span class="s">0</span>] / a[<span class="s">3</span>] - b[<span class="s">0</span>] / b[<span class="s">3</span>]) * frame.framebuffer[<span class="s">0</span>] <span class="k">as</span> f32 * <span class="s">0</span>.<span class="s">5</span>;
        <span class="k">let</span> y = (a[<span class="s">1</span>] / a[<span class="s">3</span>] - b[<span class="s">1</span>] / b[<span class="s">3</span>]) * frame.framebuffer[<span class="s">1</span>] <span class="k">as</span> f32 * <span class="s">0</span>.<span class="s">5</span>;
        projected_em = projected_em.max(x.hypot(y));
    }

    <span class="c">// twice the screen size, for sharp edges</span>
    <span class="k">let</span> needed = (projected_em * <span class="s">2</span>.<span class="s">0</span>).clamp(<span class="s">32</span>.<span class="s">0</span>, <span class="s">256</span>.<span class="s">0</span>);
    <span class="k">let</span> <span class="k">mut</span> bucket = <span class="s">32</span>;

    <span class="k">while</span> (bucket <span class="k">as</span> f32) &lt; needed {
        bucket *= <span class="s">2</span>;
    }

    bucket
}</code></pre></div>
<p><code>lessons/11/src/engine/gpu/text_plane.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Paint the glyphs into one coverage image: 0 = empty, 255 = fully inside a letter.</span>
<span class="k">fn</span> rasterize(
    run: &amp;TextRun,
    fonts: &amp;<span class="k">mut</span> FontSystem,
    raster: &amp;<span class="k">mut</span> SwashCache,
    em_pixels: u32,
) -&gt; anyhow::Result&lt;(Vec&lt;u8&gt;, [u32; 2], [f32; 4])&gt; {
    <span class="k">let</span> scale = em_pixels <span class="k">as</span> f32 / run.label.font_size;
    <span class="k">let</span> <span class="k">mut</span> glyphs = Vec::new();
    <span class="k">let</span> <span class="k">mut</span> bounds = [<span class="s">0i32</span>; <span class="s">4</span>];

    <span class="c">// first pass: glyph positions and the box around them</span>
    <span class="k">for</span> line <span class="k">in</span> run.buffer.layout_runs() {
        bounds[<span class="s">2</span>] = bounds[<span class="s">2</span>].max((line.line_w * scale).ceil() <span class="k">as</span> i32);
        bounds[<span class="s">3</span>] = bounds[<span class="s">3</span>].max(((line.line_top + line.line_height) * scale).ceil() <span class="k">as</span> i32);
        anyhow::ensure!(
            bounds[<span class="s">2</span>] &lt;= <span class="s">4092</span> &amp;&amp; bounds[<span class="s">3</span>] &lt;= <span class="s">4092</span>, <span class="c">// early exit; the exact 4096 limit is checked on the final box</span>
            &quot;<span class="s">world text layout exceeds 4096px extent</span>&quot;
        );

        <span class="k">for</span> glyph <span class="k">in</span> line.glyphs {
            <span class="k">let</span> physical = glyph.physical((<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), scale);
            <span class="k">let</span> Some(image) = raster.get_image(fonts, physical.cache_key) <span class="k">else</span> {
                <span class="k">continue</span>;
            };
            <span class="k">let</span> x = physical.x + image.placement.left;
            <span class="k">let</span> y = (line.line_y * scale).round() <span class="k">as</span> i32 + physical.y - image.placement.top;
            bounds[<span class="s">0</span>] = bounds[<span class="s">0</span>].min(x);
            bounds[<span class="s">1</span>] = bounds[<span class="s">1</span>].min(y);
            bounds[<span class="s">2</span>] = bounds[<span class="s">2</span>].max(x + image.placement.width <span class="k">as</span> i32);
            bounds[<span class="s">3</span>] = bounds[<span class="s">3</span>].max(y + image.placement.height <span class="k">as</span> i32);
            glyphs.push((physical.cache_key, x, y));
        }
    }

    bounds[<span class="s">0</span>] -= <span class="s">2</span>; <span class="c">// 2 px of empty border, so the quad's edge samples zero coverage</span>
    bounds[<span class="s">1</span>] -= <span class="s">2</span>;
    bounds[<span class="s">2</span>] += <span class="s">2</span>;
    bounds[<span class="s">3</span>] += <span class="s">2</span>;
    <span class="k">let</span> size = [
        (bounds[<span class="s">2</span>] - bounds[<span class="s">0</span>]) <span class="k">as</span> u32,
        (bounds[<span class="s">3</span>] - bounds[<span class="s">1</span>]) <span class="k">as</span> u32,
    ];
    anyhow::ensure!(
        size[<span class="s">0</span>] &lt;= <span class="s">4096</span> &amp;&amp; size[<span class="s">1</span>] &lt;= <span class="s">4096</span>,
        &quot;<span class="s">world text texture exceeds 4096px extent</span>&quot;
    );
    <span class="k">let</span> <span class="k">mut</span> pixels = vec![<span class="s">0u8</span>; (size[<span class="s">0</span>] * size[<span class="s">1</span>]) <span class="k">as</span> usize];

    <span class="c">// second pass: paint each glyph into the image</span>
    <span class="k">for</span> (key, x, y) <span class="k">in</span> glyphs {
        <span class="k">let</span> Some(image) = raster.get_image(fonts, key) <span class="k">else</span> {
            <span class="k">continue</span>;
        };

        <span class="k">for</span> row <span class="k">in</span> <span class="s">0</span>..image.placement.height {
            <span class="k">for</span> column <span class="k">in</span> <span class="s">0</span>..image.placement.width {
                <span class="k">let</span> source = (row * image.placement.width + column) <span class="k">as</span> usize;
                <span class="c">// swash gives a mask, a color emoji or a subpixel mask: one coverage byte from each</span>
                <span class="k">let</span> coverage = <span class="k">match</span> image.content {
                    SwashContent::Mask =&gt; image.data[source],
                    SwashContent::Color =&gt; image.data[source * <span class="s">4</span> + <span class="s">3</span>],
                    SwashContent::SubpixelMask =&gt; {
                        ((u16::from(image.data[source * <span class="s">4</span>])
                            + u16::from(image.data[source * <span class="s">4</span> + <span class="s">1</span>])
                            + u16::from(image.data[source * <span class="s">4</span> + <span class="s">2</span>]))
                            / <span class="s">3</span>) <span class="k">as</span> u8
                    }
                };
                <span class="k">let</span> target = ((y - bounds[<span class="s">1</span>] + row <span class="k">as</span> i32) <span class="k">as</span> u32 * size[<span class="s">0</span>]
                    + (x - bounds[<span class="s">0</span>] + column <span class="k">as</span> i32) <span class="k">as</span> u32) <span class="k">as</span> usize;
                <span class="k">let</span> previous = u16::from(pixels[target]);
                <span class="c">// overlapping glyphs combine as a + b(1 - a), so coverage never passes 255</span>
                pixels[target] = (previous + u16::from(coverage) * (<span class="s">255</span> - previous) / <span class="s">255</span>) <span class="k">as</span> u8;
            }
        }
    }

    Ok((
        pixels,
        size,
        <span class="c">// back to font units, so the quad does not depend on em_pixels</span>
        [
            bounds[<span class="s">0</span>] <span class="k">as</span> f32 / scale,
            bounds[<span class="s">1</span>] <span class="k">as</span> f32 / scale,
            bounds[<span class="s">2</span>] <span class="k">as</span> f32 / scale,
            bounds[<span class="s">3</span>] <span class="k">as</span> f32 / scale,
        ],
    ))
}</code></pre></div>
<p><code>lessons/11/src/engine/gpu/text_plane.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// The label's quad as six clip-space vertices, projected on the CPU.</span>
<span class="k">fn</span> append_quad(
    vertices: &amp;<span class="k">mut</span> Vec&lt;[f32; 14]&gt;,
    label: &amp;TextLabel,
    extent: [f32; <span class="s">4</span>],
    frame: &amp;TextFrame,
    srgb: bool,
) {
    <span class="k">let</span> TextPlacement::WorldPlane {
        world,
        right,
        up,
        world_height,
    } = label.placement
    <span class="k">else</span> {
        <span class="k">return</span>;
    };
    <span class="c">// world units per font unit</span>
    <span class="k">let</span> unit = world_height / f64::from(label.font_size);
    <span class="k">let</span> <span class="k">mut</span> color = [<span class="s">0</span>.<span class="s">0</span>; <span class="s">4</span>];

    <span class="c">// an sRGB canvas re-encodes what it stores, so it wants linear values: undo the sRGB curve</span>
    <span class="k">for</span> (index, component) <span class="k">in</span> color.iter_mut().enumerate() {
        <span class="k">let</span> value = f32::from(label.color[index]) / <span class="s">255</span>.<span class="s">0</span>;
        *component = <span class="k">if</span> srgb &amp;&amp; index &lt; <span class="s">3</span> {
            <span class="k">if</span> value &lt;= <span class="s">0</span>.<span class="s">04045</span> {
                value / <span class="s">12</span>.<span class="s">92</span>
            } <span class="k">else</span> {
                ((value + <span class="s">0</span>.<span class="s">055</span>) / <span class="s">1</span>.<span class="s">055</span>).powf(<span class="s">2</span>.<span class="s">4</span>)
            }
        } <span class="k">else</span> {
            value
        };
    }

    <span class="k">let</span> scale = frame.framebuffer[<span class="s">0</span>] <span class="k">as</span> f32 / frame.logical[<span class="s">0</span>] <span class="k">as</span> f32; <span class="c">// real pixels per CSS pixel</span>
    <span class="k">let</span> <span class="k">mut</span> bounds = [
        <span class="s">0</span>.<span class="s">0</span>,
        <span class="s">0</span>.<span class="s">0</span>,
        frame.framebuffer[<span class="s">0</span>] <span class="k">as</span> f32,
        frame.framebuffer[<span class="s">1</span>] <span class="k">as</span> f32,
    ];

    <span class="k">if</span> <span class="k">let</span> Some(clip) = label.clip {
        <span class="k">for</span> (index, value) <span class="k">in</span> bounds.iter_mut().enumerate() {
            *value = clip[index] * scale;
        }
    }

    <span class="k">for</span> [u, v] <span class="k">in</span> [
        [<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>],
        [<span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>],
        [<span class="s">1</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>],
        [<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>],
        [<span class="s">1</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>],
        [<span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>],
    ] {
        <span class="k">let</span> x = f64::from(extent[<span class="s">0</span>] + (extent[<span class="s">2</span>] - extent[<span class="s">0</span>]) * u) * unit;
        <span class="k">let</span> y = f64::from(extent[<span class="s">1</span>] + (extent[<span class="s">3</span>] - extent[<span class="s">1</span>]) * v) * unit;
        <span class="k">let</span> <span class="k">mut</span> point = world;

        <span class="k">for</span> axis <span class="k">in</span> <span class="s">0</span>..<span class="s">3</span> {
            point[axis] += right[axis] * x - up[axis] * y; <span class="c">// text y points down, \`up\` points up</span>
        }

        <span class="k">let</span> clip = project(point, frame);
        vertices.push([
            clip[<span class="s">0</span>], clip[<span class="s">1</span>], clip[<span class="s">2</span>], clip[<span class="s">3</span>], u, v, color[<span class="s">0</span>], color[<span class="s">1</span>], color[<span class="s">2</span>], color[<span class="s">3</span>],
            bounds[<span class="s">0</span>], bounds[<span class="s">1</span>], bounds[<span class="s">2</span>], bounds[<span class="s">3</span>],
        ]);
    }
}</code></pre></div>
<p><code>lessons/11/src/engine/gpu/text_plane.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Depth-tested but never written: a solid in front hides the label, and labels never hide each other.</span>
<span class="k">fn</span> pipeline(ctx: &amp;GpuCtx, target: Target, layout: &amp;wgpu::BindGroupLayout) -&gt; wgpu::RenderPipeline {
    <span class="k">let</span> shader = ctx
        .device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(&quot;<span class="s">world text shader</span>&quot;),
            source: wgpu::ShaderSource::Wgsl(include_str!(&quot;<span class="s">../../shaders/text_plane.wgsl</span>&quot;).into()),
        });
    <span class="k">let</span> pipeline_layout = ctx
        .device
        .create_pipeline_layout(&amp;wgpu::PipelineLayoutDescriptor {
            label: Some(&quot;<span class="s">world text</span>&quot;),
            bind_group_layouts: &amp;[Some(layout)], <span class="c">// only the texture: the camera is already in the vertices</span>
            immediate_size: <span class="s">0</span>,
        });
    ctx.device.create_render_pipeline(&amp;wgpu::RenderPipelineDescriptor {
        label: Some(&quot;<span class="s">world text</span>&quot;), layout: Some(&amp;pipeline_layout),
        vertex: wgpu::VertexState { module: &amp;shader, entry_point: Some(&quot;<span class="s">vs_main</span>&quot;), buffers: &amp;[wgpu::VertexBufferLayout { array_stride: <span class="s">56</span>, step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &amp;wgpu::vertex_attr_array![<span class="s">0</span> =&gt; Float32x4, <span class="s">1</span> =&gt; Float32x2, <span class="s">2</span> =&gt; Float32x4, <span class="s">3</span> =&gt; Float32x4] }], compilation_options: Default::default() },
        fragment: Some(wgpu::FragmentState { module: &amp;shader, entry_point: Some(&quot;<span class="s">fs_main</span>&quot;), targets: &amp;[Some(wgpu::ColorTargetState { format: target.format, blend: Some(wgpu::BlendState::ALPHA_BLENDING), write_mask: wgpu::ColorWrites::ALL })], compilation_options: Default::default() }),
        primitive: Default::default(), depth_stencil: Some(wgpu::DepthStencilState { format: wgpu::TextureFormat::Depth32Float, depth_write_enabled: Some(<span class="s">false</span>), depth_compare: Some(wgpu::CompareFunction::GreaterEqual), stencil: Default::default(), bias: Default::default() }),
        multisample: wgpu::MultisampleState { count: target.samples, ..Default::default() }, multiview_mask: None, cache: None,
    })
}</code></pre></div>
<h2 id="step-4-srcshaderstext_planewgsl">Step 4 · src/shaders/text_plane.wgsl<a class="anchor" href="#/course/11-text-rendering#step-4-srcshaderstext_planewgsl" aria-label="Link to this section">#</a></h2>
<p>New shader: read the label&#39;s coverage texture and cut it to the clip box.</p>
<p><code>lessons/11/src/shaders/text_plane.wgsl</code> · 31 lines · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// What the vertex shader hands the fragment shader.</span>
<span class="k">struct</span> Vertex {
    @builtin(position) position: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,
    @location(<span class="s">0</span>) uv: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // 0..1 across the texture</span>
    @location(<span class="s">1</span>) color: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,
    @location(<span class="s">2</span>) @interpolate(flat) clip: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // flat: the box as is, not blended between vertices</span>
}

@group(<span class="s">0</span>) @binding(<span class="s">0</span>) <span class="k">var</span> coverage_texture: texture_2d&lt;<span class="k">f32</span>&gt;;<span class="c"> // Unorm: byte 255 reads as 1.0</span>
@group(<span class="s">0</span>) @binding(<span class="s">1</span>) <span class="k">var</span> coverage_sampler: sampler;

@vertex
<span class="c">// Corners arrive projected, w included, from append_quad.</span>
<span class="k">fn</span> vs_main(@location(<span class="s">0</span>) position: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;, @location(<span class="s">1</span>) uv: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;,
           @location(<span class="s">2</span>) color: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;, @location(<span class="s">3</span>) clip: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;) -&gt; Vertex {
    <span class="k">var</span> out: Vertex;
    out.position = position; out.uv = uv; out.color = color; out.clip = clip;
    <span class="k">return</span> out;
}

<span class="c">// Label color on the glyphs, black around them; alpha is the label's, so the quad is a solid plate.</span>
@fragment
<span class="k">fn</span> fs_main(in: Vertex) -&gt; @location(<span class="s">0</span>) <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt; {
<span class="c">    // in the fragment stage \`position\` is the pixel center in framebuffer pixels, the clip box's units</span>
    <span class="k">if</span> (in.position.x &lt; in.clip.x || in.position.y &lt; in.clip.y || in.position.x &gt;= in.clip.z || in.position.y &gt;= in.clip.w) {
        <span class="k">discard</span>;
    }

    <span class="k">let</span> coverage = textureSample(coverage_texture, coverage_sampler, in.uv).r;
    <span class="k">return</span> <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(in.color.rgb * coverage, in.color.a);
}</code></pre></div>
<h2 id="step-5-srcenginegputextrs">Step 5 · src/engine/gpu/text.rs<a class="anchor" href="#/course/11-text-rendering#step-5-srcenginegputextrs" aria-label="Link to this section">#</a></h2>
<p>New file: the text lane places every label each frame, then draws planes, depth-tested text, plates and overlays.</p>
<p><code>lessons/11/src/engine/gpu/text.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! GPU text with glyphon: each glyph is rasterized once into an atlas, one shared texture, and every letter is a quad sampling it.</span>
<span class="k">use</span> super::buffers::GpuCtx;
<span class="c">// \`#[path]\` loads a sibling file as a private submodule: only this file sees Planes and Plates</span>
#[path = &quot;<span class="s">text_plane.rs</span>&quot;]
<span class="k">mod</span> plane;
#[path = &quot;<span class="s">text_plate.rs</span>&quot;]
<span class="k">mod</span> plate;
<span class="k">use</span> <span class="k">crate</span>::engine::pipelines::Target;
<span class="k">use</span> <span class="k">crate</span>::engine::text::{TextDocument, TextLabel, TextPlacement, TextRun};
<span class="k">use</span> glyphon::{
    Cache, Color, ColorMode, Resolution, SwashCache, TextArea, TextAtlas, TextBounds, TextRenderer,
    Viewport,
};
<span class="k">use</span> serde::Serialize;
<span class="k">use</span> std::collections::{HashMap, HashSet};

<span class="c">/// Camera and canvas for one frame; \`PartialEq\` lets \`prepare\` skip a frame equal to the last one.</span>
#[derive(Clone, Debug, PartialEq)]
<span class="k">pub</span> <span class="k">struct</span> TextFrame {
    <span class="k">pub</span> mvp: [f32; <span class="s">16</span>], <span class="c">// world to clip space, column-major</span>
    <span class="k">pub</span> origin: [f64; <span class="s">3</span>], <span class="c">// subtracted from world points before \`mvp\`, so f32 keeps its precision</span>
    <span class="k">pub</span> framebuffer: [u32; <span class="s">2</span>], <span class="c">// real pixels, e.g. 1600 x 1200</span>
    <span class="k">pub</span> logical: [f64; <span class="s">2</span>], <span class="c">// CSS pixels, e.g. 800 x 600 at scale 2.0</span>
    <span class="k">pub</span> ortho_half_height: f32, <span class="c">// 0 in perspective</span>
}

<span class="c">/// Counters for the diagnostics panel: they answer &quot;why is text slow&quot; and &quot;where did the memory go&quot;.</span>
#[derive(Clone, Debug, Default, Serialize)]
<span class="k">pub</span> <span class="k">struct</span> TextStats {
    <span class="k">pub</span> preparations: u64,
    <span class="k">pub</span> skipped_preparations: u64, <span class="c">// frames where nothing moved</span>
    <span class="k">pub</span> shape_count: u64,
    <span class="k">pub</span> shaping_ms: f64,
    <span class="k">pub</span> requested_glyphs: usize, <span class="c">// this frame</span>
    <span class="k">pub</span> distinct_raster_keys: usize, <span class="c">// raster key = glyph + size + subpixel offset: one atlas entry each</span>
    <span class="k">pub</span> new_raster_keys: usize, <span class="c">// this frame</span>
    <span class="k">pub</span> raster_images: usize, <span class="c">// glyph bitmaps swash keeps on the CPU</span>
    <span class="k">pub</span> new_raster_images: usize, <span class="c">// this frame</span>
    <span class="k">pub</span> raster_image_capacity_bytes: usize,
    <span class="k">pub</span> atlas_resets: u64,
    <span class="k">pub</span> preparation_ms: f64, <span class="c">// the last rebuild</span>
    <span class="k">pub</span> active_instance_bytes: usize, <span class="c">// 28 bytes per drawn glyph</span>
    <span class="k">pub</span> missing_glyphs: usize, <span class="c">// glyph id 0: no font has it, drawn as a box</span>
    <span class="k">pub</span> nameplate_capacity_bytes: u64,
    <span class="k">pub</span> world_plane_buffer_bytes: u64,
    <span class="k">pub</span> world_plane_texture_bytes: u64,
    <span class="k">pub</span> world_plane_rasterizations: u64,
}</code></pre></div>
<p><code>lessons/11/src/engine/gpu/text.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// Overlay = text always on top; anchored = text at a scene depth, hidden behind nearer geometry.</span>
<span class="c">/// Draws every label: planes, anchored text, plates and overlays.</span>
<span class="k">pub</span> <span class="k">struct</span> TextLane {
    <span class="k">pub</span> document: TextDocument, <span class="c">// the labels and their shaped glyphs</span>
    <span class="k">pub</span> stats: TextStats,
    cache: Cache, <span class="c">// glyphon's shaders and layouts, shared by the atlas and both renderers</span>
    atlas: TextAtlas,
    viewport: Viewport, <span class="c">// canvas size, for glyphon's pixel to clip space math</span>
    raster: SwashCache, <span class="c">// swash rasterizes glyph outlines into bitmaps on the CPU</span>
    overlay: TextRenderer,
    anchored: TextRenderer,
    plates: plate::Plates,
    planes: plane::Planes,
    target: Target,
    atlas_font_revision: u64,
    prepared: Option&lt;(u64, u64, TextFrame)&gt;, <span class="c">// key of the last rebuild; the same key skips the next</span>
    raster_keys: HashSet&lt;glyphon::CacheKey&gt;, <span class="c">// since the last atlas reset</span>
    overlay_count: u32, <span class="c">// 0 or 1: nothing to draw, or one draw</span>
    anchored_count: u32,
}

<span class="k">impl</span> TextLane {
    <span class="c">/// Bytes reserved by the plate and plane buffers.</span>
    <span class="k">pub</span> <span class="k">fn</span> allocated_bytes(&amp;<span class="k">self</span>) -&gt; u64 {
        <span class="k">self</span>.plates.allocated_bytes() + <span class="k">self</span>.planes.buffer_bytes()
    }

    <span class="c">/// Bytes of the plane textures.</span>
    <span class="k">pub</span> <span class="k">fn</span> texture_bytes(&amp;<span class="k">self</span>) -&gt; u64 {
        <span class="k">self</span>.planes.texture_bytes()
    }

    <span class="c">/// One atlas feeds both renderers; they differ only in the depth test.</span>
    <span class="k">pub</span> <span class="k">fn</span> new(ctx: &amp;GpuCtx, target: Target) -&gt; <span class="k">Self</span> {
        <span class="k">let</span> cache = Cache::new(&amp;ctx.device);
        <span class="k">let</span> viewport = Viewport::new(&amp;ctx.device, &amp;cache);
        <span class="k">let</span> <span class="k">mut</span> atlas = make_atlas(ctx, &amp;cache, target.format);
        <span class="k">let</span> overlay = renderer(ctx, &amp;<span class="k">mut</span> atlas, target, wgpu::CompareFunction::Always);
        <span class="k">let</span> anchored = renderer(ctx, &amp;<span class="k">mut</span> atlas, target, wgpu::CompareFunction::GreaterEqual);
        <span class="k">Self</span> {
            document: TextDocument::new(),
            stats: TextStats::default(),
            cache,
            atlas,
            viewport,
            raster: SwashCache::new(),
            overlay,
            anchored,
            plates: plate::Plates::new(ctx, target),
            planes: plane::Planes::new(ctx, target),
            target,
            atlas_font_revision: <span class="s">1</span>, <span class="c">// a new document starts at 1, so the first frame keeps this atlas</span>
            prepared: None,
            raster_keys: HashSet::new(),
            overlay_count: <span class="s">0</span>,
            anchored_count: <span class="s">0</span>,
        }
    }</code></pre></div>
<p><code>lessons/11/src/engine/gpu/text.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Replace every label; an invalid set keeps the old ones.</span>
    <span class="k">pub</span> <span class="k">fn</span> set_labels(&amp;<span class="k">mut</span> <span class="k">self</span>, labels: Vec&lt;TextLabel&gt;) -&gt; anyhow::Result&lt;()&gt; {
        <span class="k">self</span>.document.set_labels(labels)
    }

    <span class="c">/// Rebuild the renderers for a new MSAA sample count.</span>
    <span class="k">pub</span> <span class="k">fn</span> retarget(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, target: Target) {
        <span class="k">self</span>.target = target;
        <span class="k">self</span>.plates.retarget(ctx, target);
        <span class="k">self</span>.planes.retarget(ctx, target);
        <span class="k">self</span>.overlay = renderer(ctx, &amp;<span class="k">mut</span> <span class="k">self</span>.atlas, target, wgpu::CompareFunction::Always);
        <span class="k">self</span>.anchored = renderer(
            ctx,
            &amp;<span class="k">mut</span> <span class="k">self</span>.atlas,
            target,
            wgpu::CompareFunction::GreaterEqual,
        );
        <span class="k">self</span>.prepared = None; <span class="c">// the next prepare must run</span>
    }</code></pre></div>
<p><code>lessons/11/src/engine/gpu/text.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Place every label for this frame; skipped when labels, fonts and camera match the last call.</span>
    <span class="k">pub</span> <span class="k">fn</span> prepare(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, frame: &amp;TextFrame) -&gt; anyhow::Result&lt;()&gt; {
        <span class="k">let</span> key = (
            <span class="k">self</span>.document.revision,
            <span class="k">self</span>.document.font_revision,
            frame.clone(),
        );

        <span class="k">if</span> <span class="k">self</span>.prepared.as_ref() == Some(&amp;key) {
            <span class="k">self</span>.stats.skipped_preparations += <span class="s">1</span>;
            <span class="k">return</span> Ok(());
        }

        <span class="k">self</span>.overlay_count = <span class="s">0</span>;
        <span class="k">self</span>.anchored_count = <span class="s">0</span>;
        <span class="k">self</span>.plates.reset();
        <span class="k">let</span> scale = frame.scale()?;
        <span class="k">let</span> start = now_ms();
        <span class="k">let</span> fonts_changed = <span class="k">self</span>.atlas_font_revision != <span class="k">self</span>.document.font_revision;

        <span class="c">// a fresh atlas after a font change, or past 4096 raster keys, so a long zoom cannot grow it forever</span>
        <span class="k">if</span> fonts_changed || <span class="k">self</span>.raster_keys.len() &gt; <span class="s">4096</span> {
            <span class="k">self</span>.rebuild_resources(ctx);
        }

        <span class="k">let</span> raster_images_before = <span class="k">self</span>.raster.image_cache.values().flatten().count();
        <span class="k">self</span>.planes
            .prepare(ctx, &amp;<span class="k">mut</span> <span class="k">self</span>.document, &amp;<span class="k">mut</span> <span class="k">self</span>.raster, frame)?;
        <span class="k">self</span>.atlas.trim(); <span class="c">// glyphs this prepare does not use may now be evicted</span>
        <span class="k">self</span>.viewport.update(
            &amp;ctx.queue,
            Resolution {
                width: frame.framebuffer[<span class="s">0</span>],
                height: frame.framebuffer[<span class="s">1</span>],
            },
        );
        <span class="k">let</span> <span class="k">mut</span> overlays = Vec::new();
        <span class="k">let</span> <span class="k">mut</span> anchors = Vec::new();
        <span class="k">let</span> <span class="k">mut</span> depths = HashMap::new();
        <span class="k">let</span> <span class="k">mut</span> plates = Vec::new();
        <span class="k">self</span>.stats.new_raster_keys = <span class="s">0</span>;
        <span class="k">self</span>.stats.requested_glyphs = <span class="s">0</span>;
        <span class="k">self</span>.stats.missing_glyphs = <span class="s">0</span>;

        <span class="c">// screen position, plate and depth of every label</span>
        <span class="k">for</span> run <span class="k">in</span> &amp;<span class="k">self</span>.document.runs {
            <span class="k">let</span> Some(<span class="k">mut</span> placed) = place(&amp;run.label, frame, scale) <span class="k">else</span> {
                <span class="k">continue</span>;
            };

            <span class="k">if</span> <span class="k">let</span> Some(rectangle) = center_nameplate(run, &amp;<span class="k">mut</span> placed, frame, scale) {
                plates.push(rectangle);
            }

            <span class="c">// what glyphon draws: one shaped buffer at a position, scale and clip box</span>
            <span class="k">let</span> area = TextArea {
                buffer: &amp;run.buffer,
                left: placed.left,
                top: placed.top,
                scale: placed.scale,
                bounds: clip_bounds(&amp;run.label, frame, scale),
                default_color: Color::rgba(
                    run.label.color[<span class="s">0</span>],
                    run.label.color[<span class="s">1</span>],
                    run.label.color[<span class="s">2</span>],
                    run.label.color[<span class="s">3</span>],
                ),
                custom_glyphs: &amp;[],
            };

            <span class="c">// count glyphs and new raster keys, for the stats</span>
            <span class="k">for</span> line <span class="k">in</span> run.buffer.layout_runs() {
                <span class="k">for</span> glyph <span class="k">in</span> line.glyphs {
                    <span class="k">self</span>.stats.requested_glyphs += <span class="s">1</span>;
                    <span class="k">self</span>.stats.missing_glyphs += usize::from(glyph.glyph_id == <span class="s">0</span>);
                    <span class="k">let</span> physical = glyph.physical((placed.left, placed.top), placed.scale);
                    <span class="k">self</span>.stats.new_raster_keys +=
                        usize::from(<span class="k">self</span>.raster_keys.insert(physical.cache_key));
                }
            }

            <span class="k">if</span> <span class="k">let</span> Some(depth) = placed.depth {
                depths.insert(run.label.id <span class="k">as</span> usize, depth);
                anchors.push(area);
            } <span class="k">else</span> {
                overlays.push(area);
            }
        }</code></pre></div>
<p><code>lessons/11/src/engine/gpu/text.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.overlay.prepare(
            &amp;ctx.device,
            &amp;ctx.queue,
            &amp;<span class="k">mut</span> <span class="k">self</span>.document.fonts,
            &amp;<span class="k">mut</span> <span class="k">self</span>.atlas,
            &amp;<span class="k">self</span>.viewport,
            overlays,
            &amp;<span class="k">mut</span> <span class="k">self</span>.raster,
        )?;
        <span class="c">// one call draws every anchored label, so glyphon asks for each label's depth by id</span>
        <span class="k">self</span>.anchored.prepare_with_depth(
            &amp;ctx.device,
            &amp;ctx.queue,
            &amp;<span class="k">mut</span> <span class="k">self</span>.document.fonts,
            &amp;<span class="k">mut</span> <span class="k">self</span>.atlas,
            &amp;<span class="k">self</span>.viewport,
            anchors,
            &amp;<span class="k">mut</span> <span class="k">self</span>.raster,
            |id| depth_for(&amp;depths, id),
        )?;
        <span class="k">self</span>.plates.prepare(ctx, &amp;plates, frame.framebuffer);

        <span class="c">// note whether each renderer has anything to draw</span>
        <span class="k">for</span> run <span class="k">in</span> &amp;<span class="k">self</span>.document.runs {
            <span class="k">if</span> place(&amp;run.label, frame, scale).is_some() &amp;&amp; !run.label.text.is_empty() {
                <span class="k">match</span> run.label.placement {
                    TextPlacement::Screen { .. } | TextPlacement::Nameplate { .. } =&gt; {
                        <span class="k">self</span>.overlay_count = <span class="s">1</span>
                    }
                    _ =&gt; <span class="k">self</span>.anchored_count = <span class="s">1</span>,
                }
            }
        }

        <span class="k">self</span>.stats.raster_images = <span class="s">0</span>;
        <span class="k">self</span>.stats.raster_image_capacity_bytes = <span class="s">0</span>;

        <span class="k">for</span> image <span class="k">in</span> <span class="k">self</span>.raster.image_cache.values().flatten() {
            <span class="k">self</span>.stats.raster_images += <span class="s">1</span>;
            <span class="k">self</span>.stats.raster_image_capacity_bytes += image.data.capacity();
        }

        <span class="k">self</span>.stats.new_raster_images = <span class="k">self</span>
            .stats
            .raster_images
            .saturating_sub(raster_images_before);
        <span class="k">self</span>.stats.preparations += <span class="s">1</span>;
        <span class="k">self</span>.stats.shape_count = <span class="k">self</span>.document.shape_count;
        <span class="k">self</span>.stats.shaping_ms = <span class="k">self</span>.document.shaping_ms;
        <span class="k">self</span>.stats.distinct_raster_keys = <span class="k">self</span>.raster_keys.len();
        <span class="k">self</span>.stats.active_instance_bytes = <span class="k">self</span>.stats.requested_glyphs * <span class="s">28</span>;
        <span class="k">self</span>.stats.nameplate_capacity_bytes = <span class="k">self</span>.plates.allocated_bytes();
        <span class="k">self</span>.stats.world_plane_buffer_bytes = <span class="k">self</span>.planes.buffer_bytes();
        <span class="k">self</span>.stats.world_plane_texture_bytes = <span class="k">self</span>.planes.texture_bytes();
        <span class="k">self</span>.stats.world_plane_rasterizations = <span class="k">self</span>.planes.rasterizations;
        <span class="k">self</span>.stats.preparation_ms = now_ms() - start;
        <span class="k">self</span>.prepared = Some(key);
        Ok(())
    }</code></pre></div>
<p><code>lessons/11/src/engine/gpu/text.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Order matters: planes and depth-tested text first, then plates, then overlay text on the plates.</span>
    <span class="k">pub</span> <span class="k">fn</span> draw(&amp;<span class="k">self</span>, pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;) -&gt; u32 {
        <span class="k">let</span> <span class="k">mut</span> draws = <span class="k">self</span>.planes.draw(pass);

        <span class="k">if</span> <span class="k">self</span>.anchored_count != <span class="s">0</span> {
            <span class="k">match</span> <span class="k">self</span>.anchored.render(&amp;<span class="k">self</span>.atlas, &amp;<span class="k">self</span>.viewport, pass) {
                Ok(()) =&gt; draws += <span class="s">1</span>,
                Err(error) =&gt; log::error!(&quot;<span class="s">anchored text render: </span>{<span class="s">error</span>}&quot;),
            }
        }

        draws += <span class="k">self</span>.plates.draw(pass);

        <span class="k">if</span> <span class="k">self</span>.overlay_count != <span class="s">0</span> {
            <span class="k">match</span> <span class="k">self</span>.overlay.render(&amp;<span class="k">self</span>.atlas, &amp;<span class="k">self</span>.viewport, pass) {
                Ok(()) =&gt; draws += <span class="s">1</span>,
                Err(error) =&gt; log::error!(&quot;<span class="s">overlay text render: </span>{<span class="s">error</span>}&quot;),
            }
        }

        draws
    }

    <span class="c">/// Forget every label.</span>
    <span class="k">pub</span> <span class="k">fn</span> reset(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">self</span>.document.clear();
        <span class="k">self</span>.prepared = None;
        <span class="k">self</span>.overlay_count = <span class="s">0</span>;
        <span class="k">self</span>.anchored_count = <span class="s">0</span>;
        <span class="k">self</span>.plates.reset();
        <span class="k">self</span>.planes.reset();
    }

    <span class="c">/// Forget every label and free the buffers and atlas.</span>
    <span class="k">pub</span> <span class="k">fn</span> release(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx) {
        <span class="k">self</span>.reset();
        <span class="k">self</span>.plates.release(ctx);
        <span class="k">self</span>.planes.release(ctx);
        <span class="k">self</span>.stats.nameplate_capacity_bytes = <span class="k">self</span>.plates.allocated_bytes();
        <span class="k">self</span>.stats.world_plane_buffer_bytes = <span class="k">self</span>.planes.buffer_bytes();
        <span class="k">self</span>.stats.world_plane_texture_bytes = <span class="s">0</span>;
        <span class="k">self</span>.rebuild_resources(ctx);
    }

    <span class="c">/// Start the atlas and CPU glyph cache over, and rebuild both renderers from the new atlas.</span>
    <span class="k">fn</span> rebuild_resources(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx) {
        <span class="k">self</span>.atlas = make_atlas(ctx, &amp;<span class="k">self</span>.cache, <span class="k">self</span>.target.format);
        <span class="k">self</span>.overlay = renderer(
            ctx,
            &amp;<span class="k">mut</span> <span class="k">self</span>.atlas,
            <span class="k">self</span>.target,
            wgpu::CompareFunction::Always,
        );
        <span class="k">self</span>.anchored = renderer(
            ctx,
            &amp;<span class="k">mut</span> <span class="k">self</span>.atlas,
            <span class="k">self</span>.target,
            wgpu::CompareFunction::GreaterEqual,
        );
        <span class="k">self</span>.raster = SwashCache::new();
        <span class="k">self</span>.raster_keys.clear();
        <span class="k">self</span>.stats.raster_images = <span class="s">0</span>;
        <span class="k">self</span>.stats.new_raster_images = <span class="s">0</span>;
        <span class="k">self</span>.stats.raster_image_capacity_bytes = <span class="s">0</span>;
        <span class="k">self</span>.stats.atlas_resets += <span class="s">1</span>;
        <span class="k">self</span>.atlas_font_revision = <span class="k">self</span>.document.font_revision;
        <span class="k">self</span>.prepared = None;
    }
}</code></pre></div>
<p><code>lessons/11/src/engine/gpu/text.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">impl</span> TextFrame {
    <span class="c">/// Framebuffer pixels per CSS pixel; fails on a stretched canvas.</span>
    <span class="k">pub</span> <span class="k">fn</span> scale(&amp;<span class="k">self</span>) -&gt; anyhow::Result&lt;f32&gt; {
        anyhow::ensure!(
            <span class="k">self</span>.framebuffer[<span class="s">0</span>] &gt; <span class="s">0</span> &amp;&amp; <span class="k">self</span>.framebuffer[<span class="s">1</span>] &gt; <span class="s">0</span>,
            &quot;<span class="s">empty text framebuffer</span>&quot;
        );
        anyhow::ensure!(
            <span class="k">self</span>.logical[<span class="s">0</span>].is_finite()
                &amp;&amp; <span class="k">self</span>.logical[<span class="s">1</span>].is_finite()
                &amp;&amp; <span class="k">self</span>.logical[<span class="s">0</span>] &gt; <span class="s">0</span>.<span class="s">0</span>
                &amp;&amp; <span class="k">self</span>.logical[<span class="s">1</span>] &gt; <span class="s">0</span>.<span class="s">0</span>,
            &quot;<span class="s">empty text CSS box</span>&quot;
        );
        <span class="k">let</span> x = <span class="k">self</span>.framebuffer[<span class="s">0</span>] <span class="k">as</span> f64 / <span class="k">self</span>.logical[<span class="s">0</span>];
        <span class="k">let</span> y = <span class="k">self</span>.framebuffer[<span class="s">1</span>] <span class="k">as</span> f64 / <span class="k">self</span>.logical[<span class="s">1</span>];
        <span class="c">// the canvas is rounded to whole pixels, which may shift each axis by one</span>
        <span class="k">let</span> tolerance = <span class="s">1</span>.<span class="s">0</span> / <span class="k">self</span>.logical[<span class="s">0</span>] + <span class="s">1</span>.<span class="s">0</span> / <span class="k">self</span>.logical[<span class="s">1</span>];
        anyhow::ensure!(
            (x - y).abs() &lt;= tolerance,
            &quot;<span class="s">text canvas is stretched non-uniformly</span>&quot;
        );
        Ok(y <span class="k">as</span> f32)
    }
}

<span class="c">/// Where a label lands on screen, in framebuffer pixels.</span>
<span class="k">struct</span> PlacedText {
    left: f32,
    top: f32,
    scale: f32, <span class="c">// real pixels per font pixel: the device scale, or less for far world-sized text</span>
    depth: Option&lt;f32&gt;, <span class="c">// None = overlay</span>
}</code></pre></div>
<p><code>lessons/11/src/engine/gpu/text.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Screen position of a label; None behind the eye, outside near and far, or for plane text.</span>
<span class="k">fn</span> place(label: &amp;TextLabel, frame: &amp;TextFrame, scale: f32) -&gt; Option&lt;PlacedText&gt; {
    <span class="k">let</span> (world, offset, world_height) = <span class="k">match</span> label.placement {
        TextPlacement::WorldPlane { .. } =&gt; <span class="k">return</span> None,
        TextPlacement::Screen { left, top } =&gt; {
            <span class="k">return</span> Some(PlacedText {
                left: left * scale,
                top: top * scale,
                scale,
                depth: None,
            });
        }
        TextPlacement::Anchor { world, offset } =&gt; (world, offset, None),
        TextPlacement::Nameplate { world, .. } =&gt; (world, [<span class="s">0</span>.<span class="s">0</span>; <span class="s">2</span>], None),
        TextPlacement::WorldBillboard {
            world,
            world_height,
        } =&gt; (world, [<span class="s">0</span>.<span class="s">0</span>; <span class="s">2</span>], Some(world_height)),
    };
    <span class="k">let</span> p = [
        (world[<span class="s">0</span>] - frame.origin[<span class="s">0</span>]) <span class="k">as</span> f32,
        (world[<span class="s">1</span>] - frame.origin[<span class="s">1</span>]) <span class="k">as</span> f32,
        (world[<span class="s">2</span>] - frame.origin[<span class="s">2</span>]) <span class="k">as</span> f32,
    ];
    <span class="k">let</span> m = &amp;frame.mvp;
    <span class="c">// row 3 of the matrix gives w; w &lt;= 0 is behind the eye</span>
    <span class="k">let</span> w = m[<span class="s">3</span>] * p[<span class="s">0</span>] + m[<span class="s">7</span>] * p[<span class="s">1</span>] + m[<span class="s">11</span>] * p[<span class="s">2</span>] + m[<span class="s">15</span>];

    <span class="k">if</span> !w.is_finite() || w &lt;= <span class="s">0</span>.<span class="s">0</span> {
        <span class="k">return</span> None;
    }

    <span class="c">// depth 0..1 lies between the far and near planes</span>
    <span class="k">let</span> z = (m[<span class="s">2</span>] * p[<span class="s">0</span>] + m[<span class="s">6</span>] * p[<span class="s">1</span>] + m[<span class="s">10</span>] * p[<span class="s">2</span>] + m[<span class="s">14</span>]) / w;

    <span class="k">if</span> !(<span class="s">0</span>.<span class="s">0</span>..=<span class="s">1</span>.<span class="s">0</span>).contains(&amp;z) {
        <span class="k">return</span> None;
    }

    <span class="k">let</span> x = (m[<span class="s">0</span>] * p[<span class="s">0</span>] + m[<span class="s">4</span>] * p[<span class="s">1</span>] + m[<span class="s">8</span>] * p[<span class="s">2</span>] + m[<span class="s">12</span>]) / w;
    <span class="k">let</span> y = (m[<span class="s">1</span>] * p[<span class="s">0</span>] + m[<span class="s">5</span>] * p[<span class="s">1</span>] + m[<span class="s">9</span>] * p[<span class="s">2</span>] + m[<span class="s">13</span>]) / w;
    <span class="c">// world-sized text shrinks with w: pixel height = world height x y scale x half the framebuffer / w</span>
    <span class="k">let</span> raster_scale = <span class="k">match</span> world_height {
        Some(height) =&gt; {
            <span class="k">let</span> projection = (m[<span class="s">1</span>] * m[<span class="s">1</span>] + m[<span class="s">5</span>] * m[<span class="s">5</span>] + m[<span class="s">9</span>] * m[<span class="s">9</span>]).sqrt();
            height <span class="k">as</span> f32 * projection * frame.framebuffer[<span class="s">1</span>] <span class="k">as</span> f32 / (<span class="s">2</span>.<span class="s">0</span> * w * label.font_size)
        }
        None =&gt; scale,
    };

    <span class="k">if</span> !raster_scale.is_finite() || raster_scale &lt;= <span class="s">0</span>.<span class="s">0</span> {
        <span class="k">return</span> None;
    }

    Some(PlacedText {
        left: (x + <span class="s">1</span>.<span class="s">0</span>) * <span class="s">0</span>.<span class="s">5</span> * frame.framebuffer[<span class="s">0</span>] <span class="k">as</span> f32 + offset[<span class="s">0</span>] * scale,
        top: (<span class="s">1</span>.<span class="s">0</span> - y) * <span class="s">0</span>.<span class="s">5</span> * frame.framebuffer[<span class="s">1</span>] <span class="k">as</span> f32 + offset[<span class="s">1</span>] * scale,
        scale: raster_scale,
        depth: <span class="k">match</span> label.placement {
            TextPlacement::Nameplate { .. } =&gt; None, <span class="c">// stays on top of its own object</span>
            _ =&gt; Some(z),
        },
    })
}</code></pre></div>
<p><code>lessons/11/src/engine/gpu/text.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Center a nameplate on its anchor and return the plate around it.</span>
<span class="k">fn</span> center_nameplate(
    run: &amp;TextRun,
    placed: &amp;<span class="k">mut</span> PlacedText,
    frame: &amp;TextFrame,
    scale: f32,
) -&gt; Option&lt;plate::Rectangle&gt; {
    <span class="k">let</span> TextPlacement::Nameplate {
        padding, rounded, ..
    } = run.label.placement
    <span class="k">else</span> {
        <span class="k">return</span> None;
    };

    <span class="k">if</span> run.label.text.is_empty() {
        <span class="k">return</span> None;
    }

    <span class="k">let</span> <span class="k">mut</span> width = <span class="s">0</span>.<span class="s">0f32</span>;
    <span class="k">let</span> <span class="k">mut</span> top = f32::INFINITY;
    <span class="k">let</span> <span class="k">mut</span> bottom = f32::NEG_INFINITY;

    <span class="k">for</span> line <span class="k">in</span> run.buffer.layout_runs() {
        width = width.max(line.line_w);
        top = top.min(line.line_top);
        bottom = bottom.max(line.line_top + line.line_height);
    }

    <span class="k">if</span> !top.is_finite() || !bottom.is_finite() {
        <span class="k">return</span> None;
    }

    placed.left -= width * scale * <span class="s">0</span>.<span class="s">5</span>;
    placed.top -= (top + bottom) * scale * <span class="s">0</span>.<span class="s">5</span>;
    <span class="k">let</span> bounds = clip_bounds(&amp;run.label, frame, scale);
    <span class="k">let</span> bounds = [
        bounds.left <span class="k">as</span> f32,
        bounds.top <span class="k">as</span> f32,
        bounds.right <span class="k">as</span> f32,
        bounds.bottom <span class="k">as</span> f32,
    ];
    Some(plate::Rectangle {
        bounds: [
            placed.left - padding[<span class="s">0</span>] * scale,
            placed.top + (top - padding[<span class="s">1</span>]) * scale,
            placed.left + (width + padding[<span class="s">0</span>]) * scale,
            placed.top + (bottom + padding[<span class="s">1</span>]) * scale,
        ],
        clip: bounds,
        rounded,
    })
}

<span class="c">/// The label's clip box in framebuffer pixels, or the whole canvas.</span>
<span class="k">fn</span> clip_bounds(label: &amp;TextLabel, frame: &amp;TextFrame, scale: f32) -&gt; TextBounds {
    <span class="k">match</span> label.clip {
        Some(c) =&gt; TextBounds {
            left: (c[<span class="s">0</span>] * scale).floor() <span class="k">as</span> i32,
            top: (c[<span class="s">1</span>] * scale).floor() <span class="k">as</span> i32,
            right: (c[<span class="s">2</span>] * scale).ceil() <span class="k">as</span> i32, <span class="c">// floor and ceil round outward: an edge glyph keeps its last pixel</span>
            bottom: (c[<span class="s">3</span>] * scale).ceil() <span class="k">as</span> i32,
        },
        None =&gt; TextBounds {
            left: <span class="s">0</span>,
            top: <span class="s">0</span>,
            right: frame.framebuffer[<span class="s">0</span>] <span class="k">as</span> i32,
            bottom: frame.framebuffer[<span class="s">1</span>] <span class="k">as</span> i32,
        },
    }
}</code></pre></div>
<p><code>lessons/11/src/engine/gpu/text.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// A glyphon renderer; \`compare\` decides whether scene geometry can hide its text.</span>
<span class="k">fn</span> renderer(
    ctx: &amp;GpuCtx,
    atlas: &amp;<span class="k">mut</span> TextAtlas,
    target: Target,
    compare: wgpu::CompareFunction,
) -&gt; TextRenderer {
    TextRenderer::new(
        atlas,
        &amp;ctx.device,
        wgpu::MultisampleState {
            count: target.samples,
            ..Default::default()
        },
        Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: Some(<span class="s">false</span>), <span class="c">// text never hides geometry or other text</span>
            depth_compare: Some(compare),
            stencil: Default::default(),
            bias: Default::default(),
        }),
    )
}

<span class="c">/// Accurate blends in linear light for an sRGB canvas; Web blends like a browser does.</span>
<span class="k">fn</span> make_atlas(ctx: &amp;GpuCtx, cache: &amp;Cache, format: wgpu::TextureFormat) -&gt; TextAtlas {
    <span class="k">let</span> mode = <span class="k">if</span> format.is_srgb() {
        ColorMode::Accurate
    } <span class="k">else</span> {
        ColorMode::Web
    };
    TextAtlas::with_color_mode(&amp;ctx.device, &amp;ctx.queue, cache, format, mode)
}

<span class="c">/// A missing id gets 0, the far plane in reverse-Z: visible only over empty background.</span>
<span class="k">fn</span> depth_for(depths: &amp;HashMap&lt;usize, f32&gt;, id: usize) -&gt; f32 {
    depths.get(&amp;id).copied().unwrap_or(<span class="s">0</span>.<span class="s">0</span>)
}

<span class="c">/// Time in ms; 0 outside the browser.</span>
<span class="k">fn</span> now_ms() -&gt; f64 {
    #[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
    <span class="k">if</span> <span class="k">let</span> Some(window) = web_sys::window()
        &amp;&amp; <span class="k">let</span> Some(performance) = window.performance()
    {
        <span class="k">return</span> performance.now();
    }

    <span class="s">0</span>.<span class="s">0</span>
}</code></pre></div>
<p>Copy this part from the lesson folder to the path shown.</p>
<p><code>lessons/11/src/engine/gpu/text.rs</code> · copy the file, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>#[cfg(test)]
<span class="k">mod</span> tests {
    <span class="k">use</span> super::*;

    <span class="c">/// A frame with an identity camera and the given device scale.</span>
    <span class="k">fn</span> frame(scale: f64) -&gt; TextFrame {
        TextFrame {
            mvp: [
                <span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>,
            ],
            origin: [<span class="s">0</span>.<span class="s">0</span>; <span class="s">3</span>],
            framebuffer: [(<span class="s">800</span>.<span class="s">0</span> * scale) <span class="k">as</span> u32, (<span class="s">600</span>.<span class="s">0</span> * scale) <span class="k">as</span> u32],
            logical: [<span class="s">800</span>.<span class="s">0</span>, <span class="s">600</span>.<span class="s">0</span>],
            ortho_half_height: <span class="s">1</span>.<span class="s">0</span>,
        }
    }

    #[test]
    <span class="c">/// Screen labels scale with the device scale exactly once.</span>
    <span class="k">fn</span> logical_to_physical_scale_is_applied_once() {
        <span class="k">for</span> scale <span class="k">in</span> [<span class="s">1</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">25</span>, <span class="s">1</span>.<span class="s">5</span>, <span class="s">2</span>.<span class="s">0</span>] {
            <span class="k">let</span> frame = frame(scale);
            assert_eq!(frame.scale().unwrap(), scale <span class="k">as</span> f32);
            <span class="k">let</span> label = TextLabel {
                id: <span class="s">1</span>,
                text: &quot;<span class="s">A</span>&quot;.into(),
                font_size: <span class="s">16</span>.<span class="s">0</span>,
                line_height: <span class="s">24</span>.<span class="s">0</span>,
                color: [<span class="s">255</span>; <span class="s">4</span>],
                placement: TextPlacement::Screen {
                    left: <span class="s">12</span>.<span class="s">25</span>,
                    top: <span class="s">8</span>.<span class="s">5</span>,
                },
                clip: None,
            };
            <span class="k">let</span> placed = place(&amp;label, &amp;frame, frame.scale().unwrap()).unwrap();
            assert_eq!(placed.left, <span class="s">12</span>.<span class="s">25</span> * scale <span class="k">as</span> f32);
            assert_eq!(placed.scale * label.font_size, <span class="s">16</span>.<span class="s">0</span> * scale <span class="k">as</span> f32);
        }
    }

    #[test]
    <span class="c">/// An anchored label keeps its depth; past the far plane it is dropped.</span>
    <span class="k">fn</span> scene_anchor_preserves_rebased_depth_and_culls_near_plane() {
        <span class="k">let</span> <span class="k">mut</span> frame = frame(<span class="s">2</span>.<span class="s">0</span>);
        frame.origin = [<span class="s">1_000_000</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>];
        <span class="k">let</span> <span class="k">mut</span> label = TextLabel {
            id: <span class="s">1</span>,
            text: &quot;<span class="s">A</span>&quot;.into(),
            font_size: <span class="s">16</span>.<span class="s">0</span>,
            line_height: <span class="s">24</span>.<span class="s">0</span>,
            color: [<span class="s">255</span>; <span class="s">4</span>],
            placement: TextPlacement::Anchor {
                world: [<span class="s">1_000_000</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">25</span>],
                offset: [<span class="s">0</span>.<span class="s">0</span>; <span class="s">2</span>],
            },
            clip: None,
        };
        <span class="k">let</span> placed = place(&amp;label, &amp;frame, <span class="s">2</span>.<span class="s">0</span>).unwrap();
        assert_eq!((placed.left, placed.top), (<span class="s">800</span>.<span class="s">0</span>, <span class="s">600</span>.<span class="s">0</span>));
        assert_eq!(placed.depth, Some(<span class="s">0</span>.<span class="s">25</span>));
        label.placement = TextPlacement::Anchor {
            world: [<span class="s">1_000_000</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">01</span>],
            offset: [<span class="s">0</span>.<span class="s">0</span>; <span class="s">2</span>],
        };
        assert!(place(&amp;label, &amp;frame, <span class="s">2</span>.<span class="s">0</span>).is_none());
    }

    #[test]
    <span class="c">/// A nameplate is centered and padded in CSS pixels at every scale.</span>
    <span class="k">fn</span> nameplate_center_padding_clip_and_scale_share_one_coordinate_system() {
        <span class="k">let</span> <span class="k">mut</span> document = TextDocument::new();
        <span class="k">let</span> <span class="k">mut</span> label = TextLabel {
            id: <span class="s">1</span>,
            text: &quot;<span class="s">Sphere Ø25</span>&quot;.into(),
            font_size: <span class="s">18</span>.<span class="s">0</span>,
            line_height: <span class="s">26</span>.<span class="s">0</span>,
            color: [<span class="s">255</span>; <span class="s">4</span>],
            placement: TextPlacement::Nameplate {
                world: [<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">25</span>],
                padding: [<span class="s">6</span>.<span class="s">0</span>, <span class="s">4</span>.<span class="s">0</span>],
                rounded: <span class="s">false</span>,
            },
            clip: None,
        };
        document.set_labels(vec![label.clone()]).unwrap();

        <span class="k">for</span> scale <span class="k">in</span> [<span class="s">1</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">25</span>, <span class="s">2</span>.<span class="s">0</span>] {
            <span class="k">let</span> frame = frame(scale);
            <span class="k">let</span> run = &amp;document.runs[<span class="s">0</span>];
            <span class="k">let</span> <span class="k">mut</span> placed = place(&amp;run.label, &amp;frame, scale <span class="k">as</span> f32).unwrap();
            <span class="k">let</span> rectangle = center_nameplate(run, &amp;<span class="k">mut</span> placed, &amp;frame, scale <span class="k">as</span> f32)
                .unwrap()
                .bounds;
            assert_eq!(
                placed.depth, None,
                &quot;<span class="s">a source-center annotation overlays its own solid</span>&quot;
            );
            assert!(((rectangle[<span class="s">0</span>] + rectangle[<span class="s">2</span>]) * <span class="s">0</span>.<span class="s">5</span> - <span class="s">400</span>.<span class="s">0</span> * scale <span class="k">as</span> f32).abs() &lt; <span class="s">0</span>.<span class="s">001</span>);
            assert!(((rectangle[<span class="s">1</span>] + rectangle[<span class="s">3</span>]) * <span class="s">0</span>.<span class="s">5</span> - <span class="s">300</span>.<span class="s">0</span> * scale <span class="k">as</span> f32).abs() &lt; <span class="s">0</span>.<span class="s">001</span>);
            assert!((rectangle[<span class="s">3</span>] - rectangle[<span class="s">1</span>] - <span class="s">34</span>.<span class="s">0</span> * scale <span class="k">as</span> f32).abs() &lt; <span class="s">0</span>.<span class="s">001</span>);
        }

        label.clip = Some([<span class="s">390</span>.<span class="s">0</span>, <span class="s">290</span>.<span class="s">0</span>, <span class="s">410</span>.<span class="s">0</span>, <span class="s">310</span>.<span class="s">0</span>]);
        label.color = [<span class="s">255</span>, <span class="s">255</span>, <span class="s">0</span>, <span class="s">255</span>];
        document.set_labels(vec![label.clone()]).unwrap();
        assert_eq!(
            document.shape_count, <span class="s">1</span>,
            &quot;<span class="s">annotation style and clip reuse shaping</span>&quot;
        );
        <span class="k">let</span> frame = frame(<span class="s">2</span>.<span class="s">0</span>);
        <span class="k">let</span> run = &amp;document.runs[<span class="s">0</span>];
        <span class="k">let</span> <span class="k">mut</span> placed = place(&amp;run.label, &amp;frame, <span class="s">2</span>.<span class="s">0</span>).unwrap();
        assert_eq!(
            center_nameplate(run, &amp;<span class="k">mut</span> placed, &amp;frame, <span class="s">2</span>.<span class="s">0</span>)
                .unwrap()
                .clip,
            [<span class="s">780</span>.<span class="s">0</span>, <span class="s">580</span>.<span class="s">0</span>, <span class="s">820</span>.<span class="s">0</span>, <span class="s">620</span>.<span class="s">0</span>]
        );
        label.placement = TextPlacement::Nameplate {
            world: [<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">25</span>],
            padding: [f32::NAN, <span class="s">4</span>.<span class="s">0</span>],
            rounded: <span class="s">false</span>,
        };
        assert!(document.set_labels(vec![label]).is_err());
        assert_eq!(document.shape_count, <span class="s">1</span>);
        assert_eq!(
            document.runs.len(),
            <span class="s">1</span>,
            &quot;<span class="s">invalid padding preserves the previous document</span>&quot;
        );
    }
}</code></pre></div>
<p>Run <code>cargo check</code> in <code>lessons/11/</code>.</p>
<h2 id="step-6-srcenginegpumodrs">Step 6 · src/engine/gpu/mod.rs<a class="anchor" href="#/course/11-text-rendering#step-6-srcenginegpumodrs" aria-label="Link to this section">#</a></h2>
<p>The GPU owner connects buffers, pipelines and frame resources.</p>
<p><code>lessons/11/src/engine/gpu/mod.rs</code> · edit · type this</p>
<p>Added after the <code>pub mod targets;</code> line of <code>lessons/10/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> <span class="k">mod</span> text;</code></pre></div>
<p>Added after the <code>pub glyphs: glyphs::GlyphLane,</code> line in <code>struct Gpu</code> of <code>lessons/10/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> text: text::TextLane, <span class="c">// labels</span></code></pre></div>
<p>Added after the <code>let glyphs = glyphs::GlyphLane::new(&amp;ctx, &amp;la…</code> line in <code>fn new</code> of <code>lessons/10/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> text = text::TextLane::new(&amp;ctx, target);</code></pre></div>
<p>Added after the <code>glyphs,</code> line in <code>fn new</code> of <code>lessons/10/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            text,</code></pre></div>
<p>Added after the <code>self.glyphs.retarget(&amp;self.ctx, &amp;self.layouts…</code> line in <code>fn retarget</code> of <code>lessons/10/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">self</span>.text.retarget(&amp;<span class="k">self</span>.ctx, target);</code></pre></div>
<p>Replaces <code>fn write_frame_uniforms</code> in <code>lessons/10/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Write the camera and pen scale for every lane.</span>
    <span class="k">pub</span> <span class="k">fn</span> write_frame_uniforms(&amp;<span class="k">mut</span> <span class="k">self</span>, input: &amp;FrameInput) -&gt; anyhow::Result&lt;()&gt; {</code></pre></div>
<p>Replaces the 5 lines from <code>}</code> in <code>impl Gpu</code> of <code>lessons/10/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.text.prepare(
            &amp;<span class="k">self</span>.ctx,
            &amp;text::TextFrame {
                mvp: <span class="k">self</span>.frame.mvp_f32,
                origin: <span class="k">self</span>.objects.anchor(),
                framebuffer: [<span class="k">self</span>.config.width, <span class="k">self</span>.config.height], <span class="c">// size in physical pixels</span>
                logical: <span class="k">self</span>.logical_size,
                ortho_half_height: <span class="k">self</span>.frame.ortho_h,
            },
        )?;
        Ok(())
    }

    <span class="c">/// Encode cloud depth first, then physical mesh faces, then analytic ink.</span>
    <span class="k">pub</span> <span class="k">fn</span> render(&amp;<span class="k">mut</span> <span class="k">self</span>, input: &amp;FrameInput) -&gt; anyhow::Result&lt;()&gt; {
        <span class="k">self</span>.write_frame_uniforms(input)?;</code></pre></div>
<p>Added after the <code>self.arena.draw_text(&amp;mut pass, &amp;basic);</code> line in <code>fn render</code> of <code>lessons/10/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">self</span>.text.draw(&amp;<span class="k">mut</span> pass);</code></pre></div>
<h2 id="step-7-srclibrs">Step 7 · src/lib.rs<a class="anchor" href="#/course/11-text-rendering#step-7-srclibrs" aria-label="Link to this section">#</a></h2>
<p>The crate entry point connects the camera, scene and GPU owners.</p>
<p><code>lessons/11/src/lib.rs</code> · edit · type this</p>
<p>Replaces <code>mod app</code> in <code>lessons/10/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> session_rust::AABB;
<span class="k">pub</span> <span class="k">mod</span> app;
<span class="k">pub</span> <span class="k">mod</span> camera;
<span class="k">pub</span> <span class="k">mod</span> engine;
<span class="k">pub</span> <span class="k">mod</span> fixture;
#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">pub</span> <span class="k">mod</span> text_quality;</code></pre></div>
<p>Added after the <code>fixture.upload.drop_uploaded();</code> line in <code>fn create</code> of <code>lessons/10/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        gpu.text
            .set_labels(text_labels(&amp;gpu.bounds))
            .map_err(js_error)?;</code></pre></div>
<p>Replaces the <code>Ok(serde_json::json!({&quot;stage&quot;:10,&quot;objects&quot;:se…</code> line in <code>fn render</code> of <code>lessons/10/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        Ok(serde_json::json!({&quot;<span class="s">stage</span>&quot;:<span class="s">11</span>,&quot;<span class="s">objects</span>&quot;:<span class="k">self</span>.gpu.objects.len(),&quot;<span class="s">width</span>&quot;:w,&quot;<span class="s">height</span>&quot;:h,&quot;<span class="s">scale</span>&quot;:scale,&quot;<span class="s">drawn</span>&quot;:<span class="s">true</span>,&quot;<span class="s">text</span>&quot;:<span class="k">self</span>.gpu.text.stats,&quot;<span class="s">sourceObjects</span>&quot;:<span class="k">self</span>.fixture.identities,&quot;<span class="s">sourceEdgeIds</span>&quot;:<span class="k">self</span>.fixture.pipe_source_edges,&quot;<span class="s">samples</span>&quot;:<span class="k">self</span>.gpu.targets.samples,</code></pre></div>
<p>Added after the <code>}</code> line of <code>lessons/10/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Overlay text (CSS size) or world label (scene depth).</span>
<span class="k">fn</span> text_labels(bounds: &amp;AABB) -&gt; Vec&lt;engine::text::TextLabel&gt; {
    <span class="k">use</span> engine::text::{TextLabel, TextPlacement};
    vec![
        TextLabel {
            id: <span class="s">1</span>,
            text: &quot;<span class="s">AVATAR To office ffi — Ø 25 ± 0.1 mm</span>&quot;.into(),
            font_size: <span class="s">16</span>.<span class="s">0</span>,
            line_height: <span class="s">24</span>.<span class="s">0</span>,
            color: [<span class="s">255</span>; <span class="s">4</span>],
            placement: TextPlacement::Nameplate {
                world: [<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, bounds.max_point()[<span class="s">2</span>] + <span class="s">30</span>.<span class="s">0</span>],
                padding: [<span class="s">6</span>.<span class="s">0</span>, <span class="s">4</span>.<span class="s">0</span>],
                rounded: <span class="s">false</span>,
            },
            clip: None,
        },
        TextLabel {
            id: <span class="s">2</span>,
            text: &quot;<span class="s">Centered white nameplate</span>&quot;.into(),
            font_size: <span class="s">13</span>.<span class="s">5</span>,
            line_height: <span class="s">19</span>.<span class="s">5</span>,
            color: [<span class="s">255</span>; <span class="s">4</span>],
            placement: TextPlacement::Nameplate {
                world: [bounds.cx, bounds.cy, bounds.max_point()[<span class="s">2</span>]],
                padding: [<span class="s">12</span>.<span class="s">75</span>, <span class="s">3</span>.<span class="s">0</span>],
                rounded: <span class="s">true</span>,
            },
            clip: None,
        },
        TextLabel {
            id: <span class="s">3</span>,
            text: &quot;<span class="s">text_not_oriented_to_camera</span>&quot;.into(),
            font_size: <span class="s">18</span>.<span class="s">0</span>,
            line_height: <span class="s">26</span>.<span class="s">0</span>,
            color: [<span class="s">255</span>; <span class="s">4</span>],
            placement: TextPlacement::WorldPlane {
                world: [
                    bounds.min_point()[<span class="s">0</span>] - <span class="s">100</span>.<span class="s">0</span>,
                    bounds.min_point()[<span class="s">1</span>] - <span class="s">100</span>.<span class="s">0</span>,
                    bounds.max_point()[<span class="s">2</span>],
                ],
                right: [<span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>],
                up: [<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>],
                world_height: <span class="s">16</span>.<span class="s">0</span>,
            },
            clip: None,
        },
    ]
}</code></pre></div>
<h2 id="step-8-srctext_layoutrs">Step 8 · src/text_layout.rs<a class="anchor" href="#/course/11-text-rendering#step-8-srctext_layoutrs" aria-label="Link to this section">#</a></h2>
<p>Delete lesson 10&#39;s shaping check; the text-quality page below replaces it.</p>
<p>Delete <code>src/text_layout.rs</code> (it exists in <code>lessons/10/</code>, not in <code>lessons/11/</code>).</p>
<h2 id="step-9-assetstext-layouthtml">Step 9 · assets/text-layout.html<a class="anchor" href="#/course/11-text-rendering#step-9-assetstext-layouthtml" aria-label="Link to this section">#</a></h2>
<p>Delete its check page too.</p>
<p>Delete <code>assets/text-layout.html</code> (it exists in <code>lessons/10/</code>, not in <code>lessons/11/</code>).</p>
<h2 id="step-10-assetstext-qualityhtml">Step 10 · assets/text-quality.html<a class="anchor" href="#/course/11-text-rendering#step-10-assetstext-qualityhtml" aria-label="Link to this section">#</a></h2>
<p>Copy the test page that draws GPU text beside browser text at five sizes and any device scale.</p>
<p><code>lessons/11/assets/text-quality.html</code> · 167 lines · copy the file, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>&lt;!doctype html&gt;
&lt;html lang=&quot;<span class="s">en</span>&quot;&gt;
&lt;meta charset=&quot;<span class="s">utf-8</span>&quot;&gt;
&lt;meta name=&quot;<span class="s">viewport</span>&quot; content=&quot;<span class="s">width=device-width,initial-scale=1</span>&quot;&gt;
&lt;title&gt;Session Viewer — text regression&lt;/title&gt;
&lt;style&gt;
@<span class="k">font-face</span> {font-family:'<span class="s">Noto Sans</span>';src:url('<span class="s">./text/NotoSans-Regular.ttf</span>') format('<span class="s">truetype</span>');font-weight:<span class="s">400</span>;font-style:normal;font-display:block}
@<span class="k">font-face</span> {font-family:'<span class="s">Noto Sans Symbols</span>';src:url('<span class="s">./text/NotoSansSymbols-Regular.ttf</span>') format('<span class="s">truetype</span>');font-weight:<span class="s">400</span>;font-style:normal;font-display:block}
@<span class="k">font-face</span> {font-family:'<span class="s">Noto Sans Symbols 2</span>';src:url('<span class="s">./text/NotoSansSymbols2-Regular.ttf</span>') format('<span class="s">truetype</span>');font-weight:<span class="s">400</span>;font-style:normal;font-display:block}
* {box-sizing:border-box} body {margin:<span class="s">16</span><span class="k">px</span>;background:#171717;color:#eee;font:<span class="s">14</span><span class="k">px</span> system-ui}
h1 {font-size:<span class="s">20</span><span class="k">px</span>} .controls {display:flex;flex-wrap:wrap;gap:<span class="s">16</span><span class="k">px</span>;margin:<span class="s">12</span><span class="k">px</span> <span class="s">0</span>}
.columns {display:grid;grid-template-columns:minmax(<span class="s">550</span><span class="k">px</span>,<span class="s">1</span><span class="k">fr</span>) minmax(<span class="s">550</span><span class="k">px</span>,<span class="s">1</span><span class="k">fr</span>);gap:<span class="s">16</span><span class="k">px</span>}
.panel {position:relative;background:#000;color:#fff;height:<span class="s">1040</span><span class="k">px</span>;overflow:hidden}
canvas {display:block;width:<span class="s">100</span><span class="k">%</span>;height:<span class="s">1040</span><span class="k">px</span>} #diagnostic-canvas {position:absolute;inset:<span class="s">0</span>;pointer-events:none}
.reference {position:absolute;white-space:pre;font-family:'<span class="s">Noto Sans</span>','<span class="s">Noto Sans Symbols</span>','<span class="s">Noto Sans Symbols 2</span>';font-weight:<span class="s">400</span>;font-style:normal;font-kerning:normal;font-variant-ligatures:common-ligatures;font-feature-settings:'<span class="s">kern</span>' <span class="s">1</span>,'<span class="s">liga</span>' <span class="s">1</span>,'<span class="s">clig</span>' <span class="s">1</span>;letter-spacing:normal}
pre {white-space:pre-wrap;overflow-wrap:anywhere;font-size:<span class="s">12</span><span class="k">px</span>} button,select {font:inherit}
.caption {font-size:<span class="s">13</span><span class="k">px</span>;margin:<span class="s">6</span><span class="k">px</span> <span class="s">0</span>} .failure {color:#ff9d9d} #metric-result {padding:<span class="s">10</span><span class="k">px</span> <span class="s">0</span>}
&lt;/style&gt;
&lt;h1&gt;White-on-black text regression&lt;/h1&gt;
&lt;p&gt;Production WebGPU text beside browser text. Both use the bundled Noto Sans Regular (400), advanced shaping, default kerning and common ligatures. Sizes: 12, 14, 16, 18 and 24 CSS px; line height: 1.5. Inspect at normal size.&lt;/p&gt;
&lt;div class=&quot;<span class="s">controls</span>&quot;&gt;
&lt;label&gt;Specimen &lt;select id=&quot;<span class="s">specimen</span>&quot;&gt;&lt;option value=&quot;<span class="s">reference</span>&quot;&gt;Font comparison&lt;/option&gt;&lt;option value=&quot;<span class="s">nameplate</span>&quot;&gt;Object nameplate&lt;/option&gt;&lt;/select&gt;&lt;/label&gt;
&lt;label&gt;Raster scale &lt;select id=&quot;<span class="s">scale</span>&quot;&gt;&lt;option value=&quot;<span class="s">0</span>&quot;&gt;Actual canvas / browser DPR&lt;/option&gt;&lt;option&gt;1&lt;/option&gt;&lt;option&gt;1.25&lt;/option&gt;&lt;option&gt;1.5&lt;/option&gt;&lt;option&gt;2&lt;/option&gt;&lt;/select&gt;&lt;/label&gt;
&lt;label&gt;Background &lt;select id=&quot;<span class="s">background</span>&quot;&gt;&lt;option value=&quot;<span class="s">0</span>&quot;&gt;Black&lt;/option&gt;&lt;option value=&quot;<span class="s">8421504</span>&quot;&gt;Gray&lt;/option&gt;&lt;option value=&quot;<span class="s">16777215</span>&quot;&gt;White&lt;/option&gt;&lt;/select&gt;&lt;/label&gt;
&lt;label&gt;&lt;input id=&quot;<span class="s">selected</span>&quot; type=&quot;<span class="s">checkbox</span>&quot;&gt;Selected yellow&lt;/label&gt;
&lt;label&gt;&lt;input id=&quot;<span class="s">diagnostics</span>&quot; type=&quot;<span class="s">checkbox</span>&quot;&gt;Baselines, origins and raster bounds&lt;/label&gt;
&lt;button id=&quot;<span class="s">reset</span>&quot;&gt;Release resources and reload&lt;/button&gt;
&lt;button id=&quot;<span class="s">download</span>&quot;&gt;Download diagnostic JSON&lt;/button&gt;
&lt;/div&gt;
&lt;div class=&quot;<span class="s">columns</span>&quot;&gt;&lt;div&gt;&lt;p class=&quot;<span class="s">caption</span>&quot;&gt;WebGPU / Glyphon coverage atlas&lt;/p&gt;&lt;div class=&quot;<span class="s">panel</span>&quot;&gt;&lt;canvas id=&quot;<span class="s">text-quality-canvas</span>&quot;&gt;&lt;/canvas&gt;&lt;canvas id=&quot;<span class="s">diagnostic-canvas</span>&quot;&gt;&lt;/canvas&gt;&lt;/div&gt;&lt;/div&gt;&lt;div&gt;&lt;p class=&quot;<span class="s">caption</span>&quot;&gt;Browser / same loaded font&lt;/p&gt;&lt;div class=&quot;<span class="s">panel</span>&quot; id=&quot;<span class="s">reference</span>&quot;&gt;&lt;/div&gt;&lt;/div&gt;&lt;/div&gt;
&lt;div id=&quot;<span class="s">metric-result</span>&quot; role=&quot;<span class="s">status</span>&quot;&gt;Loading fonts and WebGPU…&lt;/div&gt;
&lt;details&gt;&lt;summary&gt;Environment, shaping and preparation counters&lt;/summary&gt;&lt;pre id=&quot;<span class="s">metadata</span>&quot;&gt;&lt;/pre&gt;&lt;/details&gt;
&lt;p&gt;Unsupported font characters are shown as the font’s missing-glyph box and counted. Atlas packing, upload byte counts and allocation capacity are private to Glyphon; reported raster keys and active instance bytes are not substitutes for those measurements. Browser zoom should be tested separately from forced raster scale.&lt;/p&gt;
&lt;script type=&quot;<span class="s">module</span>&quot;&gt;
<span class="k">let</span> fixture, rows, lastReport, lastDiagnostics, observer;
<span class="k">const</span> canvas=document.getElementById('<span class="s">text-quality-canvas</span>');
<span class="k">const</span> reference=document.getElementById('<span class="s">reference</span>');
<span class="k">const</span> status=document.getElementById('<span class="s">metric-result</span>');

<span class="c">/** Load the same fonts in the browser first, then open the viewer's text lane on the canvas. */</span>
<span class="k">async</span> <span class="k">function</span> start() {
  <span class="k">if</span> (!navigator.gpu) <span class="k">throw</span> new Error('<span class="s">WebGPU is unavailable in this browser.</span>');
  <span class="k">await</span> Promise.all([document.fonts.load('<span class="s">16px &quot;Noto Sans&quot;</span>'),document.fonts.load('<span class="s">16px &quot;Noto Sans Symbols 2&quot;</span>'),document.fonts.load('<span class="s">16px &quot;Noto Sans Symbols&quot;</span>')]);
  <span class="k">await</span> document.fonts.ready;
  <span class="c">// Trunk adds a hash to the .js and .wasm names; read the real names from index.html</span>
  <span class="k">const</span> html=<span class="k">await</span> fetch('<span class="s">./index.html</span>',{cache:'<span class="s">no-store</span>'}).then(readText);
  <span class="k">const</span> match=html.match(/(?:<span class="s">import</span>[^<span class="s">;</span>]*?<span class="s">from\\s</span>*|<span class="s">import\\s</span>*)[<span class="s">&quot;'</span>]([^<span class="s">&quot;'</span>]*<span class="s">session_viewer</span>[^<span class="s">&quot;'</span>]*<span class="s">\\.js</span>)[<span class="s">&quot;'</span>]/);
  <span class="k">if</span>(!match) <span class="k">throw</span> new Error('<span class="s">Cannot find the built viewer module in index.html. Build this fixture with Trunk.</span>');
  <span class="k">const</span> module=<span class="k">await</span> import(new URL(match[<span class="s">1</span>],location.href).href);
  <span class="k">const</span> wasmMatch=html.match(/<span class="s">module_or_path:\\s</span>*[<span class="s">&quot;'</span>]([^<span class="s">&quot;'</span>]+<span class="s">\\.wasm</span>)[<span class="s">&quot;'</span>]/);
  <span class="k">if</span>(!wasmMatch) <span class="k">throw</span> new Error('<span class="s">Cannot find the built WASM path in index.html.</span>');
  <span class="k">await</span> module.default({module_or_path:new URL(wasmMatch[<span class="s">1</span>],location.href).href});
  fixture=<span class="k">await</span> module.TextQuality.create(canvas);
  rows=JSON.parse(fixture.specimens());
  buildReference();
  render();
  observer=new ResizeObserver(render);observer.observe(canvas);
  window.addEventListener('<span class="s">resize</span>',render);
  document.getElementById('<span class="s">scale</span>').addEventListener('<span class="s">change</span>',render);
  document.getElementById('<span class="s">specimen</span>').addEventListener('<span class="s">change</span>',render);
  document.getElementById('<span class="s">background</span>').addEventListener('<span class="s">change</span>',render);
  document.getElementById('<span class="s">selected</span>').addEventListener('<span class="s">change</span>',render);
  document.getElementById('<span class="s">diagnostics</span>').addEventListener('<span class="s">change</span>',render);
  document.getElementById('<span class="s">reset</span>').addEventListener('<span class="s">click</span>',reset);
  document.getElementById('<span class="s">download</span>').addEventListener('<span class="s">click</span>',download);
  window.addEventListener('<span class="s">pagehide</span>',dispose,{once:<span class="s">true</span>});
  window.textQuality={
    render,reset,fixture,
    <span class="c">/** The last measurement, without drawing again. */</span>
    <span class="k">get</span> report(){<span class="k">return</span> lastReport},
    <span class="c">/** Glyph origins and bitmap boxes from that same frame. */</span>
    <span class="k">get</span> diagnostics(){<span class="k">return</span> lastDiagnostics}
  };
}
<span class="c">/** A failed fetch throws instead of handing an error page to the parser. */</span>
<span class="k">function</span> readText(response) { <span class="k">if</span>(!response.ok) <span class="k">throw</span> new Error(\`<span class="s">Build asset HTTP </span>\${<span class="s">response</span>.<span class="s">status</span>}\`);<span class="k">return</span> response.text(); }
<span class="c">/** Browser text at the same CSS positions as the GPU labels. */</span>
<span class="k">function</span> buildReference() {
  reference.replaceChildren();reference.dataset.specimen='<span class="s">reference</span>';
  <span class="k">for</span>(<span class="k">const</span> row of rows) {
    <span class="k">const</span> element=document.createElement('<span class="s">div</span>');element.className='<span class="s">reference</span>';element.dataset.id=row.id;
    element.style.cssText=\`<span class="s">left:</span>\${<span class="s">row</span>.<span class="s">left</span>}<span class="s">px;top:</span>\${<span class="s">row</span>.<span class="s">top</span>}<span class="s">px;font-size:</span>\${<span class="s">row</span>.<span class="s">size</span>}<span class="s">px;line-height:</span>\${<span class="s">row</span>.<span class="s">lineHeight</span>}<span class="s">px</span>\`;
    element.textContent=row.text;reference.append(element);
  }
}
<span class="c">/** Draw with the current controls and compare line widths with the browser. */</span>
<span class="k">function</span> render() {
  <span class="k">if</span>(!fixture) <span class="k">return</span>;
  <span class="k">try</span> {
    <span class="k">if</span>(document.getElementById('<span class="s">specimen</span>').value==='<span class="s">nameplate</span>'){renderNameplate();<span class="k">return</span>;}
    <span class="k">if</span>(reference.dataset.specimen!=='<span class="s">reference</span>')buildReference();
    <span class="k">const</span> selected=document.getElementById('<span class="s">selected</span>').checked;
    <span class="k">const</span> background=Number(document.getElementById('<span class="s">background</span>').value);
    <span class="k">const</span> scale=Number(document.getElementById('<span class="s">scale</span>').value);
    <span class="k">const</span> report=JSON.parse(fixture.render(scale,selected,background));
    reference.style.background=\`<span class="s">#</span>\${<span class="s">background</span>.<span class="s">toString(16)</span>.<span class="s">padStart(6</span>,'<span class="s">0</span>'<span class="s">)</span>}\`;
    reference.style.color=selected?'<span class="s">#ffff00</span>':'<span class="s">#ffffff</span>';
    lastDiagnostics=JSON.parse(fixture.diagnostics());
    lastReport={...report,browser:navigator.userAgent,platform:navigator.platform,browserDpr:devicePixelRatio,fontsLoaded:document.fonts.check('<span class="s">16px &quot;Noto Sans&quot;</span>'),lineMetrics:compareMetrics(lastDiagnostics)};
    document.getElementById('<span class="s">metadata</span>').textContent=JSON.stringify(lastReport,<span class="s">null</span>,<span class="s">2</span>);
    <span class="k">const</span> differences=lastReport.lineMetrics.map(widthDifference);
    <span class="k">const</span> maximum=Math.max(...differences);
    status.className=report.stats.missing_glyphs?'<span class="s">failure</span>':'';
    status.textContent=\`<span class="s">Ready. Maximum browser/shaper line-width difference: </span>\${<span class="s">maximum</span>.<span class="s">toFixed(3)</span>}<span class="s"> CSS px. Missing glyphs: </span>\${<span class="s">report</span>.<span class="s">stats</span>.<span class="s">missing_glyphs</span>}<span class="s">. Effective scale: </span>\${<span class="s">report</span>.<span class="s">effectiveScale</span>}<span class="s">.</span>\`;
    drawDiagnostics(lastDiagnostics);
    document.documentElement.dataset.ready='<span class="s">true</span>';
  } <span class="k">catch</span>(error) {fail(error)}
}
<span class="c">/** One centered nameplate; its browser twin is 25.5 px wider because of the padding. */</span>
<span class="k">function</span> renderNameplate() {
  <span class="k">const</span> scale=Number(document.getElementById('<span class="s">scale</span>').value);
  <span class="k">const</span> report=JSON.parse(fixture.render_nameplate(scale));
  reference.replaceChildren();reference.dataset.specimen='<span class="s">nameplate</span>';reference.style.background='<span class="s">#ffffff</span>';
  <span class="k">const</span> name=document.createElement('<span class="s">div</span>');name.className='<span class="s">reference</span>';name.textContent='<span class="s">Source sphere Ø25</span>';
  name.style.cssText='<span class="s">left:50%;top:50%;transform:translate(-50%,-50%);font-size:13.5px;line-height:19.5px;padding:3px 12.75px;border-radius:999px;background:#000;color:#fff</span>';
  reference.append(name);
  lastDiagnostics=JSON.parse(fixture.diagnostics());
  <span class="k">const</span> glyph=lastDiagnostics.glyphs[<span class="s">0</span>];
  <span class="k">const</span> lineMetrics=[{label:<span class="s">100</span>,line:<span class="s">0</span>,browser:name.getBoundingClientRect().width-<span class="s">25.5</span>,shaper:glyph.line_width}];
  lastReport={...report,browser:navigator.userAgent,browserDpr:devicePixelRatio,lineMetrics,nameplate:<span class="s">true</span>};
  document.getElementById('<span class="s">metadata</span>').textContent=JSON.stringify(lastReport,<span class="s">null</span>,<span class="s">2</span>);
  status.className=report.stats.missing_glyphs?'<span class="s">failure</span>':'';
  status.textContent=\`<span class="s">Ready. Centered 13.5px rounded white-on-black nameplate. Width difference: </span>\${<span class="s">widthDifference(lineMetrics[0])</span>.<span class="s">toFixed(3)</span>}<span class="s"> CSS px. Effective scale: </span>\${<span class="s">report</span>.<span class="s">effectiveScale</span>}<span class="s">.</span>\`;
  drawDiagnostics({glyphs:[],bitmaps:[],scale:report.effectiveScale});
  document.documentElement.dataset.ready='<span class="s">true</span>';
}
<span class="c">/** Compare line widths, not pixels: two rasterizers never agree pixel for pixel. */</span>
<span class="k">function</span> compareMetrics(diagnostics) {
  <span class="k">const</span> context=document.createElement('<span class="s">canvas</span>').getContext('<span class="s">2d</span>');
  context.fontKerning='<span class="s">normal</span>';<span class="k">const</span> metrics=[];
  <span class="k">for</span>(<span class="k">const</span> row of rows) {
    context.font=\`<span class="s">400 </span>\${<span class="s">row</span>.<span class="s">size</span>}<span class="s">px &quot;Noto Sans&quot;, &quot;Noto Sans Symbols&quot;, &quot;Noto Sans Symbols 2&quot;</span>\`;
    <span class="k">const</span> lines=row.text.split('<span class="s">\\n</span>');
    <span class="k">for</span>(<span class="k">let</span> line=<span class="s">0</span>;line&lt;lines.length;line++) {
      <span class="k">let</span> glyph;
      <span class="k">for</span>(<span class="k">const</span> item of diagnostics.glyphs){<span class="k">if</span>(item.label===row.id &amp;&amp; item.line===line){glyph=item;<span class="k">break</span>;}}
      <span class="k">if</span>(glyph) metrics.push({label:row.id,line,browser:context.measureText(lines[line]).width,shaper:glyph.line_width});
    }
  }
  <span class="k">return</span> metrics;
}
<span class="c">/** Width difference in CSS pixels. */</span>
<span class="k">function</span> widthDifference(row){<span class="k">return</span> Math.abs(row.browser-row.shaper)}
<span class="c">/** Draw baselines, glyph origins and bitmap boxes over the canvas. */</span>
<span class="k">function</span> drawDiagnostics(data) {
  <span class="k">const</span> overlay=document.getElementById('<span class="s">diagnostic-canvas</span>');overlay.width=canvas.width;overlay.height=canvas.height;
  <span class="k">if</span>(!document.getElementById('<span class="s">diagnostics</span>').checked)<span class="k">return</span>;
  <span class="k">const</span> ctx=overlay.getContext('<span class="s">2d</span>');ctx.lineWidth=<span class="s">0.5</span>;
  <span class="k">for</span>(<span class="k">const</span> glyph of data.bitmaps){ctx.strokeStyle='<span class="s">#f8795270</span>';ctx.strokeRect(...glyph.rect);ctx.fillStyle='<span class="s">#20dfff</span>';ctx.fillRect(glyph.origin[<span class="s">0</span>],glyph.origin[<span class="s">1</span>]-<span class="s">2</span>,<span class="s">1</span>,<span class="s">4</span>)}
  ctx.strokeStyle='<span class="s">#20dfff70</span>';
  <span class="k">for</span>(<span class="k">const</span> row of rows){
    <span class="k">const</span> baselines=new Set();
    <span class="k">for</span>(<span class="k">const</span> glyph of data.glyphs){<span class="k">if</span>(glyph.label===row.id)baselines.add(glyph.baseline);}
    <span class="k">for</span>(<span class="k">const</span> baseline of baselines){ctx.beginPath();ctx.moveTo(row.left*data.scale,(row.top+baseline)*data.scale);ctx.lineTo(canvas.width,(row.top+baseline)*data.scale);ctx.stroke()}
  }
}
<span class="c">/** Free the GPU text, then draw it again from scratch. */</span>
<span class="k">function</span> reset(){fixture.reset();render()}
<span class="c">/** Save the report and diagnostics as JSON. */</span>
<span class="k">function</span> download(){<span class="k">const</span> blob=new Blob([JSON.stringify({report:lastReport,diagnostics:lastDiagnostics},<span class="s">null</span>,<span class="s">2</span>)],{type:'<span class="s">application/json</span>'});<span class="k">const</span> url=URL.createObjectURL(blob);<span class="k">const</span> a=document.createElement('<span class="s">a</span>');a.href=url;a.download='<span class="s">text-quality.json</span>';a.click();URL.revokeObjectURL(url)}
<span class="c">/** Free the wasm object and stop listening for resizes. */</span>
<span class="k">function</span> dispose(){observer?.disconnect();window.removeEventListener('<span class="s">resize</span>',render);fixture?.free();fixture=<span class="s">null</span>}
<span class="c">/** Show a failure on the page, in a data attribute for automated checks, and in the console. */</span>
<span class="k">function</span> fail(error){status.className='<span class="s">failure</span>';status.textContent=String(error);document.documentElement.dataset.error=String(error);console.error(error)}
start().catch(fail);
&lt;/script&gt;
&lt;/html&gt;</code></pre></div>
<h2 id="step-11-indexhtml">Step 11 · index.html<a class="anchor" href="#/course/11-text-rendering#step-11-indexhtml" aria-label="Link to this section">#</a></h2>
<p>Copy this file from the lesson folder to the path shown.</p>
<p><code>lessons/11/index.html</code> · edit · copy the file</p>
<p>Replaces the <code>&lt;title&gt;Session checkpoint 10&lt;/title&gt;</code> line of <code>lessons/10/index.html</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    &lt;title&gt;Session checkpoint 11&lt;/title&gt;</code></pre></div>
<p>Replaces the 7 lines from <code>&lt;link data-trunk rel=&quot;copy-file&quot; href=&quot;assets…</code> of <code>lessons/10/index.html</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    &lt;link data-trunk rel=&quot;<span class="s">copy-file</span>&quot; href=&quot;<span class="s">assets/text-quality.html</span>&quot;&gt;
  &lt;/head&gt;
  &lt;body&gt;
    <span class="c">&lt;!-- wgpu wraps this canvas as the Surface; tabindex=&quot;0&quot; lets it take keyboard focus. --&gt;</span>
    &lt;canvas id=&quot;<span class="s">canvas</span>&quot; tabindex=&quot;<span class="s">0</span>&quot;&gt;&lt;/canvas&gt;
    &lt;a href=&quot;<span class="s">./text-quality.html</span>&quot; style=&quot;<span class="s">position: fixed; right: 12px; top: 12px; color: white;</span>&quot;&gt;Compare GPU text&lt;/a&gt;
    &lt;output id=&quot;<span class="s">status</span>&quot;&gt;Starting checkpoint 11&lt;/output&gt;</code></pre></div>
<p>Replaces the <code>document.getElementById(&#39;status&#39;).textContent…</code> line of <code>lessons/10/index.html</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        document.getElementById('status').textContent = 'Checkpoint 11 · ' + state.objects + ' objects · ' + state.width + '×' + state.height;</code></pre></div>
<h2 id="check">Check<a class="anchor" href="#/course/11-text-rendering#check" aria-label="Link to this section">#</a></h2>
<p>Run <code>trunk serve</code> in <code>lessons/11/</code> and open <a href="http://127.0.0.1:8770/" target="_blank" rel="noopener">http://127.0.0.1:8770/</a>.</p>
<p>Expected: Screen-sized nameplates and a foreshortened scene label appear above the model; status: <strong>3 objects</strong>.</p>
<p><img src="/session/docs/course/docs/screenshots/11.png" alt="Checkpoint 11: two nameplates above the sphere, one rounded, and a fixed-plane label foreshortened on its own plane." loading="lazy" decoding="async"></p>
<p>If it fails:</p>
<ul>
<li>Letters blur at one zoom level: the text frame scale disagrees with the framebuffer.</li>
<li>The last glyph clips: plate padding excludes the glyph overhang.</li>
</ul>
<h2 id="what-changed">What changed<a class="anchor" href="#/course/11-text-rendering#what-changed" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>lessons/11/src/engine/gpu/
├── arena.rs
├── backdrop.rs
├── buffers.rs
├── cloud.rs
├── frame.rs
├── glyphs.rs
├── instance.rs
├── lod.rs
├── mod.rs  ~
├── objects.rs
├── segments.rs
├── splat.rs
├── targets.rs
├── text.rs  +
├── text_outline.rs
├── text_plane.rs  +
├── text_plate.rs  +
├── upload.rs
└── view.rs</code></pre></div>
<p><code>+</code> new in this lesson · <code>~</code> changed in this lesson</p>
<p>Every file at this point: <code>lessons/11/</code>.</p>
<h2 id="next">Next<a class="anchor" href="#/course/11-text-rendering#next" aria-label="Link to this section">#</a></h2>
<p><a href="#/course/12-picking">12 · maintained viewer shell and picking</a></p>
<h2 id="expected-viewer-result">Expected viewer result<a class="anchor" href="#/course/11-text-rendering#expected-viewer-result" aria-label="Link to this section">#</a></h2>
<p>Checkpoint 11: two nameplates above the sphere, one rounded, and a fixed-plane label foreshortened on its own plane.</p>
<p><a href="/session/docs/course/docs/screenshots/11.png"><img src="/session/docs/course/docs/screenshots/11.png" alt="Full viewer result for 11 text rendering" loading="lazy" decoding="async"></a></p>
`,toc:[{level:2,id:"step-1-srcenginegputext_platers",text:"Step 1 · src/engine/gpu/text_plate.rs"},{level:2,id:"step-2-srcshaderstext_platewgsl",text:"Step 2 · src/shaders/text_plate.wgsl"},{level:2,id:"step-3-srcenginegputext_planers",text:"Step 3 · src/engine/gpu/text_plane.rs"},{level:2,id:"step-4-srcshaderstext_planewgsl",text:"Step 4 · src/shaders/text_plane.wgsl"},{level:2,id:"step-5-srcenginegputextrs",text:"Step 5 · src/engine/gpu/text.rs"},{level:2,id:"step-6-srcenginegpumodrs",text:"Step 6 · src/engine/gpu/mod.rs"},{level:2,id:"step-7-srclibrs",text:"Step 7 · src/lib.rs"},{level:2,id:"step-8-srctext_layoutrs",text:"Step 8 · src/text_layout.rs"},{level:2,id:"step-9-assetstext-layouthtml",text:"Step 9 · assets/text-layout.html"},{level:2,id:"step-10-assetstext-qualityhtml",text:"Step 10 · assets/text-quality.html"},{level:2,id:"step-11-indexhtml",text:"Step 11 · index.html"},{level:2,id:"check",text:"Check"},{level:2,id:"what-changed",text:"What changed"},{level:2,id:"next",text:"Next"},{level:2,id:"expected-viewer-result",text:"Expected viewer result"}]};export{s as default};
