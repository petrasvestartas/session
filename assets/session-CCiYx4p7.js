const s={title:"session_rust/src/session.rs",html:`<h1 id="session_rustsrcsessionrs">session_rust/src/session.rs<a class="anchor" href="#/course/kernel/session#session_rustsrcsessionrs" aria-label="Link to this section">#</a></h1>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> <span class="k">crate</span>::collection::Keyed;
<span class="k">use</span> <span class="k">crate</span>::color::Color;
<span class="k">use</span> <span class="k">crate</span>::history::clone;
<span class="k">use</span> <span class="k">crate</span>::history::weight;
<span class="k">use</span> <span class="k">crate</span>::history::Entry;
<span class="k">use</span> <span class="k">crate</span>::history::History;
<span class="k">use</span> <span class="k">crate</span>::history::Op;
<span class="k">use</span> <span class="k">crate</span>::history::ReplaceOp;
<span class="k">use</span> <span class="k">crate</span>::history::Tomb;
<span class="k">use</span> <span class="k">crate</span>::history::Tombstone;
<span class="k">use</span> <span class="k">crate</span>::history::TreeOp;
<span class="k">use</span> <span class="k">crate</span>::history::XformOp;
<span class="k">use</span> <span class="k">crate</span>::history::RECORD;
<span class="k">use</span> <span class="k">crate</span>::interaction::Interaction;
<span class="k">use</span> <span class="k">crate</span>::intersection::line_line;
<span class="k">use</span> <span class="k">crate</span>::intersection::line_plane;
<span class="k">use</span> <span class="k">crate</span>::intersection::ray_box;
<span class="k">use</span> <span class="k">crate</span>::intersection::ray_mesh_bvh;
<span class="k">use</span> <span class="k">crate</span>::objects::Component;
<span class="k">use</span> <span class="k">crate</span>::tree::node_head;
<span class="k">use</span> <span class="k">crate</span>::tree::node_tail;
<span class="k">use</span> <span class="k">crate</span>::BRep;
<span class="k">use</span> <span class="k">crate</span>::Collection;
<span class="k">use</span> <span class="k">crate</span>::Element;
<span class="k">use</span> <span class="k">crate</span>::Graph;
<span class="k">use</span> <span class="k">crate</span>::InstanceRef;
<span class="k">use</span> <span class="k">crate</span>::Line;
<span class="k">use</span> <span class="k">crate</span>::Mesh;
<span class="k">use</span> <span class="k">crate</span>::NurbsCurve;
<span class="k">use</span> <span class="k">crate</span>::NurbsSurface;
<span class="k">use</span> <span class="k">crate</span>::Objects;
<span class="k">use</span> <span class="k">crate</span>::Plane;
<span class="k">use</span> <span class="k">crate</span>::Point;
<span class="k">use</span> <span class="k">crate</span>::PointCloud;
<span class="k">use</span> <span class="k">crate</span>::Polyline;
<span class="k">use</span> <span class="k">crate</span>::SpatialBVH;
<span class="k">use</span> <span class="k">crate</span>::Tolerance;
<span class="k">use</span> <span class="k">crate</span>::Tree;
<span class="k">use</span> <span class="k">crate</span>::TreeNode;
<span class="k">use</span> <span class="k">crate</span>::Vector;
<span class="k">use</span> <span class="k">crate</span>::Xform;
<span class="k">use</span> <span class="k">crate</span>::OBB;
<span class="k">use</span> serde::Deserialize;
<span class="k">use</span> serde::Serialize;
<span class="k">use</span> std::cell::RefCell;
<span class="k">use</span> std::collections::BTreeMap;
<span class="k">use</span> std::collections::BTreeSet;
<span class="k">use</span> std::collections::HashMap;
<span class="k">use</span> std::fmt;
<span class="k">use</span> std::fs;
<span class="k">use</span> std::rc::Rc;
<span class="k">use</span> std::rc::Weak;

<span class="c">/// All geometry types as a variant; a new type joins here and in the Objects vectors.</span>
#[derive(Debug, Clone)]
<span class="k">pub</span> <span class="k">enum</span> Geometry {
    OBB(Rc&lt;OBB&gt;),
    BRep(Rc&lt;BRep&gt;),
    Element(Rc&lt;Element&gt;),
    Line(Rc&lt;Line&gt;),
    Mesh(Rc&lt;Mesh&gt;),
    NurbsCurve(Rc&lt;NurbsCurve&gt;),
    NurbsSurface(Rc&lt;NurbsSurface&gt;),
    Plane(Rc&lt;Plane&gt;),
    Point(Rc&lt;Point&gt;),
    PointCloud(Rc&lt;PointCloud&gt;),
    Polyline(Rc&lt;Polyline&gt;),
}

<span class="k">impl</span> Geometry {
    <span class="c">/// Return the guid of the wrapped object.</span>
    <span class="k">pub</span> <span class="k">fn</span> guid(&amp;<span class="k">self</span>) -&gt; &amp;str {
        <span class="k">match</span> <span class="k">self</span> {
            Geometry::OBB(g) =&gt; g.guid(),
            Geometry::BRep(g) =&gt; g.guid(),
            Geometry::Element(g) =&gt; g.guid(),
            Geometry::Line(g) =&gt; g.guid(),
            Geometry::Mesh(g) =&gt; g.guid(),
            Geometry::NurbsCurve(g) =&gt; g.guid(),
            Geometry::NurbsSurface(g) =&gt; g.guid(),
            Geometry::Plane(g) =&gt; g.guid(),
            Geometry::Point(g) =&gt; g.guid(),
            Geometry::PointCloud(g) =&gt; g.guid(),
            Geometry::Polyline(g) =&gt; g.guid(),
        }
    }

    <span class="c">/// Return the name of the wrapped object.</span>
    <span class="k">pub</span> <span class="k">fn</span> name(&amp;<span class="k">self</span>) -&gt; &amp;str {
        <span class="k">match</span> <span class="k">self</span> {
            Geometry::OBB(g) =&gt; &amp;g.name,
            Geometry::BRep(g) =&gt; &amp;g.name,
            Geometry::Element(g) =&gt; &amp;g.name,
            Geometry::Line(g) =&gt; &amp;g.name,
            Geometry::Mesh(g) =&gt; &amp;g.name,
            Geometry::NurbsCurve(g) =&gt; &amp;g.name,
            Geometry::NurbsSurface(g) =&gt; &amp;g.name,
            Geometry::Plane(g) =&gt; &amp;g.name,
            Geometry::Point(g) =&gt; &amp;g.name,
            Geometry::PointCloud(g) =&gt; &amp;g.name,
            Geometry::Polyline(g) =&gt; &amp;g.name,
        }
    }

    <span class="c">/// Overwrite the guid.</span>
    <span class="k">pub</span> <span class="k">fn</span> set_guid(&amp;<span class="k">mut</span> <span class="k">self</span>, guid: &amp;str) {
        <span class="k">match</span> <span class="k">self</span> {
            Geometry::OBB(g) =&gt; Rc::make_mut(g).set_guid(guid.to_string()),
            Geometry::BRep(g) =&gt; Rc::make_mut(g).set_guid(guid.to_string()),
            Geometry::Element(g) =&gt; Rc::make_mut(g).set_guid(guid.to_string()),
            Geometry::Line(g) =&gt; Rc::make_mut(g).set_guid(guid.to_string()),
            Geometry::Mesh(g) =&gt; Rc::make_mut(g).set_guid(guid.to_string()),
            Geometry::NurbsCurve(g) =&gt; Rc::make_mut(g).set_guid(guid.to_string()),
            Geometry::NurbsSurface(g) =&gt; Rc::make_mut(g).set_guid(guid.to_string()),
            Geometry::Plane(g) =&gt; Rc::make_mut(g).set_guid(guid.to_string()),
            Geometry::Point(g) =&gt; Rc::make_mut(g).set_guid(guid.to_string()),
            Geometry::PointCloud(g) =&gt; Rc::make_mut(g).set_guid(guid.to_string()),
            Geometry::Polyline(g) =&gt; Rc::make_mut(g).set_guid(guid.to_string()),
        }
    }

    <span class="c">/// Overwrite the name.</span>
    <span class="k">pub</span>(<span class="k">crate</span>) <span class="k">fn</span> set_name(&amp;<span class="k">mut</span> <span class="k">self</span>, name: &amp;str) {
        <span class="k">match</span> <span class="k">self</span> {
            Geometry::OBB(g) =&gt; Rc::make_mut(g).name = name.to_string(),
            Geometry::BRep(g) =&gt; Rc::make_mut(g).name = name.to_string(),
            Geometry::Element(g) =&gt; Rc::make_mut(g).name = name.to_string(),
            Geometry::Line(g) =&gt; Rc::make_mut(g).name = name.to_string(),
            Geometry::Mesh(g) =&gt; Rc::make_mut(g).name = name.to_string(),
            Geometry::NurbsCurve(g) =&gt; Rc::make_mut(g).name = name.to_string(),
            Geometry::NurbsSurface(g) =&gt; Rc::make_mut(g).name = name.to_string(),
            Geometry::Plane(g) =&gt; Rc::make_mut(g).name = name.to_string(),
            Geometry::Point(g) =&gt; Rc::make_mut(g).name = name.to_string(),
            Geometry::PointCloud(g) =&gt; Rc::make_mut(g).name = name.to_string(),
            Geometry::Polyline(g) =&gt; Rc::make_mut(g).name = name.to_string(),
        }
    }
}

<span class="c">/// Anything an Objects collection holds: geometry, a Component or an InstanceRef.</span>
#[derive(Debug, Clone)]
<span class="k">pub</span> <span class="k">enum</span> Item {
    Geometry(Geometry),
    Component(Component),
    InstanceRef(Rc&lt;InstanceRef&gt;),
}

<span class="k">impl</span> Item {
    <span class="c">/// Return the guid of the wrapped object.</span>
    <span class="k">pub</span> <span class="k">fn</span> guid(&amp;<span class="k">self</span>) -&gt; &amp;str {
        <span class="k">match</span> <span class="k">self</span> {
            Item::Geometry(g) =&gt; g.guid(),
            Item::Component(c) =&gt; c.guid(),
            Item::InstanceRef(i) =&gt; i.guid(),
        }
    }

    <span class="c">/// Return the name of the wrapped object.</span>
    <span class="k">pub</span> <span class="k">fn</span> name(&amp;<span class="k">self</span>) -&gt; &amp;str {
        <span class="k">match</span> <span class="k">self</span> {
            Item::Geometry(g) =&gt; g.name(),
            Item::Component(c) =&gt; &amp;c.name,
            Item::InstanceRef(i) =&gt; &amp;i.name,
        }
    }
}

<span class="k">impl</span> From&lt;Geometry&gt; <span class="k">for</span> Item {
    <span class="c">/// Wrap geometry.</span>
    <span class="k">fn</span> from(geometry: Geometry) -&gt; <span class="k">Self</span> {
        Item::Geometry(geometry)
    }
}

<span class="c">/// Extract a concrete geometry type out of a \`Geometry\` variant, what C++ gets from \`std::get_if\`.</span>
<span class="k">pub</span> <span class="k">trait</span> FromGeometry: Sized {
    <span class="c">/// The object inside \`geometry\`, or None when the variant holds another type.</span>
    <span class="k">fn</span> from_geometry(geometry: &amp;Geometry) -&gt; Option&lt;&amp;<span class="k">Self</span>&gt;;
}

macro_rules! impl_from_geometry {
    ($($variant:ident =&gt; $type:ty),* $(,)?) =&gt; {
        $(<span class="k">impl</span> FromGeometry <span class="k">for</span> $type {
            <span class="k">fn</span> from_geometry(geometry: &amp;Geometry) -&gt; Option&lt;&amp;<span class="k">Self</span>&gt; {

                <span class="k">match</span> geometry {
                    Geometry::$variant(g) =&gt; Some(g.as_ref()),
                    _ =&gt; None,
                }
            }
        })*
    };
}

impl_from_geometry!(
    OBB =&gt; OBB,
    BRep =&gt; BRep,
    Element =&gt; Element,
    Line =&gt; Line,
    Mesh =&gt; Mesh,
    NurbsCurve =&gt; NurbsCurve,
    NurbsSurface =&gt; NurbsSurface,
    Plane =&gt; Plane,
    Point =&gt; Point,
    PointCloud =&gt; PointCloud,
    Polyline =&gt; Polyline,
);

<span class="c">/// The Objects vectors, those \`order()\` walks first and in its sequence, each with the prefix of its graph node attribute.</span>
<span class="k">pub</span> <span class="k">const</span> COLLECTIONS: [(&amp;str, &amp;str); <span class="s">13</span>] = [
    (&quot;<span class="s">points</span>&quot;, &quot;<span class="s">point</span>&quot;),
    (&quot;<span class="s">lines</span>&quot;, &quot;<span class="s">line</span>&quot;),
    (&quot;<span class="s">planes</span>&quot;, &quot;<span class="s">plane</span>&quot;),
    (&quot;<span class="s">bboxes</span>&quot;, &quot;<span class="s">bbox</span>&quot;),
    (&quot;<span class="s">polylines</span>&quot;, &quot;<span class="s">polyline</span>&quot;),
    (&quot;<span class="s">pointclouds</span>&quot;, &quot;<span class="s">pointcloud</span>&quot;),
    (&quot;<span class="s">meshes</span>&quot;, &quot;<span class="s">mesh</span>&quot;),
    (&quot;<span class="s">nurbscurves</span>&quot;, &quot;<span class="s">nurbscurve</span>&quot;),
    (&quot;<span class="s">nurbssurfaces</span>&quot;, &quot;<span class="s">nurbssurface</span>&quot;),
    (&quot;<span class="s">breps</span>&quot;, &quot;<span class="s">brep</span>&quot;),
    (&quot;<span class="s">elements</span>&quot;, &quot;<span class="s">element</span>&quot;),
    (&quot;<span class="s">components</span>&quot;, &quot;<span class="s">component</span>&quot;),
    (&quot;<span class="s">instances</span>&quot;, &quot;<span class="s">instance</span>&quot;),
];

<span class="c">/// The slot bookkeeping every Collection shares, whatever it holds.</span>
<span class="k">trait</span> Slots {
    <span class="c">/// Return the slot of a live guid.</span>
    <span class="k">fn</span> get_slot(&amp;<span class="k">self</span>, guid: &amp;str) -&gt; Option&lt;usize&gt;;

    <span class="c">/// Return the guid in a slot, dead or alive.</span>
    <span class="k">fn</span> key_at(&amp;<span class="k">self</span>, slot: usize) -&gt; &amp;str;

    <span class="c">/// Return whether a slot is dead.</span>
    <span class="k">fn</span> is_dead(&amp;<span class="k">self</span>, slot: usize) -&gt; bool;

    <span class="c">/// Kill or revive a slot.</span>
    <span class="k">fn</span> set_dead(&amp;<span class="k">mut</span> <span class="k">self</span>, slot: usize, dead: bool);

    <span class="c">/// Return the tomb pinning a slot while a record still holds it.</span>
    <span class="k">fn</span> get_tomb(&amp;<span class="k">self</span>, slot: usize) -&gt; Option&lt;Rc&lt;Tomb&gt;&gt;;

    <span class="c">/// Pin a slot to a tomb.</span>
    <span class="k">fn</span> set_tomb(&amp;<span class="k">mut</span> <span class="k">self</span>, slot: usize, tomb: &amp;Rc&lt;Tomb&gt;);

    <span class="c">/// Return the number of dead slots not yet purged.</span>
    <span class="k">fn</span> number_of_dead(&amp;<span class="k">self</span>) -&gt; usize;

    <span class="c">/// Return the number of raw slots, dead ones included.</span>
    <span class="k">fn</span> number_of_slots(&amp;<span class="k">self</span>) -&gt; usize;

    <span class="c">/// Return whether a compaction is part way.</span>
    <span class="k">fn</span> is_compacting(&amp;<span class="k">self</span>) -&gt; bool;

    <span class="c">/// Purge unpinned dead slots for at most \`work\` slots; returns the slots examined.</span>
    <span class="k">fn</span> compact_step(&amp;<span class="k">mut</span> <span class="k">self</span>, work: usize) -&gt; usize;
}

<span class="k">impl</span>&lt;E: Keyed&gt; Slots <span class="k">for</span> Collection&lt;E&gt; {
    <span class="k">fn</span> get_slot(&amp;<span class="k">self</span>, guid: &amp;str) -&gt; Option&lt;usize&gt; {
        Collection::get_slot(<span class="k">self</span>, guid)
    }

    <span class="k">fn</span> key_at(&amp;<span class="k">self</span>, slot: usize) -&gt; &amp;str {
        <span class="k">self</span>.get_item(slot).key()
    }

    <span class="k">fn</span> is_dead(&amp;<span class="k">self</span>, slot: usize) -&gt; bool {
        Collection::is_dead(<span class="k">self</span>, slot)
    }

    <span class="k">fn</span> set_dead(&amp;<span class="k">mut</span> <span class="k">self</span>, slot: usize, dead: bool) {
        Collection::set_dead(<span class="k">self</span>, slot, dead)
    }

    <span class="k">fn</span> get_tomb(&amp;<span class="k">self</span>, slot: usize) -&gt; Option&lt;Rc&lt;Tomb&gt;&gt; {
        Collection::get_tomb(<span class="k">self</span>, slot)
    }

    <span class="k">fn</span> set_tomb(&amp;<span class="k">mut</span> <span class="k">self</span>, slot: usize, tomb: &amp;Rc&lt;Tomb&gt;) {
        Collection::set_tomb(<span class="k">self</span>, slot, tomb)
    }

    <span class="k">fn</span> number_of_dead(&amp;<span class="k">self</span>) -&gt; usize {
        Collection::number_of_dead(<span class="k">self</span>)
    }

    <span class="k">fn</span> number_of_slots(&amp;<span class="k">self</span>) -&gt; usize {
        Collection::number_of_slots(<span class="k">self</span>)
    }

    <span class="k">fn</span> is_compacting(&amp;<span class="k">self</span>) -&gt; bool {
        Collection::is_compacting(<span class="k">self</span>)
    }

    <span class="k">fn</span> compact_step(&amp;<span class="k">mut</span> <span class="k">self</span>, work: usize) -&gt; usize {
        Collection::compact_step(<span class="k">self</span>, work)
    }
}

<span class="c">/// The Collection of that name, the Rust spelling of getattr(objects, collection).</span>
<span class="k">fn</span> list&lt;'a&gt;(objects: &amp;'a Objects, collection: &amp;str) -&gt; Option&lt;&amp;'a <span class="k">dyn</span> Slots&gt; {
    <span class="k">match</span> collection {
        &quot;<span class="s">points</span>&quot; =&gt; Some(&amp;objects.points),
        &quot;<span class="s">lines</span>&quot; =&gt; Some(&amp;objects.lines),
        &quot;<span class="s">planes</span>&quot; =&gt; Some(&amp;objects.planes),
        &quot;<span class="s">bboxes</span>&quot; =&gt; Some(&amp;objects.bboxes),
        &quot;<span class="s">polylines</span>&quot; =&gt; Some(&amp;objects.polylines),
        &quot;<span class="s">pointclouds</span>&quot; =&gt; Some(&amp;objects.pointclouds),
        &quot;<span class="s">meshes</span>&quot; =&gt; Some(&amp;objects.meshes),
        &quot;<span class="s">nurbscurves</span>&quot; =&gt; Some(&amp;objects.nurbscurves),
        &quot;<span class="s">nurbssurfaces</span>&quot; =&gt; Some(&amp;objects.nurbssurfaces),
        &quot;<span class="s">breps</span>&quot; =&gt; Some(&amp;objects.breps),
        &quot;<span class="s">elements</span>&quot; =&gt; Some(&amp;objects.elements),
        &quot;<span class="s">components</span>&quot; =&gt; Some(&amp;objects.components),
        &quot;<span class="s">instances</span>&quot; =&gt; Some(&amp;objects.instances),
        _ =&gt; None,
    }
}

<span class="c">/// The Collection of that name, mutable.</span>
<span class="k">fn</span> list_mut&lt;'a&gt;(objects: &amp;'a <span class="k">mut</span> Objects, collection: &amp;str) -&gt; Option&lt;&amp;'a <span class="k">mut</span> <span class="k">dyn</span> Slots&gt; {
    <span class="k">match</span> collection {
        &quot;<span class="s">points</span>&quot; =&gt; Some(&amp;<span class="k">mut</span> objects.points),
        &quot;<span class="s">lines</span>&quot; =&gt; Some(&amp;<span class="k">mut</span> objects.lines),
        &quot;<span class="s">planes</span>&quot; =&gt; Some(&amp;<span class="k">mut</span> objects.planes),
        &quot;<span class="s">bboxes</span>&quot; =&gt; Some(&amp;<span class="k">mut</span> objects.bboxes),
        &quot;<span class="s">polylines</span>&quot; =&gt; Some(&amp;<span class="k">mut</span> objects.polylines),
        &quot;<span class="s">pointclouds</span>&quot; =&gt; Some(&amp;<span class="k">mut</span> objects.pointclouds),
        &quot;<span class="s">meshes</span>&quot; =&gt; Some(&amp;<span class="k">mut</span> objects.meshes),
        &quot;<span class="s">nurbscurves</span>&quot; =&gt; Some(&amp;<span class="k">mut</span> objects.nurbscurves),
        &quot;<span class="s">nurbssurfaces</span>&quot; =&gt; Some(&amp;<span class="k">mut</span> objects.nurbssurfaces),
        &quot;<span class="s">breps</span>&quot; =&gt; Some(&amp;<span class="k">mut</span> objects.breps),
        &quot;<span class="s">elements</span>&quot; =&gt; Some(&amp;<span class="k">mut</span> objects.elements),
        &quot;<span class="s">components</span>&quot; =&gt; Some(&amp;<span class="k">mut</span> objects.components),
        &quot;<span class="s">instances</span>&quot; =&gt; Some(&amp;<span class="k">mut</span> objects.instances),
        _ =&gt; None,
    }
}

<span class="c">/// A deep copy of a list, every object cloned, guids included.</span>
<span class="k">fn</span> deep&lt;T: Clone&gt;(list: &amp;Collection&lt;Rc&lt;T&gt;&gt;) -&gt; Collection&lt;Rc&lt;T&gt;&gt;
<span class="k">where</span>
    Rc&lt;T&gt;: Keyed,
{
    list.iter().map(|item| Rc::new((**item).clone())).collect()
}

<span class="c">/// A copy of a list with each object's world placement baked into its coordinates.</span>
<span class="k">fn</span> baked&lt;T: Clone&gt;(
    list: &amp;Collection&lt;Rc&lt;T&gt;&gt;,
    world: &amp;HashMap&lt;String, Xform&gt;,
    bake: <span class="k">impl</span> Fn(&amp;<span class="k">mut</span> T, &amp;Xform),
) -&gt; Collection&lt;Rc&lt;T&gt;&gt;
<span class="k">where</span>
    Rc&lt;T&gt;: Keyed,
{
    list.iter()
        .map(|item| {
            <span class="k">let</span> <span class="k">mut</span> copy = (**item).clone();

            <span class="k">if</span> <span class="k">let</span> Some(xform) = world.get(item.key()) {
                <span class="k">if</span> !xform.is_identity() {
                    bake(&amp;<span class="k">mut</span> copy, xform);
                }
            }

            Rc::new(copy)
        })
        .collect()
}

<span class="c">/// A deep copy of every vector and every object in it, guids included.</span>
<span class="k">fn</span> clone_objects(objects: &amp;Objects) -&gt; Objects {
    <span class="k">let</span> <span class="k">mut</span> out = objects.clone();
    out.points = deep(&amp;objects.points);
    out.lines = deep(&amp;objects.lines);
    out.planes = deep(&amp;objects.planes);
    out.bboxes = deep(&amp;objects.bboxes);
    out.polylines = deep(&amp;objects.polylines);
    out.pointclouds = deep(&amp;objects.pointclouds);
    out.meshes = deep(&amp;objects.meshes);
    out.nurbscurves = deep(&amp;objects.nurbscurves);
    out.nurbssurfaces = deep(&amp;objects.nurbssurfaces);
    out.breps = deep(&amp;objects.breps);
    out.elements = deep(&amp;objects.elements);
    out.instances = deep(&amp;objects.instances);

    out
}

<span class="c">/// The vectors of objects re-pointed at lookup: \`Rc::make_mut\` on a lookup entry splits it from its vector, and the lookup is the mutable truth.</span>
<span class="k">fn</span> synced(objects: &amp;Objects, lookup: &amp;HashMap&lt;String, Geometry&gt;) -&gt; Objects {
    <span class="k">let</span> <span class="k">mut</span> objects = objects.clone();
    repoint(&amp;<span class="k">mut</span> objects, lookup);

    objects
}

<span class="c">/// Point the live slot of guid at held when it stores another pointer.</span>
<span class="k">fn</span> resync&lt;T&gt;(list: &amp;<span class="k">mut</span> Collection&lt;Rc&lt;T&gt;&gt;, guid: &amp;str, held: &amp;Rc&lt;T&gt;)
<span class="k">where</span>
    Rc&lt;T&gt;: Keyed,
{
    <span class="k">let</span> Some(slot) = list.get_slot(guid) <span class="k">else</span> {
        <span class="k">return</span>;
    };

    <span class="k">if</span> !Rc::ptr_eq(list.get_item(slot), held) {
        list.set_item(slot, Rc::clone(held));
    }
}

<span class="c">/// Point every live slot whose guid lookup holds with another value at the lookup value.</span>
<span class="k">fn</span> repoint(objects: &amp;<span class="k">mut</span> Objects, lookup: &amp;HashMap&lt;String, Geometry&gt;) {
    <span class="k">for</span> (guid, geometry) <span class="k">in</span> lookup {
        <span class="k">match</span> geometry {
            Geometry::Point(g) =&gt; resync(&amp;<span class="k">mut</span> objects.points, guid, g),
            Geometry::Line(g) =&gt; resync(&amp;<span class="k">mut</span> objects.lines, guid, g),
            Geometry::Plane(g) =&gt; resync(&amp;<span class="k">mut</span> objects.planes, guid, g),
            Geometry::OBB(g) =&gt; resync(&amp;<span class="k">mut</span> objects.bboxes, guid, g),
            Geometry::Polyline(g) =&gt; resync(&amp;<span class="k">mut</span> objects.polylines, guid, g),
            Geometry::PointCloud(g) =&gt; resync(&amp;<span class="k">mut</span> objects.pointclouds, guid, g),
            Geometry::Mesh(g) =&gt; resync(&amp;<span class="k">mut</span> objects.meshes, guid, g),
            Geometry::NurbsCurve(g) =&gt; resync(&amp;<span class="k">mut</span> objects.nurbscurves, guid, g),
            Geometry::NurbsSurface(g) =&gt; resync(&amp;<span class="k">mut</span> objects.nurbssurfaces, guid, g),
            Geometry::BRep(g) =&gt; resync(&amp;<span class="k">mut</span> objects.breps, guid, g),
            Geometry::Element(g) =&gt; resync(&amp;<span class="k">mut</span> objects.elements, guid, g),
        }
    }
}

<span class="c">/// Index every live entry of a list that lookup lacks, wrapped as its Geometry variant.</span>
<span class="k">fn</span> index&lt;T&gt;(
    list: &amp;Collection&lt;Rc&lt;T&gt;&gt;,
    lookup: &amp;<span class="k">mut</span> HashMap&lt;String, Geometry&gt;,
    wrap: <span class="k">fn</span>(Rc&lt;T&gt;) -&gt; Geometry,
) <span class="k">where</span>
    Rc&lt;T&gt;: Keyed,
{
    <span class="k">for</span> item <span class="k">in</span> list {
        <span class="k">if</span> !lookup.contains_key(item.key()) {
            lookup.insert(item.key().to_string(), wrap(Rc::clone(item)));
        }
    }
}

<span class="c">/// Index every live slot lookup lacks, then push every geometry only lookup holds, in guid order.</span>
<span class="k">fn</span> adopt(objects: &amp;<span class="k">mut</span> Objects, lookup: &amp;<span class="k">mut</span> HashMap&lt;String, Geometry&gt;) {
    index(&amp;objects.points, lookup, Geometry::Point);
    index(&amp;objects.lines, lookup, Geometry::Line);
    index(&amp;objects.planes, lookup, Geometry::Plane);
    index(&amp;objects.bboxes, lookup, Geometry::OBB);
    index(&amp;objects.polylines, lookup, Geometry::Polyline);
    index(&amp;objects.pointclouds, lookup, Geometry::PointCloud);
    index(&amp;objects.meshes, lookup, Geometry::Mesh);
    index(&amp;objects.nurbscurves, lookup, Geometry::NurbsCurve);
    index(&amp;objects.nurbssurfaces, lookup, Geometry::NurbsSurface);
    index(&amp;objects.breps, lookup, Geometry::BRep);
    index(&amp;objects.elements, lookup, Geometry::Element);
    <span class="k">let</span> <span class="k">mut</span> orphans: Vec&lt;&amp;Geometry&gt; = Vec::new();

    <span class="k">for</span> geometry <span class="k">in</span> lookup.values() {
        <span class="k">let</span> (collection, _) = collection_of(geometry);

        <span class="k">if</span> slot_of(objects, collection, geometry.guid()).is_none() {
            orphans.push(geometry);
        }
    }

    orphans.sort_by(|a, b| a.guid().cmp(b.guid()));

    <span class="k">for</span> geometry <span class="k">in</span> orphans {
        push(objects, &amp;Item::Geometry(geometry.clone()));
    }
}

<span class="c">/// The COLLECTIONS entry whose vector holds the type of geometry.</span>
<span class="k">fn</span> collection_of(geometry: &amp;Geometry) -&gt; (&amp;'static str, &amp;'static str) {
    <span class="k">match</span> geometry {
        Geometry::Point(_) =&gt; (&quot;<span class="s">points</span>&quot;, &quot;<span class="s">point</span>&quot;),
        Geometry::Line(_) =&gt; (&quot;<span class="s">lines</span>&quot;, &quot;<span class="s">line</span>&quot;),
        Geometry::Plane(_) =&gt; (&quot;<span class="s">planes</span>&quot;, &quot;<span class="s">plane</span>&quot;),
        Geometry::OBB(_) =&gt; (&quot;<span class="s">bboxes</span>&quot;, &quot;<span class="s">bbox</span>&quot;),
        Geometry::Polyline(_) =&gt; (&quot;<span class="s">polylines</span>&quot;, &quot;<span class="s">polyline</span>&quot;),
        Geometry::PointCloud(_) =&gt; (&quot;<span class="s">pointclouds</span>&quot;, &quot;<span class="s">pointcloud</span>&quot;),
        Geometry::Mesh(_) =&gt; (&quot;<span class="s">meshes</span>&quot;, &quot;<span class="s">mesh</span>&quot;),
        Geometry::NurbsCurve(_) =&gt; (&quot;<span class="s">nurbscurves</span>&quot;, &quot;<span class="s">nurbscurve</span>&quot;),
        Geometry::NurbsSurface(_) =&gt; (&quot;<span class="s">nurbssurfaces</span>&quot;, &quot;<span class="s">nurbssurface</span>&quot;),
        Geometry::BRep(_) =&gt; (&quot;<span class="s">breps</span>&quot;, &quot;<span class="s">brep</span>&quot;),
        Geometry::Element(_) =&gt; (&quot;<span class="s">elements</span>&quot;, &quot;<span class="s">element</span>&quot;),
    }
}

<span class="c">/// The COLLECTIONS entry whose list holds an item.</span>
<span class="k">fn</span> collection_for(item: &amp;Item) -&gt; (&amp;'static str, &amp;'static str) {
    <span class="k">match</span> item {
        Item::Geometry(geometry) =&gt; collection_of(geometry),
        Item::Component(_) =&gt; (&quot;<span class="s">components</span>&quot;, &quot;<span class="s">component</span>&quot;),
        Item::InstanceRef(_) =&gt; (&quot;<span class="s">instances</span>&quot;, &quot;<span class="s">instance</span>&quot;),
    }
}

<span class="c">/// The graph attribute prefix of the list of that name.</span>
<span class="k">fn</span> prefix_of(collection: &amp;str) -&gt; &amp;'static str {
    <span class="k">for</span> (name, prefix) <span class="k">in</span> COLLECTIONS {
        <span class="k">if</span> name == collection {
            <span class="k">return</span> prefix;
        }
    }

    &quot;&quot;
}

<span class="c">/// The live slot of a guid in the list of that name.</span>
<span class="k">fn</span> slot_of(objects: &amp;Objects, collection: &amp;str, guid: &amp;str) -&gt; Option&lt;usize&gt; {
    list(objects, collection)?.get_slot(guid)
}

<span class="c">/// The stored pointer in a slot of a list, dead or alive.</span>
<span class="k">fn</span> held&lt;T&gt;(list: &amp;Collection&lt;Rc&lt;T&gt;&gt;, slot: usize) -&gt; Rc&lt;T&gt; {
    Rc::clone(list.get_item(slot))
}

<span class="c">/// The item in a slot of the list of that name, dead or alive, as the stored pointer.</span>
<span class="k">fn</span> item_at(objects: &amp;Objects, collection: &amp;str, slot: usize) -&gt; Option&lt;Item&gt; {
    <span class="k">match</span> collection {
        &quot;<span class="s">points</span>&quot; =&gt; Some(Geometry::Point(held(&amp;objects.points, slot)).into()),
        &quot;<span class="s">lines</span>&quot; =&gt; Some(Geometry::Line(held(&amp;objects.lines, slot)).into()),
        &quot;<span class="s">planes</span>&quot; =&gt; Some(Geometry::Plane(held(&amp;objects.planes, slot)).into()),
        &quot;<span class="s">bboxes</span>&quot; =&gt; Some(Geometry::OBB(held(&amp;objects.bboxes, slot)).into()),
        &quot;<span class="s">polylines</span>&quot; =&gt; Some(Geometry::Polyline(held(&amp;objects.polylines, slot)).into()),
        &quot;<span class="s">pointclouds</span>&quot; =&gt; Some(Geometry::PointCloud(held(&amp;objects.pointclouds, slot)).into()),
        &quot;<span class="s">meshes</span>&quot; =&gt; Some(Geometry::Mesh(held(&amp;objects.meshes, slot)).into()),
        &quot;<span class="s">nurbscurves</span>&quot; =&gt; Some(Geometry::NurbsCurve(held(&amp;objects.nurbscurves, slot)).into()),
        &quot;<span class="s">nurbssurfaces</span>&quot; =&gt; Some(Geometry::NurbsSurface(held(&amp;objects.nurbssurfaces, slot)).into()),
        &quot;<span class="s">breps</span>&quot; =&gt; Some(Geometry::BRep(held(&amp;objects.breps, slot)).into()),
        &quot;<span class="s">elements</span>&quot; =&gt; Some(Geometry::Element(held(&amp;objects.elements, slot)).into()),
        &quot;<span class="s">components</span>&quot; =&gt; Some(Item::Component(objects.components.get_item(slot).clone())),
        &quot;<span class="s">instances</span>&quot; =&gt; Some(Item::InstanceRef(held(&amp;objects.instances, slot))),
        _ =&gt; None,
    }
}

<span class="c">/// Append an entry to a list and return its slot.</span>
<span class="k">fn</span> pushed&lt;E: Keyed&gt;(list: &amp;<span class="k">mut</span> Collection&lt;E&gt;, item: E) -&gt; usize {
    list.push(item);

    list.number_of_slots() - <span class="s">1</span>
}

<span class="c">/// Append an item to the list of its type and return its slot.</span>
<span class="k">fn</span> push(objects: &amp;<span class="k">mut</span> Objects, obj: &amp;Item) -&gt; usize {
    <span class="k">match</span> obj {
        Item::Geometry(Geometry::Point(g)) =&gt; pushed(&amp;<span class="k">mut</span> objects.points, Rc::clone(g)),
        Item::Geometry(Geometry::Line(g)) =&gt; pushed(&amp;<span class="k">mut</span> objects.lines, Rc::clone(g)),
        Item::Geometry(Geometry::Plane(g)) =&gt; pushed(&amp;<span class="k">mut</span> objects.planes, Rc::clone(g)),
        Item::Geometry(Geometry::OBB(g)) =&gt; pushed(&amp;<span class="k">mut</span> objects.bboxes, Rc::clone(g)),
        Item::Geometry(Geometry::Polyline(g)) =&gt; pushed(&amp;<span class="k">mut</span> objects.polylines, Rc::clone(g)),
        Item::Geometry(Geometry::PointCloud(g)) =&gt; pushed(&amp;<span class="k">mut</span> objects.pointclouds, Rc::clone(g)),
        Item::Geometry(Geometry::Mesh(g)) =&gt; pushed(&amp;<span class="k">mut</span> objects.meshes, Rc::clone(g)),
        Item::Geometry(Geometry::NurbsCurve(g)) =&gt; pushed(&amp;<span class="k">mut</span> objects.nurbscurves, Rc::clone(g)),
        Item::Geometry(Geometry::NurbsSurface(g)) =&gt; {
            pushed(&amp;<span class="k">mut</span> objects.nurbssurfaces, Rc::clone(g))
        }
        Item::Geometry(Geometry::BRep(g)) =&gt; pushed(&amp;<span class="k">mut</span> objects.breps, Rc::clone(g)),
        Item::Geometry(Geometry::Element(g)) =&gt; pushed(&amp;<span class="k">mut</span> objects.elements, Rc::clone(g)),
        Item::Component(component) =&gt; pushed(&amp;<span class="k">mut</span> objects.components, component.clone()),
        Item::InstanceRef(instance) =&gt; pushed(&amp;<span class="k">mut</span> objects.instances, Rc::clone(instance)),
    }
}

<span class="c">/// Put an item in a slot of the list of that name; an item of another type is left out.</span>
<span class="k">fn</span> store(objects: &amp;<span class="k">mut</span> Objects, collection: &amp;str, slot: usize, obj: &amp;Item) {
    <span class="k">if</span> collection_for(obj).<span class="s">0</span> != collection {
        <span class="k">return</span>;
    }

    <span class="k">match</span> obj {
        Item::Geometry(Geometry::Point(g)) =&gt; objects.points.set_item(slot, Rc::clone(g)),
        Item::Geometry(Geometry::Line(g)) =&gt; objects.lines.set_item(slot, Rc::clone(g)),
        Item::Geometry(Geometry::Plane(g)) =&gt; objects.planes.set_item(slot, Rc::clone(g)),
        Item::Geometry(Geometry::OBB(g)) =&gt; objects.bboxes.set_item(slot, Rc::clone(g)),
        Item::Geometry(Geometry::Polyline(g)) =&gt; objects.polylines.set_item(slot, Rc::clone(g)),
        Item::Geometry(Geometry::PointCloud(g)) =&gt; objects.pointclouds.set_item(slot, Rc::clone(g)),
        Item::Geometry(Geometry::Mesh(g)) =&gt; objects.meshes.set_item(slot, Rc::clone(g)),
        Item::Geometry(Geometry::NurbsCurve(g)) =&gt; objects.nurbscurves.set_item(slot, Rc::clone(g)),
        Item::Geometry(Geometry::NurbsSurface(g)) =&gt; {
            objects.nurbssurfaces.set_item(slot, Rc::clone(g))
        }
        Item::Geometry(Geometry::BRep(g)) =&gt; objects.breps.set_item(slot, Rc::clone(g)),
        Item::Geometry(Geometry::Element(g)) =&gt; objects.elements.set_item(slot, Rc::clone(g)),
        Item::Component(component) =&gt; objects.components.set_item(slot, component.clone()),
        Item::InstanceRef(instance) =&gt; objects.instances.set_item(slot, Rc::clone(instance)),
    }
}

<span class="c">/// Kill or revive a slot of the list of that name.</span>
<span class="k">fn</span> flag(objects: &amp;<span class="k">mut</span> Objects, collection: &amp;str, slot: usize, dead: bool) {
    <span class="k">if</span> <span class="k">let</span> Some(list) = list_mut(objects, collection) {
        list.set_dead(slot, dead);
    }
}

<span class="c">/// The tomb pinning a slot of the list of that name, while a record still holds it.</span>
<span class="k">fn</span> tomb_at(objects: &amp;Objects, collection: &amp;str, slot: usize) -&gt; Option&lt;Rc&lt;Tomb&gt;&gt; {
    list(objects, collection)?.get_tomb(slot)
}

<span class="c">/// Pin a slot of the list of that name to a tomb.</span>
<span class="k">fn</span> pin(objects: &amp;<span class="k">mut</span> Objects, collection: &amp;str, slot: usize, tomb: &amp;Rc&lt;Tomb&gt;) {
    <span class="k">if</span> <span class="k">let</span> Some(list) = list_mut(objects, collection) {
        list.set_tomb(slot, tomb);
    }
}

<span class="c">/// Whether two items are the same stored pointer; a component is never, so the map value is stored back.</span>
<span class="k">fn</span> same(a: &amp;Item, b: &amp;Item) -&gt; bool {
    <span class="k">match</span> (a, b) {
        (Item::Geometry(Geometry::OBB(x)), Item::Geometry(Geometry::OBB(y))) =&gt; Rc::ptr_eq(x, y),
        (Item::Geometry(Geometry::BRep(x)), Item::Geometry(Geometry::BRep(y))) =&gt; Rc::ptr_eq(x, y),
        (Item::Geometry(Geometry::Element(x)), Item::Geometry(Geometry::Element(y))) =&gt; {
            Rc::ptr_eq(x, y)
        }
        (Item::Geometry(Geometry::Line(x)), Item::Geometry(Geometry::Line(y))) =&gt; Rc::ptr_eq(x, y),
        (Item::Geometry(Geometry::Mesh(x)), Item::Geometry(Geometry::Mesh(y))) =&gt; Rc::ptr_eq(x, y),
        (Item::Geometry(Geometry::NurbsCurve(x)), Item::Geometry(Geometry::NurbsCurve(y))) =&gt; {
            Rc::ptr_eq(x, y)
        }
        (Item::Geometry(Geometry::NurbsSurface(x)), Item::Geometry(Geometry::NurbsSurface(y))) =&gt; {
            Rc::ptr_eq(x, y)
        }
        (Item::Geometry(Geometry::Plane(x)), Item::Geometry(Geometry::Plane(y))) =&gt; {
            Rc::ptr_eq(x, y)
        }
        (Item::Geometry(Geometry::Point(x)), Item::Geometry(Geometry::Point(y))) =&gt; {
            Rc::ptr_eq(x, y)
        }
        (Item::Geometry(Geometry::PointCloud(x)), Item::Geometry(Geometry::PointCloud(y))) =&gt; {
            Rc::ptr_eq(x, y)
        }
        (Item::Geometry(Geometry::Polyline(x)), Item::Geometry(Geometry::Polyline(y))) =&gt; {
            Rc::ptr_eq(x, y)
        }
        (Item::InstanceRef(x), Item::InstanceRef(y)) =&gt; Rc::ptr_eq(x, y),
        _ =&gt; <span class="s">false</span>,
    }
}

<span class="c">/// Move geometry in place: an element is placed, anything else transformed; identity leaves it untouched.</span>
<span class="k">fn</span> place(geometry: &amp;<span class="k">mut</span> Geometry, xform: &amp;Xform) {
    <span class="k">if</span> xform.is_identity() {
        <span class="k">return</span>;
    }

    <span class="k">match</span> geometry {
        Geometry::Element(g) =&gt; Rc::make_mut(g).place(xform),
        Geometry::OBB(g) =&gt; Rc::make_mut(g).transform(xform),
        Geometry::BRep(g) =&gt; Rc::make_mut(g).transform(xform),
        Geometry::Line(g) =&gt; Rc::make_mut(g).transform(xform),
        Geometry::Mesh(g) =&gt; {
            Rc::make_mut(g).transform(xform);
        }

        Geometry::NurbsCurve(g) =&gt; {
            Rc::make_mut(g).transform(xform);
        }

        Geometry::NurbsSurface(g) =&gt; {
            Rc::make_mut(g).transform(xform);
        }

        Geometry::Plane(g) =&gt; Rc::make_mut(g).transform(xform),
        Geometry::Point(g) =&gt; Rc::make_mut(g).transform(xform),
        Geometry::PointCloud(g) =&gt; Rc::make_mut(g).transform(xform),
        Geometry::Polyline(g) =&gt; Rc::make_mut(g).transform(xform),
    }
}

<span class="c">/// The definition copied as the instance, its guid and name and on an element its features, then moved by xform.</span>
<span class="k">fn</span> resolve(instance: &amp;InstanceRef, definition: &amp;Geometry, xform: &amp;Xform) -&gt; Geometry {
    <span class="k">let</span> <span class="k">mut</span> copy = clone(definition);
    copy.set_guid(instance.guid());
    copy.set_name(&amp;instance.name);

    <span class="k">if</span> <span class="k">let</span> Geometry::Element(element) = &amp;<span class="k">mut</span> copy {
        <span class="k">for</span> feature <span class="k">in</span> &amp;instance.features {
            Rc::make_mut(element).add_feature(feature.clone());
        }
    }

    place(&amp;<span class="k">mut</span> copy, xform);

    copy
}

<span class="c">/// Inflated box around the placed points, around the origin when there are none.</span>
<span class="k">fn</span> placed_box(points: &amp;[Point], xform: &amp;Xform, inflate: f64) -&gt; OBB {
    <span class="k">if</span> points.is_empty() {
        <span class="k">return</span> OBB::from_point(&amp;Point::new(<span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>, <span class="s">0</span>.<span class="s">0</span>), inflate);
    }

    <span class="k">let</span> <span class="k">mut</span> placed = Vec::with_capacity(points.len());

    <span class="k">for</span> point <span class="k">in</span> points {
        placed.push(xform.transform_point(point));
    }

    OBB::from_points(&amp;placed, inflate, None)
}

<span class="c">/// The point on the ray closest to point when it lies ahead and within tolerance.</span>
<span class="k">fn</span> ray_point(ray: &amp;Line, point: &amp;Point, tolerance: f64) -&gt; Option&lt;Point&gt; {
    <span class="k">let</span> ray_dir = ray.end() - ray.start();
    <span class="k">let</span> to_point = point - &amp;ray.start();
    <span class="k">let</span> t = to_point.dot(&amp;ray_dir) / ray_dir.dot(&amp;ray_dir);

    <span class="k">if</span> t &lt; <span class="s">0</span>.<span class="s">0</span> {
        <span class="k">return</span> None;
    }

    <span class="k">let</span> closest = ray.start() + ray_dir * t;

    <span class="k">if</span> point.distance(&amp;closest, None) &gt; tolerance {
        <span class="k">return</span> None;
    }

    Some(closest)
}

<span class="c">/// The segment hit closest to the ray start.</span>
<span class="k">fn</span> ray_polyline(ray: &amp;Line, polyline: &amp;Polyline, tolerance: f64) -&gt; Option&lt;Point&gt; {
    <span class="k">let</span> <span class="k">mut</span> closest: Option&lt;Point&gt; = None;
    <span class="k">let</span> <span class="k">mut</span> min_dist = f64::INFINITY;

    <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..polyline.segment_count() {
        <span class="k">let</span> segment = Line::from_points(&amp;polyline.get_point(i)?, &amp;polyline.get_point(i + <span class="s">1</span>)?);
        <span class="k">let</span> Some(hit) = line_line(ray, &amp;segment, tolerance) <span class="k">else</span> {
            <span class="k">continue</span>;
        };
        <span class="k">let</span> dist = ray.start().distance(&amp;hit, None);

        <span class="k">if</span> dist &lt; min_dist {
            min_dist = dist;
            closest = Some(hit);
        }
    }

    closest
}

<span class="c">/// The ray point closest to a cloud point within tolerance.</span>
<span class="k">fn</span> ray_pointcloud(ray: &amp;Line, pointcloud: &amp;PointCloud, tolerance: f64) -&gt; Option&lt;Point&gt; {
    <span class="k">let</span> <span class="k">mut</span> closest: Option&lt;Point&gt; = None;
    <span class="k">let</span> <span class="k">mut</span> min_dist = f64::INFINITY;

    <span class="k">for</span> point <span class="k">in</span> pointcloud.get_points() {
        <span class="k">let</span> Some(hit) = ray_point(ray, &amp;point, tolerance) <span class="k">else</span> {
            <span class="k">continue</span>;
        };
        <span class="k">let</span> dist = point.distance(&amp;hit, None);

        <span class="k">if</span> dist &lt; min_dist {
            min_dist = dist;
            closest = Some(hit);
        }
    }

    closest
}

<span class="c">/// The first hit of the ray on the placed mesh, tested in the mesh frame.</span>
<span class="k">fn</span> ray_mesh(ray: &amp;Line, mesh: &amp;Mesh, tolerance: f64, placement: &amp;Xform) -&gt; Option&lt;Point&gt; {
    <span class="k">let</span> inverse = placement.inverse()?;
    <span class="k">let</span> local_ray = Line::from_points(
        &amp;inverse.transform_point(&amp;ray.start()),
        &amp;inverse.transform_point(&amp;ray.end()),
    );
    <span class="k">let</span> hits = ray_mesh_bvh(&amp;local_ray, mesh, tolerance, <span class="s">true</span>)?;

    Some(placement.transform_point(hits.first()?))
}

<span class="c">/// The vertices of a BRep and a 3x3 sample of each of its surfaces.</span>
<span class="k">fn</span> brep_points(brep: &amp;BRep) -&gt; Vec&lt;Point&gt; {
    <span class="k">let</span> <span class="k">mut</span> points: Vec&lt;Point&gt; = Vec::new();

    <span class="k">for</span> vertex <span class="k">in</span> &amp;brep.m_vertices {
        points.push(vertex.point.clone());
    }

    <span class="k">for</span> surface <span class="k">in</span> &amp;brep.m_surfaces {
        <span class="k">let</span> (Some((u0, u1)), Some((v0, v1))) = (surface.domain(<span class="s">0</span>), surface.domain(<span class="s">1</span>)) <span class="k">else</span> {
            <span class="k">continue</span>;
        };

        <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..=<span class="s">2usize</span> {
            <span class="k">for</span> j <span class="k">in</span> <span class="s">0</span>..=<span class="s">2usize</span> {
                <span class="k">let</span> u = u0 + (u1 - u0) * i <span class="k">as</span> f64 / <span class="s">2</span>.<span class="s">0</span>;
                <span class="k">let</span> v = v0 + (v1 - v0) * j <span class="k">as</span> f64 / <span class="s">2</span>.<span class="s">0</span>;

                <span class="k">if</span> <span class="k">let</span> Some(point) = surface.point_at(u, v) {
                    points.push(point);
                }
            }
        }
    }

    points
}

<span class="c">/// The points whose box bounds a geometry: vertices, control points or surface samples.</span>
<span class="k">fn</span> box_points(geometry: &amp;Geometry) -&gt; Vec&lt;Point&gt; {
    <span class="k">let</span> <span class="k">mut</span> points: Vec&lt;Point&gt; = Vec::new();

    <span class="k">match</span> geometry {
        Geometry::Line(line) =&gt; {
            points.push(line.start());
            points.push(line.end());
        }

        Geometry::Polyline(polyline) =&gt; points = polyline.get_points(),
        Geometry::PointCloud(pointcloud) =&gt; points = pointcloud.get_points(),
        Geometry::Mesh(mesh) =&gt; {
            <span class="k">for</span> vertex <span class="k">in</span> mesh.vertex.values() {
                points.push(vertex.position());
            }
        }

        Geometry::BRep(brep) =&gt; points = brep_points(brep),

        Geometry::NurbsCurve(nurbscurve) =&gt; {
            <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..nurbscurve.cv_count() {
                <span class="k">if</span> <span class="k">let</span> Some(point) = nurbscurve.get_cv(i) {
                    points.push(point);
                }
            }
        }

        Geometry::NurbsSurface(nurbssurface) =&gt; {
            <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..nurbssurface.cv_count(<span class="s">0</span>) {
                <span class="k">for</span> j <span class="k">in</span> <span class="s">0</span>..nurbssurface.cv_count(<span class="s">1</span>) {
                    <span class="k">if</span> <span class="k">let</span> Some(point) = nurbssurface.get_cv(i, j) {
                        points.push(point);
                    }
                }
            }
        }

        _ =&gt; {}
    }

    points
}

<span class="c">/// Whether guid is a graph node held by an object, instance or component.</span>
<span class="k">fn</span> registered(session: &amp;Session, guid: &amp;str) -&gt; bool {
    <span class="k">let</span> held = session.lookup.contains_key(guid)
        || session.instance_lookup.contains_key(guid)
        || session.component_lookup.contains_key(guid);

    session.graph.has_node(guid) &amp;&amp; held
}

<span class="c">/// One object a ray touched: which one, where, and how far from the ray origin.</span>
#[derive(Debug, Clone)]
<span class="k">pub</span> <span class="k">struct</span> RayHit {
    <span class="k">pub</span> guid: String,     <span class="c">// GUID of the hit object.</span>
    <span class="k">pub</span> hit_point: Point, <span class="c">// Intersection point in world coordinates.</span>
    <span class="k">pub</span> distance: f64,    <span class="c">// Distance from the ray origin.</span>
}

<span class="c">/// Work units of one idle purge or checkpoint step, about 2 ms: one raw slot, child, vertex or entry each.</span>
<span class="k">pub</span> <span class="k">const</span> PURGE_WORK: usize = <span class="s">16_384</span>;

<span class="k">const</span> HEAD: usize = <span class="s">0</span>; <span class="c">// Checkpoint phase: the session name and guid.</span>
<span class="k">const</span> OBJECTS: usize = <span class="s">1</span>; <span class="c">// Checkpoint phases 1..=13: the objects lists.</span>
<span class="k">const</span> TREE: usize = <span class="s">14</span>; <span class="c">// Checkpoint phase: the tree, depth first.</span>
<span class="k">const</span> VERTICES: usize = <span class="s">15</span>; <span class="c">// Checkpoint phase: the graph vertices.</span>
<span class="k">const</span> EDGES: usize = <span class="s">16</span>; <span class="c">// Checkpoint phase: the graph edges.</span>
<span class="k">const</span> ORDERED: usize = <span class="s">17</span>; <span class="c">// Checkpoint phases 17..=27: xforms in order() sequence.</span>
<span class="k">const</span> REST: usize = <span class="s">28</span>; <span class="c">// Checkpoint phase: xforms outside order(), by guid.</span>
<span class="k">const</span> DEFINITIONS: usize = <span class="s">29</span>; <span class="c">// Checkpoint phases 29..=41: the definitions lists.</span>
<span class="k">const</span> INTERACTIONS: usize = <span class="s">42</span>; <span class="c">// Checkpoint phase: the interactions, by edge guid.</span>
<span class="k">const</span> ASSEMBLY: usize = <span class="s">43</span>; <span class="c">// Checkpoint phase: the sections joined into one message.</span>
<span class="k">const</span> CHUNK: usize = <span class="s">64</span> &lt;&lt; <span class="s">10</span>; <span class="c">// Bytes past which a finished tree node's chunks move instead of being copied.</span>

<span class="c">/// Field numbers the checkpoint writer frames by hand, mirrored from session.proto; the tests check them against the prost messages.</span>
<span class="k">pub</span>(<span class="k">crate</span>) <span class="k">struct</span> Tags {
    <span class="k">pub</span> sections: [u32; <span class="s">7</span>], <span class="c">// Session field per section, 0 for one written framed: head, objects, tree, graph, xforms, definitions, interactions.</span>
    <span class="k">pub</span> root: u32,          <span class="c">// Tree.root</span>
    <span class="k">pub</span> children: u32,      <span class="c">// TreeNode.children</span>
    <span class="k">pub</span> lists: [u32; <span class="s">13</span>],   <span class="c">// Objects field per COLLECTIONS entry.</span>
}

<span class="k">pub</span>(<span class="k">crate</span>) <span class="k">const</span> TAGS: Tags = Tags {
    sections: [<span class="s">0</span>, <span class="s">3</span>, <span class="s">4</span>, <span class="s">5</span>, <span class="s">0</span>, <span class="s">8</span>, <span class="s">0</span>],
    root: <span class="s">3</span>,
    children: <span class="s">4</span>,
    lists: [<span class="s">3</span>, <span class="s">4</span>, <span class="s">5</span>, <span class="s">6</span>, <span class="s">7</span>, <span class="s">8</span>, <span class="s">9</span>, <span class="s">12</span>, <span class="s">13</span>, <span class="s">14</span>, <span class="s">15</span>, <span class="s">16</span>, <span class="s">18</span>],
};

<span class="c">/// A tree node being written.</span>
<span class="k">struct</span> Frame {
    node: Rc&lt;RefCell&lt;TreeNode&gt;&gt;, <span class="c">// The node.</span>
    next: usize,                 <span class="c">// Its next raw child.</span>
    chunks: Vec&lt;Vec&lt;u8&gt;&gt;,        <span class="c">// Its bytes so far.</span>
}

<span class="k">impl</span> Frame {
    <span class="c">/// Open a node with its head bytes.</span>
    <span class="k">fn</span> new(node: Rc&lt;RefCell&lt;TreeNode&gt;&gt;) -&gt; <span class="k">Self</span> {
        <span class="k">let</span> head = node_head(&amp;node.borrow());

        <span class="k">Self</span> {
            node,
            next: <span class="s">0</span>,
            chunks: vec![head],
        }
    }
}

<span class="c">/// A resumable protobuf writer over a session: live entries only, the layout to_proto encodes.</span>
<span class="k">struct</span> Checkpoint {
    revision: u64,          <span class="c">// The revision it writes.</span>
    phase: usize,           <span class="c">// The section being written.</span>
    cursor: usize,          <span class="c">// Slot or entry count in the phase.</span>
    key: String,            <span class="c">// The last key or guid written.</span>
    stack: Vec&lt;Frame&gt;,      <span class="c">// Nodes being written, root first.</span>
    tree: Vec&lt;Vec&lt;u8&gt;&gt;,     <span class="c">// The framed root, in chunks after the Tree head.</span>
    sections: Vec&lt;Vec&lt;u8&gt;&gt;, <span class="c">// The seven Session fields.</span>
    out: Vec&lt;u8&gt;,           <span class="c">// The joined message.</span>
    hits: usize,            <span class="c">// Xforms entries order() reached, every one once the rest scan is done.</span>
    rest: BTreeSet&lt;String&gt;, <span class="c">// Xforms guids outside order(), in guid order.</span>
}

<span class="k">impl</span> Checkpoint {
    <span class="c">/// Construct a writer at the first phase.</span>
    <span class="k">fn</span> new(revision: u64) -&gt; <span class="k">Self</span> {
        <span class="k">Self</span> {
            revision,
            phase: HEAD,
            cursor: <span class="s">0</span>,
            key: String::new(),
            stack: Vec::new(),
            tree: Vec::new(),
            sections: vec![Vec::new(); <span class="s">7</span>],
            out: Vec::new(),
            hits: <span class="s">0</span>,
            rest: BTreeSet::new(),
        }
    }
}

<span class="c">/// The key and length of a length-delimited protobuf field.</span>
<span class="k">fn</span> prefix(tag: u32, length: usize) -&gt; Vec&lt;u8&gt; {
    <span class="k">let</span> <span class="k">mut</span> bytes = Vec::new();
    prost::encoding::encode_key(tag, prost::encoding::WireType::LengthDelimited, &amp;<span class="k">mut</span> bytes);
    prost::encoding::encode_varint(length <span class="k">as</span> u64, &amp;<span class="k">mut</span> bytes);

    bytes
}

<span class="c">/// Append bytes to a chunked buffer: a small piece is copied into the last chunk, a large one moves whole.</span>
<span class="k">fn</span> append(chunks: &amp;<span class="k">mut</span> Vec&lt;Vec&lt;u8&gt;&gt;, bytes: Vec&lt;u8&gt;) {
    <span class="k">match</span> chunks.last_mut() {
        Some(last) <span class="k">if</span> bytes.len() &lt;= CHUNK &amp;&amp; last.len() &lt; CHUNK =&gt; last.extend(bytes),
        _ =&gt; chunks.push(bytes),
    }
}

<span class="c">/// Encode the live entries of a list from slot \`start\` for at most \`work\` slots, each framed under tag; returns the end slot and the slot count.</span>
<span class="k">fn</span> emit&lt;E&gt;(
    list: &amp;Collection&lt;E&gt;,
    tag: u32,
    start: usize,
    work: usize,
    buffer: &amp;<span class="k">mut</span> Vec&lt;u8&gt;,
    entry: <span class="k">impl</span> Fn(&amp;E) -&gt; Vec&lt;u8&gt;,
) -&gt; (usize, usize) {
    <span class="k">let</span> total = list.number_of_slots();
    <span class="k">let</span> end = total.min(start.saturating_add(work));

    <span class="k">for</span> slot <span class="k">in</span> start..end {
        <span class="k">if</span> list.is_dead(slot) {
            <span class="k">continue</span>;
        }

        <span class="k">let</span> bytes = entry(list.get_item(slot));
        buffer.extend(prefix(tag, bytes.len()));
        buffer.extend(bytes);
    }

    (end, total)
}

<span class="c">/// The lookup's pointer for an entry when it holds one of that type, else the entry itself.</span>
<span class="k">fn</span> truth&lt;'a, T: FromGeometry&gt;(lookup: &amp;'a HashMap&lt;String, Geometry&gt;, item: &amp;'a Rc&lt;T&gt;) -&gt; &amp;'a T
<span class="k">where</span>
    Rc&lt;T&gt;: Keyed,
{
    lookup
        .get(item.key())
        .and_then(T::from_geometry)
        .unwrap_or(item.as_ref())
}

<span class="c">/// Append the live entries of the geometry list of that name from slot \`start\` for at most \`work\` slots, each as its lookup entry; returns the end slot and the slot count.</span>
<span class="k">fn</span> emit_geometry(
    objects: &amp;Objects,
    lookup: &amp;HashMap&lt;String, Geometry&gt;,
    name: &amp;str,
    tag: u32,
    start: usize,
    work: usize,
    buffer: &amp;<span class="k">mut</span> Vec&lt;u8&gt;,
) -&gt; (usize, usize) {
    <span class="k">use</span> prost::Message;

    <span class="k">match</span> name {
        &quot;<span class="s">points</span>&quot; =&gt; emit(&amp;objects.points, tag, start, work, buffer, |item| {
            truth(lookup, item).to_proto().encode_to_vec()
        }),
        &quot;<span class="s">lines</span>&quot; =&gt; emit(&amp;objects.lines, tag, start, work, buffer, |item| {
            truth(lookup, item).to_proto().encode_to_vec()
        }),
        &quot;<span class="s">planes</span>&quot; =&gt; emit(&amp;objects.planes, tag, start, work, buffer, |item| {
            truth(lookup, item).to_proto().encode_to_vec()
        }),
        &quot;<span class="s">bboxes</span>&quot; =&gt; emit(&amp;objects.bboxes, tag, start, work, buffer, |item| {
            truth(lookup, item).to_proto().encode_to_vec()
        }),
        &quot;<span class="s">polylines</span>&quot; =&gt; emit(&amp;objects.polylines, tag, start, work, buffer, |item| {
            truth(lookup, item).to_proto().encode_to_vec()
        }),
        &quot;<span class="s">pointclouds</span>&quot; =&gt; emit(&amp;objects.pointclouds, tag, start, work, buffer, |item| {
            truth(lookup, item).to_proto().encode_to_vec()
        }),
        &quot;<span class="s">meshes</span>&quot; =&gt; emit(&amp;objects.meshes, tag, start, work, buffer, |item| {
            truth(lookup, item).to_proto().encode_to_vec()
        }),
        &quot;<span class="s">nurbscurves</span>&quot; =&gt; emit(&amp;objects.nurbscurves, tag, start, work, buffer, |item| {
            truth(lookup, item).to_proto().encode_to_vec()
        }),
        &quot;<span class="s">nurbssurfaces</span>&quot; =&gt; emit(&amp;objects.nurbssurfaces, tag, start, work, buffer, |item| {
            truth(lookup, item).to_proto().encode_to_vec()
        }),
        &quot;<span class="s">breps</span>&quot; =&gt; emit(&amp;objects.breps, tag, start, work, buffer, |item| {
            truth(lookup, item).to_proto().encode_to_vec()
        }),
        _ =&gt; emit(&amp;objects.elements, tag, start, work, buffer, |item| {
            truth(lookup, item).to_proto().encode_to_vec()
        }),
    }
}

<span class="c">/// The name and guid fields of an Objects message.</span>
<span class="k">fn</span> objects_head(objects: &amp;Objects) -&gt; Vec&lt;u8&gt; {
    <span class="k">use</span> prost::Message;

    <span class="k">let</span> guid = <span class="k">if</span> objects.has_guid() {
        objects.guid().to_string()
    } <span class="k">else</span> {
        String::new()
    };

    <span class="k">crate</span>::proto::Objects {
        name: objects.name.clone(),
        guid,
        ..Default::default()
    }
    .encode_to_vec()
}

<span class="c">/// A session containing geometry objects.</span>
#[derive(Serialize, Deserialize)]
#[serde(tag = &quot;<span class="s">type</span>&quot;, rename = &quot;<span class="s">Session</span>&quot;)]
<span class="k">pub</span> <span class="k">struct</span> Session {
    #[serde(
        serialize_with = &quot;<span class="s">crate::guid_serde::serialize</span>&quot;,
        deserialize_with = &quot;<span class="s">crate::guid_serde::deserialize</span>&quot;
    )]
    guid: std::sync::OnceLock&lt;String&gt;, <span class="c">// Lazily minted guid.</span>
    <span class="k">pub</span> name: String,     <span class="c">// The name of the session.</span>
    <span class="k">pub</span> objects: Objects, <span class="c">// Collection of geometry objects.</span>
    #[serde(skip)]
    <span class="k">pub</span> lookup: HashMap&lt;String, Geometry&gt;, <span class="c">// Fast lookup table for geometry by GUID.</span>
    <span class="k">pub</span> tree: Tree,       <span class="c">// Tree structure for hierarchy.</span>
    <span class="k">pub</span> graph: Graph,     <span class="c">// Graph structure for relationships.</span>
    #[serde(skip)]
    <span class="k">pub</span> component_lookup: HashMap&lt;String, Component&gt;, <span class="c">// Fast lookup table for components by GUID.</span>
    #[serde(skip)]
    <span class="k">pub</span> xforms: HashMap&lt;String, Xform&gt;, <span class="c">// LOCAL transform per guid, relative to the parent.</span>
    #[serde(default)]
    <span class="k">pub</span> definitions: Objects, <span class="c">// Shared geometry instances place, each in its own frame; never in order(), the tree, the graph or xforms.</span>
    #[serde(skip)]
    <span class="k">pub</span> definition_lookup: HashMap&lt;String, Geometry&gt;, <span class="c">// Definitions by guid.</span>
    #[serde(skip)]
    <span class="k">pub</span> instance_lookup: HashMap&lt;String, Rc&lt;InstanceRef&gt;&gt;, <span class="c">// Instances by guid.</span>
    #[serde(skip)]
    <span class="k">pub</span> interactions: BTreeMap&lt;String, Vec&lt;Box&lt;<span class="k">dyn</span> Interaction&gt;&gt;&gt;, <span class="c">// Interactions per graph edge, by the edge's guid; a boxed implementor keeps its type.</span>
    #[serde(skip)]
    <span class="k">pub</span> history: History, <span class="c">// Undo/redo buffer, in memory only; every save purges it.</span>
    #[serde(skip)]
    <span class="k">pub</span> bvh: SpatialBVH, <span class="c">// Bounding volume hierarchy for collision detection.</span>
    #[serde(skip)]
    <span class="k">pub</span> cached_ray_bvh: Option&lt;SpatialBVH&gt;, <span class="c">// Cached SpatialBVH for ray casting.</span>
    #[serde(skip)]
    <span class="k">pub</span> cached_guids: Vec&lt;String&gt;, <span class="c">// GUID per leaf of cached_ray_bvh.</span>
    #[serde(skip)]
    <span class="k">pub</span> cached_boxes: Vec&lt;OBB&gt;, <span class="c">// Box per leaf of cached_ray_bvh.</span>
    #[serde(skip)]
    <span class="k">pub</span> bvh_cache_dirty: bool, <span class="c">// Flag to rebuild cached_ray_bvh.</span>
    #[serde(skip)]
    <span class="k">pub</span> node_lookup: HashMap&lt;String, Rc&lt;RefCell&lt;TreeNode&gt;&gt;&gt;, <span class="c">// Tree node per live object guid.</span>
    #[serde(skip)]
    indexed: Option&lt;Weak&lt;RefCell&lt;TreeNode&gt;&gt;&gt;, <span class="c">// Tree root at the last reindex, stale after a wholesale tree swap.</span>
    #[serde(skip)]
    <span class="k">pub</span> revision: u64, <span class="c">// Bumped by every Session mutation.</span>
    #[serde(skip)]
    sweep: Vec&lt;Weak&lt;RefCell&lt;TreeNode&gt;&gt;&gt;, <span class="c">// Parents whose children died, for the purge to compact.</span>
    #[serde(skip)]
    pinned: Vec&lt;Weak&lt;RefCell&lt;TreeNode&gt;&gt;&gt;, <span class="c">// Parents the purge left while a record pinned a child.</span>
    #[serde(skip)]
    purging: Option&lt;usize&gt;, <span class="c">// The purge phase: 0..=12 objects lists, 13..=25 definitions lists, 26 the tree; None between cycles.</span>
    #[serde(skip)]
    writer: Option&lt;Checkpoint&gt;, <span class="c">// The checkpoint being written, stale once revision moves.</span>
}

<span class="k">impl</span> Default <span class="k">for</span> Session {
    <span class="c">/// Construct a session named &quot;my_session&quot;.</span>
    <span class="k">fn</span> default() -&gt; <span class="k">Self</span> {
        <span class="k">Self</span>::new(&quot;<span class="s">my_session</span>&quot;)
    }
}

<span class="k">impl</span> Clone <span class="k">for</span> Session {
    <span class="c">/// Copy every live table and object into compacted lists, guids included; caches are rebuilt on demand and history starts empty.</span>
    <span class="k">fn</span> clone(&amp;<span class="k">self</span>) -&gt; <span class="k">Self</span> {
        <span class="k">let</span> <span class="k">mut</span> session = Session::new(&amp;<span class="k">self</span>.name);

        <span class="k">if</span> <span class="k">self</span>.has_guid() {
            session.set_guid(<span class="k">self</span>.guid().to_string());
        }

        session.objects = clone_objects(&amp;<span class="k">self</span>.objects_synced());
        session.definitions = clone_objects(&amp;synced(&amp;<span class="k">self</span>.definitions, &amp;<span class="k">self</span>.definition_lookup));
        session.tree = <span class="k">self</span>.tree.clone();
        session.graph = <span class="k">self</span>.graph.clone();
        session.graph.renumber();
        session.xforms = <span class="k">self</span>.xforms.clone();
        session.interactions = <span class="k">self</span>.interactions.clone();
        session.reindex();

        session
    }
}

<span class="k">impl</span> Session {
    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">// Constructors</span>
    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">/// Construct an empty session whose tree root carries the session name.</span>
    <span class="k">pub</span> <span class="k">fn</span> new(name: &amp;str) -&gt; <span class="k">Self</span> {
        <span class="k">let</span> <span class="k">mut</span> tree = Tree::new(&amp;format!(&quot;{<span class="s">name</span>}<span class="s">_tree</span>&quot;));
        tree.add(&amp;TreeNode::new(name), None);
        <span class="k">let</span> indexed = tree.root().map(|root| Rc::downgrade(&amp;root));

        <span class="k">Self</span> {
            guid: std::sync::OnceLock::new(),
            name: name.to_string(),
            objects: Objects::new(),
            lookup: HashMap::new(),
            tree,
            graph: Graph::new(&amp;format!(&quot;{<span class="s">name</span>}<span class="s">_graph</span>&quot;)),
            component_lookup: HashMap::new(),
            xforms: HashMap::new(),
            definitions: Objects::new(),
            definition_lookup: HashMap::new(),
            instance_lookup: HashMap::new(),
            interactions: BTreeMap::new(),
            history: History::new(),
            bvh: SpatialBVH::new(),
            cached_ray_bvh: None,
            cached_guids: Vec::new(),
            cached_boxes: Vec::new(),
            bvh_cache_dirty: <span class="s">true</span>,
            node_lookup: HashMap::new(),
            indexed,
            revision: <span class="s">0</span>,
            sweep: Vec::new(),
            pinned: Vec::new(),
            purging: None,
            writer: None,
        }
    }

    <span class="c">/// Return whether the lazy guid has been created.</span>
    <span class="k">pub</span> <span class="k">fn</span> has_guid(&amp;<span class="k">self</span>) -&gt; bool {
        <span class="k">self</span>.guid.get().is_some()
    }

    <span class="c">/// Return the guid, creating it on first access.</span>
    <span class="k">pub</span> <span class="k">fn</span> guid(&amp;<span class="k">self</span>) -&gt; &amp;str {
        <span class="k">self</span>.guid.get_or_init(|| uuid::Uuid::new_v4().to_string())
    }

    <span class="c">/// Set the guid.</span>
    <span class="k">pub</span> <span class="k">fn</span> set_guid(&amp;<span class="k">mut</span> <span class="k">self</span>, guid: String) {
        <span class="k">self</span>.guid = std::sync::OnceLock::from(guid);
    }

    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">// Accessors</span>
    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">/// Get a geometry object by GUID with type safety.</span>
    <span class="k">pub</span> <span class="k">fn</span> get_object(&amp;<span class="k">self</span>, guid: &amp;str) -&gt; Option&lt;&amp;Geometry&gt; {
        <span class="k">self</span>.lookup.get(guid)
    }

    <span class="c">/// The tree node of a live object in O(1) through node_lookup; a tree search when the index is stale, None for a guid that is no live object.</span>
    <span class="k">pub</span> <span class="k">fn</span> get_node(&amp;<span class="k">self</span>, guid: &amp;str) -&gt; Option&lt;Rc&lt;RefCell&lt;TreeNode&gt;&gt;&gt; {
        <span class="k">if</span> !<span class="k">self</span>._is_live(guid) {
            <span class="k">return</span> None;
        }

        <span class="k">let</span> root = <span class="k">self</span>.tree.root();
        <span class="k">let</span> fresh = <span class="k">match</span> (&amp;<span class="k">self</span>.indexed, &amp;root) {
            (Some(indexed), Some(root)) =&gt; std::ptr::eq(indexed.as_ptr(), Rc::as_ptr(root)),
            _ =&gt; <span class="s">false</span>,
        };

        <span class="k">if</span> <span class="k">let</span> (<span class="s">true</span>, Some(node)) = (fresh, <span class="k">self</span>.node_lookup.get(guid)) {
            <span class="k">let</span> owned = node.borrow().name == guid &amp;&amp; node.borrow().parent().is_some();

            <span class="k">if</span> owned {
                <span class="k">return</span> Some(Rc::clone(node));
            }
        }

        <span class="k">self</span>.tree.get_node_by_name(guid)
    }

    <span class="c">/// Select objects of one type, grouped by the top-level nodes of the tree.</span>
    <span class="k">pub</span> <span class="k">fn</span> select_by_type&lt;T: FromGeometry + Clone&gt;(&amp;<span class="k">self</span>) -&gt; Vec&lt;Vec&lt;T&gt;&gt; {
        <span class="k">let</span> <span class="k">mut</span> groups: Vec&lt;Vec&lt;T&gt;&gt; = Vec::new();
        <span class="k">let</span> Some(root) = <span class="k">self</span>.tree.root() <span class="k">else</span> {
            <span class="k">return</span> groups;
        };
        <span class="k">let</span> children = root.borrow().children();

        <span class="k">for</span> group <span class="k">in</span> children {
            <span class="k">let</span> <span class="k">mut</span> items: Vec&lt;T&gt; = Vec::new();
            <span class="k">let</span> descendants = group.borrow().descendants();

            <span class="k">for</span> node <span class="k">in</span> descendants {
                <span class="k">let</span> name = node.borrow().name.clone();

                <span class="k">if</span> <span class="k">let</span> Some(object) = <span class="k">self</span>.get_object(&amp;name).and_then(T::from_geometry) {
                    items.push(object.clone());
                }
            }

            <span class="k">if</span> !items.is_empty() {
                groups.push(items);
            }
        }

        groups
    }

    <span class="c">/// Find an existing group by name; panics when there is none.</span>
    <span class="k">pub</span> <span class="k">fn</span> find_group(&amp;<span class="k">self</span>, group_name: &amp;str) -&gt; Rc&lt;RefCell&lt;TreeNode&gt;&gt; {
        <span class="k">if</span> <span class="k">let</span> Some(root) = <span class="k">self</span>.tree.root() {
            <span class="k">for</span> child <span class="k">in</span> root.borrow().children() {
                <span class="k">if</span> child.borrow().name == group_name {
                    <span class="k">return</span> child.clone();
                }
            }
        }

        panic!(&quot;<span class="s">Group '</span>{}<span class="s">' not found</span>&quot;, group_name);
    }

    <span class="c">/// Canonical object order: the objects vectors walked in one fixed type sequence; instances are not in it.</span>
    <span class="k">pub</span> <span class="k">fn</span> order(&amp;<span class="k">self</span>) -&gt; Vec&lt;String&gt; {
        <span class="k">let</span> <span class="k">mut</span> order = Vec::with_capacity(<span class="k">self</span>.lookup.len());

        <span class="k">for</span> point <span class="k">in</span> &amp;<span class="k">self</span>.objects.points {
            order.push(point.guid().to_string());
        }

        <span class="k">for</span> line <span class="k">in</span> &amp;<span class="k">self</span>.objects.lines {
            order.push(line.guid().to_string());
        }

        <span class="k">for</span> plane <span class="k">in</span> &amp;<span class="k">self</span>.objects.planes {
            order.push(plane.guid().to_string());
        }

        <span class="k">for</span> bbox <span class="k">in</span> &amp;<span class="k">self</span>.objects.bboxes {
            order.push(bbox.guid().to_string());
        }

        <span class="k">for</span> polyline <span class="k">in</span> &amp;<span class="k">self</span>.objects.polylines {
            order.push(polyline.guid().to_string());
        }

        <span class="k">for</span> pointcloud <span class="k">in</span> &amp;<span class="k">self</span>.objects.pointclouds {
            order.push(pointcloud.guid().to_string());
        }

        <span class="k">for</span> mesh <span class="k">in</span> &amp;<span class="k">self</span>.objects.meshes {
            order.push(mesh.guid().to_string());
        }

        <span class="k">for</span> nurbscurve <span class="k">in</span> &amp;<span class="k">self</span>.objects.nurbscurves {
            order.push(nurbscurve.guid().to_string());
        }

        <span class="k">for</span> nurbssurface <span class="k">in</span> &amp;<span class="k">self</span>.objects.nurbssurfaces {
            order.push(nurbssurface.guid().to_string());
        }

        <span class="k">for</span> brep <span class="k">in</span> &amp;<span class="k">self</span>.objects.breps {
            order.push(brep.guid().to_string());
        }

        <span class="k">for</span> element <span class="k">in</span> &amp;<span class="k">self</span>.objects.elements {
            order.push(element.guid().to_string());
        }

        order
    }

    <span class="c">/// The LOCAL transform of an object, identity when none was set.</span>
    <span class="k">pub</span> <span class="k">fn</span> xform(&amp;<span class="k">self</span>, guid: &amp;str) -&gt; Xform {
        <span class="k">self</span>.xforms
            .get(guid)
            .cloned()
            .unwrap_or_else(Xform::identity)
    }

    <span class="c">/// The CUMULATIVE placement of an object: every ancestor's transform multiplied down the tree onto its own.</span>
    <span class="k">pub</span> <span class="k">fn</span> world_xform(&amp;<span class="k">self</span>, guid: &amp;str) -&gt; Xform {
        <span class="k">let</span> <span class="k">mut</span> acc = <span class="k">self</span>.xform(guid);
        <span class="k">let</span> Some(node) = <span class="k">self</span>.get_node(guid) <span class="k">else</span> {
            <span class="k">return</span> acc;
        };

        <span class="k">for</span> ancestor <span class="k">in</span> node.borrow().ancestors() {
            <span class="k">let</span> name = ancestor.borrow().name.clone();

            <span class="k">if</span> <span class="k">let</span> Some(xform) = <span class="k">self</span>.xforms.get(&amp;name) {
                acc = xform * &amp;acc;
            }
        }

        acc
    }

    <span class="c">/// Every object's cumulative placement, computed in one downward pass.</span>
    <span class="k">pub</span> <span class="k">fn</span> world_xforms(&amp;<span class="k">self</span>) -&gt; HashMap&lt;String, Xform&gt; {
        <span class="k">let</span> <span class="k">mut</span> out: HashMap&lt;String, Xform&gt; = HashMap::new();

        <span class="k">if</span> <span class="k">self</span>.xforms.is_empty() {
            <span class="k">return</span> out;
        }

        <span class="k">let</span> <span class="k">mut</span> stack: Vec&lt;(Rc&lt;RefCell&lt;TreeNode&gt;&gt;, Xform)&gt; = Vec::new();

        <span class="k">if</span> <span class="k">let</span> Some(root) = <span class="k">self</span>.tree.root() {
            stack.push((root, Xform::identity()));
        }

        <span class="k">while</span> <span class="k">let</span> Some((node, parent_xform)) = stack.pop() {
            <span class="k">let</span> name = node.borrow().name.clone();
            <span class="k">let</span> current = <span class="k">match</span> <span class="k">self</span>.xforms.get(&amp;name) {
                Some(local) =&gt; &amp;parent_xform * local,
                None =&gt; parent_xform.clone(),
            };
            out.insert(name, current.clone());

            <span class="k">for</span> child <span class="k">in</span> node.borrow().children() {
                stack.push((child, current.clone()));
            }
        }

        <span class="k">for</span> (obj_guid, obj_xform) <span class="k">in</span> &amp;<span class="k">self</span>.xforms {
            out.entry(obj_guid.clone())
                .or_insert_with(|| obj_xform.clone());
        }

        out
    }

    <span class="c">/// Get the children of a parent GUID.</span>
    <span class="k">pub</span> <span class="k">fn</span> get_children(&amp;<span class="k">self</span>, obj_guid: &amp;str) -&gt; Vec&lt;String&gt; {
        <span class="k">self</span>.tree.get_children_guids(obj_guid)
    }

    <span class="c">/// Get the neighbours of a GUID.</span>
    <span class="k">pub</span> <span class="k">fn</span> get_neighbours(&amp;<span class="k">self</span>, obj_guid: &amp;str) -&gt; Vec&lt;String&gt; {
        <span class="k">self</span>.graph.neighbors(obj_guid)
    }

    <span class="c">/// All geometry with its hierarchical placement BAKED into the coordinates; each instance becomes its definition placed, in the definition's vector.</span>
    <span class="k">pub</span> <span class="k">fn</span> get_geometry(&amp;<span class="k">self</span>) -&gt; Objects {
        <span class="k">let</span> objects = <span class="k">self</span>.objects_synced();
        <span class="k">let</span> <span class="k">mut</span> out = objects.clone();
        <span class="k">let</span> world = <span class="k">self</span>.world_xforms();
        out.points = baked(&amp;objects.points, &amp;world, Point::transform);
        out.lines = baked(&amp;objects.lines, &amp;world, Line::transform);
        out.planes = baked(&amp;objects.planes, &amp;world, Plane::transform);
        out.bboxes = baked(&amp;objects.bboxes, &amp;world, OBB::transform);
        out.polylines = baked(&amp;objects.polylines, &amp;world, Polyline::transform);
        out.pointclouds = baked(&amp;objects.pointclouds, &amp;world, PointCloud::transform);
        out.meshes = baked(&amp;objects.meshes, &amp;world, |mesh, xform| {
            mesh.transform(xform);
        });
        out.nurbscurves = baked(&amp;objects.nurbscurves, &amp;world, |curve, xform| {
            curve.transform(xform);
        });
        out.nurbssurfaces = baked(&amp;objects.nurbssurfaces, &amp;world, |surface, xform| {
            surface.transform(xform);
        });
        out.breps = baked(&amp;objects.breps, &amp;world, BRep::transform);
        out.elements = baked(&amp;objects.elements, &amp;world, Element::place);

        <span class="k">for</span> instance <span class="k">in</span> &amp;objects.instances {
            <span class="k">let</span> Some(definition) = <span class="k">self</span>.definition_lookup.get(&amp;instance.definition_guid) <span class="k">else</span> {
                <span class="k">continue</span>;
            };
            <span class="k">let</span> placement = world
                .get(instance.guid())
                .cloned()
                .unwrap_or_else(Xform::identity);
            <span class="k">let</span> resolved = resolve(instance, definition, &amp;placement);
            push(&amp;<span class="k">mut</span> out, &amp;Item::Geometry(resolved));
        }

        out.instances.clear();

        out
    }

    <span class="c">/// The definition an instance places; None when guid is no instance or its definition is missing.</span>
    <span class="k">pub</span> <span class="k">fn</span> definition_of(&amp;<span class="k">self</span>, instance_guid: &amp;str) -&gt; Option&lt;Geometry&gt; {
        <span class="k">let</span> instance = <span class="k">self</span>.instance_lookup.get(instance_guid)?;

        <span class="k">self</span>.definition_lookup
            .get(&amp;instance.definition_guid)
            .cloned()
    }

    <span class="c">/// Guids of every instance of a definition, in objects.instances order.</span>
    <span class="k">pub</span> <span class="k">fn</span> instances_of(&amp;<span class="k">self</span>, definition_guid: &amp;str) -&gt; Vec&lt;String&gt; {
        <span class="k">let</span> <span class="k">mut</span> guids = Vec::new();

        <span class="k">for</span> instance <span class="k">in</span> &amp;<span class="k">self</span>.objects.instances {
            <span class="k">if</span> instance.definition_guid == definition_guid {
                guids.push(instance.guid().to_string());
            }
        }

        guids
    }

    <span class="c">/// One object in world placement, as a copy: an instance becomes its definition moved by the world transform, carrying the instance's guid, name and features.</span>
    <span class="k">pub</span> <span class="k">fn</span> world_geometry(&amp;<span class="k">self</span>, guid: &amp;str) -&gt; Option&lt;Geometry&gt; {
        <span class="k">let</span> world = <span class="k">self</span>.world_xform(guid);

        <span class="k">if</span> <span class="k">let</span> Some(geometry) = <span class="k">self</span>.lookup.get(guid) {
            <span class="k">let</span> <span class="k">mut</span> copy = clone(geometry);
            place(&amp;<span class="k">mut</span> copy, &amp;world);

            <span class="k">return</span> Some(copy);
        }

        <span class="k">let</span> definition = <span class="k">self</span>.definition_of(guid)?;

        Some(resolve(&amp;<span class="k">self</span>.instance_lookup[guid], &amp;definition, &amp;world))
    }

    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">// Geometry management</span>
    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">/// Add a point; a guid already live adds nothing and returns the node that guid has.</span>
    <span class="k">pub</span> <span class="k">fn</span> add_point(
        &amp;<span class="k">mut</span> <span class="k">self</span>,
        point: Point,
        parent: Option&lt;&amp;Rc&lt;RefCell&lt;TreeNode&gt;&gt;&gt;,
    ) -&gt; Rc&lt;RefCell&lt;TreeNode&gt;&gt; {
        <span class="k">self</span>._add_object(&quot;<span class="s">points</span>&quot;, Geometry::Point(Rc::new(point)), &quot;<span class="s">point</span>&quot;, parent)
            .unwrap_or_else(|guid| <span class="k">self</span>._node_of(&amp;guid))
    }

    <span class="c">/// Add a line; a guid already live adds nothing and returns the node that guid has.</span>
    <span class="k">pub</span> <span class="k">fn</span> add_line(
        &amp;<span class="k">mut</span> <span class="k">self</span>,
        line: Line,
        parent: Option&lt;&amp;Rc&lt;RefCell&lt;TreeNode&gt;&gt;&gt;,
    ) -&gt; Rc&lt;RefCell&lt;TreeNode&gt;&gt; {
        <span class="k">self</span>._add_object(&quot;<span class="s">lines</span>&quot;, Geometry::Line(Rc::new(line)), &quot;<span class="s">line</span>&quot;, parent)
            .unwrap_or_else(|guid| <span class="k">self</span>._node_of(&amp;guid))
    }

    <span class="c">/// Add a plane; a guid already live adds nothing and returns the node that guid has.</span>
    <span class="k">pub</span> <span class="k">fn</span> add_plane(
        &amp;<span class="k">mut</span> <span class="k">self</span>,
        plane: Plane,
        parent: Option&lt;&amp;Rc&lt;RefCell&lt;TreeNode&gt;&gt;&gt;,
    ) -&gt; Rc&lt;RefCell&lt;TreeNode&gt;&gt; {
        <span class="k">self</span>._add_object(&quot;<span class="s">planes</span>&quot;, Geometry::Plane(Rc::new(plane)), &quot;<span class="s">plane</span>&quot;, parent)
            .unwrap_or_else(|guid| <span class="k">self</span>._node_of(&amp;guid))
    }

    <span class="c">/// Add a bounding box; a guid already live adds nothing and returns the node that guid has.</span>
    <span class="k">pub</span> <span class="k">fn</span> add_obb(&amp;<span class="k">mut</span> <span class="k">self</span>, bbox: OBB) -&gt; Rc&lt;RefCell&lt;TreeNode&gt;&gt; {
        <span class="k">self</span>._add_object(&quot;<span class="s">bboxes</span>&quot;, Geometry::OBB(Rc::new(bbox)), &quot;<span class="s">bbox</span>&quot;, None)
            .unwrap_or_else(|guid| <span class="k">self</span>._node_of(&amp;guid))
    }

    <span class="c">/// Add a polyline; fewer than two points, or a guid already live, adds nothing and returns None.</span>
    <span class="k">pub</span> <span class="k">fn</span> add_polyline(
        &amp;<span class="k">mut</span> <span class="k">self</span>,
        polyline: Polyline,
        parent: Option&lt;&amp;Rc&lt;RefCell&lt;TreeNode&gt;&gt;&gt;,
    ) -&gt; Option&lt;Rc&lt;RefCell&lt;TreeNode&gt;&gt;&gt; {
        <span class="k">if</span> polyline.point_count() &lt; <span class="s">2</span> {
            <span class="k">return</span> None;
        }

        <span class="k">self</span>._add_object(
            &quot;<span class="s">polylines</span>&quot;,
            Geometry::Polyline(Rc::new(polyline)),
            &quot;<span class="s">polyline</span>&quot;,
            parent,
        )
        .ok()
    }

    <span class="c">/// Add a point cloud; no points, or a guid already live, adds nothing and returns None.</span>
    <span class="k">pub</span> <span class="k">fn</span> add_pointcloud(
        &amp;<span class="k">mut</span> <span class="k">self</span>,
        pointcloud: PointCloud,
        parent: Option&lt;&amp;Rc&lt;RefCell&lt;TreeNode&gt;&gt;&gt;,
    ) -&gt; Option&lt;Rc&lt;RefCell&lt;TreeNode&gt;&gt;&gt; {
        <span class="k">if</span> pointcloud.is_empty() {
            <span class="k">return</span> None;
        }

        <span class="k">self</span>._add_object(
            &quot;<span class="s">pointclouds</span>&quot;,
            Geometry::PointCloud(Rc::new(pointcloud)),
            &quot;<span class="s">pointcloud</span>&quot;,
            parent,
        )
        .ok()
    }

    <span class="c">/// Add a mesh; no faces, or a guid already live, adds nothing and returns None.</span>
    <span class="k">pub</span> <span class="k">fn</span> add_mesh(
        &amp;<span class="k">mut</span> <span class="k">self</span>,
        mesh: Mesh,
        parent: Option&lt;&amp;Rc&lt;RefCell&lt;TreeNode&gt;&gt;&gt;,
    ) -&gt; Option&lt;Rc&lt;RefCell&lt;TreeNode&gt;&gt;&gt; {
        <span class="k">if</span> mesh.is_empty() || mesh.number_of_faces() == <span class="s">0</span> {
            <span class="k">return</span> None;
        }

        <span class="k">self</span>._add_object(&quot;<span class="s">meshes</span>&quot;, Geometry::Mesh(Rc::new(mesh)), &quot;<span class="s">mesh</span>&quot;, parent)
            .ok()
    }

    <span class="c">/// Add a curve; fewer than two control vertices, or a guid already live, adds nothing and returns None.</span>
    <span class="k">pub</span> <span class="k">fn</span> add_nurbscurve(
        &amp;<span class="k">mut</span> <span class="k">self</span>,
        nurbscurve: NurbsCurve,
        parent: Option&lt;&amp;Rc&lt;RefCell&lt;TreeNode&gt;&gt;&gt;,
    ) -&gt; Option&lt;Rc&lt;RefCell&lt;TreeNode&gt;&gt;&gt; {
        <span class="k">if</span> nurbscurve.cv_count() &lt; <span class="s">2</span> {
            <span class="k">return</span> None;
        }

        <span class="k">self</span>._add_object(
            &quot;<span class="s">nurbscurves</span>&quot;,
            Geometry::NurbsCurve(Rc::new(nurbscurve)),
            &quot;<span class="s">nurbscurve</span>&quot;,
            parent,
        )
        .ok()
    }

    <span class="c">/// Add a surface; no control vertices, or a guid already live, adds nothing and returns None.</span>
    <span class="k">pub</span> <span class="k">fn</span> add_nurbssurface(
        &amp;<span class="k">mut</span> <span class="k">self</span>,
        nurbssurface: NurbsSurface,
        parent: Option&lt;&amp;Rc&lt;RefCell&lt;TreeNode&gt;&gt;&gt;,
    ) -&gt; Option&lt;Rc&lt;RefCell&lt;TreeNode&gt;&gt;&gt; {
        <span class="k">if</span> nurbssurface.cv_count_total() == <span class="s">0</span> {
            <span class="k">return</span> None;
        }

        <span class="k">self</span>._add_object(
            &quot;<span class="s">nurbssurfaces</span>&quot;,
            Geometry::NurbsSurface(Rc::new(nurbssurface)),
            &quot;<span class="s">nurbssurface</span>&quot;,
            parent,
        )
        .ok()
    }

    <span class="c">/// Add a brep; no faces and no vertices, or a guid already live, adds nothing and returns None.</span>
    <span class="k">pub</span> <span class="k">fn</span> add_brep(
        &amp;<span class="k">mut</span> <span class="k">self</span>,
        brep: BRep,
        parent: Option&lt;&amp;Rc&lt;RefCell&lt;TreeNode&gt;&gt;&gt;,
    ) -&gt; Option&lt;Rc&lt;RefCell&lt;TreeNode&gt;&gt;&gt; {
        <span class="k">if</span> brep.face_count() == <span class="s">0</span> &amp;&amp; brep.vertex_count() == <span class="s">0</span> {
            <span class="k">return</span> None;
        }

        <span class="k">self</span>._add_object(&quot;<span class="s">breps</span>&quot;, Geometry::BRep(Rc::new(brep)), &quot;<span class="s">brep</span>&quot;, parent)
            .ok()
    }

    <span class="c">/// Add an element, a data record kept even without geometry; a guid already live adds nothing and returns the node that guid has.</span>
    <span class="k">pub</span> <span class="k">fn</span> add_element(
        &amp;<span class="k">mut</span> <span class="k">self</span>,
        element: Element,
        parent: Option&lt;&amp;Rc&lt;RefCell&lt;TreeNode&gt;&gt;&gt;,
    ) -&gt; Rc&lt;RefCell&lt;TreeNode&gt;&gt; {
        <span class="k">self</span>._add_object(
            &quot;<span class="s">elements</span>&quot;,
            Geometry::Element(Rc::new(element)),
            &quot;<span class="s">element</span>&quot;,
            parent,
        )
        .unwrap_or_else(|guid| <span class="k">self</span>._node_of(&amp;guid))
    }

    <span class="c">/// Add a custom component (any object with type_name/guid/name/extra); a guid already live adds nothing and returns the node that guid has.</span>
    <span class="k">pub</span> <span class="k">fn</span> add_component(
        &amp;<span class="k">mut</span> <span class="k">self</span>,
        component: Component,
        parent: Option&lt;&amp;Rc&lt;RefCell&lt;TreeNode&gt;&gt;&gt;,
    ) -&gt; Rc&lt;RefCell&lt;TreeNode&gt;&gt; {
        <span class="k">self</span>._add_object(
            &quot;<span class="s">components</span>&quot;,
            Item::Component(component),
            &quot;<span class="s">component</span>&quot;,
            parent,
        )
        .unwrap_or_else(|guid| <span class="k">self</span>._node_of(&amp;guid))
    }

    <span class="c">/// Add a definition, geometry in its own frame that instances share; returns its guid, also when that guid is already defined, and &quot;&quot; for a guid an object, instance or component holds.</span>
    <span class="k">pub</span> <span class="k">fn</span> add_definition(&amp;<span class="k">mut</span> <span class="k">self</span>, definition: Geometry) -&gt; String {
        <span class="k">let</span> guid = definition.guid().to_string();

        <span class="k">if</span> <span class="k">self</span>.definition_lookup.contains_key(&amp;guid) {
            <span class="k">return</span> guid;
        }

        <span class="k">if</span> <span class="k">self</span>._is_live(&amp;guid) {
            <span class="k">return</span> String::new();
        }

        <span class="k">let</span> (collection, _) = collection_of(&amp;definition);
        <span class="k">let</span> slot = push(&amp;<span class="k">mut</span> <span class="k">self</span>.definitions, &amp;Item::Geometry(definition.clone()));
        <span class="k">self</span>.definition_lookup.insert(guid.clone(), definition);
        <span class="k">self</span>.bvh_cache_dirty = <span class="s">true</span>;
        <span class="k">self</span>.revision += <span class="s">1</span>;

        <span class="k">if</span> <span class="k">self</span>.history.current.is_some() {
            <span class="k">let</span> tomb = Tomb::new(collection, <span class="s">true</span>, slot, None);
            pin(&amp;<span class="k">mut</span> <span class="k">self</span>.definitions, collection, slot, &amp;tomb);
            <span class="k">self</span>.history.record(
                Op::Add(Tombstone::new(
                    guid.clone(),
                    &quot;<span class="s">definitions</span>&quot;.to_string(),
                    None,
                    <span class="s">0</span>,
                    None,
                    tomb,
                )),
                RECORD,
            );
        }

        guid
    }

    <span class="c">/// Add an instance under parent, placed by xform relative to the parent with its own xform folded in; None when its definition_guid names no definition or its guid is already live.</span>
    <span class="k">pub</span> <span class="k">fn</span> add_instance(
        &amp;<span class="k">mut</span> <span class="k">self</span>,
        <span class="k">mut</span> instance: InstanceRef,
        xform: Xform,
        parent: Option&lt;&amp;Rc&lt;RefCell&lt;TreeNode&gt;&gt;&gt;,
    ) -&gt; Option&lt;Rc&lt;RefCell&lt;TreeNode&gt;&gt;&gt; {
        <span class="k">if</span> !<span class="k">self</span>
            .definition_lookup
            .contains_key(&amp;instance.definition_guid)
        {
            <span class="k">return</span> None;
        }

        <span class="k">let</span> placement = &amp;xform * &amp;instance.xform;
        instance.xform = Xform::identity();
        <span class="k">let</span> guid = instance.guid().to_string();
        <span class="k">let</span> node = <span class="k">self</span>
            ._add_object(
                &quot;<span class="s">instances</span>&quot;,
                Item::InstanceRef(Rc::new(instance)),
                &quot;<span class="s">instance</span>&quot;,
                parent,
            )
            .ok()?;

        <span class="k">if</span> !placement.is_identity() {
            <span class="k">self</span>.set_xform(&amp;guid, placement);
        }

        Some(node)
    }

    <span class="c">/// Put a TreeNode under a parent, the root when none is given: a placed node moves and leaves a ghost, one already there is left alone.</span>
    <span class="k">pub</span> <span class="k">fn</span> add&lt;'a&gt;(
        &amp;<span class="k">mut</span> <span class="k">self</span>,
        node: &amp;Rc&lt;RefCell&lt;TreeNode&gt;&gt;,
        parent: <span class="k">impl</span> Into&lt;Option&lt;&amp;'a Rc&lt;RefCell&lt;TreeNode&gt;&gt;&gt;&gt;,
    ) <span class="k">where</span>
        Rc&lt;RefCell&lt;TreeNode&gt;&gt;: 'a,
    {
        <span class="k">let</span> Some(parent) = parent.into().cloned().or_else(|| <span class="k">self</span>.tree.root()) <span class="k">else</span> {
            <span class="k">return</span>;
        };
        <span class="k">let</span> name = node.borrow().name.clone();
        <span class="k">let</span> held = node
            .borrow()
            .parent()
            .is_some_and(|held| Rc::ptr_eq(&amp;held, &amp;parent));

        <span class="k">if</span> held || Rc::ptr_eq(node, &amp;parent) {
            <span class="k">return</span>;
        }

        <span class="k">let</span> was_dead = node.borrow().is_dead();
        <span class="k">let</span> old = node.borrow().parent();
        <span class="k">let</span> ghost = parent.borrow_mut().add(node);

        <span class="k">if</span> !parent.borrow().has_child(node) {
            <span class="k">return</span>;
        }

        node.borrow_mut().set_dead(<span class="s">false</span>);
        <span class="k">self</span>.revision += <span class="s">1</span>;

        <span class="k">if</span> <span class="k">let</span> (Some(old), <span class="s">true</span>) = (old, ghost.is_some()) {
            <span class="k">self</span>._queue(&amp;old);
        }

        <span class="k">let</span> tomb = node
            .borrow()
            .get_tomb()
            .filter(|tomb| tomb.collection.is_empty());

        <span class="k">if</span> <span class="k">let</span> (<span class="s">true</span>, Some(tomb)) = (was_dead, tomb) {
            <span class="k">if</span> <span class="k">let</span> Some(xform) = tomb.xform.borrow_mut().take() {
                <span class="k">self</span>.xforms.insert(name.clone(), xform);
            }
        }

        <span class="k">if</span> <span class="k">self</span>._is_live(&amp;name) {
            <span class="k">self</span>.node_lookup.insert(name, Rc::clone(node));
        }

        <span class="k">self</span>._record_add(node, ghost, was_dead);
    }

    <span class="c">/// Create a named group (TreeNode) and add it to the root of the tree.</span>
    <span class="k">pub</span> <span class="k">fn</span> add_group(&amp;<span class="k">mut</span> <span class="k">self</span>, group_name: &amp;str) -&gt; Rc&lt;RefCell&lt;TreeNode&gt;&gt; {
        <span class="k">let</span> node = TreeNode::new(group_name);
        <span class="k">self</span>.add(&amp;node, None);

        node
    }

    <span class="c">/// Rename a group node; false for an object node, a dead node or the same name.</span>
    <span class="k">pub</span> <span class="k">fn</span> rename_node(&amp;<span class="k">mut</span> <span class="k">self</span>, node: &amp;Rc&lt;RefCell&lt;TreeNode&gt;&gt;, name: &amp;str) -&gt; bool {
        <span class="k">let</span> before = node.borrow().name.clone();

        <span class="k">if</span> <span class="k">self</span>._is_live(&amp;before) || node.borrow().is_dead() || before == name {
            <span class="k">return</span> <span class="s">false</span>;
        }

        node.borrow_mut().name = name.to_string();
        <span class="k">self</span>.revision += <span class="s">1</span>;

        <span class="k">if</span> <span class="k">self</span>.history.current.is_some() {
            <span class="k">let</span> tomb = <span class="k">self</span>._node_tomb(node);
            <span class="k">let</span> color = node.borrow().color.clone();
            <span class="k">self</span>.history.record(
                Op::Tree(TreeOp::new(
                    before.clone(),
                    Rc::clone(node),
                    tomb,
                    None,
                    before,
                    name.to_string(),
                    color.clone(),
                    color,
                    <span class="s">false</span>,
                    <span class="s">false</span>,
                )),
                RECORD,
            );
        }

        <span class="s">true</span>
    }

    <span class="c">/// Set or clear (None) the display colour of a node; false for a dead node.</span>
    <span class="k">pub</span> <span class="k">fn</span> set_node_color(&amp;<span class="k">mut</span> <span class="k">self</span>, node: &amp;Rc&lt;RefCell&lt;TreeNode&gt;&gt;, color: Option&lt;Color&gt;) -&gt; bool {
        <span class="k">if</span> node.borrow().is_dead() {
            <span class="k">return</span> <span class="s">false</span>;
        }

        <span class="k">let</span> before = node.borrow().color.clone();
        node.borrow_mut().color = color.clone();
        <span class="k">self</span>.revision += <span class="s">1</span>;

        <span class="k">if</span> <span class="k">self</span>.history.current.is_some() {
            <span class="k">let</span> name = node.borrow().name.clone();
            <span class="k">let</span> tomb = <span class="k">self</span>._node_tomb(node);
            <span class="k">self</span>.history.record(
                Op::Tree(TreeOp::new(
                    name.clone(),
                    Rc::clone(node),
                    tomb,
                    None,
                    name.clone(),
                    name,
                    before,
                    color,
                    <span class="s">false</span>,
                    <span class="s">false</span>,
                )),
                RECORD,
            );
        }

        <span class="s">true</span>
    }

    <span class="c">/// Kill a group node with everything below it, parking its transform; false for an object node, the root or a dead node.</span>
    <span class="k">pub</span> <span class="k">fn</span> remove_group(&amp;<span class="k">mut</span> <span class="k">self</span>, node: &amp;Rc&lt;RefCell&lt;TreeNode&gt;&gt;) -&gt; bool {
        <span class="k">let</span> name = node.borrow().name.clone();
        <span class="k">let</span> Some(parent) = node.borrow().parent() <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };

        <span class="k">if</span> <span class="k">self</span>._is_live(&amp;name) {
            <span class="k">return</span> <span class="s">false</span>;
        }

        <span class="k">let</span> tomb = <span class="k">self</span>._node_tomb(node);
        node.borrow_mut().set_dead(<span class="s">true</span>);
        *tomb.xform.borrow_mut() = <span class="k">self</span>.xforms.remove(&amp;name);
        <span class="k">self</span>._queue(&amp;parent);
        <span class="k">self</span>.revision += <span class="s">1</span>;

        <span class="k">if</span> <span class="k">self</span>.history.current.is_none() {
            <span class="k">self</span>.history.dropped += <span class="s">1</span>;

            <span class="k">return</span> <span class="s">true</span>;
        }

        <span class="k">let</span> color = node.borrow().color.clone();
        <span class="k">self</span>.history.record(
            Op::Tree(TreeOp::new(
                name.clone(),
                Rc::clone(node),
                tomb,
                None,
                name.clone(),
                name,
                color.clone(),
                color,
                <span class="s">false</span>,
                <span class="s">true</span>,
            )),
            RECORD,
        );

        <span class="s">true</span>
    }

    <span class="c">/// Add an edge between two geometry objects in the graph.</span>
    <span class="k">pub</span> <span class="k">fn</span> add_edge(&amp;<span class="k">mut</span> <span class="k">self</span>, guid1: &amp;str, guid2: &amp;str, attribute: &amp;str) {
        <span class="k">self</span>.revision += <span class="s">1</span>;
        <span class="k">self</span>.graph.add_edge(guid1, guid2, attribute);
    }

    <span class="c">/// Add a parent-child relationship in the tree.</span>
    <span class="k">pub</span> <span class="k">fn</span> add_hierarchy(&amp;<span class="k">mut</span> <span class="k">self</span>, parent_guid: &amp;str, child_guid: &amp;str) -&gt; bool {
        <span class="k">self</span>.revision += <span class="s">1</span>;
        <span class="k">self</span>.tree.add_child_by_guid(parent_guid, child_guid)
    }

    <span class="c">/// Add a relationship edge in the graph.</span>
    <span class="k">pub</span> <span class="k">fn</span> add_relationship(&amp;<span class="k">mut</span> <span class="k">self</span>, from_guid: &amp;str, to_guid: &amp;str, relationship_type: &amp;str) {
        <span class="k">self</span>.revision += <span class="s">1</span>;
        <span class="k">self</span>.graph.add_edge(from_guid, to_guid, relationship_type);
    }

    <span class="c">/// Kill an object in place: its slot, node, transform, vertex, edges and interactions flip dead until undo revives them; O(1 + d log V).</span>
    <span class="k">pub</span> <span class="k">fn</span> remove_object(&amp;<span class="k">mut</span> <span class="k">self</span>, obj_guid: &amp;str) -&gt; bool {
        <span class="k">let</span> Some(obj) = <span class="k">self</span>._item(obj_guid) <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="k">let</span> tomb = <span class="k">self</span>._tomb(obj_guid, &amp;obj);
        <span class="k">let</span> degree = <span class="k">self</span>.graph.edges.get(obj_guid).map_or(<span class="s">0</span>, BTreeMap::len);
        <span class="k">let</span> node = tomb
            .node
            .clone()
            .filter(|node| node.borrow().parent().is_some());
        <span class="k">let</span> index = node.as_ref().map_or(<span class="s">0</span>, |node| node.borrow().at());
        <span class="k">let</span> parent_guid = node
            .as_ref()
            .and_then(|node| node.borrow().parent())
            .map(|parent| parent.borrow().name.clone());
        <span class="k">self</span>._kill(&amp;tomb);

        <span class="k">if</span> <span class="k">self</span>.history.current.is_none() {
            <span class="k">self</span>.history.dropped += <span class="s">1</span>;

            <span class="k">return</span> <span class="s">true</span>;
        }

        <span class="k">let</span> bytes = RECORD + weight(&amp;obj) + <span class="s">128</span> * degree;
        <span class="k">let</span> collection = tomb.collection.clone();
        <span class="k">self</span>.history.record(
            Op::Remove(Tombstone::new(
                obj_guid.to_string(),
                collection,
                parent_guid,
                index,
                node,
                tomb,
            )),
            bytes,
        );

        <span class="s">true</span>
    }

    <span class="c">/// Swap the object stored under guid for obj, which takes over that guid; a different type moves the guid to that type's list, the node and edges staying.</span>
    <span class="k">pub</span> <span class="k">fn</span> replace(&amp;<span class="k">mut</span> <span class="k">self</span>, guid: &amp;str, obj: Geometry) -&gt; bool {
        <span class="k">let</span> Some(before) = <span class="k">self</span>.lookup.get(guid).cloned() <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="k">let</span> <span class="k">mut</span> obj = obj;

        <span class="k">if</span> obj.guid() != guid {
            obj.set_guid(guid);
        }

        <span class="k">let</span> (old, _) = collection_of(&amp;before);
        <span class="k">let</span> (new, prefix) = collection_of(&amp;obj);
        <span class="k">let</span> bytes = RECORD + weight(&amp;Item::Geometry(before.clone()));

        <span class="k">if</span> old == new {
            <span class="k">let</span> entry = Entry::Object(<span class="k">self</span>.get_node(guid));

            <span class="k">if</span> <span class="k">self</span>.history.current.is_some() {
                <span class="k">self</span>.history.record(
                    Op::Replace(ReplaceOp::new(
                        guid.to_string(),
                        Item::Geometry(before),
                        Item::Geometry(obj.clone()),
                        entry.clone(),
                    )),
                    bytes,
                );
            }

            <span class="k">self</span>._swap(guid, Item::Geometry(obj), &amp;entry);

            <span class="k">return</span> <span class="s">true</span>;
        }

        <span class="k">let</span> node = <span class="k">self</span>.get_node(guid);
        <span class="k">let</span> Some(removed) = <span class="k">self</span>._half(<span class="s">false</span>, old, guid) <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="k">self</span>._kill(&amp;removed);
        <span class="k">let</span> slot = push(&amp;<span class="k">mut</span> <span class="k">self</span>.objects, &amp;Item::Geometry(obj.clone()));
        <span class="k">let</span> added = Tomb::new(new, <span class="s">false</span>, slot, None);
        pin(&amp;<span class="k">mut</span> <span class="k">self</span>.objects, new, slot, &amp;added);
        <span class="k">self</span>.lookup.insert(guid.to_string(), obj.clone());
        <span class="k">self</span>._label(guid, &amp;format!(&quot;{<span class="s">prefix</span>}<span class="s">_</span>{}&quot;, obj.name()));
        <span class="k">self</span>._pair(guid, old, new, node, removed, added, bytes);

        <span class="s">true</span>
    }

    <span class="c">/// Swap the geometry of a definition, which keeps its guid, so every instance of it changes at once; false when guid is no definition.</span>
    <span class="k">pub</span> <span class="k">fn</span> replace_definition(&amp;<span class="k">mut</span> <span class="k">self</span>, guid: &amp;str, definition: Geometry) -&gt; bool {
        <span class="k">let</span> Some(before) = <span class="k">self</span>.definition_lookup.get(guid).cloned() <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="k">let</span> <span class="k">mut</span> definition = definition;

        <span class="k">if</span> definition.guid() != guid {
            definition.set_guid(guid);
        }

        <span class="k">let</span> (old, _) = collection_of(&amp;before);
        <span class="k">let</span> (new, _) = collection_of(&amp;definition);
        <span class="k">let</span> bytes = RECORD + weight(&amp;Item::Geometry(before.clone()));

        <span class="k">if</span> old == new {
            <span class="k">let</span> Some(tomb) = <span class="k">self</span>._half(<span class="s">true</span>, old, guid) <span class="k">else</span> {
                <span class="k">return</span> <span class="s">false</span>;
            };
            <span class="k">let</span> entry = Entry::Definition(tomb);

            <span class="k">if</span> <span class="k">self</span>.history.current.is_some() {
                <span class="k">self</span>.history.record(
                    Op::Replace(ReplaceOp::new(
                        guid.to_string(),
                        Item::Geometry(before),
                        Item::Geometry(definition.clone()),
                        entry.clone(),
                    )),
                    bytes,
                );
            }

            <span class="k">self</span>._swap(guid, Item::Geometry(definition), &amp;entry);

            <span class="k">return</span> <span class="s">true</span>;
        }

        <span class="k">let</span> Some(removed) = <span class="k">self</span>._half(<span class="s">true</span>, old, guid) <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="k">self</span>._kill(&amp;removed);
        <span class="k">let</span> slot = push(&amp;<span class="k">mut</span> <span class="k">self</span>.definitions, &amp;Item::Geometry(definition.clone()));
        <span class="k">let</span> added = Tomb::new(new, <span class="s">true</span>, slot, None);
        pin(&amp;<span class="k">mut</span> <span class="k">self</span>.definitions, new, slot, &amp;added);
        <span class="k">self</span>.definition_lookup.insert(guid.to_string(), definition);
        <span class="k">self</span>._pair(
            guid,
            &quot;<span class="s">definitions</span>&quot;,
            &quot;<span class="s">definitions</span>&quot;,
            None,
            removed,
            added,
            bytes,
        );

        <span class="s">true</span>
    }

    <span class="c">/// Remove a definition; false when guid is no definition or an instance still names it.</span>
    <span class="k">pub</span> <span class="k">fn</span> remove_definition(&amp;<span class="k">mut</span> <span class="k">self</span>, guid: &amp;str) -&gt; bool {
        <span class="k">let</span> Some(before) = <span class="k">self</span>.definition_lookup.get(guid).cloned() <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };

        <span class="k">if</span> !<span class="k">self</span>.instances_of(guid).is_empty() {
            <span class="k">return</span> <span class="s">false</span>;
        }

        <span class="k">let</span> (collection, _) = collection_of(&amp;before);
        <span class="k">let</span> Some(tomb) = <span class="k">self</span>._half(<span class="s">true</span>, collection, guid) <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="k">self</span>._kill(&amp;tomb);

        <span class="k">if</span> <span class="k">self</span>.history.current.is_none() {
            <span class="k">self</span>.history.dropped += <span class="s">1</span>;

            <span class="k">return</span> <span class="s">true</span>;
        }

        <span class="k">self</span>.history.record(
            Op::Remove(Tombstone::new(
                guid.to_string(),
                &quot;<span class="s">definitions</span>&quot;.to_string(),
                None,
                <span class="s">0</span>,
                None,
                tomb,
            )),
            RECORD + weight(&amp;Item::Geometry(before)),
        );

        <span class="s">true</span>
    }

    <span class="c">/// Turn an object into an instance of a definition, keeping its guid, name, tree node and edges; frame maps the definition onto the object and is folded into its local transform.</span>
    <span class="k">pub</span> <span class="k">fn</span> to_instance(&amp;<span class="k">mut</span> <span class="k">self</span>, guid: &amp;str, definition_guid: &amp;str, frame: Xform) -&gt; bool {
        <span class="k">let</span> Some(object) = <span class="k">self</span>.lookup.get(guid).cloned() <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };

        <span class="k">if</span> !<span class="k">self</span>.definition_lookup.contains_key(definition_guid) {
            <span class="k">return</span> <span class="s">false</span>;
        }

        <span class="k">let</span> <span class="k">mut</span> instance = InstanceRef::new(definition_guid, Xform::identity());
        instance.set_guid(guid.to_string());
        instance.name = object.name().to_string();
        <span class="k">let</span> label = format!(&quot;<span class="s">instance_</span>{}&quot;, instance.name);
        <span class="k">let</span> placement = &amp;<span class="k">self</span>.xform(guid) * &amp;frame;
        <span class="k">let</span> (old, _) = collection_of(&amp;object);
        <span class="k">let</span> node = <span class="k">self</span>.get_node(guid);
        <span class="k">let</span> Some(removed) = <span class="k">self</span>._half(<span class="s">false</span>, old, guid) <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="k">self</span>._kill(&amp;removed);
        <span class="k">let</span> instance = Rc::new(instance);
        <span class="k">let</span> slot = push(&amp;<span class="k">mut</span> <span class="k">self</span>.objects, &amp;Item::InstanceRef(Rc::clone(&amp;instance)));
        <span class="k">let</span> added = Tomb::new(&quot;<span class="s">instances</span>&quot;, <span class="s">false</span>, slot, None);
        pin(&amp;<span class="k">mut</span> <span class="k">self</span>.objects, &quot;<span class="s">instances</span>&quot;, slot, &amp;added);
        <span class="k">self</span>.instance_lookup.insert(guid.to_string(), instance);
        <span class="k">self</span>._label(guid, &amp;label);
        <span class="k">let</span> bytes = RECORD + weight(&amp;Item::Geometry(object));
        <span class="k">self</span>._pair(guid, old, &quot;<span class="s">instances</span>&quot;, node, removed, added, bytes);

        <span class="k">if</span> placement.is_identity() {
            <span class="k">self</span>.remove_xform(guid);
        } <span class="k">else</span> {
            <span class="k">self</span>.set_xform(guid, placement);
        }

        <span class="s">true</span>
    }

    <span class="c">/// Turn an instance into a standalone copy of its definition in the definition frame, keeping its guid, name, transform, tree node and edges, and on an element its features.</span>
    <span class="k">pub</span> <span class="k">fn</span> explode(&amp;<span class="k">mut</span> <span class="k">self</span>, instance_guid: &amp;str) -&gt; bool {
        <span class="k">let</span> Some(definition) = <span class="k">self</span>.definition_of(instance_guid) <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="k">let</span> instance = Rc::clone(&amp;<span class="k">self</span>.instance_lookup[instance_guid]);
        <span class="k">let</span> copy = resolve(&amp;instance, &amp;definition, &amp;Xform::identity());
        <span class="k">let</span> (collection, prefix) = collection_of(&amp;copy);
        <span class="k">let</span> label = format!(&quot;{<span class="s">prefix</span>}<span class="s">_</span>{}&quot;, instance.name);
        <span class="k">let</span> node = <span class="k">self</span>.get_node(instance_guid);
        <span class="k">let</span> Some(removed) = <span class="k">self</span>._half(<span class="s">false</span>, &quot;<span class="s">instances</span>&quot;, instance_guid) <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };
        <span class="k">self</span>._kill(&amp;removed);
        <span class="k">let</span> slot = push(&amp;<span class="k">mut</span> <span class="k">self</span>.objects, &amp;Item::Geometry(copy.clone()));
        <span class="k">let</span> added = Tomb::new(collection, <span class="s">false</span>, slot, None);
        pin(&amp;<span class="k">mut</span> <span class="k">self</span>.objects, collection, slot, &amp;added);
        <span class="k">self</span>.lookup.insert(instance_guid.to_string(), copy);
        <span class="k">self</span>._label(instance_guid, &amp;label);
        <span class="k">let</span> bytes = RECORD + weight(&amp;Item::InstanceRef(instance));
        <span class="k">self</span>._pair(
            instance_guid,
            &quot;<span class="s">instances</span>&quot;,
            collection,
            node,
            removed,
            added,
            bytes,
        );

        <span class="s">true</span>
    }

    <span class="c">/// Set the LOCAL transform of an object, relative to its tree parent; a guid that names only a definition is ignored.</span>
    <span class="k">pub</span> <span class="k">fn</span> set_xform(&amp;<span class="k">mut</span> <span class="k">self</span>, guid: &amp;str, xform: Xform) {
        <span class="k">if</span> <span class="k">self</span>.definition_lookup.contains_key(guid)
            &amp;&amp; !<span class="k">self</span>.lookup.contains_key(guid)
            &amp;&amp; !<span class="k">self</span>.instance_lookup.contains_key(guid)
        {
            <span class="k">return</span>;
        }

        <span class="k">if</span> <span class="k">self</span>.history.current.is_some() {
            <span class="k">let</span> before = <span class="k">self</span>.xforms.get(guid).cloned();
            <span class="k">let</span> node = <span class="k">self</span>.get_node(guid);
            <span class="k">self</span>.history.record(
                Op::Xform(XformOp::new(
                    guid.to_string(),
                    before,
                    Some(xform.clone()),
                    node,
                )),
                RECORD,
            );
        }

        <span class="k">self</span>.xforms.insert(guid.to_string(), xform);
        <span class="k">self</span>.bvh_cache_dirty = <span class="s">true</span>;
        <span class="k">self</span>.revision += <span class="s">1</span>;
    }

    <span class="c">/// Remove an object's local transform, returning whether one was present.</span>
    <span class="k">pub</span> <span class="k">fn</span> remove_xform(&amp;<span class="k">mut</span> <span class="k">self</span>, guid: &amp;str) -&gt; bool {
        <span class="k">let</span> Some(before) = <span class="k">self</span>.xforms.get(guid).cloned() <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };

        <span class="k">if</span> <span class="k">self</span>.history.current.is_some() {
            <span class="k">let</span> node = <span class="k">self</span>.get_node(guid);
            <span class="k">self</span>.history.record(
                Op::Xform(XformOp::new(guid.to_string(), Some(before), None, node)),
                RECORD,
            );
        }

        <span class="k">self</span>.xforms.remove(guid);
        <span class="k">self</span>.bvh_cache_dirty = <span class="s">true</span>;
        <span class="k">self</span>.revision += <span class="s">1</span>;

        <span class="s">true</span>
    }

    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">// Session - Interactions</span>
    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">/// Make or reuse the pair's undirected edge, an existing edge keeping its attributes, and append interaction to its list; returns the stored interaction. Errs unless both elements are in the session and distinct.</span>
    <span class="k">pub</span> <span class="k">fn</span> add_interaction(
        &amp;<span class="k">mut</span> <span class="k">self</span>,
        a: &amp;Element,
        b: &amp;Element,
        interaction: Box&lt;<span class="k">dyn</span> Interaction&gt;,
    ) -&gt; Result&lt;&amp;<span class="k">dyn</span> Interaction, Box&lt;<span class="k">dyn</span> std::error::Error&gt;&gt; {
        <span class="k">let</span> first = a.guid();
        <span class="k">let</span> second = b.guid();

        <span class="k">if</span> first == second || !registered(<span class="k">self</span>, first) || !registered(<span class="k">self</span>, second) {
            <span class="k">return</span> Err(
                &quot;<span class="s">Session::add_interaction: add two distinct elements to the session first</span>&quot;.into(),
            );
        }

        <span class="k">if</span> !<span class="k">self</span>.graph.has_edge((first, second)) {
            <span class="k">self</span>.graph.add_edge(first, second, &quot;&quot;);
        }

        <span class="k">self</span>.revision += <span class="s">1</span>;

        <span class="k">let</span> id = <span class="k">self</span>.graph.edges[first][second].guid().to_string();
        <span class="k">let</span> list = <span class="k">self</span>.interactions.entry(id).or_default();
        list.push(interaction);

        Ok(list[list.len() - <span class="s">1</span>].as_ref())
    }

    <span class="c">/// The pair's interactions in either order, empty when there are none.</span>
    <span class="k">pub</span> <span class="k">fn</span> get_interaction(&amp;<span class="k">self</span>, a: &amp;Element, b: &amp;Element) -&gt; &amp;[Box&lt;<span class="k">dyn</span> Interaction&gt;] {
        <span class="k">if</span> !<span class="k">self</span>.graph.has_edge((a.guid(), b.guid())) {
            <span class="k">return</span> &amp;[];
        }

        <span class="k">let</span> id = <span class="k">self</span>.graph.edges[a.guid()][b.guid()].guid();

        <span class="k">match</span> <span class="k">self</span>.interactions.get(id) {
            Some(list) =&gt; list,
            None =&gt; &amp;[],
        }
    }

    <span class="c">/// True when the pair has an edge in either order.</span>
    <span class="k">pub</span> <span class="k">fn</span> has_interaction(&amp;<span class="k">self</span>, a: &amp;Element, b: &amp;Element) -&gt; bool {
        <span class="k">self</span>.graph.has_edge((a.guid(), b.guid()))
    }

    <span class="c">/// Remove the pair's edge and all of its interactions in either order; a missing pair is a no-op.</span>
    <span class="k">pub</span> <span class="k">fn</span> remove_interaction(&amp;<span class="k">mut</span> <span class="k">self</span>, a: &amp;Element, b: &amp;Element) {
        <span class="k">if</span> !<span class="k">self</span>.graph.has_edge((a.guid(), b.guid())) {
            <span class="k">return</span>;
        }

        <span class="k">let</span> id = <span class="k">self</span>.graph.edges[a.guid()][b.guid()].guid().to_string();
        <span class="k">self</span>.revision += <span class="s">1</span>;
        <span class="k">self</span>.interactions.remove(&amp;id);
        <span class="k">self</span>.graph.remove_edge((a.guid(), b.guid()));
    }

    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">// History</span>
    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">/// Open a history transaction: every add, remove, replace and xform change until commit becomes one undo step.</span>
    <span class="k">pub</span> <span class="k">fn</span> begin(&amp;<span class="k">mut</span> <span class="k">self</span>, label: &amp;str) {
        <span class="k">self</span>.history.begin(label);
    }

    <span class="c">/// Close the open transaction as one undo step.</span>
    <span class="k">pub</span> <span class="k">fn</span> commit(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">self</span>.history.commit();
    }

    <span class="c">/// Revert the latest committed transaction, returning whether there was one.</span>
    <span class="k">pub</span> <span class="k">fn</span> undo(&amp;<span class="k">mut</span> <span class="k">self</span>) -&gt; bool {
        <span class="k">self</span>.revision += <span class="s">1</span>;
        <span class="k">let</span> <span class="k">mut</span> history = std::mem::take(&amp;<span class="k">mut</span> <span class="k">self</span>.history);
        <span class="k">let</span> undone = history.undo(<span class="k">self</span>);
        <span class="k">self</span>.history = history;

        undone
    }

    <span class="c">/// Reapply the latest undone transaction, returning whether there was one.</span>
    <span class="k">pub</span> <span class="k">fn</span> redo(&amp;<span class="k">mut</span> <span class="k">self</span>) -&gt; bool {
        <span class="k">self</span>.revision += <span class="s">1</span>;
        <span class="k">let</span> <span class="k">mut</span> history = std::mem::take(&amp;<span class="k">mut</span> <span class="k">self</span>.history);
        <span class="k">let</span> redone = history.redo(<span class="k">self</span>);
        <span class="k">self</span>.history = history;

        redone
    }

    <span class="c">/// Revert and drop the open transaction, leaving the stacks as they are; false when none is open.</span>
    <span class="k">pub</span> <span class="k">fn</span> abort(&amp;<span class="k">mut</span> <span class="k">self</span>) -&gt; bool {
        <span class="k">self</span>.revision += <span class="s">1</span>;
        <span class="k">let</span> <span class="k">mut</span> history = std::mem::take(&amp;<span class="k">mut</span> <span class="k">self</span>.history);
        <span class="k">let</span> aborted = history.abort(<span class="k">self</span>);
        <span class="k">self</span>.history = history;

        aborted
    }

    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">// Purge</span>
    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">/// Return the dead slots not yet purged, over the objects and the definitions lists.</span>
    <span class="k">pub</span> <span class="k">fn</span> number_of_dead(&amp;<span class="k">self</span>) -&gt; usize {
        <span class="k">let</span> <span class="k">mut</span> count = <span class="s">0</span>;

        <span class="k">for</span> (collection, _) <span class="k">in</span> COLLECTIONS {
            count += list(&amp;<span class="k">self</span>.objects, collection).map_or(<span class="s">0</span>, Slots::number_of_dead);
            count += list(&amp;<span class="k">self</span>.definitions, collection).map_or(<span class="s">0</span>, Slots::number_of_dead);
        }

        count
    }

    <span class="c">/// Return whether dropped records left dead entries or swept parents a purge cycle can free.</span>
    <span class="k">pub</span> <span class="k">fn</span> purge_due(&amp;<span class="k">self</span>) -&gt; bool {
        <span class="k">self</span>.history.dropped &gt; <span class="s">0</span> &amp;&amp; (<span class="k">self</span>.number_of_dead() &gt; <span class="s">0</span> || !<span class="k">self</span>.sweep.is_empty())
    }

    <span class="c">/// Return whether a purge cycle is part way.</span>
    <span class="k">pub</span> <span class="k">fn</span> is_purging(&amp;<span class="k">self</span>) -&gt; bool {
        <span class="k">self</span>.purging.is_some()
    }

    <span class="c">/// Purge what no record reaches for at most \`work\` slots or children, resuming the running cycle; true while it is unfinished.</span>
    <span class="k">pub</span> <span class="k">fn</span> purge_step(&amp;<span class="k">mut</span> <span class="k">self</span>, work: usize) -&gt; bool {
        <span class="k">let</span> fresh = <span class="k">self</span>
            .writer
            .as_ref()
            .is_some_and(|writer| writer.revision == <span class="k">self</span>.revision);

        <span class="k">if</span> fresh || (<span class="k">self</span>.purging.is_none() &amp;&amp; !<span class="k">self</span>.purge_due()) {
            <span class="k">return</span> <span class="s">false</span>;
        }

        <span class="k">self</span>._purge(work);

        <span class="k">self</span>.purging.is_some()
    }

    <span class="c">/// Drop the history, then purge everything in one call: a whole cycle, every live tree node, dense graph indices no record could revive into; O(n + N + V log V + E log E).</span>
    <span class="k">pub</span> <span class="k">fn</span> purge(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">self</span>.history.clear();
        <span class="k">self</span>.writer = None;

        <span class="k">if</span> <span class="k">self</span>.purging.is_some() {
            <span class="k">self</span>._purge(usize::MAX);
        }

        <span class="k">self</span>._purge(usize::MAX);

        <span class="k">for</span> node <span class="k">in</span> <span class="k">self</span>.tree.nodes() {
            node.borrow_mut().compact();
        }

        <span class="k">self</span>.graph.renumber();
        <span class="k">self</span>.revision += <span class="s">1</span>;
    }

    <span class="c">/// Write the live session as protobuf bytes for at most \`work\` units, purging first when due; Some once done, history kept, restarted by any edit.</span>
    <span class="k">pub</span> <span class="k">fn</span> checkpoint(&amp;<span class="k">mut</span> <span class="k">self</span>, <span class="k">mut</span> work: usize) -&gt; Option&lt;Vec&lt;u8&gt;&gt; {
        <span class="k">if</span> <span class="k">self</span>
            .writer
            .as_ref()
            .is_some_and(|writer| writer.revision != <span class="k">self</span>.revision)
        {
            <span class="k">self</span>.writer = None;
        }

        <span class="k">if</span> <span class="k">self</span>.writer.is_none() &amp;&amp; (<span class="k">self</span>.purging.is_some() || <span class="k">self</span>.purge_due()) {
            work = <span class="k">self</span>._purge(work);
        }

        <span class="k">if</span> <span class="k">self</span>.purging.is_some() || work == <span class="s">0</span> {
            <span class="k">return</span> None;
        }

        <span class="k">let</span> <span class="k">mut</span> writer = <span class="k">self</span>
            .writer
            .take()
            .unwrap_or_else(|| Checkpoint::new(<span class="k">self</span>.revision));

        <span class="k">if</span> <span class="k">self</span>._write(&amp;<span class="k">mut</span> writer, work) {
            <span class="k">return</span> Some(writer.out);
        }

        <span class="k">self</span>.writer = Some(writer);

        None
    }

    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">// Collision detection and ray casting</span>
    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">/// Bounding box of an object in WORLD placement, inflated by tolerance.</span>
    <span class="k">pub</span> <span class="k">fn</span> compute_bounding_box(geometry: &amp;Geometry, xform: &amp;Xform) -&gt; OBB {
        <span class="k">let</span> inflate = Tolerance::APPROXIMATION;

        <span class="k">match</span> geometry {
            Geometry::Point(point) =&gt; OBB::from_point(&amp;xform.transform_point(point), inflate),
            Geometry::Plane(plane) =&gt; {
                OBB::from_point(&amp;xform.transform_point(&amp;plane.origin()), inflate * <span class="s">10</span>.<span class="s">0</span>)
            }

            Geometry::OBB(bbox) =&gt; {
                <span class="k">let</span> <span class="k">mut</span> inflated = (**bbox).clone();
                inflated.half_size = &amp;inflated.half_size + &amp;Vector::new(inflate, inflate, inflate);
                inflated.transform(xform);

                inflated
            }

            Geometry::Element(element) =&gt; {
                <span class="k">let</span> <span class="k">mut</span> copy = (**element).clone();
                <span class="k">let</span> <span class="k">mut</span> bbox = copy.aabb();
                bbox.transform(xform);

                bbox
            }

            _ =&gt; placed_box(&amp;box_points(geometry), xform, inflate),
        }
    }

    <span class="c">/// Get all collision pairs using SpatialBVH and add them as graph edges.</span>
    <span class="k">pub</span> <span class="k">fn</span> get_collisions(&amp;<span class="k">mut</span> <span class="k">self</span>) -&gt; Vec&lt;(String, String)&gt; {
        <span class="k">let</span> <span class="k">mut</span> guids: Vec&lt;String&gt; = Vec::new();
        <span class="k">let</span> boxes = <span class="k">self</span>._compute_boxes(&amp;<span class="k">mut</span> guids);

        <span class="k">if</span> boxes.is_empty() {
            <span class="k">return</span> Vec::new();
        }

        <span class="k">self</span>.bvh = SpatialBVH::from_boxes(&amp;boxes, SpatialBVH::compute_world_size(&amp;boxes));
        <span class="k">let</span> (pairs, _colliding, _checks) = <span class="k">self</span>.bvh.check_all_collisions(&amp;boxes);
        <span class="k">let</span> <span class="k">mut</span> guid_pairs: Vec&lt;(String, String)&gt; = Vec::with_capacity(pairs.len());

        <span class="k">for</span> (i, j) <span class="k">in</span> pairs {
            <span class="k">if</span> i &gt;= guids.len() || j &gt;= guids.len() {
                <span class="k">continue</span>;
            }

            guid_pairs.push((guids[i].clone(), guids[j].clone()));
            <span class="k">self</span>.graph.add_edge(&amp;guids[i], &amp;guids[j], &quot;<span class="s">bvh_collision</span>&quot;);
        }

        guid_pairs
    }

    <span class="c">/// Cast a ray through the scene, returning the hits within tolerance of the closest one.</span>
    <span class="k">pub</span> <span class="k">fn</span> ray_cast(&amp;<span class="k">mut</span> <span class="k">self</span>, origin: &amp;Point, direction: &amp;Vector, tolerance: f64) -&gt; Vec&lt;RayHit&gt; {
        <span class="k">if</span> <span class="k">self</span>.bvh_cache_dirty {
            <span class="k">self</span>._rebuild_ray_bvh_cache();
            <span class="k">self</span>.bvh_cache_dirty = <span class="s">false</span>;
        }

        <span class="k">if</span> <span class="k">self</span>.cached_guids.is_empty() {
            <span class="k">return</span> Vec::new();
        }

        <span class="k">let</span> <span class="k">mut</span> candidates: Vec&lt;usize&gt; = Vec::new();

        <span class="k">if</span> <span class="k">let</span> Some(bvh) = &amp;<span class="k">self</span>.cached_ray_bvh {
            bvh.ray_cast(origin, direction, &amp;<span class="k">mut</span> candidates, <span class="s">true</span>);
        }

        <span class="k">let</span> ray = Line::from_points(origin, &amp;(origin + direction * <span class="s">10000</span>.<span class="s">0</span>));
        <span class="k">let</span> world = <span class="k">self</span>.world_xforms();
        <span class="k">let</span> <span class="k">mut</span> hits: Vec&lt;RayHit&gt; = Vec::new();
        <span class="k">let</span> <span class="k">mut</span> closest = f64::INFINITY;

        <span class="k">for</span> index <span class="k">in</span> candidates {
            <span class="k">let</span> guid = <span class="k">self</span>.cached_guids[index].clone();
            <span class="k">let</span> Some(geometry) = <span class="k">self</span>
                .lookup
                .get(&amp;guid)
                .cloned()
                .or_else(|| <span class="k">self</span>.definition_of(&amp;guid))
            <span class="k">else</span> {
                <span class="k">continue</span>;
            };
            <span class="k">let</span> placement = world.get(&amp;guid).cloned().unwrap_or_else(Xform::identity);
            <span class="k">let</span> Some(hit) = <span class="k">self</span>._ray_intersect_geometry(&amp;ray, &amp;geometry, tolerance, &amp;placement)
            <span class="k">else</span> {
                <span class="k">continue</span>;
            };
            <span class="k">let</span> distance = origin.distance(&amp;hit, None);

            <span class="k">if</span> distance &gt;= closest {
                <span class="k">continue</span>;
            }

            <span class="k">if</span> distance &lt; closest - tolerance {
                hits.clear();
            }

            hits.push(RayHit {
                guid,
                hit_point: hit,
                distance,
            });
            closest = distance;
        }

        hits
    }

    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">// JSON</span>
    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">/// Serialize to a JSON object.</span>
    <span class="k">fn</span> to_json_value(&amp;<span class="k">self</span>) -&gt; Result&lt;serde_json::Value, Box&lt;<span class="k">dyn</span> std::error::Error&gt;&gt; {
        <span class="k">let</span> <span class="k">mut</span> xforms_json: Vec&lt;serde_json::Value&gt; = Vec::new();

        <span class="k">for</span> (obj_guid, obj_xform) <span class="k">in</span> <span class="k">self</span>._xforms_ordered() {
            xforms_json.push(serde_json::json!({ &quot;<span class="s">guid</span>&quot;: obj_guid, &quot;<span class="s">xform</span>&quot;: obj_xform }));
        }

        <span class="k">let</span> <span class="k">mut</span> interactions_json: Vec&lt;serde_json::Value&gt; = Vec::new();

        <span class="k">for</span> (edge, interactions) <span class="k">in</span> &amp;<span class="k">self</span>.interactions {
            <span class="k">let</span> <span class="k">mut</span> items: Vec&lt;serde_json::Value&gt; = Vec::new();

            <span class="k">for</span> interaction <span class="k">in</span> interactions {
                items.push(interaction.jsondump());
            }

            interactions_json.push(serde_json::json!({ &quot;<span class="s">guid</span>&quot;: edge, &quot;<span class="s">interactions</span>&quot;: items }));
        }

        <span class="k">let</span> <span class="k">mut</span> json_obj = serde_json::json!({
            &quot;<span class="s">type</span>&quot;: &quot;<span class="s">Session</span>&quot;,
            &quot;<span class="s">name</span>&quot;: <span class="k">self</span>.name,
            &quot;<span class="s">guid</span>&quot;: <span class="k">self</span>.guid(),
            &quot;<span class="s">objects</span>&quot;: <span class="k">self</span>.objects_synced(),
            &quot;<span class="s">tree</span>&quot;: <span class="k">self</span>.tree,
            &quot;<span class="s">graph</span>&quot;: <span class="k">self</span>.graph.to_json_value()?,
            &quot;<span class="s">interactions</span>&quot;: interactions_json,
            &quot;<span class="s">xforms</span>&quot;: xforms_json
        });

        <span class="k">if</span> !<span class="k">self</span>.definition_lookup.is_empty() {
            json_obj[&quot;<span class="s">definitions</span>&quot;] =
                serde_json::to_value(synced(&amp;<span class="k">self</span>.definitions, &amp;<span class="k">self</span>.definition_lookup))?;
        }

        Ok(json_obj)
    }

    <span class="c">/// Serialize to a sorted JSON string.</span>
    <span class="k">pub</span> <span class="k">fn</span> jsondump(&amp;<span class="k">self</span>) -&gt; Result&lt;String, Box&lt;<span class="k">dyn</span> std::error::Error&gt;&gt; {
        <span class="k">crate</span>::file_encoders::file_json_dumps(&amp;<span class="k">self</span>.to_json_value()?, <span class="s">false</span>)
    }

    <span class="c">/// Deserialize from a JSON string.</span>
    <span class="k">pub</span> <span class="k">fn</span> jsonload(json_data: &amp;str) -&gt; Result&lt;<span class="k">Self</span>, Box&lt;<span class="k">dyn</span> std::error::Error&gt;&gt; {
        <span class="k">let</span> json_obj: serde_json::Value = serde_json::from_str(json_data)?;
        <span class="k">let</span> <span class="k">mut</span> session = Session::new(json_obj[&quot;<span class="s">name</span>&quot;].as_str().unwrap_or(&quot;<span class="s">my_session</span>&quot;));

        <span class="k">if</span> <span class="k">let</span> Some(guid) = json_obj[&quot;<span class="s">guid</span>&quot;].as_str() {
            session.set_guid(guid.to_string());
        }

        <span class="k">if</span> json_obj[&quot;<span class="s">objects</span>&quot;].is_object() {
            session.objects = serde_json::from_value(json_obj[&quot;<span class="s">objects</span>&quot;].clone())?;
        }

        <span class="k">if</span> json_obj[&quot;<span class="s">tree</span>&quot;].is_object() {
            session.tree = serde_json::from_value(json_obj[&quot;<span class="s">tree</span>&quot;].clone())?;
        }

        <span class="k">if</span> json_obj[&quot;<span class="s">graph</span>&quot;].is_object() {
            session.graph = Graph::jsonload(&amp;serde_json::to_string(&amp;json_obj[&quot;<span class="s">graph</span>&quot;])?)?;
        }

        <span class="k">if</span> json_obj[&quot;<span class="s">definitions</span>&quot;].is_object() {
            session.definitions = serde_json::from_value(json_obj[&quot;<span class="s">definitions</span>&quot;].clone())?;
        }

        <span class="k">if</span> <span class="k">let</span> Some(entries) = json_obj[&quot;<span class="s">xforms</span>&quot;].as_array() {
            <span class="k">for</span> entry <span class="k">in</span> entries {
                <span class="k">let</span> Some(guid) = entry[&quot;<span class="s">guid</span>&quot;].as_str() <span class="k">else</span> {
                    <span class="k">continue</span>;
                };
                <span class="k">let</span> xform: Xform = serde_json::from_value(entry[&quot;<span class="s">xform</span>&quot;].clone())?;
                session.xforms.insert(guid.to_string(), xform);
            }
        }

        <span class="k">if</span> <span class="k">let</span> Some(entries) = json_obj[&quot;<span class="s">interactions</span>&quot;].as_array() {
            <span class="k">for</span> entry <span class="k">in</span> entries {
                <span class="k">let</span> guid = entry[&quot;<span class="s">guid</span>&quot;].as_str().unwrap_or(&quot;&quot;).to_string();
                <span class="k">let</span> Some(items) = entry[&quot;<span class="s">interactions</span>&quot;].as_array() <span class="k">else</span> {
                    <span class="k">continue</span>;
                };

                <span class="k">for</span> item <span class="k">in</span> items {
                    session
                        .interactions
                        .entry(guid.clone())
                        .or_default()
                        .push(&lt;<span class="k">dyn</span> Interaction&gt;::jsonload(item));
                }
            }
        }

        session.reindex();

        Ok(session)
    }

    <span class="c">/// Serialize to a JSON string; saving purges, which clears the history, so no undo crosses a save.</span>
    <span class="k">pub</span> <span class="k">fn</span> file_json_dumps(&amp;<span class="k">mut</span> <span class="k">self</span>) -&gt; String {
        <span class="k">self</span>.purge();

        <span class="k">self</span>.jsondump().expect(&quot;<span class="s">Failed to serialize Session JSON</span>&quot;)
    }

    <span class="c">/// Deserialize from a JSON string.</span>
    <span class="k">pub</span> <span class="k">fn</span> file_json_loads(json_string: &amp;str) -&gt; <span class="k">Self</span> {
        <span class="k">Self</span>::jsonload(json_string).expect(&quot;<span class="s">Failed to parse Session JSON</span>&quot;)
    }

    <span class="c">/// Write to a JSON file.</span>
    <span class="k">pub</span> <span class="k">fn</span> file_json_dump(&amp;<span class="k">mut</span> <span class="k">self</span>, filename: &amp;str) {
        <span class="k">self</span>.purge();

        <span class="k">let</span> json_obj = <span class="k">self</span>
            .to_json_value()
            .expect(&quot;<span class="s">Failed to serialize Session JSON</span>&quot;);

        <span class="k">crate</span>::file_encoders::file_json_dump(&amp;json_obj, filename, <span class="s">true</span>)
            .expect(&quot;<span class="s">Failed to write JSON file</span>&quot;);
    }

    <span class="c">/// Read from a JSON file.</span>
    <span class="k">pub</span> <span class="k">fn</span> file_json_load(filename: &amp;str) -&gt; Result&lt;<span class="k">Self</span>, Box&lt;<span class="k">dyn</span> std::error::Error&gt;&gt; {
        <span class="k">Self</span>::jsonload(&amp;fs::read_to_string(filename)?)
    }

    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">// Protobuf</span>
    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">/// Convert to the protobuf message.</span>
    <span class="k">pub</span> <span class="k">fn</span> to_proto(&amp;<span class="k">self</span>) -&gt; <span class="k">crate</span>::proto::Session {
        <span class="k">use</span> prost::Message;

        <span class="k">let</span> <span class="k">mut</span> xforms: Vec&lt;<span class="k">crate</span>::proto::XformEntry&gt; = Vec::new();

        <span class="k">for</span> (obj_guid, obj_xform) <span class="k">in</span> <span class="k">self</span>._xforms_ordered() {
            xforms.push(<span class="k">crate</span>::proto::XformEntry {
                guid: obj_guid,
                xform: Some(obj_xform.to_proto()),
            });
        }

        <span class="k">let</span> <span class="k">mut</span> interactions: Vec&lt;<span class="k">crate</span>::proto::InteractionEntry&gt; = Vec::new();

        <span class="k">for</span> (edge, list) <span class="k">in</span> &amp;<span class="k">self</span>.interactions {
            <span class="k">let</span> <span class="k">mut</span> items: Vec&lt;<span class="k">crate</span>::proto::Interaction&gt; = Vec::new();

            <span class="k">for</span> interaction <span class="k">in</span> list {
                items.push(interaction.to_proto());
            }

            interactions.push(<span class="k">crate</span>::proto::InteractionEntry {
                guid: edge.clone(),
                interactions: items,
            });
        }

        <span class="k">let</span> definitions = <span class="k">if</span> <span class="k">self</span>.definition_lookup.is_empty() {
            None
        } <span class="k">else</span> {
            Some(synced(&amp;<span class="k">self</span>.definitions, &amp;<span class="k">self</span>.definition_lookup).to_proto())
        };

        <span class="k">crate</span>::proto::Session {
            name: <span class="k">self</span>.name.clone(),
            guid: <span class="k">self</span>.guid.get().cloned().unwrap_or_default(),
            objects: Some(<span class="k">self</span>.objects_synced().to_proto()),
            tree: <span class="k">crate</span>::proto::Tree::decode(<span class="k">self</span>.tree.pb_dumps().as_slice()).ok(),
            graph: Some(<span class="k">self</span>.graph.to_proto()),
            bvh_boxes: Vec::new(),
            xforms,
            definitions,
            interactions,
        }
    }

    <span class="c">/// Construct from the protobuf message.</span>
    <span class="k">pub</span> <span class="k">fn</span> from_proto(proto: <span class="k">crate</span>::proto::Session) -&gt; Result&lt;<span class="k">Self</span>, Box&lt;<span class="k">dyn</span> std::error::Error&gt;&gt; {
        <span class="k">use</span> prost::Message;

        <span class="k">let</span> <span class="k">mut</span> session = Session::new(&amp;proto.name);

        <span class="k">if</span> !proto.guid.is_empty() {
            session.set_guid(proto.guid);
        }

        <span class="k">if</span> <span class="k">let</span> Some(objects) = proto.objects {
            session.objects = Objects::from_proto(objects)?;
        }

        <span class="k">if</span> <span class="k">let</span> Some(tree) = proto.tree {
            session.tree = Tree::pb_loads(&amp;tree.encode_to_vec())?;
        }

        <span class="k">if</span> <span class="k">let</span> Some(graph) = proto.graph {
            session.graph = Graph::from_proto(graph);
        }

        <span class="k">if</span> <span class="k">let</span> Some(definitions) = proto.definitions {
            session.definitions = Objects::from_proto(definitions)?;
        }

        <span class="k">for</span> entry <span class="k">in</span> proto.xforms {
            <span class="k">let</span> Some(xform) = entry.xform <span class="k">else</span> {
                <span class="k">continue</span>;
            };
            session.xforms.insert(entry.guid, Xform::from_proto(xform));
        }

        <span class="k">for</span> entry <span class="k">in</span> proto.interactions {
            <span class="k">for</span> item <span class="k">in</span> entry.interactions {
                session
                    .interactions
                    .entry(entry.guid.clone())
                    .or_default()
                    .push(&lt;<span class="k">dyn</span> Interaction&gt;::from_proto(item));
            }
        }

        session.reindex();

        Ok(session)
    }

    <span class="c">/// Serialize to protobuf bytes; saving purges, which clears the history, so no undo crosses a save.</span>
    <span class="k">pub</span> <span class="k">fn</span> pb_dumps(&amp;<span class="k">mut</span> <span class="k">self</span>) -&gt; Vec&lt;u8&gt; {
        <span class="k">use</span> prost::Message;

        <span class="k">self</span>.purge();

        <span class="k">self</span>.to_proto().encode_to_vec()
    }

    <span class="c">/// Deserialize from protobuf bytes.</span>
    <span class="k">pub</span> <span class="k">fn</span> pb_loads(data: &amp;[u8]) -&gt; Result&lt;<span class="k">Self</span>, Box&lt;<span class="k">dyn</span> std::error::Error&gt;&gt; {
        <span class="k">use</span> prost::Message;

        <span class="k">Self</span>::from_proto(<span class="k">crate</span>::proto::Session::decode(data)?)
    }

    <span class="c">/// Write to a protobuf file.</span>
    <span class="k">pub</span> <span class="k">fn</span> pb_dump(&amp;<span class="k">mut</span> <span class="k">self</span>, filename: &amp;str) {
        fs::write(filename, <span class="k">self</span>.pb_dumps()).expect(&quot;<span class="s">Failed to write protobuf file</span>&quot;);
    }

    <span class="c">/// Read from a protobuf file.</span>
    <span class="k">pub</span> <span class="k">fn</span> pb_load(filename: &amp;str) -&gt; Result&lt;<span class="k">Self</span>, Box&lt;<span class="k">dyn</span> std::error::Error&gt;&gt; {
        <span class="k">Self</span>::pb_loads(&amp;fs::read(filename)?)
    }

    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">// String</span>
    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">/// Return the spatial hierarchy and the element interactions as a banner block.</span>
    <span class="k">pub</span> <span class="k">fn</span> str(&amp;<span class="k">self</span>) -&gt; String {
        <span class="k">let</span> bar = &quot;<span class="s">=</span>&quot;.repeat(<span class="s">80</span>);

        format!(
            &quot;{<span class="s">0</span>}<span class="s">\\nSpatial Hierarchy\\n</span>{<span class="s">0</span>}<span class="s">\\n</span>{<span class="s">1</span>}{<span class="s">0</span>}<span class="s">\\nElement Interactions\\n</span>{<span class="s">0</span>}<span class="s">\\n</span>{<span class="s">2</span>}<span class="s">\\n</span>{<span class="s">0</span>}<span class="s">\\n</span>&quot;,
            bar,
            <span class="k">self</span>.tree.str(),
            <span class="k">self</span>.graph.str()
        )
    }

    <span class="c">/// Return &quot;Session(name=..., objects=..., tree=..., graph=...)&quot;.</span>
    <span class="k">pub</span> <span class="k">fn</span> repr(&amp;<span class="k">self</span>) -&gt; String {
        format!(
            &quot;<span class="s">Session(name=</span>{}<span class="s">, objects=</span>{}<span class="s">, tree=</span>{}<span class="s">, graph=</span>{}<span class="s">)</span>&quot;,
            <span class="k">self</span>.name,
            <span class="k">self</span>.objects,
            <span class="k">self</span>.tree.repr(),
            <span class="k">self</span>.graph.repr()
        )
    }

    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">// Details</span>
    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">/// The objects vectors re-pointed at \`lookup\` and \`instance_lookup\`: \`Rc::make_mut\` on a lookup entry splits it from the vector, and the lookup is the mutable truth.</span>
    <span class="k">fn</span> objects_synced(&amp;<span class="k">self</span>) -&gt; Objects {
        <span class="k">let</span> <span class="k">mut</span> objects = synced(&amp;<span class="k">self</span>.objects, &amp;<span class="k">self</span>.lookup);

        <span class="k">for</span> (guid, instance) <span class="k">in</span> &amp;<span class="k">self</span>.instance_lookup {
            <span class="k">if</span> <span class="k">let</span> Some(slot) = objects.instances.get_slot(guid) {
                objects.instances.set_item(slot, Rc::clone(instance));
            }
        }

        objects
    }

    <span class="c">/// Store an object in its list, lookup, graph and tree, recording an add when a transaction is open; Err carries a guid that is already live, as an object or a definition, which adds nothing.</span>
    <span class="k">fn</span> _add_object(
        &amp;<span class="k">mut</span> <span class="k">self</span>,
        collection: &amp;str,
        obj: <span class="k">impl</span> Into&lt;Item&gt;,
        type_prefix: &amp;str,
        parent: Option&lt;&amp;Rc&lt;RefCell&lt;TreeNode&gt;&gt;&gt;,
    ) -&gt; Result&lt;Rc&lt;RefCell&lt;TreeNode&gt;&gt;, String&gt; {
        <span class="k">let</span> obj: Item = obj.into();
        <span class="k">let</span> guid = obj.guid().to_string();

        <span class="k">if</span> <span class="k">self</span>._is_live(&amp;guid) || <span class="k">self</span>.definition_lookup.contains_key(&amp;guid) {
            <span class="k">return</span> Err(guid);
        }

        <span class="k">let</span> slot = push(&amp;<span class="k">mut</span> <span class="k">self</span>.objects, &amp;obj);
        <span class="k">let</span> attribute = format!(&quot;{<span class="s">type_prefix</span>}<span class="s">_</span>{}&quot;, obj.name());
        <span class="k">self</span>._hold(&amp;guid, obj);
        <span class="k">self</span>.graph.add_node(&amp;guid, &amp;attribute);
        <span class="k">self</span>.bvh_cache_dirty = <span class="s">true</span>;
        <span class="k">let</span> node = TreeNode::new(&amp;guid);
        <span class="k">self</span>.node_lookup.insert(guid.clone(), Rc::clone(&amp;node));
        <span class="k">self</span>.revision += <span class="s">1</span>;
        <span class="k">let</span> host = parent.cloned().or_else(|| <span class="k">self</span>.tree.root());
        <span class="k">let</span> <span class="k">mut</span> parent_guid: Option&lt;String&gt; = None;

        <span class="k">if</span> <span class="k">let</span> Some(host) = &amp;host {
            <span class="k">self</span>.tree.add(&amp;node, Some(host));
            parent_guid = Some(host.borrow().name.clone());
        }

        <span class="k">if</span> <span class="k">self</span>.history.current.is_some() {
            <span class="k">let</span> tomb = Tomb::new(collection, <span class="s">false</span>, slot, Some(Rc::clone(&amp;node)));
            pin(&amp;<span class="k">mut</span> <span class="k">self</span>.objects, collection, slot, &amp;tomb);
            node.borrow_mut().set_tomb(&amp;tomb);
            <span class="k">let</span> index = node.borrow().at();
            <span class="k">self</span>.history.record(
                Op::Add(Tombstone::new(
                    guid,
                    collection.to_string(),
                    parent_guid,
                    index,
                    Some(Rc::clone(&amp;node)),
                    tomb,
                )),
                RECORD,
            );
        }

        Ok(node)
    }

    <span class="c">/// The node of a live guid, a detached one named guid when the object is outside the tree.</span>
    <span class="k">fn</span> _node_of(&amp;<span class="k">self</span>, guid: &amp;str) -&gt; Rc&lt;RefCell&lt;TreeNode&gt;&gt; {
        <span class="k">self</span>.get_node(guid).unwrap_or_else(|| TreeNode::new(guid))
    }

    <span class="c">/// Whether the live entry under guid, if any, is another entry than the one in a slot of the list of that name: one on the other side of the object/definition divide, of another type, or of the same type at another slot.</span>
    <span class="k">fn</span> _twin(&amp;<span class="k">self</span>, definition: bool, collection: &amp;str, slot: usize, guid: &amp;str) -&gt; bool {
        <span class="k">let</span> (objects, held, other) = <span class="k">if</span> definition {
            <span class="k">let</span> held = <span class="k">self</span>
                .definition_lookup
                .get(guid)
                .cloned()
                .map(Item::Geometry);

            (&amp;<span class="k">self</span>.definitions, held, <span class="k">self</span>._is_live(guid))
        } <span class="k">else</span> {
            <span class="k">let</span> other = <span class="k">self</span>.definition_lookup.contains_key(guid);

            (&amp;<span class="k">self</span>.objects, <span class="k">self</span>._item(guid), other)
        };

        <span class="k">if</span> other {
            <span class="k">return</span> <span class="s">true</span>;
        }

        <span class="k">let</span> Some(held) = held <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };

        collection_for(&amp;held).<span class="s">0</span> != collection || slot_of(objects, collection, guid) != Some(slot)
    }

    <span class="c">/// Whether the entry under guid is the one a record was taken on: any entry when no node was recorded, else the live entry whose node it is.</span>
    <span class="k">fn</span> _owns(&amp;<span class="k">self</span>, guid: &amp;str, node: Option&lt;&amp;Rc&lt;RefCell&lt;TreeNode&gt;&gt;&gt;) -&gt; bool {
        node.is_none_or(|node| {
            <span class="k">self</span>.get_node(guid)
                .is_some_and(|live| Rc::ptr_eq(&amp;live, node))
        })
    }

    <span class="c">/// Whether guid names a live object, component or instance.</span>
    <span class="k">fn</span> _is_live(&amp;<span class="k">self</span>, guid: &amp;str) -&gt; bool {
        <span class="k">self</span>.lookup.contains_key(guid)
            || <span class="k">self</span>.component_lookup.contains_key(guid)
            || <span class="k">self</span>.instance_lookup.contains_key(guid)
    }

    <span class="c">/// Take the object, component or instance under guid out of its map, the pointer itself.</span>
    <span class="k">fn</span> _take(&amp;<span class="k">mut</span> <span class="k">self</span>, guid: &amp;str) -&gt; Option&lt;Item&gt; {
        <span class="k">if</span> <span class="k">let</span> Some(geometry) = <span class="k">self</span>.lookup.remove(guid) {
            <span class="k">return</span> Some(Item::Geometry(geometry));
        }

        <span class="k">if</span> <span class="k">let</span> Some(component) = <span class="k">self</span>.component_lookup.remove(guid) {
            <span class="k">return</span> Some(Item::Component(component));
        }

        Some(Item::InstanceRef(<span class="k">self</span>.instance_lookup.remove(guid)?))
    }

    <span class="c">/// Put an object, component or instance in its map under guid.</span>
    <span class="k">fn</span> _hold(&amp;<span class="k">mut</span> <span class="k">self</span>, guid: &amp;str, item: Item) {
        <span class="k">match</span> item {
            Item::Geometry(geometry) =&gt; {
                <span class="k">self</span>.lookup.insert(guid.to_string(), geometry);
            }

            Item::Component(component) =&gt; {
                <span class="k">self</span>.component_lookup.insert(guid.to_string(), component);
            }

            Item::InstanceRef(instance) =&gt; {
                <span class="k">self</span>.instance_lookup.insert(guid.to_string(), instance);
            }
        }
    }

    <span class="c">/// The stored object, component or instance under guid, the pointer itself.</span>
    <span class="k">fn</span> _item(&amp;<span class="k">self</span>, guid: &amp;str) -&gt; Option&lt;Item&gt; {
        <span class="k">if</span> <span class="k">let</span> Some(geometry) = <span class="k">self</span>.lookup.get(guid) {
            <span class="k">return</span> Some(Item::Geometry(geometry.clone()));
        }

        <span class="k">if</span> <span class="k">let</span> Some(component) = <span class="k">self</span>.component_lookup.get(guid) {
            <span class="k">return</span> Some(Item::Component(component.clone()));
        }

        Some(Item::InstanceRef(Rc::clone(
            <span class="k">self</span>.instance_lookup.get(guid)?,
        )))
    }

    <span class="c">/// The object tomb of a live guid holding item, reused while a record still holds it, else made and pinned on its slot and node; O(1).</span>
    <span class="k">fn</span> _tomb(&amp;<span class="k">mut</span> <span class="k">self</span>, guid: &amp;str, item: &amp;Item) -&gt; Rc&lt;Tomb&gt; {
        <span class="k">let</span> (collection, _) = collection_for(item);
        <span class="k">let</span> slot = <span class="k">match</span> slot_of(&amp;<span class="k">self</span>.objects, collection, guid) {
            Some(slot) =&gt; slot,
            None =&gt; push(&amp;<span class="k">mut</span> <span class="k">self</span>.objects, item),
        };

        <span class="k">if</span> <span class="k">let</span> Some(tomb) = tomb_at(&amp;<span class="k">self</span>.objects, collection, slot) {
            <span class="k">if</span> tomb.node.is_some() {
                <span class="k">return</span> tomb;
            }
        }

        <span class="k">let</span> node = <span class="k">match</span> <span class="k">self</span>.get_node(guid) {
            Some(node) =&gt; {
                <span class="k">self</span>.node_lookup.insert(guid.to_string(), Rc::clone(&amp;node));
                node
            }

            None =&gt; TreeNode::new(guid),
        };
        <span class="k">let</span> tomb = Tomb::new(collection, <span class="s">false</span>, slot, Some(Rc::clone(&amp;node)));
        pin(&amp;<span class="k">mut</span> <span class="k">self</span>.objects, collection, slot, &amp;tomb);
        node.borrow_mut().set_tomb(&amp;tomb);

        tomb
    }

    <span class="c">/// The node-only tomb pinned on a node, reused while a record still holds it.</span>
    <span class="k">fn</span> _node_tomb(&amp;<span class="k">mut</span> <span class="k">self</span>, node: &amp;Rc&lt;RefCell&lt;TreeNode&gt;&gt;) -&gt; Rc&lt;Tomb&gt; {
        <span class="k">if</span> <span class="k">let</span> Some(tomb) = node.borrow().get_tomb() {
            <span class="k">if</span> tomb.collection.is_empty() {
                <span class="k">return</span> tomb;
            }
        }

        <span class="k">let</span> tomb = Tomb::new(&quot;&quot;, <span class="s">false</span>, <span class="s">0</span>, Some(Rc::clone(node)));
        node.borrow_mut().set_tomb(&amp;tomb);

        tomb
    }

    <span class="c">/// Record a tree add of node, which left ghost at its old parent or revived when was_dead; outside a transaction it only counts a dropped move.</span>
    <span class="k">fn</span> _record_add(
        &amp;<span class="k">mut</span> <span class="k">self</span>,
        node: &amp;Rc&lt;RefCell&lt;TreeNode&gt;&gt;,
        ghost: Option&lt;Rc&lt;RefCell&lt;TreeNode&gt;&gt;&gt;,
        was_dead: bool,
    ) {
        <span class="k">if</span> <span class="k">self</span>.history.current.is_none() {
            <span class="k">self</span>.history.dropped += usize::from(ghost.is_some());

            <span class="k">return</span>;
        }

        <span class="k">let</span> name = node.borrow().name.clone();
        <span class="k">let</span> tomb = <span class="k">self</span>._node_tomb(ghost.as_ref().unwrap_or(node));
        <span class="k">let</span> color = node.borrow().color.clone();
        <span class="k">let</span> dead_before = was_dead || ghost.is_none();
        <span class="k">self</span>.history.record(
            Op::Tree(TreeOp::new(
                name.clone(),
                Rc::clone(node),
                tomb,
                ghost,
                name.clone(),
                name,
                color.clone(),
                color,
                dead_before,
                <span class="s">false</span>,
            )),
            RECORD,
        );
    }

    <span class="c">/// A slot-only tomb on the live slot of guid in the list of that name, reused while a record still holds it; a map-only entry is pushed first.</span>
    <span class="k">fn</span> _half(&amp;<span class="k">mut</span> <span class="k">self</span>, definition: bool, collection: &amp;str, guid: &amp;str) -&gt; Option&lt;Rc&lt;Tomb&gt;&gt; {
        <span class="k">let</span> item = <span class="k">if</span> definition {
            Item::Geometry(<span class="k">self</span>.definition_lookup.get(guid)?.clone())
        } <span class="k">else</span> {
            <span class="k">self</span>._item(guid)?
        };
        <span class="k">let</span> objects = <span class="k">if</span> definition {
            &amp;<span class="k">mut</span> <span class="k">self</span>.definitions
        } <span class="k">else</span> {
            &amp;<span class="k">mut</span> <span class="k">self</span>.objects
        };
        <span class="k">let</span> slot = slot_of(objects, collection, guid).unwrap_or_else(|| push(objects, &amp;item));

        <span class="k">if</span> <span class="k">let</span> Some(tomb) = tomb_at(objects, collection, slot) {
            <span class="k">if</span> tomb.node.is_none() &amp;&amp; tomb.definition == definition {
                <span class="k">return</span> Some(tomb);
            }
        }

        <span class="k">let</span> tomb = Tomb::new(collection, definition, slot, None);
        pin(objects, collection, slot, &amp;tomb);

        Some(tomb)
    }

    <span class="c">/// Record the halves of a type change under one guid, or drop them when no transaction is open.</span>
    #[allow(clippy::too_many_arguments)]
    <span class="k">fn</span> _pair(
        &amp;<span class="k">mut</span> <span class="k">self</span>,
        guid: &amp;str,
        old: &amp;str,
        new: &amp;str,
        node: Option&lt;Rc&lt;RefCell&lt;TreeNode&gt;&gt;&gt;,
        removed: Rc&lt;Tomb&gt;,
        added: Rc&lt;Tomb&gt;,
        bytes: usize,
    ) {
        <span class="k">self</span>.revision += <span class="s">1</span>;
        <span class="k">self</span>.bvh_cache_dirty = <span class="s">true</span>;

        <span class="k">if</span> <span class="k">self</span>.history.current.is_none() {
            <span class="k">self</span>.history.dropped += <span class="s">1</span>;

            <span class="k">return</span>;
        }

        <span class="k">let</span> index = node.as_ref().map_or(<span class="s">0</span>, |node| node.borrow().at());
        <span class="k">let</span> parent_guid = node
            .as_ref()
            .and_then(|node| node.borrow().parent())
            .map(|parent| parent.borrow().name.clone());
        <span class="k">self</span>.history.record(
            Op::Remove(Tombstone::new(
                guid.to_string(),
                old.to_string(),
                parent_guid.clone(),
                index,
                node.clone(),
                removed,
            )),
            bytes,
        );
        <span class="k">self</span>.history.record(
            Op::Add(Tombstone::new(
                guid.to_string(),
                new.to_string(),
                parent_guid,
                index,
                node,
                added,
            )),
            RECORD,
        );
    }

    <span class="c">/// Relabel the graph vertex of guid, when it has one.</span>
    <span class="k">fn</span> _label(&amp;<span class="k">mut</span> <span class="k">self</span>, guid: &amp;str, label: &amp;str) {
        <span class="k">if</span> <span class="k">self</span>.graph.has_node(guid) {
            <span class="k">self</span>.graph.node_label(guid, Some(label));
        }
    }

    <span class="c">/// Remember a parent whose child died, once, for the sweep.</span>
    <span class="k">fn</span> _queue(&amp;<span class="k">mut</span> <span class="k">self</span>, parent: &amp;Rc&lt;RefCell&lt;TreeNode&gt;&gt;) {
        <span class="k">if</span> parent.borrow().is_queued() {
            <span class="k">return</span>;
        }

        parent.borrow_mut().set_queued(<span class="s">true</span>);
        <span class="k">self</span>.sweep.push(Rc::downgrade(parent));
    }

    <span class="c">/// Run the purge cycle for at most \`work\` units, starting one when idle; returns the work left.</span>
    <span class="k">fn</span> _purge(&amp;<span class="k">mut</span> <span class="k">self</span>, <span class="k">mut</span> work: usize) -&gt; usize {
        <span class="k">if</span> <span class="k">self</span>.purging.is_none() {
            <span class="k">self</span>.purging = Some(<span class="s">0</span>);
            <span class="k">self</span>.history.dropped = <span class="s">0</span>;
        }

        <span class="k">while</span> work &gt; <span class="s">0</span> {
            <span class="k">let</span> Some(phase) = <span class="k">self</span>.purging <span class="k">else</span> {
                <span class="k">break</span>;
            };

            <span class="k">if</span> phase &lt; <span class="s">26</span> {
                work = <span class="k">self</span>._purge_list(phase, work);
                <span class="k">continue</span>;
            }

            <span class="k">let</span> Some(weak) = <span class="k">self</span>.sweep.last() <span class="k">else</span> {
                <span class="k">self</span>.sweep = std::mem::take(&amp;<span class="k">mut</span> <span class="k">self</span>.pinned);
                <span class="k">self</span>.purging = None;
                <span class="k">break</span>;
            };
            <span class="k">let</span> Some(parent) = weak.upgrade() <span class="k">else</span> {
                <span class="k">self</span>.sweep.pop();
                work -= <span class="s">1</span>;
                <span class="k">continue</span>;
            };
            <span class="k">let</span> spent = parent.borrow_mut().compact_step(work);
            work -= spent.clamp(<span class="s">1</span>, work);

            <span class="k">if</span> parent.borrow().is_compacting() {
                <span class="k">continue</span>;
            }

            <span class="k">self</span>.sweep.pop();

            <span class="k">if</span> parent.borrow().has_dead() {
                <span class="k">self</span>.pinned.push(Rc::downgrade(&amp;parent));
            } <span class="k">else</span> {
                parent.borrow_mut().set_queued(<span class="s">false</span>);
            }
        }

        work
    }

    <span class="c">/// Compact the list of a purge phase (objects below 13, definitions from 13) for at most \`work\` slots, stepping to the next phase once it is done; returns the work left.</span>
    <span class="k">fn</span> _purge_list(&amp;<span class="k">mut</span> <span class="k">self</span>, phase: usize, work: usize) -&gt; usize {
        <span class="k">let</span> objects = <span class="k">if</span> phase &lt; <span class="s">13</span> {
            &amp;<span class="k">mut</span> <span class="k">self</span>.objects
        } <span class="k">else</span> {
            &amp;<span class="k">mut</span> <span class="k">self</span>.definitions
        };
        <span class="k">let</span> <span class="k">mut</span> spent = <span class="s">0</span>;
        <span class="k">let</span> <span class="k">mut</span> done = <span class="s">true</span>;

        <span class="k">if</span> <span class="k">let</span> Some(list) = list_mut(objects, COLLECTIONS[phase % <span class="s">13</span>].<span class="s">0</span>) {
            <span class="k">if</span> list.number_of_dead() &gt; <span class="s">0</span> || list.is_compacting() {
                spent = list.compact_step(work);
                done = !list.is_compacting();
            }
        }

        <span class="k">if</span> done {
            <span class="k">self</span>.purging = Some(phase + <span class="s">1</span>);
        }

        work - spent.min(work)
    }

    <span class="c">/// Advance a checkpoint writer for at most \`work\` units; true once its message is complete.</span>
    <span class="k">fn</span> _write(&amp;<span class="k">self</span>, writer: &amp;<span class="k">mut</span> Checkpoint, <span class="k">mut</span> work: usize) -&gt; bool {
        <span class="k">use</span> prost::Message;

        <span class="k">while</span> work &gt; <span class="s">0</span> {
            <span class="k">let</span> spent = <span class="k">match</span> writer.phase {
                HEAD =&gt; {
                    writer.sections[<span class="s">0</span>] = <span class="k">crate</span>::proto::Session {
                        name: <span class="k">self</span>.name.clone(),
                        guid: <span class="k">self</span>.guid.get().cloned().unwrap_or_default(),
                        ..Default::default()
                    }
                    .encode_to_vec();
                    writer.sections[<span class="s">1</span>] = objects_head(&amp;<span class="k">self</span>.objects);
                    writer.phase = OBJECTS;

                    <span class="s">1</span>
                }

                OBJECTS..TREE =&gt; <span class="k">self</span>._write_list(writer, <span class="s">false</span>, work),
                TREE =&gt; <span class="k">self</span>._write_tree(writer, work),
                VERTICES =&gt; <span class="k">self</span>._write_vertices(writer, work),
                EDGES =&gt; <span class="k">self</span>._write_edges(writer, work),
                ORDERED..REST =&gt; <span class="k">self</span>._write_ordered(writer, work),
                REST =&gt; <span class="k">self</span>._write_rest(writer, work),
                DEFINITIONS..INTERACTIONS =&gt; <span class="k">self</span>._write_list(writer, <span class="s">true</span>, work),
                INTERACTIONS =&gt; <span class="k">self</span>._write_interactions(writer, work),
                _ =&gt; <span class="k">return</span> <span class="k">self</span>._assemble(writer, work),
            };
            work -= spent.clamp(<span class="s">1</span>, work);
        }

        <span class="s">false</span>
    }

    <span class="c">/// Write the live entries of one objects or definitions list from the cursor slot; returns the slots examined.</span>
    <span class="k">fn</span> _write_list(&amp;<span class="k">self</span>, writer: &amp;<span class="k">mut</span> Checkpoint, definition: bool, work: usize) -&gt; usize {
        <span class="k">use</span> prost::Message;

        <span class="k">let</span> (objects, lookup, first, section) = <span class="k">if</span> definition {
            (&amp;<span class="k">self</span>.definitions, &amp;<span class="k">self</span>.definition_lookup, DEFINITIONS, <span class="s">5</span>)
        } <span class="k">else</span> {
            (&amp;<span class="k">self</span>.objects, &amp;<span class="k">self</span>.lookup, OBJECTS, <span class="s">1</span>)
        };
        <span class="k">let</span> phase = writer.phase - first;
        <span class="k">let</span> tag = TAGS.lists[phase];
        <span class="k">let</span> start = writer.cursor;
        <span class="k">let</span> buffer = &amp;<span class="k">mut</span> writer.sections[section];
        <span class="k">let</span> (end, total) = <span class="k">match</span> COLLECTIONS[phase].<span class="s">0</span> {
            &quot;<span class="s">components</span>&quot; =&gt; emit(&amp;objects.components, tag, start, work, buffer, |item| {
                item.to_proto().encode_to_vec()
            }),
            &quot;<span class="s">instances</span>&quot; =&gt; emit(&amp;objects.instances, tag, start, work, buffer, |item| {
                <span class="k">let</span> held = <span class="k">self</span>
                    .instance_lookup
                    .get(item.guid())
                    .filter(|_| !definition);

                held.unwrap_or(item).to_proto().encode_to_vec()
            }),
            name =&gt; emit_geometry(objects, lookup, name, tag, start, work, buffer),
        };
        writer.cursor = end;

        <span class="k">if</span> end &gt;= total {
            writer.cursor = <span class="s">0</span>;
            writer.phase += <span class="s">1</span>;
        }

        end - start
    }

    <span class="c">/// Write the live tree depth first from an explicit stack, a finished node appended to its parent; returns the children examined.</span>
    <span class="k">fn</span> _write_tree(&amp;<span class="k">self</span>, writer: &amp;<span class="k">mut</span> Checkpoint, work: usize) -&gt; usize {
        <span class="k">if</span> writer.cursor == <span class="s">0</span> {
            writer.cursor = <span class="s">1</span>;
            writer.sections[<span class="s">2</span>] = <span class="k">self</span>._tree_head();

            <span class="k">if</span> <span class="k">let</span> Some(root) = <span class="k">self</span>.tree.root() {
                writer.stack.push(Frame::new(root));
            }
        }

        <span class="k">let</span> <span class="k">mut</span> spent = <span class="s">0</span>;

        <span class="k">while</span> spent &lt; work {
            <span class="k">let</span> Some(frame) = writer.stack.last_mut() <span class="k">else</span> {
                <span class="k">break</span>;
            };
            <span class="k">let</span> child = frame.node.borrow().get_child(frame.next);
            frame.next += <span class="s">1</span>;
            spent += <span class="s">1</span>;

            <span class="k">if</span> <span class="k">let</span> Some(child) = child {
                <span class="k">if</span> !child.borrow().is_dead() {
                    writer.stack.push(Frame::new(child));
                }

                <span class="k">continue</span>;
            }

            <span class="k">let</span> Some(frame) = writer.stack.pop() <span class="k">else</span> {
                <span class="k">break</span>;
            };
            <span class="k">let</span> <span class="k">mut</span> chunks = frame.chunks;
            append(&amp;<span class="k">mut</span> chunks, node_tail(&amp;frame.node.borrow()));
            <span class="k">let</span> length = chunks.iter().map(Vec::len).sum();
            <span class="k">let</span> (tag, parent) = <span class="k">match</span> writer.stack.last_mut() {
                Some(frame) =&gt; (TAGS.children, &amp;<span class="k">mut</span> frame.chunks),
                None =&gt; (TAGS.root, &amp;<span class="k">mut</span> writer.tree),
            };
            append(parent, prefix(tag, length));

            <span class="k">for</span> chunk <span class="k">in</span> chunks {
                append(parent, chunk);
            }
        }

        <span class="k">if</span> writer.stack.is_empty() {
            writer.cursor = <span class="s">0</span>;
            writer.phase = VERTICES;
        }

        spent
    }

    <span class="c">/// The guid and name fields of the Tree message.</span>
    <span class="k">fn</span> _tree_head(&amp;<span class="k">self</span>) -&gt; Vec&lt;u8&gt; {
        <span class="k">use</span> prost::Message;

        <span class="k">let</span> guid = <span class="k">if</span> <span class="k">self</span>.tree.has_guid() {
            <span class="k">self</span>.tree.guid().to_string()
        } <span class="k">else</span> {
            String::new()
        };

        <span class="k">crate</span>::proto::Tree {
            guid,
            name: <span class="k">self</span>.tree.name.clone(),
            root: None,
        }
        .encode_to_vec()
    }

    <span class="c">/// Write the graph head, then the vertices in name order after the last key written, at most work per call; returns the vertices written.</span>
    <span class="k">fn</span> _write_vertices(&amp;<span class="k">self</span>, writer: &amp;<span class="k">mut</span> Checkpoint, work: usize) -&gt; usize {
        <span class="k">use</span> <span class="k">crate</span>::graph::vertex_to_proto;
        <span class="k">use</span> prost::Message;

        <span class="k">let</span> after = (writer.cursor &gt; <span class="s">0</span>).then_some(writer.key.as_str());
        <span class="k">let</span> buffer = &amp;<span class="k">mut</span> writer.sections[<span class="s">3</span>];
        <span class="k">let</span> <span class="k">mut</span> spent = <span class="s">0</span>;
        <span class="k">let</span> <span class="k">mut</span> last = None;

        <span class="k">if</span> writer.cursor == <span class="s">0</span> {
            <span class="k">let</span> guid = <span class="k">if</span> <span class="k">self</span>.graph.has_guid() {
                <span class="k">self</span>.graph.guid().to_string()
            } <span class="k">else</span> {
                String::new()
            };
            *buffer = <span class="k">crate</span>::proto::Graph {
                name: <span class="k">self</span>.graph.name.clone(),
                guid,
                ..Default::default()
            }
            .encode_to_vec();
        }

        <span class="k">for</span> (name, vertex) <span class="k">in</span> <span class="k">self</span>.graph.vertices_after(after).take(work) {
            <span class="k">crate</span>::proto::Graph {
                vertices: BTreeMap::from([(name.clone(), vertex_to_proto(vertex))]),
                ..Default::default()
            }
            .encode_raw(buffer);
            last = Some(name.clone());
            spent += <span class="s">1</span>;
        }

        <span class="k">if</span> <span class="k">let</span> Some(last) = last {
            writer.key = last;
            writer.cursor += <span class="s">1</span>;

            <span class="k">if</span> spent &gt;= work {
                <span class="k">return</span> spent;
            }
        }

        writer.key.clear();
        writer.cursor = <span class="s">0</span>;
        writer.phase = EDGES;

        spent
    }

    <span class="c">/// Write the graph edges in vertex order after the last key written, at most work entries per call, then the counts and defaults; returns the entries examined.</span>
    <span class="k">fn</span> _write_edges(&amp;<span class="k">self</span>, writer: &amp;<span class="k">mut</span> Checkpoint, work: usize) -&gt; usize {
        <span class="k">use</span> <span class="k">crate</span>::graph::edge_to_proto;
        <span class="k">use</span> prost::Message;

        <span class="k">let</span> start = <span class="k">if</span> writer.cursor &gt; <span class="s">0</span> {
            std::ops::Bound::Excluded(writer.key.as_str())
        } <span class="k">else</span> {
            std::ops::Bound::Unbounded
        };
        <span class="k">let</span> buffer = &amp;<span class="k">mut</span> writer.sections[<span class="s">3</span>];
        <span class="k">let</span> <span class="k">mut</span> spent = <span class="s">0</span>;
        <span class="k">let</span> <span class="k">mut</span> last = None;

        <span class="k">for</span> (u, neighbors) <span class="k">in</span> <span class="k">self</span>
            .graph
            .edges
            .range::&lt;str, _&gt;((start, std::ops::Bound::Unbounded))
        {
            <span class="k">if</span> spent &gt;= work {
                <span class="k">break</span>;
            }

            <span class="k">for</span> (v, edge) <span class="k">in</span> neighbors {
                <span class="k">if</span> u &lt;= v {
                    <span class="k">crate</span>::proto::Graph {
                        edges: vec![edge_to_proto(edge)],
                        ..Default::default()
                    }
                    .encode_raw(buffer);
                }
            }

            last = Some(u.clone());
            spent += neighbors.len().max(<span class="s">1</span>);
        }

        <span class="k">if</span> <span class="k">let</span> Some(last) = last {
            writer.key = last;
            writer.cursor += <span class="s">1</span>;

            <span class="k">if</span> spent &gt;= work {
                <span class="k">return</span> spent;
            }
        }

        <span class="k">crate</span>::proto::Graph {
            vertex_count: <span class="k">self</span>.graph.vertex_count,
            edge_count: <span class="k">self</span>.graph.edge_count,
            default_vertex_attributes: <span class="k">self</span>.graph.default_vertex_attributes.clone(),
            default_edge_attributes: <span class="k">self</span>.graph.default_edge_attributes.clone(),
            ..Default::default()
        }
        .encode_raw(buffer);
        writer.key.clear();
        writer.cursor = <span class="s">0</span>;
        writer.phase = ORDERED;

        spent
    }

    <span class="c">/// Write the non-identity xforms of the live objects of one order() list; returns the slots examined.</span>
    <span class="k">fn</span> _write_ordered(&amp;<span class="k">self</span>, writer: &amp;<span class="k">mut</span> Checkpoint, work: usize) -&gt; usize {
        <span class="k">let</span> start = writer.cursor;
        <span class="k">let</span> <span class="k">mut</span> end = start;
        <span class="k">let</span> <span class="k">mut</span> total = <span class="s">0</span>;

        <span class="k">if</span> <span class="k">let</span> Some(list) = list(&amp;<span class="k">self</span>.objects, COLLECTIONS[writer.phase - ORDERED].<span class="s">0</span>) {
            total = list.number_of_slots();
            end = total.min(start.saturating_add(work));

            <span class="k">for</span> slot <span class="k">in</span> start..end {
                <span class="k">if</span> list.is_dead(slot) {
                    <span class="k">continue</span>;
                }

                <span class="k">let</span> guid = list.key_at(slot);
                <span class="k">let</span> Some(xform) = <span class="k">self</span>.xforms.get(guid) <span class="k">else</span> {
                    <span class="k">continue</span>;
                };
                writer.hits += <span class="s">1</span>;

                <span class="k">if</span> !xform.is_identity() {
                    <span class="k">self</span>._write_xform(writer, guid.to_string(), xform);
                }
            }
        }

        writer.cursor = end;

        <span class="k">if</span> end &gt;= total {
            writer.cursor = <span class="s">0</span>;
            writer.phase += <span class="s">1</span>;
        }

        end - start
    }

    <span class="c">/// Write the non-identity xforms of guids outside order() in guid order, after a scan of xforms in slices of \`work\` that runs only when order() missed some; returns the entries examined or written.</span>
    <span class="k">fn</span> _write_rest(&amp;<span class="k">self</span>, writer: &amp;<span class="k">mut</span> Checkpoint, work: usize) -&gt; usize {
        <span class="k">if</span> writer.hits &lt; <span class="k">self</span>.xforms.len() {
            <span class="k">return</span> <span class="k">self</span>._scan_rest(writer, work);
        }

        <span class="k">let</span> start = <span class="k">if</span> writer.cursor &gt; <span class="s">0</span> {
            std::ops::Bound::Excluded(writer.key.as_str())
        } <span class="k">else</span> {
            std::ops::Bound::Unbounded
        };
        <span class="k">let</span> rest = std::mem::take(&amp;<span class="k">mut</span> writer.rest);
        <span class="k">let</span> <span class="k">mut</span> spent = <span class="s">0</span>;
        <span class="k">let</span> <span class="k">mut</span> last = None;

        <span class="k">for</span> guid <span class="k">in</span> rest
            .range::&lt;str, _&gt;((start, std::ops::Bound::Unbounded))
            .take(work)
        {
            <span class="k">self</span>._write_xform(writer, guid.clone(), &amp;<span class="k">self</span>.xforms[guid]);
            last = Some(guid.clone());
            spent += <span class="s">1</span>;
        }

        writer.rest = rest;

        <span class="k">if</span> <span class="k">let</span> (Some(last), <span class="s">true</span>) = (last, spent &gt;= work) {
            writer.key = last;
            writer.cursor += <span class="s">1</span>;

            <span class="k">return</span> spent;
        }

        writer.cursor = <span class="s">0</span>;
        writer.phase = <span class="k">if</span> <span class="k">self</span>.definition_lookup.is_empty() {
            INTERACTIONS
        } <span class="k">else</span> {
            writer.sections[<span class="s">5</span>] = objects_head(&amp;<span class="k">self</span>.definitions);
            DEFINITIONS
        };

        spent
    }

    <span class="c">/// Scan a slice of at most \`work\` xforms for non-identity ones whose guid order() misses, into the rest set; returns the entries scanned.</span>
    <span class="k">fn</span> _scan_rest(&amp;<span class="k">self</span>, writer: &amp;<span class="k">mut</span> Checkpoint, work: usize) -&gt; usize {
        <span class="k">let</span> start = writer.cursor;
        <span class="k">let</span> end = <span class="k">self</span>.xforms.len().min(start.saturating_add(work));

        <span class="k">for</span> (guid, xform) <span class="k">in</span> <span class="k">self</span>.xforms.iter().skip(start).take(end - start) {
            <span class="k">let</span> ordered = <span class="k">self</span>.lookup.get(guid).is_some_and(|geometry| {
                slot_of(&amp;<span class="k">self</span>.objects, collection_of(geometry).<span class="s">0</span>, guid).is_some()
            });

            <span class="k">if</span> !ordered &amp;&amp; !xform.is_identity() {
                writer.rest.insert(guid.clone());
            }
        }

        writer.cursor = end;

        <span class="k">if</span> end &gt;= <span class="k">self</span>.xforms.len() {
            writer.hits = <span class="k">self</span>.xforms.len();
            writer.cursor = <span class="s">0</span>;
        }

        end - start
    }

    <span class="c">/// Append one XformEntry to the xforms section.</span>
    <span class="k">fn</span> _write_xform(&amp;<span class="k">self</span>, writer: &amp;<span class="k">mut</span> Checkpoint, guid: String, xform: &amp;Xform) {
        <span class="k">use</span> prost::Message;

        <span class="k">crate</span>::proto::Session {
            xforms: vec![<span class="k">crate</span>::proto::XformEntry {
                guid,
                xform: Some(xform.to_proto()),
            }],
            ..Default::default()
        }
        .encode_raw(&amp;<span class="k">mut</span> writer.sections[<span class="s">4</span>]);
    }

    <span class="c">/// Write the interactions per edge guid, resuming after the last guid written; returns the entries written.</span>
    <span class="k">fn</span> _write_interactions(&amp;<span class="k">self</span>, writer: &amp;<span class="k">mut</span> Checkpoint, work: usize) -&gt; usize {
        <span class="k">use</span> prost::Message;

        <span class="k">let</span> start = <span class="k">if</span> writer.cursor &gt; <span class="s">0</span> {
            std::ops::Bound::Excluded(writer.key.as_str())
        } <span class="k">else</span> {
            std::ops::Bound::Unbounded
        };
        <span class="k">let</span> <span class="k">mut</span> spent = <span class="s">0</span>;
        <span class="k">let</span> <span class="k">mut</span> last = None;

        <span class="k">for</span> (guid, list) <span class="k">in</span> <span class="k">self</span>
            .interactions
            .range::&lt;str, _&gt;((start, std::ops::Bound::Unbounded))
            .take(work)
        {
            <span class="k">crate</span>::proto::Session {
                interactions: vec![<span class="k">crate</span>::proto::InteractionEntry {
                    guid: guid.clone(),
                    interactions: list.iter().map(|item| item.to_proto()).collect(),
                }],
                ..Default::default()
            }
            .encode_raw(&amp;<span class="k">mut</span> writer.sections[<span class="s">6</span>]);
            last = Some(guid.clone());
            spent += <span class="s">1</span>;
        }

        <span class="k">if</span> <span class="k">let</span> (Some(last), <span class="s">true</span>) = (last, spent &gt;= work) {
            writer.key = last;
            writer.cursor += <span class="s">1</span>;

            <span class="k">return</span> spent;
        }

        writer.phase = ASSEMBLY;
        writer.cursor = <span class="s">0</span>;

        spent
    }

    <span class="c">/// Join the sections into one Session message, copying at most \`work\` KiB; true once complete.</span>
    <span class="k">fn</span> _assemble(&amp;<span class="k">self</span>, writer: &amp;<span class="k">mut</span> Checkpoint, work: usize) -&gt; bool {
        <span class="k">if</span> writer.phase == ASSEMBLY {
            <span class="k">let</span> <span class="k">mut</span> pieces = Vec::new();

            <span class="k">for</span> (i, body) <span class="k">in</span> std::mem::take(&amp;<span class="k">mut</span> writer.sections).into_iter().enumerate() {
                <span class="k">let</span> tag = TAGS.sections[i];

                <span class="k">if</span> i == <span class="s">5</span> &amp;&amp; <span class="k">self</span>.definition_lookup.is_empty() {
                    <span class="k">continue</span>;
                }

                <span class="k">let</span> tree = <span class="k">if</span> i == <span class="s">2</span> {
                    std::mem::take(&amp;<span class="k">mut</span> writer.tree)
                } <span class="k">else</span> {
                    Vec::new()
                };

                <span class="k">if</span> tag &gt; <span class="s">0</span> {
                    <span class="k">let</span> length = body.len() + tree.iter().map(Vec::len).sum::&lt;usize&gt;();
                    pieces.push(prefix(tag, length));
                }

                pieces.push(body);
                pieces.extend(tree);
            }
            writer.out.reserve_exact(pieces.iter().map(Vec::len).sum());
            writer.sections = pieces;
            writer.phase += <span class="s">1</span>;
        }

        <span class="k">let</span> <span class="k">mut</span> budget = work.saturating_mul(<span class="s">1024</span>);
        <span class="k">let</span> <span class="k">mut</span> skip = writer.out.len();

        <span class="k">for</span> piece <span class="k">in</span> &amp;writer.sections {
            <span class="k">if</span> skip &gt;= piece.len() {
                skip -= piece.len();
                <span class="k">continue</span>;
            }

            <span class="k">let</span> end = piece.len().min(skip.saturating_add(budget));
            writer.out.extend_from_slice(&amp;piece[skip..end]);
            budget -= end - skip;
            skip = <span class="s">0</span>;

            <span class="k">if</span> budget == <span class="s">0</span> {
                <span class="k">break</span>;
            }
        }

        writer.out.len() == writer.sections.iter().map(Vec::len).sum::&lt;usize&gt;()
    }

    <span class="c">/// Flip a tomb dead: its slot and map entry, and for an object tomb its node, transform, vertex, edges and interactions; a guid a live twin owns keeps those with the twin; O(1 + d log V).</span>
    <span class="k">pub</span>(<span class="k">crate</span>) <span class="k">fn</span> _kill(&amp;<span class="k">mut</span> <span class="k">self</span>, tomb: &amp;Rc&lt;Tomb&gt;) {
        <span class="k">if</span> tomb.collection.is_empty() {
            <span class="k">return</span>;
        }

        <span class="k">let</span> slot = tomb.slot.get();
        <span class="k">let</span> collection = tomb.collection.as_str();
        <span class="k">self</span>.revision += <span class="s">1</span>;
        <span class="k">self</span>.bvh_cache_dirty = <span class="s">true</span>;

        <span class="k">if</span> tomb.definition {
            <span class="k">self</span>._kill_definition(tomb);

            <span class="k">return</span>;
        }

        <span class="k">let</span> Some(stored) = item_at(&amp;<span class="k">self</span>.objects, collection, slot) <span class="k">else</span> {
            <span class="k">return</span>;
        };
        <span class="k">let</span> guid = stored.guid();
        <span class="k">let</span> owner = !<span class="k">self</span>._twin(<span class="s">false</span>, collection, slot, guid)
            &amp;&amp; (<span class="k">self</span>._is_live(guid) || slot_of(&amp;<span class="k">self</span>.objects, collection, guid) == Some(slot));
        <span class="k">let</span> held = <span class="k">self</span>._take(guid);

        <span class="k">if</span> <span class="k">let</span> Some(held) = held {
            <span class="k">if</span> owner &amp;&amp; !same(&amp;held, &amp;stored) {
                store(&amp;<span class="k">mut</span> <span class="k">self</span>.objects, collection, slot, &amp;held);
            }

            <span class="k">if</span> !owner {
                <span class="k">self</span>._hold(guid, held);
            }
        }

        flag(&amp;<span class="k">mut</span> <span class="k">self</span>.objects, collection, slot, <span class="s">true</span>);

        <span class="k">let</span> Some(node) = &amp;tomb.node <span class="k">else</span> {
            <span class="k">return</span>;
        };
        <span class="k">let</span> parent = node.borrow().parent();
        node.borrow_mut().set_dead(<span class="s">true</span>);
        node.borrow_mut().set_tomb(tomb);

        <span class="k">if</span> <span class="k">let</span> Some((key, held)) = <span class="k">self</span>.node_lookup.remove_entry(guid) {
            <span class="k">if</span> !Rc::ptr_eq(&amp;held, node) {
                <span class="k">self</span>.node_lookup.insert(key, held);
            }
        }

        <span class="k">if</span> <span class="k">let</span> Some(parent) = parent {
            <span class="k">self</span>._queue(&amp;parent);
        }

        <span class="k">if</span> owner {
            <span class="k">self</span>._park(tomb, guid);
        }
    }

    <span class="c">/// Flip a tomb live again: the same slot and pointer, and for an object tomb the same node, transform, vertex, edges and interactions; a guid a live twin owns stays dead; O(1 + d log V).</span>
    <span class="k">pub</span>(<span class="k">crate</span>) <span class="k">fn</span> _revive(&amp;<span class="k">mut</span> <span class="k">self</span>, tomb: &amp;Rc&lt;Tomb&gt;) {
        <span class="k">if</span> tomb.collection.is_empty() {
            <span class="k">return</span>;
        }

        <span class="k">let</span> slot = tomb.slot.get();
        <span class="k">let</span> collection = tomb.collection.as_str();
        <span class="k">self</span>.revision += <span class="s">1</span>;
        <span class="k">self</span>.bvh_cache_dirty = <span class="s">true</span>;

        <span class="k">if</span> tomb.definition {
            <span class="k">self</span>._revive_definition(tomb);

            <span class="k">return</span>;
        }

        <span class="k">let</span> Some(item) = item_at(&amp;<span class="k">self</span>.objects, collection, slot) <span class="k">else</span> {
            <span class="k">return</span>;
        };
        <span class="k">let</span> guid = item.guid().to_string();

        <span class="k">if</span> <span class="k">self</span>._twin(<span class="s">false</span>, collection, slot, &amp;guid) {
            <span class="k">return</span>;
        }

        flag(&amp;<span class="k">mut</span> <span class="k">self</span>.objects, collection, slot, <span class="s">false</span>);
        <span class="k">self</span>._hold(&amp;guid, item.clone());

        <span class="k">let</span> Some(node) = &amp;tomb.node <span class="k">else</span> {
            <span class="k">self</span>._label(&amp;guid, &amp;format!(&quot;{}<span class="s">_</span>{}&quot;, prefix_of(collection), item.name()));

            <span class="k">return</span>;
        };
        node.borrow_mut().set_dead(<span class="s">false</span>);

        <span class="k">if</span> node.borrow().parent().is_some() {
            <span class="k">self</span>.node_lookup.insert(guid.clone(), Rc::clone(node));
        }

        <span class="k">self</span>._unpark(tomb, &amp;guid);
    }

    <span class="c">/// Flip a definition tomb dead: its slot, and its map entry when it owns the guid; O(1).</span>
    <span class="k">fn</span> _kill_definition(&amp;<span class="k">mut</span> <span class="k">self</span>, tomb: &amp;Rc&lt;Tomb&gt;) {
        <span class="k">let</span> slot = tomb.slot.get();
        <span class="k">let</span> collection = tomb.collection.as_str();
        <span class="k">let</span> Some(stored) = item_at(&amp;<span class="k">self</span>.definitions, collection, slot) <span class="k">else</span> {
            <span class="k">return</span>;
        };
        <span class="k">let</span> guid = stored.guid().to_string();
        <span class="k">let</span> owner = !<span class="k">self</span>._twin(<span class="s">true</span>, collection, slot, &amp;guid);

        <span class="k">if</span> <span class="k">let</span> Some(held) = <span class="k">self</span>.definition_lookup.get(&amp;guid) {
            <span class="k">let</span> held = Item::Geometry(held.clone());

            <span class="k">if</span> owner &amp;&amp; !same(&amp;held, &amp;stored) {
                store(&amp;<span class="k">mut</span> <span class="k">self</span>.definitions, collection, slot, &amp;held);
            }
        }

        flag(&amp;<span class="k">mut</span> <span class="k">self</span>.definitions, collection, slot, <span class="s">true</span>);

        <span class="k">if</span> owner {
            <span class="k">self</span>.definition_lookup.remove(&amp;guid);
        }
    }

    <span class="c">/// Flip a definition tomb live again, unless a live twin owns its guid; O(1).</span>
    <span class="k">fn</span> _revive_definition(&amp;<span class="k">mut</span> <span class="k">self</span>, tomb: &amp;Rc&lt;Tomb&gt;) {
        <span class="k">let</span> slot = tomb.slot.get();
        <span class="k">let</span> collection = tomb.collection.as_str();
        <span class="k">let</span> Some(Item::Geometry(geometry)) = item_at(&amp;<span class="k">self</span>.definitions, collection, slot) <span class="k">else</span> {
            <span class="k">return</span>;
        };
        <span class="k">let</span> guid = geometry.guid().to_string();

        <span class="k">if</span> <span class="k">self</span>._twin(<span class="s">true</span>, collection, slot, &amp;guid) {
            <span class="k">return</span>;
        }

        flag(&amp;<span class="k">mut</span> <span class="k">self</span>.definitions, collection, slot, <span class="s">false</span>);
        <span class="k">self</span>.definition_lookup.insert(guid, geometry);
    }

    <span class="c">/// Move the transform, graph vertex, incident edges and their interactions of guid into its tomb; O(d log V).</span>
    <span class="k">fn</span> _park(&amp;<span class="k">mut</span> <span class="k">self</span>, tomb: &amp;Rc&lt;Tomb&gt;, guid: &amp;str) {
        *tomb.xform.borrow_mut() = <span class="k">self</span>.xforms.remove(guid);

        <span class="k">let</span> Some((vertex, edges)) = <span class="k">self</span>.graph.take_node(guid) <span class="k">else</span> {
            <span class="k">return</span>;
        };

        <span class="k">for</span> edge <span class="k">in</span> &amp;edges {
            <span class="k">if</span> !edge.has_guid() {
                <span class="k">continue</span>;
            }

            <span class="k">if</span> <span class="k">let</span> Some(list) = <span class="k">self</span>.interactions.remove(edge.guid()) {
                tomb.interactions
                    .borrow_mut()
                    .insert(edge.guid().to_string(), list);
            }
        }

        *tomb.vertex.borrow_mut() = Some(vertex);
        *tomb.edges.borrow_mut() = edges;
    }

    <span class="c">/// Move a tomb's transform, vertex and edges back, with the interactions of every edge that returns; O(d log V).</span>
    <span class="k">fn</span> _unpark(&amp;<span class="k">mut</span> <span class="k">self</span>, tomb: &amp;Rc&lt;Tomb&gt;, guid: &amp;str) {
        <span class="k">if</span> <span class="k">let</span> Some(xform) = tomb.xform.borrow_mut().take() {
            <span class="k">self</span>.xforms.insert(guid.to_string(), xform);
        }

        <span class="k">let</span> Some(vertex) = tomb.vertex.borrow_mut().take() <span class="k">else</span> {
            <span class="k">return</span>;
        };
        <span class="k">let</span> edges = std::mem::take(&amp;<span class="k">mut</span> *tomb.edges.borrow_mut());
        <span class="k">let</span> <span class="k">mut</span> ids: Vec&lt;(String, String)&gt; = Vec::with_capacity(edges.len());

        <span class="k">for</span> edge <span class="k">in</span> &amp;edges {
            <span class="k">if</span> edge.has_guid() {
                ids.push((edge.other_vertex(guid), edge.guid().to_string()));
            }
        }

        <span class="k">self</span>.graph.put_node(vertex, edges);

        <span class="k">for</span> (other, id) <span class="k">in</span> ids {
            <span class="k">let</span> Some(neighbours) = <span class="k">self</span>.graph.edges.get(guid) <span class="k">else</span> {
                <span class="k">continue</span>;
            };
            <span class="k">let</span> Some(back) = neighbours.get(&amp;other) <span class="k">else</span> {
                <span class="k">continue</span>;
            };

            <span class="k">if</span> !back.has_guid() || back.guid() != id {
                <span class="k">continue</span>;
            }

            <span class="k">if</span> <span class="k">let</span> Some(list) = tomb.interactions.borrow_mut().remove(&amp;id) {
                <span class="k">self</span>.interactions.insert(id, list);
            }
        }
    }

    <span class="c">/// Store obj under guid in the slot and map of the recorded entry, relabelling an object's vertex; a guid now live on the other side, or on the same side as another entry, is left alone; O(1).</span>
    <span class="k">pub</span>(<span class="k">crate</span>) <span class="k">fn</span> _swap(&amp;<span class="k">mut</span> <span class="k">self</span>, guid: &amp;str, obj: Item, entry: &amp;Entry) {
        <span class="k">let</span> (collection, prefix) = collection_for(&amp;obj);

        <span class="k">match</span> entry {
            Entry::Definition(tomb) =&gt; {
                <span class="k">let</span> Item::Geometry(geometry) = &amp;obj <span class="k">else</span> {
                    <span class="k">return</span>;
                };
                <span class="k">let</span> slot = tomb.slot.get();

                <span class="k">if</span> <span class="k">self</span>._is_live(guid) || slot_of(&amp;<span class="k">self</span>.definitions, collection, guid) != Some(slot)
                {
                    <span class="k">return</span>;
                }

                store(&amp;<span class="k">mut</span> <span class="k">self</span>.definitions, collection, slot, &amp;obj);
                <span class="k">self</span>.definition_lookup
                    .insert(guid.to_string(), geometry.clone());
            }

            Entry::Object(node) =&gt; {
                <span class="k">if</span> <span class="k">self</span>.definition_lookup.contains_key(guid) || !<span class="k">self</span>._owns(guid, node.as_ref()) {
                    <span class="k">return</span>;
                }

                <span class="k">let</span> Some(slot) = slot_of(&amp;<span class="k">self</span>.objects, collection, guid) <span class="k">else</span> {
                    <span class="k">return</span>;
                };
                store(&amp;<span class="k">mut</span> <span class="k">self</span>.objects, collection, slot, &amp;obj);
                <span class="k">self</span>._label(guid, &amp;format!(&quot;{<span class="s">prefix</span>}<span class="s">_</span>{}&quot;, obj.name()));
                <span class="k">self</span>._hold(guid, obj);
            }
        }

        <span class="k">self</span>.revision += <span class="s">1</span>;
        <span class="k">self</span>.bvh_cache_dirty = <span class="s">true</span>;
    }

    <span class="c">/// Rebuild every index from the tables in O(n + N): the maps win over the slots, map-only and slot-only entries are adopted, a non-identity instance xform folds into xforms, node_lookup is refilled from the live tree.</span>
    <span class="k">pub</span> <span class="k">fn</span> reindex(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        repoint(&amp;<span class="k">mut</span> <span class="k">self</span>.objects, &amp;<span class="k">self</span>.lookup);
        adopt(&amp;<span class="k">mut</span> <span class="k">self</span>.objects, &amp;<span class="k">mut</span> <span class="k">self</span>.lookup);
        repoint(&amp;<span class="k">mut</span> <span class="k">self</span>.definitions, &amp;<span class="k">self</span>.definition_lookup);
        adopt(&amp;<span class="k">mut</span> <span class="k">self</span>.definitions, &amp;<span class="k">mut</span> <span class="k">self</span>.definition_lookup);
        <span class="k">self</span>._reindex_components();
        <span class="k">self</span>._reindex_instances();
        <span class="k">self</span>.node_lookup.clear();

        <span class="k">for</span> node <span class="k">in</span> <span class="k">self</span>.tree.nodes() {
            <span class="k">let</span> guid = node.borrow().name.clone();

            <span class="k">if</span> <span class="k">self</span>._is_live(&amp;guid) &amp;&amp; !<span class="k">self</span>.node_lookup.contains_key(&amp;guid) {
                <span class="k">self</span>.node_lookup.insert(guid, node);
            }
        }

        <span class="k">self</span>.indexed = <span class="k">self</span>.tree.root().map(|root| Rc::downgrade(&amp;root));
    }

    <span class="c">/// Adopt slot-only components into component_lookup and map-only ones into the slots, sorted by guid; the map wins a shared guid.</span>
    <span class="k">fn</span> _reindex_components(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">for</span> slot <span class="k">in</span> <span class="s">0</span>..<span class="k">self</span>.objects.components.number_of_slots() {
            <span class="k">if</span> <span class="k">self</span>.objects.components.is_dead(slot) {
                <span class="k">continue</span>;
            }

            <span class="k">let</span> guid = <span class="k">self</span>.objects.components.get_item(slot).guid.clone();

            <span class="k">match</span> <span class="k">self</span>.component_lookup.get(&amp;guid) {
                Some(component) =&gt; {
                    <span class="k">let</span> component = component.clone();
                    <span class="k">self</span>.objects.components.set_item(slot, component);
                }

                None =&gt; {
                    <span class="k">let</span> component = <span class="k">self</span>.objects.components.get_item(slot).clone();
                    <span class="k">self</span>.component_lookup.insert(guid, component);
                }
            }
        }

        <span class="k">let</span> <span class="k">mut</span> components: Vec&lt;&amp;Component&gt; = Vec::new();

        <span class="k">for</span> (guid, component) <span class="k">in</span> &amp;<span class="k">self</span>.component_lookup {
            <span class="k">if</span> <span class="k">self</span>.objects.components.get_slot(guid).is_none() {
                components.push(component);
            }
        }

        components.sort_by(|a, b| a.guid.cmp(&amp;b.guid));

        <span class="k">for</span> component <span class="k">in</span> components {
            <span class="k">self</span>.objects.components.push(component.clone());
        }
    }

    <span class="c">/// Adopt map-only instances into the slots, sorted by guid, the map winning a shared guid; a non-identity instance xform folds into xforms.</span>
    <span class="k">fn</span> _reindex_instances(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">let</span> <span class="k">mut</span> instances: Vec&lt;&amp;Rc&lt;InstanceRef&gt;&gt; = Vec::new();

        <span class="k">for</span> (guid, instance) <span class="k">in</span> &amp;<span class="k">self</span>.instance_lookup {
            <span class="k">if</span> <span class="k">self</span>.objects.instances.get_slot(guid).is_none() {
                instances.push(instance);
            }
        }

        instances.sort_by(|a, b| a.guid().cmp(b.guid()));

        <span class="k">for</span> instance <span class="k">in</span> instances {
            <span class="k">self</span>.objects.instances.push(Rc::clone(instance));
        }

        <span class="k">for</span> slot <span class="k">in</span> <span class="s">0</span>..<span class="k">self</span>.objects.instances.number_of_slots() {
            <span class="k">if</span> <span class="k">self</span>.objects.instances.is_dead(slot) {
                <span class="k">continue</span>;
            }

            <span class="k">let</span> <span class="k">mut</span> instance = Rc::clone(<span class="k">self</span>.objects.instances.get_item(slot));
            <span class="k">let</span> guid = instance.guid().to_string();

            <span class="k">if</span> <span class="k">let</span> Some(truth) = <span class="k">self</span>.instance_lookup.get(&amp;guid) {
                instance = Rc::clone(truth);
            }

            <span class="k">if</span> !instance.xform.is_identity() {
                <span class="k">let</span> folded = &amp;<span class="k">self</span>.xform(&amp;guid) * &amp;instance.xform;
                <span class="k">self</span>.xforms.insert(guid.clone(), folded);
                Rc::make_mut(&amp;<span class="k">mut</span> instance).xform = Xform::identity();
            }

            <span class="k">self</span>.objects.instances.set_item(slot, Rc::clone(&amp;instance));
            <span class="k">self</span>.instance_lookup.insert(guid, instance);
        }
    }

    <span class="c">/// Apply the before (back) or after state of a tree record: name, colour, liveness, and for a move the swap of node and ghost.</span>
    <span class="k">pub</span>(<span class="k">crate</span>) <span class="k">fn</span> _tree(&amp;<span class="k">mut</span> <span class="k">self</span>, op: &amp;TreeOp, back: bool) {
        <span class="k">let</span> (name, color, dead) = <span class="k">if</span> back {
            (&amp;op.name_before, &amp;op.color_before, op.dead_before)
        } <span class="k">else</span> {
            (&amp;op.name_after, &amp;op.color_after, op.dead_after)
        };
        <span class="k">let</span> was = op.node.borrow().is_dead();
        <span class="k">let</span> live = <span class="k">self</span>._is_live(name);

        <span class="k">if</span> <span class="k">let</span> Some(ghost) = &amp;op.ghost {
            <span class="k">let</span> from = op.node.borrow().parent();
            TreeNode::swap(&amp;op.node, ghost);
            ghost.borrow_mut().set_tomb(&amp;op.tomb);

            <span class="k">if</span> <span class="k">let</span> Some(from) = from {
                <span class="k">self</span>._queue(&amp;from);
            }
        }

        <span class="k">let</span> parent = op.node.borrow().parent();
        op.node.borrow_mut().name = name.clone();
        op.node.borrow_mut().color = color.clone();
        op.node.borrow_mut().set_dead(dead);
        <span class="k">self</span>.revision += <span class="s">1</span>;

        <span class="k">if</span> dead &amp;&amp; !was {
            op.node.borrow_mut().set_tomb(&amp;op.tomb);

            <span class="k">if</span> <span class="k">let</span> Some(parent) = parent {
                <span class="k">self</span>._queue(&amp;parent);
            }
        }

        <span class="k">if</span> dead &amp;&amp; !was &amp;&amp; !live {
            *op.tomb.xform.borrow_mut() = <span class="k">self</span>.xforms.remove(name);
        }

        <span class="k">if</span> was &amp;&amp; !dead &amp;&amp; !live {
            <span class="k">if</span> <span class="k">let</span> Some(xform) = op.tomb.xform.borrow_mut().take() {
                <span class="k">self</span>.xforms.insert(name.clone(), xform);
            }
        }

        <span class="k">if</span> !live {
            <span class="k">return</span>;
        }

        <span class="k">if</span> !dead {
            <span class="k">self</span>.node_lookup.insert(name.clone(), Rc::clone(&amp;op.node));
        } <span class="k">else</span> <span class="k">if</span> <span class="k">self</span>
            .node_lookup
            .get(name)
            .is_some_and(|held| Rc::ptr_eq(held, &amp;op.node))
        {
            <span class="k">self</span>.node_lookup.remove(name);
        }
    }

    <span class="c">/// Set or drops (None) the local transform under guid, unrecorded; a guid whose entry is not the recorded node is left alone.</span>
    <span class="k">pub</span>(<span class="k">crate</span>) <span class="k">fn</span> _place(
        &amp;<span class="k">mut</span> <span class="k">self</span>,
        guid: &amp;str,
        xform: Option&lt;&amp;Xform&gt;,
        node: Option&lt;&amp;Rc&lt;RefCell&lt;TreeNode&gt;&gt;&gt;,
    ) {
        <span class="k">if</span> !<span class="k">self</span>._owns(guid, node) {
            <span class="k">return</span>;
        }

        <span class="k">match</span> xform {
            Some(xform) =&gt; {
                <span class="k">self</span>.xforms.insert(guid.to_string(), xform.clone());
            }

            None =&gt; {
                <span class="k">self</span>.xforms.remove(guid);
            }
        }

        <span class="k">self</span>.bvh_cache_dirty = <span class="s">true</span>;
        <span class="k">self</span>.revision += <span class="s">1</span>;
    }

    <span class="c">/// The xforms in canonical order() sequence, identity entries omitted, the exact sequence jsondump and pb_dumps write.</span>
    <span class="k">fn</span> _xforms_ordered(&amp;<span class="k">self</span>) -&gt; Vec&lt;(String, &amp;Xform)&gt; {
        <span class="k">let</span> <span class="k">mut</span> ordered: Vec&lt;(String, &amp;Xform)&gt; = Vec::new();
        <span class="k">let</span> <span class="k">mut</span> rest: BTreeMap&lt;String, &amp;Xform&gt; = BTreeMap::new();

        <span class="k">for</span> (obj_guid, obj_xform) <span class="k">in</span> &amp;<span class="k">self</span>.xforms {
            <span class="k">if</span> !obj_xform.is_identity() {
                rest.insert(obj_guid.clone(), obj_xform);
            }
        }

        <span class="k">for</span> obj_guid <span class="k">in</span> <span class="k">self</span>.order() {
            <span class="k">let</span> Some(obj_xform) = rest.remove(&amp;obj_guid) <span class="k">else</span> {
                <span class="k">continue</span>;
            };
            ordered.push((obj_guid, obj_xform));
        }

        <span class="k">for</span> (obj_guid, obj_xform) <span class="k">in</span> rest {
            ordered.push((obj_guid, obj_xform));
        }

        ordered
    }

    <span class="c">/// World bounding box of every object in order() sequence, then of every instance, with the guid of each.</span>
    <span class="k">fn</span> _compute_boxes(&amp;<span class="k">self</span>, guids: &amp;<span class="k">mut</span> Vec&lt;String&gt;) -&gt; Vec&lt;OBB&gt; {
        guids.clear();
        <span class="k">let</span> <span class="k">mut</span> boxes: Vec&lt;OBB&gt; = Vec::with_capacity(<span class="k">self</span>.lookup.len());
        <span class="k">let</span> world = <span class="k">self</span>.world_xforms();

        <span class="k">for</span> guid <span class="k">in</span> <span class="k">self</span>.order() {
            <span class="k">let</span> Some(geometry) = <span class="k">self</span>.lookup.get(&amp;guid) <span class="k">else</span> {
                <span class="k">continue</span>;
            };
            <span class="k">let</span> placement = world.get(&amp;guid).cloned().unwrap_or_else(Xform::identity);
            boxes.push(<span class="k">Self</span>::compute_bounding_box(geometry, &amp;placement));
            guids.push(guid);
        }

        <span class="k">let</span> <span class="k">mut</span> local: HashMap&lt;String, OBB&gt; = HashMap::new();

        <span class="k">for</span> instance <span class="k">in</span> &amp;<span class="k">self</span>.objects.instances {
            <span class="k">let</span> Some(definition) = <span class="k">self</span>.definition_lookup.get(&amp;instance.definition_guid) <span class="k">else</span> {
                <span class="k">continue</span>;
            };
            <span class="k">let</span> <span class="k">mut</span> placed = local
                .entry(instance.definition_guid.clone())
                .or_insert_with(|| <span class="k">Self</span>::compute_bounding_box(definition, &amp;Xform::identity()))
                .clone();

            <span class="k">if</span> <span class="k">let</span> Some(xform) = world.get(instance.guid()) {
                placed.transform(xform);
            }

            boxes.push(placed);
            guids.push(instance.guid().to_string());
        }

        boxes
    }

    <span class="c">/// Rebuild the cached SpatialBVH for ray casting.</span>
    <span class="k">fn</span> _rebuild_ray_bvh_cache(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">let</span> <span class="k">mut</span> guids = std::mem::take(&amp;<span class="k">mut</span> <span class="k">self</span>.cached_guids);
        <span class="k">self</span>.cached_boxes = <span class="k">self</span>._compute_boxes(&amp;<span class="k">mut</span> guids);
        <span class="k">self</span>.cached_guids = guids;

        <span class="k">if</span> <span class="k">self</span>.cached_boxes.is_empty() {
            <span class="k">self</span>.cached_ray_bvh = None;
        } <span class="k">else</span> {
            <span class="k">let</span> world_size = SpatialBVH::compute_world_size(&amp;<span class="k">self</span>.cached_boxes);
            <span class="k">self</span>.cached_ray_bvh = Some(SpatialBVH::from_boxes(&amp;<span class="k">self</span>.cached_boxes, world_size));
        }
    }

    <span class="c">/// Test ray intersection with a specific geometry object, returning the world hit.</span>
    <span class="k">fn</span> _ray_intersect_geometry(
        &amp;<span class="k">self</span>,
        ray: &amp;Line,
        geometry: &amp;Geometry,
        tolerance: f64,
        placement: &amp;Xform,
    ) -&gt; Option&lt;Point&gt; {
        <span class="k">match</span> geometry {
            Geometry::Point(point) =&gt; ray_point(ray, point, tolerance),
            Geometry::Line(line) =&gt; line_line(ray, line, tolerance),
            Geometry::Plane(plane) =&gt; line_plane(ray, plane, <span class="s">true</span>),
            Geometry::Polyline(polyline) =&gt; ray_polyline(ray, polyline, tolerance),
            Geometry::PointCloud(pointcloud) =&gt; ray_pointcloud(ray, pointcloud, tolerance),
            Geometry::Mesh(mesh) =&gt; ray_mesh(ray, mesh, tolerance, placement),
            Geometry::OBB(bbox) =&gt; {
                <span class="k">let</span> hits = ray_box(ray, bbox, <span class="s">0</span>.<span class="s">0</span>, <span class="s">1</span>.<span class="s">0</span>)?;

                hits.first().cloned()
            }

            _ =&gt; None,
        }
    }
}

<span class="k">impl</span> fmt::Display <span class="k">for</span> Session {
    <span class="c">/// Write the session block to a formatter.</span>
    <span class="k">fn</span> fmt(&amp;<span class="k">self</span>, f: &amp;<span class="k">mut</span> fmt::Formatter&lt;'_&gt;) -&gt; fmt::Result {
        write!(f, &quot;{}&quot;, <span class="k">self</span>.str())
    }
}

<span class="k">impl</span> fmt::Debug <span class="k">for</span> Session {
    <span class="c">/// Write the one-line session string to a formatter.</span>
    <span class="k">fn</span> fmt(&amp;<span class="k">self</span>, f: &amp;<span class="k">mut</span> fmt::Formatter&lt;'_&gt;) -&gt; fmt::Result {
        write!(f, &quot;{}&quot;, <span class="k">self</span>.repr())
    }
}</code></pre></div>
`,toc:[]};export{s as default};
