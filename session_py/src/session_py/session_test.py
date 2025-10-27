from .session import Session
from .point import Point
from .line import Line
from .plane import Plane
from .boundingbox import BoundingBox
from .polyline import Polyline
from .pointcloud import PointCloud
from .mesh import Mesh
from .cylinder import Cylinder
from .arrow import Arrow
from .vector import Vector


def test_session_serialization_with_all_geometry_types():
    from pathlib import Path
    from .encoders import json_dump, json_load
    from .treenode import TreeNode

    my_session = Session("test_session")

    # Create all geometry types (in specified order)
    arrow = Arrow(Line(0.0, 0.0, 0.0, 1.0, 0.0, 0.0), 0.1)
    bbox = BoundingBox.from_point(Point(0.0, 0.0, 0.0), 1.0)
    # color - not a geometry type that can be added to session
    cylinder = Cylinder(Line(0.0, 0.0, 0.0, 0.0, 0.0, 1.0), 0.5)
    line = Line(0.0, 0.0, 0.0, 1.0, 1.0, 1.0)
    mesh = Mesh()
    plane = Plane.from_point_normal(Point(0.0, 0.0, 0.0), Vector(0.0, 0.0, 1.0))
    point = Point(1.0, 2.0, 3.0)
    pointcloud = PointCloud([Point(0.0, 0.0, 0.0)], [], [])
    polyline = Polyline([Point(0.0, 0.0, 0.0), Point(1.0, 0.0, 0.0)])

    # Demonstrate 3-level tree hierarchy
    # Level 1: Root -> "geometry" folder
    geometry_folder = TreeNode("geometry")
    my_session.add(geometry_folder)  # defaults to root

    # Level 2: "geometry" -> "primitives" and "complex" folders
    primitives_folder = TreeNode("primitives")
    complex_folder = TreeNode("complex")
    my_session.add(primitives_folder, geometry_folder)
    my_session.add(complex_folder, geometry_folder)

    # Add all geometry to session - returns TreeNode for easy nesting!
    arrow_node = my_session.add_arrow(arrow)
    bbox_node = my_session.add_bbox(bbox)
    cylinder_node = my_session.add_cylinder(cylinder)
    line_node = my_session.add_line(line)
    mesh_node = my_session.add_mesh(mesh)
    plane_node = my_session.add_plane(plane)
    point_node = my_session.add_point(point)
    pointcloud_node = my_session.add_pointcloud(pointcloud)
    polyline_node = my_session.add_polyline(polyline)

    # Level 3: Organize geometry under folders
    # Primitives: point, line, plane
    my_session.add(point_node, primitives_folder)
    my_session.add(line_node, primitives_folder)
    my_session.add(plane_node, primitives_folder)

    # Complex: mesh, polyline, pointcloud, bbox, cylinder, arrow
    my_session.add(mesh_node, complex_folder)
    my_session.add(polyline_node, complex_folder)
    my_session.add(pointcloud_node, complex_folder)
    my_session.add(bbox_node, complex_folder)
    my_session.add(cylinder_node, complex_folder)
    my_session.add(arrow_node, complex_folder)

    # Add some edges between objects
    my_session.add_edge(point.guid, line.guid, "point_to_line")
    my_session.add_edge(line.guid, plane.guid, "line_to_plane")

    # Graph structure before serialization
    original_graph_vertices = my_session.graph.number_of_vertices()
    original_graph_edges = my_session.graph.number_of_edges()
    assert original_graph_vertices == 9
    assert original_graph_edges == 2

    # Tree should have: root + geometry + primitives + complex + 9 geometry nodes = 13 nodes
    original_tree_nodes = list(my_session.tree.nodes)
    assert len(original_tree_nodes) == 13

    filepath = Path(__file__).resolve().parents[2] / "test_session.json"
    json_dump(my_session, filepath)
    loaded = json_load(filepath)

    assert loaded.name == my_session.name
    assert len(loaded.objects.arrows) == len(my_session.objects.arrows)
    assert len(loaded.objects.bboxes) == len(my_session.objects.bboxes)
    assert len(loaded.objects.cylinders) == len(my_session.objects.cylinders)
    assert len(loaded.objects.lines) == len(my_session.objects.lines)
    assert len(loaded.objects.meshes) == len(my_session.objects.meshes)
    assert len(loaded.objects.planes) == len(my_session.objects.planes)
    assert len(loaded.objects.points) == len(my_session.objects.points)
    assert len(loaded.objects.pointclouds) == len(my_session.objects.pointclouds)
    assert len(loaded.objects.polylines) == len(my_session.objects.polylines)
    assert len(loaded.lookup) == len(my_session.lookup)

    # Verify graph structure is fully preserved
    assert loaded.graph.number_of_vertices() == original_graph_vertices
    assert loaded.graph.number_of_edges() == original_graph_edges
    assert loaded.graph.has_edge((point.guid, line.guid))
    assert loaded.graph.has_edge((line.guid, plane.guid))

    # Verify tree structure is preserved
    loaded_tree_nodes = list(loaded.tree.nodes)
    assert len(loaded_tree_nodes) == len(original_tree_nodes)
    assert loaded.tree.root is not None


