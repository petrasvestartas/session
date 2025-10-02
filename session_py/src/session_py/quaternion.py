import uuid
import json
import math
from .vector import Vector


class Quaternion:
    def __init__(self, s=1.0, v=None):
        self.typ = "Quaternion"
        self.guid = str(uuid.uuid4())
        self.name = "my_quaternion"
        self.s = s
        self.v = v if v is not None else Vector(0.0, 0.0, 0.0)

    @staticmethod
    def identity():
        return Quaternion(1.0, Vector(0.0, 0.0, 0.0))

    @staticmethod
    def from_sv(s, x, y, z):
        return Quaternion(s, Vector(x, y, z))

    @staticmethod
    def from_axis_angle(axis, angle):
        axis = axis.normalize()
        half_angle = angle * 0.5
        s = math.cos(half_angle)
        v = axis * math.sin(half_angle)
        return Quaternion(s, v)

    def rotate_vector(self, v):
        qv = self.v
        uv = qv.cross(v)
        uuv = qv.cross(uv)
        return v + (uv * self.s + uuv) * 2.0

    def magnitude(self):
        return math.sqrt(
            self.s * self.s + self.v.x * self.v.x + self.v.y * self.v.y + self.v.z * self.v.z
        )

    def normalize(self):
        mag = self.magnitude()
        if mag > 1e-10:
            q = Quaternion(self.s / mag, self.v / mag)
            q.typ = self.typ
            q.guid = self.guid
            q.name = self.name
            return q
        else:
            return Quaternion.identity()

    def conjugate(self):
        q = Quaternion(self.s, self.v * -1.0)
        q.typ = self.typ
        q.guid = self.guid
        q.name = self.name
        return q

    def __mul__(self, other):
        if isinstance(other, Quaternion):
            s = self.s * other.s - self.v.dot(other.v)
            v = other.v * self.s + self.v * other.s + self.v.cross(other.v)
            return Quaternion(s, v)
        raise TypeError("Quaternion can only be multiplied with another Quaternion")

    def to_json_data(self):
        return {
            "type": self.typ,
            "guid": self.guid,
            "name": self.name,
            "s": self.s,
            "x": self.v.x,
            "y": self.v.y,
            "z": self.v.z,
        }

    @staticmethod
    def from_json_data(data):
        q = Quaternion(data["s"], Vector(data["x"], data["y"], data["z"]))
        q.typ = data.get("type", "Quaternion")
        q.guid = data["guid"]
        q.name = data["name"]
        return q

    def to_json(self, filepath):
        with open(filepath, "w") as f:
            json.dump(self.to_json_data(), f, indent=4)

    @staticmethod
    def from_json(filepath):
        with open(filepath, "r") as f:
            data = json.load(f)
            return Quaternion.from_json_data(data)
