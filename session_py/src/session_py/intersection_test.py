from . import Line
from . import intersection


def test_line_line_intersection():
    l0 = Line(500.000, -573.576, -819.152, 500.000, 573.576, 819.152)
    l1 = Line(13.195, 234.832, 534.315, 986.805, 421.775, 403.416)
    
    p = intersection.line_line(l0, l1, 1e-3)
    
    assert p is not None
    assert abs(p.x - 500.0) < 0.1
    assert abs(p.y - 328.303) < 0.1
    assert abs(p.z - 468.866) < 0.1
    print(f"✓ Python line_line: {p.x}, {p.y}, {p.z}")


def test_line_line_parameters():
    l0 = Line(500.000, -573.576, -819.152, 500.000, 573.576, 819.152)
    l1 = Line(13.195, 234.832, 534.315, 986.805, 421.775, 403.416)
    
    result = intersection.line_line_parameters(l0, l1, 1e-3)
    
    assert result is not None
    t0, t1 = result
    assert 0.0 <= t0 <= 1.0
    assert 0.0 <= t1 <= 1.0
    print(f"✓ Python line_line_parameters: t0={t0}, t1={t1}")
