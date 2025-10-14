import math
from .xform import Xform
from .point import Point
from .vector import Vector


def approx_f32(a, b):
    return abs(float(a) - float(b)) < 1e-5


def matrices_close(a, b):
    for i in range(16):
        if not approx_f32(a.m[i], b.m[i]):
            return False
    return True


def test_xform_identity():
    id_xform = Xform.identity()
    assert id_xform.is_identity()


def test_xform_default():
    def_xform = Xform()
    assert def_xform.is_identity()


def test_xform_identity_transformed_point():
    p = Point(1.0, 2.0, 3.0)
    t = Xform.identity().transformed_point(p)
    assert t.x == 1.0
    assert t.y == 2.0
    assert t.z == 3.0


def test_xform_translation_point():
    t = Xform.translation(1.0, 2.0, 3.0)
    p = Point(4.0, 5.0, 6.0)
    tp = t.transformed_point(p)
    assert tp.x == 5.0
    assert tp.y == 7.0
    assert tp.z == 9.0


def test_xform_translation_vector():
    t = Xform.translation(1.0, 2.0, 3.0)
    v = Vector(1.0, 2.0, 3.0)
    tv = t.transformed_vector(v)
    assert tv.x == 1.0
    assert tv.y == 2.0
    assert tv.z == 3.0


def test_xform_scaling_point():
    s = Xform.scaling(2.0, 3.0, 4.0)
    p = Point(1.0, -2.0, 0.5)
    sp = s.transformed_point(p)
    assert sp.x == 2.0
    assert sp.y == -6.0
    assert sp.z == 2.0


def test_xform_scaling_vector():
    s = Xform.scaling(2.0, 3.0, 4.0)
    v = Vector(1.0, -2.0, 0.5)
    sv = s.transformed_vector(v)
    assert sv.x == 2.0
    assert sv.y == -6.0
    assert sv.z == 2.0


def test_xform_rotation_z():
    r = Xform.rotation_z(math.pi / 2.0)
    p = Point(1.0, 0.0, 0.0)
    rp = r.transformed_point(p)
    assert approx_f32(rp.x, 0.0)
    assert approx_f32(rp.y, 1.0)
    assert approx_f32(rp.z, 0.0)


def test_xform_axis_rotation():
    axis = Vector(0.0, 0.0, 1.0)
    r1 = Xform.rotation_z(math.pi / 2.0)
    r2 = Xform.axis_rotation(math.pi / 2.0, axis)
    p = Point(1.0, 0.0, 0.0)
    p1 = r1.transformed_point(p)
    p2 = r2.transformed_point(p)
    assert approx_f32(p1.x, p2.x)
    assert approx_f32(p1.y, p2.y)
    assert approx_f32(p1.z, p2.z)


def test_xform_inverse():
    t = (
        Xform.translation(1.0, 2.0, 3.0)
        * Xform.rotation_z(0.7)
        * Xform.scaling(2.0, 2.0, 2.0)
    )
    inv = t.inverse()
    id_result = t * inv
    assert matrices_close(id_result, Xform.identity())


def test_xform_change_basis_alt_identity():
    o0 = Point(0.0, 0.0, 0.0)
    o1 = Point(0.0, 0.0, 0.0)
    x = Vector(1.0, 0.0, 0.0)
    y = Vector(0.0, 1.0, 0.0)
    z = Vector(0.0, 0.0, 1.0)
    cb = Xform.change_basis_alt(o1, x, y, z, o0, x, y, z)
    assert cb.is_identity()


def test_xform_change_basis_alt_translation():
    o0 = Point(4.0, 5.0, 6.0)
    o1 = Point(1.0, 2.0, 3.0)
    x = Vector(1.0, 0.0, 0.0)
    y = Vector(0.0, 1.0, 0.0)
    z = Vector(0.0, 0.0, 1.0)
    cb = Xform.change_basis_alt(o1, x, y, z, o0, x, y, z)
    p = Point(1.0, 1.0, 1.0)
    tp = cb.transformed_point(p)
    assert approx_f32(tp.x, p.x + 3.0)
    assert approx_f32(tp.y, p.y + 3.0)
    assert approx_f32(tp.z, p.z + 3.0)


