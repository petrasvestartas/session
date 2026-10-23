use session_rust::{AABB,Point,Mesh};
use session_rust::mesh::ColorMode;
use crate::production_ribbon::{encode_width,floats};
use crate::source_mesh_topology::{MeshTopo,SlotMap,mesh_topology};
use crate::production_curve_walk::Row;
const FACING_UNKNOWN:u32=u32::MAX;
pub const GLYPH:&str=include_str!("production_glyph.wgsl");
pub const SPHERE:&str=include_str!("production_sphere.wgsl");
pub const PROVENANCE:&str=include_str!("production_glyph_provenance.json");
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GlyphPoint {
    pub center: [f32; 3],     // world position
    pub radius: f32,          // 0 = pen width; > 0 world mm; < 0 screen px
    pub color: [f32; 4],      // rgba
    pub instance_id: u32,     // object row
    pub facing: u32,          // packed normals of the faces around it
    pub facing_ext: [u32; 2], // more packed normals
}

const _: () = assert!(std::mem::size_of::<GlyphPoint>() == 48);


#[derive(Default)]
pub struct GlyphRows {pub dots:Vec<GlyphPoint>,pub spheres:Vec<GlyphPoint>}
struct Ink<'a>{glyph:&'a mut GlyphRows}
struct InkCx<'a>{row:u32,vpos:&'a [[f32;3]],slots:&'a SlotMap}
#[derive(Clone,Copy,Debug,PartialEq)]
pub struct Style {pub radius:f32,pub facing:[u32;3]}
/// Sign of `v`, +1 for zero.
fn sign_not_zero(v: f64) -> f64 {
    if v < 0.0 { -1.0 } else { 1.0 }
}

/// One octahedral coordinate to a signed byte.
fn quant_snorm8(v: f64) -> u32 {
    (((v.clamp(-1.0, 1.0) * 127.0).round() as i32) as u32) & 0xff
}

/// A direction packed into 16 bits.
pub fn oct16(n: &[f64; 3]) -> Option<u32> {
    let l = n[0].abs() + n[1].abs() + n[2].abs();

    if l.partial_cmp(&0.0) != Some(std::cmp::Ordering::Greater) {
        return None; // zero or NaN vector
    }

    let (mut x, mut y) = (n[0] / l, n[1] / l);

    if n[2] < 0.0 {
        // fold the lower half onto the square
        let (ax, ay) = (x.abs(), y.abs());
        (x, y) = ((1.0 - ay) * sign_not_zero(x), (1.0 - ax) * sign_not_zero(y));
    }

    Some(quant_snorm8(x) | quant_snorm8(y) << 8)
}

/// One SDF dot.
pub fn walk_point(glyph: &mut GlyphRows, p: &Point, row: u32) -> Row {
    let center = p.to_f32();
    let radius = encode_width(p.width); // negative = pixels
    glyph.dots.push(GlyphPoint {
        center,
        radius: if radius == 0.0 { -3.0 } else { radius }, // no pen: 3 px dot
        color: p.pointcolor.to_f32(),
        instance_id: row,
        facing: FACING_UNKNOWN,          // no face orientation
        facing_ext: [FACING_UNKNOWN; 2], // no neighbour orientation
    });
    let mut bounds = AABB::empty();
    bounds.union_with_point(center[0] as f64, center[1] as f64, center[2] as f64);
    Row {flags:0,bounds}
}
/// Pen width of edge `i`; one entry applies to all.
fn width_at(w: &[f64], i: usize) -> f64 {
    if w.len() == 1 {
        w[0]
    } else {
        w.get(i).copied().unwrap_or(1.0)
    }
}

/// Width 0 hides the edge.
fn hidden(w: &[f64], i: usize) -> bool {
    width_at(w, i) == 0.0
}

/// Append edge `ei`'s faces to `fkeys`, deduped.
fn push_faces(edge_faces: &[[u32; 2]], ei: usize, fkeys: &mut Vec<usize>) {
    for &f in edge_faces[ei].iter() {
        if f == u32::MAX {
            continue;
        }

        let fk = f as usize;

        if !fkeys.contains(&fk) {
            fkeys.push(fk);
        }
    }
}

