#! python3
# venv: session_py

from session_py.reload import reload_all
reload_all()

from session_py import NurbsCurve
from session_py import Point
from session_rhino.session import Session
from pathlib import Path

session = Session()

points = [
    Point(0.0, 0.0, 0.0),
    Point(1.0, 1.0, 0.0),
    Point(2.0, 0.0, 0.0),
    Point(3.0, 1.0, 0.0)
]

curve = NurbsCurve.create(periodic=False, degree=2, points=points)
curve.set_domain(0.0, 1.0)
session.add(curve)

print(f"str: {str(curve)}")
print(f"repr: {repr(curve)}")

ccopy = curve.duplicate()
cother = NurbsCurve.create(periodic=False, degree=2, points=points)

divided, params = curve.divide_by_count(10)
print("Divided points:")
for p in divided:
    print(p)
session.add(divided)

serial_dir = Path(__file__).resolve().parent.parent / "session_cpp" / "serialization"
json_path = serial_dir / "test_nurbscurve.json"
bin_path = serial_dir / "test_nurbscurve.bin"

curve.json_dump(json_path)
curve.pb_dump(bin_path)

loaded_json = NurbsCurve.json_load(json_path)
loaded_pb = NurbsCurve.pb_load(bin_path)

print(f"Loaded JSON str: {str(loaded_json)}")
print(f"Loaded Protobuf str: {str(loaded_pb)}")

session.draw(delete=True)
