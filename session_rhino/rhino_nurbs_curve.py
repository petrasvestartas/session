#! python3
# venv: session_py

from session_py.reload import reload_session
reload_session()

# from session_py import NurbsCurve
# from session_py import Point
# from pathlib import Path
from session_py import NurbsCurve, Point
from pathlib import Path
import Rhino
from Rhino.Geometry import Point3d

points = [
    Point(0.0, 0.0, 0.0),
    Point(1.0, 1.0, 0.0),
    Point(2.0, 0.0, 0.0),
    Point(3.0, 1.0, 0.0)
]

# The first the curve is closed or open
# For linear curves use degree 1
# When 3 points use degree 2 curve, Rhino default
# When x>3 points use degree 3 curve
curve = NurbsCurve.create(periodic=False, degree=2, points=points)
curve.set_domain(0.0, 1.0)

# Minimal and Full String Representation
cstr = str(curve)
crepr = repr(curve)
print(f"str: {cstr}")
print(f"repr: {crepr}")

# Copy (duplicates everything except guid)
ccopy = curve.duplicate()
cother = NurbsCurve.create(periodic=False, degree=2, points=points)

# Point division
divided, params = curve.divide_by_count(10)
print("Divided points:")
for p in divided:
    print(p)

# Serialization
serial_dir = Path(__file__).resolve().parent.parent / "session_cpp" / "serialization"
json_path = serial_dir / "test_nurbscurve.json"
bin_path = serial_dir / "test_nurbscurve.bin"

curve.json_dump(json_path)
curve.protobuf_dump(bin_path)

loaded_json = NurbsCurve.json_load(json_path)
loaded_pb = NurbsCurve.protobuf_load(bin_path)

print(f"Loaded JSON str: {str(loaded_json)}")
print(f"Loaded Protobuf str: {str(loaded_pb)}")

# Add points to Rhino document
for p in divided:
    Rhino.RhinoDoc.ActiveDoc.Objects.AddPoint(Point3d(p[0], p[1], p[2]))
