// Auto-generated unified API index
window.API_INDEX = {
  "version": "3.0",
  "type": "concept-unified",
  "concepts": [
    {
      "name": "Point.__init__",
      "implementations": {
        "python": {
          "sig": "__init__(x=0.0, y=0.0, z=0.0, name=\"my_point\")",
          "code": "def __init__(self, x=0.0, y=0.0, z=0.0, name=\"my_point\"):\n\n        self.guid = str(uuid.uuid4())\n        self.name = name\n        self._x = x\n        self._y = y\n        self._z = z\n        self.width = 1.0\n        self.pointcolor = Color.blue()\n        self.xform = Xform.identity()\n\n    ###########################################################################################\n    # Operators"
        }
      }
    },
    {
      "name": "Point.__deepcopy__",
      "implementations": {
        "python": {
          "sig": "__deepcopy__(memo)",
          "code": "def __deepcopy__(self, memo):\n\n\n        cls = self.__class__\n        result = cls.__new__(cls)\n        memo[id(self)] = result\n\n        # New guid\n        result.guid = str(uuid.uuid4())\n\n        # Copy remaining fields\n        result.name = copy.deepcopy(self.name, memo)\n        result._x = self._x"
        }
      }
    },
    {
      "name": "Point.duplicate",
      "implementations": {
        "python": {
          "sig": "duplicate()",
          "code": "def duplicate(self):\n\n        \"\"\"Duplicate the point.\"\"\"\n        return copy.deepcopy(self)\n\n    def __str__(self):\n        return f\"{self[0]}, {self[1]}, {self[2]}\"\n\n    def __repr__(self):\n        return f\"Point({self.name}, {self[0]}, {self[1]}, {self[2]}, {repr(self.pointcolor)}, {self.width})\"\n\n    def __eq__(self, other):\n        return ("
        },
        "rust": {
          "sig": "duplicate() -> Self",
          "code": "pub fn duplicate(&self) -> Self {\n        let mut copy = self.clone();\n        copy.guid = Uuid::new_v4().to_string();\n        copy\n    }"
        }
      }
    },
    {
      "name": "Point.__str__",
      "implementations": {
        "python": {
          "sig": "__str__()",
          "code": "def __str__(self):\n\n        return f\"{self[0]}, {self[1]}, {self[2]}\"\n\n    def __repr__(self):\n        return f\"Point({self.name}, {self[0]}, {self[1]}, {self[2]}, {repr(self.pointcolor)}, {self.width})\"\n\n    def __eq__(self, other):\n        return (\n            self.name == other.name\n            and round(self[0], Tolerance.ROUNDING) == round(other[0], Tolerance.ROUNDING)\n            and round(self[1], Tolerance.ROUNDING) == round(other[1], Tolerance.ROUNDING)\n            and round(self[2], Tolerance.ROUNDING) == round(other[2], Tolerance.ROUNDING)"
        }
      }
    },
    {
      "name": "Point.__repr__",
      "implementations": {
        "python": {
          "sig": "__repr__()",
          "code": "def __repr__(self):\n\n        return f\"Point({self.name}, {self[0]}, {self[1]}, {self[2]}, {repr(self.pointcolor)}, {self.width})\"\n\n    def __eq__(self, other):\n        return (\n            self.name == other.name\n            and round(self[0], Tolerance.ROUNDING) == round(other[0], Tolerance.ROUNDING)\n            and round(self[1], Tolerance.ROUNDING) == round(other[1], Tolerance.ROUNDING)\n            and round(self[2], Tolerance.ROUNDING) == round(other[2], Tolerance.ROUNDING)\n            and round(self.width, Tolerance.ROUNDING) == round(other.width, Tolerance.ROUNDING)\n            and self.pointcolor == other.pointcolor\n            and self.xform == other.xform"
        }
      }
    },
    {
      "name": "Point.__eq__",
      "implementations": {
        "python": {
          "sig": "__eq__(other)",
          "code": "def __eq__(self, other):\n\n        return (\n            self.name == other.name\n            and round(self[0], Tolerance.ROUNDING) == round(other[0], Tolerance.ROUNDING)\n            and round(self[1], Tolerance.ROUNDING) == round(other[1], Tolerance.ROUNDING)\n            and round(self[2], Tolerance.ROUNDING) == round(other[2], Tolerance.ROUNDING)\n            and round(self.width, Tolerance.ROUNDING) == round(other.width, Tolerance.ROUNDING)\n            and self.pointcolor == other.pointcolor\n            and self.xform == other.xform\n        )\n\n    def __ne__(self, other):"
        }
      }
    },
    {
      "name": "Point.__ne__",
      "implementations": {
        "python": {
          "sig": "__ne__(other)",
          "code": "def __ne__(self, other):\n\n        return not self == other\n\n    ###########################################################################################\n    # No-copy Operators\n    ###########################################################################################\n\n    def __getitem__(self, index):\n        if index == 0:\n            return self._x\n        elif index == 1:\n            return self._y"
        }
      }
    },
    {
      "name": "Point.__getitem__",
      "implementations": {
        "python": {
          "sig": "__getitem__(index)",
          "code": "def __getitem__(self, index):\n\n        if index == 0:\n            return self._x\n        elif index == 1:\n            return self._y\n        elif index == 2:\n            return self._z\n        else:\n            raise IndexError(\"Index out of range\")\n\n    def __setitem__(self, index, value):\n        if index == 0:"
        }
      }
    },
    {
      "name": "Point.__setitem__",
      "implementations": {
        "python": {
          "sig": "__setitem__(index, value)",
          "code": "def __setitem__(self, index, value):\n\n        if index == 0:\n            self._x = value\n        elif index == 1:\n            self._y = value\n        elif index == 2:\n            self._z = value\n        else:\n            raise IndexError(\"Index out of range\")\n\n    def __imul__(self, other):\n        self._x *= other"
        }
      }
    },
    {
      "name": "Point.__imul__",
      "implementations": {
        "python": {
          "sig": "__imul__(other)",
          "code": "def __imul__(self, other):\n\n        self._x *= other\n        self._y *= other\n        self._z *= other\n        return self\n\n    def __itruediv__(self, other):\n        self._x /= other\n        self._y /= other\n        self._z /= other\n        return self"
        }
      }
    },
    {
      "name": "Point.__itruediv__",
      "implementations": {
        "python": {
          "sig": "__itruediv__(other)",
          "code": "def __itruediv__(self, other):\n\n        self._x /= other\n        self._y /= other\n        self._z /= other\n        return self\n\n    def __iadd__(self, other):\n        if isinstance(other, Vector):\n            self._x += other.x\n            self._y += other.y\n            self._z += other.z\n        else:"
        }
      }
    },
    {
      "name": "Point.__iadd__",
      "implementations": {
        "python": {
          "sig": "__iadd__(other)",
          "code": "def __iadd__(self, other):\n\n        if isinstance(other, Vector):\n            self._x += other.x\n            self._y += other.y\n            self._z += other.z\n        else:\n            raise TypeError(\"Point can only be added with Vector\")\n        return self\n\n    def __isub__(self, other):\n        if isinstance(other, Vector):\n            self._x -= other.x"
        }
      }
    },
    {
      "name": "Point.__isub__",
      "implementations": {
        "python": {
          "sig": "__isub__(other)",
          "code": "def __isub__(self, other):\n\n        if isinstance(other, Vector):\n            self._x -= other.x\n            self._y -= other.y\n            self._z -= other.z\n        else:\n            raise TypeError(\"Point can only be subtracted with Vector\")\n        return self\n\n    ###########################################################################################\n    # Copy Operators\n    ###########################################################################################"
        }
      }
    },
    {
      "name": "Point.__mul__",
      "implementations": {
        "python": {
          "sig": "__mul__(other)",
          "code": "def __mul__(self, other):\n\n        return Point(self[0] * other, self[1] * other, self[2] * other)\n\n    def __truediv__(self, other):\n        return Point(self[0] / other, self[1] / other, self[2] / other)\n\n    def __add__(self, other):\n        return Point(self[0] + other[0], self[1] + other[1], self[2] + other[2])\n\n    def __sub__(self, other):\n        return Vector(self[0] - other[0], self[1] - other[1], self[2] - other[2])"
        }
      }
    },
    {
      "name": "Point.__truediv__",
      "implementations": {
        "python": {
          "sig": "__truediv__(other)",
          "code": "def __truediv__(self, other):\n\n        return Point(self[0] / other, self[1] / other, self[2] / other)\n\n    def __add__(self, other):\n        return Point(self[0] + other[0], self[1] + other[1], self[2] + other[2])\n\n    def __sub__(self, other):\n        return Vector(self[0] - other[0], self[1] - other[1], self[2] - other[2])\n\n    ###########################################################################################\n    # Transformation\n    ###########################################################################################"
        }
      }
    },
    {
      "name": "Point.__add__",
      "implementations": {
        "python": {
          "sig": "__add__(other)",
          "code": "def __add__(self, other):\n\n        return Point(self[0] + other[0], self[1] + other[1], self[2] + other[2])\n\n    def __sub__(self, other):\n        return Vector(self[0] - other[0], self[1] - other[1], self[2] - other[2])\n\n    ###########################################################################################\n    # Transformation\n    ###########################################################################################\n\n    def transform(self):\n        \"\"\"Apply the stored xform transformation to the point coordinates."
        }
      }
    },
    {
      "name": "Point.__sub__",
      "implementations": {
        "python": {
          "sig": "__sub__(other)",
          "code": "def __sub__(self, other):\n\n        return Vector(self[0] - other[0], self[1] - other[1], self[2] - other[2])\n\n    ###########################################################################################\n    # Transformation\n    ###########################################################################################\n\n    def transform(self):\n        \"\"\"Apply the stored xform transformation to the point coordinates.\n\n        Transforms the point in-place and resets xform to identity.\n        \"\"\""
        }
      }
    },
    {
      "name": "Point.transform",
      "implementations": {
        "python": {
          "sig": "transform()",
          "code": "def transform(self):\n\n        \"\"\"Apply the stored xform transformation to the point coordinates.\n\n        Transforms the point in-place and resets xform to identity.\n        \"\"\"\n        self.xform.transform_point(self)\n        self.xform = Xform.identity()\n\n    def transformed(self):\n        \"\"\"Return a transformed copy of the point.\n\n        Returns a new point with the transformation applied."
        },
        "cpp": {
          "sig": "void transform()",
          "code": "void Point::transform() {\n  xform.transform_point(*this);\n  xform = Xform::identity();\n}"
        },
        "rust": {
          "sig": "transform()",
          "code": "pub fn transform(&mut self) {\n        let xform = self.xform.clone();\n        xform.transform_point(self);\n        self.xform = Xform::identity();\n    }"
        }
      }
    },
    {
      "name": "Point.transformed",
      "implementations": {
        "python": {
          "sig": "transformed()",
          "code": "def transformed(self):\n\n        \"\"\"Return a transformed copy of the point.\n\n        Returns a new point with the transformation applied.\n        The original point and its xform remain unchanged.\n\n        Returns\n        -------\n        Point\n            A new transformed point.\n        \"\"\""
        },
        "cpp": {
          "sig": "Point transformed()",
          "code": "Point Point::transformed() const {\n  Point result = *this;\n  result.transform();\n  return result;\n}"
        },
        "rust": {
          "sig": "transformed() -> Self",
          "code": "pub fn transformed(&self) -> Self {\n        let mut result = self.clone();\n        result.transform();\n        result\n    }"
        }
      }
    },
    {
      "name": "Point.is_ccw",
      "implementations": {
        "python": {
          "sig": "is_ccw(a, b, c)",
          "code": "def is_ccw(a, b, c):\n\n        \"\"\"Check if the points are in counter-clockwise order on xy plane.\n\n        Parameters\n        ----------\n        a : :class:`Point`\n            First point.\n        b : :class:`Point`\n            Second point.\n        c : :class:`Point`\n            Third point."
        },
        "cpp": {
          "sig": "bool is_ccw(const Point& a, const Point& b, const Point& c)",
          "code": "bool Point::is_ccw(const Point& a, const Point& b, const Point& c) {\n    return ccw(a, b, c);\n}"
        },
        "rust": {
          "sig": "is_ccw(a: &Point, b: &Point, c: &Point) -> bool",
          "code": "pub fn is_ccw(a: &Point, b: &Point, c: &Point) -> bool {\n        Self::ccw(a, b, c)\n    }"
        }
      }
    },
    {
      "name": "Point.mid_point",
      "implementations": {
        "python": {
          "sig": "mid_point(p)",
          "code": "def mid_point(self, p):\n\n        \"\"\"Calculate the mid point between this point and another point.\n\n        Parameters\n        ----------\n        p : :class:`Point`\n            The other point.\n\n        Returns\n        -------\n        :class:`Point`\n            The mid point between this point and the other point."
        },
        "cpp": {
          "sig": "Point mid_point(const Point& a, const Point& b)",
          "code": "Point Point::mid_point(const Point& a, const Point& b) {\n    return a.mid_point(b);\n}"
        },
        "rust": {
          "sig": "mid_point(a: &Point, b: &Point) -> Point",
          "code": "pub fn mid_point(a: &Point, b: &Point) -> Point {\n        Point::new(\n            (a._x + b._x) / 2.0,\n            (a._y + b._y) / 2.0,\n            (a._z + b._z) / 2.0,\n        )\n    }"
        }
      }
    },
    {
      "name": "Point.distance",
      "implementations": {
        "python": {
          "sig": "distance(p, double_min=1e-12)",
          "code": "def distance(self, p, double_min=1e-12):\n\n        \"\"\"Calculate the distance between this point and another point.\n\n        Parameters\n        ----------\n        p : :class:`Point`\n            The other point.\n        double_min : float, optional\n            The minimum value for the distance. Defaults to 1e-12.\n\n        Returns\n        -------"
        },
        "cpp": {
          "sig": "double distance(const Point& a, const Point& b, double float_min)",
          "code": "double Point::distance(const Point& a, const Point& b, double float_min) {\n    return a.distance(b, float_min);\n}"
        },
        "rust": {
          "sig": "distance(p: &Point, double_min: Option<f64>) -> f64",
          "code": "pub fn distance(&self, p: &Point, double_min: Option<f64>) -> f64 {\n        let double_min = double_min.unwrap_or(1e-12);\n        let mut dx = (self[0] - p[0]).abs();\n        let mut dy = (self[1] - p[1]).abs();\n        let mut dz = (self[2] - p[2]).abs();\n\n        // Reorder coordinates to put largest in dx\n        if dy >= dx && dy >= dz {\n            std::mem::swap(&mut dx, &mut dy);\n        } else if dz >= dx && dz >= dy {\n            std::mem::swap(&mut dx, &mut dz);\n        }"
        }
      }
    },
    {
      "name": "Point.squared_distance",
      "implementations": {
        "python": {
          "sig": "squared_distance(p, double_min=1e-12)",
          "code": "def squared_distance(self, p, double_min=1e-12):\n\n        \"\"\"Calculate the squared distance between this point and another point.\n\n        Parameters\n        ----------\n        p : :class:`Point`\n            The other point.\n        double_min : float, optional\n            The minimum value for the distance. Defaults to 1e-12.\n\n        Returns\n        -------"
        },
        "cpp": {
          "sig": "double squared_distance(const Point& a, const Point& b, double float_min)",
          "code": "double Point::squared_distance(const Point& a, const Point& b, double float_min) {\n    return a.squared_distance(b, float_min);\n}"
        },
        "rust": {
          "sig": "squared_distance(p: &Point, double_min: Option<f64>) -> f64",
          "code": "pub fn squared_distance(&self, p: &Point, double_min: Option<f64>) -> f64 {\n        let double_min = double_min.unwrap_or(1e-12);\n        let mut dx = (self[0] - p[0]).abs();\n        let mut dy = (self[1] - p[1]).abs();\n        let mut dz = (self[2] - p[2]).abs();\n\n        if dy >= dx && dy >= dz {\n            std::mem::swap(&mut dx, &mut dy);\n        } else if dz >= dx && dz >= dy {\n            std::mem::swap(&mut dx, &mut dz);\n        }\n\n        if dx > double_min {\n            dy /= dx;"
        }
      }
    },
    {
      "name": "Point.area",
      "implementations": {
        "python": {
          "sig": "area(points)",
          "code": "def area(points):\n\n        \"\"\"Calculate the area of a 2d polygon.\n\n        Parameters\n        ----------\n        points : list of :class:`Point`\n            The points of the polygon.\n\n        Returns\n        -------\n        float\n            The area of the polygon."
        },
        "cpp": {
          "sig": "double area(const std::vector<Point>& points)",
          "code": "double Point::area(const std::vector<Point>& points) {\n    size_t n = points.size();\n    double area = 0.0;\n    \n    for (size_t i = 0; i < n; ++i) {\n        size_t j = (i + 1) % n;\n        area += points[i][0] * points[j][1];\n        area -= points[j][0] * points[i][1];\n    }"
        },
        "rust": {
          "sig": "area(points: &[Point]) -> f64",
          "code": "pub fn area(points: &[Point]) -> f64 {\n        let n = points.len();\n        let mut area = 0.0;\n\n        for i in 0..n {\n            let j = (i + 1) % n;\n            area += points[i][0] * points[j][1];\n            area -= points[j][0] * points[i][1];\n        }\n\n        area.abs() / 2.0\n    }"
        }
      }
    },
    {
      "name": "Point.centroid_quad",
      "implementations": {
        "python": {
          "sig": "centroid_quad(vertices)",
          "code": "def centroid_quad(vertices):\n\n        \"\"\"Calculate the centroid of a quadrilateral.\n\n        Parameters\n        ----------\n        vertices : list of :class:`Point`\n            The vertices of the quadrilateral.\n\n        Returns\n        -------\n        :class:`Point`\n            The centroid of the quadrilateral."
        },
        "cpp": {
          "sig": "Point centroid_quad(const std::vector<Point>& vertices)",
          "code": "Point Point::centroid_quad(const std::vector<Point>& vertices) {\n    if (vertices.size() != 4) {\n        throw std::invalid_argument(\"Polygon must have exactly 4 vertices.\");\n    }"
        },
        "rust": {
          "sig": "centroid_quad(vertices: &[Point]) -> Result<Point, &'static str>",
          "code": "pub fn centroid_quad(vertices: &[Point]) -> Result<Point, &'static str> {\n        if vertices.len() != 4 {\n            return Err(\"Polygon must have exactly 4 vertices.\");\n        }\n\n        let mut total_area = 0.0;\n        let mut centroid_sum = Vector::new(0.0, 0.0, 0.0);\n\n        for i in 0..4 {\n            let p0 = &vertices[i];\n            let p1 = &vertices[(i + 1) % 4];\n            let p2 = &vertices[(i + 2) % 4];\n\n            let tri_area =\n                ((p0[0] * (p1[1] - p2[1])"
        }
      }
    },
    {
      "name": "Point.__jsondump__",
      "implementations": {
        "python": {
          "sig": "__jsondump__()",
          "code": "def __jsondump__(self):\n\n        \"\"\"Serialize to polymorphic JSON format with type field.\n\n        Returns\n        -------\n        dict\n            Dictionary with 'type', 'guid', 'name', and object fields.\n\n        \"\"\"\n        return {\n            \"type\": f\"{self.__class__.__name__}\",\n            \"guid\": self.guid,"
        }
      }
    },
    {
      "name": "Point.__jsonload__",
      "implementations": {
        "python": {
          "sig": "__jsonload__(cls, data, guid=None, name=None)",
          "code": "def __jsonload__(cls, data, guid=None, name=None):\n\n        \"\"\"Deserialize from polymorphic JSON format.\"\"\"\n        from .encoders import decode_node\n\n        pt = cls(data[\"x\"], data[\"y\"], data[\"z\"])\n        pt.width = data.get(\"width\", 1.0)\n\n        # Decode nested color (supports polymorphic dicts and plain values)\n        pt.pointcolor = decode_node(data.get(\"pointcolor\"))\n\n        # Always assign metadata (per project convention)\n        pt.guid = guid"
        }
      }
    },
    {
      "name": "Point.to_protobuf",
      "implementations": {
        "python": {
          "sig": "to_protobuf()",
          "code": "def to_protobuf(self):\n\n        \"\"\"Convert to protobuf binary format.\"\"\"\n        from .proto import point_pb2\n        \n        proto = point_pb2.Point()\n        proto.guid = self.guid\n        proto.name = self.name\n        proto.x = self[0]\n        proto.y = self[1]\n        proto.z = self[2]\n        proto.width = self.width"
        },
        "cpp": {
          "sig": "std::string to_protobuf()",
          "code": "std::string Point::to_protobuf() const {\n  session_proto::Point proto;\n  proto.set_guid(guid);\n  proto.set_name(name);\n  proto.set_x(_x);\n  proto.set_y(_y);\n  proto.set_z(_z);\n  proto.set_width(width);\n  \n  // Set color (no guid in proto schema)\n  auto* color_proto = proto.mutable_pointcolor();\n  color_proto->set_name(pointcolor.name);\n  color_proto->set_r(pointcolor.r);\n  color_proto->set_g(pointcolor.g);\n  color_proto->set_b(pointcolor.b);\n  color_proto->set_a(pointcolor.a);\n  \n  // Set xform"
        },
        "rust": {
          "sig": "to_protobuf() -> Vec<u8>",
          "code": "pub fn to_protobuf(&self) -> Vec<u8> {\n        use prost::Message;\n        \n        let proto = crate::proto::Point {\n            guid: self.guid.clone(),\n            name: self.name.clone(),\n            x: self._x,\n            y: self._y,\n            z: self._z,\n            width: self.width,\n            pointcolor: Some(crate::proto::Color {\n                guid: self.pointcolor.guid.clone(),\n                name: self.pointcolor.name.clone(),\n                r: self.pointcolor.r as i32,"
        }
      }
    },
    {
      "name": "Point.from_protobuf",
      "implementations": {
        "python": {
          "sig": "from_protobuf(cls, data)",
          "code": "def from_protobuf(cls, data):\n\n        \"\"\"Create Point from protobuf binary data.\"\"\"\n        from .proto import point_pb2\n        from .color import Color\n        from .xform import Xform\n        \n        proto = point_pb2.Point()\n        proto.ParseFromString(data)\n        \n        pt = cls(proto.x, proto.y, proto.z)\n        pt.guid = proto.guid\n        pt.name = proto.name"
        },
        "cpp": {
          "sig": "Point from_protobuf(const std::string& data)",
          "code": "Point Point::from_protobuf(const std::string& data) {\n  session_proto::Point proto;\n  proto.ParseFromString(data);\n  \n  Point point(proto.x(), proto.y(), proto.z());\n  point.guid = proto.guid();\n  point.name = proto.name();\n  point.width = proto.width();\n  \n  // Load color (no guid in proto schema)\n  const auto& color_proto = proto.pointcolor();\n  point.pointcolor.name = color_proto.name();\n  point.pointcolor.r = color_proto.r();\n  point.pointcolor.g = color_proto.g();\n  point.pointcolor.b = co"
        },
        "rust": {
          "sig": "from_protobuf(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>>",
          "code": "pub fn from_protobuf(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {\n        use prost::Message;\n        \n        let proto = crate::proto::Point::decode(data)?;\n        \n        let mut pt = Self::new(proto.x, proto.y, proto.z);\n        pt.guid = proto.guid;\n        pt.name = proto.name;\n        pt.width = proto.width;\n        \n        if let Some(color) = proto.pointcolor {\n            pt.pointcolor.name = color.name;\n            pt.pointcolor.r = color.r as u8;\n            pt.p"
        }
      }
    },
    {
      "name": "Point.protobuf_dump",
      "implementations": {
        "python": {
          "sig": "protobuf_dump(filepath)",
          "code": "def protobuf_dump(self, filepath):\n\n        \"\"\"Write protobuf to file.\"\"\"\n        data = self.to_protobuf()\n        with open(filepath, 'wb') as f:\n            f.write(data)\n\n    @classmethod\n    def protobuf_load(cls, filepath):\n        \"\"\"Read protobuf from file.\"\"\"\n        with open(filepath, 'rb') as f:\n            data = f.read()\n        return cls.from_protobuf(data)"
        },
        "cpp": {
          "sig": "void protobuf_dump(const std::string& filename)",
          "code": "void Point::protobuf_dump(const std::string& filename) const {\n  std::string data = to_protobuf();\n  std::ofstream file(filename, std::ios::binary);\n  file.write(data.data(), data.size());\n}"
        },
        "rust": {
          "sig": "protobuf_dump(filepath: &str)",
          "code": "pub fn protobuf_dump(&self, filepath: &str) {\n        let data = self.to_protobuf();\n        std::fs::write(filepath, data).expect(\"Failed to write protobuf file\");\n    }"
        }
      }
    },
    {
      "name": "Point.protobuf_load",
      "implementations": {
        "python": {
          "sig": "protobuf_load(cls, filepath)",
          "code": "def protobuf_load(cls, filepath):\n\n        \"\"\"Read protobuf from file.\"\"\"\n        with open(filepath, 'rb') as f:\n            data = f.read()\n        return cls.from_protobuf(data)"
        },
        "cpp": {
          "sig": "Point protobuf_load(const std::string& filename)",
          "code": "Point Point::protobuf_load(const std::string& filename) {\n  std::ifstream file(filename, std::ios::binary);\n  std::string data((std::istreambuf_iterator<char>(file)),\n                    std::istreambuf_iterator<char>());\n  return from_protobuf(data);\n}"
        },
        "rust": {
          "sig": "protobuf_load(filepath: &str) -> Self",
          "code": "pub fn protobuf_load(filepath: &str) -> Self {\n        let data = std::fs::read(filepath).expect(\"Failed to read protobuf file\");\n        Self::from_protobuf(&data).expect(\"Failed to parse protobuf\")\n    }"
        }
      }
    },
    {
      "name": "Color.__init__",
      "implementations": {
        "python": {
          "sig": "__init__(r: int, g: int, b: int, a: int, name: str = \"my_color\")",
          "code": "def __init__(self, r: int, g: int, b: int, a: int, name: str = \"my_color\"):\n\n        self.guid = str(uuid.uuid4())\n        self.name = name\n        self._r = int(r)\n        self._g = int(g)\n        self._b = int(b)\n        self._a = int(a)\n\n    ###########################################################################################\n    # Operators\n    ###########################################################################################"
        }
      }
    },
    {
      "name": "Color.__deepcopy__",
      "implementations": {
        "python": {
          "sig": "__deepcopy__(memo)",
          "code": "def __deepcopy__(self, memo):\n\n\n        cls = self.__class__\n        result = cls.__new__(cls)\n        memo[id(self)] = result\n\n        # New guid\n        result.guid = str(uuid.uuid4())\n\n        # Copy remaining fields\n        result.name = copy.deepcopy(self.name, memo)\n        result._r = self._r"
        }
      }
    },
    {
      "name": "Color.duplicate",
      "implementations": {
        "python": {
          "sig": "duplicate()",
          "code": "def duplicate(self) -> \"Color\":\n\n        \"\"\"Duplicate the color.\"\"\"\n        return copy.deepcopy(self)\n\n    def __str__(self) -> str:\n        \"\"\"String representation.\"\"\"\n        return f\"{self[0]}, {self[1]}, {self[2]}, {self[3]}\"\n\n    def __repr__(self) -> str:\n        return f\"Color({self.name}, {self[0]}, {self[1]}, {self[2]}, {self[3]})\"\n\n    def __eq__(self, other) -> bool:"
        },
        "cpp": {
          "sig": "Color duplicate()",
          "code": "Color Color::duplicate() const {\n  return Color(*this);\n}"
        },
        "rust": {
          "sig": "duplicate() -> Self",
          "code": "pub fn duplicate(&self) -> Self {\n        Color {\n            guid: Uuid::new_v4().to_string(),\n            name: self.name.clone(),\n            r: self.r,\n            g: self.g,\n            b: self.b,\n            a: self.a,\n        }\n    }"
        }
      }
    },
    {
      "name": "Color.__str__",
      "implementations": {
        "python": {
          "sig": "__str__()",
          "code": "def __str__(self) -> str:\n\n        \"\"\"String representation.\"\"\"\n        return f\"{self[0]}, {self[1]}, {self[2]}, {self[3]}\"\n\n    def __repr__(self) -> str:\n        return f\"Color({self.name}, {self[0]}, {self[1]}, {self[2]}, {self[3]})\"\n\n    def __eq__(self, other) -> bool:\n        if not isinstance(other, Color):\n            return False\n        return (\n            self.name == other.name"
        }
      }
    },
    {
      "name": "Color.__repr__",
      "implementations": {
        "python": {
          "sig": "__repr__()",
          "code": "def __repr__(self) -> str:\n\n        return f\"Color({self.name}, {self[0]}, {self[1]}, {self[2]}, {self[3]})\"\n\n    def __eq__(self, other) -> bool:\n        if not isinstance(other, Color):\n            return False\n        return (\n            self.name == other.name\n            and self[0] == other[0]\n            and self[1] == other[1]\n            and self[2] == other[2]\n            and self[3] == other[3]"
        }
      }
    },
    {
      "name": "Color.__eq__",
      "implementations": {
        "python": {
          "sig": "__eq__(other)",
          "code": "def __eq__(self, other) -> bool:\n\n        if not isinstance(other, Color):\n            return False\n        return (\n            self.name == other.name\n            and self[0] == other[0]\n            and self[1] == other[1]\n            and self[2] == other[2]\n            and self[3] == other[3]\n        )\n\n    def __ne__(self, other) -> bool:"
        }
      }
    },
    {
      "name": "Color.__ne__",
      "implementations": {
        "python": {
          "sig": "__ne__(other)",
          "code": "def __ne__(self, other) -> bool:\n\n        return not self == other\n\n    ###########################################################################################\n    # No-copy Operators\n    ###########################################################################################\n\n    def __getitem__(self, index):\n        if index == 0:\n            return self._r\n        elif index == 1:\n            return self._g"
        }
      }
    },
    {
      "name": "Color.__getitem__",
      "implementations": {
        "python": {
          "sig": "__getitem__(index)",
          "code": "def __getitem__(self, index):\n\n        if index == 0:\n            return self._r\n        elif index == 1:\n            return self._g\n        elif index == 2:\n            return self._b\n        elif index == 3:\n            return self._a\n        else:\n            raise IndexError(\"Index out of range\")"
        }
      }
    },
    {
      "name": "Color.__setitem__",
      "implementations": {
        "python": {
          "sig": "__setitem__(index, value)",
          "code": "def __setitem__(self, index, value):\n\n        if index == 0:\n            self._r = value\n        elif index == 1:\n            self._g = value\n        elif index == 2:\n            self._b = value\n        elif index == 3:\n            self._a = value\n        else:\n            raise IndexError(\"Index out of range\")"
        }
      }
    },
    {
      "name": "Color.to_unified_array",
      "implementations": {
        "python": {
          "sig": "to_unified_array()",
          "code": "def to_unified_array(self) -> list[float]:\n\n        \"\"\"Convert to normalized float array [0-1] (matches Rust implementation).\"\"\"\n        return [self[0] / 255.0, self[1] / 255.0, self[2] / 255.0, self[3] / 255.0]\n\n    @classmethod\n    def from_unified_array(cls, arr) -> \"Color\":\n        \"\"\"Create color from normalized float values [0-1].\"\"\"\n        return cls(int(arr[0] * 255.0 + 0.5), int(arr[1] * 255.0 + 0.5), int(arr[2] * 255.0 + 0.5), int(arr[3] * 255.0 + 0.5))\n\n    ###########################################################################################\n    # Presets\n    ###########################################################################################"
        }
      }
    },
    {
      "name": "Color.from_unified_array",
      "implementations": {
        "python": {
          "sig": "from_unified_array(cls, arr)",
          "code": "def from_unified_array(cls, arr) -> \"Color\":\n\n        \"\"\"Create color from normalized float values [0-1].\"\"\"\n        return cls(int(arr[0] * 255.0 + 0.5), int(arr[1] * 255.0 + 0.5), int(arr[2] * 255.0 + 0.5), int(arr[3] * 255.0 + 0.5))\n\n    ###########################################################################################\n    # Presets\n    ###########################################################################################\n\n    @classmethod\n    def white(cls) -> \"Color\":\n        \"\"\"Create a white color.\"\"\"\n        color = cls(255, 255, 255, 255)"
        },
        "cpp": {
          "sig": "Color from_unified_array(std::array<double, 4> arr)",
          "code": "Color Color::from_unified_array(std::array<double, 4> arr) {\n  return Color(static_cast<unsigned int>(arr[0] * 255.0 + 0.5),\n               static_cast<unsigned int>(arr[1] * 255.0 + 0.5),\n               static_cast<unsigned int>(arr[2] * 255.0 + 0.5),\n               static_cast<unsigned int>(arr[3] * 255.0 + 0.5));\n}"
        }
      }
    },
    {
      "name": "Color.white",
      "implementations": {
        "python": {
          "sig": "white(cls)",
          "code": "def white(cls) -> \"Color\":\n\n        \"\"\"Create a white color.\"\"\"\n        color = cls(255, 255, 255, 255)\n        color.name = \"white\"\n        return color\n\n    @classmethod\n    def black(cls) -> \"Color\":\n        \"\"\"Create a black color.\"\"\"\n        color = cls(0, 0, 0, 255)\n        color.name = \"black\"\n        return color"
        },
        "cpp": {
          "sig": "Color white()",
          "code": "Color Color::white() { return Color(255, 255, 255, 255, \"white\"); }"
        },
        "rust": {
          "sig": "white() -> Self",
          "code": "pub fn white() -> Self {\n        let mut color = Color::new(255, 255, 255, 255);\n        color.name = \"white\".to_string();\n        color\n    }"
        }
      }
    },
    {
      "name": "Color.black",
      "implementations": {
        "python": {
          "sig": "black(cls)",
          "code": "def black(cls) -> \"Color\":\n\n        \"\"\"Create a black color.\"\"\"\n        color = cls(0, 0, 0, 255)\n        color.name = \"black\"\n        return color\n\n    @classmethod\n    def grey(cls) -> \"Color\":\n        \"\"\"Create a grey color.\"\"\"\n        color = cls(128, 128, 128, 255)\n        color.name = \"grey\"\n        return color"
        },
        "cpp": {
          "sig": "Color black()",
          "code": "Color Color::black() { return Color(0, 0, 0, 255, \"black\"); }"
        },
        "rust": {
          "sig": "black() -> Self",
          "code": "pub fn black() -> Self {\n        let mut color = Color::new(0, 0, 0, 255);\n        color.name = \"black\".to_string();\n        color\n    }"
        }
      }
    },
    {
      "name": "Color.grey",
      "implementations": {
        "python": {
          "sig": "grey(cls)",
          "code": "def grey(cls) -> \"Color\":\n\n        \"\"\"Create a grey color.\"\"\"\n        color = cls(128, 128, 128, 255)\n        color.name = \"grey\"\n        return color\n\n    @classmethod\n    def red(cls) -> \"Color\":\n        \"\"\"Create a red color.\"\"\"\n        color = cls(255, 0, 0, 255)\n        color.name = \"red\"\n        return color"
        },
        "cpp": {
          "sig": "Color grey()",
          "code": "Color Color::grey() { return Color(128, 128, 128, 255, \"grey\"); }"
        },
        "rust": {
          "sig": "grey() -> Self",
          "code": "pub fn grey() -> Self {\n        let mut color = Color::new(128, 128, 128, 255);\n        color.name = \"grey\".to_string();\n        color\n    }"
        }
      }
    },
    {
      "name": "Color.red",
      "implementations": {
        "python": {
          "sig": "red(cls)",
          "code": "def red(cls) -> \"Color\":\n\n        \"\"\"Create a red color.\"\"\"\n        color = cls(255, 0, 0, 255)\n        color.name = \"red\"\n        return color\n\n    @classmethod\n    def orange(cls) -> \"Color\":\n        \"\"\"Create an orange color.\"\"\"\n        color = cls(255, 128, 0, 255)\n        color.name = \"orange\"\n        return color"
        },
        "cpp": {
          "sig": "Color red()",
          "code": "Color Color::red() { return Color(255, 0, 0, 255, \"red\"); }"
        },
        "rust": {
          "sig": "red() -> Self",
          "code": "pub fn red() -> Self {\n        let mut color = Color::new(255, 0, 0, 255);\n        color.name = \"red\".to_string();\n        color\n    }"
        }
      }
    },
    {
      "name": "Color.orange",
      "implementations": {
        "python": {
          "sig": "orange(cls)",
          "code": "def orange(cls) -> \"Color\":\n\n        \"\"\"Create an orange color.\"\"\"\n        color = cls(255, 128, 0, 255)\n        color.name = \"orange\"\n        return color\n\n    @classmethod\n    def yellow(cls) -> \"Color\":\n        \"\"\"Create a yellow color.\"\"\"\n        color = cls(255, 255, 0, 255)\n        color.name = \"yellow\"\n        return color"
        },
        "cpp": {
          "sig": "Color orange()",
          "code": "Color Color::orange() { return Color(255, 128, 0, 255, \"orange\"); }"
        },
        "rust": {
          "sig": "orange() -> Self",
          "code": "pub fn orange() -> Self {\n        let mut color = Color::new(255, 128, 0, 255);\n        color.name = \"orange\".to_string();\n        color\n    }"
        }
      }
    },
    {
      "name": "Color.yellow",
      "implementations": {
        "python": {
          "sig": "yellow(cls)",
          "code": "def yellow(cls) -> \"Color\":\n\n        \"\"\"Create a yellow color.\"\"\"\n        color = cls(255, 255, 0, 255)\n        color.name = \"yellow\"\n        return color\n\n    @classmethod\n    def lime(cls) -> \"Color\":\n        \"\"\"Create a lime color.\"\"\"\n        color = cls(128, 255, 0, 255)\n        color.name = \"lime\"\n        return color"
        },
        "cpp": {
          "sig": "Color yellow()",
          "code": "Color Color::yellow() { return Color(255, 255, 0, 255, \"yellow\"); }"
        },
        "rust": {
          "sig": "yellow() -> Self",
          "code": "pub fn yellow() -> Self {\n        let mut color = Color::new(255, 255, 0, 255);\n        color.name = \"yellow\".to_string();\n        color\n    }"
        }
      }
    },
    {
      "name": "Color.lime",
      "implementations": {
        "python": {
          "sig": "lime(cls)",
          "code": "def lime(cls) -> \"Color\":\n\n        \"\"\"Create a lime color.\"\"\"\n        color = cls(128, 255, 0, 255)\n        color.name = \"lime\"\n        return color\n\n    @classmethod\n    def green(cls) -> \"Color\":\n        \"\"\"Create a green color.\"\"\"\n        color = cls(0, 255, 0, 255)\n        color.name = \"green\"\n        return color"
        },
        "cpp": {
          "sig": "Color lime()",
          "code": "Color Color::lime() { return Color(128, 255, 0, 255, \"lime\"); }"
        },
        "rust": {
          "sig": "lime() -> Self",
          "code": "pub fn lime() -> Self {\n        let mut color = Color::new(128, 255, 0, 255);\n        color.name = \"lime\".to_string();\n        color\n    }"
        }
      }
    },
    {
      "name": "Color.green",
      "implementations": {
        "python": {
          "sig": "green(cls)",
          "code": "def green(cls) -> \"Color\":\n\n        \"\"\"Create a green color.\"\"\"\n        color = cls(0, 255, 0, 255)\n        color.name = \"green\"\n        return color\n\n    @classmethod\n    def mint(cls) -> \"Color\":\n        \"\"\"Create a mint color.\"\"\"\n        color = cls(0, 255, 128, 255)\n        color.name = \"mint\"\n        return color"
        },
        "cpp": {
          "sig": "Color green()",
          "code": "Color Color::green() { return Color(0, 255, 0, 255, \"green\"); }"
        },
        "rust": {
          "sig": "green() -> Self",
          "code": "pub fn green() -> Self {\n        let mut color = Color::new(0, 255, 0, 255);\n        color.name = \"green\".to_string();\n        color\n    }"
        }
      }
    },
    {
      "name": "Color.mint",
      "implementations": {
        "python": {
          "sig": "mint(cls)",
          "code": "def mint(cls) -> \"Color\":\n\n        \"\"\"Create a mint color.\"\"\"\n        color = cls(0, 255, 128, 255)\n        color.name = \"mint\"\n        return color\n\n    @classmethod\n    def cyan(cls) -> \"Color\":\n        \"\"\"Create a cyan color.\"\"\"\n        color = cls(0, 255, 255, 255)\n        color.name = \"cyan\"\n        return color"
        },
        "cpp": {
          "sig": "Color mint()",
          "code": "Color Color::mint() { return Color(0, 255, 128, 255, \"mint\"); }"
        },
        "rust": {
          "sig": "mint() -> Self",
          "code": "pub fn mint() -> Self {\n        let mut color = Color::new(0, 255, 128, 255);\n        color.name = \"mint\".to_string();\n        color\n    }"
        }
      }
    },
    {
      "name": "Color.cyan",
      "implementations": {
        "python": {
          "sig": "cyan(cls)",
          "code": "def cyan(cls) -> \"Color\":\n\n        \"\"\"Create a cyan color.\"\"\"\n        color = cls(0, 255, 255, 255)\n        color.name = \"cyan\"\n        return color\n\n    @classmethod\n    def azure(cls) -> \"Color\":\n        \"\"\"Create an azure color.\"\"\"\n        color = cls(0, 128, 255, 255)\n        color.name = \"azure\"\n        return color"
        },
        "cpp": {
          "sig": "Color cyan()",
          "code": "Color Color::cyan() { return Color(0, 255, 255, 255, \"cyan\"); }"
        },
        "rust": {
          "sig": "cyan() -> Self",
          "code": "pub fn cyan() -> Self {\n        let mut color = Color::new(0, 255, 255, 255);\n        color.name = \"cyan\".to_string();\n        color\n    }"
        }
      }
    },
    {
      "name": "Color.azure",
      "implementations": {
        "python": {
          "sig": "azure(cls)",
          "code": "def azure(cls) -> \"Color\":\n\n        \"\"\"Create an azure color.\"\"\"\n        color = cls(0, 128, 255, 255)\n        color.name = \"azure\"\n        return color\n\n    @classmethod\n    def blue(cls) -> \"Color\":\n        \"\"\"Create a blue color.\"\"\"\n        color = cls(0, 0, 255, 255)\n        color.name = \"blue\"\n        return color"
        },
        "cpp": {
          "sig": "Color azure()",
          "code": "Color Color::azure() { return Color(0, 128, 255, 255, \"azure\"); }"
        },
        "rust": {
          "sig": "azure() -> Self",
          "code": "pub fn azure() -> Self {\n        let mut color = Color::new(0, 128, 255, 255);\n        color.name = \"azure\".to_string();\n        color\n    }"
        }
      }
    },
    {
      "name": "Color.blue",
      "implementations": {
        "python": {
          "sig": "blue(cls)",
          "code": "def blue(cls) -> \"Color\":\n\n        \"\"\"Create a blue color.\"\"\"\n        color = cls(0, 0, 255, 255)\n        color.name = \"blue\"\n        return color\n\n    @classmethod\n    def violet(cls) -> \"Color\":\n        \"\"\"Create a violet color.\"\"\"\n        color = cls(128, 0, 255, 255)\n        color.name = \"violet\"\n        return color"
        },
        "cpp": {
          "sig": "Color blue()",
          "code": "Color Color::blue() { return Color(0, 0, 255, 255, \"blue\"); }"
        },
        "rust": {
          "sig": "blue() -> Self",
          "code": "pub fn blue() -> Self {\n        let mut color = Color::new(0, 0, 255, 255);\n        color.name = \"blue\".to_string();\n        color\n    }"
        }
      }
    },
    {
      "name": "Color.violet",
      "implementations": {
        "python": {
          "sig": "violet(cls)",
          "code": "def violet(cls) -> \"Color\":\n\n        \"\"\"Create a violet color.\"\"\"\n        color = cls(128, 0, 255, 255)\n        color.name = \"violet\"\n        return color\n\n    @classmethod\n    def magenta(cls) -> \"Color\":\n        \"\"\"Create a magenta color.\"\"\"\n        color = cls(255, 0, 255, 255)\n        color.name = \"magenta\"\n        return color"
        },
        "cpp": {
          "sig": "Color violet()",
          "code": "Color Color::violet() { return Color(128, 0, 255, 255, \"violet\"); }"
        },
        "rust": {
          "sig": "violet() -> Self",
          "code": "pub fn violet() -> Self {\n        let mut color = Color::new(128, 0, 255, 255);\n        color.name = \"violet\".to_string();\n        color\n    }"
        }
      }
    },
    {
      "name": "Color.magenta",
      "implementations": {
        "python": {
          "sig": "magenta(cls)",
          "code": "def magenta(cls) -> \"Color\":\n\n        \"\"\"Create a magenta color.\"\"\"\n        color = cls(255, 0, 255, 255)\n        color.name = \"magenta\"\n        return color\n\n    @classmethod\n    def pink(cls) -> \"Color\":\n        \"\"\"Create a pink color.\"\"\"\n        color = cls(255, 0, 128, 255)\n        color.name = \"pink\"\n        return color"
        },
        "cpp": {
          "sig": "Color magenta()",
          "code": "Color Color::magenta() { return Color(255, 0, 255, 255, \"magenta\"); }"
        },
        "rust": {
          "sig": "magenta() -> Self",
          "code": "pub fn magenta() -> Self {\n        let mut color = Color::new(255, 0, 255, 255);\n        color.name = \"magenta\".to_string();\n        color\n    }"
        }
      }
    },
    {
      "name": "Color.pink",
      "implementations": {
        "python": {
          "sig": "pink(cls)",
          "code": "def pink(cls) -> \"Color\":\n\n        \"\"\"Create a pink color.\"\"\"\n        color = cls(255, 0, 128, 255)\n        color.name = \"pink\"\n        return color\n\n    @classmethod\n    def maroon(cls) -> \"Color\":\n        \"\"\"Create a maroon color.\"\"\"\n        color = cls(128, 0, 0, 255)\n        color.name = \"maroon\"\n        return color"
        },
        "cpp": {
          "sig": "Color pink()",
          "code": "Color Color::pink() { return Color(255, 0, 128, 255, \"pink\"); }"
        },
        "rust": {
          "sig": "pink() -> Self",
          "code": "pub fn pink() -> Self {\n        let mut color = Color::new(255, 0, 128, 255);\n        color.name = \"pink\".to_string();\n        color\n    }"
        }
      }
    },
    {
      "name": "Color.maroon",
      "implementations": {
        "python": {
          "sig": "maroon(cls)",
          "code": "def maroon(cls) -> \"Color\":\n\n        \"\"\"Create a maroon color.\"\"\"\n        color = cls(128, 0, 0, 255)\n        color.name = \"maroon\"\n        return color\n\n    @classmethod\n    def brown(cls) -> \"Color\":\n        \"\"\"Create a brown color.\"\"\"\n        color = cls(128, 64, 0, 255)\n        color.name = \"brown\"\n        return color"
        },
        "cpp": {
          "sig": "Color maroon()",
          "code": "Color Color::maroon() { return Color(128, 0, 0, 255, \"maroon\"); }"
        },
        "rust": {
          "sig": "maroon() -> Self",
          "code": "pub fn maroon() -> Self {\n        let mut color = Color::new(128, 0, 0, 255);\n        color.name = \"maroon\".to_string();\n        color\n    }"
        }
      }
    },
    {
      "name": "Color.brown",
      "implementations": {
        "python": {
          "sig": "brown(cls)",
          "code": "def brown(cls) -> \"Color\":\n\n        \"\"\"Create a brown color.\"\"\"\n        color = cls(128, 64, 0, 255)\n        color.name = \"brown\"\n        return color\n\n    @classmethod\n    def olive(cls) -> \"Color\":\n        \"\"\"Create an olive color.\"\"\"\n        color = cls(128, 128, 0, 255)\n        color.name = \"olive\"\n        return color"
        },
        "cpp": {
          "sig": "Color brown()",
          "code": "Color Color::brown() { return Color(128, 64, 0, 255, \"brown\"); }"
        },
        "rust": {
          "sig": "brown() -> Self",
          "code": "pub fn brown() -> Self {\n        let mut color = Color::new(128, 64, 0, 255);\n        color.name = \"brown\".to_string();\n        color\n    }"
        }
      }
    },
    {
      "name": "Color.olive",
      "implementations": {
        "python": {
          "sig": "olive(cls)",
          "code": "def olive(cls) -> \"Color\":\n\n        \"\"\"Create an olive color.\"\"\"\n        color = cls(128, 128, 0, 255)\n        color.name = \"olive\"\n        return color\n\n    @classmethod\n    def teal(cls) -> \"Color\":\n        \"\"\"Create a teal color.\"\"\"\n        color = cls(0, 128, 128, 255)\n        color.name = \"teal\"\n        return color"
        },
        "cpp": {
          "sig": "Color olive()",
          "code": "Color Color::olive() { return Color(128, 128, 0, 255, \"olive\"); }"
        },
        "rust": {
          "sig": "olive() -> Self",
          "code": "pub fn olive() -> Self {\n        let mut color = Color::new(128, 128, 0, 255);\n        color.name = \"olive\".to_string();\n        color\n    }"
        }
      }
    },
    {
      "name": "Color.teal",
      "implementations": {
        "python": {
          "sig": "teal(cls)",
          "code": "def teal(cls) -> \"Color\":\n\n        \"\"\"Create a teal color.\"\"\"\n        color = cls(0, 128, 128, 255)\n        color.name = \"teal\"\n        return color\n\n    @classmethod\n    def navy(cls) -> \"Color\":\n        \"\"\"Create a navy color.\"\"\"\n        color = cls(0, 0, 128, 255)\n        color.name = \"navy\"\n        return color"
        },
        "cpp": {
          "sig": "Color teal()",
          "code": "Color Color::teal() { return Color(0, 128, 128, 255, \"teal\"); }"
        },
        "rust": {
          "sig": "teal() -> Self",
          "code": "pub fn teal() -> Self {\n        let mut color = Color::new(0, 128, 128, 255);\n        color.name = \"teal\".to_string();\n        color\n    }"
        }
      }
    },
    {
      "name": "Color.navy",
      "implementations": {
        "python": {
          "sig": "navy(cls)",
          "code": "def navy(cls) -> \"Color\":\n\n        \"\"\"Create a navy color.\"\"\"\n        color = cls(0, 0, 128, 255)\n        color.name = \"navy\"\n        return color\n\n    @classmethod\n    def purple(cls) -> \"Color\":\n        \"\"\"Create a purple color.\"\"\"\n        color = cls(128, 0, 128, 255)\n        color.name = \"purple\"\n        return color"
        },
        "cpp": {
          "sig": "Color navy()",
          "code": "Color Color::navy() { return Color(0, 0, 128, 255, \"navy\"); }"
        },
        "rust": {
          "sig": "navy() -> Self",
          "code": "pub fn navy() -> Self {\n        let mut color = Color::new(0, 0, 128, 255);\n        color.name = \"navy\".to_string();\n        color\n    }"
        }
      }
    },
    {
      "name": "Color.purple",
      "implementations": {
        "python": {
          "sig": "purple(cls)",
          "code": "def purple(cls) -> \"Color\":\n\n        \"\"\"Create a purple color.\"\"\"\n        color = cls(128, 0, 128, 255)\n        color.name = \"purple\"\n        return color\n\n    @classmethod\n    def silver(cls) -> \"Color\":\n        \"\"\"Create a silver color.\"\"\"\n        color = cls(192, 192, 192, 255)\n        color.name = \"silver\"\n        return color"
        },
        "cpp": {
          "sig": "Color purple()",
          "code": "Color Color::purple() { return Color(128, 0, 128, 255, \"purple\"); }"
        },
        "rust": {
          "sig": "purple() -> Self",
          "code": "pub fn purple() -> Self {\n        let mut color = Color::new(128, 0, 128, 255);\n        color.name = \"purple\".to_string();\n        color\n    }"
        }
      }
    },
    {
      "name": "Color.silver",
      "implementations": {
        "python": {
          "sig": "silver(cls)",
          "code": "def silver(cls) -> \"Color\":\n\n        \"\"\"Create a silver color.\"\"\"\n        color = cls(192, 192, 192, 255)\n        color.name = \"silver\"\n        return color\n\n    ###########################################################################################\n    # JSON Serialization\n    ###########################################################################################\n\n    def __jsondump__(self):\n        \"\"\"Serialize to polymorphic JSON format with type field.\"\"\""
        },
        "cpp": {
          "sig": "Color silver()",
          "code": "Color Color::silver() { return Color(192, 192, 192, 255, \"silver\"); }"
        },
        "rust": {
          "sig": "silver() -> Self",
          "code": "pub fn silver() -> Self {\n        let mut color = Color::new(192, 192, 192, 255);\n        color.name = \"silver\".to_string();\n        color\n    }"
        }
      }
    },
    {
      "name": "Color.__jsondump__",
      "implementations": {
        "python": {
          "sig": "__jsondump__()",
          "code": "def __jsondump__(self):\n\n        \"\"\"Serialize to polymorphic JSON format with type field.\"\"\"\n        return {\n            \"type\": f\"{self.__class__.__name__}\",\n            \"guid\": self.guid,\n            \"name\": self.name,\n            \"r\": self[0],\n            \"g\": self[1],\n            \"b\": self[2],\n            \"a\": self[3],\n        }"
        }
      }
    },
    {
      "name": "Color.__jsonload__",
      "implementations": {
        "python": {
          "sig": "__jsonload__(cls, data, guid=None, name=None)",
          "code": "def __jsonload__(cls, data, guid=None, name=None):\n\n        \"\"\"Deserialize from polymorphic JSON format.\"\"\"\n        color = cls(data[\"r\"], data[\"g\"], data[\"b\"], data.get(\"a\", 255))\n        color.guid = guid\n        color.name = name\n        return color\n\n    ###########################################################################################\n    # Protobuf Serialization\n    ###########################################################################################\n\n    def to_protobuf(self):"
        }
      }
    },
    {
      "name": "Color.to_protobuf",
      "implementations": {
        "python": {
          "sig": "to_protobuf()",
          "code": "def to_protobuf(self):\n\n        \"\"\"Convert to protobuf binary format.\"\"\"\n        if not _HAS_PROTOBUF:\n            raise ImportError(\"protobuf not available\")\n        proto = color_pb2.Color()\n        proto.guid = self.guid\n        proto.name = self.name\n        proto.r = self[0]\n        proto.g = self[1]\n        proto.b = self[2]\n        proto.a = self[3]\n        return proto.SerializeToString()"
        },
        "cpp": {
          "sig": "std::string to_protobuf()",
          "code": "std::string Color::to_protobuf() const {\n  session_proto::Color proto;\n  proto.set_guid(guid);\n  proto.set_name(name);\n  proto.set_r(r);\n  proto.set_g(g);\n  proto.set_b(b);\n  proto.set_a(a);\n  return proto.SerializeAsString();\n}"
        },
        "rust": {
          "sig": "to_protobuf() -> Vec<u8>",
          "code": "pub fn to_protobuf(&self) -> Vec<u8> {\n        use prost::Message;\n        \n        let proto = crate::proto::Color {\n            guid: self.guid.clone(),\n            name: self.name.clone(),\n            r: self.r as i32,\n            g: self.g as i32,\n            b: self.b as i32,\n            a: self.a as i32,\n        };\n        proto.encode_to_vec()\n    }"
        }
      }
    },
    {
      "name": "Color.from_protobuf",
      "implementations": {
        "python": {
          "sig": "from_protobuf(cls, data)",
          "code": "def from_protobuf(cls, data):\n\n        \"\"\"Create color from protobuf binary data.\"\"\"\n        if not _HAS_PROTOBUF:\n            raise ImportError(\"protobuf not available\")\n        proto = color_pb2.Color()\n        proto.ParseFromString(data)\n        \n        color = cls(proto.r, proto.g, proto.b, proto.a)\n        color.guid = proto.guid\n        color.name = proto.name\n        return color"
        },
        "cpp": {
          "sig": "Color from_protobuf(const std::string& data)",
          "code": "Color Color::from_protobuf(const std::string& data) {\n  session_proto::Color proto;\n  proto.ParseFromString(data);\n  \n  Color color(proto.r(), proto.g(), proto.b(), proto.a(), proto.name());\n  color.guid = proto.guid();\n  return color;\n}"
        },
        "rust": {
          "sig": "from_protobuf(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>>",
          "code": "pub fn from_protobuf(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {\n        use prost::Message;\n        \n        let proto = crate::proto::Color::decode(data)?;\n        \n        let mut color = Self::new(proto.r as u8, proto.g as u8, proto.b as u8, proto.a as u8);\n        color.guid = proto.guid;\n        color.name = proto.name;\n        Ok(color)\n    }"
        }
      }
    },
    {
      "name": "Color.protobuf_dump",
      "implementations": {
        "python": {
          "sig": "protobuf_dump(filepath)",
          "code": "def protobuf_dump(self, filepath):\n\n        \"\"\"Write protobuf to file.\"\"\"\n        data = self.to_protobuf()\n        with open(filepath, 'wb') as f:\n            f.write(data)\n\n    @classmethod\n    def protobuf_load(cls, filepath):\n        \"\"\"Read protobuf from file.\"\"\"\n        with open(filepath, 'rb') as f:\n            data = f.read()\n        return cls.from_protobuf(data)"
        },
        "cpp": {
          "sig": "void protobuf_dump(const std::string& filename)",
          "code": "void Color::protobuf_dump(const std::string& filename) const {\n  std::string data = to_protobuf();\n  std::ofstream file(filename, std::ios::binary);\n  file.write(data.data(), data.size());\n}"
        },
        "rust": {
          "sig": "protobuf_dump(filepath: &str)",
          "code": "pub fn protobuf_dump(&self, filepath: &str) {\n        let data = self.to_protobuf();\n        std::fs::write(filepath, data).expect(\"Failed to write protobuf file\");\n    }"
        }
      }
    },
    {
      "name": "Color.protobuf_load",
      "implementations": {
        "python": {
          "sig": "protobuf_load(cls, filepath)",
          "code": "def protobuf_load(cls, filepath):\n\n        \"\"\"Read protobuf from file.\"\"\"\n        with open(filepath, 'rb') as f:\n            data = f.read()\n        return cls.from_protobuf(data)"
        },
        "cpp": {
          "sig": "Color protobuf_load(const std::string& filename)",
          "code": "Color Color::protobuf_load(const std::string& filename) {\n  std::ifstream file(filename, std::ios::binary);\n  std::string data((std::istreambuf_iterator<char>(file)),\n                    std::istreambuf_iterator<char>());\n  return from_protobuf(data);\n}"
        },
        "rust": {
          "sig": "protobuf_load(filepath: &str) -> Self",
          "code": "pub fn protobuf_load(filepath: &str) -> Self {\n        let data = std::fs::read(filepath).expect(\"Failed to read protobuf file\");\n        Self::from_protobuf(&data).expect(\"Failed to parse protobuf\")\n    }"
        }
      }
    },
    {
      "name": "Point.str",
      "implementations": {
        "cpp": {
          "sig": "std::string str()",
          "code": "std::string Point::str() const {\n  int prec = static_cast<int>(Tolerance::ROUNDING);\n  return fmt::format(\n      \"{}"
        },
        "rust": {
          "sig": "str() -> String",
          "code": "pub fn str(&self) -> String {\n        use crate::tolerance::TOL;\n        let prec = Some(crate::tolerance::Tolerance::ROUNDING);\n        format!(\n            \"{}, {}, {}\",\n            TOL.format_number(self._x, prec),\n            TOL.format_number(self._y, prec),\n            TOL.format_number(self._z, prec),\n        )\n    }"
        }
      }
    },
    {
      "name": "fmt.format",
      "implementations": {
        "cpp": {
          "sig": "return format(\"Color({}, {}, {}, {}, {})",
          "code": "return fmt::format(\"Color({}, {}, {}, {}, {})\", name, r, g, b, a);\n}"
        }
      }
    },
    {
      "name": "Point.repr",
      "implementations": {
        "cpp": {
          "sig": "std::string repr()",
          "code": "std::string Point::repr() const {\n  int prec = static_cast<int>(Tolerance::ROUNDING);\n  return fmt::format(\n      \"Point({}"
        },
        "rust": {
          "sig": "repr() -> String",
          "code": "pub fn repr(&self) -> String {\n        use crate::tolerance::TOL;\n        let prec = Some(crate::tolerance::Tolerance::ROUNDING);\n        format!(\n            \"Point({}, {}, {}, {}, Color({}, {}, {}, {}), {})\",\n            self.name,\n            TOL.format_number(self._x, prec),\n            TOL.format_number(self._y, prec),\n            TOL.format_number(self._z, prec),\n            self.pointcolor.r,\n            self.pointcolor.g,\n            self.pointcolor.b,\n            self.pointcolor.a,"
        }
      }
    },
    {
      "name": "Point.jsondump",
      "implementations": {
        "cpp": {
          "sig": "nlohmann::ordered_json jsondump()",
          "code": "nlohmann::ordered_json Point::jsondump() const {\n  auto clean_float = [](double val) -> double { return std::round(val * 100.0) / 100.0; }"
        },
        "rust": {
          "sig": "jsondump() -> Result<String, Box<dyn std::error::Error>>",
          "code": "pub fn jsondump(&self) -> Result<String, Box<dyn std::error::Error>> {\n        let mut buf = Vec::new();\n        let formatter = serde_json::ser::PrettyFormatter::with_indent(b\"    \");\n        let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);\n        SerTrait::serialize(self, &mut ser)?;\n        Ok(String::from_utf8(buf)?)\n    }"
        }
      }
    },
    {
      "name": "Point.jsonload",
      "implementations": {
        "cpp": {
          "sig": "Point jsonload(const nlohmann::json &data)",
          "code": "Point Point::jsonload(const nlohmann::json &data) {\n  Point point(data[\"x\"], data[\"y\"], data[\"z\"]);\n  point.guid = data[\"guid\"];\n  point.name = data[\"name\"];\n  point.pointcolor = Color::jsonload(data[\"pointcolor\"]);\n  point.width = data[\"width\"];\n  if (data.contains(\"xform\")) {\n    point.xform = Xform::jsonload(data[\"xform\"]);\n  }"
        },
        "rust": {
          "sig": "jsonload(json_data: &str) -> Result<Self, Box<dyn std::error::Error>>",
          "code": "pub fn jsonload(json_data: &str) -> Result<Self, Box<dyn std::error::Error>> {\n        Ok(serde_json::from_str(json_data)?)\n    }"
        }
      }
    },
    {
      "name": "Point.json_dump",
      "implementations": {
        "cpp": {
          "sig": "void json_dump(const std::string& filename)",
          "code": "void Point::json_dump(const std::string& filename) const {\n  std::ofstream file(filename);\n  file << jsondump().dump(4);\n}"
        }
      }
    },
    {
      "name": "Point.json_load",
      "implementations": {
        "cpp": {
          "sig": "Point json_load(const std::string& filename)",
          "code": "Point Point::json_load(const std::string& filename) {\n  std::ifstream file(filename);\n  nlohmann::json data = nlohmann::json::parse(file);\n  return jsonload(data);\n}"
        }
      }
    },
    {
      "name": "std.out_of_range",
      "implementations": {
        "cpp": {
          "sig": "throw out_of_range(\"Index out of range\")",
          "code": "throw std::out_of_range(\"Index out of range\");\n  }"
        }
      }
    },
    {
      "name": "Point.ccw",
      "implementations": {
        "cpp": {
          "sig": "bool ccw(const Point& a, const Point& b, const Point& c)",
          "code": "bool Point::ccw(const Point& a, const Point& b, const Point& c) {\n    return (c._y - a._y) * (b._x - a._x) > (b._y - a._y) * (c._x - a._x);\n}"
        },
        "rust": {
          "sig": "ccw(a: &Point, b: &Point, c: &Point) -> bool",
          "code": "pub fn ccw(a: &Point, b: &Point, c: &Point) -> bool {\n        (c._y - a._y) * (b._x - a._x) > (b._y - a._y) * (c._x - a._x)\n    }"
        }
      }
    },
    {
      "name": "std.abs",
      "implementations": {
        "cpp": {
          "sig": "return abs(area)",
          "code": "return std::abs(area) / 2.0;\n}"
        }
      }
    },
    {
      "name": "std.invalid_argument",
      "implementations": {
        "cpp": {
          "sig": "throw invalid_argument(\"Polygon must have exactly 4 vertices.\")",
          "code": "throw std::invalid_argument(\"Polygon must have exactly 4 vertices.\");\n    }"
        }
      }
    },
    {
      "name": "fmt.format_to",
      "implementations": {
        "cpp": {
          "sig": "return format_to(ctx.out()",
          "code": "return fmt::format_to(ctx.out(), \"{}"
        }
      }
    },
    {
      "name": "Color.str",
      "implementations": {
        "cpp": {
          "sig": "std::string str()",
          "code": "std::string Color::str() const {\n  return fmt::format(\"{}"
        },
        "rust": {
          "sig": "str() -> String",
          "code": "pub fn str(&self) -> String {\n        format!(\"{}, {}, {}, {}\", self.r, self.g, self.b, self.a)\n    }"
        }
      }
    },
    {
      "name": "Color.repr",
      "implementations": {
        "cpp": {
          "sig": "std::string repr()",
          "code": "std::string Color::repr() const {\n  return fmt::format(\"Color({}"
        },
        "rust": {
          "sig": "repr() -> String",
          "code": "pub fn repr(&self) -> String {\n        format!(\"Color({}, {}, {}, {}, {})\", self.name, self.r, self.g, self.b, self.a)\n    }"
        }
      }
    },
    {
      "name": "Color.to_string",
      "implementations": {
        "cpp": {
          "sig": "std::string to_string()",
          "code": "std::string Color::to_string() const {\n  return repr();\n}"
        }
      }
    },
    {
      "name": "Color.jsondump",
      "implementations": {
        "cpp": {
          "sig": "nlohmann::ordered_json jsondump()",
          "code": "nlohmann::ordered_json Color::jsondump() const {\n  return nlohmann::ordered_json{{\"type\", \"Color\"}"
        },
        "rust": {
          "sig": "jsondump() -> Result<String, Box<dyn std::error::Error>>",
          "code": "pub fn jsondump(&self) -> Result<String, Box<dyn std::error::Error>> {\n        let mut buf = Vec::new();\n        let formatter = serde_json::ser::PrettyFormatter::with_indent(b\"    \");\n        let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);\n        SerTrait::serialize(self, &mut ser)?;\n        Ok(String::from_utf8(buf)?)\n    }"
        }
      }
    },
    {
      "name": "Color.jsonload",
      "implementations": {
        "cpp": {
          "sig": "Color jsonload(const nlohmann::json &data)",
          "code": "Color Color::jsonload(const nlohmann::json &data) {\n  Color color(static_cast<unsigned int>(data[\"r\"]),\n                      static_cast<unsigned int>(data[\"g\"]),\n                      static_cast<unsigned int>(data[\"b\"]),\n                      static_cast<unsigned int>(data[\"a\"]), data[\"name\"]);\n  color.guid = data[\"guid\"];\n  return color;\n}"
        },
        "rust": {
          "sig": "jsonload(json_data: &str) -> Result<Self, Box<dyn std::error::Error>>",
          "code": "pub fn jsonload(json_data: &str) -> Result<Self, Box<dyn std::error::Error>> {\n        Ok(serde_json::from_str(json_data)?)\n    }"
        }
      }
    },
    {
      "name": "Color.json_dump",
      "implementations": {
        "cpp": {
          "sig": "void json_dump(const std::string& filename)",
          "code": "void Color::json_dump(const std::string& filename) const {\n  std::ofstream file(filename);\n  file << jsondump().dump(4);\n}"
        }
      }
    },
    {
      "name": "Color.json_load",
      "implementations": {
        "cpp": {
          "sig": "Color json_load(const std::string& filename)",
          "code": "Color Color::json_load(const std::string& filename) {\n  std::ifstream file(filename);\n  nlohmann::json data = nlohmann::json::parse(file);\n  return jsonload(data);\n}"
        }
      }
    },
    {
      "name": "Point.new",
      "implementations": {
        "rust": {
          "sig": "new(x: f64, y: f64, z: f64) -> Self",
          "code": "pub fn new(x: f64, y: f64, z: f64) -> Self {\n        Self {\n            _x: x,\n            _y: y,\n            _z: z,\n            ..Default::default()\n        }\n    }"
        }
      }
    },
    {
      "name": "Point.with_name",
      "implementations": {
        "rust": {
          "sig": "with_name(x: f64, y: f64, z: f64, name: &str) -> Self",
          "code": "pub fn with_name(x: f64, y: f64, z: f64, name: &str) -> Self {\n        Self {\n            _x: x,\n            _y: y,\n            _z: z,\n            name: name.to_string(),\n            ..Default::default()\n        }\n    }"
        }
      }
    },
    {
      "name": "Point.to_json",
      "implementations": {
        "rust": {
          "sig": "to_json(filepath: &str) -> Result<(), Box<dyn std::error::Error>>",
          "code": "pub fn to_json(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {\n        let json = self.jsondump()?;\n        std::fs::write(filepath, json)?;\n        Ok(())\n    }"
        }
      }
    },
    {
      "name": "Point.from_json",
      "implementations": {
        "rust": {
          "sig": "from_json(filepath: &str) -> Result<Self, Box<dyn std::error::Error>>",
          "code": "pub fn from_json(filepath: &str) -> Result<Self, Box<dyn std::error::Error>> {\n        let json = std::fs::read_to_string(filepath)?;\n        Self::jsonload(&json)\n    }"
        }
      }
    },
    {
      "name": "Color.new",
      "implementations": {
        "rust": {
          "sig": "new(r: u8, g: u8, b: u8, a: u8) -> Self",
          "code": "pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {\n        Color {\n            guid: Uuid::new_v4().to_string(),\n            name: \"my_color\".to_string(),\n            r,\n            g,\n            b,\n            a,\n        }\n    }"
        }
      }
    },
    {
      "name": "Color.with_name",
      "implementations": {
        "rust": {
          "sig": "with_name(r: u8, g: u8, b: u8, a: u8, name: &str) -> Self",
          "code": "pub fn with_name(r: u8, g: u8, b: u8, a: u8, name: &str) -> Self {\n        Color {\n            guid: Uuid::new_v4().to_string(),\n            name: name.to_string(),\n            r,\n            g,\n            b,\n            a,\n        }\n    }"
        }
      }
    },
    {
      "name": "Color.to_float_array",
      "implementations": {
        "rust": {
          "sig": "to_float_array() -> [f64; 4]",
          "code": "pub fn to_float_array(&self) -> [f64; 4] {\n        [\n            self.r as f64 / 255.0,\n            self.g as f64 / 255.0,\n            self.b as f64 / 255.0,\n            self.a as f64 / 255.0,\n        ]\n    }"
        }
      }
    },
    {
      "name": "Color.from_float",
      "implementations": {
        "rust": {
          "sig": "from_float(r: f64, g: f64, b: f64, a: f64) -> Self",
          "code": "pub fn from_float(r: f64, g: f64, b: f64, a: f64) -> Self {\n        Color::new(\n            (r * 255.0).round() as u8,\n            (g * 255.0).round() as u8,\n            (b * 255.0).round() as u8,\n            (a * 255.0).round() as u8,\n        )\n    }"
        }
      }
    },
    {
      "name": "Color.to_json",
      "implementations": {
        "rust": {
          "sig": "to_json(filepath: &str) -> Result<(), Box<dyn std::error::Error>>",
          "code": "pub fn to_json(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {\n        let json = self.jsondump()?;\n        std::fs::write(filepath, json)?;\n        Ok(())\n    }"
        }
      }
    },
    {
      "name": "Color.from_json",
      "implementations": {
        "rust": {
          "sig": "from_json(filepath: &str) -> Result<Self, Box<dyn std::error::Error>>",
          "code": "pub fn from_json(filepath: &str) -> Result<Self, Box<dyn std::error::Error>> {\n        let json = std::fs::read_to_string(filepath)?;\n        Self::jsonload(&json)\n    }"
        }
      }
    }
  ]
};
