#!/usr/bin/env python3
from src.session_py.point import Point

def main():
    print("=== Python Point JSON Demo ===")
    
    # Create a point
    point = Point(1.5, 2.5, 3.5)
    print(f"Created point: {point}")
    
    # Show JSON serialization output
    json_data = point.to_json_data()
    print(f"\nSerialized JSON:")
    import json
    print(json.dumps(json_data, indent=2))
    
    # Test deserialization from JSON data
    loaded_point = Point.from_json_data(json_data)
    print(f"\nDeserialized point: {loaded_point}")
    
    # Also save to file
    filename = "point_python.json"
    point.to_json(filename)
    print(f"\nAlso saved to file: {filename}")
    
    print("JSON serialization demo completed!")

if __name__ == "__main__":
    main()
