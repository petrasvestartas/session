// Import a Potree 1.x octree (POSITION_CARTESIAN + COLOR_PACKED + NORMAL_OCT16) into one
// kernel PointCloud .pb. Every point lives in exactly ONE node file, so the union of all
// r*.bin files IS the cloud; each node's positions are u32 * scale relative to the NODE's
// bounding box min, and a node's box comes from the root cube by walking the digits of its
// name (bit 4 = +x half, bit 2 = +y half, bit 1 = +z half).
//
// cargo run --example potree_import --target x86_64-unknown-linux-gnu --release -- \
//     assets/lion_src assets/pb/lion.pb 1000

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let dir = a.first().cloned().unwrap_or_else(|| "assets/lion_src".into());
    let out = a.get(1).cloned().unwrap_or_else(|| "assets/pb/lion.pb".into());
    let unit: f64 = a.get(2).and_then(|v| v.parse().ok()).unwrap_or(1000.0); // metres -> mm

    let cloud: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(format!("{dir}/cloud.js")).expect("cloud.js")).expect("json");
    let bb = &cloud["boundingBox"];
    let root_min = [bb["lx"].as_f64().unwrap(), bb["ly"].as_f64().unwrap(), bb["lz"].as_f64().unwrap()];
    let root_size = bb["ux"].as_f64().unwrap() - root_min[0]; // potree root is a cube
    let scale = cloud["scale"].as_f64().unwrap();

    let mut coords = Vec::new();
    let mut colors = Vec::new();
    let mut normals = Vec::new();
    let mut files: Vec<_> = std::fs::read_dir(&dir).unwrap()
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|e| e == "bin"))
        .collect();
    files.sort();
    for path in &files {
        let name = path.file_stem().unwrap().to_str().unwrap(); // "r", "r07", ...
        let (mut min, mut size) = (root_min, root_size);
        for d in name[1..].chars() {
            let i = d.to_digit(10).unwrap();
            size *= 0.5;
            if i & 0b100 != 0 { min[0] += size; }
            if i & 0b010 != 0 { min[1] += size; }
            if i & 0b001 != 0 { min[2] += size; }
        }
        let data = std::fs::read(path).unwrap();
        for rec in data.chunks_exact(18) {
            for k in 0..3 {
                let v = u32::from_le_bytes(rec[k * 4..k * 4 + 4].try_into().unwrap()) as f64;
                coords.push((v * scale + min[k]) * unit);
            }
            colors.extend_from_slice(&[rec[12] as i32, rec[13] as i32, rec[14] as i32, 255]);
            // potree NORMAL_OCT16: two UNSIGNED bytes mapped to [-1,1], octahedral unfold
            let u = rec[16] as f64 / 255.0 * 2.0 - 1.0;
            let v = rec[17] as f64 / 255.0 * 2.0 - 1.0;
            let z = 1.0 - u.abs() - v.abs();
            let (x, y) = if z < 0.0 {
                let s = |t: f64| if t < 0.0 { -1.0 } else { 1.0 };
                ((1.0 - v.abs()) * s(u), (1.0 - u.abs()) * s(v))
            } else { (u, v) };
            let l = (x * x + y * y + z * z).sqrt().max(1e-9);
            normals.extend_from_slice(&[x / l, y / l, z / l]);
        }
    }
    let n = coords.len() / 3;
    let mut pc = session_rust::PointCloud::from_coords(coords, colors, normals);
    pc.point_size = 3.0;
    pc.name = "lion_takanawa".to_string();
    let mut s = session_rust::Session::new("lion_takanawa");
    s.add_pointcloud(pc, None);
    s.pb_dump(&out);
    println!("wrote {out}: {n} points from {} nodes", files.len());
}