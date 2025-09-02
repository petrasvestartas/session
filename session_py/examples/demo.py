import json
from session_py import Point

p = Point(0, 0, 0)

p.to_json("point.json")

p2 = Point.from_json("point.json")

print(p2)