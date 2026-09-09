/** Measure every label of every illustration in a real browser; fail on any overflow.
 *
 * A <text data-box="k"> must lie inside the <rect data-box="k"> with 4 px slack; every text
 * must lie inside the viewBox; no two texts may overlap. With --write, each text receives a
 * `textLength` equal to its measured width, so a machine with a different system font
 * squeezes or stretches glyphs instead of overflowing.
 */
const assert = require('node:assert/strict');
const fs = require('node:fs/promises');
const path = require('node:path');
const {chromium} = require('playwright');

const DIR = path.join(__dirname, 'illustrations');

function measure() {
  const svg = document.querySelector('svg');
  const view = svg.viewBox.baseVal;
  const boxes = {};
  for (const rect of svg.querySelectorAll('rect[data-box]')) {
    const b = rect.getBBox();
    boxes[rect.dataset.box] = {x: b.x, y: b.y, width: b.width, height: b.height};
  }
  const texts = [];
  for (const text of svg.querySelectorAll('text')) {
    const b = text.getBBox();
    texts.push({content: text.textContent, box: text.dataset.box ?? null, x: b.x, y: b.y, w: b.width, h: b.height,
                length: text.getComputedTextLength()});
  }
  return {view: {w: view.width, h: view.height}, boxes, texts};
}

function problems(report) {
  const out = [];
  const slack = 4;
  for (const t of report.texts) {
    if (t.x < 0 || t.y < 0 || t.x + t.w > report.view.w || t.y + t.h > report.view.h) {
      out.push(`outside viewBox: "${t.content}"`);
    }
    if (t.box !== null) {
      const b = report.boxes[t.box];
      if (!b) { out.push(`unknown box ${t.box} for "${t.content}"`); continue; }
      if (t.x < b.x + slack || t.x + t.w > b.x + b.width - slack || t.y < b.y || t.y + t.h > b.y + b.height) {
        out.push(`overflows its box: "${t.content}" (text ${t.x.toFixed(0)}..${(t.x + t.w).toFixed(0)}, box ${b.x.toFixed(0)}..${(b.x + b.width).toFixed(0)})`);
      }
    }
  }
  // A label may not sit on a box it does not belong to (arrow labels squeezed between boxes).
  for (const t of report.texts) {
    if (!t.w) continue;
    for (const [id, b] of Object.entries(report.boxes)) {
      if (t.box === id) continue;
      const dx = Math.min(t.x + t.w, b.x + b.width) - Math.max(t.x, b.x);
      const dy = Math.min(t.y + t.h, b.y + b.height) - Math.max(t.y, b.y);
      if (dx > 2 && dy > 2) out.push(`label crosses another box: "${t.content}" and box ${id}`);
    }
  }
  for (let i = 0; i < report.texts.length; i++) {
    for (let j = i + 1; j < report.texts.length; j++) {
      const a = report.texts[i], b = report.texts[j];
      // Line boxes of stacked labels may touch by their descender/ascender allowance; require
      // a real intrusion of more than 2 px on both axes.
      const dx = Math.min(a.x + a.w, b.x + b.w) - Math.max(a.x, b.x);
      const dy = Math.min(a.y + a.h, b.y + b.h) - Math.max(a.y, b.y);
      const overlap = a.w > 0 && b.w > 0 && dx > 2 && dy > 2;
      if (overlap) out.push(`labels overlap: "${a.content}" and "${b.content}"`);
    }
  }
  return out;
}

async function main() {
  const write = process.argv.includes('--write');
  const browser = await chromium.launch({executablePath: process.env.CHROME_BIN || '/usr/bin/google-chrome',
                                          headless: process.env.VIEWER_HEADLESS !== '0',
                                          args: ['--no-sandbox', ...JSON.parse(process.env.VIEWER_CHROME_ARGS || '[]')]});
  const failures = [];
  try {
    const page = await (await browser.newContext({viewport: {width: 1200, height: 900}})).newPage();
    for (const name of (await fs.readdir(DIR)).filter((n) => n.endsWith('.svg')).sort()) {
      const file = path.join(DIR, name);
      let source = await fs.readFile(file, 'utf8');
      if (write) source = source.replace(/ textLength="[^"]*" lengthAdjust="spacingAndGlyphs"/g, '');
      await page.setContent(`<!doctype html><body style="margin:0">${source}</body>`);
      const report = await page.evaluate(measure);
      const bad = problems(report);
      if (bad.length) failures.push(`${name}\n  ${bad.join('\n  ')}`);
      if (write) {
        let index = 0;
        source = source.replace(/<text([^>]*)>/g, (match, attrs) => {
          const length = report.texts[index++].length;
          return `<text${attrs} textLength="${length.toFixed(1)}" lengthAdjust="spacingAndGlyphs">`;
        });
        await fs.writeFile(file, source);
      }
      console.log(`${name}: ${report.texts.length} labels, ${bad.length} problems`);
    }
  } finally { await browser.close(); }
  if (failures.length) { console.error(failures.join('\n')); process.exit(1); }
  console.log('PASS: every label sits inside its box and the viewBox, no overlaps');
}

main().catch((error) => { console.error(error); process.exit(1); });
