const s={title:"08 · Trims, holes and periodic seams",html:`<h1 id="08-trims-holes-and-periodic-seams">08 · Trims, holes and periodic seams<a class="anchor" href="#/course/08-trimming#08-trims-holes-and-periodic-seams" aria-label="Link to this section">#</a></h1>
<p>A trimmed patch has an empty hole and a torus keeps its periodic seams attached.</p>
<p><img src="/session/docs/course/docs/illustrations/trims-seams.svg" alt="Left: outer and inner loops select the face in u,v and the hole stays empty. Right: a cylinder's seam is one XYZ curve used at u=0 and u=1." loading="lazy" decoding="async"></p>
<h2 id="step-1-srcappwalkbreprs">Step 1 · src/app/walk/brep.rs<a class="anchor" href="#/course/08-trimming#step-1-srcappwalkbreprs" aria-label="Link to this section">#</a></h2>
<p>The BRep walk uses the cached trim mesh when the surface has one.</p>
<p><code>lessons/08/src/app/walk/brep.rs</code> · edit · type this</p>
<p>Replaces the line <code>use session_rust::{BRep, Color, NurbsSurface, RenderMesh};</code> in <code>lessons/07/src/app/walk/brep.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> session_rust::{BRep, Color, Mesh, NurbsSurface, RenderMesh};</code></pre></div>
<p><code>lessons/08/src/app/walk/brep.rs</code> · edit · type this</p>
<p>Replaces the <code>fn walk_surface</code> lines in <code>lessons/07/src/app/walk/brep.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// A NURBS surface is a smooth maths surface, so it has no triangles until it is sampled on a grid of u,v values.</span>
<span class="k">pub</span> <span class="k">fn</span> walk_surface(arena: &amp;<span class="k">mut</span> ArenaRows, ink: &amp;<span class="k">mut</span> Ink, s: &amp;NurbsSurface, cx: &amp;WalkCx) -&gt; Row {
    <span class="k">let</span> <span class="k">mut</span> sm = <span class="k">if</span> <span class="k">let</span> Some(mesh) = &amp;s.m_mesh {
        mesh.clone()
    } <span class="k">else</span> {
        RemeshNurbsSurfaceGrid::from_u_v_q(s, <span class="s">0</span>, <span class="s">0</span>, QUALITY.<span class="s">0</span>, QUALITY.<span class="s">1</span>)
    };

    <span class="k">if</span> <span class="k">let</span> Some(c) = s.facecolors.first() {
        sm.set_objectcolor(c.clone());
    }

    <span class="k">let</span> first_pipe = ink.seg.pipes.len();
    <span class="k">let</span> <span class="k">mut</span> row = walk_mesh(
        arena,
        ink,
        &amp;sm,
        &amp;MeshCx {
            cx,
            opts: &amp;MeshOpts::SURFACE,
        },
    );
    row.flags |= Instance::FLAG_SINGLE;
    map_surface_boundaries(ink, s, &amp;sm, first_pipe);
    row
}

<span class="c">/// Name the four UV border edges.</span>
<span class="k">fn</span> map_surface_boundaries(ink: &amp;<span class="k">mut</span> Ink, s: &amp;NurbsSurface, mesh: &amp;Mesh, first_pipe: usize) {
    <span class="k">let</span> <span class="k">mut</span> masks = std::collections::HashMap::&lt;[u32; 3], u8&gt;::new();

    <span class="k">for</span> vertex <span class="k">in</span> mesh.vertex.values() {
        <span class="k">let</span> <span class="k">mut</span> mask = <span class="s">0u8</span>;

        <span class="k">for</span> (direction, name) <span class="k">in</span> [(<span class="s">0</span>, &quot;<span class="s">u</span>&quot;), (<span class="s">1</span>, &quot;<span class="s">v</span>&quot;)] {
            <span class="k">if</span> s.is_closed(direction) {
                <span class="k">continue</span>;
            }

            <span class="k">let</span> (Some(&amp;parameter), Some((start, end))) =
                (vertex.attributes.get(name), s.domain(direction))
            <span class="k">else</span> {
                <span class="k">continue</span>;
            };

            <span class="k">if</span> parameter == start {
                mask |= <span class="s">1</span> &lt;&lt; (direction * <span class="s">2</span>);
            }

            <span class="k">if</span> parameter == end {
                mask |= <span class="s">1</span> &lt;&lt; (direction * <span class="s">2</span> + <span class="s">1</span>);
            }
        }

        <span class="k">let</span> position = [vertex.x <span class="k">as</span> f32, vertex.y <span class="k">as</span> f32, vertex.z <span class="k">as</span> f32];
        *masks.entry(position.map(f32::to_bits)).or_insert(<span class="s">0</span>) |= mask;
    }

    ink.seg.pipe_ids.resize(ink.seg.pipes.len(), u32::MAX);

    <span class="k">for</span> index <span class="k">in</span> first_pipe..ink.seg.pipes.len() {
        <span class="k">let</span> pipe = &amp;ink.seg.pipes[index];
        <span class="k">let</span> a = masks.get(&amp;pipe.p0.map(f32::to_bits)).copied().unwrap_or(<span class="s">0</span>);
        <span class="k">let</span> b = masks.get(&amp;pipe.p1.map(f32::to_bits)).copied().unwrap_or(<span class="s">0</span>);
        <span class="k">let</span> common = a &amp; b;
        ink.seg.pipe_ids[index] = <span class="k">if</span> common.count_ones() == <span class="s">1</span> {
            common.trailing_zeros()
        } <span class="k">else</span> {
            u32::MAX
        };
    }
}

#[cfg(test)]
<span class="k">mod</span> tests {
    <span class="k">use</span> super::*;
    <span class="k">use</span> <span class="k">crate</span>::app::walk::brep_edges::edge_chains;
    <span class="k">use</span> <span class="k">crate</span>::engine::gpu::Instance;
    <span class="k">use</span> <span class="k">crate</span>::engine::gpu::glyphs::GlyphRows;
    <span class="k">use</span> <span class="k">crate</span>::engine::gpu::segments::SegRows;

    <span class="c">/// Walk one BRep at row 5.</span>
    <span class="k">fn</span> walked(b: &amp;BRep) -&gt; (ArenaRows, SegRows, GlyphRows, Row) {
        <span class="k">let</span> <span class="k">mut</span> arena = ArenaRows::default();
        <span class="k">let</span> <span class="k">mut</span> seg = SegRows::default();
        <span class="k">let</span> <span class="k">mut</span> glyph = GlyphRows::default();
        <span class="k">let</span> cx = WalkCx {
            vert_base: <span class="s">100</span>,
            cloud_px: <span class="s">0</span>.<span class="s">0</span>,
            row: <span class="s">5</span>,
        };
        <span class="k">let</span> row = {
            <span class="k">let</span> <span class="k">mut</span> ink = Ink {
                seg: &amp;<span class="k">mut</span> seg,
                glyph: &amp;<span class="k">mut</span> glyph,
            };
            walk_brep(&amp;<span class="k">mut</span> arena, &amp;<span class="k">mut</span> ink, b, &amp;cx)
        };
        (arena, seg, glyph, row)
    }

    <span class="c">/// A cylinder uploads unwelded faces with unit normals.</span>
    #[test]
    <span class="k">fn</span> cylinder_walks_unwelded_with_normals() {
        <span class="k">let</span> b = BRep::create_cylinder(<span class="s">150</span>.<span class="s">0</span>, <span class="s">400</span>.<span class="s">0</span>);
        <span class="k">let</span> fms = b.face_meshes_q(Some(QUALITY));
        <span class="k">let</span> verts: usize = fms.iter().map(|m| m.vertex.len()).sum();
        <span class="k">let</span> tris: usize = fms.iter().map(|m| m.to_render().indices.len()).sum();
        <span class="k">let</span> segments: usize = edge_chains(&amp;b, &amp;fms)
            .iter()
            .flatten()
            .map(|c| c.keys.len() - <span class="s">1</span>)
            .sum();
        <span class="k">let</span> (arena, seg, glyph, row) = walked(&amp;b);
        assert_eq!(arena.verts.len(), verts);
        assert_eq!(arena.vids.len(), verts);
        assert_eq!(arena.idx.len(), tris);
        assert!(
            arena
                .idx
                .iter()
                .all(|&amp;i| i &gt;= <span class="s">100</span> &amp;&amp; i &lt; <span class="s">100</span> + verts <span class="k">as</span> u32)
        );

        <span class="k">for</span> v <span class="k">in</span> &amp;arena.verts {
            <span class="k">let</span> n = v.normal;
            <span class="k">let</span> l = (n[<span class="s">0</span>] * n[<span class="s">0</span>] + n[<span class="s">1</span>] * n[<span class="s">1</span>] + n[<span class="s">2</span>] * n[<span class="s">2</span>]).sqrt();
            assert!((l - <span class="s">1</span>.<span class="s">0</span>).abs() &lt; <span class="s">1</span>e-<span class="s">3</span>, &quot;<span class="s">normal </span>{<span class="s">n:?</span>}&quot;);
        }

        assert_eq!(seg.pipes.len(), segments);
        assert!(seg.ribbons.is_empty());
        assert!(glyph.spheres.is_empty());
        assert_ne!(row.flags &amp; Instance::FLAG_SMOOTH, <span class="s">0</span>);
        assert_eq!(row.flags &amp; Instance::FLAG_OPEN, <span class="s">0</span>);
        assert!(row.faces);
        assert!(row.bounds.diagonal() &gt; <span class="s">400</span>.<span class="s">0</span>);
    }

    <span class="c">/// A shell without a solid is flagged open.</span>
    #[test]
    <span class="k">fn</span> open_brep_is_flagged_open() {
        <span class="k">let</span> <span class="k">mut</span> b = BRep::create_box(<span class="s">400</span>.<span class="s">0</span>, <span class="s">300</span>.<span class="s">0</span>, <span class="s">250</span>.<span class="s">0</span>);
        b.m_solids.clear();
        <span class="k">let</span> (_, _, _, row) = walked(&amp;b);
        assert_ne!(row.flags &amp; Instance::FLAG_OPEN, <span class="s">0</span>);
    }

    <span class="c">/// Flipped face uses upload the same vertices and triangles.</span>
    #[test]
    <span class="k">fn</span> reversed_solid_uses_repair_faces_and_boundaries_together() {
        <span class="k">let</span> b = BRep::create_cylinder(<span class="s">150</span>.<span class="s">0</span>, <span class="s">400</span>.<span class="s">0</span>);
        <span class="k">let</span> <span class="k">mut</span> reversed = b.clone();

        <span class="k">for</span> face <span class="k">in</span> reversed.m_shells[<span class="s">0</span>].faces.iter_mut().take(<span class="s">2</span>) {
            face.orientation = session_rust::brep::brep_reverse(face.orientation);
        }

        <span class="k">let</span> (expected, _, _, _) = walked(&amp;b);
        <span class="k">let</span> (actual, _, _, _) = walked(&amp;reversed);
        assert_eq!(actual.idx.len(), expected.idx.len());

        <span class="k">for</span> (a, b) <span class="k">in</span> actual.idx.chunks_exact(<span class="s">3</span>).zip(expected.idx.chunks_exact(<span class="s">3</span>)) {
            assert!(a == b || a == [b[<span class="s">1</span>], b[<span class="s">2</span>], b[<span class="s">0</span>]] || a == [b[<span class="s">2</span>], b[<span class="s">0</span>], b[<span class="s">1</span>]]);
        }

        assert_eq!(actual.verts.len(), expected.verts.len());

        <span class="k">for</span> (a, b) <span class="k">in</span> actual.verts.iter().zip(&amp;expected.verts) {
            assert_eq!(a.position, b.position);
            assert_eq!(a.normal, b.normal);
        }
    }

    <span class="c">/// A surface's four border edges carry ids 0 to 3.</span>
    #[test]
    <span class="k">fn</span> surface_domain_edges_have_source_ids_and_open_visibility() {
        <span class="k">let</span> b = BRep::create_box(<span class="s">40</span>.<span class="s">0</span>, <span class="s">30</span>.<span class="s">0</span>, <span class="s">25</span>.<span class="s">0</span>);
        <span class="k">let</span> <span class="k">mut</span> arena = ArenaRows::default();
        <span class="k">let</span> <span class="k">mut</span> seg = SegRows::default();
        <span class="k">let</span> <span class="k">mut</span> glyph = GlyphRows::default();
        <span class="k">let</span> <span class="k">mut</span> ink = Ink {
            seg: &amp;<span class="k">mut</span> seg,
            glyph: &amp;<span class="k">mut</span> glyph,
        };
        <span class="k">let</span> cx = WalkCx {
            vert_base: <span class="s">0</span>,
            cloud_px: <span class="s">0</span>.<span class="s">0</span>,
            row: <span class="s">7</span>,
        };
        <span class="k">let</span> row = walk_surface(&amp;<span class="k">mut</span> arena, &amp;<span class="k">mut</span> ink, &amp;b.m_surfaces[<span class="s">0</span>], &amp;cx);
        assert_ne!(row.flags &amp; Instance::FLAG_OPEN, <span class="s">0</span>);
        assert_eq!(seg.pipe_ids.len(), seg.pipes.len());
        <span class="k">let</span> <span class="k">mut</span> ids = seg.pipe_ids.clone();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids, vec![<span class="s">0</span>, <span class="s">1</span>, <span class="s">2</span>, <span class="s">3</span>]);
    }

    <span class="c">/// Sphere poles and box corners keep unit normals.</span>
    #[test]
    <span class="k">fn</span> poles_and_sharp_faces_keep_finite_unit_normals() {
        <span class="k">for</span> b <span class="k">in</span> [
            BRep::create_sphere(<span class="s">180</span>.<span class="s">0</span>),
            BRep::create_cone(<span class="s">150</span>.<span class="s">0</span>, <span class="s">400</span>.<span class="s">0</span>),
            BRep::create_box(<span class="s">40</span>.<span class="s">0</span>, <span class="s">30</span>.<span class="s">0</span>, <span class="s">25</span>.<span class="s">0</span>),
        ] {
            <span class="k">let</span> (arena, seg, _, _) = walked(&amp;b);

            <span class="k">for</span> vertex <span class="k">in</span> &amp;arena.verts {
                <span class="k">let</span> [x, y, z] = vertex.normal;
                assert!(
                    (x * x + y * y + z * z - <span class="s">1</span>.<span class="s">0</span>).abs() &lt; <span class="s">1</span>e-<span class="s">5</span>,
                    &quot;{}<span class="s"> normal </span>{<span class="s">:?</span>}&quot;,
                    b.name,
                    vertex.normal
                );
                assert!(vertex.position.iter().all(|value| value.is_finite()));
            }

            <span class="k">for</span> pipe <span class="k">in</span> &amp;seg.pipes {
                assert_ne!(pipe.p0, pipe.p1);
            }
        }

        <span class="k">let</span> (arena, _, _, _) = walked(&amp;BRep::create_box(<span class="s">40</span>.<span class="s">0</span>, <span class="s">30</span>.<span class="s">0</span>, <span class="s">25</span>.<span class="s">0</span>));

        <span class="k">for</span> vertex <span class="k">in</span> &amp;arena.verts {
            assert_eq!(
                vertex
                    .normal
                    .iter()
                    .filter(|component| component.abs() &gt; <span class="s">0</span>.<span class="s">0</span>)
                    .count(),
                <span class="s">1</span>
            );
        }
    }</code></pre></div>
<p>Run <code>cargo check</code> in <code>lessons/08/</code>.</p>
<h2 id="step-2-srcfixturers">Step 2 · src/fixture.rs<a class="anchor" href="#/course/08-trimming#step-2-srcfixturers" aria-label="Link to this section">#</a></h2>
<p>Copy the test scene: a trimmed patch and a torus.
Copy this file from the lesson folder to the path shown.</p>
<p><code>lessons/08/src/fixture.rs</code> · edit · copy the file</p>
<p>Replaces the line <code>use session_rust::{BRep, Color, Geometry, Xform};</code> in <code>lessons/07/src/fixture.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> session_rust::{BRep, Color, Geometry, NurbsSurface, Point, Xform};</code></pre></div>
<p>Replaces the lines from <code>let mut cylinder = BRep::create_cylinder(100.0, 220.0);</code> to <code>Xform::translation(220.0, 0.0, 0.0),</code> in <code>lessons/07/src/fixture.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// A curved patch with a hole, mesh cached.</span>
<span class="k">fn</span> trimmed_surface() -&gt; NurbsSurface {
    <span class="k">use</span> session_rust::{NurbsSurfaceTrimmed, TrimLoops};
    <span class="k">let</span> points = [
        Point::new(-<span class="s">200</span>.<span class="s">0</span>, -<span class="s">150</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>),
        Point::new(-<span class="s">200</span>.<span class="s">0</span>, <span class="s">150</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>),
        Point::new(<span class="s">0</span>.<span class="s">0</span>, -<span class="s">150</span>.<span class="s">0</span>, <span class="s">160</span>.<span class="s">0</span>),
        Point::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">150</span>.<span class="s">0</span>, <span class="s">160</span>.<span class="s">0</span>),
        Point::new(<span class="s">200</span>.<span class="s">0</span>, -<span class="s">150</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>),
        Point::new(<span class="s">200</span>.<span class="s">0</span>, <span class="s">150</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>),
    ];
    <span class="k">let</span> surface = NurbsSurface::create(<span class="s">false</span>, <span class="s">false</span>, <span class="s">2</span>, <span class="s">1</span>, <span class="s">3</span>, <span class="s">2</span>, &amp;points).unwrap();
    <span class="k">let</span> <span class="k">mut</span> trimmed = NurbsSurfaceTrimmed::new();
    trimmed.m_surface = surface;
    <span class="k">let</span> <span class="k">mut</span> loops = TrimLoops::default();
    <span class="k">let</span> <span class="k">mut</span> outer = Vec::new();

    <span class="k">for</span> edge <span class="k">in</span> <span class="s">0</span>..<span class="s">4</span> {
        <span class="k">for</span> sample <span class="k">in</span> <span class="s">0</span>..<span class="s">24</span> {
            <span class="k">let</span> t = sample <span class="k">as</span> f64 / <span class="s">24</span>.<span class="s">0</span>;
            <span class="k">let</span> (u, v) = <span class="k">match</span> edge {
                <span class="s">0</span> =&gt; (t, <span class="s">0</span>.<span class="s">0</span>),
                <span class="s">1</span> =&gt; (<span class="s">1</span>.<span class="s">0</span>, t),
                <span class="s">2</span> =&gt; (<span class="s">1</span>.<span class="s">0</span> - t, <span class="s">1</span>.<span class="s">0</span>),
                _ =&gt; (<span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span> - t),
            };
            outer.push(Point::new(u, v, <span class="s">0</span>.<span class="s">0</span>));
        }
    }

    <span class="k">let</span> <span class="k">mut</span> hole = Vec::new();

    <span class="k">for</span> sample <span class="k">in</span> <span class="s">0</span>..<span class="s">48</span> {
        <span class="k">let</span> angle = sample <span class="k">as</span> f64 / <span class="s">48</span>.<span class="s">0</span> * std::f64::consts::TAU;
        hole.push(Point::new(
            <span class="s">0</span>.<span class="s">5</span> + <span class="s">0</span>.<span class="s">2</span> * angle.cos(),
            <span class="s">0</span>.<span class="s">5</span> + <span class="s">0</span>.<span class="s">2</span> * angle.sin(),
            <span class="s">0</span>.<span class="s">0</span>,
        ));
    }

    loops.uv = vec![outer, hole];

    <span class="k">for</span> ring <span class="k">in</span> &amp;loops.uv {
        <span class="k">let</span> <span class="k">mut</span> positions = Vec::with_capacity(ring.len());

        <span class="k">for</span> uv <span class="k">in</span> ring {
            positions.push(
                trimmed
                    .m_surface
                    .point_at(uv[<span class="s">0</span>], uv[<span class="s">1</span>])
                    .expect(&quot;<span class="s">fixture UV lies in the source domain</span>&quot;),
            );
        }

        loops.xyz.push(positions);
    }

    <span class="k">let</span> mesh = trimmed.mesh_loops(&amp;loops, <span class="s">20</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">005</span>);
    assert!(
        !mesh.face.is_empty(),
        &quot;<span class="s">the local constrained hole must triangulate</span>&quot;
    );
    <span class="k">let</span> <span class="k">mut</span> surface = trimmed.m_surface;
    surface.m_mesh = Some(mesh);
    surface.facecolors = vec![Color::grey()];
    surface.name = &quot;<span class="s">cached curved trim with circular hole</span>&quot;.into();
    surface
}

<span class="c">/// Build the first source-face checkpoint entirely from local geometry.</span>
<span class="k">pub</span> <span class="k">fn</span> build() -&gt; CadFixture {
    <span class="k">let</span> <span class="k">mut</span> scene = CadFixture::new();
    scene.add(
        Geometry::NurbsSurface(Rc::new(trimmed_surface())),
        Xform::translation(-<span class="s">290</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>),
    );
    <span class="k">let</span> <span class="k">mut</span> torus = BRep::create_torus(<span class="s">120</span>.<span class="s">0</span>, <span class="s">35</span>.<span class="s">0</span>);
    torus.surfacecolor = Color::grey();
    scene.add(
        Geometry::BRep(Rc::new(torus)),
        Xform::translation(<span class="s">290</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>),</code></pre></div>
<h2 id="step-3-srclibrs">Step 3 · src/lib.rs<a class="anchor" href="#/course/08-trimming#step-3-srclibrs" aria-label="Link to this section">#</a></h2>
<p>The entry point reports the trimmed faces.</p>
<p><code>lessons/08/src/lib.rs</code> · edit · type this</p>
<p>Replaces the line <code>Ok(serde_json::json!({&quot;stage&quot;:7,&quot;objects&quot;:self.gpu.object…</code> in <code>lessons/07/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        Ok(serde_json::json!({&quot;<span class="s">stage</span>&quot;:<span class="s">8</span>,&quot;<span class="s">objects</span>&quot;:<span class="k">self</span>.gpu.objects.len(),&quot;<span class="s">width</span>&quot;:w,&quot;<span class="s">height</span>&quot;:h,&quot;<span class="s">scale</span>&quot;:scale,&quot;<span class="s">drawn</span>&quot;:<span class="s">true</span>,&quot;<span class="s">sourceObjects</span>&quot;:<span class="k">self</span>.fixture.identities,&quot;<span class="s">sourceEdgeIds</span>&quot;:<span class="k">self</span>.fixture.pipe_source_edges,&quot;<span class="s">samples</span>&quot;:<span class="k">self</span>.gpu.targets.samples,</code></pre></div>
<h2 id="step-4-indexhtml">Step 4 · index.html<a class="anchor" href="#/course/08-trimming#step-4-indexhtml" aria-label="Link to this section">#</a></h2>
<p>Copy the page: the status says checkpoint 08.
Copy this file from the lesson folder to the path shown.</p>
<p><code>lessons/08/index.html</code> · edit · copy the file</p>
<p>Replaces the line <code>&lt;title&gt;Session checkpoint 07&lt;/title&gt;</code> in <code>lessons/07/index.html</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    &lt;title&gt;Session checkpoint 08&lt;/title&gt;</code></pre></div>
<p>Replaces the line <code>&lt;output id=&quot;status&quot;&gt;Starting checkpoint 07&lt;/output&gt;</code> in <code>lessons/07/index.html</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    &lt;output id=&quot;<span class="s">status</span>&quot;&gt;Starting checkpoint 08&lt;/output&gt;</code></pre></div>
<p>Replaces the line <code>document.getElementById(&#39;status&#39;).textContent = &#39;Checkpoi…</code> in <code>lessons/07/index.html</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        document.getElementById('status').textContent = 'Checkpoint 08 · ' + state.objects + ' objects · ' + state.width + '×' + state.height;</code></pre></div>
<h2 id="check">Check<a class="anchor" href="#/course/08-trimming#check" aria-label="Link to this section">#</a></h2>
<p>Run <code>trunk serve</code> in <code>lessons/08/</code> and open <a href="http://127.0.0.1:8770/" target="_blank" rel="noopener">http://127.0.0.1:8770/</a>.</p>
<p>Expected: A trimmed patch has an empty hole and a torus keeps its periodic seams attached; status: <strong>2 objects</strong>.</p>
<p><img src="/session/docs/course/docs/screenshots/08.png" alt="Checkpoint 08: a trimmed patch with its hole left empty and a torus whose seams are drawn once." loading="lazy" decoding="async"></p>
<p>If it fails:</p>
<ul>
<li>A periodic boundary crosses the patch: its UV branch or oriented use mapping is wrong.</li>
<li>The hole stays filled: trimming changes ink without removing covered triangles.</li>
</ul>
<h2 id="what-changed">What changed<a class="anchor" href="#/course/08-trimming#what-changed" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>lessons/08/src/app/
├── walk/
│   ├── bounds.rs
│   ├── brep.rs  ~
│   ├── brep_edges.rs
│   ├── brep_orient.rs
│   ├── curves.rs
│   ├── encode.rs
│   ├── mesh.rs
│   ├── mesh_ink.rs
│   ├── mesh_topology.rs
│   └── mod.rs
├── knobs.rs
├── mod.rs
└── route.rs</code></pre></div>
<p><code>+</code> new in this lesson · <code>~</code> changed in this lesson</p>
<p>Data flow: source files → retained scene state → GPU buffers → visible result. Every file at this point: <code>lessons/08/</code>.</p>
<h2 id="next">Next<a class="anchor" href="#/course/08-trimming#next" aria-label="Link to this section">#</a></h2>
<p><a href="#/course/09-normals">09 · Normals and shading</a></p>
<h2 id="expected-viewer-result">Expected viewer result<a class="anchor" href="#/course/08-trimming#expected-viewer-result" aria-label="Link to this section">#</a></h2>
<p>Checkpoint 08: a trimmed patch with its hole left empty and a torus whose seams are drawn once.</p>
<p><a href="/session/docs/course/docs/screenshots/08.png"><img src="/session/docs/course/docs/screenshots/08.png" alt="Full viewer result for 08 trimming" loading="lazy" decoding="async"></a></p>
`,toc:[{level:2,id:"step-1-srcappwalkbreprs",text:"Step 1 · src/app/walk/brep.rs"},{level:2,id:"step-2-srcfixturers",text:"Step 2 · src/fixture.rs"},{level:2,id:"step-3-srclibrs",text:"Step 3 · src/lib.rs"},{level:2,id:"step-4-indexhtml",text:"Step 4 · index.html"},{level:2,id:"check",text:"Check"},{level:2,id:"what-changed",text:"What changed"},{level:2,id:"next",text:"Next"},{level:2,id:"expected-viewer-result",text:"Expected viewer result"}]};export{s as default};