def test_xform_plane_to_plane():
    o0 = Point(1.0, 2.0, 3.0)
    o1 = Point(-2.0, 0.5, 7.0)
    x0 = Vector(1.0, 0.0, 0.0)
    y0 = Vector(0.0, 1.0, 0.0)
    z0 = Vector(0.0, 0.0, 1.0)
    x1 = Vector(1.0, 0.0, 0.0)
    y1 = Vector(0.0, 1.0, 0.0)
    z1 = Vector(0.0, 0.0, 1.0)
    m = Xform.plane_to_plane(o0, x0, y0, z0, o1, x1, y1, z1)
    mapped = m.transformed_point(o0)
    assert approx_f32(mapped.x, o1.x)
    assert approx_f32(mapped.y, o1.y)
    assert approx_f32(mapped.z, o1.z)


def test_xform_mul():
    a = Xform.translation(1.0, 2.0, 3.0)
    b = Xform.scaling(2.0, 3.0, 4.0)
    r_ref = a * b
    r_owned = a * b
    assert matrices_close(r_ref, r_owned)


def test_xform_mul_assign():
    a = Xform.translation(1.0, 2.0, 3.0)
    b = Xform.scaling(2.0, 3.0, 4.0)
    acc = Xform.identity()
    acc *= a
    acc *= b
    r2 = Xform.identity() * (
        Xform.translation(1.0, 2.0, 3.0) * Xform.scaling(2.0, 3.0, 4.0)
    )
    assert matrices_close(acc, r2)


def test_xform_json_round_trip():
    from session_py.encoders import json_dumps, json_loads

    x = Xform.from_matrix(
        [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 4.0, 5.0, 6.0, 1.0]
    )
    s = json_dumps(x)
    y = json_loads(s)
    assert matrices_close(x, y)


def test_xform_from_matrix():
    m = [
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
        5.0,
        10.0,
        15.0,
        1.0,
    ]
    x = Xform.from_matrix(m)
    assert x.m == m


def test_xform_rotation_x():
    r = Xform.rotation_x(math.pi / 2.0)
    p = Point(0.0, 1.0, 0.0)
    rp = r.transformed_point(p)
    assert approx_f32(rp.x, 0.0)
    assert approx_f32(rp.y, 0.0)
    assert approx_f32(rp.z, 1.0)


def test_xform_rotation_y():
    r = Xform.rotation_y(math.pi / 2.0)
    p = Point(1.0, 0.0, 0.0)
    rp = r.transformed_point(p)
    assert approx_f32(rp.x, 0.0)
    assert approx_f32(rp.y, 0.0)
    assert approx_f32(rp.z, -1.0)


def test_xform_rotation():
    axis = Vector(0.0, 0.0, 1.0)
    r = Xform.rotation(axis, math.pi / 2.0)
    p = Point(1.0, 0.0, 0.0)
    rp = r.transformed_point(p)
    assert approx_f32(rp.x, 0.0)
    assert approx_f32(rp.y, 1.0)
    assert approx_f32(rp.z, 0.0)


def test_xform_change_basis():
    o = Point(1.0, 2.0, 3.0)
    x = Vector(1.0, 0.0, 0.0)
    y = Vector(0.0, 1.0, 0.0)
    z = Vector(0.0, 0.0, 1.0)
    cb = Xform.change_basis(o, x, y, z)
    assert approx_f32(cb.m[12], 1.0)
    assert approx_f32(cb.m[13], 2.0)
    assert approx_f32(cb.m[14], 3.0)


def test_xform_plane_to_xy():
    o = Point(1.0, 2.0, 3.0)
    x = Vector(1.0, 0.0, 0.0)
    y = Vector(0.0, 1.0, 0.0)
    z = Vector(0.0, 0.0, 1.0)
    m = Xform.plane_to_xy(o, x, y, z)
    mapped = m.transformed_point(o)
    assert approx_f32(mapped.x, 0.0)
    assert approx_f32(mapped.y, 0.0)
    assert approx_f32(mapped.z, 0.0)


