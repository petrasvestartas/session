const s={title:"12 · maintained viewer shell and picking",html:`<h1 id="12-maintained-viewer-shell-and-picking">12 · maintained viewer shell and picking<a class="anchor" href="#/course/12-picking#12-maintained-viewer-shell-and-picking" aria-label="Link to this section">#</a></h1>
<p>The seven-object fixture supports object and source-edge selection.</p>
<p><img src="/session/docs/course/docs/illustrations/picking.svg" alt="A pointer release becomes a scissored ID window, an asynchronous bounded readback, a Scene lookup and a selected flag; stale generations are dropped." loading="lazy" decoding="async"></p>
<p>Copy each file from the lesson folder to the path shown.</p>
<p>Copy from <code>lessons/12/</code> (tooling this checkpoint needs but the course does not teach):</p>
<ul>
<li><code>lessons/12/assets/pb/interaction.pb.json</code></li>
<li><code>lessons/12/src/selftest/lifecycle.rs</code></li>
<li><code>lessons/12/assets/pb/interaction.pb</code> (binary)</li>
</ul>
<h2 id="step-1-srcenginegpudevicers">Step 1 · src/engine/gpu/device.rs<a class="anchor" href="#/course/12-picking#step-1-srcenginegpudevicers" aria-label="Link to this section">#</a></h2>
<p>New file: open the GPU as in lesson 01, now with adapter choice, a larger buffer limit and stored errors.</p>
<p><code>lessons/12/src/engine/gpu/device.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! The adapter and device request, moved out of lib.rs now that the viewer is more than one file.</span>
<span class="k">use</span> std::sync::Arc;
<span class="k">use</span> winit::window::Window;

<span class="c">/// Everything \`open\` set up.</span>
<span class="k">pub</span> <span class="k">struct</span> DeviceSetup {
    <span class="k">pub</span> surface: Option&lt;wgpu::Surface&lt;'static&gt;&gt;, <span class="c">// the canvas; None when headless</span>
    <span class="k">pub</span> device: wgpu::Device,
    <span class="k">pub</span> queue: wgpu::Queue,
    <span class="k">pub</span> config: wgpu::SurfaceConfiguration,
    <span class="k">pub</span> device_type: wgpu::DeviceType, <span class="c">// discrete, integrated or CPU</span>
    <span class="k">pub</span> failure: Arc&lt;std::sync::Mutex&lt;Option&lt;String&gt;&gt;&gt;, <span class="c">// Arc = shared owner, Mutex = one writer at a time: error callbacks write, each frame reads</span>
}

<span class="c">/// Open the GPU: instance, surface, adapter, device, surface config.</span>
<span class="k">pub</span> <span class="k">async</span> <span class="k">fn</span> open(window: Option&lt;Arc&lt;Window&gt;&gt;, size: (u32, u32)) -&gt; anyhow::Result&lt;DeviceSetup&gt; {
    <span class="c">// PRIMARY = Vulkan, Metal or DX12, whichever the OS has</span>
    <span class="k">let</span> backends = <span class="k">if</span> cfg!(target_arch = &quot;<span class="s">wasm32</span>&quot;) {
        wgpu::Backends::BROWSER_WEBGPU
    } <span class="k">else</span> {
        wgpu::Backends::PRIMARY
    };
    <span class="k">let</span> instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends,
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
        display: None,
    });

    <span class="c">// no window in native tests: they render off-screen</span>
    <span class="k">let</span> surface = <span class="k">match</span> &amp;window {
        Some(w) =&gt; Some(instance.create_surface(w.clone())?),
        None =&gt; None,
    };

    <span class="c">// native: the integrated GPU keeps a laptop cool; the browser decides for itself</span>
    <span class="k">let</span> default_power = <span class="k">if</span> cfg!(target_arch = &quot;<span class="s">wasm32</span>&quot;) {
        wgpu::PowerPreference::None
    } <span class="k">else</span> {
        wgpu::PowerPreference::LowPower
    };
    <span class="c">// ?gpu=high in the URL, or VIEWER_GPU=high natively, asks for the fast GPU</span>
    <span class="k">let</span> preferred = <span class="k">if</span> super::view::knob(&quot;<span class="s">VIEWER_GPU</span>&quot;, &quot;<span class="s">gpu</span>&quot;).as_deref() == Some(&quot;<span class="s">high</span>&quot;) {
        wgpu::PowerPreference::HighPerformance
    } <span class="k">else</span> {
        default_power
    };
    <span class="k">let</span> options = |power_preference| wgpu::RequestAdapterOptions {
        power_preference,
        compatible_surface: surface.as_ref(),
        force_fallback_adapter: <span class="s">false</span>,
    };
    <span class="c">// the GPU: named, else preferred, else default</span>
    <span class="k">let</span> adapter = <span class="k">match</span> named_adapter(&amp;instance, backends).<span class="k">await</span> {
        Some(named) =&gt; named,
        None =&gt; <span class="k">match</span> instance.request_adapter(&amp;options(preferred)).<span class="k">await</span> {
            Ok(adapter) =&gt; adapter,
            Err(_) <span class="k">if</span> preferred != default_power =&gt; {
                instance.request_adapter(&amp;options(default_power)).<span class="k">await</span>?
            }
            Err(error) =&gt; <span class="k">return</span> Err(error.into()),
        },
    };
    <span class="k">let</span> info = adapter.get_info();
    log::info!(
        &quot;<span class="s">adapter: </span>{}<span class="s"> (</span>{<span class="s">:?</span>}<span class="s">, </span>{<span class="s">:?</span>}<span class="s">)</span>&quot;,
        info.name,
        info.device_type,
        info.backend
    );

    <span class="k">if</span> info.device_type == wgpu::DeviceType::Cpu {
        log::warn!(&quot;<span class="s">software adapter - rendering on the CPU will be slow</span>&quot;);
    }</code></pre></div>
<p><code>lessons/12/src/engine/gpu/device.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">// the default is 128 MiB per storage buffer; big meshes and clouds need more, so take up to 256 MiB</span>
    <span class="k">let</span> limits = wgpu::Limits {
        max_storage_buffer_binding_size: adapter
            .limits()
            .max_storage_buffer_binding_size
            .min(<span class="s">256</span> * <span class="s">1024</span> * <span class="s">1024</span>),
        ..wgpu::Limits::default()
    };

    <span class="k">let</span> (device, queue) = adapter
        .request_device(&amp;wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            required_limits: limits,
            memory_hints: Default::default(),
            ..Default::default()
        })
        .<span class="k">await</span>?;
    <span class="c">// a browser callback must not panic, so it stores the message for the next frame to report</span>
    <span class="k">let</span> failure = Arc::new(std::sync::Mutex::new(None));
    #[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
    {
        <span class="k">let</span> errors = failure.clone();
        device.on_uncaptured_error(Arc::new(<span class="k">move</span> |error| remember_gpu_error(&amp;errors, error)));
        <span class="k">let</span> lost = failure.clone();
        device.set_device_lost_callback(<span class="k">move</span> |reason, message| {
            remember_device_loss(&amp;lost, reason, &amp;message)
        });
    }
    #[cfg(not(target_arch = &quot;<span class="s">wasm32</span>&quot;))]
    device.on_uncaptured_error(Arc::new(report_gpu_error));

    <span class="c">// sRGB: the GPU converts our linear colours for the screen</span>
    <span class="k">let</span> (format, present_mode, alpha_mode) = <span class="k">match</span> &amp;surface {
        Some(s) =&gt; {</code></pre></div>
<p><code>lessons/12/src/engine/gpu/device.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">let</span> caps = s.get_capabilities(&amp;adapter);
            <span class="k">let</span> <span class="k">mut</span> f = caps.formats[<span class="s">0</span>];

            <span class="k">for</span> format <span class="k">in</span> &amp;caps.formats {
                <span class="k">if</span> format.is_srgb() {
                    f = *format;
                    <span class="k">break</span>;
                }
            }

            (f, caps.present_modes[<span class="s">0</span>], caps.alpha_modes[<span class="s">0</span>])
        }
        None =&gt; (
            <span class="c">// no surface: the format off-screen tests render into</span>
            wgpu::TextureFormat::Rgba8UnormSrgb,
            wgpu::PresentMode::Fifo,
            wgpu::CompositeAlphaMode::Auto,
        ),
    };
    <span class="k">let</span> config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format,
        width: size.<span class="s">0</span>.max(<span class="s">1</span>),
        height: size.<span class="s">1</span>.max(<span class="s">1</span>),
        present_mode,
        alpha_mode,
        view_formats: vec![],
        desired_maximum_frame_latency: <span class="s">2</span>, <span class="c">// at most 2 frames queued ahead of the screen: less input lag</span>
    };

    <span class="k">if</span> <span class="k">let</span> Some(s) = &amp;surface {
        s.configure(&amp;device, &amp;config);
    }

    Ok(DeviceSetup {
        surface,
        device,
        queue,
        config,
        device_type: info.device_type,
        failure,
    })
}

<span class="c">/// VIEWER_ADAPTER=nvidia picks the first native GPU whose name contains &quot;nvidia&quot;; the browser cannot choose.</span>
<span class="k">async</span> <span class="k">fn</span> named_adapter(
    instance: &amp;wgpu::Instance,
    backends: wgpu::Backends,
) -&gt; Option&lt;wgpu::Adapter&gt; {</code></pre></div>
<p>Copy this part from the lesson folder to the path shown.</p>
<p><code>lessons/12/src/engine/gpu/device.rs</code> · copy the file, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    #[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
    {
        <span class="k">let</span> _ = (instance, backends);
        None
    }
    #[cfg(not(target_arch = &quot;<span class="s">wasm32</span>&quot;))]
    {
        <span class="k">let</span> want = std::env::var(&quot;<span class="s">VIEWER_ADAPTER</span>&quot;).ok()?.to_lowercase();
        <span class="k">let</span> <span class="k">mut</span> selected = None;

        <span class="k">for</span> adapter <span class="k">in</span> instance.enumerate_adapters(backends).<span class="k">await</span> {
            <span class="k">if</span> adapter.get_info().name.to_lowercase().contains(&amp;want) {
                selected = Some(adapter);
                <span class="k">break</span>;
            }
        }

        selected
    }
}

<span class="c">/// Remember failure without unwinding through a browser callback.</span>
#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">fn</span> remember_failure(failure: &amp;std::sync::Mutex&lt;Option&lt;String&gt;&gt;, message: String) {
    <span class="k">if</span> <span class="k">let</span> Ok(<span class="k">mut</span> state) = failure.lock() {
        *state = Some(message);
    }
}

<span class="c">/// Native: a GPU error stops the program.</span>
#[cfg(not(target_arch = &quot;<span class="s">wasm32</span>&quot;))]
<span class="k">fn</span> report_gpu_error(e: wgpu::Error) {
    panic!(&quot;<span class="s">wgpu: </span>{<span class="s">e</span>}&quot;);
}

<span class="c">/// A broken shader must reach the error callback.</span>
#[cfg(all(test, not(target_arch = &quot;<span class="s">wasm32</span>&quot;)))]
#[test]
#[ignore = &quot;<span class="s">requires a native GPU adapter</span>&quot;]
#[should_panic(expected = &quot;<span class="s">wgpu: Validation Error</span>&quot;)]
<span class="k">fn</span> invalid_gpu_shader_is_fatal() {
    <span class="k">let</span> setup = pollster::block_on(open(None, (<span class="s">1</span>, <span class="s">1</span>))).expect(&quot;<span class="s">open native adapter</span>&quot;);
    <span class="k">let</span> _ = setup
        .device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(&quot;<span class="s">intentional verification failure</span>&quot;),
            source: wgpu::ShaderSource::Wgsl(
                &quot;<span class="s">@compute @workgroup_size(1) fn main() </span>{<span class="s"> let broken: u32 = true; </span>}&quot;.into(),
            ),
        });
}

#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">fn</span> remember_gpu_error(failure: &amp;std::sync::Mutex&lt;Option&lt;String&gt;&gt;, error: wgpu::Error) {
    remember_failure(failure, format!(&quot;<span class="s">WebGPU error: </span>{<span class="s">error</span>}&quot;));
}

<span class="c">/// Device lost = the browser took the GPU away, e.g. after a driver reset or when memory ran out.</span>
#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">fn</span> remember_device_loss(
    failure: &amp;std::sync::Mutex&lt;Option&lt;String&gt;&gt;,
    reason: wgpu::DeviceLostReason,
    message: &amp;str,
) {
    remember_failure(
        failure,
        format!(&quot;<span class="s">WebGPU device lost (</span>{<span class="s">reason:?</span>}<span class="s">): </span>{<span class="s">message</span>}&quot;),
    );
}</code></pre></div>
<h2 id="step-2-srcenginegpupresentrs">Step 2 · src/engine/gpu/present.rs<a class="anchor" href="#/course/12-picking#step-2-srcenginegpupresentrs" aria-label="Link to this section">#</a></h2>
<p>New file: draw a frame to the canvas, run a pick-only frame, or render off-screen for native tests.</p>
<p><code>lessons/12/src/engine/gpu/present.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! More methods of Gpu: one type may have several \`impl\` blocks, so each file adds its own.</span>
<span class="k">use</span> super::Gpu;
<span class="k">use</span> super::frame::{FrameCx, FrameInput};
#[cfg(not(target_arch = &quot;<span class="s">wasm32</span>&quot;))]
<span class="k">use</span> super::targets::{TextureSpec, texture};

<span class="k">impl</span> Gpu {
    <span class="c">/// Write this frame's uniforms and prepare the widget and text.</span>
    <span class="k">fn</span> write_frame_uniforms(&amp;<span class="k">mut</span> <span class="k">self</span>, input: &amp;FrameInput) {
        <span class="k">let</span> size = (<span class="k">self</span>.config.width, <span class="k">self</span>.config.height);
        <span class="k">let</span> cx = FrameCx {
            view: &amp;<span class="k">self</span>.view,
            anchor: <span class="k">self</span>.objects.anchor_f32(),
            size,
            <span class="c">// e.g. 1600 real pixels over 800 CSS pixels = 2.0</span>
            pixel_scale: size.<span class="s">0</span> <span class="k">as</span> f32 / <span class="k">self</span>.logical_size[<span class="s">0</span>].max(<span class="s">1</span>.<span class="s">0</span>) <span class="k">as</span> f32,
        };
        <span class="k">self</span>.frame.write(&amp;<span class="k">self</span>.ctx, input, &amp;cx);
        <span class="k">self</span>.objects
            .update_inside(&amp;<span class="k">self</span>.ctx, <span class="k">self</span>.frame.eye, &amp;<span class="k">self</span>.bounds);
        <span class="k">let</span> frame = super::text::TextFrame {
            mvp: <span class="k">self</span>.frame.mvp_f32,
            origin: <span class="k">self</span>.objects.anchor(),
            framebuffer: [size.<span class="s">0</span>, size.<span class="s">1</span>],
            logical: <span class="k">self</span>.logical_size,
            ortho_half_height: <span class="k">self</span>.frame.ortho_h,
        };

        <span class="k">if</span> <span class="k">let</span> Err(error) = <span class="k">self</span>.text.prepare(&amp;<span class="k">self</span>.ctx, &amp;frame) {
            log::warn!(&quot;<span class="s">text preparation: </span>{<span class="s">error</span>}&quot;);
        }
    }

    <span class="c">/// Draw one frame to the canvas; returns encode time in ms.</span>
    <span class="k">pub</span> <span class="k">fn</span> present(&amp;<span class="k">mut</span> <span class="k">self</span>, input: &amp;FrameInput) -&gt; Option&lt;f64&gt; {
        <span class="k">self</span>.write_frame_uniforms(input);
        <span class="k">let</span> surface = <span class="k">self</span>.surface.as_ref()?;
        <span class="c">// Suboptimal still draws; lost or outdated: reconfigure and skip this frame</span>
        <span class="k">let</span> output = <span class="k">match</span> surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(t)
            | wgpu::CurrentSurfaceTexture::Suboptimal(t) =&gt; t,
            _ =&gt; {
                surface.configure(&amp;<span class="k">self</span>.ctx.device, &amp;<span class="k">self</span>.config);
                <span class="k">return</span> None;
            }
        };
        <span class="k">let</span> view = output
            .texture
            .create_view(&amp;wgpu::TextureViewDescriptor::default());
        <span class="k">let</span> <span class="k">mut</span> encoder = <span class="k">self</span>
            .ctx
            .device
            .create_command_encoder(&amp;wgpu::CommandEncoderDescriptor {
                label: Some(&quot;<span class="s">frame</span>&quot;),
            });

        <span class="k">let</span> t0 = <span class="k">crate</span>::engine::performance::now_ms();
        <span class="k">let</span> (draws, objects) = <span class="k">self</span>.encode_frame(&amp;<span class="k">mut</span> encoder, &amp;view, input.clear);
        <span class="k">let</span> encode_ms = <span class="k">crate</span>::engine::performance::now_ms() - t0;
        <span class="k">self</span>.ctx.queue.submit([encoder.finish()]);
        <span class="k">self</span>.pick.map(); <span class="c">// after submit: start reading back a pending pick, if any</span>
        output.present();
        <span class="k">self</span>.performance
            .frame(draws, objects, input.now_ms, <span class="k">self</span>.view.perf);
        Some(encode_ms)
    }</code></pre></div>
<p><code>lessons/12/src/engine/gpu/present.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Run only the id pass for a pick; nothing is shown.</span>
    <span class="k">pub</span> <span class="k">fn</span> pick_frame(&amp;<span class="k">mut</span> <span class="k">self</span>, input: &amp;FrameInput, at: (u32, u32)) {
        <span class="k">self</span>.write_frame_uniforms(input);
        <span class="k">let</span> <span class="k">mut</span> encoder = <span class="k">self</span>
            .ctx
            .device
            .create_command_encoder(&amp;wgpu::CommandEncoderDescriptor {
                label: Some(&quot;<span class="s">pick</span>&quot;),
            });
        <span class="k">self</span>.point_pass(&amp;<span class="k">mut</span> encoder);
        <span class="k">self</span>.id_pass(&amp;<span class="k">mut</span> encoder, Some(at));
        <span class="k">self</span>.ctx.queue.submit([encoder.finish()]);
        <span class="k">self</span>.pick.map();
    }</code></pre></div>
<p>Copy this part from the lesson folder to the path shown.</p>
<p><code>lessons/12/src/engine/gpu/present.rs</code> · copy the file, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Draw one frame into a texture and return its RGBA8 pixels; native only.</span>
    #[cfg(not(target_arch = &quot;<span class="s">wasm32</span>&quot;))]
    <span class="k">pub</span> <span class="k">fn</span> render_offscreen(&amp;<span class="k">mut</span> <span class="k">self</span>, input: &amp;FrameInput) -&gt; Vec&lt;u8&gt; {
        <span class="k">let</span> (w, h) = (<span class="k">self</span>.config.width, <span class="k">self</span>.config.height);
        <span class="k">let</span> usage = wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC;
        <span class="k">let</span> tex = texture(
            &amp;<span class="k">self</span>.ctx,
            &quot;<span class="s">headless.color</span>&quot;,
            &amp;TextureSpec {
                size: (w, h),
                format: <span class="k">self</span>.config.format,
                samples: <span class="s">1</span>,
                usage,
            },
        );
        <span class="k">let</span> view = tex.create_view(&amp;wgpu::TextureViewDescriptor::default());
        <span class="c">// wgpu copies rows 256-byte aligned: a 100 px row of 400 bytes is padded to 512</span>
        <span class="k">let</span> padded = (w * <span class="s">4</span>).div_ceil(<span class="s">256</span>) * <span class="s">256</span>;
        <span class="k">let</span> readback = <span class="k">self</span>.ctx.device.create_buffer(&amp;wgpu::BufferDescriptor {
            label: Some(&quot;<span class="s">headless.readback</span>&quot;),
            size: (padded * h) <span class="k">as</span> u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: <span class="s">false</span>,
        });

        <span class="k">self</span>.write_frame_uniforms(input);
        <span class="k">let</span> <span class="k">mut</span> encoder = <span class="k">self</span>.ctx.device.create_command_encoder(&amp;Default::default());
        <span class="k">let</span> (draws, objects) = <span class="k">self</span>.encode_frame(&amp;<span class="k">mut</span> encoder, &amp;view, input.clear);
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture: &amp;tex,
                mip_level: <span class="s">0</span>,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &amp;readback,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: <span class="s">0</span>,
                    bytes_per_row: Some(padded),
                    rows_per_image: Some(h),
                },
            },
            wgpu::Extent3d {
                width: w,
                height: h,
                depth_or_array_layers: <span class="s">1</span>,
            },
        );
        <span class="k">self</span>.ctx.queue.submit([encoder.finish()]);
        <span class="k">self</span>.pick.map();
        log::info!(&quot;<span class="s">headless frame: </span>{<span class="s">draws</span>}<span class="s"> draws, </span>{<span class="s">objects</span>}<span class="s"> objects, </span>{<span class="s">w</span>}<span class="s">x</span>{<span class="s">h</span>}&quot;);

        <span class="k">let</span> slice = readback.slice(..);
        <span class="c">// map_async only asks; poll(Wait) blocks until the copy has reached the CPU</span>
        slice.map_async(wgpu::MapMode::Read, |_| {});
        <span class="k">let</span> _ = <span class="k">self</span>.ctx.device.poll(wgpu::PollType::Wait {
            submission_index: None,
            timeout: None,
        });
        <span class="k">let</span> data = slice.get_mapped_range();
        <span class="k">let</span> <span class="k">mut</span> out = Vec::with_capacity((w * <span class="s">4</span> * h) <span class="k">as</span> usize);

        <span class="c">// drop the row padding</span>
        <span class="k">for</span> row <span class="k">in</span> <span class="s">0</span>..h {
            <span class="k">let</span> a = (row * padded) <span class="k">as</span> usize;
            out.extend_from_slice(&amp;data[a..a + (w * <span class="s">4</span>) <span class="k">as</span> usize]);
        }

        drop(data); <span class="c">// the mapped view must be gone before unmap</span>
        readback.unmap();
        out
    }

    <span class="c">/// Draw one frame and return (object, sub) per pixel; native only.</span>
    #[cfg(not(target_arch = &quot;<span class="s">wasm32</span>&quot;))]
    <span class="k">pub</span> <span class="k">fn</span> render_ids_offscreen(&amp;<span class="k">mut</span> <span class="k">self</span>, input: &amp;FrameInput) -&gt; Vec&lt;[u32; 2]&gt; {
        <span class="k">let</span> size = (<span class="k">self</span>.config.width, <span class="k">self</span>.config.height);
        <span class="k">let</span> texture = texture(
            &amp;<span class="k">self</span>.ctx,
            &quot;<span class="s">headless.ids.color</span>&quot;,
            &amp;TextureSpec {
                size,
                format: <span class="k">self</span>.config.format,
                samples: <span class="s">1</span>,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            },
        );
        <span class="k">let</span> view = texture.create_view(&amp;wgpu::TextureViewDescriptor::default());
        <span class="k">self</span>.write_frame_uniforms(input);
        <span class="k">let</span> <span class="k">mut</span> encoder = <span class="k">self</span>
            .ctx
            .device
            .create_command_encoder(&amp;wgpu::CommandEncoderDescriptor {
                label: Some(&quot;<span class="s">headless.ids</span>&quot;),
            });
        <span class="k">self</span>.encode_frame(&amp;<span class="k">mut</span> encoder, &amp;view, input.clear);
        <span class="k">self</span>.id_pass(&amp;<span class="k">mut</span> encoder, None);
        <span class="k">let</span> readback = <span class="k">self</span>.pick.copy_frame(&amp;<span class="k">self</span>.ctx, &amp;<span class="k">mut</span> encoder);
        <span class="k">self</span>.ctx.queue.submit([encoder.finish()]);
        readback.read(&amp;<span class="k">self</span>.ctx)
    }
}</code></pre></div>
<h2 id="step-3-srcenginegpurenderrs">Step 3 · src/engine/gpu/render.rs<a class="anchor" href="#/course/12-picking#step-3-srcenginegpurenderrs" aria-label="Link to this section">#</a></h2>
<p>The frame encoder orders face, ink, picking and overlay passes.</p>
<p><code>lessons/12/src/engine/gpu/render.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! Records one frame: every pass in order into one encoder, then a single submit.</span>
<span class="k">use</span> super::Gpu;
<span class="k">use</span> super::frame::Binds;
<span class="k">use</span> super::pick::PickMode;
<span class="k">use</span> super::splat::RecordCx;

<span class="k">impl</span> Gpu {
    <span class="c">/// Encode one frame into \`view\`; returns (draws, objects).</span>
    <span class="k">pub</span> <span class="k">fn</span> encode_frame(
        &amp;<span class="k">mut</span> <span class="k">self</span>,
        encoder: &amp;<span class="k">mut</span> wgpu::CommandEncoder,
        view: &amp;wgpu::TextureView,
        clear: wgpu::Color,
    ) -&gt; (u32, u32) {
        <span class="k">self</span>.point_pass(encoder);

        <span class="c">// pass 1: background, faces and clouds write depth</span>
        <span class="k">let</span> <span class="k">mut</span> draws = {
            <span class="k">let</span> b = Binds {
                mvp: &amp;<span class="k">self</span>.frame.mvp_group, <span class="c">// the camera matrix</span>
                line: &amp;<span class="k">self</span>.frame.line_group, <span class="c">// pen settings</span>
                instances: &amp;<span class="k">self</span>.objects.group,
            };
            <span class="k">let</span> <span class="k">mut</span> pass = <span class="k">self</span>.targets.begin_faces(encoder, view, clear);
            <span class="k">self</span>.face_list(&amp;<span class="k">mut</span> pass, &amp;b)
        };

        <span class="k">if</span> <span class="k">self</span>.selection_outline.prepare(
            &amp;<span class="k">self</span>.ctx,
            (<span class="k">self</span>.config.width, <span class="k">self</span>.config.height),
            <span class="k">self</span>.targets.samples,
            <span class="k">self</span>.logical_size[<span class="s">0</span>],
            <span class="k">self</span>.arena.face_count() &gt; <span class="s">0</span>,
        ) {
            <span class="k">let</span> b = Binds {
                mvp: &amp;<span class="k">self</span>.frame.mvp_group, <span class="c">// the camera matrix</span>
                line: &amp;<span class="k">self</span>.frame.line_group, <span class="c">// pen settings</span>
                instances: &amp;<span class="k">self</span>.objects.group,
            };
            <span class="k">let</span> <span class="k">mut</span> pass = <span class="k">self</span>.selection_outline.begin_mask(encoder, &amp;<span class="k">self</span>.targets);
            draws += <span class="k">self</span>.arena.draw_selection_mask(&amp;<span class="k">mut</span> pass, &amp;b);
        }

        {
            <span class="c">// pass 3: lines, markers, outlines and text over the faces</span>
            <span class="k">let</span> <span class="k">mut</span> pass = <span class="k">self</span>.targets.begin_ink(encoder, view);
            draws += <span class="k">self</span>.scene_list(&amp;<span class="k">mut</span> pass);
        }

        <span class="c">// a click waiting: draw the id pass now</span>
        <span class="k">if</span> <span class="k">let</span> Some(at) = <span class="k">self</span>.pick.take_pending() {
            <span class="k">self</span>.id_pass(encoder, Some(at));
        }

        (draws, <span class="k">self</span>.objects.len())
    }</code></pre></div>
<p><code>lessons/12/src/engine/gpu/render.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Draw the point clouds; skipped while nothing changed.</span>
    <span class="k">pub</span>(super) <span class="k">fn</span> point_pass(&amp;<span class="k">mut</span> <span class="k">self</span>, encoder: &amp;<span class="k">mut</span> wgpu::CommandEncoder) {
        <span class="k">let</span> cx = RecordCx {
            mvp: &amp;<span class="k">self</span>.frame.mvp_f32,
            ortho_h: <span class="k">self</span>.frame.ortho_h,
            eye: <span class="k">self</span>.frame.eye,
            size: (<span class="k">self</span>.config.width, <span class="k">self</span>.config.height),
            <span class="c">// point size in framebuffer pixels</span>
            cloud_size: <span class="k">self</span>.view.cloud_size * <span class="k">self</span>.config.width <span class="k">as</span> f32
                / <span class="k">self</span>.logical_size[<span class="s">0</span>].max(<span class="s">1</span>.<span class="s">0</span>) <span class="k">as</span> f32,
            lod_px: <span class="k">self</span>.view.lod_px,
            objects: &amp;<span class="k">self</span>.objects,
            clouds: &amp;<span class="k">self</span>.cloud.clouds,
            nodes: &amp;<span class="k">self</span>.cloud.nodes,
        };
        <span class="k">self</span>.splat.prelude(
            &amp;<span class="k">self</span>.ctx,
            &amp;<span class="k">self</span>.layouts,
            encoder,
            &amp;cx,
            &amp;<span class="k">self</span>.frame.cloud_group,
        );
    }

    <span class="c">/// Draws of the first pass: background, grid, faces, clouds.</span>
    <span class="k">fn</span> face_list(&amp;<span class="k">self</span>, pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;, b: &amp;Binds) -&gt; u32 {
        <span class="k">let</span> <span class="k">mut</span> draws = <span class="k">self</span>.backdrop.draw_background(pass);

        <span class="k">if</span> <span class="k">self</span>.view.show_grid {
            draws += <span class="k">self</span>.backdrop.draw_grid(pass, b);
        }

        draws += <span class="k">self</span>.arena.draw_faces(pass, b);
        draws += <span class="k">self</span>.splat.draw_resolve(pass, &amp;<span class="k">self</span>.frame.cloud_group);
        draws
    }

    <span class="c">/// Draws of the ink pass, back to front.</span>
    <span class="k">fn</span> scene_list(&amp;<span class="k">self</span>, pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;) -&gt; u32 {
        <span class="k">let</span> v = &amp;<span class="k">self</span>.view;
        <span class="k">let</span> basic = Binds {
            mvp: &amp;<span class="k">self</span>.frame.mvp_group, <span class="c">// the camera matrix</span>
            line: &amp;<span class="k">self</span>.frame.line_group, <span class="c">// pen settings</span>
            instances: &amp;<span class="k">self</span>.objects.group,
        };
        <span class="k">let</span> b = Binds {
            mvp: &amp;<span class="k">self</span>.frame.mvp_group, <span class="c">// the camera matrix</span>
            line: &amp;<span class="k">self</span>.frame.line_group, <span class="c">// pen settings</span>
            instances: &amp;<span class="k">self</span>.objects.ink_group, <span class="c">// per-object rows plus depth</span>
        };
        <span class="k">let</span> <span class="k">mut</span> draws = <span class="k">self</span>.arena.draw_print(pass, &amp;basic);

        <span class="k">if</span> v.show_mesh_edges {
            draws += <span class="k">self</span>.segments.draw_pipes(pass, &amp;b);
        }

        <span class="k">if</span> v.show_lines {
            draws += <span class="k">self</span>.segments.draw_ribbons(pass, &amp;b);
        }

        <span class="k">if</span> v.show_mesh_edges &amp;&amp; v.markers {
            draws += <span class="k">self</span>.glyphs.draw_spheres(pass, &amp;b);
        }

        draws += <span class="k">self</span>.arena.draw_text(pass, &amp;basic);

        <span class="k">if</span> v.show_points {
            draws += <span class="k">self</span>.glyphs.draw_dots(pass, &amp;b);
        }

        draws += <span class="k">self</span>.selection_outline.draw(pass);
        draws += <span class="k">self</span>.text.draw(pass);
        draws
    }</code></pre></div>
<h2 id="step-4-srcappinputrs">Step 4 · src/app/input.rs<a class="anchor" href="#/course/12-picking#step-4-srcappinputrs" aria-label="Link to this section">#</a></h2>
<p>Input routes gestures and keyboard actions to State.</p>
<p><code>lessons/12/src/app/input.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! Collects browser pointer, key and touch events into one state the viewer can ask questions of, instead of handling events everywhere.</span>
<span class="k">use</span> super::touch::{Act, Touches};
<span class="k">use</span> <span class="k">crate</span>::State;
<span class="k">use</span> <span class="k">crate</span>::camera::View;
<span class="k">use</span> winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
<span class="k">use</span> winit::keyboard::{Key, NamedKey};

<span class="c">/// A press moving less than this many pixels is a click.</span>
<span class="k">const</span> CLICK_SLOP: f64 = <span class="s">4</span>.<span class="s">0</span>;

<span class="c">/// Mouse, keyboard and finger state between events.</span>
<span class="k">pub</span> <span class="k">struct</span> Input {
    orbiting: bool, <span class="c">// right button held</span>
    panning: bool, <span class="c">// middle button held</span>
    ctrl: bool,
    last_cursor: (f64, f64), <span class="c">// last pointer position in pixels</span>
    left_down: Option&lt;(f64, f64)&gt;,
    touch: Touches, <span class="c">// camera finger gestures</span>
}

<span class="k">impl</span> Default <span class="k">for</span> Input {
    <span class="c">/// Same as \`new\`.</span>
    <span class="k">fn</span> default() -&gt; <span class="k">Self</span> {
        <span class="k">Self</span>::new()
    }
}

<span class="k">impl</span> Input {
    <span class="c">/// Nothing held, cursor at the origin.</span>
    <span class="k">pub</span> <span class="k">fn</span> new() -&gt; <span class="k">Self</span> {
        <span class="k">Self</span> {
            orbiting: <span class="s">false</span>,
            panning: <span class="s">false</span>,
            ctrl: <span class="s">false</span>,
            last_cursor: (<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>),
            left_down: None,
            touch: Touches::new(),
        }
    }</code></pre></div>
<p><code>lessons/12/src/app/input.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// One key press; true when the frame must be redrawn.</span>
    <span class="k">pub</span> <span class="k">fn</span> key(&amp;<span class="k">mut</span> <span class="k">self</span>, state: &amp;<span class="k">mut</span> State, key: Key&lt;&amp;str&gt;) -&gt; bool {
        <span class="k">match</span> key {
            Key::Named(NamedKey::Space) =&gt; state
                .camera
                .toggle_projection_framed(&amp;state.gpu.bounds, state.aspect()),
            Key::Named(NamedKey::Escape) =&gt; state.escape_selection(),
            Key::Character(&quot;<span class="s">1</span>&quot;) =&gt; state.camera.set_view(View::Front),
            Key::Character(&quot;<span class="s">2</span>&quot;) =&gt; state.camera.set_view(View::Back),
            Key::Character(&quot;<span class="s">3</span>&quot;) =&gt; state.camera.set_view(View::Left),
            Key::Character(&quot;<span class="s">4</span>&quot;) =&gt; state.camera.set_view(View::Right),
            Key::Character(&quot;<span class="s">5</span>&quot;) =&gt; state.camera.set_view(View::Top),
            Key::Character(&quot;<span class="s">6</span>&quot;) =&gt; state.camera.set_view(View::Bottom),
            Key::Character(&quot;<span class="s">7</span>&quot;) =&gt; state.camera.set_view(View::Iso),
            Key::Character(&quot;<span class="s">c</span>&quot; | &quot;<span class="s">C</span>&quot;) =&gt; state.camera.reset(),
            Key::Character(&quot;<span class="s">f</span>&quot; | &quot;<span class="s">F</span>&quot;) =&gt; state.fit_selected_or_all(),
            Key::Character(&quot;<span class="s">q</span>&quot; | &quot;<span class="s">Q</span>&quot;) =&gt; state.gpu.view.show_points = !state.gpu.view.show_points,
            Key::Character(&quot;<span class="s">w</span>&quot; | &quot;<span class="s">W</span>&quot;) =&gt; state.gpu.view.show_lines = !state.gpu.view.show_lines,
            Key::Character(&quot;<span class="s">e</span>&quot; | &quot;<span class="s">E</span>&quot;) =&gt; {
                state.gpu.view.show_mesh_edges = !state.gpu.view.show_mesh_edges
            }
            Key::Character(&quot;<span class="s">d</span>&quot; | &quot;<span class="s">D</span>&quot;) =&gt; state.gpu.view.lit = !state.gpu.view.lit,
            Key::Character(&quot;<span class="s">h</span>&quot; | &quot;<span class="s">H</span>&quot;) =&gt; state.hide_selected(),
            Key::Character(&quot;<span class="s">s</span>&quot; | &quot;<span class="s">S</span>&quot;) =&gt; state.show_all(),
            Key::Character(&quot;<span class="s">t</span>&quot; | &quot;<span class="s">T</span>&quot;) =&gt; state.toggle_selected_names(),
            Key::Character(&quot;<span class="s">b</span>&quot; | &quot;<span class="s">B</span>&quot;) =&gt; state.gpu.view.backface = !state.gpu.view.backface,
            Key::Character(&quot;<span class="s">p</span>&quot; | &quot;<span class="s">P</span>&quot;) =&gt; state.toggle_xray(),
            Key::Character(&quot;<span class="s">[</span>&quot;) =&gt; state.set_cloud_size(state.gpu.view.cloud_size - <span class="s">0</span>.<span class="s">25</span>),
            Key::Character(&quot;<span class="s">]</span>&quot;) =&gt; state.set_cloud_size(state.gpu.view.cloud_size + <span class="s">0</span>.<span class="s">25</span>),
            _ =&gt; <span class="k">return</span> <span class="s">false</span>,
        }

        <span class="s">true</span>
    }</code></pre></div>
<p><code>lessons/12/src/app/input.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// One mouse or touch event; true when the frame must be redrawn.</span>
    <span class="k">pub</span> <span class="k">fn</span> mouse(&amp;<span class="k">mut</span> <span class="k">self</span>, state: &amp;<span class="k">mut</span> State, event: &amp;WindowEvent) -&gt; bool {
        <span class="k">let</span> viewport = state.viewport();

        <span class="k">match</span> event {
            WindowEvent::MouseInput {
                state: btn,
                button: MouseButton::Right,
                ..
            } =&gt; {
                <span class="k">self</span>.orbiting = *btn == ElementState::Pressed;
                <span class="s">false</span>
            }
            WindowEvent::MouseInput {
                state: btn,
                button: MouseButton::Middle,
                ..
            } =&gt; {
                <span class="k">self</span>.panning = *btn == ElementState::Pressed;
                <span class="s">false</span>
            }
            WindowEvent::MouseInput {
                state: btn,
                button: MouseButton::Left,
                ..
            } =&gt; <span class="k">self</span>.left(state, *btn),
            WindowEvent::CursorMoved { position, .. } =&gt; {
                <span class="k">let</span> dragging = <span class="k">self</span>.orbiting || <span class="k">self</span>.panning;

                <span class="c">// camera moves in CSS pixels</span>
                <span class="k">if</span> dragging {
                    <span class="k">let</span> dx = ((position.x - <span class="k">self</span>.last_cursor.<span class="s">0</span>) / device_pixel_ratio()) <span class="k">as</span> f32;
                    <span class="k">let</span> dy = ((position.y - <span class="k">self</span>.last_cursor.<span class="s">1</span>) / device_pixel_ratio()) <span class="k">as</span> f32;

                    <span class="k">if</span> <span class="k">self</span>.panning || <span class="k">self</span>.ctrl {
                        state.camera.pan(dx, dy);
                    } <span class="k">else</span> {
                        state.camera.orbit(dx, dy);
                    }
                }

                <span class="k">self</span>.last_cursor = (position.x, position.y);
                dragging
            }
            WindowEvent::MouseWheel { delta, .. } =&gt; {
                <span class="k">let</span> amount = <span class="k">match</span> delta {
                    MouseScrollDelta::LineDelta(_, y) =&gt; *y,
                    MouseScrollDelta::PixelDelta(p) =&gt; p.y <span class="k">as</span> f32 / <span class="s">100</span>.<span class="s">0</span>,
                };
                state.camera.zoom_at(amount, <span class="k">self</span>.last_cursor, viewport);
                <span class="s">true</span>
            }
            WindowEvent::ModifiersChanged(mods) =&gt; {
                <span class="k">self</span>.ctrl = mods.state().control_key();
                <span class="s">false</span>
            }
            WindowEvent::Focused(<span class="s">false</span>) =&gt; {
                <span class="k">self</span>.cancel();
                <span class="s">true</span>
            }
            WindowEvent::Touch(t) =&gt; {
                <span class="c">// otherwise the fingers move the camera</span>
                <span class="k">match</span> <span class="k">self</span>
                    .touch
                    .event(&amp;<span class="k">mut</span> state.camera, t, viewport, device_pixel_ratio())
                {
                    Act::None =&gt; <span class="s">false</span>,
                    Act::Moved =&gt; <span class="s">true</span>,
                    Act::Tap(at) =&gt; {
                        state.request_selection(at.<span class="s">0</span> <span class="k">as</span> u32, at.<span class="s">1</span> <span class="k">as</span> u32, <span class="s">false</span>);
                        <span class="s">false</span>
                    }
                    Act::Fit =&gt; {
                        state.fit_all();
                        <span class="s">true</span>
                    }
                }
            }
            _ =&gt; <span class="s">false</span>,
        }
    }

    <span class="c">/// Forget every gesture in progress.</span>
    <span class="k">pub</span> <span class="k">fn</span> cancel(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">self</span>.orbiting = <span class="s">false</span>;
        <span class="k">self</span>.panning = <span class="s">false</span>;
        <span class="k">self</span>.ctrl = <span class="s">false</span>;
        <span class="k">self</span>.left_down = None;
        <span class="k">self</span>.touch = Touches::new();
    }</code></pre></div>
<p><code>lessons/12/src/app/input.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Left button: control drag, then gizmo drag, then a click.</span>
    <span class="k">fn</span> left(&amp;<span class="k">mut</span> <span class="k">self</span>, state: &amp;<span class="k">mut</span> State, btn: ElementState) -&gt; bool {
        <span class="k">match</span> btn {
            ElementState::Pressed =&gt; {
                <span class="k">self</span>.left_down = Some(<span class="k">self</span>.last_cursor);
                <span class="s">false</span>
            }
            ElementState::Released =&gt; {
                <span class="k">let</span> Some(down) = <span class="k">self</span>.left_down.take() <span class="k">else</span> {
                    <span class="k">return</span> <span class="s">false</span>;
                };
                <span class="k">let</span> moved = (<span class="k">self</span>.last_cursor.<span class="s">0</span> - down.<span class="s">0</span>)
                    .abs()
                    .max((<span class="k">self</span>.last_cursor.<span class="s">1</span> - down.<span class="s">1</span>).abs());

                <span class="k">if</span> moved &gt; CLICK_SLOP * device_pixel_ratio() {
                    <span class="k">return</span> <span class="s">false</span>; <span class="c">// a drag, not a click</span>
                }

                state.request_selection(
                    <span class="k">self</span>.last_cursor.<span class="s">0</span> <span class="k">as</span> u32,
                    <span class="k">self</span>.last_cursor.<span class="s">1</span> <span class="k">as</span> u32,
                    <span class="k">self</span>.ctrl,
                );
                <span class="s">false</span>
            }
        }
    }
}</code></pre></div>
<p>Copy this part from the lesson folder to the path shown.</p>
<p><code>lessons/12/src/app/input.rs</code> · copy the file, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// A \`pointercancel\` listener on the canvas.</span>
#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">pub</span> <span class="k">struct</span> PointerCancellation {
    canvas: web_sys::HtmlCanvasElement,
    callback: wasm_bindgen::closure::Closure&lt;<span class="k">dyn</span> FnMut(web_sys::Event)&gt;,
}

#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">impl</span> PointerCancellation {
    <span class="c">/// Install the listener; it sends one message per event.</span>
    <span class="k">pub</span> <span class="k">fn</span> new(
        canvas: web_sys::HtmlCanvasElement,
        proxy: winit::event_loop::EventLoopProxy&lt;<span class="k">crate</span>::Msg&gt;,
    ) -&gt; Result&lt;<span class="k">Self</span>, wasm_bindgen::JsValue&gt; {
        <span class="k">use</span> wasm_bindgen::JsCast;
        <span class="k">let</span> callback =
            wasm_bindgen::closure::Closure::&lt;<span class="k">dyn</span> FnMut(web_sys::Event)&gt;::new(<span class="k">move</span> |_| {
                cancel_pointer(&amp;proxy)
            });
        canvas
            .add_event_listener_with_callback(&quot;<span class="s">pointercancel</span>&quot;, callback.as_ref().unchecked_ref())?;
        Ok(<span class="k">Self</span> { canvas, callback })
    }
}

#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">impl</span> Drop <span class="k">for</span> PointerCancellation {
    <span class="c">/// Remove the listener.</span>
    <span class="k">fn</span> drop(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">use</span> wasm_bindgen::JsCast;
        <span class="k">let</span> _ = <span class="k">self</span>.canvas.remove_event_listener_with_callback(
            &quot;<span class="s">pointercancel</span>&quot;,
            <span class="k">self</span>.callback.as_ref().unchecked_ref(),
        );
    }
}

<span class="c">/// Send the cancel message to the event loop.</span>
#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">fn</span> cancel_pointer(proxy: &amp;winit::event_loop::EventLoopProxy&lt;<span class="k">crate</span>::Msg&gt;) {
    <span class="k">let</span> _ = proxy.send_event(<span class="k">crate</span>::Msg::CancelPointer);
}

<span class="c">/// Physical pixels per CSS pixel: 1 on a desktop monitor, 2-4 on a phone.</span>
#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">fn</span> device_pixel_ratio() -&gt; f64 {
    <span class="k">if</span> <span class="k">let</span> Some(window) = web_sys::window() {
        <span class="k">let</span> ratio = window.device_pixel_ratio();

        <span class="k">if</span> ratio &gt; <span class="s">0</span>.<span class="s">0</span> {
            <span class="k">return</span> ratio;
        }
    }

    <span class="s">1</span>.<span class="s">0</span>
}

<span class="c">/// Native windows report logical pixels already.</span>
#[cfg(not(target_arch = &quot;<span class="s">wasm32</span>&quot;))]
<span class="k">fn</span> device_pixel_ratio() -&gt; f64 {
    <span class="s">1</span>.<span class="s">0</span>
}</code></pre></div>
<h2 id="step-5-srcapptouchrs">Step 5 · src/app/touch.rs<a class="anchor" href="#/course/12-picking#step-5-srcapptouchrs" aria-label="Link to this section">#</a></h2>
<p>New file: one finger orbits, two fingers pan and pinch, a tap picks and a double tap fits.</p>
<p><code>lessons/12/src/app/touch.rs</code> · copy the file, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! Turns raw touch points into one camera gesture: one finger orbits, two fingers pan and zoom.</span>
<span class="k">use</span> winit::event::{Touch, TouchPhase};

<span class="k">use</span> <span class="k">crate</span>::camera::Camera;
<span class="k">use</span> <span class="k">crate</span>::engine::performance::now_ms;

<span class="c">/// 2·tan(30°) / 0.0015: with a 60° view the point under the finger stays under it; \`pan\` multiplies by 0.0015.</span>
<span class="k">const</span> PAN_PER_PX: f64 = <span class="s">2</span>.<span class="s">0</span> * <span class="s">0</span>.<span class="s">577_350_269_189_625_7</span> / <span class="s">0</span>.<span class="s">001_5</span>; <span class="c">// 769.8</span>

<span class="c">/// ln 0.9: zoom counts wheel steps of x0.9, so a pinch ratio r is -ln r / ln 0.9 steps, i.e. distance / r.</span>
<span class="k">const</span> PINCH_LOG: f64 = -<span class="s">0</span>.<span class="s">105_360_515_657_826_28</span>;

<span class="c">/// Largest pinch ratio one event may apply.</span>
<span class="k">const</span> PINCH_MAX: f64 = <span class="s">2</span>.<span class="s">0</span>;

<span class="c">/// A finger moving less than this many pixels is a tap.</span>
<span class="k">const</span> TAP_SLOP: f64 = <span class="s">12</span>.<span class="s">0</span>;

<span class="c">/// A finger lifting within this many ms is a tap.</span>
<span class="k">const</span> TAP_MS: f64 = <span class="s">300</span>.<span class="s">0</span>;

<span class="c">/// A second tap within this many ms is a double tap.</span>
<span class="k">const</span> DOUBLE_TAP_MS: f64 = <span class="s">320</span>.<span class="s">0</span>;

<span class="c">/// A second tap within this many pixels is a double tap.</span>
<span class="k">const</span> DOUBLE_TAP_SLOP: f64 = <span class="s">40</span>.<span class="s">0</span>;</code></pre></div>
<p><code>lessons/12/src/app/touch.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// What one touch event asks the caller to do.</span>
<span class="k">pub</span> <span class="k">enum</span> Act {
    None,
    Moved, <span class="c">// the camera moved, redraw</span>
    Fit, <span class="c">// double tap: fit the scene</span>
    Tap((f64, f64)), <span class="c">// single tap: pick at these pixels</span>
}

<span class="c">/// One finger on the screen, in physical pixels.</span>
<span class="k">struct</span> Finger {
    id: u64, <span class="c">// the same while this finger stays down</span>
    pos: (f64, f64),
    down: (f64, f64), <span class="c">// where it landed</span>
    t0: f64, <span class="c">// when it landed, ms</span>
}

<span class="c">/// Every finger down and the last two-finger measurement.</span>
<span class="k">pub</span> <span class="k">struct</span> Touches {
    fingers: Vec&lt;Finger&gt;,
    span: f64, <span class="c">// last distance between the first two, 0 = not yet measured</span>
    mid: (f64, f64), <span class="c">// last midpoint of the first two</span>
    tap: Option&lt;(f64, (f64, f64))&gt;, <span class="c">// when and where the last tap lifted</span>
}

<span class="k">impl</span> Touches {
    <span class="c">/// No fingers down, no tap pending.</span>
    <span class="k">pub</span> <span class="k">fn</span> new() -&gt; <span class="k">Self</span> {
        <span class="k">Self</span> {
            fingers: Vec::new(),
            span: <span class="s">0</span>.<span class="s">0</span>,
            mid: (<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>),
            tap: None,
        }
    }

    <span class="c">/// Apply one touch event to the camera; \`vp\` is the viewport size in pixels.</span>
    <span class="k">pub</span> <span class="k">fn</span> event(&amp;<span class="k">mut</span> <span class="k">self</span>, cam: &amp;<span class="k">mut</span> Camera, t: &amp;Touch, vp: (f64, f64), dpr: f64) -&gt; Act {
        <span class="k">let</span> p = (t.location.x, t.location.y);

        <span class="k">match</span> t.phase {
            TouchPhase::Started =&gt; {
                <span class="k">self</span>.fingers.push(Finger {
                    id: t.id,
                    pos: p,
                    down: p,
                    t0: now_ms(),
                });
                <span class="k">self</span>.span = <span class="s">0</span>.<span class="s">0</span>; <span class="c">// finger count changed, measure again</span>
                Act::None
            }
            TouchPhase::Moved =&gt; <span class="k">self</span>.moved(cam, t.id, p, vp, dpr),
            TouchPhase::Ended =&gt; <span class="k">self</span>.lifted(t.id, p, dpr),
            <span class="c">// the browser took the gesture away</span>
            TouchPhase::Cancelled =&gt; {
                <span class="k">self</span>.drop_finger(t.id);
                <span class="k">self</span>.tap = None;
                Act::None
            }
        }
    }</code></pre></div>
<p>Copy this part from the lesson folder to the path shown.</p>
<p><code>lessons/12/src/app/touch.rs</code> · copy the file, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// One finger orbits; two pan by their midpoint and zoom by their distance.</span>
    <span class="k">fn</span> moved(&amp;<span class="k">mut</span> <span class="k">self</span>, cam: &amp;<span class="k">mut</span> Camera, id: u64, p: (f64, f64), vp: (f64, f64), dpr: f64) -&gt; Act {
        <span class="k">let</span> Some(i) = <span class="k">self</span>.finger_index(id) <span class="k">else</span> {
            <span class="k">return</span> Act::None;
        };
        <span class="k">let</span> d = (p.<span class="s">0</span> - <span class="k">self</span>.fingers[i].pos.<span class="s">0</span>, p.<span class="s">1</span> - <span class="k">self</span>.fingers[i].pos.<span class="s">1</span>);
        <span class="k">self</span>.fingers[i].pos = p;

        <span class="k">if</span> <span class="k">self</span>.fingers.len() == <span class="s">1</span> {
            cam.orbit((d.<span class="s">0</span> / dpr) <span class="k">as</span> f32, (d.<span class="s">1</span> / dpr) <span class="k">as</span> f32);
            <span class="k">return</span> Act::Moved;
        }

        <span class="c">// always the first two fingers, even with more down</span>
        <span class="k">let</span> (a, b) = (<span class="k">self</span>.fingers[<span class="s">0</span>].pos, <span class="k">self</span>.fingers[<span class="s">1</span>].pos);
        <span class="k">let</span> span = (b.<span class="s">0</span> - a.<span class="s">0</span>).hypot(b.<span class="s">1</span> - a.<span class="s">1</span>).max(<span class="s">1</span>.<span class="s">0</span>);
        <span class="k">let</span> mid = ((a.<span class="s">0</span> + b.<span class="s">0</span>) * <span class="s">0</span>.<span class="s">5</span>, (a.<span class="s">1</span> + b.<span class="s">1</span>) * <span class="s">0</span>.<span class="s">5</span>);

        <span class="k">if</span> <span class="k">self</span>.span == <span class="s">0</span>.<span class="s">0</span> {
            <span class="k">self</span>.span = span; <span class="c">// first measurement: record only</span>
            <span class="k">self</span>.mid = mid;
            <span class="k">return</span> Act::None;
        }

        <span class="c">// pan by the midpoint's move, then zoom about it</span>
        <span class="k">let</span> h = vp.<span class="s">1</span>.max(<span class="s">1</span>.<span class="s">0</span>);
        cam.pan(
            ((mid.<span class="s">0</span> - <span class="k">self</span>.mid.<span class="s">0</span>) * PAN_PER_PX / h) <span class="k">as</span> f32,
            ((mid.<span class="s">1</span> - <span class="k">self</span>.mid.<span class="s">1</span>) * PAN_PER_PX / h) <span class="k">as</span> f32,
        );
        <span class="k">let</span> r = (span / <span class="k">self</span>.span).clamp(<span class="s">1</span>.<span class="s">0</span> / PINCH_MAX, PINCH_MAX);
        cam.zoom_at((-r.ln() / PINCH_LOG) <span class="k">as</span> f32, mid, vp);

        <span class="k">self</span>.span = span;
        <span class="k">self</span>.mid = mid;
        Act::Moved
    }

    <span class="c">/// A finger lifted; the last one up may be a tap.</span>
    <span class="k">fn</span> lifted(&amp;<span class="k">mut</span> <span class="k">self</span>, id: u64, p: (f64, f64), dpr: f64) -&gt; Act {
        <span class="k">let</span> Some(f) = <span class="k">self</span>.drop_finger(id) <span class="k">else</span> {
            <span class="k">return</span> Act::None;
        };

        <span class="k">if</span> !<span class="k">self</span>.fingers.is_empty() {
            <span class="k">self</span>.tap = None;
            <span class="k">return</span> Act::None;
        }

        <span class="k">let</span> now = now_ms();

        <span class="k">if</span> (p.<span class="s">0</span> - f.down.<span class="s">0</span>).hypot(p.<span class="s">1</span> - f.down.<span class="s">1</span>) / dpr &gt; TAP_SLOP || now - f.t0 &gt; TAP_MS {
            <span class="k">self</span>.tap = None; <span class="c">// a drag or a long press</span>
            <span class="k">return</span> Act::None;
        }

        <span class="k">let</span> second = <span class="k">match</span> <span class="k">self</span>.tap.take() {
            Some((t0, at)) =&gt; {
                now - t0 &lt; DOUBLE_TAP_MS &amp;&amp; (p.<span class="s">0</span> - at.<span class="s">0</span>).hypot(p.<span class="s">1</span> - at.<span class="s">1</span>) / dpr &lt; DOUBLE_TAP_SLOP
            }
            None =&gt; <span class="s">false</span>,
        };

        <span class="k">if</span> second {
            <span class="k">return</span> Act::Fit; <span class="c">// double tap</span>
        }

        <span class="k">self</span>.tap = Some((now, p));
        Act::Tap(p)
    }

    <span class="k">fn</span> finger_index(&amp;<span class="k">self</span>, id: u64) -&gt; Option&lt;usize&gt; {
        <span class="k">for</span> (index, finger) <span class="k">in</span> <span class="k">self</span>.fingers.iter().enumerate() {
            <span class="k">if</span> finger.id == id {
                <span class="k">return</span> Some(index);
            }
        }

        None
    }

    <span class="c">/// Remove one finger and reset the pinch.</span>
    <span class="k">fn</span> drop_finger(&amp;<span class="k">mut</span> <span class="k">self</span>, id: u64) -&gt; Option&lt;Finger&gt; {
        <span class="k">let</span> i = <span class="k">self</span>.finger_index(id)?;
        <span class="k">self</span>.span = <span class="s">0</span>.<span class="s">0</span>;
        Some(<span class="k">self</span>.fingers.remove(i))
    }
}

<span class="k">impl</span> Default <span class="k">for</span> Touches {
    <span class="c">/// Clippy asks for Default whenever a type has a no-argument \`new\`.</span>
    <span class="k">fn</span> default() -&gt; <span class="k">Self</span> {
        <span class="k">Self</span>::new()
    }
}</code></pre></div>
<h2 id="step-6-srcappsceners">Step 6 · src/app/scene.rs<a class="anchor" href="#/course/12-picking#step-6-srcappsceners" aria-label="Link to this section">#</a></h2>
<p>The scene owns source documents and maps their identities to GPU rows.</p>
<p><code>lessons/12/src/app/scene.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! The document the viewer shows: the objects, the rows they occupy on the GPU, and what is currently selected.</span>
<span class="k">use</span> <span class="k">crate</span>::app::knobs;
<span class="k">use</span> <span class="k">crate</span>::app::stream::{CloudFields, CloudLod};
<span class="k">use</span> <span class="k">crate</span>::app::walk::bounds::{Baselines, file_extent, is_planar, mark_sheet};
<span class="k">use</span> <span class="k">crate</span>::app::walk::cloud::{StreamRows, StreamSlice, walk_stream_slice};
<span class="k">use</span> <span class="k">crate</span>::app::walk::mesh::Lap;
<span class="k">use</span> <span class="k">crate</span>::app::walk::{Walk, WalkCx, is_drawable, walk_geometry};
<span class="k">use</span> <span class="k">crate</span>::engine::gpu::{Gpu, Instance, ObjectRow, Pick, Upload};
<span class="k">use</span> session_rust::{Geometry, Session, Xform};
<span class="k">use</span> std::collections::{HashMap, HashSet};
<span class="k">use</span> std::rc::Rc;

<span class="c">/// One loaded file and its placement.</span>
<span class="k">pub</span> <span class="k">struct</span> FileDoc {
    <span class="k">pub</span> name: String,
    <span class="k">pub</span> place: Xform,
    <span class="k">pub</span> session: Rc&lt;Session&gt;, <span class="c">// the kernel document, shared when a file is loaded twice</span>
    <span class="k">pub</span> point_px: f32, <span class="c">// point size override, 0 = the file's own</span>
    <span class="k">pub</span> display_only: bool, <span class="c">// a streamed shell with no kernel objects</span>
}

<span class="c">/// A streamed cloud's first slice.</span>
<span class="k">pub</span> <span class="k">struct</span> StreamedInit {
    <span class="k">pub</span> name: String,
    <span class="k">pub</span> url: String, <span class="c">// the cloud file</span>
    <span class="k">pub</span> place: Xform,
    <span class="k">pub</span> rows: StreamRows, <span class="c">// the first points</span>
    <span class="k">pub</span> lod: CloudLod, <span class="c">// the whole node table</span>
    <span class="k">pub</span> fields: CloudFields, <span class="c">// array positions in the file</span>
    <span class="k">pub</span> resident: u32, <span class="c">// points in this slice</span>
    <span class="k">pub</span> point_px: f32, <span class="c">// point size override</span>
    <span class="k">pub</span> col_at: u64, <span class="c">// byte position of the next colour</span>
}

<span class="c">/// A streamed cloud's slot in the scene.</span>
<span class="k">pub</span> <span class="k">struct</span> StreamedCloud {
    <span class="k">pub</span> name: String,
    <span class="k">pub</span> url: String, <span class="c">// the cloud file</span>
    <span class="k">pub</span> row: u32,
    <span class="k">pub</span> lod: CloudLod, <span class="c">// the whole node table</span>
    <span class="k">pub</span> fields: CloudFields, <span class="c">// array positions in the file</span>
    <span class="k">pub</span> place: Xform,
    <span class="k">pub</span> done_to: u32,
    <span class="k">pub</span> total: u32, <span class="c">// points in the file</span>
    <span class="k">pub</span> point_px: f32, <span class="c">// point size override</span>
}</code></pre></div>
<p><code>lessons/12/src/app/scene.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// What a pick landed on.</span>
#[derive(Clone, Debug)]
<span class="k">pub</span> <span class="k">struct</span> Picked {
    <span class="k">pub</span> doc: String, <span class="c">// document name</span>
    <span class="k">pub</span> guid: String,
    <span class="k">pub</span> row: u32,
    <span class="k">pub</span> point: Option&lt;PickedPoint&gt;,
}

<span class="c">/// A picked cloud point.</span>
#[derive(Clone, Debug)]
<span class="k">pub</span> <span class="k">struct</span> PickedPoint {
    <span class="k">pub</span> local: u32, <span class="c">// index in the cloud</span>
    <span class="k">pub</span> id: u32, <span class="c">// the point's stable id</span>
    <span class="k">pub</span> position: [f64; <span class="s">3</span>],
}

<span class="c">/// Rows already on the GPU, per table.</span>
#[derive(Default)]
<span class="k">struct</span> Bases {
    vert: u32,
    ribbon: u32,
    obj: u32, <span class="c">// object rows</span>
}

<span class="c">/// The open documents and their object rows.</span>
<span class="k">pub</span> <span class="k">struct</span> Scene {
    <span class="k">pub</span> docs: Vec&lt;FileDoc&gt;,
    <span class="k">pub</span> tables: Upload, <span class="c">// rows walked but not yet uploaded</span>
    <span class="k">pub</span> streamed: Vec&lt;StreamedCloud&gt;,
    <span class="k">pub</span> hidden: HashSet&lt;(usize, Rc&lt;str&gt;)&gt;, <span class="c">// (document, guid) hidden</span>
    <span class="k">pub</span> selected: Option&lt;u32&gt;, <span class="c">// selected object row</span>
    order: Vec&lt;Rc&lt;str&gt;&gt;, <span class="c">// guid of each row</span>
    owners: Vec&lt;usize&gt;, <span class="c">// document of each row</span>
    edge_sources: Vec&lt;(u32, u32)&gt;, <span class="c">// (object row, edge index) of each pipe</span>
    ribbon_ranges: Vec&lt;Option&lt;std::ops::Range&lt;u32&gt;&gt;&gt;, <span class="c">// ribbon rows of each object</span>
    guid_to_row: HashMap&lt;(usize, Rc&lt;str&gt;), u32&gt;,
    bases: Bases, <span class="c">// rows already on the GPU</span>
}

<span class="k">impl</span> Default <span class="k">for</span> Scene {
    <span class="c">/// Same as \`new\`.</span>
    <span class="k">fn</span> default() -&gt; <span class="k">Self</span> {
        <span class="k">Self</span>::new()
    }
}

<span class="k">impl</span> Scene {
    <span class="c">/// Empty: no documents, no rows.</span>
    <span class="k">pub</span> <span class="k">fn</span> new() -&gt; <span class="k">Self</span> {
        <span class="k">Self</span> {
            docs: Vec::new(),
            tables: Upload::default(),
            streamed: Vec::new(),
            hidden: HashSet::new(),
            selected: None,</code></pre></div>
<p><code>lessons/12/src/app/scene.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            order: Vec::new(),
            owners: Vec::new(),
            edge_sources: Vec::new(),
            ribbon_ranges: Vec::new(),
            guid_to_row: HashMap::new(),
            bases: Bases::default(),
        }
    }

    <span class="c">/// Drop every document and its GPU rows.</span>
    <span class="k">pub</span> <span class="k">fn</span> clear(&amp;<span class="k">mut</span> <span class="k">self</span>, gpu: &amp;<span class="k">mut</span> Gpu) {
        <span class="k">self</span>.docs.clear();
        <span class="k">self</span>.tables = Upload::default();
        <span class="k">self</span>.streamed.clear();
        <span class="k">self</span>.order.clear();
        <span class="k">self</span>.owners.clear();
        <span class="k">self</span>.edge_sources.clear();
        <span class="k">self</span>.ribbon_ranges.clear();
        <span class="k">self</span>.guid_to_row.clear();
        <span class="k">self</span>.hidden.clear();
        <span class="k">self</span>.selected = None;
        <span class="k">self</span>.bases = Bases::default();
        gpu.release();
    }

    <span class="c">/// Walk every document again and upload from scratch.</span>
    <span class="k">pub</span> <span class="k">fn</span> rebuild(&amp;<span class="k">mut</span> <span class="k">self</span>, gpu: &amp;<span class="k">mut</span> Gpu) {
        <span class="k">let</span> docs = std::mem::take(&amp;<span class="k">mut</span> <span class="k">self</span>.docs);
        <span class="k">self</span>.tables = Upload::default();
        <span class="k">self</span>.streamed.clear();
        <span class="k">self</span>.order.clear();
        <span class="k">self</span>.owners.clear();
        <span class="k">self</span>.edge_sources.clear();
        <span class="k">self</span>.ribbon_ranges.clear();
        <span class="k">self</span>.guid_to_row.clear();
        <span class="k">self</span>.selected = None;
        <span class="k">self</span>.bases = Bases::default();
        gpu.reset();

        <span class="k">for</span> d <span class="k">in</span> docs {
            <span class="k">if</span> d.display_only {
                log::warn!(
                    &quot;<span class="s">rebuild: '</span>{}<span class="s">' is display_only, its geometry was released</span>&quot;,
                    d.name
                );
            }

            <span class="k">self</span>.add_file(FileDoc {
                name: d.name, <span class="c">// the document's title</span>
                session: d.session,
                place: d.place, <span class="c">// where the document sits</span>
                point_px: d.point_px, <span class="c">// point size in CSS pixels</span>
                display_only: d.display_only, <span class="c">// streamed, not editable</span>
            });
        }

        <span class="k">self</span>.upload_to(gpu);
    }</code></pre></div>
<p><code>lessons/12/src/app/scene.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Upload the walked tables and clear them.</span>
    <span class="k">pub</span> <span class="k">fn</span> upload_to(&amp;<span class="k">mut</span> <span class="k">self</span>, gpu: &amp;<span class="k">mut</span> Gpu) {
        <span class="k">for</span> (index, pipe) <span class="k">in</span> <span class="k">self</span>.tables.seg.pipes.iter().enumerate() {
            <span class="k">let</span> edge = <span class="k">self</span>
                .tables
                .seg
                .pipe_ids
                .get(index)
                .copied()
                .unwrap_or(u32::MAX);
            <span class="k">self</span>.edge_sources.push((pipe.instance_id, edge));
        }

        gpu.set_scene(&amp;<span class="k">self</span>.tables);
        <span class="k">self</span>.bases.vert += <span class="k">self</span>.tables.arena.verts.len() <span class="k">as</span> u32;
        <span class="k">self</span>.bases.obj += <span class="k">self</span>.tables.obj.rows.len() <span class="k">as</span> u32;
        <span class="k">self</span>.bases.ribbon += <span class="k">self</span>.tables.seg.ribbons.len() <span class="k">as</span> u32;
        <span class="k">self</span>.tables.drop_uploaded();
    }

    <span class="c">/// Add one object row for \`guid\` of document \`owner\`.</span>
    <span class="k">pub</span>(super) <span class="k">fn</span> push_row(&amp;<span class="k">mut</span> <span class="k">self</span>, owner: usize, guid: &amp;str, place: Xform, flags: u32) -&gt; u32 {
        <span class="k">let</span> row = <span class="k">self</span>.bases.obj + <span class="k">self</span>.tables.obj.rows.len() <span class="k">as</span> u32;
        <span class="k">self</span>.tables.obj.rows.push(ObjectRow::new(place, flags));</code></pre></div>
<p><code>lessons/12/src/app/scene.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> guid: Rc&lt;str&gt; = Rc::from(guid);
        <span class="k">self</span>.guid_to_row.insert((owner, Rc::clone(&amp;guid)), row);
        <span class="k">self</span>.order.push(guid);
        <span class="k">self</span>.owners.push(owner);
        <span class="k">self</span>.ribbon_ranges.push(None);
        row
    }

    <span class="c">/// Add one document: one row per object, then the file sweeps.</span>
    <span class="k">pub</span> <span class="k">fn</span> add_file(&amp;<span class="k">mut</span> <span class="k">self</span>, doc: FileDoc) {
        <span class="k">let</span> FileDoc {
            name,
            session,
            place,
            point_px,
            display_only,
        } = doc;
        <span class="k">let</span> from = Baselines::capture(&amp;<span class="k">self</span>.tables);
        <span class="k">let</span> world = session.world_xforms();
        <span class="k">let</span> <span class="k">mut</span> lap = Lap::start(&quot;<span class="s">walk</span>&quot;);
        <span class="k">let</span> count = session.lookup.len();
        <span class="k">self</span>.tables.obj.rows.reserve(count);
        <span class="k">self</span>.order.reserve(count);
        <span class="k">self</span>.guid_to_row.reserve(count);

        <span class="k">for</span> guid <span class="k">in</span> session.order() {
            <span class="k">let</span> Some(geom) = session.lookup.get(&amp;guid) <span class="k">else</span> {
                <span class="k">continue</span>;
            };

            <span class="k">if</span> !is_drawable(geom) {
                <span class="k">continue</span>;
            }

            <span class="k">let</span> flags = <span class="k">if</span> <span class="k">self</span>
                .hidden
                .contains(&amp;(<span class="k">self</span>.docs.len(), Rc::from(guid.as_str())))
            {
                Instance::FLAG_HIDDEN
            } <span class="k">else</span> {
                <span class="s">0</span>
            };
            <span class="k">let</span> object_place = placement(&amp;world, &amp;place, &amp;guid);
            <span class="k">let</span> row = <span class="k">self</span>.push_row(<span class="k">self</span>.docs.len(), &amp;guid, object_place, flags);
            <span class="k">let</span> ribbon_start = <span class="k">self</span>.tables.seg.ribbons.len();
            <span class="k">let</span> cx = WalkCx {
                vert_base: <span class="k">self</span>.bases.vert,
                cloud_px: point_px,
                row,
            };
            <span class="k">let</span> r = walk_geometry(&amp;<span class="k">mut</span> Walk::of(&amp;<span class="k">mut</span> <span class="k">self</span>.tables), &amp;cx, geom);
            <span class="k">let</span> o = <span class="k">self</span>.tables.obj.rows.last_mut().unwrap();
            o.flags |= r.flags;
            o.bounds = r.bounds;
            o.spacing = r.spacing;
            o.faces = r.faces;
            <span class="k">let</span> ribbon_end = <span class="k">self</span>.tables.seg.ribbons.len();

            <span class="k">if</span> ribbon_start != ribbon_end {
                <span class="k">self</span>.ribbon_ranges[row <span class="k">as</span> usize] = Some(
                    <span class="k">self</span>.bases.ribbon + ribbon_start <span class="k">as</span> u32..<span class="k">self</span>.bases.ribbon + ribbon_end <span class="k">as</span> u32,
                );
            }
        }

        lap.mark(&quot;<span class="s">objects</span>&quot;);

        <span class="k">let</span> extent = file_extent(&amp;<span class="k">self</span>.tables, &amp;from);</code></pre></div>
<p><code>lessons/12/src/app/scene.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.tables.bounds.union_with(&amp;extent);

        <span class="k">if</span> is_planar(&amp;<span class="k">self</span>.tables, &amp;from, &amp;place) {
            mark_sheet(&amp;<span class="k">mut</span> <span class="k">self</span>.tables, &amp;from);
        }

        lap.mark(&quot;<span class="s">sweeps</span>&quot;);

        <span class="k">if</span> display_only || knobs::drop_sessions() {
            log::info!(
                &quot;<span class="s">'</span>{<span class="s">name</span>}<span class="s">': retaining source geometry for controls; the legacy display_only/drop_sessions hint no longer releases it</span>&quot;
            );
        }

        <span class="k">self</span>.docs.push(FileDoc {
            name,
            place,
            session,
            point_px,
            display_only: <span class="s">false</span>,
        });
    }

    <span class="c">/// Add a streamed cloud from its first slice; returns its slot.</span>
    <span class="k">pub</span> <span class="k">fn</span> add_streamed_cloud(&amp;<span class="k">mut</span> <span class="k">self</span>, init: StreamedInit, gpu: &amp;<span class="k">mut</span> Gpu) -&gt; usize {
        <span class="k">let</span> StreamedInit {
            name,
            url,
            place,
            rows,
            lod,
            fields,
            resident,
            point_px,
            col_at: _,
        } = init;
        <span class="k">let</span> total = fields.count;
        <span class="k">let</span> row = <span class="k">self</span>.push_row(<span class="k">self</span>.docs.len(), &amp;format!(&quot;<span class="s">stream:</span>{<span class="s">url</span>}&quot;), place.clone(), <span class="s">0</span>);
        <span class="k">let</span> slice = StreamSlice {
            rows,
            lod: &amp;lod,
            from: <span class="s">0</span>,
            to: resident,
            row,
            point_px,
        };
        <span class="k">let</span> bounds = walk_stream_slice(&amp;<span class="k">mut</span> <span class="k">self</span>.tables.cloud, &amp;slice);
        <span class="k">let</span> o = <span class="k">self</span>.tables.obj.rows.last_mut().unwrap();
        o.bounds = bounds;
        o.spacing = point_px;
        <span class="k">self</span>.tables.bounds.union_with(&amp;bounds.transformed(&amp;place));
        <span class="k">self</span>.upload_to(gpu);

        <span class="k">let</span> model = place.clone();
        <span class="k">self</span>.docs.push(FileDoc {
            name: name.clone(),
            place,
            session: Rc::new(Session::new(&amp;name)),
            point_px,
            display_only: <span class="s">true</span>,
        });
        <span class="k">self</span>.streamed.push(StreamedCloud {
            name,
            url,</code></pre></div>
<p><code>lessons/12/src/app/scene.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            row,
            lod,
            fields,
            place: model,
            done_to: resident,
            total,
            point_px,
        });
        <span class="k">self</span>.streamed.len() - <span class="s">1</span>
    }

    <span class="c">/// Add the next slice of streamed cloud \`idx\`.</span>
    <span class="k">pub</span> <span class="k">fn</span> extend_streamed_cloud(&amp;<span class="k">mut</span> <span class="k">self</span>, idx: usize, rows: StreamRows, to: u32, gpu: &amp;<span class="k">mut</span> Gpu) {
        <span class="k">let</span> Some(sc) = <span class="k">self</span>.streamed.get(idx) <span class="k">else</span> {
            <span class="k">return</span>;</code></pre></div>
<p><code>lessons/12/src/app/scene.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        };

        <span class="k">if</span> to &lt;= sc.done_to {
            <span class="k">return</span>;
        }

        <span class="k">let</span> place = <span class="k">match</span> <span class="k">self</span>.document(sc.row) {
            Some(document) =&gt; document.place.clone(),
            None =&gt; Xform::identity(),
        };
        <span class="k">let</span> slice = StreamSlice {
            rows,
            lod: &amp;sc.lod,
            from: sc.done_to,
            to,
            row: sc.row,
            point_px: sc.point_px,
        };
        <span class="k">let</span> bounds = walk_stream_slice(&amp;<span class="k">mut</span> <span class="k">self</span>.tables.cloud, &amp;slice);
        <span class="k">self</span>.tables.bounds.union_with(&amp;bounds.transformed(&amp;place));
        <span class="k">self</span>.streamed[idx].done_to = to;
        <span class="k">self</span>.upload_to(gpu);
    }

    <span class="c">/// What a GPU pick landed on.</span>
    <span class="k">pub</span> <span class="k">fn</span> resolve(&amp;<span class="k">self</span>, pick: Pick, gpu: &amp;Gpu) -&gt; Option&lt;Picked&gt; {
        <span class="k">let</span> guid = <span class="k">self</span>.order.get(pick.row <span class="k">as</span> usize)?.to_string();
        <span class="k">let</span> <span class="k">mut</span> point = None;

        <span class="k">if</span> <span class="k">let</span> Some((parent, local)) = gpu.cloud.row_of(pick.sub)
            &amp;&amp; parent == pick.row
        {
            point = <span class="k">self</span>.point_at(pick.row, local);
        }

        <span class="k">let</span> doc = <span class="k">match</span> <span class="k">self</span>.document(pick.row) {
            Some(document) =&gt; document.name.clone(),
            None =&gt; String::new(),
        };
        Some(Picked {
            doc,
            guid,
            row: pick.row,
            point,
        })
    }

    <span class="c">/// The document a row belongs to.</span>
    <span class="k">pub</span> <span class="k">fn</span> document(&amp;<span class="k">self</span>, row: u32) -&gt; Option&lt;&amp;FileDoc&gt; {
        <span class="k">self</span>.docs.get(*<span class="k">self</span>.owners.get(row <span class="k">as</span> usize)?)
    }

    <span class="c">/// The kernel geometry of a row.</span>
    <span class="k">pub</span> <span class="k">fn</span> geometry(&amp;<span class="k">self</span>, row: u32) -&gt; Option&lt;&amp;Geometry&gt; {
        <span class="k">self</span>.document(row)?
            .session
            .lookup
            .get(<span class="k">self</span>.order.get(row <span class="k">as</span> usize)?.as_ref())
    }

    <span class="c">/// The row's name, or its type when unnamed.</span>
    <span class="k">pub</span> <span class="k">fn</span> object_name(&amp;<span class="k">self</span>, row: u32) -&gt; &amp;str {
        <span class="k">let</span> (name, kind) = <span class="k">match</span> <span class="k">self</span>.geometry(row) {
            Some(Geometry::OBB(value)) =&gt; (value.name.as_str(), &quot;<span class="s">Box</span>&quot;),
            Some(Geometry::BRep(value)) =&gt; (value.name.as_str(), &quot;<span class="s">BRep</span>&quot;),
            Some(Geometry::Element(value)) =&gt; (value.name.as_str(), &quot;<span class="s">Element</span>&quot;),
            Some(Geometry::Line(value)) =&gt; (value.name.as_str(), &quot;<span class="s">Line</span>&quot;),
            Some(Geometry::Mesh(value)) =&gt; (value.name.as_str(), &quot;<span class="s">Mesh</span>&quot;),</code></pre></div>
<p><code>lessons/12/src/app/scene.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            Some(Geometry::NurbsCurve(value)) =&gt; (value.name.as_str(), &quot;<span class="s">NURBS curve</span>&quot;),
            Some(Geometry::NurbsSurface(value)) =&gt; (value.name.as_str(), &quot;<span class="s">NURBS surface</span>&quot;),
            Some(Geometry::Plane(value)) =&gt; (value.name.as_str(), &quot;<span class="s">Plane</span>&quot;),
            Some(Geometry::Point(value)) =&gt; (value.name.as_str(), &quot;<span class="s">Point</span>&quot;),
            Some(Geometry::PointCloud(value)) =&gt; (value.name.as_str(), &quot;<span class="s">Point cloud</span>&quot;),
            Some(Geometry::Polyline(value)) =&gt; (value.name.as_str(), &quot;<span class="s">Polyline</span>&quot;),
            None =&gt; (&quot;&quot;, &quot;<span class="s">Object</span>&quot;),
        };

        <span class="k">if</span> name.trim().is_empty() { kind } <span class="k">else</span> { name }
    }

    <span class="c">/// The edge index a pipe pick landed on.</span>
    <span class="k">pub</span> <span class="k">fn</span> edge_at(&amp;<span class="k">self</span>, pick: Pick) -&gt; Option&lt;u32&gt; {
        <span class="k">if</span> pick.sub &amp; <span class="s">0x8000_0000</span> == <span class="s">0</span> {
            <span class="k">return</span> None;
        }

        <span class="k">let</span> &amp;(parent, edge) = <span class="k">self</span>.edge_sources.get((pick.sub &amp; <span class="s">0x7fff_ffff</span>) <span class="k">as</span> usize)?;
        (parent == pick.row &amp;&amp; edge != u32::MAX).then_some(edge)
    }

    <span class="c">/// Point \`local\` of the cloud on \`row\`; None when streamed.</span>
    <span class="k">pub</span> <span class="k">fn</span> point_at(&amp;<span class="k">self</span>, row: u32, local: u32) -&gt; Option&lt;PickedPoint&gt; {
        <span class="k">let</span> Some(Geometry::PointCloud(pc)) = <span class="k">self</span>.geometry(row) <span class="k">else</span> {
            <span class="k">return</span> None;
        };
        <span class="k">let</span> c = pc.coords();
        <span class="k">let</span> i = local <span class="k">as</span> usize * <span class="s">3</span>;

        <span class="k">if</span> i + <span class="s">2</span> &gt;= c.len() {
            <span class="k">return</span> None;
        }

        Some(PickedPoint {
            local,
            id: pc.point_id(local <span class="k">as</span> usize),
            position: [c[i], c[i + <span class="s">1</span>], c[i + <span class="s">2</span>]],
        })
    }

    <span class="c">/// The ribbon rows of one object.</span>
    <span class="k">pub</span> <span class="k">fn</span> ribbon_range(&amp;<span class="k">self</span>, row: u32) -&gt; Option&lt;std::ops::Range&lt;u32&gt;&gt; {
        <span class="k">self</span>.ribbon_ranges.get(row <span class="k">as</span> usize).cloned().flatten()
    }

    <span class="c">/// The document and guid of a row.</span>
    <span class="k">pub</span> <span class="k">fn</span> identity_of(&amp;<span class="k">self</span>, row: u32) -&gt; Option&lt;(usize, Rc&lt;str&gt;)&gt; {
        Some((
            *<span class="k">self</span>.owners.get(row <span class="k">as</span> usize)?,
            Rc::clone(<span class="k">self</span>.order.get(row <span class="k">as</span> usize)?),
        ))
    }

    <span class="c">/// The rows currently hidden.</span>
    <span class="k">pub</span> <span class="k">fn</span> hidden_rows(&amp;<span class="k">self</span>) -&gt; Vec&lt;u32&gt; {
        <span class="k">let</span> <span class="k">mut</span> rows = Vec::new();

        <span class="k">for</span> identity <span class="k">in</span> &amp;<span class="k">self</span>.hidden {
            <span class="k">if</span> <span class="k">let</span> Some(&amp;row) = <span class="k">self</span>.guid_to_row.get(identity) {
                rows.push(row);
            }
        }

        rows
    }

    <span class="c">/// Objects in row order.</span>
    <span class="k">pub</span> <span class="k">fn</span> object_count(&amp;<span class="k">self</span>) -&gt; usize {
        <span class="k">self</span>.order.len()
    }
}

<span class="c">/// File placement times the object's own transform.</span>
<span class="k">fn</span> placement(world: &amp;HashMap&lt;String, Xform&gt;, place: &amp;Xform, guid: &amp;str) -&gt; Xform {
    <span class="k">match</span> world.get(guid) {
        Some(local) =&gt; place * local,
        None =&gt; place.clone(),
    }
}</code></pre></div>
<h2 id="step-7-srcappselectionrs">Step 7 · src/app/selection.rs<a class="anchor" href="#/course/12-picking#step-7-srcappselectionrs" aria-label="Link to this section">#</a></h2>
<p>New file: what inside the picked object is selected, nothing or one edge.</p>
<p><code>lessons/12/src/app/selection.rs</code> · 31 lines · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! The picked object lives in Scene; this says what inside that object is picked, if anything.</span>

<span class="c">/// \`#[default]\` marks the variant that \`Default::default()\` returns.</span>
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize)]
<span class="k">pub</span> <span class="k">enum</span> SelectionMode {
    #[default]
    Object,
    Edge {
        parent: u32, <span class="c">// object row</span>
        edge: u32,
    },
}

<span class="k">impl</span> SelectionMode {
    <span class="c">/// The object holding the sub-selection; None when the whole object is selected.</span>
    <span class="k">pub</span> <span class="k">fn</span> parent(&amp;<span class="k">self</span>) -&gt; Option&lt;u32&gt; {
        <span class="k">match</span> <span class="k">self</span> {
            <span class="k">Self</span>::Object =&gt; None,
            <span class="k">Self</span>::Edge { parent, .. } =&gt; Some(*parent),
        }
    }

    <span class="k">pub</span> <span class="k">fn</span> select_edge(&amp;<span class="k">mut</span> <span class="k">self</span>, parent: u32, edge: u32) {
        *<span class="k">self</span> = <span class="k">Self</span>::Edge { parent, edge };
    }

    <span class="c">/// Back to object selection; returns the parent.</span>
    <span class="k">pub</span> <span class="k">fn</span> escape(&amp;<span class="k">mut</span> <span class="k">self</span>) -&gt; Option&lt;u32&gt; {
        <span class="k">let</span> parent = <span class="k">self</span>.parent();
        *<span class="k">self</span> = <span class="k">Self</span>::Object;
        parent
    }
}</code></pre></div>
<h2 id="step-8-srcappwalkcloudrs">Step 8 · src/app/walk/cloud.rs<a class="anchor" href="#/course/12-picking#step-8-srcappwalkcloudrs" aria-label="Link to this section">#</a></h2>
<p>New file: walk a point cloud, or one streamed slice of it, into point rows, octree nodes and one draw.</p>
<p><code>lessons/12/src/app/walk/cloud.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> super::encode::oct16;
<span class="k">use</span> super::{Row, WalkCx};
<span class="k">use</span> <span class="k">crate</span>::app::stream::CloudLod;
<span class="k">use</span> <span class="k">crate</span>::engine::gpu::cloud::CloudRows;
<span class="k">use</span> <span class="k">crate</span>::engine::gpu::{CloudDraw, LodNode, NO_NORMALS};
<span class="k">use</span> session_rust::AABB;
<span class="k">use</span> session_rust::PointCloud;

<span class="c">/// 20 mm, for a cloud too small or too flat to measure.</span>
<span class="k">const</span> DEFAULT_SPACING: f32 = <span class="s">20</span>.<span class="s">0</span>;

<span class="c">/// A point cloud: points, octree nodes, one draw.</span>
<span class="k">pub</span> <span class="k">fn</span> walk_cloud(c: &amp;<span class="k">mut</span> CloudRows, pc: &amp;PointCloud, cx: &amp;WalkCx) -&gt; Row {
    <span class="k">let</span> first = c.point_count(); <span class="c">// every cloud shares the tables; this one starts here</span>
    <span class="k">let</span> node_first = c.nodes.len() <span class="k">as</span> u32;
    <span class="c">// normals only when every point has one</span>
    <span class="k">let</span> nrm_first = <span class="k">if</span> pc.normals().len() &gt;= pc.len() * <span class="s">3</span> {
        c.nrm.len() <span class="k">as</span> u32
    } <span class="k">else</span> {
        NO_NORMALS
    };
    <span class="k">let</span> bounds = push_points(c, pc);
    push_nodes(c, pc);
    c.draws.push(CloudDraw {
        instance: cx.row,
        from: <span class="s">0</span>,
        count: pc.len() <span class="k">as</span> u32,
        first,
        spacing: cloud_spacing(pc, &amp;bounds),
        node_first,
        node_count: pc.lod_node_count() <span class="k">as</span> u32,
        nrm_first,
    });
    <span class="c">// file override wins over the cloud's own size</span>
    <span class="k">let</span> px = <span class="k">if</span> cx.cloud_px &gt; <span class="s">0</span>.<span class="s">0</span> {
        cx.cloud_px
    } <span class="k">else</span> {
        pc.point_size <span class="k">as</span> f32
    };
    Row {
        bounds,
        spacing: px,
        flags: <span class="s">0</span>,
        faces: <span class="s">false</span>,
    }
}</code></pre></div>
<p><code>lessons/12/src/app/walk/cloud.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Copy positions, colours and normals into the rows.</span>
<span class="k">fn</span> push_points(rows: &amp;<span class="k">mut</span> CloudRows, pc: &amp;PointCloud) -&gt; AABB {
    <span class="k">let</span> coords = pc.coords();
    <span class="k">let</span> colors = pc.colors();
    <span class="k">let</span> normals = pc.normals();
    <span class="k">let</span> n = pc.len();
    <span class="k">let</span> has_normals = normals.len() &gt;= n * <span class="s">3</span>;
    rows.pos.reserve(n * <span class="s">3</span>);
    rows.col.reserve(n);
    <span class="k">let</span> <span class="k">mut</span> bounds = AABB::empty();

    <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..n {
        <span class="k">let</span> p = [
            coords[i * <span class="s">3</span>] <span class="k">as</span> f32,
            coords[i * <span class="s">3</span> + <span class="s">1</span>] <span class="k">as</span> f32,
            coords[i * <span class="s">3</span> + <span class="s">2</span>] <span class="k">as</span> f32,
        ];
        bounds.union_with_point(p[<span class="s">0</span>] <span class="k">as</span> f64, p[<span class="s">1</span>] <span class="k">as</span> f64, p[<span class="s">2</span>] <span class="k">as</span> f64);
        rows.pos.extend_from_slice(&amp;p);
        <span class="k">let</span> c = i * <span class="s">4</span>;
        rows.col.push(<span class="k">if</span> c + <span class="s">3</span> &lt; colors.len() {
            pack_color(&amp;colors[c..c + <span class="s">4</span>])
        } <span class="k">else</span> {
            <span class="s">0xff00_0000</span> <span class="c">// black when missing</span>
        });

        <span class="k">if</span> has_normals {
            rows.nrm.push(
                oct16(&amp;[normals[i * <span class="s">3</span>], normals[i * <span class="s">3</span> + <span class="s">1</span>], normals[i * <span class="s">3</span> + <span class="s">2</span>]]).unwrap_or(<span class="s">0</span>), <span class="c">// a zero normal decodes as +z</span>
            );
        }
    }

    bounds
}</code></pre></div>
<p><code>lessons/12/src/app/walk/cloud.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Copy the cloud's octree nodes.</span>
<span class="k">fn</span> push_nodes(rows: &amp;<span class="k">mut</span> CloudRows, pc: &amp;PointCloud) {
    <span class="k">for</span> k <span class="k">in</span> <span class="s">0</span>..pc.lod_node_count() {
        <span class="k">let</span> (c, size) = pc.lod_cube(k);
        <span class="k">let</span> (nf, nc) = pc.lod_range(k); <span class="c">// a node's points are one run: first, count</span>
        <span class="k">let</span> <span class="k">mut</span> children = [-<span class="s">1i32</span>; <span class="s">8</span>]; <span class="c">// -1 = no child</span>

        <span class="k">for</span> (slot, v) <span class="k">in</span> pc.lod_children(k).into_iter().enumerate().take(<span class="s">8</span>) {
            children[slot] = v;
        }

        rows.nodes.push(LodNode {
            center: [c[<span class="s">0</span>] <span class="k">as</span> f32, c[<span class="s">1</span>] <span class="k">as</span> f32, c[<span class="s">2</span>] <span class="k">as</span> f32],
            size: size <span class="k">as</span> f32,
            spacing: pc.lod_spacing(k) <span class="k">as</span> f32,
            first: nf <span class="k">as</span> u32,
            count: nc <span class="k">as</span> u32,
            children,
        });
    }
}

<span class="c">/// Four 0-255 channels, red in the lowest byte like pack_rgba.</span>
<span class="k">fn</span> pack_color(c: &amp;[i32]) -&gt; u32 {
    (c[<span class="s">0</span>] <span class="k">as</span> u32 &amp; <span class="s">255</span>)
        | (c[<span class="s">1</span>] <span class="k">as</span> u32 &amp; <span class="s">255</span>) &lt;&lt; <span class="s">8</span>
        | (c[<span class="s">2</span>] <span class="k">as</span> u32 &amp; <span class="s">255</span>) &lt;&lt; <span class="s">16</span>
        | (c[<span class="s">3</span>] <span class="k">as</span> u32 &amp; <span class="s">255</span>) &lt;&lt; <span class="s">24</span>
}

<span class="c">/// Treats the cloud as a surface: sqrt(area / points), e.g. 10 m x 10 m with 1 M points gives 10 mm.</span>
<span class="k">fn</span> cloud_spacing(pc: &amp;PointCloud, bounds: &amp;AABB) -&gt; f32 {
    <span class="k">let</span> n = pc.len();

    <span class="k">if</span> n &lt; <span class="s">2</span> || !bounds.is_valid() {
        <span class="k">return</span> DEFAULT_SPACING;
    }

    <span class="k">let</span> <span class="k">mut</span> e = [
        (<span class="s">2</span>.<span class="s">0</span> * bounds.hx) <span class="k">as</span> f32,
        (<span class="s">2</span>.<span class="s">0</span> * bounds.hy) <span class="k">as</span> f32,
        (<span class="s">2</span>.<span class="s">0</span> * bounds.hz) <span class="k">as</span> f32,
    ];
    e.sort_by(descending_extent);
    <span class="k">let</span> area = e[<span class="s">0</span>] <span class="k">as</span> f64 * e[<span class="s">1</span>] <span class="k">as</span> f64; <span class="c">// two longest sides</span>

    <span class="k">if</span> area &lt;= <span class="s">0</span>.<span class="s">0</span> || !area.is_finite() {
        <span class="k">return</span> DEFAULT_SPACING;
    }

    (area / n <span class="k">as</span> f64).sqrt() <span class="k">as</span> f32</code></pre></div>
<p><code>lessons/12/src/app/walk/cloud.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>}

<span class="c">/// Raw point columns of one streamed slice.</span>
<span class="k">pub</span> <span class="k">struct</span> StreamRows {
    <span class="k">pub</span> positions: Vec&lt;f32&gt;, <span class="c">// three floats per point</span>
    <span class="k">pub</span> colors: Vec&lt;u32&gt;, <span class="c">// packed RGBA per point</span>
}

<span class="c">/// One slice of a streamed cloud and where it goes.</span>
<span class="k">pub</span> <span class="k">struct</span> StreamSlice&lt;'a&gt; {
    <span class="k">pub</span> rows: StreamRows,
    <span class="k">pub</span> lod: &amp;'a CloudLod, <span class="c">// the whole node table</span>
    <span class="k">pub</span> from: u32, <span class="c">// first point index in the cloud</span>
    <span class="k">pub</span> to: u32, <span class="c">// one past the last</span>
    <span class="k">pub</span> row: u32,
    <span class="k">pub</span> point_px: f32, <span class="c">// file's point size</span>
}

<span class="c">/// Append one streamed slice; return its box.</span>
<span class="k">pub</span> <span class="k">fn</span> walk_stream_slice(c: &amp;<span class="k">mut</span> CloudRows, s: &amp;StreamSlice) -&gt; AABB {
    <span class="k">let</span> first = c.point_count();
    <span class="k">let</span> node_first = c.nodes.len() <span class="k">as</span> u32;
    <span class="k">let</span> <span class="k">mut</span> node_count = <span class="s">0u32</span>;

    <span class="c">// the first slice brings the node table</span>
    <span class="k">if</span> s.from == <span class="s">0</span> {
        <span class="k">for</span> k <span class="k">in</span> <span class="s">0</span>..s.lod.len() {
            c.nodes.push(lod_node(s.lod, k));
        }

        node_count = s.lod.len() <span class="k">as</span> u32;
    }

    <span class="k">let</span> <span class="k">mut</span> bounds = AABB::empty();

    <span class="k">for</span> p <span class="k">in</span> s.rows.positions.chunks_exact(<span class="s">3</span>) {
        bounds.union_with_point(p[<span class="s">0</span>] <span class="k">as</span> f64, p[<span class="s">1</span>] <span class="k">as</span> f64, p[<span class="s">2</span>] <span class="k">as</span> f64);
    }

    <span class="k">let</span> count = (s.rows.positions.len() / <span class="s">3</span>) <span class="k">as</span> u32;
    <span class="k">let</span> colors = &amp;s.rows.colors[..s.rows.colors.len().min(count <span class="k">as</span> usize)];
    c.col.extend_from_slice(colors);
    c.col.resize(first <span class="k">as</span> usize + count <span class="k">as</span> usize, <span class="s">0xff00_0000</span>); <span class="c">// pad with black</span>
    c.pos.extend_from_slice(&amp;s.rows.positions);
    c.draws.push(CloudDraw {
        instance: s.row,
        from: s.from,
        count,
        first,
        spacing: resident_spacing(s.lod, s.to).unwrap_or(s.point_px.max(DEFAULT_SPACING)), <span class="c">// no whole node yet: guess</span>
        node_first,
        node_count,
        nrm_first: NO_NORMALS,
    });
    bounds
}</code></pre></div>
<p><code>lessons/12/src/app/walk/cloud.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Finest spacing of the nodes fully loaded so far.</span>
<span class="k">fn</span> resident_spacing(lod: &amp;CloudLod, to: u32) -&gt; Option&lt;f32&gt; {
    <span class="k">let</span> <span class="k">mut</span> spacing = f64::INFINITY;

    <span class="k">for</span> k <span class="k">in</span> <span class="s">0</span>..lod.len() {
        <span class="k">let</span> (f, n) = (lod.first[k], lod.count[k]);

        <span class="k">if</span> f &gt;= <span class="s">0</span> &amp;&amp; n &gt;= <span class="s">0</span> &amp;&amp; (f + n) <span class="k">as</span> u32 &lt;= to {
            spacing = spacing.min(lod.spacing[k]);
        }
    }

    spacing.is_finite().then_some(spacing <span class="k">as</span> f32)
}

<span class="c">/// The file stores a node's min corner; the GPU wants its centre.</span>
<span class="k">fn</span> lod_node(lod: &amp;CloudLod, k: usize) -&gt; LodNode {
    <span class="k">let</span> <span class="k">mut</span> children = [-<span class="s">1i32</span>; <span class="s">8</span>];

    <span class="k">for</span> (slot, v) <span class="k">in</span> lod.children[k * <span class="s">8</span>..k * <span class="s">8</span> + <span class="s">8</span>].iter().enumerate() {
        children[slot] = *v;
    }

    <span class="k">let</span> half = lod.size[k] <span class="k">as</span> f32 * <span class="s">0</span>.<span class="s">5</span>;
    LodNode {
        center: [
            lod.min[k * <span class="s">3</span>] <span class="k">as</span> f32 + half,
            lod.min[k * <span class="s">3</span> + <span class="s">1</span>] <span class="k">as</span> f32 + half,
            lod.min[k * <span class="s">3</span> + <span class="s">2</span>] <span class="k">as</span> f32 + half,
        ],
        size: lod.size[k] <span class="k">as</span> f32,
        spacing: lod.spacing[k] <span class="k">as</span> f32,
        first: lod.first[k] <span class="k">as</span> u32,
        count: lod.count[k] <span class="k">as</span> u32,
        children,
    }
}

<span class="c">/// Largest first; unwrap panics on NaN, which a valid box never has.</span>
<span class="k">fn</span> descending_extent(a: &amp;f32, b: &amp;f32) -&gt; std::cmp::Ordering {
    b.partial_cmp(a).unwrap()
}</code></pre></div>
<h2 id="step-9-srcappwalkframesrs">Step 9 · src/app/walk/frames.rs<a class="anchor" href="#/course/12-picking#step-9-srcappwalkframesrs" aria-label="Link to this section">#</a></h2>
<p>New file: draw a plane as a 1 m square and a box as its 12 edges.</p>
<p><code>lessons/12/src/app/walk/frames.rs</code> · 86 lines · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> super::Row;
<span class="k">use</span> super::encode::{FACING_UNKNOWN, Pen, encode_width, pack_rgba};
<span class="k">use</span> <span class="k">crate</span>::engine::gpu::CylinderSegment;
<span class="k">use</span> <span class="k">crate</span>::engine::gpu::segments::SegRows;
<span class="k">use</span> session_rust::AABB;
<span class="k">use</span> session_rust::{OBB, Plane, Point, Vector};

<span class="c">/// A plane is infinite; it is drawn as a 1 m square, 500 mm each way from its origin.</span>
<span class="k">const</span> PLANE_SIZE: f64 = <span class="s">500</span>.<span class="s">0</span>;

<span class="c">/// The 12 box edges, corners bottom 0-3 then top 4-7.</span>
<span class="k">const</span> BOX_EDGES: [[usize; <span class="s">2</span>]; <span class="s">12</span>] = [
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
];

<span class="c">/// s = (±1, ±1) picks the corner.</span>
<span class="k">fn</span> corner(o: &amp;Point, x: &amp;Vector, y: &amp;Vector, s: [f64; <span class="s">2</span>]) -&gt; [f32; <span class="s">3</span>] {
    <span class="k">let</span> <span class="k">mut</span> position = [<span class="s">0</span>.<span class="s">0</span>; <span class="s">3</span>];

    <span class="k">for</span> (k, value) <span class="k">in</span> position.iter_mut().enumerate() {
        *value = (o[k] + (x[k] * s[<span class="s">0</span>] + y[k] * s[<span class="s">1</span>]) * PLANE_SIZE) <span class="k">as</span> f32;
    }

    position
}

<span class="c">/// Push the edges as segments; return the points' box.</span>
<span class="k">fn</span> push_loop(seg: &amp;<span class="k">mut</span> SegRows, pts: &amp;[[f32; <span class="s">3</span>]], edges: &amp;[[usize; <span class="s">2</span>]], pen: &amp;Pen) -&gt; AABB {
    <span class="k">let</span> <span class="k">mut</span> bounds = AABB::empty();

    <span class="k">for</span> p <span class="k">in</span> pts {
        bounds.union_with_point(p[<span class="s">0</span>] <span class="k">as</span> f64, p[<span class="s">1</span>] <span class="k">as</span> f64, p[<span class="s">2</span>] <span class="k">as</span> f64);
    }

    <span class="k">for</span> &amp;[i, j] <span class="k">in</span> edges {
        seg.ribbons.push(CylinderSegment {
            p0: pts[i],
            radius: pen.radius,
            p1: pts[j],
            instance_id: pen.row,
            color: pen.color,
            facing: FACING_UNKNOWN,
        });
    }

    bounds
}

<span class="c">/// The four edges of the plane's square.</span>
<span class="k">pub</span> <span class="k">fn</span> walk_plane(seg: &amp;<span class="k">mut</span> SegRows, pl: &amp;Plane, row: u32) -&gt; Row {
    <span class="k">let</span> (o, x, y) = (pl.origin(), pl.x_axis(), pl.y_axis());
    <span class="k">let</span> c = [
        corner(&amp;o, &amp;x, &amp;y, [<span class="s">1</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>]),
        corner(&amp;o, &amp;x, &amp;y, [-<span class="s">1</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>]),
        corner(&amp;o, &amp;x, &amp;y, [-<span class="s">1</span>.<span class="s">0</span>, -<span class="s">1</span>.<span class="s">0</span>]),
        corner(&amp;o, &amp;x, &amp;y, [<span class="s">1</span>.<span class="s">0</span>, -<span class="s">1</span>.<span class="s">0</span>]),
    ];
    <span class="k">let</span> pen = Pen {
        row,
        radius: encode_width(pl.width),
        color: pack_rgba(pl.linecolor.to_f32()),
    };
    Row::thin(push_loop(seg, &amp;c, &amp;[[<span class="s">0</span>, <span class="s">1</span>], [<span class="s">1</span>, <span class="s">2</span>], [<span class="s">2</span>, <span class="s">3</span>], [<span class="s">3</span>, <span class="s">0</span>]], &amp;pen))
}

<span class="c">/// A box as 12 black edges.</span>
<span class="k">pub</span> <span class="k">fn</span> walk_obb(seg: &amp;<span class="k">mut</span> SegRows, b: &amp;OBB, row: u32) -&gt; Row {
    <span class="k">let</span> c = b.corners_f32();
    <span class="k">let</span> pen = Pen {
        row,
        radius: <span class="s">0</span>.<span class="s">0</span>,
        color: pack_rgba([<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>]),
    };
    Row::thin(push_loop(seg, &amp;c, &amp;BOX_EDGES, &amp;pen))
}</code></pre></div>
<h2 id="step-10-srcappwalkpointsrs">Step 10 · src/app/walk/points.rs<a class="anchor" href="#/course/12-picking#step-10-srcappwalkpointsrs" aria-label="Link to this section">#</a></h2>
<p>New file: draw a point as one dot.</p>
<p><code>lessons/12/src/app/walk/points.rs</code> · 22 lines · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> super::Row;
<span class="k">use</span> super::encode::{FACING_UNKNOWN, encode_width};
<span class="k">use</span> <span class="k">crate</span>::engine::gpu::GlyphPoint;
<span class="k">use</span> <span class="k">crate</span>::engine::gpu::glyphs::GlyphRows;
<span class="k">use</span> session_rust::AABB;
<span class="k">use</span> session_rust::Point;

<span class="c">/// A point becomes one dot row and a box of zero size.</span>
<span class="k">pub</span> <span class="k">fn</span> walk_point(glyph: &amp;<span class="k">mut</span> GlyphRows, p: &amp;Point, row: u32) -&gt; Row {
    <span class="k">let</span> center = p.to_f32();
    glyph.dots.push(GlyphPoint {
        center,
        radius: encode_width(p.width),
        color: p.pointcolor.to_f32(),
        instance_id: row,
        facing: FACING_UNKNOWN,
        facing_ext: [FACING_UNKNOWN; <span class="s">2</span>],
    });
    <span class="k">let</span> <span class="k">mut</span> bounds = AABB::empty();
    bounds.union_with_point(center[<span class="s">0</span>] <span class="k">as</span> f64, center[<span class="s">1</span>] <span class="k">as</span> f64, center[<span class="s">2</span>] <span class="k">as</span> f64);
    Row::thin(bounds)
}</code></pre></div>
<h2 id="step-11-srcappstreamrs">Step 11 · src/app/stream.rs<a class="anchor" href="#/course/12-picking#step-11-srcappstreamrs" aria-label="Link to this section">#</a></h2>
<p>Streaming reads bounded chunks and keeps stable source addresses.</p>
<p><code>lessons/12/src/app/stream.rs</code> · 38 lines · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! Reads a file in ranges as it arrives, so a big scene starts drawing before the whole download has finished.</span>

<span class="c">/// Byte positions of a cloud's arrays in its file.</span>
#[derive(Clone, Debug)]
<span class="k">pub</span> <span class="k">struct</span> CloudFields {
    <span class="k">pub</span> end: u64, <span class="c">// end of the cloud message</span>
    <span class="k">pub</span> coords_at: u64,
    <span class="k">pub</span> coords_len: u64,
    <span class="k">pub</span> colors_at: u64,
    <span class="k">pub</span> colors_len: u64, <span class="c">// their length</span>
    <span class="k">pub</span> count: u32, <span class="c">// points in the cloud</span>
    <span class="k">pub</span> ids_at: u64, <span class="c">// start of the original ids, 0 = none</span>
    <span class="k">pub</span> ids_len: u64, <span class="c">// their length</span>
    <span class="k">pub</span> revision: Option&lt;String&gt;, <span class="c">// file ETag every read must match</span>
}

<span class="c">/// A cloud's octree node table.</span>
#[derive(Clone, Default)]
<span class="k">pub</span> <span class="k">struct</span> CloudLod {
    <span class="k">pub</span> min: Vec&lt;f64&gt;, <span class="c">// cube corner, three per node</span>
    <span class="k">pub</span> size: Vec&lt;f64&gt;, <span class="c">// cube size per node</span>
    <span class="k">pub</span> spacing: Vec&lt;f64&gt;, <span class="c">// point spacing per node</span>
    <span class="k">pub</span> level: Vec&lt;i32&gt;, <span class="c">// depth per node</span>
    <span class="k">pub</span> first: Vec&lt;i32&gt;, <span class="c">// first point per node</span>
    <span class="k">pub</span> count: Vec&lt;i32&gt;, <span class="c">// points per node</span>
    <span class="k">pub</span> children: Vec&lt;i32&gt;, <span class="c">// eight child indices per node, -1 = none</span>
}

<span class="k">impl</span> CloudLod {
    <span class="c">/// Number of nodes.</span>
    <span class="k">pub</span> <span class="k">fn</span> len(&amp;<span class="k">self</span>) -&gt; usize {
        <span class="k">self</span>.size.len()
    }

    <span class="c">/// True when the file carried no octree.</span>
    <span class="k">pub</span> <span class="k">fn</span> is_empty(&amp;<span class="k">self</span>) -&gt; bool {
        <span class="k">self</span>.size.is_empty()
    }
}</code></pre></div>
<h2 id="step-12-srcappfeedbackrs">Step 12 · src/app/feedback.rs<a class="anchor" href="#/course/12-picking#step-12-srcappfeedbackrs" aria-label="Link to this section">#</a></h2>
<p>New file: status messages, and the error panel with its reload button.</p>
<p><code>lessons/12/src/app/feedback.rs</code> · 29 lines · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! Messages for the person using the viewer: what loaded, what failed, what is selected.</span>

<span class="c">/// Show a message in the status line.</span>
<span class="k">pub</span> <span class="k">fn</span> status(message: &amp;str) {
    <span class="c">// #[cfg] on a statement: the native build drops it and only logs</span>
    #[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
    <span class="k">if</span> <span class="k">let</span> Some(window) = web_sys::window()
        &amp;&amp; <span class="k">let</span> Some(document) = window.document()
        &amp;&amp; <span class="k">let</span> Some(status) = document.get_element_by_id(&quot;<span class="s">viewer-status</span>&quot;)
    {
        status.set_text_content(Some(message));
    }

    log::info!(&quot;{<span class="s">message</span>}&quot;);
}

<span class="c">/// Show the error panel with a reload button.</span>
<span class="k">pub</span> <span class="k">fn</span> error(message: &amp;str) {
    #[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
    <span class="k">if</span> <span class="k">let</span> Some(window) = web_sys::window()
        &amp;&amp; <span class="k">let</span> Some(document) = window.document()
        &amp;&amp; <span class="k">let</span> Some(panel) = document.get_element_by_id(&quot;<span class="s">viewer-error</span>&quot;)
    {
        <span class="k">if</span> <span class="k">let</span> Some(text) = document.get_element_by_id(&quot;<span class="s">viewer-error-message</span>&quot;) {
            text.set_text_content(Some(message));
        }

        <span class="k">let</span> _ = panel.remove_attribute(&quot;<span class="s">hidden</span>&quot;);
    }

    log::error!(&quot;{<span class="s">message</span>}&quot;);
}</code></pre></div>
<h2 id="step-13-srcappinspectionrs">Step 13 · src/app/inspection.rs<a class="anchor" href="#/course/12-picking#step-13-srcappinspectionrs" aria-label="Link to this section">#</a></h2>
<p>Copy the file: with ?inspect=1, a JSON snapshot of counts and memory for the browser tests.</p>
<p><code>lessons/12/src/app/inspection.rs</code> · 113 lines · copy the file, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! With ?inspect=1, a JSON snapshot of counts, memory and timings for the browser tests.</span>
#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">use</span> <span class="k">crate</span>::State;

<span class="c">/// Stored in the canvas attribute data-viewer-inspection, so a test reads it without calling into wasm.</span>
#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">pub</span> <span class="k">fn</span> publish(state: &amp;State) {
    <span class="k">if</span> super::route::query(&quot;<span class="s">inspect</span>&quot;).as_deref() != Some(&quot;<span class="s">1</span>&quot;) {
        <span class="k">return</span>; <span class="c">// only with ?inspect=1</span>
    }

    <span class="k">let</span> Some(window) = web_sys::window() <span class="k">else</span> {
        <span class="k">return</span>;
    };
    <span class="k">let</span> Some(document) = window.document() <span class="k">else</span> {
        <span class="k">return</span>;
    };
    <span class="k">let</span> Some(canvas) = document.get_element_by_id(&quot;<span class="s">canvas</span>&quot;) <span class="k">else</span> {
        <span class="k">return</span>;
    };
    <span class="k">let</span> (buffers, textures) = state.gpu.allocated_bytes();
    <span class="k">let</span> parent = state.scene.selected;
    <span class="k">let</span> model = <span class="k">match</span> parent {
        Some(row) =&gt; state.gpu.objects.anchored_model(row),
        None =&gt; None,
    };
    <span class="k">let</span> identity = selected_identity(state);
    <span class="k">let</span> snapshot = serde_json::json!({
        &quot;<span class="s">submitted_at_ms</span>&quot;: <span class="k">crate</span>::engine::performance::now_ms(),
        &quot;<span class="s">frames</span>&quot;: state.gpu.performance.frames,
        &quot;<span class="s">draw_calls</span>&quot;: state.gpu.performance.draws,
        &quot;<span class="s">selected</span>&quot;: parent,
        &quot;<span class="s">identity</span>&quot;: identity,
        &quot;<span class="s">selection</span>&quot;: state.selection,
        &quot;<span class="s">controls</span>&quot;: Vec::&lt;serde_json::Value&gt;::new(),
        &quot;<span class="s">markers</span>&quot;: state.gpu.controls.dot_count(),
        &quot;<span class="s">control_segments</span>&quot;: state.gpu.control_net.ribbon_count(),
        &quot;<span class="s">pick_busy</span>&quot;: state.gpu.pick.busy(),
        &quot;<span class="s">objects</span>&quot;: state.scene.object_count(),
        &quot;<span class="s">cloud_points</span>&quot;: state.gpu.cloud.resident(),
        &quot;<span class="s">vertices</span>&quot;: state.gpu.arena.vert_count(),
        &quot;<span class="s">pipes</span>&quot;: state.gpu.segments.pipe_count(),
        &quot;<span class="s">ribbons</span>&quot;: state.gpu.segments.ribbon_count(),
        &quot;<span class="s">mvp</span>&quot;: state.gpu.frame.mvp_f32,
        &quot;<span class="s">model</span>&quot;: model,
        &quot;<span class="s">origin</span>&quot;: state.gpu.objects.anchor(),
        &quot;<span class="s">canvas</span>&quot;: [state.gpu.config.width, state.gpu.config.height],
        &quot;<span class="s">logical_canvas</span>&quot;: state.gpu.logical_size,
        &quot;<span class="s">samples</span>&quot;: state.gpu.targets.samples,
        &quot;<span class="s">text_cpu_raster_image_capacity_bytes</span>&quot;: state.gpu.text.stats.raster_image_capacity_bytes,
        &quot;<span class="s">text_cpu_scope</span>&quot;: &quot;<span class="s">Swash image byte-vector capacity only; font/shaper/layout/hash metadata excluded</span>&quot;,
        &quot;<span class="s">gpu_buffer_capacity_bytes</span>&quot;: buffers,
        &quot;<span class="s">gpu_texture_estimate_bytes</span>&quot;: textures,
        &quot;<span class="s">glyphon_private_gpu_capacity</span>&quot;: &quot;<span class="s">not exposed by pinned dependency; separate from totals</span>&quot;,
        &quot;<span class="s">text</span>&quot;: state.gpu.text.stats,
        &quot;<span class="s">text_labels</span>&quot;: text_labels(state),
        &quot;<span class="s">wasm_capacity_bytes</span>&quot;: <span class="k">crate</span>::engine::performance::heap_mb() * <span class="s">1_048_576</span>.<span class="s">0</span>,
    });
    <span class="k">let</span> _ = canvas.set_attribute(&quot;<span class="s">data-viewer-inspection</span>&quot;, &amp;snapshot.to_string());
}

<span class="c">/// Document index and guid of the selected row.</span>
#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">fn</span> selected_identity(state: &amp;State) -&gt; Option&lt;(usize, String)&gt; {
    <span class="k">let</span> row = state.scene.selected?;
    <span class="k">let</span> (document, guid) = state.scene.identity_of(row)?;
    Some((document, guid.to_string()))
}

<span class="c">/// Every drawn text label, as JSON.</span>
#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">fn</span> text_labels(state: &amp;State) -&gt; Vec&lt;serde_json::Value&gt; {
    <span class="k">use</span> <span class="k">crate</span>::engine::text::TextPlacement;
    <span class="k">let</span> <span class="k">mut</span> labels = Vec::new();

    <span class="k">for</span> run <span class="k">in</span> &amp;state.gpu.text.document.runs {
        <span class="k">let</span> (kind, world, padding) = <span class="k">match</span> run.label.placement {
            TextPlacement::Screen { .. } =&gt; (&quot;<span class="s">screen</span>&quot;, None, None),
            TextPlacement::Anchor { world, .. } =&gt; (&quot;<span class="s">anchor</span>&quot;, Some(world), None),
            TextPlacement::WorldBillboard { world, .. } =&gt; (&quot;<span class="s">world_billboard</span>&quot;, Some(world), None),
            TextPlacement::WorldPlane { world, .. } =&gt; (&quot;<span class="s">world_plane</span>&quot;, Some(world), None),
            TextPlacement::Nameplate { world, padding, .. } =&gt; {
                (&quot;<span class="s">nameplate</span>&quot;, Some(world), Some(padding))
            }
        };
        <span class="k">let</span> <span class="k">mut</span> width = <span class="s">0</span>.<span class="s">0f32</span>;
        <span class="k">let</span> <span class="k">mut</span> height = <span class="s">0</span>.<span class="s">0f32</span>;

        <span class="c">// the widest line and the lowest line bottom give the label's box</span>
        <span class="k">for</span> line <span class="k">in</span> run.buffer.layout_runs() {
            width = width.max(line.line_w);
            height = height.max(line.line_top + line.line_height);
        }

        labels.push(serde_json::json!({
            &quot;<span class="s">id</span>&quot;: run.label.id,
            &quot;<span class="s">text</span>&quot;: run.label.text,
            &quot;<span class="s">font_size</span>&quot;: run.label.font_size,
            &quot;<span class="s">line_height</span>&quot;: run.label.line_height,
            &quot;<span class="s">color</span>&quot;: run.label.color,
            &quot;<span class="s">placement</span>&quot;: kind,
            &quot;<span class="s">world</span>&quot;: world,
            &quot;<span class="s">padding</span>&quot;: padding,
            &quot;<span class="s">rounded</span>&quot;: matches!(run.label.placement, TextPlacement::Nameplate { rounded: <span class="s">true</span>, .. }),
            &quot;<span class="s">line_box</span>&quot;: [width, height],
            &quot;<span class="s">plane</span>&quot;: <span class="k">match</span> run.label.placement {
                TextPlacement::WorldPlane { right, up, world_height, .. } =&gt; Some((right, up, world_height)),
                _ =&gt; None,
            },
        }));
    }

    labels
}</code></pre></div>
<h2 id="step-14-srcapploaderrs">Step 14 · src/app/loader.rs<a class="anchor" href="#/course/12-picking#step-14-srcapploaderrs" aria-label="Link to this section">#</a></h2>
<p>The loader stages manifest and geometry work before publishing it.</p>
<p><code>lessons/12/src/app/loader.rs</code> · 83 lines · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! Loads a scene file and turns it into GPU rows a chunk at a time, so the page never freezes.</span>
<span class="k">use</span> super::scene::{FileDoc, Scene};
<span class="k">use</span> super::stream::CloudFields;
<span class="k">use</span> <span class="k">crate</span>::{Msg, State};
<span class="k">use</span> session_rust::{Session, Xform};
<span class="k">use</span> std::cell::RefCell;
<span class="k">use</span> std::rc::Rc;
<span class="k">use</span> std::sync::Arc;
<span class="k">use</span> winit::event_loop::EventLoopProxy;
<span class="k">use</span> winit::window::Window;

thread_local! {
    <span class="c">/// Sends messages into the event loop.</span>
    <span class="k">static</span> PROXY: RefCell&lt;Option&lt;EventLoopProxy&lt;Msg&gt;&gt;&gt; = <span class="k">const</span> { RefCell::new(None) };
}

<span class="c">/// The query-result delivery interface.</span>
<span class="k">pub</span>(super) <span class="k">fn</span> post(message: Msg) -&gt; bool {
    PROXY.with_borrow(|proxy| post_with_proxy(proxy, message))
}

<span class="c">/// Hand the result to the event loop.</span>
<span class="k">fn</span> post_with_proxy(proxy: &amp;Option&lt;EventLoopProxy&lt;Msg&gt;&gt;, message: Msg) -&gt; bool {
    <span class="k">match</span> proxy {
        Some(proxy) =&gt; proxy.send_event(message).is_ok(),
        None =&gt; <span class="s">false</span>,
    }
}

<span class="c">/// Own the proxy for asynchronous source-query messages.</span>
<span class="k">fn</span> retain_proxy(slot: &amp;<span class="k">mut</span> Option&lt;EventLoopProxy&lt;Msg&gt;&gt;, proxy: &amp;EventLoopProxy&lt;Msg&gt;) {
    *slot = Some(proxy.clone());
}

<span class="c">/// Start the viewer, load the first scene, then keep polling.</span>
<span class="k">pub</span> <span class="k">async</span> <span class="k">fn</span> boot(window: Arc&lt;Window&gt;, proxy: EventLoopProxy&lt;Msg&gt;) {
    PROXY.with_borrow_mut(|slot| retain_proxy(slot, &amp;proxy));
    <span class="k">let</span> state = <span class="k">match</span> State::new(window, Scene::new()).<span class="k">await</span> {
        Ok(state) =&gt; state,
        Err(error) =&gt; {
            super::feedback::error(&amp;format!(&quot;<span class="s">Cannot initialize WebGPU: </span>{<span class="s">error</span>}&quot;));
            <span class="k">return</span>;
        }
    };
    post(Msg::Ready(Box::new(state)));

    <span class="k">if</span> <span class="k">let</span> Err(error) = fixture().<span class="k">await</span> {
        super::feedback::error(&amp;error);
        <span class="k">return</span>;
    }

    post(Msg::Fit);
    super::feedback::status(&quot;&quot;);
}

<span class="c">/// Only the explicit local streaming test selects the ranged source fixture.</span>
<span class="k">async</span> <span class="k">fn</span> fixture() -&gt; Result&lt;(), String&gt; {
    <span class="k">let</span> session = Session::pb_loads(include_bytes!(&quot;<span class="s">../../assets/pb/interaction.pb</span>&quot;))
        .map_err(fixture_error)?;
    post(Msg::File(FileDoc {
        name: &quot;<span class="s">Interaction fixture</span>&quot;.to_string(), <span class="c">// the scene's title</span>
        session: Rc::new(session),
        place: Xform::identity(),
        point_px: <span class="s">0</span>.<span class="s">0</span>,
        display_only: <span class="s">false</span>,
    }));
    Ok(())
}

<span class="c">/// A decode failure is reported, not hidden.</span>
<span class="k">fn</span> fixture_error(error: Box&lt;<span class="k">dyn</span> std::error::Error&gt;) -&gt; String {
    format!(&quot;<span class="s">Bundled interaction fixture: </span>{<span class="s">error</span>}&quot;)
}

<span class="c">/// Where a cloud's streaming continues.</span>
<span class="k">pub</span> <span class="k">struct</span> StreamCursor {
    <span class="k">pub</span> idx: usize, <span class="c">// the cloud's slot in the scene</span>
    <span class="k">pub</span> url: String, <span class="c">// the cloud file</span>
    <span class="k">pub</span> fields: CloudFields, <span class="c">// array positions in the file</span>
    <span class="k">pub</span> from: u32, <span class="c">// next point to read</span>
    <span class="k">pub</span> col_at: u64, <span class="c">// byte position of its colour</span>
}

<span class="c">/// Fixed display residency; F10 reads the source.</span>
<span class="k">pub</span> <span class="k">fn</span> spawn_stream_rest(_cursor: StreamCursor) {}</code></pre></div>
<p>Run <code>cargo check</code> in <code>lessons/12/</code>.</p>
<h2 id="step-15-srcenginegpupickrs">Step 15 · src/engine/gpu/pick.rs<a class="anchor" href="#/course/12-picking#step-15-srcenginegpupickrs" aria-label="Link to this section">#</a></h2>
<p>Picking reads an object and subobject ID asynchronously.</p>
<p><code>lessons/12/src/engine/gpu/pick.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! Picking draws the scene into a hidden texture where every pixel is an object number, then reads back the one pixel under the cursor.</span>
<span class="k">use</span> super::buffers::GpuCtx;
<span class="k">use</span> super::targets::{TextureSpec, texture, texture_view};
<span class="k">use</span> std::sync::Arc;
<span class="k">use</span> std::sync::atomic::{AtomicU8, Ordering};

<span class="c">/// What the pixel under the cursor holds.</span>
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
<span class="k">pub</span> <span class="k">struct</span> Pick {
    <span class="k">pub</span> row: u32,
    <span class="k">pub</span> sub: u32, <span class="c">// 0 = the object; else a tagged edge, face, dot or point id</span>
}

<span class="c">/// Textures the id pass draws into, sized to the pick window.</span>
<span class="k">struct</span> IdTargets {
    id: wgpu::Texture, <span class="c">// one object id per pixel</span>
    id_view: wgpu::TextureView,
    depth: wgpu::TextureView, <span class="c">// depth of the pick frame</span>
    gradient: wgpu::TextureView, <span class="c">// depth slope of the pick frame</span>
    size: (u32, u32), <span class="c">// texture size, px</span>
}

<span class="c">/// Default click tolerance, CSS pixels.</span>
<span class="k">pub</span> <span class="k">const</span> PICK_RADIUS: u32 = <span class="s">6</span>;

<span class="c">/// Largest tolerance, framebuffer pixels.</span>
<span class="k">const</span> MAX_RADIUS: u32 = <span class="s">128</span>;

<span class="c">/// What a pick may answer with.</span>
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
<span class="k">pub</span> <span class="k">enum</span> PickMode {
    #[default]
    Object,
    Edge,
}

<span class="c">/// The square of pixels read back around the cursor.</span>
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
<span class="k">pub</span> <span class="k">struct</span> Window {
    <span class="k">pub</span> x: u32, <span class="c">// left edge, px</span>
    <span class="k">pub</span> y: u32, <span class="c">// top edge, px</span>
    <span class="k">pub</span> w: u32, <span class="c">// width, px</span>
    <span class="k">pub</span> h: u32, <span class="c">// height, px</span>
    <span class="k">pub</span> cx: u32, <span class="c">// cursor x inside the window</span>
    <span class="k">pub</span> cy: u32, <span class="c">// cursor y inside the window</span>
    <span class="k">pub</span> radius: u32, <span class="c">// tolerance, px</span>
}

<span class="k">impl</span> Window {
    <span class="c">/// The default window around \`at\` inside a \`size\` canvas.</span>
    <span class="k">pub</span> <span class="k">fn</span> about(at: (u32, u32), size: (u32, u32)) -&gt; <span class="k">Self</span> {
        <span class="k">Self</span>::with_radius(at, size, PICK_RADIUS)
    }</code></pre></div>
<p><code>lessons/12/src/engine/gpu/pick.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// A window of \`radius\` around \`at\`, kept inside the canvas.</span>
    <span class="k">pub</span> <span class="k">fn</span> with_radius(at: (u32, u32), size: (u32, u32), radius: u32) -&gt; <span class="k">Self</span> {
        <span class="k">let</span> radius = radius.min(MAX_RADIUS);
        <span class="k">let</span> size = (size.<span class="s">0</span>.max(<span class="s">1</span>), size.<span class="s">1</span>.max(<span class="s">1</span>));
        <span class="k">let</span> (w, h) = ((<span class="s">2</span> * radius + <span class="s">1</span>).min(size.<span class="s">0</span>), (<span class="s">2</span> * radius + <span class="s">1</span>).min(size.<span class="s">1</span>));
        <span class="k">let</span> x = at.<span class="s">0</span>.saturating_sub(radius).min(size.<span class="s">0</span> - w);
        <span class="k">let</span> y = at.<span class="s">1</span>.saturating_sub(radius).min(size.<span class="s">1</span> - h);
        <span class="k">Self</span> {
            x,
            y,
            w,
            h,
            cx: at.<span class="s">0</span>.min(size.<span class="s">0</span> - <span class="s">1</span>) - x,
            cy: at.<span class="s">1</span>.min(size.<span class="s">1</span> - <span class="s">1</span>) - y,
            radius,
        }
    }
}

<span class="c">/// Bytes per row of the readback buffer, 256-aligned as wgpu requires.</span>
<span class="k">const</span> ROW_BYTES: u32 = ((<span class="s">2</span> * MAX_RADIUS + <span class="s">1</span>) * <span class="s">8</span>).div_ceil(<span class="s">256</span>) * <span class="s">256</span>;</code></pre></div>
<p><code>lessons/12/src/engine/gpu/pick.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Runs picks: request, draw, copy, map, read.</span>
<span class="k">pub</span> <span class="k">struct</span> Picker {
    pending: Option&lt;(u32, u32)&gt;, <span class="c">// cursor position waiting to be picked</span>
    inflight: bool, <span class="c">// a copy is on the GPU</span>
    window: Window, <span class="c">// window of the copy in flight</span>
    copied: bool, <span class="c">// a copy was encoded this frame, map it after submit</span>
    ready: Arc&lt;AtomicU8&gt;, <span class="c">// 0 waiting, 1 mapped, 2 failed</span>
    generation: u64, <span class="c">// bumps on every request or cancel</span>
    submitted: u64, <span class="c">// generation of the copy in flight</span>
    <span class="k">pub</span> mode: PickMode, <span class="c">// what to answer with</span>
    radius: u32, <span class="c">// tolerance, framebuffer px</span>
    readback: Option&lt;wgpu::Buffer&gt;, <span class="c">// CPU-readable copy of the window</span>
    targets: Option&lt;IdTargets&gt;,
}

<span class="c">/// A whole-frame id copy, native only.</span>
#[cfg(not(target_arch = &quot;<span class="s">wasm32</span>&quot;))]
<span class="k">pub</span>(super) <span class="k">struct</span> IdReadback {
    buffer: wgpu::Buffer, <span class="c">// CPU-readable copy</span>
    size: (u32, u32), <span class="c">// frame size, px</span>
    row_bytes: u32, <span class="c">// bytes per row, 256-aligned</span>
}

#[cfg(not(target_arch = &quot;<span class="s">wasm32</span>&quot;))]
<span class="k">impl</span> IdReadback {
    <span class="c">/// Wait for the copy and return (object, sub) per pixel.</span>
    <span class="k">pub</span>(super) <span class="k">fn</span> read(<span class="k">self</span>, ctx: &amp;GpuCtx) -&gt; Vec&lt;[u32; 2]&gt; {
        <span class="k">let</span> (send, receive) = std::sync::mpsc::sync_channel(<span class="s">1</span>);
        <span class="k">self</span>.buffer
            .slice(..)
            .map_async(wgpu::MapMode::Read, <span class="k">move</span> |result| {
                send_id_map(&amp;send, result)
            });
        ctx.device
            .poll(wgpu::PollType::Wait {
                submission_index: None,
                timeout: None,
            })
            .expect(&quot;<span class="s">ID GPU poll</span>&quot;);
        receive
            .recv()
            .expect(&quot;<span class="s">ID map callback</span>&quot;)
            .expect(&quot;<span class="s">ID buffer map</span>&quot;);
        <span class="k">let</span> bytes = <span class="k">self</span>.buffer.slice(..).get_mapped_range();
        <span class="k">let</span> <span class="k">mut</span> ids = Vec::with_capacity((<span class="k">self</span>.size.<span class="s">0</span> * <span class="k">self</span>.size.<span class="s">1</span>) <span class="k">as</span> usize);

        <span class="k">for</span> y <span class="k">in</span> <span class="s">0</span>..<span class="k">self</span>.size.<span class="s">1</span> {
            <span class="k">let</span> start = (y * <span class="k">self</span>.row_bytes) <span class="k">as</span> usize;

            <span class="k">for</span> pixel <span class="k">in</span> bytes[start..start + (<span class="k">self</span>.size.<span class="s">0</span> * <span class="s">8</span>) <span class="k">as</span> usize].chunks_exact(<span class="s">8</span>) {
                ids.push([
                    u32::from_le_bytes(pixel[..<span class="s">4</span>].try_into().expect(&quot;<span class="s">object ID bytes</span>&quot;)),
                    u32::from_le_bytes(pixel[<span class="s">4</span>..<span class="s">8</span>].try_into().expect(&quot;<span class="s">sub-object ID bytes</span>&quot;)),
                ]);
            }
        }

        drop(bytes);
        <span class="k">self</span>.buffer.unmap();
        ids
    }
}</code></pre></div>
<p><code>lessons/12/src/engine/gpu/pick.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">impl</span> Picker {
    <span class="c">/// Bytes reserved on the GPU: (buffer, textures).</span>
    <span class="k">pub</span> <span class="k">fn</span> allocated_bytes(&amp;<span class="k">self</span>) -&gt; (u64, u64) {
        <span class="k">let</span> buffer = <span class="k">match</span> &amp;<span class="k">self</span>.readback {
            Some(buffer) =&gt; buffer.size(),
            None =&gt; <span class="s">0</span>,
        };
        <span class="k">let</span> pixels = <span class="k">match</span> &amp;<span class="k">self</span>.targets {
            Some(target) =&gt; u64::from(target.size.<span class="s">0</span>) * u64::from(target.size.<span class="s">1</span>),
            None =&gt; <span class="s">0</span>,
        };
        (buffer, pixels * <span class="s">16</span>)
    }

    <span class="c">/// An idle picker with nothing allocated.</span>
    <span class="k">pub</span> <span class="k">fn</span> new() -&gt; <span class="k">Self</span> {
        <span class="k">Self</span> {
            pending: None,
            inflight: <span class="s">false</span>,
            window: Window {
                x: <span class="s">0</span>,
                y: <span class="s">0</span>,
                w: <span class="s">1</span>,
                h: <span class="s">1</span>,
                cx: <span class="s">0</span>,
                cy: <span class="s">0</span>,
                radius: PICK_RADIUS,
            },
            copied: <span class="s">false</span>,
            ready: Arc::new(AtomicU8::new(<span class="s">0</span>)),
            generation: <span class="s">0</span>,
            submitted: <span class="s">0</span>,
            mode: PickMode::Object,
            radius: PICK_RADIUS,
            readback: None,
            targets: None,
        }
    }

    <span class="c">/// Ask for a pick at canvas pixel (x, y).</span>
    <span class="k">pub</span> <span class="k">fn</span> request(&amp;<span class="k">mut</span> <span class="k">self</span>, x: u32, y: u32) {
        <span class="k">self</span>.generation = <span class="k">self</span>.generation.wrapping_add(<span class="s">1</span>);
        <span class="k">self</span>.pending = Some((x, y));
    }

    <span class="c">/// Set the mode and the tolerance in CSS pixels.</span>
    <span class="k">pub</span> <span class="k">fn</span> configure(&amp;<span class="k">mut</span> <span class="k">self</span>, mode: PickMode, radius_css: f64, scale: f64) {
        <span class="k">self</span>.mode = mode;
        <span class="k">self</span>.radius = (radius_css * scale)
            .ceil()
            .clamp(<span class="s">1</span>.<span class="s">0</span>, f64::from(MAX_RADIUS)) <span class="k">as</span> u32;
    }

    <span class="c">/// Drop the pending request and ignore any answer in flight.</span>
    <span class="k">pub</span> <span class="k">fn</span> cancel(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">self</span>.generation = <span class="k">self</span>.generation.wrapping_add(<span class="s">1</span>);
        <span class="k">self</span>.pending = None;
    }</code></pre></div>
<p><code>lessons/12/src/engine/gpu/pick.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// The readback window around \`at\` at the current tolerance.</span>
    <span class="k">pub</span> <span class="k">fn</span> window(&amp;<span class="k">self</span>, at: (u32, u32), size: (u32, u32)) -&gt; Window {
        Window::with_radius(at, size, <span class="k">self</span>.radius)
    }

    <span class="c">/// True while a pick is requested or in flight.</span>
    <span class="k">pub</span> <span class="k">fn</span> busy(&amp;<span class="k">self</span>) -&gt; bool {
        <span class="k">self</span>.inflight || <span class="k">self</span>.pending.is_some()
    }

    <span class="c">/// The request to draw this frame, unless one is in flight.</span>
    <span class="k">pub</span> <span class="k">fn</span> take_pending(&amp;<span class="k">mut</span> <span class="k">self</span>) -&gt; Option&lt;(u32, u32)&gt; {
        <span class="k">if</span> <span class="k">self</span>.inflight {
            None
        } <span class="k">else</span> {
            <span class="k">self</span>.pending.take()
        }
    }</code></pre></div>
<p><code>lessons/12/src/engine/gpu/pick.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Open the id pass over textures sized to \`view\`, cleared.</span>
    <span class="k">pub</span> <span class="k">fn</span> begin_pass&lt;'a&gt;(
        &amp;'a <span class="k">mut</span> <span class="k">self</span>,
        ctx: &amp;GpuCtx,
        encoder: &amp;'a <span class="k">mut</span> wgpu::CommandEncoder,
        size: (u32, u32),
    ) -&gt; wgpu::RenderPass&lt;'a&gt; {
        <span class="c">// remake the textures when the size changed</span>
        <span class="k">if</span> !matches!(&amp;<span class="k">self</span>.targets, Some(targets) <span class="k">if</span> targets.size == size) {
            <span class="k">let</span> usage = wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC;
            <span class="k">let</span> id = texture(
                ctx,
                &quot;<span class="s">pick.id</span>&quot;,
                &amp;TextureSpec {
                    size,
                    format: wgpu::TextureFormat::Rg32Uint,
                    samples: <span class="s">1</span>,
                    usage,
                },
            );
            <span class="k">let</span> id_view = id.create_view(&amp;wgpu::TextureViewDescriptor::default());
            <span class="k">let</span> depth = texture_view(
                ctx,
                &quot;<span class="s">pick.depth</span>&quot;,
                &amp;TextureSpec {
                    size,
                    format: wgpu::TextureFormat::Depth32Float,
                    samples: <span class="s">1</span>,
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                        | wgpu::TextureUsages::TEXTURE_BINDING,
                },
            );
            <span class="k">let</span> gradient = texture_view(
                ctx,
                &quot;<span class="s">pick.gradient</span>&quot;,
                &amp;TextureSpec {
                    size,
                    format: wgpu::TextureFormat::Rg16Float, <span class="c">// two half floats</span>
                    samples: <span class="s">1</span>,
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                        | wgpu::TextureUsages::TEXTURE_BINDING,
                },
            );
            <span class="k">self</span>.targets = Some(IdTargets {
                id,
                id_view,
                depth,
                gradient,
                size,
            });
        }

        <span class="k">let</span> t = <span class="k">self</span>.targets.as_ref().unwrap();
        encoder.begin_render_pass(&amp;wgpu::RenderPassDescriptor {
            label: Some(&quot;<span class="s">pick pass</span>&quot;),
            color_attachments: &amp;[
                Some(wgpu::RenderPassColorAttachment {
                    view: &amp;t.id_view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                }),
                Some(wgpu::RenderPassColorAttachment {
                    view: &amp;t.gradient,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                }),
            ],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &amp;t.depth,
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
    }</code></pre></div>
<p><code>lessons/12/src/engine/gpu/pick.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// The gradient texture of the id pass.</span>
    <span class="k">pub</span> <span class="k">fn</span> gradient(&amp;<span class="k">self</span>) -&gt; &amp;wgpu::TextureView {
        &amp;<span class="k">self</span>.targets.as_ref().expect(&quot;<span class="s">physical ID targets</span>&quot;).gradient
    }

    <span class="c">/// The depth texture of the id pass, once it exists.</span>
    <span class="k">pub</span> <span class="k">fn</span> depth(&amp;<span class="k">self</span>) -&gt; Option&lt;&amp;wgpu::TextureView&gt; {
        <span class="k">match</span> &amp;<span class="k">self</span>.targets {
            Some(targets) =&gt; Some(&amp;targets.depth),
            None =&gt; None,
        }
    }

    <span class="c">/// Open a second pass that adds ink ids over the same textures.</span>
    <span class="k">pub</span> <span class="k">fn</span> begin_ink&lt;'a&gt;(&amp;'a <span class="k">self</span>, encoder: &amp;'a <span class="k">mut</span> wgpu::CommandEncoder) -&gt; wgpu::RenderPass&lt;'a&gt; {
        <span class="k">let</span> targets = <span class="k">self</span>
            .targets
            .as_ref()
            .expect(&quot;<span class="s">physical ID pass initializes targets</span>&quot;);
        encoder.begin_render_pass(&amp;wgpu::RenderPassDescriptor {
            label: Some(&quot;<span class="s">pick ink</span>&quot;),
            color_attachments: &amp;[Some(wgpu::RenderPassColorAttachment {
                view: &amp;targets.id_view,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
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
<p><code>lessons/12/src/engine/gpu/pick.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Copy the window around \`at\` into the readback buffer.</span>
    <span class="k">pub</span> <span class="k">fn</span> copy_window(
        &amp;<span class="k">mut</span> <span class="k">self</span>,
        ctx: &amp;GpuCtx,
        encoder: &amp;<span class="k">mut</span> wgpu::CommandEncoder,
        at: (u32, u32),
    ) {
        <span class="k">let</span> Some(t) = &amp;<span class="k">self</span>.targets <span class="k">else</span> { <span class="k">return</span> };
        <span class="k">let</span> win = <span class="k">self</span>.window(at, t.size);

        <span class="k">if</span> <span class="k">self</span>.readback.is_none() {
            <span class="k">self</span>.readback = Some(readback_buffer(ctx));
        }

        <span class="k">let</span> buf = <span class="k">self</span>.readback.as_ref().expect(&quot;<span class="s">readback initialized above</span>&quot;);
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture: &amp;t.id,
                mip_level: <span class="s">0</span>,
                <span class="c">// window position inside the textures</span>
                origin: wgpu::Origin3d {
                    x: win.x, <span class="c">// window left, physical pixels</span>
                    y: win.y, <span class="c">// window top, physical pixels</span>
                    z: <span class="s">0</span>,
                },
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: buf,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: <span class="s">0</span>,
                    bytes_per_row: Some(ROW_BYTES),
                    rows_per_image: Some(win.h),
                },
            },
            wgpu::Extent3d {
                width: win.w,
                height: win.h,
                depth_or_array_layers: <span class="s">1</span>,
            },
        );
        <span class="k">self</span>.window = win;
        <span class="k">self</span>.submitted = <span class="k">self</span>.generation;
        <span class="k">self</span>.inflight = <span class="s">true</span>;
        <span class="k">self</span>.copied = <span class="s">true</span>;
    }</code></pre></div>
<p><code>lessons/12/src/engine/gpu/pick.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Copy the whole id texture, native only.</span>
    #[cfg(not(target_arch = &quot;<span class="s">wasm32</span>&quot;))]
    <span class="k">pub</span>(super) <span class="k">fn</span> copy_frame(
        &amp;<span class="k">self</span>,
        ctx: &amp;GpuCtx,
        encoder: &amp;<span class="k">mut</span> wgpu::CommandEncoder,
    ) -&gt; IdReadback {
        <span class="k">let</span> target = <span class="k">self</span>.targets.as_ref().expect(&quot;<span class="s">ID pass must precede capture</span>&quot;);
        <span class="k">let</span> row_bytes = (target.size.<span class="s">0</span> * <span class="s">8</span>).div_ceil(<span class="s">256</span>) * <span class="s">256</span>;
        <span class="k">let</span> buffer = ctx.device.create_buffer(&amp;wgpu::BufferDescriptor {
            label: Some(&quot;<span class="s">pick.frame.readback</span>&quot;),
            size: u64::from(row_bytes) * u64::from(target.size.<span class="s">1</span>),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: <span class="s">false</span>,
        });
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture: &amp;target.id,
                mip_level: <span class="s">0</span>,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &amp;buffer,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: <span class="s">0</span>,
                    bytes_per_row: Some(row_bytes),
                    rows_per_image: Some(target.size.<span class="s">1</span>),
                },
            },
            wgpu::Extent3d {
                width: target.size.<span class="s">0</span>,
                height: target.size.<span class="s">1</span>,
                depth_or_array_layers: <span class="s">1</span>,
            },
        );
        IdReadback {
            buffer,
            size: target.size,
            row_bytes,
        }
    }</code></pre></div>
<p><code>lessons/12/src/engine/gpu/pick.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Start mapping the readback buffer; call once after submit.</span>
    <span class="k">pub</span> <span class="k">fn</span> map(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">if</span> !<span class="k">self</span>.copied {
            <span class="k">return</span>;
        }

        <span class="k">self</span>.copied = <span class="s">false</span>;
        <span class="k">let</span> Some(buf) = &amp;<span class="k">self</span>.readback <span class="k">else</span> { <span class="k">return</span> };
        <span class="k">let</span> flag = <span class="k">self</span>.ready.clone();
        buf.slice(..)
            .map_async(wgpu::MapMode::Read, <span class="k">move</span> |result| finish_map(&amp;flag, result));
    }

    <span class="c">/// Result of the last pick: None while in flight, Some(None) for background.</span>
    <span class="k">pub</span> <span class="k">fn</span> poll(&amp;<span class="k">mut</span> <span class="k">self</span>) -&gt; Option&lt;Option&lt;Pick&gt;&gt; {
        <span class="k">let</span> status = <span class="k">self</span>.ready.load(Ordering::Acquire);

        <span class="k">if</span> !<span class="k">self</span>.inflight || status == <span class="s">0</span> {
            <span class="k">return</span> None;
        }

        <span class="k">if</span> status == <span class="s">2</span> {
            <span class="k">self</span>.ready.store(<span class="s">0</span>, Ordering::Release);
            <span class="k">self</span>.inflight = <span class="s">false</span>;
            log::warn!(&quot;<span class="s">selection readback failed; click again to retry</span>&quot;);
            <span class="k">return</span> None;
        }

        <span class="k">let</span> buf = <span class="k">self</span>.readback.as_ref()?;
        <span class="k">let</span> win = <span class="k">self</span>.window;
        <span class="k">let</span> best = {
            <span class="k">let</span> bytes = buf.slice(..).get_mapped_range();
            nearest_hit(&amp;bytes, win)
        };
        buf.unmap();
        <span class="k">self</span>.ready.store(<span class="s">0</span>, Ordering::Release);
        <span class="k">self</span>.inflight = <span class="s">false</span>;

        <span class="c">// a newer request made this answer stale</span>
        <span class="k">if</span> <span class="k">self</span>.submitted != <span class="k">self</span>.generation {
            <span class="k">return</span> None;
        }

        Some(best.map(decode_pick))
    }

    <span class="c">/// Drop the textures; the next pick remakes them.</span>
    <span class="k">pub</span> <span class="k">fn</span> resize(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">self</span>.cancel();
        <span class="k">self</span>.targets = None;
    }
}</code></pre></div>
<p><code>lessons/12/src/engine/gpu/pick.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Best pixel in the window: ink before faces, then nearest to the cursor.</span>
<span class="k">fn</span> nearest_hit(bytes: &amp;[u8], win: Window) -&gt; Option&lt;(u32, u32)&gt; {
    <span class="k">let</span> <span class="k">mut</span> best: Option&lt;(bool, u64, u32, u32)&gt; = None;

    <span class="k">for</span> y <span class="k">in</span> <span class="s">0</span>..win.h {
        <span class="k">for</span> x <span class="k">in</span> <span class="s">0</span>..win.w {
            <span class="k">let</span> at = (y * ROW_BYTES + x * <span class="s">8</span>) <span class="k">as</span> usize;
            <span class="k">let</span> object = u32::from_le_bytes(bytes[at..at + <span class="s">4</span>].try_into().unwrap());

            <span class="k">if</span> object == <span class="s">0</span> {
                <span class="k">continue</span>;
            }

            <span class="k">let</span> sub = u32::from_le_bytes(bytes[at + <span class="s">4</span>..at + <span class="s">8</span>].try_into().unwrap());
            <span class="k">let</span> (dx, dy) = (x <span class="k">as</span> i64 - win.cx <span class="k">as</span> i64, y <span class="k">as</span> i64 - win.cy <span class="k">as</span> i64);
            <span class="k">let</span> distance = (dx * dx + dy * dy) <span class="k">as</span> u64;

            <span class="c">// outside the circle</span>
            <span class="k">if</span> distance &gt; u64::from(win.radius).pow(<span class="s">2</span>) {
                <span class="k">continue</span>;
            }

            <span class="k">let</span> key = (sub == <span class="s">0</span>, distance, object, sub);

            <span class="k">match</span> best {
                Some(previous) <span class="k">if</span> key &gt;= previous =&gt; {}
                _ =&gt; best = Some(key),
            }
        }
    }

    best.map(hit_ids)
}

<span class="c">/// The CPU-readable buffer for the largest window.</span>
<span class="k">fn</span> readback_buffer(ctx: &amp;GpuCtx) -&gt; wgpu::Buffer {
    ctx.device.create_buffer(&amp;wgpu::BufferDescriptor {
        label: Some(&quot;<span class="s">pick.readback</span>&quot;),
        size: u64::from(ROW_BYTES) * u64::from(<span class="s">2</span> * MAX_RADIUS + <span class="s">1</span>),
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: <span class="s">false</span>,
    })
}

<span class="c">/// Record whether the map succeeded: 1 ok, 2 failed.</span>
<span class="k">fn</span> finish_map(flag: &amp;AtomicU8, result: Result&lt;(), wgpu::BufferAsyncError&gt;) {
    flag.store(<span class="k">if</span> result.is_ok() { <span class="s">1</span> } <span class="k">else</span> { <span class="s">2</span> }, Ordering::Release);
}

<span class="c">/// Send the map result to the waiting thread.</span>
#[cfg(not(target_arch = &quot;<span class="s">wasm32</span>&quot;))]
<span class="k">fn</span> send_id_map(
    send: &amp;std::sync::mpsc::SyncSender&lt;Result&lt;(), wgpu::BufferAsyncError&gt;&gt;,
    result: Result&lt;(), wgpu::BufferAsyncError&gt;,
) {
    send.send(result).expect(&quot;<span class="s">ID map receiver</span>&quot;);
}

<span class="c">/// Texture ids are one-based; 0 means nothing.</span>
<span class="k">fn</span> decode_pick((object, sub): (u32, u32)) -&gt; Pick {
    Pick {
        row: object - <span class="s">1</span>,
        sub: sub.saturating_sub(<span class="s">1</span>),
    }
}

<span class="c">/// Keep only the ids of the chosen pixel.</span>
<span class="k">fn</span> hit_ids((_, _, object, sub): (bool, u64, u32, u32)) -&gt; (u32, u32) {
    (object, sub)
}</code></pre></div>
<p>Copy this part from the lesson folder to the path shown.</p>
<p><code>lessons/12/src/engine/gpu/pick.rs</code> · copy the file, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>#[cfg(test)]
<span class="k">mod</span> tests {
    <span class="k">use</span> super::*;

    <span class="c">/// Build a readback buffer with the given hits.</span>
    <span class="k">fn</span> texels(win: Window, hits: &amp;[(u32, u32, u32, u32)]) -&gt; Vec&lt;u8&gt; {
        <span class="k">let</span> <span class="k">mut</span> bytes = vec![<span class="s">0u8</span>; (ROW_BYTES * win.h) <span class="k">as</span> usize];

        <span class="k">for</span> &amp;(x, y, object, sub) <span class="k">in</span> hits {
            <span class="k">let</span> at = (y * ROW_BYTES + x * <span class="s">8</span>) <span class="k">as</span> usize;
            bytes[at..at + <span class="s">4</span>].copy_from_slice(&amp;object.to_le_bytes());
            bytes[at + <span class="s">4</span>..at + <span class="s">8</span>].copy_from_slice(&amp;sub.to_le_bytes());
        }

        bytes
    }

    <span class="c">/// The window stays inside the canvas and tracks the cursor.</span>
    #[test]
    <span class="k">fn</span> window_clamps() {
        <span class="k">let</span> r = PICK_RADIUS;
        assert_eq!(
            Window::about((<span class="s">100</span>, <span class="s">100</span>), (<span class="s">400</span>, <span class="s">300</span>)),
            Window {
                x: <span class="s">100</span> - r,
                y: <span class="s">100</span> - r,
                w: <span class="s">2</span> * r + <span class="s">1</span>,
                h: <span class="s">2</span> * r + <span class="s">1</span>,
                cx: r,
                cy: r,
                radius: r
            }
        );
        assert_eq!(
            Window::about((<span class="s">0</span>, <span class="s">299</span>), (<span class="s">400</span>, <span class="s">300</span>)),
            Window {
                x: <span class="s">0</span>,
                y: <span class="s">300</span> - (<span class="s">2</span> * r + <span class="s">1</span>),
                w: <span class="s">2</span> * r + <span class="s">1</span>,
                h: <span class="s">2</span> * r + <span class="s">1</span>,
                cx: <span class="s">0</span>,
                cy: <span class="s">2</span> * r,
                radius: r
            }
        );
        assert_eq!(
            Window::about((<span class="s">5</span>, <span class="s">5</span>), (<span class="s">4</span>, <span class="s">4</span>)),
            Window {
                x: <span class="s">0</span>,
                y: <span class="s">0</span>,
                w: <span class="s">4</span>,
                h: <span class="s">4</span>,
                cx: <span class="s">3</span>,
                cy: <span class="s">3</span>,
                radius: r
            }
        );
    }

    <span class="c">/// Pick order: an edge or point wins over a face, then the nearest wins.</span>
    #[test]
    <span class="k">fn</span> ink_first_then_nearest() {
        <span class="k">let</span> win = Window::about((<span class="s">50</span>, <span class="s">50</span>), (<span class="s">200</span>, <span class="s">200</span>));
        <span class="k">let</span> (c, seg) = (PICK_RADIUS, <span class="s">0x8000_0000</span> | <span class="s">3</span>);
        assert_eq!(
            nearest_hit(&amp;texels(win, &amp;[(c, c, <span class="s">7</span>, <span class="s">0</span>), (c + <span class="s">5</span>, c, <span class="s">9</span>, seg)]), win),
            Some((<span class="s">9</span>, seg))
        );
        assert_eq!(
            nearest_hit(
                &amp;texels(win, &amp;[(c + <span class="s">5</span>, c, <span class="s">9</span>, seg), (c - <span class="s">2</span>, c + <span class="s">1</span>, <span class="s">4</span>, <span class="s">12</span>)]),
                win
            ),
            Some((<span class="s">4</span>, <span class="s">12</span>))
        );
        assert_eq!(
            nearest_hit(&amp;texels(win, &amp;[(c + <span class="s">3</span>, c, <span class="s">7</span>, <span class="s">0</span>), (c - <span class="s">1</span>, c, <span class="s">2</span>, <span class="s">0</span>)]), win),
            Some((<span class="s">2</span>, <span class="s">0</span>))
        );
        assert_eq!(nearest_hit(&amp;texels(win, &amp;[]), win), None);
    }
}</code></pre></div>
<h2 id="step-16-srcenginegpurenderrs">Step 16 · src/engine/gpu/render.rs<a class="anchor" href="#/course/12-picking#step-16-srcenginegpurenderrs" aria-label="Link to this section">#</a></h2>
<p>The frame encoder orders face, ink, picking and overlay passes.</p>
<p><code>lessons/12/src/engine/gpu/render.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Draw object ids around the cursor for a pick, then copy them out.</span>
    <span class="k">pub</span>(super) <span class="k">fn</span> id_pass(&amp;<span class="k">mut</span> <span class="k">self</span>, encoder: &amp;<span class="k">mut</span> wgpu::CommandEncoder, at: Option&lt;(u32, u32)&gt;) {
        <span class="k">let</span> size = (<span class="k">self</span>.config.width, <span class="k">self</span>.config.height);
        <span class="k">let</span> mode = <span class="k">self</span>.pick.mode;
        <span class="k">let</span> window = <span class="k">match</span> at {
            Some(position) =&gt; Some(<span class="k">self</span>.pick.window(position, size)),
            None =&gt; None,
        };
        <span class="k">let</span> basic = Binds {
            mvp: &amp;<span class="k">self</span>.frame.mvp_group, <span class="c">// the camera matrix</span>
            line: &amp;<span class="k">self</span>.frame.line_group, <span class="c">// pen settings</span>
            instances: &amp;<span class="k">self</span>.objects.group,
        };
        {
            <span class="k">let</span> <span class="k">mut</span> pass = <span class="k">self</span>.pick.begin_pass(&amp;<span class="k">self</span>.ctx, encoder, size);

            <span class="c">// Plane reconstruction also reads neighboring texels: render a small halo around</span>
            <span class="k">if</span> <span class="k">let</span> Some(window) = window {
                <span class="k">let</span> left = window.x.saturating_sub(<span class="s">3</span>);
                <span class="k">let</span> top = window.y.saturating_sub(<span class="s">3</span>);
                <span class="k">let</span> right = (window.x + window.w + <span class="s">3</span>).min(size.<span class="s">0</span>);
                <span class="k">let</span> bottom = (window.y + window.h + <span class="s">3</span>).min(size.<span class="s">1</span>);
                pass.set_scissor_rect(left, top, right - left, bottom - top);
            }

            <span class="k">self</span>.arena.draw_face_ids(&amp;<span class="k">mut</span> pass, &amp;basic);
            <span class="k">self</span>.splat.draw_ids(&amp;<span class="k">mut</span> pass, &amp;<span class="k">self</span>.frame.cloud_group);
        }
        <span class="c">// ink ids test against the depth just drawn</span>
        <span class="k">let</span> depth = <span class="k">self</span>.pick.depth().expect(&quot;<span class="s">physical ID pass creates depth</span>&quot;);
        <span class="k">let</span> group = <span class="k">self</span>.objects.pick_group(
            &amp;<span class="k">self</span>.ctx,
            &amp;<span class="k">self</span>.layouts,
            [depth, &amp;<span class="k">self</span>.targets.depth_msaa],
            [<span class="k">self</span>.pick.gradient(), &amp;<span class="k">self</span>.targets.gradient_msaa],
        );
        <span class="k">let</span> ink = Binds {
            mvp: &amp;<span class="k">self</span>.frame.mvp_group, <span class="c">// the camera matrix</span>
            line: &amp;<span class="k">self</span>.frame.line_group, <span class="c">// pen settings</span>
            instances: &amp;group, <span class="c">// per-object rows</span>
        };
        {
            <span class="k">let</span> <span class="k">mut</span> pass = <span class="k">self</span>.pick.begin_ink(encoder);

            <span class="k">if</span> <span class="k">let</span> Some(window) = window {
                pass.set_scissor_rect(window.x, window.y, window.w, window.h);
            }

            <span class="c">// which ink may answer depends on the mode</span>
            <span class="k">match</span> mode {
                PickMode::Edge =&gt; {
                    <span class="k">if</span> <span class="k">self</span>.view.show_mesh_edges {
                        <span class="k">self</span>.segments.draw_edge_ids(&amp;<span class="k">mut</span> pass, &amp;ink);
                    }
                }
                PickMode::Object =&gt; {
                    <span class="k">if</span> <span class="k">self</span>.view.show_mesh_edges {
                        <span class="k">self</span>.segments.draw_pipe_ids(&amp;<span class="k">mut</span> pass, &amp;ink);
                    }

                    <span class="k">if</span> <span class="k">self</span>.view.show_lines {
                        <span class="k">self</span>.segments.draw_ribbon_ids(&amp;<span class="k">mut</span> pass, &amp;ink);
                    }

                    <span class="k">if</span> <span class="k">self</span>.view.show_mesh_edges &amp;&amp; <span class="k">self</span>.view.markers {
                        <span class="k">self</span>.glyphs.draw_sphere_ids(&amp;<span class="k">mut</span> pass, &amp;ink);
                    }

                    <span class="k">self</span>.arena.draw_text_ids(&amp;<span class="k">mut</span> pass, &amp;basic);

                    <span class="k">if</span> <span class="k">self</span>.view.show_points {
                        <span class="k">self</span>.glyphs.draw_dot_ids(&amp;<span class="k">mut</span> pass, &amp;ink);
                    }
                }
            }
        }

        <span class="k">if</span> <span class="k">let</span> Some(at) = at {
            <span class="k">self</span>.pick.copy_window(&amp;<span class="k">self</span>.ctx, encoder, at);
        }
    }
}</code></pre></div>
<h2 id="step-17-srcenginegpuselection_outliners">Step 17 · src/engine/gpu/selection_outline.rs<a class="anchor" href="#/course/12-picking#step-17-srcenginegpuselection_outliners" aria-label="Link to this section">#</a></h2>
<p>A selection mask draws a border around visible selected geometry.</p>
<p><code>lessons/12/src/engine/gpu/selection_outline.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! Draws a border around the selected object by redrawing it slightly larger behind itself.</span>
<span class="k">use</span> std::collections::HashSet;

<span class="k">use</span> super::buffers::GpuCtx;
<span class="k">use</span> super::targets::{Targets, TextureSpec, texture_view};
<span class="k">use</span> <span class="k">crate</span>::engine::pipelines::{ColorWrite, DepthMode, PipelineDesc, Target, build, module};

<span class="k">struct</span> Mask {
    resolved: wgpu::TextureView, <span class="c">// coverage at 1x</span>
    multisampled: Option&lt;wgpu::TextureView&gt;, <span class="c">// MSAA coverage, if on</span>
    group: wgpu::BindGroup,
    size: (u32, u32),
    samples: u32,
}

<span class="c">/// Full-frame coverage mask, allocated only while a selection exists.</span>
<span class="k">pub</span> <span class="k">struct</span> SelectionOutline {
    selected: HashSet&lt;u32&gt;,
    layout: wgpu::BindGroupLayout,
    uniform: wgpu::Buffer,
    pipeline: wgpu::RenderPipeline,
    mask: Option&lt;Mask&gt;,
}

<span class="k">impl</span> SelectionOutline {
    <span class="c">/// Create the mask layout and compositor, no textures yet.</span>
    <span class="k">pub</span> <span class="k">fn</span> new(ctx: &amp;GpuCtx, target: Target) -&gt; <span class="k">Self</span> {
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
                ],
            });
        <span class="k">let</span> uniform = ctx.device.create_buffer(&amp;wgpu::BufferDescriptor {
            label: Some(&quot;<span class="s">selection outline radius</span>&quot;),
            size: <span class="s">16</span>,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: <span class="s">false</span>,
        });
        <span class="k">let</span> pipeline = pipeline(ctx, &amp;layout, target);
        <span class="k">Self</span> {
            selected: HashSet::new(),
            layout,
            uniform,
            pipeline,
            mask: None,
        }
    }</code></pre></div>
<p><code>lessons/12/src/engine/gpu/selection_outline.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Selected rows; none selected frees coverage.</span>
    <span class="k">pub</span> <span class="k">fn</span> set_selected(&amp;<span class="k">mut</span> <span class="k">self</span>, row: u32, selected: bool) {
        <span class="k">if</span> selected {
            <span class="k">self</span>.selected.insert(row);
        } <span class="k">else</span> {
            <span class="k">self</span>.selected.remove(&amp;row);
        }

        <span class="k">if</span> <span class="k">self</span>.selected.is_empty() {
            <span class="k">self</span>.mask = None;
        }
    }

    <span class="c">/// Clear source selection and its size-dependent coverage resources.</span>
    <span class="k">pub</span> <span class="k">fn</span> reset(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">self</span>.selected.clear();
        <span class="k">self</span>.mask = None;
    }

    <span class="c">/// Rebuild the compositor for this sample count.</span>
    <span class="k">pub</span> <span class="k">fn</span> retarget(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, target: Target) {
        <span class="k">self</span>.pipeline = pipeline(ctx, &amp;<span class="k">self</span>.layout, target);
        <span class="k">self</span>.mask = None;
    }</code></pre></div>
<p><code>lessons/12/src/engine/gpu/selection_outline.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Return whether to render selected surface coverage this frame.</span>
    <span class="k">pub</span> <span class="k">fn</span> prepare(
        &amp;<span class="k">mut</span> <span class="k">self</span>,
        ctx: &amp;GpuCtx,
        size: (u32, u32),
        samples: u32,
        css_width: f64,
        faces: bool,
    ) -&gt; bool {
        <span class="k">if</span> <span class="k">self</span>.selected.is_empty() || !faces {
            <span class="k">self</span>.mask = None;
            <span class="k">return</span> <span class="s">false</span>;
        }

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
                ],
            });
            <span class="k">self</span>.mask = Some(Mask {
                resolved,
                multisampled,
                group,
                size,
                samples,
            });
        }

        <span class="k">let</span> radius = (<span class="s">1</span>.<span class="s">5</span> * f64::from(size.<span class="s">0</span>) / css_width.max(<span class="s">1</span>.<span class="s">0</span>)).clamp(<span class="s">1</span>.<span class="s">0</span>, <span class="s">8</span>.<span class="s">0</span>) <span class="k">as</span> f32;
        ctx.queue.write_buffer(
            &amp;<span class="k">self</span>.uniform,
            <span class="s">0</span>,
            bytemuck::cast_slice(&amp;[radius, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>]),
        );
        <span class="s">true</span>
    }</code></pre></div>
<p><code>lessons/12/src/engine/gpu/selection_outline.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Clear coverage, depth-tested against the frame.</span>
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
    }

    <span class="c">/// Composite the outside border before control markers and labels.</span>
    <span class="k">pub</span> <span class="k">fn</span> draw(&amp;<span class="k">self</span>, pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;) -&gt; u32 {
        <span class="k">let</span> Some(mask) = &amp;<span class="k">self</span>.mask <span class="k">else</span> {
            <span class="k">return</span> <span class="s">0</span>;
        };
        pass.set_pipeline(&amp;<span class="k">self</span>.pipeline);
        pass.set_bind_group(<span class="s">0</span>, &amp;mask.group, &amp;[]);
        pass.draw(<span class="s">0</span>..<span class="s">3</span>, <span class="s">0</span>..<span class="s">1</span>);
        <span class="s">1</span>
    }

    <span class="c">/// Exact owned buffer and texture payload, excluding driver allocation overhead.</span>
    <span class="k">pub</span> <span class="k">fn</span> allocated_bytes(&amp;<span class="k">self</span>) -&gt; (u64, u64) {
        <span class="k">let</span> textures = <span class="k">match</span> &amp;<span class="k">self</span>.mask {
            Some(mask) =&gt; {
                <span class="k">let</span> samples = <span class="k">if</span> mask.samples &gt; <span class="s">1</span> {
                    u64::from(mask.samples) + <span class="s">1</span>
                } <span class="k">else</span> {
                    <span class="s">1</span>
                };
                u64::from(mask.size.<span class="s">0</span>) * u64::from(mask.size.<span class="s">1</span>) * samples
            }
            None =&gt; <span class="s">0</span>,
        };
        (<span class="k">self</span>.uniform.size(), textures)
    }
}</code></pre></div>
<p><code>lessons/12/src/engine/gpu/selection_outline.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Build the outline compositor for this pass format.</span>
<span class="k">fn</span> pipeline(ctx: &amp;GpuCtx, layout: &amp;wgpu::BindGroupLayout, target: Target) -&gt; wgpu::RenderPipeline {
    <span class="k">let</span> shader = module(
        &amp;ctx.device,
        &quot;<span class="s">selection outline</span>&quot;,
        include_str!(&quot;<span class="s">../../shaders/selection_outline.wgsl</span>&quot;),
    );
    <span class="k">let</span> groups = [layout];
    <span class="k">let</span> desc = PipelineDesc::new(&amp;shader, &amp;groups, &amp;[], wgpu::PrimitiveTopology::TriangleList)
        .with(&quot;<span class="s">black selection outline</span>&quot;, &quot;<span class="s">fs_main</span>&quot;)
        .depth(DepthMode::Always)
        .color(ColorWrite::Blended);
    build(&amp;ctx.device, target, &amp;desc)
}</code></pre></div>
<p>Copy this part from the lesson folder to the path shown.</p>
<p><code>lessons/12/src/engine/gpu/selection_outline.rs</code> · copy the file, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>#[cfg(all(test, not(target_arch = &quot;<span class="s">wasm32</span>&quot;)))]
<span class="k">mod</span> tests {
    <span class="k">use</span> <span class="k">crate</span>::engine::gpu::{FrameInput, Gpu, ObjectRow, Upload};
    <span class="k">use</span> session_rust::{RenderVertex, Xform};

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
            assert_eq!(capacity, <span class="s">200</span> * <span class="s">200</span> * <span class="k">if</span> samples &gt; <span class="s">1</span> { <span class="s">5</span> } <span class="k">else</span> { <span class="s">1</span> });
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
            assert!(
                plain
                    .chunks_exact(<span class="s">4</span>)
                    .all(|p| p[<span class="s">0</span>] &gt; <span class="s">8</span> || p[<span class="s">1</span>] &gt; <span class="s">8</span> || p[<span class="s">2</span>] &gt; <span class="s">8</span>)
            );
            gpu.set_hidden(<span class="s">1</span>, <span class="s">false</span>);
        }

        gpu.release();
        assert_eq!(gpu.selection_outline.allocated_bytes(), (<span class="s">16</span>, <span class="s">0</span>));
    }
}</code></pre></div>
<h2 id="step-18-srcshadersselection_outlinewgsl">Step 18 · src/shaders/selection_outline.wgsl<a class="anchor" href="#/course/12-picking#step-18-srcshadersselection_outlinewgsl" aria-label="Link to this section">#</a></h2>
<p>The selection outline expands the selected coverage mask.</p>
<p><code>lessons/12/src/shaders/selection_outline.wgsl</code> · 39 lines · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>@group(<span class="s">0</span>) @binding(<span class="s">0</span>) <span class="k">var</span> mask: texture_2d&lt;<span class="k">f32</span>&gt;;
@group(<span class="s">0</span>) @binding(<span class="s">1</span>) <span class="k">var</span>&lt;<span class="k">uniform</span>&gt; radius: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;;

@vertex
<span class="k">fn</span> vs_main(@builtin(vertex_index) i: <span class="k">u32</span>) -&gt; @builtin(position) <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt; {
    <span class="k">let</span> xy = <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(f32((i &lt;&lt; <span class="s">1u</span>) &amp; <span class="s">2u</span>), f32(i &amp; <span class="s">2u</span>));
    <span class="k">return</span> <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(xy * <span class="s">2.0</span> - <span class="s">1.0</span>, <span class="s">0.0</span>, <span class="s">1.0</span>);
}

@fragment
<span class="k">fn</span> fs_main(@builtin(position) position: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;) -&gt; @location(<span class="s">0</span>) <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt; {
    <span class="k">let</span> size = <span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;(textureDimensions(mask));
    <span class="k">let</span> p = <span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;(position.xy);
    <span class="k">let</span> center = textureLoad(mask, p, <span class="s">0</span>).r;

    <span class="k">if</span> (center &gt;= <span class="s">0.999</span>) {
        <span class="k">discard</span>;
    }

    <span class="k">let</span> r = radius.x;
    <span class="k">let</span> extent = i32(ceil(r + <span class="s">0.5</span>));
    <span class="k">var</span> coverage = <span class="s">0.0</span>;

    <span class="k">for</span> (<span class="k">var</span> y = -extent; y &lt;= extent; y++) {
        <span class="k">for</span> (<span class="k">var</span> x = -extent; x &lt;= extent; x++) {
            <span class="k">let</span> q = p + <span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;(x, y);

            <span class="k">if</span> (any(q &lt; <span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;(<span class="s">0</span>)) || any(q &gt;= size)) {
                <span class="k">continue</span>;
            }

            <span class="k">let</span> distance = length(<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(f32(x), f32(y)));
            <span class="k">let</span> weight = <span class="s">1.0</span> - smoothstep(r - <span class="s">0.5</span>, r + <span class="s">0.5</span>, distance);
            coverage = max(coverage, textureLoad(mask, q, <span class="s">0</span>).r * weight);
        }
    }

    <span class="k">return</span> <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>, <span class="s">0.0</span>, <span class="s">0.0</span>, coverage * (<span class="s">1.0</span> - center));
}</code></pre></div>
<p>Run <code>cargo check</code> in <code>lessons/12/</code>.</p>
<h2 id="step-19-srcstaters">Step 19 · src/state.rs<a class="anchor" href="#/course/12-picking#step-19-srcstaters" aria-label="Link to this section">#</a></h2>
<p>State coordinates input, selection and frame requests.</p>
<p><code>lessons/12/src/state.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! One place for what the viewer knows between frames, so no pass has to reach into another pass's data.</span>
<span class="k">use</span> <span class="k">crate</span>::app::scene::{FileDoc, Scene, StreamedInit};
<span class="k">use</span> <span class="k">crate</span>::app::selection::SelectionMode;
<span class="k">use</span> <span class="k">crate</span>::app::walk::cloud::StreamRows;
<span class="k">use</span> <span class="k">crate</span>::camera::Camera;
<span class="k">use</span> <span class="k">crate</span>::engine::gpu::pick::PickMode;
<span class="k">use</span> <span class="k">crate</span>::engine::gpu::{FrameInput, Gpu, Pick};
<span class="k">use</span> <span class="k">crate</span>::engine::performance::{heap_mb, now_ms};
<span class="k">use</span> <span class="k">crate</span>::engine::text::{TextLabel, TextPlacement};
<span class="k">use</span> session_rust::AABB;
<span class="k">use</span> std::sync::Arc;
<span class="k">use</span> winit::window::Window;

<span class="c">/// Background color.</span>
<span class="k">const</span> CLEAR: wgpu::Color = wgpu::Color {
    r: <span class="s">0</span>.<span class="s">9</span>,
    g: <span class="s">0</span>.<span class="s">9</span>,
    b: <span class="s">0</span>.<span class="s">9</span>,
    a: <span class="s">1</span>.<span class="s">0</span>,
};

<span class="c">/// Orbit step per frame in \`?spin=1\` mode.</span>
<span class="k">const</span> SPIN_STEP: f32 = <span class="s">0</span>.<span class="s">004</span>;

<span class="c">/// Everything the viewer holds: window, GPU, camera, scene, selection.</span>
<span class="k">pub</span> <span class="k">struct</span> State {
    <span class="k">pub</span> window: Arc&lt;Window&gt;, <span class="c">// the winit window on the canvas</span>
    <span class="k">pub</span> gpu: Gpu, <span class="c">// device, buffers, pipelines</span>
    <span class="k">pub</span> camera: Camera, <span class="c">// the view</span>
    <span class="k">pub</span> scene: Scene, <span class="c">// the loaded documents</span>
    <span class="k">pub</span> needs_frame: bool, <span class="c">// draw again on the next redraw</span>
    dirty: bool, <span class="c">// the picture changed</span>
    last_frame_ms: f64,
    <span class="k">pub</span> selection: SelectionMode, <span class="c">// object, edge, face or control points</span>
    requested: PickMode, <span class="c">// what the pending pick looks for</span>
    <span class="k">pub</span> selection_radius_css: f64, <span class="c">// click tolerance in CSS pixels</span>
    scene_labels: Vec&lt;TextLabel&gt;,
    show_selected_names: bool, <span class="c">// name label on the selection, T toggles</span>
}</code></pre></div>
<p><code>lessons/12/src/state.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">impl</span> State {
    <span class="c">/// Open the GPU and upload the scene.</span>
    <span class="k">pub</span> <span class="k">async</span> <span class="k">fn</span> new(window: Arc&lt;Window&gt;, <span class="k">mut</span> scene: Scene) -&gt; anyhow::Result&lt;<span class="k">Self</span>&gt; {
        <span class="k">let</span> t0 = now_ms();
        <span class="k">let</span> <span class="k">mut</span> gpu = Gpu::new(window.clone()).<span class="k">await</span>?;
        <span class="c">// the scene rows go to the GPU once</span>
        scene.upload_to(&amp;<span class="k">mut</span> gpu);
        log::info!(&quot;<span class="s">gpu init </span>{<span class="s">:.0</span>}<span class="s"> ms</span>&quot;, now_ms() - t0);
        Ok(<span class="k">Self</span> {
            window,
            gpu,
            camera: Camera::new(), <span class="c">// default view</span>
            scene,
            needs_frame: <span class="s">true</span>,
            dirty: <span class="s">true</span>,
            last_frame_ms: <span class="s">0</span>.<span class="s">0</span>,
            selection: SelectionMode::Object,
            requested: PickMode::Object,
            selection_radius_css: <span class="s">6</span>.<span class="s">0</span>,
            scene_labels: Vec::new(), <span class="c">// no text yet</span>
            show_selected_names: <span class="s">true</span>,
        })
    }

    <span class="c">/// Width over height of the canvas.</span>
    <span class="k">pub</span> <span class="k">fn</span> aspect(&amp;<span class="k">self</span>) -&gt; f64 {
        <span class="k">self</span>.gpu.config.width.max(<span class="s">1</span>) <span class="k">as</span> f64 / <span class="k">self</span>.gpu.config.height.max(<span class="s">1</span>) <span class="k">as</span> f64
    }

    <span class="c">/// Canvas size in device pixels.</span>
    <span class="k">pub</span> <span class="k">fn</span> viewport(&amp;<span class="k">self</span>) -&gt; (f64, f64) {
        (<span class="k">self</span>.gpu.config.width <span class="k">as</span> f64, <span class="k">self</span>.gpu.config.height <span class="k">as</span> f64)
    }

    <span class="c">/// Add one loaded document to the scene.</span>
    <span class="k">pub</span> <span class="k">fn</span> append(&amp;<span class="k">mut</span> <span class="k">self</span>, doc: FileDoc) {
        <span class="k">let</span> t0 = now_ms();
        <span class="k">let</span> first_row = <span class="k">self</span>.scene.object_count(); <span class="c">// rows before this document</span>
        <span class="k">self</span>.scene.add_file(doc);
        <span class="k">let</span> t1 = now_ms();
        <span class="c">// only the new rows go to the GPU</span>
        <span class="k">self</span>.scene.upload_to(&amp;<span class="k">mut</span> <span class="k">self</span>.gpu);
        <span class="k">self</span>.camera.grow_extent(&amp;<span class="k">self</span>.gpu.bounds);
        <span class="k">self</span>.annotate_document(first_row);
        <span class="k">self</span>.update_label();
        log::info!(
            &quot;<span class="s">appended: walk </span>{<span class="s">:.0</span>}<span class="s"> ms, upload </span>{<span class="s">:.0</span>}<span class="s"> ms | </span>{}<span class="s"> docs | memory observation </span>{<span class="s">:.0</span>}<span class="s"> MiB</span>&quot;,
            t1 - t0,
            now_ms() - t1,
            <span class="k">self</span>.scene.docs.len(),
            heap_mb()
        );
        <span class="k">self</span>.touch();
    }

    <span class="c">/// Start a streamed point cloud; returns its slot.</span>
    <span class="k">pub</span> <span class="k">fn</span> add_streamed(&amp;<span class="k">mut</span> <span class="k">self</span>, init: StreamedInit) -&gt; usize {
        <span class="k">let</span> idx = <span class="k">self</span>.scene.add_streamed_cloud(init, &amp;<span class="k">mut</span> <span class="k">self</span>.gpu);
        <span class="k">self</span>.camera.grow_extent(&amp;<span class="k">self</span>.gpu.bounds);
        <span class="k">self</span>.touch();
        idx
    }</code></pre></div>
<p><code>lessons/12/src/state.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Add more points to streamed cloud \`idx\`.</span>
    <span class="k">pub</span> <span class="k">fn</span> extend_streamed(&amp;<span class="k">mut</span> <span class="k">self</span>, idx: usize, rows: StreamRows, to: u32) {
        <span class="k">self</span>.scene
            .extend_streamed_cloud(idx, rows, to, &amp;<span class="k">mut</span> <span class="k">self</span>.gpu);
        <span class="k">self</span>.camera.grow_extent(&amp;<span class="k">self</span>.gpu.bounds);
        log::info!(
            &quot;<span class="s">cloud slice: </span>{<span class="s">to</span>}<span class="s"> points resident | heap </span>{<span class="s">:.0</span>}<span class="s"> MB</span>&quot;,
            heap_mb()
        );
        <span class="k">self</span>.touch();
    }

    <span class="c">/// Remove every document; camera and GPU stay.</span>
    <span class="k">pub</span> <span class="k">fn</span> clear(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">self</span>.selection = SelectionMode::Object;
        <span class="k">self</span>.scene_labels.clear();
        <span class="k">self</span>.scene.clear(&amp;<span class="k">mut</span> <span class="k">self</span>.gpu);
        <span class="k">self</span>.touch();
    }

    <span class="c">/// Fit the camera to everything loaded.</span>
    <span class="k">pub</span> <span class="k">fn</span> fit_all(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">let</span> b = &amp;<span class="k">self</span>.gpu.bounds;
        log::info!(&quot;<span class="s">fit: bounds </span>{}<span class="s"> aspect </span>{<span class="s">:.3</span>}&quot;, b.str(), <span class="k">self</span>.aspect());
        <span class="k">self</span>.camera.fit(&amp;<span class="k">self</span>.gpu.bounds, <span class="k">self</span>.aspect());
        <span class="k">self</span>.touch();
    }

    <span class="c">/// Fit the camera to the selection, or to everything.</span>
    <span class="k">pub</span> <span class="k">fn</span> fit_selected_or_all(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">let</span> row = <span class="k">match</span> <span class="k">self</span>.scene.selected {
            Some(row) =&gt; <span class="k">self</span>.gpu.objects.row_bounds(row),
            None =&gt; None,
        };
        <span class="k">let</span> Some(b) = row <span class="k">else</span> {
            <span class="k">self</span>.fit_all();
            <span class="k">return</span>;
        };
        log::info!(
            &quot;<span class="s">fit selected: bounds </span>{}<span class="s"> aspect </span>{<span class="s">:.3</span>}&quot;,
            b.str(),
            <span class="k">self</span>.aspect()
        );
        <span class="k">self</span>.camera.fit(&amp;b, <span class="k">self</span>.aspect());
        <span class="k">self</span>.camera.grow_extent(&amp;<span class="k">self</span>.gpu.bounds);
        <span class="k">self</span>.touch();
    }</code></pre></div>
<p><code>lessons/12/src/state.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Forward a canvas resize to the GPU layer.</span>
    <span class="k">pub</span> <span class="k">fn</span> resize(&amp;<span class="k">mut</span> <span class="k">self</span>, width: u32, height: u32) {
        <span class="k">self</span>.gpu.resize(width, height);
        <span class="k">self</span>.gpu.logical_size = <span class="k">self</span>.logical_size();
        <span class="k">self</span>.touch();
    }

    <span class="c">/// Set the point size of clouds.</span>
    <span class="k">pub</span> <span class="k">fn</span> set_cloud_size(&amp;<span class="k">mut</span> <span class="k">self</span>, size: f32) {
        <span class="k">self</span>.gpu.view.cloud_size = size.clamp(<span class="s">0</span>.<span class="s">25</span>, <span class="s">8</span>.<span class="s">0</span>);
        <span class="k">self</span>.touch();
    }

    <span class="c">/// P: switch between shaded faces and x-ray.</span>
    <span class="k">pub</span> <span class="k">fn</span> toggle_xray(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">self</span>.gpu.view.opacity = <span class="k">if</span> <span class="k">self</span>.gpu.view.opacity &gt; <span class="s">0</span>.<span class="s">0</span> {
            <span class="s">0</span>.<span class="s">0</span>
        } <span class="k">else</span> {
            <span class="s">1</span>.<span class="s">0</span>
        };
        <span class="k">self</span>.touch();
    }

    <span class="c">/// Ask what is under a pixel; the answer comes in a later frame.</span>
    <span class="k">pub</span> <span class="k">fn</span> request_pick(&amp;<span class="k">mut</span> <span class="k">self</span>, x: u32, y: u32) {
        <span class="k">self</span>.request_selection(x, y, <span class="s">false</span>);</code></pre></div>
<p><code>lessons/12/src/state.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    }

    <span class="c">/// Something changed: drop pending picks, draw again.</span>
    <span class="k">pub</span> <span class="k">fn</span> touch(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">self</span>.gpu.pick.cancel();
        <span class="k">self</span>.dirty = <span class="s">true</span>;
        <span class="k">self</span>.needs_frame = <span class="s">true</span>;
    }

    <span class="c">/// Select one row, or nothing.</span>
    <span class="k">pub</span> <span class="k">fn</span> select(&amp;<span class="k">mut</span> <span class="k">self</span>, row: Option&lt;u32&gt;) {
        <span class="k">self</span>.selection = SelectionMode::Object;
        <span class="k">self</span>.gpu.controls.reset();
        <span class="k">self</span>.gpu.control_net.reset();
        <span class="k">self</span>.gpu.segments.set_edge(&amp;<span class="k">self</span>.gpu.ctx, None);
        <span class="k">self</span>.gpu.splat.set_controls(None);

        <span class="k">if</span> <span class="k">let</span> Some(old) = <span class="k">self</span>.scene.selected.take() {
            <span class="k">self</span>.gpu.set_selected(old, <span class="s">false</span>);
        }</code></pre></div>
<p><code>lessons/12/src/state.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">if</span> <span class="k">let</span> Some(r) = row {
            <span class="k">self</span>.gpu.set_selected(r, <span class="s">true</span>);
        }

        <span class="k">self</span>.scene.selected = row;
        <span class="k">self</span>.update_label();
        <span class="k">self</span>.touch();
    }

    <span class="c">/// H: hide the selection.</span>
    <span class="k">pub</span> <span class="k">fn</span> hide_selected(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">let</span> Some(row) = <span class="k">self</span>.scene.selected <span class="k">else</span> {
            <span class="k">return</span>;
        };
        <span class="k">let</span> Some(guid) = <span class="k">self</span>.scene.identity_of(row) <span class="k">else</span> {
            <span class="k">return</span>;
        };
        <span class="k">self</span>.select(None);
        <span class="k">self</span>.scene.hidden.insert(guid); <span class="c">// by id, so a rebuild keeps it hidden</span>
        <span class="k">self</span>.gpu.set_hidden(row, <span class="s">true</span>);
        <span class="k">self</span>.touch();
    }

    <span class="c">/// S: show everything hidden.</span>
    <span class="k">pub</span> <span class="k">fn</span> show_all(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">for</span> row <span class="k">in</span> <span class="k">self</span>.scene.hidden_rows() {
            <span class="k">self</span>.gpu.set_hidden(row, <span class="s">false</span>);
        }

        <span class="k">self</span>.scene.hidden.clear();
        <span class="k">self</span>.touch();
    }

    <span class="c">/// A pick answer arrived: select what it hit.</span>
    <span class="k">fn</span> apply_pick(&amp;<span class="k">mut</span> <span class="k">self</span>, pick: Option&lt;Pick&gt;) {
        <span class="k">match</span> <span class="k">self</span>.requested {
            PickMode::Edge =&gt; {
                <span class="c">// an edge hit wins over a face hit</span>
                <span class="k">if</span> <span class="k">let</span> Some(pick) = pick
                    &amp;&amp; <span class="k">let</span> Some(edge) = <span class="k">self</span>.scene.edge_at(pick)
                {
                    <span class="k">self</span>.select(Some(pick.row));
                    <span class="k">self</span>.gpu.set_selected(pick.row, <span class="s">false</span>);
                    <span class="k">self</span>.selection.select_edge(pick.row, edge);
                    <span class="k">self</span>.gpu
                        .segments
                        .set_edge(&amp;<span class="k">self</span>.gpu.ctx, Some((pick.row, edge)));
                    <span class="k">self</span>.status(&amp;format!(&quot;<span class="s">Edge </span>{<span class="s">edge</span>}<span class="s"> selected</span>&quot;));
                }

                <span class="k">return</span>;
            }
            PickMode::Object =&gt; {}
        }

        <span class="c">// nothing hit: clear, unless Shift is adding</span>
        <span class="k">let</span> Some(p) = pick <span class="k">else</span> {
            log::info!(&quot;<span class="s">pick: nothing</span>&quot;);
            <span class="k">self</span>.select(None);</code></pre></div>
<p><code>lessons/12/src/state.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">return</span>;
        };

        <span class="k">match</span> <span class="k">self</span>.scene.resolve(p, &amp;<span class="k">self</span>.gpu) {
            Some(hit) =&gt; {
                <span class="k">match</span> &amp;hit.point {
                    Some(pt) =&gt; log::info!(
                        &quot;<span class="s">pick: '</span>{}<span class="s">' </span>{}<span class="s"> row </span>{}<span class="s"> point </span>{}<span class="s"> id </span>{}<span class="s"> at (</span>{<span class="s">:.1</span>}<span class="s">, </span>{<span class="s">:.1</span>}<span class="s">, </span>{<span class="s">:.1</span>}<span class="s">)</span>&quot;,
                        hit.doc,
                        hit.guid,
                        hit.row,
                        pt.local,
                        pt.id,
                        pt.position[<span class="s">0</span>],
                        pt.position[<span class="s">1</span>],
                        pt.position[<span class="s">2</span>]
                    ),
                    None =&gt; log::info!(&quot;<span class="s">pick: '</span>{}<span class="s">' </span>{}<span class="s"> row </span>{}&quot;, hit.doc, hit.guid, hit.row),
                }

                <span class="k">let</span> toggle = <span class="k">if</span> <span class="k">self</span>.scene.selected == Some(hit.row) {
                    None
                } <span class="k">else</span> {
                    Some(hit.row)
                };
                <span class="k">self</span>.select(toggle);
            }
            None =&gt; log::info!(&quot;<span class="s">pick: row </span>{}<span class="s"> sub </span>{}<span class="s"> (no document)</span>&quot;, p.row, p.sub),
        }
    }

    <span class="c">/// Draw one frame; a still scene asks for no more.</span>
    <span class="k">pub</span> <span class="k">fn</span> render(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">let</span> logical = <span class="k">self</span>.logical_size();

        <span class="c">// the CSS size changed: control dots keep their pixel size</span>
        <span class="k">if</span> logical != <span class="k">self</span>.gpu.logical_size {
            <span class="k">self</span>.gpu.logical_size = logical;
            <span class="k">self</span>.touch();
        }

        <span class="c">// a GPU error from the last frame</span>
        <span class="k">let</span> failure = <span class="k">match</span> <span class="k">self</span>.gpu.failure.lock() {
            Ok(failure) =&gt; failure.clone(),
            Err(_) =&gt; None,
        };

        <span class="k">if</span> <span class="k">let</span> Some(message) = failure {
            <span class="k">crate</span>::app::feedback::error(&amp;message);
            <span class="k">self</span>.gpu.pick.cancel();
            <span class="k">self</span>.needs_frame = <span class="s">false</span>;
            <span class="k">return</span>;
        }

        <span class="c">// apply a pick answer first, so this frame shows it</span>
        <span class="k">if</span> <span class="k">let</span> Some(pick) = <span class="k">self</span>.gpu.pick.poll() {
            <span class="k">self</span>.apply_pick(pick);
        }

        <span class="k">self</span>.needs_frame = <span class="s">false</span>;</code></pre></div>
<p><code>lessons/12/src/state.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">if</span> <span class="k">self</span>.gpu.view.spin {
            <span class="k">self</span>.camera.orbit(SPIN_STEP, <span class="s">0</span>.<span class="s">0</span>);
        }

        <span class="k">let</span> now_ms = now_ms();
        <span class="k">self</span>.camera.grow_extent(&amp;<span class="k">self</span>.gpu.bounds);
        <span class="k">let</span> origin = <span class="k">self</span>.camera.origin();
        <span class="c">// the point GPU rows are measured from</span>
        <span class="k">let</span> rebase = <span class="k">self</span>
            .gpu
            .rebase_anchor(&amp;origin, <span class="k">self</span>.camera.distance_world(), now_ms);
        <span class="k">let</span> view_proj = <span class="k">self</span>
            .camera
            .view_proj_anchored(<span class="k">self</span>.aspect(), &amp;rebase.anchor);
        <span class="k">let</span> input = FrameInput {
            view_proj,
            clear: CLEAR,
            now_ms,
        };
        <span class="k">self</span>.dirty |= rebase.moved || <span class="k">self</span>.gpu.view.perf || <span class="k">self</span>.gpu.view.spin; <span class="c">// redraw reasons</span>

        <span class="k">let</span> <span class="k">mut</span> dropped = <span class="s">false</span>;

        <span class="k">if</span> <span class="k">self</span>.dirty {
            <span class="k">let</span> gap = now_ms - <span class="k">self</span>.last_frame_ms;
            <span class="k">self</span>.last_frame_ms = now_ms;
            <span class="k">let</span> drawn = <span class="k">self</span>.gpu.present(&amp;input); <span class="c">// encode time, None when the frame was dropped</span>
            dropped = drawn.is_none() &amp;&amp; <span class="k">self</span>.gpu.surface.is_some(); <span class="c">// try again next frame</span>
            <span class="k">self</span>.dirty = dropped;

            <span class="k">if</span> <span class="k">let</span> (<span class="s">true</span>, Some(encode_ms)) = (<span class="k">self</span>.gpu.view.perf, drawn) {
                <span class="k">self</span>.perf_line(gap, encode_ms);
            }
        }

        <span class="c">// a pending pick draws its own id frame</span>
        <span class="k">if</span> !dropped &amp;&amp; <span class="k">let</span> Some(at) = <span class="k">self</span>.gpu.pick.take_pending() {
            <span class="k">self</span>.gpu.pick_frame(&amp;input, at);
        }

        <span class="c">// reasons to draw again</span>
        <span class="k">self</span>.needs_frame |= dropped
            || rebase.pending
            || <span class="k">self</span>.gpu.pick.busy()
            || <span class="k">self</span>.gpu.view.perf
            || <span class="k">self</span>.gpu.view.spin;
        #[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
        <span class="k">crate</span>::app::inspection::publish(<span class="k">self</span>);
    }

    <span class="c">/// CSS size from the canvas; native size is physical.</span>
    <span class="k">fn</span> logical_size(&amp;<span class="k">self</span>) -&gt; [f64; <span class="s">2</span>] {
        #[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
        <span class="k">if</span> <span class="k">let</span> Some(window) = web_sys::window()
            &amp;&amp; <span class="k">let</span> Some(document) = window.document()
            &amp;&amp; <span class="k">let</span> Some(canvas) = document.get_element_by_id(&quot;<span class="s">canvas</span>&quot;)
        {
            <span class="k">return</span> [
                f64::from(canvas.client_width().max(<span class="s">1</span>)),
                f64::from(canvas.client_height().max(<span class="s">1</span>)),
            ];
        }

        [
            f64::from(<span class="k">self</span>.gpu.config.width),
            f64::from(<span class="k">self</span>.gpu.config.height),
        ]
    }

    <span class="c">/// One gesture, one pick pass; Ctrl never selects.</span>
    <span class="k">pub</span> <span class="k">fn</span> request_selection(&amp;<span class="k">mut</span> <span class="k">self</span>, x: u32, y: u32, edge: bool) {
        <span class="k">self</span>.gpu.pick.cancel();
        <span class="k">let</span> mode = <span class="k">if</span> edge {</code></pre></div>
<p><code>lessons/12/src/state.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            PickMode::Edge
        } <span class="k">else</span> {
            PickMode::Object
        };
        <span class="k">self</span>.requested = mode;
        <span class="k">let</span> logical = <span class="k">self</span>.logical_size();
        <span class="k">let</span> scale = f64::from(<span class="k">self</span>.gpu.config.width) / logical[<span class="s">0</span>]; <span class="c">// device pixels per CSS pixel</span>
        <span class="k">self</span>.gpu
            .pick
            .configure(mode, <span class="k">self</span>.selection_radius_css, scale);
        <span class="k">self</span>.gpu.pick.request(x, y);
        <span class="k">self</span>.needs_frame = <span class="s">true</span>;
    }

    <span class="c">/// Esc: leave control points, keep the object selected.</span>
    <span class="k">pub</span> <span class="k">fn</span> escape_selection(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">let</span> parent = <span class="k">self</span>.selection.escape();
        <span class="k">self</span>.select(parent);
        <span class="k">self</span>.status(&quot;&quot;);
    }

    <span class="c">/// T: show or hide the name label on the selection.</span>
    <span class="k">pub</span> <span class="k">fn</span> toggle_selected_names(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">self</span>.show_selected_names = !<span class="k">self</span>.show_selected_names;
        <span class="k">self</span>.update_label();
        <span class="k">self</span>.touch();
    }

    <span class="c">/// Prepare one source-document title when its CAD rows arrive.</span>
    <span class="k">fn</span> annotate_document(&amp;<span class="k">mut</span> <span class="k">self</span>, first_row: usize) {
        <span class="k">let</span> <span class="k">mut</span> bounds = AABB::empty();

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
        anchor[<span class="s">2</span>] = bounds.max_point()[<span class="s">2</span>] + bounds.diagonal() * <span class="s">0</span>.<span class="s">08</span>;
        <span class="k">self</span>.scene_labels.push(nameplate(</code></pre></div>
<p><code>lessons/12/src/state.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">self</span>.scene.docs.len() <span class="k">as</span> u32,
            document.name.clone(),
            anchor,
        ));
    }

    <span class="c">/// Document titles plus the selected name, same color.</span>
    <span class="k">fn</span> update_label(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">let</span> <span class="k">mut</span> labels = <span class="k">self</span>.scene_labels.clone();

        <span class="k">if</span> <span class="k">self</span>.show_selected_names
            &amp;&amp; <span class="k">let</span> Some(row) = <span class="k">self</span>.scene.selected
            &amp;&amp; <span class="k">let</span> Some(bounds) = <span class="k">self</span>.gpu.objects.row_bounds(row)
        {
            <span class="c">// Text ID 0 belongs to selection; document IDs start at 1, independent of rows.</span>
            labels.push(nameplate(
                <span class="s">0</span>,
                <span class="k">self</span>.scene.object_name(row).to_string(),
                label_center(&amp;bounds),
            ));
        }

        <span class="k">if</span> <span class="k">let</span> Err(error) = <span class="k">self</span>.gpu.text.set_labels(labels) {
            <span class="k">self</span>.status(&amp;format!(&quot;<span class="s">Text: </span>{<span class="s">error</span>}&quot;));
        }
    }

    <span class="c">/// Show a message in the status line.</span>
    <span class="k">fn</span> status(&amp;<span class="k">self</span>, message: &amp;str) {
        <span class="k">crate</span>::app::feedback::status(message);
    }

    <span class="c">/// The \`?perf=1\` line: frame number, gap, encode time, heap.</span>
    #[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
    <span class="k">fn</span> perf_line(&amp;<span class="k">self</span>, gap_ms: f64, encode_ms: f64) {
        <span class="k">let</span> line = format!(
            &quot;<span class="s">f</span>{}<span class="s"> gap </span>{<span class="s">gap_ms:.0</span>}<span class="s"> enc </span>{<span class="s">encode_ms:.1</span>}<span class="s"> ms wasm capacity </span>{<span class="s">:.0</span>}<span class="s"> MiB</span>&quot;,
            <span class="k">self</span>.gpu.performance.frames,
            heap_mb()
        );
        <span class="k">crate</span>::engine::performance::perf_line(&amp;line);
    }

    <span class="c">/// Natively the perf line goes nowhere.</span>
    #[cfg(not(target_arch = &quot;<span class="s">wasm32</span>&quot;))]
    <span class="k">fn</span> perf_line(&amp;<span class="k">self</span>, _gap_ms: f64, _encode_ms: f64) {}
}

<span class="c">/// Center an annotation in the source object bounds.</span>
<span class="k">fn</span> label_center(bounds: &amp;AABB) -&gt; [f64; <span class="s">3</span>] {
    <span class="k">let</span> <span class="k">mut</span> center = [<span class="s">0</span>.<span class="s">0</span>; <span class="s">3</span>];

    <span class="k">for</span> (axis, coordinate) <span class="k">in</span> center.iter_mut().enumerate() {
        *coordinate = (bounds.min_point()[axis] + bounds.max_point()[axis]) * <span class="s">0</span>.<span class="s">5</span>;
    }

    center
}</code></pre></div>
<p>Copy this part from the lesson folder to the path shown.</p>
<p><code>lessons/12/src/state.rs</code> · copy the file, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Match production padding, rounded caps and white glyph style.</span>
<span class="k">fn</span> nameplate(id: u32, text: String, world: [f64; <span class="s">3</span>]) -&gt; TextLabel {
    <span class="k">let</span> scale = <span class="k">if</span> id == <span class="s">0</span> { <span class="s">0</span>.<span class="s">75</span> } <span class="k">else</span> { <span class="s">1</span>.<span class="s">0</span> };
    <span class="k">let</span> line_height = <span class="s">26</span>.<span class="s">0</span> * scale;
    <span class="k">let</span> vertical_padding = <span class="s">4</span>.<span class="s">0</span> * scale;
    <span class="c">// a cap at each end keeps the text inside</span>
    <span class="k">let</span> horizontal_padding = <span class="k">if</span> id == <span class="s">0</span> {
        line_height * <span class="s">0</span>.<span class="s">5</span> + vertical_padding
    } <span class="k">else</span> {
        <span class="s">6</span>.<span class="s">0</span> * scale
    };
    TextLabel {
        id,
        text,
        font_size: <span class="s">18</span>.<span class="s">0</span> * scale,
        line_height,
        color: [<span class="s">255</span>; <span class="s">4</span>],
        placement: TextPlacement::Nameplate {
            world,
            padding: [horizontal_padding, vertical_padding],
            rounded: id == <span class="s">0</span>, <span class="c">// the title gets round corners</span>
        },
        clip: None,
    }
}</code></pre></div>
<h2 id="step-20-srcenginegpumodrs">Step 20 · src/engine/gpu/mod.rs<a class="anchor" href="#/course/12-picking#step-20-srcenginegpumodrs" aria-label="Link to this section">#</a></h2>
<p>The GPU owner connects buffers, pipelines and frame resources.</p>
<p><code>lessons/12/src/engine/gpu/mod.rs</code> · edit · type this</p>
<p>Replaces <code>mod frame</code> in <code>lessons/11/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> <span class="k">mod</span> device;
<span class="k">pub</span> <span class="k">mod</span> frame;
<span class="k">pub</span> <span class="k">mod</span> glyphs;
<span class="k">pub</span> <span class="k">mod</span> instance;
<span class="k">pub</span> <span class="k">mod</span> lod;
<span class="k">pub</span> <span class="k">mod</span> objects;
<span class="k">pub</span> <span class="k">mod</span> pick;
<span class="k">pub</span> <span class="k">mod</span> present;
<span class="k">pub</span> <span class="k">mod</span> render;
<span class="k">pub</span> <span class="k">mod</span> segments;
<span class="k">pub</span> <span class="k">mod</span> selection_outline;
<span class="k">pub</span> <span class="k">mod</span> splat;
<span class="k">pub</span> <span class="k">mod</span> targets;
<span class="k">pub</span> <span class="k">mod</span> text;
<span class="k">pub</span> <span class="k">mod</span> text_outline;
<span class="k">pub</span> <span class="k">mod</span> upload;
<span class="k">pub</span> <span class="k">mod</span> view;

<span class="k">use</span> <span class="k">crate</span>::engine::performance::Performance;
<span class="k">use</span> <span class="k">crate</span>::engine::pipelines::{Layouts, Target};
<span class="k">use</span> session_rust::{AABB, Point};

<span class="k">use</span> arena::ArenaLane;
<span class="k">use</span> backdrop::BackdropLane;
<span class="k">use</span> buffers::GpuCtx;
<span class="k">use</span> cloud::CloudLane;
<span class="k">use</span> device::DeviceSetup;
<span class="k">use</span> frame::FrameUniforms;
<span class="k">use</span> glyphs::GlyphLane;
<span class="k">use</span> objects::{InkScene, InstanceTable};
<span class="k">use</span> pick::Picker;
<span class="k">use</span> segments::SegmentLane;
<span class="k">use</span> splat::Splat;
<span class="k">use</span> targets::Targets;

<span class="k">pub</span> <span class="k">use</span> cloud::{CloudDraw, LodNode, NO_NORMALS};
<span class="k">pub</span> <span class="k">use</span> frame::FrameInput;
<span class="k">pub</span> <span class="k">use</span> glyphs::GlyphPoint;
<span class="k">pub</span> <span class="k">use</span> instance::Instance;
<span class="k">pub</span> <span class="k">use</span> objects::{ObjectRow, Rebase};
<span class="k">pub</span> <span class="k">use</span> pick::Pick;
<span class="k">pub</span> <span class="k">use</span> segments::CylinderSegment;
<span class="k">pub</span> <span class="k">use</span> upload::Upload;
<span class="k">pub</span> <span class="k">use</span> view::View;

<span class="c">/// Everything on the GPU: the device, the frame and one field per lane.</span>
<span class="k">pub</span> <span class="k">struct</span> Gpu {
    <span class="k">pub</span> surface: Option&lt;wgpu::Surface&lt;'static&gt;&gt;, <span class="c">// the canvas; None when headless</span>
    <span class="k">pub</span> ctx: GpuCtx, <span class="c">// device and queue</span>
    <span class="k">pub</span> config: wgpu::SurfaceConfiguration, <span class="c">// canvas size and format</span>
    <span class="k">pub</span> layouts: Layouts, <span class="c">// shared bind group layouts</span>
    <span class="k">pub</span> frame: FrameUniforms,
    <span class="k">pub</span> targets: Targets, <span class="c">// depth and color textures</span>
    <span class="k">pub</span> view: View, <span class="c">// display settings</span>
    <span class="k">pub</span> objects: InstanceTable,
    <span class="k">pub</span> backdrop: BackdropLane,
    <span class="k">pub</span> arena: ArenaLane, <span class="c">// meshes</span>
    <span class="k">pub</span> segments: SegmentLane, <span class="c">// lines</span>
    <span class="k">pub</span> glyphs: GlyphLane, <span class="c">// markers and dots</span>
    <span class="k">pub</span> controls: GlyphLane, <span class="c">// control point dots</span>
    <span class="k">pub</span> control_net: SegmentLane, <span class="c">// control polygon lines</span>
    <span class="k">pub</span> text: text::TextLane, <span class="c">// labels</span>
    <span class="k">pub</span> selection_outline: selection_outline::SelectionOutline,
    <span class="k">pub</span> logical_size: [f64; <span class="s">2</span>], <span class="c">// canvas size in CSS pixels</span>
    <span class="k">pub</span> cloud: CloudLane, <span class="c">// point cloud buffers</span>
    <span class="k">pub</span> splat: Splat, <span class="c">// point cloud drawing</span>
    <span class="k">pub</span> pick: Picker, <span class="c">// reads object ids under the cursor</span>
    <span class="k">pub</span> performance: Performance, <span class="c">// frame timing</span>
    <span class="k">pub</span> bounds: AABB, <span class="c">// world box of everything uploaded</span>
    device_type: wgpu::DeviceType, <span class="c">// discrete, integrated or CPU</span>
    <span class="k">pub</span> failure: std::sync::Arc&lt;std::sync::Mutex&lt;Option&lt;String&gt;&gt;&gt;, <span class="c">// first GPU error</span>
}

<span class="k">impl</span> Gpu {
    <span class="c">/// Bytes reserved on the GPU: (buffers, textures).</span>
    <span class="k">pub</span> <span class="k">fn</span> allocated_bytes(&amp;<span class="k">self</span>) -&gt; (u64, u64) {
        <span class="k">let</span> (splat_buffers, splat_textures) = <span class="k">self</span>.splat.allocated_bytes();
        <span class="k">let</span> (pick_buffers, pick_textures) = <span class="k">self</span>.pick.allocated_bytes();
        <span class="k">let</span> (outline_buffers, outline_textures) = <span class="k">self</span>.selection_outline.allocated_bytes();
        <span class="k">let</span> buffers = <span class="k">self</span>.arena.allocated_bytes()
            + <span class="k">self</span>.segments.allocated_bytes()
            + <span class="k">self</span>.glyphs.allocated_bytes()
            + <span class="k">self</span>.controls.allocated_bytes()
            + <span class="k">self</span>.control_net.allocated_bytes()
            + <span class="k">self</span>.cloud.allocated_bytes()
            + <span class="k">self</span>.objects.allocated_bytes()
            + <span class="k">self</span>.frame.allocated_bytes()
            + <span class="k">self</span>.text.allocated_bytes()
            + splat_buffers
            + pick_buffers
            + outline_buffers;
        <span class="k">let</span> pixels = u64::from(<span class="k">self</span>.config.width) * u64::from(<span class="k">self</span>.config.height);
        <span class="k">let</span> samples = u64::from(<span class="k">self</span>.targets.samples);
        <span class="k">let</span> frame_textures =
            pixels * <span class="k">if</span> samples &gt; <span class="s">1</span> { samples * <span class="s">12</span> } <span class="k">else</span> { <span class="s">8</span> } + <span class="k">if</span> samples &gt; <span class="s">1</span> { <span class="s">8</span> } <span class="k">else</span> { <span class="s">32</span> };
        (
            buffers,
            frame_textures
                + splat_textures
                + pick_textures
                + <span class="k">self</span>.text.texture_bytes()
                + outline_textures,
        )
    }

    <span class="c">/// Open the GPU for a window.</span>
    <span class="k">pub</span> <span class="k">async</span> <span class="k">fn</span> new(window: std::sync::Arc&lt;winit::window::Window&gt;) -&gt; anyhow::Result&lt;<span class="k">Self</span>&gt; {
        <span class="k">let</span> size = window.inner_size();
        <span class="k">Self</span>::build(Some(window), (size.width, size.height)).<span class="k">await</span>
    }

    <span class="c">/// Open the GPU with no window, drawing into a texture.</span>
    <span class="k">pub</span> <span class="k">async</span> <span class="k">fn</span> new_headless(width: u32, height: u32) -&gt; anyhow::Result&lt;<span class="k">Self</span>&gt; {
        <span class="k">Self</span>::build(None, (width, height)).<span class="k">await</span>
    }

    <span class="c">/// Open the device and create every lane, empty.</span>
    <span class="k">async</span> <span class="k">fn</span> build(
        window: Option&lt;std::sync::Arc&lt;winit::window::Window&gt;&gt;,
        size: (u32, u32),
    ) -&gt; anyhow::Result&lt;<span class="k">Self</span>&gt; {
        <span class="k">let</span> DeviceSetup {
            surface,
            device,
            queue,
            config,
            device_type,
            failure,
        } = device::open(window, size).<span class="k">await</span>?;
        <span class="k">let</span> ctx = GpuCtx { device, queue };
        <span class="k">let</span> size = (config.width, config.height);
        <span class="c">// start without MSAA; retarget flips it later</span>
        <span class="k">let</span> target = Target {
            format: config.format,
            samples: <span class="s">1</span>,
        };

        <span class="k">let</span> layouts = Layouts::new(&amp;ctx.device);
        <span class="k">let</span> frame = FrameUniforms::new(&amp;ctx, &amp;layouts, size);
        <span class="k">let</span> targets = Targets::new(&amp;ctx, size, config.format, target.samples);
        <span class="k">let</span> arena = ArenaLane::new(&amp;ctx, &amp;layouts, target);
        <span class="k">let</span> objects = InstanceTable::new(&amp;ctx, &amp;layouts, &amp;InkScene { targets: &amp;targets });
        <span class="k">let</span> backdrop = BackdropLane::new(&amp;ctx, &amp;layouts, target);
        <span class="k">let</span> segments = SegmentLane::new(&amp;ctx, &amp;layouts, target);
        <span class="k">let</span> glyphs = GlyphLane::new(&amp;ctx, &amp;layouts, target);
        <span class="k">let</span> controls = GlyphLane::new(&amp;ctx, &amp;layouts, target);
        <span class="k">let</span> control_net = SegmentLane::new(&amp;ctx, &amp;layouts, target);
        <span class="k">let</span> text = text::TextLane::new(&amp;ctx, target);
        <span class="k">let</span> selection_outline = selection_outline::SelectionOutline::new(&amp;ctx, target);
        <span class="k">let</span> cloud = CloudLane::new(&amp;ctx);
        <span class="k">let</span> splat = Splat::new(&amp;ctx, &amp;layouts, target, cloud.buffers());

        log::info!(
            &quot;<span class="s">viewer init OK - surface </span>{}<span class="s">x</span>{}<span class="s">, format </span>{<span class="s">:?</span>}&quot;,
            config.width,
            config.height,
            config.format
        );</code></pre></div>
<p>Replaces the 12 lines from <code>view: view::View::from_env(),</code> in <code>fn new</code> of <code>lessons/11/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            view: View::from_env(),
            objects,
            backdrop,
            arena,
            segments,
            glyphs,
            controls,
            control_net,
            text,
            selection_outline,
            logical_size: [size.<span class="s">0</span> <span class="k">as</span> f64, size.<span class="s">1</span> <span class="k">as</span> f64],
            cloud,
            splat,
            pick: Picker::new(),
            performance: Performance::new(),
            bounds: AABB::empty(),
            device_type,
            failure,
        })
    }

    <span class="c">/// Append one upload to every lane.</span></code></pre></div>
<p>Replaces the 10 lines from <code>self.retarget(false);</code> in <code>fn set_scene</code> of <code>lessons/11/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        log::info!(
            &quot;<span class="s">scene: </span>{}<span class="s"> objects, </span>{}<span class="s"> verts, </span>{}<span class="s"> pipes, </span>{}<span class="s"> ribbons, </span>{}<span class="s"> markers, </span>{}<span class="s"> dots, </span>{}<span class="s"> points</span>&quot;,
            <span class="k">self</span>.objects.len(),
            <span class="k">self</span>.arena.vert_count(),
            <span class="k">self</span>.segments.pipe_count(),
            <span class="k">self</span>.segments.ribbon_count(),
            <span class="k">self</span>.glyphs.sphere_count(),
            <span class="k">self</span>.glyphs.dot_count(),
            <span class="k">self</span>.cloud.point_count
        );
        <span class="k">self</span>.retarget(<span class="s">false</span>);
        <span class="k">self</span>.objects.rebind_ink(
            &amp;<span class="k">self</span>.ctx,
            &amp;<span class="k">self</span>.layouts,
            &amp;InkScene {
                targets: &amp;<span class="k">self</span>.targets,
            },
        );
    }

    <span class="c">/// Current color format and sample count.</span></code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Remake targets and pipelines when the sample count changes.</span></code></pre></div>
<p>Replaces the <code>self.text.retarget(&amp;self.ctx, target);</code> line in <code>fn retarget</code> of <code>lessons/11/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">self</span>.controls.retarget(&amp;<span class="k">self</span>.ctx, &amp;<span class="k">self</span>.layouts, target);
            <span class="k">self</span>.control_net.retarget(&amp;<span class="k">self</span>.ctx, &amp;<span class="k">self</span>.layouts, target);
            <span class="k">self</span>.text.retarget(&amp;<span class="k">self</span>.ctx, target);
            <span class="k">self</span>.selection_outline.retarget(&amp;<span class="k">self</span>.ctx, target);
            <span class="k">self</span>.splat.retarget(&amp;<span class="k">self</span>.ctx, &amp;<span class="k">self</span>.layouts, target);
            log::info!(&quot;<span class="s">msaa: </span>{}<span class="s">x</span>&quot;, samples);
        }
    }

    <span class="c">/// Pixels this GPU can afford at 4x MSAA.</span>
    <span class="k">pub</span> <span class="k">fn</span> msaa_budget(&amp;<span class="k">self</span>) -&gt; Option&lt;u32&gt; {
        Targets::msaa_budget(<span class="k">self</span>.device_type)
    }

    <span class="c">/// MSAA samples for the current scene: 4x only with solid geometry.</span></code></pre></div>
<p>Replaces <code>fn rebase_anchor</code> in <code>lessons/11/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Move the scene origin near the camera when it drifted far.</span>
    <span class="k">pub</span> <span class="k">fn</span> rebase_anchor(&amp;<span class="k">mut</span> <span class="k">self</span>, origin: &amp;Point, view_dist: f64, now: f64) -&gt; Rebase {
        <span class="k">let</span> rebase = <span class="k">self</span>
            .objects
            .rebase_anchor(&amp;<span class="k">self</span>.ctx, origin, view_dist, now);

        <span class="k">if</span> rebase.moved {
            <span class="k">self</span>.splat.invalidate();
        }

        rebase
    }

    <span class="c">/// Resize the canvas and every texture that follows it.</span>
    <span class="k">pub</span> <span class="k">fn</span> resize(&amp;<span class="k">mut</span> <span class="k">self</span>, width: u32, height: u32) {
        <span class="k">if</span> width == <span class="s">0</span> || height == <span class="s">0</span> {
            <span class="k">return</span>;
        }

        <span class="k">self</span>.config.width = width;
        <span class="k">self</span>.config.height = height;

        <span class="k">if</span> <span class="k">let</span> Some(s) = &amp;<span class="k">self</span>.surface {
            s.configure(&amp;<span class="k">self</span>.ctx.device, &amp;<span class="k">self</span>.config);
        }

        <span class="k">self</span>.retarget(<span class="s">true</span>);
        <span class="k">self</span>.splat.resize();
        <span class="k">self</span>.pick.resize();
    }

    <span class="c">/// Forget every row; keep the buffers.</span>
    <span class="k">pub</span> <span class="k">fn</span> reset(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">self</span>.objects.reset();
        <span class="k">self</span>.arena.reset();
        <span class="k">self</span>.segments.reset();
        <span class="k">self</span>.glyphs.reset();
        <span class="k">self</span>.controls.reset();
        <span class="k">self</span>.control_net.reset();
        <span class="k">self</span>.text.reset();
        <span class="k">self</span>.selection_outline.reset();
        <span class="k">self</span>.pick.cancel();
        <span class="k">self</span>.segments.set_edge(&amp;<span class="k">self</span>.ctx, None);
        <span class="k">self</span>.cloud.reset();
        <span class="k">self</span>.splat.invalidate();
        <span class="k">self</span>.bounds = AABB::empty();
    }

    <span class="c">/// Forget every row and free the buffers.</span>
    <span class="k">pub</span> <span class="k">fn</span> release(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">self</span>.objects.release(&amp;<span class="k">self</span>.ctx, &amp;<span class="k">self</span>.layouts);
        <span class="k">self</span>.arena.release(&amp;<span class="k">self</span>.ctx);
        <span class="k">self</span>.segments.release(&amp;<span class="k">self</span>.ctx, &amp;<span class="k">self</span>.layouts);
        <span class="k">self</span>.glyphs.release(&amp;<span class="k">self</span>.ctx, &amp;<span class="k">self</span>.layouts);
        <span class="k">self</span>.controls.release(&amp;<span class="k">self</span>.ctx, &amp;<span class="k">self</span>.layouts);
        <span class="k">self</span>.control_net.release(&amp;<span class="k">self</span>.ctx, &amp;<span class="k">self</span>.layouts);
        <span class="k">self</span>.text.release(&amp;<span class="k">self</span>.ctx);
        <span class="k">self</span>.selection_outline.reset();
        <span class="k">self</span>.pick.cancel();
        <span class="k">self</span>.segments.set_edge(&amp;<span class="k">self</span>.ctx, None);
        <span class="k">self</span>.cloud.release(&amp;<span class="k">self</span>.ctx);
        <span class="k">self</span>.splat.release();
        <span class="k">self</span>.splat
            .rebind(&amp;<span class="k">self</span>.ctx, &amp;<span class="k">self</span>.layouts, <span class="k">self</span>.cloud.buffers());
        <span class="k">self</span>.bounds = AABB::empty();
        <span class="k">self</span>.retarget(<span class="s">false</span>);
        <span class="k">self</span>.objects.rebind_ink(
            &amp;<span class="k">self</span>.ctx,
            &amp;<span class="k">self</span>.layouts,
            &amp;InkScene {
                targets: &amp;<span class="k">self</span>.targets,
            },
        );
    }

    <span class="c">/// Select or deselect object \`row\`.</span>
    <span class="k">pub</span> <span class="k">fn</span> set_selected(&amp;<span class="k">mut</span> <span class="k">self</span>, row: u32, on: bool) {
        <span class="k">self</span>.selection_outline.set_selected(row, on);
        <span class="k">self</span>.objects
            .set_flag(&amp;<span class="k">self</span>.ctx, row, Instance::FLAG_SELECTED, on);
        <span class="k">self</span>.splat.invalidate();
    }

    <span class="c">/// Hide or show object \`row\`.</span>
    <span class="k">pub</span> <span class="k">fn</span> set_hidden(&amp;<span class="k">mut</span> <span class="k">self</span>, row: u32, on: bool) {
        <span class="k">self</span>.objects
            .set_flag(&amp;<span class="k">self</span>.ctx, row, Instance::FLAG_HIDDEN, on);
        <span class="k">self</span>.splat.invalidate();
    }
}

<span class="c">/// Every lane's shader sources, for the tests.</span>
#[cfg(test)]
<span class="k">pub</span>(<span class="k">crate</span>) <span class="k">fn</span> lane_shaders() -&gt; Vec&lt;(&amp;'static str, &amp;'static str)&gt; {
    <span class="k">let</span> <span class="k">mut</span> out = Vec::new();
    out.extend_from_slice(backdrop::SHADERS);
    out.extend_from_slice(arena::SHADERS);
    out.extend_from_slice(segments::SHADERS);
    out.extend_from_slice(glyphs::SHADERS);
    out</code></pre></div>
<h2 id="step-21-srcappmodrs">Step 21 · src/app/mod.rs<a class="anchor" href="#/course/12-picking#step-21-srcappmodrs" aria-label="Link to this section">#</a></h2>
<p>The application module connects source loading and interaction helpers.</p>
<p><code>lessons/12/src/app/mod.rs</code> · edit · type this</p>
<p>Replaces <code>mod knobs</code> in <code>lessons/11/src/app/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> <span class="k">mod</span> feedback;
<span class="k">pub</span> <span class="k">mod</span> input;
<span class="k">pub</span> <span class="k">mod</span> knobs;
<span class="k">pub</span> <span class="k">mod</span> scene;
<span class="k">pub</span> <span class="k">mod</span> selection;
<span class="k">pub</span> <span class="k">mod</span> stream;
<span class="k">pub</span> <span class="k">mod</span> touch;
<span class="k">pub</span> <span class="k">mod</span> walk;

#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">pub</span> <span class="k">mod</span> loader;
#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">pub</span> <span class="k">mod</span> route;

#[cfg(any(target_arch = &quot;<span class="s">wasm32</span>&quot;, test))]
<span class="k">pub</span> <span class="k">mod</span> inspection;</code></pre></div>
<h2 id="step-22-srcappwalkmodrs">Step 22 · src/app/walk/mod.rs<a class="anchor" href="#/course/12-picking#step-22-srcappwalkmodrs" aria-label="Link to this section">#</a></h2>
<p>The geometry walk dispatches source types into their render buffers.</p>
<p><code>lessons/12/src/app/walk/mod.rs</code> · edit · type this</p>
<p>Added at the top of <code>lessons/11/src/app/walk/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> <span class="k">crate</span>::engine::gpu::Upload;
<span class="k">use</span> <span class="k">crate</span>::engine::gpu::arena::ArenaRows;
<span class="k">use</span> <span class="k">crate</span>::engine::gpu::cloud::CloudRows;
<span class="k">use</span> <span class="k">crate</span>::engine::gpu::glyphs::GlyphRows;
<span class="k">use</span> <span class="k">crate</span>::engine::gpu::segments::SegRows;
<span class="k">use</span> brep::{walk_brep, walk_surface};
<span class="k">use</span> cloud::walk_cloud;
<span class="k">use</span> curves::{walk_line, walk_nurbscurve, walk_polyline};
<span class="k">use</span> frames::{walk_obb, walk_plane};
<span class="k">use</span> mesh::{MeshCx, MeshOpts, walk_mesh};
<span class="k">use</span> mesh_ink::Ink;
<span class="k">use</span> points::walk_point;
<span class="k">use</span> session_rust::AABB;
<span class="k">use</span> session_rust::Geometry;
<span class="k">use</span> session_rust::element::ElementGeometry;

<span class="k">pub</span> <span class="k">mod</span> bounds;
<span class="k">pub</span> <span class="k">mod</span> brep;
<span class="k">pub</span> <span class="k">mod</span> brep_edges;
<span class="k">pub</span> <span class="k">mod</span> brep_orient;
<span class="k">pub</span> <span class="k">mod</span> cloud;
<span class="k">pub</span> <span class="k">mod</span> curves;
<span class="k">pub</span> <span class="k">mod</span> encode;
<span class="k">pub</span> <span class="k">mod</span> frames;
<span class="k">pub</span> <span class="k">mod</span> mesh;
<span class="k">pub</span> <span class="k">mod</span> mesh_ink;
<span class="k">pub</span> <span class="k">mod</span> mesh_topology;
<span class="k">pub</span> <span class="k">mod</span> points;

<span class="c">/// The four row tables one object writes into.</span>
<span class="k">pub</span> <span class="k">struct</span> Walk&lt;'a&gt; {
    <span class="k">pub</span> arena: &amp;'a <span class="k">mut</span> ArenaRows, <span class="c">// triangles of faces</span>
    <span class="k">pub</span> seg: &amp;'a <span class="k">mut</span> SegRows, <span class="c">// line segments</span>
    <span class="k">pub</span> glyph: &amp;'a <span class="k">mut</span> GlyphRows, <span class="c">// dots and labels</span>
    <span class="k">pub</span> cloud: &amp;'a <span class="k">mut</span> CloudRows, <span class="c">// point cloud points</span>
}

<span class="k">impl</span>&lt;'a&gt; Walk&lt;'a&gt; {
    <span class="c">/// Borrow every table of one upload.</span>
    <span class="k">pub</span> <span class="k">fn</span> of(t: &amp;'a <span class="k">mut</span> Upload) -&gt; <span class="k">Self</span> {
        <span class="k">Self</span> {
            arena: &amp;<span class="k">mut</span> t.arena,
            seg: &amp;<span class="k">mut</span> t.seg,
            glyph: &amp;<span class="k">mut</span> t.glyph,
            cloud: &amp;<span class="k">mut</span> t.cloud,
        }
    }

    <span class="c">/// Tables a solid needs: faces plus its edge ink.</span>
    <span class="k">fn</span> solid(&amp;<span class="k">mut</span> <span class="k">self</span>) -&gt; (&amp;<span class="k">mut</span> ArenaRows, Ink&lt;'_&gt;) {
        (
            <span class="k">self</span>.arena,
            Ink {
                seg: <span class="k">self</span>.seg,
                glyph: <span class="k">self</span>.glyph,
            },
        )
    }
}</code></pre></div>
<p><code>lessons/12/src/app/walk/mod.rs</code> · edit · type this</p>
<p>Added after the <code>}</code> line of <code>lessons/11/src/app/walk/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// An element without geometry gets no row.</span>
<span class="k">pub</span> <span class="k">fn</span> is_drawable(geom: &amp;Geometry) -&gt; bool {
    <span class="k">match</span> geom {
        Geometry::Element(e) =&gt; !matches!(e.geometry(), ElementGeometry::None),
        _ =&gt; <span class="s">true</span>,
    }
}

<span class="c">/// Write one object into the tables and report its row.</span>
<span class="k">pub</span> <span class="k">fn</span> walk_geometry(w: &amp;<span class="k">mut</span> Walk, cx: &amp;WalkCx, geom: &amp;Geometry) -&gt; Row {
    <span class="k">match</span> geom {
        Geometry::Mesh(m) =&gt; {
            <span class="k">let</span> (arena, <span class="k">mut</span> ink) = w.solid();
            walk_mesh(
                arena,
                &amp;<span class="k">mut</span> ink,
                m,
                &amp;MeshCx {
                    cx,
                    opts: &amp;MeshOpts::OBJECT,
                },
            )
        }
        Geometry::BRep(b) =&gt; {
            <span class="k">let</span> (arena, <span class="k">mut</span> ink) = w.solid();
            walk_brep(arena, &amp;<span class="k">mut</span> ink, b, cx)
        }
        Geometry::NurbsSurface(s) =&gt; {
            <span class="k">let</span> (arena, <span class="k">mut</span> ink) = w.solid();
            walk_surface(arena, &amp;<span class="k">mut</span> ink, s, cx)
        }
        Geometry::Line(l) =&gt; walk_line(w.seg, l, cx.row),
        Geometry::Polyline(pl) =&gt; walk_polyline(w.seg, pl, cx.row),
        Geometry::NurbsCurve(c) =&gt; walk_nurbscurve(w.seg, c, cx.row),
        Geometry::Plane(p) =&gt; walk_plane(w.seg, p, cx.row),
        Geometry::OBB(b) =&gt; walk_obb(w.seg, b, cx.row),
        Geometry::Point(p) =&gt; walk_point(w.glyph, p, cx.row),
        Geometry::PointCloud(pc) =&gt; walk_cloud(w.cloud, pc, cx),
        Geometry::Element(e) =&gt; <span class="k">match</span> e.geometry() {
            ElementGeometry::Mesh(m) =&gt; {
                <span class="k">let</span> (arena, <span class="k">mut</span> ink) = w.solid();
                walk_mesh(
                    arena,
                    &amp;<span class="k">mut</span> ink,
                    m,
                    &amp;MeshCx {
                        cx,
                        opts: &amp;MeshOpts::ELEMENT,
                    },
                )
            }
            ElementGeometry::BRep(b) =&gt; {
                <span class="k">let</span> (arena, <span class="k">mut</span> ink) = w.solid();
                walk_brep(arena, &amp;<span class="k">mut</span> ink, b, cx)
            }
            ElementGeometry::None =&gt; Row::thin(AABB::empty()),
        },
    }
}</code></pre></div>
<h2 id="step-23-srcapprouters">Step 23 · src/app/route.rs<a class="anchor" href="#/course/12-picking#step-23-srcapprouters" aria-label="Link to this section">#</a></h2>
<p>Route helpers read viewer options from the page URL.</p>
<p><code>lessons/12/src/app/route.rs</code> · edit · type this</p>
<p>Replaces <code>fn query</code> in <code>lessons/11/src/app/route.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// The \`?name=\` value of the page URL.</span>
<span class="k">pub</span> <span class="k">fn</span> query(name: &amp;str) -&gt; Option&lt;String&gt; {
    <span class="k">let</span> search = web_sys::window()?.location().search().ok()?;
    <span class="k">let</span> raw = search.strip_prefix('<span class="s">?</span>')?;
    <span class="k">let</span> prefix = format!(&quot;{<span class="s">name</span>}<span class="s">=</span>&quot;);

    <span class="k">for</span> pair <span class="k">in</span> raw.split('<span class="s">&amp;</span>') {
        <span class="k">if</span> <span class="k">let</span> Some(v) = pair.strip_prefix(prefix.as_str()) {
            <span class="k">return</span> js_sys::decode_uri_component(v).ok()?.as_string();
        }

        <span class="k">if</span> pair == name {
            <span class="k">return</span> Some(String::new());</code></pre></div>
<h2 id="step-24-srclibrs">Step 24 · src/lib.rs<a class="anchor" href="#/course/12-picking#step-24-srclibrs" aria-label="Link to this section">#</a></h2>
<p>The crate entry point connects the camera, scene and GPU owners.</p>
<p><code>lessons/12/src/lib.rs</code> · type this, replace the whole file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> <span class="k">mod</span> app;
<span class="k">mod</span> camera;
<span class="k">mod</span> engine;
<span class="k">mod</span> state;
#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">pub</span> <span class="k">mod</span> text_quality;

<span class="k">use</span> <span class="k">crate</span>::app::scene::{FileDoc, StreamedInit};
<span class="k">use</span> <span class="k">crate</span>::app::walk::cloud::StreamRows;
<span class="k">pub</span> <span class="k">use</span> state::State;

<span class="c">/// One more slice of streamed cloud \`idx\`.</span>
<span class="k">pub</span> <span class="k">struct</span> CloudChunk {
    <span class="k">pub</span> idx: usize, <span class="c">// which cloud</span>
    <span class="k">pub</span> rows: StreamRows, <span class="c">// the new points</span>
    <span class="k">pub</span> to: u32, <span class="c">// rows loaded so far</span>
}

<span class="c">/// Messages the async loader sends to the event loop.</span>
<span class="k">pub</span> <span class="k">enum</span> Msg {
    Ready(Box&lt;State&gt;), <span class="c">// GPU is up, here is the state</span>
    File(FileDoc),
    Clear, <span class="c">// empty the scene</span>
    Fit, <span class="c">// frame the camera on everything</span>
    StreamedCloud(Box&lt;StreamedInit&gt;),
    CloudChunk(CloudChunk), <span class="c">// more points arrived</span>
    CancelPointer, <span class="c">// the browser lost the pointer</span>
}</code></pre></div>
<p><code>lessons/12/src/lib.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">use</span> {
    <span class="k">crate</span>::app::{input::Input, loader},
    std::sync::Arc,
    wasm_bindgen::JsCast,
    wasm_bindgen::prelude::*,
    winit::application::ApplicationHandler,
    winit::event::{ElementState, WindowEvent},
    winit::event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    winit::platform::web::{EventLoopExtWebSys, WindowAttributesExtWebSys},
    winit::window::{Window, WindowId},
};

<span class="c">/// The winit application: owns the state and the gestures.</span>
#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">pub</span> <span class="k">struct</span> App {
    state: Option&lt;State&gt;, <span class="c">// everything drawn, once the GPU is up</span>
    proxy: Option&lt;EventLoopProxy&lt;Msg&gt;&gt;, <span class="c">// sends messages into the loop</span>
    input: Input, <span class="c">// mouse and key gestures</span>
    pointer_cancellation: Option&lt;app::input::PointerCancellation&gt;, <span class="c">// browser pointer-lost listener</span>
}

#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">impl</span> App {
    <span class="c">/// Create the event loop and spawn the app on the browser's main loop.</span>
    <span class="k">pub</span> <span class="k">fn</span> run() -&gt; anyhow::Result&lt;()&gt; {
        <span class="c">// log::info! goes to the browser console</span>
        console_log::init_with_level(log::Level::Info).ok();
        <span class="k">let</span> event_loop = EventLoop::&lt;Msg&gt;::with_user_event().build()?;
        <span class="k">let</span> app = App {
            proxy: Some(event_loop.create_proxy()),
            state: None,
            input: Input::new(),
            pointer_cancellation: None,
        };
        event_loop.spawn_app(app);
        Ok(())
    }

    <span class="c">/// Take the ready state, size it to the canvas, draw.</span>
    <span class="k">fn</span> adopt(&amp;<span class="k">mut</span> <span class="k">self</span>, <span class="k">mut</span> state: State) {
        <span class="c">// match the canvas pixel size</span>
        <span class="k">if</span> <span class="k">let</span> Some((w, h)) = desired_canvas_size() {
            state.resize(w, h);
        }

        state.window.request_redraw();
        <span class="k">self</span>.state = Some(state);
    }

    <span class="c">/// Ask for a redraw only when something changed.</span>
    <span class="k">fn</span> request_if_needed(&amp;<span class="k">self</span>) {
        <span class="k">if</span> <span class="k">let</span> Some(state) = &amp;<span class="k">self</span>.state
            &amp;&amp; state.needs_frame
        {
            state.window.request_redraw();
        }
    }
}</code></pre></div>
<p><code>lessons/12/src/lib.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">impl</span> ApplicationHandler&lt;Msg&gt; <span class="k">for</span> App {
    <span class="c">/// Bind the window to the page canvas and start loading.</span>
    <span class="k">fn</span> resumed(&amp;<span class="k">mut</span> <span class="k">self</span>, event_loop: &amp;ActiveEventLoop) {
        <span class="c">// runs once</span>
        <span class="k">if</span> <span class="k">self</span>.state.is_some() || <span class="k">self</span>.proxy.is_none() {
            <span class="k">return</span>;
        }

        <span class="k">let</span> Some(canvas) = viewer_canvas() <span class="k">else</span> {
            app::feedback::error(&quot;<span class="s">The viewer canvas is missing or invalid</span>&quot;);
            <span class="k">return</span>;
        };
        <span class="c">// the winit window is the page canvas</span>
        <span class="k">let</span> attrs = Window::default_attributes().with_canvas(Some(canvas.clone()));
        <span class="k">let</span> window = <span class="k">match</span> event_loop.create_window(attrs) {
            Ok(window) =&gt; Arc::new(window),
            Err(error) =&gt; {
                app::feedback::error(&amp;format!(&quot;<span class="s">Cannot initialize the viewer window: </span>{<span class="s">error</span>}&quot;));
                <span class="k">return</span>;
            }
        };

        <span class="k">if</span> <span class="k">let</span> Some(proxy) = <span class="k">self</span>.proxy.take() {
            <span class="k">match</span> app::input::PointerCancellation::new(canvas, proxy.clone()) {
                Ok(listener) =&gt; <span class="k">self</span>.pointer_cancellation = Some(listener),
                Err(error) =&gt; log::warn!(&quot;<span class="s">Cannot register pointer cancellation: </span>{<span class="s">error:?</span>}&quot;),
            }

            <span class="c">// async: GPU setup, then Msg::Ready</span>
            wasm_bindgen_futures::spawn_local(loader::boot(window, proxy));
        }
    }

    <span class="c">/// Apply one loader message to the scene.</span>
    <span class="k">fn</span> user_event(&amp;<span class="k">mut</span> <span class="k">self</span>, _event_loop: &amp;ActiveEventLoop, msg: Msg) {
        <span class="c">// Ready is the only message without a state yet</span>
        <span class="k">let</span> msg = <span class="k">match</span> msg {
            Msg::Ready(state) =&gt; <span class="k">return</span> <span class="k">self</span>.adopt(*state),
            other =&gt; other,
        };
        <span class="k">let</span> Some(state) = &amp;<span class="k">mut</span> <span class="k">self</span>.state <span class="k">else</span> { <span class="k">return</span> };

        <span class="k">match</span> msg {
            Msg::Ready(_) =&gt; {}
            Msg::Clear =&gt; state.clear(),
            Msg::Fit =&gt; state.fit_all(),
            Msg::File(doc) =&gt; state.append(doc),
            Msg::StreamedCloud(init) =&gt; {
                <span class="c">// add the first rows, keep loading the rest</span>
                <span class="k">let</span> (url, fields, from, col_at) = (
                    init.url.clone(),
                    init.fields.clone(),
                    init.resident,
                    init.col_at,
                );
                <span class="k">let</span> idx = state.add_streamed(*init);
                loader::spawn_stream_rest(loader::StreamCursor {
                    idx,
                    url,
                    fields,
                    from,
                    col_at,
                });
            }
            Msg::CloudChunk(c) =&gt; state.extend_streamed(c.idx, c.rows, c.to),
            Msg::CancelPointer =&gt; {
                <span class="k">self</span>.input.cancel();
                state.touch();
            }
        }

        <span class="k">self</span>.request_if_needed();
    }</code></pre></div>
<p><code>lessons/12/src/lib.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Handle one window event: redraw, resize, key or mouse.</span>
    <span class="k">fn</span> window_event(&amp;<span class="k">mut</span> <span class="k">self</span>, event_loop: &amp;ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        <span class="k">let</span> Some(state) = &amp;<span class="k">mut</span> <span class="k">self</span>.state <span class="k">else</span> { <span class="k">return</span> };
        <span class="c">// true when the scene must be drawn again</span>
        <span class="k">let</span> changed = <span class="k">match</span> event {
            WindowEvent::CloseRequested =&gt; {
                event_loop.exit();
                <span class="s">false</span>
            }
            WindowEvent::RedrawRequested =&gt; {
                <span class="k">if</span> page_hidden() || desired_canvas_size().is_none() {
                    <span class="k">return</span>;
                }

                <span class="k">if</span> <span class="k">let</span> Some((w, h)) = desired_canvas_size()
                    &amp;&amp; (w, h) != (state.gpu.config.width, state.gpu.config.height)
                {
                    state.resize(w, h);
                }

                state.render();
                <span class="s">false</span>
            }
            WindowEvent::Resized(_) =&gt; <span class="s">true</span>,
            WindowEvent::KeyboardInput { event, .. } =&gt; {
                <span class="c">// first press only, and only while the canvas has focus</span>
                viewer_focused()
                    &amp;&amp; event.state == ElementState::Pressed
                    &amp;&amp; !event.repeat
                    &amp;&amp; <span class="k">self</span>.input.key(state, event.logical_key.as_ref())
            }
            other =&gt; <span class="k">self</span>.input.mouse(state, &amp;other),
        };

        <span class="k">if</span> changed {
            state.touch();
        }

        <span class="k">self</span>.request_if_needed();
    }
}</code></pre></div>
<p>Copy this part from the lesson folder to the path shown.</p>
<p><code>lessons/12/src/lib.rs</code> · copy the file, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// The page element with id \`canvas\`.</span>
#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">fn</span> viewer_canvas() -&gt; Option&lt;web_sys::HtmlCanvasElement&gt; {
    web_sys::window()?
        .document()?
        .get_element_by_id(&quot;<span class="s">canvas</span>&quot;)?
        .dyn_into()
        .ok()
}

<span class="c">/// True while the canvas has keyboard focus.</span>
#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">fn</span> viewer_focused() -&gt; bool {
    <span class="k">let</span> Some(window) = web_sys::window() <span class="k">else</span> {
        <span class="k">return</span> <span class="s">false</span>;
    };
    <span class="k">let</span> Some(document) = window.document() <span class="k">else</span> {
        <span class="k">return</span> <span class="s">false</span>;
    };

    <span class="k">match</span> document.active_element() {
        Some(element) =&gt; element.id() == &quot;<span class="s">canvas</span>&quot;,
        None =&gt; <span class="s">false</span>,
    }
}

<span class="c">/// True while the browser tab is hidden.</span>
#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">fn</span> page_hidden() -&gt; bool {
    <span class="k">let</span> Some(window) = web_sys::window() <span class="k">else</span> {
        <span class="k">return</span> <span class="s">true</span>;
    };

    <span class="k">match</span> window.document() {
        Some(document) =&gt; document.hidden(),
        None =&gt; <span class="s">true</span>,
    }
}

<span class="c">/// The canvas size in device pixels, \`None\` when zero.</span>
#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">fn</span> desired_canvas_size() -&gt; Option&lt;(u32, u32)&gt; {
    <span class="k">let</span> win = web_sys::window()?;
    <span class="k">let</span> dpr = win.device_pixel_ratio();
    <span class="k">let</span> canvas = viewer_canvas()?;
    <span class="k">let</span> w = (canvas.client_width() <span class="k">as</span> f64 * dpr).round() <span class="k">as</span> u32;
    <span class="k">let</span> h = (canvas.client_height() <span class="k">as</span> f64 * dpr).round() <span class="k">as</span> u32;
    (w &gt; <span class="s">0</span> &amp;&amp; h &gt; <span class="s">0</span>).then_some((w, h))
}

<span class="c">/// Browser entry point.</span>
#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
#[wasm_bindgen(start)]
<span class="k">pub</span> <span class="k">fn</span> run_web() -&gt; Result&lt;(), wasm_bindgen::JsValue&gt; {
    <span class="c">// panics print to the console</span>
    console_error_panic_hook::set_once();

    <span class="c">// the text-quality page runs its own code</span>
    <span class="k">if</span> <span class="k">let</span> Some(window) = web_sys::window()
        &amp;&amp; <span class="k">let</span> Some(document) = window.document()
        &amp;&amp; document.get_element_by_id(&quot;<span class="s">text-quality-canvas</span>&quot;).is_some()
    {
        <span class="k">return</span> Ok(());
    }

    <span class="k">if</span> <span class="k">let</span> Err(error) = App::run() {
        app::feedback::error(&amp;format!(&quot;<span class="s">Cannot start the viewer: </span>{<span class="s">error</span>}&quot;));
    }

    Ok(())
}</code></pre></div>
<h2 id="step-25-indexhtml">Step 25 · index.html<a class="anchor" href="#/course/12-picking#step-25-indexhtml" aria-label="Link to this section">#</a></h2>
<p>Copy this file from the lesson folder to the path shown.</p>
<p><code>lessons/12/index.html</code> · edit · copy the file</p>
<p>Replaces the 109 lines from <code>&lt;!doctype html&gt;</code> of <code>lessons/11/index.html</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>&lt;!DOCTYPE html&gt;
&lt;html lang=&quot;<span class="s">en</span>&quot;&gt;
  &lt;head&gt;
    &lt;meta charset=&quot;<span class="s">UTF-8</span>&quot;/&gt;
    &lt;meta name=&quot;<span class="s">viewport</span>&quot; content=&quot;<span class="s">width=device-width, initial-scale=1.0, viewport-fit=cover</span>&quot;/&gt;
    &lt;title&gt;Session Viewer&lt;/title&gt;
    &lt;style&gt;
      #viewer-error:not([hidden]) {
        position: fixed;
        inset: <span class="s">0</span>;
        background: #111e;
        color: #fff;
        padding: <span class="s">25</span><span class="k">vh</span> <span class="s">15</span><span class="k">vw</span>;
        font: <span class="s">16</span><span class="k">px</span>/<span class="s">1.5</span> system-ui;
        z-index: <span class="s">20</span>;
      }
      #viewer-error button {
        padding: <span class="s">8</span><span class="k">px</span> <span class="s">16</span><span class="k">px</span>;
        margin-top: <span class="s">16</span><span class="k">px</span>;
      }
    &lt;/style&gt;
    &lt;style&gt;
      * {
        margin: <span class="s">0</span>;
        padding: <span class="s">0</span>;
        box-sizing: border-box;
      }
      <span class="c">/* overscroll-behavior kills pull-to-refresh and the rubber band: a downward drag on the</span>
<span class="c">      canvas is an orbit, and a page that bounces under it swallows half the gesture. */</span>
      html, body {
        height: <span class="s">100</span><span class="k">%</span>;
        overscroll-behavior: none;
      }
      body {
        background: #000;
        overflow: hidden;
      }
      <span class="c">/* No focus ring. winit gives the canvas a tabindex so it can receive key</span>
<span class="c">      events, and the browser then rings the focused element - a black</span>
<span class="c">      rectangle around the whole viewport, drawn over the model. Both spellings</span>
<span class="c">      are needed: engines differ on which one they paint. */</span>
      <span class="c">/* touch-action: none is what actually delivers the gestures. winit does call preventDefault,</span>
<span class="c">      on \`pointerdown\` and on a non-passive \`touchstart\` (winit-0.30.13 web_sys/pointer.rs:115,</span>
<span class="c">      web_sys/canvas.rs:245), but that is a LATE veto: the compositor has already begun deciding</span>
<span class="c">      whether the gesture is a scroll, and where it decides yes the app gets a pointercancel and</span>
<span class="c">      nothing more. touch-action tells it up front, before any of that, and it is the only one</span>
<span class="c">      of the three that iOS Safari applies to page pinch-zoom.</span>
<span class="c">      The rest turns off the things a long press and a double tap mean to a DOCUMENT: the iOS</span>
<span class="c">      callout menu, text selection, the grey tap flash, and the legacy double-tap-to-zoom.</span>
<span class="c">      100dvh after 100vh: dvh follows a mobile toolbar sliding away, vh does not, and browsers</span>
<span class="c">      that never heard of dvh keep the line above. */</span>
      canvas {
        display: block;
        width: <span class="s">100</span><span class="k">vw</span>;
        height: <span class="s">100</span><span class="k">vh</span>;
        height: <span class="s">100</span><span class="k">dvh</span>;
        touch-action: none;
        -webkit-touch-callout: none;
        -webkit-user-select: none;
        user-select: none;
        -webkit-tap-highlight-color: transparent;
      }
      canvas:focus, canvas:focus-visible {
        outline: none;
      }
      <span class="c">/* The documentation corner: a black folded-corner triangle, always above the canvas. */</span>
      #viewer-docs {
        position: fixed;
        top: <span class="s">0</span>;
        right: <span class="s">0</span>;
        width: <span class="s">0</span>;
        height: <span class="s">0</span>;
        z-index: <span class="s">10</span>;
        display: block;
        border-top: <span class="s">40</span><span class="k">px</span> solid #111;
        border-left: <span class="s">40</span><span class="k">px</span> solid transparent;
        filter: drop-shadow(<span class="s">-1</span><span class="k">px</span> <span class="s">1</span><span class="k">px</span> <span class="s">0</span> #ffffff99);
        transition: border-top-width <span class="s">.25</span><span class="k">s</span> cubic-bezier(<span class="s">.2</span>, <span class="s">.8</span>, <span class="s">.3</span>, <span class="s">1.25</span>), border-left-width <span class="s">.25</span><span class="k">s</span> cubic-bezier(<span class="s">.2</span>, <span class="s">.8</span>, <span class="s">.3</span>, <span class="s">1.25</span>);
      }
      #viewer-docs:hover, #viewer-docs:focus-visible {
        border-top-width: <span class="s">52</span><span class="k">px</span>;
        border-left-width: <span class="s">52</span><span class="k">px</span>;
        outline: none;
      }
      #no-webgpu {
        position: fixed;
        inset: <span class="s">0</span>;
        background: #111;
        color: #eee;
        font: <span class="s">1</span><span class="k">rem</span> system-ui;
        text-align: center;
        padding-top: <span class="s">40</span><span class="k">vh</span>;
      }
    &lt;/style&gt;
  &lt;/head&gt;
  &lt;body&gt;
    &lt;link data-trunk rel=&quot;<span class="s">rust</span>&quot; data-target-name=&quot;<span class="s">session_viewer</span>&quot; data-wasm-opt=&quot;<span class="s">0</span>&quot;/&gt;
    <span class="c">&lt;!-- Trunk copies local fixtures into dist/pb. Named remote scenes resolve through</span>
<span class="c">    app/route.rs; their manifests and geometry are fetched from the configured data host. --&gt;</span>
    &lt;link data-trunk rel=&quot;<span class="s">copy-dir</span>&quot; href=&quot;<span class="s">assets/pb</span>&quot; data-target-path=&quot;<span class="s">pb</span>&quot;/&gt;
    <span class="c">&lt;!-- The ONE local manifest. Every other scene is opened from the R2 bucket with</span>
<span class="c">    ?scene=scenes/view_&lt;name&gt;.yaml and is never copied into dist. --&gt;</span>
    &lt;link data-trunk rel=&quot;<span class="s">copy-file</span>&quot; href=&quot;<span class="s">assets/view_local.yaml</span>&quot;/&gt;
    &lt;div id=&quot;<span class="s">viewer-error</span>&quot; role=&quot;<span class="s">alert</span>&quot; hidden&gt;
      &lt;p id=&quot;<span class="s">viewer-error-message</span>&quot;&gt;&lt;/p&gt;
      &lt;button type=&quot;<span class="s">button</span>&quot; onclick=&quot;<span class="s">location</span>.<span class="s">reload()</span>&quot;&gt;Reload viewer&lt;/button&gt;
    &lt;/div&gt;
    &lt;canvas id=&quot;<span class="s">canvas</span>&quot; tabindex=&quot;<span class="s">0</span>&quot;&gt;&lt;/canvas&gt;
    &lt;a id=&quot;<span class="s">viewer-docs</span>&quot; href=&quot;<span class="s">docs/</span>&quot; target=&quot;<span class="s">_blank</span>&quot; rel=&quot;<span class="s">noopener</span>&quot; title=&quot;<span class="s">Open the documentation</span>&quot; aria-label=&quot;<span class="s">Open the documentation</span>&quot;&gt;&lt;/a&gt;
    &lt;div id=&quot;<span class="s">viewer-status</span>&quot; role=&quot;<span class="s">status</span>&quot; aria-live=&quot;<span class="s">polite</span>&quot; style=&quot;<span class="s">position: fixed; bottom: 12px; left: 12px; color: #fff; background: #222b; font: 14px system-ui; padding: 4px 8px; pointer-events: none;</span>&quot;&gt;&lt;/div&gt;
    &lt;link data-trunk rel=&quot;<span class="s">copy-dir</span>&quot; href=&quot;<span class="s">assets/text</span>&quot; data-target-path=&quot;<span class="s">text</span>&quot;/&gt;
    &lt;link data-trunk rel=&quot;<span class="s">copy-file</span>&quot; href=&quot;<span class="s">assets/text-quality.html</span>&quot;/&gt;
    <span class="c">&lt;!-- Message telling that Webgpu is not available. --&gt;</span>
    &lt;script&gt;
      <span class="c">// These named registrations belong to this document's lifetime and are installed once.</span>
      <span class="k">function</span> preventContextMenu(event) {
        event.preventDefault();
      }

      <span class="k">function</span> installCanvasHandlers() {
        <span class="k">var</span> canvas = document.getElementById(&quot;<span class="s">canvas</span>&quot;);
        <span class="k">if</span> (canvas)
          canvas.addEventListener(&quot;<span class="s">contextmenu</span>&quot;, preventContextMenu);
      }

      document.addEventListener(&quot;<span class="s">DOMContentLoaded</span>&quot;, installCanvasHandlers, { once: <span class="s">true</span> });

      <span class="c">// Reload the scene in place when the embedding page asks for it, e.g. after</span>
      <span class="c">// an example was re-run and rewrote its .pb. Reloading this whole frame</span>
      <span class="c">// would restart WebGPU and reset the camera; this swaps only the geometry.</span>
      <span class="k">function</span> reloadSceneMessage(event) {
        <span class="k">if</span> (event.source !== window.parent &amp;&amp; event.origin !== window.location.origin)
          <span class="k">return</span>;
        <span class="k">var</span> data = event.data;

        <span class="k">if</span> (!data || data.type !== &quot;<span class="s">session-viewer:reload-scene</span>&quot;)
          <span class="k">return</span>;

        <span class="k">if</span> (window.wasmBindings &amp;&amp; window.wasmBindings.reload_scene) {
          window.wasmBindings.reload_scene(data.scene || <span class="s">undefined</span>);
        }
      }

      window.addEventListener(&quot;<span class="s">message</span>&quot;, reloadSceneMessage);

      <span class="k">if</span> (!navigator.gpu) {
        document.body.insertAdjacentHTML(&quot;<span class="s">beforeend</span>&quot;,
          '<span class="s">&lt;div id=&quot;no-webgpu&quot;&gt;WebGPU is unavailable. Open this viewer in a WebGPU-enabled browser over HTTPS or localhost; check the browser’s GPU settings.&lt;/div&gt;</span>');
      }</code></pre></div>
<h2 id="step-26-assetsview_localyaml">Step 26 · assets/view_local.yaml<a class="anchor" href="#/course/12-picking#step-26-assetsview_localyaml" aria-label="Link to this section">#</a></h2>
<p>Copy this file from the lesson folder to the path shown.</p>
<p><code>lessons/12/assets/view_local.yaml</code> · 3 lines · copy the file, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="s">name</span>: <span class="s">Selection fixture</span>
<span class="s">items</span>:
  - <span class="s">file</span>: <span class="s">pb/interaction.pb</span></code></pre></div>
<h2 id="step-27-srcfixturers">Step 27 · src/fixture.rs<a class="anchor" href="#/course/12-picking#step-27-srcfixturers" aria-label="Link to this section">#</a></h2>
<p>Remove this file; its replacement is now part of the rendering modules.</p>
<p>Delete <code>src/fixture.rs</code> (it exists in <code>lessons/11/</code>, not in <code>lessons/12/</code>).</p>
<h2 id="step-28-tests-that-need-a-headless-gpu">Step 28 · tests that need a headless GPU<a class="anchor" href="#/course/12-picking#step-28-tests-that-need-a-headless-gpu" aria-label="Link to this section">#</a></h2>
<p>Copy the tests that wait for this lesson: they open <code>Gpu::new_headless</code> or read <code>lane_shaders()</code>, both new here.</p>
<ul>
<li><code>lessons/12/src/engine/gpu/instance.rs</code> · copy the file</li>
<li><code>lessons/12/src/engine/gpu/text_outline.rs</code> · copy the file</li>
<li><code>lessons/12/src/engine/gpu/text_plane.rs</code> · copy the file</li>
<li><code>lessons/12/src/engine/gpu/text.rs</code> · copy the file</li>
</ul>
<p>Run <code>cargo check</code> and <code>cargo xtest</code> in <code>lessons/12/</code>.</p>
<h2 id="check">Check<a class="anchor" href="#/course/12-picking#check" aria-label="Link to this section">#</a></h2>
<p>Run <code>trunk serve</code> in <code>lessons/12/</code> and open <a href="http://127.0.0.1:8770/" target="_blank" rel="noopener">http://127.0.0.1:8770/</a>.</p>
<p>Expected: The seven-object fixture supports object and source-edge selection; status: <strong>7 objects</strong>.</p>
<p><img src="/session/docs/course/docs/screenshots/12.png" alt="Checkpoint 12: the seven-object interaction fixture in the production shell, nothing selected." loading="lazy" decoding="async"></p>
<p>If it fails:</p>
<ul>
<li>The highlight and GUID disagree: the row-to-identity map is wrong.</li>
<li>Orbiting selects an old object: a stale asynchronous pick is accepted.</li>
</ul>
<h2 id="what-changed">What changed<a class="anchor" href="#/course/12-picking#what-changed" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>lessons/12/src/
├── app/
│   ├── walk/
│   │   ├── bounds.rs
│   │   ├── brep.rs
│   │   ├── brep_edges.rs
│   │   ├── brep_orient.rs
│   │   ├── cloud.rs  +
│   │   ├── curves.rs
│   │   ├── encode.rs
│   │   ├── frames.rs  +
│   │   ├── mesh.rs
│   │   ├── mesh_ink.rs
│   │   ├── mesh_topology.rs
│   │   ├── mod.rs  ~
│   │   └── points.rs  +
│   ├── feedback.rs  +
│   ├── input.rs  +
│   ├── inspection.rs  +
│   ├── knobs.rs
│   ├── loader.rs  +
│   ├── mod.rs  ~
│   ├── route.rs  ~
│   ├── scene.rs  +
│   ├── selection.rs  +
│   ├── stream.rs  +
│   └── touch.rs  +
├── engine/
│   ├── gpu/
│   │   ├── arena.rs
│   │   ├── backdrop.rs
│   │   ├── buffers.rs
│   │   ├── cloud.rs
│   │   ├── device.rs  +
│   │   ├── frame.rs
│   │   ├── glyphs.rs
│   │   ├── instance.rs
│   │   ├── lod.rs
│   │   ├── mod.rs  ~
│   │   ├── objects.rs
│   │   ├── pick.rs  +
│   │   ├── present.rs  +
│   │   ├── render.rs  +
│   │   ├── segments.rs
│   │   ├── selection_outline.rs  +
│   │   ├── splat.rs
│   │   ├── targets.rs
│   │   ├── text.rs
│   │   ├── text_outline.rs
│   │   ├── text_plane.rs
│   │   ├── text_plate.rs
│   │   ├── upload.rs
│   │   └── view.rs
│   ├── pipelines/
│   │   ├── layouts.rs
│   │   └── mod.rs
│   ├── mod.rs  ~
│   ├── performance.rs
│   └── text.rs
├── shaders/
│   ├── background.wgsl
│   ├── glyph.wgsl
│   ├── grid.wgsl
│   ├── ink_visibility.wgsl
│   ├── normals.wgsl
│   ├── physical.wgsl
│   ├── ribbon.wgsl
│   ├── scene.wgsl
│   ├── selection_outline.wgsl  +
│   ├── sphere.wgsl
│   ├── splat.wgsl
│   ├── splat_resolve.wgsl
│   ├── text_outline.wgsl
│   ├── text_plane.wgsl
│   ├── text_plate.wgsl
│   └── triangle.wgsl
├── camera.rs
├── lib.rs  ~
└── state.rs  +</code></pre></div>
<p><code>+</code> new in this lesson · <code>~</code> changed in this lesson</p>
<p>Every file at this point: <code>lessons/12/</code>.</p>
<h2 id="next">Next<a class="anchor" href="#/course/12-picking#next" aria-label="Link to this section">#</a></h2>
<p><a href="#/course/13-controls">13 · Source controls</a></p>
<h2 id="expected-viewer-result">Expected viewer result<a class="anchor" href="#/course/12-picking#expected-viewer-result" aria-label="Link to this section">#</a></h2>
<p>Checkpoint 12: the seven-object interaction fixture in the production shell, nothing selected.</p>
<p><a href="/session/docs/course/docs/screenshots/12.png"><img src="/session/docs/course/docs/screenshots/12.png" alt="Full viewer result for 12 picking" loading="lazy" decoding="async"></a></p>
`,toc:[{level:2,id:"step-1-srcenginegpudevicers",text:"Step 1 · src/engine/gpu/device.rs"},{level:2,id:"step-2-srcenginegpupresentrs",text:"Step 2 · src/engine/gpu/present.rs"},{level:2,id:"step-3-srcenginegpurenderrs",text:"Step 3 · src/engine/gpu/render.rs"},{level:2,id:"step-4-srcappinputrs",text:"Step 4 · src/app/input.rs"},{level:2,id:"step-5-srcapptouchrs",text:"Step 5 · src/app/touch.rs"},{level:2,id:"step-6-srcappsceners",text:"Step 6 · src/app/scene.rs"},{level:2,id:"step-7-srcappselectionrs",text:"Step 7 · src/app/selection.rs"},{level:2,id:"step-8-srcappwalkcloudrs",text:"Step 8 · src/app/walk/cloud.rs"},{level:2,id:"step-9-srcappwalkframesrs",text:"Step 9 · src/app/walk/frames.rs"},{level:2,id:"step-10-srcappwalkpointsrs",text:"Step 10 · src/app/walk/points.rs"},{level:2,id:"step-11-srcappstreamrs",text:"Step 11 · src/app/stream.rs"},{level:2,id:"step-12-srcappfeedbackrs",text:"Step 12 · src/app/feedback.rs"},{level:2,id:"step-13-srcappinspectionrs",text:"Step 13 · src/app/inspection.rs"},{level:2,id:"step-14-srcapploaderrs",text:"Step 14 · src/app/loader.rs"},{level:2,id:"step-15-srcenginegpupickrs",text:"Step 15 · src/engine/gpu/pick.rs"},{level:2,id:"step-16-srcenginegpurenderrs",text:"Step 16 · src/engine/gpu/render.rs"},{level:2,id:"step-17-srcenginegpuselection_outliners",text:"Step 17 · src/engine/gpu/selection_outline.rs"},{level:2,id:"step-18-srcshadersselection_outlinewgsl",text:"Step 18 · src/shaders/selection_outline.wgsl"},{level:2,id:"step-19-srcstaters",text:"Step 19 · src/state.rs"},{level:2,id:"step-20-srcenginegpumodrs",text:"Step 20 · src/engine/gpu/mod.rs"},{level:2,id:"step-21-srcappmodrs",text:"Step 21 · src/app/mod.rs"},{level:2,id:"step-22-srcappwalkmodrs",text:"Step 22 · src/app/walk/mod.rs"},{level:2,id:"step-23-srcapprouters",text:"Step 23 · src/app/route.rs"},{level:2,id:"step-24-srclibrs",text:"Step 24 · src/lib.rs"},{level:2,id:"step-25-indexhtml",text:"Step 25 · index.html"},{level:2,id:"step-26-assetsview_localyaml",text:"Step 26 · assets/view_local.yaml"},{level:2,id:"step-27-srcfixturers",text:"Step 27 · src/fixture.rs"},{level:2,id:"step-28-tests-that-need-a-headless-gpu",text:"Step 28 · tests that need a headless GPU"},{level:2,id:"check",text:"Check"},{level:2,id:"what-changed",text:"What changed"},{level:2,id:"next",text:"Next"},{level:2,id:"expected-viewer-result",text:"Expected viewer result"}]};export{s as default};
