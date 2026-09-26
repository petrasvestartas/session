const s={title:"04d · Point clouds",html:`<h1 id="04d-point-clouds">04d · Point clouds<a class="anchor" href="#/course/04d-clouds#04d-point-clouds" aria-label="Link to this section">#</a></h1>
<p>A grid of blue points appears beside the mesh, polyline and marker.</p>
<p><img src="/session/docs/course/docs/illustrations/lod.svg" alt="One node, one question: a spacing that projects wider than lod_px descends into the eight children, and one that fits draws the node whole." loading="lazy" decoding="async"></p>
<h2 id="step-1-srcenginegpucloudrs">Step 1 · src/engine/gpu/cloud.rs<a class="anchor" href="#/course/04d-clouds#step-1-srcenginegpucloudrs" aria-label="Link to this section">#</a></h2>
<p>New file: point cloud positions, colours and source ids on the GPU.</p>
<p><code>lessons/04d/src/engine/gpu/cloud.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> super::buffers::{GpuCtx, GrowBuf, ROWS};
<span class="k">use</span> super::upload::drop_rows;

<span class="c">/// Marker for a cloud that has no normals.</span>
<span class="k">pub</span> <span class="k">const</span> NO_NORMALS: u32 = u32::MAX;

<span class="c">/// One batch of points added to a cloud.</span>
<span class="k">pub</span> <span class="k">struct</span> CloudDraw {
    <span class="k">pub</span> instance: u32, <span class="c">// object row of the cloud</span>
    <span class="k">pub</span> from: u32, <span class="c">// first point index within the cloud</span>
    <span class="k">pub</span> count: u32, <span class="c">// points in this batch</span>
    <span class="k">pub</span> first: u32, <span class="c">// row of the first point in the upload</span>
    <span class="k">pub</span> spacing: f32, <span class="c">// typical distance between points</span>
    <span class="k">pub</span> node_first: u32, <span class="c">// This cloud's nodes; 0 nodes = no octree.</span>
    <span class="k">pub</span> node_count: u32,
    <span class="k">pub</span> nrm_first: u32, <span class="c">// first normal row, or NO_NORMALS</span>
}

<span class="c">/// One box of the cloud's octree.</span>
#[derive(Clone, Copy)]
<span class="k">pub</span> <span class="k">struct</span> LodNode {
    <span class="k">pub</span> center: [f32; <span class="s">3</span>], <span class="c">// box center</span>
    <span class="k">pub</span> size: f32, <span class="c">// box edge length</span>
    <span class="k">pub</span> spacing: f32, <span class="c">// point spacing inside the box</span>
    <span class="k">pub</span> first: u32, <span class="c">// first point of the box, within the cloud</span>
    <span class="k">pub</span> count: u32, <span class="c">// points in the box</span>
    <span class="k">pub</span> children: [i32; <span class="s">8</span>], <span class="c">// child node indices, -1 = none</span>
}

<span class="c">/// A run of cloud points stored on the GPU.</span>
#[derive(Clone, Copy)]
<span class="k">pub</span> <span class="k">struct</span> Chunk {
    <span class="k">pub</span> from: u32, <span class="c">// first point index within the cloud</span>
    <span class="k">pub</span> to: u32, <span class="c">// one past the last point index</span>
    <span class="k">pub</span> row: u32, <span class="c">// GPU row of the first point</span>
}

<span class="k">impl</span> Chunk {
    <span class="c">/// GPU row of cloud point \`i\`.</span>
    <span class="k">pub</span> <span class="k">fn</span> row_of(&amp;<span class="k">self</span>, i: u32) -&gt; u32 {
        <span class="k">self</span>.row + (i - <span class="k">self</span>.from)
    }
}</code></pre></div>
<p><code>lessons/04d/src/engine/gpu/cloud.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// One point cloud on the GPU.</span>
<span class="k">pub</span> <span class="k">struct</span> Cloud {
    <span class="k">pub</span> instance: u32, <span class="c">// object row</span>
    <span class="k">pub</span> spacing: f32, <span class="c">// typical distance between points</span>
    <span class="k">pub</span> node_first: u32,
    <span class="k">pub</span> node_count: u32,
    <span class="k">pub</span> nrm_first: u32, <span class="c">// first normal row, or NO_NORMALS</span>
    <span class="k">pub</span> resident: u32, <span class="c">// points uploaded so far</span>
    <span class="k">pub</span> chunks: Vec&lt;Chunk&gt;, <span class="c">// where those points live</span>
}

<span class="k">impl</span> Cloud {
    <span class="c">/// GPU row of cloud point \`i\`, None if not uploaded.</span>
    <span class="k">pub</span> <span class="k">fn</span> row_of(&amp;<span class="k">self</span>, i: u32) -&gt; Option&lt;u32&gt; {
        <span class="k">for</span> chunk <span class="k">in</span> &amp;<span class="k">self</span>.chunks {
            <span class="k">if</span> i &gt;= chunk.from &amp;&amp; i &lt; chunk.to {
                <span class="k">return</span> Some(chunk.row_of(i));
            }
        }

        None
    }
}

<span class="c">/// Point rows of one upload.</span>
#[derive(Default)]
<span class="k">pub</span> <span class="k">struct</span> CloudRows {
    <span class="k">pub</span> pos: Vec&lt;f32&gt;, <span class="c">// x, y, z per point</span>
    <span class="k">pub</span> col: Vec&lt;u32&gt;, <span class="c">// packed color per point</span>
    <span class="k">pub</span> nrm: Vec&lt;u32&gt;, <span class="c">// packed normal per point</span>
    <span class="k">pub</span> draws: Vec&lt;CloudDraw&gt;, <span class="c">// point batches in this upload</span>
    <span class="k">pub</span> nodes: Vec&lt;LodNode&gt;, <span class="c">// octree nodes in this upload</span>
}

<span class="k">impl</span> CloudRows {
    <span class="c">/// Points in this upload so far.</span>
    <span class="k">pub</span> <span class="k">fn</span> point_count(&amp;<span class="k">self</span>) -&gt; u32 {
        (<span class="k">self</span>.pos.len() / <span class="s">3</span>) <span class="k">as</span> u32
    }

    <span class="c">/// Empty every table and free its memory.</span>
    <span class="k">pub</span> <span class="k">fn</span> drop_rows(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        drop_rows(&amp;<span class="k">mut</span> <span class="k">self</span>.pos);
        drop_rows(&amp;<span class="k">mut</span> <span class="k">self</span>.col);
        drop_rows(&amp;<span class="k">mut</span> <span class="k">self</span>.nrm);
        drop_rows(&amp;<span class="k">mut</span> <span class="k">self</span>.draws);
        drop_rows(&amp;<span class="k">mut</span> <span class="k">self</span>.nodes);
    }
}

<span class="c">/// The three point buffers, borrowed for binding.</span>
<span class="k">pub</span> <span class="k">struct</span> PointBufs&lt;'a&gt; {
    <span class="k">pub</span> pos: &amp;'a wgpu::Buffer,
    <span class="k">pub</span> col: &amp;'a wgpu::Buffer,
    <span class="k">pub</span> nrm: &amp;'a wgpu::Buffer,
}</code></pre></div>
<p><code>lessons/04d/src/engine/gpu/cloud.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// All point clouds on the GPU.</span>
<span class="k">pub</span> <span class="k">struct</span> CloudLane {
    pos: GrowBuf, <span class="c">// positions</span>
    col: GrowBuf, <span class="c">// colors</span>
    nrm: GrowBuf, <span class="c">// normals</span>
    <span class="k">pub</span> clouds: Vec&lt;Cloud&gt;, <span class="c">// one entry per cloud</span>
    <span class="k">pub</span> nodes: Vec&lt;LodNode&gt;, <span class="c">// octree nodes of every cloud</span>
    <span class="k">pub</span> point_count: u32, <span class="c">// points on the GPU</span>
}

<span class="k">impl</span> CloudLane {
    <span class="c">/// Bytes reserved on the GPU by this lane.</span>
    <span class="k">pub</span> <span class="k">fn</span> allocated_bytes(&amp;<span class="k">self</span>) -&gt; u64 {
        <span class="k">self</span>.pos.buf.size() + <span class="k">self</span>.col.buf.size() + <span class="k">self</span>.nrm.buf.size()
    }

    <span class="c">/// Create the lane with empty buffers.</span>
    <span class="k">pub</span> <span class="k">fn</span> new(ctx: &amp;GpuCtx) -&gt; <span class="k">Self</span> {
        <span class="k">Self</span> {
            pos: GrowBuf::new(ctx, &quot;<span class="s">points.buffer</span>&quot;, <span class="s">4</span>, ROWS),
            col: GrowBuf::new(ctx, &quot;<span class="s">points.col.buffer</span>&quot;, <span class="s">4</span>, ROWS),
            nrm: GrowBuf::new(ctx, &quot;<span class="s">points.nrm.buffer</span>&quot;, <span class="s">4</span>, ROWS),
            clouds: Vec::new(),
            nodes: Vec::new(),
            point_count: <span class="s">0</span>,
        }
    }</code></pre></div>
<p><code>lessons/04d/src/engine/gpu/cloud.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Append one upload; returns true if a buffer was replaced.</span>
    <span class="k">pub</span> <span class="k">fn</span> append(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, up: &amp;CloudRows) -&gt; bool {
        debug_assert_eq!(up.col.len() * <span class="s">3</span>, up.pos.len());
        <span class="c">// rows before this upload</span>
        <span class="k">let</span> point_base = <span class="k">self</span>.point_count;
        <span class="k">let</span> nrm_base = <span class="k">self</span>.nrm.len();
        <span class="k">let</span> node_base = <span class="k">self</span>.nodes.len() <span class="k">as</span> u32;

        <span class="k">let</span> <span class="k">mut</span> moved = <span class="k">self</span>.pos.append(ctx, &amp;up.pos);
        moved |= <span class="k">self</span>.col.append(ctx, &amp;up.col);
        moved |= <span class="k">self</span>.nrm.append(ctx, &amp;up.nrm);
        <span class="k">self</span>.point_count = <span class="k">self</span>.pos.len() / <span class="s">3</span>;
        <span class="k">self</span>.nodes.extend_from_slice(&amp;up.nodes);

        <span class="k">for</span> d <span class="k">in</span> &amp;up.draws {
            <span class="k">let</span> chunk = Chunk {
                from: d.from,
                to: d.from + d.count,
                row: point_base + d.first,
            };

            <span class="c">// a later batch extends an existing cloud</span>
            <span class="k">if</span> d.from &gt; <span class="s">0</span> {
                <span class="k">self</span>.extend(d.instance, chunk);
                <span class="k">continue</span>;
            }

            <span class="k">let</span> nrm_first = <span class="k">if</span> d.nrm_first == NO_NORMALS {
                NO_NORMALS
            } <span class="k">else</span> {
                nrm_base + d.nrm_first
            };
            <span class="c">// first batch opens a new cloud</span>
            <span class="k">self</span>.clouds.push(Cloud {
                instance: d.instance,
                spacing: d.spacing,
                node_first: d.node_first + node_base,
                node_count: d.node_count,
                nrm_first,
                resident: chunk.to,
                chunks: vec![chunk],
            });
        }

        moved
    }

    <span class="c">/// Add a batch to the cloud on object row \`instance\`.</span>
    <span class="k">fn</span> extend(&amp;<span class="k">mut</span> <span class="k">self</span>, instance: u32, chunk: Chunk) {
        <span class="k">for</span> cloud <span class="k">in</span> &amp;<span class="k">mut</span> <span class="k">self</span>.clouds {
            <span class="k">if</span> cloud.instance != instance {
                <span class="k">continue</span>;
            }

            <span class="c">// batches must arrive in order</span>
            <span class="k">if</span> chunk.from != cloud.resident {
                log::warn!(
                    &quot;<span class="s">cloud chunk [</span>{}<span class="s">, </span>{}<span class="s">) does not continue the </span>{}<span class="s"> resident points; dropped</span>&quot;,
                    chunk.from,
                    chunk.to,
                    cloud.resident
                );
                <span class="k">return</span>;
            }

            cloud.resident = chunk.to;
            cloud.chunks.push(chunk);
            <span class="k">return</span>;
        }

        log::warn!(&quot;<span class="s">cloud chunk for row </span>{<span class="s">instance</span>}<span class="s"> arrived before its cloud; dropped</span>&quot;);
    }</code></pre></div>
<p><code>lessons/04d/src/engine/gpu/cloud.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Cloud of a GPU point row: (object row, point index).</span>
    <span class="k">pub</span> <span class="k">fn</span> row_of(&amp;<span class="k">self</span>, row: u32) -&gt; Option&lt;(u32, u32)&gt; {
        <span class="k">for</span> c <span class="k">in</span> &amp;<span class="k">self</span>.clouds {
            <span class="k">for</span> k <span class="k">in</span> &amp;c.chunks {
                <span class="k">if</span> row &gt;= k.row &amp;&amp; row &lt; k.row + (k.to - k.from) {
                    <span class="k">return</span> Some((c.instance, k.from + (row - k.row)));
                }
            }
        }

        None
    }

    <span class="c">/// The three point buffers.</span>
    <span class="k">pub</span> <span class="k">fn</span> buffers(&amp;<span class="k">self</span>) -&gt; PointBufs&lt;'_&gt; {
        PointBufs {
            pos: &amp;<span class="k">self</span>.pos.buf,
            col: &amp;<span class="k">self</span>.col.buf,
            nrm: &amp;<span class="k">self</span>.nrm.buf,
        }
    }

    <span class="c">/// Points uploaded across every cloud.</span>
    <span class="k">pub</span> <span class="k">fn</span> resident(&amp;<span class="k">self</span>) -&gt; u32 {
        <span class="k">let</span> <span class="k">mut</span> resident = <span class="s">0</span>;

        <span class="k">for</span> cloud <span class="k">in</span> &amp;<span class="k">self</span>.clouds {
            resident += cloud.resident;
        }

        resident
    }

    <span class="c">/// Forget every row; keep the buffers.</span>
    <span class="k">pub</span> <span class="k">fn</span> reset(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">self</span>.pos.reset();
        <span class="k">self</span>.col.reset();
        <span class="k">self</span>.nrm.reset();
        <span class="k">self</span>.point_count = <span class="s">0</span>;
        <span class="k">self</span>.clouds.clear();
        <span class="k">self</span>.nodes.clear();
    }

    <span class="c">/// Forget every row and free the buffers.</span>
    <span class="k">pub</span> <span class="k">fn</span> release(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx) {
        <span class="k">self</span>.reset();
        <span class="k">self</span>.pos.release(ctx);
        <span class="k">self</span>.col.release(ctx);
        <span class="k">self</span>.nrm.release(ctx);
        <span class="k">self</span>.clouds.shrink_to_fit();
        <span class="k">self</span>.nodes.shrink_to_fit();
    }
}</code></pre></div>
<h2 id="step-2-srcenginegpulodrs">Step 2 · src/engine/gpu/lod.rs<a class="anchor" href="#/course/04d-clouds#step-2-srcenginegpulodrs" aria-label="Link to this section">#</a></h2>
<p>New file: the octree walk that picks which runs of points to draw at this distance.</p>
<p><code>lessons/04d/src/engine/gpu/lod.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> super::cloud::{Cloud, LodNode};
<span class="k">use</span> session_rust::Xform;

<span class="c">/// Below 2 million points a cloud is cheap enough to draw in full.</span>
<span class="k">const</span> LOD_MIN_POINTS: u32 = <span class="s">2_000_000</span>;

<span class="c">/// One run of points to draw: a distant cloud draws fewer, wider-spaced points, since the rest would land on the same pixel.</span>
<span class="k">pub</span> <span class="k">struct</span> Range {
    <span class="k">pub</span> first: u32, <span class="c">// index into the cloud's points</span>
    <span class="k">pub</span> count: u32,
    <span class="k">pub</span> spacing: f32, <span class="c">// gap between neighbouring points, mm</span>
    <span class="k">pub</span> tile: bool, <span class="c">// true = an octree node, not the whole cloud</span>
}

<span class="c">/// An octree splits space into eight boxes again and again, so a whole box can be skipped or coarsened in one test.</span>
<span class="k">struct</span> Visit {
    first: u32,
    count: u32,
    spacing: f32, <span class="c">// finest spacing found in or below it</span>
    parent: usize, <span class="c">// index of the parent visit; usize::MAX for the root</span>
}

<span class="c">/// Camera facts the walk needs.</span>
<span class="k">pub</span> <span class="k">struct</span> Projection&lt;'a&gt; {
    <span class="k">pub</span> eye: [f32; <span class="s">3</span>],
    <span class="k">pub</span> ortho_h: f32, <span class="c">// world mm; 0 = perspective</span>
    <span class="k">pub</span> height_px: u32,
    <span class="k">pub</span> lod_px: f32, <span class="c">// split a node while its spacing is wider than this</span>
    <span class="k">pub</span> nodes: &amp;'a [LodNode], <span class="c">// every cloud's octree nodes</span>
}

<span class="c">/// Kept between frames, so the lists reuse their memory instead of allocating every frame.</span>
#[derive(Default)]
<span class="k">pub</span> <span class="k">struct</span> LodWalk {
    <span class="k">pub</span> ranges: Vec&lt;Range&gt;, <span class="c">// result: the runs to draw</span>
    stack: Vec&lt;(usize, usize)&gt;, <span class="c">// nodes still to visit, with parent</span>
    visits: Vec&lt;Visit&gt;,
}</code></pre></div>
<p><code>lessons/04d/src/engine/gpu/lod.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">impl</span> LodWalk {
    <span class="c">/// Pick which runs of one cloud to draw at this camera.</span>
    <span class="k">pub</span> <span class="k">fn</span> select(&amp;<span class="k">mut</span> <span class="k">self</span>, p: &amp;Projection, c: &amp;Cloud, model: &amp;[f32; <span class="s">16</span>]) {
        <span class="k">self</span>.ranges.clear();

        <span class="c">// no octree or small cloud: draw everything</span>
        <span class="k">if</span> c.node_count == <span class="s">0</span> || p.lod_px &lt;= <span class="s">0</span>.<span class="s">0</span> || c.resident &lt; LOD_MIN_POINTS {
            <span class="k">self</span>.ranges.push(Range {
                first: <span class="s">0</span>,
                count: c.resident,
                spacing: c.spacing,
                tile: <span class="s">false</span>,
            });
            <span class="k">return</span>;
        }

        <span class="k">let</span> base = c.node_first <span class="k">as</span> usize;
        <span class="c">// object scale, applied to spacings</span>
        <span class="k">let</span> scale = Xform::from_matrix(model.map(f64::from)).uniform_scale();
        <span class="k">self</span>.stack.clear();
        <span class="k">self</span>.visits.clear();
        <span class="c">// the root has no parent</span>
        <span class="k">self</span>.stack.push((<span class="s">0</span>, usize::MAX));

        <span class="k">while</span> <span class="k">let</span> Some((n, parent)) = <span class="k">self</span>.stack.pop() { <span class="c">// depth-first, with a stack instead of recursion</span>
            <span class="k">let</span> Some(node) = p.nodes.get(base + n) <span class="k">else</span> {
                <span class="k">continue</span>;
            };

            <span class="k">if</span> node.first &gt;= c.resident { <span class="c">// not streamed in yet</span>
                <span class="k">continue</span>;
            }

            <span class="c">// only points uploaded so far</span>
            <span class="k">let</span> count = node.count.min(c.resident - node.first);
            <span class="k">let</span> slot = <span class="k">self</span>.visits.len();
            <span class="k">self</span>.visits.push(Visit {
                first: node.first,
                count,
                spacing: node.spacing,
                parent,
            });

            <span class="c">// still too coarse on screen: visit the children</span>
            <span class="k">if</span> projected_spacing(p, node, model, scale) &gt; p.lod_px <span class="k">as</span> f64 {
                <span class="k">for</span> &amp;child <span class="k">in</span> &amp;node.children {
                    <span class="k">if</span> child &gt;= <span class="s">0</span> { <span class="c">// -1 = no child in that eighth</span>
                        <span class="k">self</span>.stack.push((child <span class="k">as</span> usize, slot));
                    }
                }
            }
        }

        <span class="c">// pass the finest spacing up; in reverse, because children sit after their parent</span>
        <span class="k">for</span> i <span class="k">in</span> (<span class="s">0</span>..<span class="k">self</span>.visits.len()).rev() {
            <span class="k">let</span> (fine, parent) = (<span class="k">self</span>.visits[i].spacing, <span class="k">self</span>.visits[i].parent);

            <span class="k">if</span> parent != usize::MAX &amp;&amp; fine &lt; <span class="k">self</span>.visits[parent].spacing {
                <span class="k">self</span>.visits[parent].spacing = fine;
            }
        }

        <span class="c">// every visited node becomes a run</span>
        <span class="k">for</span> v <span class="k">in</span> &amp;<span class="k">self</span>.visits {
            <span class="k">if</span> v.count &gt; <span class="s">0</span> {
                <span class="k">self</span>.ranges.push(Range {
                    first: v.first,
                    count: v.count,
                    spacing: v.spacing,
                    tile: <span class="s">true</span>,
                });
            }
        }
    }
}</code></pre></div>
<p><code>lessons/04d/src/engine/gpu/lod.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Pixels between two neighbouring points of the node on screen.</span>
<span class="k">fn</span> projected_spacing(p: &amp;Projection, node: &amp;LodNode, model: &amp;[f32; <span class="s">16</span>], scale: f64) -&gt; f64 {
    <span class="c">// spacing in metres</span>
    <span class="k">let</span> world = node.spacing <span class="k">as</span> f64 * scale * <span class="s">0</span>.<span class="s">001</span>;
    <span class="k">let</span> c = node.center;
    <span class="c">// node center in world space</span>
    <span class="k">let</span> wx = (model[<span class="s">0</span>] * c[<span class="s">0</span>] + model[<span class="s">4</span>] * c[<span class="s">1</span>] + model[<span class="s">8</span>] * c[<span class="s">2</span>] + model[<span class="s">12</span>]) <span class="k">as</span> f64;
    <span class="k">let</span> wy = (model[<span class="s">1</span>] * c[<span class="s">0</span>] + model[<span class="s">5</span>] * c[<span class="s">1</span>] + model[<span class="s">9</span>] * c[<span class="s">2</span>] + model[<span class="s">13</span>]) <span class="k">as</span> f64;
    <span class="k">let</span> wz = (model[<span class="s">2</span>] * c[<span class="s">0</span>] + model[<span class="s">6</span>] * c[<span class="s">1</span>] + model[<span class="s">10</span>] * c[<span class="s">2</span>] + model[<span class="s">14</span>]) <span class="k">as</span> f64;
    <span class="k">let</span> e = p.eye;
    <span class="c">// distance from the eye, metres</span>
    <span class="k">let</span> dist =
        ((wx - e[<span class="s">0</span>] <span class="k">as</span> f64).powi(<span class="s">2</span>) + (wy - e[<span class="s">1</span>] <span class="k">as</span> f64).powi(<span class="s">2</span>) + (wz - e[<span class="s">2</span>] <span class="k">as</span> f64).powi(<span class="s">2</span>))
            .sqrt()
            .max(<span class="s">1</span>.<span class="s">0</span>e-<span class="s">6</span>)
            * <span class="s">0</span>.<span class="s">001</span>;
    <span class="c">// spacing as a fraction of the view height</span>
    <span class="k">let</span> frac = <span class="k">if</span> p.ortho_h &gt; <span class="s">0</span>.<span class="s">0</span> {
        world / (<span class="s">2</span>.<span class="s">0</span> * p.ortho_h <span class="k">as</span> f64 * <span class="s">0</span>.<span class="s">001</span>)
    } <span class="k">else</span> {
        world * <span class="s">1</span>.<span class="s">7320508</span> * <span class="s">0</span>.<span class="s">5</span> / dist <span class="c">// 1.732 = 1 / tan(30°), half the 60° field of view</span>
    };
    frac * p.height_px <span class="k">as</span> f64
}

<span class="c">/// Disc radius of a run, folded so the shader divides once.</span>
<span class="k">pub</span> <span class="k">fn</span> radius_factor(r: &amp;Range, px: f32, scale: f64, ortho_h: f32) -&gt; f32 {
    <span class="c">// radius in metres; a size of 6 spans one spacing</span>
    <span class="k">let</span> <span class="k">mut</span> world_r = (r.spacing <span class="k">as</span> f64).max(<span class="s">1</span>.<span class="s">0</span>e-<span class="s">9</span>) * scale * <span class="s">0</span>.<span class="s">001</span> * (px <span class="k">as</span> f64) / <span class="s">6</span>.<span class="s">0</span>;

    <span class="c">// an octree node needs discs at least half a spacing wide</span>
    <span class="k">if</span> r.tile {
        world_r = world_r.max(r.spacing <span class="k">as</span> f64 * scale * <span class="s">0</span>.<span class="s">001</span> * <span class="s">0</span>.<span class="s">5</span>);
    }

    <span class="k">let</span> k = <span class="k">if</span> ortho_h &gt; <span class="s">0</span>.<span class="s">0</span> {
        world_r / (<span class="s">2</span>.<span class="s">0</span> * ortho_h <span class="k">as</span> f64 * <span class="s">0</span>.<span class="s">001</span>)
    } <span class="k">else</span> {
        world_r * <span class="s">1</span>.<span class="s">7320508</span> * <span class="s">0</span>.<span class="s">5</span>
    };
    k <span class="k">as</span> f32
}</code></pre></div>
<h2 id="step-3-srcenginegpusplatrs">Step 3 · src/engine/gpu/splat.rs<a class="anchor" href="#/course/04d-clouds#step-3-srcenginegpusplatrs" aria-label="Link to this section">#</a></h2>
<p>New file: the point lane: points drawn into an offscreen texture, then resolved into the scene.</p>
<p><code>lessons/04d/src/engine/gpu/splat.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> super::buffers::{GpuCtx, bind_group, zeroed_buffer};
<span class="k">use</span> super::cloud::{Cloud, LodNode, NO_NORMALS, PointBufs};
<span class="k">use</span> super::instance::Instance;
<span class="k">use</span> super::lod::{LodWalk, Projection, radius_factor};
<span class="k">use</span> super::objects::InstanceTable;
<span class="k">use</span> super::targets::{TextureSpec, texture_view};
<span class="k">use</span> <span class="k">crate</span>::engine::pipelines::{DepthMode, Layouts, PipelineDesc, Target, build, module};
<span class="k">use</span> session_rust::Xform;
<span class="k">use</span> wgpu::PrimitiveTopology::TriangleList;

<span class="c">/// At most 4096 runs a frame, so the record buffer is sized once: 16 + 4096 x 160 bytes, about 640 KB.</span>
<span class="k">pub</span> <span class="k">const</span> MAX_RECORDS: usize = <span class="s">4096</span>;

<span class="k">const</span> COLOR_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;

<span class="c">/// Each point is a screen-facing square of 2 triangles, 6 vertices, so it keeps its size however the camera turns.</span>
<span class="k">const</span> POINT_VERTS: u32 = <span class="s">6</span>;

<span class="c">/// Bytes before the records: record count, point total, 0, 0.</span>
<span class="k">const</span> HEADER_BYTES: u64 = <span class="s">16</span>;

<span class="c">/// One run of points to draw, 160 bytes, as the shader reads it.</span>
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
<span class="k">pub</span> <span class="k">struct</span> SplatRecord {
    <span class="k">pub</span> mvp_model: [f32; <span class="s">16</span>], <span class="c">// camera matrix times object matrix</span>
    <span class="k">pub</span> tint: [f32; <span class="s">4</span>], <span class="c">// rgb tint; a = smallest radius in px</span>
    <span class="k">pub</span> first: u32, <span class="c">// first GPU point row</span>
    <span class="k">pub</span> count: u32,
    <span class="k">pub</span> cum: u32, <span class="c">// points drawn before this run, so the shader can tell which run a vertex is in</span>
    <span class="k">pub</span> k: f32, <span class="c">// radius factor; the shader divides by depth</span>
    <span class="k">pub</span> rot: [f32; <span class="s">12</span>], <span class="c">// object rotation, 3 columns of 4 floats: WGSL pads each vec3 column to 16 bytes</span>
    <span class="k">pub</span> nrm_first: u32, <span class="c">// first normal row, or NO_NORMALS</span>
    <span class="k">pub</span> instance: u32,
    <span class="k">pub</span> flags: u32,
    <span class="k">pub</span> selected_point: u32, <span class="c">// highlighted point row + 1, or 0</span>
}

<span class="k">const</span> _: () = assert!(std::mem::size_of::&lt;SplatRecord&gt;() == <span class="s">160</span>);

<span class="c">/// Frame facts the record builder needs.</span>
<span class="k">pub</span> <span class="k">struct</span> RecordCx&lt;'a&gt; {
    <span class="k">pub</span> mvp: &amp;'a [f32; <span class="s">16</span>],
    <span class="k">pub</span> ortho_h: f32,
    <span class="k">pub</span> eye: [f32; <span class="s">3</span>],
    <span class="k">pub</span> size: (u32, u32),
    <span class="k">pub</span> cloud_size: f32,
    <span class="k">pub</span> lod_px: f32, <span class="c">// split octree nodes wider than this</span>
    <span class="k">pub</span> objects: &amp;'a InstanceTable,
    <span class="k">pub</span> clouds: &amp;'a [Cloud],
    <span class="k">pub</span> nodes: &amp;'a [LodNode],
}</code></pre></div>
<p><code>lessons/04d/src/engine/gpu/splat.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// What the last point pass depended on; same key = skip it.</span>
#[derive(Clone, PartialEq)]
<span class="k">struct</span> Key {
    mvp: [f32; <span class="s">16</span>],
    cloud_size: f32,
    lod_px: f32,
    point_count: u32, <span class="c">// grows while a cloud streams in</span>
}

<span class="c">/// Textures the point pass draws into, made when the first cloud arrives.</span>
<span class="k">struct</span> SplatTargets {
    depth: wgpu::TextureView,
    color: wgpu::TextureView,
    size: (u32, u32),
    resolve_group: wgpu::BindGroup, <span class="c">// both textures, for the resolve</span>
}

<span class="k">impl</span> SplatTargets {
    <span class="k">fn</span> new(ctx: &amp;GpuCtx, l: &amp;Layouts, size: (u32, u32)) -&gt; <span class="k">Self</span> {
        <span class="k">let</span> usage = wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING; <span class="c">// drawn into, then read by the resolve</span>
        <span class="k">let</span> depth = texture_view(
            ctx,
            &quot;<span class="s">splat.depth</span>&quot;,
            &amp;TextureSpec {
                size,
                format: wgpu::TextureFormat::Depth32Float,
                samples: <span class="s">1</span>,
                usage,
            },
        );
        <span class="k">let</span> color = texture_view(
            ctx,
            &quot;<span class="s">splat.color</span>&quot;,
            &amp;TextureSpec {
                size,
                format: COLOR_FORMAT,
                samples: <span class="s">1</span>,
                usage,
            },
        );
        <span class="k">let</span> resolve_group = ctx.device.create_bind_group(&amp;wgpu::BindGroupDescriptor {
            label: Some(&quot;<span class="s">splat.resolve.group</span>&quot;),
            layout: &amp;l.resolve,
            entries: &amp;[
                wgpu::BindGroupEntry {
                    binding: <span class="s">0</span>,
                    resource: wgpu::BindingResource::TextureView(&amp;depth),
                },
                wgpu::BindGroupEntry {
                    binding: <span class="s">1</span>,
                    resource: wgpu::BindingResource::TextureView(&amp;color),
                },
            ],
        });
        <span class="k">Self</span> {
            depth,
            color,
            size,
            resolve_group,
        }
    }
}</code></pre></div>
<p><code>lessons/04d/src/engine/gpu/splat.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Settings that differ between the color and id point pipelines.</span>
<span class="k">struct</span> PointVariant {
    target: Target,
    label: &amp;'static str,
    fs: &amp;'static str, <span class="c">// fragment shader entry point</span>
}

<span class="c">/// Two passes: points into an offscreen texture, then a resolve pass copies it, with depth, into the scene.</span>
<span class="k">pub</span> <span class="k">struct</span> Splat {
    control_parent: Option&lt;u32&gt;, <span class="c">// cloud being edited, drawn without LOD</span>
    selected_point: Option&lt;u32&gt;,
    records: Vec&lt;SplatRecord&gt;, <span class="c">// runs to draw this frame</span>
    walk: LodWalk,
    record_buf: wgpu::Buffer,
    total: u32, <span class="c">// points drawn last pass</span>
    key: Option&lt;Key&gt;,
    targets: Option&lt;SplatTargets&gt;, <span class="c">// None until the first cloud arrives</span>
    points_group: wgpu::BindGroup,
    resolve_shader: wgpu::ShaderModule,
    point_pipeline: wgpu::RenderPipeline,
    resolve_pipeline: wgpu::RenderPipeline,
    id_pipeline: wgpu::RenderPipeline,
}</code></pre></div>
<p><code>lessons/04d/src/engine/gpu/splat.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">impl</span> Splat {
    <span class="c">/// Bytes reserved on the GPU: (buffers, textures).</span>
    <span class="k">pub</span> <span class="k">fn</span> allocated_bytes(&amp;<span class="k">self</span>) -&gt; (u64, u64) {
        <span class="k">let</span> pixels = <span class="k">match</span> &amp;<span class="k">self</span>.targets {
            Some(target) =&gt; u64::from(target.size.<span class="s">0</span>) * u64::from(target.size.<span class="s">1</span>),
            None =&gt; <span class="s">0</span>,
        };
        (<span class="k">self</span>.record_buf.size(), pixels * <span class="s">8</span>) <span class="c">// 4 bytes of depth + 4 of colour per pixel</span>
    }

    <span class="k">pub</span> <span class="k">fn</span> new(ctx: &amp;GpuCtx, l: &amp;Layouts, target: Target, bufs: PointBufs) -&gt; <span class="k">Self</span> {
        <span class="k">let</span> record_buf = zeroed_buffer(
            &amp;ctx.device,
            &quot;<span class="s">splat.records</span>&quot;,
            HEADER_BYTES + MAX_RECORDS <span class="k">as</span> u64 * <span class="s">160</span>,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        );
        <span class="k">let</span> points_group = points_group(ctx, l, &amp;record_buf, &amp;bufs);
        <span class="k">let</span> point_shader = module(
            &amp;ctx.device,
            &quot;<span class="s">splat.shader</span>&quot;,
            include_str!(&quot;<span class="s">../../shaders/splat.wgsl</span>&quot;),
        );
        <span class="k">let</span> resolve_shader = module(
            &amp;ctx.device,
            &quot;<span class="s">splat.resolve.shader</span>&quot;,
            include_str!(&quot;<span class="s">../../shaders/splat_resolve.wgsl</span>&quot;),
        );
        <span class="k">let</span> point_pipeline = build_point(
            ctx,
            l,
            &amp;point_shader,
            &amp;PointVariant {
                target: Target {
                    format: COLOR_FORMAT,
                    samples: <span class="s">1</span>,
                },
                label: &quot;<span class="s">splat.points</span>&quot;,
                fs: &quot;<span class="s">fs_point</span>&quot;,
            },
        );
        <span class="k">let</span> id_pipeline = build_point(
            ctx,
            l,
            &amp;point_shader,
            &amp;PointVariant {
                target: Target::ID,
                label: &quot;<span class="s">splat.points.id</span>&quot;,
                fs: &quot;<span class="s">fs_point_id</span>&quot;,
            },
        );
        <span class="k">let</span> resolve_pipeline = build_resolve(ctx, l, &amp;resolve_shader, target);

        <span class="k">Self</span> {
            control_parent: None,
            selected_point: None,
            records: Vec::new(),
            walk: LodWalk::default(),
            record_buf,
            total: <span class="s">0</span>,
            key: None,
            targets: None,
            points_group,
            resolve_shader,
            point_pipeline,
            resolve_pipeline,
            id_pipeline,
        }
    }</code></pre></div>
<p><code>lessons/04d/src/engine/gpu/splat.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Draw cloud \`parent\` without LOD while it is edited.</span>
    <span class="k">pub</span> <span class="k">fn</span> set_controls(&amp;<span class="k">mut</span> <span class="k">self</span>, parent: Option&lt;u32&gt;) {
        <span class="k">self</span>.control_parent = parent;
        <span class="k">self</span>.selected_point = None;
        <span class="k">self</span>.invalidate();
    }

    <span class="c">/// Highlight one point row; None clears it.</span>
    <span class="k">pub</span> <span class="k">fn</span> set_point(&amp;<span class="k">mut</span> <span class="k">self</span>, point: Option&lt;u32&gt;) {
        <span class="k">self</span>.selected_point = point;
        <span class="k">self</span>.invalidate();
    }

    <span class="c">/// Rebuild the resolve pipeline for a new MSAA sample count.</span>
    <span class="k">pub</span> <span class="k">fn</span> retarget(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, l: &amp;Layouts, target: Target) {
        <span class="k">self</span>.resolve_pipeline = build_resolve(ctx, l, &amp;<span class="k">self</span>.resolve_shader, target);
    }

    <span class="c">/// Drop the textures; the next pass remakes them.</span>
    <span class="k">pub</span> <span class="k">fn</span> resize(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">self</span>.targets = None;
        <span class="k">self</span>.key = None;
    }

    <span class="c">/// Rebuild the bind group after a point buffer moved.</span>
    <span class="k">pub</span> <span class="k">fn</span> rebind(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, l: &amp;Layouts, bufs: PointBufs) {
        <span class="k">self</span>.points_group = points_group(ctx, l, &amp;<span class="k">self</span>.record_buf, &amp;bufs);
        <span class="k">self</span>.key = None;
    }

    <span class="c">/// Make the next frame redraw the points.</span>
    <span class="k">pub</span> <span class="k">fn</span> invalidate(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">self</span>.key = None;
    }

    <span class="c">/// Drop the textures and forget the points.</span>
    <span class="k">pub</span> <span class="k">fn</span> release(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">self</span>.targets = None;
        <span class="k">self</span>.total = <span class="s">0</span>;
        <span class="k">self</span>.key = None;
    }

    <span class="k">pub</span> <span class="k">fn</span> total(&amp;<span class="k">self</span>) -&gt; u32 {
        <span class="k">self</span>.total
    }</code></pre></div>
<p><code>lessons/04d/src/engine/gpu/splat.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Draw the points into their texture, unless nothing changed.</span>
    <span class="k">pub</span> <span class="k">fn</span> prelude(
        &amp;<span class="k">mut</span> <span class="k">self</span>,
        ctx: &amp;GpuCtx,
        l: &amp;Layouts,
        encoder: &amp;<span class="k">mut</span> wgpu::CommandEncoder,
        cx: &amp;RecordCx,
        cloud_group: &amp;wgpu::BindGroup,
    ) {
        <span class="k">let</span> <span class="k">mut</span> point_count = <span class="s">0u32</span>;

        <span class="k">for</span> c <span class="k">in</span> cx.clouds {
            point_count += c.resident;
        }

        <span class="c">// skip the pass when the inputs are unchanged</span>
        <span class="k">let</span> key = Key {
            mvp: *cx.mvp,
            cloud_size: cx.cloud_size,
            lod_px: cx.lod_px,
            point_count,
        };

        <span class="k">if</span> <span class="k">self</span>.key.as_ref() == Some(&amp;key) {
            <span class="k">return</span>;
        }

        <span class="k">self</span>.key = Some(key);
        <span class="k">self</span>.build_records(cx);

        <span class="k">if</span> <span class="k">self</span>.total == <span class="s">0</span> {
            <span class="k">return</span>;
        }

        <span class="c">// remake the textures when the size changed</span>
        <span class="k">if</span> !matches!(&amp;<span class="k">self</span>.targets, Some(targets) <span class="k">if</span> targets.size == cx.size) { <span class="c">// matches!: true when the value fits the pattern</span>
            <span class="k">self</span>.targets = Some(SplatTargets::new(ctx, l, cx.size));
        }

        <span class="c">// upload the header and the records</span>
        <span class="k">let</span> header = [<span class="k">self</span>.records.len() <span class="k">as</span> u32, <span class="k">self</span>.total, <span class="s">0</span>, <span class="s">0</span>];
        ctx.queue
            .write_buffer(&amp;<span class="k">self</span>.record_buf, <span class="s">0</span>, bytemuck::bytes_of(&amp;header));
        ctx.queue.write_buffer(
            &amp;<span class="k">self</span>.record_buf,
            HEADER_BYTES,
            bytemuck::cast_slice(&amp;<span class="k">self</span>.records),
        );

        <span class="k">let</span> Some(targets) = &amp;<span class="k">self</span>.targets <span class="k">else</span> { <span class="k">return</span> };
        <span class="k">let</span> <span class="k">mut</span> pass = begin_point_pass(encoder, targets);
        pass.set_pipeline(&amp;<span class="k">self</span>.point_pipeline);
        pass.set_bind_group(<span class="s">0</span>, cloud_group, &amp;[]);
        pass.set_bind_group(<span class="s">1</span>, &amp;<span class="k">self</span>.points_group, &amp;[]);
        pass.draw(<span class="s">0</span>..POINT_VERTS * <span class="k">self</span>.total, <span class="s">0</span>..<span class="s">1</span>); <span class="c">// no vertex buffer: the shader finds each point's run and row</span>
    }

    <span class="c">/// Draw the point texture into the scene; returns the draw count.</span>
    <span class="k">pub</span> <span class="k">fn</span> draw_resolve(
        &amp;<span class="k">self</span>,
        pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;,
        cloud_group: &amp;wgpu::BindGroup,
    ) -&gt; u32 {
        <span class="k">let</span> Some(targets) = &amp;<span class="k">self</span>.targets <span class="k">else</span> {
            <span class="k">return</span> <span class="s">0</span>;
        };

        <span class="k">if</span> <span class="k">self</span>.total == <span class="s">0</span> {
            <span class="k">return</span> <span class="s">0</span>;
        }

        pass.set_pipeline(&amp;<span class="k">self</span>.resolve_pipeline);
        pass.set_bind_group(<span class="s">0</span>, cloud_group, &amp;[]);
        pass.set_bind_group(<span class="s">1</span>, &amp;targets.resolve_group, &amp;[]);
        pass.draw(<span class="s">0</span>..<span class="s">3</span>, <span class="s">0</span>..<span class="s">1</span>); <span class="c">// one triangle big enough to cover the whole screen</span>
        <span class="s">1</span>
    }</code></pre></div>
<p><code>lessons/04d/src/engine/gpu/splat.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Draw the points as (object row, point row) ids.</span>
    <span class="k">pub</span> <span class="k">fn</span> draw_ids(&amp;<span class="k">self</span>, pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;, cloud_group: &amp;wgpu::BindGroup) -&gt; u32 {
        <span class="k">if</span> <span class="k">self</span>.total == <span class="s">0</span> {
            <span class="k">return</span> <span class="s">0</span>;
        }

        pass.set_pipeline(&amp;<span class="k">self</span>.id_pipeline);
        pass.set_bind_group(<span class="s">0</span>, cloud_group, &amp;[]);
        pass.set_bind_group(<span class="s">1</span>, &amp;<span class="k">self</span>.points_group, &amp;[]);
        pass.draw(<span class="s">0</span>..POINT_VERTS * <span class="k">self</span>.total, <span class="s">0</span>..<span class="s">1</span>);
        <span class="s">1</span>
    }

    <span class="c">/// Build one record per run of points to draw.</span>
    <span class="k">fn</span> build_records(&amp;<span class="k">mut</span> <span class="k">self</span>, cx: &amp;RecordCx) {</code></pre></div>
<p><code>lessons/04d/src/engine/gpu/splat.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.records.clear();
        <span class="k">let</span> <span class="k">mut</span> p = Projection {
            eye: cx.eye,
            ortho_h: cx.ortho_h,
            height_px: cx.size.<span class="s">1</span>,
            lod_px: cx.lod_px,
            nodes: cx.nodes,
        };
        <span class="k">let</span> <span class="k">mut</span> cum = <span class="s">0u32</span>;

        <span class="k">for</span> c <span class="k">in</span> cx.clouds {
            <span class="k">let</span> Some(row) = cx.objects.row(c.instance) <span class="k">else</span> {
                <span class="k">continue</span>;
            };

            <span class="k">if</span> row.flags &amp; Instance::FLAG_HIDDEN != <span class="s">0</span> {
                <span class="k">continue</span>;
            }

            <span class="k">let</span> Some(model) = cx.objects.anchored_model(c.instance) <span class="k">else</span> {
                <span class="k">continue</span>;
            };
            <span class="c">// point size in px; 3 when the cloud named none</span>
            <span class="k">let</span> px = <span class="k">if</span> row.spacing &gt; <span class="s">0</span>.<span class="s">0</span> { row.spacing } <span class="k">else</span> { <span class="s">3</span>.<span class="s">0</span> } * cx.cloud_size;

            <span class="c">// no LOD while the cloud is edited</span>
            p.lod_px = <span class="k">if</span> <span class="k">self</span>.control_parent == Some(c.instance) {
                <span class="s">0</span>.<span class="s">0</span>
            } <span class="k">else</span> {
                cx.lod_px
            };
            <span class="k">self</span>.walk.select(&amp;p, c, &amp;model);
            <span class="c">// camera times object, so the shader does one multiply per point</span>
            <span class="k">let</span> m = (&amp;Xform::from_matrix(cx.mvp.map(f64::from))
                * &amp;Xform::from_matrix(model.map(f64::from)))
                .to_f32();
            <span class="c">// rotation only, for the normals</span>
            <span class="k">let</span> rot = [
                model[<span class="s">0</span>], model[<span class="s">1</span>], model[<span class="s">2</span>], <span class="s">0</span>.<span class="s">0</span>, model[<span class="s">4</span>], model[<span class="s">5</span>], model[<span class="s">6</span>], <span class="s">0</span>.<span class="s">0</span>, model[<span class="s">8</span>],
                model[<span class="s">9</span>], model[<span class="s">10</span>], <span class="s">0</span>.<span class="s">0</span>,
            ];
            <span class="k">let</span> scale = Xform::from_matrix(model.map(f64::from)).uniform_scale();
            <span class="k">let</span> selected = row.flags &amp; Instance::FLAG_SELECTED != <span class="s">0</span>;
            <span class="c">// yellow when selected</span>
            <span class="k">let</span> tint = <span class="k">if</span> selected {
                [<span class="s">1</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, (px * <span class="s">0</span>.<span class="s">5</span>).max(<span class="s">0</span>.<span class="s">5</span>)]
            } <span class="k">else</span> {
                [
                    row.color[<span class="s">0</span>],
                    row.color[<span class="s">1</span>],
                    row.color[<span class="s">2</span>],
                    (px * <span class="s">0</span>.<span class="s">5</span>).max(<span class="s">0</span>.<span class="s">5</span>),
                ]
            };

            <span class="k">for</span> r <span class="k">in</span> &amp;<span class="k">self</span>.walk.ranges {
                <span class="k">let</span> k = radius_factor(r, px, scale, cx.ortho_h);

                <span class="c">// clip the run to each uploaded chunk</span>
                <span class="k">for</span> chunk <span class="k">in</span> &amp;c.chunks {
                    <span class="k">let</span> a = r.first.max(chunk.from);
                    <span class="k">let</span> b = (r.first + r.count).min(chunk.to);

                    <span class="k">if</span> a &gt;= b || <span class="k">self</span>.records.len() &gt;= MAX_RECORDS {
                        <span class="k">continue</span>;
                    }

                    <span class="k">let</span> nrm_first = <span class="k">if</span> c.nrm_first == NO_NORMALS {
                        NO_NORMALS
                    } <span class="k">else</span> {
                        c.nrm_first + a
                    };
                    <span class="k">self</span>.records.push(SplatRecord {
                        mvp_model: m,
                        tint,
                        first: chunk.row_of(a),
                        count: b - a,
                        cum,
                        k,
                        rot,
                        nrm_first,
                        instance: c.instance,
                        flags: row.flags,
                        selected_point: <span class="k">match</span> <span class="k">self</span>.selected_point {
                            Some(point) =&gt; point + <span class="s">1</span>,
                            None =&gt; <span class="s">0</span>,
                        },
                    });
                    cum += b - a;
                }
            }
        }

        <span class="k">self</span>.total = cum;
    }
}</code></pre></div>
<p><code>lessons/04d/src/engine/gpu/splat.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Colour cleared to transparent, depth to 0.0, which is far under reverse-Z.</span>
<span class="k">fn</span> begin_point_pass&lt;'a&gt;(
    encoder: &amp;'a <span class="k">mut</span> wgpu::CommandEncoder,
    t: &amp;'a SplatTargets,
) -&gt; wgpu::RenderPass&lt;'a&gt; {
    encoder.begin_render_pass(&amp;wgpu::RenderPassDescriptor {
        label: Some(&quot;<span class="s">splat.points</span>&quot;),
        color_attachments: &amp;[Some(wgpu::RenderPassColorAttachment {
            view: &amp;t.color,
            resolve_target: None,
            depth_slice: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                store: wgpu::StoreOp::Store,
            },
        })],
        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
            view: &amp;t.depth,
            depth_ops: Some(wgpu::Operations {
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

<span class="c">/// Bind group 1 of the point pass: records, positions, colors, normals.</span>
<span class="k">fn</span> points_group(
    ctx: &amp;GpuCtx,
    l: &amp;Layouts,
    records: &amp;wgpu::Buffer,
    bufs: &amp;PointBufs,
) -&gt; wgpu::BindGroup {
    bind_group(
        ctx,
        &amp;l.points,
        &quot;<span class="s">splat.points.group</span>&quot;,
        &amp;[records, bufs.pos, bufs.col, bufs.nrm],
    )
}

<span class="c">/// Point pipeline: quads, nearest point wins, no blending.</span>
<span class="k">fn</span> build_point(
    ctx: &amp;GpuCtx,
    l: &amp;Layouts,
    shader: &amp;wgpu::ShaderModule,
    v: &amp;PointVariant,
) -&gt; wgpu::RenderPipeline {
    <span class="k">let</span> groups = [&amp;l.line, &amp;l.points];
    <span class="k">let</span> desc = PipelineDesc::new(shader, &amp;groups, &amp;[], TriangleList)
        .with(v.label, v.fs)
        .vertex(&quot;<span class="s">vs_point</span>&quot;);
    build(&amp;ctx.device, v.target, &amp;desc)
}</code></pre></div>
<p><code>lessons/04d/src/engine/gpu/splat.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Resolve pipeline: a fullscreen triangle writing color and depth.</span>
<span class="k">fn</span> build_resolve(
    ctx: &amp;GpuCtx,
    l: &amp;Layouts,
    shader: &amp;wgpu::ShaderModule,
    target: Target,
) -&gt; wgpu::RenderPipeline {
    <span class="k">let</span> groups = [&amp;l.line, &amp;l.resolve];
    <span class="k">let</span> desc = PipelineDesc::new(shader, &amp;groups, &amp;[], TriangleList)
        .with(&quot;<span class="s">splat.resolve</span>&quot;, &quot;<span class="s">fs_main</span>&quot;)
        .depth(DepthMode::Opaque);
    build(&amp;ctx.device, target, &amp;desc)
}</code></pre></div>
<h2 id="step-4-srcshaderssplatwgsl">Step 4 · src/shaders/splat.wgsl<a class="anchor" href="#/course/04d-clouds#step-4-srcshaderssplatwgsl" aria-label="Link to this section">#</a></h2>
<p>New file: the shader that writes the nearest point per pixel.</p>
<p><code>lessons/04d/src/shaders/splat.wgsl</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// The first 16 of CloudUniform's 48 bytes: a shader may declare less than the buffer holds.</span>
<span class="k">struct</span> CloudUniform {
    size: <span class="k">f32</span>,
    vp_w: <span class="k">f32</span>,
    vp_h: <span class="k">f32</span>,
    edl: <span class="k">f32</span>,
};

@group(<span class="s">0</span>) @binding(<span class="s">0</span>) <span class="k">var</span>&lt;<span class="k">uniform</span>&gt; cloud: CloudUniform;

<span class="c">// SplatRecord read as raw u32 words: 160 bytes = 40 words; word 20 is first, 22 cum, 23 k.</span>
<span class="k">const</span> REC_WORDS: <span class="k">u32</span> = <span class="s">40u</span>;
<span class="k">const</span> NO_NORMALS: <span class="k">u32</span> = <span class="s">0xffffffffu</span>;<span class="c"> // marker for a run without normals</span>
@group(<span class="s">1</span>) @binding(<span class="s">0</span>) <span class="k">var</span>&lt;<span class="k">storage</span>, <span class="k">read</span>&gt; table: <span class="k">array</span>&lt;<span class="k">u32</span>&gt;;<span class="c"> // header {records, points, 0, 0}, then the records</span>
@group(<span class="s">1</span>) @binding(<span class="s">1</span>) <span class="k">var</span>&lt;<span class="k">storage</span>, <span class="k">read</span>&gt; positions: <span class="k">array</span>&lt;<span class="k">f32</span>&gt;;<span class="c"> // x, y, z per point</span>
@group(<span class="s">1</span>) @binding(<span class="s">2</span>) <span class="k">var</span>&lt;<span class="k">storage</span>, <span class="k">read</span>&gt; colors: <span class="k">array</span>&lt;<span class="k">u32</span>&gt;;<span class="c"> // packed rgba per point</span>
@group(<span class="s">1</span>) @binding(<span class="s">3</span>) <span class="k">var</span>&lt;<span class="k">storage</span>, <span class="k">read</span>&gt; normals: <span class="k">array</span>&lt;<span class="k">u32</span>&gt;;<span class="c"> // octahedral normal per point</span>

<span class="c">// One point projected to the screen.</span>
<span class="k">struct</span> Splat {
    px: <span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;,<span class="c"> // center pixel</span>
    r: <span class="k">f32</span>,<span class="c"> // radius, px</span>
    z: <span class="k">f32</span>,
    color: <span class="k">u32</span>,<span class="c"> // packed rgba, lit</span>
    row: <span class="k">u32</span>,
    instance: <span class="k">u32</span>,
    ok: <span class="k">bool</span>,<span class="c"> // false = off screen or behind the camera</span>
};

<span class="c">// bitcast: the same 32 bits read as an f32, no conversion.</span>
<span class="k">fn</span> rec_f(base: <span class="k">u32</span>, w: <span class="k">u32</span>) -&gt; <span class="k">f32</span> {
    <span class="k">return</span> <span class="k">bitcast</span>&lt;<span class="k">f32</span>&gt;(table[base + w]);
}

<span class="c">// Which record holds drawn point gid: a binary search on cum, 12 steps for 4096 records.</span>
<span class="k">fn</span> record_of(gid: <span class="k">u32</span>) -&gt; <span class="k">u32</span> {
    <span class="k">let</span> n = table[<span class="s">0</span>];
    <span class="k">var</span> lo = <span class="s">0u</span>;
    <span class="k">var</span> hi = n;

    <span class="k">while</span> (hi - lo &gt; <span class="s">1u</span>) {
        <span class="k">let</span> mid = (lo + hi) / <span class="s">2u</span>;

        <span class="k">if</span> (table[<span class="s">4u</span> + mid * REC_WORDS + <span class="s">22u</span>] &lt;= gid) {
            lo = mid;
        } <span class="k">else</span> {
            hi = mid;
        }
    }

    <span class="k">return</span> lo;</code></pre></div>
<p><code>lessons/04d/src/shaders/splat.wgsl</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>}

<span class="c">// Project drawn point \`gid\`: pixel, radius, depth, lit color.</span>
<span class="k">fn</span> project(gid: <span class="k">u32</span>) -&gt; Splat {
    <span class="k">var</span> s: Splat;
    s.ok = <span class="s">false</span>;

    <span class="k">if</span> (gid &gt;= table[<span class="s">1</span>]) {
        <span class="k">return</span> s;
    }

    <span class="k">let</span> base = <span class="s">4u</span> + record_of(gid) * REC_WORDS;
<span class="c">    // index within the run</span>
    <span class="k">let</span> offset = gid - table[base + <span class="s">22u</span>];
    <span class="k">let</span> i = table[base + <span class="s">20u</span>] + offset;
    <span class="k">let</span> m = <span class="k">mat4x4</span>&lt;<span class="k">f32</span>&gt;(
        <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(rec_f(base, <span class="s">0u</span>), rec_f(base, <span class="s">1u</span>), rec_f(base, <span class="s">2u</span>), rec_f(base, <span class="s">3u</span>)),
        <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(rec_f(base, <span class="s">4u</span>), rec_f(base, <span class="s">5u</span>), rec_f(base, <span class="s">6u</span>), rec_f(base, <span class="s">7u</span>)),
        <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(rec_f(base, <span class="s">8u</span>), rec_f(base, <span class="s">9u</span>), rec_f(base, <span class="s">10u</span>), rec_f(base, <span class="s">11u</span>)),
        <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(rec_f(base, <span class="s">12u</span>), rec_f(base, <span class="s">13u</span>), rec_f(base, <span class="s">14u</span>), rec_f(base, <span class="s">15u</span>)),
    );
    s.row = i;
    s.instance = table[base + <span class="s">37u</span>];
    <span class="k">let</span> clip = m * <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(positions[i * <span class="s">3u</span>], positions[i * <span class="s">3u</span> + <span class="s">1u</span>], positions[i * <span class="s">3u</span> + <span class="s">2u</span>], <span class="s">1.0</span>);

    <span class="k">if</span> (clip.w &lt;= <span class="s">0.0</span>) {<span class="c"> // behind the eye</span>
        <span class="k">return</span> s;
    }

    <span class="k">let</span> ndc = clip.xyz / clip.w;

    <span class="k">if</span> (ndc.z &lt; <span class="s">0.0</span> || ndc.z &gt; <span class="s">1.0</span>) {
        <span class="k">return</span> s;
    }

<span class="c">    // radius in px, between the record's minimum and 8</span>
    <span class="k">let</span> r_min = rec_f(base, <span class="s">19u</span>);
    s.r = clamp(<span class="k">bitcast</span>&lt;<span class="k">f32</span>&gt;(table[base + <span class="s">23u</span>]) * cloud.vp_h / clip.w, r_min, <span class="s">8.0</span>);
    <span class="k">let</span> x = (ndc.x * <span class="s">0.5</span> + <span class="s">0.5</span>) * cloud.vp_w;
    <span class="k">let</span> y = (<span class="s">0.5</span> - ndc.y * <span class="s">0.5</span>) * cloud.vp_h;

    <span class="k">if</span> (x &lt; -s.r || y &lt; -s.r || x &gt;= cloud.vp_w + s.r || y &gt;= cloud.vp_h + s.r) {
        <span class="k">return</span> s;
    }

    s.px = <span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;(i32(x), i32(y));
    s.z = ndc.z;

    <span class="k">let</span> tint = <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(rec_f(base, <span class="s">16u</span>), rec_f(base, <span class="s">17u</span>), rec_f(base, <span class="s">18u</span>), <span class="s">1.0</span>);</code></pre></div>
<p><code>lessons/04d/src/shaders/splat.wgsl</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">var</span> rgba = unpack4x8unorm(colors[i]) * tint;
    <span class="k">let</span> nrm_first = table[base + <span class="s">36u</span>];

<span class="c">    // light the point by its normal</span>
    <span class="k">if</span> (nrm_first != NO_NORMALS) {
        <span class="k">let</span> packed_n = normals[nrm_first + offset];
        <span class="k">let</span> rot = <span class="k">mat3x3</span>&lt;<span class="k">f32</span>&gt;(
            <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(rec_f(base, <span class="s">24u</span>), rec_f(base, <span class="s">25u</span>), rec_f(base, <span class="s">26u</span>)),
            <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(rec_f(base, <span class="s">28u</span>), rec_f(base, <span class="s">29u</span>), rec_f(base, <span class="s">30u</span>)),
            <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(rec_f(base, <span class="s">32u</span>), rec_f(base, <span class="s">33u</span>), rec_f(base, <span class="s">34u</span>)),
        );
        <span class="k">let</span> nw = transform_normal(rot, oct16_decode(packed_n));
        <span class="k">let</span> light = normalize(<span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.4</span>, <span class="s">0.4</span>, <span class="s">0.8</span>));
        <span class="k">let</span> lambert = <span class="s">0.25</span> + <span class="s">0.75</span> * abs(dot(nw, light));<span class="c"> // Lambert: brightness follows the cosine to the light; 0.25 keeps the dark side visible</span>
        rgba = <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(rgba.rgb * lambert, rgba.a);
    }

<span class="c">    // selected object or highlighted point: yellow</span>
    <span class="k">if</span> ((table[base + <span class="s">38u</span>] &amp; <span class="s">1u</span>) != <span class="s">0u</span> || table[base + <span class="s">39u</span>] == i + <span class="s">1u</span>) {
        rgba = <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(<span class="s">1.0</span>, <span class="s">1.0</span>, <span class="s">0.0</span>, <span class="s">1.0</span>);
    }

    s.color = pack4x8unorm(rgba);
    s.ok = <span class="s">true</span>;
    <span class="k">return</span> s;
}

<span class="k">struct</span> PointOut {
    @builtin(position) pos: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,
    @location(<span class="s">0</span>) @interpolate(flat) center: <span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;,
    @location(<span class="s">1</span>) @interpolate(flat) rr: <span class="k">f32</span>,<span class="c"> // radius squared, px</span>
    @location(<span class="s">2</span>) @interpolate(flat) color: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,
    @location(<span class="s">3</span>) @interpolate(flat) row: <span class="k">u32</span>,
    @location(<span class="s">4</span>) @interpolate(flat) instance: <span class="k">u32</span>,
};

<span class="c">// One corner of a pixel-aligned square around the point.</span>
@vertex
<span class="k">fn</span> vs_point(@builtin(vertex_index) vid: <span class="k">u32</span>) -&gt; PointOut {
    <span class="k">var</span> o: PointOut;
    <span class="k">let</span> s = project(vid / <span class="s">6u</span>);<span class="c"> // 6 vertices per point: vertex 13 is corner 1 of point 2</span>

    <span class="k">if</span> (!s.ok) {
        o.pos = <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(<span class="s">3.0</span>, <span class="s">3.0</span>, <span class="s">0.5</span>, <span class="s">1.0</span>);
        <span class="k">return</span> o;
    }

<span class="c">    // half the square, whole pixels</span>
    <span class="k">let</span> ir = i32(ceil(s.r - <span class="s">0.5</span>));
<span class="c">    // keeps the square's corners outside the disc</span>
    <span class="k">let</span> corner_rr = <span class="s">2.0</span> * f32(ir * ir) - <span class="s">0.001</span>;
    <span class="k">let</span> lo = <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(f32(s.px.x - ir), f32(s.px.y - ir));
    <span class="k">let</span> hi = <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(f32(s.px.x + ir + <span class="s">1</span>), f32(s.px.y + ir + <span class="s">1</span>));
    <span class="k">let</span> c = vid % <span class="s">6u</span>;<span class="c"> // two triangles: corners 0-1-2 and 3-4-5</span>
    <span class="k">let</span> right = c == <span class="s">1u</span> || c == <span class="s">4u</span> || c == <span class="s">5u</span>;
    <span class="k">let</span> bottom = c == <span class="s">2u</span> || c == <span class="s">3u</span> || c == <span class="s">5u</span>;
    <span class="k">let</span> p = <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(select(lo.x, hi.x, right), select(lo.y, hi.y, bottom));
    o.pos = <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(p.x / cloud.vp_w * <span class="s">2.0</span> - <span class="s">1.0</span>, <span class="s">1.0</span> - p.y / cloud.vp_h * <span class="s">2.0</span>, s.z, <span class="s">1.0</span>);<span class="c"> // pixel back to clip space; w = 1, no perspective divide</span>
    o.center = s.px;
    o.rr = select(s.r * s.r, min(s.r * s.r, corner_rr), ir &gt;= <span class="s">1</span>);
    o.color = unpack4x8unorm(s.color);
    o.row = s.row;
    o.instance = s.instance;
    <span class="k">return</span> o;
}

<span class="c">// True for pixels outside the disc.</span>
<span class="k">fn</span> outside(in: PointOut) -&gt; <span class="k">bool</span> {
    <span class="k">let</span> q = <span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;(floor(in.pos.xy));
    <span class="k">let</span> d = q - in.center;
    <span class="k">return</span> f32(d.x * d.x + d.y * d.y) &gt; in.rr;
}

@fragment
<span class="k">fn</span> fs_point(in: PointOut) -&gt; @location(<span class="s">0</span>) <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt; {
    <span class="k">if</span> (outside(in)) {
        <span class="k">discard</span>;
    }

    <span class="k">return</span> in.color;
}

<span class="c">// Pick id: (object row + 1, point row + 1).</span>
@fragment
<span class="k">fn</span> fs_point_id(in: PointOut) -&gt; @location(<span class="s">0</span>) <span class="k">vec2</span>&lt;<span class="k">u32</span>&gt; {
    <span class="k">if</span> (outside(in)) {
        <span class="k">discard</span>;
    }

    <span class="k">return</span> <span class="k">vec2</span>&lt;<span class="k">u32</span>&gt;(in.instance + <span class="s">1u</span>, in.row + <span class="s">1u</span>);
}</code></pre></div>
<h2 id="step-5-srcshaderssplat_resolvewgsl">Step 5 · src/shaders/splat_resolve.wgsl<a class="anchor" href="#/course/04d-clouds#step-5-srcshaderssplat_resolvewgsl" aria-label="Link to this section">#</a></h2>
<p>New file: the shader that copies point colour and depth into the scene.</p>
<p><code>lessons/04d/src/shaders/splat_resolve.wgsl</code> · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// The first 16 bytes of CloudUniform, as in splat.wgsl.</span>
<span class="k">struct</span> CloudUniform {
    size: <span class="k">f32</span>,
    vp_w: <span class="k">f32</span>,
    vp_h: <span class="k">f32</span>,
    edl: <span class="k">f32</span>,
};

@group(<span class="s">0</span>) @binding(<span class="s">0</span>) <span class="k">var</span>&lt;<span class="k">uniform</span>&gt; cloud: CloudUniform;
@group(<span class="s">1</span>) @binding(<span class="s">0</span>) <span class="k">var</span> sdepth: texture_depth_2d;<span class="c"> // nearest point depth per pixel</span>
@group(<span class="s">1</span>) @binding(<span class="s">1</span>) <span class="k">var</span> scolor: texture_2d&lt;<span class="k">f32</span>&gt;;<span class="c"> // its color</span>

<span class="k">struct</span> VsOut {
    @builtin(position) pos: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,
};

@vertex
<span class="c">// Corners (-1, -1), (3, -1), (-1, 3): one triangle covers the whole -1..1 screen.</span>
<span class="k">fn</span> vs_main(@builtin(vertex_index) vid: <span class="k">u32</span>) -&gt; VsOut {
    <span class="k">var</span> o: VsOut;
    <span class="k">let</span> x = f32(i32(vid &amp; <span class="s">1u</span>) * <span class="s">4</span> - <span class="s">1</span>);
    <span class="k">let</span> y = f32(i32(vid &gt;&gt; <span class="s">1u</span>) * <span class="s">4</span> - <span class="s">1</span>);
    o.pos = <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(x, y, <span class="s">0.0</span>, <span class="s">1.0</span>);
    <span class="k">return</span> o;
}

<span class="c">// frag_depth: the fragment sets its own depth instead of taking the triangle's.</span>
<span class="k">struct</span> FsOut {
    @location(<span class="s">0</span>) color: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,
    @builtin(frag_depth) depth: <span class="k">f32</span>,
};

<span class="c">// Depth on a log scale that grows with distance.</span>
<span class="k">fn</span> log_depth(d: <span class="k">f32</span>) -&gt; <span class="k">f32</span> {
    <span class="k">return</span> -log2(max(d, <span class="s">1.0e-7</span>));
}

<span class="c">// Copy the point texture into the scene, darkening edges when EDL is on.</span>
<span class="k">fn</span> shade(in: VsOut) -&gt; FsOut {
    <span class="k">let</span> pix = <span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;(in.pos.xy);
    <span class="k">let</span> d = textureLoad(sdepth, pix, <span class="s">0</span>);

<span class="c">    // depth 0 is still the clear value: no point here</span>
    <span class="k">if</span> (d == <span class="s">0.0</span>) {
        <span class="k">discard</span>;
    }

    <span class="k">var</span> o: FsOut;
    <span class="k">var</span> rgb = textureLoad(scolor, pix, <span class="s">0</span>).rgb;

<span class="c">    // eye-dome lighting: darken where neighbours are nearer</span>
    <span class="k">if</span> (cloud.edl &gt; <span class="s">0.0</span>) {
        <span class="k">let</span> w = i32(cloud.vp_w);
        <span class="k">let</span> h = i32(cloud.vp_h);
        <span class="k">let</span> me = log_depth(d);
        <span class="k">var</span> sum = <span class="s">0.0</span>;
        <span class="k">var</span> taps = <span class="k">array</span>&lt;<span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;, <span class="s">4</span>&gt;(<span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;(-<span class="s">1</span>, <span class="s">0</span>), <span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;(<span class="s">1</span>, <span class="s">0</span>), <span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;(<span class="s">0</span>, -<span class="s">1</span>), <span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;(<span class="s">0</span>, <span class="s">1</span>));

        <span class="k">for</span> (<span class="k">var</span> k = <span class="s">0</span>; k &lt; <span class="s">4</span>; k++) {
            <span class="k">let</span> q = pix + taps[k];

            <span class="k">if</span> (q.x &lt; <span class="s">0</span> || q.y &lt; <span class="s">0</span> || q.x &gt;= w || q.y &gt;= h) {
                <span class="k">continue</span>;
            }

            <span class="k">let</span> nd = textureLoad(sdepth, q, <span class="s">0</span>);

            <span class="k">if</span> (nd == <span class="s">0.0</span>) {
                <span class="k">continue</span>;
            }

            sum += max(<span class="s">0.0</span>, me - log_depth(nd));
        }

<span class="c">        // darken by the depth steps, never below a quarter</span>
        <span class="k">let</span> shade = max(exp(-sum * <span class="s">75.0</span> * cloud.edl), <span class="s">0.25</span>);
        rgb *= shade;
    }

    o.color = <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(rgb, <span class="s">1.0</span>);
    o.depth = d;
    <span class="k">return</span> o;
}

@fragment
<span class="k">fn</span> fs_main(in: VsOut) -&gt; FsOut {
    <span class="k">return</span> shade(in);
}</code></pre></div>
<p>Run <code>cargo check</code> in <code>lessons/04d/</code>.</p>
<h2 id="step-6-srcenginepipelineslayoutsrs">Step 6 · src/engine/pipelines/layouts.rs<a class="anchor" href="#/course/04d-clouds#step-6-srcenginepipelineslayoutsrs" aria-label="Link to this section">#</a></h2>
<p>Add the cloud bind group layouts.</p>
<p><code>lessons/04d/src/engine/pipelines/layouts.rs</code> · edit · type this</p>
<p>Added above</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// The bind-group layouts every lane shares.</span></code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Group 1 for points: records, positions, colors, normals.</span>
<span class="k">fn</span> points_layout(device: &amp;wgpu::Device) -&gt; wgpu::BindGroupLayout {
    device.create_bind_group_layout(&amp;wgpu::BindGroupLayoutDescriptor {
        label: Some(&quot;<span class="s">points.layout</span>&quot;),
        entries: &amp;[
            storage_entry(<span class="s">0</span>),
            storage_entry(<span class="s">1</span>),
            storage_entry(<span class="s">2</span>),
            storage_entry(<span class="s">3</span>),
        ],
    })
}

<span class="c">/// Group 1 for the point resolve: depth and color textures.</span>
<span class="k">fn</span> resolve_layout(device: &amp;wgpu::Device) -&gt; wgpu::BindGroupLayout {
    device.create_bind_group_layout(&amp;wgpu::BindGroupLayoutDescriptor {
        label: Some(&quot;<span class="s">splat.resolve.layout</span>&quot;),
        entries: &amp;[
            wgpu::BindGroupLayoutEntry {
                binding: <span class="s">0</span>,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Depth,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: <span class="s">false</span>,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: <span class="s">1</span>,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: <span class="s">false</span> },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: <span class="s">false</span>,
                },
                count: None,
            },
        ],
    })
}</code></pre></div>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> segment_rows: wgpu::BindGroupLayout,</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> points: wgpu::BindGroupLayout, <span class="c">// group 1 for points</span>
    <span class="k">pub</span> resolve: wgpu::BindGroupLayout, <span class="c">// group 1 for the point resolve</span></code></pre></div>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            segment_rows: segment_rows_layout(device),</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            points: points_layout(device),
            resolve: resolve_layout(device),</code></pre></div>
<h2 id="step-7-srcenginegpuuploadrs">Step 7 · src/engine/gpu/upload.rs<a class="anchor" href="#/course/04d-clouds#step-7-srcenginegpuuploadrs" aria-label="Link to this section">#</a></h2>
<p>Add clouds to the upload.</p>
<p><code>lessons/04d/src/engine/gpu/upload.rs</code> · edit · type this</p>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> super::arena::ArenaRows;</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> super::cloud::CloudRows;</code></pre></div>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> glyph: GlyphRows,</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> cloud: CloudRows, <span class="c">// point clouds</span></code></pre></div>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            glyph: GlyphRows::default(),</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            cloud: CloudRows::default(),</code></pre></div>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.glyph.drop_rows();</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.cloud.drop_rows();</code></pre></div>
<h2 id="step-8-srcenginegpumodrs">Step 8 · src/engine/gpu/mod.rs<a class="anchor" href="#/course/04d-clouds#step-8-srcenginegpumodrs" aria-label="Link to this section">#</a></h2>
<p>Add the cloud lane to the GPU owner, resize and frame.</p>
<p><code>lessons/04d/src/engine/gpu/mod.rs</code> · edit · type this</p>
<p>Replaces the lines from <code>pub mod frame;</code> to <code>use buffers::GpuCtx;</code> in <code>lessons/04c/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> <span class="k">mod</span> cloud;
<span class="k">pub</span> <span class="k">mod</span> frame;
<span class="k">pub</span> <span class="k">mod</span> glyphs;
<span class="k">pub</span> <span class="k">mod</span> instance;
<span class="k">pub</span> <span class="k">mod</span> lod;
<span class="k">pub</span> <span class="k">mod</span> objects;
<span class="k">pub</span> <span class="k">mod</span> segments;
<span class="k">pub</span> <span class="k">mod</span> splat;
<span class="k">pub</span> <span class="k">mod</span> targets;
<span class="k">pub</span> <span class="k">mod</span> text_outline;
<span class="k">pub</span> <span class="k">mod</span> upload;
<span class="k">pub</span> <span class="k">mod</span> view;
<span class="k">use</span> <span class="k">crate</span>::engine::pipelines::{Layouts, Target};
<span class="k">use</span> buffers::GpuCtx;
<span class="k">pub</span> <span class="k">use</span> cloud::{CloudDraw, LodNode, NO_NORMALS};</code></pre></div>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> glyphs: glyphs::GlyphLane,</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> cloud: cloud::CloudLane,
    <span class="k">pub</span> splat: splat::Splat,</code></pre></div>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> glyphs = glyphs::GlyphLane::new(&amp;ctx, &amp;layouts, target);</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> cloud = cloud::CloudLane::new(&amp;ctx);
        <span class="k">let</span> splat = splat::Splat::new(&amp;ctx, &amp;layouts, target, cloud.buffers());</code></pre></div>
<p>Replaces the <code>fn set_scene</code> lines in <code>lessons/04c/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            cloud,
            splat,
            bounds: AABB::empty(),
            logical_size: [<span class="s">1</span>.<span class="s">0</span>; <span class="s">2</span>],
            device_type,
        })
    }

    <span class="c">/// Append one upload to every lane.</span>
    <span class="k">pub</span> <span class="k">fn</span> set_scene(&amp;<span class="k">mut</span> <span class="k">self</span>, up: &amp;Upload) {
        <span class="k">self</span>.objects.append(&amp;<span class="k">self</span>.ctx, &amp;<span class="k">self</span>.layouts, &amp;up.obj);
        <span class="k">self</span>.arena.append(&amp;<span class="k">self</span>.ctx, &amp;up.arena);
        <span class="k">self</span>.segments.append(&amp;<span class="k">self</span>.ctx, &amp;<span class="k">self</span>.layouts, &amp;up.seg);
        <span class="k">self</span>.glyphs.append(&amp;<span class="k">self</span>.ctx, &amp;<span class="k">self</span>.layouts, &amp;up.glyph);

        <span class="c">// a moved cloud buffer needs a new bind group</span>
        <span class="k">if</span> <span class="k">self</span>.cloud.append(&amp;<span class="k">self</span>.ctx, &amp;up.cloud) {
            <span class="k">self</span>.splat
                .rebind(&amp;<span class="k">self</span>.ctx, &amp;<span class="k">self</span>.layouts, <span class="k">self</span>.cloud.buffers());
        }

        <span class="k">self</span>.splat.invalidate();</code></pre></div>
<p>Added at the end of <code>fn resize</code> in <code>lessons/04c/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.splat.resize();</code></pre></div>
<p>Replaces the line <code>self.objects.rebase_anchor(&amp;self.ctx, origin, distance, now)</code> in <code>lessons/04c/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> result = <span class="k">self</span>.objects.rebase_anchor(&amp;<span class="k">self</span>.ctx, origin, distance, now);

        <span class="k">if</span> result.moved {
            <span class="k">self</span>.splat.invalidate();
        }

        result</code></pre></div>
<p>Replaces the <code>///</code> line above <code>fn render</code> in <code>lessons/04c/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code></code></pre></div>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> <span class="k">mut</span> encoder = <span class="k">self</span>.ctx.device.create_command_encoder(&amp;Default::default());</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.splat.prelude(
            &amp;<span class="k">self</span>.ctx,
            &amp;<span class="k">self</span>.layouts,
            &amp;<span class="k">mut</span> encoder,
            &amp;splat::RecordCx {
                mvp: &amp;<span class="k">self</span>.frame.mvp_f32,
                ortho_h: <span class="k">self</span>.frame.ortho_h,
                eye: <span class="k">self</span>.frame.eye,
                size: (<span class="k">self</span>.config.width, <span class="k">self</span>.config.height),
                cloud_size: <span class="k">self</span>.view.cloud_size * <span class="k">self</span>.config.width <span class="k">as</span> f32
                    / <span class="k">self</span>.logical_size[<span class="s">0</span>] <span class="k">as</span> f32,
                lod_px: <span class="k">self</span>.view.lod_px,
                objects: &amp;<span class="k">self</span>.objects,
                clouds: &amp;<span class="k">self</span>.cloud.clouds,
                nodes: &amp;<span class="k">self</span>.cloud.nodes,
            },
            &amp;<span class="k">self</span>.frame.cloud_group,
        );</code></pre></div>
<p>Added below</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">self</span>.arena.draw_faces(&amp;<span class="k">mut</span> pass, &amp;basic);</code></pre></div>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">self</span>.splat.draw_resolve(&amp;<span class="k">mut</span> pass, &amp;<span class="k">self</span>.frame.cloud_group);</code></pre></div>
<h2 id="step-9-srcfixturers">Step 9 · src/fixture.rs<a class="anchor" href="#/course/04d-clouds#step-9-srcfixturers" aria-label="Link to this section">#</a></h2>
<p>Copy the test scene: it now has a point grid.
Copy this file from the lesson folder to the path shown.</p>
<p><code>lessons/04d/src/fixture.rs</code> · edit · copy the file</p>
<p>Replaces the line <code>use crate::engine::gpu::{CylinderSegment, GlyphPoint, Ins…</code> in <code>lessons/04c/src/fixture.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> <span class="k">crate</span>::engine::gpu::{
    CloudDraw, CylinderSegment, GlyphPoint, Instance, NO_NORMALS, ObjectRow, Upload,
};</code></pre></div>
<p>Replaces the line <code>for _ in 0..3 {</code> in <code>lessons/04c/src/fixture.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">for</span> _ <span class="k">in</span> <span class="s">0</span>..<span class="s">4</span> {</code></pre></div>
<p>Added after the last marker, before <code>upload</code> is returned, in <code>lessons/04c/src/fixture.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">for</span> y <span class="k">in</span> <span class="s">0</span>..<span class="s">9</span> {
        <span class="k">for</span> x <span class="k">in</span> <span class="s">0</span>..<span class="s">13</span> {
            upload
                .cloud
                .pos
                .extend([<span class="s">0</span>.<span class="s">4</span> + x <span class="k">as</span> f32 * <span class="s">0</span>.<span class="s">1</span>, -<span class="s">1</span>.<span class="s">1</span> + y <span class="k">as</span> f32 * <span class="s">0</span>.<span class="s">07</span>, <span class="s">0</span>.<span class="s">0</span>]);
            upload.cloud.col.push(<span class="s">0xffffaa55</span>);
        }
    }

    upload.cloud.draws.push(CloudDraw {
        instance: <span class="s">3</span>,
        from: <span class="s">0</span>,
        count: <span class="s">117</span>,
        first: <span class="s">0</span>,
        spacing: <span class="s">0</span>.<span class="s">07</span>,
        node_first: <span class="s">0</span>,
        node_count: <span class="s">0</span>,
        nrm_first: NO_NORMALS,
    });
    upload.obj.rows[<span class="s">3</span>].spacing = <span class="s">2</span>.<span class="s">0</span>;</code></pre></div>
<h2 id="step-10-srclibrs">Step 10 · src/lib.rs<a class="anchor" href="#/course/04d-clouds#step-10-srclibrs" aria-label="Link to this section">#</a></h2>
<p>Report the point count from the entry point.</p>
<p><code>lessons/04d/src/lib.rs</code> · edit · type this</p>
<p>Replaces the <code>///</code> line above <code>pub async fn create</code> in <code>lessons/04c/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code></code></pre></div>
<p>Replaces the line <code>&quot;meshVertices&quot;:self.gpu.arena.vert_count(),&quot;segments&quot;:sel…</code> in <code>lessons/04c/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            &quot;<span class="s">meshVertices</span>&quot;:<span class="k">self</span>.gpu.arena.vert_count(),&quot;<span class="s">segments</span>&quot;:<span class="k">self</span>.gpu.segments.ribbon_count(),&quot;<span class="s">dots</span>&quot;:<span class="k">self</span>.gpu.glyphs.dot_count(),&quot;<span class="s">cloudPoints</span>&quot;:<span class="k">self</span>.gpu.cloud.point_count}).to_string())</code></pre></div>
<h2 id="step-11-indexhtml">Step 11 · index.html<a class="anchor" href="#/course/04d-clouds#step-11-indexhtml" aria-label="Link to this section">#</a></h2>
<p>Copy the page: the status now reports cloud points.
Copy this file from the lesson folder to the path shown.</p>
<p><code>lessons/04d/index.html</code> · edit · copy the file</p>
<p>Replaces the line <code>&lt;title&gt;Session checkpoint 04c&lt;/title&gt;</code> in <code>lessons/04c/index.html</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    &lt;title&gt;Session checkpoint 04&lt;/title&gt;</code></pre></div>
<p>Replaces the line <code>&lt;output id=&quot;status&quot;&gt;Starting checkpoint 04c&lt;/output&gt;</code> in <code>lessons/04c/index.html</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    &lt;output id=&quot;<span class="s">status</span>&quot;&gt;Starting checkpoint 04&lt;/output&gt;</code></pre></div>
<p>Replaces the line <code>document.getElementById(&#39;status&#39;).textContent = &#39;Checkpoi…</code> in <code>lessons/04c/index.html</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        document.getElementById('status').textContent = 'Checkpoint 04 · ' + state.objects + ' objects · ' + state.width + '×' + state.height;</code></pre></div>
<h2 id="check">Check<a class="anchor" href="#/course/04d-clouds#check" aria-label="Link to this section">#</a></h2>
<p>Run <code>trunk serve</code> in <code>lessons/04d/</code> and open <a href="http://127.0.0.1:8770/" target="_blank" rel="noopener">http://127.0.0.1:8770/</a>.</p>
<p>Expected: A grid of blue points appears beside the mesh, polyline and marker; status: <strong>Checkpoint 04 · 4 objects</strong>.</p>
<p><img src="/session/docs/course/docs/screenshots/04d.png" alt="Checkpoint 04d: a point cloud through the splat prelude and resolve, beside the mesh, stroke and marker lanes." loading="lazy" decoding="async"></p>
<p>If it fails:</p>
<ul>
<li>A cloud paints over a solid: the resolve does not write the winning point depth.</li>
<li>Points change size on a high-DPI screen: CSS pixels and framebuffer pixels are mixed.</li>
</ul>
<h2 id="what-changed">What changed<a class="anchor" href="#/course/04d-clouds#what-changed" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>lessons/04d/src/engine/
├── gpu/
│   ├── arena.rs
│   ├── buffers.rs
│   ├── cloud.rs  +
│   ├── frame.rs
│   ├── glyphs.rs
│   ├── instance.rs
│   ├── lod.rs  +
│   ├── mod.rs  ~
│   ├── objects.rs
│   ├── segments.rs
│   ├── splat.rs  +
│   ├── targets.rs
│   ├── text_outline.rs
│   ├── upload.rs  ~
│   └── view.rs
├── pipelines/
│   ├── layouts.rs  ~
│   └── mod.rs
└── mod.rs</code></pre></div>
<p><code>+</code> new in this lesson · <code>~</code> changed in this lesson</p>
<p>Data flow: source files → retained scene state → GPU buffers → visible result. Every file at this point: <code>lessons/04d/</code>.</p>
<h2 id="next">Next<a class="anchor" href="#/course/04d-clouds#next" aria-label="Link to this section">#</a></h2>
<p><a href="#/course/05-visibility">05 · Depth and visible ink</a></p>
<h2 id="expected-viewer-result">Expected viewer result<a class="anchor" href="#/course/04d-clouds#expected-viewer-result" aria-label="Link to this section">#</a></h2>
<p>Checkpoint 04d: a point cloud through the splat prelude and resolve, beside the mesh, stroke and marker lanes.</p>
<p><a href="/session/docs/course/docs/screenshots/04d.png"><img src="/session/docs/course/docs/screenshots/04d.png" alt="Full viewer result for 04d clouds" loading="lazy" decoding="async"></a></p>
`,toc:[{level:2,id:"step-1-srcenginegpucloudrs",text:"Step 1 · src/engine/gpu/cloud.rs"},{level:2,id:"step-2-srcenginegpulodrs",text:"Step 2 · src/engine/gpu/lod.rs"},{level:2,id:"step-3-srcenginegpusplatrs",text:"Step 3 · src/engine/gpu/splat.rs"},{level:2,id:"step-4-srcshaderssplatwgsl",text:"Step 4 · src/shaders/splat.wgsl"},{level:2,id:"step-5-srcshaderssplat_resolvewgsl",text:"Step 5 · src/shaders/splat_resolve.wgsl"},{level:2,id:"step-6-srcenginepipelineslayoutsrs",text:"Step 6 · src/engine/pipelines/layouts.rs"},{level:2,id:"step-7-srcenginegpuuploadrs",text:"Step 7 · src/engine/gpu/upload.rs"},{level:2,id:"step-8-srcenginegpumodrs",text:"Step 8 · src/engine/gpu/mod.rs"},{level:2,id:"step-9-srcfixturers",text:"Step 9 · src/fixture.rs"},{level:2,id:"step-10-srclibrs",text:"Step 10 · src/lib.rs"},{level:2,id:"step-11-indexhtml",text:"Step 11 · index.html"},{level:2,id:"check",text:"Check"},{level:2,id:"what-changed",text:"What changed"},{level:2,id:"next",text:"Next"},{level:2,id:"expected-viewer-result",text:"Expected viewer result"}]};export{s as default};
