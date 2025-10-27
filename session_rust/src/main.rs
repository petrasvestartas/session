use session_rust::{
    read_obj, BoundingBox, Line, Mesh, Plane, Point, Session, Tolerance, Vector, BVH,
};
use std::path::Path;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Intersection Examples (Rust) ===");

    let l0 = Line::new(500.0, -573.576, -819.152, 500.0, 573.576, 819.152);
    let l1 = Line::new(13.195, 234.832, 534.315, 986.805, 421.775, 403.416);

    if let Some(p) = session_rust::intersection::line_line(&l0, &l1, Tolerance::APPROXIMATION) {
        println!("1. line_line: {}, {}, {}", p.x(), p.y(), p.z());
    }

    if let Some((t0, t1)) = session_rust::intersection::line_line_parameters(
        &l0,
        &l1,
        Tolerance::APPROXIMATION,
        true,
        false,
    ) {
        println!("2. line_line_parameters: t0={t0}, t1={t1}");
    }

    let plane_origin_0 = Point::new(213.787_107, 513.797_811, -24.743_845);
    let plane_xaxis_0 = Vector::new(0.907_673, -0.258_819, 0.330_366);
    let plane_yaxis_0 = Vector::new(0.272_094, 0.962_25, 0.006_285);
    let pl0 = Plane::new(plane_origin_0, plane_xaxis_0, plane_yaxis_0);

    let plane_origin_1 = Point::new(247.179_24, 499.115_486, 59.619_568);
    let plane_xaxis_1 = Vector::new(0.552_465, 0.816_035, 0.169_91);
    let plane_yaxis_1 = Vector::new(0.172_987, 0.087_156, -0.981_06);
    let pl1 = Plane::new(plane_origin_1, plane_xaxis_1, plane_yaxis_1);

    let plane_origin_2 = Point::new(221.399_816, 605.893_667, -54.000_116);
    let plane_xaxis_2 = Vector::new(0.903_451, -0.360_516, -0.231_957);
    let plane_yaxis_2 = Vector::new(0.172_742, -0.189_057, 0.966_653);
    let pl2 = Plane::new(plane_origin_2, plane_xaxis_2, plane_yaxis_2);

    if let Some(intersection_line) = session_rust::intersection::plane_plane(&pl0, &pl1) {
        println!("3. plane_plane: {intersection_line}");
    }

    if let Some(lp) = session_rust::intersection::line_plane(&l0, &pl0, true) {
        println!("4. line_plane: {}, {}, {}", lp.x(), lp.y(), lp.z());
    }

    if let Some(ppp) = session_rust::intersection::plane_plane_plane(&pl0, &pl1, &pl2) {
        println!(
            "5. plane_plane_plane: {}, {}, {}",
            ppp.x(),
            ppp.y(),
            ppp.z()
        );
    }

    let min = Point::new(214.0, 192.0, 484.0);
    let max = Point::new(694.0, 567.0, 796.0);
    let pts = vec![min.clone(), max.clone()];
    let bbox = BoundingBox::from_points(&pts, 0.0);
    if let Some(intersection_points) = session_rust::intersection::ray_box(&l0, &bbox, 0.0, 1000.0)
    {
        if intersection_points.len() >= 2 {
            println!(
                "6. ray_box: entry={}, exit={}",
                intersection_points[0], intersection_points[1]
            );
        }
    }

    let sphere_center_test = Point::new(457.0, 192.0, 207.0);
    if let Some(sphere_points) =
        session_rust::intersection::ray_sphere(&l0, &sphere_center_test, 265.0)
    {
        print!("7. ray_sphere: {} hits", sphere_points.len());
        for (i, p) in sphere_points.iter().enumerate() {
            print!(", p{i}={p}");
        }
        println!();
    } else {
        println!("7. ray_sphere: 0 hits");
    }

    let tp1 = Point::new(214.0, 567.0, 484.0);
    let tp2 = Point::new(214.0, 192.0, 796.0);
    let tp3 = Point::new(694.0, 192.0, 484.0);
    if let Some(tri_hit) =
        session_rust::intersection::ray_triangle(&l0, &tp1, &tp2, &tp3, Tolerance::APPROXIMATION)
    {
        println!("8. ray_triangle: {tri_hit}");
    }

    println!("\n9. ray_mesh - Load bunny mesh");
    let mut bunny_opt: Option<Mesh> = None;
    let try_paths = [
        "../data/bunny.obj",
        "../../data/bunny.obj",
        "data/bunny.obj",
    ];
    for p in try_paths.iter() {
        if Path::new(p).exists() {
            if let Ok(m) = read_obj(p) {
                bunny_opt = Some(m);
                break;
            }
        }
    }
    if let Some(bunny) = bunny_opt {
        println!(
            "Bunny: {} vertices, {} faces",
            bunny.number_of_vertices(),
            bunny.number_of_faces()
        );

        let (vertices, faces) = bunny.to_vertices_and_faces();
        let tri_build_start = Instant::now();
        let mut tris: Vec<[usize; 3]> = Vec::new();
        let mut tri_boxes: Vec<BoundingBox> = Vec::new();
        for face in faces.iter() {
            if face.len() >= 3 {
                let v0 = face[0];
                for i in 1..(face.len() - 1) {
                    let t = [v0, face[i], face[i + 1]];
                    tris.push(t);
                    let pts = [
                        vertices[t[0]].clone(),
                        vertices[t[1]].clone(),
                        vertices[t[2]].clone(),
                    ];
                    tri_boxes.push(BoundingBox::from_points(&pts, 0.0));
                }
            }
        }
        let world_size = BVH::compute_world_size(&tri_boxes);
        let tri_bvh = BVH::from_boxes(&tri_boxes, world_size);
        let tri_build_end = Instant::now();
        let bvh_build_time_ms = (tri_build_end - tri_build_start).as_secs_f64() * 1000.0;
        println!("BVH build: {bvh_build_time_ms:.3} ms");

        let zaxis = Line::new(0.201, -0.212, 0.036, -0.326, 0.677, -0.060);

        let brute_start = Instant::now();
        let mut mesh_hits = 0usize;
        for t in tris.iter() {
            let v0 = &vertices[t[0]];
            let v1 = &vertices[t[1]];
            let v2 = &vertices[t[2]];
            if session_rust::intersection::ray_triangle(
                &zaxis,
                v0,
                v1,
                v2,
                Tolerance::ZERO_TOLERANCE,
            )
            .is_some()
            {
                mesh_hits += 1;
            }
        }
        let brute_end = Instant::now();
        let mesh_time_ms = (brute_end - brute_start).as_secs_f64() * 1000.0;
        println!("Ray-mesh (brute): {mesh_hits} hits, {mesh_time_ms:.3} ms");

        let bvh_start = Instant::now();
        let mut candidate_ids: Vec<usize> = Vec::new();
        let origin = zaxis.start();
        let dir = zaxis.to_vector();
        let len = dir.compute_length();
        let dir_unit = Vector::new(dir.x() / len, dir.y() / len, dir.z() / len);
        tri_bvh.ray_cast(&origin, &dir_unit, &mut candidate_ids, true);
        let mut bvh_hits = 0usize;
        for idx in candidate_ids.iter() {
            let t = tris[*idx];
            let v0 = &vertices[t[0]];
            let v1 = &vertices[t[1]];
            let v2 = &vertices[t[2]];
            if session_rust::intersection::ray_triangle(
                &zaxis,
                v0,
                v1,
                v2,
                Tolerance::ZERO_TOLERANCE,
            )
            .is_some()
            {
                bvh_hits += 1;
            }
        }
        let bvh_end = Instant::now();
        let bvh_time_ms = (bvh_end - bvh_start).as_secs_f64() * 1000.0;
        print!("Ray-mesh (BVH):   {bvh_hits} hits, {bvh_time_ms:.3} ms");
        if bvh_time_ms > 0.0 && mesh_time_ms > 0.0 {
            println!(" ({:.2}x faster)", mesh_time_ms / bvh_time_ms);
        } else {
            println!();
        }
    } else {
        println!("ERROR: Cannot find bunny.obj in ../data/ or ../../data/ or data/");
    }

    println!("\n=== BVH Collision Detection (Rust) ===");
    let box_counts = [100usize, 5000usize, 10000usize];
    for &box_count in box_counts.iter() {
        let world_size = 100.0f64;
        let min_size = 5.0f64;
        let max_size = 10.0f64;
        unsafe { libc::srand(42) }; // match C++ seeding per dataset
        let rand_max = 2147483647.0f64; // typical RAND_MAX on macOS
        let next_rand01 = || -> f64 { unsafe { (libc::rand() as i64) as f64 / rand_max } };
        let mut boxes: Vec<BoundingBox> = Vec::with_capacity(box_count);
        for _ in 0..box_count {
            let x = (next_rand01() - 0.5) * world_size;
            let y = (next_rand01() - 0.5) * world_size;
            let z = (next_rand01() - 0.5) * world_size;
            let w = min_size + next_rand01() * (max_size - min_size);
            let h = min_size + next_rand01() * (max_size - min_size);
            let d = min_size + next_rand01() * (max_size - min_size);
            let center = Point::new(x, y, z);
            let half = Vector::new(w * 0.5, h * 0.5, d * 0.5);
            boxes.push(BoundingBox::new(
                center,
                Vector::new(1.0, 0.0, 0.0),
                Vector::new(0.0, 1.0, 0.0),
                Vector::new(0.0, 0.0, 1.0),
                half,
            ));
        }
        let bvh_start = Instant::now();
        let bvh = BVH::from_boxes(&boxes, world_size);
        let bvh_end = Instant::now();
        let bvh_build_ms = (bvh_end - bvh_start).as_secs_f64() * 1000.0;
        let coll_start = Instant::now();
        let (pairs, _indices, checks) = bvh.check_all_collisions(&boxes);
        let coll_end = Instant::now();
        let coll_ms = (coll_end - coll_start).as_secs_f64() * 1000.0;
        println!(
            "{} boxes: build={:.3}ms, collisions={:.3}ms ({} pairs, {} checks)",
            box_count,
            bvh_build_ms,
            coll_ms,
            pairs.len(),
            checks
        );
    }

    println!("\n=== Session Ray Casting (Rust) ===");
    {
        let mut scene = Session::new("ray_test");
        let mut pt1 = Point::new(5.0, 0.0, 0.0);
        pt1.name = "point_at_5".to_string();
        let pt1_guid = pt1.guid.clone();
        scene.add_point(pt1.clone());

        let mut pt2 = Point::new(15.0, 0.0, 0.0);
        pt2.name = "point_at_15".to_string();
        let pt2_guid = pt2.guid.clone();
        scene.add_point(pt2.clone());

        let mut line1 =
            Line::from_points(&Point::new(10.0, -2.0, 0.0), &Point::new(10.0, 2.0, 0.0));
        line1.name = "vertical_line_at_10".to_string();
        let line1_guid = line1.guid.clone();
        scene.add_line(line1.clone());

        let mut plane1 = Plane::new(
            Point::new(20.0, 0.0, 0.0),
            Vector::new(1.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
        );
        plane1.name = "plane_at_20".to_string();
        let plane1_guid = plane1.guid.clone();
        scene.add_plane(plane1.clone());

        let poly_pts = vec![
            Point::new(25.0, -1.0, -1.0),
            Point::new(25.0, 0.0, 0.0),
            Point::new(25.0, 1.0, 1.0),
        ];
        let mut polyline1 = session_rust::Polyline::new(poly_pts);
        polyline1.name = "polyline_at_25".to_string();
        let polyline1_guid = polyline1.guid.clone();
        scene.add_polyline(polyline1.clone());

        let ray_origin = Point::new(0.0, 0.0, 0.0);
        let ray_direction = Vector::new(1.0, 0.0, 0.0);
        let tolerance = 0.5;
        let hits = scene.ray_cast(&ray_origin, &ray_direction, tolerance);
        println!("{} hit(s):", hits.len());
        for h in hits.iter() {
            let name = if h.guid == pt1_guid {
                pt1.name.clone()
            } else if h.guid == pt2_guid {
                pt2.name.clone()
            } else if h.guid == line1_guid {
                line1.name.clone()
            } else if h.guid == plane1_guid {
                plane1.name.clone()
            } else if h.guid == polyline1_guid {
                polyline1.name.clone()
            } else {
                "unknown".to_string()
            };
            println!("  {} (dist={})", name, h.distance);
        }
    }

    println!("\n=== Performance Test (10k Objects) (Rust) ===");
    {
        let object_count = 10_000usize;
        let world_size = 100.0f64;
        let mut scene = Session::new("perf_test");
        let mut pure_boxes: Vec<BoundingBox> = Vec::with_capacity(object_count);
        unsafe { libc::srand(42) }; // match C++
        let rand_max = 2147483647.0f64;
        for i in 0..object_count {
            let x = (unsafe { libc::rand() } as f64 / rand_max - 0.5) * world_size;
            let y = (unsafe { libc::rand() } as f64 / rand_max - 0.5) * world_size;
            let z = (unsafe { libc::rand() } as f64 / rand_max - 0.5) * world_size;
            let mut pt = Point::new(x, y, z);
            pt.name = format!("point_{i}");
            scene.add_point(pt.clone());
            pure_boxes.push(BoundingBox::new(
                Point::new(x, y, z),
                Vector::new(1.0, 0.0, 0.0),
                Vector::new(0.0, 1.0, 0.0),
                Vector::new(0.0, 0.0, 1.0),
                Vector::new(0.5, 0.5, 0.5),
            ));
        }
        let ray_origin = Point::new(0.0, 0.0, 0.0);
        let ray_dir_x = Vector::new(1.0, 0.0, 0.0);
        let ray_dir_y = Vector::new(0.0, 1.0, 0.0);
        let tol = 1.0;
        let t0 = Instant::now();
        let hits0 = scene.ray_cast(&ray_origin, &ray_dir_x, tol);
        let session_ms = (Instant::now() - t0).as_secs_f64() * 1000.0;
        let t1 = Instant::now();
        let hits1 = scene.ray_cast(&ray_origin, &ray_dir_y, tol);
        let session_cached_ms = (Instant::now() - t1).as_secs_f64() * 1000.0;
        let bvh_start = Instant::now();
        let pure_bvh = BVH::from_boxes(&pure_boxes, world_size);
        let mut candidate_ids: Vec<usize> = Vec::new();
        pure_bvh.ray_cast(&ray_origin, &ray_dir_x, &mut candidate_ids, true);
        let bvh_ms = (Instant::now() - bvh_start).as_secs_f64() * 1000.0;
        println!(
            "Session (first):  {:.3}ms ({} hits)",
            session_ms,
            hits0.len()
        );
        println!(
            "Session (cached): {:.3}ms ({} hits, {:.2}x faster)",
            session_cached_ms,
            hits1.len(),
            if session_cached_ms > 0.0 {
                session_ms / session_cached_ms
            } else {
                0.0
            }
        );
        println!(
            "Pure BVH:         {:.3}ms ({} candidates)",
            bvh_ms,
            candidate_ids.len()
        );
    }

    Ok(())
}
