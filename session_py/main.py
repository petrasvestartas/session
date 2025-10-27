#!/usr/bin/env python3
"""Main benchmark script matching C++/Rust functionality.

For optimal performance, install Numba for JIT compilation:
    pip install numba

Without Numba, collision detection will be ~200x slower (pure Python loops).
With Numba, performance is comparable to C++/Rust (~10-15ms for 10k boxes).
"""

import time
import random
from src.session_py.point import Point
from src.session_py.line import Line
from src.session_py.plane import Plane
from src.session_py.vector import Vector
from src.session_py.boundingbox import BoundingBox
from src.session_py.bvh import BVH
from src.session_py import intersection
from src.session_py.tolerance import Tolerance

def main():
    print("=== Intersection Examples (Python) ===\n")
    
    # Test data
    l0 = Line(500.0, -573.576, -819.152, 500.0, 573.576, 819.152)
    l1 = Line(13.195, 234.832, 534.315, 986.805, 421.775, 403.416)
    
    # 1. line_line
    p = intersection.line_line(l0, l1, Tolerance.APPROXIMATION)
    if p:
        print(f"1. line_line: {p.x}, {p.y}, {p.z}")
    
    # 2. line_line_parameters
    result = intersection.line_line_parameters(l0, l1, Tolerance.APPROXIMATION)
    if result:
        t0, t1 = result
        print(f"2. line_line_parameters: t0={t0}, t1={t1}")
    
    # 3. plane_plane
    plane_origin_0 = Point(213.787107, 513.797811, -24.743845)
    plane_xaxis_0 = Vector(0.907673, -0.258819, 0.330366)
    plane_yaxis_0 = Vector(0.272094, 0.96225, 0.006285)
    pl0 = Plane(plane_origin_0, plane_xaxis_0, plane_yaxis_0)
    
    plane_origin_1 = Point(247.17924, 499.115486, 59.619568)
    plane_xaxis_1 = Vector(0.552465, 0.816035, 0.16991)
    plane_yaxis_1 = Vector(0.172987, 0.087156, -0.98106)
    pl1 = Plane(plane_origin_1, plane_xaxis_1, plane_yaxis_1)
    
    plane_origin_2 = Point(221.399816, 605.893667, -54.000116)
    plane_xaxis_2 = Vector(0.903451, -0.360516, -0.231957)
    plane_yaxis_2 = Vector(0.172742, -0.189057, 0.966653)
    pl2 = Plane(plane_origin_2, plane_xaxis_2, plane_yaxis_2)
    
    intersection_line = intersection.plane_plane(pl0, pl1)
    if intersection_line:
        print(f"3. plane_plane: {intersection_line}")
    
    # 4. line_plane
    lp = intersection.line_plane(l0, pl0, True)
    if lp:
        print(f"4. line_plane: {lp.x}, {lp.y}, {lp.z}")
    
    # 5. plane_plane_plane
    ppp = intersection.plane_plane_plane(pl0, pl1, pl2)
    if ppp:
        print(f"5. plane_plane_plane: {ppp.x}, {ppp.y}, {ppp.z}")
    
    # 6. ray_box
    min_pt = Point(214, 192, 484)
    max_pt = Point(694, 567, 796)
    bbox = BoundingBox.from_points([min_pt, max_pt])
    intersection_points = intersection.ray_box(l0, bbox, 0.0, 1000.0)
    if intersection_points and len(intersection_points) >= 2:
        print(f"6. ray_box: entry={intersection_points[0]}, exit={intersection_points[1]}")
    
    # 7. ray_sphere
    sphere_center_test = Point(457.0, 192.0, 207.0)
    sphere_points = intersection.ray_sphere(l0, sphere_center_test, 265.0)
    if sphere_points:
        print(f"7. ray_sphere: {len(sphere_points)} hits", end="")
        for i, p in enumerate(sphere_points):
            print(f", p{i}={p}", end="")
        print()
    else:
        print("7. ray_sphere: 0 hits")
    
    # 8. ray_triangle
    tp1 = Point(214, 567, 484)
    tp2 = Point(214, 192, 796)
    tp3 = Point(694, 192, 484)
    tri_hit = intersection.ray_triangle(l0, tp1, tp2, tp3, Tolerance.APPROXIMATION)
    if tri_hit:
        print(f"8. ray_triangle: {tri_hit}")
    
    # 9. ray_mesh - Load bunny mesh
    print("\n9. ray_mesh - Load bunny mesh")
    try:
        from src.session_py.mesh import Mesh
        from src.session_py.obj import read_obj
        
        bunny = None
        try_paths = ["../data/bunny.obj", "../../data/bunny.obj", "data/bunny.obj"]
        for path in try_paths:
            try:
                bunny = read_obj(path)
                break
            except Exception:
                continue
        
        if bunny:
            print(f"Bunny: {bunny.number_of_vertices()} vertices, {bunny.number_of_faces()} faces")
            
            # Build triangle BVH
            bvh_build_start = time.perf_counter()
            bunny.build_triangle_bvh()
            bvh_build_time = (time.perf_counter() - bvh_build_start) * 1000.0
            print(f"BVH build: {bvh_build_time:.3f} ms")
            
            zaxis = Line(0.201, -0.212, 0.036, -0.326, 0.677, -0.060)
            
            # Brute force test
            brute_start = time.perf_counter()
            mesh_hits = intersection.ray_mesh(zaxis, bunny, Tolerance.APPROXIMATION, True)
            brute_time = (time.perf_counter() - brute_start) * 1000.0
            print(f"Ray-mesh (brute): {len(mesh_hits)} hits, {brute_time:.3f} ms")
            
            # BVH accelerated test
            bvh_start = time.perf_counter()
            bvh_hits = intersection.ray_mesh_bvh(zaxis, bunny, Tolerance.APPROXIMATION, True)
            bvh_time = (time.perf_counter() - bvh_start) * 1000.0
            
            speedup = brute_time / bvh_time if bvh_time > 0 else 0
            print(f"Ray-mesh (BVH):   {len(bvh_hits)} hits, {bvh_time:.3f} ms", end="")
            if speedup > 0:
                print(f" ({speedup:.2f}x faster)")
            else:
                print()
        else:
            print("ERROR: Cannot find bunny.obj in ../data/ or ../../data/ or data/")
    except Exception as e:
        print(f"Skipping ray_mesh test: {e}")
    
    print("\n=== BVH Collision Detection (Python) ===")
    
    # Test with different box counts to compare with C++/Rust
    box_counts = [100, 5000, 10000]
    
    for box_count in box_counts:
        # Create random boxes
        boxes = []
        WORLD_SIZE = 100.0
        MIN_SIZE = 5.0
        MAX_SIZE = 10.0
        
        random.seed(42)  # Fixed seed for consistency with C++/Rust
        for i in range(box_count):
            # Random position
            x = (random.random() - 0.5) * WORLD_SIZE
            y = (random.random() - 0.5) * WORLD_SIZE
            z = (random.random() - 0.5) * WORLD_SIZE
            
            # Random box size
            w = MIN_SIZE + random.random() * (MAX_SIZE - MIN_SIZE)
            h = MIN_SIZE + random.random() * (MAX_SIZE - MIN_SIZE)
            d = MIN_SIZE + random.random() * (MAX_SIZE - MIN_SIZE)
            
            center = Point(x, y, z)
            half_size = Vector(w * 0.5, h * 0.5, d * 0.5)
            boxes.append(BoundingBox(center, Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1), half_size))
        
        # Build BVH and time it
        bvh_start = time.perf_counter()
        bvh = BVH.from_boxes(boxes, WORLD_SIZE)
        bvh_build_time = (time.perf_counter() - bvh_start) * 1000.0
        
        coll_start = time.perf_counter()
        pairs, colliding_indices, checks = bvh.check_all_collisions(boxes)
        coll_time = (time.perf_counter() - coll_start) * 1000.0
        
        print(f"{box_count} boxes: build={bvh_build_time:.3f}ms, collisions={coll_time:.3f}ms ({len(pairs)} pairs, {checks} checks)")
    
    print("\n\n=== Comprehensive 10k Mixed Geometry Test (Python) ===")
    
    try:
        from src.session_py.session import Session
        from src.session_py.mesh import Mesh
        from src.session_py.polyline import Polyline
        
        OBJECT_COUNT = 10000
        WORLD_SIZE = 50.0
        
        scene = Session("comprehensive_test")
        aabb_boxes = []
        
        random.seed(42)
        
        print(f"Creating {OBJECT_COUNT} mixed geometry objects...")
        
        for i in range(OBJECT_COUNT):
            x = (random.random() - 0.5) * WORLD_SIZE
            y = (random.random() - 0.5) * WORLD_SIZE
            z = (random.random() - 0.5) * WORLD_SIZE
            
            geom_type = i % 7
            
            if geom_type == 0:
                # Point
                pt = Point(x, y, z)
                scene.add_point(pt)
                aabb_boxes.append(BoundingBox.from_point(pt, 0.1))
            elif geom_type == 1:
                # Line
                dx = (random.random() - 0.5) * 5.0
                dy = (random.random() - 0.5) * 5.0
                dz = (random.random() - 0.5) * 5.0
                line = Line.from_points(Point(x, y, z), Point(x + dx, y + dy, z + dz))
                scene.add_line(line)
                aabb_boxes.append(BoundingBox.from_line(line, 0.1))
            elif geom_type == 2:
                # Plane
                plane = Plane(Point(x, y, z), Vector(1, 0, 0), Vector(0, 1, 0))
                scene.add_plane(plane)
                aabb_boxes.append(BoundingBox.from_plane(plane, 2.0, 2.0, 0.1))
            elif geom_type == 3:
                # Polyline
                num_pts = 3 + (i % 5)
                pts = []
                for j in range(num_pts):
                    jx = x + (random.random() - 0.5) * 3.0
                    jy = y + (random.random() - 0.5) * 3.0
                    jz = z + (random.random() - 0.5) * 3.0
                    pts.append(Point(jx, jy, jz))
                poly = Polyline(pts)
                scene.add_polyline(poly)
                aabb_boxes.append(BoundingBox.from_polyline(poly, inflate=0.1))
            elif geom_type == 4:
                # Mesh
                mesh = Mesh()
                h = 0.5
                verts = [
                    Point(x-h, y-h, z-h), Point(x+h, y-h, z-h),
                    Point(x+h, y+h, z-h), Point(x-h, y+h, z-h),
                    Point(x-h, y-h, z+h), Point(x+h, y-h, z+h),
                    Point(x+h, y+h, z+h), Point(x-h, y+h, z+h)
                ]
                for vi, v in enumerate(verts):
                    mesh.add_vertex(v, vi)
                mesh.add_face([0,1,2,3])
                mesh.add_face([4,7,6,5])
                scene.add_mesh(mesh)
                aabb_boxes.append(BoundingBox.from_mesh(mesh, inflate=0.1))
            elif geom_type == 5:
                # Cylinder
                from src.session_py.cylinder import Cylinder
                cyl_line = Line.from_points(Point(x - 1, y, z), Point(x + 1, y, z))
                cyl = Cylinder(cyl_line, 0.3)
                scene.add_cylinder(cyl)
                aabb_boxes.append(BoundingBox.from_cylinder(cyl, inflate=0.1))
            elif geom_type == 6:
                # Arrow
                from src.session_py.arrow import Arrow
                arrow_line = Line.from_points(Point(x - 1, y, z), Point(x + 1, y, z))
                arrow = Arrow(arrow_line, 0.3)
                scene.add_arrow(arrow)
                aabb_boxes.append(BoundingBox.from_arrow(arrow, inflate=0.1))
            else:
                # Point
                pt = Point(x, y, z)
                scene.add_point(pt)
                aabb_boxes.append(BoundingBox.from_point(pt, 0.1))
        
        print("\n(a) AABB BVH Collision Detection:")
        aabb_start = time.perf_counter()
        aabb_bvh = BVH.from_boxes(aabb_boxes, WORLD_SIZE)
        aabb_collisions, aabb_indices, aabb_checks = aabb_bvh.check_all_collisions(aabb_boxes)
        aabb_time = (time.perf_counter() - aabb_start) * 1000.0
        
        print(f"  Build + query: {aabb_time:.3f}ms")
        print(f"  Collision pairs: {len(aabb_collisions)}")
        
        print("\n(b) Ray BVH Intersection:")
        ray_origin = Point(0, 0, 0)
        ray_dir = Vector(1, 0, 0)
        
        ray_start = time.perf_counter()
        ray_candidates = []
        aabb_bvh.ray_cast(ray_origin, ray_dir, ray_candidates, True)
        ray_time = (time.perf_counter() - ray_start) * 1000.0
        
        print(f"  Query: {ray_time:.3f}ms")
        print(f"  Candidates: {len(ray_candidates)}")
        
    except Exception as e:
        print(f"Skipping comprehensive test: {e}")
    
    print("\n=== Session Ray Casting (Python) ===")
    
    try:
        scene = Session("ray_test")
        
        # Add various geometry along X axis
        pt1 = Point(5, 0, 0)
        pt1.name = "point_at_5"
        pt1_guid = pt1.guid
        scene.add_point(pt1)
        
        pt2 = Point(15, 0, 0)
        pt2.name = "point_at_15"
        pt2_guid = pt2.guid
        scene.add_point(pt2)
        
        line1 = Line.from_points(Point(10, -2, 0), Point(10, 2, 0))
        line1.name = "vertical_line_at_10"
        line1_guid = line1.guid
        scene.add_line(line1)
        
        plane1 = Plane(Point(20, 0, 0), Vector(1, 0, 0), Vector(0, 1, 0))
        plane1.name = "plane_at_20"
        plane1_guid = plane1.guid
        scene.add_plane(plane1)
        
        poly_pts = [
            Point(25, -1, -1),
            Point(25, 0, 0),
            Point(25, 1, 1)
        ]
        poly1 = Polyline(poly_pts)
        poly1.name = "polyline_at_25"
        poly1_guid = poly1.guid
        scene.add_polyline(poly1)
        
        ray_origin = Point(0, 0, 0)
        ray_direction = Vector(1, 0, 0)
        tolerance = 0.5
        
        hits = scene.ray_cast(ray_origin, ray_direction, tolerance)
        
        print(f"{len(hits)} hit(s):")
        for hit in hits:
            name = "unknown"
            if hit.guid == pt1_guid:
                name = pt1.name
            elif hit.guid == pt2_guid:
                name = pt2.name
            elif hit.guid == line1_guid:
                name = line1.name
            elif hit.guid == plane1_guid:
                name = plane1.name
            elif hit.guid == poly1_guid:
                name = poly1.name
            print(f"  {name} (dist={hit.distance:.3f})")
    except Exception as e:
        print(f"Session ray casting error: {e}")
    
    print("\n=== All Geometry Types Test (Python) ===")
    
    try:
        from src.session_py.cylinder import Cylinder
        from src.session_py.arrow import Arrow
        
        scene = Session("comprehensive_test")
        
        # Add all geometry types along Y axis
        pt = Point(0, 10, 0)
        pt.name = "point_10"
        pt_guid = pt.guid
        scene.add_point(pt)
        
        line = Line.from_points(Point(-1, 20, 0), Point(1, 20, 0))
        line.name = "line_20"
        line_guid = line.guid
        scene.add_line(line)
        
        plane = Plane(Point(0, 30, 0), Vector(1, 0, 0), Vector(0, 0, 1))
        plane.name = "plane_30"
        plane_guid = plane.guid
        scene.add_plane(plane)
        
        bbox = BoundingBox(
            Point(0, 40, 0),
            Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1),
            Vector(2, 2, 2)
        )
        bbox.name = "bbox_40"
        bbox_guid = bbox.guid
        scene.add_bbox(bbox)
        
        cyl_line = Line.from_points(Point(-1, 50, 0), Point(1, 50, 0))
        cyl = Cylinder(cyl_line, 1.0)
        cyl.name = "cylinder_50"
        cyl_guid = cyl.guid
        scene.add_cylinder(cyl)
        
        arrow_line = Line.from_points(Point(-1, 60, 0), Point(1, 60, 0))
        arrow = Arrow(arrow_line, 1.0)
        arrow.name = "arrow_60"
        arrow_guid = arrow.guid
        scene.add_arrow(arrow)
        
        poly_pts = [
            Point(-1, 70, 0),
            Point(0, 70, 0),
            Point(1, 70, 0)
        ]
        poly = Polyline(poly_pts)
        poly.name = "polyline_70"
        poly_guid = poly.guid
        scene.add_polyline(poly)
        
        ray_origin = Point(0, 0, 0)
        ray_dir = Vector(0, 1, 0)
        tolerance = 1.0
        
        hits = scene.ray_cast(ray_origin, ray_dir, tolerance)
        
        print(f"{len(hits)} hit(s):")
        for hit in hits:
            name = "unknown"
            if hit.guid == pt_guid:
                name = pt.name
            elif hit.guid == line_guid:
                name = line.name
            elif hit.guid == plane_guid:
                name = plane.name
            elif hit.guid == bbox_guid:
                name = bbox.name
            elif hit.guid == cyl_guid:
                name = cyl.name
            elif hit.guid == arrow_guid:
                name = arrow.name
            elif hit.guid == poly_guid:
                name = poly.name
            print(f"  {name} (dist={hit.distance:.3f})")
    except Exception as e:
        print(f"All geometry types test error: {e}")
    
    print("\n=== Performance Test (10k Objects) (Python) ===")
    
    try:
        OBJECT_COUNT = 10000
        WORLD_SIZE = 100.0
        
        scene = Session("perf_test")
        pure_boxes = []
        
        random.seed(42)
        for i in range(OBJECT_COUNT):
            x = (random.random() - 0.5) * WORLD_SIZE
            y = (random.random() - 0.5) * WORLD_SIZE
            z = (random.random() - 0.5) * WORLD_SIZE
            
            pt = Point(x, y, z)
            pt.name = f"point_{i}"
            scene.add_point(pt)
            
            # Also create AABB for pure BVH test
            pure_boxes.append(BoundingBox(
                Point(x, y, z),
                Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1),
                Vector(0.5, 0.5, 0.5)
            ))
        
        ray_origin = Point(0, 0, 0)
        ray_dir_x = Vector(1, 0, 0)
        ray_dir_y = Vector(0, 1, 0)
        tolerance = 1.0
        
        # Session first call
        t0 = time.perf_counter()
        hits0 = scene.ray_cast(ray_origin, ray_dir_x, tolerance)
        session_ms = (time.perf_counter() - t0) * 1000.0
        
        # Session cached call
        t1 = time.perf_counter()
        hits1 = scene.ray_cast(ray_origin, ray_dir_y, tolerance)
        session_cached_ms = (time.perf_counter() - t1) * 1000.0
        
        # Pure BVH
        bvh_start = time.perf_counter()
        pure_bvh = BVH.from_boxes(pure_boxes, WORLD_SIZE)
        candidate_ids = []
        pure_bvh.ray_cast(ray_origin, ray_dir_x, candidate_ids, find_all=True)
        bvh_ms = (time.perf_counter() - bvh_start) * 1000.0
        
        speedup = session_ms / session_cached_ms if session_cached_ms > 0 else 0
        
        print(f"Session (first):  {session_ms:.3f}ms ({len(hits0)} hits)")
        print(f"Session (cached): {session_cached_ms:.3f}ms ({len(hits1)} hits, {speedup:.2f}x faster)")
        print(f"Pure BVH:         {bvh_ms:.3f}ms ({len(candidate_ids)} candidates)")
    except Exception as e:
        print(f"Performance test error: {e}")

if __name__ == "__main__":
    main()
