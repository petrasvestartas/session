from src.session_py.session import Session
from src.session_py.point import Point
from src.session_py.line import Line
from src.session_py.boundingbox import BoundingBox
from src.session_py.vector import Vector

session = Session("demo")

# Add geometry with some overlaps, some separated
session.add_point(Point(0, 0, 0))                    # Point 1
session.add_point(Point(0.0005, 0, 0))               # Point 2 - collides with Point 1
session.add_line(Line(0, 0, 0, 0.1, 0.1, 0.1))       # Line 1 - collides with both points
session.add_line(Line(5, 5, 5, 5.1, 5.1, 5.1))       # Line 2 - far away, no collision
session.add_bbox(BoundingBox(Point(10, 10, 10), Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1), Vector(0.5, 0.5, 0.5)))  # Box - far away

# Detect collisions
collisions = session.get_collisions()
print(f"Objects: {len(session.lookup)}, Collisions: {len(collisions)}")

# Print graph edges
print("\nGraph edges:")
for node, edges in session.graph.edges.items():
    for neighbor_guid, edge in edges.items():
        print(f"  {node[:8]}... -> {neighbor_guid[:8]}... [{edge.attribute}]")
        

