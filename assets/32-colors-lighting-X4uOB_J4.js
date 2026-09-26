const s={title:"32 · Finish the command workspace and soft ambient lighting",html:`<h1 id="32-finish-the-command-workspace-and-soft-ambient-lighting">32 · Finish the command workspace and soft ambient lighting<a class="anchor" href="#/course/32-colors-lighting#32-finish-the-command-workspace-and-soft-ambient-lighting" aria-label="Link to this section">#</a></h1>
<p>A command field below the model controls grouped selection, responsive surface editing, independent colors and soft ambient shadows.</p>
<h2 id="step-1-indexhtml">Step 1 · index.html<a class="anchor" href="#/course/32-colors-lighting#step-1-indexhtml" aria-label="Link to this section">#</a></h2>
<p>Copy this file from the lesson folder to the path shown.</p>
<p><code>lessons/32/index.html</code> · edit · copy the file</p>
<p>Replaces the line <code>background: #000;</code> in <code>lessons/31/index.html</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        background: #f0f0f0;</code></pre></div>
<p>Replaces the line <code>top: 0;</code> in <code>lessons/31/index.html</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        bottom: 0;
        right: 0;
        width: 0;
        height: 0;
        z-index: 10;
        display: block;
        border-bottom: 20px solid #111;
        border-left: 20px solid transparent;
        transition: border-bottom-width .25s cubic-bezier(.2, .8, .3, 1.25), border-left-width .25s cubic-bezier(.2, .8, .3, 1.25);
      }
      #viewer-docs:hover, #viewer-docs:focus-visible {
        border-bottom-width: 26px;
        border-left-width: 26px;</code></pre></div>
<p>Replaces the line <code>&lt;canvas id=&quot;canvas&quot; tabindex=&quot;0&quot; role=&quot;application&quot; aria-…</code> in <code>lessons/31/index.html</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    &lt;canvas id=&quot;<span class="s">canvas</span>&quot; tabindex=&quot;<span class="s">0</span>&quot; role=&quot;<span class="s">application</span>&quot; aria-label=&quot;<span class="s">3D viewer. Click selects, Shift-click adds to selection, right drag orbits. Type Layers On in the command line to show layers. Enter executes and Escape cancels.</span>&quot;&gt;&lt;/canvas&gt;</code></pre></div>
<h2 id="step-2-srcappdeformrs">Step 2 · src/app/deform.rs<a class="anchor" href="#/course/32-colors-lighting#step-2-srcappdeformrs" aria-label="Link to this section">#</a></h2>
<p>Expose mesh vertex keys and retain shared-vertex behavior for component moves.</p>
<p><code>lessons/32/src/app/deform.rs</code> · edit · type this</p>
<p>Replaces the line <code>fn mesh_keys(mesh: &amp;Mesh, target: Target) -&gt; Result&lt;Vec&lt;u…</code> in <code>lessons/31/src/app/deform.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// The mesh vertex keys a target covers.</span>
<span class="k">pub</span>(<span class="k">crate</span>) <span class="k">fn</span> mesh_keys(mesh: &amp;Mesh, target: Target) -&gt; Result&lt;Vec&lt;usize&gt;, String&gt; {</code></pre></div>
<p>Replaces the 2 lines from <code>mesh.triangulation.clear();</code> in <code>lessons/31/src/app/deform.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            mesh.clear_triangle_bvh(); <span class="c">// stale after moving vertices</span></code></pre></div>
<h2 id="step-3-srcappfeedbackrs">Step 3 · src/app/feedback.rs<a class="anchor" href="#/course/32-colors-lighting#step-3-srcappfeedbackrs" aria-label="Link to this section">#</a></h2>
<p>Carry status and layer information from the scene into the interface.</p>
<p><code>lessons/32/src/app/feedback.rs</code> · edit · type this</p>
<p>Replaces the line <code>#[derive(Clone, Default)]</code> in <code>lessons/31/src/app/feedback.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// One row of the layers panel.</span>
#[derive(Clone, Default, serde::Serialize)]
<span class="k">pub</span> <span class="k">struct</span> LayerRow {
    <span class="k">pub</span> key: String,                 <span class="c">// unique id of the row</span>
    <span class="k">pub</span> label: String,               <span class="c">// text shown</span>
    <span class="k">pub</span> count: usize,                <span class="c">// objects under it</span>
    <span class="k">pub</span> hidden: bool,                <span class="c">// eye toggled off</span>
    <span class="k">pub</span> locked: bool,                <span class="c">// not editable</span>
    <span class="k">pub</span> selected: bool,              <span class="c">// highlighted</span>
    <span class="k">pub</span> color: Option&lt;[u8; 3]&gt;,      <span class="c">// face colour swatch</span>
    <span class="k">pub</span> edge_color: Option&lt;[u8; 3]&gt;, <span class="c">// edge colour swatch</span>
    <span class="k">pub</span> has_faces: bool,             <span class="c">// shows a face swatch</span></code></pre></div>
<h2 id="step-4-srcappinspectionrs">Step 4 · src/app/inspection.rs<a class="anchor" href="#/course/32-colors-lighting#step-4-srcappinspectionrs" aria-label="Link to this section">#</a></h2>
<p>Expose the new selection and resource state to the browser inspection data.</p>
<p><code>lessons/32/src/app/inspection.rs</code> · edit · type this</p>
<p>Added after the line <code>});</code> in <code>lessons/31/src/app/inspection.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    snapshot[&quot;<span class="s">selected_rows</span>&quot;] = serde_json::json!(state.selected_rows());
    snapshot[&quot;<span class="s">selected_models</span>&quot;] = serde_json::json!(
        state
            .selected_rows()
            .iter()
            .map(|r| state.gpu.objects.anchored_model(*r))
            .collect::&lt;Vec&lt;_&gt;&gt;()
    );
    snapshot[&quot;<span class="s">drawing</span>&quot;] = state.drawing_status();
    snapshot[&quot;<span class="s">snap_enabled</span>&quot;] = serde_json::json!(state.snap_enabled);
    snapshot[&quot;<span class="s">ssao</span>&quot;] = serde_json::json!(state.gpu.view.ssao);
    snapshot[&quot;<span class="s">locked_count</span>&quot;] = serde_json::json!(state.scene.locked.len());
    snapshot[&quot;<span class="s">color_count</span>&quot;] = serde_json::json!(state.scene.colors.len());
    snapshot[&quot;<span class="s">edge_color_count</span>&quot;] = serde_json::json!(state.scene.edge_colors.len());</code></pre></div>
<h2 id="step-5-srcappmesh_previewrs">Step 5 · src/app/mesh_preview.rs<a class="anchor" href="#/course/32-colors-lighting#step-5-srcappmesh_previewrs" aria-label="Link to this section">#</a></h2>
<p>Cache source-to-render vertex mappings and update the affected neighborhood during a drag.</p>
<p><code>lessons/32/src/app/mesh_preview.rs</code> · 319 lines · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> super::deform::{Target, mesh_keys};
<span class="k">use</span> <span class="k">crate</span>::engine::gpu::glyphs::GlyphPoint;
<span class="k">use</span> <span class="k">crate</span>::engine::gpu::patch::{Counts, Span};
<span class="k">use</span> <span class="k">crate</span>::engine::gpu::segments::{CylinderSegment, SegRows};
<span class="k">use</span> <span class="k">crate</span>::engine::gpu::{Gpu, Upload};
<span class="k">use</span> session_rust::{Geometry, Mesh, RenderVertex, Xform};
<span class="k">use</span> std::collections::{HashMap, HashSet};

<span class="c">/// A mesh's GPU rows, each tagged with its source vertex key.</span>
<span class="k">pub</span> <span class="k">struct</span> MeshPreview {
    vertices: Vec&lt;(usize, RenderVertex)&gt;,       <span class="c">// (vertex key, GPU vertex)</span>
    pipes: Vec&lt;([usize; 2], CylinderSegment)&gt;, <span class="c">// (edge keys, GPU pipe)</span>
    spheres: Vec&lt;(usize, GlyphPoint)&gt;,         <span class="c">// (vertex key, vertex sphere)</span>
    dots: Vec&lt;(usize, GlyphPoint)&gt;,            <span class="c">// (vertex key, dot)</span>
    span: Span,                                <span class="c">// where the rows sit on the GPU</span>
}

<span class="c">/// The GPU rows one drag touches.</span>
<span class="k">pub</span> <span class="k">struct</span> Gesture {
    vertices: Vec&lt;(u32, RenderVertex, bool)&gt;,     <span class="c">// (GPU index, original, moves)</span>
    pipes: Vec&lt;(u32, CylinderSegment, [bool; 2])&gt;, <span class="c">// (GPU index, original, each end moves)</span>
    spheres: Vec&lt;(u32, GlyphPoint, bool)&gt;,        <span class="c">// (GPU index, original, moves)</span>
    dots: Vec&lt;(u32, GlyphPoint, bool)&gt;,           <span class="c">// (GPU index, original, moves)</span>
}

<span class="c">/// The mesh inside a geometry, if any.</span>
<span class="k">pub</span> <span class="k">fn</span> mesh(geometry: &amp;Geometry) -&gt; Option&lt;&amp;Mesh&gt; {
    <span class="k">match</span> geometry {
        Geometry::Mesh(mesh) =&gt; Some(mesh),
        Geometry::Element(element) =&gt; <span class="k">match</span> element.geometry() {
            session_rust::element::ElementGeometry::Mesh(mesh) =&gt; Some(mesh),
            _ =&gt; None,
        },
        _ =&gt; None,
    }
}

<span class="k">impl</span> MeshPreview {
    <span class="c">/// Match the uploaded rows back to the mesh's vertex keys.</span>
    <span class="k">pub</span>(<span class="k">crate</span>) <span class="k">fn</span> capture(
        up: &amp;Upload,
        span: Span,
        local: Counts,
        geometry: &amp;Geometry,
    ) -&gt; Option&lt;<span class="k">Self</span>&gt; {
        <span class="k">let</span> mesh = mesh(geometry)?;

        <span class="k">if</span> span.count.ribbons != <span class="s">0</span> {
            <span class="k">return</span> None; <span class="c">// not a plain mesh upload</span>
        }

        <span class="k">let</span> <span class="k">mut</span> keys = mesh.vertices(); <span class="c">// one GPU vertex per key</span>

        <span class="c">// face colours split every triangle: keys per triangle corner</span>
        <span class="k">if</span> mesh.color_mode == session_rust::mesh::ColorMode::FACECOLORS
            &amp;&amp; mesh.get_facecolors().len() == mesh.face.len()
        {
            keys.clear();

            <span class="k">for</span> face <span class="k">in</span> mesh.faces() {
                <span class="k">let</span> ring = &amp;mesh.face[&amp;face];
                <span class="k">let</span> triangles = mesh
                    .triangulation
                    .get(&amp;face)
                    .filter(|t| !t.is_empty())
                    .cloned()
                    .unwrap_or_else(|| {
                        (<span class="s">1</span>..ring.len().saturating_sub(<span class="s">1</span>))
                            .map(|i| [ring[<span class="s">0</span>], ring[i], ring[i + <span class="s">1</span>]])
                            .collect()
                    });

                <span class="k">for</span> triangle <span class="k">in</span> triangles {
                    <span class="k">if</span> triangle.iter().all(|key| mesh.vertex.contains_key(key)) {
                        keys.extend(triangle);
                    }
                }
            }
        }

        <span class="k">if</span> keys.len() != span.count.verts <span class="k">as</span> usize {
            <span class="k">return</span> None; <span class="c">// upload does not match</span>
        }

        <span class="k">let</span> vertices = keys
            .into_iter()
            .zip(
                up.arena.verts[local.verts <span class="k">as</span> usize..(local.verts + span.count.verts) <span class="k">as</span> usize]
                    .iter()
                    .copied(),
            )
            .collect();
        <span class="c">// edges in the order the walk numbered them</span>
        <span class="k">let</span> <span class="k">mut</span> seen = HashSet::new();
        <span class="k">let</span> <span class="k">mut</span> edges = Vec::new();

        <span class="k">for</span> face <span class="k">in</span> mesh.faces() {
            <span class="k">let</span> ring = &amp;mesh.face[&amp;face];

            <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..ring.len() {
                <span class="k">let</span> pair = [
                    ring[i].min(ring[(i + <span class="s">1</span>) % ring.len()]),
                    ring[i].max(ring[(i + <span class="s">1</span>) % ring.len()]),
                ];

                <span class="k">if</span> seen.insert(pair) {
                    edges.push(pair);
                }
            }
        }

        <span class="k">let</span> pipes = (local.pipes..local.pipes + span.count.pipes)
            .map(|i| {
                Some((
                    *edges.get(*up.seg.pipe_ids.get(i <span class="k">as</span> usize)? <span class="k">as</span> usize)?,
                    up.seg.pipes[i <span class="k">as</span> usize],
                ))
            })
            .collect::&lt;Option&lt;Vec&lt;_&gt;&gt;&gt;()?;
        <span class="c">// markers are matched by position; a shared position gives up</span>
        <span class="k">let</span> <span class="k">mut</span> positions = HashMap::new();

        <span class="k">for</span> (&amp;key, v) <span class="k">in</span> &amp;mesh.vertex {
            <span class="k">let</span> p = [v.x <span class="k">as</span> f32, v.y <span class="k">as</span> f32, v.z <span class="k">as</span> f32].map(f32::to_bits);
            positions
                .entry(p)
                .and_modify(|value| *value = None)
                .or_insert(Some(key));
        }

        <span class="k">let</span> markers = |items: &amp;[GlyphPoint]| {
            items
                .iter()
                .map(|g| Some(((*positions.get(&amp;g.center.map(f32::to_bits))?)?, *g)))
                .collect::&lt;Option&lt;Vec&lt;_&gt;&gt;&gt;()
        };
        Some(<span class="k">Self</span> {
            vertices,
            pipes,
            spheres: markers(
                &amp;up.glyph.spheres
                    [local.spheres <span class="k">as</span> usize..(local.spheres + span.count.spheres) <span class="k">as</span> usize],
            )?,
            dots: markers(
                &amp;up.glyph.dots[local.dots <span class="k">as</span> usize..(local.dots + span.count.dots) <span class="k">as</span> usize],
            )?,
            span,
        })
    }

    <span class="c">/// The rows a drag of \`target\` touches.</span>
    <span class="k">pub</span> <span class="k">fn</span> begin(&amp;<span class="k">self</span>, geometry: &amp;Geometry, target: Target) -&gt; Option&lt;Gesture&gt; {
        <span class="k">let</span> mesh = mesh(geometry)?;
        <span class="k">let</span> selected: HashSet&lt;_&gt; = mesh_keys(mesh, target).ok()?.into_iter().collect(); <span class="c">// keys that move</span>
        <span class="k">let</span> <span class="k">mut</span> affected = selected.clone(); <span class="c">// keys whose faces change</span>

        <span class="k">for</span> ring <span class="k">in</span> mesh.face.values() {
            <span class="k">if</span> ring.iter().any(|k| selected.contains(k)) {
                affected.extend(ring);
            }
        }

        <span class="k">let</span> vertices = <span class="k">self</span>
            .vertices
            .iter()
            .enumerate()
            .filter(|(_, (k, _))| affected.contains(k))
            .map(|(i, (k, v))| (<span class="k">self</span>.span.start.verts + i <span class="k">as</span> u32, *v, selected.contains(k)))
            .collect();
        <span class="k">let</span> pipes = <span class="k">self</span>
            .pipes
            .iter()
            .enumerate()
            .filter(|(_, (keys, _))| keys.iter().any(|k| affected.contains(k)))
            .map(|(i, (keys, p))| {
                (
                    <span class="k">self</span>.span.start.pipes + i <span class="k">as</span> u32,
                    *p,
                    keys.map(|k| selected.contains(&amp;k)),
                )
            })
            .collect();
        <span class="k">let</span> markers = |items: &amp;[(usize, GlyphPoint)], start| {
            items
                .iter()
                .enumerate()
                .filter(|(_, (k, _))| affected.contains(k))
                .map(|(i, (k, g))| (start + i <span class="k">as</span> u32, *g, selected.contains(k)))
                .collect()
        };
        Some(Gesture {
            vertices,
            pipes,
            spheres: markers(&amp;<span class="k">self</span>.spheres, <span class="k">self</span>.span.start.spheres),
            dots: markers(&amp;<span class="k">self</span>.dots, <span class="k">self</span>.span.start.dots),
        })
    }

    <span class="c">/// Memory held by the preview.</span>
    <span class="k">pub</span> <span class="k">fn</span> allocated_bytes(&amp;<span class="k">self</span>) -&gt; usize {
        <span class="k">self</span>.vertices.capacity() * std::mem::size_of::&lt;(usize, RenderVertex)&gt;()
            + <span class="k">self</span>.pipes.capacity() * std::mem::size_of::&lt;([usize; <span class="s">2</span>], CylinderSegment)&gt;()
            + (<span class="k">self</span>.spheres.capacity() + <span class="k">self</span>.dots.capacity())
                * std::mem::size_of::&lt;(usize, GlyphPoint)&gt;()
    }
}

<span class="k">impl</span> Gesture {
    <span class="c">/// Patch the GPU rows with \`delta\` applied, or put them back.</span>
    <span class="k">pub</span> <span class="k">fn</span> apply(&amp;<span class="k">self</span>, gpu: &amp;<span class="k">mut</span> Gpu, delta: &amp;Xform, restore: bool) {
        <span class="c">// transform a point by the 4x4 matrix</span>
        <span class="k">let</span> position = |p: [f32; <span class="s">3</span>]| {
            <span class="k">let</span> m = &amp;delta.m;
            std::array::from_fn(|r| {
                (m[r] * p[<span class="s">0</span>] <span class="k">as</span> f64 + m[<span class="s">4</span> + r] * p[<span class="s">1</span>] <span class="k">as</span> f64 + m[<span class="s">8</span> + r] * p[<span class="s">2</span>] <span class="k">as</span> f64 + m[<span class="s">12</span> + r])
                    <span class="k">as</span> f32
            })
        };

        <span class="k">for</span> &amp;(index, <span class="k">mut</span> v, moved) <span class="k">in</span> &amp;<span class="k">self</span>.vertices {
            <span class="k">if</span> !restore {
                <span class="k">if</span> moved {
                    v.position = position(v.position);
                }

                v.normal = [<span class="s">0</span>.; <span class="s">3</span>]; <span class="c">// flat shading while dragging</span>
            }

            gpu.arena.patch_vertices(&amp;gpu.ctx, index, &amp;[v]);
        }

        <span class="k">for</span> &amp;(index, <span class="k">mut</span> p, moved) <span class="k">in</span> &amp;<span class="k">self</span>.pipes {
            <span class="k">if</span> !restore {
                <span class="k">if</span> moved[<span class="s">0</span>] {
                    p.p0 = position(p.p0);
                }

                <span class="k">if</span> moved[<span class="s">1</span>] {
                    p.p1 = position(p.p1);
                }

                p.facing = u32::MAX; <span class="c">// always draw while dragging</span>
            }

            gpu.segments.patch_pipes(
                &amp;gpu.ctx,
                index,
                &amp;SegRows {
                    pipes: vec![p],
                    ..Default::default()
                },
            );
        }

        <span class="k">for</span> (items, sphere) <span class="k">in</span> [(&amp;<span class="k">self</span>.spheres, <span class="s">true</span>), (&amp;<span class="k">self</span>.dots, <span class="s">false</span>)] {
            <span class="k">for</span> &amp;(index, <span class="k">mut</span> g, moved) <span class="k">in</span> items {
                <span class="k">if</span> !restore {
                    <span class="k">if</span> moved {
                        g.center = position(g.center);
                    }

                    g.facing = u32::MAX;
                    g.facing_ext = [u32::MAX; <span class="s">2</span>];
                }

                gpu.glyphs.patch_marker(&amp;gpu.ctx, index, sphere, g);
            }
        }

        gpu.objects.geometry_changed();
    }
}

#[cfg(test)]
<span class="k">mod</span> tests {
    <span class="k">use</span> super::*;
    <span class="k">use</span> <span class="k">crate</span>::app::walk::{Walk, WalkCx, walk_geometry};
    <span class="k">use</span> session_rust::Point;
    <span class="k">use</span> std::rc::Rc;

    <span class="c">/// A drag touches only the rows around the target.</span>
    #[test]
    <span class="k">fn</span> large_mesh_gesture_only_patches_local_neighborhood() {
        <span class="k">let</span> <span class="k">mut</span> mesh = Mesh::new();

        <span class="k">for</span> y <span class="k">in</span> <span class="s">0</span>..<span class="s">101</span> {
            <span class="k">for</span> x <span class="k">in</span> <span class="s">0</span>..<span class="s">101</span> {
                mesh.add_vertex(Point::new(x <span class="k">as</span> f64, y <span class="k">as</span> f64, <span class="s">0</span>.), Some(y * <span class="s">101</span> + x));
            }
        }

        <span class="k">for</span> y <span class="k">in</span> <span class="s">0</span>..<span class="s">100</span> {
            <span class="k">for</span> x <span class="k">in</span> <span class="s">0</span>..<span class="s">100</span> {
                <span class="k">let</span> a = y * <span class="s">101</span> + x;
                mesh.add_face(vec![a, a + <span class="s">1</span>, a + <span class="s">102</span>], None);
                mesh.add_face(vec![a, a + <span class="s">102</span>, a + <span class="s">101</span>], None);
            }
        }

        <span class="k">let</span> geometry = Geometry::Mesh(Rc::new(mesh));
        <span class="k">let</span> <span class="k">mut</span> up = Upload::default();
        walk_geometry(
            &amp;<span class="k">mut</span> Walk::of(&amp;<span class="k">mut</span> up),
            &amp;WalkCx {
                vert_base: <span class="s">0</span>,
                cloud_px: <span class="s">0</span>.,
                row: <span class="s">0</span>,
            },
            &amp;geometry,
        );
        <span class="k">let</span> span = Span {
            start: Counts::default(),
            count: Counts::of(&amp;up),
        };
        <span class="k">let</span> preview = MeshPreview::capture(&amp;up, span, Counts::default(), &amp;geometry).unwrap();
        <span class="k">let</span> gesture = preview.begin(&amp;geometry, Target::Face(<span class="s">10000</span>)).unwrap();
        assert!(
            gesture.vertices.len() &lt; <span class="s">30</span>,
            &quot;<span class="s">drag cost follows adjacency, not the 10,201-vertex mesh</span>&quot;
        );
        assert_eq!(gesture.vertices.iter().filter(|v| v.<span class="s">2</span>).count(), <span class="s">3</span>);
        assert!(gesture.pipes.len() &lt; <span class="s">100</span>);
        assert_eq!(
            super::mesh(&amp;geometry).unwrap().vertex[&amp;<span class="s">0</span>].z,
            <span class="s">0</span>.,
            &quot;<span class="s">preview does not mutate source</span>&quot;
        );
    }
}</code></pre></div>
<h2 id="step-6-srcappmodrs">Step 6 · src/app/mod.rs<a class="anchor" href="#/course/32-colors-lighting#step-6-srcappmodrs" aria-label="Link to this section">#</a></h2>
<p>Declare the new application modules so their files join the crate.</p>
<p><code>lessons/32/src/app/mod.rs</code> · edit · type this</p>
<p>Added after the line <code>pub mod manifest;</code> in <code>lessons/31/src/app/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> <span class="k">mod</span> mesh_preview;</code></pre></div>
<h2 id="step-7-srcappsceners">Step 7 · src/app/scene.rs<a class="anchor" href="#/course/32-colors-lighting#step-7-srcappsceners" aria-label="Link to this section">#</a></h2>
<p>Retain mesh preview caches and separate face and edge overrides by source identity.</p>
<p><code>lessons/32/src/app/scene.rs</code> · edit · type this</p>
<p>Added after the line <code>pub colors: HashMap&lt;(usize, Rc&lt;str&gt;), [u8; 3]&gt;,</code> in <code>lessons/31/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> edge_colors: HashMap&lt;(usize, Rc&lt;str&gt;), [u8; 3]&gt;, <span class="c">// edge colour overrides</span></code></pre></div>
<p>Added after the line <code>surface_previews: Vec&lt;Option&lt;crate::app::surface_preview:…</code> in <code>lessons/31/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span>(<span class="k">crate</span>) mesh_previews: Vec&lt;Option&lt;<span class="k">crate</span>::app::mesh_preview::MeshPreview&gt;&gt;, <span class="c">// per row, for live mesh edits</span></code></pre></div>
<p>Added after the line <code>colors: HashMap::new(),</code> in <code>lessons/31/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            edge_colors: HashMap::new(),</code></pre></div>
<p>Added after the line <code>surface_previews: Vec::new(),</code> in <code>lessons/31/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            mesh_previews: Vec::new(),</code></pre></div>
<p>Added after the line <code>self.colors.clear();</code> in <code>lessons/31/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.edge_colors.clear();</code></pre></div>
<p>Added after the line <code>self.surface_previews.clear();</code> in <code>lessons/31/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.mesh_previews.clear();</code></pre></div>
<p>Added after the line <code>}</code> in <code>lessons/31/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">if</span> <span class="k">let</span> Some(color) = <span class="k">self</span>.edge_colors.get(&amp;(owner, Rc::from(guid))) {
            <span class="k">let</span> object = <span class="k">self</span>.tables.obj.rows.last_mut().expect(&quot;<span class="s">row just appended</span>&quot;);
            object.edge_color = u32::from_le_bytes([color[<span class="s">0</span>], color[<span class="s">1</span>], color[<span class="s">2</span>], <span class="s">255</span>]);
            object.flags |= Instance::FLAG_EDGE_COLOR;
        }</code></pre></div>
<p>Added after the line <code>self.surface_previews.push(None);</code> in <code>lessons/31/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.mesh_previews.push(None);</code></pre></div>
<p>Added after the line <code>);</code> in <code>lessons/31/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">self</span>.mesh_previews[row <span class="k">as</span> usize] = <span class="k">crate</span>::app::mesh_preview::MeshPreview::capture(
                &amp;<span class="k">self</span>.tables,
                span,
                start.minus(<span class="k">self</span>.uploaded),
                geom,
            );
            <span class="k">let</span> o = <span class="k">self</span>.tables.obj.rows.last_mut().unwrap();
            o.flags |= r.flags;
            o.bounds = r.bounds;
            o.spacing = r.spacing;
            o.faces = r.faces;

            <span class="k">if</span> r.faces {
                o.flags |= Instance::FLAG_HAS_FACES;
            }</code></pre></div>
<p>Added after the line <code>}</code> in <code>lessons/31/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.mesh_previews[row <span class="k">as</span> usize] =
            <span class="k">crate</span>::app::mesh_preview::MeshPreview::capture(&amp;up, span, Counts::default(), geometry);</code></pre></div>
<p>Added after the line <code>.sum::&lt;usize&gt;()</code> in <code>lessons/31/src/app/scene.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            + <span class="k">self</span>
                .mesh_previews
                .iter()
                .flatten()
                .map(|p| p.allocated_bytes())
                .sum::&lt;usize&gt;()</code></pre></div>
<h2 id="step-8-srcappsession_iors">Step 8 · src/app/session_io.rs<a class="anchor" href="#/course/32-colors-lighting#step-8-srcappsession_iors" aria-label="Link to this section">#</a></h2>
<p>Save and restore the two independent display color channels.</p>
<p><code>lessons/32/src/app/session_io.rs</code> · edit · type this</p>
<p>Added after the line <code>colors: Vec&lt;(usize, String, [u8; 3])&gt;,</code> in <code>lessons/31/src/app/session_io.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    #[serde(default)]
    edge_colors: Option&lt;Vec&lt;(usize, String, [u8; 3])&gt;&gt;, <span class="c">// edge colour overrides, None in old files</span></code></pre></div>
<p>Added after the line <code>colors,</code> in <code>lessons/31/src/app/session_io.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        edge_colors: Some({
            <span class="k">let</span> <span class="k">mut</span> colors: Vec&lt;_&gt; = scene
                .edge_colors
                .iter()
                .map(|((doc, id), color)| (*doc, id.to_string(), *color))
                .collect();
            colors.sort();
            colors
        }),</code></pre></div>
<p>Added after the line <code>.map(|(doc, id)| (doc, Rc::from(id)))</code> in <code>lessons/31/src/app/session_io.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">// old files have one colour for faces and edges</span>
    scene.edge_colors = metadata
        .edge_colors
        .unwrap_or_else(|| metadata.colors.clone())
        .into_iter()
        .map(|(doc, id, color)| ((doc, Rc::from(id)), color))
        .collect();</code></pre></div>
<p>Added after the line <code>.insert(scene.identity_of(1).unwrap(), [240, 80, 30]);</code> in <code>lessons/31/src/app/session_io.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        scene
            .edge_colors
            .insert(scene.identity_of(<span class="s">1</span>).unwrap(), [<span class="s">30</span>, <span class="s">80</span>, <span class="s">240</span>]);</code></pre></div>
<p>Added after the line <code>assert_eq!(restored.colors, scene.colors);</code> in <code>lessons/31/src/app/session_io.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        assert_eq!(restored.edge_colors, scene.edge_colors);
        <span class="k">let</span> <span class="k">mut</span> archive = Archive::decode(&amp;bytes[MAGIC.len()..]).unwrap();
        <span class="k">let</span> <span class="k">mut</span> old: serde_json::Value = serde_json::from_slice(&amp;archive.metadata).unwrap();
        old.as_object_mut().unwrap().remove(&quot;<span class="s">edge_colors</span>&quot;);
        archive.metadata = serde_json::to_vec(&amp;old).unwrap();
        <span class="k">let</span> <span class="k">mut</span> legacy = MAGIC.to_vec();
        archive.encode(&amp;<span class="k">mut</span> legacy).unwrap();
        <span class="k">let</span> legacy = open(&amp;legacy).unwrap();
        assert_eq!(
            legacy.edge_colors, legacy.colors,
            &quot;<span class="s">legacy overrides still color both channels</span>&quot;
        );</code></pre></div>
<h2 id="step-9-srcappsplittingrs">Step 9 · src/app/splitting.rs<a class="anchor" href="#/course/32-colors-lighting#step-9-srcappsplittingrs" aria-label="Link to this section">#</a></h2>
<p>Carry face and edge overrides onto split results.</p>
<p><code>lessons/32/src/app/splitting.rs</code> · edit · type this</p>
<p>Added after the line <code>let color = self.colors.get(&amp;(doc, Rc::clone(&amp;guid))).cop…</code> in <code>lessons/31/src/app/splitting.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> edge_color = <span class="k">self</span>.edge_colors.get(&amp;(doc, Rc::clone(&amp;guid))).copied();</code></pre></div>
<p>Replaces the line <code>self.colors.insert((doc, Rc::from(id)), color);</code> in <code>lessons/31/src/app/splitting.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                <span class="k">self</span>.colors.insert((doc, Rc::from(id.as_str())), color);
            }

            <span class="k">if</span> <span class="k">let</span> Some(color) = edge_color {
                <span class="k">self</span>.edge_colors.insert((doc, Rc::from(id.as_str())), color);</code></pre></div>
<h2 id="step-10-srcappuirs">Step 10 · src/app/ui.rs<a class="anchor" href="#/course/32-colors-lighting#step-10-srcappuirs" aria-label="Link to this section">#</a></h2>
<p>Place the command input below its output window with full-width dividers and inline options.</p>
<p><code>lessons/32/src/app/ui.rs</code> · edit · type this</p>
<p>Added after the line <code>use winit::window::Window;</code> in <code>lessons/31/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// Every panel reads and writes this one struct, because an immediate-mode frame keeps no widget state of its own.</span>
#[derive(Default)]
<span class="k">pub</span> <span class="k">struct</span> Model {
    <span class="k">pub</span> layers_open: bool,                          <span class="c">// layers panel shown</span>
    <span class="k">pub</span> rows: Vec&lt;LayerRow&gt;,                        <span class="c">// its rows</span>
    <span class="k">pub</span> command_open: bool,                         <span class="c">// command line shown</span>
    <span class="k">pub</span> command: String,                            <span class="c">// text in the command field</span>
    <span class="k">pub</span> drawing_prompt: String,
    <span class="k">pub</span> focus_command: bool,                        <span class="c">// give the field focus next frame</span>
    <span class="k">pub</span> status: String,                             <span class="c">// status line text</span>
    history: VecDeque&lt;String&gt;,                      <span class="c">// past commands and answers</span>
    command_collapsed: bool,
    layers_collapsed: bool,
    completion: usize,                              <span class="c">// highlighted completion index</span>
    completion_prefix: String,
    inline_suffix: bool,                            <span class="c">// completion suffix shown in the field</span>
    completion_visible: bool,
    completion_rect: Option&lt;egui::Rect&gt;, <span class="c">// where the completion list was drawn</span>
    command_rect: Option&lt;egui::Rect&gt;, <span class="c">// where the command field was drawn</span></code></pre></div>
<p>Replaces the 4 lines from <code>pub fn new(window: &amp;Window, logical_width: f64) -&gt; Self {</code> in <code>lessons/31/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Set up egui with the light theme.</span>
    <span class="k">pub</span> <span class="k">fn</span> new(window: &amp;Window, _logical_width: f64) -&gt; <span class="k">Self</span> {
        <span class="k">let</span> context = egui::Context::default();
        <span class="c">// one layout pass, so text events are never replayed</span>
        context.options_mut(|options| options.max_passes = <span class="s">1</span>.try_into().unwrap());
        context.set_theme(egui::Theme::Light);
        context.set_visuals(visuals());
        MODEL.with_borrow_mut(|model| model.focus_command = <span class="s">true</span>);</code></pre></div>
<p>Added after the line <code>let mut consumed = response.consumed;</code> in <code>lessons/31/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> in_popup =
            |point| MODEL.with_borrow(|m| m.completion_rect.is_some_and(|r| r.contains(point)));

        <span class="k">match</span> event {
            WindowEvent::CursorMoved { position, .. } =&gt; {
                <span class="k">self</span>.pointer = egui::pos2(position.x <span class="k">as</span> f32 / ratio, position.y <span class="k">as</span> f32 / ratio);
                consumed = <span class="k">self</span>.ui_drag
                    || in_popup(<span class="k">self</span>.pointer)
                    || !<span class="k">self</span>.scene_rect.contains(<span class="k">self</span>.pointer);
            }
            WindowEvent::MouseInput { state, .. } =&gt; {
                <span class="k">if</span> *state == ElementState::Pressed {
                    <span class="k">self</span>.ui_drag =
                        in_popup(<span class="k">self</span>.pointer) || !<span class="k">self</span>.scene_rect.contains(<span class="k">self</span>.pointer);
                    <span class="c">// focus now so the first key is not lost</span>
                    <span class="k">let</span> input = MODEL
                        .with_borrow(|m| m.command_rect.is_some_and(|r| r.contains(<span class="k">self</span>.pointer)));
                    <span class="k">if</span> input {
                        <span class="k">self</span>.context.memory_mut(|memory| {
                            memory.request_focus(egui::Id::new(&quot;<span class="s">command-input</span>&quot;))
                        });
                        MODEL.with_borrow_mut(|model| model.command_open = <span class="s">true</span>);
                    } <span class="k">else</span> <span class="k">if</span> !in_popup(<span class="k">self</span>.pointer) {
                        <span class="k">self</span>.context.memory_mut(|memory| {
                            memory.surrender_focus(egui::Id::new(&quot;<span class="s">command-input</span>&quot;))
                        });
                        MODEL.with_borrow_mut(|model| model.command_open = <span class="s">false</span>);
                    }</code></pre></div>
<p>Replaces the line <code>WindowEvent::MouseWheel { .. } =&gt; consumed = !self.scene_…</code> in <code>lessons/31/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            WindowEvent::MouseWheel { .. } =&gt; {
                consumed = in_popup(<span class="k">self</span>.pointer) || !<span class="k">self</span>.scene_rect.contains(<span class="k">self</span>.pointer)
            }</code></pre></div>
<p>Replaces the line <code>self.ui_drag = !self.scene_rect.contains(self.pointer);</code> in <code>lessons/31/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                        <span class="k">self</span>.ui_drag =
                            in_popup(<span class="k">self</span>.pointer) || !<span class="k">self</span>.scene_rect.contains(<span class="k">self</span>.pointer);</code></pre></div>
<p>Added after the line <code>}</code> in <code>lessons/31/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">if</span> matches!(event, WindowEvent::KeyboardInput { .. })
            &amp;&amp; MODEL.with_borrow(|model| model.command_open)
        {
            consumed = <span class="s">true</span>;
        }</code></pre></div>
<p>Replaces the 2 lines from <code>let mut tool = None;</code> in <code>lessons/31/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        MODEL.with_borrow_mut(|model| model.drawing_prompt = state.drawing_prompt());
        <span class="k">let</span> drawing = state.drawing_overlay();
        <span class="c">// TextEdit processes text before its pointer cursor placement. When a click and</span>
        <span class="k">let</span> <span class="k">mut</span> clicked = <span class="s">false</span>;
        <span class="k">let</span> split = input.events.iter().position(|event| {
            clicked |= matches!(event, egui::Event::PointerButton { .. });
            clicked
                &amp;&amp; matches!(
                    event,
                    egui::Event::Key { .. } | egui::Event::Text(_) | egui::Event::Paste(_)
                )
        });
        <span class="k">let</span> pointer_input = split.map(|at| {
            <span class="k">let</span> keyboard = input.events.split_off(at);
            <span class="k">let</span> pointer = input.clone();
            input.events = keyboard;
            pointer
        });
        <span class="k">let</span> <span class="k">mut</span> draw = |root: &amp;<span class="k">mut</span> egui::Ui| {
            <span class="k">if</span> <span class="k">let</span> Some(controls) = <span class="k">self</span>.controls.as_mut() {
                controls.clear();
            }
            MODEL.with_borrow_mut(|model| {
                commands(root, model, &amp;<span class="k">mut</span> <span class="k">self</span>.controls, &amp;<span class="k">mut</span> command);
                layers(root, model, &amp;<span class="k">mut</span> <span class="k">self</span>.controls, &amp;<span class="k">mut</span> action);
            });
            <span class="k">self</span>.scene_rect = root.available_rect_before_wrap();
            <span class="k">let</span> painter = root.painter().with_clip_rect(<span class="k">self</span>.scene_rect);
            <span class="k">let</span> scale = state.pixel_scale() <span class="k">as</span> f32;
            <span class="k">let</span> points: Vec&lt;egui::Pos2&gt; = drawing
                .<span class="s">0</span>
                .iter()
                .map(|p| egui::pos2(p.<span class="s">0</span> <span class="k">as</span> f32 / scale, p.<span class="s">1</span> <span class="k">as</span> f32 / scale))
                .collect();
            <span class="k">for</span> pair <span class="k">in</span> points.windows(<span class="s">2</span>) {
                painter.line_segment(
                    [pair[<span class="s">0</span>], pair[<span class="s">1</span>]],
                    egui::Stroke::new(<span class="s">1</span>.<span class="s">5_f32</span>, egui::Color32::from_rgb(<span class="s">30</span>, <span class="s">110</span>, <span class="s">170</span>)),
                );
            }
            <span class="k">if</span> <span class="k">let</span> Some(p) = points.last() {
                painter.rect_stroke(
                    egui::Rect::from_center_size(*p, egui::vec2(<span class="s">8</span>.<span class="s">0</span>, <span class="s">8</span>.<span class="s">0</span>)),
                    <span class="s">0</span>.<span class="s">0</span>,
                    egui::Stroke::new(<span class="s">1</span>.<span class="s">5_f32</span>, egui::Color32::from_rgb(<span class="s">30</span>, <span class="s">110</span>, <span class="s">170</span>)),
                    egui::StrokeKind::Middle,
                );
                painter.text(
                    *p + egui::vec2(<span class="s">10</span>.<span class="s">0</span>, -<span class="s">12</span>.<span class="s">0</span>),
                    egui::Align2::LEFT_BOTTOM,
                    &amp;drawing.<span class="s">1</span>,
                    egui::FontId::proportional(<span class="s">14</span>.<span class="s">0</span>),
                    egui::Color32::from_rgb(<span class="s">20</span>, <span class="s">80</span>, <span class="s">130</span>),
                );
            }
        };
        <span class="k">let</span> <span class="k">mut</span> output = <span class="k">if</span> <span class="k">let</span> Some(pointer) = pointer_input {
            <span class="k">let</span> <span class="k">mut</span> output = <span class="k">self</span>.context.run_ui(pointer, &amp;<span class="k">mut</span> draw);
            output.append(<span class="k">self</span>.context.run_ui(input, draw));
            output
        } <span class="k">else</span> {
            <span class="k">self</span>.context.run_ui(input, draw)
        };
        <span class="k">self</span>.input
            .handle_platform_output(&amp;state.window, std::mem::take(&amp;<span class="k">mut</span> output.platform_output));
        <span class="k">let</span> changed = action.is_some() || command.is_some();</code></pre></div>
<p>Replaces the line <code>if model.history.len() == 8 {</code> in <code>lessons/31/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                <span class="k">if</span> model.history.len() == <span class="s">200</span> {
                    model.history.pop_front();
                }

                model.history.push_back(format!(&quot;<span class="s">&gt; </span>{<span class="s">text</span>}<span class="s">\\n</span>{<span class="s">message</span>}&quot;));
                <span class="k">if</span> !model.command_open &amp;&amp; !model.focus_command {
                    <span class="k">self</span>.context.memory_mut(|memory| {
                        memory.surrender_focus(egui::Id::new(&quot;<span class="s">command-input</span>&quot;))
                    });
                }</code></pre></div>
<p>Replaces the line <code>let snapshot = MODEL.with_borrow(|model| serde_json::json…</code> in <code>lessons/31/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">let</span> snapshot = MODEL.with_borrow(|model| serde_json::json!({&quot;<span class="s">framework</span>&quot;: &quot;<span class="s">egui 0.34.3</span>&quot;, &quot;<span class="s">scene_rect</span>&quot;: [<span class="k">self</span>.scene_rect.min.x,<span class="k">self</span>.scene_rect.min.y,<span class="k">self</span>.scene_rect.max.x,<span class="k">self</span>.scene_rect.max.y], &quot;<span class="s">completion_rect</span>&quot;: model.completion_rect.map(|r| [r.min.x,r.min.y,r.max.x,r.max.y]), &quot;<span class="s">rows</span>&quot;: model.rows, &quot;<span class="s">controls</span>&quot;: <span class="k">self</span>.controls, &quot;<span class="s">command_open</span>&quot;: model.command_open, &quot;<span class="s">layers_open</span>&quot;: model.layers_open, &quot;<span class="s">command</span>&quot;: model.command, &quot;<span class="s">history</span>&quot;: model.history, &quot;<span class="s">hint</span>&quot;: <span class="k">crate</span>::app::command::hint(&amp;model.command)}));</code></pre></div>
<p>Replaces the line <code>visuals.panel_fill = egui::Color32::from_gray(247);</code> in <code>lessons/31/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    visuals.panel_fill = egui::Color32::from_gray(<span class="s">245</span>);
    visuals.window_stroke = egui::Stroke::NONE;
    visuals.extreme_bg_color = egui::Color32::WHITE;
    visuals.selection.bg_fill = egui::Color32::from_rgb(<span class="s">200</span>, <span class="s">222</span>, <span class="s">245</span>);
    visuals.selection.stroke = egui::Stroke::new(<span class="s">1</span>.<span class="s">0_f32</span>, egui::Color32::BLACK);
    visuals.text_cursor.stroke = egui::Stroke::new(<span class="s">1</span>.<span class="s">5_f32</span>, egui::Color32::BLACK);
    visuals.text_cursor.blink = <span class="s">false</span>; <span class="c">// frames are drawn on demand</span></code></pre></div>
<p>Replaces the line <code>widget.bg_fill = egui::Color32::WHITE;</code> in <code>lessons/31/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        widget.bg_stroke = egui::Stroke::NONE;
        widget.bg_fill = egui::Color32::from_gray(<span class="s">245</span>);</code></pre></div>
<p>Replaces the 2 lines from <code>let width = (root.available_width() * 0.25).clamp(180.0, …</code> in <code>lessons/31/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">let</span> collapsed = model.layers_collapsed;
    <span class="k">let</span> width = <span class="k">if</span> collapsed {
        <span class="s">32</span>.<span class="s">0</span>
    } <span class="k">else</span> {
        (root.available_width() * <span class="s">0</span>.<span class="s">25</span>).clamp(<span class="s">180</span>.<span class="s">0</span>, <span class="s">310</span>.<span class="s">0</span>)
    };
    egui::Panel::right(<span class="k">if</span> collapsed {
        &quot;<span class="s">session-layers-collapsed</span>&quot;
    } <span class="k">else</span> {
        &quot;<span class="s">session-layers</span>&quot;
    })
    .default_size(width)
    .size_range(<span class="k">if</span> collapsed {
        <span class="s">32</span>.<span class="s">0</span>..=<span class="s">32</span>.<span class="s">0</span>
    } <span class="k">else</span> {
        <span class="s">180</span>.<span class="s">0</span>..=<span class="s">360</span>.<span class="s">0</span>
    })
    .show_separator_line(<span class="s">false</span>)
    .frame(
        <span class="c">// the panel frame</span>
        egui::Frame::new()
            .fill(egui::Color32::from_gray(<span class="s">245</span>))
            .inner_margin(<span class="s">4</span>),
    )
    .resizable(!collapsed)
    .show_inside(root, |ui| {
        ui.set_min_width(ui.available_width());
        <span class="k">let</span> collapse = ui
            .button(<span class="k">if</span> collapsed { &quot;<span class="s">+</span>&quot; } <span class="k">else</span> { &quot;<span class="s">−</span>&quot; })
            .on_hover_text(&quot;<span class="s">Collapse or expand panel</span>&quot;);
        record(
            controls,
            &quot;<span class="s">layers/collapse</span>&quot;,
            &quot;<span class="s">Collapse or expand panel</span>&quot;,
            &amp;collapse,
        );
        <span class="k">if</span> collapse.clicked() {
            model.layers_collapsed = !model.layers_collapsed;
            ui.ctx().request_repaint();
        }
        <span class="k">if</span> collapsed {
            <span class="k">return</span>;
        }
        <span class="c">// one line per row</span>
        egui::ScrollArea::vertical()
            .auto_shrink([<span class="s">false</span>, <span class="s">false</span>])
            .show(ui, |ui| {
                <span class="k">for</span> row <span class="k">in</span> &amp;model.rows {
                    ui.horizontal(|ui| {
                        <span class="k">if</span> <span class="k">let</span> Some((_, index)) = row.key.split_once('<span class="s">/</span>')
                            &amp;&amp; row.key.starts_with(&quot;<span class="s">select/</span>&quot;)
                        {
                            ui.spacing_mut().item_spacing.x = <span class="s">2</span>.;
                            ui.add_space(row.depth.min(<span class="s">8</span>) <span class="k">as</span> f32 * <span class="s">10</span>.);
                            <span class="k">let</span> response = layer_icon(ui, &quot;<span class="s">open</span>&quot;, row);
                            record(controls, &amp;format!(&quot;<span class="s">open/</span>{<span class="s">index</span>}&quot;), &amp;row.label, &amp;response);

                            <span class="k">if</span> response.clicked() &amp;&amp; row.expanded.is_some() {
                                *action = Some(format!(&quot;<span class="s">open/</span>{<span class="s">index</span>}&quot;));
                            }

                            <span class="k">let</span> width = (ui.available_width() - <span class="s">86</span>.).max(<span class="s">24</span>.);
                            <span class="k">let</span> response = ui
                                .add_sized(
                                    [width, <span class="s">28</span>.],
                                    egui::Button::new(&amp;row.label)
                                        .selected(row.selected)
                                        .frame(row.selected)
                                        .truncate(),
                                )
                                .on_hover_text(format!(&quot;{}<span class="s"> · </span>{}<span class="s"> objects</span>&quot;, row.label, row.count));
                            record(
                                controls,
                                &amp;row.key,
                                &amp;format!(&quot;<span class="s">Select </span>{}&quot;, row.label),
                                &amp;response,
                            );

                            <span class="k">if</span> response.clicked() {
                                *action = Some(<span class="k">if</span> ui.input(|i| i.modifiers.shift) {
                                    row.key.replacen(&quot;<span class="s">select/</span>&quot;, &quot;<span class="s">add/</span>&quot;, <span class="s">1</span>)
                                } <span class="k">else</span> {
                                    row.key.clone()
                                });
                            }

                            <span class="k">for</span> kind <span class="k">in</span> [&quot;<span class="s">hide</span>&quot;, &quot;<span class="s">lock</span>&quot;] {
                                <span class="k">let</span> response = layer_icon(ui, kind, row);
                                record(
                                    controls,
                                    &amp;format!(&quot;{<span class="s">kind</span>}<span class="s">/</span>{<span class="s">index</span>}&quot;),
                                    &amp;format!(
                                        &quot;{}<span class="s"> </span>{}&quot;,
                                        <span class="k">if</span> kind == &quot;<span class="s">hide</span>&quot; {
                                            <span class="k">if</span> row.hidden { &quot;<span class="s">Show</span>&quot; } <span class="k">else</span> { &quot;<span class="s">Hide</span>&quot; }
                                        } <span class="k">else</span> <span class="k">if</span> row.locked {
                                            &quot;<span class="s">Unlock</span>&quot;
                                        } <span class="k">else</span> {
                                            &quot;<span class="s">Lock</span>&quot;
                                        },
                                        row.label
                                    ),
                                    &amp;response,
                                );

                                <span class="k">if</span> response.clicked() {
                                    *action = Some(format!(&quot;{<span class="s">kind</span>}<span class="s">/</span>{<span class="s">index</span>}&quot;));
                                }
                            }

                            layer_color(ui, row, index, controls, action);
                        } <span class="k">else</span> {
                            <span class="k">let</span> response = ui.button(&amp;row.label);
                            record(controls, &amp;row.key, &amp;row.label, &amp;response);

                            <span class="k">if</span> response.clicked() {
                                *action = Some(row.key.clone());
                            }
                        }
                    });
                }
            });
    });</code></pre></div>
<p>Added after the line <code>let mut color = row.color.unwrap_or([180, 180, 180]);</code> in <code>lessons/31/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">let</span> response = egui::containers::menu::MenuButton::new(
        egui::RichText::new(&quot;<span class="s">■</span>&quot;).color(egui::Color32::from_rgb(color[<span class="s">0</span>], color[<span class="s">1</span>], color[<span class="s">2</span>])),
    )
    .config(
        egui::containers::menu::MenuConfig::default()
            .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside),
    )
    .ui(ui, |ui| {
        <span class="k">let</span> channel_id = egui::Id::new((&quot;<span class="s">layer-color-channel</span>&quot;, index));
        <span class="k">let</span> <span class="k">mut</span> edge = ui
            .ctx()
            .data_mut(|data| data.get_temp::&lt;bool&gt;(channel_id).unwrap_or(<span class="s">false</span>))
            &amp;&amp; row.has_faces;

        <span class="k">if</span> row.has_faces {
            ui.horizontal(|ui| {
                <span class="k">for</span> (label, value) <span class="k">in</span> [(&quot;<span class="s">Faces</span>&quot;, <span class="s">false</span>), (&quot;<span class="s">Edges</span>&quot;, <span class="s">true</span>)] {
                    <span class="k">let</span> response = ui.selectable_label(edge == value, label);
                    record(
                        controls,
                        &amp;format!(&quot;<span class="s">color-channel/</span>{<span class="s">index</span>}<span class="s">/</span>{<span class="s">label</span>}&quot;),
                        label,
                        &amp;response,
                    );

                    <span class="k">if</span> response.clicked() {
                        edge = value;
                    }
                }
            });
        } <span class="k">else</span> {
            ui.label(&quot;<span class="s">Object and child colors</span>&quot;);
        }

        ui.ctx().data_mut(|data| data.insert_temp(channel_id, edge));
        <span class="k">let</span> channel = <span class="k">if</span> edge { &quot;<span class="s">edge</span>&quot; } <span class="k">else</span> { &quot;<span class="s">face</span>&quot; };
        color = <span class="k">if</span> edge { row.edge_color } <span class="k">else</span> { row.color }.unwrap_or([<span class="s">180</span>; <span class="s">3</span>]);
        <span class="k">let</span> response = ui
            .button(&quot;<span class="s">Original</span>&quot;)
            .on_hover_text(&quot;<span class="s">Restore the source colors for this channel and its children</span>&quot;);
        <span class="k">let</span> key = format!(&quot;<span class="s">color/</span>{<span class="s">index</span>}<span class="s">/</span>{<span class="s">channel</span>}<span class="s">/original</span>&quot;);
        record(controls, &amp;key, &quot;<span class="s">Original</span>&quot;, &amp;response);

        <span class="k">if</span> response.clicked() {
            *action = Some(key);
            ui.close();
        }

        ui.separator();
        <span class="k">for</span> colors <span class="k">in</span> [
            [
                (&quot;<span class="s">Red</span>&quot;, [<span class="s">230</span>, <span class="s">65</span>, <span class="s">55</span>]),
                (&quot;<span class="s">Orange</span>&quot;, [<span class="s">240</span>, <span class="s">145</span>, <span class="s">45</span>]),
                (&quot;<span class="s">Yellow</span>&quot;, [<span class="s">240</span>, <span class="s">210</span>, <span class="s">60</span>]),
            ],
            [
                (&quot;<span class="s">Green</span>&quot;, [<span class="s">60</span>, <span class="s">170</span>, <span class="s">100</span>]),
                (&quot;<span class="s">Blue</span>&quot;, [<span class="s">65</span>, <span class="s">130</span>, <span class="s">225</span>]),
                (&quot;<span class="s">Violet</span>&quot;, [<span class="s">160</span>, <span class="s">85</span>, <span class="s">210</span>]),
            ],
            [
                (&quot;<span class="s">White</span>&quot;, [<span class="s">245</span>, <span class="s">245</span>, <span class="s">245</span>]),
                (&quot;<span class="s">Gray</span>&quot;, [<span class="s">150</span>, <span class="s">150</span>, <span class="s">150</span>]),
                (&quot;<span class="s">Black</span>&quot;, [<span class="s">35</span>, <span class="s">35</span>, <span class="s">35</span>]),
            ],
        ] {
            ui.horizontal(|ui| {
                <span class="k">for</span> (name, rgb) <span class="k">in</span> colors {
                    <span class="k">let</span> response = ui
                        .add_sized(
                            [<span class="s">48</span>., <span class="s">28</span>.],
                            egui::Button::new(
                                egui::RichText::new(&quot;<span class="s">■</span>&quot;)
                                    .color(egui::Color32::from_rgb(rgb[<span class="s">0</span>], rgb[<span class="s">1</span>], rgb[<span class="s">2</span>])),
                            ),
                        )
                        .on_hover_text(name);
                    <span class="k">let</span> key = format!(
                        &quot;<span class="s">color/</span>{<span class="s">index</span>}<span class="s">/</span>{<span class="s">channel</span>}<span class="s">/</span>{<span class="s">:02x</span>}{<span class="s">:02x</span>}{<span class="s">:02x</span>}&quot;,
                        rgb[<span class="s">0</span>], rgb[<span class="s">1</span>], rgb[<span class="s">2</span>]
                    );
                    record(controls, &amp;key, name, &amp;response);

                    <span class="k">if</span> response.clicked() {
                        *action = Some(key);
                        ui.close();
                    }
                }
            });
        }
        ui.separator();
        <span class="k">let</span> <span class="k">mut</span> changed = <span class="s">false</span>;

        <span class="k">for</span> (channel, value) <span class="k">in</span> [&quot;<span class="s">R</span>&quot;, &quot;<span class="s">G</span>&quot;, &quot;<span class="s">B</span>&quot;].into_iter().zip(color.iter_mut()) {
            changed |= ui
                .add(egui::Slider::new(value, <span class="s">0</span>..=<span class="s">255</span>).text(channel))
                .changed();
        }

        <span class="k">if</span> changed {
            *action = Some(format!(
                &quot;<span class="s">color/</span>{<span class="s">index</span>}<span class="s">/</span>{<span class="s">channel</span>}<span class="s">/</span>{<span class="s">:02x</span>}{<span class="s">:02x</span>}{<span class="s">:02x</span>}&quot;,
                color[<span class="s">0</span>], color[<span class="s">1</span>], color[<span class="s">2</span>]
            ));
        }
    })
    .<span class="s">0</span>
    .on_hover_text(&quot;<span class="s">Change object and child colors</span>&quot;);</code></pre></div>
<p>Replaces the line <code>let height = if model.command_open { 160.0 } else { 104.0 };</code> in <code>lessons/31/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">let</span> previous_popup = model.completion_rect.take();
    egui::Panel::bottom(&quot;<span class="s">command-line</span>&quot;)
        .exact_size(<span class="k">if</span> model.command_collapsed { <span class="s">34</span>.<span class="s">0</span> } <span class="k">else</span> { <span class="s">104</span>.<span class="s">0</span> })
        .show_separator_line(<span class="s">false</span>)
        .frame(
            egui::Frame::new()
                .fill(egui::Color32::WHITE)
                .inner_margin(<span class="s">6</span>),
        )
        .show_inside(root, |ui| {
            ui.painter().hline(
                ui.max_rect().x_range().expand(<span class="s">6</span>.<span class="s">0</span>),
                ui.max_rect().top() - <span class="s">6</span>.<span class="s">0</span>,
                egui::Stroke::new(<span class="s">1</span>.<span class="s">0_f32</span>, egui::Color32::from_gray(<span class="s">110</span>)),
            );
            ui.set_clip_rect(ui.max_rect().expand(<span class="s">6</span>.<span class="s">0</span>));
            ui.style_mut().override_font_id = Some(egui::FontId::proportional(<span class="s">14</span>.<span class="s">0</span>));
            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
            <span class="k">if</span> !model.command_collapsed {
                egui::ScrollArea::vertical()
                    .id_salt(&quot;<span class="s">command-history</span>&quot;)
                    .auto_shrink([<span class="s">false</span>, <span class="s">false</span>])
                    .stick_to_bottom(<span class="s">true</span>)
                    .min_scrolled_height(<span class="s">54</span>.<span class="s">0</span>)
                    .max_height(<span class="s">54</span>.<span class="s">0</span>)
                    .show(ui, |ui| {
                        ui.set_max_width(ui.available_width());
                        <span class="k">for</span> text <span class="k">in</span> &amp;model.history {
                            ui.add(egui::Label::new(text).wrap());
                        }
                        <span class="k">if</span> !model.status.is_empty()
                            &amp;&amp; !model
                                .history
                                .back()
                                .is_some_and(|text| text.ends_with(&amp;model.status))
                        {
                            ui.label(&amp;model.status);
                        }
                        <span class="k">if</span> !model.drawing_prompt.is_empty() {
                            <span class="k">let</span> response = ui.add(egui::Label::new(&amp;model.drawing_prompt).wrap());
                            record(controls, &quot;<span class="s">command/hint</span>&quot;, &amp;model.drawing_prompt, &amp;response);
                        }
                    });
                <span class="c">// divider between history and the field</span>
                <span class="k">let</span> (rect, _) = ui.allocate_exact_size(
                    egui::vec2(ui.available_width(), <span class="s">1</span>.<span class="s">0</span>),
                    egui::Sense::hover(),
                );
                ui.painter().hline(
                    ui.max_rect().x_range().expand(<span class="s">6</span>.<span class="s">0</span>),
                    rect.center().y,
                    egui::Stroke::new(<span class="s">1</span>.<span class="s">0_f32</span>, egui::Color32::from_gray(<span class="s">210</span>)),
                );
            }
            ui.horizontal(|ui| {
                <span class="c">// keep clear of the docs corner</span>
                ui.set_max_width((ui.available_width() - <span class="s">26</span>.<span class="s">0</span>).max(<span class="s">80</span>.<span class="s">0</span>));
                ui.label(&quot;<span class="s">Command:</span>&quot;);
                <span class="k">let</span> id = egui::Id::new(&quot;<span class="s">command-input</span>&quot;);
                <span class="k">if</span> model.focus_command || model.command_open {
                    ui.memory_mut(|memory| memory.request_focus(id));
                    <span class="k">if</span> model.focus_command {
                        command_cursor_end(ui.ctx(), id, &amp;model.command);
                        model.focus_command = <span class="s">false</span>;
                    }
                    model.command_open = <span class="s">true</span>;
                }
                <span class="c">// mouse wheel over the field browses the completions</span>
                <span class="k">let</span> wheel = ui.input_mut(|i| {
                    <span class="k">let</span> over = i.pointer.hover_pos().is_some_and(|p| {
                        model.command_rect.is_some_and(|r| r.contains(p))
                            || previous_popup.is_some_and(|r| r.contains(p))
                    });
                    <span class="k">if</span> !over {
                        <span class="k">return</span> <span class="s">0</span>;
                    }
                    <span class="k">let</span> delta: f32 = i
                        .events
                        .iter()
                        .filter_map(|e| <span class="k">match</span> e {
                            egui::Event::MouseWheel { delta, .. } =&gt; Some(delta.y),
                            _ =&gt; None,
                        })
                        .sum();
                    i.smooth_scroll_delta = egui::Vec2::ZERO;
                    <span class="k">if</span> delta &gt; <span class="s">0</span>.<span class="s">0</span> {
                        -<span class="s">1</span>
                    } <span class="k">else</span> <span class="k">if</span> delta &lt; <span class="s">0</span>.<span class="s">0</span> {
                        <span class="s">1</span>
                    } <span class="k">else</span> {
                        <span class="s">0</span>
                    }
                });
                <span class="k">if</span> wheel != <span class="s">0</span> {
                    ui.memory_mut(|memory| memory.request_focus(id));
                    model.command_open = <span class="s">true</span>;
                }
                <span class="k">let</span> has_focus = ui.memory(|memory| memory.has_focus(id));
                <span class="c">// wheel or arrow keys: -1 up, +1 down</span>
                <span class="k">let</span> browse = <span class="k">if</span> wheel != <span class="s">0</span> {
                    wheel
                } <span class="k">else</span> <span class="k">if</span> has_focus {
                    ui.input_mut(|i| {
                        <span class="k">if</span> i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown) {
                            <span class="s">1</span>
                        } <span class="k">else</span> <span class="k">if</span> i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowUp) {
                            -<span class="s">1</span>
                        } <span class="k">else</span> {
                            <span class="s">0</span>
                        }
                    })
                } <span class="k">else</span> {
                    <span class="s">0</span>
                };
                <span class="k">let</span> enter = has_focus
                    &amp;&amp; ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Enter)); <span class="c">// run the line</span>
                <span class="k">let</span> tab = has_focus
                    &amp;&amp; ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Tab)); <span class="c">// accept the completion</span>
                <span class="k">let</span> deletes = ui.input(|i| {
                    i.key_pressed(egui::Key::Backspace) || i.key_pressed(egui::Key::Delete)
                });
                <span class="c">// space accepts the completion</span>
                <span class="k">if</span> model.inline_suffix
                    &amp;&amp; has_focus
                    &amp;&amp; ui.input(|i| {
                        i.events
                            .iter()
                            .find_map(|event| <span class="k">match</span> event {
                                egui::Event::Text(text) =&gt; Some(text),
                                _ =&gt; None,
                            })
                            .is_some_and(|text| text.starts_with('<span class="s"> </span>'))
                    })
                {
                    command_cursor_end(ui.ctx(), id, &amp;model.command);
                    model.inline_suffix = <span class="s">false</span>;
                }
                <span class="k">let</span> inline_options = <span class="k">crate</span>::app::command::options(&amp;model.command);
                <span class="k">let</span> option_width: f32 = inline_options
                    .iter()
                    .map(|name| {
                        <span class="k">let</span> label = name.split_once('<span class="s"> </span>').map_or(*name, |(_, option)| option);
                        ui.painter()
                            .layout_no_wrap(
                                label.into(),
                                egui::FontId::proportional(<span class="s">14</span>.<span class="s">0</span>),
                                egui::Color32::BLACK,
                            )
                            .size()
                            .x
                            + <span class="s">16</span>.<span class="s">0</span>
                    })
                    .sum();
                <span class="k">let</span> response = ui.add_sized(
                    [(ui.available_width() - option_width - <span class="s">28</span>.<span class="s">0</span>).max(<span class="s">40</span>.<span class="s">0</span>), <span class="s">22</span>.<span class="s">0</span>],
                    egui::TextEdit::singleline(&amp;<span class="k">mut</span> model.command)
                        .id(id)
                        .font(egui::FontId::proportional(<span class="s">14</span>.<span class="s">0</span>))
                        .frame(egui::Frame::NONE)
                        .clip_text(<span class="s">true</span>)
                        .char_limit(<span class="s">2048</span>)
                        .hint_text(&quot;<span class="s">Type a command</span>&quot;),
                );
                <span class="c">// caret visible on an empty field</span>
                <span class="k">if</span> model.command_open &amp;&amp; model.command.is_empty() {
                    <span class="k">let</span> y = response.rect.center().y;
                    ui.painter().vline(
                        response.rect.left(),
                        y - <span class="s">7</span>.<span class="s">0</span>..=y + <span class="s">7</span>.<span class="s">0</span>,
                        ui.visuals().text_cursor.stroke,
                    );
                }
                model.command_rect = Some(response.rect);
                record(controls, &quot;<span class="s">command/input</span>&quot;, &quot;<span class="s">Command</span>&quot;, &amp;response);
                <span class="k">let</span> focused = model.command_open || response.has_focus() || response.lost_focus();
                <span class="k">if</span> response.gained_focus() {
                    model.command_open = <span class="s">true</span>;
                }
                <span class="k">if</span> response.changed() {
                    model.completion = <span class="s">0</span>;
                    model.completion_visible = !model.command.is_empty();
                    model.completion_prefix.clone_from(&amp;model.command);
                    model.inline_suffix = <span class="s">false</span>;
                    <span class="c">// complete only when typing at the end</span>
                    <span class="k">let</span> at_end = egui::TextEdit::load_state(ui.ctx(), id)
                        .and_then(|state| state.cursor.char_range())
                        .is_some_and(|range| {
                            range.is_empty() &amp;&amp; range.primary.index == model.command.chars().count()
                        });
                    <span class="k">if</span> !deletes
                        &amp;&amp; at_end
                        &amp;&amp; !model.command.is_empty()
                        &amp;&amp; !model.command.ends_with('<span class="s"> </span>')
                        &amp;&amp; <span class="k">let</span> Some(name) = <span class="k">crate</span>::app::command::completions(&amp;model.command).first()
                    {
                        <span class="k">let</span> prefix = model.command.chars().count();
                        <span class="k">if</span> name.chars().count() &gt; prefix {
                            model.command = (*name).into();
                            command_cursor_select(
                                ui.ctx(),
                                id,
                                prefix,
                                model.command.chars().count(),
                            );
                            model.inline_suffix = <span class="s">true</span>;
                            ui.ctx().request_repaint();
                        }
                    }
                } <span class="k">else</span> <span class="k">if</span> !model.inline_suffix {
                    model.completion_prefix.clone_from(&amp;model.command);
                }
                <span class="c">// the completion list</span>
                <span class="k">let</span> choices = <span class="k">crate</span>::app::command::browse(&amp;model.completion_prefix);
                <span class="k">let</span> <span class="k">mut</span> complete = None; <span class="c">// completion chosen this frame</span>
                <span class="k">let</span> opening_list = !model.completion_visible;
                <span class="k">if</span> browse != <span class="s">0</span> || tab {
                    model.completion_visible = <span class="s">true</span>;
                }
                <span class="k">if</span> focused &amp;&amp; model.completion_visible &amp;&amp; !choices.is_empty() {
                    model.completion = model.completion.min(choices.len() - <span class="s">1</span>);
                    <span class="k">if</span> browse != <span class="s">0</span> {
                        model.completion = <span class="k">if</span> opening_list &amp;&amp; !model.completion_prefix.contains('<span class="s"> </span>')
                        {
                            <span class="k">if</span> browse &lt; <span class="s">0</span> { choices.len() - <span class="s">1</span> } <span class="k">else</span> { <span class="s">0</span> }
                        } <span class="k">else</span> {
                            (model.completion <span class="k">as</span> isize + browse).rem_euclid(choices.len() <span class="k">as</span> isize)
                                <span class="k">as</span> usize
                        };
                        model.command = choices[model.completion].into();
                        command_cursor_select(
                            ui.ctx(),
                            id,
                            <span class="k">if</span> model
                                .command
                                .to_ascii_lowercase()
                                .starts_with(&amp;model.completion_prefix.to_ascii_lowercase())
                            {
                                model.completion_prefix.chars().count()
                            } <span class="k">else</span> {
                                <span class="s">0</span>
                            },
                            model.command.chars().count(),
                        );
                        model.inline_suffix = <span class="s">true</span>;
                        ui.ctx().request_repaint();
                    }
                    <span class="k">if</span> tab {
                        complete = Some(choices[model.completion]);
                    }
                    <span class="k">if</span> !model.completion_prefix.contains('<span class="s"> </span>') {
                        <span class="k">let</span> popup_width =
                            (ui.ctx().content_rect().right() - response.rect.left() - <span class="s">12</span>.<span class="s">0</span>)
                                .clamp(<span class="s">60</span>.<span class="s">0</span>, <span class="s">220</span>.<span class="s">0</span>);
                        <span class="k">let</span> popup_height = (response.rect.top() - <span class="s">12</span>.<span class="s">0</span>).clamp(<span class="s">22</span>.<span class="s">0</span>, <span class="s">220</span>.<span class="s">0</span>);
                        <span class="k">let</span> popup = egui::Area::new(egui::Id::new(&quot;<span class="s">command-completions</span>&quot;))
                            .pivot(egui::Align2::LEFT_BOTTOM)
                            .fixed_pos(egui::pos2(response.rect.left(), response.rect.top()))
                            .order(egui::Order::Foreground)
                            .show(ui.ctx(), |ui| {
                                egui::Frame::new()
                                    .fill(egui::Color32::WHITE)
                                    .stroke(egui::Stroke::new(
                                        <span class="s">1</span>.<span class="s">0_f32</span>,
                                        egui::Color32::from_gray(<span class="s">215</span>),
                                    ))
                                    .inner_margin(<span class="s">5</span>)
                                    .show(ui, |ui| {
                                        ui.set_width(popup_width);
                                        ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
                                        egui::ScrollArea::vertical()
                                            .id_salt(&quot;<span class="s">command-choices</span>&quot;)
                                            .max_height(popup_height)
                                            .show(ui, |ui| {
                                                <span class="k">for</span> (index, name) <span class="k">in</span> choices.iter().enumerate() {
                                                    <span class="k">let</span> item = ui.selectable_label(
                                                        index == model.completion,
                                                        *name,
                                                    );
                                                    record(
                                                        controls,
                                                        &amp;format!(&quot;<span class="s">command/completion/</span>{<span class="s">name</span>}&quot;),
                                                        name,
                                                        &amp;item,
                                                    );
                                                    <span class="k">if</span> index == model.completion &amp;&amp; browse != <span class="s">0</span> {
                                                        item.scroll_to_me(None);
                                                    }
                                                    <span class="k">if</span> item.clicked() {
                                                        complete = Some(*name);
                                                    }
                                                }
                                            });
                                    });
                            });
                        model.completion_rect = Some(popup.response.rect);
                    }
                }
                <span class="c">// a chosen completion fills the field, maybe runs it</span>
                <span class="k">if</span> <span class="k">let</span> Some(name) = complete {
                    model.command = format!(&quot;{<span class="s">name</span>}<span class="s"> </span>&quot;);
                    model.inline_suffix = <span class="s">false</span>;
                    model.focus_command = <span class="s">true</span>;
                }
                <span class="c">// options before the field</span>
                <span class="k">for</span> name <span class="k">in</span> inline_options {
                    <span class="k">let</span> label = name.split_once('<span class="s"> </span>').map_or(*name, |(_, option)| option);
                    <span class="k">let</span> selected = model.command.trim().eq_ignore_ascii_case(name)
                        || (model.command.ends_with('<span class="s"> </span>')
                            &amp;&amp; model.command.split_whitespace().count() == <span class="s">1</span>
                            &amp;&amp; Some(name) == inline_options.first());
                    <span class="k">let</span> option = ui.selectable_label(selected, label);
                    record(controls, &amp;format!(&quot;<span class="s">command/option/</span>{<span class="s">label</span>}&quot;), label, &amp;option);
                    <span class="k">if</span> option.clicked() {
                        <span class="k">let</span> (text, run) = <span class="k">crate</span>::app::command::accept(name);
                        <span class="k">if</span> run {
                            *command = Some(text);
                            model.command.clear();
                        } <span class="k">else</span> {
                            model.command = text;
                        }
                        model.completion_visible = <span class="s">false</span>;
                        model.inline_suffix = <span class="s">false</span>;
                        model.focus_command = <span class="s">true</span>;
                    }
                }
                <span class="c">// Enter runs the line</span>
                <span class="k">if</span> enter &amp;&amp; (!model.command.trim().is_empty() || !model.drawing_prompt.is_empty()) {
                    <span class="k">let</span> (text, run) =
                        <span class="k">crate</span>::app::command::accept(&amp;std::mem::take(&amp;<span class="k">mut</span> model.command));
                    <span class="k">if</span> run {
                        *command = Some(text);
                    } <span class="k">else</span> {
                        model.command = text;
                    }
                    model.completion_visible = <span class="s">false</span>;
                    model.inline_suffix = <span class="s">false</span>;
                    model.focus_command = <span class="s">true</span>;
                }
                <span class="c">// Escape clears the field</span>
                <span class="k">if</span> ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                    model.command.clear();
                    model.completion_visible = <span class="s">false</span>;
                    model.inline_suffix = <span class="s">false</span>;
                    model.command_open = <span class="s">false</span>;
                    model.focus_command = <span class="s">false</span>;
                    response.surrender_focus();
                    *command = Some(&quot;<span class="s">Escape</span>&quot;.into());
                    <span class="k">crate</span>::app::feedback::focus_canvas();
                }
                <span class="c">// the +/− button folds the history</span>
                <span class="k">let</span> collapse = ui
                    .button(<span class="k">if</span> model.command_collapsed { &quot;<span class="s">+</span>&quot; } <span class="k">else</span> { &quot;<span class="s">−</span>&quot; })
                    .on_hover_text(&quot;<span class="s">Collapse or expand history</span>&quot;);
                record(
                    controls,
                    &quot;<span class="s">command/collapse</span>&quot;,
                    &quot;<span class="s">Collapse or expand history</span>&quot;,
                    &amp;collapse,
                );
                <span class="k">if</span> collapse.clicked() {
                    model.command_collapsed = !model.command_collapsed;
                    ui.ctx().request_repaint();
                }
            });
        });
}

<span class="c">/// Put the caret at the end of the field.</span>
<span class="k">fn</span> command_cursor_end(context: &amp;egui::Context, id: egui::Id, command: &amp;str) {
    <span class="k">let</span> end = command.chars().count();
    command_cursor_select(context, id, end, end);
}

<span class="c">/// Select \`start..end\` in the field.</span>
<span class="k">fn</span> command_cursor_select(context: &amp;egui::Context, id: egui::Id, start: usize, end: usize) {
    <span class="k">if</span> <span class="k">let</span> Some(<span class="k">mut</span> state) = egui::TextEdit::load_state(context, id) {
        state
            .cursor
            .set_char_range(Some(egui::text::CCursorRange::two(
                egui::text::CCursor::new(start),
                egui::text::CCursor::new(end),
            )));
        egui::TextEdit::store_state(context, id, state);
    }</code></pre></div>
<h2 id="step-11-srcenginegpuglyphsrs">Step 11 · src/engine/gpu/glyphs.rs<a class="anchor" href="#/course/32-colors-lighting#step-11-srcenginegpuglyphsrs" aria-label="Link to this section">#</a></h2>
<p>Update an individual control marker during a mesh preview.</p>
<p><code>lessons/32/src/engine/gpu/glyphs.rs</code> · edit · type this</p>
<p>Added after the line <code>impl GlyphLane {</code> in <code>lessons/31/src/engine/gpu/glyphs.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Overwrite one marker or dot row.</span>
    <span class="k">pub</span>(<span class="k">crate</span>) <span class="k">fn</span> patch_marker(
        &amp;<span class="k">mut</span> <span class="k">self</span>,
        ctx: &amp;GpuCtx,
        index: u32,
        sphere: bool,
        glyph: GlyphPoint,
    ) {
        <span class="k">let</span> table = <span class="k">if</span> sphere {
            &amp;<span class="k">mut</span> <span class="k">self</span>.spheres
        } <span class="k">else</span> {
            &amp;<span class="k">mut</span> <span class="k">self</span>.dots
        };
        table.buf.write_at(ctx, index, &amp;[glyph]);
    }

    <span class="c">/// Overwrite one object's rows in place.</span></code></pre></div>
<h2 id="step-12-srcenginegpuinstancers">Step 12 · src/engine/gpu/instance.rs<a class="anchor" href="#/course/32-colors-lighting#step-12-srcenginegpuinstancers" aria-label="Link to this section">#</a></h2>
<p>Store the separate edge color and its override flag in the GPU instance data.</p>
<p><code>lessons/32/src/engine/gpu/instance.rs</code> · edit · type this</p>
<p>Added after the line <code>pub const FLAG_COLOR: u32 = 1 &lt;&lt; 8;</code> in <code>lessons/31/src/engine/gpu/instance.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Use the layer color for edges too.</span>
    <span class="k">pub</span> <span class="k">const</span> FLAG_EDGE_COLOR: u32 = <span class="s">1</span> &lt;&lt; <span class="s">9</span>;

    <span class="c">/// The object has faces, not only lines or points.</span>
    <span class="k">pub</span> <span class="k">const</span> FLAG_HAS_FACES: u32 = <span class="s">1</span> &lt;&lt; <span class="s">10</span>;

    <span class="c">/// The one row an empty scene binds: identity, grey, no flags.</span></code></pre></div>
<p>Replaces the line <code>let rust = [&quot;model&quot;, &quot;color&quot;, &quot;flags&quot;, &quot;_pad0&quot;, &quot;spacing&quot;];</code> in <code>lessons/31/src/engine/gpu/instance.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> rust = [&quot;<span class="s">model</span>&quot;, &quot;<span class="s">color</span>&quot;, &quot;<span class="s">flags</span>&quot;, &quot;<span class="s">_pad0</span>&quot;, &quot;<span class="s">spacing</span>&quot;, &quot;<span class="s">edge_color</span>&quot;];</code></pre></div>
<h2 id="step-13-srcenginegpumodrs">Step 13 · src/engine/gpu/mod.rs<a class="anchor" href="#/course/32-colors-lighting#step-13-srcenginegpumodrs" aria-label="Link to this section">#</a></h2>
<p>Own optional ambient resources and report their buffer and texture sizes.</p>
<p><code>lessons/32/src/engine/gpu/mod.rs</code> · edit · type this</p>
<p>Added after the line <code>pub mod splat;</code> in <code>lessons/31/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">mod</span> ssao;</code></pre></div>
<p>Added after the line <code>pub backdrop: BackdropLane,</code> in <code>lessons/31/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    ssao: Option&lt;ssao::Ssao&gt;,</code></pre></div>
<p>Replaces the line <code>+ outline_buffers;</code> in <code>lessons/31/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            + outline_buffers
            + <span class="k">if</span> <span class="k">self</span>.ssao.is_some() { <span class="s">144</span> } <span class="k">else</span> { <span class="s">0</span> };</code></pre></div>
<p>Replaces the line <code>+ outline_textures,</code> in <code>lessons/31/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                + outline_textures
                + <span class="k">self</span>.ssao.as_ref().map_or(<span class="s">0</span>, ssao::Ssao::texture_bytes),</code></pre></div>
<p>Added after the line <code>backdrop,</code> in <code>lessons/31/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            ssao: None,</code></pre></div>
<p>Replaces the 2 lines from <code>pub fn set_object_color(&amp;mut self, row: u32, color: [u8; …</code> in <code>lessons/31/src/engine/gpu/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Set the face or edge color of object \`row\`; None restores its own.</span>
    <span class="k">pub</span> <span class="k">fn</span> set_object_color(&amp;<span class="k">mut</span> <span class="k">self</span>, row: u32, edge: bool, color: Option&lt;[u8; 3]&gt;) {
        <span class="k">self</span>.objects.set_color(&amp;<span class="k">self</span>.ctx, row, edge, color);</code></pre></div>
<h2 id="step-14-srcenginegpuobjectsrs">Step 14 · src/engine/gpu/objects.rs<a class="anchor" href="#/course/32-colors-lighting#step-14-srcenginegpuobjectsrs" aria-label="Link to this section">#</a></h2>
<p>Update either color channel and advance the geometry revision after a preview change.</p>
<p><code>lessons/32/src/engine/gpu/objects.rs</code> · edit · type this</p>
<p>Added after the line <code>pub color: [f32; 4],</code> in <code>lessons/31/src/engine/gpu/objects.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> edge_color: u32, <span class="c">// packed edge color</span></code></pre></div>
<p>Added after the line <code>color: [1.0; 4],</code> in <code>lessons/31/src/engine/gpu/objects.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            edge_color: <span class="s">0</span>,</code></pre></div>
<p>Replaces the line <code>_pad: 0,</code> in <code>lessons/31/src/engine/gpu/objects.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                _pad: r.edge_color,</code></pre></div>
<p>Replaces the line <code>pub fn set_color(&amp;mut self, ctx: &amp;GpuCtx, row: u32, color…</code> in <code>lessons/31/src/engine/gpu/objects.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Set a row's face or edge color; None restores its own.</span>
    <span class="k">pub</span> <span class="k">fn</span> set_color(&amp;<span class="k">mut</span> <span class="k">self</span>, ctx: &amp;GpuCtx, row: u32, edge: bool, color: Option&lt;[u8; 3]&gt;) {
        <span class="k">if</span> <span class="k">let</span> Some(r) = <span class="k">self</span>.rows.get_mut(row <span class="k">as</span> usize) {
            <span class="k">let</span> flag = <span class="k">if</span> edge {
                Instance::FLAG_EDGE_COLOR
            } <span class="k">else</span> {
                Instance::FLAG_COLOR
            };
            r.flags &amp;= !flag;

            <span class="k">if</span> color.is_some() {
                r.flags |= flag;
            }

            <span class="c">// edge color is packed into the padding word</span>
            <span class="k">if</span> edge {
                r._pad = color
                    .map(|c| u32::from_le_bytes([c[<span class="s">0</span>], c[<span class="s">1</span>], c[<span class="s">2</span>], <span class="s">255</span>]))
                    .unwrap_or(<span class="s">0</span>);
            } <span class="k">else</span> {
                r.color = color
                    .map(|c| {
                        [
                            c[<span class="s">0</span>] <span class="k">as</span> f32 / <span class="s">255</span>.,
                            c[<span class="s">1</span>] <span class="k">as</span> f32 / <span class="s">255</span>.,
                            c[<span class="s">2</span>] <span class="k">as</span> f32 / <span class="s">255</span>.,
                            <span class="s">1</span>.,
                        ]
                    })
                    .unwrap_or([<span class="s">1</span>.; <span class="s">4</span>]);
            }</code></pre></div>
<p>Added after the line <code>self.buffer.write_at(ctx, row, std::slice::from_ref(r));</code> in <code>lessons/31/src/engine/gpu/objects.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Note that geometry changed without a row edit.</span>
    <span class="k">pub</span>(<span class="k">crate</span>) <span class="k">fn</span> geometry_changed(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">self</span>.geometry_revision = <span class="k">self</span>.geometry_revision.wrapping_add(<span class="s">1</span>);
    }

    <span class="c">/// Forget every row; keep the buffers.</span></code></pre></div>
<h2 id="step-15-srcenginegpusplatrs">Step 15 · src/engine/gpu/splat.rs<a class="anchor" href="#/course/32-colors-lighting#step-15-srcenginegpusplatrs" aria-label="Link to this section">#</a></h2>
<p>Invalidate cached point-cloud pixels when object geometry or placement changes.</p>
<p><code>lessons/32/src/engine/gpu/splat.rs</code> · edit · type this</p>
<p>Added after the line <code>point_count: u32,</code> in <code>lessons/31/src/engine/gpu/splat.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    geometry: u64, <span class="c">// object change count</span></code></pre></div>
<p>Added after the line <code>point_count,</code> in <code>lessons/31/src/engine/gpu/splat.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            geometry: cx.objects.geometry_revision(),</code></pre></div>
<h2 id="step-16-srcshadersglyphwgsl">Step 16 · src/shaders/glyph.wgsl<a class="anchor" href="#/course/32-colors-lighting#step-16-srcshadersglyphwgsl" aria-label="Link to this section">#</a></h2>
<p>Use the correct authored or overridden marker color.</p>
<p><code>lessons/32/src/shaders/glyph.wgsl</code> · edit · type this</p>
<p>Replaces the line <code>var color = object_color(g.color, inst);</code> in <code>lessons/31/src/shaders/glyph.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">var</span> color = edge_color(g.color, inst);</code></pre></div>
<h2 id="step-17-srcshadersribbonwgsl">Step 17 · src/shaders/ribbon.wgsl<a class="anchor" href="#/course/32-colors-lighting#step-17-srcshadersribbonwgsl" aria-label="Link to this section">#</a></h2>
<p>Resolve independent edge colors and keep 3D stroke widths constant on screen.</p>
<p><code>lessons/32/src/shaders/ribbon.wgsl</code> · edit · type this</p>
<p>Delete the two <code>const WIRE_MIN_PENS</code> and <code>const TAPER_MIN</code> lines and the comment above them from <code>lessons/31/src/shaders/ribbon.wgsl</code>.</p>
<p>Added after the line <code>fn half_width_px(radius: f32, w: f32) -&gt; f32 {</code> in <code>lessons/31/src/shaders/ribbon.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// Half width in px at depth \`w\`, from the radius field.</span>
<span class="k">fn</span> half_width_px(radius: <span class="k">f32</span>, w: <span class="k">f32</span>) -&gt; <span class="k">f32</span> {
    <span class="k">if</span> (radius &lt; <span class="s">0.0</span>) { <span class="k">return</span> -radius * line.thickness; }</code></pre></div>
<p>Delete the <code>fn density_taper</code> block from <code>lessons/31/src/shaders/ribbon.wgsl</code>.</p>
<p>Replaces the lines from <code>let cad_boundary = (inst.flags &amp; FLAG_SMOOTH) != 0u &amp;&amp; so…</code> in <code>lessons/31/src/shaders/ribbon.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">    // corner: sideways by the width, outward by the filter reach</span>
    <span class="k">let</span> along = select(-<span class="s">1.0</span>, <span class="s">1.0</span>, at_end1);
    <span class="k">let</span> p = select(s0, s1, at_end1) + (n * side + dir * along) * (px + FILTER_REACH);

    <span class="k">var</span> o: VsOut;
    <span class="k">let</span> ndc = (p / vp - <span class="s">0.5</span>) * <span class="s">2.0</span>;
    o.pos = <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(ndc * clip.w, clip.z, clip.w);
    <span class="k">var</span> color = edge_color(unpack4x8unorm(seg.color), inst);</code></pre></div>
<p>Replaces the 2 lines from <code>o.hw0 = raw0 * crowd;</code> in <code>lessons/31/src/shaders/ribbon.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    o.hw0 = raw0;
    o.hw1 = raw1;</code></pre></div>
<p>Replaces the line <code>if (alpha &lt;= 0.0 || !ink_visible(in.pos.xy, ink_axis(in),…</code> in <code>lessons/31/src/shaders/ribbon.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">if</span> (alpha &lt;= <span class="s">0.0</span> || !ink_visible(in.pos.xy, ink_axis(in), sample, (instances[in.inst_id].flags &amp; FLAG_SMOOTH) != <span class="s">0u</span>)) {</code></pre></div>
<p>Replaces the line <code>if (alpha &lt;= 0.0 || !ink_visible(in.pos.xy, ink_axis(in),…</code> in <code>lessons/31/src/shaders/ribbon.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">if</span> (alpha &lt;= <span class="s">0.0</span> || !ink_visible(in.pos.xy, ink_axis(in), sample, (instances[in.inst_id].flags &amp; FLAG_SMOOTH) != <span class="s">0u</span>)) {</code></pre></div>
<p>Replaces the line <code>if (alpha &lt;= 0.0 || !ink_visible(in.pos.xy, ink_axis(in),…</code> in <code>lessons/31/src/shaders/ribbon.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">if</span> (alpha &lt;= <span class="s">0.0</span> || !ink_visible(in.pos.xy, ink_axis(in), sample, (instances[in.inst_id].flags &amp; FLAG_SMOOTH) != <span class="s">0u</span>)) {</code></pre></div>
<p>Replaces the 2 lines from <code>if (alpha &lt;= 0.0 || !ink_visible(in.pos.xy, ink_axis(in),…</code> in <code>lessons/31/src/shaders/ribbon.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">if</span> (alpha &lt;= <span class="s">0.0</span> || !ink_visible(in.pos.xy, ink_axis(in), sample, (instances[in.inst_id].flags &amp; FLAG_SMOOTH) != <span class="s">0u</span>)) {</code></pre></div>
<p>Replaces the line <code>if (coverage(in) &lt;= 0.0 || !ink_visible(in.pos.xy, ink_ax…</code> in <code>lessons/31/src/shaders/ribbon.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">if</span> (coverage(in) &lt;= <span class="s">0.0</span> || !ink_visible(in.pos.xy, ink_axis(in), <span class="s">0u</span>, (instances[in.inst_id].flags &amp; FLAG_SMOOTH) != <span class="s">0u</span>)) {</code></pre></div>
<p>Replaces the line <code>if (in.source_edge == 0xffffffffu || coverage(in) &lt;= 0.0 …</code> in <code>lessons/31/src/shaders/ribbon.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">if</span> (in.source_edge == <span class="s">0xffffffffu</span> || coverage(in) &lt;= <span class="s">0.0</span> || !ink_visible(in.pos.xy, ink_axis(in), <span class="s">0u</span>, (instances[in.inst_id].flags &amp; FLAG_SMOOTH) != <span class="s">0u</span>)) {</code></pre></div>
<h2 id="step-18-srcshadersscenewgsl">Step 18 · src/shaders/scene.wgsl<a class="anchor" href="#/course/32-colors-lighting#step-18-srcshadersscenewgsl" aria-label="Link to this section">#</a></h2>
<p>Resolve face and edge colors independently from their override flags.</p>
<p><code>lessons/32/src/shaders/scene.wgsl</code> · edit · type this</p>
<p>Added after the line <code>spacing: f32,</code> in <code>lessons/31/src/shaders/scene.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    edge_color: <span class="k">u32</span>,<span class="c"> // packed edge color</span></code></pre></div>
<p>Added after the line <code>}</code> in <code>lessons/31/src/shaders/scene.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// Edge color: the layer edge color when set, else the face rule.</span>
<span class="k">fn</span> edge_color(authored: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;, inst: Instance) -&gt; <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt; {
<span class="c">    // no faces: edges follow the face rule</span>
    <span class="k">if</span> ((inst.flags &amp; <span class="s">1024u</span>) == <span class="s">0u</span>) {
        <span class="k">return</span> object_color(authored, inst);
    }

<span class="c">    // layer edge color set</span>
    <span class="k">if</span> ((inst.flags &amp; <span class="s">512u</span>) != <span class="s">0u</span>) {
        <span class="k">return</span> <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(unpack4x8unorm(inst.edge_color).rgb, authored.a);
    }

    <span class="k">return</span> authored;
}

<span class="c">// No face normals known: always drawn.</span></code></pre></div>
<h2 id="step-19-srcshadersspherewgsl">Step 19 · src/shaders/sphere.wgsl<a class="anchor" href="#/course/32-colors-lighting#step-19-srcshadersspherewgsl" aria-label="Link to this section">#</a></h2>
<p>Keep marker colors consistent with the new channel flags.</p>
<p><code>lessons/32/src/shaders/sphere.wgsl</code> · edit · type this</p>
<p>Replaces the line <code>var color = object_color(g.color, inst);</code> in <code>lessons/31/src/shaders/sphere.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">var</span> color = edge_color(g.color, inst);</code></pre></div>
<h2 id="step-20-srcstateeditrs">Step 20 · src/state/edit.rs<a class="anchor" href="#/course/32-colors-lighting#step-20-srcstateeditrs" aria-label="Link to this section">#</a></h2>
<p>Preview all selected placements together and commit once on release.</p>
<p><code>lessons/32/src/state/edit.rs</code> · edit · type this</p>
<p>Replaces the line <code>base_local: Xform,</code> in <code>lessons/31/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    group: Vec&lt;(u32, Xform)&gt;,                             <span class="c">// every selected row and where it started</span>
    base_place: Xform,                                    <span class="c">// the main row's placement at the grab</span>
    drag: Drag,                                           <span class="c">// the handle and where it was grabbed</span>
    target: Option&lt;<span class="k">crate</span>::app::deform::Target&gt;,           <span class="c">// a face, edge or control point being moved</span>
    source: Option&lt;session_rust::Geometry&gt;,               <span class="c">// the geometry before the drag</span>
    origin: Point,                                        <span class="c">// the gizmo center at the grab</span>
    mesh_preview: Option&lt;<span class="k">crate</span>::app::mesh_preview::Gesture&gt;,</code></pre></div>
<p>Replaces the line <code>let mut origin = box_.center();</code> in <code>lessons/31/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="c">// the box around every selected row</span>
        <span class="k">let</span> <span class="k">mut</span> bounds = box_;
        <span class="k">if</span> row.is_some() {
            <span class="k">for</span> selected <span class="k">in</span> &amp;<span class="k">self</span>.hierarchy.selected {
                <span class="k">if</span> <span class="k">let</span> Some(b) = <span class="k">self</span>.gpu.objects.row_bounds(*selected) {
                    bounds.union_with(&amp;b);
                }
            }
        }
        <span class="k">let</span> <span class="k">mut</span> origin = bounds.center();</code></pre></div>
<p>Added after the line <code>};</code> in <code>lessons/31/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="c">// every selected row with where it is now</span>
        <span class="k">let</span> group = <span class="k">self</span>
            .selected_rows()
            .into_iter()
            .filter_map(|r| Some((r, <span class="k">self</span>.scene.placement_of(r)?)))
            .collect();</code></pre></div>
<p>Added after the line <code>};</code> in <code>lessons/31/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> target = <span class="k">crate</span>::app::deform::Target::selected(&amp;<span class="k">self</span>.selection);
        <span class="c">// a mesh vertex drag previews on the GPU</span>
        <span class="k">let</span> mesh_preview = target.and_then(|target| {
            <span class="k">self</span>.scene
                .mesh_previews
                .get(row <span class="k">as</span> usize)?
                .as_ref()?
                .begin(<span class="k">self</span>.scene.geometry(row)?, target)
        });

        <span class="k">self</span>.dragging = Some(GizmoDrag {
            group,
            row,
            base_place,
            drag,
            target: <span class="k">crate</span>::app::deform::Target::selected(&amp;<span class="k">self</span>.selection),
            source: <span class="k">self</span>.scene.geometry(row).cloned(),
            origin: gizmo.origin.clone(),
            mesh_preview,</code></pre></div>
<p>Added after the line <code>let local = &amp;(&amp;back * &amp;delta) * &amp;place;</code> in <code>lessons/31/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">if</span> <span class="k">let</span> Some(preview) = active.mesh_preview.as_ref() {
                preview.apply(&amp;<span class="k">mut</span> <span class="k">self</span>.gpu, &amp;local, <span class="s">false</span>);
                <span class="k">let</span> origin = active.origin.transformed(&amp;delta);
                <span class="k">self</span>.gpu
                    .bounds
                    .union_with_point(origin[<span class="s">0</span>], origin[<span class="s">1</span>], origin[<span class="s">2</span>]);

                <span class="k">if</span> <span class="k">let</span> Some(gizmo) = <span class="k">self</span>.gizmo.as_mut() {
                    gizmo.origin = origin;
                }

                <span class="k">self</span>.upload_gizmo();
                <span class="k">self</span>.touch();
                <span class="k">return</span> <span class="s">true</span>;
            }</code></pre></div>
<p>Replaces the line <code>let place = &amp;delta * &amp;active.base_place;</code> in <code>lessons/31/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="c">// whole objects: move every row's placement</span>
        <span class="k">for</span> (row, base) <span class="k">in</span> &amp;active.group {
            <span class="k">self</span>.gpu
                .objects
                .set_placement(&amp;<span class="k">self</span>.gpu.ctx, *row, &amp;(&amp;delta * base));
            <span class="k">self</span>.gpu.grew_bounds(*row);
        }</code></pre></div>
<p>Replaces the line <code>self.gpu.grew_bounds(active.row);</code> in <code>lessons/31/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">for</span> (row, base) <span class="k">in</span> &amp;active.group {
            <span class="k">self</span>.gpu.objects.set_placement(&amp;<span class="k">self</span>.gpu.ctx, *row, base);
            <span class="k">self</span>.gpu.grew_bounds(*row);
        }</code></pre></div>
<p>Replaces the 5 lines from <code>if let Some(place) = self.scene.set_row_xform(active.row,…</code> in <code>lessons/31/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">if</span> <span class="k">let</span> Err(error) = <span class="k">self</span>.apply(delta, label) {
            <span class="k">self</span>.status(&amp;error);
            <span class="k">return</span> <span class="s">false</span>;</code></pre></div>
<p>Replaces the line <code>if active.target.is_some() {</code> in <code>lessons/31/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">if</span> <span class="k">let</span> Some(preview) = active.mesh_preview.as_ref() {
                preview.apply(&amp;<span class="k">mut</span> <span class="k">self</span>.gpu, &amp;Xform::identity(), <span class="s">true</span>);
                <span class="k">self</span>.restore_edit_selection(active.row);
            } <span class="k">else</span> <span class="k">if</span> active.target.is_some() {</code></pre></div>
<p>Added after the line <code>}</code> in <code>lessons/31/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">for</span> (row, base) <span class="k">in</span> &amp;active.group {
                <span class="k">self</span>.gpu.objects.set_placement(&amp;<span class="k">self</span>.gpu.ctx, *row, base);
                <span class="k">self</span>.gpu.grew_bounds(*row);
            }
            <span class="k">self</span>.place_gizmo(Some(active.row));
            <span class="k">self</span>.touch();
        }

        <span class="k">if</span> <span class="k">let</span> Some(active) = <span class="k">self</span>.control_drag.take() {
            <span class="k">self</span>.restore_source_render(active.parent);</code></pre></div>
<p>Replaces the line <code>fn pixel_scale(&amp;self) -&gt; f64 {</code> in <code>lessons/31/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Device pixels per CSS pixel: 1 on a monitor, 2 or more on a phone.</span>
    <span class="k">pub</span>(<span class="k">crate</span>) <span class="k">fn</span> pixel_scale(&amp;<span class="k">self</span>) -&gt; f64 {</code></pre></div>
<p>Added after the line <code>self.cancel_gesture();</code> in <code>lessons/31/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="c">// while drawing, points and Enter go to the draft</span>
        <span class="k">if</span> <span class="k">let</span> Some(result) = <span class="k">self</span>.drawing_command(line) {
            <span class="k">return</span> result;
        }
        <span class="k">let</span> command = <span class="k">crate</span>::app::command::parse(line)?;
        <span class="c">// any other command ends the draft</span>
        <span class="k">if</span> !matches!(command, Command::Snap(_)) {
            <span class="k">self</span>.draft = None;
        }</code></pre></div>
<p>Added after the line <code>match command {</code> in <code>lessons/31/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            Command::Snap(value) =&gt; {
                <span class="k">self</span>.snap_enabled = value.unwrap_or(!<span class="k">self</span>.snap_enabled);
                Ok(format!(
                    &quot;<span class="s">Snap </span>{}&quot;,
                    <span class="k">if</span> <span class="k">self</span>.snap_enabled { &quot;<span class="s">On</span>&quot; } <span class="k">else</span> { &quot;<span class="s">Off</span>&quot; }
                ))
            }
            Command::Layers(value) =&gt; {
                <span class="k">if</span> <span class="k">let</span> Some(open) = value {
                    <span class="k">crate</span>::app::feedback::layers_visible(open);
                    <span class="k">self</span>.refresh_layers();
                }
                Ok(&quot;<span class="s">Layers (On Off)</span>&quot;.into())
            }
            Command::Selection(tool) =&gt; {
                <span class="k">self</span>.escape_selection();
                <span class="k">self</span>.selection_tool = tool;
                Ok(format!(&quot;{<span class="s">tool:?</span>}<span class="s"> selection</span>&quot;))
            }
            Command::Controls =&gt; {
                <span class="k">self</span>.enable_controls();
                Ok(&quot;<span class="s">Control points</span>&quot;.into())
            }
            Command::Ssao(value) =&gt; {
                <span class="k">self</span>.gpu.view.ssao = value.unwrap_or(!<span class="k">self</span>.gpu.view.ssao);
                Ok(format!(
                    &quot;<span class="s">SSAO </span>{}&quot;,
                    <span class="k">if</span> <span class="k">self</span>.gpu.view.ssao { &quot;<span class="s">On</span>&quot; } <span class="k">else</span> { &quot;<span class="s">Off</span>&quot; }
                ))
            }</code></pre></div>
<p>Replaces the 3 lines from <code>let Some(place) = self.scene.transform_row(row, &amp;delta, l…</code> in <code>lessons/31/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="c">// whole objects: the document moves them, the GPU follows</span>
        <span class="k">let</span> rows = <span class="k">self</span>.selected_rows();
        <span class="k">let</span> places = <span class="k">self</span>
            .scene
            .transform_rows(&amp;rows, &amp;delta, label)
            .ok_or(&quot;<span class="s">this selection cannot be edited</span>&quot;)?;
        <span class="k">for</span> (row, place) <span class="k">in</span> places {
            <span class="k">self</span>.gpu.objects.set_placement(&amp;<span class="k">self</span>.gpu.ctx, row, &amp;place);
            <span class="k">self</span>.gpu.grew_bounds(row);
        }</code></pre></div>
<p>Added after the line <code>let index = active.index;</code> in <code>lessons/31/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> parent = active.parent;
        <span class="k">let</span> id = active.id;
        <span class="c">// preview the geometry with the point moved</span>
        <span class="k">if</span> <span class="k">let</span> Some(source) = <span class="k">self</span>.scene.geometry(parent) {
            <span class="k">let</span> target = <span class="k">crate</span>::app::deform::Target::Control(id);
            <span class="k">if</span> <span class="k">let</span> Ok(points) = <span class="k">crate</span>::app::deform::points(source, target)
                &amp;&amp; <span class="k">let</span> Some(from) = points.first()
                &amp;&amp; <span class="k">let</span> Ok(edited) = <span class="k">crate</span>::app::deform::transform(
                    source,
                    target,
                    &amp;Xform::translation(point[<span class="s">0</span>] - from[<span class="s">0</span>], point[<span class="s">1</span>] - from[<span class="s">1</span>], point[<span class="s">2</span>] - from[<span class="s">2</span>]),
                )
            {
                <span class="k">let</span> _ = <span class="k">self</span>.scene.preview_geometry(parent, edited, &amp;<span class="k">mut</span> <span class="k">self</span>.gpu);
            }
        }</code></pre></div>
<p>Added after the line <code>};</code> in <code>lessons/31/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.restore_source_render(active.parent);</code></pre></div>
<p>Added after the line <code>let free = active.plane.hit(&amp;active.origin, &amp;from, &amp;dir)?;</code> in <code>lessons/31/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">if</span> !<span class="k">self</span>.snap_enabled {
            <span class="k">return</span> Some(free);
        }</code></pre></div>
<p>Replaces the line <code>fn project(&amp;self, at: [f64; 3]) -&gt; Option&lt;(f64, f64)&gt; {</code> in <code>lessons/31/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// A scene point in device pixels, or \`None\` behind the eye.</span>
    <span class="k">pub</span>(super) <span class="k">fn</span> project(&amp;<span class="k">self</span>, at: [f64; <span class="s">3</span>]) -&gt; Option&lt;(f64, f64)&gt; {</code></pre></div>
<h2 id="step-21-srcstatepanelrs">Step 21 · src/state/panel.rs<a class="anchor" href="#/course/32-colors-lighting#step-21-srcstatepanelrs" aria-label="Link to this section">#</a></h2>
<p>Apply selection and locking recursively through the layer hierarchy.</p>
<p><code>lessons/32/src/state/panel.rs</code> · edit · type this</p>
<p>Replaces the 2 lines from <code>if let Some((index, hex)) = value.split_once(&#39;/&#39;)</code> in <code>lessons/31/src/state/panel.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">let</span> parts: Vec&lt;_&gt; = value.split('<span class="s">/</span>').collect();

            <span class="k">if</span> parts.len() == <span class="s">3</span>
                &amp;&amp; <span class="k">let</span> Ok(index) = parts[<span class="s">0</span>].parse::&lt;usize&gt;()
                &amp;&amp; index &lt; <span class="k">self</span>.hierarchy.nodes.len()
                &amp;&amp; matches!(parts[<span class="s">1</span>], &quot;<span class="s">face</span>&quot; | &quot;<span class="s">edge</span>&quot;)
            {
                <span class="k">let</span> edge = parts[<span class="s">1</span>] == &quot;<span class="s">edge</span>&quot;;
                <span class="k">let</span> color = <span class="k">if</span> parts[<span class="s">2</span>] == &quot;<span class="s">original</span>&quot; {
                    None
                } <span class="k">else</span> {
                    <span class="k">let</span> Ok(rgb) = u32::from_str_radix(parts[<span class="s">2</span>], <span class="s">16</span>) <span class="k">else</span> {
                        <span class="k">return</span>;
                    };
                    Some([(rgb &gt;&gt; <span class="s">16</span>) <span class="k">as</span> u8, (rgb &gt;&gt; <span class="s">8</span>) <span class="k">as</span> u8, rgb <span class="k">as</span> u8])
                };

                <span class="k">for</span> row <span class="k">in</span> <span class="k">self</span>.hierarchy.targets(index) {
                    <span class="c">// an edge color needs faces to sit on</span>
                    <span class="k">if</span> edge
                        &amp;&amp; !<span class="k">self</span>.gpu.objects.row(row).is_some_and(|r| {
                            r.flags &amp; <span class="k">crate</span>::engine::gpu::Instance::FLAG_HAS_FACES != <span class="s">0</span>
                        })
                    {
                        <span class="k">continue</span>;
                    }

                    <span class="k">if</span> <span class="k">let</span> Some(id) = <span class="k">self</span>.scene.identity_of(row) {
                        <span class="k">let</span> colors = <span class="k">if</span> edge {
                            &amp;<span class="k">mut</span> <span class="k">self</span>.scene.edge_colors
                        } <span class="k">else</span> {
                            &amp;<span class="k">mut</span> <span class="k">self</span>.scene.colors
                        };

                        <span class="k">if</span> <span class="k">let</span> Some(color) = color {
                            colors.insert(id, color);
                        } <span class="k">else</span> {
                            colors.remove(&amp;id);
                        }

                        <span class="k">self</span>.gpu.set_object_color(row, edge, color);</code></pre></div>
<p>Replaces the line <code>&quot;select&quot; =&gt; {</code> in <code>lessons/31/src/state/panel.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                &quot;<span class="s">select</span>&quot; | &quot;<span class="s">add</span>&quot; =&gt; {</code></pre></div>
<p>Replaces the 13 lines from <code>self.select(None);</code> in <code>lessons/31/src/state/panel.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                    <span class="k">self</span>.select_rows(rows, action == &quot;<span class="s">add</span>&quot;);</code></pre></div>
<p>Added after the line <code>locked,</code> in <code>lessons/31/src/state/panel.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                selected: targets.iter().any(|r| {
                    <span class="k">self</span>.scene.selected == Some(*r) || <span class="k">self</span>.hierarchy.selected.contains(r)
                }),
                color,
                edge_color: targets
                    .first()
                    .and_then(|row| <span class="k">self</span>.scene.identity_of(*row))
                    .and_then(|id| <span class="k">self</span>.scene.edge_colors.get(&amp;id).copied())
                    .filter(|first| {
                        targets.iter().all(|row| {
                            <span class="k">self</span>.scene
                                .identity_of(*row)
                                .is_some_and(|id| <span class="k">self</span>.scene.edge_colors.get(&amp;id) == Some(first))
                        })
                    }),
                has_faces: targets.iter().any(|row| {
                    <span class="k">self</span>.gpu.objects.row(*row).is_some_and(|r| {
                        r.flags &amp; <span class="k">crate</span>::engine::gpu::Instance::FLAG_HAS_FACES != <span class="s">0</span>
                    })
                }),</code></pre></div>
<h2 id="step-22-session_rustsrccolorrs">Step 22 · session_rust/src/color.rs<a class="anchor" href="#/course/32-colors-lighting#step-22-session_rustsrccolorrs" aria-label="Link to this section">#</a></h2>
<p>Use very light grey for default geometry colors throughout the kernel.</p>
<p><code>session_rust/src/color.rs</code> · edit · type this</p>
<p>The kernel&#39;s <code>Color::lightgrey</code> is now 0.94 grey instead of 0.9; this is the live kernel, already changed:</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> <span class="k">fn</span> lightgrey() -&gt; <span class="k">Self</span> {
        <span class="k">Self</span>::with_name(<span class="s">0</span>.<span class="s">94</span>, <span class="s">0</span>.<span class="s">94</span>, <span class="s">0</span>.<span class="s">94</span>, <span class="s">1</span>.<span class="s">0</span>, &quot;<span class="s">lightgrey</span>&quot;)
    }</code></pre></div>
<p>The kernel&#39;s default colour is the same 0.94 grey:</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">impl</span> Default <span class="k">for</span> Color {
    <span class="c">/// Construct the default light grey color.</span>
    <span class="k">fn</span> default() -&gt; <span class="k">Self</span> {
        <span class="k">Self</span>::new(<span class="s">0</span>.<span class="s">94</span>, <span class="s">0</span>.<span class="s">94</span>, <span class="s">0</span>.<span class="s">94</span>, <span class="s">1</span>.<span class="s">0</span>)
    }
}</code></pre></div>
<h2 id="step-23-srcappcommandrs">Step 23 · src/app/command.rs<a class="anchor" href="#/course/32-colors-lighting#step-23-srcappcommandrs" aria-label="Link to this section">#</a></h2>
<p>Accept a partial command with Enter, then accept its default or arrow-selected option.</p>
<p><code>lessons/32/src/app/command.rs</code> · edit · type this</p>
<p>Added after the line <code>Fit,</code> in <code>lessons/31/src/app/command.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    Layers(Option&lt;bool&gt;),     <span class="c">// layer panel on, off or toggle</span>
    Selection(<span class="k">crate</span>::app::selection::SelectionTool), <span class="c">// pick objects, edges or faces</span>
    Controls,                 <span class="c">// pick control points</span>
    Ssao(Option&lt;bool&gt;),       <span class="c">// contact shading on, off or toggle</span>
    Snap(Option&lt;bool&gt;),       <span class="c">// snapping on, off or toggle</span></code></pre></div>
<p>Replaces the 2 lines from <code>&quot;point&quot; =&gt; &quot;Point x,y,z · Example: Point 0,0,0 · Enter cr…</code> in <code>lessons/31/src/app/command.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        &quot;<span class="s">point</span>&quot; =&gt; &quot;<span class="s">Point · Enter then click or type x,y,z</span>&quot;,
        &quot;<span class="s">line</span>&quot; =&gt; &quot;<span class="s">Line · Enter then click or type endpoints · Example: Line 0,0,0 100,0,0</span>&quot;,</code></pre></div>
<p>Replaces the line <code>_ =&gt; &quot;Try Point 0,0,0 · Line 0,0,0 100,0,0 · Fit · Undo ·…</code> in <code>lessons/31/src/app/command.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        &quot;<span class="s">layers</span>&quot; =&gt; &quot;<span class="s">Layers (On Off): show or hide the layer panel</span>&quot;,
        &quot;<span class="s">snap</span>&quot; =&gt; &quot;<span class="s">Snap (On Off): endpoints, vertices and midpoints within 12 pixels</span>&quot;,
        &quot;<span class="s">ssao</span>&quot; | &quot;<span class="s">arctic</span>&quot; =&gt; {
            &quot;<span class="s">SSAO (On Off): soft contact shading and studio lighting · G toggles in the viewport</span>&quot;
        }
        _ =&gt; &quot;<span class="s">Type a command · Up/Down browse · Tab completes · Enter executes · Esc cancels</span>&quot;,</code></pre></div>
<p>Added after the line <code>match verb.as_str() {</code> in <code>lessons/31/src/app/command.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        &quot;<span class="s">layers</span>&quot; =&gt; <span class="k">match</span> rest.as_slice() {
            [] =&gt; Ok(Command::Layers(None)),
            [value] <span class="k">if</span> value.eq_ignore_ascii_case(&quot;<span class="s">on</span>&quot;) =&gt; Ok(Command::Layers(Some(<span class="s">true</span>))),
            [value] <span class="k">if</span> value.eq_ignore_ascii_case(&quot;<span class="s">off</span>&quot;) =&gt; Ok(Command::Layers(Some(<span class="s">false</span>))),
            _ =&gt; Err(&quot;<span class="s">Layers (On Off)</span>&quot;.into()),
        },
        &quot;<span class="s">snap</span>&quot; =&gt; <span class="k">match</span> rest.as_slice() {
            [] =&gt; Ok(Command::Snap(None)),
            [value] <span class="k">if</span> value.eq_ignore_ascii_case(&quot;<span class="s">on</span>&quot;) =&gt; Ok(Command::Snap(Some(<span class="s">true</span>))),
            [value] <span class="k">if</span> value.eq_ignore_ascii_case(&quot;<span class="s">off</span>&quot;) =&gt; Ok(Command::Snap(Some(<span class="s">false</span>))),
            _ =&gt; Err(&quot;<span class="s">Snap (On Off)</span>&quot;.into()),
        },
        &quot;<span class="s">ssao</span>&quot; | &quot;<span class="s">arctic</span>&quot; =&gt; <span class="k">match</span> rest.as_slice() {
            [] =&gt; Ok(Command::Ssao(None)),
            [value] <span class="k">if</span> value.eq_ignore_ascii_case(&quot;<span class="s">on</span>&quot;) =&gt; Ok(Command::Ssao(Some(<span class="s">true</span>))),
            [value] <span class="k">if</span> value.eq_ignore_ascii_case(&quot;<span class="s">off</span>&quot;) =&gt; Ok(Command::Ssao(Some(<span class="s">false</span>))),
            _ =&gt; Err(&quot;<span class="s">SSAO (On Off)</span>&quot;.into()),
        },
        &quot;<span class="s">object</span>&quot; | &quot;<span class="s">edge</span>&quot; | &quot;<span class="s">face</span>&quot; | &quot;<span class="s">controls</span>&quot; <span class="k">if</span> rest.is_empty() =&gt; {
            <span class="k">use</span> <span class="k">crate</span>::app::selection::SelectionTool;
            Ok(<span class="k">match</span> verb.as_str() {
                &quot;<span class="s">object</span>&quot; =&gt; Command::Selection(SelectionTool::Object),
                &quot;<span class="s">edge</span>&quot; =&gt; Command::Selection(SelectionTool::Edge),
                &quot;<span class="s">face</span>&quot; =&gt; Command::Selection(SelectionTool::Face),
                _ =&gt; Command::Controls,
            })
        }</code></pre></div>
<p>Added after the line <code>other =&gt; Err(format!(&quot;no command &#39;{other}&#39;&quot;)),</code> in <code>lessons/31/src/app/command.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Clickable choices for the verb being typed.</span>
<span class="k">pub</span> <span class="k">fn</span> options(line: &amp;str) -&gt; &amp;'static [&amp;'static str] {
    <span class="k">match</span> line
        .split_whitespace()
        .next()
        .unwrap_or(&quot;&quot;)
        .to_ascii_lowercase()
        .as_str()
    {
        &quot;<span class="s">layers</span>&quot; =&gt; &amp;[&quot;<span class="s">Layers On</span>&quot;, &quot;<span class="s">Layers Off</span>&quot;],
        &quot;<span class="s">ssao</span>&quot; =&gt; &amp;[&quot;<span class="s">SSAO On</span>&quot;, &quot;<span class="s">SSAO Off</span>&quot;],
        &quot;<span class="s">arctic</span>&quot; =&gt; &amp;[&quot;<span class="s">Arctic On</span>&quot;, &quot;<span class="s">Arctic Off</span>&quot;],
        &quot;<span class="s">snap</span>&quot; =&gt; &amp;[&quot;<span class="s">Snap On</span>&quot;, &quot;<span class="s">Snap Off</span>&quot;],
        &quot;<span class="s">rotate</span>&quot; | &quot;<span class="s">rot</span>&quot; =&gt; &amp;[&quot;<span class="s">Rotate x</span>&quot;, &quot;<span class="s">Rotate y</span>&quot;, &quot;<span class="s">Rotate z</span>&quot;],
        _ =&gt; &amp;[],
    }
}

<span class="c">/// Commands or options starting with the typed text.</span>
<span class="k">pub</span> <span class="k">fn</span> completions(line: &amp;str) -&gt; Vec&lt;&amp;'static str&gt; {
    <span class="k">const</span> COMMANDS: &amp;[&amp;str] = &amp;[
        &quot;<span class="s">Arctic</span>&quot;, &quot;<span class="s">Controls</span>&quot;, &quot;<span class="s">Curve</span>&quot;, &quot;<span class="s">Delete</span>&quot;, &quot;<span class="s">Edge</span>&quot;, &quot;<span class="s">Escape</span>&quot;, &quot;<span class="s">Explode</span>&quot;, &quot;<span class="s">Extend</span>&quot;, &quot;<span class="s">Face</span>&quot;,
        &quot;<span class="s">Fit</span>&quot;, &quot;<span class="s">Hide</span>&quot;, &quot;<span class="s">Layers</span>&quot;, &quot;<span class="s">Line</span>&quot;, &quot;<span class="s">Move</span>&quot;, &quot;<span class="s">Object</span>&quot;, &quot;<span class="s">Open</span>&quot;, &quot;<span class="s">Point</span>&quot;, &quot;<span class="s">Polyline</span>&quot;, &quot;<span class="s">Redo</span>&quot;,
        &quot;<span class="s">Rotate</span>&quot;, &quot;<span class="s">Save</span>&quot;, &quot;<span class="s">Scale</span>&quot;, &quot;<span class="s">Show</span>&quot;, &quot;<span class="s">Snap</span>&quot;, &quot;<span class="s">Split</span>&quot;, &quot;<span class="s">SSAO</span>&quot;, &quot;<span class="s">Trim</span>&quot;, &quot;<span class="s">Undo</span>&quot;,
    ];
    <span class="k">let</span> lower = line.to_ascii_lowercase();
    <span class="c">// after a space, complete the option instead</span>
    <span class="k">let</span> choices = <span class="k">if</span> lower.contains('<span class="s"> </span>') {
        options(line)
    } <span class="k">else</span> {
        COMMANDS
    };
    choices
        .iter()
        .copied()
        .filter(|name| name.to_ascii_lowercase().starts_with(&amp;lower))
        .collect()
}

<span class="c">/// Every command, matching ones first.</span>
<span class="k">pub</span> <span class="k">fn</span> browse(line: &amp;str) -&gt; Vec&lt;&amp;'static str&gt; {
    <span class="k">if</span> line.contains('<span class="s"> </span>') {
        <span class="k">return</span> options(line).to_vec();
    }
    <span class="k">let</span> all = completions(&quot;&quot;);
    <span class="k">let</span> lower = line.to_ascii_lowercase();
    all.iter()
        .copied()
        .filter(|name| name.to_ascii_lowercase().starts_with(&amp;lower))
        .chain(
            all.iter()
                .copied()
                .filter(|name| !name.to_ascii_lowercase().starts_with(&amp;lower)),
        )
        .collect()
}

<span class="c">/// Take the first completion; false when arguments are still needed.</span>
<span class="k">pub</span> <span class="k">fn</span> accept(line: &amp;str) -&gt; (String, bool) {
    <span class="k">if</span> line.trim().is_empty() {
        <span class="k">return</span> (String::new(), <span class="s">true</span>);
    }
    <span class="k">let</span> choices = completions(line);
    <span class="k">let</span> text = choices.first().copied().unwrap_or(line).trim();
    <span class="k">let</span> words: Vec&lt;_&gt; = text.split_whitespace().collect();
    <span class="k">if</span> (!options(text).is_empty() &amp;&amp; words.len() == <span class="s">1</span>)
        || (words.len() == <span class="s">2</span>
            &amp;&amp; words[<span class="s">0</span>].eq_ignore_ascii_case(&quot;<span class="s">rotate</span>&quot;)
            &amp;&amp; [&quot;<span class="s">x</span>&quot;, &quot;<span class="s">y</span>&quot;, &quot;<span class="s">z</span>&quot;]
                .iter()
                .any(|axis| words[<span class="s">1</span>].eq_ignore_ascii_case(axis)))
    {
        (format!(&quot;{<span class="s">text</span>}<span class="s"> </span>&quot;), <span class="s">false</span>)
    } <span class="k">else</span> {
        (text.to_owned(), <span class="s">true</span>)
    }
}

<span class="c">/// A move offset from \`10 0 0\`, \`@10,0\` or \`10&lt;45\`.</span></code></pre></div>
<p>Added after the line <code>use super::*;</code> in <code>lessons/31/src/app/command.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Tab completes the verb, then its option.</span>
    #[test]
    <span class="k">fn</span> partial_entries_accept_commands_then_options() {
        assert_eq!(accept(&quot;<span class="s">Lay</span>&quot;), (&quot;<span class="s">Layers </span>&quot;.into(), <span class="s">false</span>));
        assert_eq!(accept(&quot;<span class="s">Layers </span>&quot;), (&quot;<span class="s">Layers On</span>&quot;.into(), <span class="s">true</span>));
        assert_eq!(accept(&quot;<span class="s">Layers of</span>&quot;), (&quot;<span class="s">Layers Off</span>&quot;.into(), <span class="s">true</span>));
        assert_eq!(accept(&quot;<span class="s">Lin</span>&quot;), (&quot;<span class="s">Line</span>&quot;.into(), <span class="s">true</span>));
        assert_eq!(accept(&quot;<span class="s">Rotate </span>&quot;), (&quot;<span class="s">Rotate x </span>&quot;.into(), <span class="s">false</span>));
        assert_eq!(accept(&quot;<span class="s">Rotate x 45</span>&quot;), (&quot;<span class="s">Rotate x 45</span>&quot;.into(), <span class="s">true</span>));
        assert_eq!(accept(&quot;&quot;), (String::new(), <span class="s">true</span>));
        assert_eq!(browse(&quot;<span class="s">la</span>&quot;)[<span class="s">0</span>], &quot;<span class="s">Layers</span>&quot;);
        assert_eq!(browse(&quot;<span class="s">la</span>&quot;).len(), completions(&quot;&quot;).len());
        assert_eq!(browse(&quot;<span class="s">forgot</span>&quot;), completions(&quot;&quot;));
        assert_eq!(browse(&quot;<span class="s">Layers o</span>&quot;), vec![&quot;<span class="s">Layers On</span>&quot;, &quot;<span class="s">Layers Off</span>&quot;]);
    }

    <span class="c">/// Completion and parsing ignore case.</span>
    #[test]
    <span class="k">fn</span> discovery_and_layer_options_are_case_insensitive() {
        assert_eq!(completions(&quot;<span class="s">la</span>&quot;), vec![&quot;<span class="s">Layers</span>&quot;]);
        assert_eq!(completions(&quot;<span class="s">Layers </span>&quot;), vec![&quot;<span class="s">Layers On</span>&quot;, &quot;<span class="s">Layers Off</span>&quot;]);
        assert!(completions(&quot;&quot;).contains(&amp;&quot;<span class="s">Controls</span>&quot;));
        assert_eq!(parse(&quot;<span class="s">Layers OFF</span>&quot;), Ok(Command::Layers(Some(<span class="s">false</span>))));
        assert!(parse(&quot;<span class="s">Layers maybe</span>&quot;).is_err());
    }

    <span class="c">/// Short forms parse like the full verb.</span></code></pre></div>
<h2 id="step-24-srcappeditrs">Step 24 · src/app/edit.rs<a class="anchor" href="#/course/32-colors-lighting#step-24-srcappeditrs" aria-label="Link to this section">#</a></h2>
<p>Collect selected descendants and transform the selection as one set.</p>
<p><code>lessons/32/src/app/edit.rs</code> · edit · type this</p>
<p>Added after the line <code>impl Scene {</code> in <code>lessons/31/src/app/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Move several rows by one world delta, one undo step per document.</span>
    <span class="k">pub</span> <span class="k">fn</span> transform_rows(
        &amp;<span class="k">mut</span> <span class="k">self</span>,
        rows: &amp;[u32],
        delta: &amp;Xform,
        label: &amp;str,
    ) -&gt; Option&lt;Vec&lt;(u32, Xform)&gt;&gt; {
        <span class="c">// (document, guid, new local transform) per row</span>
        <span class="k">let</span> <span class="k">mut</span> changes = rows
            .iter()
            .map(|&amp;row| {
                <span class="k">let</span> (doc, guid) = <span class="k">self</span>.identity_of(row)?;
                <span class="k">if</span> <span class="k">self</span>.docs.get(doc)?.display_only {
                    <span class="k">return</span> None;
                }
                <span class="k">let</span> base = <span class="k">self</span>.local_xform_of(row)?;
                Some((doc, guid, <span class="k">self</span>.local_for_world_delta(row, delta, &amp;base)?))
            })
            .collect::&lt;Option&lt;Vec&lt;_&gt;&gt;&gt;()?;
        <span class="k">let</span> selected: std::collections::HashSet&lt;_&gt; = changes
            .iter()
            .map(|(doc, guid, _)| (*doc, guid.to_string()))
            .collect();
        <span class="c">// drop a child whose parent is also selected</span>
        changes.retain(|(doc, guid, _)| {
            <span class="k">self</span>.docs[*doc]
                .session
                .tree
                .get_node_by_name(guid)
                .is_none_or(|node| {
                    node.borrow()
                        .ancestors()
                        .iter()
                        .all(|ancestor| !selected.contains(&amp;(*doc, ancestor.borrow().name.clone())))
                })
        });
        <span class="k">let</span> <span class="k">mut</span> docs: Vec&lt;_&gt; = changes.iter().map(|c| c.<span class="s">0</span>).collect();
        docs.sort_unstable();
        docs.dedup(); <span class="c">// each document once</span>
        <span class="k">for</span> doc <span class="k">in</span> docs {
            <span class="k">let</span> session = Rc::make_mut(&amp;<span class="k">mut</span> <span class="k">self</span>.docs[doc].session);
            session.begin(label);
            <span class="k">for</span> (_, guid, local) <span class="k">in</span> changes.iter().filter(|c| c.<span class="s">0</span> == doc) {
                session.set_xform(guid, local.clone());
            }
            session.commit();
            <span class="k">self</span>.last_edited = Some(doc);
        }
        rows.iter()
            .map(|&amp;row| Some((row, <span class="k">self</span>.placement_of(row)?)))
            .collect()
    }

    <span class="c">/// The row's document and guid, with its session made private.</span></code></pre></div>
<p>Added after the line <code>use session_rust::{Point, Session};</code> in <code>lessons/31/src/app/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// One undo puts every moved row back.</span>
    #[test]
    <span class="k">fn</span> group_transform_undo_restores_every_member_in_one_document() {
        <span class="k">let</span> <span class="k">mut</span> source = Session::new(&quot;<span class="s">group</span>&quot;);
        source.add_point(Point::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), None);
        source.add_point(Point::new(<span class="s">10</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), None);
        <span class="k">let</span> <span class="k">mut</span> scene = Scene::new();
        scene.add_file(file(&quot;<span class="s">group</span>&quot;, Rc::new(source)));
        <span class="k">let</span> moved = scene
            .transform_rows(&amp;[<span class="s">0</span>, <span class="s">1</span>], &amp;Xform::translation(<span class="s">5</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), &quot;<span class="s">move group</span>&quot;)
            .unwrap();
        assert!(moved.iter().all(|(_, m)| m.m[<span class="s">12</span>] == <span class="s">5</span>.<span class="s">0</span>));
        assert!(scene.undo());
        assert!(
            [<span class="s">0</span>, <span class="s">1</span>]
                .into_iter()
                .all(|r| scene.placement_of(r).unwrap().m[<span class="s">12</span>] == <span class="s">0</span>.<span class="s">0</span>)
        );
    }

    <span class="c">/// A child of a moved parent is not moved twice.</span>
    #[test]
    <span class="k">fn</span> selected_child_inherits_the_selected_parents_delta_once() {
        <span class="k">let</span> <span class="k">mut</span> source = Session::new(&quot;<span class="s">nested</span>&quot;);
        <span class="k">let</span> parent = source.add_point(Point::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), None);
        source.add_point(Point::new(<span class="s">10</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), Some(&amp;parent));
        <span class="k">let</span> <span class="k">mut</span> scene = Scene::new();
        scene.add_file(file(&quot;<span class="s">nested</span>&quot;, Rc::new(source)));
        <span class="k">let</span> moved = scene
            .transform_rows(&amp;[<span class="s">0</span>, <span class="s">1</span>], &amp;Xform::translation(<span class="s">5</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), &quot;<span class="s">move nested</span>&quot;)
            .unwrap();
        assert!(moved.iter().all(|(_, m)| m.m[<span class="s">12</span>] == <span class="s">5</span>.<span class="s">0</span>));
        assert!(scene.undo());
        assert!(
            [<span class="s">0</span>, <span class="s">1</span>]
                .into_iter()
                .all(|r| scene.placement_of(r).unwrap().m[<span class="s">12</span>] == <span class="s">0</span>.<span class="s">0</span>)
        );
    }

    <span class="c">/// A document at the origin.</span></code></pre></div>
<h2 id="step-25-srcappgizmors">Step 25 · src/app/gizmo.rs<a class="anchor" href="#/course/32-colors-lighting#step-25-srcappgizmors" aria-label="Link to this section">#</a></h2>
<p>Measure the distance from the pointer ray to each handle.</p>
<p><code>lessons/32/src/app/gizmo.rs</code> · edit · type this</p>
<p>Replaces the line <code>let (_, t) = line_line_parameters(&amp;ray, &amp;line, 1e-9, fals…</code> in <code>lessons/31/src/app/gizmo.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">let</span> (_, t) = line_line_parameters(&amp;ray, &amp;line, <span class="s">0</span>.<span class="s">0</span>, <span class="s">false</span>, <span class="s">false</span>)?;</code></pre></div>
<p>Added after the line <code>use super::*;</code> in <code>lessons/31/src/app/gizmo.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// A ray a few pixels off the arm still grabs it.</span>
    #[test]
    <span class="k">fn</span> arm_grab_accepts_a_ray_within_the_screen_aperture() {
        <span class="k">let</span> gizmo = Gizmo::new(Point::new(<span class="s">5</span>.<span class="s">0</span>, <span class="s">45</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>));
        <span class="k">let</span> from = Point::new(<span class="s">25</span>.<span class="s">0</span>, <span class="s">44</span>.<span class="s">5</span>, <span class="s">100</span>.<span class="s">0</span>);
        assert_eq!(
            gizmo.hit(&amp;from, &amp;Vector::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, -<span class="s">1</span>.<span class="s">0</span>), <span class="s">0</span>.<span class="s">25</span>),
            Some(Handle::Translate(Axis::X))
        );
    }</code></pre></div>
<h2 id="step-26-srcappinputrs">Step 26 · src/app/input.rs<a class="anchor" href="#/course/32-colors-lighting#step-26-srcappinputrs" aria-label="Link to this section">#</a></h2>
<p>Route pending drawing clicks before selection; use Shift to add objects and G to toggle Arctic lighting.</p>
<p><code>lessons/32/src/app/input.rs</code> · edit · type this</p>
<p>Replaces the 2 lines from <code>Key::Named(NamedKey::Escape) =&gt; state.escape_selection(),</code> in <code>lessons/31/src/app/input.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            Key::Named(NamedKey::Escape) =&gt; {
                state.draft = None;
                state.escape_selection();
            }
            Key::Named(NamedKey::Enter) =&gt; {
                <span class="k">if</span> state.draft.is_some() {
                    <span class="k">let</span> result = state.run_command(&quot;&quot;);
                    <span class="k">crate</span>::app::feedback::status(&amp;result.unwrap_or_else(|e| e));
                } <span class="k">else</span> {
                    state.confirm_split();
                }
            }</code></pre></div>
<p>Added after the line <code>}</code> in <code>lessons/31/src/app/input.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            Key::Character(&quot;<span class="s">g</span>&quot; | &quot;<span class="s">G</span>&quot;) =&gt; state.gpu.view.ssao = !state.gpu.view.ssao,</code></pre></div>
<p>Replaces the line <code>dragging || state.hover_gizmo(position.x, position.y)</code> in <code>lessons/31/src/app/input.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                dragging
                    || state.hover_drawing(position.x, position.y)
                    || state.hover_gizmo(position.x, position.y)</code></pre></div>
<p>Replaces the line <code>if !self.ctrl &amp;&amp; state.begin_control_drag(self.last_curso…</code> in <code>lessons/31/src/app/input.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                <span class="c">// while drawing, a press is only a click</span>
                <span class="k">if</span> state.draft.is_some() {
                    <span class="k">self</span>.left_down = Some(<span class="k">self</span>.last_cursor);
                    <span class="k">return</span> <span class="s">false</span>;
                }
                <span class="k">if</span> !<span class="k">self</span>.ctrl
                    &amp;&amp; !<span class="k">self</span>.shift
                    &amp;&amp; state.begin_control_drag(<span class="k">self</span>.last_cursor.<span class="s">0</span>, <span class="k">self</span>.last_cursor.<span class="s">1</span>)
                {
                    <span class="k">self</span>.control_drag = <span class="s">true</span>;
                    <span class="k">return</span> <span class="s">false</span>;
                }

                <span class="k">if</span> !<span class="k">self</span>.ctrl
                    &amp;&amp; !<span class="k">self</span>.shift
                    &amp;&amp; state.begin_gizmo(<span class="k">self</span>.last_cursor.<span class="s">0</span>, <span class="k">self</span>.last_cursor.<span class="s">1</span>)
                {</code></pre></div>
<p>Added after the line <code>}</code> in <code>lessons/31/src/app/input.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                <span class="k">if</span> state.draft.is_some() {
                    <span class="k">return</span> state.click_drawing(<span class="k">self</span>.last_cursor.<span class="s">0</span>, <span class="k">self</span>.last_cursor.<span class="s">1</span>);
                }
                state.additive_selection = <span class="k">self</span>.shift &amp;&amp; !<span class="k">self</span>.ctrl; <span class="c">// Shift adds to the selection</span></code></pre></div>
<h2 id="step-27-srcappsurface_previewrs">Step 27 · src/app/surface_preview.rs<a class="anchor" href="#/course/32-colors-lighting#step-27-srcappsurface_previewrs" aria-label="Link to this section">#</a></h2>
<p>Reuse the sampled surface grid and update its positions during a gesture.</p>
<p><code>lessons/32/src/app/surface_preview.rs</code> · edit · type this</p>
<p>Replaces the 7 lines from <code>let pipe_vertices = pipes</code> in <code>lessons/31/src/app/surface_preview.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="c">// pipe ends by vertex index, not by position</span>
        <span class="k">let</span> boundary: HashMap&lt;_, _&gt; = up.arena.surface_boundaries.iter().copied().collect();
        <span class="k">let</span> pipe_vertices = (start_pipe..end_pipe)
            .map(|pipe| {
                <span class="k">let</span> ends = boundary.get(&amp;(pipe <span class="k">as</span> u32))?;
                Some([ends[<span class="s">0</span>] <span class="k">as</span> usize - first, ends[<span class="s">1</span>] <span class="k">as</span> usize - first])</code></pre></div>
<p>Added after the line <code>let b = vertices[normals[1]].normal.map(f64::from);</code> in <code>lessons/31/src/app/surface_preview.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">if</span> pipe.facing != super::walk::encode::FACING_UNKNOWN {
                pipe.facing = super::walk::encode::pack_facing(Some(&amp;a), Some(&amp;b));
            }</code></pre></div>
<p>Added after the line <code>use std::rc::Rc;</code> in <code>lessons/31/src/app/surface_preview.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Vertices at one point keep their own uv when pulled apart.</span>
    #[test]
    <span class="k">fn</span> coincident_boundary_samples_keep_their_own_uv_after_edit() {
        <span class="k">let</span> <span class="k">mut</span> surface = BRep::create_box(<span class="s">40</span>.<span class="s">0</span>, <span class="s">30</span>.<span class="s">0</span>, <span class="s">25</span>.<span class="s">0</span>).m_surfaces[<span class="s">0</span>].clone();
        <span class="c">// collapse one edge to a point, then open it</span>
        <span class="k">let</span> pole = surface.get_cv(<span class="s">0</span>, <span class="s">0</span>).unwrap();
        surface.set_cv(<span class="s">0</span>, <span class="s">1</span>, &amp;pole);
        <span class="k">let</span> source = Geometry::NurbsSurface(Rc::new(surface.clone()));
        <span class="k">let</span> <span class="k">mut</span> upload = Upload::default();
        walk_geometry(
            &amp;<span class="k">mut</span> Walk::of(&amp;<span class="k">mut</span> upload),
            &amp;WalkCx {
                vert_base: <span class="s">0</span>,
                cloud_px: <span class="s">0</span>.<span class="s">0</span>,
                row: <span class="s">0</span>,
            },
            &amp;source,
        );
        <span class="k">let</span> span = Span {
            start: Counts::default(),
            count: Counts::of(&amp;upload),
        };
        <span class="k">let</span> preview = SurfacePreview::capture(&amp;upload, span, Counts::default(), &amp;source).unwrap();
        surface.set_cv(
            <span class="s">0</span>,
            <span class="s">1</span>,
            &amp;session_rust::Point::new(pole[<span class="s">0</span>], pole[<span class="s">1</span>], pole[<span class="s">2</span>] + <span class="s">12</span>.<span class="s">0</span>),
        );
        <span class="k">let</span> (_, pipes, _) = preview
            .evaluate(&amp;Geometry::NurbsSurface(Rc::new(surface)))
            .unwrap();
        <span class="k">let</span> first_boundary = &amp;upload.seg.pipe_chains[<span class="s">0</span>];
        assert!(
            pipes.pipes[first_boundary.start <span class="k">as</span> usize..first_boundary.end <span class="k">as</span> usize]
                .iter()
                .all(|p| p.p0 != p.p1),
            &quot;<span class="s">each formerly coincident endpoint must follow its own parameter</span>&quot;
        );
        <span class="k">for</span> pair <span class="k">in</span>
            pipes.pipes[first_boundary.start <span class="k">as</span> usize..first_boundary.end <span class="k">as</span> usize].windows(<span class="s">2</span>)
        {
            assert_eq!(pair[<span class="s">0</span>].p1, pair[<span class="s">1</span>].p0);
        }
    }

    <span class="c">/// A surface preview keeps its four edges joined.</span>
    #[test]
    <span class="k">fn</span> standalone_surface_preview_keeps_connectivity_and_only_four_boundary_ids() {
        <span class="k">let</span> box_ = BRep::create_box(<span class="s">40</span>.<span class="s">0</span>, <span class="s">30</span>.<span class="s">0</span>, <span class="s">25</span>.<span class="s">0</span>);
        <span class="k">let</span> source = Geometry::NurbsSurface(Rc::new(box_.m_surfaces[<span class="s">0</span>].clone()));
        <span class="k">let</span> <span class="k">mut</span> upload = Upload::default();
        walk_geometry(
            &amp;<span class="k">mut</span> Walk::of(&amp;<span class="k">mut</span> upload),
            &amp;WalkCx {
                vert_base: <span class="s">0</span>,
                cloud_px: <span class="s">0</span>.<span class="s">0</span>,
                row: <span class="s">0</span>,
            },
            &amp;source,
        );
        <span class="k">let</span> span = Span {
            start: Counts::default(),
            count: Counts::of(&amp;upload),
        };
        <span class="k">let</span> preview = SurfacePreview::capture(&amp;upload, span, Counts::default(), &amp;source)
            .expect(&quot;<span class="s">every grid vertex retains its UV</span>&quot;);
        assert!(upload.seg.pipe_ids.iter().all(|id| *id &lt; <span class="s">4</span>));
        assert_eq!(upload.seg.pipe_chains.len(), <span class="s">4</span>);
        <span class="k">for</span> edge <span class="k">in</span> <span class="s">0</span>..<span class="s">4</span> {
            <span class="k">let</span> changed = deform::transform(
                &amp;source,
                Target::Edge(edge),
                &amp;Xform::translation(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">7</span>.<span class="s">0</span>),
            )
            .unwrap();
            <span class="k">let</span> (vertices, pipes, _) = preview.evaluate(&amp;changed).unwrap();
            assert_eq!(vertices.len(), upload.arena.verts.len());
            assert_eq!(pipes.pipes.len(), upload.seg.pipes.len());
            assert!(
                vertices
                    .iter()
                    .zip(&amp;upload.arena.verts)
                    .any(|(a, b)| a.position != b.position)
            );
            <span class="k">for</span> pipe <span class="k">in</span> pipes.pipes {
                assert_eq!(pipe.facing, super::super::walk::encode::FACING_UNKNOWN);
                assert!(vertices.iter().any(|v| v.position == pipe.p0));
                assert!(vertices.iter().any(|v| v.position == pipe.p1));
            }
        }
    }

    <span class="c">/// A box preview moves with a face and restores on cancel.</span></code></pre></div>
<h2 id="step-28-srcappwalkbreprs">Step 28 · src/app/walk/brep.rs<a class="anchor" href="#/course/32-colors-lighting#step-28-srcappwalkbreprs" aria-label="Link to this section">#</a></h2>
<p>Sample a bounded grid for standalone NURBS surfaces and build their natural boundary curves once.</p>
<p><code>lessons/32/src/app/walk/brep.rs</code> · edit · type this</p>
<p>Replaces the line <code>use super::mesh::{MeshCx, MeshOpts, mesh_spacing, walk_me…</code> in <code>lessons/31/src/app/walk/brep.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> super::mesh::mesh_spacing;</code></pre></div>
<p>Added after the line <code>let mut verts = 0;</code> in <code>lessons/31/src/app/walk/brep.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">let</span> <span class="k">mut</span> boundary_vertices = Vec::with_capacity(fms.len()); <span class="c">// per face: vertex key to GPU index</span></code></pre></div>
<p>Added after the line <code>}</code> in <code>lessons/31/src/app/walk/brep.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> <span class="k">mut</span> keys: Vec&lt;_&gt; = fm.vertex.keys().copied().collect();
        keys.sort_unstable();
        boundary_vertices.push(
            keys.into_iter()
                .enumerate()
                .map(|(i, key)| (key, arena.verts.len() <span class="k">as</span> u32 + i <span class="k">as</span> u32))
                .collect::&lt;std::collections::HashMap&lt;_, _&gt;&gt;(),
        );</code></pre></div>
<p>Replaces the line <code>walk_brep_edges(ink, b, &amp;chains, (&amp;ep, &amp;mut row.bounds));</code> in <code>lessons/31/src/app/walk/brep.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        walk_brep_edges(
            ink,
            b,
            &amp;chains,
            (&amp;ep, &amp;<span class="k">mut</span> row.bounds),
            arena,
            &amp;boundary_vertices,
        );</code></pre></div>
<p>Added after the line <code>out: (&amp;EdgePen, &amp;mut AABB),</code> in <code>lessons/31/src/app/walk/brep.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    arena: &amp;<span class="k">mut</span> ArenaRows,
    boundary_vertices: &amp;[std::collections::HashMap&lt;usize, u32&gt;],
) {
    <span class="k">let</span> (ep, bounds) = out;

    <span class="k">for</span> (ei, chain) <span class="k">in</span> chains.iter().enumerate() {
        <span class="k">match</span> chain {
            Some(c) =&gt; {
                <span class="k">let</span> <span class="k">mut</span> pipe = ink.seg.pipes.len() <span class="k">as</span> u32;
                push_edge_pipes(ink.seg, c, ep, bounds);
                <span class="c">// remember which vertices each pipe joins</span>
                <span class="k">for</span> pair <span class="k">in</span> c.keys.windows(<span class="s">2</span>) {
                    <span class="k">let</span> ends = [
                        boundary_vertices[c.face][&amp;pair[<span class="s">0</span>]],
                        boundary_vertices[c.face][&amp;pair[<span class="s">1</span>]],
                    ];
                    <span class="k">let</span> a = arena.verts[ends[<span class="s">0</span>] <span class="k">as</span> usize].position;
                    <span class="k">let</span> b = arena.verts[ends[<span class="s">1</span>] <span class="k">as</span> usize].position;
                    <span class="k">if</span> a == b || !a.into_iter().chain(b).all(f32::is_finite) {
                        <span class="k">continue</span>;
                    }
                    arena.surface_boundaries.push((pipe, ends));
                    pipe += <span class="s">1</span>;
                }</code></pre></div>
<p>Replaces the lines from <code>let mut sm = if let Some(mesh) = &amp;s.m_mesh {</code> in <code>lessons/31/src/app/walk/brep.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// A NURBS surface as a fixed UV grid with its four border edges.</span>
<span class="k">pub</span> <span class="k">fn</span> walk_surface(arena: &amp;<span class="k">mut</span> ArenaRows, ink: &amp;<span class="k">mut</span> Ink, s: &amp;NurbsSurface, cx: &amp;WalkCx) -&gt; Row {
    <span class="k">let</span> (Some((u0, u1)), Some((v0, v1))) = (s.domain(<span class="s">0</span>), s.domain(<span class="s">1</span>)) <span class="k">else</span> {
        <span class="k">return</span> Row::thin(AABB::empty());
    };
    <span class="k">let</span> nu = (s.m_cv_count[<span class="s">0</span>] * <span class="s">4</span>).clamp(<span class="s">16</span>, <span class="s">96</span>); <span class="c">// grid steps in u</span>
    <span class="k">let</span> nv = (s.m_cv_count[<span class="s">1</span>] * <span class="s">4</span>).clamp(<span class="s">16</span>, <span class="s">96</span>); <span class="c">// grid steps in v</span>
    <span class="k">let</span> first = arena.verts.len(); <span class="c">// index of this surface's first vertex</span>
    <span class="k">let</span> base = cx.vert_base + first <span class="k">as</span> u32; <span class="c">// first GPU vertex index</span>
    <span class="k">let</span> color = s.facecolors.first().cloned().unwrap_or_default().to_f32();
    <span class="k">let</span> <span class="k">mut</span> bounds = AABB::empty();
    <span class="c">// one vertex per grid point</span>
    <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..=nu {
        <span class="k">for</span> j <span class="k">in</span> <span class="s">0</span>..=nv {
            <span class="k">let</span> u = u0 + (u1 - u0) * i <span class="k">as</span> f64 / nu <span class="k">as</span> f64;
            <span class="k">let</span> v = v0 + (v1 - v0) * j <span class="k">as</span> f64 / nv <span class="k">as</span> f64;
            <span class="k">let</span> point = s.point_at(u, v).unwrap_or_default();
            <span class="k">let</span> normal = s.normal_at(u, v);
            bounds.union_with_point(point[<span class="s">0</span>], point[<span class="s">1</span>], point[<span class="s">2</span>]);
            arena
                .surface_samples
                .push(<span class="k">crate</span>::app::surface_preview::Sample {
                    index: arena.verts.len() <span class="k">as</span> u32,
                    surface: <span class="s">0</span>,
                    uv: [u, v],
                    sign: <span class="s">1</span>.<span class="s">0</span>,
                });
            arena.verts.push(session_rust::RenderVertex {
                position: point.to_f32(),
                normal: [normal[<span class="s">0</span>] <span class="k">as</span> f32, normal[<span class="s">1</span>] <span class="k">as</span> f32, normal[<span class="s">2</span>] <span class="k">as</span> f32],
                color,
            });
            arena.vids.push(cx.row);
        }
    }
    <span class="k">let</span> address = arena.face_sources.len() <span class="k">as</span> u32;
    arena
        .face_sources
        .push(<span class="k">crate</span>::engine::gpu::faces::FaceSource {
            parent: cx.row,
            face: <span class="s">0</span>,
        });
    <span class="c">// two triangles per grid cell</span>
    <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..nu {
        <span class="k">for</span> j <span class="k">in</span> <span class="s">0</span>..nv {
            <span class="k">let</span> a = base + (i * (nv + <span class="s">1</span>) + j) <span class="k">as</span> u32;
            <span class="k">let</span> b = a + (nv + <span class="s">1</span>) <span class="k">as</span> u32;
            arena.idx.extend_from_slice(&amp;[a, b, b + <span class="s">1</span>, a, b + <span class="s">1</span>, a + <span class="s">1</span>]);
            arena.face_ids.extend_from_slice(&amp;[address, address]);
        }
    }
    <span class="k">if</span> !knobs::no_edges() {
        ink.seg.pipe_ids.resize(ink.seg.pipes.len(), u32::MAX);
        <span class="c">// the four border edges, skipping a closed direction</span>
        <span class="k">for</span> edge <span class="k">in</span> <span class="s">0</span>..<span class="s">4</span> {
            <span class="k">let</span> direction = edge / <span class="s">2</span>;
            <span class="k">if</span> s.is_closed(direction) {
                <span class="k">continue</span>;
            }
            <span class="k">let</span> indices: Vec&lt;_&gt; = <span class="k">match</span> edge {
                <span class="s">0</span> =&gt; (<span class="s">0</span>..=nv).collect(),
                <span class="s">1</span> =&gt; (<span class="s">0</span>..=nv).map(|j| nu * (nv + <span class="s">1</span>) + j).collect(),
                <span class="s">2</span> =&gt; (<span class="s">0</span>..=nu).map(|i| i * (nv + <span class="s">1</span>)).collect(),
                _ =&gt; (<span class="s">0</span>..=nu).map(|i| i * (nv + <span class="s">1</span>) + nv).collect(),
            };
            <span class="k">let</span> start = ink.seg.pipes.len() <span class="k">as</span> u32;
            <span class="k">for</span> ends <span class="k">in</span> indices.windows(<span class="s">2</span>) {
                ink.seg.pipes.push(<span class="k">crate</span>::engine::gpu::CylinderSegment {
                    p0: arena.verts[first + ends[<span class="s">0</span>]].position,
                    p1: arena.verts[first + ends[<span class="s">1</span>]].position,
                    radius: encode_width(s.width),
                    instance_id: cx.row,
                    color: super::encode::BLACK,
                    facing: super::encode::FACING_UNKNOWN,
                });
                arena.surface_boundaries.push((
                    ink.seg.pipes.len() <span class="k">as</span> u32 - <span class="s">1</span>,
                    [(first + ends[<span class="s">0</span>]) <span class="k">as</span> u32, (first + ends[<span class="s">1</span>]) <span class="k">as</span> u32],
                ));
                ink.seg.pipe_ids.push(edge <span class="k">as</span> u32);
            }
            ink.seg.pipe_chains.push(start..ink.seg.pipes.len() <span class="k">as</span> u32);
        }
    }
    Row {
        bounds,
        spacing: mesh_spacing(&amp;bounds, (nu + <span class="s">1</span>) * (nv + <span class="s">1</span>)),
        flags: Instance::FLAG_SINGLE | Instance::FLAG_SMOOTH | Instance::FLAG_OPEN,
        faces: <span class="s">true</span>,</code></pre></div>
<p>Added after the line <code>(arena, seg, glyph, row)</code> in <code>lessons/31/src/app/walk/brep.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// The teapot's pipes lie on its authored edge curves.</span>
    #[test]
    <span class="k">fn</span> teapot_ink_is_only_authored_patch_boundaries() {
        <span class="k">let</span> scene = session_rust::Session::pb_load(concat!(
            env!(&quot;<span class="s">CARGO_MANIFEST_DIR</span>&quot;),
            &quot;<span class="s">/assets/pb/view_mixed_teapot.pb</span>&quot;
        ))
        .unwrap();
        <span class="k">let</span> brep = &amp;scene.objects.breps[<span class="s">0</span>];
        <span class="k">let</span> (arena, seg, glyph, _) = walked(brep);
        assert!(seg.ribbons.is_empty() &amp;&amp; glyph.spheres.is_empty());
        assert_eq!(arena.surface_boundaries.len(), seg.pipes.len());
        <span class="k">let</span> curves: Vec&lt;Vec&lt;_&gt;&gt; = brep
            .m_edges
            .iter()
            .map(|edge| {
                <span class="k">if</span> edge.curve_3d_index &lt; <span class="s">0</span> {
                    <span class="k">return</span> Vec::new();
                }
                <span class="k">let</span> curve = &amp;brep.m_curves_3d[edge.curve_3d_index <span class="k">as</span> usize];
                <span class="k">let</span> (a, b) = curve.domain();
                (<span class="s">0</span>..=<span class="s">1024</span>)
                    .map(|i| curve.point_at(a + (b - a) * i <span class="k">as</span> f64 / <span class="s">1024</span>.<span class="s">0</span>).to_f32())
                    .collect()
            })
            .collect();
        <span class="k">for</span> &amp;(pipe, ends) <span class="k">in</span> &amp;arena.surface_boundaries {
            <span class="k">let</span> edge = seg.pipe_ids[pipe <span class="k">as</span> usize] <span class="k">as</span> usize;
            <span class="k">for</span> index <span class="k">in</span> ends {
                <span class="k">let</span> p = arena.verts[index <span class="k">as</span> usize].position;
                <span class="k">let</span> distance = curves[edge]
                    .windows(<span class="s">2</span>)
                    .map(|pair| {
                        <span class="k">let</span> d = std::array::from_fn::&lt;_, <span class="s">3</span>, _&gt;(|i| pair[<span class="s">1</span>][i] - pair[<span class="s">0</span>][i]);
                        <span class="k">let</span> q = std::array::from_fn::&lt;_, <span class="s">3</span>, _&gt;(|i| p[i] - pair[<span class="s">0</span>][i]);
                        <span class="k">let</span> length: f32 = d.iter().map(|v| v * v).sum();
                        <span class="k">let</span> t = ((<span class="s">0</span>..<span class="s">3</span>).map(|i| q[i] * d[i]).sum::&lt;f32&gt;() / length.max(<span class="s">1</span>e-<span class="s">20</span>))
                            .clamp(<span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>);
                        (<span class="s">0</span>..<span class="s">3</span>)
                            .map(|i| (q[i] - d[i] * t).powi(<span class="s">2</span>))
                            .sum::&lt;f32&gt;()
                            .sqrt()
                    })
                    .fold(f32::INFINITY, f32::min);
                assert!(
                    distance &lt; <span class="s">0</span>.<span class="s">01</span>,
                    &quot;<span class="s">edge </span>{<span class="s">edge</span>}<span class="s"> endpoint </span>{<span class="s">p:?</span>}<span class="s"> is </span>{<span class="s">distance</span>}<span class="s"> from its authored curve</span>&quot;
                );
            }
            assert_eq!(
                seg.pipes[pipe <span class="k">as</span> usize].p0,
                arena.verts[ends[<span class="s">0</span>] <span class="k">as</span> usize].position
            );
            assert_eq!(
                seg.pipes[pipe <span class="k">as</span> usize].p1,
                arena.verts[ends[<span class="s">1</span>] <span class="k">as</span> usize].position
            );
            assert!((seg.pipe_ids[pipe <span class="k">as</span> usize] <span class="k">as</span> usize) &lt; brep.m_edges.len());
        }
    }

    <span class="c">/// A cylinder uploads unwelded faces with unit normals.</span></code></pre></div>
<h2 id="step-29-srcappwalkcurvesrs">Step 29 · src/app/walk/curves.rs<a class="anchor" href="#/course/32-colors-lighting#step-29-srcappwalkcurvesrs" aria-label="Link to this section">#</a></h2>
<p>Remove consecutive duplicate polyline points before constructing strokes.</p>
<p><code>lessons/32/src/app/walk/curves.rs</code> · edit · type this</p>
<p>Added after the line <code>for w in pts.windows(2) {</code> in <code>lessons/31/src/app/walk/curves.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="c">// skip a repeated point</span>
        <span class="k">if</span> w[<span class="s">0</span>] == w[<span class="s">1</span>] {
            <span class="k">continue</span>;
        }</code></pre></div>
<h2 id="step-30-srcappwalkencoders">Step 30 · src/app/walk/encode.rs<a class="anchor" href="#/course/32-colors-lighting#step-30-srcappwalkencoders" aria-label="Link to this section">#</a></h2>
<p>Encode ordinary 3D pen widths as screen pixels.</p>
<p><code>lessons/32/src/app/walk/encode.rs</code> · edit · type this</p>
<p>Replaces the lines from <code>(w as f32) * 0.5</code> in <code>lessons/31/src/app/walk/encode.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Pen width to a radius: negative = pixels, 0 = default pen.</span>
<span class="k">pub</span> <span class="k">fn</span> encode_width(w: f64) -&gt; f32 {
    <span class="k">if</span> w.is_finite() &amp;&amp; w &gt; <span class="s">0</span>.<span class="s">0</span> &amp;&amp; (w - <span class="s">1</span>.<span class="s">0</span>).abs() &gt; <span class="s">1</span>e-<span class="s">9</span> {
        -(w <span class="k">as</span> f32) * <span class="s">0</span>.<span class="s">5</span></code></pre></div>
<h2 id="step-31-srcenginegpuframers">Step 31 · src/engine/gpu/frame.rs<a class="anchor" href="#/course/32-colors-lighting#step-31-srcenginegpuframers" aria-label="Link to this section">#</a></h2>
<p>Select the soft hemisphere shader when ambient lighting is enabled.</p>
<p><code>lessons/32/src/engine/gpu/frame.rs</code> · edit · type this</p>
<p>Replaces the line <code>lit: f32::from(cx.view.lit),</code> in <code>lessons/31/src/engine/gpu/frame.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            lit: <span class="k">if</span> cx.view.ssao {
                <span class="s">2</span>.<span class="s">0</span>
            } <span class="k">else</span> {
                f32::from(cx.view.lit)
            },</code></pre></div>
<h2 id="step-32-srcenginegpurenderrs">Step 32 · src/engine/gpu/render.rs<a class="anchor" href="#/course/32-colors-lighting#step-32-srcenginegpurenderrs" aria-label="Link to this section">#</a></h2>
<p>Shade faces, then apply ambient contact shadows before drawing ink.</p>
<p><code>lessons/32/src/engine/gpu/render.rs</code> · edit · type this</p>
<p>Added after the line <code>};</code> in <code>lessons/31/src/engine/gpu/render.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="c">// pass 2: ambient occlusion over the faces</span>
        <span class="k">if</span> <span class="k">self</span>.view.ssao &amp;&amp; <span class="k">self</span>.view.opacity &gt; <span class="s">0</span>.<span class="s">0</span> &amp;&amp; <span class="k">self</span>.arena.face_count() &gt; <span class="s">0</span> {
            <span class="k">let</span> target = <span class="k">crate</span>::engine::pipelines::Target {
                format: <span class="k">self</span>.config.format,
                samples: <span class="k">self</span>.targets.samples,
            };
            <span class="k">let</span> ssao = <span class="k">self</span>.ssao.get_or_insert_with(|| {
                super::ssao::Ssao::new(&amp;<span class="k">self</span>.ctx, target, (<span class="k">self</span>.config.width, <span class="k">self</span>.config.height))
            });
            <span class="k">let</span> receiver = ssao.receiver(&amp;<span class="k">self</span>.objects);
            draws += ssao.draw(
                &amp;<span class="k">self</span>.ctx,
                target,
                &amp;<span class="k">self</span>.targets,
                encoder,
                view,
                <span class="k">self</span>.frame.mvp_f32,
                receiver,
                <span class="k">self</span>.objects.geometry_revision(),
            );
        } <span class="k">else</span> {
            <span class="k">self</span>.ssao = None;
        }</code></pre></div>
<p>Replaces the line <code>let mut draws = self.backdrop.draw_background(pass);</code> in <code>lessons/31/src/engine/gpu/render.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> <span class="k">mut</span> draws = <span class="k">self</span>.backdrop.draw_background(pass, b);</code></pre></div>
<h2 id="step-33-srcenginegpussaors">Step 33 · src/engine/gpu/ssao.rs<a class="anchor" href="#/course/32-colors-lighting#step-33-srcenginegpussaors" aria-label="Link to this section">#</a></h2>
<p>Cache hemisphere occlusion in two R8 textures at half resolution, capped at 960 pixels on its longest side.</p>
<p><code>lessons/32/src/engine/gpu/ssao.rs</code> · 459 lines · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> super::{buffers::GpuCtx, targets::Targets};
<span class="k">use</span> <span class="k">crate</span>::engine::pipelines::{ColorWrite, DepthMode, PipelineDesc, Target, build, module};

<span class="c">/// Screen-space ambient occlusion: soft shadows where surfaces meet.</span>
<span class="k">pub</span> <span class="k">struct</span> Ssao {
    target: Target, <span class="c">// scene color format and samples</span>
    layout: wgpu::BindGroupLayout, <span class="c">// depth, uniform, gradient, triangles</span>
    raw: wgpu::RenderPipeline, <span class="c">// computes occlusion per pixel</span>
    composite: wgpu::RenderPipeline, <span class="c">// darkens the scene with it</span>
    filter: wgpu::RenderPipeline, <span class="c">// the blur pass</span>
    filtered: wgpu::Texture, <span class="c">// half-blurred occlusion</span>
    filtered_view: wgpu::TextureView, <span class="c">// view of it</span>
    filtered_group: wgpu::BindGroup, <span class="c">// binds it</span>
    inverse: wgpu::Buffer, <span class="c">// inverse camera, camera, ground, size</span>
    texture: wgpu::Texture, <span class="c">// occlusion per pixel</span>
    view: wgpu::TextureView, <span class="c">// view of it</span>
    sampled: wgpu::BindGroup, <span class="c">// binds it</span>
    size: (u32, u32), <span class="c">// occlusion texture size, px</span>
    cached: Option&lt;([f32; 36], u64)&gt;, <span class="c">// uniform and geometry the occlusion was computed for</span>
    receiver_bounds: Option&lt;(u64, session_rust::AABB)&gt;, <span class="c">// bounds the radius was fitted to</span>
}

<span class="k">impl</span> Ssao {
    <span class="c">/// Ground height and contact radius from the visible solids.</span>
    <span class="k">pub</span> <span class="k">fn</span> receiver(&amp;<span class="k">mut</span> <span class="k">self</span>, objects: &amp;super::objects::InstanceTable) -&gt; [f32; <span class="s">2</span>] {
        <span class="k">let</span> revision = objects.geometry_revision();
        <span class="c">// recompute when the objects changed</span>
        <span class="k">if</span> <span class="k">self</span>
            .receiver_bounds
            .as_ref()
            .is_none_or(|(r, _)| *r != revision)
        {
            <span class="k">let</span> <span class="k">mut</span> bounds = session_rust::AABB::empty();
            <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..objects.len() {
                <span class="k">let</span> flags = objects.row(i).unwrap().flags;
                <span class="k">if</span> flags &amp; super::Instance::FLAG_HAS_FACES != <span class="s">0</span>
                    &amp;&amp; flags &amp; (super::Instance::FLAG_HIDDEN | super::Instance::FLAG_SHEET) == <span class="s">0</span>
                    &amp;&amp; <span class="k">let</span> Some(b) = objects.row_bounds(i)
                {
                    bounds.union_with(&amp;b);
                }
            }
            <span class="k">self</span>.receiver_bounds = Some((revision, bounds));
        }
        <span class="k">let</span> b = &amp;<span class="k">self</span>.receiver_bounds.as_ref().unwrap().<span class="s">1</span>;
        <span class="k">let</span> radius = (<span class="s">2</span>.<span class="s">0</span> * (b.hx * b.hx + b.hy * b.hy + b.hz * b.hz).sqrt() * <span class="s">0</span>.<span class="s">01</span>).max(<span class="s">0</span>.<span class="s">01</span>);
        [(b.cz - b.hz - objects.anchor()[<span class="s">2</span>]) <span class="k">as</span> f32, radius <span class="k">as</span> f32]
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
        <span class="k">let</span> source = include_str!(&quot;<span class="s">../../shaders/ssao.wgsl</span>&quot;).replace(
            &quot;<span class="s">texture_depth_2d</span>&quot;,
            <span class="k">if</span> target.samples &gt; <span class="s">1</span> {
                &quot;<span class="s">texture_depth_multisampled_2d</span>&quot;
            } <span class="k">else</span> {
                &quot;<span class="s">texture_depth_2d</span>&quot;
            },
        );
        <span class="k">let</span> shader = module(&amp;ctx.device, &quot;<span class="s">ambient</span>&quot;, &amp;source);
        <span class="c">// occlusion into a one-channel half-float texture</span>
        <span class="k">let</span> raw = build(
            &amp;ctx.device,
            Target {
                format: wgpu::TextureFormat::R8Unorm, <span class="c">// one byte per pixel</span>
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
        <span class="k">let</span> filter = build(
            &amp;ctx.device,
            Target {
                format: wgpu::TextureFormat::R8Unorm, <span class="c">// one byte per pixel</span>
                samples: <span class="s">1</span>,
            },
            &amp;PipelineDesc::new(
                &amp;shader,
                &amp;[&amp;layout, &amp;sample_layout],
                &amp;[],
                wgpu::PrimitiveTopology::TriangleList,
            )
            .with(&quot;<span class="s">ambient denoise</span>&quot;, &quot;<span class="s">fs_filter</span>&quot;)
            .depth(DepthMode::Detached),
        );
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
        <span class="k">let</span> scale = <span class="s">0</span>.<span class="s">5_f64</span>.min(<span class="s">960</span>.<span class="s">0</span> / f64::from(full.<span class="s">0</span>.max(full.<span class="s">1</span>).max(<span class="s">1</span>)));
        <span class="k">let</span> size = (
            (f64::from(full.<span class="s">0</span>) * scale).ceil().max(<span class="s">1</span>.<span class="s">0</span>) <span class="k">as</span> u32,
            (f64::from(full.<span class="s">1</span>) * scale).ceil().max(<span class="s">1</span>.<span class="s">0</span>) <span class="k">as</span> u32,
        );
        <span class="k">let</span> descriptor = wgpu::TextureDescriptor {
            label: Some(&quot;<span class="s">ambient R8 quarter pixels</span>&quot;), <span class="c">// debug name</span>
            size: wgpu::Extent3d {
                width: size.<span class="s">0</span>,
                height: size.<span class="s">1</span>,
                depth_or_array_layers: <span class="s">1</span>,
            },
            mip_level_count: <span class="s">1</span>,
            sample_count: <span class="s">1</span>,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Unorm, <span class="c">// one byte per pixel</span>
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
        <span class="s">2</span> * u64::from(<span class="k">self</span>.size.<span class="s">0</span>) * u64::from(<span class="k">self</span>.size.<span class="s">1</span>)
    }

    <span class="c">/// Compute occlusion if needed, then darken the scene; returns the draw count.</span>
    #[allow(clippy::too_many_arguments)]
    <span class="k">pub</span> <span class="k">fn</span> draw(
        &amp;<span class="k">mut</span> <span class="k">self</span>,
        ctx: &amp;GpuCtx,
        target: Target,
        targets: &amp;Targets,
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
                .is_some_and(|(key, _)| key[<span class="s">18</span>] != size.<span class="s">0</span> <span class="k">as</span> f32 || key[<span class="s">19</span>] != size.<span class="s">1</span> <span class="k">as</span> f32)
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
            <span class="k">let</span> <span class="k">mut</span> pass = encoder.begin_render_pass(&amp;wgpu::RenderPassDescriptor {
                label: Some(&quot;<span class="s">ambient denoise</span>&quot;),
                color_attachments: &amp;[Some(wgpu::RenderPassColorAttachment {
                    view: &amp;<span class="k">self</span>.filtered_view, <span class="c">// the blurred occlusion</span>
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
            pass.set_pipeline(&amp;<span class="k">self</span>.filter);
            pass.set_bind_group(<span class="s">0</span>, &amp;group, &amp;[]);
            pass.set_bind_group(<span class="s">1</span>, &amp;<span class="k">self</span>.sampled, &amp;[]);
            pass.draw(<span class="s">0</span>..<span class="s">3</span>, <span class="s">0</span>..<span class="s">1</span>);
            <span class="k">self</span>.cached = Some((uniform, revision));
            draws += <span class="s">2</span>;
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
        pass.set_bind_group(<span class="s">1</span>, &amp;<span class="k">self</span>.filtered_group, &amp;[]);
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
        <span class="k">let</span> input = FrameInput {
            view_proj: camera.view_proj_anchored(<span class="s">1</span>.<span class="s">0</span>, &amp;rebase.anchor),
            clear: wgpu::Color::WHITE,
            now_ms: <span class="s">0</span>.<span class="s">0</span>,
        };
        <span class="k">for</span> samples <span class="k">in</span> [<span class="s">1</span>, <span class="s">4</span>] {
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
                (memory.<span class="s">0</span> + <span class="s">144</span>, memory.<span class="s">1</span> + <span class="s">2</span> * <span class="s">128</span> * <span class="s">128</span>)
            );
            gpu.view.ssao = <span class="s">false</span>;
            assert_eq!(gpu.render_offscreen(&amp;input), plain);
            assert_eq!(gpu.allocated_bytes(), memory);
        }
    }

    #[test]
    <span class="c">/// The shader compiles at 1x and 4x.</span>
    <span class="k">fn</span> shader_validates_for_both_depth_sample_counts() {
        <span class="k">for</span> texture <span class="k">in</span> [&quot;<span class="s">texture_depth_2d</span>&quot;, &quot;<span class="s">texture_depth_multisampled_2d</span>&quot;] {
            <span class="k">let</span> source =
                include_str!(&quot;<span class="s">../../shaders/ssao.wgsl</span>&quot;).replace(&quot;<span class="s">texture_depth_2d</span>&quot;, texture);
            <span class="k">let</span> module = naga::front::wgsl::parse_str(&amp;source).unwrap();
            naga::valid::Validator::new(
                naga::valid::ValidationFlags::all(),
                naga::valid::Capabilities::all(),
            )
            .validate(&amp;module)
            .unwrap();
        }
    }
}</code></pre></div>
<h2 id="step-34-srcenginegpuviewrs">Step 34 · src/engine/gpu/view.rs<a class="anchor" href="#/course/32-colors-lighting#step-34-srcenginegpuviewrs" aria-label="Link to this section">#</a></h2>
<p>Start with ambient lighting disabled.</p>
<p><code>lessons/32/src/engine/gpu/view.rs</code> · edit · type this</p>
<p>Added after the line <code>pub struct View {</code> in <code>lessons/31/src/engine/gpu/view.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> ssao: bool,</code></pre></div>
<p>Added after the line <code>Self {</code> in <code>lessons/31/src/engine/gpu/view.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            ssao: <span class="s">false</span>,</code></pre></div>
<h2 id="step-35-srcshadersbackgroundwgsl">Step 35 · src/shaders/background.wgsl<a class="anchor" href="#/course/32-colors-lighting#step-35-srcshadersbackgroundwgsl" aria-label="Link to this section">#</a></h2>
<p>Use a white background normally and very light grey with Arctic enabled.</p>
<p><code>lessons/32/src/shaders/background.wgsl</code> · edit · type this</p>
<p>Replaces the line <code>return PhysicalColor(vec4&lt;f32&gt;(1.0, 1.0, 1.0, 1.0), vec4&lt;…</code> in <code>lessons/31/src/shaders/background.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">let</span> value = select(<span class="s">1.0</span>,<span class="s">0.94</span>,line.lit &gt; <span class="s">1.5</span>);
    <span class="k">return</span> PhysicalColor(<span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(<span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(value),<span class="s">1.0</span>), <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>));</code></pre></div>
<h2 id="step-36-srcshadersssaowgsl">Step 36 · src/shaders/ssao.wgsl<a class="anchor" href="#/course/32-colors-lighting#step-36-srcshadersssaowgsl" aria-label="Link to this section">#</a></h2>
<p>Reconstruct positions from depth and sample surface occlusion with slightly stronger shading.</p>
<p><code>lessons/32/src/shaders/ssao.wgsl</code> · 176 lines · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// Uniform: camera matrices, ground height, radius, canvas size.</span>
<span class="k">struct</span> Ambient {
    inverse_mvp: <span class="k">mat4x4</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // clip space back to world</span>
    mvp: <span class="k">mat4x4</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // camera matrix</span>
    params: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;,<span class="c"> // ground z, largest contact radius, canvas width, height</span>
};
@group(<span class="s">0</span>) @binding(<span class="s">0</span>) <span class="k">var</span> depth: texture_depth_2d;<span class="c"> // scene depth</span>
@group(<span class="s">0</span>) @binding(<span class="s">1</span>) <span class="k">var</span>&lt;<span class="k">uniform</span>&gt; ambient: Ambient;<span class="c"> // settings</span>
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
<span class="c">// depth at a pixel</span>
<span class="k">fn</span> depth_at(pixel: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;) -&gt; <span class="k">f32</span> {
    <span class="k">let</span> xy = clamp(<span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;(pixel),<span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;(<span class="s">0</span>),<span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;(ambient.params.zw)-<span class="s">1</span>);
    <span class="k">return</span> textureLoad(depth, xy, <span class="s">0</span>);
}
<span class="c">// A virtual receiver beneath the model: no giant quad, depth writes or pick IDs.</span>
<span class="k">fn</span> surface(pixel: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;) -&gt; <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt; {
    <span class="k">let</span> z = depth_at(pixel);
    <span class="k">if</span> z &gt; <span class="s">0.0</span> { <span class="k">return</span> <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(world(pixel,z),<span class="s">1.0</span>); }
    <span class="k">let</span> near = world(pixel,<span class="s">1.0</span>);
    <span class="k">let</span> direction = world(pixel,<span class="s">0.5</span>)-near;
    <span class="k">if</span> direction.z &gt;= -1e-<span class="s">7</span> || near.z &lt; ambient.params.x { <span class="k">return</span> <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>); }
    <span class="k">let</span> t = (ambient.params.x-near.z)/direction.z;
    <span class="k">return</span> <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(near+direction*t,<span class="s">1.0</span>);
}
<span class="c">// surface normal at a pixel from its neighbours</span>
<span class="k">fn</span> normal_at(pixel: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;, p: <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;) -&gt; <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt; {
    <span class="k">if</span> depth_at(pixel) == <span class="s">0.0</span> { <span class="k">return</span> <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>,<span class="s">0.0</span>,<span class="s">1.0</span>); }
    <span class="k">let</span> a = surface(pixel+<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">1.0</span>,<span class="s">0.0</span>));
    <span class="k">let</span> b = surface(pixel-<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">1.0</span>,<span class="s">0.0</span>));
    <span class="k">let</span> c = surface(pixel+<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>,<span class="s">1.0</span>));
    <span class="k">let</span> d = surface(pixel-<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>,<span class="s">1.0</span>));
<span class="c">    // nearest neighbour per axis, so edges do not mix surfaces</span>
    <span class="k">let</span> dx = select(p-b.xyz,a.xyz-p,a.w&gt;<span class="s">0.0</span> &amp;&amp; (b.w==<span class="s">0.0</span> || distance(a.xyz,p)&lt;distance(b.xyz,p)));
    <span class="k">let</span> dy = select(p-d.xyz,c.xyz-p,c.w&gt;<span class="s">0.0</span> &amp;&amp; (d.w==<span class="s">0.0</span> || distance(c.xyz,p)&lt;distance(d.xyz,p)));
    <span class="k">let</span> cross_n = cross(dx,dy);
    <span class="k">let</span> n = cross_n / max(length(cross_n),1e-<span class="s">12</span>);
    <span class="k">return</span> select(-n,n,dot(n,world(pixel,<span class="s">1.0</span>)-p)&gt;<span class="s">0.0</span>);
}
<span class="c">// Occlusion texture size: the canvas, at most 1920 px wide.</span>
<span class="k">fn</span> ao_size() -&gt; <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt; {
    <span class="k">let</span> scale = min(<span class="s">0.5</span>,<span class="s">960.0</span>/max(ambient.params.z,ambient.params.w));
    <span class="k">return</span> ceil(ambient.params.zw*scale);
}
<span class="c">// Shadow on the virtual ground from geometry just above it, 0..0.65.</span>
<span class="k">fn</span> ground_contact(at: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;, p: <span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;, noise: <span class="k">f32</span>) -&gt; <span class="k">f32</span> {
    <span class="k">let</span> clip = ambient.mvp*<span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(p,<span class="s">1.0</span>);
    <span class="k">let</span> pixel_world = distance(world(at+<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">1.0</span>,<span class="s">0.0</span>),clip.z/clip.w),p);
    <span class="k">let</span> radius = ambient.params.y*<span class="s">8.0</span>;
    <span class="k">let</span> radius_px = clamp(radius/max(pixel_world,1e-<span class="s">6</span>),<span class="s">4.0</span>,<span class="s">180.0</span>);
    <span class="k">var</span> sum = <span class="s">0.0</span>;
    <span class="k">for</span> (<span class="k">var</span> direction=<span class="s">0u</span>; direction&lt;<span class="s">8u</span>; direction++) {
        <span class="k">let</span> angle = (f32(direction)+noise)*<span class="s">0.78539816</span>;
        <span class="k">let</span> axis = <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(cos(angle),sin(angle));
        <span class="k">let</span> jitter = fract(noise+f32(direction)*<span class="s">0.618034</span>);
        <span class="k">var</span> horizon = <span class="s">0.0</span>;
        <span class="k">for</span> (<span class="k">var</span> step=<span class="s">0u</span>; step&lt;<span class="s">4u</span>; step++) {
            <span class="k">let</span> fraction = (f32(step)+<span class="s">0.5</span>+jitter*<span class="s">0.5</span>)/<span class="s">4.0</span>;
            <span class="k">let</span> qxy = floor(at+axis*max(<span class="s">1.5</span>,radius_px*fraction*fraction))+<span class="s">0.5</span>;
            <span class="k">if</span> any(qxy&lt;<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>)) || any(qxy&gt;=ambient.params.zw) { <span class="k">continue</span>; }
            <span class="k">let</span> z = depth_at(qxy);
            <span class="k">if</span> z==<span class="s">0.0</span> { <span class="k">continue</span>; }
            <span class="k">let</span> delta = world(qxy,z)-p;
            <span class="k">let</span> span = length(delta);
            <span class="k">let</span> height = max(delta.z,<span class="s">0.0</span>);
            <span class="k">let</span> elevation = max(height/max(span,1e-<span class="s">6</span>)-<span class="s">0.04</span>,<span class="s">0.0</span>);
            <span class="k">let</span> radial = <span class="s">1.0</span>-smoothstep(radius*<span class="s">0.1</span>,radius,length(delta.xy));
            <span class="k">let</span> weight = radial*exp(-height/(radius*<span class="s">0.5</span>));
            horizon = max(horizon,elevation*weight);
        }
        sum += horizon;
    }
    <span class="k">return</span> clamp(sum/<span class="s">8.0</span>*<span class="s">2.6</span>,<span class="s">0.0</span>,<span class="s">0.65</span>);
}
<span class="c">// Occlusion at one pixel: ground shadow, or hemisphere samples on geometry.</span>
@fragment <span class="k">fn</span> fs_main(@builtin(position) pixel: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;) -&gt; @location(<span class="s">0</span>) <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt; {
    <span class="k">let</span> at = floor(pixel.xy/ao_size()*ambient.params.zw)+<span class="s">0.5</span>;
    <span class="k">let</span> center = surface(at);
    <span class="k">if</span> center.w == <span class="s">0.0</span> { <span class="k">return</span> <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>); }
    <span class="k">let</span> p = center.xyz;
    <span class="k">let</span> n = normal_at(at,p);
    <span class="k">let</span> ground = depth_at(at)==<span class="s">0.0</span>;
<span class="c">    // A 4x4 tiled rotation is removed by the small bilateral filter. Unlike</span>
    <span class="k">let</span> tile = <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="k">vec2</span>&lt;<span class="k">u32</span>&gt;(pixel.xy) &amp; <span class="k">vec2</span>&lt;<span class="k">u32</span>&gt;(<span class="s">3u</span>));
    <span class="k">let</span> noise = fract(<span class="s">52.9829189</span>*fract(dot(tile,<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.06711056</span>,<span class="s">0.00583715</span>))));
    <span class="k">if</span> ground { <span class="k">return</span> <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(ground_contact(at,p,noise),<span class="s">0.0</span>,<span class="s">0.0</span>,<span class="s">1.0</span>); }
<span class="c">    // tangent frame around the normal</span>
    <span class="k">let</span> up = select(<span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>,<span class="s">1.0</span>,<span class="s">0.0</span>),<span class="k">vec3</span>&lt;<span class="k">f32</span>&gt;(<span class="s">1.0</span>,<span class="s">0.0</span>,<span class="s">0.0</span>),abs(n.y)&gt;<span class="s">0.9</span>);
    <span class="k">let</span> tangent = normalize(cross(up,n));
    <span class="k">let</span> bitangent = cross(n,tangent);
    <span class="k">let</span> view = normalize(world(at,<span class="s">1.0</span>)-p);
    <span class="k">let</span> bias = ambient.params.y*<span class="s">0.025</span>*(<span class="s">1.0</span>+<span class="s">3.0</span>*(<span class="s">1.0</span>-max(dot(n,view),<span class="s">0.0</span>)));
    <span class="k">var</span> near_occ = <span class="s">0.0</span>;
    <span class="k">var</span> far_occ = <span class="s">0.0</span>;
<span class="c">    // Sixteen local and sixteen broad sky samples, all at half resolution.</span>
    <span class="k">for</span> (<span class="k">var</span> i=<span class="s">0u</span>; i&lt;<span class="s">32u</span>; i++) {
        <span class="k">let</span> j = f32(i%<span class="s">16u</span>);
        <span class="k">let</span> far = i&gt;=<span class="s">16u</span>;
        <span class="k">let</span> radius = ambient.params.y*select(<span class="s">1.0</span>,<span class="s">8.0</span>,far);
        <span class="k">let</span> angle = j*<span class="s">2.39996323</span>+noise*<span class="s">6.2831853</span>;
        <span class="k">let</span> z = <span class="s">0.2</span>+<span class="s">0.8</span>*(j+<span class="s">0.5</span>)/<span class="s">16.0</span>;
        <span class="k">let</span> xy = sqrt(<span class="s">1.0</span>-z*z);
        <span class="k">let</span> direction = tangent*(cos(angle)*xy)+bitangent*(sin(angle)*xy)+n*z;
<span class="c">        // Permute sample lengths so elevations and distances are uncorrelated.</span>
        <span class="k">let</span> fraction = (f32((i*<span class="s">7u</span>)%<span class="s">16u</span>)+<span class="s">0.5</span>)/<span class="s">16.0</span>;
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
    <span class="k">return</span> <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(clamp(max(near_occ/<span class="s">16.0</span>,far_occ/<span class="s">16.0</span>*<span class="s">0.75</span>)*<span class="s">1.08</span>,<span class="s">0.0</span>,<span class="s">0.65</span>),<span class="s">0.0</span>,<span class="s">0.0</span>,<span class="s">1.0</span>);
}
<span class="c">// Denoise at the small resolution so a broad, soft kernel stays cheap.</span>
@fragment <span class="k">fn</span> fs_filter(@builtin(position) pixel: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;) -&gt; @location(<span class="s">0</span>) <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt; {
    <span class="k">let</span> at = floor(pixel.xy/ao_size()*ambient.params.zw)+<span class="s">0.5</span>;
    <span class="k">return</span> <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(reconstruct(at,<span class="s">3</span>),<span class="s">0.0</span>,<span class="s">0.0</span>,<span class="s">1.0</span>);
}
<span class="c">// Darken the scene: black with the occlusion as alpha.</span>
@fragment <span class="k">fn</span> fs_composite(@builtin(position) pixel: <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;) -&gt; @location(<span class="s">0</span>) <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt; {
    <span class="k">return</span> <span class="k">vec4</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>,<span class="s">0.0</span>,<span class="s">0.0</span>,reconstruct(pixel.xy,<span class="s">0</span>));
}
<span class="c">// occlusion at a pixel from nearby depths</span>
<span class="k">fn</span> reconstruct(pixel: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;, radius: <span class="k">i32</span>) -&gt; <span class="k">f32</span> {
    <span class="k">let</span> center = surface(pixel);
    <span class="k">if</span> center.w == <span class="s">0.0</span> { <span class="k">return</span> <span class="s">0.0</span>; }
    <span class="k">let</span> n = normal_at(pixel,center.xyz);
    <span class="k">let</span> geometry = depth_at(pixel)&gt;<span class="s">0.0</span>;
    <span class="k">let</span> dims = <span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;(textureDimensions(occlusion));
    <span class="k">let</span> source = pixel/ambient.params.zw*<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(dims)-<span class="s">0.5</span>;
    <span class="k">let</span> base = <span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;(floor(source));
    <span class="k">var</span> sum = <span class="s">0.0</span>;
    <span class="k">var</span> weight = <span class="s">0.0</span>;
<span class="c">    // Four-tap depth-aware upsample, or a 7x7 Gaussian for the small denoise pass.</span>
    <span class="k">let</span> lo = select(<span class="s">0</span>,-<span class="s">3</span>,radius&gt;<span class="s">0</span>);
    <span class="k">let</span> hi = select(<span class="s">1</span>,<span class="s">3</span>,radius&gt;<span class="s">0</span>);
    <span class="k">for</span> (<span class="k">var</span> y=lo; y&lt;=hi; y++) {
        <span class="k">for</span> (<span class="k">var</span> x=lo; x&lt;=hi; x++) {
            <span class="k">let</span> q = clamp(base+<span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;(x,y),<span class="k">vec2</span>&lt;<span class="k">i32</span>&gt;(<span class="s">0</span>),dims-<span class="s">1</span>);
            <span class="k">let</span> full = floor((<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(q)+<span class="s">0.5</span>)/<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(dims)*ambient.params.zw)+<span class="s">0.5</span>;
            <span class="k">let</span> sample = surface(full);
            <span class="k">let</span> delta = sample.xyz-center.xyz;
            <span class="k">let</span> separation = abs(dot(delta,n));
            <span class="k">let</span> tolerance = max(ambient.params.y*<span class="s">0.024</span>,length(delta)*<span class="s">0.08</span>);
            <span class="k">let</span> offset = <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(q)-source;
            <span class="k">let</span> tent = max(<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">0.0</span>),<span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;(<span class="s">1.0</span>)-abs(offset));
            <span class="k">let</span> spatial = select(tent.x*tent.y,exp(-dot(offset,offset)*<span class="s">0.16</span>),radius&gt;<span class="s">0</span>);
            <span class="k">let</span> w = spatial*exp(-separation/max(tolerance,1e-<span class="s">6</span>));
            <span class="k">let</span> valid = select(<span class="s">0.0</span>,w,sample.w&gt;<span class="s">0.0</span> &amp;&amp; ((depth_at(full)&gt;<span class="s">0.0</span>) == geometry));
            sum += textureLoad(occlusion,q,<span class="s">0</span>).r*valid;
            weight += valid;
        }
    }
    <span class="k">return</span> sum/max(weight,1e-<span class="s">6</span>);
}</code></pre></div>
<h2 id="step-37-srcshaderstrianglewgsl">Step 37 · src/shaders/triangle.wgsl<a class="anchor" href="#/course/32-colors-lighting#step-37-srcshaderstrianglewgsl" aria-label="Link to this section">#</a></h2>
<p>Apply the archive viewer’s soft sky and ground shading while preserving authored colors.</p>
<p><code>lessons/32/src/shaders/triangle.wgsl</code> · edit · type this</p>
<p>Replaces the line <code>let shaded = select(1.0, lit, line.lit &gt; 0.5 &amp;&amp; in.print …</code> in <code>lessons/31/src/shaders/triangle.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">    // SSAO mode: soft sky light from above</span>
    <span class="k">let</span> ambient = mix(<span class="s">0.72</span>, <span class="s">1.0</span>, <span class="s">0.5</span> + <span class="s">0.5</span> * n.z);
    <span class="k">let</span> lighting = select(lit, ambient, line.lit &gt; <span class="s">1.5</span>);
    <span class="k">let</span> shaded = select(<span class="s">1.0</span>, lighting, line.lit &gt; <span class="s">0.5</span> &amp;&amp; in.print &lt;= <span class="s">0.5</span>);</code></pre></div>
<h2 id="step-38-srcstaters">Step 38 · src/state.rs<a class="anchor" href="#/course/32-colors-lighting#step-38-srcstaters" aria-label="Link to this section">#</a></h2>
<p>Retain the selected object set and place one gumball around its combined bounds.</p>
<p><code>lessons/32/src/state.rs</code> · edit · type this</p>
<p>Added after the line <code>mod cloud_query;</code> in <code>lessons/31/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">mod</span> drawing;</code></pre></div>
<p>Added after the line <code>pending_split: Option&lt;splitting::Pending&gt;,</code> in <code>lessons/31/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span>(<span class="k">crate</span>) draft: Option&lt;drawing::Draft&gt;,               <span class="c">// a shape being drawn</span>
    <span class="k">pub</span>(<span class="k">crate</span>) snap_enabled: bool,
    controls: Controls,                                     <span class="c">// control points of the selected object</span>
    requested: PickMode,                                    <span class="c">// what the pending pick looks for</span>
    <span class="k">pub</span>(<span class="k">crate</span>) additive_selection: bool,                    <span class="c">// Shift held: add to the selection</span></code></pre></div>
<p>Added after the line <code>pending_split: None,</code> in <code>lessons/31/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            draft: None,
            snap_enabled: <span class="s">true</span>,
            controls: Controls::default(),
            requested: PickMode::Object,
            additive_selection: <span class="s">false</span>,</code></pre></div>
<p>Replaces the lines from <code>let row = match self.scene.selected {</code> in <code>lessons/31/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Fit the camera to the selection, or to everything.</span>
    <span class="k">pub</span> <span class="k">fn</span> fit_selected_or_all(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="c">// the box of every selected row</span>
        <span class="k">let</span> <span class="k">mut</span> bounds = <span class="k">self</span>
            .selected_rows()
            .into_iter()
            .filter_map(|row| <span class="k">self</span>.gpu.objects.row_bounds(row));
        <span class="k">let</span> Some(<span class="k">mut</span> b) = bounds.next() <span class="k">else</span> {
            <span class="k">self</span>.fit_all();
            <span class="k">return</span>;
        };
        <span class="k">for</span> next <span class="k">in</span> bounds {
            b.union_with(&amp;next);
        }</code></pre></div>
<p>Added after the line <code>self.scene.selected = row;</code> in <code>lessons/31/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.refresh_layers();
        <span class="k">self</span>.place_gizmo(row);
        <span class="k">self</span>.update_label();
        <span class="k">self</span>.touch();
    }

    <span class="c">/// Every selected row.</span>
    <span class="k">pub</span>(<span class="k">crate</span>) <span class="k">fn</span> selected_rows(&amp;<span class="k">self</span>) -&gt; Vec&lt;u32&gt; {
        <span class="k">if</span> <span class="k">self</span>.hierarchy.selected.is_empty() {
            <span class="k">self</span>.scene.selected.into_iter().collect()
        } <span class="k">else</span> {
            <span class="k">self</span>.hierarchy.selected.clone()
        }
    }

    <span class="c">/// Select several rows, added to the selection or replacing it.</span>
    <span class="k">pub</span>(<span class="k">crate</span>) <span class="k">fn</span> select_rows(&amp;<span class="k">mut</span> <span class="k">self</span>, rows: Vec&lt;u32&gt;, additive: bool) {
        <span class="k">let</span> <span class="k">mut</span> selected = <span class="k">if</span> additive {
            <span class="k">self</span>.selected_rows()
        } <span class="k">else</span> {
            Vec::new()
        };
        <span class="c">// skip rows that cannot be selected or are hidden</span>
        selected.extend(rows.into_iter().filter(|r| {
            <span class="k">self</span>.scene.selectable(*r)
                &amp;&amp; <span class="k">self</span>
                    .scene
                    .identity_of(*r)
                    .is_some_and(|id| !<span class="k">self</span>.scene.hidden.contains(&amp;id))
        }));
        selected.sort_unstable();
        selected.dedup();
        <span class="k">self</span>.select(None);
        <span class="k">self</span>.scene.selected = selected.first().copied(); <span class="c">// the first is the main one</span>
        <span class="k">for</span> &amp;row <span class="k">in</span> &amp;selected {
            <span class="k">self</span>.gpu.set_selected(row, <span class="s">true</span>);
        }
        <span class="k">if</span> selected.len() &gt; <span class="s">1</span> {
            <span class="k">self</span>.hierarchy.selected = selected;
        }
        <span class="k">self</span>.place_gizmo(<span class="k">self</span>.scene.selected);
        <span class="k">self</span>.refresh_layers();
        <span class="k">self</span>.update_label();
        <span class="k">self</span>.touch();
    }

    <span class="c">/// T: show or hide the name label on the selection.</span></code></pre></div>
<p>Added after the line <code>log::info!(&quot;pick: nothing&quot;);</code> in <code>lessons/31/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">if</span> !<span class="k">self</span>.additive_selection {
                <span class="k">self</span>.select(None);
            }</code></pre></div>
<p>Replaces the 6 lines from <code>let toggle = if self.scene.selected == Some(hit.row) {</code> in <code>lessons/31/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                <span class="k">self</span>.select_rows(vec![hit.row], <span class="k">self</span>.additive_selection);</code></pre></div>
<h2 id="step-39-srcenginegpubackdroprs">Step 39 · src/engine/gpu/backdrop.rs<a class="anchor" href="#/course/32-colors-lighting#step-39-srcenginegpubackdroprs" aria-label="Link to this section">#</a></h2>
<p>Bind the shared lighting uniform before drawing the background.</p>
<p><code>lessons/32/src/engine/gpu/backdrop.rs</code> · edit · type this</p>
<p>Replaces the 3 lines from <code>use crate::engine::pipelines::{</code> in <code>lessons/31/src/engine/gpu/backdrop.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> <span class="k">crate</span>::engine::pipelines::{DepthMode, Layouts, PipelineDesc, Target, build, scene_module};</code></pre></div>
<p>Replaces the line <code>let background_shader = module(</code> in <code>lessons/31/src/engine/gpu/backdrop.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> background_shader = scene_module(</code></pre></div>
<p>Replaces the line <code>let background = build_background(ctx, &amp;background_shader…</code> in <code>lessons/31/src/engine/gpu/backdrop.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> background = build_background(ctx, l, &amp;background_shader, target);</code></pre></div>
<p>Replaces the line <code>self.background = build_background(ctx, &amp;self.background_…</code> in <code>lessons/31/src/engine/gpu/backdrop.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.background = build_background(ctx, l, &amp;<span class="k">self</span>.background_shader, target);
        <span class="k">self</span>.grid = build_grid(ctx, l, &amp;<span class="k">self</span>.grid_shader, target);
    }

    <span class="c">/// Draw the background as one fullscreen triangle.</span>
    <span class="k">pub</span> <span class="k">fn</span> draw_background(&amp;<span class="k">self</span>, pass: &amp;<span class="k">mut</span> wgpu::RenderPass&lt;'_&gt;, b: &amp;Binds) -&gt; u32 {
        pass.set_pipeline(&amp;<span class="k">self</span>.background);
        pass.set_bind_group(<span class="s">0</span>, b.mvp, &amp;[]);
        pass.set_bind_group(<span class="s">1</span>, b.line, &amp;[]);
        <span class="c">// three vertices, the shader places them</span></code></pre></div>
<p>Added after the line <code>ctx: &amp;GpuCtx,</code> in <code>lessons/31/src/engine/gpu/backdrop.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    l: &amp;Layouts,
    shader: &amp;wgpu::ShaderModule,
    target: Target,
) -&gt; wgpu::RenderPipeline {
    <span class="k">let</span> groups = [&amp;l.mvp, &amp;l.line];
    <span class="k">let</span> base = PipelineDesc::new(shader, &amp;groups, &amp;[], TriangleList);</code></pre></div>
<h2 id="step-40-srcstatedrawingrs">Step 40 · src/state/drawing.rs<a class="anchor" href="#/course/32-colors-lighting#step-40-srcstatedrawingrs" aria-label="Link to this section">#</a></h2>
<p>Collect clicked or typed points, preview the next span and snap to existing geometry.</p>
<p><code>lessons/32/src/state/drawing.rs</code> · 289 lines · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> super::State;
<span class="k">use</span> <span class="k">crate</span>::app::{
    coords,
    cplane::CPlane,
    selection::Controls,
    snap::{<span class="k">self</span>, Snap, SnapKind},
};
<span class="k">use</span> session_rust::{Geometry, Point, Vector};

<span class="c">/// A shape being drawn, not yet in the scene.</span>
<span class="k">pub</span>(<span class="k">crate</span>) <span class="k">struct</span> Draft {
    verb: String,               <span class="c">// point, line, polyline or curve</span>
    points: Vec&lt;Point&gt;,         <span class="c">// the points placed so far</span>
    plane: CPlane,              <span class="c">// the plane clicks land on</span>
    candidates: Vec&lt;Snap&gt;,      <span class="c">// scene points the cursor can snap to</span>
    hover: Option&lt;Point&gt;,       <span class="c">// where the cursor is now, in the scene</span>
    snapped: Option&lt;SnapKind&gt;,  <span class="c">// what the cursor snapped to</span>
}

<span class="k">impl</span> State {
    <span class="c">/// A command line entry while drawing; \`None\` if not one.</span>
    <span class="k">pub</span>(super) <span class="k">fn</span> drawing_command(&amp;<span class="k">mut</span> <span class="k">self</span>, text: &amp;str) -&gt; Option&lt;Result&lt;String, String&gt;&gt; {
        <span class="k">let</span> words: Vec&lt;_&gt; = text.split_whitespace().collect();
        <span class="k">let</span> verb = words.first().copied().unwrap_or(&quot;&quot;).to_ascii_lowercase();
        <span class="c">// a drawing verb with too few points starts a draft</span>
        <span class="k">if</span> matches!(verb.as_str(), &quot;<span class="s">point</span>&quot; | &quot;<span class="s">line</span>&quot; | &quot;<span class="s">polyline</span>&quot; | &quot;<span class="s">curve</span>&quot;)
            &amp;&amp; words.len() &lt; <span class="k">if</span> verb == &quot;<span class="s">point</span>&quot; { <span class="s">2</span> } <span class="k">else</span> { <span class="s">3</span> }
        {
            <span class="k">if</span> !<span class="k">self</span>.scene.streamed.is_empty() || !<span class="k">self</span>.scene.sheets.is_empty() {
                <span class="k">return</span> Some(Err(
                    &quot;<span class="s">geometry edits require a scene without streamed sources</span>&quot;.into(),
                ));
            }
            <span class="k">self</span>.cancel_split();
            <span class="c">// draw on the plane the camera faces most</span>
            <span class="k">let</span> plane = CPlane::facing(&amp;<span class="k">self</span>.camera.orientation.rotate_vector(Vector::y_axis()));
            <span class="k">let</span> candidates = <span class="k">self</span>.drawing_candidates();
            <span class="k">self</span>.draft = Some(Draft {
                verb,
                points: Vec::new(),
                plane,
                candidates,
                hover: None,
                snapped: None,
            });
            <span class="k">self</span>.gpu.pick.cancel();
            <span class="k">return</span> Some(<span class="k">if</span> words.len() &gt; <span class="s">1</span> {
                <span class="k">self</span>.accept_coordinates(&amp;words[<span class="s">1</span>..].join(&quot;<span class="s"> </span>&quot;))
            } <span class="k">else</span> {
                Ok(<span class="k">self</span>.drawing_prompt())
            });
        }
        <span class="k">self</span>.draft.as_ref()?; <span class="c">// not drawing: not ours</span>
        <span class="c">// Enter alone finishes</span>
        <span class="k">if</span> text.trim().is_empty() {
            <span class="k">return</span> Some(<span class="k">self</span>.finish_drawing());
        }
        <span class="c">// a coordinate adds a point</span>
        <span class="k">if</span> coords::parse(words.first().copied().unwrap_or(&quot;&quot;)).is_some() {
            <span class="k">return</span> Some(<span class="k">self</span>.accept_coordinates(text));
        }
        None
    }

    <span class="c">/// Every scene point the cursor can snap to.</span>
    <span class="k">fn</span> drawing_candidates(&amp;<span class="k">self</span>) -&gt; Vec&lt;Snap&gt; {
        <span class="k">let</span> <span class="k">mut</span> out = Vec::new();
        <span class="k">for</span> row <span class="k">in</span> <span class="s">0</span>..<span class="k">self</span>.gpu.objects.len() {
            <span class="k">if</span> !<span class="k">self</span>.scene.selectable(row) {
                <span class="k">continue</span>;
            }
            <span class="k">let</span> (Some(geometry), Some(place)) =
                (<span class="k">self</span>.scene.geometry(row), <span class="k">self</span>.scene.placement_of(row))
            <span class="k">else</span> {
                <span class="k">continue</span>;
            };
            <span class="k">match</span> geometry {
                Geometry::Point(p) =&gt; out.push(Snap {
                    point: p.transformed(&amp;place),
                    kind: SnapKind::End,
                    owner: row,
                }),
                Geometry::Line(line) =&gt; snap::from_polyline(
                    &amp;[
                        line.start().transformed(&amp;place),
                        line.end().transformed(&amp;place),
                    ],
                    <span class="s">false</span>,
                    row,
                    &amp;<span class="k">mut</span> out,
                ),
                Geometry::Polyline(line) =&gt; {
                    <span class="k">let</span> points: Vec&lt;_&gt; = line
                        .get_points()
                        .iter()
                        .map(|p| p.transformed(&amp;place))
                        .collect();
                    snap::from_polyline(&amp;points, <span class="s">false</span>, row, &amp;<span class="k">mut</span> out);
                }
                Geometry::NurbsCurve(curve) =&gt; {
                    <span class="k">let</span> (a, b) = curve.domain();
                    <span class="k">for</span> t <span class="k">in</span> [a, b] {
                        out.push(Snap {
                            point: curve.point_at(t).transformed(&amp;place),
                            kind: SnapKind::End,
                            owner: row,
                        });
                    }
                }
                Geometry::Mesh(_) | Geometry::BRep(_) | Geometry::NurbsSurface(_) =&gt; {
                    <span class="k">let</span> controls = Controls::from_geometry(geometry);
                    out.extend(controls.points.iter().map(|p| {
                        Snap {
                            point: Point::new(p.position[<span class="s">0</span>], p.position[<span class="s">1</span>], p.position[<span class="s">2</span>])
                                .transformed(&amp;place),
                            kind: SnapKind::Vertex,
                            owner: row,
                        }
                    }));
                }
                _ =&gt; {}
            }
        }
        out
    }

    <span class="c">/// Add typed coordinates to the draft.</span>
    <span class="k">fn</span> accept_coordinates(&amp;<span class="k">mut</span> <span class="k">self</span>, text: &amp;str) -&gt; Result&lt;String, String&gt; {
        <span class="c">// check every word before adding any</span>
        <span class="k">let</span> draft = <span class="k">self</span>.draft.as_ref().unwrap();
        <span class="k">let</span> <span class="k">mut</span> points = draft.points.clone();
        <span class="k">let</span> (x, y) = axes(draft.plane);
        <span class="k">for</span> word <span class="k">in</span> text.split_whitespace() {
            <span class="k">let</span> typed = coords::parse(word).ok_or(&quot;<span class="s">Use x,y,z, @dx,dy,dz, or distance&lt;angle</span>&quot;)?;
            <span class="c">// the direction from the last point to the cursor</span>
            <span class="k">let</span> along = points.last().zip(draft.hover.as_ref()).and_then(|(p, h)| {
                <span class="k">let</span> delta = Vector::new(h[<span class="s">0</span>] - p[<span class="s">0</span>], h[<span class="s">1</span>] - p[<span class="s">1</span>], h[<span class="s">2</span>] - p[<span class="s">2</span>]);
                <span class="k">let</span> length =
                    (delta[<span class="s">0</span>] * delta[<span class="s">0</span>] + delta[<span class="s">1</span>] * delta[<span class="s">1</span>] + delta[<span class="s">2</span>] * delta[<span class="s">2</span>]).sqrt();
                (length &gt; <span class="s">1</span>e-<span class="s">12</span>)
                    .then(|| Vector::new(delta[<span class="s">0</span>] / length, delta[<span class="s">1</span>] / length, delta[<span class="s">2</span>] / length))
            });
            <span class="k">let</span> p = coords::resolve(
                typed,
                &amp;Point::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>),
                &amp;x,
                &amp;y,
                points.last(),
                along.as_ref(),
            )
            .ok_or(&quot;<span class="s">This coordinate needs a previous point or a cursor direction</span>&quot;)?;
            <span class="k">if</span> (<span class="s">0</span>..<span class="s">3</span>).any(|i| !p[i].is_finite() || p[i].abs() &gt; <span class="s">1</span>e<span class="s">12</span>) {
                <span class="k">return</span> Err(&quot;<span class="s">Coordinates must be finite and within ±1e12</span>&quot;.into());
            }
            points.push(p);
        }
        <span class="c">// how many points the verb takes</span>
        <span class="k">let</span> limit = <span class="k">match</span> draft.verb.as_str() {
            &quot;<span class="s">point</span>&quot; =&gt; <span class="s">1</span>,
            &quot;<span class="s">line</span>&quot; =&gt; <span class="s">2</span>,
            _ =&gt; <span class="k">crate</span>::app::modeling::MAX_POINTS,
        };
        <span class="k">if</span> points.len() &gt; limit {
            <span class="k">return</span> Err(format!(&quot;{}<span class="s"> accepts at most </span>{<span class="s">limit</span>}<span class="s"> points</span>&quot;, draft.verb));
        }
        <span class="k">let</span> previous = std::mem::replace(&amp;<span class="k">mut</span> <span class="k">self</span>.draft.as_mut().unwrap().points, points);
        <span class="k">let</span> result = <span class="k">self</span>.advance_drawing();
        <span class="c">// a failed finish keeps the old points</span>
        <span class="k">if</span> result.is_err() {
            <span class="k">self</span>.draft.as_mut().unwrap().points = previous;
        }
        result
    }

    <span class="c">/// Finish when the draft has all its points, else prompt for the next.</span>
    <span class="k">fn</span> advance_drawing(&amp;<span class="k">mut</span> <span class="k">self</span>) -&gt; Result&lt;String, String&gt; {
        <span class="k">let</span> draft = <span class="k">self</span>.draft.as_ref().unwrap();
        <span class="k">if</span> (draft.verb == &quot;<span class="s">point</span>&quot; &amp;&amp; draft.points.len() == <span class="s">1</span>)
            || (draft.verb == &quot;<span class="s">line</span>&quot; &amp;&amp; draft.points.len() == <span class="s">2</span>)
        {
            <span class="k">self</span>.finish_drawing()
        } <span class="k">else</span> {
            Ok(<span class="k">self</span>.drawing_prompt())
        }
    }

    <span class="c">/// Turn the draft into a typed command and run it.</span>
    <span class="k">fn</span> finish_drawing(&amp;<span class="k">mut</span> <span class="k">self</span>) -&gt; Result&lt;String, String&gt; {
        <span class="k">let</span> draft = <span class="k">self</span>.draft.take().unwrap();
        <span class="c">// &quot;line 0,0,0 1,1,1&quot;</span>
        <span class="k">let</span> <span class="k">mut</span> command = draft.verb.clone();
        <span class="k">for</span> p <span class="k">in</span> &amp;draft.points {
            command.push_str(&amp;format!(&quot;<span class="s"> </span>{}<span class="s">,</span>{}<span class="s">,</span>{}&quot;, p[<span class="s">0</span>], p[<span class="s">1</span>], p[<span class="s">2</span>]));
        }
        <span class="c">// same path as a typed command: checks, selection, undo</span>
        <span class="k">let</span> result = <span class="k">crate</span>::app::command::parse(&amp;command).and_then(|_| <span class="k">self</span>.run_command(&amp;command));
        <span class="k">if</span> result.is_err() {
            <span class="k">self</span>.draft = Some(draft);
        }
        result
    }

    <span class="c">/// The draft as JSON, for the inspection tests.</span>
    <span class="k">pub</span> <span class="k">fn</span> drawing_status(&amp;<span class="k">self</span>) -&gt; serde_json::Value {
        <span class="k">let</span> Some(draft) = &amp;<span class="k">self</span>.draft <span class="k">else</span> {
            <span class="k">return</span> serde_json::Value::Null;
        };
        serde_json::json!({&quot;<span class="s">command</span>&quot;:draft.verb,&quot;<span class="s">points</span>&quot;:draft.points.iter().map(|p| [p[<span class="s">0</span>],p[<span class="s">1</span>],p[<span class="s">2</span>]]).collect::&lt;Vec&lt;_&gt;&gt;(),&quot;<span class="s">hover</span>&quot;:draft.hover.as_ref().map(|p| [p[<span class="s">0</span>],p[<span class="s">1</span>],p[<span class="s">2</span>]]),&quot;<span class="s">snap</span>&quot;:draft.snapped.map(|k| format!(&quot;{<span class="s">k:?</span>}&quot;))})
    }

    <span class="c">/// The status line text while drawing.</span>
    <span class="k">pub</span> <span class="k">fn</span> drawing_prompt(&amp;<span class="k">self</span>) -&gt; String {
        <span class="k">let</span> Some(draft) = &amp;<span class="k">self</span>.draft <span class="k">else</span> {
            <span class="k">return</span> String::new();
        };
        <span class="k">let</span> point = <span class="k">if</span> draft.points.is_empty() {
            &quot;<span class="s">First point</span>&quot;
        } <span class="k">else</span> {
            &quot;<span class="s">Next point</span>&quot;
        };
        <span class="k">let</span> finish = <span class="k">if</span> matches!(draft.verb.as_str(), &quot;<span class="s">curve</span>&quot; | &quot;<span class="s">polyline</span>&quot;) {
            &quot;<span class="s"> · Enter finishes</span>&quot;
        } <span class="k">else</span> {
            &quot;&quot;
        };
        format!(
            &quot;{}<span class="s">: </span>{<span class="s">point</span>}<span class="s"> · click or type x,y,z · Snap </span>{}{<span class="s">finish</span>}<span class="s"> · Esc cancels</span>&quot;,
            draft.verb,
            <span class="k">if</span> <span class="k">self</span>.snap_enabled { &quot;<span class="s">On</span>&quot; } <span class="k">else</span> { &quot;<span class="s">Off</span>&quot; }
        )
    }

    <span class="c">/// Move the cursor while drawing: snap or land on the plane.</span>
    <span class="k">pub</span> <span class="k">fn</span> hover_drawing(&amp;<span class="k">mut</span> <span class="k">self</span>, x: f64, y: f64) -&gt; bool {
        <span class="k">let</span> Some(draft) = &amp;<span class="k">self</span>.draft <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="c">// the nearest snap point within 12 pixels</span>
        <span class="k">let</span> hit = <span class="k">if</span> <span class="k">self</span>.snap_enabled {
            snap::best(&amp;draft.candidates, (x, y), <span class="s">12</span>.<span class="s">0</span> * <span class="k">self</span>.pixel_scale(), |p| {
                <span class="k">self</span>.project([p[<span class="s">0</span>], p[<span class="s">1</span>], p[<span class="s">2</span>]])
            })
        } <span class="k">else</span> {
            None
        };
        <span class="c">// otherwise, where the cursor ray meets the plane</span>
        <span class="k">let</span> free = <span class="k">self</span>.camera.ray((x, y), <span class="k">self</span>.viewport()).and_then(|(p, d)| {
            draft.plane.hit(
                draft.points.last().unwrap_or(&amp;Point::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>)),
                &amp;p,
                &amp;d,
            )
        });
        <span class="k">let</span> draft = <span class="k">self</span>.draft.as_mut().unwrap();
        draft.snapped = hit.as_ref().map(|s| s.kind);
        draft.hover = hit.map(|s| s.point).or(free);
        <span class="s">true</span>
    }

    <span class="c">/// Click while drawing: place a point.</span>
    <span class="k">pub</span> <span class="k">fn</span> click_drawing(&amp;<span class="k">mut</span> <span class="k">self</span>, x: f64, y: f64) -&gt; bool {
        <span class="k">if</span> !<span class="k">self</span>.hover_drawing(x, y) {
            <span class="k">return</span> <span class="s">false</span>;
        }
        <span class="k">let</span> draft = <span class="k">self</span>.draft.as_mut().unwrap();
        <span class="k">let</span> Some(p) = draft.hover.clone() <span class="k">else</span> {
            <span class="k">self</span>.status(&quot;<span class="s">Point is outside the construction plane</span>&quot;);
            <span class="k">return</span> <span class="s">true</span>;
        };
        <span class="k">if</span> draft.points.len() &gt;= <span class="k">crate</span>::app::modeling::MAX_POINTS {
            <span class="k">self</span>.status(&quot;<span class="s">Too many points</span>&quot;);
            <span class="k">return</span> <span class="s">true</span>;
        }
        draft.points.push(p);
        <span class="k">let</span> message = <span class="k">self</span>.advance_drawing().unwrap_or_else(|e| {
            <span class="k">self</span>.draft.as_mut().unwrap().points.pop(); <span class="c">// a failed finish drops the point</span>
            e
        });
        <span class="k">self</span>.status(&amp;message);
        <span class="k">crate</span>::app::feedback::command_line(<span class="s">true</span>);
        <span class="s">true</span>
    }

    <span class="c">/// The draft as screen points for the preview, plus the snap name.</span>
    <span class="k">pub</span> <span class="k">fn</span> drawing_overlay(&amp;<span class="k">self</span>) -&gt; (Vec&lt;(f64, f64)&gt;, String) {
        <span class="k">let</span> Some(draft) = &amp;<span class="k">self</span>.draft <span class="k">else</span> {
            <span class="k">return</span> (Vec::new(), String::new());
        };
        <span class="k">let</span> points = draft
            .points
            .iter()
            .chain(draft.hover.iter())
            .filter_map(|p| <span class="k">self</span>.project([p[<span class="s">0</span>], p[<span class="s">1</span>], p[<span class="s">2</span>]]))
            .collect();
        (
            points,
            draft.snapped.map(|k| format!(&quot;{<span class="s">k:?</span>}&quot;)).unwrap_or_default(),
        )
    }
}

<span class="c">/// The two axes of a construction plane.</span>
<span class="k">fn</span> axes(plane: CPlane) -&gt; (Vector, Vector) {
    <span class="k">match</span> plane {
        CPlane::Xy =&gt; (Vector::new(<span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), Vector::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>)),
        CPlane::Xz =&gt; (Vector::new(<span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), Vector::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>)),
        CPlane::Yz =&gt; (Vector::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), Vector::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>)),
    }
}</code></pre></div>
<h2 id="step-41-srcenginegpuarenars">Step 41 · src/engine/gpu/arena.rs<a class="anchor" href="#/course/32-colors-lighting#step-41-srcenginegpuarenars" aria-label="Link to this section">#</a></h2>
<p>Retain exact source indices for boundary endpoints until the preview cache captures them.</p>
<p><code>lessons/32/src/engine/gpu/arena.rs</code> · edit · type this</p>
<p>Added after the line <code>pub face_sources: Vec&lt;super::faces::FaceSource&gt;,</code> in <code>lessons/31/src/engine/gpu/arena.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> surface_boundaries: Vec&lt;(u32, [u32; 2])&gt;, <span class="c">// pipe and sample range per surface edge</span></code></pre></div>
<p>Added after the line <code>drop_rows(&amp;mut self.surface_samples);</code> in <code>lessons/31/src/engine/gpu/arena.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        drop_rows(&amp;<span class="k">mut</span> <span class="k">self</span>.surface_boundaries);</code></pre></div>
<h2 id="step-42-srcshadersink_visibilitywgsl">Step 42 · src/shaders/ink_visibility.wgsl<a class="anchor" href="#/course/32-colors-lighting#step-42-srcshadersink_visibilitywgsl" aria-label="Link to this section">#</a></h2>
<p>Test a NURBS boundary against triangles at its actual projected position.</p>
<p><code>lessons/32/src/shaders/ink_visibility.wgsl</code> · edit · type this</p>
<p>Replaces the 2 lines from <code>fn ink_visible(pixel: vec2&lt;f32&gt;, axis: InkAxis, sample: u…</code> in <code>lessons/31/src/shaders/ink_visibility.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">// plane fit first, then the exact triangles</span>
<span class="k">fn</span> ink_visible(pixel: <span class="k">vec2</span>&lt;<span class="k">f32</span>&gt;, axis: InkAxis, sample: <span class="k">u32</span>, boundary: <span class="k">bool</span>) -&gt; <span class="k">bool</span> {
    <span class="k">let</span> plane_visible = ink_visible_plane(pixel, axis, sample);
    <span class="k">if</span> (plane_visible &amp;&amp; !boundary) {
        <span class="k">return</span> <span class="s">true</span>;
    }

<span class="c">    // no tile lists: the plane fit decides</span>
    <span class="k">if</span> (triangle_tiles[<span class="s">0</span>].x==<span class="s">0u</span>) {
        <span class="k">return</span> plane_visible;</code></pre></div>
<p>Replaces the 3 lines from <code>if (fringe==0u) {</code> in <code>lessons/31/src/shaders/ink_visibility.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">if</span> (fringe!=<span class="s">0u</span>) {
        <span class="k">let</span> fringe_hit = projected_triangle_at(projected[fringe-<span class="s">1u</span>], at);
        <span class="k">if</span> (fringe_hit.y&gt;<span class="s">0.5</span> &amp;&amp; fringe_hit.x&gt;axis.depth+abs(axis.depth)*DEPTH_REL_TOL) {
            <span class="k">return</span> <span class="s">false</span>;
        }</code></pre></div>
<p>Added after the line <code>}</code> in <code>lessons/31/src/shaders/ink_visibility.wgsl</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">    // a boundary stroke also checks every triangle in its tile</span>
    <span class="k">if</span> (plane_visible) { <span class="k">return</span> <span class="s">true</span>; }</code></pre></div>
<h2 id="step-43-supplied-files">Step 43 · supplied files<a class="anchor" href="#/course/32-colors-lighting#step-43-supplied-files" aria-label="Link to this section">#</a></h2>
<p>Copy each file from the lesson folder to the path shown.</p>
<p>Copy from <code>lessons/32/</code> (tooling this checkpoint needs but the course does not teach):</p>
<ul>
<li><code>session_cpp/src/color.cpp</code> (in the kernel checkout, not in the lesson folder)</li>
<li><code>session_cpp/src/color.h</code> (in the kernel checkout, not in the lesson folder)</li>
<li><code>session_cpp/src/color_test.cpp</code> (in the kernel checkout, not in the lesson folder)</li>
<li><code>session_py/src/session_py/color.py</code> (in the kernel checkout, not in the lesson folder)</li>
<li><code>session_py/src/session_py/color_test.py</code> (in the kernel checkout, not in the lesson folder)</li>
<li><code>session_rust/src/color_test.rs</code> (in the kernel checkout, not in the lesson folder)</li>
<li><code>lessons/32/tests/ambient-lighting.cjs</code></li>
<li><code>lessons/32/tests/command-workspace.cjs</code></li>
<li><code>lessons/32/tests/teapot.cjs</code></li>
</ul>
<h2 id="check">Check<a class="anchor" href="#/course/32-colors-lighting#check" aria-label="Link to this section">#</a></h2>
<p>Run <code>trunk serve</code> in <code>lessons/32/</code> and open <a href="http://127.0.0.1:8770/" target="_blank" rel="noopener">http://127.0.0.1:8770/</a>.</p>
<p>Expected: the output window sits above Command, options stay inline, and Arctic On adds concentrated ground shadows with <strong>SSAO On</strong> in the history.
<img src="/session/docs/course/docs/screenshots/current-workspace.png" alt="The completed command workspace with soft ambient lighting" loading="lazy" decoding="async"></p>
<p>If it fails:</p>
<ul>
<li>Typed letters duplicate: a sizing pass processes the same input events twice.</li>
<li>Suggestions disappear below the window: the popup opens downward instead of above the command field.</li>
<li>Surface edits show mesh diagonals: the display derives boundaries from tessellation.</li>
<li>Shadows remain after G turns them off: the optional texture or lighting uniform stays enabled.</li>
</ul>
<h2 id="what-changed">What changed<a class="anchor" href="#/course/32-colors-lighting#what-changed" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>lessons/32/src/
├── app/
│   ├── inspection/
│   │   └── source_memory.rs
│   ├── walk/
│   │   ├── bounds.rs
│   │   ├── brep.rs  ~
│   │   ├── brep_edges.rs
│   │   ├── brep_orient.rs
│   │   ├── cloud.rs
│   │   ├── curves.rs  ~
│   │   ├── encode.rs  ~
│   │   ├── frames.rs
│   │   ├── mesh.rs
│   │   ├── mesh_ink.rs
│   │   ├── mesh_topology.rs
│   │   ├── mod.rs
│   │   ├── points.rs
│   │   └── sheet.rs
│   ├── cloud_query.rs
│   ├── command.rs  ~
│   ├── coords.rs
│   ├── cplane.rs
│   ├── decode.rs
│   ├── deform.rs  ~
│   ├── edit.rs  ~
│   ├── feedback.rs  ~
│   ├── fetch.rs
│   ├── gizmo.rs  ~
│   ├── hierarchy.rs
│   ├── input.rs  ~
│   ├── inspection.rs  ~
│   ├── knobs.rs
│   ├── layers.rs
│   ├── live.rs
│   ├── loader.rs
│   ├── manifest.rs
│   ├── mesh_preview.rs  +
│   ├── mod.rs  ~
│   ├── modeling.rs
│   ├── route.rs
│   ├── scene.rs  ~
│   ├── scene_text.rs
│   ├── selection.rs
│   ├── session_io.rs  ~
│   ├── sheet_query.rs
│   ├── snap.rs
│   ├── splitting.rs  ~
│   ├── stream.rs
│   ├── surface_preview.rs  ~
│   ├── touch.rs
│   ├── ui.rs  ~
│   └── validate.rs
├── engine/
│   ├── gpu/
│   │   ├── arena.rs  ~
│   │   ├── backdrop.rs  ~
│   │   ├── buffers.rs
│   │   ├── cloud.rs
│   │   ├── device.rs
│   │   ├── faces.rs
│   │   ├── frame.rs  ~
│   │   ├── glyphs.rs  ~
│   │   ├── instance.rs  ~
│   │   ├── lod.rs
│   │   ├── mod.rs  ~
│   │   ├── objects.rs  ~
│   │   ├── patch.rs
│   │   ├── pick.rs
│   │   ├── present.rs
│   │   ├── render.rs  ~
│   │   ├── segments.rs
│   │   ├── splat.rs  ~
│   │   ├── ssao.rs  +
│   │   ├── surface_outline.rs
│   │   ├── targets.rs
│   │   ├── text.rs
│   │   ├── text_outline.rs
│   │   ├── text_plane.rs
│   │   ├── text_plate.rs
│   │   ├── triangle_tiles.rs
│   │   ├── ui.rs
│   │   ├── upload.rs
│   │   ├── view.rs  ~
│   │   ├── widget.rs
│   │   └── widget_mesh.rs
│   ├── pipelines/
│   │   ├── layouts.rs
│   │   └── mod.rs
│   ├── mod.rs
│   ├── performance.rs
│   └── text.rs
├── shaders/
│   ├── background.wgsl  ~
│   ├── glyph.wgsl  ~
│   ├── grid.wgsl
│   ├── ink_visibility.wgsl  ~
│   ├── normals.wgsl
│   ├── physical.wgsl
│   ├── project_triangles.wgsl
│   ├── projected_triangle.wgsl
│   ├── ribbon.wgsl  ~
│   ├── scan_triangle_tiles.wgsl
│   ├── scene.wgsl  ~
│   ├── sphere.wgsl  ~
│   ├── splat.wgsl
│   ├── splat_resolve.wgsl
│   ├── ssao.wgsl  +
│   ├── surface_outline.wgsl
│   ├── text_outline.wgsl
│   ├── text_plane.wgsl
│   ├── text_plate.wgsl
│   ├── triangle.wgsl  ~
│   ├── triangle_tiles.wgsl
│   └── widget.wgsl
├── state/
│   ├── cloud_query.rs
│   ├── drawing.rs  +
│   ├── edit.rs  ~
│   ├── panel.rs  ~
│   ├── sheet_query.rs
│   ├── splitting.rs
│   └── text.rs
├── camera.rs
├── lib.rs
└── state.rs  ~</code></pre></div>
<p><code>+</code> new in this lesson · <code>~</code> changed in this lesson</p>
<p>Data flow: command or layer selection → shared selection → cached previews; scene depth → small occlusion texture → smooth contact shadows.
Every file at this point: <code>lessons/32/</code>.</p>
<h2 id="next">Next<a class="anchor" href="#/course/32-colors-lighting#next" aria-label="Link to this section">#</a></h2>
<p><a href="#/course/33-contact-shadows">33 · Contact shadows that follow object size</a></p>
<h2 id="expected-viewer-result">Expected viewer result<a class="anchor" href="#/course/32-colors-lighting#expected-viewer-result" aria-label="Link to this section">#</a></h2>
<p>The command field sits below the viewport with a visible caret, inline completion, a scrollable command list and clickable options. Layers start hidden; Layers On reveals selection highlights and recursive layer controls. Point, Line, Polyline and Curve accept clicks or coordinates with Snap On by default. Shift selects multiple objects, one gumball moves them together, and G toggles Arctic shading and ground shadows. The background is white when Arctic is off. NURBS surfaces display only their true boundaries while editing. Face and edge colors remain independent.</p>
<p><a href="/session/docs/course/docs/screenshots/current-workspace.png"><img src="/session/docs/course/docs/screenshots/current-workspace.png" alt="The completed command workspace" loading="lazy" decoding="async"></a></p>
`,toc:[{level:2,id:"step-1-indexhtml",text:"Step 1 · index.html"},{level:2,id:"step-2-srcappdeformrs",text:"Step 2 · src/app/deform.rs"},{level:2,id:"step-3-srcappfeedbackrs",text:"Step 3 · src/app/feedback.rs"},{level:2,id:"step-4-srcappinspectionrs",text:"Step 4 · src/app/inspection.rs"},{level:2,id:"step-5-srcappmesh_previewrs",text:"Step 5 · src/app/mesh_preview.rs"},{level:2,id:"step-6-srcappmodrs",text:"Step 6 · src/app/mod.rs"},{level:2,id:"step-7-srcappsceners",text:"Step 7 · src/app/scene.rs"},{level:2,id:"step-8-srcappsession_iors",text:"Step 8 · src/app/session_io.rs"},{level:2,id:"step-9-srcappsplittingrs",text:"Step 9 · src/app/splitting.rs"},{level:2,id:"step-10-srcappuirs",text:"Step 10 · src/app/ui.rs"},{level:2,id:"step-11-srcenginegpuglyphsrs",text:"Step 11 · src/engine/gpu/glyphs.rs"},{level:2,id:"step-12-srcenginegpuinstancers",text:"Step 12 · src/engine/gpu/instance.rs"},{level:2,id:"step-13-srcenginegpumodrs",text:"Step 13 · src/engine/gpu/mod.rs"},{level:2,id:"step-14-srcenginegpuobjectsrs",text:"Step 14 · src/engine/gpu/objects.rs"},{level:2,id:"step-15-srcenginegpusplatrs",text:"Step 15 · src/engine/gpu/splat.rs"},{level:2,id:"step-16-srcshadersglyphwgsl",text:"Step 16 · src/shaders/glyph.wgsl"},{level:2,id:"step-17-srcshadersribbonwgsl",text:"Step 17 · src/shaders/ribbon.wgsl"},{level:2,id:"step-18-srcshadersscenewgsl",text:"Step 18 · src/shaders/scene.wgsl"},{level:2,id:"step-19-srcshadersspherewgsl",text:"Step 19 · src/shaders/sphere.wgsl"},{level:2,id:"step-20-srcstateeditrs",text:"Step 20 · src/state/edit.rs"},{level:2,id:"step-21-srcstatepanelrs",text:"Step 21 · src/state/panel.rs"},{level:2,id:"step-22-session_rustsrccolorrs",text:"Step 22 · session_rust/src/color.rs"},{level:2,id:"step-23-srcappcommandrs",text:"Step 23 · src/app/command.rs"},{level:2,id:"step-24-srcappeditrs",text:"Step 24 · src/app/edit.rs"},{level:2,id:"step-25-srcappgizmors",text:"Step 25 · src/app/gizmo.rs"},{level:2,id:"step-26-srcappinputrs",text:"Step 26 · src/app/input.rs"},{level:2,id:"step-27-srcappsurface_previewrs",text:"Step 27 · src/app/surface_preview.rs"},{level:2,id:"step-28-srcappwalkbreprs",text:"Step 28 · src/app/walk/brep.rs"},{level:2,id:"step-29-srcappwalkcurvesrs",text:"Step 29 · src/app/walk/curves.rs"},{level:2,id:"step-30-srcappwalkencoders",text:"Step 30 · src/app/walk/encode.rs"},{level:2,id:"step-31-srcenginegpuframers",text:"Step 31 · src/engine/gpu/frame.rs"},{level:2,id:"step-32-srcenginegpurenderrs",text:"Step 32 · src/engine/gpu/render.rs"},{level:2,id:"step-33-srcenginegpussaors",text:"Step 33 · src/engine/gpu/ssao.rs"},{level:2,id:"step-34-srcenginegpuviewrs",text:"Step 34 · src/engine/gpu/view.rs"},{level:2,id:"step-35-srcshadersbackgroundwgsl",text:"Step 35 · src/shaders/background.wgsl"},{level:2,id:"step-36-srcshadersssaowgsl",text:"Step 36 · src/shaders/ssao.wgsl"},{level:2,id:"step-37-srcshaderstrianglewgsl",text:"Step 37 · src/shaders/triangle.wgsl"},{level:2,id:"step-38-srcstaters",text:"Step 38 · src/state.rs"},{level:2,id:"step-39-srcenginegpubackdroprs",text:"Step 39 · src/engine/gpu/backdrop.rs"},{level:2,id:"step-40-srcstatedrawingrs",text:"Step 40 · src/state/drawing.rs"},{level:2,id:"step-41-srcenginegpuarenars",text:"Step 41 · src/engine/gpu/arena.rs"},{level:2,id:"step-42-srcshadersink_visibilitywgsl",text:"Step 42 · src/shaders/ink_visibility.wgsl"},{level:2,id:"step-43-supplied-files",text:"Step 43 · supplied files"},{level:2,id:"check",text:"Check"},{level:2,id:"what-changed",text:"What changed"},{level:2,id:"next",text:"Next"},{level:2,id:"expected-viewer-result",text:"Expected viewer result"}]};export{s as default};
