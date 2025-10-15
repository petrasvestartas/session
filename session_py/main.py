from session_py import BVH, BoundingBox, Point, Vector
import random

# Create 100 random boxes
random.seed(42)  # For reproducible results
boxes = []
for i in range(1000):
    # Random center position
    center = Point(
        random.uniform(-50, 50),
        random.uniform(-50, 50),
        random.uniform(-50, 50)
    )
    # Random half-size (dimensions)
    half_size = Vector(
        random.uniform(0.5, 3.0),
        random.uniform(0.5, 3.0),
        random.uniform(0.5, 3.0)
    )
    # Create axis-aligned bounding box
    bbox = BoundingBox(
        center,
        Vector(1, 0, 0),  # X axis
        Vector(0, 1, 0),  # Y axis
        Vector(0, 0, 1),  # Z axis
        half_size
    )
    boxes.append(bbox)

# Print min/max corners using new API
for i, box in enumerate(boxes, 1):
    min_corner = box.min_point()
    max_corner = box.max_point()
    print(f"Box {i} - Min: ({min_corner.x}, {min_corner.y}, {min_corner.z}), Max: ({max_corner.x}, {max_corner.y}, {max_corner.z})")

# Use BVH for collision detection
bvh = BVH(boxes)
collisions, colliding_indices, check_count = bvh.check_all_collisions(boxes)

print(collisions)
print(colliding_indices)
print(check_count)

from compas_viewer import Viewer, viewer
from compas.geometry import Box
from compas.colors import Color

# Convert boxes to compas boxes using new API
viewer = Viewer()
for i, box in enumerate(boxes):
    min_pt = box.min_point()
    max_pt = box.max_point()
    p0 = [min_pt.x, min_pt.y, min_pt.z]
    p1 = [max_pt.x, max_pt.y, max_pt.z]
    compas_box = Box.from_points([p0, p1])

    color = Color.white()
    print("index: ", i)
    if i in colliding_indices:
        color = Color.red()
        print("index: ", i)
    viewer.scene.add(compas_box, color=color, linecolor=Color.black())
viewer.show()
