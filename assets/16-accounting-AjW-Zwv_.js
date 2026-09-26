const s={title:"16 · Resource accounting",html:`<h1 id="16-resource-accounting">16 · Resource accounting<a class="anchor" href="#/course/16-accounting#16-resource-accounting" aria-label="Link to this section">#</a></h1>
<p>The scene stays visible while the inspection snapshot reports retained source memory.</p>
<p><img src="/session/docs/course/docs/illustrations/source-cache.svg" alt="Scene owns documents through Rc; the cache keeps Weak identities and a payload figure, reuses it while the pointers match, walks once when a document is replaced, and never keeps a dropped document alive." loading="lazy" decoding="async"></p>
<p>Copy each file from the lesson folder to the path shown.</p>
<p>Copy from <code>lessons/16/</code> (tooling this checkpoint needs but the course does not teach):</p>
<ul>
<li><code>lessons/16/.gitignore</code></li>
<li><code>lessons/16/assets/pb/.gitkeep</code></li>
<li><code>lessons/16/examples/add_lod.rs</code></li>
<li><code>lessons/16/examples/cad_boundary_audit.rs</code></li>
<li><code>lessons/16/examples/cad_fixture.rs</code></li>
<li><code>lessons/16/examples/census_plates.rs</code></li>
<li><code>lessons/16/examples/check_cad_fixture.rs</code></li>
<li><code>lessons/16/examples/check_determinism.rs</code></li>
<li><code>lessons/16/examples/check_hidden_line_lifecycle.rs</code></li>
<li><code>lessons/16/examples/interaction_fixture.rs</code></li>
<li><code>lessons/16/examples/mk_brep_probe.rs</code></li>
<li><code>lessons/16/examples/mk_cylinder_hidden_probe.rs</code></li>
<li><code>lessons/16/examples/mk_hidden_line_probe.rs</code></li>
<li><code>lessons/16/examples/mk_joint_probe.rs</code></li>
<li><code>lessons/16/examples/mk_mixed_solids.rs</code></li>
<li><code>lessons/16/examples/mk_plate_outline.rs</code></li>
<li><code>lessons/16/examples/mk_shade_probe.rs</code></li>
<li><code>lessons/16/examples/mk_teapot.rs</code></li>
<li><code>lessons/16/examples/selftest.rs</code></li>
<li><code>lessons/16/src/selftest.rs</code></li>
<li><code>lessons/16/src/selftest/lifecycle.rs</code></li>
<li><code>lessons/16/tests/README.md</code></li>
<li><code>lessons/16/tests/cad-boundary-plot.py</code></li>
<li><code>lessons/16/tests/cad-quality.py</code></li>
<li><code>lessons/16/tests/depth/_closeup_box.py</code></li>
<li><code>lessons/16/tests/depth/_count_colors.py</code></li>
<li><code>lessons/16/tests/depth/_gate.sh</code></li>
<li><code>lessons/16/tests/depth/_hidden_line_matrix.py</code></li>
<li><code>lessons/16/tests/depth/_ink_suite.sh</code></li>
<li><code>lessons/16/tests/depth/_orbit_check.py</code></li>
<li><code>lessons/16/tests/depth/_probe_matrix.py</code></li>
<li><code>lessons/16/tests/depth/_shade_scanline.py</code></li>
<li><code>lessons/16/tests/depth/_stroke_weight.py</code></li>
<li><code>lessons/16/tests/format.py</code></li>
<li><code>lessons/16/tests/interaction.cjs</code></li>
<li><code>lessons/16/tests/nameplate-scene.cjs</code></li>
<li><code>lessons/16/tests/nameplate.cjs</code></li>
<li><code>lessons/16/tests/teapot.cjs</code></li>
<li><code>lessons/16/tests/text-quality.cjs</code></li>
<li><code>lessons/16/tests/world-text.cjs</code></li>
</ul>
<h2 id="step-1-cargotoml">Step 1 · Cargo.toml<a class="anchor" href="#/course/16-accounting#step-1-cargotoml" aria-label="Link to this section">#</a></h2>
<p>Copy this file from the lesson folder to the path shown.</p>
<p><code>lessons/16/Cargo.toml</code> · edit · copy the file</p>
<p>Replaces the 16 lines from <code>getrandom = { version = &quot;0.2&quot;, features = [&quot;j…</code> of <code>lessons/15/Cargo.toml</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c"># Never called here: the kernel's rand 0.8 pulls in getrandom 0.2, which builds for wasm32 only with \`js\`.</span>
getrandom = { version = &quot;<span class="s">0.2</span>&quot;, features = [&quot;<span class="s">js</span>&quot;] }
js-sys = &quot;<span class="s">0.3</span>&quot;
prost = &quot;<span class="s">0.14</span>&quot;
bytemuck = { version = &quot;<span class="s">1</span>&quot;, features = [&quot;<span class="s">derive</span>&quot;] }

[profile.dev.package.&quot;*&quot;]
opt-level = <span class="s">3</span> <span class="c"># dependencies optimised even in debug builds, so a large scene still parses fast</span>

[profile.release]
strip = <span class="s">true</span>

[dev-dependencies]
naga = { version = &quot;<span class="s">=29.0.4</span>&quot;, features = [&quot;<span class="s">wgsl-in</span>&quot;] }</code></pre></div>
<h2 id="step-2-srcappinspectionsource_memoryrs">Step 2 · src/app/inspection/source_memory.rs<a class="anchor" href="#/course/16-accounting#step-2-srcappinspectionsource_memoryrs" aria-label="Link to this section">#</a></h2>
<p>The source cache counts retained payload once per shared identity.</p>
<p><code>lessons/16/src/app/inspection/source_memory.rs</code> · type this, new file, start with these lines</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! Adds up what the loaded document costs in memory, counting each Vec and String once, to explain the status-line number.</span>
<span class="k">use</span> super::super::scene::FileDoc;
<span class="k">use</span> session_rust::{
    BRep, Collection, Element, Geometry, Line, Mesh, NurbsCurve, NurbsSurface, NurbsSurfaceTrimmed,
    OBB, Plane, Point, PointCloud, Polyline, Session,
};
<span class="k">use</span> std::collections::{HashMap, HashSet};
<span class="k">use</span> std::mem::{size_of, size_of_val};
<span class="k">use</span> std::rc::{Rc, Weak};

<span class="c">/// Memory held by the loaded documents, by category.</span>
#[derive(Clone, Copy, Default, serde::Serialize)]
<span class="k">pub</span>(super) <span class="k">struct</span> Payload {
    <span class="k">pub</span> vector_capacity_bytes: usize,
    <span class="k">pub</span> string_capacity_bytes: usize,
    <span class="k">pub</span> exposed_slice_bytes: usize, <span class="c">// slices whose capacity is hidden</span>
    <span class="k">pub</span> occupied_map_entry_bytes: usize,
    <span class="k">pub</span> shared_value_bytes: usize, <span class="c">// each Rc value once</span>
    <span class="k">pub</span> unique_sessions: usize,
    <span class="k">pub</span> unique_geometry_values: usize,
    <span class="k">pub</span> scans: u64, <span class="c">// how many times counted</span>
}

<span class="k">impl</span> Payload {
    <span class="c">/// Total of the byte categories.</span>
    <span class="k">pub</span> <span class="k">fn</span> known_bytes(&amp;<span class="k">self</span>) -&gt; usize {
        <span class="k">self</span>.vector_capacity_bytes
            + <span class="k">self</span>.string_capacity_bytes
            + <span class="k">self</span>.exposed_slice_bytes
            + <span class="k">self</span>.occupied_map_entry_bytes
            + <span class="k">self</span>.shared_value_bytes
    }

    <span class="c">/// Add a Vec's capacity.</span>
    <span class="k">fn</span> vector&lt;T&gt;(&amp;<span class="k">mut</span> <span class="k">self</span>, value: &amp;Vec&lt;T&gt;) {
        <span class="k">self</span>.vector_capacity_bytes += value.capacity() * size_of::&lt;T&gt;();
    }

    <span class="c">/// A kernel Collection keeps a deleted object's slot until the next purge, so undo can revive it: every slot costs memory.</span>
    <span class="k">fn</span> collection&lt;T&gt;(&amp;<span class="k">mut</span> <span class="k">self</span>, value: &amp;Collection&lt;T&gt;) {
        <span class="k">self</span>.vector_capacity_bytes += value.number_of_slots() * size_of::&lt;T&gt;();
    }

    <span class="c">/// Add a slice's length.</span>
    <span class="k">fn</span> slice&lt;T&gt;(&amp;<span class="k">mut</span> <span class="k">self</span>, value: &amp;[T]) {
        <span class="k">self</span>.exposed_slice_bytes += size_of_val(value);
    }

    <span class="c">/// Add a map's entries.</span>
    <span class="k">fn</span> map&lt;K, V&gt;(&amp;<span class="k">mut</span> <span class="k">self</span>, value: &amp;HashMap&lt;K, V&gt;) {
        <span class="k">self</span>.occupied_map_entry_bytes += value.len() * size_of::&lt;(K, V)&gt;();
    }

    <span class="c">/// Add a String's capacity.</span>
    <span class="k">fn</span> string(&amp;<span class="k">mut</span> <span class="k">self</span>, value: &amp;String) {
        <span class="k">self</span>.string_capacity_bytes += value.capacity();
    }
}</code></pre></div>
<p><code>lessons/16/src/app/inspection/source_memory.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// The last count, reused while the documents are the same.</span>
#[derive(Default)]
<span class="k">pub</span>(super) <span class="k">struct</span> SourceCache {
    documents: Vec&lt;Weak&lt;Session&gt;&gt;, <span class="c">// sessions counted, without keeping them alive</span>
    payload: Payload, <span class="c">// their count</span>
}

<span class="k">impl</span> SourceCache {
    <span class="c">/// The payload of \`docs\`, recounted when they changed.</span>
    <span class="k">pub</span> <span class="k">fn</span> snapshot(&amp;<span class="k">mut</span> <span class="k">self</span>, docs: &amp;[FileDoc]) -&gt; Payload {
        <span class="k">let</span> <span class="k">mut</span> matches = docs.len() == <span class="k">self</span>.documents.len();

        <span class="k">if</span> matches {
            <span class="k">for</span> (old, doc) <span class="k">in</span> <span class="k">self</span>.documents.iter().zip(docs) {
                <span class="k">if</span> old.as_ptr() != Rc::as_ptr(&amp;doc.session) {
                    matches = <span class="s">false</span>;
                    <span class="k">break</span>;
                }
            }
        }

        <span class="k">if</span> matches {
            <span class="k">return</span> <span class="k">self</span>.payload;
        }

        <span class="k">let</span> <span class="k">mut</span> payload = Payload {
            scans: <span class="k">self</span>.payload.scans + <span class="s">1</span>,
            ..Payload::default()
        };
        <span class="k">let</span> <span class="k">mut</span> seen_sessions = HashSet::new();
        <span class="k">let</span> <span class="k">mut</span> seen_geometry = HashSet::new();
        <span class="k">self</span>.documents.clear();

        <span class="k">for</span> doc <span class="k">in</span> docs {
            <span class="k">self</span>.documents.push(Rc::downgrade(&amp;doc.session));

            <span class="k">if</span> seen_sessions.insert(Rc::as_ptr(&amp;doc.session) <span class="k">as</span> usize) {
                payload.unique_sessions += <span class="s">1</span>;
                payload.shared_value_bytes += size_of::&lt;Session&gt;();
                session_payload(&amp;doc.session, &amp;<span class="k">mut</span> payload, &amp;<span class="k">mut</span> seen_geometry);
            }
        }

        <span class="k">self</span>.payload = payload;
        payload
    }
}</code></pre></div>
<p><code>lessons/16/src/app/inspection/source_memory.rs</code> · type this, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Add a Collection of Rc values, each live value once.</span>
<span class="k">fn</span> shared_all&lt;T&gt;(
    values: &amp;Collection&lt;Rc&lt;T&gt;&gt;,
    payload: &amp;<span class="k">mut</span> Payload,
    seen: &amp;<span class="k">mut</span> HashSet&lt;usize&gt;,
    children: <span class="k">fn</span>(&amp;T, &amp;<span class="k">mut</span> Payload),
) {
    payload.collection(values);

    <span class="k">for</span> value <span class="k">in</span> values {
        shared(value, payload, seen, children);
    }
}

<span class="c">/// Add one Rc value unless already seen.</span>
<span class="k">fn</span> shared&lt;T&gt;(
    value: &amp;Rc&lt;T&gt;,
    payload: &amp;<span class="k">mut</span> Payload,
    seen: &amp;<span class="k">mut</span> HashSet&lt;usize&gt;,
    children: <span class="k">fn</span>(&amp;T, &amp;<span class="k">mut</span> Payload),
) {
    <span class="k">if</span> !seen.insert(Rc::as_ptr(value) <span class="k">as</span> usize) {
        <span class="k">return</span>;
    }

    payload.unique_geometry_values += <span class="s">1</span>;
    payload.shared_value_bytes += size_of::&lt;T&gt;();
    children(value, payload);
}

<span class="c">/// Add one session's bytes.</span>
<span class="k">fn</span> session_payload(session: &amp;Session, p: &amp;<span class="k">mut</span> Payload, seen: &amp;<span class="k">mut</span> HashSet&lt;usize&gt;) {
    p.string(&amp;session.name);
    p.string(&amp;session.objects.name);
    <span class="k">let</span> objects = &amp;session.objects;
    shared_all(&amp;objects.points, p, seen, point_payload);
    shared_all(&amp;objects.lines, p, seen, line_payload);
    shared_all(&amp;objects.planes, p, seen, plane_payload);
    shared_all(&amp;objects.bboxes, p, seen, box_payload);
    shared_all(&amp;objects.polylines, p, seen, polyline_payload);
    shared_all(&amp;objects.pointclouds, p, seen, cloud_payload);
    shared_all(&amp;objects.meshes, p, seen, mesh_payload);
    shared_all(&amp;objects.nurbscurves, p, seen, curve_payload);
    shared_all(&amp;objects.nurbssurfaces, p, seen, surface_payload);
    p.vector(&amp;objects.nurbssurfacetrimmeds); <span class="c">// the one kernel list that is still a plain Vec</span>

    <span class="k">for</span> value <span class="k">in</span> &amp;objects.nurbssurfacetrimmeds {
        shared(value, p, seen, trimmed_payload);
    }

    shared_all(&amp;objects.breps, p, seen, brep_payload);
    shared_all(&amp;objects.elements, p, seen, element_payload);</code></pre></div>
<p>Copy this part from the lesson folder to the path shown.</p>
<p><code>lessons/16/src/app/inspection/source_memory.rs</code> · copy the file, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    p.collection(&amp;objects.components);

    <span class="k">for</span> component <span class="k">in</span> &amp;objects.components {
        p.string(&amp;component.name);
        p.string(&amp;component.type_name);
        p.string(&amp;component.guid);
    }

    p.map(&amp;session.lookup);

    <span class="k">for</span> (name, geometry) <span class="k">in</span> &amp;session.lookup {
        p.string(name);

        <span class="k">match</span> geometry {
            Geometry::Point(value) =&gt; shared(value, p, seen, point_payload),
            Geometry::Line(value) =&gt; shared(value, p, seen, line_payload),
            Geometry::Plane(value) =&gt; shared(value, p, seen, plane_payload),
            Geometry::OBB(value) =&gt; shared(value, p, seen, box_payload),
            Geometry::Polyline(value) =&gt; shared(value, p, seen, polyline_payload),
            Geometry::PointCloud(value) =&gt; shared(value, p, seen, cloud_payload),
            Geometry::Mesh(value) =&gt; shared(value, p, seen, mesh_payload),
            Geometry::NurbsCurve(value) =&gt; shared(value, p, seen, curve_payload),
            Geometry::NurbsSurface(value) =&gt; shared(value, p, seen, surface_payload),
            Geometry::BRep(value) =&gt; shared(value, p, seen, brep_payload),
            Geometry::Element(value) =&gt; shared(value, p, seen, element_payload),
        }
    }

    p.map(&amp;session.xforms);

    <span class="k">for</span> name <span class="k">in</span> session.xforms.keys() {
        p.string(name);
    }

    p.vector(&amp;session.cached_guids);

    <span class="k">for</span> name <span class="k">in</span> &amp;session.cached_guids {
        p.string(name);
    }

    p.vector(&amp;session.cached_boxes);

    <span class="k">for</span> value <span class="k">in</span> &amp;session.cached_boxes {
        box_payload(value, p);
    }
}

<span class="c">/// Add a point's name bytes.</span>
<span class="k">fn</span> point_payload(value: &amp;Point, p: &amp;<span class="k">mut</span> Payload) {
    p.string(&amp;value.name);
    p.string(&amp;value.pointcolor.name);
}

<span class="c">/// Add a line's name and dash bytes.</span>
<span class="k">fn</span> line_payload(value: &amp;Line, p: &amp;<span class="k">mut</span> Payload) {
    p.string(&amp;value.name);
    p.vector(&amp;value.dash);
    p.string(&amp;value.linecolor.name);
}

<span class="c">/// Add a plane's name bytes.</span>
<span class="k">fn</span> plane_payload(value: &amp;Plane, p: &amp;<span class="k">mut</span> Payload) {
    p.string(&amp;value.name);
    p.string(&amp;value.linecolor.name);
}

<span class="c">/// Add a box's name and centre bytes.</span>
<span class="k">fn</span> box_payload(value: &amp;OBB, p: &amp;<span class="k">mut</span> Payload) {
    p.string(&amp;value.name);
    point_payload(&amp;value.center, p);
    p.string(&amp;value.x_axis.name);
    p.string(&amp;value.y_axis.name);
    p.string(&amp;value.z_axis.name);
    p.string(&amp;value.half_size.name);
}

<span class="c">/// Add a polyline's coordinate, dash and name bytes.</span>
<span class="k">fn</span> polyline_payload(value: &amp;Polyline, p: &amp;<span class="k">mut</span> Payload) {
    p.string(&amp;value.name);
    p.vector(&amp;value.coords);
    p.vector(&amp;value.dash);
    plane_payload(&amp;value.plane, p);
    p.string(&amp;value.linecolor.name);
}

<span class="c">/// Add a cloud's array bytes.</span>
<span class="k">fn</span> cloud_payload(value: &amp;PointCloud, p: &amp;<span class="k">mut</span> Payload) {
    p.string(&amp;value.name);
    p.slice(value.coords());
    p.slice(value.colors());
    p.slice(value.normals());
    p.slice(value.point_ids());
}

<span class="c">/// Add a curve's control, knot and colour bytes.</span>
<span class="k">fn</span> curve_payload(value: &amp;NurbsCurve, p: &amp;<span class="k">mut</span> Payload) {
    p.string(&amp;value.name);
    p.vector(&amp;value.m_cv);
    p.vector(&amp;value.m_nurbsknot);
    p.vector(&amp;value.pointcolors);
    p.vector(&amp;value.linecolors);
    color_names(&amp;value.pointcolors, p);
    color_names(&amp;value.linecolors, p);
}

<span class="c">/// Add a surface's bytes, cached mesh included.</span>
<span class="k">fn</span> surface_payload(value: &amp;NurbsSurface, p: &amp;<span class="k">mut</span> Payload) {
    p.string(&amp;value.name);
    p.vector(&amp;value.m_cv);
    p.vector(&amp;value.m_nurbsknot[<span class="s">0</span>]);
    p.vector(&amp;value.m_nurbsknot[<span class="s">1</span>]);
    p.vector(&amp;value.pointcolors);
    p.vector(&amp;value.linecolors);
    p.vector(&amp;value.facecolors);
    color_names(&amp;value.pointcolors, p);
    color_names(&amp;value.linecolors, p);
    color_names(&amp;value.facecolors, p);

    <span class="k">if</span> <span class="k">let</span> Some(mesh) = &amp;value.m_mesh {
        mesh_payload(mesh, p);
    }
}

<span class="c">/// Add a trimmed surface's loop bytes.</span>
<span class="k">fn</span> trimmed_payload(value: &amp;NurbsSurfaceTrimmed, p: &amp;<span class="k">mut</span> Payload) {
    p.string(&amp;value.name);
    p.string(&amp;value.surfacecolor.name);
    surface_payload(&amp;value.m_surface, p);
    p.vector(&amp;value.m_inner_loops);
    p.vector(&amp;value.cut_planes);

    <span class="k">if</span> <span class="k">let</span> Some(curve) = &amp;value.m_outer_loop {
        curve_payload(curve, p);
    }

    <span class="k">for</span> curve <span class="k">in</span> &amp;value.m_inner_loops {
        curve_payload(curve, p);
    }

    <span class="k">if</span> <span class="k">let</span> Some(point) = &amp;value.cut_q0 {
        point_payload(point, p);
    }

    <span class="k">if</span> <span class="k">let</span> Some(normal) = &amp;value.cut_n {
        p.string(&amp;normal.name);
    }

    <span class="k">for</span> (point, normal) <span class="k">in</span> &amp;value.cut_planes {
        point_payload(point, p);
        p.string(&amp;normal.name);
    }
}

<span class="c">/// Add a BRep's topology, curve and surface bytes.</span>
<span class="k">fn</span> brep_payload(value: &amp;BRep, p: &amp;<span class="k">mut</span> Payload) {
    p.string(&amp;value.name);
    p.string(&amp;value.surfacecolor.name);
    p.vector(&amp;value.m_surfaces);
    p.vector(&amp;value.m_curves_3d);
    p.vector(&amp;value.m_curves_2d);
    p.vector(&amp;value.m_vertices);
    p.vector(&amp;value.m_edges);
    p.vector(&amp;value.m_wires);
    p.vector(&amp;value.m_faces);
    p.vector(&amp;value.m_shells);
    p.vector(&amp;value.m_solids);

    <span class="k">for</span> surface <span class="k">in</span> &amp;value.m_surfaces {
        surface_payload(surface, p);
    }

    <span class="k">for</span> curve <span class="k">in</span> &amp;value.m_curves_3d {
        curve_payload(curve, p);
    }

    <span class="k">for</span> curve <span class="k">in</span> &amp;value.m_curves_2d {
        curve_payload(curve, p);
    }

    <span class="k">for</span> vertex <span class="k">in</span> &amp;value.m_vertices {
        point_payload(&amp;vertex.point, p);
    }

    <span class="k">for</span> edge <span class="k">in</span> &amp;value.m_edges {
        p.vector(&amp;edge.pcurves);
    }

    <span class="k">for</span> wire <span class="k">in</span> &amp;value.m_wires {
        p.vector(&amp;wire.edges);
    }

    <span class="k">for</span> face <span class="k">in</span> &amp;value.m_faces {
        p.vector(&amp;face.wires);

        <span class="k">if</span> <span class="k">let</span> Some(color) = &amp;face.facecolor {
            p.string(&amp;color.name);
        }
    }

    <span class="k">for</span> shell <span class="k">in</span> &amp;value.m_shells {
        p.vector(&amp;shell.faces);
    }

    <span class="k">for</span> solid <span class="k">in</span> &amp;value.m_solids {
        p.vector(&amp;solid.shells);
    }
}

<span class="c">/// Add colour name bytes.</span>
<span class="k">fn</span> color_names(values: &amp;[session_rust::Color], p: &amp;<span class="k">mut</span> Payload) {
    <span class="k">for</span> color <span class="k">in</span> values {
        p.string(&amp;color.name);
    }
}

<span class="c">/// Add attribute entry and key bytes.</span>
<span class="k">fn</span> attributes_payload(value: &amp;HashMap&lt;String, f64&gt;, p: &amp;<span class="k">mut</span> Payload) {
    p.map(value);

    <span class="k">for</span> key <span class="k">in</span> value.keys() {
        p.string(key);
    }
}

<span class="c">/// Add a mesh's vertex, face and cache bytes.</span>
<span class="k">fn</span> mesh_payload(value: &amp;Mesh, p: &amp;<span class="k">mut</span> Payload) {
    p.string(&amp;value.name);
    p.map(&amp;value.vertex);
    p.map(&amp;value.face);
    p.map(&amp;value.halfedge);

    <span class="k">for</span> vertex <span class="k">in</span> value.vertex.values() {
        p.occupied_map_entry_bytes += vertex.attributes.len() * size_of::&lt;(String, f64)&gt;();

        <span class="k">for</span> key <span class="k">in</span> vertex.attributes.keys() {
            p.string(key);
        }
    }

    <span class="k">for</span> face <span class="k">in</span> value.face.values() {
        p.vector(face);
    }

    <span class="k">for</span> edges <span class="k">in</span> value.halfedge.values() {
        p.map(edges);
    }

    p.map(&amp;value.facedata);

    <span class="k">for</span> attributes <span class="k">in</span> value.facedata.values() {
        attributes_payload(attributes, p);
    }

    p.map(&amp;value.edgedata);

    <span class="k">for</span> attributes <span class="k">in</span> value.edgedata.values() {
        attributes_payload(attributes, p);
    }

    attributes_payload(&amp;value.default_vertex_attributes, p);
    attributes_payload(&amp;value.default_face_attributes, p);
    attributes_payload(&amp;value.default_edge_attributes, p);
    p.map(&amp;value.triangulation);

    <span class="k">for</span> triangles <span class="k">in</span> value.triangulation.values() {
        p.vector(triangles);
    }

    p.map(&amp;value.face_holes);

    <span class="k">for</span> holes <span class="k">in</span> value.face_holes.values() {
        p.vector(holes);

        <span class="k">for</span> hole <span class="k">in</span> holes {</code></pre></div>
<p>Copy this part from the lesson folder to the path shown.</p>
<p><code>lessons/16/src/app/inspection/source_memory.rs</code> · copy the file, append at the end of the file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            p.vector(hole);
        }
    }

    p.vector(&amp;value.tri_tris);
    p.vector(&amp;value.tri_vertices);

    <span class="k">for</span> point <span class="k">in</span> &amp;value.tri_vertices {
        point_payload(point, p);
    }

    p.slice(value.get_pointcolors());
    p.slice(value.get_linecolors());
    p.slice(value.get_facecolors());
    p.slice(value.widths());
    color_names(value.get_pointcolors(), p);
    color_names(value.get_linecolors(), p);
    color_names(value.get_facecolors(), p);
    p.string(&amp;value.objectcolor().name);
}

<span class="c">/// Add an element's geometry and feature bytes.</span>
<span class="k">fn</span> element_payload(value: &amp;Element, p: &amp;<span class="k">mut</span> Payload) {
    <span class="k">use</span> session_rust::element::ElementGeometry;
    p.string(&amp;value.name);
    p.string(&amp;value.element_type);
    p.vector(&amp;value.element_data);
    p.vector(&amp;value.features);
    p.vector(&amp;value.insertion_vectors);

    <span class="k">for</span> vector <span class="k">in</span> &amp;value.insertion_vectors {
        p.string(&amp;vector.name);
    }

    <span class="k">if</span> <span class="k">let</span> Some(vector) = &amp;value.dimensions {
        p.string(&amp;vector.name);
    }

    <span class="k">for</span> feature <span class="k">in</span> &amp;value.features {
        p.string(&amp;feature.name);
        p.string(&amp;feature.feature_type);
        p.vector(&amp;feature.outlines);

        <span class="k">for</span> outline <span class="k">in</span> &amp;feature.outlines {
            polyline_payload(outline, p);
        }
    }

    <span class="k">match</span> value.geometry() {
        ElementGeometry::Mesh(mesh) =&gt; mesh_payload(mesh, p),
        ElementGeometry::BRep(brep) =&gt; brep_payload(brep, p),
        ElementGeometry::None =&gt; {}
    }
}

#[cfg(test)]
<span class="k">mod</span> tests {
    <span class="k">use</span> super::*;
    <span class="k">use</span> session_rust::Xform;

    <span class="c">/// A document at the origin.</span>
    <span class="k">fn</span> document(session: Rc&lt;Session&gt;) -&gt; FileDoc {
        FileDoc {
            name: &quot;<span class="s">memory fixture</span>&quot;.into(),
            place: Xform::identity(),
            session,
            point_px: <span class="s">0</span>.<span class="s">0</span>,
            display_only: <span class="s">false</span>,
        }
    }

    <span class="c">/// A session with spare capacity in a polyline.</span>
    <span class="k">fn</span> source() -&gt; Rc&lt;Session&gt; {
        <span class="k">let</span> <span class="k">mut</span> line = Polyline::new(vec![Point::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), Point::new(<span class="s">1</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>)]);
        line.coords.reserve(<span class="s">128</span>);
        <span class="k">let</span> <span class="k">mut</span> session = Session::new(&quot;<span class="s">memory fixture</span>&quot;);
        session.add_polyline(line, None);
        Rc::new(session)
    }

    <span class="c">/// A geometry shared twice is counted once.</span>
    #[test]
    <span class="k">fn</span> shared_documents_and_lookup_do_not_duplicate_geometry_payload() {
        <span class="k">let</span> source = source();
        <span class="k">let</span> <span class="k">mut</span> cache = SourceCache::default();
        <span class="k">let</span> <span class="k">mut</span> docs = vec![document(Rc::clone(&amp;source))];
        <span class="k">let</span> once = cache.snapshot(&amp;docs);
        assert!(once.known_bytes() &gt;= once.vector_capacity_bytes);
        assert_eq!(once.unique_sessions, <span class="s">1</span>);
        assert_eq!(once.unique_geometry_values, <span class="s">1</span>);
        assert!(once.vector_capacity_bytes &gt;= <span class="s">128</span> * size_of::&lt;f64&gt;());
        assert_eq!(cache.snapshot(&amp;docs).scans, once.scans);
        docs.push(document(Rc::clone(&amp;source)));
        <span class="k">let</span> twice = cache.snapshot(&amp;docs);
        assert_eq!(twice.unique_sessions, <span class="s">1</span>);
        assert_eq!(twice.unique_geometry_values, <span class="s">1</span>);
        assert_eq!(twice.vector_capacity_bytes, once.vector_capacity_bytes);
        assert_eq!(twice.string_capacity_bytes, once.string_capacity_bytes);
        assert_eq!(twice.shared_value_bytes, once.shared_value_bytes);
    }

    <span class="c">/// A replaced session is recounted and released.</span>
    #[test]
    <span class="k">fn</span> replacement_invalidates_without_retaining_the_old_source() {
        <span class="k">let</span> <span class="k">mut</span> docs = vec![document(source())];
        <span class="k">let</span> old = Rc::downgrade(&amp;docs[<span class="s">0</span>].session);
        <span class="k">let</span> <span class="k">mut</span> cache = SourceCache::default();
        <span class="k">let</span> before = cache.snapshot(&amp;docs);
        assert_eq!(Rc::strong_count(&amp;docs[<span class="s">0</span>].session), <span class="s">1</span>);
        docs[<span class="s">0</span>] = document(source());
        assert!(old.upgrade().is_none());
        <span class="k">let</span> after = cache.snapshot(&amp;docs);
        assert_eq!(after.scans, before.scans + <span class="s">1</span>);
        docs.clear();
        <span class="k">let</span> empty = cache.snapshot(&amp;docs);
        assert_eq!(empty.vector_capacity_bytes, <span class="s">0</span>);
        assert_eq!(empty.shared_value_bytes, <span class="s">0</span>);
        assert_eq!(empty.unique_sessions, <span class="s">0</span>);
    }

    <span class="c">/// Cloud slices count separately from Vec capacity.</span>
    #[test]
    <span class="k">fn</span> cloud_borrowed_payload_is_separate_from_known_vector_capacity() {
        <span class="k">let</span> cloud = PointCloud::from_coords(vec![<span class="s">0</span>.<span class="s">0</span>; <span class="s">30</span>], vec![<span class="s">255</span>; <span class="s">40</span>], vec![<span class="s">0</span>.<span class="s">0</span>; <span class="s">30</span>]);
        <span class="k">let</span> <span class="k">mut</span> payload = Payload::default();
        cloud_payload(&amp;cloud, &amp;<span class="k">mut</span> payload);
        assert_eq!(
            payload.exposed_slice_bytes,
            <span class="s">60</span> * size_of::&lt;f64&gt;() + <span class="s">40</span> * size_of::&lt;i32&gt;()
        );
        assert_eq!(payload.vector_capacity_bytes, <span class="s">0</span>);
    }
}</code></pre></div>
<p>Run <code>cargo check</code> in <code>lessons/16/</code>.</p>
<h2 id="step-3-srcappinspectionrs">Step 3 · src/app/inspection.rs<a class="anchor" href="#/course/16-accounting#step-3-srcappinspectionrs" aria-label="Link to this section">#</a></h2>
<p>Inspection reports retained resources and source information.</p>
<p><code>lessons/16/src/app/inspection.rs</code> · edit · type this</p>
<p>Added after the <code>use crate::State;</code> line of <code>lessons/15/src/app/inspection.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">mod</span> source_memory;

thread_local! {
    <span class="k">static</span> SOURCE_MEMORY: std::cell::RefCell&lt;source_memory::SourceCache&gt; = Default::default();
}</code></pre></div>
<p>Added after the <code>let (buffers, textures) = state.gpu.allocated…</code> line in <code>fn publish</code> of <code>lessons/15/src/app/inspection.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">let</span> source_memory = SOURCE_MEMORY.with_borrow_mut(|cache| cache.snapshot(&amp;state.scene.docs));</code></pre></div>
<p>Added after the <code>&quot;samples&quot;: state.gpu.targets.samples,</code> line in <code>fn publish</code> of <code>lessons/15/src/app/inspection.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        &quot;<span class="s">source_cpu_known_payload_bytes</span>&quot;: source_memory.known_bytes(),
        &quot;<span class="s">source_cpu_known_payload</span>&quot;: source_memory,
        &quot;<span class="s">source_cpu_scope</span>&quot;: &quot;<span class="s">retained Session arrays/strings/values; Rc objects deduplicated; not RSS or total heap</span>&quot;,
        &quot;<span class="s">source_cpu_exclusions</span>&quot;: &quot;<span class="s">allocator/Rc/map overhead and spare map slots, private cloud LOD/capacity, GUID allocations, private nested metadata/element/BVH caches, tree/graph/component-extra payloads, streamed descriptors, upload staging and loader buffers</span>&quot;,</code></pre></div>
<h2 id="check">Check<a class="anchor" href="#/course/16-accounting#check" aria-label="Link to this section">#</a></h2>
<p>Run <code>trunk serve</code> in <code>lessons/16/</code> and open <a href="http://127.0.0.1:8770/" target="_blank" rel="noopener">http://127.0.0.1:8770/</a>.</p>
<p>Expected: The scene stays visible while the inspection snapshot reports retained source memory; status: <strong>the status clears when loading finishes</strong>.</p>
<p><img src="/session/docs/course/docs/screenshots/16.png" alt="Checkpoint 16: the scene is unchanged; the new figures live in the inspection snapshot above." loading="lazy" decoding="async"></p>
<p>If it fails:</p>
<ul>
<li>Payload bytes double for shared files: shared source values are counted more than once.</li>
<li>The scan count grows each frame: accounting is not cached by document identity.</li>
</ul>
<h2 id="what-changed">What changed<a class="anchor" href="#/course/16-accounting#what-changed" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>lessons/16/src/app/
├── inspection/
│   └── source_memory.rs  +
├── walk/
│   ├── bounds.rs
│   ├── brep.rs
│   ├── brep_edges.rs
│   ├── brep_orient.rs
│   ├── cloud.rs
│   ├── curves.rs
│   ├── encode.rs
│   ├── frames.rs
│   ├── mesh.rs
│   ├── mesh_ink.rs
│   ├── mesh_topology.rs
│   ├── mod.rs
│   └── points.rs
├── cloud_query.rs
├── decode.rs
├── feedback.rs
├── fetch.rs
├── input.rs
├── inspection.rs  ~
├── knobs.rs
├── live.rs
├── loader.rs
├── manifest.rs
├── mod.rs
├── route.rs
├── scene.rs
├── selection.rs
├── stream.rs
├── touch.rs
└── validate.rs</code></pre></div>
<p><code>+</code> new in this lesson · <code>~</code> changed in this lesson</p>
<p>Every file at this point: <code>lessons/16/</code>.</p>
<h2 id="next">Next<a class="anchor" href="#/course/16-accounting#next" aria-label="Link to this section">#</a></h2>
<p><a href="#/course/17-source-presentation">17 · Source faces, text objects and one silhouette</a></p>
<h2 id="expected-viewer-result">Expected viewer result<a class="anchor" href="#/course/16-accounting#expected-viewer-result" aria-label="Link to this section">#</a></h2>
<p>Checkpoint 16: the scene is unchanged; the new figures live in the inspection snapshot above.</p>
<p><a href="/session/docs/course/docs/screenshots/16.png"><img src="/session/docs/course/docs/screenshots/16.png" alt="Full viewer result for 16 accounting" loading="lazy" decoding="async"></a></p>
`,toc:[{level:2,id:"step-1-cargotoml",text:"Step 1 · Cargo.toml"},{level:2,id:"step-2-srcappinspectionsource_memoryrs",text:"Step 2 · src/app/inspection/source_memory.rs"},{level:2,id:"step-3-srcappinspectionrs",text:"Step 3 · src/app/inspection.rs"},{level:2,id:"check",text:"Check"},{level:2,id:"what-changed",text:"What changed"},{level:2,id:"next",text:"Next"},{level:2,id:"expected-viewer-result",text:"Expected viewer result"}]};export{s as default};
