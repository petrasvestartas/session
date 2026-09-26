const s={title:"01 · First WebGPU frame",html:`<h1 id="01-first-webgpu-frame">01 · First WebGPU frame<a class="anchor" href="#/course/01-first-frame#01-first-webgpu-frame" aria-label="Link to this section">#</a></h1>
<p>One triangle on a dark canvas.</p>
<p><img src="/session/docs/course/docs/illustrations/01-objects.svg" alt="What one WebGPU frame needs: nine objects made once in create(), seven steps repeated in every render(). The pipeline and bind group made on the left are what the pass on the right uses." loading="lazy" decoding="async"></p>
<p>Left column: made once. Right column: done every frame.</p>
<p>This lesson is the <strong>Shell</strong> and <strong>Shaders</strong> part of the map.</p>
<p><img src="/session/docs/course/docs/illustrations/map.svg" alt="The whole viewer as one map: documents come in along the top row, a frame is drawn along the bottom one." loading="lazy" decoding="async"></p>
<h2 id="step-1-srclibrs-part-1-the-struct">Step 1 · <code>src/lib.rs</code>, part 1: the struct<a class="anchor" href="#/course/01-first-frame#step-1-srclibrs-part-1-the-struct" aria-label="Link to this section">#</a></h2>
<p>Replace the whole file: one struct owns every GPU object, and its four public methods are what the page calls.</p>
<p><code>lessons/01/src/lib.rs</code> · type this, replace the whole file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> wasm_bindgen::prelude::*;
<span class="k">use</span> wgpu::util::DeviceExt; <span class="c">// a trait that adds create_buffer_init to Device; its methods work only once imported</span>

<span class="c">/// Everything needed to draw one frame: the browser owns the canvas, this struct owns the GPU.</span>
#[wasm_bindgen] <span class="c">// JavaScript sees this struct as a class</span>
<span class="k">pub</span> <span class="k">struct</span> Tutorial {
    canvas: web_sys::HtmlCanvasElement,
    surface: wgpu::Surface&lt;'static&gt;, <span class="c">// 'static: borrows nothing, so it may live as long as the struct</span>
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pipeline: wgpu::RenderPipeline,
    group: wgpu::BindGroup,
    scale: f64, <span class="c">// device pixels per CSS pixel, 2.0 on most phones</span>
}

#[wasm_bindgen] <span class="c">// every pub fn in this block becomes a JavaScript method</span>
<span class="k">impl</span> Tutorial {
    <span class="c">/// async: JavaScript receives a Promise, because asking the browser for a GPU takes a moment.</span>
    #[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)] <span class="c">// build this only for the browser: a PC has no canvas, and \`cargo xtest\` builds for the PC</span>
    <span class="k">pub</span> <span class="k">async</span> <span class="k">fn</span> create(canvas: web_sys::HtmlCanvasElement) -&gt; Result&lt;Tutorial, JsValue&gt; {
        console_error_panic_hook::set_once();
        <span class="k">Self</span>::open(canvas).<span class="k">await</span>.map_err(js_error) <span class="c">// JavaScript cannot read a Rust error, so it becomes text</span>
    }

    <span class="c">/// The page calls this after loading, on resize and on every drag; it returns a JSON status line.</span>
    <span class="k">pub</span> <span class="k">fn</span> render(&amp;<span class="k">mut</span> <span class="k">self</span>, width: u32, height: u32, scale: f64) -&gt; Result&lt;String, JsValue&gt; {
        <span class="k">self</span>.render_frame(width, height, scale).map_err(js_error)
    }

    <span class="c">/// Empty until lesson 02 adds the camera; the page already calls it.</span>
    <span class="k">pub</span> <span class="k">fn</span> drag(&amp;<span class="k">mut</span> <span class="k">self</span>, dx: f32, dy: f32, pan: bool) {
        <span class="k">let</span> _ = (dx, dy, pan); <span class="c">// \`let _ =\` silences the unused-argument warning</span>
    }

    <span class="c">/// Empty until lesson 02, like drag.</span>
    <span class="k">pub</span> <span class="k">fn</span> zoom(&amp;<span class="k">mut</span> <span class="k">self</span>, delta: f32, x: f64, y: f64) {
        <span class="k">let</span> _ = (delta, x, y);
    }
}</code></pre></div>
<h2 id="step-2-part-2-instance-surface-adapter-device">Step 2 · part 2: instance, surface, adapter, device<a class="anchor" href="#/course/01-first-frame#step-2-part-2-instance-surface-adapter-device" aria-label="Link to this section">#</a></h2>
<p>Append: create the GPU connection in four steps, instance → surface → adapter → device and queue.</p>
<p><img src="/session/docs/course/docs/illustrations/01-gpu-chain.svg" alt="The four objects of step 2 and what each one is for. Grey boxes are made in open(), used once and dropped: the Instance is the WebGPU API itself, the Adapter is one physical GPU. Pink boxes are stored in Tutorial and used by every later step: the Surface hands out one texture per frame, the Device makes every GPU object, the Queue is where finished commands are submitted. The dashed notes say which later step uses each one." loading="lazy" decoding="async"></p>
<p><code>lessons/01/src/lib.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// A second impl block without #[wasm_bindgen]: these helpers return Rust errors JavaScript cannot see.</span>
<span class="k">impl</span> Tutorial {
    <span class="c">/// anyhow::Result = Ok(value) or Err(any error with a message).</span>
    #[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
    <span class="k">async</span> <span class="k">fn</span> open(canvas: web_sys::HtmlCanvasElement) -&gt; anyhow::Result&lt;<span class="k">Self</span>&gt; {
        <span class="c">// Instance = the WebGPU API itself</span>
        <span class="c">// Surface  = the canvas, as something the GPU can draw into</span>
        <span class="c">// Adapter  = one physical GPU that can draw to that surface</span>
        <span class="c">// Device   = your connection to that GPU; it creates every GPU object</span>
        <span class="c">// Queue    = where finished command lists are sent</span>

        <span class="k">let</span> instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::BROWSER_WEBGPU, <span class="c">// WebGPU only, no WebGL fallback</span>
            flags: Default::default(),
            memory_budget_thresholds: Default::default(),
            backend_options: Default::default(),
            display: None,
        });

        <span class="c">// \`?\` returns the error to the caller at once, so below this line the surface exists.</span>
        <span class="c">// clone() makes a second handle to the same canvas; the struct keeps the first.</span>
        <span class="k">let</span> surface = instance.create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone()))?;

        <span class="k">let</span> adapter = instance
            .request_adapter(&amp;wgpu::RequestAdapterOptions {
                compatible_surface: Some(&amp;surface),
                ..Default::default() <span class="c">// every field not written above keeps its default</span>
            })
            .<span class="k">await</span>?; <span class="c">// await: wait for the browser's answer without freezing the page</span>

        log::info!(&quot;<span class="s">tutorial adapter: </span>{<span class="s">:?</span>}&quot;, adapter.get_info());

        <span class="k">let</span> (device, queue) = adapter
            .request_device(&amp;wgpu::DeviceDescriptor::default())
            .<span class="k">await</span>?;

        <span class="c">// A validation error would otherwise only log a warning and leave the canvas black.</span>
        device.on_uncaptured_error(std::sync::Arc::new(gpu_error));</code></pre></div>
<h2 id="step-3-part-3-surface-configuration-and-the-camera-uniform">Step 3 · part 3: surface configuration and the camera uniform<a class="anchor" href="#/course/01-first-frame#step-3-part-3-surface-configuration-and-the-camera-uniform" aria-label="Link to this section">#</a></h2>
<p>Append: describe the canvas texture, and give the shader one 4×4 matrix through a uniform buffer.</p>
<p><code>lessons/01/src/lib.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="c">// What this canvas supports on this GPU; index 0 of each list is the preferred choice.</span>
        <span class="k">let</span> caps = surface.get_capabilities(&amp;adapter);

        <span class="k">let</span> config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT, <span class="c">// a render pass may draw into it</span>
            format: caps.formats[<span class="s">0</span>], <span class="c">// e.g. Bgra8UnormSrgb: 4 bytes per pixel, blue first</span>
            width: <span class="s">1</span>,                <span class="c">// 1 x 1 for now; the first frame sets the real size</span>
            height: <span class="s">1</span>,
            present_mode: caps.present_modes[<span class="s">0</span>], <span class="c">// on the web always Fifo: one frame per display refresh</span>
            alpha_mode: caps.alpha_modes[<span class="s">0</span>],     <span class="c">// whether the canvas blends with the page behind it</span>
            view_formats: vec![],
            desired_maximum_frame_latency: <span class="s">2</span>,    <span class="c">// at most 2 frames queued ahead of the screen</span>
        };

        <span class="c">// The identity matrix: points pass through unchanged until lesson 02 adds a camera.</span>
        <span class="k">let</span> identity = [
            <span class="s">1</span>.<span class="s">0f32</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>,
        ];

        <span class="c">// A uniform buffer is one small block of GPU memory that every vertex reads, the same for all of them.</span>
        <span class="k">let</span> uniform = device.create_buffer_init(&amp;wgpu::util::BufferInitDescriptor {
            label: Some(&quot;<span class="s">camera matrix</span>&quot;),
            contents: bytemuck::cast_slice(&amp;identity), <span class="c">// the same 64 bytes seen as &amp;[u8]; nothing is copied</span>
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, <span class="c">// shaders read it; write_buffer may overwrite it</span>
        });

        <span class="c">// A bind group layout is the shape of the shader's inputs; the bind group below fills it with real buffers.</span>
        <span class="k">let</span> layout = device.create_bind_group_layout(&amp;wgpu::BindGroupLayoutDescriptor {
            label: Some(&quot;<span class="s">camera</span>&quot;),
            entries: &amp;[wgpu::BindGroupLayoutEntry {
                binding: <span class="s">0</span>, <span class="c">// @binding(0) in first.wgsl</span>
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: <span class="s">false</span>, <span class="c">// true would let one buffer hold several cameras, chosen per draw</span>
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        <span class="k">let</span> group = device.create_bind_group(&amp;wgpu::BindGroupDescriptor {
            label: Some(&quot;<span class="s">camera</span>&quot;),
            layout: &amp;layout,
            entries: &amp;[wgpu::BindGroupEntry {
                binding: <span class="s">0</span>,                            <span class="c">// fills the layout entry with the same number</span>
                resource: uniform.as_entire_binding(),
            }],
        });</code></pre></div>
<h2 id="step-4-part-4-shader-module-and-pipeline">Step 4 · part 4: shader module and pipeline<a class="anchor" href="#/course/01-first-frame#step-4-part-4-shader-module-and-pipeline" aria-label="Link to this section">#</a></h2>
<p>Append: load the shader and build the pipeline, the fixed recipe for drawing.</p>
<p><code>lessons/01/src/lib.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="c">// include_str! pastes the shader text into the binary at compile time, so a missing file stops the build.</span>
        <span class="k">let</span> shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(&quot;<span class="s">first triangle</span>&quot;),
            source: wgpu::ShaderSource::Wgsl(include_str!(&quot;<span class="s">shaders/first.wgsl</span>&quot;).into()),
        });

        <span class="c">// One bind group layout per @group number; this pipeline has only group 0.</span>
        <span class="k">let</span> pipeline_layout = device.create_pipeline_layout(&amp;wgpu::PipelineLayoutDescriptor {
            label: Some(&quot;<span class="s">first triangle</span>&quot;),
            bind_group_layouts: &amp;[Some(&amp;layout)],
            immediate_size: <span class="s">0</span>,
        });

        <span class="c">// Create render pipeline: the full set of rules the GPU uses to draw, built and checked once.</span>
        <span class="k">let</span> pipeline = device.create_render_pipeline(&amp;wgpu::RenderPipelineDescriptor {
            label: Some(&quot;<span class="s">first triangle</span>&quot;),
            layout: Some(&amp;pipeline_layout),
            vertex: wgpu::VertexState {
                module: &amp;shader,
                entry_point: Some(&quot;<span class="s">vs_main</span>&quot;), <span class="c">// the function name in first.wgsl</span>
                buffers: &amp;[], <span class="c">// no vertex buffer: the shader makes its three corners itself</span>
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &amp;shader,
                entry_point: Some(&quot;<span class="s">fs_main</span>&quot;),
                targets: &amp;[Some(wgpu::ColorTargetState {
                    <span class="c">// must equal the canvas format, or the pass is rejected</span>
                    format: config.format,
                    blend: None,                        <span class="c">// overwrite the pixel, no transparency</span>
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: Default::default(), <span class="c">// a list of triangles, none culled</span>
            depth_stencil: None, <span class="c">// no depth test yet: a later draw simply covers an earlier one</span>
            multisample: Default::default(), <span class="c">// 1 sample per pixel, no anti-aliasing</span>
            multiview_mask: None,
            cache: None,
        });

        <span class="c">// Create an instance of the struct; Ok is needed to return also the error message Err(...).</span>
        Ok(<span class="k">Self</span> {
            canvas,
            surface,
            device,
            queue,
            config,
            pipeline,
            group,
            scale: <span class="s">1</span>.<span class="s">0</span>,
        })
    }</code></pre></div>
<h2 id="step-5-part-5-one-frame">Step 5 · part 5: one frame<a class="anchor" href="#/course/01-first-frame#step-5-part-5-one-frame" aria-label="Link to this section">#</a></h2>
<p>Append: draw one frame — resize if needed, clear the canvas, draw three vertices, present.</p>
<p><code>lessons/01/src/lib.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// One frame: resize if needed, record the commands, submit them, show the result.</span>
    <span class="k">fn</span> render_frame(&amp;<span class="k">mut</span> <span class="k">self</span>, width: u32, height: u32, scale: f64) -&gt; anyhow::Result&lt;String&gt; {
        anyhow::ensure!(scale.is_finite() &amp;&amp; scale &gt; <span class="s">0</span>.<span class="s">0</span>, &quot;<span class="s">invalid device scale</span>&quot;); <span class="c">// returns Err when false</span>

        <span class="k">self</span>.scale = scale;

        <span class="c">// Convert the logical size to real pixel size, e.g. width = 800, scale = 2.0 gives 1600.</span>
        <span class="k">let</span> width = (f64::from(width.max(<span class="s">1</span>)) * scale).round() <span class="k">as</span> u32;
        <span class="k">let</span> height = (f64::from(height.max(<span class="s">1</span>)) * scale).round() <span class="k">as</span> u32;

        <span class="c">// Reconfigure only when the size changed: it reallocates the canvas textures.</span>
        <span class="k">if</span> <span class="k">self</span>.canvas.width() != width || <span class="k">self</span>.canvas.height() != height || <span class="k">self</span>.config.width == <span class="s">1</span>
        {
            <span class="k">self</span>.canvas.set_width(width);
            <span class="k">self</span>.canvas.set_height(height);
            <span class="k">self</span>.config.width = width;
            <span class="k">self</span>.config.height = height;
            <span class="k">self</span>.surface.configure(&amp;<span class="k">self</span>.device, &amp;<span class="k">self</span>.config);
        }

        <span class="c">// The texture the canvas shows next; any result other than these two ends the frame with an error.</span>
        <span class="k">let</span> output = <span class="k">match</span> <span class="k">self</span>.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(output)
            | wgpu::CurrentSurfaceTexture::Suboptimal(output) =&gt; output,
            error =&gt; anyhow::bail!(&quot;<span class="s">surface unavailable: </span>{<span class="s">error:?</span>}&quot;),
        };

        <span class="c">// A view is a handle onto that texture saying how to read or write it; the pass draws through it.</span>
        <span class="k">let</span> view = output.texture.create_view(&amp;Default::default());

        <span class="c">// The GPU does not execute calls one by one, you record a list of commands and hand over this encoder.</span>
        <span class="k">let</span> <span class="k">mut</span> encoder = <span class="k">self</span>.device.create_command_encoder(&amp;Default::default());

        <span class="c">// The inner braces end the pass early, because it borrows the encoder and finish() needs it back.</span>
        {
            <span class="k">let</span> <span class="k">mut</span> pass = encoder.begin_render_pass(&amp;wgpu::RenderPassDescriptor {
                label: Some(&quot;<span class="s">first frame</span>&quot;),
                color_attachments: &amp;[Some(wgpu::RenderPassColorAttachment {
                    view: &amp;view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        <span class="c">// fill with dark blue before drawing; channels run 0 to 1, not 0 to 255</span>
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: <span class="s">0</span>.<span class="s">025</span>,
                            g: <span class="s">0</span>.<span class="s">035</span>,
                            b: <span class="s">0</span>.<span class="s">055</span>,
                            a: <span class="s">1</span>.<span class="s">0</span>,
                        }),
                        store: wgpu::StoreOp::Store, <span class="c">// keep the pixels, so they can be shown</span>
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            pass.set_pipeline(&amp;<span class="k">self</span>.pipeline);
            pass.set_bind_group(<span class="s">0</span>, &amp;<span class="k">self</span>.group, &amp;[]); <span class="c">// 0 = @group(0) in the shader; &amp;[] = no dynamic offsets</span>
            pass.draw(<span class="s">0</span>..<span class="s">3</span>, <span class="s">0</span>..<span class="s">1</span>); <span class="c">// vertices 0..3, instances 0..1: the vertex shader runs three times</span>
        }

        <span class="c">// The GPU starts working only now.</span>
        <span class="k">self</span>.queue.submit([encoder.finish()]);

        <span class="c">// The canvas shows the new frame at the next display refresh.</span>
        output.present();

        <span class="c">// The page prints this JSON in its status line.</span>
        Ok(serde_json::json!({
            &quot;<span class="s">stage</span>&quot; : <span class="s">1</span>,
            &quot;<span class="s">objects</span>&quot; : <span class="s">1</span>,
            &quot;<span class="s">width</span>&quot; : width,
            &quot;<span class="s">height</span>&quot; : height,
            &quot;<span class="s">scale</span>&quot; : scale,
            &quot;<span class="s">drawn</span>&quot; : <span class="s">true</span>,
        })
        .to_string())
    }
}

<span class="c">/// \`impl Display\` accepts any error type that can print itself.</span>
<span class="k">fn</span> js_error(error: <span class="k">impl</span> std::fmt::Display) -&gt; JsValue {
    JsValue::from_str(&amp;error.to_string())
}

<span class="c">/// Panic, so a validation error appears in the console instead of a silently black canvas.</span>
<span class="k">fn</span> gpu_error(error: wgpu::Error) {
    panic!(&quot;<span class="s">tutorial WebGPU error: </span>{<span class="s">error</span>}&quot;);
}</code></pre></div>
<h2 id="step-6-srcshadersfirstwgsl">Step 6 · <code>src/shaders/first.wgsl</code><a class="anchor" href="#/course/01-first-frame#step-6-srcshadersfirstwgsl" aria-label="Link to this section">#</a></h2>
<p>New file: the vertex shader places three corners, the fragment shader colors the pixels between them.</p>
<p><code>lessons/01/src/shaders/first.wgsl</code> · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// The @group and @binding numbers must match the bind group layout built in Rust, or the draw is rejected.</span>
@group(<span class="s">0</span>) @binding(<span class="s">0</span>) <span class="k">var</span>&lt;<span class="k">uniform</span>&gt; mvp: <span class="k">mat4x4</span>&lt;<span class="k">f32</span>&gt;;<span class="c"> // mvp = model, view and projection in one 4x4 matrix</span>

<span class="c">// Vertex output, fragment input</span>
<span class="k">struct</span> VertexOut{
    @builtin(position) position: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // the one output the GPU requires</span>
    @location(<span class="s">0</span>) color: <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;,
}

<span class="c">// Runs once per vertex: draw(0..3, ..) gives index 0, 1 and 2.</span>
@vertex
<span class="k">fn</span> vs_main(@builtin(vertex_index) index: <span class="k">u32</span>) -&gt; VertexOut{
    <span class="k">let</span> points = <span class="k">array</span>&lt;<span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;, <span class="s">3</span>&gt;(
        <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(-<span class="s">0.7</span>, -<span class="s">0.55</span>, <span class="s">0.0</span>),
        <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.7</span>, -<span class="s">0.55</span>, <span class="s">0.0</span>),
        <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>, <span class="s">0.65</span>, <span class="s">0.0</span>)
    );

    <span class="k">let</span> colors = <span class="k">array</span>&lt;<span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;, <span class="s">3</span>&gt;(
        <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.95</span>, <span class="s">0.25</span>, <span class="s">0.2</span>),
        <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.2</span>, <span class="s">0.8</span>, <span class="s">0.45</span>),
        <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.3</span>, <span class="s">0.5</span>, <span class="s">1.0</span>)
    );

    <span class="k">var</span> output: VertexOut;
<span class="c">    // Clip space: whatever you return here, x and y from -1 to 1 is the visible square, and the GPU maps it to pixels.</span>
    output.position = mvp * <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(points[index], <span class="s">1.0</span>);
    output.color = colors[index];
    <span class="k">return</span> output;
}

<span class="c">// Runs once per pixel with the interpolated vertex output.</span>
@fragment
<span class="k">fn</span> fs_main(input: VertexOut) -&gt; @location(<span class="s">0</span>) <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;{
<span class="c">    // The three corner colors are blended across the triangle before this runs, which is why it looks like a gradient.</span>
    <span class="k">return</span> <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(input.color, <span class="s">1.0</span>);
}</code></pre></div>
<p>Run <code>cargo check</code> in <code>lessons/01/</code>.</p>
<p>Left: clip space, as the shader sees it. Right: the canvas in pixels.
<img src="/session/docs/course/docs/illustrations/clip-space.svg" alt="Clip space is a square from -1 to +1 with y up; the viewport transform turns it into pixels with y down." loading="lazy" decoding="async"></p>
<h2 id="step-7-indexhtml">Step 7 · <code>index.html</code><a class="anchor" href="#/course/01-first-frame#step-7-indexhtml" aria-label="Link to this section">#</a></h2>
<p>Replace the whole file: the page gets a canvas and calls <code>create</code> once, then <code>render</code> on every resize and pointer event.</p>
<p><code>lessons/01/index.html</code> · edit · copy the file</p>
<p>Replaces the lines from <code>&lt;title&gt;Session checkpoint 00&lt;/title&gt;</code> to <code>&lt;output id=&quot;status&quot;&gt;Loading WASM&lt;/output&gt;</code> in <code>lessons/00/index.html</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">&lt;!-- Without this a phone lays the page out 980 px wide and shrinks it to fit. --&gt;</span>
    &lt;meta name=&quot;<span class="s">viewport</span>&quot; content=&quot;<span class="s">width=device-width, initial-scale=1</span>&quot;&gt;
    &lt;title&gt;01 - First WebGPU frame&lt;/title&gt;
    <span class="c">&lt;!-- data-target-name = the crate to compile, the name in Cargo.toml. --&gt;</span>
    &lt;link data-trunk rel=&quot;<span class="s">rust</span>&quot; data-target-name=&quot;<span class="s">session_viewer</span>&quot; data-wasm-opt=&quot;<span class="s">0</span>&quot;&gt;
    &lt;style&gt;
        html, body {
            margin: <span class="s">0</span>;
            width: <span class="s">100</span><span class="k">%</span>;
            height: <span class="s">100</span><span class="k">%</span>;
            background: #171b23;
            color: white;
            font: <span class="s">14</span><span class="k">px</span> sans-serif;
        }
        canvas {
            display: block;
            width: <span class="s">100</span><span class="k">%</span>;
            height: <span class="s">100</span><span class="k">%</span>;
            touch-action: none;
        }
        #status {
            position: fixed;
            left: <span class="s">12</span><span class="k">px</span>;
            top: <span class="s">12</span><span class="k">px</span>;
            background: #171b23cc;
            padding: <span class="s">8</span><span class="k">px</span>;
            pointer-events: none;
        }
    &lt;/style&gt;
&lt;/head&gt;

&lt;body&gt;
    <span class="c">&lt;!-- wgpu wraps this canvas as the Surface; tabindex=&quot;0&quot; lets it take keyboard focus. --&gt;</span>
    &lt;canvas id=&quot;<span class="s">canvas</span>&quot; tabindex=&quot;<span class="s">0</span>&quot;&gt;&lt;/canvas&gt;
    &lt;output id=&quot;<span class="s">status</span>&quot;&gt;Starting checkpoint 01&lt;/output&gt;
    &lt;script&gt;

        <span class="k">let</span> tutorial; <span class="c">// the Rust Tutorial, once create() has finished</span>
        <span class="k">let</span> pointer; <span class="c">// [x, y] of the last event while dragging, else undefined</span>

        <span class="k">async</span> <span class="k">function</span> start(){

            <span class="c">// Trunk sets window.wasmBindings once the .wasm has loaded; until then, try again every 20 ms.</span>
            <span class="k">if</span> (!window.wasmBindings){
                setTimeout(start, <span class="s">20</span>);
                <span class="k">return</span>;
            }

            <span class="k">try</span> {

                tutorial = <span class="k">await</span> window.wasmBindings.Tutorial.create(document.getElementById('<span class="s">canvas</span>'));

                <span class="c">// Try it in the console: tutorial.render(800, 600, 1)</span>
                window.tutorial = tutorial;

                render();
            } <span class="k">catch</span> (error){
                document.getElementById('<span class="s">status</span>').textContent = String(error);
                <span class="k">throw</span> error;
            }
        }

        <span class="c">// Draw one frame, then show the status Rust returns.</span>
        <span class="k">function</span> render(){
            <span class="k">const</span> canvas = document.getElementById('<span class="s">canvas</span>');
            <span class="k">const</span> state = JSON.parse(tutorial.render(canvas.clientWidth, canvas.clientHeight, devicePixelRatio));
            canvas.dataset.tutorialInspection = JSON.stringify(state); <span class="c">// kept on the canvas for automated checks</span>
            document.getElementById('<span class="s">status</span>').textContent = '<span class="s">Checkpoint 01 · </span>' + state.objects + '<span class="s"> objects · </span>' + state.width + '<span class="s">×</span>' + state.height;
        }

        <span class="k">function</span> down(event){
            pointer = [event.clientX, event.clientY];
            event.target.setPointerCapture(event.pointerId); <span class="c">// keep receiving moves when the pointer leaves the canvas</span>
        }

        <span class="k">function</span> move(event){
            <span class="k">if</span> (!pointer) <span class="c">// not dragging</span>
                <span class="k">return</span>;
            <span class="k">const</span> dx = event.clientX - pointer[<span class="s">0</span>], dy = event.clientY - pointer[<span class="s">1</span>]; <span class="c">// CSS pixels since the last event</span>
            pointer = [event.clientX, event.clientY];
            tutorial.drag(dx, dy, event.shiftKey); <span class="c">// Shift = pan, otherwise orbit</span>
            render(); <span class="c">// no animation loop: a frame is drawn only when something changed</span>
        }

        <span class="k">function</span> up(){
            pointer = <span class="s">undefined</span>;
        }

        <span class="k">function</span> wheel(event){
            event.preventDefault(); <span class="c">// the page must not scroll</span>
            tutorial.zoom(-event.deltaY / <span class="s">100</span>, event.offsetX, event.offsetY); <span class="c">// one wheel notch is deltaY 100 = one step; up zooms in</span>
            render();
        }

        <span class="k">function</span> resize() {
            <span class="k">if</span> (tutorial) <span class="c">// resize can fire before create() has finished</span>
                render();
        }

        <span class="k">const</span> canvas = document.getElementById('<span class="s">canvas</span>');

        <span class="c">// Pointer events cover mouse, pen and touch with one set of handlers.</span>
        canvas.addEventListener('<span class="s">pointerdown</span>', down);
        canvas.addEventListener('<span class="s">pointermove</span>', move);
        canvas.addEventListener('<span class="s">pointerup</span>', up);
        canvas.addEventListener('<span class="s">pointercancel</span>', up);
        canvas.addEventListener('<span class="s">wheel</span>', wheel, {passive: <span class="s">false</span>}); <span class="c">// passive: false, or preventDefault is ignored</span>
        window.addEventListener('<span class="s">resize</span>', resize);
        start();

    &lt;/script&gt;</code></pre></div>
<h2 id="check">Check<a class="anchor" href="#/course/01-first-frame#check" aria-label="Link to this section">#</a></h2>
<p>Run <code>trunk serve</code> in <code>lessons/01/</code> and open <a href="http://127.0.0.1:8770/" target="_blank" rel="noopener">http://127.0.0.1:8770/</a>.</p>
<p>Expected: a red, green and blue triangle and the status <strong>Checkpoint 01 · 1 objects · W×H</strong>; it stretches with the window.</p>
<p><img src="/session/docs/course/docs/screenshots/01.png" alt="Checkpoint 01: the first triangle, colors interpolated from the three vertices." loading="lazy" decoding="async"></p>
<p>If it fails:</p>
<ul>
<li>Dark canvas, no triangle: one of the three names in step 6 does not match the Rust, or <code>draw(0..3, 0..1)</code> was mistyped.</li>
<li>Status shows an error instead of the size: read it. It is the adapter or device request failing, and the browser has no WebGPU.</li>
<li>Canvas stays empty and the console shows <em>tutorial WebGPU error</em>: a validation error; the message names the descriptor field.</li>
</ul>
<h2 id="what-changed">What changed<a class="anchor" href="#/course/01-first-frame#what-changed" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>lessons/01/src/
├── shaders/
│   └── first.wgsl  +
└── lib.rs  ~</code></pre></div>
<p><code>+</code> new in this lesson · <code>~</code> changed in this lesson</p>
<p>Every file at this point: <code>lessons/01/</code>.</p>
<h2 id="next">Next<a class="anchor" href="#/course/01-first-frame#next" aria-label="Link to this section">#</a></h2>
<p><a href="#/course/02-camera">02 · Camera</a></p>
<h2 id="expected-viewer-result">Expected viewer result<a class="anchor" href="#/course/01-first-frame#expected-viewer-result" aria-label="Link to this section">#</a></h2>
<p>Checkpoint 01: the first triangle, colors interpolated from the three vertices.</p>
<p><a href="/session/docs/course/docs/screenshots/01.png"><img src="/session/docs/course/docs/screenshots/01.png" alt="Full viewer result for 01 first frame" loading="lazy" decoding="async"></a></p>
`,toc:[{level:2,id:"step-1-srclibrs-part-1-the-struct",text:"Step 1 · src/lib.rs , part 1: the struct"},{level:2,id:"step-2-part-2-instance-surface-adapter-device",text:"Step 2 · part 2: instance, surface, adapter, device"},{level:2,id:"step-3-part-3-surface-configuration-and-the-camera-uniform",text:"Step 3 · part 3: surface configuration and the camera uniform"},{level:2,id:"step-4-part-4-shader-module-and-pipeline",text:"Step 4 · part 4: shader module and pipeline"},{level:2,id:"step-5-part-5-one-frame",text:"Step 5 · part 5: one frame"},{level:2,id:"step-6-srcshadersfirstwgsl",text:"Step 6 · src/shaders/first.wgsl"},{level:2,id:"step-7-indexhtml",text:"Step 7 · index.html"},{level:2,id:"check",text:"Check"},{level:2,id:"what-changed",text:"What changed"},{level:2,id:"next",text:"Next"},{level:2,id:"expected-viewer-result",text:"Expected viewer result"}]};export{s as default};