def test_session_get_geometry_with_transformations():
    from .xform import Xform

    session = Session("transform_test")

    # Create a simple hierarchy with transformations
    # Root -> parent_node -> child_node

    # Create two points
    parent_point = Point(1.0, 0.0, 0.0)
    parent_point.xform = Xform.translation(10.0, 0.0, 0.0)  # Translate by (10, 0, 0)

    child_point = Point(1.0, 0.0, 0.0)
    child_point.xform = Xform.translation(5.0, 0.0, 0.0)  # Translate by (5, 0, 0)

    # Add to session
    parent_node = session.add_point(parent_point)
    child_node = session.add_point(child_point)

    # Create hierarchy: root -> parent -> child
    session.add(parent_node)
    session.add(child_node, parent_node)

    # Get transformed geometry
    transformed = session.get_geometry()

    # Should have 2 points
    assert len(transformed.points) == 2

    # Find parent and child in transformed objects
    parent_transformed = next(
        p for p in transformed.points if p.guid == parent_point.guid
    )
    child_transformed = next(
        p for p in transformed.points if p.guid == child_point.guid
    )

    # Parent should be transformed to world coordinates
    # Original: (1, 0, 0) + translation(10, 0, 0) = (11, 0, 0)
    assert abs(parent_transformed.x - 11.0) < 1e-6
    assert abs(parent_transformed.y - 0.0) < 1e-6
    assert abs(parent_transformed.z - 0.0) < 1e-6

    # Child should have composed transformation applied
    # Original: (1, 0, 0) + parent_translation(10, 0, 0) + child_translation(5, 0, 0) = (16, 0, 0)
    assert abs(child_transformed.x - 16.0) < 1e-6
    assert abs(child_transformed.y - 0.0) < 1e-6
    assert abs(child_transformed.z - 0.0) < 1e-6

    # Transformations should be reset to identity (check translation components are 0)
    assert abs(parent_transformed.xform.m[12]) < 1e-6
    assert abs(parent_transformed.xform.m[13]) < 1e-6
    assert abs(parent_transformed.xform.m[14]) < 1e-6
    assert abs(child_transformed.xform.m[12]) < 1e-6
    assert abs(child_transformed.xform.m[13]) < 1e-6
    assert abs(child_transformed.xform.m[14]) < 1e-6


