import { renderTree } from './tree-panel.ts';
import { renderGraph } from './graph-panel.ts';
import type { Session, Graph } from './types.ts';

const sessionNameEl = document.getElementById('session-name')!;
const treeRoot = document.getElementById('tree-root')!;
const graphSvg = document.getElementById('graph-svg') as unknown as SVGSVGElement;
const graphPanel = document.getElementById('graph-panel')!;

let lastGraph: Graph | null = null;

function loadSession(json: string): void {
  const session = JSON.parse(json) as Session;
  sessionNameEl.textContent = session.name;
  lastGraph = session.graph;
  renderTree(treeRoot, session.tree.root);
  renderGraph(graphSvg, session.graph);
}

new ResizeObserver(() => {
  if (lastGraph) renderGraph(graphSvg, lastGraph);
}).observe(graphPanel);

fetch('./session.json')
  .then((r) => r.text())
  .then(loadSession)
  .catch(() => { sessionNameEl.textContent = 'no session loaded'; });

document.getElementById('file-input')!.addEventListener('change', (e) => {
  const file = (e.target as HTMLInputElement).files?.[0];
  if (!file) return;
  const reader = new FileReader();
  reader.onload = (ev) => loadSession(ev.target!.result as string);
  reader.readAsText(file);
  (e.target as HTMLInputElement).value = '';
});

setupDivider();

function setupDivider(): void {
  const divider = document.getElementById('divider')!;
  const treePanel = document.getElementById('tree-panel')!;
  let dragging = false;
  let startX = 0;
  let startW = 0;

  divider.addEventListener('mousedown', (e) => {
    dragging = true;
    startX = e.clientX;
    startW = treePanel.getBoundingClientRect().width;
    divider.classList.add('dragging');
    e.preventDefault();
  });

  window.addEventListener('mousemove', (e) => {
    if (!dragging) return;
    const newW = Math.max(100, Math.min(startW + e.clientX - startX, window.innerWidth * 0.6));
    document.documentElement.style.setProperty('--tree-width', `${newW}px`);
  });

  window.addEventListener('mouseup', () => {
    dragging = false;
    divider.classList.remove('dragging');
  });
}
