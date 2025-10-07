import json
import uuid
from typing import List

from .color import Color
from .point import Point
from .vector import Vector
from .xform import Xform


class PointCloud:
    """A point cloud with points, normals, colors, and transformation.

    Parameters
    ----------
    points : List[Point], optional
        Collection of points.
    normals : List[Vector], optional
        Collection of normals.
    colors : List[Color], optional
        Collection of colors.

    Attributes
    ----------
    guid : str
        Unique identifier.
    name : str
        Name of the point cloud.
    points : List[Point]
        Collection of points.
    normals : List[Vector]
        Collection of normals.
    colors : List[Color]
        Collection of colors.
    xform : Xform
        Transformation matrix.
    """

    def __init__(self, points=None, normals=None, colors=None):
        self.guid = str(uuid.uuid4())
        self.name = "my_pointcloud"
        self.points = points if points is not None else []
        self.normals = normals if normals is not None else []
        self.colors = colors if colors is not None else []
        self.xform = Xform()

    ###########################################################################################
    # Operators
    ###########################################################################################

    def __str__(self):
        return f"PointCloud(points={len(self.points)}, normals={len(self.normals)}, colors={len(self.colors)}, guid={self.guid}, name={self.name})"

    def __repr__(self):
        return self.__str__()

    def __len__(self):
        return len(self.points)

    ###########################################################################################
    # JSON
    ###########################################################################################

    def to_json_data(self) -> dict:
        """Convert to JSON-serializable dictionary with flat arrays.

        Returns
        -------
        dict
            JSON-serializable dictionary.
        """
        # Flatten points to [x, y, z, x, y, z, ...]
        points_flat = []
        for p in self.points:
            points_flat.extend([p.x, p.y, p.z])

        # Flatten normals to [x, y, z, x, y, z, ...]
        normals_flat = []
        for n in self.normals:
            normals_flat.extend([n.x, n.y, n.z])

        # Flatten colors to [r, g, b, r, g, b, ...] (no alpha)
        colors_flat = []
        for c in self.colors:
            colors_flat.extend([c.r, c.g, c.b])

        return {
            "type": "PointCloud",
            "guid": self.guid,
            "name": self.name,
            "points": points_flat,
            "normals": normals_flat,
            "colors": colors_flat,
            "xform": self.xform.to_json_data(),
        }

    @staticmethod
    def from_json_data(data: dict) -> "PointCloud":
        """Create PointCloud from JSON data.

        Parameters
        ----------
        data : dict
            JSON data dictionary.

        Returns
        -------
        PointCloud
            PointCloud instance.
        """
        cloud = PointCloud()
        cloud.guid = data["guid"]
        cloud.name = data["name"]

        # Reconstruct points from flat array
        points_flat = data["points"]
        cloud.points = [
            Point(points_flat[i], points_flat[i + 1], points_flat[i + 2])
            for i in range(0, len(points_flat), 3)
        ]

        # Reconstruct normals from flat array
        normals_flat = data["normals"]
        cloud.normals = [
            Vector(normals_flat[i], normals_flat[i + 1], normals_flat[i + 2])
            for i in range(0, len(normals_flat), 3)
        ]

        # Reconstruct colors from flat array (RGB only, alpha always 255)
        colors_flat = data["colors"]
        cloud.colors = [
            Color(colors_flat[i], colors_flat[i + 1], colors_flat[i + 2], 255)
            for i in range(0, len(colors_flat), 3)
        ]

        cloud.xform = Xform.from_json_data(data["xform"])

        return cloud

    def to_json(self, filepath: str) -> None:
        """Serialize to JSON file.

        Parameters
        ----------
        filepath : str
            Path to JSON file.
        """
        with open(filepath, "w") as f:
            json.dump(self.to_json_data(), f, indent=4)

    @staticmethod
    def from_json(filepath: str) -> "PointCloud":
        """Deserialize from JSON file.

        Parameters
        ----------
        filepath : str
            Path to JSON file.

        Returns
        -------
        PointCloud
            PointCloud instance.
        """
        with open(filepath, "r") as f:
            data = json.load(f)
        return PointCloud.from_json_data(data)

    ###########################################################################################
    # No-copy Operators
    ###########################################################################################

    def __iadd__(self, other):
        """Translate point cloud by vector (in-place)."""
        if isinstance(other, Vector):
            for p in self.points:
                p.x += other.x
                p.y += other.y
                p.z += other.z
        return self

    def __isub__(self, other):
        """Translate point cloud by negative vector (in-place)."""
        if isinstance(other, Vector):
            for p in self.points:
                p.x -= other.x
                p.y -= other.y
                p.z -= other.z
        return self

    ###########################################################################################
    # Copy Operators
    ###########################################################################################

    def __add__(self, other):
        """Translate point cloud by vector (copy)."""
        if isinstance(other, Vector):
            cloud = PointCloud(
                [Point(p.x, p.y, p.z) for p in self.points],
                [Vector(n.x, n.y, n.z) for n in self.normals],
                [Color(c.r, c.g, c.b, c.a) for c in self.colors],
            )
            cloud.guid = self.guid
            cloud.name = self.name
            cloud.xform = self.xform
            cloud += other
            return cloud
        return NotImplemented

    def __sub__(self, other):
        """Translate point cloud by negative vector (copy)."""
        if isinstance(other, Vector):
            cloud = PointCloud(
                [Point(p.x, p.y, p.z) for p in self.points],
                [Vector(n.x, n.y, n.z) for n in self.normals],
                [Color(c.r, c.g, c.b, c.a) for c in self.colors],
            )
            cloud.guid = self.guid
            cloud.name = self.name
            cloud.xform = self.xform
            cloud -= other
            return cloud
        return NotImplemented

    ###########################################################################################
    # Details
    ###########################################################################################

    def size(self) -> int:
        """Get number of points."""
        return len(self.points)

    def is_empty(self) -> bool:
        """Check if point cloud is empty."""
        return len(self.points) == 0
