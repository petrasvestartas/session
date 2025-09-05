#!/usr/bin/env python3
"""Demo script for testing Tree and Graph structures."""

from src.session_py import Tree, TreeNode, Graph, GraphNode, GraphEdge

def test_tree():
    print("=== Tree Structure Test ===")
    
    # Create tree and nodes
    tree = Tree(name="MyTree")
    root = TreeNode(name='root')
    branch1 = TreeNode(name='branch1')
    branch2 = TreeNode(name='branch2')
    leaf1 = TreeNode(name='leaf1')
    leaf2 = TreeNode(name='leaf2')
    leaf3 = TreeNode(name='leaf3')
    
    # Build tree structure
    tree.add(root)
    root.add(branch1)
    root.add(branch2)
    branch1.add(leaf1)
    branch1.add(leaf2)
    branch2.add(leaf3)
    
    print(f"Tree: {tree}")
    print(f"Root: {root}")
    print(f"Root children: {[child.name for child in root.children]}")
    print(f"Branch1 children: {[child.name for child in branch1.children]}")
    print(f"Is leaf1 a leaf?: {leaf1.is_leaf}")
    print(f"Is branch1 a branch?: {branch1.is_branch}")
    
    # Print hierarchy
    print("\\nTree hierarchy:")
    tree.print_hierarchy()
    
    # Find nodes by name
    found = tree.get_node_by_name('leaf1')
    print(f"\\nFound node by name 'leaf1': {found}")
    
    # JSON serialization
    tree.to_json("demo_tree.json")
    print("\\nTree saved to demo_tree.json")
    
    # Load from JSON
    loaded_tree = Tree.from_json("demo_tree.json")
    print(f"Loaded tree: {loaded_tree}")
    print("\\nLoaded tree hierarchy:")
    loaded_tree.print_hierarchy()


def test_graph():
    print("\\n\\n=== Graph Structure Test ===")
    
    # Create graph and nodes
    graph = Graph(name="MyGraph")
    nodeA = GraphNode(name='A')
    nodeB = GraphNode(name='B')
    nodeC = GraphNode(name='C')
    nodeD = GraphNode(name='D')
    
    # Add nodes to graph
    graph.add_node(nodeA)
    graph.add_node(nodeB)
    graph.add_node(nodeC)
    graph.add_node(nodeD)
    
    # Create edges
    edge1 = GraphEdge(nodeA, nodeB, relationship_type='connects_to')
    edge2 = GraphEdge(nodeB, nodeC, relationship_type='leads_to')
    edge3 = GraphEdge(nodeA, nodeC, relationship_type='shortcut_to')
    edge4 = GraphEdge(nodeC, nodeD, relationship_type='flows_to')
    
    # Add edges to graph
    graph.add_edge(edge1)
    graph.add_edge(edge2)
    graph.add_edge(edge3)
    graph.add_edge(edge4)
    
    print(f"Graph: {graph}")
    print(f"Node A neighbors: {[n.name for n in nodeA.neighbors]}")
    print(f"Node B neighbors: {[n.name for n in nodeB.neighbors]}")
    print(f"Node C neighbors: {[n.name for n in nodeC.neighbors]}")
    print(f"Node A degree: {nodeA.degree}")
    print(f"Node C degree: {nodeC.degree}")
    
    # Test connections
    print(f"\\nA connected to B?: {nodeA.is_connected_to(nodeB)}")
    print(f"A connected to D?: {nodeA.is_connected_to(nodeD)}")
    
    # Test edge queries
    edges_from_A = nodeA.get_edges_to(nodeC)
    print(f"Edges from A to C: {[e.relationship_type for e in edges_from_A]}")
    
    # JSON serialization
    graph.to_json("demo_graph.json")
    print("\\nGraph saved to demo_graph.json")
    
    # Load from JSON
    loaded_graph = Graph.from_json("demo_graph.json")
    print(f"Loaded graph: {loaded_graph}")
    
    # Test loaded graph
    loaded_nodeA = loaded_graph.get_node_by_name('A')
    if loaded_nodeA:
        print(f"Loaded node A neighbors: {[n.name for n in loaded_nodeA.neighbors]}")


if __name__ == "__main__":
    test_tree()
    test_graph()