/// Two normal codes packed into word `k`.
fn facing_word(codes: &[u32], k: usize) -> u32 {
    match (codes.get(2 * k).copied(), codes.get(2 * k + 1).copied()) {
        (Some(a), b) => {
            let v = a | b.unwrap_or(a) << 16;

            if v == FACING_UNKNOWN { v ^ 1 } else { v }
        }
        _ => FACING_UNKNOWN,
    }
}

/// Which edges touch each vertex.
struct Incidence {
    best: Vec<(f64, usize)>, // per vertex: widest edge (width, index)
    vstart: Vec<u32>,        // per vertex: start into `vinc`
    vinc: Vec<u32>,          // edge indices, grouped by vertex
}

/// Build the vertex to edge tables.
fn incidence(m: &Mesh, topo: &MeshTopo, cx: &InkCx) -> Incidence {
    let w = m.widths();
    let nv = cx.vpos.len();
    let mut best = vec![(f64::NEG_INFINITY, 0usize); nv];

    for (i, (a, b, _)) in topo.edges.iter().enumerate() {
        if hidden(w, i) {
            continue;
        }

        let wi = width_at(w, i);

        for vk in [*a, *b] {
            let e = &mut best[cx.slots.slot(vk)];

            if wi > e.0 {
                *e = (wi, i);
            }
        }
    }

    // count edges per vertex, then prefix sum
    let mut vstart = vec![0u32; nv + 1];

    for (a, b, _) in topo.edges.iter() {
        vstart[cx.slots.slot(*a) + 1] += 1;
        vstart[cx.slots.slot(*b) + 1] += 1;
    }

    for i in 0..nv {
        vstart[i + 1] += vstart[i];
    }

    // fill each vertex's edge list
    let mut vinc = vec![0u32; 2 * topo.edges.len()];
    let mut cur = vstart.clone();

    for (i, (a, b, _)) in topo.edges.iter().enumerate() {
        for vk in [*a, *b] {
            let s = cx.slots.slot(vk);
            vinc[cur[s] as usize] = i as u32;
            cur[s] += 1;
        }
    }

    Incidence { best, vstart, vinc }
}

/// What the marker loop reads.
struct MarkerCx<'a, 'b> {
    cx: &'a InkCx<'b>,  // ink context
    inc: &'a Incidence, // vertex to edge tables
}

/// One dot per vertex with a visible edge.
fn push_markers(ink: &mut Ink, m: &Mesh, topo: &MeshTopo, input: &MarkerCx) {
    let (cx, inc) = (input.cx, input.inc);
    let pc = m.get_pointcolors();
    let dots_colored = m.color_mode == ColorMode::POINTCOLORS && pc.len() == m.number_of_vertices(); // per-vertex colours
    let nv = cx.vpos.len();
    let mut fkeys: Vec<usize> = Vec::new(); // faces around one vertex
    let mut codes: Vec<u32> = Vec::new(); // packed normals of those faces
    ink.glyph.spheres.reserve(nv);

    for (i, &(vw, ei)) in inc.best.iter().enumerate().take(nv) {
        if vw == f64::NEG_INFINITY {
            continue; // no visible edge
        }

        fkeys.clear();
        push_faces(&topo.edge_faces, ei, &mut fkeys);

        for &j in &inc.vinc[inc.vstart[i] as usize..inc.vstart[i + 1] as usize] {
            push_faces(&topo.edge_faces, j as usize, &mut fkeys);
        }

        codes.clear();

        for fk in &fkeys {
            if let Some(n) = topo.normals[*fk]
                && let Some(code) = oct16(&n)
                && !codes.contains(&code)
            {
                codes.push(code);
            }
        }

        ink.glyph.spheres.push(GlyphPoint {
            center: cx.vpos[i],
            radius: encode_width(vw),
            color: if dots_colored {
                pc[i].to_f32()
            } else {
                [0.1, 0.1, 0.1, 1.0]
            },
            instance_id: cx.row,
            // more than six faces: always draw
            facing: if codes.len() > 6 {
                FACING_UNKNOWN
            } else {
                facing_word(&codes, 0)
            },
            facing_ext: if codes.len() > 6 {
                [FACING_UNKNOWN; 2]
            } else {
                [facing_word(&codes, 1), facing_word(&codes, 2)]
            },
        });
    }
}