def test_session_tree_transformation_hierarchy():
    """Test tree transformation hierarchy with 3 boxes (matching C++ test)."""
    from .mesh import Mesh
    from .xform import Xform
    import math

    scene = Session("tree_transformation_test")

    # Helper to create box mesh
    def create_box(center, size):
        mesh = Mesh()
        h = size * 0.5
        verts = [
            Point(center.x - h, center.y - h, center.z - h),
            Point(center.x + h, center.y - h, center.z - h),
            Point(center.x + h, center.y + h, center.z - h),
            Point(center.x - h, center.y + h, center.z - h),
            Point(center.x - h, center.y - h, center.z + h),
            Point(center.x + h, center.y - h, center.z + h),
            Point(center.x + h, center.y + h, center.z + h),
            Point(center.x - h, center.y + h, center.z + h),
        ]
        for i, v in enumerate(verts):
            mesh.add_vertex(v, i)
        faces = [
            [0, 1, 2, 3],
            [4, 7, 6, 5],
            [0, 4, 5, 1],
            [2, 6, 7, 3],
            [0, 3, 7, 4],
            [1, 5, 6, 2],
        ]
        for f in faces:
            mesh.add_face(f)
        return mesh

    # Create boxes at same location
    box1 = create_box(Point(0, 0, 0), 2.0)
    box1.name = "box_1"
    box1_node = scene.add_mesh(box1)

    box2 = create_box(Point(0, 0, 0), 2.0)
    box2.name = "box_2"
    box2_node = scene.add_mesh(box2)

    box3 = create_box(Point(0, 0, 0), 2.0)
    box3.name = "box_3"
    box3_node = scene.add_mesh(box3)

    # Setup tree hierarchy
    scene.add(box1_node)
    scene.add(box2_node, box1_node)
    scene.add(box3_node, box2_node)

    # Apply transformations
    from .vector import Vector
    from .plane import Plane

    box1_top = Point(0, 0, 1.0)
    normal = Vector(0, 0, 1)
    x = Vector(1, 0, 0)
    y = Vector(0, 1, 0)
    xy_origin = Point(0, 0, 0)
    xy_x = Vector(1, 0, 0)
    xy_y = Vector(0, 1, 0)
    xy_z = Vector(0, 0, 1)

    xy_to_top = Xform.plane_to_plane(
        xy_origin, xy_x, xy_y, xy_z, box1_top, x, y, normal
    )
    box1.xform = Xform.rotation_z(math.pi / 1.5) * xy_to_top

    box2.xform = Xform.translation(2.0, 0, 0) * Xform.rotation_z(math.pi / 6.0)
    box3.xform = Xform.translation(2.0, 0, 0)

    # Extract transformed geometry
    transformed = scene.get_geometry()

    assert len(transformed.meshes) == 3

    # Expected vertices for box_1
    expected_box1 = [
        [1.36603, -0.366025, 0],
        [0.366025, 1.36603, 0],
        [-1.36603, 0.366025, 0],
        [-0.366025, -1.36603, 0],
        [1.36603, -0.366025, 2],
        [0.366025, 1.36603, 2],
        [-1.36603, 0.366025, 2],
        [-0.366025, -1.36603, 2],
    ]

    # Expected vertices for box_2
    expected_box2 = [
        [0.366025, 2.09808, 0],
        [-1.36603, 3.09808, 0],
        [-2.36603, 1.36603, 0],
        [-0.633975, 0.366025, 0],
        [0.366025, 2.09808, 2],
        [-1.36603, 3.09808, 2],
        [-2.36603, 1.36603, 2],
        [-0.633975, 0.366025, 2],
    ]

    # Expected vertices for box_3
    expected_box3 = [
        [-1.36603, 3.09808, 0],
        [-3.09808, 4.09808, 0],
        [-4.09808, 2.36603, 0],
        [-2.36603, 1.36603, 0],
        [-1.36603, 3.09808, 2],
        [-3.09808, 4.09808, 2],
        [-4.09808, 2.36603, 2],
        [-2.36603, 1.36603, 2],
    ]

    # Expected faces (same for all boxes)
    expected_faces = [
        [0, 1, 2, 3],
        [4, 7, 6, 5],
        [0, 4, 5, 1],
        [2, 6, 7, 3],
        [0, 3, 7, 4],
        [1, 5, 6, 2],
    ]

    # Validate box_1
    m1 = transformed.meshes[0]
    assert len(m1.vertex) == 8
    for i in range(8):
        v = m1.vertex[i]
        assert abs(v.x - expected_box1[i][0]) < 1e-4
        assert abs(v.y - expected_box1[i][1]) < 1e-4
        assert abs(v.z - expected_box1[i][2]) < 1e-4

    # Validate box_2
    m2 = transformed.meshes[1]
    assert len(m2.vertex) == 8
    for i in range(8):
        v = m2.vertex[i]
        assert abs(v.x - expected_box2[i][0]) < 1e-4
        assert abs(v.y - expected_box2[i][1]) < 1e-4
        assert abs(v.z - expected_box2[i][2]) < 1e-4

    # Validate box_3
    m3 = transformed.meshes[2]
    assert len(m3.vertex) == 8
    for i in range(8):
        v = m3.vertex[i]
        assert abs(v.x - expected_box3[i][0]) < 1e-4
        assert abs(v.y - expected_box3[i][1]) < 1e-4
        assert abs(v.z - expected_box3[i][2]) < 1e-4

    # Validate faces (all boxes have same topology)
    for mesh in [m1, m2, m3]:
        assert len(mesh.face) == 6
        face_idx = 0
        for key, face in mesh.face.items():
            assert len(face) == len(expected_faces[face_idx])
            for i in range(len(face)):
                assert face[i] == expected_faces[face_idx][i]
            face_idx += 1


