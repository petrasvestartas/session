import session_py.globals as globals


def test_globals_initial_values():
    assert globals.ZERO_TOLERANCE == 1e-12
    assert globals.SCALE == 1000.0
    assert globals.PI == 3.141592653589793


def test_globals_modification():
    globals.ZERO_TOLERANCE = 1e-15
    globals.SCALE = 2000.0
    assert globals.ZERO_TOLERANCE == 1e-15
    assert globals.SCALE == 2000.0


def test_globals_persistence():
    import session_py.globals as globals_again

    assert globals_again.ZERO_TOLERANCE == 1e-15
    assert globals_again.SCALE == 2000.0
    globals.ZERO_TOLERANCE = 1e-12  # reset to default fot the other tests
    globals.SCALE = 1000.0  # reset to default fot the other tests