pub fn prepare(source:&Mesh)->(Vec<Style>,u32) {
    let keys=source.vertices();let points:Vec<_>=keys.iter().map(|key|{let p=&source.vertex[key];[p.x,p.y,p.z]}).collect();
    let slots=SlotMap::new(&keys);let topo=mesh_topology(source,&keys,&points,&slots);let points:Vec<_>=points.into_iter().map(|p|p.map(|v|v as f32)).collect();
    let cx=InkCx{row:0,vpos:&points,slots:&slots};let inc=incidence(source,&topo,&cx);let mut glyph=GlyphRows::default();
    push_markers(&mut Ink{glyph:&mut glyph},source,&topo,&MarkerCx{cx:&cx,inc:&inc});
    let flags=1024|if topo.closed{0}else{16}|if source.number_of_faces()==1{128}else{0};
    (glyph.spheres.into_iter().map(|row|Style{radius:row.radius,facing:[row.facing,row.facing_ext[0],row.facing_ext[1]]}).collect(),flags)
}
pub fn bytes(row:&GlyphPoint,out:&mut Vec<u8>){
    floats(out,row.center);floats(out,[row.radius]);floats(out,row.color);
    out.extend([row.instance_id,row.facing,row.facing_ext[0],row.facing_ext[1]].into_iter().flat_map(u32::to_ne_bytes));
}
pub fn append(scene:&crate::scene::Scene,out:&mut crate::production_joined::Rows,anchor:[f64;3],selected:&[crate::scene::ObjectId]) {
    let add=|out:&mut crate::production_joined::Rows,id,model:&session_rust::Xform,flags,spacing:f32|{
        let (mut object,translation)=crate::production_joined::object(scene,id,model,anchor,selected.contains(&id),flags);
        object[88..92].copy_from_slice(&spacing.to_ne_bytes());
        if let Some(rgb)=scene.view.edges.get(&id){let flags=u32::from_ne_bytes(object[80..84].try_into().unwrap())|512;object[80..84].copy_from_slice(&flags.to_ne_bytes());object[92..96].copy_from_slice(&crate::production_ribbon::pack_rgba([rgb[0]as f32/255.,rgb[1]as f32/255.,rgb[2]as f32/255.,1.]).to_ne_bytes());}
        out.objects.extend(object);out.translations.extend(translation);out.object_ids.push(id);out.pick_owners.push(crate::feature_owner::display_owner(scene,id).map_or(0,crate::scene::ObjectId::value));
    };
    for row in scene.points().iter().filter(|r|scene.visible(r.id)) {
        let index=out.object_ids.len()as u32;let mut rows=GlyphRows::default();walk_point(&mut rows,&row.prepared.source,index);
        add(out,row.id,&row.model,0,0.);for glyph in &rows.dots{bytes(glyph,&mut out.dots);}
    }
    for row in scene.objects().iter().filter(|r|scene.visible(r.id)&&r.cad.is_none()&&r.surface.is_none()&&!scene.forest.derived(r.id)&&!r.mesh.parts.dots.is_empty()) {
        let index=out.object_ids.len()as u32;add(out,row.id,&row.model,row.mesh.parts.glyph_flags,row.mesh.spacing);
        if let Some(bounds)=scene.selected_bounds(row.id){out.inside.push((index,bounds));}
        for(i,&address)in row.mesh.parts.dots.iter().enumerate(){
            let p=row.mesh.parts.endpoints[address as usize/2][address as usize%2];let style=row.mesh.parts.dot_styles[i];
            bytes(&GlyphPoint{center:p.map(|v|v as f32),radius:style.radius,color:row.mesh.parts.dot_colour(i),instance_id:index,facing:style.facing[0],facing_ext:[style.facing[1],style.facing[2]]},&mut out.spheres);
        }
    }
}
