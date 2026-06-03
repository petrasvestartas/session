import type { TreeNode } from './types.ts';

export function renderTree(container: HTMLElement, root: TreeNode): void {
  container.innerHTML = '';
  container.appendChild(buildNode(root, 0));
}

function buildNode(node: TreeNode, depth: number): HTMLElement {
  const wrapper = document.createElement('div');
  wrapper.className = 'tree-node';

  const row = document.createElement('div');
  row.className = 'tree-row';
  row.style.paddingLeft = `${depth * 16 + 6}px`;

  const hasChildren = node.children && node.children.length > 0;

  const toggle = document.createElement('span');
  toggle.className = 'tree-toggle' + (hasChildren ? '' : ' leaf');
  toggle.textContent = hasChildren ? (depth === 0 ? '▼' : '▶') : '·';
  row.appendChild(toggle);

  if (node.color) {
    const [r, g, b, a] = node.color;
    const swatch = document.createElement('span');
    swatch.className = 'tree-swatch';
    swatch.style.background = `rgba(${r},${g},${b},${a / 255})`;
    row.appendChild(swatch);
  }

  const label = document.createElement('span');
  label.className = 'tree-label';
  label.textContent = node.name;
  row.appendChild(label);

  wrapper.appendChild(row);

  if (hasChildren) {
    const childContainer = document.createElement('div');
    childContainer.className = 'tree-children';

    if (depth > 0) {
      childContainer.style.display = 'none';
    }

    for (const child of node.children) {
      childContainer.appendChild(buildNode(child, depth + 1));
    }
    wrapper.appendChild(childContainer);

    toggle.addEventListener('click', (e) => {
      e.stopPropagation();
      const collapsed = childContainer.style.display === 'none';
      childContainer.style.display = collapsed ? '' : 'none';
      toggle.textContent = collapsed ? '▼' : '▶';
    });
  }

  return wrapper;
}