def test_xform_xy_to_plane():
    o = Point(1.0, 2.0, 3.0)
    x = Vector(1.0, 0.0, 0.0)
    y = Vector(0.0, 1.0, 0.0)
    z = Vector(0.0, 0.0, 1.0)
    m = Xform.xy_to_plane(o, x, y, z)
    origin = Point(0.0, 0.0, 0.0)
    mapped = m.transformed_point(origin)
    assert approx_f32(mapped.x, o.x)
    assert approx_f32(mapped.y, o.y)
    assert approx_f32(mapped.z, o.z)


def test_xform_scale_xyz():
    s = Xform.scale_xyz(2.0, 3.0, 4.0)
    p = Point(1.0, 1.0, 1.0)
    sp = s.transformed_point(p)
    assert sp.x == 2.0
    assert sp.y == 3.0
    assert sp.z == 4.0


def test_xform_scale_uniform():
    o = Point(1.0, 1.0, 1.0)
    s = Xform.scale_uniform(o, 2.0)
    p = Point(2.0, 2.0, 2.0)
    sp = s.transformed_point(p)
    assert approx_f32(sp.x, 3.0)
    assert approx_f32(sp.y, 3.0)
    assert approx_f32(sp.z, 3.0)


def test_xform_scale_non_uniform():
    o = Point(0.0, 0.0, 0.0)
    s = Xform.scale_non_uniform(o, 2.0, 3.0, 4.0)
    p = Point(1.0, 1.0, 1.0)
    sp = s.transformed_point(p)
    assert sp.x == 2.0
    assert sp.y == 3.0
    assert sp.z == 4.0


def test_xform_is_identity():
    x = Xform.identity()
    assert x.is_identity()
    x.m[0] = 2.0
    assert not x.is_identity()


def test_xform_transformed_point():
    t = Xform.translation(1.0, 2.0, 3.0)
    p = Point(0.0, 0.0, 0.0)
    tp = t.transformed_point(p)
    assert tp.x == 1.0
    assert tp.y == 2.0
    assert tp.z == 3.0


def test_xform_transformed_vector():
    s = Xform.scaling(2.0, 3.0, 4.0)
    v = Vector(1.0, 1.0, 1.0)
    sv = s.transformed_vector(v)
    assert sv.x == 2.0
    assert sv.y == 3.0
    assert sv.z == 4.0


def test_xform_transform_point():
    t = Xform.translation(1.0, 2.0, 3.0)
    p = Point(0.0, 0.0, 0.0)
    t.transform_point(p)
    assert p.x == 1.0
    assert p.y == 2.0
    assert p.z == 3.0


def test_xform_transform_vector():
    s = Xform.scaling(2.0, 3.0, 4.0)
    v = Vector(1.0, 1.0, 1.0)
    s.transform_vector(v)
    assert v.x == 2.0
    assert v.y == 3.0
    assert v.z == 4.0


def test_xform_getitem():
    x = Xform.identity()
    assert x[0, 0] == 1.0
    assert x[1, 1] == 1.0
    assert x[2, 2] == 1.0
    assert x[3, 3] == 1.0
    assert x[0, 3] == 0.0


def test_xform_setitem():
    x = Xform.identity()
    x[0, 3] = 5.0
    x[1, 3] = 10.0
    x[2, 3] = 15.0
    assert x[0, 3] == 5.0
    assert x[1, 3] == 10.0
    assert x[2, 3] == 15.0


def test_xform_json_roundtrip():
    from pathlib import Path
    from session_py.encoders import json_dump, json_load

    xform = Xform.translation(1.0, 2.0, 3.0)
    xform.name = "test_xform"

    path = Path(__file__).resolve().parents[2] / "test_xform.json"
    json_dump(xform, path)
    loaded = json_load(path)

    assert isinstance(loaded, Xform)
    assert loaded[0, 3] == xform[0, 3]
    assert loaded.name == xform.name
