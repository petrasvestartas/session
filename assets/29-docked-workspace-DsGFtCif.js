const s={title:"29 · Dock the workspace, edit source geometry and save",html:`<h1 id="29-dock-the-workspace-edit-source-geometry-and-save">29 · Dock the workspace, edit source geometry and save<a class="anchor" href="#/course/29-docked-workspace#29-dock-the-workspace-edit-source-geometry-and-save" aria-label="Link to this section">#</a></h1>
<p>Commands dock below the viewport, selection tools support touch, and Save/Open retains editable source geometry.</p>
<h2 id="step-1-srcappcommandrs">Step 1 · src/app/command.rs<a class="anchor" href="#/course/29-docked-workspace#step-1-srcappcommandrs" aria-label="Link to this section">#</a></h2>
<p>Add Save and Open to the command vocabulary and argument checks.</p>
<p><code>lessons/29/src/app/command.rs</code> · edit · type this</p>
<p>Added after the line <code>Scale(f64),</code> in <code>lessons/28/src/app/command.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    Save,       <span class="c">// download the scene</span>
    Open,       <span class="c">// load a scene file</span></code></pre></div>
<p>Replaces the line <code>&quot;delete&quot; | &quot;del&quot; | &quot;undo&quot; | &quot;redo&quot; | &quot;hide&quot; | &quot;show&quot; | &quot;f…</code> in <code>lessons/28/src/app/command.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        &quot;<span class="s">save</span>&quot; | &quot;<span class="s">open</span>&quot; | &quot;<span class="s">delete</span>&quot; | &quot;<span class="s">del</span>&quot; | &quot;<span class="s">undo</span>&quot; | &quot;<span class="s">redo</span>&quot; | &quot;<span class="s">hide</span>&quot; | &quot;<span class="s">show</span>&quot; | &quot;<span class="s">fit</span>&quot;
        | &quot;<span class="s">escape</span>&quot; | &quot;<span class="s">esc</span>&quot; =&gt; Some(<span class="s">0</span>),</code></pre></div>
<p>Added after the line <code>}</code> in <code>lessons/28/src/app/command.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        &quot;<span class="s">save</span>&quot; =&gt; Ok(Command::Save),
        &quot;<span class="s">open</span>&quot; =&gt; Ok(Command::Open),</code></pre></div>
<h2 id="step-2-srcappdeformrs">Step 2 · src/app/deform.rs<a class="anchor" href="#/course/29-docked-workspace#step-2-srcappdeformrs" aria-label="Link to this section">#</a></h2>
<p>Resolve selected source controls, edges and faces, then transform their shared geometry.</p>
<p><code>lessons/29/src/app/deform.rs</code> · 481 lines · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">//! Dragging one face, edge or control point instead of the whole object; the drag is applied in the object's own frame.</span>

<span class="k">use</span> super::selection::{ControlId, Controls, SelectionMode};
<span class="k">use</span> session_rust::{Geometry, Mesh, NurbsSurface, Point, Xform};
<span class="k">use</span> std::collections::HashSet;
<span class="k">use</span> std::rc::Rc;

<span class="c">// A drag has to know what it grabbed: the whole object, one face, one edge, or a single control point.</span>
#[derive(Clone, Copy, Debug)]
<span class="k">pub</span> <span class="k">enum</span> Target {
    Control(ControlId), <span class="c">// one control point or vertex</span>
    Edge(u32),          <span class="c">// one edge by index</span>
    Face(usize),        <span class="c">// one face by key</span>
}

<span class="k">impl</span> Target {
    <span class="c">/// The target the current selection names, if any.</span>
    <span class="k">pub</span> <span class="k">fn</span> selected(mode: &amp;SelectionMode) -&gt; Option&lt;<span class="k">Self</span>&gt; {
        <span class="k">match</span> *mode {
            SelectionMode::Controls {
                selected: Some(id), ..
            } =&gt; Some(<span class="k">Self</span>::Control(id)),
            SelectionMode::Edge { edge, .. } =&gt; Some(<span class="k">Self</span>::Edge(edge)),
            SelectionMode::Face { face, .. } =&gt; Some(<span class="k">Self</span>::Face(face)),
            _ =&gt; None,
        }
    }
}

<span class="c">/// The mesh vertices a deform target selects.</span>
<span class="k">fn</span> mesh_keys(mesh: &amp;Mesh, target: Target) -&gt; Result&lt;Vec&lt;usize&gt;, String&gt; {
    <span class="k">match</span> target {
        Target::Control(ControlId::Vertex(key)) <span class="k">if</span> mesh.vertex.contains_key(&amp;key) =&gt; Ok(vec![key]),
        Target::Face(key) =&gt; {
            <span class="k">let</span> <span class="k">mut</span> keys = mesh.face.get(&amp;key).ok_or(&quot;<span class="s">Unknown mesh face</span>&quot;)?.clone();

            <span class="k">if</span> <span class="k">let</span> Some(holes) = mesh.face_holes.get(&amp;key) {
                keys.extend(holes.iter().flatten()); <span class="c">// hole rims move with the face</span>
            }

            keys.sort_unstable();
            keys.dedup();
            Ok(keys)
        }
        Target::Edge(index) =&gt; {
            <span class="c">// edges are numbered in order of first appearance</span>
            <span class="k">let</span> <span class="k">mut</span> seen = HashSet::new();
            <span class="k">let</span> <span class="k">mut</span> at = <span class="s">0</span>;

            <span class="k">for</span> face <span class="k">in</span> mesh.faces() {
                <span class="k">let</span> keys = &amp;mesh.face[&amp;face];

                <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..keys.len() {
                    <span class="k">let</span> (a, b) = (keys[i], keys[(i + <span class="s">1</span>) % keys.len()]);
                    <span class="k">let</span> pair = (a.min(b), a.max(b));

                    <span class="k">if</span> seen.insert(pair) {
                        <span class="k">if</span> at == index {
                            <span class="k">return</span> Ok(vec![pair.<span class="s">0</span>, pair.<span class="s">1</span>]);
                        }

                        at += <span class="s">1</span>;
                    }
                }
            }

            Err(&quot;<span class="s">Unknown mesh edge</span>&quot;.into())
        }
        _ =&gt; Err(&quot;<span class="s">Select a mesh vertex, edge or face</span>&quot;.into()),
    }
}

<span class="c">/// The surface control (u, v) pairs a target covers.</span>
<span class="k">fn</span> surface_keys(surface: &amp;NurbsSurface, target: Target) -&gt; Result&lt;Vec&lt;(usize, usize)&gt;, String&gt; {
    <span class="k">let</span> [nu, nv] = surface.m_cv_count; <span class="c">// control grid size</span>
    <span class="k">let</span> all = || (<span class="s">0</span>..nu).flat_map(|u| (<span class="s">0</span>..nv).map(<span class="k">move</span> |v| (u, v))).collect();

    <span class="k">match</span> target {
        Target::Control(ControlId::Surface { surface: <span class="s">0</span>, u, v }) <span class="k">if</span> u &lt; nu &amp;&amp; v &lt; nv =&gt; {
            Ok(vec![(u, v)])
        }
        Target::Face(<span class="s">0</span>) =&gt; Ok(all()),
        <span class="c">// edge 0..3: the four sides of the control grid</span>
        Target::Edge(edge) <span class="k">if</span> edge &lt; <span class="s">4</span> =&gt; Ok((<span class="s">0</span>..nu)
            .flat_map(|u| (<span class="s">0</span>..nv).map(<span class="k">move</span> |v| (u, v)))
            .filter(|&amp;(u, v)| <span class="k">match</span> edge {
                <span class="s">0</span> =&gt; u == <span class="s">0</span>,
                <span class="s">1</span> =&gt; u + <span class="s">1</span> == nu,
                <span class="s">2</span> =&gt; v == <span class="s">0</span>,
                _ =&gt; v + <span class="s">1</span> == nv,
            })
            .collect()),
        _ =&gt; Err(&quot;<span class="s">Unknown surface control, boundary or face</span>&quot;.into()),
    }
}

<span class="c">/// The world points a target covers.</span>
<span class="k">pub</span> <span class="k">fn</span> points(geometry: &amp;Geometry, target: Target) -&gt; Result&lt;Vec&lt;Point&gt;, String&gt; {
    <span class="k">match</span> geometry {
        Geometry::Mesh(mesh) =&gt; mesh_keys(mesh, target)?
            .into_iter()
            .map(|k| mesh.vertex_point(k).ok_or(&quot;<span class="s">Missing mesh vertex</span>&quot;.into()))
            .collect(),
        Geometry::NurbsSurface(surface) =&gt; surface_keys(surface, target)?
            .into_iter()
            .map(|(u, v)| surface.get_cv(u, v).ok_or(&quot;<span class="s">Missing surface control</span>&quot;.into()))
            .collect(),
        Geometry::BRep(brep) =&gt; <span class="k">match</span> target {
            Target::Face(face) =&gt; {
                <span class="k">let</span> face = brep.m_faces.get(face).ok_or(&quot;<span class="s">Unknown BRep face</span>&quot;)?;
                <span class="k">let</span> surface = brep
                    .m_surfaces
                    .get(face.surface_index <span class="k">as</span> usize)
                    .ok_or(&quot;<span class="s">Missing face surface</span>&quot;)?;
                Ok((<span class="s">0</span>..surface.m_cv_count[<span class="s">0</span>])
                    .flat_map(|u| {
                        (<span class="s">0</span>..surface.m_cv_count[<span class="s">1</span>]).filter_map(<span class="k">move</span> |v| surface.get_cv(u, v))
                    })
                    .collect())
            }
            Target::Edge(edge) =&gt; {
                <span class="k">let</span> edge = brep.m_edges.get(edge <span class="k">as</span> usize).ok_or(&quot;<span class="s">Unknown BRep edge</span>&quot;)?;
                <span class="k">let</span> curve = brep
                    .m_curves_3d
                    .get(edge.curve_3d_index <span class="k">as</span> usize)
                    .ok_or(&quot;<span class="s">Degenerate edge has no movable curve</span>&quot;)?;
                Ok((<span class="s">0</span>..curve.cv_count())
                    .filter_map(|i| curve.get_cv(i))
                    .collect())
            }
            Target::Control(id) =&gt; control_point(geometry, id).map(|p| vec![p]),
        },
        Geometry::Element(element) =&gt; <span class="k">match</span> element.geometry() {
            session_rust::element::ElementGeometry::Mesh(mesh) =&gt; {
                points(&amp;Geometry::Mesh(Rc::new(mesh.clone())), target)
            }
            session_rust::element::ElementGeometry::BRep(brep) =&gt; {
                points(&amp;Geometry::BRep(Rc::new(brep.clone())), target)
            }
            _ =&gt; Err(&quot;<span class="s">Element has no source geometry</span>&quot;.into()),
        },
        _ =&gt; <span class="k">match</span> target {
            Target::Control(id) =&gt; control_point(geometry, id).map(|p| vec![p]),
            _ =&gt; Err(&quot;<span class="s">This source has no editable faces or edges</span>&quot;.into()),
        },
    }
}

<span class="c">/// One control point of any geometry.</span>
<span class="k">fn</span> control_point(geometry: &amp;Geometry, id: ControlId) -&gt; Result&lt;Point, String&gt; {
    Controls::from_geometry(geometry)
        .points
        .into_iter()
        .find(|p| p.id == id)
        .map(|p| Point::new(p.position[<span class="s">0</span>], p.position[<span class="s">1</span>], p.position[<span class="s">2</span>]))
        .ok_or(&quot;<span class="s">Unknown source control</span>&quot;.into())
}

<span class="c">/// A copy of the geometry with the target moved by \`delta\`.</span>
<span class="k">pub</span> <span class="k">fn</span> transform(geometry: &amp;Geometry, target: Target, delta: &amp;Xform) -&gt; Result&lt;Geometry, String&gt; {
    <span class="k">if</span> !delta.m.iter().all(|v| v.is_finite()) {
        <span class="k">return</span> Err(&quot;<span class="s">Transform must be finite</span>&quot;.into());
    }

    <span class="k">let</span> edited = <span class="k">match</span> geometry {
        Geometry::Mesh(source) =&gt; {
            <span class="k">let</span> <span class="k">mut</span> mesh = (**source).clone();

            <span class="k">for</span> key <span class="k">in</span> mesh_keys(&amp;mesh, target)? {
                <span class="k">let</span> point = mesh
                    .vertex_point(key)
                    .ok_or(&quot;<span class="s">Missing vertex</span>&quot;)?
                    .transformed(delta);
                mesh.vertex
                    .get_mut(&amp;key)
                    .ok_or(&quot;<span class="s">Missing vertex</span>&quot;)?
                    .set_position(point);
            }

            mesh.triangulation.clear();
            <span class="c">// The identity transform invalidates kernel render/BVH caches in both the frozen</span>
            mesh.transform(&amp;Xform::identity());
            Geometry::Mesh(Rc::new(mesh))
        }
        Geometry::NurbsSurface(source) =&gt; {
            <span class="k">let</span> <span class="k">mut</span> surface = (**source).clone();

            <span class="k">for</span> (u, v) <span class="k">in</span> surface_keys(&amp;surface, target)? {
                <span class="k">let</span> p = surface
                    .get_cv(u, v)
                    .ok_or(&quot;<span class="s">Missing surface control</span>&quot;)?
                    .transformed(delta);

                <span class="k">if</span> !surface.set_cv(u, v, &amp;p) {
                    <span class="k">return</span> Err(&quot;<span class="s">Cannot set surface control</span>&quot;.into());
                }
            }

            surface.m_mesh = None; <span class="c">// drop the cached mesh</span>
            Geometry::NurbsSurface(Rc::new(surface))
        }
        Geometry::Polyline(source) =&gt; {
            <span class="k">let</span> Target::Control(ControlId::Vertex(i)) = target <span class="k">else</span> {
                <span class="k">return</span> Err(&quot;<span class="s">Select a polyline vertex</span>&quot;.into());
            };
            <span class="k">let</span> p = source
                .get_point(i)
                .ok_or(&quot;<span class="s">Unknown polyline vertex</span>&quot;)?
                .transformed(delta);
            <span class="k">let</span> <span class="k">mut</span> next = (**source).clone();
            next.set_point(i, &amp;p);
            Geometry::Polyline(Rc::new(next))
        }
        Geometry::NurbsCurve(source) =&gt; {
            <span class="k">let</span> Target::Control(ControlId::Curve { curve: <span class="s">0</span>, point }) = target <span class="k">else</span> {
                <span class="k">return</span> Err(&quot;<span class="s">Select a curve control point</span>&quot;.into());
            };
            <span class="k">let</span> p = source
                .get_cv(point)
                .ok_or(&quot;<span class="s">Unknown curve control</span>&quot;)?
                .transformed(delta);
            <span class="k">let</span> <span class="k">mut</span> next = (**source).clone();

            <span class="k">if</span> !next.set_cv_point(point, &amp;p) {
                <span class="k">return</span> Err(&quot;<span class="s">Cannot set curve control</span>&quot;.into());
            }

            Geometry::NurbsCurve(Rc::new(next))
        }
        Geometry::Line(source) =&gt; {
            <span class="k">let</span> Target::Control(ControlId::Vertex(i @ <span class="s">0</span>..=<span class="s">1</span>)) = target <span class="k">else</span> {
                <span class="k">return</span> Err(&quot;<span class="s">Select a line endpoint</span>&quot;.into());
            };
            <span class="k">let</span> <span class="k">mut</span> ends = [source.start(), source.end()];
            ends[i] = ends[i].transformed(delta);
            <span class="k">let</span> <span class="k">mut</span> next = session_rust::Line::from_points(&amp;ends[<span class="s">0</span>], &amp;ends[<span class="s">1</span>]);
            next.set_guid(source.guid().to_string());
            next.name = source.name.clone();
            next.width = source.width;
            next.dash = source.dash.clone();
            next.linecolor = source.linecolor.clone();
            Geometry::Line(Rc::new(next))
        }
        Geometry::Point(source) =&gt; {
            <span class="k">let</span> Target::Control(ControlId::Vertex(<span class="s">0</span>)) = target <span class="k">else</span> {
                <span class="k">return</span> Err(&quot;<span class="s">Unknown point</span>&quot;.into());
            };
            Geometry::Point(Rc::new(source.transformed(delta)))
        }
        Geometry::BRep(source) =&gt; {
            <span class="k">let</span> selected = points(geometry, target)?;
            <span class="k">let</span> <span class="k">mut</span> next = (**source).clone();
            <span class="c">// every vertex, curve or surface point at a selected position moves</span>
            <span class="k">let</span> matches = |p: &amp;Point| {
                selected
                    .iter()
                    .any(|s| (<span class="s">0</span>..<span class="s">3</span>).all(|i| (s[i] - p[i]).abs() &lt;= <span class="s">1</span>e-<span class="s">8</span>))
            };

            <span class="k">for</span> vertex <span class="k">in</span> &amp;<span class="k">mut</span> next.m_vertices {
                <span class="k">if</span> matches(&amp;vertex.point) {
                    vertex.point = vertex.point.transformed(delta);
                }
            }

            <span class="k">for</span> curve <span class="k">in</span> &amp;<span class="k">mut</span> next.m_curves_3d {
                <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..curve.cv_count() {
                    <span class="k">if</span> <span class="k">let</span> Some(p) = curve.get_cv(i)
                        &amp;&amp; matches(&amp;p)
                    {
                        curve.set_cv_point(i, &amp;p.transformed(delta));
                    }
                }
            }

            <span class="k">for</span> surface <span class="k">in</span> &amp;<span class="k">mut</span> next.m_surfaces {
                <span class="k">for</span> u <span class="k">in</span> <span class="s">0</span>..surface.m_cv_count[<span class="s">0</span>] {
                    <span class="k">for</span> v <span class="k">in</span> <span class="s">0</span>..surface.m_cv_count[<span class="s">1</span>] {
                        <span class="k">if</span> <span class="k">let</span> Some(p) = surface.get_cv(u, v)
                            &amp;&amp; matches(&amp;p)
                        {
                            surface.set_cv(u, v, &amp;p.transformed(delta));
                        }
                    }
                }

                surface.m_mesh = None; <span class="c">// drop the cached mesh</span>
            }

            validate_boundaries(&amp;next)?;
            Geometry::BRep(Rc::new(next))
        }
        Geometry::Element(source) =&gt; {
            <span class="k">let</span> <span class="k">mut</span> next = (**source).clone();

            <span class="k">match</span> source.geometry() {
                session_rust::element::ElementGeometry::Mesh(mesh) =&gt; {
                    <span class="k">let</span> Geometry::Mesh(mesh) =
                        transform(&amp;Geometry::Mesh(Rc::new(mesh.clone())), target, delta)?
                    <span class="k">else</span> {
                        unreachable!()
                    };
                    next.set_geometry((*mesh).clone());
                }
                session_rust::element::ElementGeometry::BRep(brep) =&gt; {
                    <span class="k">let</span> Geometry::BRep(brep) =
                        transform(&amp;Geometry::BRep(Rc::new(brep.clone())), target, delta)?
                    <span class="k">else</span> {
                        unreachable!()
                    };
                    next.set_brep_geometry((*brep).clone());
                }
                _ =&gt; <span class="k">return</span> Err(&quot;<span class="s">Element has no source geometry</span>&quot;.into()),
            }

            Geometry::Element(Rc::new(next))
        }
        _ =&gt; <span class="k">return</span> Err(&quot;<span class="s">This source control cannot be transformed</span>&quot;.into()),
    };
    Ok(edited)
}

<span class="c">/// Keep the edge curve on every surface it touches.</span>
<span class="k">fn</span> validate_boundaries(brep: &amp;session_rust::BRep) -&gt; Result&lt;(), String&gt; {
    <span class="k">if</span> !brep.is_valid() {
        <span class="k">return</span> Err(&quot;<span class="s">Edit would invalidate BRep topology</span>&quot;.into());
    }

    <span class="k">for</span> edge <span class="k">in</span> &amp;brep.m_edges {
        <span class="k">if</span> edge.degenerated {
            <span class="k">continue</span>;
        }

        <span class="k">let</span> curve = &amp;brep.m_curves_3d[edge.curve_3d_index <span class="k">as</span> usize];
        <span class="k">let</span> (a, b) = curve.domain();

        <span class="c">// sample the 3D edge against each surface's 2D curve</span>
        <span class="k">for</span> pc <span class="k">in</span> &amp;edge.pcurves {
            <span class="k">let</span> surface = &amp;brep.m_surfaces[pc.surface_index <span class="k">as</span> usize];

            <span class="k">for</span> ci <span class="k">in</span> [pc.curve_2d_index, pc.curve_2d_index_2] {
                <span class="k">if</span> ci &lt; <span class="s">0</span> {
                    <span class="k">continue</span>;
                }

                <span class="k">let</span> uv = &amp;brep.m_curves_2d[ci <span class="k">as</span> usize];
                <span class="k">let</span> (u0, u1) = uv.domain();

                <span class="k">for</span> sample <span class="k">in</span> <span class="s">0</span>..=<span class="s">16</span> {
                    <span class="k">let</span> t = sample <span class="k">as</span> f64 / <span class="s">16</span>.<span class="s">0</span>;
                    <span class="k">let</span> p = curve.point_at(a + (b - a) * t);
                    <span class="k">let</span> q = uv.point_at(u0 + (u1 - u0) * t);
                    <span class="k">let</span> actual = surface
                        .point_at(q[<span class="s">0</span>], q[<span class="s">1</span>])
                        .ok_or(&quot;<span class="s">Cannot evaluate incident surface</span>&quot;)?;
                    <span class="k">let</span> tolerance = edge.tolerance.max(<span class="s">1</span>e-<span class="s">6</span>) * <span class="s">10</span>.<span class="s">0</span>;

                    <span class="k">if</span> (<span class="s">0</span>..<span class="s">3</span>).any(|i| (p[i] - actual[i]).abs() &gt; tolerance) {
                        <span class="k">return</span> Err(&quot;<span class="s">This BRep edit requires rebuilding adjacent trims; the original solid was preserved</span>&quot;.into());
                    }
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
<span class="k">mod</span> tests {
    <span class="k">use</span> super::*;

    <span class="c">/// A quad and a triangle sharing an edge.</span>
    <span class="k">fn</span> mesh() -&gt; Geometry {
        <span class="k">let</span> <span class="k">mut</span> mesh = Mesh::new();

        <span class="k">for</span> (key, p) <span class="k">in</span> [
            (<span class="s">10</span>, [<span class="s">0</span>., <span class="s">0</span>., <span class="s">0</span>.]),
            (<span class="s">20</span>, [<span class="s">10</span>., <span class="s">0</span>., <span class="s">0</span>.]),
            (<span class="s">30</span>, [<span class="s">10</span>., <span class="s">10</span>., <span class="s">0</span>.]),
            (<span class="s">40</span>, [<span class="s">0</span>., <span class="s">10</span>., <span class="s">0</span>.]),
            (<span class="s">90</span>, [<span class="s">0</span>., <span class="s">0</span>., <span class="s">10</span>.]),
        ] {
            mesh.add_vertex(Point::new(p[<span class="s">0</span>], p[<span class="s">1</span>], p[<span class="s">2</span>]), Some(key));
        }

        mesh.add_face(vec![<span class="s">10</span>, <span class="s">20</span>, <span class="s">30</span>, <span class="s">40</span>], Some(<span class="s">7</span>));
        mesh.add_face(vec![<span class="s">10</span>, <span class="s">90</span>, <span class="s">20</span>], Some(<span class="s">19</span>));
        Geometry::Mesh(Rc::new(mesh))
    }

    <span class="c">/// Moving a face moves its vertices only.</span>
    #[test]
    <span class="k">fn</span> mesh_face_moves_shared_source_vertices_not_unrelated_vertices() {
        <span class="k">let</span> source = mesh();
        <span class="k">let</span> Geometry::Mesh(next) =
            transform(&amp;source, Target::Face(<span class="s">7</span>), &amp;Xform::translation(<span class="s">0</span>., <span class="s">0</span>., <span class="s">3</span>.)).unwrap()
        <span class="k">else</span> {
            panic!()
        };

        <span class="k">for</span> key <span class="k">in</span> [<span class="s">10</span>, <span class="s">20</span>, <span class="s">30</span>, <span class="s">40</span>] {
            assert_eq!(next.vertex[&amp;key].z, <span class="s">3</span>.);
        }

        assert_eq!(next.vertex[&amp;<span class="s">90</span>].z, <span class="s">10</span>.);
        assert_eq!(next.face[&amp;<span class="s">19</span>], vec![<span class="s">10</span>, <span class="s">90</span>, <span class="s">20</span>]);
        <span class="k">let</span> Geometry::Mesh(original) = source <span class="k">else</span> {
            panic!()
        };
        assert_eq!(original.vertex[&amp;<span class="s">10</span>].z, <span class="s">0</span>.);
    }

    <span class="c">/// Edge numbering matches the display edges.</span>
    #[test]
    <span class="k">fn</span> edge_ids_match_the_display_producer_even_with_sparse_keys() {
        <span class="k">let</span> Geometry::Mesh(mesh) = mesh() <span class="k">else</span> {
            panic!()
        };
        <span class="k">let</span> keys = mesh.vertices();
        <span class="k">let</span> positions: Vec&lt;_&gt; = keys
            .iter()
            .map(|k| {
                <span class="k">let</span> p = mesh.vertex_point(*k).unwrap();
                [p[<span class="s">0</span>], p[<span class="s">1</span>], p[<span class="s">2</span>]]
            })
            .collect();
        <span class="k">let</span> slots = super::super::walk::mesh_topology::SlotMap::new(&amp;keys);
        <span class="k">let</span> topo =
            super::super::walk::mesh_topology::mesh_topology(&amp;mesh, &amp;keys, &amp;positions, &amp;slots);

        <span class="k">for</span> (i, &amp;(a, b, _)) <span class="k">in</span> topo.edges.iter().enumerate() {
            assert_eq!(
                mesh_keys(&amp;mesh, Target::Edge(i <span class="k">as</span> u32)).unwrap(),
                vec![a, b]
            );
        }
    }

    <span class="c">/// Moving one surface edge keeps weights and the other edge.</span>
    #[test]
    <span class="k">fn</span> surface_boundary_preserves_weights_and_the_opposite_boundary() {
        <span class="k">let</span> <span class="k">mut</span> surface = NurbsSurface::create(
            <span class="s">false</span>,
            <span class="s">false</span>,
            <span class="s">1</span>,
            <span class="s">1</span>,
            <span class="s">2</span>,
            <span class="s">2</span>,
            &amp;[
                Point::new(<span class="s">0</span>., <span class="s">0</span>., <span class="s">0</span>.),
                Point::new(<span class="s">1</span>., <span class="s">0</span>., <span class="s">0</span>.),
                Point::new(<span class="s">0</span>., <span class="s">1</span>., <span class="s">0</span>.),
                Point::new(<span class="s">1</span>., <span class="s">1</span>., <span class="s">0</span>.),
            ],
        )
        .unwrap();
        assert!(surface.to_rational());

        <span class="k">for</span> u <span class="k">in</span> <span class="s">0</span>..<span class="s">2</span> {
            <span class="k">for</span> v <span class="k">in</span> <span class="s">0</span>..<span class="s">2</span> {
                surface.set_cv_4d(u, v, u <span class="k">as</span> f64 * <span class="s">2</span>., v <span class="k">as</span> f64 * <span class="s">2</span>., <span class="s">0</span>., <span class="s">2</span>.);
            }
        }

        <span class="k">let</span> Geometry::NurbsSurface(next) = transform(
            &amp;Geometry::NurbsSurface(Rc::new(surface)),
            Target::Edge(<span class="s">0</span>),
            &amp;Xform::translation(<span class="s">0</span>., <span class="s">0</span>., <span class="s">5</span>.),
        )
        .unwrap() <span class="k">else</span> {
            panic!()
        };
        assert_eq!(next.get_cv(<span class="s">0</span>, <span class="s">0</span>).unwrap()[<span class="s">2</span>], <span class="s">5</span>.);
        assert_eq!(next.get_cv(<span class="s">1</span>, <span class="s">0</span>).unwrap()[<span class="s">2</span>], <span class="s">0</span>.);
        assert!(next.m_is_rat);
        assert_eq!(next.weight(<span class="s">0</span>, <span class="s">0</span>), <span class="s">2</span>.);
        assert_eq!(next.weight(<span class="s">1</span>, <span class="s">1</span>), <span class="s">2</span>.);
    }

    <span class="c">/// Moving a box face keeps the solid valid.</span>
    #[test]
    <span class="k">fn</span> moving_box_face_keeps_edges_on_incident_surfaces() {
        <span class="k">let</span> source = Geometry::BRep(Rc::new(session_rust::BRep::create_box(<span class="s">10</span>., <span class="s">20</span>., <span class="s">30</span>.)));
        <span class="k">let</span> Geometry::BRep(next) =
            transform(&amp;source, Target::Face(<span class="s">0</span>), &amp;Xform::translation(<span class="s">0</span>., <span class="s">0</span>., <span class="s">2</span>.)).unwrap()
        <span class="k">else</span> {
            panic!()
        };
        assert!(next.is_valid());
        assert!(next.is_closed(<span class="s">0</span>));
        validate_boundaries(&amp;next).unwrap();
    }
}</code></pre></div>
<h2 id="step-3-srcappeditrs">Step 3 · src/app/edit.rs<a class="anchor" href="#/course/29-docked-workspace#step-3-srcappeditrs" aria-label="Link to this section">#</a></h2>
<p>Add source replacement, component edits and temporary geometry previews.</p>
<p><code>lessons/29/src/app/edit.rs</code> · edit · type this</p>
<p>Added after the line <code>}</code> in <code>lessons/28/src/app/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">impl</span> Scene {
    <span class="c">/// Transform part of a row's geometry by a world \`delta\`.</span>
    <span class="k">pub</span> <span class="k">fn</span> edit_subobject(
        &amp;<span class="k">mut</span> <span class="k">self</span>,
        row: u32,
        target: super::deform::Target,
        delta: &amp;Xform,
        label: &amp;str,
    ) -&gt; Result&lt;(), String&gt; {
        <span class="k">let</span> place = <span class="k">self</span>
            .placement_of(row)
            .ok_or(&quot;<span class="s">Source placement unavailable</span>&quot;)?;
        <span class="k">let</span> back = place.inverse().ok_or(&quot;<span class="s">Source placement is singular</span>&quot;)?;
        <span class="k">let</span> local = &amp;(&amp;back * delta) * &amp;place; <span class="c">// delta in the object's frame</span>
        <span class="k">let</span> geometry = <span class="k">self</span>.geometry(row).ok_or(&quot;<span class="s">Source geometry unavailable</span>&quot;)?;
        <span class="k">let</span> edited = super::deform::transform(geometry, target, &amp;local)?;
        <span class="k">self</span>.commit_geometry(row, edited, label)
    }

    <span class="c">/// Replace a row's geometry in one undo step.</span>
    <span class="k">pub</span> <span class="k">fn</span> commit_geometry(
        &amp;<span class="k">mut</span> <span class="k">self</span>,
        row: u32,
        geometry: Geometry,
        label: &amp;str,
    ) -&gt; Result&lt;(), String&gt; {
        <span class="k">if</span> !<span class="k">self</span>.streamed.is_empty() || !<span class="k">self</span>.sheets.is_empty() {
            <span class="k">return</span> Err(&quot;<span class="s">Source edits require complete documents without streamed sources</span>&quot;.into());
        }

        <span class="k">let</span> (doc, guid) = <span class="k">self</span>.writable(row).ok_or(&quot;<span class="s">Source is not editable</span>&quot;)?;
        <span class="k">let</span> session = Rc::make_mut(&amp;<span class="k">mut</span> <span class="k">self</span>.docs[doc].session);
        session.begin(label);
        <span class="k">let</span> changed = session.replace(&amp;guid, geometry);
        session.commit();

        <span class="k">if</span> !changed {
            <span class="k">return</span> Err(&quot;<span class="s">Cannot replace source geometry</span>&quot;.into());
        }

        <span class="k">self</span>.last_edited = Some(doc);
        Ok(())
    }

    <span class="c">/// Move one control point of an object to a world point.</span>
    <span class="k">pub</span> <span class="k">fn</span> set_source_control(
        &amp;<span class="k">mut</span> <span class="k">self</span>,
        row: u32,
        id: super::selection::ControlId,
        to: &amp;Point,
    ) -&gt; Result&lt;(), String&gt; {
        <span class="k">let</span> geometry = <span class="k">self</span>.geometry(row).ok_or(&quot;<span class="s">Source geometry unavailable</span>&quot;)?;
        <span class="k">let</span> target = super::deform::Target::Control(id);
        <span class="k">let</span> point = super::deform::points(geometry, target)?
            .into_iter()
            .next()
            .ok_or(&quot;<span class="s">Source control unavailable</span>&quot;)?;
        <span class="k">let</span> place = <span class="k">self</span>
            .placement_of(row)
            .ok_or(&quot;<span class="s">Source placement unavailable</span>&quot;)?;
        <span class="k">let</span> point = point.transformed(&amp;place);
        <span class="k">self</span>.edit_subobject(
            row,
            target,
            &amp;Xform::translation(to[<span class="s">0</span>] - point[<span class="s">0</span>], to[<span class="s">1</span>] - point[<span class="s">1</span>], to[<span class="s">2</span>] - point[<span class="s">2</span>]),
            &quot;<span class="s">edit control</span>&quot;,
        )
    }

    <span class="c">/// Show a geometry on the GPU without changing the document.</span>
    <span class="k">pub</span> <span class="k">fn</span> preview_geometry(
        &amp;<span class="k">mut</span> <span class="k">self</span>,
        row: u32,
        geometry: Geometry,
        gpu: &amp;<span class="k">mut</span> <span class="k">crate</span>::engine::gpu::Gpu,
    ) -&gt; Result&lt;(), String&gt; {
        <span class="k">if</span> !<span class="k">self</span>.streamed.is_empty() || !<span class="k">self</span>.sheets.is_empty() {
            <span class="k">return</span> Err(&quot;<span class="s">Source edits require complete documents without streamed sources</span>&quot;.into());
        }

        <span class="k">let</span> (doc, guid) = <span class="k">self</span>.writable(row).ok_or(&quot;<span class="s">Source is not editable</span>&quot;)?;
        <span class="c">// swap in, rebuild the rows, swap back</span>
        <span class="k">let</span> original = Rc::make_mut(&amp;<span class="k">mut</span> <span class="k">self</span>.docs[doc].session)
            .lookup
            .insert(guid.to_string(), geometry)
            .ok_or(&quot;<span class="s">Source geometry unavailable</span>&quot;)?;
        <span class="k">self</span>.rebuild(gpu);
        Rc::make_mut(&amp;<span class="k">mut</span> <span class="k">self</span>.docs[doc].session)
            .lookup
            .insert(guid.to_string(), original);
        <span class="k">self</span>.selected = Some(row);
        Ok(())
    }
}</code></pre></div>
<h2 id="step-4-srcappgizmors">Step 4 · src/app/gizmo.rs<a class="anchor" href="#/course/29-docked-workspace#step-4-srcappgizmors" aria-label="Link to this section">#</a></h2>
<p>Allow a larger hit radius for touch without changing handle geometry.</p>
<p><code>lessons/29/src/app/gizmo.rs</code> · edit · type this</p>
<p>Added after the line <code>pub fn hit(&amp;self, from: &amp;Point, dir: &amp;Vector, world_per_p…</code> in <code>lessons/28/src/app/gizmo.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.hit_with_radius(from, dir, world_per_px, GRAB)
    }

    <span class="c">/// Handle under a ray with a custom grab radius.</span>
    <span class="k">pub</span> <span class="k">fn</span> hit_with_radius(
        &amp;<span class="k">self</span>,
        from: &amp;Point,
        dir: &amp;Vector,
        world_per_px: f64,
        radius: f64,
    ) -&gt; Option&lt;Handle&gt; {
        <span class="k">let</span> s = world_per_px; <span class="c">// pixel sizes to world</span>
        <span class="k">let</span> grab = radius.max(GRAB);</code></pre></div>
<p>Replaces the line <code>if within(from, dir, &amp;at, GRAB * s) {</code> in <code>lessons/28/src/app/gizmo.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">if</span> within(from, dir, &amp;at, grab * s) {</code></pre></div>
<p>Replaces the line <code>if (HUB * s..=ARM * s).contains(&amp;t) &amp;&amp; within(from, dir, …</code> in <code>lessons/28/src/app/gizmo.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                <span class="c">// from the hub's edge to the arm tip</span>
                <span class="k">if</span> (HUB * s..=ARM * s).contains(&amp;t) &amp;&amp; within(from, dir, &amp;p, grab * s) {</code></pre></div>
<p>Replaces the line <code>if d.dot(&amp;u) &lt; 0.0 &amp;&amp; d.dot(&amp;v) &lt; 0.0 &amp;&amp; (d.magnitude() -…</code> in <code>lessons/28/src/app/gizmo.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                <span class="k">if</span> d.dot(&amp;u) &lt; <span class="s">0</span>.<span class="s">0</span> &amp;&amp; d.dot(&amp;v) &lt; <span class="s">0</span>.<span class="s">0</span> &amp;&amp; (d.magnitude() - ARM * s).abs() &lt; grab * s</code></pre></div>
<h2 id="step-5-srcappinputrs">Step 5 · src/app/input.rs<a class="anchor" href="#/course/29-docked-workspace#step-5-srcappinputrs" aria-label="Link to this section">#</a></h2>
<p>Route touch gestures through control and gumball editing before camera navigation.</p>
<p><code>lessons/29/src/app/input.rs</code> · edit · type this</p>
<p>Added after the line <code>touch: Touches,</code> in <code>lessons/28/src/app/input.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    touch_edit: Option&lt;u64&gt;,                 <span class="c">// finger dragging a handle or control</span>
    fingers: std::collections::HashSet&lt;u64&gt;,
    touch_cancelled: bool,                   <span class="c">// waiting for all fingers to lift</span></code></pre></div>
<p>Added after the line <code>touch: Touches::new(),</code> in <code>lessons/28/src/app/input.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            touch_edit: None,
            fingers: std::collections::HashSet::new(),
            touch_cancelled: <span class="s">false</span>,</code></pre></div>
<p>Added after the line <code>};</code> in <code>lessons/28/src/app/input.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                <span class="k">if</span> t.phase == TouchPhase::Started {
                    <span class="k">self</span>.fingers.insert(t.id);
                }

                <span class="c">// a second finger cancels a one-finger edit</span>
                <span class="k">if</span> <span class="k">self</span>.touch_edit.is_some()
                    &amp;&amp; t.phase == TouchPhase::Started
                    &amp;&amp; <span class="k">self</span>.fingers.len() &gt; <span class="s">1</span>
                {
                    state.cancel_gesture();
                    <span class="k">self</span>.touch_edit = None;
                    <span class="k">self</span>.gizmo_drag = <span class="s">false</span>;
                    <span class="k">self</span>.control_drag = <span class="s">false</span>;
                    <span class="k">self</span>.touch_cancelled = <span class="s">true</span>;
                }

                <span class="c">// ignore everything until every finger lifts</span>
                <span class="k">if</span> <span class="k">self</span>.touch_cancelled {
                    <span class="k">if</span> matches!(t.phase, TouchPhase::Ended | TouchPhase::Cancelled) {
                        <span class="k">self</span>.fingers.remove(&amp;t.id);
                    }

                    <span class="k">if</span> <span class="k">self</span>.fingers.is_empty() {
                        <span class="k">self</span>.touch_cancelled = <span class="s">false</span>;
                        <span class="k">self</span>.touch = Touches::new();
                    }

                    state.interacting = <span class="s">false</span>;
                    <span class="k">return</span> <span class="s">true</span>;
                }

                <span class="c">// a first finger may grab a control or a handle</span>
                <span class="k">if</span> t.phase == TouchPhase::Started &amp;&amp; <span class="k">self</span>.fingers.len() == <span class="s">1</span> {
                    <span class="k">self</span>.last_cursor = (t.location.x, t.location.y);
                    <span class="k">self</span>.control_drag = state.begin_control_drag(t.location.x, t.location.y);
                    <span class="k">self</span>.gizmo_drag =
                        !<span class="k">self</span>.control_drag &amp;&amp; state.begin_gizmo_touch(t.location.x, t.location.y);

                    <span class="k">if</span> <span class="k">self</span>.control_drag || <span class="k">self</span>.gizmo_drag {
                        <span class="k">self</span>.touch_edit = Some(t.id);
                    }
                }

                <span class="c">// the editing finger</span>
                <span class="k">if</span> <span class="k">self</span>.touch_edit == Some(t.id) {
                    <span class="k">self</span>.last_cursor = (t.location.x, t.location.y);

                    <span class="k">match</span> t.phase {
                        TouchPhase::Moved =&gt; {
                            <span class="k">if</span> <span class="k">self</span>.control_drag {
                                state.drag_control(t.location.x, t.location.y);
                            } <span class="k">else</span> {
                                state.drag_gizmo(t.location.x, t.location.y);
                            }
                        }
                        TouchPhase::Ended =&gt; {
                            <span class="k">if</span> <span class="k">self</span>.control_drag {
                                state.end_control_drag(t.location.x, t.location.y);
                            } <span class="k">else</span> {
                                state.end_gizmo(t.location.x, t.location.y);
                            }
                        }
                        TouchPhase::Cancelled =&gt; state.cancel_gesture(),
                        TouchPhase::Started =&gt; {}
                    }

                    <span class="k">if</span> matches!(t.phase, TouchPhase::Ended | TouchPhase::Cancelled) {
                        <span class="k">self</span>.fingers.remove(&amp;t.id);
                        <span class="k">self</span>.touch_edit = None;
                        <span class="k">self</span>.control_drag = <span class="s">false</span>;
                        <span class="k">self</span>.gizmo_drag = <span class="s">false</span>;
                        <span class="k">self</span>.touch = Touches::new();
                    }

                    state.interacting = <span class="k">self</span>.touch_edit.is_some();
                    <span class="k">return</span> <span class="s">true</span>;
                }

                <span class="k">if</span> matches!(t.phase, TouchPhase::Ended | TouchPhase::Cancelled) {
                    <span class="k">self</span>.fingers.remove(&amp;t.id);
                }</code></pre></div>
<p>Added after the line <code>self.touch = Touches::new();</code> in <code>lessons/28/src/app/input.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.touch_edit = None;
        <span class="k">self</span>.fingers.clear();
        <span class="k">self</span>.touch_cancelled = <span class="s">false</span>;</code></pre></div>
<h2 id="step-6-srcapploaderrs">Step 6 · src/app/loader.rs<a class="anchor" href="#/course/29-docked-workspace#step-6-srcapploaderrs" aria-label="Link to this section">#</a></h2>
<p>Install a restored editable scene through the normal loading path.</p>
<p><code>lessons/29/src/app/loader.rs</code> · edit · type this</p>
<p>Added after the line <code>}</code> in <code>lessons/28/src/app/loader.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Show a scene opened from a \`.session\` file.</span>
<span class="k">pub</span>(super) <span class="k">fn</span> install_saved_scene(scene: super::scene::Scene) {
    LOAD_GENERATION.set(LOAD_GENERATION.get().wrapping_add(<span class="s">1</span>));
    GENERATION.set(GENERATION.get().wrapping_add(<span class="s">1</span>));
    RESIDENT.set(<span class="s">0</span>);
    SHEET_RESIDENT.set(<span class="s">0</span>);
    post(Msg::SavedScene(Box::new(scene)));
}</code></pre></div>
<h2 id="step-7-srcappmodrs">Step 7 · src/app/mod.rs<a class="anchor" href="#/course/29-docked-workspace#step-7-srcappmodrs" aria-label="Link to this section">#</a></h2>
<p>Declare the new application modules so their files join the crate.</p>
<p><code>lessons/29/src/app/mod.rs</code> · edit · type this</p>
<p>Added after the line <code>pub mod cplane;</code> in <code>lessons/28/src/app/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> <span class="k">mod</span> deform;</code></pre></div>
<p>Added after the line <code>pub mod selection;</code> in <code>lessons/28/src/app/mod.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">pub</span> <span class="k">mod</span> session_io;</code></pre></div>
<h2 id="step-8-srcappscene_textrs">Step 8 · src/app/scene_text.rs<a class="anchor" href="#/course/29-docked-workspace#step-8-srcappscene_textrs" aria-label="Link to this section">#</a></h2>
<p>Retain authored text while rebuilding the scene and registering its labels.</p>
<p><code>lessons/29/src/app/scene_text.rs</code> · edit · type this</p>
<p>Replaces the 2 lines from <code>pub(super) key: String,</code> in <code>lessons/28/src/app/scene_text.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span>(<span class="k">crate</span>) key: String,  <span class="c">// stable name, e.g. manifest-text/3</span>
    <span class="k">pub</span>(<span class="k">crate</span>) active: bool, <span class="c">// false once replaced</span></code></pre></div>
<p>Replaces the line <code>pub(super) fn register_text(&amp;mut self, key: String, mut l…</code> in <code>lessons/28/src/app/scene_text.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Reuse the row for \`key\`, or add a new one.</span>
    <span class="k">pub</span>(<span class="k">crate</span>) <span class="k">fn</span> register_text(&amp;<span class="k">mut</span> <span class="k">self</span>, key: String, <span class="k">mut</span> label: TextLabel, active: bool) {</code></pre></div>
<h2 id="step-9-srcappselectionrs">Step 9 · src/app/selection.rs<a class="anchor" href="#/course/29-docked-workspace#step-9-srcappselectionrs" aria-label="Link to this section">#</a></h2>
<p>Add explicit object, edge and face selection tools.</p>
<p><code>lessons/29/src/app/selection.rs</code> · edit · type this</p>
<p>Added after the line <code>}</code> in <code>lessons/28/src/app/selection.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// What a click picks.</span>
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
<span class="k">pub</span> <span class="k">enum</span> SelectionTool {
    #[default]
    Object,
    Edge,
    Face,
}</code></pre></div>
<h2 id="step-10-srcappsession_iors">Step 10 · src/app/session_io.rs<a class="anchor" href="#/course/29-docked-workspace#step-10-srcappsession_iors" aria-label="Link to this section">#</a></h2>
<p>Save retained sessions, placements and authored text into one archive and restore them on Open.</p>
<p><code>lessons/29/src/app/session_io.rs</code> · 261 lines · type this, new file</p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> super::scene::{FileDoc, Scene};
<span class="k">use</span> <span class="k">crate</span>::engine::text::TextLabel;
<span class="k">use</span> prost::Message;
<span class="k">use</span> serde::{Deserialize, Serialize};
<span class="k">use</span> session_rust::{Session, Xform};
<span class="k">use</span> std::rc::Rc;

<span class="k">const</span> MAGIC: &amp;[u8] = <span class="s">b</span>&quot;<span class="s">SESSION-VIEWER\\x01\\n</span>&quot;; <span class="c">// file header</span>

<span class="k">const</span> LIMIT: usize = <span class="s">512</span> * <span class="s">1024</span> * <span class="s">1024</span>; <span class="c">// largest file, bytes</span>

<span class="c">/// The whole file after the header.</span>
#[derive(Clone, PartialEq, Message)]
<span class="k">struct</span> Archive {
    #[prost(bytes = &quot;<span class="s">vec</span>&quot;, repeated, tag = &quot;<span class="s">1</span>&quot;)]
    documents: Vec&lt;Vec&lt;u8&gt;&gt;, <span class="c">// one protobuf session per document</span>
    #[prost(bytes = &quot;<span class="s">vec</span>&quot;, tag = &quot;<span class="s">2</span>&quot;)]
    metadata: Vec&lt;u8&gt;, <span class="c">// \`Metadata\` as JSON</span>
}

<span class="c">/// Everything about the scene that is not a document.</span>
#[derive(Serialize, Deserialize)]
<span class="k">struct</span> Metadata {
    documents: Vec&lt;Document&gt;,   <span class="c">// one per session</span>
    hidden: Vec&lt;(usize, String)&gt;, <span class="c">// (document, guid) hidden</span>
    texts: Vec&lt;(String, TextLabel, bool)&gt;, <span class="c">// (key, label, active)</span>
}

<span class="c">/// One document's name and placement.</span>
#[derive(Serialize, Deserialize)]
<span class="k">struct</span> Document {
    name: String,     <span class="c">// file name</span>
    place: [f64; <span class="s">16</span>], <span class="c">// placement matrix</span>
    point_px: f32,    <span class="c">// point size override</span>
}

<span class="c">/// The scene as \`.session\` file bytes.</span>
<span class="k">pub</span> <span class="k">fn</span> save(scene: &amp;Scene) -&gt; Result&lt;Vec&lt;u8&gt;, String&gt; {
    <span class="k">if</span> !scene.streamed.is_empty() || !scene.sheets.is_empty() {
        <span class="k">return</span> Err(&quot;<span class="s">This scene contains streamed sources. Open complete source documents before saving an editable session.</span>&quot;.into());
    }

    <span class="k">let</span> <span class="k">mut</span> documents = Vec::new();
    <span class="k">let</span> <span class="k">mut</span> size = <span class="s">0usize</span>;

    <span class="k">for</span> file <span class="k">in</span> &amp;scene.docs {
        <span class="k">if</span> file.display_only {
            <span class="k">return</span> Err(
                &quot;<span class="s">A source document is not retained; the complete session cannot be saved.</span>&quot;.into(),
            );
        }

        <span class="k">let</span> bytes = (*file.session).clone().pb_dumps(); <span class="c">// a copy keeps the undo history</span>
        size = size.saturating_add(bytes.len());

        <span class="k">if</span> size &gt; LIMIT {
            <span class="k">return</span> Err(&quot;<span class="s">Session exceeds the 512 MiB file limit</span>&quot;.into());
        }

        documents.push(bytes);
    }

    <span class="k">let</span> <span class="k">mut</span> hidden: Vec&lt;_&gt; = scene
        .hidden
        .iter()
        .map(|(doc, id)| (*doc, id.to_string()))
        .collect();
    hidden.sort();
    <span class="k">let</span> metadata = Metadata {
        documents: scene
            .docs
            .iter()
            .map(|f| Document {
                name: f.name.clone(),
                place: f.place.m,
                point_px: f.point_px,
            })
            .collect(),
        hidden,
        texts: scene
            .texts
            .iter()
            .map(|t| (t.key.clone(), t.label.clone(), t.active))
            .collect(),
    };
    <span class="k">let</span> archive = Archive {
        documents,
        metadata: serde_json::to_vec(&amp;metadata).map_err(|e| e.to_string())?,
    };

    <span class="k">if</span> archive.encoded_len() + MAGIC.len() &gt; LIMIT {
        <span class="k">return</span> Err(&quot;<span class="s">Session exceeds the 512 MiB file limit</span>&quot;.into());
    }

    <span class="k">let</span> <span class="k">mut</span> bytes = MAGIC.to_vec();
    archive.encode(&amp;<span class="k">mut</span> bytes).map_err(|e| e.to_string())?;
    Ok(bytes)
}

<span class="c">/// A scene from \`.session\` file bytes.</span>
<span class="k">pub</span> <span class="k">fn</span> open(bytes: &amp;[u8]) -&gt; Result&lt;Scene, String&gt; {
    <span class="k">if</span> bytes.len() &gt; LIMIT {
        <span class="k">return</span> Err(&quot;<span class="s">Session exceeds the 512 MiB file limit</span>&quot;.into());
    }

    <span class="k">let</span> payload = bytes
        .strip_prefix(MAGIC)
        .ok_or(&quot;<span class="s">Not a Session Viewer file</span>&quot;)?;
    <span class="k">let</span> archive = Archive::decode(payload).map_err(|e| e.to_string())?;
    <span class="k">let</span> metadata: Metadata =
        serde_json::from_slice(&amp;archive.metadata).map_err(|e| e.to_string())?;

    <span class="k">if</span> metadata.documents.len() != archive.documents.len() {
        <span class="k">return</span> Err(&quot;<span class="s">Document inventory does not match</span>&quot;.into());
    }

    <span class="k">let</span> <span class="k">mut</span> scene = Scene::new();

    <span class="k">for</span> (meta, bytes) <span class="k">in</span> metadata.documents.into_iter().zip(archive.documents) {
        <span class="k">if</span> !meta.place.into_iter().all(f64::is_finite) || !meta.point_px.is_finite() {
            <span class="k">return</span> Err(&quot;<span class="s">Non-finite document placement</span>&quot;.into());
        }

        <span class="k">let</span> proto =
            session_rust::proto::Session::decode(bytes.as_slice()).map_err(|e| e.to_string())?;
        super::validate::session(&amp;proto)?;
        <span class="k">let</span> session = Session::pb_loads(&amp;bytes).map_err(|e| e.to_string())?;
        super::validate::retained(&amp;session)?;
        scene.add_file(FileDoc {
            name: meta.name,
            place: Xform::from_matrix(meta.place),
            point_px: meta.point_px,
            display_only: <span class="s">false</span>,
            session: Rc::new(session),
        });
    }

    <span class="k">for</span> (key, label, active) <span class="k">in</span> metadata.texts {
        scene.register_text(key, label, active);
    }

    scene.hidden = metadata
        .hidden
        .into_iter()
        .map(|(doc, id)| (doc, Rc::from(id)))
        .collect();
    Ok(scene)
}

#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">mod</span> browser {
    <span class="k">use</span> wasm_bindgen::prelude::*;
    #[wasm_bindgen(inline_js = <span class="s">r</span>#&quot;
<span class="s">export function downloadSession(bytes) {</span>
<span class="s">    const url = URL.createObjectURL(new Blob([bytes], {type:'application/octet-stream'}));</span>

<span class="s">    const a = document.createElement('a'); a.href=url; a.download='session.session';</span>
<span class="s">    document.body.append(a); a.click(); a.remove(); setTimeout(()=&gt;URL.revokeObjectURL(url),10000);</span>
<span class="s">}</span>
<span class="s">export function chooseSession() {</span>
<span class="s">    return new Promise((resolve,reject)=&gt; {</span>
<span class="s">        const input=document.createElement('input'); input.type='file'; input.accept='.session';</span>
<span class="s">        input.oncancel=()=&gt;resolve(null);</span>
<span class="s">        input.onchange=async()=&gt;{try {const file=input.files[0]; if(!file){resolve(null);return;}</span>
<span class="s">            if(file.size&gt;512*1024*1024)throw Error('Session exceeds the 512 MiB file limit');</span>
<span class="s">            resolve(new Uint8Array(await file.arrayBuffer()));}catch(e){reject(e);}};</span>
<span class="s">        input.click();</span>
<span class="s">    });</span>
<span class="s">}</span>
&quot;#)]
    <span class="k">extern</span> &quot;<span class="s">C</span>&quot; {
        #[wasm_bindgen(catch, js_name=downloadSession)]
        <span class="k">pub</span> <span class="k">fn</span> download(bytes: &amp;[u8]) -&gt; Result&lt;(), JsValue&gt;;

        #[wasm_bindgen(js_name=chooseSession)]
        <span class="k">fn</span> choose() -&gt; js_sys::Promise;
    }

    <span class="c">/// Open a file picker and install the chosen session.</span>
    <span class="k">pub</span> <span class="k">fn</span> pick() {
        <span class="k">let</span> promise = choose();
        wasm_bindgen_futures::spawn_local(<span class="k">async</span> <span class="k">move</span> {
            <span class="k">let</span> result = wasm_bindgen_futures::JsFuture::from(promise).<span class="k">await</span>;

            <span class="k">match</span> result {
                Ok(value) <span class="k">if</span> value.is_null() =&gt; {}
                Ok(value) =&gt; <span class="k">match</span> super::open(&amp;js_sys::Uint8Array::new(&amp;value).to_vec()) {
                    Ok(scene) =&gt; super::super::loader::install_saved_scene(scene),
                    Err(error) =&gt; super::super::feedback::status(&amp;error),
                },
                Err(error) =&gt; {
                    super::super::feedback::status(&amp;format!(&quot;<span class="s">Cannot open session: </span>{<span class="s">error:?</span>}&quot;))
                }
            }
        });
    }
}
#[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
<span class="k">pub</span> <span class="k">use</span> browser::{download, pick};

#[cfg(test)]
<span class="k">mod</span> tests {
    <span class="k">use</span> super::*;
    <span class="k">use</span> <span class="k">crate</span>::app::deform::Target;
    <span class="k">use</span> session_rust::{Geometry, Mesh, Point};

    <span class="c">/// A created line keeps its screen pen after reopening.</span>
    #[test]
    <span class="k">fn</span> edited_documents_placements_hidden_state_and_history_survive_save() {
        <span class="k">let</span> <span class="k">mut</span> source = Session::new(&quot;<span class="s">source</span>&quot;);
        <span class="k">let</span> mesh = Mesh::from_vertices_and_faces(
            vec![
                Point::new(<span class="s">0</span>., <span class="s">0</span>., <span class="s">0</span>.),
                Point::new(<span class="s">10</span>., <span class="s">0</span>., <span class="s">0</span>.),
                Point::new(<span class="s">0</span>., <span class="s">10</span>., <span class="s">0</span>.),
            ],
            vec![vec![<span class="s">0</span>, <span class="s">1</span>, <span class="s">2</span>]],
        );
        source.add_mesh(mesh, None);
        <span class="k">let</span> shared = Rc::new(source);
        <span class="k">let</span> <span class="k">mut</span> scene = Scene::new();

        <span class="k">for</span> x <span class="k">in</span> [<span class="s">100</span>., <span class="s">200</span>.] {
            scene.add_file(FileDoc {
                name: format!(&quot;<span class="s">placement </span>{<span class="s">x</span>}&quot;),
                place: Xform::translation(x, <span class="s">0</span>., <span class="s">0</span>.),
                session: Rc::clone(&amp;shared),
                point_px: <span class="s">3</span>.,
                display_only: <span class="s">false</span>,
            });
        }

        scene
            .edit_subobject(
                <span class="s">0</span>,
                Target::Face(<span class="s">0</span>),
                &amp;Xform::translation(<span class="s">0</span>., <span class="s">0</span>., <span class="s">7</span>.),
                &quot;<span class="s">move face</span>&quot;,
            )
            .unwrap();
        scene.hidden.insert(scene.identity_of(<span class="s">1</span>).unwrap());
        <span class="k">let</span> bytes = save(&amp;scene).unwrap();
        assert!(scene.undo(), &quot;<span class="s">saving leaves live undo available</span>&quot;);
        <span class="k">let</span> restored = open(&amp;bytes).unwrap();
        assert_eq!(restored.docs.len(), <span class="s">2</span>);
        assert_eq!(restored.hidden.len(), <span class="s">1</span>);
        assert_eq!(restored.docs[<span class="s">0</span>].place.m[<span class="s">12</span>], <span class="s">100</span>.);
        assert_eq!(restored.docs[<span class="s">1</span>].place.m[<span class="s">12</span>], <span class="s">200</span>.);
        <span class="k">let</span> Geometry::Mesh(first) = restored.geometry(<span class="s">0</span>).unwrap() <span class="k">else</span> {
            panic!()
        };
        <span class="k">let</span> Geometry::Mesh(second) = restored.geometry(<span class="s">1</span>).unwrap() <span class="k">else</span> {
            panic!()
        };
        assert_eq!(first.vertex[&amp;<span class="s">0</span>].z, <span class="s">7</span>.);
        assert_eq!(second.vertex[&amp;<span class="s">0</span>].z, <span class="s">0</span>.);
        assert_eq!(first.face[&amp;<span class="s">0</span>], vec![<span class="s">0</span>, <span class="s">1</span>, <span class="s">2</span>]);
    }

    <span class="c">/// Junk and truncated files are refused.</span>
    #[test]
    <span class="k">fn</span> incomplete_or_foreign_files_are_rejected() {
        assert!(open(<span class="s">b</span>&quot;<span class="s">not a session</span>&quot;).is_err());
        <span class="k">let</span> bytes = save(&amp;Scene::new()).unwrap();
        assert!(open(&amp;bytes[..bytes.len() - <span class="s">1</span>]).is_err());
    }
}</code></pre></div>
<h2 id="step-11-srcappuirs">Step 11 · src/app/ui.rs<a class="anchor" href="#/course/29-docked-workspace#step-11-srcappuirs" aria-label="Link to this section">#</a></h2>
<p>Dock commands below the viewport, add the tool stripe and adapt panel widths to the screen.</p>
<p><code>lessons/29/src/app/ui.rs</code> · edit · type this</p>
<p>Delete the line <code>#[derive(Default)]</code> above <code>pub struct Model</code> in <code>lessons/28/src/app/ui.rs</code>.</p>
<p>Added after the line <code>history: VecDeque&lt;String&gt;,</code> in <code>lessons/28/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">impl</span> Default <span class="k">for</span> Model {
    <span class="c">/// The panel state at start.</span>
    <span class="k">fn</span> default() -&gt; <span class="k">Self</span> {
        <span class="k">Self</span> {
            layers_open: <span class="s">true</span>,
            rows: Vec::new(),
            command_open: <span class="s">false</span>,
            command: String::new(),
            focus_command: <span class="s">false</span>,
            status: String::new(),
            history: VecDeque::new(),
        }
    }
}</code></pre></div>
<p>Added after the line <code>controls: Option&lt;Vec&lt;Control&gt;&gt;,</code> in <code>lessons/28/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    scene_rect: egui::Rect,                    <span class="c">// canvas area not covered by panels</span>
    pointer: egui::Pos2,                       <span class="c">// last pointer position</span>
    ui_drag: bool,                             <span class="c">// a drag started on a panel</span>
    touches: std::collections::HashSet&lt;u64&gt;,   <span class="c">// fingers on panels</span>
}

<span class="k">impl</span> Ui {
    <span class="c">/// Create the egui state for a window.</span>
    <span class="k">pub</span> <span class="k">fn</span> new(window: &amp;Window, logical_width: f64) -&gt; <span class="k">Self</span> {
        MODEL.with_borrow_mut(|model| {
            model.layers_open = logical_width &gt;= <span class="s">700</span>.<span class="s">0</span>;
        });</code></pre></div>
<p>Added after the line <code>controls: (super::route::query(&quot;inspect&quot;).as_deref() == S…</code> in <code>lessons/28/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            scene_rect: egui::Rect::EVERYTHING,
            pointer: egui::Pos2::ZERO,
            ui_drag: <span class="s">false</span>,
            touches: std::collections::HashSet::new(),</code></pre></div>
<p>Replaces the line <code>(response.consumed || escape, response.repaint || escape)</code> in <code>lessons/28/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="c">// use the current pointer, not last frame's hover</span>
        <span class="k">use</span> winit::event::{ElementState, TouchPhase, WindowEvent};
        <span class="k">let</span> ratio = window.scale_factor() <span class="k">as</span> f32;
        <span class="k">let</span> <span class="k">mut</span> consumed = response.consumed;

        <span class="k">match</span> event {
            WindowEvent::CursorMoved { position, .. } =&gt; {
                <span class="k">self</span>.pointer = egui::pos2(position.x <span class="k">as</span> f32 / ratio, position.y <span class="k">as</span> f32 / ratio);
                consumed = <span class="k">self</span>.ui_drag || !<span class="k">self</span>.scene_rect.contains(<span class="k">self</span>.pointer);
            }
            WindowEvent::MouseInput { state, .. } =&gt; {
                <span class="k">if</span> *state == ElementState::Pressed {
                    <span class="k">self</span>.ui_drag = !<span class="k">self</span>.scene_rect.contains(<span class="k">self</span>.pointer);
                }

                consumed = <span class="k">self</span>.ui_drag;

                <span class="k">if</span> *state == ElementState::Released {
                    <span class="k">self</span>.ui_drag = <span class="s">false</span>;
                }
            }
            WindowEvent::MouseWheel { .. } =&gt; consumed = !<span class="k">self</span>.scene_rect.contains(<span class="k">self</span>.pointer),
            WindowEvent::Touch(touch) =&gt; {
                <span class="k">self</span>.pointer = egui::pos2(
                    touch.location.x <span class="k">as</span> f32 / ratio,
                    touch.location.y <span class="k">as</span> f32 / ratio,
                );

                <span class="k">if</span> touch.phase == TouchPhase::Started {
                    <span class="k">if</span> <span class="k">self</span>.touches.is_empty() {
                        <span class="k">self</span>.ui_drag = !<span class="k">self</span>.scene_rect.contains(<span class="k">self</span>.pointer);
                    }

                    <span class="k">self</span>.touches.insert(touch.id);
                }

                consumed = <span class="k">self</span>.ui_drag;

                <span class="k">if</span> matches!(touch.phase, TouchPhase::Ended | TouchPhase::Cancelled) {
                    <span class="k">self</span>.touches.remove(&amp;touch.id);

                    <span class="k">if</span> <span class="k">self</span>.touches.is_empty() {
                        <span class="k">self</span>.ui_drag = <span class="s">false</span>;
                    }
                }
            }
            WindowEvent::Focused(<span class="s">false</span>) =&gt; {
                <span class="k">self</span>.ui_drag = <span class="s">false</span>;
                <span class="k">self</span>.touches.clear();
            }
            _ =&gt; {}
        }

        (consumed || escape, response.repaint || escape)
    }

    <span class="c">/// Lay out and draw the panels; true when the frame must be redrawn.</span>
    <span class="k">pub</span> <span class="k">fn</span> frame(&amp;<span class="k">mut</span> <span class="k">self</span>, state: &amp;<span class="k">mut</span> State) -&gt; bool {
        <span class="k">let</span> <span class="k">mut</span> input = <span class="k">self</span>.input.take_egui_input(&amp;state.window);
        <span class="c">// layout in CSS pixels</span>
        <span class="k">let</span> logical = state.logical_size();
        input.screen_rect = Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(logical[<span class="s">0</span>] <span class="k">as</span> f32, logical[<span class="s">1</span>] <span class="k">as</span> f32),
        ));</code></pre></div>
<p>Added after the line <code>let mut command = None;</code> in <code>lessons/28/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> <span class="k">mut</span> tool = None;
        <span class="k">let</span> <span class="k">mut</span> output = <span class="k">self</span>.context.run_ui(input, |root| {
            MODEL.with_borrow_mut(|model| {
                commands(root, model, &amp;<span class="k">mut</span> <span class="k">self</span>.controls, &amp;<span class="k">mut</span> command);
                toolbar(root, model, &amp;<span class="k">mut</span> <span class="k">self</span>.controls, &amp;<span class="k">mut</span> tool);
                layers(root, model, &amp;<span class="k">mut</span> <span class="k">self</span>.controls, &amp;<span class="k">mut</span> action);
            });
            <span class="k">self</span>.scene_rect = root.available_rect_before_wrap();
        });
        <span class="k">self</span>.input
            .handle_platform_output(&amp;state.window, std::mem::take(&amp;<span class="k">mut</span> output.platform_output));
        <span class="k">let</span> changed = action.is_some() || command.is_some() || tool.is_some();

        <span class="k">if</span> <span class="k">let</span> Some(tool) = tool {
            <span class="k">match</span> tool {
                &quot;<span class="s">layers</span>&quot; =&gt; state.toggle_layers_panel(),
                &quot;<span class="s">controls</span>&quot; =&gt; {
                    state.selection_tool = <span class="k">crate</span>::app::selection::SelectionTool::Object;
                    state.enable_controls();
                }
                &quot;<span class="s">object</span>&quot; | &quot;<span class="s">edge</span>&quot; | &quot;<span class="s">face</span>&quot; =&gt; {
                    state.escape_selection();
                    state.selection_tool = <span class="k">match</span> tool {
                        &quot;<span class="s">edge</span>&quot; =&gt; <span class="k">crate</span>::app::selection::SelectionTool::Edge,
                        &quot;<span class="s">face</span>&quot; =&gt; <span class="k">crate</span>::app::selection::SelectionTool::Face,
                        _ =&gt; <span class="k">crate</span>::app::selection::SelectionTool::Object,
                    };
                }
                _ =&gt; command = Some(tool.to_string()),
            }

            state.touch();
        }</code></pre></div>
<p>Replaces the 2 lines from <code>output.pixels_per_point *=</code> in <code>lessons/28/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        output.pixels_per_point = state.gpu.config.width <span class="k">as</span> f32 / logical[<span class="s">0</span>].max(<span class="s">1</span>.<span class="s">0</span>) <span class="k">as</span> f32;</code></pre></div>
<p>Replaces the lines from <code>let hidden = MODEL.with_borrow(|model| model.command_open);</code> in <code>lessons/28/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">let</span> _ = status.set_attribute(&quot;<span class="s">hidden</span>&quot;, &quot;&quot;);</code></pre></div>
<p>Replaces the line <code>visuals.panel_fill = egui::Color32::WHITE;</code> in <code>lessons/28/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    visuals.panel_fill = egui::Color32::from_gray(<span class="s">247</span>);</code></pre></div>
<p>Replaces the line <code>context: &amp;egui::Context,</code> in <code>lessons/28/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    root: &amp;<span class="k">mut</span> egui::Ui,</code></pre></div>
<p>Replaces the 7 lines from <code>egui::Window::new(&quot;Session layers&quot;)</code> in <code>lessons/28/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">let</span> width = (root.available_width() * <span class="s">0</span>.<span class="s">25</span>).clamp(<span class="s">180</span>.<span class="s">0</span>, <span class="s">310</span>.<span class="s">0</span>);
    egui::Panel::right(&quot;<span class="s">session-layers</span>&quot;)
        .default_size(width)
        .size_range(<span class="s">160</span>.<span class="s">0</span>..=<span class="s">360</span>.<span class="s">0</span>)
        .resizable(<span class="s">true</span>)
        .show_inside(root, |ui| {
            ui.horizontal(|ui| {
                ui.strong(&quot;<span class="s">Layers</span>&quot;);
                <span class="k">let</span> close = ui.button(&quot;<span class="s">Close</span>&quot;);
                record(controls, &quot;<span class="s">layers/close</span>&quot;, &quot;<span class="s">Close layers</span>&quot;, &amp;close);

                <span class="k">if</span> close.clicked() {
                    model.layers_open = <span class="s">false</span>;
                }
            });
            ui.separator();
            <span class="c">// one line per row</span>
            egui::ScrollArea::vertical()
                .auto_shrink([<span class="s">false</span>, <span class="s">false</span>])</code></pre></div>
<p>Replaces the line <code>let (rect, response) = ui.allocate_exact_size(egui::vec2(…</code> in <code>lessons/28/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> (rect, response) = ui.allocate_exact_size(egui::vec2(<span class="s">20</span>.<span class="s">0</span>, <span class="s">28</span>.<span class="s">0</span>), egui::Sense::click());</code></pre></div>
<p>Replaces the line <code>ui.button(text).on_hover_text(label)</code> in <code>lessons/28/src/app/ui.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">let</span> width = <span class="k">if</span> row.key.starts_with(&quot;<span class="s">hide/</span>&quot;) {
        <span class="s">44</span>.<span class="s">0</span>
    } <span class="k">else</span> {
        (ui.available_width()
            - <span class="k">if</span> row.key.starts_with(&quot;<span class="s">select/</span>&quot;) {
                <span class="s">52</span>.<span class="s">0</span>
            } <span class="k">else</span> {
                <span class="s">0</span>.<span class="s">0</span>
            })
        .max(<span class="s">40</span>.<span class="s">0</span>)
    };
    ui.add_sized([width, <span class="s">28</span>.<span class="s">0</span>], egui::Button::new(text).truncate())
        .on_hover_text(label)
}

<span class="c">/// The command dock; an executed line goes to \`command\`.</span>
<span class="k">fn</span> commands(
    root: &amp;<span class="k">mut</span> egui::Ui,
    model: &amp;<span class="k">mut</span> Model,
    controls: &amp;<span class="k">mut</span> Option&lt;Vec&lt;Control&gt;&gt;,
    command: &amp;<span class="k">mut</span> Option&lt;String&gt;,
) {
    <span class="k">let</span> height = <span class="k">if</span> model.command_open { <span class="s">160</span>.<span class="s">0</span> } <span class="k">else</span> { <span class="s">76</span>.<span class="s">0</span> };
    egui::Panel::bottom(&quot;<span class="s">command-line</span>&quot;)
        .default_size(height)
        .size_range(<span class="s">76</span>.<span class="s">0</span>..=<span class="s">260</span>.<span class="s">0</span>)
        .resizable(<span class="s">true</span>)
        .show_inside(root, |ui| {
            egui::ScrollArea::vertical()
                .id_salt(&quot;<span class="s">command-history</span>&quot;)
                .stick_to_bottom(<span class="s">true</span>)
                .max_height((ui.available_height() - <span class="s">40</span>.<span class="s">0</span>).max(<span class="s">20</span>.<span class="s">0</span>))
                .show(ui, |ui| {
                    <span class="k">for</span> text <span class="k">in</span> &amp;model.history {
                        ui.label(text);
                    }

                    <span class="k">if</span> !model.status.is_empty() {
                        ui.label(&amp;model.status);
                    }
                });
            ui.separator();
            ui.horizontal(|ui| {
                ui.strong(&quot;<span class="s">Command:</span>&quot;);
                <span class="k">let</span> width = (ui.available_width() - <span class="s">100</span>.<span class="s">0</span>).max(<span class="s">40</span>.<span class="s">0</span>);
                <span class="k">let</span> response = ui.add_sized(
                    [width, <span class="s">28</span>.<span class="s">0</span>],
                    egui::TextEdit::singleline(&amp;<span class="k">mut</span> model.command)
                        .char_limit(<span class="s">2048</span>)
                        .hint_text(&quot;<span class="s">Type a command</span>&quot;),
                );
                record(controls, &quot;<span class="s">command/input</span>&quot;, &quot;<span class="s">Command</span>&quot;, &amp;response);

                <span class="k">if</span> model.focus_command {
                    response.request_focus();
                    model.focus_command = <span class="s">false</span>;
                }

                <span class="k">if</span> response.gained_focus() {
                    model.command_open = <span class="s">true</span>;
                }

                <span class="k">let</span> enter =
                    response.lost_focus() &amp;&amp; ui.input(|input| input.key_pressed(egui::Key::Enter));
                <span class="k">let</span> run = ui.add_sized([<span class="s">40</span>.<span class="s">0</span>, <span class="s">28</span>.<span class="s">0</span>], egui::Button::new(&quot;<span class="s">Run</span>&quot;));
                record(controls, &quot;<span class="s">command/run</span>&quot;, &quot;<span class="s">Run</span>&quot;, &amp;run);

                <span class="k">if</span> (enter || run.clicked()) &amp;&amp; !model.command.trim().is_empty() {
                    *command = Some(std::mem::take(&amp;<span class="k">mut</span> model.command));
                    model.focus_command = <span class="s">true</span>;
                }

                <span class="k">let</span> close = ui.add_sized([<span class="s">40</span>.<span class="s">0</span>, <span class="s">28</span>.<span class="s">0</span>], egui::Button::new(&quot;<span class="s">Esc</span>&quot;));
                record(controls, &quot;<span class="s">command/close</span>&quot;, &quot;<span class="s">Close</span>&quot;, &amp;close);

                <span class="k">if</span> close.clicked() || ui.input(|input| input.key_pressed(egui::Key::Escape)) {
                    model.command_open = <span class="s">false</span>;
                    response.surrender_focus();
                    <span class="k">crate</span>::app::feedback::focus_canvas();
                }
            });
        });
}

<span class="c">/// Add a button by adding its label, tooltip and command to this table.</span>
<span class="k">const</span> TOOLBAR: &amp;[(&amp;str, &amp;str, &amp;str)] = &amp;[
    (&quot;<span class="s">Obj</span>&quot;, &quot;<span class="s">Select objects</span>&quot;, &quot;<span class="s">object</span>&quot;),
    (
        &quot;<span class="s">Vtx</span>&quot;,
        &quot;<span class="s">Show and edit source vertices / control points</span>&quot;,
        &quot;<span class="s">controls</span>&quot;,
    ),
    (&quot;<span class="s">Edge</span>&quot;, &quot;<span class="s">Select source edges</span>&quot;, &quot;<span class="s">edge</span>&quot;),
    (&quot;<span class="s">Face</span>&quot;, &quot;<span class="s">Select source faces</span>&quot;, &quot;<span class="s">face</span>&quot;),
    (&quot;<span class="s">Fit</span>&quot;, &quot;<span class="s">Fit selection or scene</span>&quot;, &quot;<span class="s">fit</span>&quot;),
    (&quot;<span class="s">Layer</span>&quot;, &quot;<span class="s">Show or hide layers</span>&quot;, &quot;<span class="s">layers</span>&quot;),
    (&quot;<span class="s">+Pt</span>&quot;, &quot;<span class="s">Create a point</span>&quot;, &quot;<span class="s">point 0,0,0</span>&quot;),
    (&quot;<span class="s">+Ln</span>&quot;, &quot;<span class="s">Create a line</span>&quot;, &quot;<span class="s">line 0,0,0 100,0,0</span>&quot;),
    (
        &quot;<span class="s">+Poly</span>&quot;,
        &quot;<span class="s">Create a polyline</span>&quot;,
        &quot;<span class="s">polyline 0,0,0 100,0,0 100,100,0</span>&quot;,
    ),
    (&quot;<span class="s">Undo</span>&quot;, &quot;<span class="s">Undo the last edit</span>&quot;, &quot;<span class="s">undo</span>&quot;),
    (&quot;<span class="s">Redo</span>&quot;, &quot;<span class="s">Redo the last edit</span>&quot;, &quot;<span class="s">redo</span>&quot;),
    (&quot;<span class="s">Save</span>&quot;, &quot;<span class="s">Save the whole session</span>&quot;, &quot;<span class="s">save</span>&quot;),
    (&quot;<span class="s">Open</span>&quot;, &quot;<span class="s">Open a saved session</span>&quot;, &quot;<span class="s">open</span>&quot;),
];

<span class="c">/// The button row above the command field.</span>
<span class="k">fn</span> toolbar(
    root: &amp;<span class="k">mut</span> egui::Ui,
    model: &amp;<span class="k">mut</span> Model,
    controls: &amp;<span class="k">mut</span> Option&lt;Vec&lt;Control&gt;&gt;,
    action: &amp;<span class="k">mut</span> Option&lt;&amp;'static str&gt;, <span class="c">// the button pressed, if any</span>
) {
    egui::Panel::left(&quot;<span class="s">tools</span>&quot;)
        .exact_size(<span class="s">58</span>.<span class="s">0</span>)
        .resizable(<span class="s">false</span>)
        .show_inside(root, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                <span class="k">for</span> &amp;(label, help, command) <span class="k">in</span> TOOLBAR {
                    <span class="k">let</span> response = ui
                        .add_sized([<span class="s">44</span>.<span class="s">0</span>, <span class="s">44</span>.<span class="s">0</span>], egui::Button::new(label))
                        .on_hover_text(help);
                    record(controls, &amp;format!(&quot;<span class="s">toolbar/</span>{<span class="s">label</span>}&quot;), help, &amp;response);

                    <span class="k">if</span> response.clicked() {
                        <span class="k">if</span> command.contains('<span class="s"> </span>') {
                            model.command = command.to_string();
                            model.command_open = <span class="s">true</span>;
                            model.focus_command = <span class="s">true</span>;
                        } <span class="k">else</span> {
                            *action = Some(command);
                        }
                    }
                }
            });</code></pre></div>
<h2 id="step-12-srcenginegpufacesrs">Step 12 · src/engine/gpu/faces.rs<a class="anchor" href="#/course/29-docked-workspace#step-12-srcenginegpufacesrs" aria-label="Link to this section">#</a></h2>
<p>Keep source-face identities attached to the updated triangle ranges.</p>
<p><code>lessons/29/src/engine/gpu/faces.rs</code> · edit · type this</p>
<p>Added after the line <code>(source.parent == row).then_some((address, source))</code> in <code>lessons/28/src/engine/gpu/faces.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="c">/// Face id of \`face\` on object \`parent\`.</span>
    <span class="k">pub</span> <span class="k">fn</span> address(&amp;<span class="k">self</span>, parent: u32, face: usize) -&gt; Option&lt;u32&gt; {
        <span class="k">self</span>.sources
            .iter()
            .position(|source| source.parent == parent &amp;&amp; source.face == face)
            .map(|i| i <span class="k">as</span> u32)
    }

    <span class="c">/// Select a face; None clears the selection.</span></code></pre></div>
<h2 id="step-13-srcenginetextrs">Step 13 · src/engine/text.rs<a class="anchor" href="#/course/29-docked-workspace#step-13-srcenginetextrs" aria-label="Link to this section">#</a></h2>
<p>Keep retained text records available when the scene is saved or replaced.</p>
<p><code>lessons/29/src/engine/text.rs</code> · edit · type this</p>
<p>Replaces the line <code>#[derive(Clone, Debug, PartialEq)]</code> in <code>lessons/28/src/engine/text.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// Where a label sits and how it is sized.</span>
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]</code></pre></div>
<p>Replaces the line <code>#[derive(Clone, Copy, Debug, PartialEq)]</code> in <code>lessons/28/src/engine/text.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="c">/// The object a label belongs to, for picks and selection.</span>
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
<span class="k">pub</span> <span class="k">struct</span> TextObject {
    <span class="k">pub</span> row: u32, <span class="c">// object row</span>
    <span class="k">pub</span> selected: bool, <span class="c">// drawn as selected</span>
}

<span class="c">/// One text label; sizes in CSS pixels.</span>
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]</code></pre></div>
<h2 id="step-14-srclibrs">Step 14 · src/lib.rs<a class="anchor" href="#/course/29-docked-workspace#step-14-srclibrs" aria-label="Link to this section">#</a></h2>
<p>Connect browser events, scene changes and drawing through the application state.</p>
<p><code>lessons/29/src/lib.rs</code> · edit · type this</p>
<p>Added after the line <code>CancelPointer,</code> in <code>lessons/28/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    SavedScene(Box&lt;app::scene::Scene&gt;),           <span class="c">// a saved session loaded</span></code></pre></div>
<p>Replaces the line <code>self.ui = Some(app::ui::Ui::new(&amp;state.window));</code> in <code>lessons/28/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="c">// the egui panels and their GPU painter</span>
        <span class="k">self</span>.ui = Some(app::ui::Ui::new(&amp;state.window, state.logical_size()[<span class="s">0</span>]));</code></pre></div>
<p>Added after the line <code>Msg::SheetEntity(resolved) =&gt; state.sheet_entity(resolved),</code> in <code>lessons/28/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            Msg::SavedScene(scene) =&gt; {
                <span class="c">// replace the scene with the saved one</span>
                state.clear();
                state.scene = *scene;
                state.scene.rebuild(&amp;<span class="k">mut</span> state.gpu);
                state.fit_all();
                state.refresh_layers();
                state.touch();
                app::feedback::status(&quot;<span class="s">Session opened</span>&quot;);
            }</code></pre></div>
<p>Replaces the line <code>}</code> in <code>lessons/28/src/lib.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                    } | WindowEvent::Touch(winit::event::Touch {
                        phase: winit::event::TouchPhase::Ended
                            | winit::event::TouchPhase::Cancelled,
                        ..
                    })</code></pre></div>
<h2 id="step-15-srcstaters">Step 15 · src/state.rs<a class="anchor" href="#/course/29-docked-workspace#step-15-srcstaters" aria-label="Link to this section">#</a></h2>
<p>Track the active selection tool and edited component while replacing or picking a scene.</p>
<p><code>lessons/29/src/state.rs</code> · edit · type this</p>
<p>Added after the line <code>pub selection: SelectionMode,</code> in <code>lessons/28/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    <span class="k">pub</span> selection_tool: <span class="k">crate</span>::app::selection::SelectionTool, <span class="c">// what a click selects</span></code></pre></div>
<p>Added after the line <code>selection: SelectionMode::Object,</code> in <code>lessons/28/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            selection_tool: <span class="k">crate</span>::app::selection::SelectionTool::default(),</code></pre></div>
<p>Added after the line <code>.set_edge(&amp;self.gpu.ctx, Some((pick.row, edge)));</code> in <code>lessons/28/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                    <span class="k">self</span>.place_gizmo(Some(pick.row));</code></pre></div>
<p>Added after the line <code>.select(&amp;self.gpu.ctx, Some(address));</code> in <code>lessons/28/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>                    <span class="k">self</span>.place_gizmo(Some(source.parent));</code></pre></div>
<p>Added after the line <code>pub fn request_selection(&amp;mut self, x: u32, y: u32, edge:…</code> in <code>lessons/28/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> face = face || <span class="k">self</span>.selection_tool == <span class="k">crate</span>::app::selection::SelectionTool::Face;
        <span class="k">let</span> edge = edge || <span class="k">self</span>.selection_tool == <span class="k">crate</span>::app::selection::SelectionTool::Edge;</code></pre></div>
<p>Added after the line <code>self.upload_controls();</code> in <code>lessons/28/src/state.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.place_gizmo(Some(parent));</code></pre></div>
<h2 id="step-16-srcstateeditrs">Step 16 · src/state/edit.rs<a class="anchor" href="#/course/29-docked-workspace#step-16-srcstateeditrs" aria-label="Link to this section">#</a></h2>
<p>Route component gumball previews, touch grabs and Save/Open through the source-editing helpers.</p>
<p><code>lessons/29/src/state/edit.rs</code> · edit · type this</p>
<p>Added after the line <code>drag: Drag,</code> in <code>lessons/28/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>    target: Option&lt;<span class="k">crate</span>::app::deform::Target&gt;,           <span class="c">// a face, edge or control point being moved</span>
    source: Option&lt;session_rust::Geometry&gt;,               <span class="c">// the geometry before the drag</span>
    origin: Point,                                        <span class="c">// the gizmo center at the grab</span></code></pre></div>
<p>Replaces the line <code>let origin = Point::new(box_.cx, box_.cy, box_.cz);</code> in <code>lessons/28/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> <span class="k">mut</span> origin = box_.center();

        <span class="c">// a selected face, edge or control point: center on it instead</span>
        <span class="k">if</span> <span class="k">let</span> Some(row) = row
            &amp;&amp; <span class="k">let</span> Some(target) = <span class="k">crate</span>::app::deform::Target::selected(&amp;<span class="k">self</span>.selection)
            &amp;&amp; <span class="k">let</span> Some(geometry) = <span class="k">self</span>.scene.geometry(row)
            &amp;&amp; <span class="k">let</span> Ok(points) = <span class="k">crate</span>::app::deform::points(geometry, target)
            &amp;&amp; !points.is_empty()
            &amp;&amp; <span class="k">let</span> Some(place) = <span class="k">self</span>.scene.placement_of(row)
        {
            <span class="k">let</span> n = points.len() <span class="k">as</span> f64;
            origin = Point::new(
                points.iter().map(|p| p[<span class="s">0</span>]).sum::&lt;f64&gt;() / n,
                points.iter().map(|p| p[<span class="s">1</span>]).sum::&lt;f64&gt;() / n,
                points.iter().map(|p| p[<span class="s">2</span>]).sum::&lt;f64&gt;() / n,
            )
            .transformed(&amp;place);
        }</code></pre></div>
<p>Added after the line <code>pub fn begin_gizmo(&amp;mut self, x: f64, y: f64) -&gt; bool {</code> in <code>lessons/28/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">self</span>.begin_gizmo_with_radius(x, y, <span class="s">8</span>.<span class="s">0</span>)
    }

    <span class="c">/// Grab a gizmo handle under a finger, with a wider reach.</span>
    <span class="k">pub</span> <span class="k">fn</span> begin_gizmo_touch(&amp;<span class="k">mut</span> <span class="k">self</span>, x: f64, y: f64) -&gt; bool {
        <span class="k">self</span>.begin_gizmo_with_radius(x, y, <span class="s">18</span>.<span class="s">0</span>)
    }

    <span class="c">/// Grab a gizmo handle within \`radius\` CSS pixels.</span>
    <span class="k">fn</span> begin_gizmo_with_radius(&amp;<span class="k">mut</span> <span class="k">self</span>, x: f64, y: f64, radius: f64) -&gt; bool {</code></pre></div>
<p>Replaces the line <code>let Some(handle) = gizmo.hit(&amp;from, &amp;dir, per_px) else {</code> in <code>lessons/28/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> Some(handle) = gizmo.hit_with_radius(&amp;from, &amp;dir, per_px, radius) <span class="k">else</span> {</code></pre></div>
<p>Added after the line <code>drag,</code> in <code>lessons/28/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            target: <span class="k">crate</span>::app::deform::Target::selected(&amp;<span class="k">self</span>.selection),
            source: <span class="k">self</span>.scene.geometry(row).cloned(),
            origin: gizmo.origin.clone(),</code></pre></div>
<p>Replaces the line <code>let Some(gizmo) = self.gizmo.as_ref() else {</code> in <code>lessons/28/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="c">// the transform the gesture means so far</span>
        <span class="k">let</span> Some(delta) = Gizmo::new(active.origin.clone()).update(&amp;active.drag, &amp;from, &amp;dir)
        <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };

        <span class="c">// a face, edge or control point moves inside the object</span>
        <span class="k">if</span> <span class="k">let</span> (Some(target), Some(source)) = (active.target, active.source.as_ref()) {
            <span class="k">let</span> row = active.row;
            <span class="k">let</span> place = active.base_place.clone();
            <span class="k">let</span> Some(back) = place.inverse() <span class="k">else</span> {
                <span class="k">return</span> <span class="s">false</span>;
            };
            <span class="k">let</span> local = &amp;(&amp;back * &amp;delta) * &amp;place; <span class="c">// the delta in the object's own frame</span>
            <span class="k">let</span> edited = <span class="k">match</span> <span class="k">crate</span>::app::deform::transform(source, target, &amp;local) {
                Ok(value) =&gt; value,
                Err(error) =&gt; {
                    <span class="k">self</span>.status(&amp;error);
                    <span class="k">return</span> <span class="s">false</span>;
                }
            };
            <span class="k">let</span> origin = active.origin.transformed(&amp;delta);

            <span class="k">if</span> <span class="k">let</span> Err(error) = <span class="k">self</span>.scene.preview_geometry(row, edited, &amp;<span class="k">mut</span> <span class="k">self</span>.gpu) {
                <span class="k">self</span>.status(&amp;error);
                <span class="k">return</span> <span class="s">false</span>;
            }

            <span class="k">if</span> <span class="k">let</span> Some(gizmo) = <span class="k">self</span>.gizmo.as_mut() {
                gizmo.origin = origin;
            }

            <span class="k">self</span>.upload_gizmo();
            <span class="k">self</span>.touch();
            <span class="k">return</span> <span class="s">true</span>;
        }</code></pre></div>
<p>Added after the line <code>};</code> in <code>lessons/28/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">if</span> active.target.is_some() {
            <span class="k">self</span>.scene.rebuild(&amp;<span class="k">mut</span> <span class="k">self</span>.gpu);
            <span class="k">self</span>.restore_edit_selection(active.row);
        }

        <span class="c">// put the preview back, the document applies the real move</span></code></pre></div>
<p>Replaces the line <code>let Some(delta) = gizmo.update(&amp;active.drag, &amp;from, &amp;dir)…</code> in <code>lessons/28/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">let</span> Some(delta) = Gizmo::new(active.origin.clone()).update(&amp;active.drag, &amp;from, &amp;dir)
        <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };

        <span class="k">if</span> <span class="k">let</span> Some(target) = active.target {
            <span class="k">let</span> result =
                <span class="k">self</span>.scene
                    .edit_subobject(active.row, target, &amp;delta, &quot;<span class="s">transform subobject</span>&quot;);
            <span class="k">self</span>.scene.rebuild(&amp;<span class="k">mut</span> <span class="k">self</span>.gpu);
            <span class="k">self</span>.restore_edit_selection(active.row);

            <span class="k">if</span> <span class="k">let</span> Err(error) = result {
                <span class="k">self</span>.status(&amp;error);
                <span class="k">return</span> <span class="s">false</span>;
            }

            <span class="k">self</span>.refresh_layers();
            <span class="k">self</span>.touch();
            <span class="k">return</span> <span class="s">true</span>;
        }

        <span class="c">// world delta to local: conjugate by the parent</span></code></pre></div>
<p>Added after the line <code>if let Some(active) = self.dragging.take() {</code> in <code>lessons/28/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            <span class="k">if</span> active.target.is_some() {
                <span class="k">self</span>.scene.rebuild(&amp;<span class="k">mut</span> <span class="k">self</span>.gpu);
                <span class="k">self</span>.restore_edit_selection(active.row);
            }</code></pre></div>
<p>Added after the line <code>match command {</code> in <code>lessons/28/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>            Command::Save =&gt; {
                <span class="k">let</span> bytes = <span class="k">crate</span>::app::session_io::save(&amp;<span class="k">self</span>.scene)?;
                #[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
                <span class="k">crate</span>::app::session_io::download(&amp;bytes)
                    .map_err(|e| format!(&quot;<span class="s">Save failed: </span>{<span class="s">e:?</span>}&quot;))?;
                Ok(format!(&quot;<span class="s">Saved complete session (</span>{}<span class="s"> bytes)</span>&quot;, bytes.len()))
            }
            Command::Open =&gt; {
                #[cfg(target_arch = &quot;<span class="s">wasm32</span>&quot;)]
                <span class="k">crate</span>::app::session_io::pick();
                Ok(&quot;<span class="s">Choose a .session file</span>&quot;.into())
            }</code></pre></div>
<p>Added after the line <code>};</code> in <code>lessons/28/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="c">// a face, edge or control point moves inside the object</span>
        <span class="k">if</span> <span class="k">let</span> Some(target) = <span class="k">crate</span>::app::deform::Target::selected(&amp;<span class="k">self</span>.selection) {
            <span class="k">self</span>.scene.edit_subobject(row, target, &amp;delta, label)?;
            <span class="k">self</span>.scene.rebuild(&amp;<span class="k">mut</span> <span class="k">self</span>.gpu);
            <span class="k">self</span>.restore_edit_selection(row);
            <span class="k">self</span>.touch();
            <span class="k">return</span> Ok(label.into());
        }</code></pre></div>
<p>Replaces the 7 lines from <code>let index = match active.id {</code> in <code>lessons/28/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>        <span class="k">if</span> <span class="k">let</span> Err(error) = <span class="k">self</span>
            .scene
            .set_source_control(active.parent, active.id, &amp;point)
        {
            <span class="k">self</span>.status(&amp;error);</code></pre></div>
<p>Added after the line <code>}</code> in <code>lessons/28/src/state/edit.rs</code></p>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">impl</span> State {
    <span class="c">/// Reselect \`row\` after a rebuild, keeping the face, edge or control mode.</span>
    <span class="k">fn</span> restore_edit_selection(&amp;<span class="k">mut</span> <span class="k">self</span>, row: u32) {
        <span class="k">let</span> selection = <span class="k">self</span>.selection.clone();
        <span class="k">self</span>.select(Some(row)); <span class="c">// resets the mode</span>
        <span class="k">self</span>.selection = selection; <span class="c">// put it back</span>

        <span class="k">match</span> <span class="k">self</span>.selection {
            SelectionMode::Controls { .. } =&gt; {
                <span class="k">self</span>.gpu.set_selected(row, <span class="s">false</span>);

                <span class="k">if</span> <span class="k">let</span> Some(geometry) = <span class="k">self</span>.scene.geometry(row) {
                    <span class="k">self</span>.controls = <span class="k">crate</span>::app::selection::Controls::from_geometry(geometry);
                }

                <span class="k">self</span>.upload_controls();
            }
            SelectionMode::Face { face, .. } =&gt; {
                <span class="k">self</span>.gpu.set_selected(row, <span class="s">false</span>);
                <span class="k">let</span> address = <span class="k">self</span>.gpu.arena.source_faces.address(row, face);
                <span class="k">self</span>.gpu.arena.source_faces.select(&amp;<span class="k">self</span>.gpu.ctx, address);
            }
            SelectionMode::Edge { edge, .. } =&gt; {
                <span class="k">self</span>.gpu.set_selected(row, <span class="s">false</span>);
                <span class="k">self</span>.gpu.segments.set_edge(&amp;<span class="k">self</span>.gpu.ctx, Some((row, edge)));
            }
            SelectionMode::Object =&gt; {}
        }

        <span class="k">self</span>.place_gizmo(Some(row));
        <span class="k">self</span>.refresh_layers();
        <span class="k">self</span>.touch();
    }
}</code></pre></div>
<h2 id="check">Check<a class="anchor" href="#/course/29-docked-workspace#check" aria-label="Link to this section">#</a></h2>
<p>Run <code>trunk serve</code> in <code>lessons/29/</code> and open <a href="http://127.0.0.1:8770/" target="_blank" rel="noopener">http://127.0.0.1:8770/</a>.</p>
<p>Expected: the bottom command dock and right layer panel surround the scene; reopening a saved session reports <strong>Session opened</strong>.</p>
<p><img src="/session/docs/course/docs/screenshots/extensions-workspace-desktop.png" alt="Full viewer result for lesson 29" loading="lazy" decoding="async"></p>
<p>If it fails:</p>
<ul>
<li>A shared mesh corner separates: the edit changes render triangles instead of source vertices.</li>
<li>Save omits geometry: a streamed prefix is treated as a complete source.</li>
</ul>
<h2 id="what-changed">What changed<a class="anchor" href="#/course/29-docked-workspace#what-changed" aria-label="Link to this section">#</a></h2>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code>lessons/29/src/
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
│   ├── command.rs  ~
│   ├── coords.rs
│   ├── cplane.rs
│   ├── decode.rs
│   ├── deform.rs  +
│   ├── edit.rs  ~
│   ├── feedback.rs
│   ├── fetch.rs
│   ├── gizmo.rs  ~
│   ├── hierarchy.rs
│   ├── input.rs  ~
│   ├── inspection.rs
│   ├── knobs.rs
│   ├── layers.rs
│   ├── live.rs
│   ├── loader.rs  ~
│   ├── manifest.rs
│   ├── mod.rs  ~
│   ├── modeling.rs
│   ├── route.rs
│   ├── scene.rs
│   ├── scene_text.rs  ~
│   ├── selection.rs  ~
│   ├── session_io.rs  +
│   ├── sheet_query.rs
│   ├── snap.rs
│   ├── stream.rs
│   ├── touch.rs
│   ├── ui.rs  ~
│   └── validate.rs
├── engine/
│   ├── gpu/
│   │   ├── arena.rs
│   │   ├── backdrop.rs
│   │   ├── buffers.rs
│   │   ├── cloud.rs
│   │   ├── device.rs
│   │   ├── faces.rs  ~
│   │   ├── frame.rs
│   │   ├── glyphs.rs
│   │   ├── instance.rs
│   │   ├── lod.rs
│   │   ├── mod.rs
│   │   ├── objects.rs
│   │   ├── pick.rs
│   │   ├── present.rs
│   │   ├── render.rs
│   │   ├── segments.rs
│   │   ├── splat.rs
│   │   ├── surface_outline.rs
│   │   ├── targets.rs
│   │   ├── text.rs
│   │   ├── text_outline.rs
│   │   ├── text_plane.rs
│   │   ├── text_plate.rs
│   │   ├── triangle_tiles.rs
│   │   ├── ui.rs
│   │   ├── upload.rs
│   │   ├── view.rs
│   │   ├── widget.rs
│   │   └── widget_mesh.rs
│   ├── pipelines/
│   │   ├── layouts.rs
│   │   └── mod.rs
│   ├── mod.rs
│   ├── performance.rs
│   └── text.rs  ~
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
│   └── widget.wgsl
├── state/
│   ├── cloud_query.rs
│   ├── edit.rs  ~
│   ├── panel.rs
│   ├── sheet_query.rs
│   └── text.rs
├── camera.rs
├── lib.rs  ~
└── state.rs  ~</code></pre></div>
<p><code>+</code> new in this lesson · <code>~</code> changed in this lesson</p>
<p>Data flow: component pick → source edit → preview → saved session archive.
Every file at this point: <code>lessons/29/</code>.</p>
<h2 id="next">Next<a class="anchor" href="#/course/29-docked-workspace#next" aria-label="Link to this section">#</a></h2>
<p><a href="#/course/30-layer-tree">30 · Keep source dragging live and build one layer tree</a></p>
<h2 id="expected-viewer-result">Expected viewer result<a class="anchor" href="#/course/29-docked-workspace#expected-viewer-result" aria-label="Link to this section">#</a></h2>
<p>The completed workspace: a command area across the entire bottom, a right-hand Layers panel, a left toolbar and a selected object with its solid gumball. Commands operate on source geometry; Save writes the complete retained session to one .session file. See the <a href="/session/docs/course/docs/screenshots/extensions-workspace-phone.png">phone capture</a> for the narrow-screen layout.</p>
<p><a href="/session/docs/course/docs/screenshots/extensions-workspace-desktop.png"><img src="/session/docs/course/docs/screenshots/extensions-workspace-desktop.png" alt="Full viewer result for lesson 29" loading="lazy" decoding="async"></a></p>
`,toc:[{level:2,id:"step-1-srcappcommandrs",text:"Step 1 · src/app/command.rs"},{level:2,id:"step-2-srcappdeformrs",text:"Step 2 · src/app/deform.rs"},{level:2,id:"step-3-srcappeditrs",text:"Step 3 · src/app/edit.rs"},{level:2,id:"step-4-srcappgizmors",text:"Step 4 · src/app/gizmo.rs"},{level:2,id:"step-5-srcappinputrs",text:"Step 5 · src/app/input.rs"},{level:2,id:"step-6-srcapploaderrs",text:"Step 6 · src/app/loader.rs"},{level:2,id:"step-7-srcappmodrs",text:"Step 7 · src/app/mod.rs"},{level:2,id:"step-8-srcappscene_textrs",text:"Step 8 · src/app/scene_text.rs"},{level:2,id:"step-9-srcappselectionrs",text:"Step 9 · src/app/selection.rs"},{level:2,id:"step-10-srcappsession_iors",text:"Step 10 · src/app/session_io.rs"},{level:2,id:"step-11-srcappuirs",text:"Step 11 · src/app/ui.rs"},{level:2,id:"step-12-srcenginegpufacesrs",text:"Step 12 · src/engine/gpu/faces.rs"},{level:2,id:"step-13-srcenginetextrs",text:"Step 13 · src/engine/text.rs"},{level:2,id:"step-14-srclibrs",text:"Step 14 · src/lib.rs"},{level:2,id:"step-15-srcstaters",text:"Step 15 · src/state.rs"},{level:2,id:"step-16-srcstateeditrs",text:"Step 16 · src/state/edit.rs"},{level:2,id:"check",text:"Check"},{level:2,id:"what-changed",text:"What changed"},{level:2,id:"next",text:"Next"},{level:2,id:"expected-viewer-result",text:"Expected viewer result"}]};export{s as default};
