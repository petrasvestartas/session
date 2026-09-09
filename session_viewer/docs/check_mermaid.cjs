/** Parse every Mermaid block of the maintained Markdown with the real Mermaid parser; fail on any syntax error. */
const fs = require('node:fs/promises');
const path = require('node:path');
const {chromium} = require('playwright');

const ROOT = path.join(__dirname, '..');
const MERMAID = 'https://cdn.jsdelivr.net/npm/mermaid@11/dist/mermaid.min.js';

async function blocks() {
  const files = [...(await fs.readdir(__dirname)).filter((n) => n.endsWith('.md')).map((n) => path.join(__dirname, n)), path.join(ROOT, 'ARCHITECTURE.md')];
  const out = [];
  for (const file of files.sort()) {
    const text = await fs.readFile(file, 'utf8');
    for (const match of text.matchAll(/```mermaid\n([\s\S]*?)```/g)) {
      out.push({file: path.relative(ROOT, file), line: text.slice(0, match.index).split('\n').length, code: match[1]});
    }
  }
  return out;
}

async function main() {
  const all = await blocks();
  const browser = await chromium.launch({executablePath: process.env.CHROME_BIN || '/usr/bin/google-chrome', headless: true, args: ['--no-sandbox']});
  try {
    const page = await browser.newPage();
    await page.setContent(`<!doctype html><script src="${MERMAID}"></script><body></body>`);
    await page.waitForFunction(() => window.mermaid !== undefined, null, {timeout: 60000});
    const failures = await page.evaluate(async (items) => {
      const bad = [];
      for (const item of items) {
        try { await window.mermaid.parse(item.code); } catch (error) { bad.push(`${item.file}:${item.line}: ${String(error.message || error).split('\n')[0]}`); }
      }
      return bad;
    }, all);
    if (failures.length) { console.error(failures.join('\n')); process.exit(1); }
    console.log(`PASS ${all.length} Mermaid blocks parse`);
  } finally { await browser.close(); }
}

main().catch((error) => { console.error(error); process.exit(1); });