def test_session_ray_cast_sanity():
    # Arrange a small scene along X axis
    scene = Session("ray_test_py")

    pt1 = Point(5, 0, 0)
    scene.add_point(pt1)
    pt2 = Point(15, 0, 0)
    scene.add_point(pt2)
    line1 = Line.from_points(Point(10, -2, 0), Point(10, 2, 0))
    scene.add_line(line1)
    plane = Plane(Point(20, 0, 0), Vector(1, 0, 0), Vector(0, 1, 0))
    scene.add_plane(plane)
    poly = Polyline([Point(25, -1, -1), Point(25, 0, 0), Point(25, 1, 1)])
    scene.add_polyline(poly)

    ray_origin = Point(0, 0, 0)
    ray_dir = Vector(1, 0, 0)

    hits = scene.ray_cast(ray_origin, ray_dir, 0.5)
    print(f"Session Ray Casting (py): {len(hits)} hit(s)")
    assert len(hits) >= 1


def test_all_geometry_types_ray_cast_subset():
    # Focus on types supported precisely by current ray_cast
    scene = Session("all_geom_py")
    scene.add_point(Point(0, 10, 0))
    scene.add_line(Line.from_points(Point(-1, 20, 0), Point(1, 20, 0)))
    scene.add_plane(Plane(Point(0, 30, 0), Vector(1, 0, 0), Vector(0, 0, 1)))
    scene.add_bbox(
        BoundingBox(
            Point(0, 40, 0),
            Vector(1, 0, 0),
            Vector(0, 1, 0),
            Vector(0, 0, 1),
            Vector(2, 2, 2),
        )
    )
    scene.add_polyline(Polyline([Point(-1, 70, 0), Point(0, 70, 0), Point(1, 70, 0)]))

    ray_origin = Point(0, 0, 0)
    ray_dir = Vector(0, 1, 0)

    hits = scene.ray_cast(ray_origin, ray_dir, 1.0)
    print(f"All Geometry Types (subset) Ray Casting (py): {len(hits)} hit(s)")
    # Session.ray_cast returns closest hit(s); ensure at least one
    assert len(hits) >= 1


def test_performance_points_vs_pure_bvh_py():
    import random
    import time

    random.seed(42)

    OBJECT_COUNT = 2000
    WORLD_SIZE = 100.0

    scene = Session("perf_points_py")
    pure_boxes = []

    for i in range(OBJECT_COUNT):
        x = (random.random() - 0.5) * WORLD_SIZE
        y = (random.random() - 0.5) * WORLD_SIZE
        z = (random.random() - 0.5) * WORLD_SIZE
        pt = Point(x, y, z)
        scene.add_point(pt)
        pure_boxes.append(
            BoundingBox(
                pt,
                Vector(1, 0, 0),
                Vector(0, 1, 0),
                Vector(0, 0, 1),
                Vector(0.5, 0.5, 0.5),
            )
        )

    ray_origin = Point(0, 0, 0)
    ray_dir = Vector(1, 0, 0)

    t0 = time.perf_counter()
    hits = scene.ray_cast(ray_origin, ray_dir, 1.0)
    t1 = time.perf_counter()
    session_ms = (t1 - t0) * 1000.0

    from .bvh import BVH

    t2 = time.perf_counter()
    bvh = BVH.from_boxes(pure_boxes, WORLD_SIZE)
    candidates = []
    bvh.ray_cast(ray_origin, ray_dir, candidates, True)
    t3 = time.perf_counter()
    bvh_ms = (t3 - t2) * 1000.0

    print(f"Session (py): {session_ms:.3f} ms ({len(hits)} hits)")
    print(f"Pure BVH (py): {bvh_ms:.3f} ms ({len(candidates)} candidates)")
    assert session_ms >= 0.0 and bvh_ms >= 0.0
