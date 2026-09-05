// Sample a mesh's surface into a point cloud WITH normals - the demo data for the splat
// lane's lambert shading (scans carry no normals; a sampled surface does). Area-weighted
// triangle sampling, barycentric position + interpolated vertex normal per point.
//
// cargo run --example mk_bunny_cloud --target x86_64-unknown-linux-gnu --release -- \
//     assets/pb/mesh_bunny_grey.pb assets/pb/bunny_cloud.pb 400000

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let src = a.first().cloned().unwrap_or_else(|| "assets/pb/mesh_bunny_grey.pb".into());
    let out = a.get(1).cloned().unwrap_or_else(|| "assets/pb/bunny_cloud.pb".into());
    let count: usize = a.get(2).and_then(|v| v.parse().ok()).unwrap_or(400_000);

    let bytes = std::fs::read(&src).expect("read src pb");
    let session = session_rust::Session::pb_loads(&bytes).expect("parse src pb");
    let mesh = session.order().into_iter().find_map(|g| match session.lookup.get(&g) {
        Some(session_rust::Geometry::Mesh(m)) => Some(m.clone()),
        _ => None,
    }).expect("no mesh in source pb");

    let rm = mesh.to_render();
    // cumulative triangle areas for area-weighted sampling
    let tri = |i: usize| {
        let f = [rm.indices[i * 3] as usize, rm.indices[i * 3 + 1] as usize, rm.indices[i * 3 + 2] as usize];
        f.map(|k| rm.vertices[k])
    };
    let ntri = rm.indices.len() / 3;
    let mut cum = Vec::with_capacity(ntri);
    let mut total = 0.0f64;
    for i in 0..ntri {
        let [a, b, c] = tri(i);
        let u = [b.position[0] as f64 - a.position[0] as f64, b.position[1] as f64 - a.position[1] as f64, b.position[2] as f64 - a.position[2] as f64];
        let v = [c.position[0] as f64 - a.position[0] as f64, c.position[1] as f64 - a.position[1] as f64, c.position[2] as f64 - a.position[2] as f64];
        let x = [u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0]];
        total += 0.5 * (x[0] * x[0] + x[1] * x[1] + x[2] * x[2]).sqrt();
        cum.push(total);
    }

    // deterministic LCG - no rand dependency, same cloud every run
    let mut state = 0x2545F491_4F6CDD1Du64;
    let mut rnd = move || { state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407); (state >> 33) as f64 / (1u64 << 31) as f64 };

    let mut coords = Vec::with_capacity(count * 3);
    let mut colors = Vec::with_capacity(count * 4);
    let mut normals = Vec::with_capacity(count * 3);
    for _ in 0..count {
        let r = rnd() * total;
        let t = cum.partition_point(|&c| c < r).min(ntri - 1);
        let [va, vb, vc] = tri(t);
        // uniform barycentric via the sqrt trick
        let (mut b1, mut b2) = (rnd(), rnd());
        if b1 + b2 > 1.0 { b1 = 1.0 - b1; b2 = 1.0 - b2; }
        let b0 = 1.0 - b1 - b2;
        for k in 0..3 {
            coords.push(b0 * va.position[k] as f64 + b1 * vb.position[k] as f64 + b2 * vc.position[k] as f64);
            normals.push(b0 * va.normal[k] as f64 + b1 * vb.normal[k] as f64 + b2 * vc.normal[k] as f64);
        }
        // near-white so the lambert term IS the picture
        colors.extend_from_slice(&[235, 230, 220, 255]);
    }

    let mut pc = session_rust::PointCloud::from_coords(coords, colors, normals);
    pc.point_size = 3.0;
    pc.name = "bunny_cloud".to_string();
    let mut s = session_rust::Session::new("bunny_cloud");
    s.add_pointcloud(pc, None);
    s.pb_dump(&out);
    println!("wrote {out}: {count} points with normals");
}