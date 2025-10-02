import session_py.globals as globals


def test_globals_initial_values():
    assert globals.SCALE == 1e6
    assert globals.PI == 3.141592653589793
    assert globals.ANGLE == 0.11
    assert globals.TOLERANCE == 1e-3


def test_globals_modification():
    original_scale = globals.SCALE
    globals.SCALE = 2000.0
    assert globals.SCALE == 2000.0
    globals.SCALE = original_scale
