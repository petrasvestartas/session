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
