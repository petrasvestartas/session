#!/usr/bin/env python3
"""Demo script showing Scene class usage with GUID-based geometry lookup."""

from src.session_py import Scene, Point, Vector

def main():
    # Create a scene
    scene = Scene()
    
    # Create some geometry objects
    point1 = Point(1.0, 2.0, 3.0)
    point1.name = "Origin Point"
    
    point2 = Point(10.0, 20.0, 30.0)
    point2.name = "End Point"
    
    vector1 = Vector(1.0, 0.0, 0.0)
    vector1.name = "X Axis"
    
    vector2 = Vector(0.0, 1.0, 0.0)
    vector2.name = "Y Axis"
    
    # Add geometry to scene
    scene.add_point(point1)
    scene.add_point(point2)
    scene.add_vector(vector1)
    scene.add_vector(vector2)
    
    print(f"Scene created: {scene}")
    print(f"Total geometry objects: {len(scene)}")
    
    # GUID-based lookup demonstration
    print("\\n=== GUID-based Lookup ===")
    retrieved_point = scene.get_geometry_by_guid(point1.guid)
    print(f"Retrieved point by GUID: {retrieved_point}")
    
    retrieved_vector = scene.get_geometry_by_guid(vector1.guid)
    print(f"Retrieved vector by GUID: {retrieved_vector}")
    
    # Build hierarchical structure
    print("\\n=== Building Hierarchy ===")
    # Make point2 a child of point1
    success = scene.add_child(point1.guid, point2.guid)
    print(f"Added point2 as child of point1: {success}")
    
    # Make vector1 a child of point1
    success = scene.add_child(point1.guid, vector1.guid)
    print(f"Added vector1 as child of point1: {success}")
    
    # Query hierarchy
    children = scene.get_children(point1.guid)
    print(f"Children of point1: {children}")
    
    parent = scene.get_parent(point2.guid)
    print(f"Parent of point2: {parent}")
    
    # Build graph relationships
    print("\\n=== Building Graph Relationships ===")
    scene.add_edge(point1.guid, point2.guid, "connects_to", {"distance": 100.0})
    scene.add_edge(point1.guid, vector1.guid, "has_direction")
    scene.add_edge(vector1.guid, vector2.guid, "perpendicular_to")
    
    # Query graph
    connected = scene.get_connected_guids(point1.guid)
    print(f"Objects connected to point1: {connected}")
    
    edges_from_point1 = scene.get_edges_from(point1.guid)
    print(f"Edges from point1: {len(edges_from_point1)}")
    for edge in edges_from_point1:
        print(f"  -> {edge.to_guid} ({edge.relationship_type})")
    
    # JSON serialization
    print("\\n=== JSON Serialization ===")
    scene.to_json("demo_scene.json")
    print("Scene saved to demo_scene.json")
    
    # Load from JSON
    loaded_scene = Scene.from_json("demo_scene.json")
    print(f"Loaded scene: {loaded_scene}")
    
    # Verify loaded data
    loaded_point = loaded_scene.get_geometry_by_guid(point1.guid)
    print(f"Loaded point name: {loaded_point.name if loaded_point else 'Not found'}")
    
    # Demonstrate removal
    print("\\n=== Removing Geometry ===")
    print(f"Scene before removal: {loaded_scene}")
    removed = loaded_scene.remove_geometry_by_guid(vector2.guid)
    print(f"Removed vector2: {removed}")
    print(f"Scene after removal: {loaded_scene}")


if __name__ == "__main__":
    main()
