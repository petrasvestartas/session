const s={title:"25 · Draw a solid, readable gumball",html:`<h1 id="25-draw-a-solid-readable-gumball">25 · Draw a solid, readable gumball<a class="anchor" href="#/course/25-gumball#25-draw-a-solid-readable-gumball" aria-label="Link to this section">#</a></h1>
<p>The selected object shows solid colored gumball arrows, scale handles and rotation rings.</p>
<h2 id="step-1-srcappgizmors">Step 1 · src/app/gizmo.rs<a class="anchor" href="#/course/25-gumball#step-1-srcappgizmors" aria-label="Link to this section">#</a></h2>
<p>Expose the shared handle dimensions for the solid gumball.</p>
<p><code>lessons/25/src/app/gizmo.rs</code> · edit · type this</p>
<p>Replaces the line <code>pub const ARM: f64 = 72.0;</code> in <code>lessons/24/src/app/gizmo.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Arm length in CSS pixels.</span>
<span class="k">pub</span> <span class="k">const</span> ARM: f64 = <span class="s">96</span>.<span class="s">0</span>;</code></pre></div>
<h2 id="step-2-srcappinputrs">Step 2 · src/app/input.rs<a class="anchor" href="#/course/25-gumball#step-2-srcappinputrs" aria-label="Link to this section">#</a></h2>
<p>Update handle hover and bypass gumball grabs while Ctrl selects a component.</p>
<p><code>lessons/25/src/app/input.rs</code> · edit · type this</p>
<p>Replaces the line <code>dragging</code> in <code>lessons/24/src/app/input.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                dragging || state.hover_gizmo(position.x, position.y)</code></pre></div>
<p>Replaces the line <code>if state.begin_control_drag(self.last_cursor.0, self.last…</code> in <code>lessons/24/src/app/input.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                <span class="k">if</span> !<span class="k">self</span>.ctrl &amp;&amp; state.begin_control_drag(<span class="k">self</span>.last_cursor.<span class="s">0</span>, <span class="k">self</span>.last_cursor.<span class="s">1</span>) {
                    <span class="k">self</span>.control_drag = <span class="s">true</span>;
                    <span class="k">return</span> <span class="s">false</span>;
                }

                <span class="k">if</span> !<span class="k">self</span>.ctrl &amp;&amp; state.begin_gizmo(<span class="k">self</span>.last_cursor.<span class="s">0</span>, <span class="k">self</span>.last_cursor.<span class="s">1</span>) {</code></pre></div>
<h2 id="step-3-srcenginegpumodrs">Step 3 · src/engine/gpu/mod.rs<a class="anchor" href="#/course/25-gumball#step-3-srcenginegpumodrs" aria-label="Link to this section">#</a></h2>
<p>Add the new GPU resources, initialize them and include their allocations in the counters.</p>
<p><code>lessons/25/src/engine/gpu/mod.rs</code> · edit · type this</p>
<p>Added after the line <code>pub mod view;</code> in <code>lessons/24/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">mod</span> widget;
<span class="k">mod</span> widget_mesh;</code></pre></div>
<p>Replaces the 2 lines from <code>pub gizmo_arms: SegmentLane,</code> in <code>lessons/24/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> widget: widget::Widget, <span class="c">// gumball mesh, own depth</span></code></pre></div>
<p>Replaces the 2 lines from <code>+ self.gizmo_arms.allocated_bytes()</code> in <code>lessons/24/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            + <span class="k">self</span>.widget.allocated_bytes().<span class="s">0</span></code></pre></div>
<p>Added after the line <code>frame_textures</code> in <code>lessons/24/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                + <span class="k">self</span>.widget.allocated_bytes().<span class="s">1</span></code></pre></div>
<p>Replaces the 2 lines from <code>let gizmo_arms = SegmentLane::new(&amp;ctx, &amp;layouts, target);</code> in <code>lessons/24/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> widget = widget::Widget::new(&amp;ctx, target);</code></pre></div>
<p>Replaces the 2 lines from <code>gizmo_arms,</code> in <code>lessons/24/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            widget,</code></pre></div>
<p>Delete the 6 lines from <code>pub fn set_widget_rows(&amp;mut self, segments: &amp;segments::Se…</code> in <code>lessons/24/src/engine/gpu/mod.rs</code>.</p>
<p>Delete the 7 lines from <code>}</code> in <code>lessons/24/src/engine/gpu/mod.rs</code>.</p>
<p>Replaces the 2 lines from <code>self.gizmo_arms.retarget(&amp;self.ctx, &amp;self.layouts, target);</code> in <code>lessons/24/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">self</span>.widget.retarget(&amp;<span class="k">self</span>.ctx, target);</code></pre></div>
<p>Replaces the 2 lines from <code>self.gizmo_arms.reset();</code> in <code>lessons/24/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.widget.clear();</code></pre></div>
<p>Replaces the 2 lines from <code>self.gizmo_arms.release(&amp;self.ctx, &amp;self.layouts);</code> in <code>lessons/24/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.widget.clear();</code></pre></div>
<h2 id="step-4-srcenginegpupresentrs">Step 4 · src/engine/gpu/present.rs<a class="anchor" href="#/course/25-gumball#step-4-srcenginegpupresentrs" aria-label="Link to this section">#</a></h2>
<p>Submit the widget overlay with the scene frame.</p>
<p><code>lessons/25/src/engine/gpu/present.rs</code> · edit · type this</p>
<p>Added after the line <code>self.frame.write(&amp;self.ctx, input, &amp;cx);</code> in <code>lessons/24/src/engine/gpu/present.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.widget.prepare(
            &amp;<span class="k">self</span>.ctx,
            &amp;input.view_proj,
            <span class="k">self</span>.objects.anchor(),
            <span class="k">self</span>.frame.eye,
            size,
        );</code></pre></div>
<h2 id="step-5-srcenginegpurenderrs">Step 5 · src/engine/gpu/render.rs<a class="anchor" href="#/course/25-gumball#step-5-srcenginegpurenderrs" aria-label="Link to this section">#</a></h2>
<p>Place the new drawing work into the frame sequence.</p>
<p><code>lessons/25/src/engine/gpu/render.rs</code> · edit · type this</p>
<p>Added after the line <code>}</code> in <code>lessons/24/src/engine/gpu/render.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="c">// gumball on top, with its own depth</span>
        draws += <span class="k">self</span>.widget.draw(encoder, view, &amp;<span class="k">self</span>.targets);</code></pre></div>
<h2 id="step-6-srcenginegpuwidgetrs">Step 6 · src/engine/gpu/widget.rs<a class="anchor" href="#/course/25-gumball#step-6-srcenginegpuwidgetrs" aria-label="Link to this section">#</a></h2>
<p>Retain the gumball mesh and draw it into a bounded antialiased tile.</p>
<p><code>lessons/25/src/engine/gpu/widget.rs</code> · 390 lines · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> super::buffers::{GpuCtx, bind_group, uniform_buffer};
<span class="k">use</span> super::targets::{Attachment, Targets, TextureSpec};
<span class="k">use</span> super::widget_mesh;
<span class="k">use</span> <span class="k">crate</span>::engine::pipelines::{ColorWrite, DepthMode, PipelineDesc, Target, build, module};
<span class="k">use</span> session_rust::Xform;
<span class="k">use</span> wgpu::util::DeviceExt;

<span class="c">/// Draws the gumball into its own small texture, then over the frame.</span>
<span class="k">pub</span> <span class="k">struct</span> Widget {
    vertices: wgpu::Buffer, <span class="c">// the gumball mesh</span>
    count: u32, <span class="c">// vertices in it</span>
    uniform: wgpu::Buffer, <span class="c">// matrix, screen box, active handle</span>
    group: wgpu::BindGroup, <span class="c">// binds the uniform</span>
    layout: wgpu::BindGroupLayout, <span class="c">// one uniform</span>
    pipeline: wgpu::RenderPipeline, <span class="c">// mesh into the tile</span>
    tile: Option&lt;Tile&gt;, <span class="c">// textures the size of the gumball on screen</span>
    composite: wgpu::RenderPipeline, <span class="c">// tile over the frame</span>
    format: wgpu::TextureFormat, <span class="c">// canvas color format</span>
    texture_layout: wgpu::BindGroupLayout, <span class="c">// texture and sampler</span>
    visible: bool, <span class="c">// placed and on screen this frame</span>
    <span class="k">pub</span> placement: Option&lt;([f64; 3], f64)&gt;, <span class="c">// world position and scale; None = hidden</span>
    <span class="k">pub</span> active: f32, <span class="c">// handle under the cursor, -1 = none</span>
}

<span class="k">impl</span> Widget {
    <span class="c">/// Upload the mesh and build both pipelines.</span>
    <span class="k">pub</span> <span class="k">fn</span> new(ctx: &amp;GpuCtx, target: Target) -&gt; <span class="k">Self</span> {
        <span class="k">let</span> mesh = widget_mesh::vertices();
        <span class="k">let</span> vertices = ctx
            .device
            .create_buffer_init(&amp;wgpu::util::BufferInitDescriptor {
                label: Some(&quot;<span class="s">widget mesh</span>&quot;),
                contents: bytemuck::cast_slice(&amp;mesh),
                usage: wgpu::BufferUsages::VERTEX,
            });
        <span class="k">let</span> layout = ctx
            .device
            .create_bind_group_layout(&amp;wgpu::BindGroupLayoutDescriptor {
                label: Some(&quot;<span class="s">widget uniform</span>&quot;),
                entries: &amp;[wgpu::BindGroupLayoutEntry {
                    binding: <span class="s">0</span>,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: <span class="s">false</span>,
                        min_binding_size: wgpu::BufferSize::new(<span class="s">96</span>),
                    },
                    count: None,
                }],
            });
        <span class="k">let</span> uniform = uniform_buffer(&amp;ctx.device, &quot;<span class="s">widget placement</span>&quot;, &amp;[<span class="s">0</span>.<span class="s">0_f32</span>; <span class="s">24</span>]);
        <span class="k">let</span> group = bind_group(ctx, &amp;layout, &quot;<span class="s">widget</span>&quot;, &amp;[&amp;uniform]);
        <span class="k">let</span> texture_layout = texture_layout(ctx);
        <span class="k">let</span> pipeline = pipeline(ctx, &amp;layout, MESH_TARGET);
        <span class="k">let</span> composite = composite_pipeline(ctx, &amp;layout, &amp;texture_layout, target.format);
        <span class="k">Self</span> {
            vertices,
            count: mesh.len() <span class="k">as</span> u32,
            uniform,
            group,
            layout,
            pipeline,
            tile: None,
            composite,
            format: target.format,
            texture_layout,
            visible: <span class="s">false</span>,
            placement: None,
            active: -<span class="s">1</span>.<span class="s">0</span>,
        }
    }

    <span class="c">/// Hide the gumball and drop its textures.</span>
    <span class="k">pub</span> <span class="k">fn</span> clear(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">self</span>.placement = None;
        <span class="k">self</span>.tile = None;
        <span class="k">self</span>.visible = <span class="s">false</span>;
        <span class="k">self</span>.active = -<span class="s">1</span>.<span class="s">0</span>;
    }

    <span class="c">/// Rebuild the composite pipeline for a new canvas format.</span>
    <span class="k">pub</span> <span class="k">fn</span> retarget(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, target: Target) {
        <span class="k">if</span> <span class="k">self</span>.format != target.format {
            <span class="k">self</span>.composite =
                composite_pipeline(ctx, &amp;<span class="k">self</span>.layout, &amp;<span class="k">self</span>.texture_layout, target.format);
            <span class="k">self</span>.format = target.format;
        }
    }

    <span class="c">/// Bytes reserved on the GPU: (buffers, textures).</span>
    <span class="k">pub</span> <span class="k">fn</span> allocated_bytes(&amp;<span class="k">self</span>) -&gt; (u64, u64) {
        <span class="k">let</span> textures = <span class="k">self</span>.tile.as_ref().map_or(<span class="s">0</span>, |tile| {
            u64::from(tile.size.<span class="s">0</span>) * u64::from(tile.size.<span class="s">1</span>) * <span class="s">36</span>
        });
        (<span class="k">self</span>.vertices.size() + <span class="k">self</span>.uniform.size(), textures)
    }

    <span class="c">/// Place the gumball for this frame and write its uniform.</span>
    <span class="k">pub</span> <span class="k">fn</span> prepare(
        &amp;<span class="k">mut</span> <span class="k">self</span>,
        ctx: &amp;GpuCtx,
        mvp: &amp;Xform,
        anchor: [f64; <span class="s">3</span>],
        _eye: [f32; <span class="s">3</span>],
        size: (u32, u32),
    ) {
        <span class="k">self</span>.visible = <span class="s">false</span>;
        <span class="k">let</span> Some((origin, scale)) = <span class="k">self</span>.placement <span class="k">else</span> {
            <span class="k">return</span>;
        };
        <span class="k">let</span> position = std::array::from_fn::&lt;_, <span class="s">3</span>, _&gt;(|i| origin[i] - anchor[i]);
        <span class="c">// world matrix of the gumball</span>
        <span class="k">let</span> model = &amp;Xform::translation(position[<span class="s">0</span>], position[<span class="s">1</span>], position[<span class="s">2</span>])
            * &amp;Xform::scale_xyz(scale, scale, scale);
        <span class="k">let</span> matrix = mvp * &amp;model;
        <span class="c">// screen box it covers</span>
        <span class="k">let</span> Some(rect) = bounds(&amp;matrix.m, size) <span class="k">else</span> {
            <span class="k">return</span>;
        };
        <span class="c">// tile size: twice the box, in 64 px steps</span>
        <span class="k">let</span> extent = ((rect[<span class="s">2</span>] * <span class="s">2</span>.<span class="s">0</span>).ceil() <span class="k">as</span> u32, (rect[<span class="s">3</span>] * <span class="s">2</span>.<span class="s">0</span>).ceil() <span class="k">as</span> u32);
        <span class="k">let</span> extent = (
            extent.<span class="s">0</span>.div_ceil(<span class="s">64</span>).clamp(<span class="s">1</span>, <span class="s">16</span>) * <span class="s">64</span>,
            extent.<span class="s">1</span>.div_ceil(<span class="s">64</span>).clamp(<span class="s">1</span>, <span class="s">16</span>) * <span class="s">64</span>,
        );

        <span class="c">// new tile when the size changed</span>
        <span class="k">if</span> <span class="k">self</span>.tile.as_ref().is_none_or(|tile| tile.size != extent) {
            <span class="k">self</span>.tile = None;
            <span class="k">self</span>.tile = Some(Tile::new(ctx, &amp;<span class="k">self</span>.texture_layout, extent));
        }

        <span class="c">// matrix that maps the box onto the tile</span>
        <span class="k">let</span> <span class="k">mut</span> uniform = [<span class="s">0</span>.<span class="s">0_f32</span>; <span class="s">24</span>];
        <span class="k">let</span> [x, y, width, height] = rect;
        <span class="k">let</span> sx = size.<span class="s">0</span> <span class="k">as</span> f64 / width;
        <span class="k">let</span> sy = size.<span class="s">1</span> <span class="k">as</span> f64 / height;
        <span class="k">let</span> tx = (size.<span class="s">0</span> <span class="k">as</span> f64 - <span class="s">2</span>.<span class="s">0</span> * x - width) / width;
        <span class="k">let</span> ty = (<span class="s">2</span>.<span class="s">0</span> * y + height - size.<span class="s">1</span> <span class="k">as</span> f64) / height;

        <span class="k">for</span> column <span class="k">in</span> <span class="s">0</span>..<span class="s">4</span> {
            <span class="k">let</span> at = column * <span class="s">4</span>;
            uniform[at] = (sx * matrix.m[at] + tx * matrix.m[at + <span class="s">3</span>]) <span class="k">as</span> f32;
            uniform[at + <span class="s">1</span>] = (sy * matrix.m[at + <span class="s">1</span>] + ty * matrix.m[at + <span class="s">3</span>]) <span class="k">as</span> f32;
            uniform[at + <span class="s">2</span>] = matrix.m[at + <span class="s">2</span>] <span class="k">as</span> f32;
            uniform[at + <span class="s">3</span>] = matrix.m[at + <span class="s">3</span>] <span class="k">as</span> f32;
        }

        <span class="c">// where the tile lands on the canvas</span>
        uniform[<span class="s">16</span>] = (<span class="s">2</span>.<span class="s">0</span> * x / size.<span class="s">0</span> <span class="k">as</span> f64 - <span class="s">1</span>.<span class="s">0</span>) <span class="k">as</span> f32;
        uniform[<span class="s">17</span>] = (<span class="s">1</span>.<span class="s">0</span> - <span class="s">2</span>.<span class="s">0</span> * y / size.<span class="s">1</span> <span class="k">as</span> f64) <span class="k">as</span> f32;
        uniform[<span class="s">18</span>] = (<span class="s">2</span>.<span class="s">0</span> * width / size.<span class="s">0</span> <span class="k">as</span> f64) <span class="k">as</span> f32;
        uniform[<span class="s">19</span>] = (-<span class="s">2</span>.<span class="s">0</span> * height / size.<span class="s">1</span> <span class="k">as</span> f64) <span class="k">as</span> f32;
        uniform[<span class="s">20</span>] = <span class="k">self</span>.active;
        ctx.queue
            .write_buffer(&amp;<span class="k">self</span>.uniform, <span class="s">0</span>, bytemuck::bytes_of(&amp;uniform));
        <span class="k">self</span>.visible = <span class="s">true</span>;
    }

    <span class="c">/// Draw the mesh into the tile, then the tile over the frame.</span>
    <span class="k">pub</span> <span class="k">fn</span> draw(
        &amp;<span class="k">self</span>,
        encoder: &amp;<span class="k">mut</span> wgpu::CommandEncoder,
        view: &amp;wgpu::TextureView,
        _targets: &amp;Targets,
    ) -&gt; u32 {
        <span class="k">let</span> Some(tile) = <span class="k">self</span>.tile.as_ref().filter(|_| <span class="k">self</span>.visible) <span class="k">else</span> {
            <span class="k">return</span> <span class="s">0</span>;
        };
        <span class="k">let</span> <span class="k">mut</span> pass = encoder.begin_render_pass(&amp;wgpu::RenderPassDescriptor {
            label: Some(&quot;<span class="s">widget overlay</span>&quot;),
            color_attachments: &amp;[Some(wgpu::RenderPassColorAttachment {
                view: &amp;tile.color,
                resolve_target: Some(&amp;tile.resolved),
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &amp;tile.depth,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(<span class="s">0</span>.<span class="s">0</span>),
                    store: wgpu::StoreOp::Discard,
                }),
                stencil_ops: None,
            }),
            ..Default::default()
        });
        pass.set_pipeline(&amp;<span class="k">self</span>.pipeline);
        pass.set_bind_group(<span class="s">0</span>, &amp;<span class="k">self</span>.group, &amp;[]);
        pass.set_vertex_buffer(<span class="s">0</span>, <span class="k">self</span>.vertices.slice(..));
        pass.draw(<span class="s">0</span>..<span class="k">self</span>.count, <span class="s">0</span>..<span class="s">1</span>);
        drop(pass);
        <span class="k">let</span> <span class="k">mut</span> pass = encoder.begin_render_pass(&amp;wgpu::RenderPassDescriptor {
            label: Some(&quot;<span class="s">widget composite</span>&quot;),
            color_attachments: &amp;[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            ..Default::default()
        });
        pass.set_pipeline(&amp;<span class="k">self</span>.composite);
        pass.set_bind_group(<span class="s">0</span>, &amp;<span class="k">self</span>.group, &amp;[]);
        pass.set_bind_group(<span class="s">1</span>, &amp;tile.group, &amp;[]);
        pass.draw(<span class="s">0</span>..<span class="s">6</span>, <span class="s">0</span>..<span class="s">1</span>);
        <span class="s">2</span>
    }
}

<span class="c">/// Free the buffers.</span>
<span class="k">impl</span> Drop <span class="k">for</span> Widget {
    <span class="c">/// Remove the DOM listener.</span>
    <span class="k">fn</span> drop(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">self</span>.vertices.destroy();
        <span class="k">self</span>.uniform.destroy();
    }
}

<span class="c">/// The mesh pipeline: lit, depth-tested, 4x MSAA.</span>
<span class="k">fn</span> pipeline(ctx: &amp;GpuCtx, layout: &amp;wgpu::BindGroupLayout, target: Target) -&gt; wgpu::RenderPipeline {
    <span class="k">let</span> shader = module(
        &amp;ctx.device,
        &quot;<span class="s">widget</span>&quot;,
        include_str!(&quot;<span class="s">../../shaders/widget.wgsl</span>&quot;),
    );
    <span class="k">let</span> vertex = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::&lt;widget_mesh::Vertex&gt;() <span class="k">as</span> u64,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &amp;wgpu::vertex_attr_array![<span class="s">0</span> =&gt; Float32x3, <span class="s">1</span> =&gt; Uint32, <span class="s">2</span> =&gt; Uint32],
    };
    build(
        &amp;ctx.device,
        target,
        &amp;PipelineDesc::new(
            &amp;shader,
            &amp;[layout],
            &amp;[vertex],
            wgpu::PrimitiveTopology::TriangleList,
        ),
    )
}

<span class="c">/// The tile's format: RGBA at 4x MSAA.</span>
<span class="k">const</span> MESH_TARGET: Target = Target {
    format: wgpu::TextureFormat::Rgba8Unorm,
    samples: <span class="s">4</span>,
};

<span class="c">/// The gumball's own textures.</span>
<span class="k">struct</span> Tile {
    size: (u32, u32), <span class="c">// texture size, px</span>
    color: Attachment, <span class="c">// color at 4x</span>
    depth: Attachment, <span class="c">// depth at 4x</span>
    resolved: Attachment, <span class="c">// color at 1x, sampled by the composite</span>
    group: wgpu::BindGroup, <span class="c">// resolved texture and sampler</span>
}

<span class="k">impl</span> Tile {
    <span class="c">/// Create the three textures and their bind group.</span>
    <span class="k">fn</span> new(ctx: &amp;GpuCtx, layout: &amp;wgpu::BindGroupLayout, size: (u32, u32)) -&gt; <span class="k">Self</span> {
        <span class="k">let</span> attachment = |label, format, samples, usage| {
            Attachment::new(
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
        <span class="k">let</span> color = attachment(
            &quot;<span class="s">widget color</span>&quot;,
            MESH_TARGET.format,
            <span class="s">4</span>,
            wgpu::TextureUsages::RENDER_ATTACHMENT,
        );
        <span class="k">let</span> depth = attachment(
            &quot;<span class="s">widget depth</span>&quot;,
            wgpu::TextureFormat::Depth32Float,
            <span class="s">4</span>,
            wgpu::TextureUsages::RENDER_ATTACHMENT,
        );
        <span class="k">let</span> resolved = attachment(
            &quot;<span class="s">widget resolved</span>&quot;,
            MESH_TARGET.format,
            <span class="s">1</span>,
            wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        );
        <span class="k">let</span> sampler = ctx.device.create_sampler(&amp;wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        <span class="k">let</span> group = ctx.device.create_bind_group(&amp;wgpu::BindGroupDescriptor {
            label: Some(&quot;<span class="s">widget image</span>&quot;),
            layout,
            entries: &amp;[
                wgpu::BindGroupEntry {
                    binding: <span class="s">0</span>,
                    resource: wgpu::BindingResource::TextureView(&amp;resolved),
                },
                wgpu::BindGroupEntry {
                    binding: <span class="s">1</span>,
                    resource: wgpu::BindingResource::Sampler(&amp;sampler),
                },
            ],
        });
        <span class="k">Self</span> {
            size,
            color,
            depth,
            resolved,
            group,
        }
    }
}

<span class="c">/// Layout for one texture and one sampler.</span>
<span class="k">fn</span> texture_layout(ctx: &amp;GpuCtx) -&gt; wgpu::BindGroupLayout {
    ctx.device
        .create_bind_group_layout(&amp;wgpu::BindGroupLayoutDescriptor {
            label: Some(&quot;<span class="s">widget image</span>&quot;),
            entries: &amp;[
                wgpu::BindGroupLayoutEntry {
                    binding: <span class="s">0</span>,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    count: None,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: <span class="s">true</span> },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: <span class="s">false</span>,
                    },
                },
                wgpu::BindGroupLayoutEntry {
                    binding: <span class="s">1</span>,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    count: None,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                },
            ],
        })
}

<span class="c">/// The composite pipeline: the tile blended over the frame.</span>
<span class="k">fn</span> composite_pipeline(
    ctx: &amp;GpuCtx,
    layout: &amp;wgpu::BindGroupLayout,
    texture: &amp;wgpu::BindGroupLayout,
    format: wgpu::TextureFormat,
) -&gt; wgpu::RenderPipeline {
    <span class="k">let</span> shader = module(
        &amp;ctx.device,
        &quot;<span class="s">widget composite</span>&quot;,
        include_str!(&quot;<span class="s">../../shaders/widget.wgsl</span>&quot;),
    );
    <span class="k">let</span> groups = [layout, texture];
    <span class="k">let</span> desc = PipelineDesc::new(&amp;shader, &amp;groups, &amp;[], wgpu::PrimitiveTopology::TriangleList)
        .vertex(&quot;<span class="s">vs_composite</span>&quot;)
        .with(&quot;<span class="s">widget composite</span>&quot;, &quot;<span class="s">fs_composite</span>&quot;)
        .depth(DepthMode::Detached)
        .color(ColorWrite::Blended);
    build(&amp;ctx.device, Target { format, samples: <span class="s">1</span> }, &amp;desc)
}

<span class="c">/// Screen box of the gumball: x, y, width, height; None when behind the camera.</span>
<span class="k">fn</span> bounds(m: &amp;[f64; <span class="s">16</span>], size: (u32, u32)) -&gt; Option&lt;[f64; 4]&gt; {
    <span class="k">let</span> radius = <span class="k">crate</span>::app::gizmo::ARM + <span class="s">2</span>.<span class="s">0</span>;
    <span class="k">let</span> <span class="k">mut</span> min = [f64::INFINITY; <span class="s">2</span>];
    <span class="k">let</span> <span class="k">mut</span> max = [f64::NEG_INFINITY; <span class="s">2</span>];

    <span class="c">// project the eight corners of its bounding cube</span>
    <span class="k">for</span> corner <span class="k">in</span> <span class="s">0</span>..<span class="s">8</span> {
        <span class="k">let</span> p = std::array::from_fn::&lt;_, <span class="s">3</span>, _&gt;(|i| {
            <span class="k">if</span> corner &amp; (<span class="s">1</span> &lt;&lt; i) == <span class="s">0</span> {
                -radius
            } <span class="k">else</span> {
                radius
            }
        });
        <span class="k">let</span> c = std::array::from_fn::&lt;_, <span class="s">4</span>, _&gt;(|i| {
            m[i] * p[<span class="s">0</span>] + m[<span class="s">4</span> + i] * p[<span class="s">1</span>] + m[<span class="s">8</span> + i] * p[<span class="s">2</span>] + m[<span class="s">12</span> + i]
        });

        <span class="k">if</span> c[<span class="s">3</span>] &lt;= <span class="s">0</span>.<span class="s">0</span> {
            <span class="k">return</span> None;
        }

        <span class="k">let</span> screen = [
            (c[<span class="s">0</span>] / c[<span class="s">3</span>] * <span class="s">0</span>.<span class="s">5</span> + <span class="s">0</span>.<span class="s">5</span>) * size.<span class="s">0</span> <span class="k">as</span> f64,
            (<span class="s">0</span>.<span class="s">5</span> - c[<span class="s">1</span>] / c[<span class="s">3</span>] * <span class="s">0</span>.<span class="s">5</span>) * size.<span class="s">1</span> <span class="k">as</span> f64,
        ];

        <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..<span class="s">2</span> {
            min[i] = min[i].min(screen[i]);
            max[i] = max[i].max(screen[i]);
        }
    }

    <span class="k">let</span> x = (min[<span class="s">0</span>] - <span class="s">2</span>.<span class="s">0</span>).floor().max(<span class="s">0</span>.<span class="s">0</span>);
    <span class="k">let</span> y = (min[<span class="s">1</span>] - <span class="s">2</span>.<span class="s">0</span>).floor().max(<span class="s">0</span>.<span class="s">0</span>);
    <span class="k">let</span> width = (max[<span class="s">0</span>] + <span class="s">2</span>.<span class="s">0</span>).ceil().min(size.<span class="s">0</span> <span class="k">as</span> f64) - x;
    <span class="k">let</span> height = (max[<span class="s">1</span>] + <span class="s">2</span>.<span class="s">0</span>).ceil().min(size.<span class="s">1</span> <span class="k">as</span> f64) - y;
    (width &gt; <span class="s">0</span>.<span class="s">0</span> &amp;&amp; height &gt; <span class="s">0</span>.<span class="s">0</span>).then_some([x, y, width, height])
}</code></pre></div>
<h2 id="step-7-srcenginegpuwidget_meshrs">Step 7 · src/engine/gpu/widget_mesh.rs<a class="anchor" href="#/course/25-gumball#step-7-srcenginegpuwidget_meshrs" aria-label="Link to this section">#</a></h2>
<p>Build reusable triangle meshes for the gumball arrows, scale handles and rotation rings.</p>
<p><code>lessons/25/src/engine/gpu/widget_mesh.rs</code> · 124 lines · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> <span class="k">crate</span>::app::gizmo::{ARM, BALL_AT, HUB};
<span class="k">use</span> std::f32::consts::{FRAC_PI_2, PI, TAU};

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
<span class="c">/// One gumball mesh vertex.</span>
<span class="k">pub</span> <span class="k">struct</span> Vertex {
    position: [f32; <span class="s">3</span>], <span class="c">// CSS pixels from the gumball center</span>
    color: u32, <span class="c">// packed rgba</span>
    handle: u32, <span class="c">// which of the ten handles it belongs to</span>
}

<span class="c">/// Axis colors: red x, green y, blue z.</span>
<span class="k">const</span> COLORS: [u32; <span class="s">3</span>] = [<span class="s">0xff2424e8</span>, <span class="s">0xff30b820</span>, <span class="s">0xffef6628</span>];

<span class="c">/// Segments around a lathed shape.</span>
<span class="k">const</span> SIDES: usize = <span class="s">24</span>;

<span class="c">/// Arrow shaft radius, CSS px.</span>
<span class="k">const</span> SHAFT: f32 = <span class="s">2</span>.<span class="s">2</span>;

<span class="c">/// Arrow tip length, CSS px.</span>
<span class="k">const</span> TIP: f32 = <span class="s">14</span>.<span class="s">0</span>;

<span class="c">/// The gumball mesh: three arrows, three rings, three balls, one hub.</span>
<span class="k">pub</span> <span class="k">fn</span> vertices() -&gt; Vec&lt;Vertex&gt; {
    <span class="k">let</span> <span class="k">mut</span> out = Vec::with_capacity(<span class="s">18_576</span>);

    <span class="k">for</span> (axis, color) <span class="k">in</span> COLORS.into_iter().enumerate() {
        <span class="k">let</span> arm = ARM <span class="k">as</span> f32;
        <span class="c">// arrow shaft, handle 0-2</span>
        <span class="k">let</span> shaft = [(HUB <span class="k">as</span> f32, SHAFT), (arm - TIP, SHAFT)];
        lathe(&amp;<span class="k">mut</span> out, axis, color, axis <span class="k">as</span> u32, &amp;shaft);
        <span class="c">// arrow tip</span>
        <span class="k">let</span> cone = [(arm - TIP, <span class="s">0</span>.<span class="s">0</span>), (arm - TIP, <span class="s">5</span>.<span class="s">5</span>), (arm, <span class="s">0</span>.<span class="s">0</span>)];
        lathe(&amp;<span class="k">mut</span> out, axis, color, axis <span class="k">as</span> u32, &amp;cone);
        <span class="c">// scale ball, handle 6-8</span>
        sphere(&amp;<span class="k">mut</span> out, axis, BALL_AT <span class="k">as</span> f32, <span class="s">5</span>.<span class="s">0</span>, color, axis <span class="k">as</span> u32 + <span class="s">6</span>);
        <span class="c">// rotation ring, handle 3-5</span>
        surface(&amp;<span class="k">mut</span> out, <span class="s">48</span>, <span class="s">12</span>, color, axis <span class="k">as</span> u32 + <span class="s">3</span>, |s, t| {
            <span class="k">let</span> angle = s * FRAC_PI_2;
            <span class="k">let</span> tube = t * TAU;
            <span class="k">let</span> radius = arm + <span class="s">1</span>.<span class="s">8</span> * tube.cos();
            <span class="k">let</span> position = [
                <span class="s">1</span>.<span class="s">8</span> * tube.sin(),
                -angle.cos() * radius,
                -angle.sin() * radius,
            ];
            orient(position, axis)
        });
    }

    <span class="c">// grey hub, handle 9</span>
    sphere(&amp;<span class="k">mut</span> out, <span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, HUB <span class="k">as</span> f32, <span class="s">0xffd8d8d8</span>, <span class="s">9</span>);
    out
}

<span class="c">/// Rotate a shape built along x onto \`axis\`.</span>
<span class="k">fn</span> orient(p: [f32; <span class="s">3</span>], axis: usize) -&gt; [f32; <span class="s">3</span>] {
    <span class="k">match</span> axis {
        <span class="s">0</span> =&gt; p,
        <span class="s">1</span> =&gt; [p[<span class="s">2</span>], p[<span class="s">0</span>], p[<span class="s">1</span>]],
        _ =&gt; [p[<span class="s">1</span>], p[<span class="s">2</span>], p[<span class="s">0</span>]],
    }
}

<span class="c">/// Spin a (distance, radius) profile around \`axis\`.</span>
<span class="k">fn</span> lathe(out: &amp;<span class="k">mut</span> Vec&lt;Vertex&gt;, axis: usize, color: u32, handle: u32, profile: &amp;[(f32, f32)]) {
    <span class="k">for</span> pair <span class="k">in</span> profile.windows(<span class="s">2</span>) {
        <span class="k">let</span> [(x0, r0), (x1, r1)] = [pair[<span class="s">0</span>], pair[<span class="s">1</span>]];
        surface(out, <span class="s">1</span>, SIDES, color, handle, |s, t| {
            <span class="k">let</span> (sin, cos) = (t * TAU).sin_cos();
            <span class="k">let</span> r = r0 + (r1 - r0) * s;
            <span class="k">let</span> position = [x0 + (x1 - x0) * s, r * cos, r * sin];
            orient(position, axis)
        });
    }
}

<span class="c">/// A sphere at distance \`at\` along \`axis\`.</span>
<span class="k">fn</span> sphere(out: &amp;<span class="k">mut</span> Vec&lt;Vertex&gt;, axis: usize, at: f32, radius: f32, color: u32, handle: u32) {
    surface(out, <span class="s">12</span>, SIDES, color, handle, |s, t| {
        <span class="k">let</span> (sin, cos) = (s * PI).sin_cos();
        <span class="k">let</span> (v, u) = (t * TAU).sin_cos();
        <span class="k">let</span> position = [at + radius * cos, radius * sin * u, radius * sin * v];
        orient(position, axis)
    });
}

<span class="c">/// Sample a parametric surface into triangles.</span>
<span class="k">fn</span> surface(
    out: &amp;<span class="k">mut</span> Vec&lt;Vertex&gt;,
    rows: usize,
    columns: usize,
    color: u32,
    handle: u32,
    sample: <span class="k">impl</span> Fn(f32, f32) -&gt; [f32; <span class="s">3</span>],
) {
    <span class="k">for</span> row <span class="k">in</span> <span class="s">0</span>..rows {
        <span class="k">for</span> column <span class="k">in</span> <span class="s">0</span>..columns {
            <span class="k">for</span> (i, j) <span class="k">in</span> [(<span class="s">0</span>, <span class="s">0</span>), (<span class="s">1</span>, <span class="s">0</span>), (<span class="s">1</span>, <span class="s">1</span>), (<span class="s">0</span>, <span class="s">0</span>), (<span class="s">1</span>, <span class="s">1</span>), (<span class="s">0</span>, <span class="s">1</span>)] {
                <span class="k">let</span> position = sample(
                    (row + i) <span class="k">as</span> f32 / rows <span class="k">as</span> f32,
                    (column + j) <span class="k">as</span> f32 / columns <span class="k">as</span> f32,
                );
                out.push(Vertex {
                    position,
                    color,
                    handle,
                });
            }
        }
    }
}

#[cfg(test)]
<span class="k">mod</span> tests {
    <span class="k">use</span> super::*;

    #[test]
    <span class="c">/// Every vertex is finite and every handle is present.</span>
    <span class="k">fn</span> mesh_is_bounded_finite_and_covers_all_handles() {
        <span class="k">let</span> mesh = vertices();
        assert!(mesh.len() &lt; <span class="s">20_000</span>);
        <span class="k">let</span> <span class="k">mut</span> handles = [<span class="s">false</span>; <span class="s">10</span>];

        <span class="k">for</span> vertex <span class="k">in</span> mesh {
            handles[vertex.handle <span class="k">as</span> usize] = <span class="s">true</span>;
            assert!(
                vertex
                    .position
                    .iter()
                    .all(|v| v.is_finite() &amp;&amp; v.abs() &lt;= ARM <span class="k">as</span> f32 + <span class="s">2</span>.<span class="s">0</span>)
            );
        }

        assert!(handles.into_iter().all(|v| v));
    }
}</code></pre></div>
<h2 id="step-8-srcshaderswidgetwgsl">Step 8 · src/shaders/widget.wgsl<a class="anchor" href="#/course/25-gumball#step-8-srcshaderswidgetwgsl" aria-label="Link to this section">#</a></h2>
<p>Draw each handle with its own unlit color, then composite the tile over the scene.</p>
<p><code>lessons/25/src/shaders/widget.wgsl</code> · 54 lines · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// Gumball uniform, 96 bytes; matches the Rust array.</span>
<span class="k">struct</span> Widget {
    mvp: <span class="k">mat4x4</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // gumball to tile clip space</span>
    rect: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // where the tile lands on the canvas: x, y, w, h</span>
    settings: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // x: active handle, -1 = none</span>
};

@group(<span class="s">0</span>) @binding(<span class="s">0</span>) <span class="k">var</span>&lt;<span class="k">uniform</span>&gt; widget: Widget;<span class="c"> // placement</span>
@group(<span class="s">1</span>) @binding(<span class="s">0</span>) <span class="k">var</span> picture: texture_2d&lt;<span class="k">f32</span>&gt;;<span class="c"> // the drawn gumball tile</span>
@group(<span class="s">1</span>) @binding(<span class="s">1</span>) <span class="k">var</span> picture_sampler: sampler;<span class="c"> // linear sampler</span>

<span class="c">// What the mesh vertex shader hands the fragment shader.</span>
<span class="k">struct</span> Varying {
    @builtin(position) position: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // clip position</span>
    @location(<span class="s">0</span>) color: <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // rgb</span>
};

@vertex
<span class="c">// Place a mesh vertex; the active handle turns orange.</span>
<span class="k">fn</span> vs_main(@location(<span class="s">0</span>) position: <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;, @location(<span class="s">1</span>) color: <span class="k">u32</span>,
           @location(<span class="s">2</span>) <span class="k">handle</span>: <span class="k">u32</span>) -&gt; Varying {
    <span class="k">var</span> out: Varying;
    out.position = widget.mvp * <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(position, <span class="s">1.0</span>);
    out.color = unpack4x8unorm(color).rgb;

    <span class="k">if</span> f32(<span class="k">handle</span>) == widget.settings.x {
        out.color = <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.95</span>, <span class="s">0.65</span>, <span class="s">0.08</span>);
    }

    <span class="k">return</span> out;
}

@fragment
<span class="c">// Flat color.</span>
<span class="k">fn</span> fs_main(in: Varying) -&gt; @location(<span class="s">0</span>) <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt; {
    <span class="k">return</span> <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(in.color, <span class="s">1.0</span>);
}

<span class="c">// What the composite vertex shader hands the fragment shader.</span>
<span class="k">struct</span> Composite {
    @builtin(position) position: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // clip position</span>
    @location(<span class="s">0</span>) uv: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // texture coordinate</span>
};

@vertex
<span class="c">// One corner of the tile's quad on the canvas.</span>
<span class="k">fn</span> vs_composite(@builtin(vertex_index) index: <span class="k">u32</span>) -&gt; Composite {
    <span class="k">let</span> corners = <span class="k">array</span>&lt;<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;, <span class="s">6</span>&gt;(vec2(<span class="s">0</span>., <span class="s">0</span>.), vec2(<span class="s">1</span>., <span class="s">0</span>.), vec2(<span class="s">1</span>., <span class="s">1</span>.),
                                     vec2(<span class="s">0</span>., <span class="s">0</span>.), vec2(<span class="s">1</span>., <span class="s">1</span>.), vec2(<span class="s">0</span>., <span class="s">1</span>.));
    <span class="k">var</span> out: Composite;
    out.uv = corners[index];
    out.position = <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(widget.rect.xy + out.uv * widget.rect.zw, <span class="s">0</span>., <span class="s">1</span>.);
    <span class="k">return</span> out;
}

@fragment
<span class="c">// Blend the tile over the frame.</span>
<span class="k">fn</span> fs_composite(in: Composite) -&gt; @location(<span class="s">0</span>) <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt; {
    <span class="k">let</span> pixel = textureSample(picture, picture_sampler, in.uv);
    <span class="k">return</span> <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(pixel.rgb / max(pixel.a, <span class="s">0.00001</span>), pixel.a);
}</code></pre></div>
<h2 id="step-9-srcstaters">Step 9 · src/state.rs<a class="anchor" href="#/course/25-gumball#step-9-srcstaters" aria-label="Link to this section">#</a></h2>
<p>Clear widget hover along with selection state.</p>
<p><code>lessons/25/src/state.rs</code> · edit · type this</p>
<p>Added after the line <code>pub fn render(&amp;mut self) {</code> in <code>lessons/24/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.upload_gizmo();</code></pre></div>
<h2 id="step-10-srcstateeditrs">Step 10 · src/state/edit.rs<a class="anchor" href="#/course/25-gumball#step-10-srcstateeditrs" aria-label="Link to this section">#</a></h2>
<p>Replace the old line widget upload with the solid widget and update its hover state.</p>
<p><code>lessons/25/src/state/edit.rs</code> · edit · type this</p>
<p>Replaces the line <code>use crate::app::gizmo::{ARM, Axis, BALL_AT, Drag, Gizmo, …</code> in <code>lessons/24/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> <span class="k">crate</span>::app::gizmo::{Axis, Drag, Gizmo, Handle};</code></pre></div>
<p>Delete the four <code>use</code> lines from <code>use crate::app::walk::encode::FACING_UNKNOWN;</code> in <code>lessons/24/src/state/edit.rs</code>.</p>
<p>Added after the line <code>fn world_per_px(&amp;self) -&gt; f64 {</code> in <code>lessons/24/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="c">// at the gizmo: from its projected depth</span>
        <span class="k">if</span> <span class="k">let</span> Some(gizmo) = <span class="k">self</span>.gizmo.as_ref() {
            <span class="k">let</span> anchor = <span class="k">self</span>.camera.origin();
            <span class="k">let</span> m = <span class="k">self</span>.camera.view_proj_anchored(<span class="k">self</span>.aspect(), &amp;anchor).m;
            <span class="k">let</span> p = [
                gizmo.origin[<span class="s">0</span>] - anchor[<span class="s">0</span>],
                gizmo.origin[<span class="s">1</span>] - anchor[<span class="s">1</span>],
                gizmo.origin[<span class="s">2</span>] - anchor[<span class="s">2</span>],
            ];
            <span class="k">let</span> w = m[<span class="s">3</span>] * p[<span class="s">0</span>] + m[<span class="s">7</span>] * p[<span class="s">1</span>] + m[<span class="s">11</span>] * p[<span class="s">2</span>] + m[<span class="s">15</span>]; <span class="c">// clip w: the depth</span>
            <span class="k">let</span> vertical = (m[<span class="s">1</span>] * m[<span class="s">1</span>] + m[<span class="s">5</span>] * m[<span class="s">5</span>] + m[<span class="s">9</span>] * m[<span class="s">9</span>]).sqrt(); <span class="c">// clip units per scene unit, vertical</span>
            <span class="k">return</span> <span class="s">2</span>.<span class="s">0</span> * w.abs() / (vertical * <span class="k">self</span>.logical_size()[<span class="s">1</span>]).max(<span class="s">1</span>e-<span class="s">12</span>);
        }</code></pre></div>
<p>Replaces the lines from <code>self.gpu</code> in <code>lessons/24/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Tell the GPU where the gizmo is and which handle lights up.</span>
    <span class="k">pub</span> <span class="k">fn</span> upload_gizmo(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">let</span> Some(gizmo) = <span class="k">self</span>.gizmo.as_ref() <span class="k">else</span> {
            <span class="k">self</span>.gpu.widget.clear();
            <span class="k">return</span>;
        };
        <span class="k">self</span>.gpu.widget.placement = Some((
            [gizmo.origin[<span class="s">0</span>], gizmo.origin[<span class="s">1</span>], gizmo.origin[<span class="s">2</span>]],
            <span class="k">self</span>.world_per_px(), <span class="c">// keeps the widget the same pixel size</span>
        ));
        <span class="c">// the dragged handle, else the hovered one</span>
        <span class="k">let</span> handle = <span class="k">self</span>
            .dragging
            .as_ref()
            .map(|drag| drag.drag.handle)
            .or(gizmo.hovered);
        <span class="c">// handle index for the shader: 0-2 move, 3-5 rotate, 6-8 scale, 9 uniform</span>
        <span class="k">self</span>.gpu.widget.active = <span class="k">match</span> handle {
            Some(Handle::Translate(axis)) =&gt; axis <span class="k">as</span> u32 <span class="k">as</span> f32,
            Some(Handle::Rotate(axis)) =&gt; axis <span class="k">as</span> u32 <span class="k">as</span> f32 + <span class="s">3</span>.<span class="s">0</span>,
            Some(Handle::Scale(axis)) =&gt; axis <span class="k">as</span> u32 <span class="k">as</span> f32 + <span class="s">6</span>.<span class="s">0</span>,
            Some(Handle::ScaleUniform) =&gt; <span class="s">9</span>.<span class="s">0</span>,
            None =&gt; -<span class="s">1</span>.<span class="s">0</span>,
        };
    }

    <span class="c">/// Light the gizmo handle under the pointer; true when it changed.</span>
    <span class="k">pub</span> <span class="k">fn</span> hover_gizmo(&amp;<span class="k">mut</span> <span class="k">self</span>, x: f64, y: f64) -&gt; bool {
        <span class="k">let</span> Some((from, dir)) = <span class="k">self</span>.camera.ray((x, y), <span class="k">self</span>.viewport()) <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="k">let</span> per_px = <span class="k">self</span>.world_per_px();
        <span class="k">let</span> Some(gizmo) = <span class="k">self</span>.gizmo.as_mut() <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="k">let</span> hovered = gizmo.hit(&amp;from, &amp;dir, per_px);

        <span class="k">if</span> gizmo.hovered == hovered {
            <span class="k">return</span> <span class="s">false</span>;
        }

        gizmo.hovered = hovered;
        <span class="k">self</span>.upload_gizmo();
        <span class="s">true</span>
    }</code></pre></div>
<p>Delete the <code>const ARC_STEPS</code> block and the <code>fn widget_rows</code> block from <code>lessons/24/src/state/edit.rs</code>.</p>
<p>Delete the <code>fn the_widget_draws_three_arms_three_arcs_and_four_balls</code> test and its <code>#[test]</code> line from <code>lessons/24/src/state/edit.rs</code>.</p>
<p>Replaces the 3 lines from <code>let widget = gpu.widget_row();</code> in <code>lessons/24/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="c">// 96 pixels at 0.5 units per pixel fits the view</span>
        gpu.widget.placement = Some(([<span class="s">0</span>.<span class="s">0</span>; <span class="s">3</span>], <span class="s">0</span>.<span class="s">5</span>));</code></pre></div>
<p>Delete the <code>fn the_arcs_are_where_the_hit_test_expects_them</code> test and its <code>#[test]</code> line from <code>lessons/24/src/state/edit.rs</code>.</p>
<h2 id="check">Check<a class="anchor" href="#/course/25-gumball#check" aria-label="Link to this section">#</a></h2>
<p>Run <code>trunk serve</code> in <code>lessons/25/</code> and open <a href="http://127.0.0.1:8770/" target="_blank" rel="noopener">http://127.0.0.1:8770/</a>.</p>
<p>Expected: the selected object has solid colored handles and the status area shows no error.</p>
<p><img src="/session/docs/course/docs/screenshots/extensions-gumball-overview.png" alt="Full viewer result for lesson 25" loading="lazy" decoding="async"></p>
<p>If it fails:</p>
<ul>
<li>A handle blocks component picking: Ctrl does not bypass the gumball grab.</li>
<li>Gumball edges look rough: its tile sample count or composite coverage is wrong.</li>
</ul>
<h2 id="what-changed">What changed<a class="anchor" href="#/course/25-gumball#what-changed" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>lessons/25/src/
├── app/
│   ├── inspection/
│   │   └── source_memory.rs
│   ├── walk/
│   │   ├── bounds.rs
│   │   ├── brep.rs
│   │   ├── brep_edges.rs
│   │   ├── brep_orient.rs
│   │   ├── cloud.rs
│   │   ├── curves.rs
│   │   ├── encode.rs
│   │   ├── frames.rs
│   │   ├── mesh.rs
│   │   ├── mesh_ink.rs
│   │   ├── mesh_topology.rs
│   │   ├── mod.rs
│   │   ├── points.rs
│   │   └── sheet.rs
│   ├── cloud_query.rs
│   ├── command.rs
│   ├── coords.rs
│   ├── cplane.rs
│   ├── decode.rs
│   ├── edit.rs
│   ├── feedback.rs
│   ├── fetch.rs
│   ├── gizmo.rs  ~
│   ├── input.rs  ~
│   ├── inspection.rs
│   ├── knobs.rs
│   ├── layers.rs
│   ├── live.rs
│   ├── loader.rs
│   ├── manifest.rs
│   ├── mod.rs
│   ├── modeling.rs
│   ├── route.rs
│   ├── scene.rs
│   ├── scene_text.rs
│   ├── selection.rs
│   ├── sheet_query.rs
│   ├── snap.rs
│   ├── stream.rs
│   ├── touch.rs
│   └── validate.rs
├── engine/
│   ├── gpu/
│   │   ├── arena.rs
│   │   ├── backdrop.rs
│   │   ├── buffers.rs
│   │   ├── cloud.rs
│   │   ├── device.rs
│   │   ├── faces.rs
│   │   ├── frame.rs
│   │   ├── glyphs.rs
│   │   ├── instance.rs
│   │   ├── lod.rs
│   │   ├── mod.rs  ~
│   │   ├── objects.rs
│   │   ├── pick.rs
│   │   ├── present.rs  ~
│   │   ├── render.rs  ~
│   │   ├── segments.rs
│   │   ├── splat.rs
│   │   ├── surface_outline.rs
│   │   ├── targets.rs
│   │   ├── text.rs
│   │   ├── text_outline.rs
│   │   ├── text_plane.rs
│   │   ├── text_plate.rs
│   │   ├── triangle_tiles.rs
│   │   ├── upload.rs
│   │   ├── view.rs
│   │   ├── widget.rs  +
│   │   └── widget_mesh.rs  +
│   ├── pipelines/
│   │   ├── layouts.rs
│   │   └── mod.rs
│   ├── mod.rs
│   ├── performance.rs
│   └── text.rs
├── shaders/
│   ├── background.wgsl
│   ├── glyph.wgsl
│   ├── grid.wgsl
│   ├── ink_visibility.wgsl
│   ├── normals.wgsl
│   ├── physical.wgsl
│   ├── project_triangles.wgsl
│   ├── projected_triangle.wgsl
│   ├── ribbon.wgsl
│   ├── scan_triangle_tiles.wgsl
│   ├── scene.wgsl
│   ├── sphere.wgsl
│   ├── splat.wgsl
│   ├── splat_resolve.wgsl
│   ├── surface_outline.wgsl
│   ├── text_outline.wgsl
│   ├── text_plane.wgsl
│   ├── text_plate.wgsl
│   ├── triangle.wgsl
│   ├── triangle_tiles.wgsl
│   └── widget.wgsl  +
├── state/
│   ├── cloud_query.rs
│   ├── edit.rs  ~
│   ├── sheet_query.rs
│   └── text.rs
├── camera.rs
├── lib.rs
└── state.rs  ~</code></pre></div>
<p><code>+</code> new in this lesson · <code>~</code> changed in this lesson</p>
<p>Data flow: selected row → retained handle mesh → antialiased tile → scene overlay.
Every file at this point: <code>lessons/25/</code>.</p>
<h2 id="next">Next<a class="anchor" href="#/course/25-gumball#next" aria-label="Link to this section">#</a></h2>
<p><a href="#/course/26-nested-panel">26 · Build the nested session and graph panel</a></p>
<h2 id="expected-viewer-result">Expected viewer result<a class="anchor" href="#/course/25-gumball#expected-viewer-result" aria-label="Link to this section">#</a></h2>
<p>Select a line and press <strong>7</strong> for an isometric view. The whole viewer shows the selected line with solid cylindrical shafts, cone tips, rotation rings and scale spheres, while the surrounding scene stays visible. The capture uses the maintained viewer and the <a href="/session/docs/course/docs/extensions/nested.pb">nested fixture</a>. The bottom dock, right Layers panel and left toolbar visible in this maintained-viewer reference are added in <a href="#/course/29-docked-workspace">checkpoint 8</a>.</p>
<p><a href="/session/docs/course/docs/screenshots/extensions-gumball-overview.png"><img src="/session/docs/course/docs/screenshots/extensions-gumball-overview.png" alt="Full viewer result for lesson 25" loading="lazy" decoding="async"></a></p>
`,toc:[{level:2,id:"step-1-srcappgizmors",text:"Step 1 · src/app/gizmo.rs"},{level:2,id:"step-2-srcappinputrs",text:"Step 2 · src/app/input.rs"},{level:2,id:"step-3-srcenginegpumodrs",text:"Step 3 · src/engine/gpu/mod.rs"},{level:2,id:"step-4-srcenginegpupresentrs",text:"Step 4 · src/engine/gpu/present.rs"},{level:2,id:"step-5-srcenginegpurenderrs",text:"Step 5 · src/engine/gpu/render.rs"},{level:2,id:"step-6-srcenginegpuwidgetrs",text:"Step 6 · src/engine/gpu/widget.rs"},{level:2,id:"step-7-srcenginegpuwidget_meshrs",text:"Step 7 · src/engine/gpu/widget_mesh.rs"},{level:2,id:"step-8-srcshaderswidgetwgsl",text:"Step 8 · src/shaders/widget.wgsl"},{level:2,id:"step-9-srcstaters",text:"Step 9 · src/state.rs"},{level:2,id:"step-10-srcstateeditrs",text:"Step 10 · src/state/edit.rs"},{level:2,id:"check",text:"Check"},{level:2,id:"what-changed",text:"What changed"},{level:2,id:"next",text:"Next"},{level:2,id:"expected-viewer-result",text:"Expected viewer result"}]};export{s as default};
