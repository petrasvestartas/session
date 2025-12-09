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
          "code": "def duplicate(self):\n\n        \"\"\"Create a deep copy of this point with a new GUID.\n\n        Returns\n        -------\n        :class:`Point`\n            A new Point with identical values but a different GUID.\n\n        \"\"\"\n        return copy.deepcopy(self)\n\n    def __str__(self):"
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
          "code": "def __ne__(self, other):\n\n        return not self == other\n\n    ###########################################################################################\n    # Coordinate Properties\n    ###########################################################################################\n\n    @property\n    def x(self):\n        \"\"\"Get the X coordinate.\"\"\"\n        return self._x"
        }
      }
    },
    {
      "name": "Point.x",
      "implementations": {
        "python": {
          "sig": "x(value)",
          "code": "def x(self, value):\n\n        \"\"\"Set the X coordinate.\"\"\"\n        self._x = value\n\n    @property\n    def y(self):\n        \"\"\"Get the Y coordinate.\"\"\"\n        return self._y\n\n    @y.setter\n    def y(self, value):\n        \"\"\"Set the Y coordinate.\"\"\""
        },
        "rust": {
          "sig": "x() -> f64",
          "code": "pub fn x(&self) -> f64 {\n        self._x\n    }"
        }
      }
    },
    {
      "name": "Point.y",
      "implementations": {
        "python": {
          "sig": "y(value)",
          "code": "def y(self, value):\n\n        \"\"\"Set the Y coordinate.\"\"\"\n        self._y = value\n\n    @property\n    def z(self):\n        \"\"\"Get the Z coordinate.\"\"\"\n        return self._z\n\n    @z.setter\n    def z(self, value):\n        \"\"\"Set the Z coordinate.\"\"\""
        },
        "rust": {
          "sig": "y() -> f64",
          "code": "pub fn y(&self) -> f64 {\n        self._y\n    }"
        }
      }
    },
    {
      "name": "Point.z",
      "implementations": {
        "python": {
          "sig": "z(value)",
          "code": "def z(self, value):\n\n        \"\"\"Set the Z coordinate.\"\"\"\n        self._z = value\n\n    ###########################################################################################\n    # No-copy Operators\n    ###########################################################################################\n\n    def __getitem__(self, index):\n        if index == 0:\n            return self._x\n        elif index == 1:"
        },
        "rust": {
          "sig": "z() -> f64",
          "code": "pub fn z(&self) -> f64 {\n        self._z\n    }"
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
          "code": "def __itruediv__(self, other):\n\n        self._x /= other\n        self._y /= other\n        self._z /= other\n        return self\n\n    def __iadd__(self, other):\n        if isinstance(other, Vector):\n            self._x += other[0]\n            self._y += other[1]\n            self._z += other[2]\n        else:"
        }
      }
    },
    {
      "name": "Point.__iadd__",
      "implementations": {
        "python": {
          "sig": "__iadd__(other)",
          "code": "def __iadd__(self, other):\n\n        if isinstance(other, Vector):\n            self._x += other[0]\n            self._y += other[1]\n            self._z += other[2]\n        else:\n            raise TypeError(\"Point can only be added with Vector\")\n        return self\n\n    def __isub__(self, other):\n        if isinstance(other, Vector):\n            self._x -= other[0]"
        }
      }
    },
    {
      "name": "Point.__isub__",
      "implementations": {
        "python": {
          "sig": "__isub__(other)",
          "code": "def __isub__(self, other):\n\n        if isinstance(other, Vector):\n            self._x -= other[0]\n            self._y -= other[1]\n            self._z -= other[2]\n        else:\n            raise TypeError(\"Point can only be subtracted with Vector\")\n        return self\n\n    ###########################################################################################\n    # Copy Operators\n    ###########################################################################################"
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
          "code": "def to_protobuf(self):\n\n        \"\"\"Convert to protobuf binary format.\n\n        Returns\n        -------\n        bytes\n            Serialized protobuf data.\n\n        \"\"\"\n        from .proto import point_pb2\n        \n        proto = point_pb2.Point()"
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
          "code": "def from_protobuf(cls, data):\n\n        \"\"\"Create Point from protobuf binary data.\n\n        Parameters\n        ----------\n        data : bytes\n            Protobuf-encoded point data.\n\n        Returns\n        -------\n        :class:`Point`\n            The deserialized Point."
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
          "code": "def protobuf_dump(self, filepath):\n\n        \"\"\"Write protobuf to file.\n\n        Parameters\n        ----------\n        filepath : str\n            Path to the output file.\n\n        \"\"\"\n        data = self.to_protobuf()\n        with open(filepath, 'wb') as f:\n            f.write(data)"
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
          "code": "def protobuf_load(cls, filepath):\n\n        \"\"\"Read protobuf from file.\n\n        Parameters\n        ----------\n        filepath : str\n            Path to the protobuf file.\n\n        Returns\n        -------\n        :class:`Point`\n            The deserialized Point."
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
          "code": "def duplicate(self) -> \"Color\":\n\n        \"\"\"Create a deep copy of this color with a new GUID.\n\n        Returns\n        -------\n        :class:`Color`\n            A new Color with identical RGBA values but a different GUID.\n\n        \"\"\"\n        return copy.deepcopy(self)\n\n    def __str__(self) -> str:"
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
          "code": "def to_unified_array(self) -> list[float]:\n\n        \"\"\"Convert to normalized float array [0-1].\n\n        Returns\n        -------\n        list[float]\n            Array [r, g, b, a] with values normalized to [0.0, 1.0].\n\n        \"\"\"\n        return [self[0] / 255.0, self[1] / 255.0, self[2] / 255.0, self[3] / 255.0]\n\n    @classmethod"
        }
      }
    },
    {
      "name": "Color.from_unified_array",
      "implementations": {
        "python": {
          "sig": "from_unified_array(cls, arr)",
          "code": "def from_unified_array(cls, arr) -> \"Color\":\n\n        \"\"\"Create color from normalized float values [0-1].\n\n        Parameters\n        ----------\n        arr : list[float]\n            Array [r, g, b, a] with values in [0.0, 1.0] range.\n\n        Returns\n        -------\n        :class:`Color`\n            A new Color with values converted to 0-255 range."
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
          "code": "def to_protobuf(self):\n\n        \"\"\"Convert to protobuf binary format.\n\n        Returns\n        -------\n        bytes\n            Serialized protobuf data.\n\n        Raises\n        ------\n        ImportError\n            If protobuf module is not available."
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
          "code": "def from_protobuf(cls, data):\n\n        \"\"\"Create color from protobuf binary data.\n\n        Parameters\n        ----------\n        data : bytes\n            Protobuf-encoded color data.\n\n        Returns\n        -------\n        :class:`Color`\n            The deserialized Color."
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
          "code": "def protobuf_dump(self, filepath):\n\n        \"\"\"Write protobuf to file.\n\n        Parameters\n        ----------\n        filepath : str\n            Path to the output file.\n\n        \"\"\"\n        data = self.to_protobuf()\n        with open(filepath, 'wb') as f:\n            f.write(data)"
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
          "code": "def protobuf_load(cls, filepath):\n\n        \"\"\"Read protobuf from file.\n\n        Parameters\n        ----------\n        filepath : str\n            Path to the protobuf file.\n\n        Returns\n        -------\n        :class:`Color`\n            The deserialized Color."
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
      "name": "Vector.__init__",
      "implementations": {
        "python": {
          "sig": "__init__(x=0.0, y=0.0, z=0.0)",
          "code": "def __init__(self, x=0.0, y=0.0, z=0.0):\n\n        self.guid = str(uuid.uuid4())\n        self.name = \"my_vector\"\n        self._x = x\n        self._y = y\n        self._z = z\n        self._length = 0.0\n        self._has_length = False\n\n    def __str__(self):\n        return f\"Vector({self[0]}, {self[1]}, {self[2]})\""
        }
      }
    },
    {
      "name": "Vector.__str__",
      "implementations": {
        "python": {
          "sig": "__str__()",
          "code": "def __str__(self):\n\n        return f\"Vector({self[0]}, {self[1]}, {self[2]})\"\n\n    def __repr__(self):\n        return f\"Vector({self.guid}, {self.name}, {self[0]}, {self[1]}, {self[2]})\"\n\n    def __eq__(self, other):\n        return (\n            self.name == other.name\n            and round(self[0], 6) == round(other[0], 6)\n            and round(self[1], 6) == round(other[1], 6)\n            and round(self[2], 6) == round(other[2], 6)"
        }
      }
    },
    {
      "name": "Vector.__repr__",
      "implementations": {
        "python": {
          "sig": "__repr__()",
          "code": "def __repr__(self):\n\n        return f\"Vector({self.guid}, {self.name}, {self[0]}, {self[1]}, {self[2]})\"\n\n    def __eq__(self, other):\n        return (\n            self.name == other.name\n            and round(self[0], 6) == round(other[0], 6)\n            and round(self[1], 6) == round(other[1], 6)\n            and round(self[2], 6) == round(other[2], 6)\n        )\n\n    def __ne__(self, other):"
        }
      }
    },
    {
      "name": "Vector.__eq__",
      "implementations": {
        "python": {
          "sig": "__eq__(other)",
          "code": "def __eq__(self, other):\n\n        return (\n            self.name == other.name\n            and round(self[0], 6) == round(other[0], 6)\n            and round(self[1], 6) == round(other[1], 6)\n            and round(self[2], 6) == round(other[2], 6)\n        )\n\n    def __ne__(self, other):\n        return not self == other\n\n    ###########################################################################################"
        }
      }
    },
    {
      "name": "Vector.__ne__",
      "implementations": {
        "python": {
          "sig": "__ne__(other)",
          "code": "def __ne__(self, other):\n\n        return not self == other\n\n    ###########################################################################################\n    # No-copy Operators\n    ###########################################################################################\n\n    def __getitem__(self, index):\n        \"\"\"Access coordinate by index (0=x, 1=y, 2=z).\"\"\"\n        if index == 0:\n            return self._x\n        elif index == 1:"
        }
      }
    },
    {
      "name": "Vector.__getitem__",
      "implementations": {
        "python": {
          "sig": "__getitem__(index)",
          "code": "def __getitem__(self, index):\n\n        \"\"\"Access coordinate by index (0=x, 1=y, 2=z).\"\"\"\n        if index == 0:\n            return self._x\n        elif index == 1:\n            return self._y\n        elif index == 2:\n            return self._z\n        else:\n            raise IndexError(\"Index out of range\")\n\n    def __setitem__(self, index, value):"
        }
      }
    },
    {
      "name": "Vector.__setitem__",
      "implementations": {
        "python": {
          "sig": "__setitem__(index, value)",
          "code": "def __setitem__(self, index, value):\n\n        \"\"\"Set coordinate by index (0=x, 1=y, 2=z). Invalidates length cache.\"\"\"\n        if index == 0:\n            self._x = value\n        elif index == 1:\n            self._y = value\n        elif index == 2:\n            self._z = value\n        else:\n            raise IndexError(\"Index out of range\")\n        self._has_length = False"
        }
      }
    },
    {
      "name": "Vector.__imul__",
      "implementations": {
        "python": {
          "sig": "__imul__(other)",
          "code": "def __imul__(self, other):\n\n        self._x *= other\n        self._y *= other\n        self._z *= other\n        self._has_length = False\n        return self\n\n    def __itruediv__(self, other):\n        self._x /= other\n        self._y /= other\n        self._z /= other\n        self._has_length = False"
        }
      }
    },
    {
      "name": "Vector.__itruediv__",
      "implementations": {
        "python": {
          "sig": "__itruediv__(other)",
          "code": "def __itruediv__(self, other):\n\n        self._x /= other\n        self._y /= other\n        self._z /= other\n        self._has_length = False\n        return self\n\n    def __iadd__(self, other):\n        self._x += other._x\n        self._y += other._y\n        self._z += other._z\n        self._has_length = False"
        }
      }
    },
    {
      "name": "Vector.__iadd__",
      "implementations": {
        "python": {
          "sig": "__iadd__(other)",
          "code": "def __iadd__(self, other):\n\n        self._x += other._x\n        self._y += other._y\n        self._z += other._z\n        self._has_length = False\n        return self\n\n    def __isub__(self, other):\n        self._x -= other._x\n        self._y -= other._y\n        self._z -= other._z\n        self._has_length = False"
        }
      }
    },
    {
      "name": "Vector.__isub__",
      "implementations": {
        "python": {
          "sig": "__isub__(other)",
          "code": "def __isub__(self, other):\n\n        self._x -= other._x\n        self._y -= other._y\n        self._z -= other._z\n        self._has_length = False\n        return self\n\n    ###########################################################################################\n    # Copy Operators\n    ###########################################################################################\n\n    def __mul__(self, other):"
        }
      }
    },
    {
      "name": "Vector.__mul__",
      "implementations": {
        "python": {
          "sig": "__mul__(other)",
          "code": "def __mul__(self, other):\n\n        return Vector(self._x * other, self._y * other, self._z * other)\n\n    def __truediv__(self, other):\n        return Vector(self._x / other, self._y / other, self._z / other)\n\n    def __add__(self, other):\n        return Vector(self._x + other._x, self._y + other._y, self._z + other._z)\n\n    def __sub__(self, other):\n        return Vector(self._x - other._x, self._y - other._y, self._z - other._z)"
        }
      }
    },
    {
      "name": "Vector.__truediv__",
      "implementations": {
        "python": {
          "sig": "__truediv__(other)",
          "code": "def __truediv__(self, other):\n\n        return Vector(self._x / other, self._y / other, self._z / other)\n\n    def __add__(self, other):\n        return Vector(self._x + other._x, self._y + other._y, self._z + other._z)\n\n    def __sub__(self, other):\n        return Vector(self._x - other._x, self._y - other._y, self._z - other._z)\n\n    ###########################################################################################\n    # Static Methods\n    ###########################################################################################"
        }
      }
    },
    {
      "name": "Vector.__add__",
      "implementations": {
        "python": {
          "sig": "__add__(other)",
          "code": "def __add__(self, other):\n\n        return Vector(self._x + other._x, self._y + other._y, self._z + other._z)\n\n    def __sub__(self, other):\n        return Vector(self._x - other._x, self._y - other._y, self._z - other._z)\n\n    ###########################################################################################\n    # Static Methods\n    ###########################################################################################\n\n    @staticmethod\n    def x_axis():"
        }
      }
    },
    {
      "name": "Vector.__sub__",
      "implementations": {
        "python": {
          "sig": "__sub__(other)",
          "code": "def __sub__(self, other):\n\n        return Vector(self._x - other._x, self._y - other._y, self._z - other._z)\n\n    ###########################################################################################\n    # Static Methods\n    ###########################################################################################\n\n    @staticmethod\n    def x_axis():\n        \"\"\"Get unit vector along the x-axis.\n\n        Returns"
        }
      }
    },
    {
      "name": "Vector.x_axis",
      "implementations": {
        "python": {
          "sig": "x_axis()",
          "code": "def x_axis():\n\n        \"\"\"Get unit vector along the x-axis.\n\n        Returns\n        -------\n        :class:`Vector`\n            Unit vector (1, 0, 0).\n\n        \"\"\"\n        return Vector(1.0, 0.0, 0.0)\n\n    @staticmethod"
        },
        "cpp": {
          "sig": "Vector x_axis()",
          "code": "Vector Vector::x_axis() { return Vector(1.0, 0.0, 0.0); }"
        },
        "rust": {
          "sig": "x_axis() -> Self",
          "code": "pub fn x_axis() -> Self {\n        Self::new(1.0, 0.0, 0.0)\n    }"
        }
      }
    },
    {
      "name": "Vector.y_axis",
      "implementations": {
        "python": {
          "sig": "y_axis()",
          "code": "def y_axis():\n\n        \"\"\"Get unit vector along the y-axis.\n\n        Returns\n        -------\n        :class:`Vector`\n            Unit vector (0, 1, 0).\n\n        \"\"\"\n        return Vector(0.0, 1.0, 0.0)\n\n    @staticmethod"
        },
        "cpp": {
          "sig": "Vector y_axis()",
          "code": "Vector Vector::y_axis() { return Vector(0.0, 1.0, 0.0); }"
        },
        "rust": {
          "sig": "y_axis() -> Self",
          "code": "pub fn y_axis() -> Self {\n        Self::new(0.0, 1.0, 0.0)\n    }"
        }
      }
    },
    {
      "name": "Vector.z_axis",
      "implementations": {
        "python": {
          "sig": "z_axis()",
          "code": "def z_axis():\n\n        \"\"\"Get unit vector along the z-axis.\n\n        Returns\n        -------\n        :class:`Vector`\n            Unit vector (0, 0, 1).\n\n        \"\"\"\n        return Vector(0.0, 0.0, 1.0)\n\n    @staticmethod"
        },
        "cpp": {
          "sig": "Vector z_axis()",
          "code": "Vector Vector::z_axis() { return Vector(0.0, 0.0, 1.0); }"
        },
        "rust": {
          "sig": "z_axis() -> Self",
          "code": "pub fn z_axis() -> Self {\n        Self::new(0.0, 0.0, 1.0)\n    }"
        }
      }
    },
    {
      "name": "Vector.from_start_and_end",
      "implementations": {
        "python": {
          "sig": "from_start_and_end(start, end)",
          "code": "def from_start_and_end(start, end):\n\n        \"\"\"Vector from start to end (end - start).\n\n        Parameters\n        ----------\n        start : :class:`Vector`\n            Start vector.\n        end : :class:`Vector`\n            End vector.\n\n        Returns\n        -------"
        },
        "cpp": {
          "sig": "Vector from_start_and_end(const Vector &start, const Vector &end)",
          "code": "Vector Vector::from_start_and_end(const Vector &start, const Vector &end) {\n  return Vector(end._x - start._x, end._y - start._y, end._z - start._z);\n}"
        },
        "rust": {
          "sig": "from_start_and_end(start: &Vector, end: &Vector) -> Self",
          "code": "pub fn from_start_and_end(start: &Vector, end: &Vector) -> Self {\n        Self::new(end._x - start._x, end._y - start._y, end._z - start._z)\n    }"
        }
      }
    },
    {
      "name": "Vector.reverse",
      "implementations": {
        "python": {
          "sig": "reverse()",
          "code": "def reverse(self):\n\n        \"\"\"Reverse the vector (negate all components).\n\n        Returns\n        -------\n        :class:`Vector`\n            Self.\n\n        \"\"\"\n        self._x = -self._x\n        self._y = -self._y\n        self._z = -self._z"
        },
        "cpp": {
          "sig": "void reverse()",
          "code": "void Vector::reverse() {\n  _x = -_x;\n  _y = -_y;\n  _z = -_z;\n  // Length magnitude stays the same, no need to invalidate cache\n}"
        },
        "rust": {
          "sig": "reverse()",
          "code": "pub fn reverse(&mut self) {\n        self._x = -self._x;\n        self._y = -self._y;\n        self._z = -self._z;\n        // Length magnitude stays the same, no need to invalidate cache\n    }"
        }
      }
    },
    {
      "name": "Vector.compute_length",
      "implementations": {
        "python": {
          "sig": "compute_length()",
          "code": "def compute_length(self):\n\n        \"\"\"Compute the length of the vector using optimized algorithm.\n\n        Returns\n        -------\n        float\n            The length of the vector.\n        \"\"\"\n        length = 0.0\n\n        x = abs(self._x)\n        y = abs(self._y)"
        },
        "cpp": {
          "sig": "double compute_length()",
          "code": "double Vector::compute_length() const {\n  double len = 0.0;\n\n  double ax = std::abs(_x);\n  double ay = std::abs(_y);\n  double az = std::abs(_z);\n\n  const bool x_zero = ax < static_cast<double>(session_cpp::Tolerance::ZERO_TOLERANCE);\n  const bool y_zero = ay < static_cast<double>(session_cpp::Tolerance::ZERO_TOLERANCE);\n  const bool z_zero = az < static_cast<double>(session_cpp::Tolerance::ZERO_TOLERANCE);\n\n  if (x_zero && y_zero && z_zero)\n    return 0.0;\n  else if (x_zero && y_zero)\n    retur"
        },
        "rust": {
          "sig": "compute_length() -> f64",
          "code": "pub fn compute_length(&self) -> f64 {\n        (self._x * self._x + self._y * self._y + self._z * self._z).sqrt()\n    }"
        }
      }
    },
    {
      "name": "Vector.magnitude",
      "implementations": {
        "python": {
          "sig": "magnitude()",
          "code": "def magnitude(self):\n\n        \"\"\"Get the cached magnitude of the vector, computing it if necessary.\n\n        Returns\n        -------\n        float\n            The magnitude (length) of the vector.\n        \"\"\"\n        if not self._has_length:\n            self._length = self.compute_length()\n            self._has_length = True"
        },
        "cpp": {
          "sig": "double magnitude()",
          "code": "double Vector::magnitude() const { return cached_length(); }"
        },
        "rust": {
          "sig": "magnitude() -> f64",
          "code": "pub fn magnitude(&mut self) -> f64 {\n        if !self._has_length {\n            self._length = self.compute_length();\n            self._has_length = true;\n        }\n        self._length\n    }"
        }
      }
    },
    {
      "name": "Vector.length_squared",
      "implementations": {
        "python": {
          "sig": "length_squared()",
          "code": "def length_squared(self):\n\n        \"\"\"Get the squared length of the vector (avoids sqrt for performance).\n\n        Returns\n        -------\n        float\n            The squared length of the vector.\n        \"\"\"\n        return self._x * self._x + self._y * self._y + self._z * self._z\n\n    def normalize_self(self):\n        \"\"\"Normalize the vector in place (make it unit length)."
        },
        "cpp": {
          "sig": "double length_squared()",
          "code": "double Vector::length_squared() const {\n  return _x * _x + _y * _y + _z * _z;\n}"
        },
        "rust": {
          "sig": "length_squared() -> f64",
          "code": "pub fn length_squared(&self) -> f64 {\n        self._x * self._x + self._y * self._y + self._z * self._z\n    }"
        }
      }
    },
    {
      "name": "Vector.normalize_self",
      "implementations": {
        "python": {
          "sig": "normalize_self()",
          "code": "def normalize_self(self):\n\n        \"\"\"Normalize the vector in place (make it unit length).\n\n        Returns\n        -------\n        bool\n            True if successful, False if vector has zero length.\n        \"\"\"\n        d = self.magnitude()\n        if d > 0.0:\n            self._x /= d\n            self._y /= d"
        },
        "cpp": {
          "sig": "bool normalize_self()",
          "code": "bool Vector::normalize_self() {\n  double d = compute_length();\n  if (d > 0.0) {\n    (*this)[0] = _x / d;\n    (*this)[1] = _y / d;\n    (*this)[2] = _z / d;\n    return true;\n  }"
        },
        "rust": {
          "sig": "normalize_self()",
          "code": "pub fn normalize_self(&mut self) {\n        let len = self.magnitude();\n        if len > Tolerance::ZERO_TOLERANCE {\n            self._x /= len;\n            self._y /= len;\n            self._z /= len;\n            self.invalidate_length_cache();\n        }\n    }"
        }
      }
    },
    {
      "name": "Vector.normalize",
      "implementations": {
        "python": {
          "sig": "normalize()",
          "code": "def normalize(self):\n\n        \"\"\"Return a normalized copy of the vector.\n\n        Returns\n        -------\n        Vector\n            A new vector that is the unit vector of this vector.\n        \"\"\"\n        normalized_vector = Vector(self._x, self._y, self._z)\n        normalized_vector.normalize_self()\n        return normalized_vector"
        },
        "rust": {
          "sig": "normalize() -> Self",
          "code": "pub fn normalize(&self) -> Self {\n        let mut result = self.clone();\n        result.normalize_self();\n        result\n    }"
        }
      }
    },
    {
      "name": "Vector.dot",
      "implementations": {
        "python": {
          "sig": "dot(other)",
          "code": "def dot(self, other):\n\n        \"\"\"Calculate dot product with another vector.\n\n        Parameters\n        ----------\n        other : :class:`Vector`\n            Other vector.\n\n        Returns\n        -------\n        float\n            Dot product value."
        },
        "cpp": {
          "sig": "double dot(const Vector &other)",
          "code": "double Vector::dot(const Vector &other) const {\n  double result = 0.0;\n  for (int i = 0; i < 3; ++i) {\n    result += (*this)[i] * other[i];\n  }"
        },
        "rust": {
          "sig": "dot(other: &Vector) -> f64",
          "code": "pub fn dot(&self, other: &Vector) -> f64 {\n        self._x * other._x + self._y * other._y + self._z * other._z\n    }"
        }
      }
    },
    {
      "name": "Vector.cross",
      "implementations": {
        "python": {
          "sig": "cross(other)",
          "code": "def cross(self, other):\n\n        \"\"\"Calculate cross product with another vector.\n\n        Parameters\n        ----------\n        other : :class:`Vector`\n            Other vector.\n\n        Returns\n        -------\n        :class:`Vector`\n            Cross product vector (orthogonal to inputs)."
        },
        "cpp": {
          "sig": "Vector cross(const Vector &other)",
          "code": "Vector Vector::cross(const Vector &other) const {\n  double cx = (*this)[1] * other[2] - (*this)[2] * other[1];\n  double cy = (*this)[2] * other[0] - (*this)[0] * other[2];\n  double cz = (*this)[0] * other[1] - (*this)[1] * other[0];\n  return Vector(cx, cy, cz);\n}"
        },
        "rust": {
          "sig": "cross(other: &Vector) -> Vector",
          "code": "pub fn cross(&self, other: &Vector) -> Vector {\n        Vector::new(\n            self._y * other._z - self._z * other._y,\n            self._z * other._x - self._x * other._z,\n            self._x * other._y - self._y * other._x,\n        )\n    }"
        }
      }
    },
    {
      "name": "Vector.is_parallel_to",
      "implementations": {
        "python": {
          "sig": "is_parallel_to(v)",
          "code": "def is_parallel_to(self, v):\n\n        \"\"\"Check if this vector is parallel/antiparallel to another.\n\n        Parameters\n        ----------\n        v : :class:`Vector`\n            Other vector.\n\n        Returns\n        -------\n        int\n            1 if parallel, -1 if antiparallel, 0 otherwise."
        },
        "cpp": {
          "sig": "int is_parallel_to(const Vector &other)",
          "code": "int Vector::is_parallel_to(const Vector &other) {\n  double ll = cached_length() * other.cached_length();\n  int result;\n  \n  if (ll > 0.0) {\n    const double cos_angle = ((*this)[0] * other[0] + (*this)[1] * other[1] + (*this)[2] * other[2]) / ll;\n    const double angle_in_radians = static_cast<double>(Tolerance::ANGLE_TOLERANCE_DEGREES) * static_cast<double>(Tolerance::TO_RADIANS);\n    const double cos_tol = std::cos(angle_in_radians);\n    if (cos_angle >= cos_tol)\n      result = 1;  // Paralle"
        },
        "rust": {
          "sig": "is_parallel_to(other: &Vector) -> i32",
          "code": "pub fn is_parallel_to(&self, other: &Vector) -> i32 {\n        let len_product = self.compute_length() * other.compute_length();\n\n        if len_product <= 0.0 {\n            return 0;\n        }\n\n        let cos_angle = self.dot(other) / len_product;\n        let angle_in_radians = Tolerance::ANGLE_TOLERANCE_DEGREES * TO_RADIANS;\n        let cos_tolerance = angle_in_radians.cos();\n\n        if cos_angle >= cos_tolerance {\n            1 // Parallel\n        } else if cos_angle <= -cos_tolerance {"
        }
      }
    },
    {
      "name": "Vector.angle",
      "implementations": {
        "python": {
          "sig": "angle(other, sign_by_cross_product=False, degrees=True, tolerance=1e-12)",
          "code": "def angle(self, other, sign_by_cross_product=False, degrees=True, tolerance=1e-12):\n\n        \"\"\"Angle between this vector and another.\n\n        Parameters\n        ----------\n        other : :class:`Vector`\n            The other vector.\n        sign_by_cross_product : bool, optional\n            If True, sign the angle using the z-component of the cross product.\n        degrees : bool, optional\n            If True (default), return angle in degrees; otherwise radians.\n        tolerance : float, optional"
        },
        "cpp": {
          "sig": "double angle(const Vector &other, bool sign_by_cross_product, bool degrees,\n                     double tolerance)",
          "code": "double Vector::angle(const Vector &other, bool sign_by_cross_product, bool degrees,\n                     double tolerance) {\n  double dotp = this->dot(other);\n  double len0 = this->cached_length();\n  double len1 = other.cached_length();\n  double denom = len0 * len1;\n  if (denom < tolerance) {\n    return 0.0;\n  }"
        },
        "rust": {
          "sig": "angle(other: &Vector, sign_by_cross_product: bool) -> f64",
          "code": "pub fn angle(&self, other: &Vector, sign_by_cross_product: bool) -> f64 {\n        let dotp = self.dot(other);\n        let len_product = self.compute_length() * other.compute_length();\n\n        if len_product < Tolerance::ZERO_TOLERANCE {\n            return 0.0;\n        }\n\n        let cos_angle = (dotp / len_product).clamp(-1.0, 1.0);\n        let mut angle = cos_angle.acos() * TO_DEGREES;\n\n        if sign_by_cross_product {\n            let cp = self.cross(other);\n            if cp[2] < 0.0 {"
        }
      }
    },
    {
      "name": "Vector.projection",
      "implementations": {
        "python": {
          "sig": "projection(projection_vector, tolerance=1e-12)",
          "code": "def projection(self, projection_vector, tolerance=1e-12):\n\n        \"\"\"Project this vector onto another vector.\n\n        Parameters\n        ----------\n        projection_vector : :class:`Vector`\n            Vector to project onto.\n        tolerance : float, optional\n            Treat `projection_vector` length below this as zero.\n\n        Returns\n        -------"
        },
        "rust": {
          "sig": "projection(onto: &Vector) -> (Vector, f64, Vector, f64)",
          "code": "pub fn projection(&self, onto: &Vector) -> (Vector, f64, Vector, f64) {\n        self.projection_with(onto, Tolerance::ZERO_TOLERANCE)\n    }"
        }
      }
    },
    {
      "name": "Vector.get_leveled_vector",
      "implementations": {
        "python": {
          "sig": "get_leveled_vector(vertical_height)",
          "code": "def get_leveled_vector(self, vertical_height):\n\n        \"\"\"Get a copy scaled by a vertical height along the Z-axis.\n\n        Parameters\n        ----------\n        vertical_height : float\n            Target vertical height.\n\n        Returns\n        -------\n        :class:`Vector`\n            Scaled copy matching the C++ implementation."
        },
        "cpp": {
          "sig": "Vector get_leveled_vector(double &vertical_height)",
          "code": "Vector Vector::get_leveled_vector(double &vertical_height) {\n  Vector copy(_x, _y, _z);\n  if (copy.normalize_self()) {\n    Vector reference(0, 0, 1);\n    double angle = copy.angle(reference, true); // returns degrees\n    // CRITICAL: statics bug - passes degrees directly to cos (expects radians)\n    double inclined_offset_by_vertical_distance = vertical_height / std::cos(angle);\n    copy *= inclined_offset_by_vertical_distance;\n  }"
        },
        "rust": {
          "sig": "get_leveled_vector(vertical_height: f64) -> Vector",
          "code": "pub fn get_leveled_vector(&self, vertical_height: f64) -> Vector {\n        let mut copy = self.clone();\n        copy.normalize_self();\n\n        if vertical_height != 0.0 {\n            let reference = Vector::z_axis();\n            let angle = copy.angle(&reference, true); // returns degrees\n                                                      // CRITICAL: statics bug - passes degrees directly to cos (expects radians)\n            let inclined_offset_by_vertical_distance = vertical_height / an"
        }
      }
    },
    {
      "name": "Vector.cosine_law",
      "implementations": {
        "python": {
          "sig": "cosine_law(\n        triangle_edge_length_a,\n        triangle_edge_length_b,\n        angle_in_between_edges,\n        degrees=True,\n    )",
          "code": "def cosine_law(\n        triangle_edge_length_a,\n        triangle_edge_length_b,\n        angle_in_between_edges,\n        degrees=True,\n    ):\n\n        \"\"\"Calculate third side of triangle using the cosine law.\n\n        Parameters\n        ----------\n        triangle_edge_length_a : float\n            Length of side a.\n        triangle_edge_length_b : float\n            Length of side b.\n        angle_in_between_edges : float\n            Angle between a and b.\n        degrees : bool, optional"
        },
        "cpp": {
          "sig": "double cosine_law(double &a, double &b, double &ang_between, bool degrees)",
          "code": "double Vector::cosine_law(double &a, double &b, double &ang_between, bool degrees) {\n  double to_rad = degrees ? static_cast<double>(Tolerance::TO_RADIANS) : 1.0;\n  return std::sqrt(a * a + b * b - 2.0 * a * b * std::cos(ang_between * to_rad));\n}"
        },
        "rust": {
          "sig": "cosine_law(\n        triangle_edge_length_a: f64,\n        triangle_edge_length_b: f64,\n        angle_in_degrees_between_edges: f64,\n        degrees: bool,\n    ) -> f64",
          "code": "pub fn cosine_law(\n        triangle_edge_length_a: f64,\n        triangle_edge_length_b: f64,\n        angle_in_degrees_between_edges: f64,\n        degrees: bool,\n    ) -> f64 {\n        let angle = if degrees {\n            angle_in_degrees_between_edges * TO_RADIANS\n        } else {\n            angle_in_degrees_between_edges\n        };\n\n        (triangle_edge_length_a.powi(2) + triangle_edge_length_b.powi(2)\n            - 2.0 * triangle_edge_length_a * triangle_edge_length_b * angle.cos())"
        }
      }
    },
    {
      "name": "Vector.sine_law_angle",
      "implementations": {
        "python": {
          "sig": "sine_law_angle(\n        triangle_edge_length_a,\n        angle_in_front_of_a,\n        triangle_edge_length_b,\n        degrees=True,\n    )",
          "code": "def sine_law_angle(\n        triangle_edge_length_a,\n        angle_in_front_of_a,\n        triangle_edge_length_b,\n        degrees=True,\n    ):\n\n        \"\"\"Calculate angle using the sine law.\n\n        Parameters\n        ----------\n        triangle_edge_length_a : float\n            Length of side a.\n        angle_in_front_of_a : float\n            Angle opposite to side a.\n        triangle_edge_length_b : float\n            Length of side b.\n        degrees : bool, optional"
        },
        "cpp": {
          "sig": "double sine_law_angle(double &a, double &A, double &b, bool degrees)",
          "code": "double Vector::sine_law_angle(double &a, double &A, double &b, bool degrees) {\n  double to_rad = degrees ? static_cast<double>(Tolerance::TO_RADIANS) : 1.0;\n  double to_deg = degrees ? static_cast<double>(Tolerance::TO_DEGREES) : 1.0;\n  return std::asin((b * std::sin(A * to_rad)) / a) * to_deg;\n}"
        },
        "rust": {
          "sig": "sine_law_angle(\n        triangle_edge_length_a: f64,\n        angle_in_degrees_in_front_of_a: f64,\n        triangle_edge_length_b: f64,\n        degrees: bool,\n    ) -> f64",
          "code": "pub fn sine_law_angle(\n        triangle_edge_length_a: f64,\n        angle_in_degrees_in_front_of_a: f64,\n        triangle_edge_length_b: f64,\n        degrees: bool,\n    ) -> f64 {\n        let angle_a = if degrees {\n            angle_in_degrees_in_front_of_a * TO_RADIANS\n        } else {\n            angle_in_degrees_in_front_of_a\n        };\n\n        let sin_b = (triangle_edge_length_b * angle_a.sin()) / triangle_edge_length_a;\n        let angle_b = sin_b.asin();\n\n        if degrees {"
        }
      }
    },
    {
      "name": "Vector.sine_law_length",
      "implementations": {
        "python": {
          "sig": "sine_law_length(\n        triangle_edge_length_a, angle_in_front_of_a, angle_in_front_of_b, degrees=True\n    )",
          "code": "def sine_law_length(\n        triangle_edge_length_a, angle_in_front_of_a, angle_in_front_of_b, degrees=True\n    ):\n\n        \"\"\"Calculate side length using the sine law.\n\n        Parameters\n        ----------\n        triangle_edge_length_a : float\n            Length of side a.\n        angle_in_front_of_a : float\n            Angle opposite to side a.\n        angle_in_front_of_b : float\n            Angle opposite to side b.\n        degrees : bool, optional"
        },
        "cpp": {
          "sig": "double sine_law_length(double &a, double &A, double &B, bool degrees)",
          "code": "double Vector::sine_law_length(double &a, double &A, double &B, bool degrees) {\n  double to_rad = degrees ? static_cast<double>(Tolerance::TO_RADIANS) : 1.0;\n  return (a * std::sin(B * to_rad)) / std::sin(A * to_rad);\n}"
        },
        "rust": {
          "sig": "sine_law_length(\n        triangle_edge_length_a: f64,\n        angle_in_degrees_in_front_of_a: f64,\n        angle_in_degrees_in_front_of_b: f64,\n        degrees: bool,\n    ) -> f64",
          "code": "pub fn sine_law_length(\n        triangle_edge_length_a: f64,\n        angle_in_degrees_in_front_of_a: f64,\n        angle_in_degrees_in_front_of_b: f64,\n        degrees: bool,\n    ) -> f64 {\n        let angle_a = if degrees {\n            angle_in_degrees_in_front_of_a * TO_RADIANS\n        } else {\n            angle_in_degrees_in_front_of_a\n        };\n\n        let angle_b = if degrees {\n            angle_in_degrees_in_front_of_b * TO_RADIANS\n        } else {\n            angle_in_degrees_in_fron"
        }
      }
    },
    {
      "name": "Vector.angle_between_vector_xy_components",
      "implementations": {
        "python": {
          "sig": "angle_between_vector_xy_components(vector, degrees=True)",
          "code": "def angle_between_vector_xy_components(vector, degrees=True):\n\n        \"\"\"Angle between the vector's XY components.\n\n        Parameters\n        ----------\n        vector : :class:`Vector`\n            Input vector.\n        degrees : bool, optional\n            If True, return degrees; otherwise radians.\n\n        Returns\n        -------"
        },
        "cpp": {
          "sig": "double angle_between_vector_xy_components(Vector &vector)",
          "code": "double Vector::angle_between_vector_xy_components(Vector &vector) {\n  return std::atan2(vector[1], vector[0]) * static_cast<double>(Tolerance::TO_DEGREES);\n}"
        },
        "rust": {
          "sig": "angle_between_vector_xy_components(vector: &Vector) -> f64",
          "code": "pub fn angle_between_vector_xy_components(vector: &Vector) -> f64 {\n        vector._y.atan2(vector._x) * TO_DEGREES\n    }"
        }
      }
    },
    {
      "name": "Vector.sum_of_vectors",
      "implementations": {
        "python": {
          "sig": "sum_of_vectors(vectors)",
          "code": "def sum_of_vectors(vectors):\n\n        \"\"\"Sum a list of vectors (component-wise).\n\n        Parameters\n        ----------\n        vectors : list[:class:`Vector`]\n            Vectors to sum.\n\n        Returns\n        -------\n        :class:`Vector`\n            The component-wise sum."
        },
        "cpp": {
          "sig": "Vector sum_of_vectors(std::vector<Vector> &vectors)",
          "code": "Vector Vector::sum_of_vectors(std::vector<Vector> &vectors) {\n  double sx = 0, sy = 0, sz = 0;\n  for (const auto &v : vectors) {\n    sx += v[0];\n    sy += v[1];\n    sz += v[2];\n  }"
        },
        "rust": {
          "sig": "sum_of_vectors(vectors: &[Vector]) -> Vector",
          "code": "pub fn sum_of_vectors(vectors: &[Vector]) -> Vector {\n        let mut result = Vector::zero();\n        for vector in vectors {\n            result._x += vector._x;\n            result._y += vector._y;\n            result._z += vector._z;\n        }\n        result\n    }"
        }
      }
    },
    {
      "name": "Vector.coordinate_direction_3angles",
      "implementations": {
        "python": {
          "sig": "coordinate_direction_3angles(degrees=True)",
          "code": "def coordinate_direction_3angles(self, degrees=True):\n\n        \"\"\"Compute coordinate direction angles (alpha, beta, gamma).\n\n        Parameters\n        ----------\n        degrees : bool, optional\n            Return angles in degrees if True, radians if False.\n\n        Returns\n        -------\n        tuple\n            (alpha, beta, gamma)"
        },
        "rust": {
          "sig": "coordinate_direction_3angles(degrees: bool) -> [f64; 3]",
          "code": "pub fn coordinate_direction_3angles(&self, degrees: bool) -> [f64; 3] {\n        let length = self.compute_length();\n        if length < Tolerance::ZERO_TOLERANCE {\n            return [0.0, 0.0, 0.0];\n        }\n\n        let cos_alpha = self._x / length;\n        let cos_beta = self._y / length;\n        let cos_gamma = self._z / length;\n\n        let alpha = cos_alpha.acos();\n        let beta = cos_beta.acos();\n        let gamma = cos_gamma.acos();\n\n        if degrees {\n            [alpha * TO_D"
        }
      }
    },
    {
      "name": "Vector.coordinate_direction_2angles",
      "implementations": {
        "python": {
          "sig": "coordinate_direction_2angles(degrees=True)",
          "code": "def coordinate_direction_2angles(self, degrees=True):\n\n        \"\"\"Compute coordinate direction angles (phi, theta).\n\n        Parameters\n        ----------\n        degrees : bool, optional\n            Return angles in degrees if True, radians if False.\n\n        Returns\n        -------\n        tuple\n            (phi, theta)"
        },
        "rust": {
          "sig": "coordinate_direction_2angles(degrees: bool) -> [f64; 2]",
          "code": "pub fn coordinate_direction_2angles(&self, degrees: bool) -> [f64; 2] {\n        let length_xy = (self._x * self._x + self._y * self._y).sqrt();\n        let length = self.compute_length();\n\n        if length < Tolerance::ZERO_TOLERANCE {\n            return [0.0, 0.0];\n        }\n\n        let phi = self._y.atan2(self._x);\n        let theta = length_xy.atan2(self._z);\n\n        if degrees {\n            [phi * TO_DEGREES, theta * TO_DEGREES]\n        } else {\n            [phi, theta]\n        }"
        }
      }
    },
    {
      "name": "Vector.perpendicular_to",
      "implementations": {
        "python": {
          "sig": "perpendicular_to(v)",
          "code": "def perpendicular_to(self, v):\n\n        \"\"\"Set this vector to be perpendicular to `v`.\n\n        Parameters\n        ----------\n        v : :class:`Vector`\n            Reference vector.\n\n        Returns\n        -------\n        bool\n            True on success, False otherwise."
        },
        "cpp": {
          "sig": "bool perpendicular_to(Vector &v)",
          "code": "bool Vector::perpendicular_to(Vector &v) {\n  int i, j, k;\n  double a, b;\n  k = 2;\n  if (std::fabs(v[1]) > std::fabs(v[0])) {\n    if (std::fabs(v[2]) > std::fabs(v[1])) {\n      // |v[2]| > |v[1]| > |v[0]|\n      i = 2; j = 1; k = 0; a = v[2]; b = -v[1];\n    }"
        },
        "rust": {
          "sig": "perpendicular_to(v: &Vector) -> bool",
          "code": "pub fn perpendicular_to(&mut self, v: &Vector) -> bool {\n        // Ported from Python implementation to ensure identical behavior\n        let i: usize;\n        let j: usize;\n        let k: usize;\n        let a: f64;\n        let b: f64;\n\n        if v[1].abs() > v[0].abs() {\n            if v[2].abs() > v[1].abs() {\n                // |v.z| > |v.y| > |v.x|\n                i = 2;\n                j = 1;\n                k = 0;\n                a = v[2];\n                b = -v[1];\n            } els"
        }
      }
    },
    {
      "name": "Vector.__jsondump__",
      "implementations": {
        "python": {
          "sig": "__jsondump__()",
          "code": "def __jsondump__(self):\n\n        \"\"\"Serialize to polymorphic JSON format with type field.\"\"\"\n        return {\n            \"type\": f\"{self.__class__.__name__}\",\n            \"guid\": self.guid,\n            \"name\": self.name,\n            \"x\": self[0],\n            \"y\": self[1],\n            \"z\": self[2],\n        }\n\n    @classmethod"
        }
      }
    },
    {
      "name": "Vector.__jsonload__",
      "implementations": {
        "python": {
          "sig": "__jsonload__(cls, data, guid=None, name=None)",
          "code": "def __jsonload__(cls, data, guid=None, name=None):\n\n        \"\"\"Deserialize from polymorphic JSON format.\"\"\"\n        vec = cls(data[\"x\"], data[\"y\"], data[\"z\"])\n        vec.guid = guid\n        vec.name = name\n        return vec"
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
          "sig": "return format(\"Vector({}, {}, {}, {}, {})",
          "code": "return fmt::format(\"Vector({}, {}, {}, {}, {})\", _x, _y, _z, guid, name);\n}"
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
      "name": "Vector.to_string",
      "implementations": {
        "cpp": {
          "sig": "std::string to_string()",
          "code": "std::string Vector::to_string() const {\n  return fmt::format(\"Vector({}"
        }
      }
    },
    {
      "name": "Vector.jsondump",
      "implementations": {
        "cpp": {
          "sig": "nlohmann::ordered_json jsondump()",
          "code": "nlohmann::ordered_json Vector::jsondump() const {\n  auto clean_float = [](double val) -> double { return std::round(val * 100.0) / 100.0; }"
        },
        "rust": {
          "sig": "jsondump() -> Result<String, Box<dyn std::error::Error>>",
          "code": "pub fn jsondump(&self) -> Result<String, Box<dyn std::error::Error>> {\n        let mut buf = Vec::new();\n        let formatter = serde_json::ser::PrettyFormatter::with_indent(b\"    \");\n        let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);\n        SerTrait::serialize(self, &mut ser)?;\n        Ok(String::from_utf8(buf)?)\n    }"
        }
      }
    },
    {
      "name": "Vector.jsonload",
      "implementations": {
        "cpp": {
          "sig": "Vector jsonload(const nlohmann::json &data)",
          "code": "Vector Vector::jsonload(const nlohmann::json &data) {\n  Vector vector(data[\"x\"], data[\"y\"], data[\"z\"]);\n  vector.guid = data[\"guid\"];\n  vector.name = data[\"name\"];\n  return vector;\n}"
        },
        "rust": {
          "sig": "jsonload(json_data: &str) -> Result<Self, Box<dyn std::error::Error>>",
          "code": "pub fn jsonload(json_data: &str) -> Result<Self, Box<dyn std::error::Error>> {\n        Ok(serde_json::from_str(json_data)?)\n    }"
        }
      }
    },
    {
      "name": "Vector.cached_length",
      "implementations": {
        "cpp": {
          "sig": "double cached_length()",
          "code": "double Vector::cached_length() const {\n  if (!_has_length) {\n    _length = compute_length();\n    _has_length = true;\n  }"
        }
      }
    },
    {
      "name": "std.sqrt",
      "implementations": {
        "cpp": {
          "sig": "return sqrt(a * a + b * b - 2.0 * a * b * std::cos(ang_between * to_rad)",
          "code": "return std::sqrt(a * a + b * b - 2.0 * a * b * std::cos(ang_between * to_rad));\n}"
        }
      }
    },
    {
      "name": "std.asin",
      "implementations": {
        "cpp": {
          "sig": "return asin((b * std::sin(A * to_rad)",
          "code": "return std::asin((b * std::sin(A * to_rad)) / a) * to_deg;\n}"
        }
      }
    },
    {
      "name": "std.atan2",
      "implementations": {
        "cpp": {
          "sig": "return atan2(vector[1], vector[0])",
          "code": "return std::atan2(vector[1], vector[0]) * static_cast<double>(Tolerance::TO_DEGREES);\n}"
        }
      }
    },
    {
      "name": "Vector.scale",
      "implementations": {
        "cpp": {
          "sig": "void scale(double factor)",
          "code": "void Vector::scale(double factor) {\n  (*this)[0] = _x * factor;\n  (*this)[1] = _y * factor;\n  (*this)[2] = _z * factor;\n}"
        },
        "rust": {
          "sig": "scale(factor: f64)",
          "code": "pub fn scale(&mut self, factor: f64) {\n        self._x *= factor;\n        self._y *= factor;\n        self._z *= factor;\n        self.invalidate_length_cache();\n    }"
        }
      }
    },
    {
      "name": "Vector.scale_up",
      "implementations": {
        "cpp": {
          "sig": "void scale_up()",
          "code": "void Vector::scale_up() { scale(static_cast<double>(session_cpp::SCALE)); }"
        },
        "rust": {
          "sig": "scale_up()",
          "code": "pub fn scale_up(&mut self) {\n        self.scale(SCALE);\n    }"
        }
      }
    },
    {
      "name": "Vector.scale_down",
      "implementations": {
        "cpp": {
          "sig": "void scale_down()",
          "code": "void Vector::scale_down() { scale(1.0 / static_cast<double>(session_cpp::SCALE)); }"
        },
        "rust": {
          "sig": "scale_down()",
          "code": "pub fn scale_down(&mut self) {\n        self.scale(1.0 / SCALE);\n    }"
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
    },
    {
      "name": "Vector.new",
      "implementations": {
        "rust": {
          "sig": "new(x: f64, y: f64, z: f64) -> Self",
          "code": "pub fn new(x: f64, y: f64, z: f64) -> Self {\n        Self {\n            _x: x,\n            _y: y,\n            _z: z,\n            ..Default::default()\n        }\n    }"
        }
      }
    },
    {
      "name": "Vector.with_name",
      "implementations": {
        "rust": {
          "sig": "with_name(x: f64, y: f64, z: f64, name: &str) -> Self",
          "code": "pub fn with_name(x: f64, y: f64, z: f64, name: &str) -> Self {\n        Self {\n            _x: x,\n            _y: y,\n            _z: z,\n            name: name.to_string(),\n            ..Default::default()\n        }\n    }"
        }
      }
    },
    {
      "name": "Vector.duplicate",
      "implementations": {
        "rust": {
          "sig": "duplicate() -> Self",
          "code": "pub fn duplicate(&self) -> Self {\n        let mut copy = self.clone();\n        copy.guid = Uuid::new_v4().to_string();\n        copy\n    }"
        }
      }
    },
    {
      "name": "Vector.zero",
      "implementations": {
        "rust": {
          "sig": "zero() -> Self",
          "code": "pub fn zero() -> Self {\n        Self::new(0.0, 0.0, 0.0)\n    }"
        }
      }
    },
    {
      "name": "Vector.projection_with",
      "implementations": {
        "rust": {
          "sig": "projection_with(onto: &Vector, tolerance: f64) -> (Vector, f64, Vector, f64)",
          "code": "pub fn projection_with(&self, onto: &Vector, tolerance: f64) -> (Vector, f64, Vector, f64) {\n        let onto_len_sq = onto.length_squared();\n\n        if onto_len_sq < tolerance {\n            return (Vector::zero(), 0.0, Vector::zero(), 0.0);\n        }\n\n        // Unit vector along 'onto'\n        let onto_len = onto_len_sq.sqrt();\n        let onto_unit = Vector::new(onto._x / onto_len, onto._y / onto_len, onto._z / onto_len);\n\n        // Scalar projection and projected vector\n        let pro"
        }
      }
    },
    {
      "name": "Vector.angle_between_vector_xy_components_degrees",
      "implementations": {
        "rust": {
          "sig": "angle_between_vector_xy_components_degrees(vector: &Vector) -> f64",
          "code": "pub fn angle_between_vector_xy_components_degrees(vector: &Vector) -> f64 {\n        Self::angle_between_vector_xy_components(vector)\n    }"
        }
      }
    },
    {
      "name": "Vector.to_json",
      "implementations": {
        "rust": {
          "sig": "to_json(filepath: &str) -> Result<(), Box<dyn std::error::Error>>",
          "code": "pub fn to_json(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {\n        let json = self.jsondump()?;\n        std::fs::write(filepath, json)?;\n        Ok(())\n    }"
        }
      }
    },
    {
      "name": "Vector.from_json",
      "implementations": {
        "rust": {
          "sig": "from_json(filepath: &str) -> Result<Self, Box<dyn std::error::Error>>",
          "code": "pub fn from_json(filepath: &str) -> Result<Self, Box<dyn std::error::Error>> {\n        let json = std::fs::read_to_string(filepath)?;\n        Self::jsonload(&json)\n    }"
        }
      }
    }
  ]
};
