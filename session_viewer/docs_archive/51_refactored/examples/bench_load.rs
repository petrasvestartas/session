//! Where the load time actually goes: prost decode vs object build vs lookup vs walk.
use std::time::Instant;
use std::rc::Rc;
use prost::Message;
use session_rust::{proto, Session, Geometry, Polyline, Point, Line, Mesh, PointCloud, Xform};
use session_viewer::app::scene::{FileDoc, Scene};

fn main() {
    let path = std::env::args().nth(1).expect("usage: bench_load <file.pb>");
    let t = Instant::now();
    let bytes = std::fs::read(&path).unwrap();
    println!("read           {:>7.0} ms  ({:.1} MB)", t.elapsed().as_secs_f64()*1e3, bytes.len() as f64/1.048576e6);

    let t = Instant::now();
    let p = proto::Session::decode(&bytes[..]).unwrap();
    println!("prost decode   {:>7.0} ms  (full: objects + tree + graph)", t.elapsed().as_secs_f64()*1e3);
    // What a display_only document decodes since lesson 51: no tree, no graph, no bvh boxes.
    let t = Instant::now();
    let lean = session_viewer::app::decode::LeanSession::decode(&bytes[..]).unwrap();
    println!("lean decode    {:>7.0} ms  (objects + xforms only, {} xforms)", t.elapsed().as_secs_f64()*1e3, lean.xforms.len());

    {
        use prost::Message;
        fn count(n: &session_rust::proto::TreeNode) -> (usize, usize) {
            let mut c = 1; let mut b = n.name.len();
            for ch in &n.children { let (cc, bb) = count(ch); c += cc; b += bb; }
            (c, b)
        }
        let tree_len = p.tree.as_ref().map_or(0, |t| t.encoded_len());
        let (nodes, namebytes) = p.tree.as_ref().and_then(|t| t.root.as_ref()).map_or((0,0), count);
        println!("  tree: {:.1} MB encoded | {nodes} nodes | {:.1} MB of names", tree_len as f64/1.048576e6, namebytes as f64/1.048576e6);
        println!("  xforms: {} entries | graph {:.1} MB", p.xforms.len(), p.graph.as_ref().map_or(0, |g| g.encoded_len()) as f64/1.048576e6);
        println!("  objects total: {:.1} MB", p.objects.as_ref().map_or(0, |o| o.encoded_len()) as f64/1.048576e6);
    }
    {
        use prost::Message;
        use session_rust::proto as sp;
        #[derive(Clone, PartialEq, prost::Message)]
        struct LinesOnly { #[prost(message, repeated, tag = "4")] lines: Vec<sp::Line> }
        #[derive(Clone, PartialEq, prost::Message)]
        struct MeshesOnly { #[prost(message, repeated, tag = "9")] meshes: Vec<sp::Mesh> }
        #[derive(Clone, PartialEq, prost::Message)]
        struct LinesSess { #[prost(message, optional, tag = "3")] objects: Option<LinesOnly> }
        #[derive(Clone, PartialEq, prost::Message)]
        struct MeshSess { #[prost(message, optional, tag = "3")] objects: Option<MeshesOnly> }
        let t = Instant::now();
        let l = LinesSess::decode(&bytes[..]).unwrap();
        println!("  lines only   {:>7.0} ms  ({} lines)", t.elapsed().as_secs_f64()*1e3, l.objects.map_or(0, |o| o.lines.len()));
        // Wire-identical mirror: protobuf encodes map<K,V> exactly as repeated {K key=1; V value=2},
        // so declaring the map fields `repeated` turns 700k+ hashed map inserts into Vec pushes.
        #[derive(Clone, PartialEq, prost::Message)]
        struct VEntry { #[prost(uint64, tag="1")] k: u64, #[prost(message, optional, tag="2")] v: Option<sp::VertexData> }
        #[derive(Clone, PartialEq, prost::Message)]
        struct FEntry { #[prost(uint64, tag="1")] k: u64, #[prost(message, optional, tag="2")] v: Option<sp::FaceData> }
        #[derive(Clone, PartialEq, prost::Message)]
        struct LeanMeshP {
            #[prost(message, repeated, tag="3")] vertices: Vec<VEntry>,
            #[prost(message, repeated, tag="4")] faces: Vec<FEntry>,
        }
        #[derive(Clone, PartialEq, prost::Message)]
        struct LeanMeshesOnly { #[prost(message, repeated, tag = "9")] meshes: Vec<LeanMeshP> }
        #[derive(Clone, PartialEq, prost::Message)]
        struct LeanMeshSess { #[prost(message, optional, tag = "3")] objects: Option<LeanMeshesOnly> }
        let t = Instant::now();
        let lm = LeanMeshSess::decode(&bytes[..]).unwrap();
        let (nv, nf) = lm.objects.as_ref().map_or((0,0), |o| (
            o.meshes.iter().map(|m| m.vertices.len()).sum::<usize>(),
            o.meshes.iter().map(|m| m.faces.len()).sum::<usize>()));
        println!("  meshes VEC   {:>7.0} ms  ({nv} verts {nf} faces, no map hashing)", t.elapsed().as_secs_f64()*1e3);

        // Decode the map fields as repeated entries (wire-identical), then BULK-BUILD the
        // BTreeMap the generated type wants. std's FromIterator sorts + bulk-builds, which beats
        // 362k individual B-tree inserts.
        let t = Instant::now();
        let lm2 = LeanMeshSess::decode(&bytes[..]).unwrap();
        let t2 = Instant::now();
        let mut nb = 0usize;
        if let Some(o) = lm2.objects {
            for mm in o.meshes {
                let v: std::collections::BTreeMap<u64, sp::VertexData> =
                    mm.vertices.into_iter().filter_map(|e| e.v.map(|x| (e.k, x))).collect();
                let f: std::collections::BTreeMap<u64, sp::FaceData> =
                    mm.faces.into_iter().filter_map(|e| e.v.map(|x| (e.k, x))).collect();
                nb += v.len() + f.len();
            }
        }
        println!("  VEC+bulk     {:>7.0} ms  (decode {:.0} + build {:.0}, {nb} entries)",
            t.elapsed().as_secs_f64()*1e3, (t2 - t).as_secs_f64()*1e3, t2.elapsed().as_secs_f64()*1e3);

        let t = Instant::now();
        let m = MeshSess::decode(&bytes[..]).unwrap();
        println!("  meshes only  {:>7.0} ms  ({} meshes)", t.elapsed().as_secs_f64()*1e3, m.objects.map_or(0, |o| o.meshes.len()));
    }
    let o = p.objects.as_ref().unwrap();
    println!("  counts: {} pts {} lines {} plines {} meshes {} clouds",
        o.points.len(), o.lines.len(), o.polylines.len(), o.meshes.len(), o.pointclouds.len());

    if let Some(l) = o.lines.first() {
        // P6: coords/linecolor_rgba are packed; the Point and Color sub-messages are gone.
        println!("  sample line: encoded {} B | guid {:?} name {:?} dash {} coords {} rgba {}",
            l.encoded_len(), l.guid, l.name, l.dash.len(), l.coords.len(), l.linecolor_rgba.len());
        let tot: usize = o.lines.iter().map(|l| l.encoded_len()).sum();
        let guids: usize = o.lines.iter().map(|l| l.guid.len() + l.name.len()).sum();
        let dash: usize = o.lines.iter().map(|l| l.dash.len()*8).sum();
        println!("  lines total {:.1} MB | guid+name {:.1} MB | dash {:.1} MB",
            tot as f64/1.048576e6, guids as f64/1.048576e6, dash as f64/1.048576e6);
    }

    {
        use prost::Message;
        let ms = &o.meshes;
        let tot: usize = ms.iter().map(|m| m.encoded_len()).sum();
        let verts: usize = ms.iter().map(|m| m.vertices.len()).sum();
        let attrs: usize = ms.iter().map(|m| m.vertices.values().map(|v| v.attributes.len()).sum::<usize>()).sum();
        let faces: usize = ms.iter().map(|m| m.faces.len()).sum();
        println!("  meshes: {:.1} MB encoded | {verts} verts ({attrs} attr entries) | {faces} faces",
            tot as f64/1.048576e6);
    }

    // object build only (no lookup)
    let o = p.objects.unwrap();
    let t = Instant::now();
    let plines: Vec<Rc<Polyline>> = o.polylines.into_iter().map(|x| Rc::new(Polyline::from_proto(x))).collect();
    println!("polyline build {:>7.0} ms  ({} objs)", t.elapsed().as_secs_f64()*1e3, plines.len());
    let t = Instant::now();
    let lines: Vec<Rc<Line>> = o.lines.into_iter().map(|x| Rc::new(Line::from_proto(x))).collect();
    println!("line build     {:>7.0} ms  ({} objs)", t.elapsed().as_secs_f64()*1e3, lines.len());
    let t = Instant::now();
    let meshes: Vec<Rc<Mesh>> = o.meshes.into_iter().map(|x| Rc::new(Mesh::from_proto(x))).collect();
    println!("mesh build     {:>7.0} ms  ({} objs)", t.elapsed().as_secs_f64()*1e3, meshes.len());
    let t = Instant::now();
    let clouds: Vec<Rc<PointCloud>> = o.pointclouds.into_iter().map(|x| Rc::new(PointCloud::from_proto(x))).collect();
    println!("cloud build    {:>7.0} ms  ({} objs)", t.elapsed().as_secs_f64()*1e3, clouds.len());

    // lookup insert cost, measured on its own
    let mut s = Session::new("bench");
    let t = Instant::now();
    for g in &lines { s.lookup.insert(g.guid().to_string(), Geometry::Line(Rc::clone(g))); }
    for g in &plines { s.lookup.insert(g.guid().to_string(), Geometry::Polyline(Rc::clone(g))); }
    for g in &meshes { s.lookup.insert(g.guid().to_string(), Geometry::Mesh(Rc::clone(g))); }
    for g in &clouds { s.lookup.insert(g.guid().to_string(), Geometry::PointCloud(Rc::clone(g))); }
    println!("lookup insert  {:>7.0} ms  ({} keys)", t.elapsed().as_secs_f64()*1e3, s.lookup.len());
    s.objects.polylines = plines;
    s.objects.lines = lines;
    s.objects.meshes = meshes;
    s.objects.pointclouds = clouds;

    {
        // Do the sheet's fills share a plane? If so the depth buffer cannot order them.
        let mut zs: std::collections::BTreeMap<i64, usize> = std::collections::BTreeMap::new();
        for m in &s.objects.meshes {
            for (_, v) in &m.vertex { *zs.entry((v.z * 1e6).round() as i64).or_insert(0) += 1; }
        }
        let shown: Vec<String> = zs.iter().take(6).map(|(z, n)| format!("z={:.6} x{n}", *z as f64 / 1e6)).collect();
        println!("  mesh vertex Z levels: {} distinct -> {}", zs.len(), shown.join(", "));
    }

    {
        // What distinguishes text from hatch? Look at the names/colors the importer wrote.
        let mut mnames: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
        for m in &s.objects.meshes { *mnames.entry(m.name.clone()).or_insert(0) += 1; }
        println!("  mesh names: {:?}", mnames.iter().take(12).collect::<Vec<_>>());
        fn walk(n: &session_rust::proto::TreeNode, out: &mut std::collections::BTreeMap<String, usize>, d: usize) {
            if d < 3 { *out.entry(format!("d{d}:{}", n.name)).or_insert(0) += 1; }
            for c in &n.children { walk(c, out, d + 1); }
        }
        let mut tn: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
        if let Some(t) = p.tree.as_ref().and_then(|t| t.root.as_ref()) { walk(t, &mut tn, 0); }
        println!("  tree names (depth<3): {:?}", tn.iter().take(14).collect::<Vec<_>>());
        for (i, m) in s.objects.meshes.iter().enumerate() {
            let oc = m.objectcolor();
            let n = m.number_of_vertices();
            let f = m.face.len();
            let ws: Vec<f64> = m.widths().iter().take(2).copied().collect();
            println!("    mesh[{i}] verts {n:>7} faces {f:>7} color ({:.2},{:.2},{:.2},a={:.2}) widths{:?} pcols {} fcols {}",
                oc.r, oc.g, oc.b, oc.a, ws, m.get_pointcolors().len(), m.get_facecolors().len());
        }
        let mut lnames: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
        for l in &s.objects.lines { *lnames.entry(l.name.clone()).or_insert(0) += 1; }
        println!("  line names: {:?}", lnames.iter().take(12).collect::<Vec<_>>());
        
    }

    // Is the per-line cost really Point::new (name String + Color::black) x2?
    let t = Instant::now();
    let mut acc = 0.0f64;
    for l in &s.objects.lines { let a = l.start(); let b = l.end(); acc += a.to_f32()[0] as f64 + b.to_f32()[0] as f64; }
    println!("  start()+end()  {:>7.0} ms  (acc {acc:.0})", t.elapsed().as_secs_f64()*1e3);
    let t = Instant::now();
    let mut acc2 = 0.0f64;
    for l in &s.objects.lines { acc2 += l.length(); }
    println!("  length() only  {:>7.0} ms  (acc {acc2:.0})", t.elapsed().as_secs_f64()*1e3);
    // The walk's way since lesson 51: six floats by index, no Point, no String.
    let t = Instant::now();
    let mut acc3 = 0.0f64;
    for l in &s.objects.lines { acc3 += l[0] as f32 as f64 + l[3] as f32 as f64; }
    println!("  l[0]..l[5]     {:>7.0} ms  (acc {acc3:.0})", t.elapsed().as_secs_f64()*1e3);

    let t = Instant::now();
    let mut closed = 0;
    for m in &s.objects.meshes { if m.is_closed() { closed += 1; } }
    println!("  is_closed()    {:>7.0} ms  ({closed}/{} closed)", t.elapsed().as_secs_f64()*1e3, s.objects.meshes.len());
    let t = Instant::now();
    let mut nv = 0;
    for m in &s.objects.meshes { nv += m.number_of_vertices(); }
    println!("  n_vertices()   {:>7.0} ms  ({nv} verts)", t.elapsed().as_secs_f64()*1e3);
    let t = Instant::now();
    let mut nr = 0;
    for m in &s.objects.meshes { nr += m.to_render().vertices.len(); }
    println!("  to_render()    {:>7.0} ms  ({nr} rows)", t.elapsed().as_secs_f64()*1e3);

    // what add_file pays BEFORE touching geometry: order() Strings + lookup hashing
    let t = Instant::now();
    let ord = s.order();
    println!("  order()      {:>7.0} ms  ({} guid Strings)", t.elapsed().as_secs_f64()*1e3, ord.len());
    let t = Instant::now();
    let mut hit = 0usize;
    for g in &ord { if s.lookup.get(g).is_some() { hit += 1; } }
    println!("  lookup.get() {:>7.0} ms  ({hit} hits)", t.elapsed().as_secs_f64()*1e3);
    let t = Instant::now();
    let w = s.world_xforms();
    println!("  world_xforms {:>7.0} ms  ({} entries)", t.elapsed().as_secs_f64()*1e3, w.len());

    let t = Instant::now();
    let mut scene = Scene::new();
    scene.add_file(FileDoc { name: "bench".into(), session: s, place: Xform::identity(), point_px: 1.0, display_only: false });
    println!("walk           {:>7.0} ms", t.elapsed().as_secs_f64()*1e3);
    let _ = Point::new(0.0,0.0,0.0);
    let _ = Line::default();
}
