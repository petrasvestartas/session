import uuid
import math
from typing import Optional, TYPE_CHECKING

if TYPE_CHECKING:
    from .point import Point

from .vector import Vector


class Xform:
    def __init__(self, m=None):
        self.guid = str(uuid.uuid4())
        self.name = "my_xform"
        if m is None:
            self.m = [
                1.0,
                0.0,
                0.0,
                0.0,
                0.0,
                1.0,
                0.0,
                0.0,
                0.0,
                0.0,
                1.0,
                0.0,
                0.0,
                0.0,
                0.0,
                1.0,
            ]
        else:
            self.m = list(m)

    @staticmethod
    def identity():
        return Xform()

    @staticmethod
    def from_matrix(m):
        return Xform(m)

    @staticmethod
    def translation(x, y, z):
        xform = Xform()
        xform.m[12] = x
        xform.m[13] = y
        xform.m[14] = z
        return xform

    @staticmethod
    def scaling(x, y, z):
        xform = Xform()
        xform.m[0] = x
        xform.m[5] = y
        xform.m[10] = z
        return xform

    @staticmethod
    def rotation_x(angle_radians):
        xform = Xform()
        cos_angle = math.cos(angle_radians)
        sin_angle = math.sin(angle_radians)
        xform.m[5] = cos_angle
        xform.m[6] = sin_angle
        xform.m[9] = -sin_angle
        xform.m[10] = cos_angle
        return xform

    @staticmethod
    def rotation_y(angle_radians):
        xform = Xform()
        cos_angle = math.cos(angle_radians)
        sin_angle = math.sin(angle_radians)
        xform.m[0] = cos_angle
        xform.m[2] = -sin_angle
        xform.m[8] = sin_angle
        xform.m[10] = cos_angle
        return xform

    @staticmethod
    def rotation_z(angle_radians):
        xform = Xform()
        cos_angle = math.cos(angle_radians)
        sin_angle = math.sin(angle_radians)
        xform.m[0] = cos_angle
        xform.m[1] = sin_angle
        xform.m[4] = -sin_angle
        xform.m[5] = cos_angle
        return xform

    @staticmethod
    def rotation(axis, angle_radians):
        xform = Xform()
        axis = axis.normalize()
        cos_angle = math.cos(angle_radians)
        sin_angle = math.sin(angle_radians)
        one_minus_cos = 1.0 - cos_angle
        xx = axis.x * axis.x
        xy = axis.x * axis.y
        xz = axis.x * axis.z
        yy = axis.y * axis.y
        yz = axis.y * axis.z
        zz = axis.z * axis.z
        xform.m[0] = cos_angle + xx * one_minus_cos
        xform.m[1] = xy * one_minus_cos + axis.z * sin_angle
        xform.m[2] = xz * one_minus_cos - axis.y * sin_angle
        xform.m[4] = xy * one_minus_cos - axis.z * sin_angle
        xform.m[5] = cos_angle + yy * one_minus_cos
        xform.m[6] = yz * one_minus_cos + axis.x * sin_angle
        xform.m[8] = xz * one_minus_cos + axis.y * sin_angle
        xform.m[9] = yz * one_minus_cos - axis.x * sin_angle
        xform.m[10] = cos_angle + zz * one_minus_cos
        return xform

    @staticmethod
    def change_basis(origin, x_axis, y_axis, z_axis):
        xform = Xform()
        x_axis = x_axis.normalize()
        y_axis = y_axis.normalize()
        z_axis = z_axis.normalize()
        xform.m[0] = x_axis.x
        xform.m[1] = x_axis.y
        xform.m[2] = x_axis.z
        xform.m[4] = y_axis.x
        xform.m[5] = y_axis.y
        xform.m[6] = y_axis.z
        xform.m[8] = z_axis.x
        xform.m[9] = z_axis.y
        xform.m[10] = z_axis.z
        xform.m[12] = origin.x
        xform.m[13] = origin.y
        xform.m[14] = origin.z
        return xform

    @staticmethod
    def change_basis_alt(
        origin_1, x_axis_1, y_axis_1, z_axis_1, origin_0, x_axis_0, y_axis_0, z_axis_0
    ):
        a = x_axis_1.dot(y_axis_1)
        b = x_axis_1.dot(z_axis_1)
        c = y_axis_1.dot(z_axis_1)
        r = [
            [
                x_axis_1.dot(x_axis_1),
                a,
                b,
                x_axis_1.dot(x_axis_0),
                x_axis_1.dot(y_axis_0),
                x_axis_1.dot(z_axis_0),
            ],
            [
                a,
                y_axis_1.dot(y_axis_1),
                c,
                y_axis_1.dot(x_axis_0),
                y_axis_1.dot(y_axis_0),
                y_axis_1.dot(z_axis_0),
            ],
            [
                b,
                c,
                z_axis_1.dot(z_axis_1),
                z_axis_1.dot(x_axis_0),
                z_axis_1.dot(y_axis_0),
                z_axis_1.dot(z_axis_0),
            ],
        ]
        i0 = 0 if r[0][0] >= r[1][1] else 1
        if r[2][2] > r[i0][i0]:
            i0 = 2
        i1 = (i0 + 1) % 3
        i2 = (i1 + 1) % 3
        if r[i0][i0] == 0.0:
            return Xform.identity()
        d = 1.0 / r[i0][i0]
        for j in range(6):
            r[i0][j] *= d
        r[i0][i0] = 1.0
        if r[i1][i0] != 0.0:
            d = -r[i1][i0]
            for j in range(6):
                r[i1][j] += d * r[i0][j]
            r[i1][i0] = 0.0
        if r[i2][i0] != 0.0:
            d = -r[i2][i0]
            for j in range(6):
                r[i2][j] += d * r[i0][j]
            r[i2][i0] = 0.0
        if abs(r[i1][i1]) < abs(r[i2][i2]):
            i1, i2 = i2, i1
        if r[i1][i1] == 0.0:
            return Xform.identity()
        d = 1.0 / r[i1][i1]
        for j in range(6):
            r[i1][j] *= d
        r[i1][i1] = 1.0
        if r[i0][i1] != 0.0:
            d = -r[i0][i1]
            for j in range(6):
                r[i0][j] += d * r[i1][j]
            r[i0][i1] = 0.0
        if r[i2][i1] != 0.0:
            d = -r[i2][i1]
            for j in range(6):
                r[i2][j] += d * r[i1][j]
            r[i2][i1] = 0.0
        if r[i2][i2] == 0.0:
            return Xform.identity()
        d = 1.0 / r[i2][i2]
        for j in range(6):
            r[i2][j] *= d
        r[i2][i2] = 1.0
        if r[i0][i2] != 0.0:
            d = -r[i0][i2]
            for j in range(6):
                r[i0][j] += d * r[i2][j]
            r[i0][i2] = 0.0
        if r[i1][i2] != 0.0:
            d = -r[i1][i2]
            for j in range(6):
                r[i1][j] += d * r[i2][j]
            r[i1][i2] = 0.0
        m_xform = Xform()
        m_xform.m[0] = r[0][3]
        m_xform.m[4] = r[0][4]
        m_xform.m[8] = r[0][5]
        m_xform.m[1] = r[1][3]
        m_xform.m[5] = r[1][4]
        m_xform.m[9] = r[1][5]
        m_xform.m[2] = r[2][3]
        m_xform.m[6] = r[2][4]
        m_xform.m[10] = r[2][5]
        t0 = Xform.translation(-origin_1.x, -origin_1.y, -origin_1.z)
        t2 = Xform.translation(origin_0.x, origin_0.y, origin_0.z)
        return t2 * (m_xform * t0)

    @staticmethod
    def plane_to_plane(
        origin_0, x_axis_0, y_axis_0, z_axis_0, origin_1, x_axis_1, y_axis_1, z_axis_1
    ):
        x0 = x_axis_0.normalize()
        y0 = y_axis_0.normalize()
        z0 = z_axis_0.normalize()
        x1 = x_axis_1.normalize()
        y1 = y_axis_1.normalize()
        z1 = z_axis_1.normalize()
        t0 = Xform.translation(-origin_0.x, -origin_0.y, -origin_0.z)
        f0 = Xform()
        f0.m[0] = x0.x
        f0.m[1] = x0.y
        f0.m[2] = x0.z
        f0.m[4] = y0.x
        f0.m[5] = y0.y
        f0.m[6] = y0.z
        f0.m[8] = z0.x
        f0.m[9] = z0.y
        f0.m[10] = z0.z
        f1 = Xform()
        f1.m[0] = x1.x
        f1.m[4] = x1.y
        f1.m[8] = x1.z
        f1.m[1] = y1.x
        f1.m[5] = y1.y
        f1.m[9] = y1.z
        f1.m[2] = z1.x
        f1.m[6] = z1.y
        f1.m[10] = z1.z
        r = f1 * f0
        t1 = Xform.translation(origin_1.x, origin_1.y, origin_1.z)
        return t1 * (r * t0)

    @staticmethod
    def plane_to_xy(origin, x_axis, y_axis, z_axis):
        x = x_axis.normalize()
        y = y_axis.normalize()
        z = z_axis.normalize()
        t = Xform.translation(-origin.x, -origin.y, -origin.z)
        f = Xform()
        f.m[0] = x.x
        f.m[1] = x.y
        f.m[2] = x.z
        f.m[4] = y.x
        f.m[5] = y.y
        f.m[6] = y.z
        f.m[8] = z.x
        f.m[9] = z.y
        f.m[10] = z.z
        return f * t

    @staticmethod
    def xy_to_plane(origin, x_axis, y_axis, z_axis):
        x = x_axis.normalize()
        y = y_axis.normalize()
        z = z_axis.normalize()
        f = Xform()
        f.m[0] = x.x
        f.m[4] = y.x
        f.m[8] = z.x
        f.m[1] = x.y
        f.m[5] = y.y
        f.m[9] = z.y
        f.m[2] = x.z
        f.m[6] = y.z
        f.m[10] = z.z
        t = Xform.translation(origin.x, origin.y, origin.z)
        return t * f

    @staticmethod
    def scale_xyz(scale_x, scale_y, scale_z):
        xform = Xform()
        xform.m[0] = scale_x
        xform.m[5] = scale_y
        xform.m[10] = scale_z
        return xform

    @staticmethod
    def scale_uniform(origin, scale_value):
        t0 = Xform.translation(-origin.x, -origin.y, -origin.z)
        t1 = Xform.scaling(scale_value, scale_value, scale_value)
        t2 = Xform.translation(origin.x, origin.y, origin.z)
        return t2 * (t1 * t0)

    @staticmethod
    def scale_non_uniform(origin, scale_x, scale_y, scale_z):
        t0 = Xform.translation(-origin.x, -origin.y, -origin.z)
        t1 = Xform.scale_xyz(scale_x, scale_y, scale_z)
        t2 = Xform.translation(origin.x, origin.y, origin.z)
        return t2 * (t1 * t0)

    @staticmethod
    def axis_rotation(angle, axis):
        c = math.cos(angle)
        s = math.sin(angle)
        ux = axis.x
        uy = axis.y
        uz = axis.z
        t = 1.0 - c
        xform = Xform()
        xform.m[0] = t * ux * ux + c
        xform.m[4] = t * ux * uy - uz * s
        xform.m[8] = t * ux * uz + uy * s
        xform.m[1] = t * ux * uy + uz * s
        xform.m[5] = t * uy * uy + c
        xform.m[9] = t * uy * uz - ux * s
        xform.m[2] = t * ux * uz - uy * s
        xform.m[6] = t * uy * uz + ux * s
        xform.m[10] = t * uz * uz + c
        return xform

    @staticmethod
    def look_at_rh(eye, target, up):
        from .vector import Vector

        f = (target - eye).normalize()
        s = f.cross(up.normalize()).normalize()
        u = s.cross(f)
        xform = Xform()
        xform.m[0] = s.x
        xform.m[4] = s.y
        xform.m[8] = s.z
        xform.m[1] = u.x
        xform.m[5] = u.y
        xform.m[9] = u.z
        xform.m[2] = -f.x
        xform.m[6] = -f.y
        xform.m[10] = -f.z
        eye_vec = Vector(eye.x, eye.y, eye.z)
        xform.m[12] = -s.dot(eye_vec)
        xform.m[13] = -u.dot(eye_vec)
        xform.m[14] = f.dot(eye_vec)
        return xform

    def inverse(self) -> Optional["Xform"]:
        a00 = self.m[0]
        a01 = self.m[4]
        a02 = self.m[8]
        a10 = self.m[1]
        a11 = self.m[5]
        a12 = self.m[9]
        a20 = self.m[2]
        a21 = self.m[6]
        a22 = self.m[10]
        det = (
            a00 * (a11 * a22 - a12 * a21)
            - a01 * (a10 * a22 - a12 * a20)
            + a02 * (a10 * a21 - a11 * a20)
        )
        if abs(det) < 1e-12:
            return None
        inv_det = 1.0 / det
        m00 = (a11 * a22 - a12 * a21) * inv_det
        m01 = (a02 * a21 - a01 * a22) * inv_det
        m02 = (a01 * a12 - a02 * a11) * inv_det
        m10 = (a12 * a20 - a10 * a22) * inv_det
        m11 = (a00 * a22 - a02 * a20) * inv_det
        m12 = (a02 * a10 - a00 * a12) * inv_det
        m20 = (a10 * a21 - a11 * a20) * inv_det
        m21 = (a01 * a20 - a00 * a21) * inv_det
        m22 = (a00 * a11 - a01 * a10) * inv_det
        tx = self.m[12]
        ty = self.m[13]
        tz = self.m[14]
        itx = -(m00 * tx + m01 * ty + m02 * tz)
        ity = -(m10 * tx + m11 * ty + m12 * tz)
        itz = -(m20 * tx + m21 * ty + m22 * tz)
        res = Xform()
        res.guid = ""
        res.name = ""
        res.m[0] = m00
        res.m[4] = m01
        res.m[8] = m02
        res.m[12] = itx
        res.m[1] = m10
        res.m[5] = m11
        res.m[9] = m12
        res.m[13] = ity
        res.m[2] = m20
        res.m[6] = m21
        res.m[10] = m22
        res.m[14] = itz
        return res

    def is_identity(self):
        identity = Xform.identity()
        for i in range(16):
            if abs(self.m[i] - identity.m[i]) > 1e-10:
                return False
        return True

    def transformed_point(self, point):
        from .point import Point

        x = point.x
        y = point.y
        z = point.z
        w = self.m[3] * x + self.m[7] * y + self.m[11] * z + self.m[15]
        w_inv = 1.0 / w if abs(w) > 1e-10 else 1.0
        return Point(
            (self.m[0] * x + self.m[4] * y + self.m[8] * z + self.m[12]) * w_inv,
            (self.m[1] * x + self.m[5] * y + self.m[9] * z + self.m[13]) * w_inv,
            (self.m[2] * x + self.m[6] * y + self.m[10] * z + self.m[14]) * w_inv,
        )

    def transformed_vector(self, vector):
        x = vector.x
        y = vector.y
        z = vector.z
        return Vector(
            self.m[0] * x + self.m[4] * y + self.m[8] * z,
            self.m[1] * x + self.m[5] * y + self.m[9] * z,
            self.m[2] * x + self.m[6] * y + self.m[10] * z,
        )

    def transform_point(self, point):
        x = point.x
        y = point.y
        z = point.z
        w = self.m[3] * x + self.m[7] * y + self.m[11] * z + self.m[15]
        w_inv = 1.0 / w if abs(w) > 1e-10 else 1.0
        point.x = (self.m[0] * x + self.m[4] * y + self.m[8] * z + self.m[12]) * w_inv
        point.y = (self.m[1] * x + self.m[5] * y + self.m[9] * z + self.m[13]) * w_inv
        point.z = (self.m[2] * x + self.m[6] * y + self.m[10] * z + self.m[14]) * w_inv

    def transform_vector(self, vector):
        x = vector.x
        y = vector.y
        z = vector.z
        vector.x = self.m[0] * x + self.m[4] * y + self.m[8] * z
        vector.y = self.m[1] * x + self.m[5] * y + self.m[9] * z
        vector.z = self.m[2] * x + self.m[6] * y + self.m[10] * z

    def __mul__(self, other):
        result = Xform()
        result.m = [0.0] * 16
        for i in range(4):
            for j in range(4):
                sum_val = 0.0
                for k in range(4):
                    sum_val += self.m[k * 4 + i] * other.m[j * 4 + k]
                result.m[j * 4 + i] = sum_val
        return result

    def __imul__(self, other):
        temp = self * other
        self.m = temp.m
        return self

    def __getitem__(self, idx):
        if isinstance(idx, tuple) and len(idx) == 2:
            row, col = idx
            if not (0 <= row < 4 and 0 <= col < 4):
                raise IndexError(f"Index out of bounds: ({row}, {col})")
            return self.m[col * 4 + row]
        raise TypeError("Index must be a tuple of (row, col)")

    def __setitem__(self, idx, value):
        if isinstance(idx, tuple) and len(idx) == 2:
            row, col = idx
            if not (0 <= row < 4 and 0 <= col < 4):
                raise IndexError(f"Index out of bounds: ({row}, {col})")
            self.m[col * 4 + row] = value
        else:
            raise TypeError("Index must be a tuple of (row, col)")

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
            "m": self.m,
        }

    @classmethod
    def __jsonload__(cls, data, guid=None, name=None):
        """Deserialize from polymorphic JSON format.

        Parameters
        ----------
        data : dict
            Dictionary containing xform data.
        guid : str, optional
            GUID for the xform.
        name : str, optional
            Name for the xform.

        Returns
        -------
        :class:`Xform`
            Reconstructed xform instance.

        """
        xform = cls.from_matrix(data["m"])
        xform.guid = guid
        xform.name = name
        return xform
