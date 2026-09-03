export interface TreeNode {
  type: 'TreeNode';
  guid: string;
  name: string;
  color?: [number, number, number, number] | null;
  children: TreeNode[];
}

export interface Tree {
  type: 'Tree';
  guid: string;
  name: string;
  root: TreeNode;
}

export interface Vertex {
  type: 'Vertex';
  guid: string;
  name: string;
  attribute: string;
  index: number;
}

export interface Edge {
  type: 'Edge';
  guid: string;
  name: string;
  v0: string;
  v1: string;
  attribute: string;
  index: number;
}

export interface Graph {
  type: 'Graph';
  guid: string;
  name: string;
  vertex_count: number;
  edge_count: number;
  vertices: Vertex[] | Record<string, Vertex>;
  edges: Edge[] | Record<string, Edge> | Record<string, Record<string, Edge>>;
}

export interface Session {
  type: 'Session';
  guid: string;
  name: string;
  tree: Tree;
  graph: Graph;
}

export interface NormGraph {
  vertices: Vertex[];
  edges: Edge[];
}

export function normalizeGraph(graph: Graph): NormGraph {
  let vertices: Vertex[];
  if (Array.isArray(graph.vertices)) {
    vertices = graph.vertices;
  } else {
    vertices = Object.values(graph.vertices);
  }

  let edges: Edge[];
  if (Array.isArray(graph.edges)) {
    edges = graph.edges;
  } else {
    edges = [];
    for (const val of Object.values(graph.edges)) {
      if (val && typeof val === 'object' && 'type' in val) {
        edges.push(val as Edge);
      } else {
        for (const inner of Object.values(val as Record<string, Edge>)) {
          edges.push(inner);
        }
      }
    }
  }

  return { vertices, edges };
}
