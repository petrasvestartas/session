import assert from 'node:assert/strict';
import fs from 'node:fs';
import test from 'node:test';
import { Marked } from 'marked';
import { svgHtml } from './svg.ts';

const md = new Marked({ gfm: true, tokenizer: { html: svgHtml } });
const svg = '<svg viewBox="0 0 20 20">\n  <rect width="20" height="20"/>\n\n    <text>Body</text>\n</svg>';

test('SVG and its immediate SVG-only wrapper retain blank lines and following Markdown', () => {
  for (const html of [svg, `<div class="webgpu-schematic">\n${svg}\n</div>`]) {
    assert.equal(md.parse(`${html}\n\n## Next\n\n**Prose**`), `${html}<h2>Next</h2>\n<p><strong>Prose</strong></p>\n`);
  }
});

test('fenced and indented SVG examples remain escaped code', () => {
  for (const source of [`\`\`\`html\n${svg}\n\`\`\``, `~~~html\n<div>\n${svg}\n</div>\n~~~`, svg.split('\n').map(line => `    ${line}`).join('\n')]) {
    const html = md.parse(source) as string;
    assert.match(html, /<pre><code/);
    assert.match(html, /&lt;svg/);
    assert.doesNotMatch(html, /<svg/);
  }
});

test('nested SVG, comments, CDATA and quoted tags do not end the outer SVG', () => {
  const html = '<svg>\n<!-- </svg> -->\n<![CDATA[</svg>]]>\n<g data-label="</svg>">\n<svg><path/></svg>\n\n    <text>Last</text>\n</g>\n</svg>';
  assert.equal(md.parse(html), html);
});

test('unclosed SVG and wrappers with other content keep normal Markdown handling', () => {
  for (const source of ['<svg>\n\nbody', `<div>\n${svg}\n\nOther content\n</div>`]) {
    assert.equal(svgHtml(source), false);
    assert.equal(md.parse(source), new Marked({ gfm: true }).parse(source));
  }
});

test('the authored opening schematic is one unchanged HTML block', () => {
  const page = fs.readFileSync(new URL('../../session_viewer/docs/journey/consolidation/01-canvas.md', import.meta.url), 'utf8').replaceAll('\r\n', '\n');
  const schematic = page.match(/<div class="webgpu-schematic"[\s\S]*?<\/svg>\s*<\/div>/)![0];
  assert.equal(md.parse(schematic), schematic);
  assert.doesNotMatch(md.parse(schematic) as string, /<pre>|<code>/);
});
