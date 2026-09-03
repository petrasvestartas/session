import type { Graph, Vertex, Edge } from './types.ts';
import { normalizeGraph } from './types.ts';

const K_REPULSE = 4000;
const K_SPRING = 0.04;
const K_CENTER = 0.003;
const REST_LENGTH = 130;
const DAMPING = 0.82;
const STOP_THRESHOLD = 0.05;
const MAX_FRAMES = 600;

interface SimNode {
  id: string;
  label: string;
  x: number;
  y: number;
  vx: number;
  vy: number;
  el: SVGCircleElement;
  textEl: SVGTextElement;
}

let rafId = 0;

function svgEl<K extends keyof SVGElementTagNameMap>(
  tag: K,
  attrs: Record<string, string | number>
): SVGElementTagNameMap[K] {
  const el = document.createElementNS('http://www.w3.org/2000/svg', tag);
  for (const [k, v] of Object.entries(attrs)) el.setAttribute(k, String(v));
  return el;
}

function tick(
  nodes: SimNode[],
  edgePairs: [SimNode, SimNode][],
  cx: number,
  cy: number
): number {
  let energy = 0;

  for (let i = 0; i < nodes.length; i++) {
    for (let j = i + 1; j < nodes.length; j++) {
      const dx = nodes[j].x - nodes[i].x;
      const dy = nodes[j].y - nodes[i].y;
      const d2 = Math.max(dx * dx + dy * dy, 1);
      const d = Math.sqrt(d2);
      const f = K_REPULSE / d2;
      const fx = (f * dx) / d;
      const fy = (f * dy) / d;
      nodes[i].vx -= fx;
      nodes[i].vy -= fy;
      nodes[j].vx += fx;
      nodes[j].vy += fy;
    }
  }

  for (const [a, b] of edgePairs) {
    const dx = b.x - a.x;
    const dy = b.y - a.y;
    const d = Math.max(Math.sqrt(dx * dx + dy * dy), 1);
    const f = K_SPRING * (d - REST_LENGTH);
    const fx = (f * dx) / d;
    const fy = (f * dy) / d;
    a.vx += fx;
    a.vy += fy;
    b.vx -= fx;
    b.vy -= fy;
  }

  for (const n of nodes) {
    n.vx += (cx - n.x) * K_CENTER;
    n.vy += (cy - n.y) * K_CENTER;
    n.vx *= DAMPING;
    n.vy *= DAMPING;
    n.x += n.vx;
    n.y += n.vy;
    energy += n.vx * n.vx + n.vy * n.vy;
  }

  return energy;
}

export function renderGraph(svg: SVGSVGElement, graph: Graph): void {
  cancelAnimationFrame(rafId);
  svg.innerHTML = '';

  const { vertices, edges } = normalizeGraph(graph);

  if (vertices.length === 0) {
    const div = document.createElement('div');
    div.className = 'graph-empty';
    div.textContent = 'No graph data';
    svg.parentElement!.appendChild(div);
    return;
  }

  const rect = svg.getBoundingClientRect();
  const W = rect.width || 600;
  const H = rect.height || 500;
  const cx = W / 2;
  const cy = H / 2;
  const R = Math.min(W, H) * 0.32;

  const edgeGroup = svgEl('g', {});
  const nodeGroup = svgEl('g', {});
  svg.appendChild(edgeGroup);
  svg.appendChild(nodeGroup);

  const nodes: SimNode[] = vertices.map((v: Vertex, i: number) => {
    const angle = (2 * Math.PI * i) / vertices.length - Math.PI / 2;
    const x = cx + R * Math.cos(angle);
    const y = cy + R * Math.sin(angle);

    const circle = svgEl('circle', {
      cx: x, cy: y, r: 16,
      fill: '#0f0f0f',
      stroke: '#555',
      'stroke-width': 1,
    });
    const text = svgEl('text', {
      x, y: y + 30,
      'text-anchor': 'middle',
      fill: '#777',
      'font-size': 11,
      'font-family': 'system-ui, sans-serif',
    });
    text.textContent = v.attribute || v.name.slice(0, 16);
    nodeGroup.appendChild(circle);
    nodeGroup.appendChild(text);

    return { id: v.name, label: v.attribute || v.name, x, y, vx: 0, vy: 0, el: circle, textEl: text };
  });

  const nodeMap = new Map(nodes.map((n) => [n.id, n]));

  const edgeLines = edges.map((e: Edge) => {
    const line = svgEl('line', { stroke: '#2a2a2a', 'stroke-width': 1 });
    edgeGroup.appendChild(line);
    return { line, a: nodeMap.get(e.v0), b: nodeMap.get(e.v1) };
  });

  function updateDOM(): void {
    for (const n of nodes) {
      n.el.setAttribute('cx', String(n.x));
      n.el.setAttribute('cy', String(n.y));
      n.textEl.setAttribute('x', String(n.x));
      n.textEl.setAttribute('y', String(n.y + 30));
    }
    for (const { line, a, b } of edgeLines) {
      if (a && b) {
        line.setAttribute('x1', String(a.x));
        line.setAttribute('y1', String(a.y));
        line.setAttribute('x2', String(b.x));
        line.setAttribute('y2', String(b.y));
      }
    }
  }

  const edgePairs: [SimNode, SimNode][] = edgeLines
    .filter(({ a, b }) => a && b)
    .map(({ a, b }) => [a!, b!]);

  let frame = 0;
  function loop(): void {
    const energy = tick(nodes, edgePairs, cx, cy);
    updateDOM();
    if (energy > STOP_THRESHOLD && frame++ < MAX_FRAMES) {
      rafId = requestAnimationFrame(loop);
    }
  }
  rafId = requestAnimationFrame(loop);
}
