#!/usr/bin/env python3
from src.session_py import Point
from src.session_py import Vector


def main():
    
    point = Point()
    point.to_json("point.json")
    point = Point.from_json("point.json")
    print(point)

    vector = Vector()
    vector.to_json("vector.json")
    vector = Vector.from_json("vector.json")
    print(vector)
    
    


if __name__ == "__main__":
    main()
