import json
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

    def corners(self) -> List[Point]:
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

    def to_json_data(self) -> dict:
        return {
            "type": "BoundingBox",
            "center": self.center.to_json_data(),
            "x_axis": self.x_axis.to_json_data(),
            "y_axis": self.y_axis.to_json_data(),
            "z_axis": self.z_axis.to_json_data(),
            "half_size": self.half_size.to_json_data(),
            "guid": self.guid,
            "name": self.name,
        }

    @classmethod
    def from_json_data(cls, data: dict) -> "BoundingBox":
        box = cls(
            center=Point.from_json_data(data["center"]),
            x_axis=Vector.from_json_data(data["x_axis"]),
            y_axis=Vector.from_json_data(data["y_axis"]),
            z_axis=Vector.from_json_data(data["z_axis"]),
            half_size=Vector.from_json_data(data["half_size"]),
        )
        box.guid = data["guid"]
        box.name = data["name"]
        return box

    def to_json(self, filepath: str):
        with open(filepath, "w") as f:
            json.dump(self.to_json_data(), f, indent=4)

    @classmethod
    def from_json(cls, filepath: str) -> "BoundingBox":
        with open(filepath, "r") as f:
            data = json.load(f)
        return cls.from_json_data(data)
