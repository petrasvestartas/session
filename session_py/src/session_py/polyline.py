"""Polyline class for representing a collection of connected points."""

import json
import uuid
from typing import List, Optional

from .plane import Plane
from .point import Point
from .vector import Vector


class Polyline:
    """A polyline defined by a collection of points with an associated plane."""

    def __init__(self, points: Optional[List[Point]] = None):
        """Creates a new Polyline with default guid and name.

        Args:
            points: The collection of points.
        """
        self.guid = str(uuid.uuid4())
        self.name = "my_polyline"
        self.points = points if points is not None else []

        # Delegate plane computation to Plane.from_points
        if len(self.points) >= 3:
            self.plane = Plane.from_points(self.points)
        else:
            self.plane = Plane()

    def __len__(self) -> int:
        """Returns the number of points in the polyline."""
        return len(self.points)

    def is_empty(self) -> bool:
        """Returns true if the polyline has no points."""
        return len(self.points) == 0

    def segment_count(self) -> int:
        """Returns the number of segments (n-1 for n points)."""
        return len(self.points) - 1 if len(self.points) > 1 else 0

    def length(self) -> float:
        """Calculates the total length of the polyline."""
        total_length = 0.0
        for i in range(self.segment_count()):
            segment_vector = self.points[i + 1] - self.points[i]
            total_length += segment_vector.magnitude()
        return total_length

    def get_point(self, index: int) -> Optional[Point]:
        """Returns the point at the given index, or None if out of bounds."""
        if 0 <= index < len(self.points):
            return self.points[index]
        return None

    def add_point(self, point: Point) -> None:
        """Adds a point to the end of the polyline."""
        self.points.append(point)
        if len(self.points) == 3:
            self._recompute_plane()

    def insert_point(self, index: int, point: Point) -> None:
        """Inserts a point at the specified index."""
        self.points.insert(index, point)
        if len(self.points) == 3:
            self._recompute_plane()

    def remove_point(self, index: int) -> Optional[Point]:
        """Removes and returns the point at the specified index."""
        if 0 <= index < len(self.points):
            point = self.points.pop(index)
            if len(self.points) == 3:
                self._recompute_plane()
            return point
        return None

    def reverse(self) -> None:
        """Reverses the order of points in the polyline."""
        self.points.reverse()
        self.plane.reverse()

    def reversed(self) -> "Polyline":
        """Returns a new polyline with reversed point order."""
        result = Polyline(self.points[:])
        result.guid = self.guid
        result.name = self.name
        result.plane = self.plane
        result.reverse()
        return result

    def _recompute_plane(self) -> None:
        """Helper to recompute plane when points change."""
        if len(self.points) >= 3:
            self.plane = Plane.from_points(self.points)

    def __iadd__(self, vector: Vector) -> "Polyline":
        """Translates all points in the polyline by a vector (+=)."""
        for point in self.points:
            point += vector
        # Update plane origin
        self.plane = Plane(
            self.plane.origin + vector, self.plane.x_axis, self.plane.y_axis
        )
        return self

    def __add__(self, vector: Vector) -> "Polyline":
        """Translates the polyline by a vector and returns a new polyline (+)."""
        result = Polyline([Point(p.x, p.y, p.z) for p in self.points])
        result.guid = self.guid
        result.name = self.name
        result.plane = self.plane
        result += vector
        return result

    def __isub__(self, vector: Vector) -> "Polyline":
        """Translates all points by the negative of a vector (-=)."""
        for point in self.points:
            point -= vector
        # Update plane origin
        self.plane = Plane(
            self.plane.origin - vector, self.plane.x_axis, self.plane.y_axis
        )
        return self

    def __sub__(self, vector: Vector) -> "Polyline":
        """Translates the polyline by the negative of a vector and returns a new polyline (-)."""
        result = Polyline([Point(p.x, p.y, p.z) for p in self.points])
        result.guid = self.guid
        result.name = self.name
        result.plane = self.plane
        result -= vector
        return result

    def __str__(self) -> str:
        """Returns a string representation of the polyline."""
        return (
            f"Polyline(guid={self.guid}, name={self.name}, points={len(self.points)})"
        )

    def __repr__(self) -> str:
        """Returns a detailed string representation."""
        return self.__str__()

    def to_json_data(self) -> str:
        """Serializes the Polyline to a JSON string."""
        data = {
            "type": "Polyline",
            "guid": self.guid,
            "name": self.name,
            "points": [p.to_json_data() for p in self.points],
            "plane": self.plane.to_json_data(),
        }
        return json.dumps(data, indent=4)

    @staticmethod
    def from_json_data(json_data: str) -> "Polyline":
        """Deserializes a Polyline from a JSON string."""
        data = json.loads(json_data)
        polyline = Polyline()
        polyline.guid = data["guid"]
        polyline.name = data["name"]
        polyline.points = [Point.from_json_data(pt) for pt in data["points"]]
        polyline.plane = Plane.from_json_data(data["plane"])
        return polyline

    def to_json(self, filepath: str) -> None:
        """Serializes the Polyline to a JSON file."""
        with open(filepath, "w") as f:
            f.write(self.to_json_data())

    @staticmethod
    def from_json(filepath: str) -> "Polyline":
        """Deserializes a Polyline from a JSON file."""
        with open(filepath, "r") as f:
            json_data = f.read()
        return Polyline.from_json_data(json_data)

    def to_data(self) -> dict:
        """Convert to dictionary for JSON serialization."""
        return {
            "type": "Polyline",
            "guid": self.guid,
            "name": self.name,
            "points": [p.to_json_data() for p in self.points],
            "plane": self.plane.to_json_data(),
        }

    @staticmethod
    def from_data(data: dict) -> "Polyline":
        """Create Polyline from dictionary."""
        polyline = Polyline()
        polyline.guid = data["guid"]
        polyline.name = data["name"]
        polyline.points = [Point.from_json_data(pt) for pt in data["points"]]
        polyline.plane = Plane.from_json_data(data["plane"])
        return polyline
