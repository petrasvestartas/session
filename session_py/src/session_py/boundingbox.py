import uuid
from typing import List
from .point import Point
from .vector import Vector
from .plane import Plane


class BoundingBox:
    def __init__(
        self,
        center: Point = None,
        x_axis: Vector = None,
        y_axis: Vector = None,
        z_axis: Vector = None,
        half_size: Vector = None,
    ):
        self.center = center if center is not None else Point(0.0, 0.0, 0.0)
        self.x_axis = x_axis if x_axis is not None else Vector(1.0, 0.0, 0.0)
        self.y_axis = y_axis if y_axis is not None else Vector(0.0, 1.0, 0.0)
        self.z_axis = z_axis if z_axis is not None else Vector(0.0, 0.0, 1.0)
        self.half_size = half_size if half_size is not None else Vector(0.5, 0.5, 0.5)
        self.guid = str(uuid.uuid4())
        self.name = "my_boundingbox"

    @classmethod
    def from_plane(cls, plane: Plane, dx: float, dy: float, dz: float):
        return cls(
            center=plane.origin,
            x_axis=plane.x_axis,
            y_axis=plane.y_axis,
            z_axis=plane.z_axis,
            half_size=Vector(dx * 0.5, dy * 0.5, dz * 0.5),
        )

    @classmethod
    def from_point(cls, point: Point, inflate: float = 0.0):
        return cls(
            center=point,
            x_axis=Vector(1.0, 0.0, 0.0),
            y_axis=Vector(0.0, 1.0, 0.0),
            z_axis=Vector(0.0, 0.0, 1.0),
            half_size=Vector(inflate, inflate, inflate),
        )

    @classmethod
    def from_points(cls, points: List[Point], inflate: float = 0.0):
        if not points:
            return cls()

        min_x = min(p.x for p in points)
        min_y = min(p.y for p in points)
        min_z = min(p.z for p in points)
        max_x = max(p.x for p in points)
        max_y = max(p.y for p in points)
        max_z = max(p.z for p in points)

        center = Point(
            (min_x + max_x) * 0.5,
            (min_y + max_y) * 0.5,
            (min_z + max_z) * 0.5,
        )
        half_size = Vector(
            (max_x - min_x) * 0.5 + inflate,
            (max_y - min_y) * 0.5 + inflate,
            (max_z - min_z) * 0.5 + inflate,
        )

        return cls(
            center=center,
            x_axis=Vector(1.0, 0.0, 0.0),
            y_axis=Vector(0.0, 1.0, 0.0),
            z_axis=Vector(0.0, 0.0, 1.0),
            half_size=half_size,
        )

    @classmethod
    def from_line(cls, line, inflate: float = 0.0):
        points = [line.start(), line.end()]
        return cls.from_points(points, inflate)

    @classmethod
    def from_polyline(cls, polyline, inflate: float = 0.0):
        return cls.from_points(polyline.points, inflate)

    def point_at(self, x: float, y: float, z: float) -> Point:
        return Point(
            self.center.x + x * self.x_axis.x + y * self.y_axis.x + z * self.z_axis.x,
            self.center.y + x * self.x_axis.y + y * self.y_axis.y + z * self.z_axis.y,
            self.center.z + x * self.x_axis.z + y * self.y_axis.z + z * self.z_axis.z,
        )

    def min_point(self) -> Point:
        """Get the minimum corner point of the axis-aligned bounding box.

        Returns
        -------
        Point
            The point with minimum x, y, z coordinates.
        """
        return Point(
            self.center.x - self.half_size.x,
            self.center.y - self.half_size.y,
            self.center.z - self.half_size.z,
        )

    def max_point(self) -> Point:
        """Get the maximum corner point of the axis-aligned bounding box.

        Returns
        -------
        Point
            The point with maximum x, y, z coordinates.
        """
        return Point(
            self.center.x + self.half_size.x,
            self.center.y + self.half_size.y,
            self.center.z + self.half_size.z,
        )

    def corners(self) -> List[Point]:
        """Get all 8 corner points of the bounding box.

        Returns
        -------
        List[Point]
            List of 8 corner points in a specific order.
        """
        return [
            self.point_at(self.half_size.x, self.half_size.y, -self.half_size.z),
            self.point_at(-self.half_size.x, self.half_size.y, -self.half_size.z),
            self.point_at(-self.half_size.x, -self.half_size.y, -self.half_size.z),
            self.point_at(self.half_size.x, -self.half_size.y, -self.half_size.z),
            self.point_at(self.half_size.x, self.half_size.y, self.half_size.z),
            self.point_at(-self.half_size.x, self.half_size.y, self.half_size.z),
            self.point_at(-self.half_size.x, -self.half_size.y, self.half_size.z),
            self.point_at(self.half_size.x, -self.half_size.y, self.half_size.z),
        ]

    def two_rectangles(self) -> List[Point]:
        return [
            self.point_at(self.half_size.x, self.half_size.y, -self.half_size.z),
            self.point_at(-self.half_size.x, self.half_size.y, -self.half_size.z),
            self.point_at(-self.half_size.x, -self.half_size.y, -self.half_size.z),
            self.point_at(self.half_size.x, -self.half_size.y, -self.half_size.z),
            self.point_at(self.half_size.x, self.half_size.y, -self.half_size.z),
            self.point_at(self.half_size.x, self.half_size.y, self.half_size.z),
            self.point_at(-self.half_size.x, self.half_size.y, self.half_size.z),
            self.point_at(-self.half_size.x, -self.half_size.y, self.half_size.z),
            self.point_at(self.half_size.x, -self.half_size.y, self.half_size.z),
            self.point_at(self.half_size.x, self.half_size.y, self.half_size.z),
        ]

    def inflate(self, amount: float):
        self.half_size = Vector(
            self.half_size.x + amount,
            self.half_size.y + amount,
            self.half_size.z + amount,
        )

    @staticmethod
    def _separating_plane_exists(
        relative_position: Vector,
        axis: Vector,
        box1: "BoundingBox",
        box2: "BoundingBox",
    ) -> bool:
        dot_rp = abs(relative_position.dot(axis))

        v1 = box1.x_axis * box1.half_size.x
        v2 = box1.y_axis * box1.half_size.y
        v3 = box1.z_axis * box1.half_size.z
        proj1 = abs(v1.dot(axis)) + abs(v2.dot(axis)) + abs(v3.dot(axis))

        v4 = box2.x_axis * box2.half_size.x
        v5 = box2.y_axis * box2.half_size.y
        v6 = box2.z_axis * box2.half_size.z
        proj2 = abs(v4.dot(axis)) + abs(v5.dot(axis)) + abs(v6.dot(axis))

        return dot_rp > (proj1 + proj2)

    def collides_with(self, other: "BoundingBox") -> bool:
        center_vec = Vector(self.center.x, self.center.y, self.center.z)
        other_center_vec = Vector(other.center.x, other.center.y, other.center.z)
        relative_position = Vector.from_start_and_end(center_vec, other_center_vec)

        return not (
            self._separating_plane_exists(relative_position, self.x_axis, self, other)
            or self._separating_plane_exists(
                relative_position, self.y_axis, self, other
            )
            or self._separating_plane_exists(
                relative_position, self.z_axis, self, other
            )
            or self._separating_plane_exists(
                relative_position, other.x_axis, self, other
            )
            or self._separating_plane_exists(
                relative_position, other.y_axis, self, other
            )
            or self._separating_plane_exists(
                relative_position, other.z_axis, self, other
            )
            or self._separating_plane_exists(
                relative_position, self.x_axis.cross(other.x_axis), self, other
            )
            or self._separating_plane_exists(
                relative_position, self.x_axis.cross(other.y_axis), self, other
            )
            or self._separating_plane_exists(
                relative_position, self.x_axis.cross(other.z_axis), self, other
            )
            or self._separating_plane_exists(
                relative_position, self.y_axis.cross(other.x_axis), self, other
            )
            or self._separating_plane_exists(
                relative_position, self.y_axis.cross(other.y_axis), self, other
            )
            or self._separating_plane_exists(
                relative_position, self.y_axis.cross(other.z_axis), self, other
            )
            or self._separating_plane_exists(
                relative_position, self.z_axis.cross(other.x_axis), self, other
            )
            or self._separating_plane_exists(
                relative_position, self.z_axis.cross(other.y_axis), self, other
            )
            or self._separating_plane_exists(
                relative_position, self.z_axis.cross(other.z_axis), self, other
            )
        )

    ###########################################################################################
    # Transformation
    ###########################################################################################

    def transform(self):
        """Apply the stored xform transformation to the bounding box.

        Transforms the bounding box in-place and resets xform to identity.
        """
        from .xform import Xform

        self.xform.transform_point(self.center)
        self.xform.transform_vector(self.x_axis)
        self.xform.transform_vector(self.y_axis)
        self.xform.transform_vector(self.z_axis)
        self.xform = Xform.identity()

    def transformed(self):
        """Return a transformed copy of the bounding box."""
        import copy

        result = copy.deepcopy(self)
        result.transform()
        return result

    ###########################################################################################
    # Polymorphic JSON Serialization (COMPAS-style)
    ###########################################################################################

    def __jsondump__(self):
        """Serialize to polymorphic JSON format with type field."""
        return {
            "type": f"{self.__class__.__name__}",
            "guid": self.guid,
            "name": self.name,
            "center": self.center.__jsondump__(),
            "x_axis": self.x_axis.__jsondump__(),
            "y_axis": self.y_axis.__jsondump__(),
            "z_axis": self.z_axis.__jsondump__(),
            "half_size": self.half_size.__jsondump__(),
        }

    @classmethod
    def __jsonload__(cls, data, guid=None, name=None):
        """Deserialize from polymorphic JSON format."""
        from .encoders import decode_node

        center = decode_node(data["center"])
        x_axis = decode_node(data["x_axis"])
        y_axis = decode_node(data["y_axis"])
        z_axis = decode_node(data["z_axis"])
        half_size = decode_node(data["half_size"])

        bbox = cls(center, x_axis, y_axis, z_axis, half_size)
        bbox.guid = guid
        bbox.name = name

        if "xform" in data:
            bbox.xform = decode_node(data["xform"])

        return bbox
