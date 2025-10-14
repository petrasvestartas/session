from .point import Point
from .line import Line
from .plane import Plane
from .boundingbox import BoundingBox
from .polyline import Polyline
from .pointcloud import PointCloud
from .mesh import Mesh
from .cylinder import Cylinder
from .arrow import Arrow
import uuid


class Objects:
    """A collection of all geometry objects.

    Attributes
    ----------
    name : str
        The name of the collection.
    guid : UUID
        The unique identifier of the collection.
    points : list[Point]
        The list of points.
    lines : list[Line]
        The list of lines.
    planes : list[Plane]
        The list of planes.
    bboxes : list[BoundingBox]
        The list of bounding boxes.
    polylines : list[Polyline]
        The list of polylines.
    pointclouds : list[PointCloud]
        The list of point clouds.
    meshes : list[Mesh]
        The list of meshes.
    cylinders : list[Cylinder]
        The list of cylinders.
    arrows : list[Arrow]
        The list of arrows.

    """

    def __init__(self):
        self.guid = str(uuid.uuid4())
        self.name = "my_objects"
        self.points: list[Point] = []
        self.lines: list[Line] = []
        self.planes: list[Plane] = []
        self.bboxes: list[BoundingBox] = []
        self.polylines: list[Polyline] = []
        self.pointclouds: list[PointCloud] = []
        self.meshes: list[Mesh] = []
        self.cylinders: list[Cylinder] = []
        self.arrows: list[Arrow] = []

    def __str__(self):
        return f"Objects(points={len(self.points)})"

    def __repr__(self):
        return f"Objects({self.guid}, {self.name}, points={len(self.points)})"

    ###########################################################################################
    # Polymorphic JSON Serialization
    ###########################################################################################

    def __jsondump__(self):
        """Serialize to polymorphic JSON format with type field.

        Returns
        -------
        dict
            Dictionary with 'type', 'guid', 'name', and object fields.

        """
        return {
            "type": f"{self.__class__.__name__}",
            "guid": self.guid,
            "name": self.name,
            "points": [p.__jsondump__() for p in self.points],
            "lines": [l.__jsondump__() for l in self.lines],
            "planes": [pl.__jsondump__() for pl in self.planes],
            "bboxes": [b.__jsondump__() for b in self.bboxes],
            "polylines": [pl.__jsondump__() for pl in self.polylines],
            "pointclouds": [pc.__jsondump__() for pc in self.pointclouds],
            "meshes": [m.__jsondump__() for m in self.meshes],
            "cylinders": [c.__jsondump__() for c in self.cylinders],
            "arrows": [a.__jsondump__() for a in self.arrows],
        }

    @classmethod
    def __jsonload__(cls, data, guid=None, name=None):
        """Deserialize from polymorphic JSON format.

        Parameters
        ----------
        data : dict
            Dictionary containing objects data.
        guid : str, optional
            GUID for the objects.
        name : str, optional
            Name for the objects.

        Returns
        -------
        :class:`Objects`
            Reconstructed objects instance.

        """
        from .encoders import decode_node

        obj = cls()
        obj.guid = guid if guid is not None else data.get("guid", obj.guid)
        obj.name = name if name is not None else data.get("name", obj.name)

        obj.points = [decode_node(p) for p in data.get("points", [])]
        obj.lines = [decode_node(l) for l in data.get("lines", [])]
        obj.planes = [decode_node(pl) for pl in data.get("planes", [])]
        obj.bboxes = [decode_node(b) for b in data.get("bboxes", [])]
        obj.polylines = [decode_node(pl) for pl in data.get("polylines", [])]
        obj.pointclouds = [decode_node(pc) for pc in data.get("pointclouds", [])]
        obj.meshes = [decode_node(m) for m in data.get("meshes", [])]
        obj.cylinders = [decode_node(c) for c in data.get("cylinders", [])]
        obj.arrows = [decode_node(a) for a in data.get("arrows", [])]

        return obj

    ###########################################################################################
    # Details
    ###########################################################################################
