(() => {
  const viewer = new URL('../../../', document.currentScript.src);
  if (viewer.port === '8772') viewer.port = '8770';
  const link = document.createElement('a');
  link.id = 'docs-viewer';
  link.href = viewer.href;
  link.title = 'Back to the viewer';
  link.setAttribute('aria-label', 'Back to the viewer');
  document.body.appendChild(link);
})();
