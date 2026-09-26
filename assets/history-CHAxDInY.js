const s={title:"session_rust/src/history.rs",html:`<h1 id="session_rustsrchistoryrs">session_rust/src/history.rs<a class="anchor" href="#/course/kernel/history#session_rustsrchistoryrs" aria-label="Link to this section">#</a></h1>
<div class="code"><button class="copy" type="button" aria-label="Copy code">Copy</button><pre><code><span class="k">use</span> <span class="k">crate</span>::color::Color;
<span class="k">use</span> <span class="k">crate</span>::graph::Edge;
<span class="k">use</span> <span class="k">crate</span>::graph::Vertex;
<span class="k">use</span> <span class="k">crate</span>::interaction::Interaction;
<span class="k">use</span> <span class="k">crate</span>::session::Geometry;
<span class="k">use</span> <span class="k">crate</span>::session::Item;
<span class="k">use</span> <span class="k">crate</span>::session::Session;
<span class="k">use</span> <span class="k">crate</span>::tree::TreeNode;
<span class="k">use</span> <span class="k">crate</span>::xform::Xform;
<span class="k">use</span> <span class="k">crate</span>::BRep;
<span class="k">use</span> <span class="k">crate</span>::Mesh;
<span class="k">use</span> <span class="k">crate</span>::NurbsCurve;
<span class="k">use</span> <span class="k">crate</span>::NurbsSurface;
<span class="k">use</span> std::cell::Cell;
<span class="k">use</span> std::cell::RefCell;
<span class="k">use</span> std::collections::BTreeMap;
<span class="k">use</span> std::fmt;
<span class="k">use</span> std::rc::Rc;

<span class="k">pub</span> <span class="k">const</span> CAPACITY: usize = <span class="s">64</span>; <span class="c">// Committed transactions kept; past it the oldest is dropped.</span>
<span class="k">pub</span> <span class="k">const</span> BUDGET: usize = <span class="s">256</span> &lt;&lt; <span class="s">20</span>; <span class="c">// Bytes the stacks may pin; past it the oldest is dropped.</span>
<span class="k">pub</span> <span class="k">const</span> RECORD: usize = <span class="s">256</span>; <span class="c">// Bytes one record costs on top of what it pins.</span>

<span class="c">/// A deep copy of geometry that keeps its guid and type, element feature guids included.</span>
<span class="k">pub</span> <span class="k">fn</span> clone(obj: &amp;Geometry) -&gt; Geometry {
    <span class="k">match</span> obj {
        Geometry::OBB(g) =&gt; Geometry::OBB(Rc::new((**g).clone())),
        Geometry::BRep(g) =&gt; Geometry::BRep(Rc::new((**g).clone())),
        Geometry::Element(g) =&gt; Geometry::Element(Rc::new((**g).clone())),
        Geometry::Line(g) =&gt; Geometry::Line(Rc::new((**g).clone())),
        Geometry::Mesh(g) =&gt; Geometry::Mesh(Rc::new((**g).clone())),
        Geometry::NurbsCurve(g) =&gt; Geometry::NurbsCurve(Rc::new((**g).clone())),
        Geometry::NurbsSurface(g) =&gt; Geometry::NurbsSurface(Rc::new((**g).clone())),
        Geometry::Plane(g) =&gt; Geometry::Plane(Rc::new((**g).clone())),
        Geometry::Point(g) =&gt; Geometry::Point(Rc::new((**g).clone())),
        Geometry::PointCloud(g) =&gt; Geometry::PointCloud(Rc::new((**g).clone())),
        Geometry::Polyline(g) =&gt; Geometry::Polyline(Rc::new((**g).clone())),
    }
}

<span class="c">/// Bytes a mesh pins, from its counts: a vertex owns its halfedge map, a face its vertex list (measured, triangle caches not counted).</span>
<span class="k">fn</span> mesh_weight(mesh: &amp;Mesh) -&gt; usize {
    <span class="s">128</span> + <span class="s">768</span> * mesh.number_of_vertices() + <span class="s">192</span> * mesh.number_of_faces()
}

<span class="c">/// Bytes a curve pins, from its control point and knot counts.</span>
<span class="k">fn</span> curve_weight(curve: &amp;NurbsCurve) -&gt; usize {
    <span class="s">256</span> + <span class="s">32</span> * curve.cv_count() + <span class="s">8</span> * curve.m_nurbsknot.len()
}

<span class="c">/// Bytes a surface pins, from its control point and knot counts.</span>
<span class="k">fn</span> surface_weight(surface: &amp;NurbsSurface) -&gt; usize {
    <span class="s">1536</span> + <span class="s">32</span> * surface.cv_count_total()
        + <span class="s">8</span> * (surface.m_nurbsknot[<span class="s">0</span>].len() + surface.m_nurbsknot[<span class="s">1</span>].len())
}

<span class="c">/// Bytes a brep pins: every surface and curve of its pools, plus its tables.</span>
<span class="k">fn</span> brep_weight(brep: &amp;BRep) -&gt; usize {
    <span class="k">let</span> <span class="k">mut</span> bytes =
        <span class="s">512</span> + <span class="s">24</span> * brep.m_vertices.len() + <span class="s">64</span> * (brep.m_edges.len() + brep.m_faces.len());

    <span class="k">for</span> surface <span class="k">in</span> &amp;brep.m_surfaces {
        bytes += surface_weight(surface);
    }

    <span class="k">for</span> curve <span class="k">in</span> &amp;brep.m_curves_3d {
        bytes += curve_weight(curve);
    }

    <span class="k">for</span> curve <span class="k">in</span> &amp;brep.m_curves_2d {
        bytes += curve_weight(curve);
    }

    bytes
}

<span class="c">/// An estimate of the bytes an item pins while a record holds it, from its container lengths.</span>
<span class="k">pub</span> <span class="k">fn</span> weight(item: &amp;Item) -&gt; usize {
    <span class="k">match</span> item {
        Item::Geometry(Geometry::Point(_)) =&gt; <span class="s">64</span>,
        Item::Geometry(Geometry::Line(_)) =&gt; <span class="s">96</span>,
        Item::Geometry(Geometry::Plane(_)) =&gt; <span class="s">160</span>,
        Item::Geometry(Geometry::OBB(_)) =&gt; <span class="s">192</span>,
        Item::Geometry(Geometry::Polyline(g)) =&gt; <span class="s">64</span> + <span class="s">24</span> * g.point_count(),
        Item::Geometry(Geometry::PointCloud(g)) =&gt; {
            <span class="s">64</span> + <span class="s">24</span> * g.point_count() + <span class="s">24</span> * g.normal_count() + <span class="s">16</span> * g.color_count()
        }
        Item::Geometry(Geometry::Mesh(g)) =&gt; mesh_weight(g),
        Item::Geometry(Geometry::NurbsCurve(g)) =&gt; curve_weight(g),
        Item::Geometry(Geometry::NurbsSurface(g)) =&gt; surface_weight(g),
        Item::Geometry(Geometry::BRep(g)) =&gt; brep_weight(g),
        Item::Geometry(Geometry::Element(g)) =&gt; {
            <span class="k">let</span> geometry = <span class="k">match</span> g.geometry() {
                <span class="k">crate</span>::element::ElementGeometry::Mesh(mesh) =&gt; mesh_weight(mesh),
                <span class="k">crate</span>::element::ElementGeometry::BRep(brep) =&gt; brep_weight(brep),
                <span class="k">crate</span>::element::ElementGeometry::None =&gt; <span class="s">0</span>,
            };

            <span class="s">256</span> + geometry + <span class="s">128</span> * g.features.len()
        }
        Item::InstanceRef(i) =&gt; <span class="s">256</span> + <span class="s">128</span> * i.features.len(),
        Item::Component(_) =&gt; <span class="s">128</span>,
    }
}

<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">// Records</span>
<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">/// One dead or revivable entity: where it lives and what it parked while dead; slots and nodes pin it weakly, records strongly.</span>
#[derive(Debug)]
<span class="k">pub</span> <span class="k">struct</span> Tomb {
    <span class="k">pub</span> collection: String, <span class="c">// The Objects list of its slot, &quot;&quot; for a node-only tomb.</span>
    <span class="k">pub</span> definition: bool,   <span class="c">// Whether the slot is in Session::definitions.</span>
    <span class="k">pub</span> slot: Cell&lt;usize&gt;,  <span class="c">// Its raw slot, moved by compaction.</span>
    <span class="k">pub</span> node: Option&lt;Rc&lt;RefCell&lt;TreeNode&gt;&gt;&gt;, <span class="c">// Its tree node, None for a slot-only tomb.</span>
    <span class="k">pub</span> vertex: RefCell&lt;Option&lt;Vertex&gt;&gt;, <span class="c">// Its graph vertex while dead.</span>
    <span class="k">pub</span> edges: RefCell&lt;Vec&lt;Edge&gt;&gt;, <span class="c">// Its incident edges while dead.</span>
    <span class="k">pub</span> xform: RefCell&lt;Option&lt;Xform&gt;&gt;, <span class="c">// Its local transform while dead.</span>
    <span class="k">pub</span> interactions: RefCell&lt;BTreeMap&lt;String, Vec&lt;Box&lt;<span class="k">dyn</span> Interaction&gt;&gt;&gt;&gt;, <span class="c">// Its edges' interactions while dead, by edge guid.</span>
}

<span class="k">impl</span> Tomb {
    <span class="c">/// Construct a tomb with nothing parked.</span>
    <span class="k">pub</span> <span class="k">fn</span> new(
        collection: &amp;str,
        definition: bool,
        slot: usize,
        node: Option&lt;Rc&lt;RefCell&lt;TreeNode&gt;&gt;&gt;,
    ) -&gt; Rc&lt;Tomb&gt; {
        Rc::new(<span class="k">Self</span> {
            collection: collection.to_string(),
            definition,
            slot: Cell::new(slot),
            node,
            vertex: RefCell::new(None),
            edges: RefCell::new(Vec::new()),
            xform: RefCell::new(None),
            interactions: RefCell::new(BTreeMap::new()),
        })
    }
}

<span class="c">/// An object added or removed: the tomb that flips it and where its node sits.</span>
#[derive(Debug, Clone)]
<span class="k">pub</span> <span class="k">struct</span> Tombstone {
    <span class="k">pub</span> guid: String,                        <span class="c">// The object's guid.</span>
    <span class="k">pub</span> collection: String,                  <span class="c">// The Objects list it lives in, or &quot;definitions&quot;.</span>
    <span class="k">pub</span> parent_guid: Option&lt;String&gt;,         <span class="c">// Name of its tree parent, None when it has no node.</span>
    <span class="k">pub</span> index: usize, <span class="c">// Its raw index among the parent's children at record time, a hint.</span>
    <span class="k">pub</span> node: Option&lt;Rc&lt;RefCell&lt;TreeNode&gt;&gt;&gt;, <span class="c">// Its tree node, for adds too; None when it has none.</span>
    <span class="k">pub</span> tomb: Rc&lt;Tomb&gt;, <span class="c">// The tomb undo and redo flip.</span>
}

<span class="k">impl</span> Tombstone {
    <span class="c">/// Construct from every field of the record.</span>
    <span class="k">pub</span> <span class="k">fn</span> new(
        guid: String,
        collection: String,
        parent_guid: Option&lt;String&gt;,
        index: usize,
        node: Option&lt;Rc&lt;RefCell&lt;TreeNode&gt;&gt;&gt;,
        tomb: Rc&lt;Tomb&gt;,
    ) -&gt; <span class="k">Self</span> {
        <span class="k">Self</span> {
            guid,
            collection,
            parent_guid,
            index,
            node,
            tomb,
        }
    }
}

<span class="c">/// The entry a replace was taken on: an object by its tree node at record time (None outside the tree) or a definition by the tomb pinning its slot.</span>
#[derive(Debug, Clone)]
<span class="k">pub</span> <span class="k">enum</span> Entry {
    Object(Option&lt;Rc&lt;RefCell&lt;TreeNode&gt;&gt;&gt;),
    Definition(Rc&lt;Tomb&gt;),
}

<span class="c">/// The object or definition under \`guid\` was swapped: the stored pointers before and after, never copies.</span>
#[derive(Debug, Clone)]
<span class="k">pub</span> <span class="k">struct</span> ReplaceOp {
    <span class="k">pub</span> guid: String, <span class="c">// The entry's guid.</span>
    <span class="k">pub</span> before: Item, <span class="c">// The entry before the swap.</span>
    <span class="k">pub</span> after: Item,  <span class="c">// The entry after the swap.</span>
    <span class="k">pub</span> entry: Entry, <span class="c">// The entry the swap was taken on.</span>
}

<span class="k">impl</span> ReplaceOp {
    <span class="c">/// Construct from the guid, the before and after items and the entry.</span>
    <span class="k">pub</span> <span class="k">fn</span> new(guid: String, before: Item, after: Item, entry: Entry) -&gt; <span class="k">Self</span> {
        <span class="k">Self</span> {
            guid,
            before,
            after,
            entry,
        }
    }
}

<span class="c">/// The local transform under \`guid\` changed; None on either side means &quot;none set&quot;.</span>
#[derive(Debug, Clone)]
<span class="k">pub</span> <span class="k">struct</span> XformOp {
    <span class="k">pub</span> guid: String,                        <span class="c">// The object's guid.</span>
    <span class="k">pub</span> before: Option&lt;Xform&gt;,               <span class="c">// Transform before the change.</span>
    <span class="k">pub</span> after: Option&lt;Xform&gt;,                <span class="c">// Transform after the change.</span>
    <span class="k">pub</span> node: Option&lt;Rc&lt;RefCell&lt;TreeNode&gt;&gt;&gt;, <span class="c">// The entry's tree node at record time; None for a group or an object outside the tree.</span>
}

<span class="k">impl</span> XformOp {
    <span class="c">/// Construct from the guid, the before and after transforms and the entry's node.</span>
    <span class="k">pub</span> <span class="k">fn</span> new(
        guid: String,
        before: Option&lt;Xform&gt;,
        after: Option&lt;Xform&gt;,
        node: Option&lt;Rc&lt;RefCell&lt;TreeNode&gt;&gt;&gt;,
    ) -&gt; <span class="k">Self</span> {
        <span class="k">Self</span> {
            guid,
            before,
            after,
            node,
        }
    }
}

<span class="c">/// A tree node added, removed, moved, renamed or recoloured: its state before and after.</span>
#[derive(Debug, Clone)]
<span class="k">pub</span> <span class="k">struct</span> TreeOp {
    <span class="k">pub</span> guid: String,                         <span class="c">// The node name at record time.</span>
    <span class="k">pub</span> node: Rc&lt;RefCell&lt;TreeNode&gt;&gt;,          <span class="c">// The node itself.</span>
    <span class="k">pub</span> tomb: Rc&lt;Tomb&gt;,                       <span class="c">// Node-only; pins the ghost of a move, else the node.</span>
    <span class="k">pub</span> ghost: Option&lt;Rc&lt;RefCell&lt;TreeNode&gt;&gt;&gt;, <span class="c">// The dead ghost a move left in the old slot.</span>
    <span class="k">pub</span> name_before: String,                  <span class="c">// Name before.</span>
    <span class="k">pub</span> name_after: String,                   <span class="c">// Name after.</span>
    <span class="k">pub</span> color_before: Option&lt;Color&gt;,          <span class="c">// Colour before.</span>
    <span class="k">pub</span> color_after: Option&lt;Color&gt;,           <span class="c">// Colour after.</span>
    <span class="k">pub</span> dead_before: bool,                    <span class="c">// Whether it was dead or absent before.</span>
    <span class="k">pub</span> dead_after: bool,                     <span class="c">// Whether it is dead after.</span>
}

<span class="k">impl</span> TreeOp {
    <span class="c">/// Construct from every field of the record.</span>
    #[allow(clippy::too_many_arguments)]
    <span class="k">pub</span> <span class="k">fn</span> new(
        guid: String,
        node: Rc&lt;RefCell&lt;TreeNode&gt;&gt;,
        tomb: Rc&lt;Tomb&gt;,
        ghost: Option&lt;Rc&lt;RefCell&lt;TreeNode&gt;&gt;&gt;,
        name_before: String,
        name_after: String,
        color_before: Option&lt;Color&gt;,
        color_after: Option&lt;Color&gt;,
        dead_before: bool,
        dead_after: bool,
    ) -&gt; <span class="k">Self</span> {
        <span class="k">Self</span> {
            guid,
            node,
            tomb,
            ghost,
            name_before,
            name_after,
            color_before,
            color_after,
            dead_before,
            dead_after,
        }
    }
}

<span class="c">/// Any one recorded op.</span>
#[derive(Debug, Clone)]
<span class="k">pub</span> <span class="k">enum</span> Op {
    Add(Tombstone),
    Remove(Tombstone),
    Replace(ReplaceOp),
    Xform(XformOp),
    Tree(TreeOp),
}

<span class="k">impl</span> Op {
    <span class="c">/// Return &quot;add&quot;, &quot;remove&quot;, &quot;replace&quot;, &quot;xform&quot; or &quot;tree&quot;.</span>
    <span class="k">pub</span> <span class="k">fn</span> kind(&amp;<span class="k">self</span>) -&gt; &amp;str {
        <span class="k">match</span> <span class="k">self</span> {
            Op::Add(_) =&gt; &quot;<span class="s">add</span>&quot;,
            Op::Remove(_) =&gt; &quot;<span class="s">remove</span>&quot;,
            Op::Replace(_) =&gt; &quot;<span class="s">replace</span>&quot;,
            Op::Xform(_) =&gt; &quot;<span class="s">xform</span>&quot;,
            Op::Tree(_) =&gt; &quot;<span class="s">tree</span>&quot;,
        }
    }

    <span class="c">/// Return the guid of the object or the node name the op touched.</span>
    <span class="k">pub</span> <span class="k">fn</span> guid(&amp;<span class="k">self</span>) -&gt; &amp;str {
        <span class="k">match</span> <span class="k">self</span> {
            Op::Add(op) | Op::Remove(op) =&gt; &amp;op.guid,
            Op::Replace(op) =&gt; &amp;op.guid,
            Op::Xform(op) =&gt; &amp;op.guid,
            Op::Tree(op) =&gt; &amp;op.guid,
        }
    }

    <span class="c">/// Return a string representation of the record.</span>
    <span class="k">pub</span> <span class="k">fn</span> str(&amp;<span class="k">self</span>) -&gt; String {
        format!(&quot;{}<span class="s">(</span>{}<span class="s">)</span>&quot;, <span class="k">self</span>.kind(), <span class="k">self</span>.guid())
    }

    <span class="c">/// Return a string representation of the record for debugging.</span>
    <span class="k">pub</span> <span class="k">fn</span> repr(&amp;<span class="k">self</span>) -&gt; String {
        <span class="k">match</span> <span class="k">self</span> {
            Op::Add(op) | Op::Remove(op) =&gt; {
                format!(&quot;{}<span class="s">(</span>{}<span class="s">, </span>{}<span class="s">)</span>&quot;, <span class="k">self</span>.kind(), op.guid, op.collection)
            }
            _ =&gt; <span class="k">self</span>.str(),
        }
    }
}

<span class="k">impl</span> fmt::Display <span class="k">for</span> Op {
    <span class="c">/// Write the record string to a formatter.</span>
    <span class="k">fn</span> fmt(&amp;<span class="k">self</span>, f: &amp;<span class="k">mut</span> fmt::Formatter&lt;'_&gt;) -&gt; fmt::Result {
        write!(f, &quot;{}&quot;, <span class="k">self</span>.str())
    }
}

<span class="c">/// One undoable step: a label, the ops it made in the order they happened, and the bytes they pin.</span>
#[derive(Debug, Clone)]
<span class="k">pub</span> <span class="k">struct</span> Transaction {
    <span class="k">pub</span> label: String, <span class="c">// What the step did.</span>
    <span class="k">pub</span> ops: Vec&lt;Op&gt;,  <span class="c">// Ops in the order they happened.</span>
    <span class="k">pub</span> bytes: usize,  <span class="c">// Bytes its records pin.</span>
}

<span class="k">impl</span> Transaction {
    <span class="c">/// Construct an empty transaction with a label.</span>
    <span class="k">pub</span> <span class="k">fn</span> new(label: &amp;str) -&gt; <span class="k">Self</span> {
        <span class="k">Self</span> {
            label: label.to_string(),
            ops: Vec::new(),
            bytes: <span class="s">0</span>,
        }
    }

    <span class="c">/// Return a string representation of the transaction.</span>
    <span class="k">pub</span> <span class="k">fn</span> str(&amp;<span class="k">self</span>) -&gt; String {
        format!(&quot;<span class="s">Transaction(</span>{}<span class="s">, </span>{}<span class="s"> ops)</span>&quot;, <span class="k">self</span>.label, <span class="k">self</span>.ops.len())
    }

    <span class="c">/// Return a string representation of the transaction for debugging.</span>
    <span class="k">pub</span> <span class="k">fn</span> repr(&amp;<span class="k">self</span>) -&gt; String {
        format!(&quot;<span class="s">Transaction(</span>{}<span class="s">, </span>{}<span class="s"> ops)</span>&quot;, <span class="k">self</span>.label, <span class="k">self</span>.ops.len())
    }
}

<span class="k">impl</span> Default <span class="k">for</span> Transaction {
    <span class="c">/// Construct an empty transaction with the default label.</span>
    <span class="k">fn</span> default() -&gt; <span class="k">Self</span> {
        <span class="k">Self</span>::new(&quot;<span class="s">my_transaction</span>&quot;)
    }
}

<span class="k">impl</span> fmt::Display <span class="k">for</span> Transaction {
    <span class="c">/// Write the transaction string to a formatter.</span>
    <span class="k">fn</span> fmt(&amp;<span class="k">self</span>, f: &amp;<span class="k">mut</span> fmt::Formatter&lt;'_&gt;) -&gt; fmt::Result {
        write!(f, &quot;{}&quot;, <span class="k">self</span>.str())
    }
}

<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">// History</span>
<span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
<span class="c">/// CAD-style undo/redo over a Session, in memory only: records flip tombs in place, every save purges them.</span>
#[derive(Debug, Clone)]
<span class="k">pub</span> <span class="k">struct</span> History {
    <span class="k">pub</span> undo_stack: Vec&lt;Transaction&gt;, <span class="c">// Committed transactions, oldest first; capped at CAPACITY and budget.</span>
    <span class="k">pub</span> redo_stack: Vec&lt;Transaction&gt;, <span class="c">// Undone transactions, cleared the moment a new transaction commits.</span>
    <span class="k">pub</span> current: Option&lt;Transaction&gt;, <span class="c">// The open transaction, None between commit and the next begin.</span>
    <span class="k">pub</span> bytes: usize,                 <span class="c">// Bytes pinned by both stacks and the open transaction.</span>
    <span class="k">pub</span> budget: usize,                <span class="c">// Bytes the stacks may pin before the oldest is dropped.</span>
    <span class="k">pub</span> dropped: usize, <span class="c">// Ops dropped since the last purge cycle began, unrecorded kills included.</span>
}

<span class="k">impl</span> Default <span class="k">for</span> History {
    <span class="c">/// Construct an empty history with the default budget.</span>
    <span class="k">fn</span> default() -&gt; <span class="k">Self</span> {
        <span class="k">Self</span> {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            current: None,
            bytes: <span class="s">0</span>,
            budget: BUDGET,
            dropped: <span class="s">0</span>,
        }
    }
}

<span class="k">impl</span> History {
    <span class="c">/// Construct an empty history.</span>
    <span class="k">pub</span> <span class="k">fn</span> new() -&gt; <span class="k">Self</span> {
        <span class="k">Self</span>::default()
    }

    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">// Accessors</span>
    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">/// Return whether a committed transaction can be undone.</span>
    <span class="k">pub</span> <span class="k">fn</span> can_undo(&amp;<span class="k">self</span>) -&gt; bool {
        !<span class="k">self</span>.undo_stack.is_empty()
    }

    <span class="c">/// Return whether an undone transaction can be redone.</span>
    <span class="k">pub</span> <span class="k">fn</span> can_redo(&amp;<span class="k">self</span>) -&gt; bool {
        !<span class="k">self</span>.redo_stack.is_empty()
    }

    <span class="c">/// Return the number of committed transactions.</span>
    <span class="k">pub</span> <span class="k">fn</span> depth(&amp;<span class="k">self</span>) -&gt; usize {
        <span class="k">self</span>.undo_stack.len()
    }

    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">// Transactions</span>
    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">/// Open a transaction; an already open one is committed first so no op is lost.</span>
    <span class="k">pub</span> <span class="k">fn</span> begin(&amp;<span class="k">mut</span> <span class="k">self</span>, label: &amp;str) {
        <span class="k">self</span>.commit();
        <span class="k">self</span>.current = Some(Transaction::new(label));
    }

    <span class="c">/// Close the open transaction. An empty one is dropped; a real one clears redo and trims the oldest past the caps.</span>
    <span class="k">pub</span> <span class="k">fn</span> commit(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">let</span> Some(transaction) = <span class="k">self</span>.current.take() <span class="k">else</span> {
            <span class="k">return</span>;
        };

        <span class="k">if</span> transaction.ops.is_empty() {
            <span class="k">return</span>;
        }

        <span class="k">self</span>.undo_stack.push(transaction);

        <span class="k">for</span> undone <span class="k">in</span> <span class="k">self</span>.redo_stack.drain(..) {
            <span class="k">self</span>.dropped += undone.ops.len();
        }

        <span class="k">self</span>.bytes = <span class="k">self</span>._pinned();

        <span class="k">while</span> <span class="k">self</span>.undo_stack.len() &gt; <span class="s">1</span>
            &amp;&amp; (<span class="k">self</span>.undo_stack.len() &gt; CAPACITY || <span class="k">self</span>.bytes &gt; <span class="k">self</span>.budget)
        {
            <span class="k">let</span> oldest = <span class="k">self</span>.undo_stack.remove(<span class="s">0</span>);
            <span class="k">self</span>.dropped += oldest.ops.len();
            <span class="k">self</span>.bytes -= oldest.bytes;
        }
    }

    <span class="c">/// Append an op pinning \`bytes\` to the open transaction; a no-op when none is open.</span>
    <span class="k">pub</span> <span class="k">fn</span> record(&amp;<span class="k">mut</span> <span class="k">self</span>, op: Op, bytes: usize) {
        <span class="k">let</span> Some(current) = <span class="k">self</span>.current.as_mut() <span class="k">else</span> {
            <span class="k">return</span>;
        };

        current.ops.push(op);
        current.bytes += bytes;
        <span class="k">self</span>.bytes += bytes;
    }

    <span class="c">/// Revert the open transaction's ops in reverse and drop it, leaving both stacks as they are; false when none is open.</span>
    <span class="k">pub</span> <span class="k">fn</span> abort(&amp;<span class="k">mut</span> <span class="k">self</span>, session: &amp;<span class="k">mut</span> Session) -&gt; bool {
        <span class="k">let</span> Some(transaction) = <span class="k">self</span>.current.take() <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };

        <span class="k">for</span> i <span class="k">in</span> (<span class="s">0</span>..transaction.ops.len()).rev() {
            <span class="k">self</span>._revert(&amp;transaction.ops[i], session);
        }

        <span class="k">self</span>.dropped += transaction.ops.len();
        <span class="k">self</span>.bytes = <span class="k">self</span>._pinned();

        <span class="s">true</span>
    }

    <span class="c">/// Revert the newest transaction, ops in reverse order, and park it for redo.</span>
    <span class="k">pub</span> <span class="k">fn</span> undo(&amp;<span class="k">mut</span> <span class="k">self</span>, session: &amp;<span class="k">mut</span> Session) -&gt; bool {
        <span class="k">self</span>.commit();

        <span class="k">let</span> Some(transaction) = <span class="k">self</span>.undo_stack.pop() <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };

        <span class="k">for</span> i <span class="k">in</span> (<span class="s">0</span>..transaction.ops.len()).rev() {
            <span class="k">self</span>._revert(&amp;transaction.ops[i], session);
        }

        <span class="k">self</span>.redo_stack.push(transaction);

        <span class="s">true</span>
    }

    <span class="c">/// Re-apply the newest undone transaction, ops in their original order.</span>
    <span class="k">pub</span> <span class="k">fn</span> redo(&amp;<span class="k">mut</span> <span class="k">self</span>, session: &amp;<span class="k">mut</span> Session) -&gt; bool {
        <span class="k">self</span>.commit();

        <span class="k">let</span> Some(transaction) = <span class="k">self</span>.redo_stack.pop() <span class="k">else</span> {
            <span class="k">return</span> <span class="s">false</span>;
        };

        <span class="k">for</span> i <span class="k">in</span> <span class="s">0</span>..transaction.ops.len() {
            <span class="k">self</span>._apply(&amp;transaction.ops[i], session);
        }

        <span class="k">self</span>.undo_stack.push(transaction);

        <span class="s">true</span>
    }

    <span class="c">/// Drop every transaction, open or committed; what they pinned is purgeable now.</span>
    <span class="k">pub</span> <span class="k">fn</span> clear(&amp;<span class="k">mut</span> <span class="k">self</span>) {
        <span class="k">for</span> transaction <span class="k">in</span> &amp;<span class="k">self</span>.undo_stack {
            <span class="k">self</span>.dropped += transaction.ops.len();
        }

        <span class="k">for</span> transaction <span class="k">in</span> &amp;<span class="k">self</span>.redo_stack {
            <span class="k">self</span>.dropped += transaction.ops.len();
        }

        <span class="k">if</span> <span class="k">let</span> Some(current) = &amp;<span class="k">self</span>.current {
            <span class="k">self</span>.dropped += current.ops.len();
        }

        <span class="k">self</span>.undo_stack.clear();
        <span class="k">self</span>.redo_stack.clear();
        <span class="k">self</span>.current = None;
        <span class="k">self</span>.bytes = <span class="s">0</span>;
    }

    <span class="c">/// Bytes pinned by both stacks.</span>
    <span class="k">fn</span> _pinned(&amp;<span class="k">self</span>) -&gt; usize {
        <span class="k">let</span> <span class="k">mut</span> pinned = <span class="s">0</span>;

        <span class="k">for</span> transaction <span class="k">in</span> &amp;<span class="k">self</span>.undo_stack {
            pinned += transaction.bytes;
        }

        <span class="k">for</span> transaction <span class="k">in</span> &amp;<span class="k">self</span>.redo_stack {
            pinned += transaction.bytes;
        }

        pinned
    }

    <span class="c">/// Undo one op against the session.</span>
    <span class="k">fn</span> _revert(&amp;<span class="k">self</span>, op: &amp;Op, session: &amp;<span class="k">mut</span> Session) {
        <span class="k">match</span> op {
            Op::Add(op) =&gt; session._kill(&amp;op.tomb),
            Op::Remove(op) =&gt; session._revive(&amp;op.tomb),
            Op::Replace(op) =&gt; session._swap(&amp;op.guid, op.before.clone(), &amp;op.entry),
            Op::Xform(op) =&gt; session._place(&amp;op.guid, op.before.as_ref(), op.node.as_ref()),
            Op::Tree(op) =&gt; session._tree(op, <span class="s">true</span>),
        }
    }

    <span class="c">/// Redo one op against the session.</span>
    <span class="k">fn</span> _apply(&amp;<span class="k">self</span>, op: &amp;Op, session: &amp;<span class="k">mut</span> Session) {
        <span class="k">match</span> op {
            Op::Add(op) =&gt; session._revive(&amp;op.tomb),
            Op::Remove(op) =&gt; session._kill(&amp;op.tomb),
            Op::Replace(op) =&gt; session._swap(&amp;op.guid, op.after.clone(), &amp;op.entry),
            Op::Xform(op) =&gt; session._place(&amp;op.guid, op.after.as_ref(), op.node.as_ref()),
            Op::Tree(op) =&gt; session._tree(op, <span class="s">false</span>),
        }
    }

    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">// String</span>
    <span class="c">// ═══════════════════════════════════════════════════════════════════════════</span>
    <span class="c">/// Return a string representation of the history.</span>
    <span class="k">pub</span> <span class="k">fn</span> str(&amp;<span class="k">self</span>) -&gt; String {
        format!(
            &quot;<span class="s">History(</span>{}<span class="s"> undo, </span>{}<span class="s"> redo)</span>&quot;,
            <span class="k">self</span>.undo_stack.len(),
            <span class="k">self</span>.redo_stack.len()
        )
    }

    <span class="c">/// Return a string representation of the history for debugging.</span>
    <span class="k">pub</span> <span class="k">fn</span> repr(&amp;<span class="k">self</span>) -&gt; String {
        format!(
            &quot;<span class="s">History(</span>{}<span class="s"> undo, </span>{}<span class="s"> redo)</span>&quot;,
            <span class="k">self</span>.undo_stack.len(),
            <span class="k">self</span>.redo_stack.len()
        )
    }
}

<span class="k">impl</span> fmt::Display <span class="k">for</span> History {
    <span class="c">/// Write the history string to a formatter.</span>
    <span class="k">fn</span> fmt(&amp;<span class="k">self</span>, f: &amp;<span class="k">mut</span> fmt::Formatter&lt;'_&gt;) -&gt; fmt::Result {
        write!(f, &quot;{}&quot;, <span class="k">self</span>.str())
    }
}</code></pre></div>
`,toc:[]};export{s as default};
