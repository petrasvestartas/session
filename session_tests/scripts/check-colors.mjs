// Checks that the same identifier gets the same colour kind in C++, Python and Rust: runs the site
// classifier (src/codeTheme.ts) over every kernel test in testData.js, then for each identifier
// seen in two or more languages compares its most common kind per language. Prints the agreement
// and the top mismatches; a report, not a gate (exit 0).
//
//   node scripts/check-colors.mjs [testData.js] [--top N] [--word name[:kind]]
//
// --word prints up to 40 lines where that identifier occurs (optionally only as that kind), the
// identifier marked [kind name]; a comma list (--word Mesh,mesh,self) prints each one's kinds.
//
// Pairing rules:
//   - paired names compare as one: this = self, nullptr = None, true = True, false = False;
//   - Python dunders (`__str__`) are skipped: their bare twin (`str`) is a different construct;
//   - a Rust macro counts as `name!`, so `vec!` is not compared with a variable `vec`;
//   - words that are a keyword in only some of the languages are skipped (`from`, `in`, `new`,
//     `match`, `loop`, `fn`, `type`, `mod`, `ref`, `move`, `is`, `not`, `and`, `or`), since in the
//     others they are ordinary names;
//   - words in strings and comments are text, not code, and are not counted.
//
// Kernel imports are checked on their own: `using session_cpp::Mesh;` (the leading commented C++
// lines, uncommented as the site shows them), `from session_py import Mesh` and `use crate::Mesh;`
// must colour the statement keyword, the module path and the imported name the same way (a C++
// class in the path, `session_cpp::Intersection::f`, stays a type).
//
// What stays different is a different construct (about 45 names, 2026-09):
//   - a Python property is a field, the C++/Rust accessor a call: `.guid` vs `.guid()`, x_axis;
//   - Python keyword arguments are parameters (`Plane(x_axis=...)`), Rust struct literals fields;
//   - one name, two things: Rust modules vs C++ locals (intersection, element, brep, proto),
//     C++ std types vs locals (pair, array, map, set) and pair fields (first, second), Python
//     builtins called as functions (range, float, list), a Python `def` helper vs a C++ lambda;
//   - a library method in one language only: `.at()`, `.end()`, `.value()`, `.keys()`, `.cloned()`.
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { createHighlighter } from 'shiki';
import { transform } from 'esbuild';

const here = path.dirname(fileURLToPath(import.meta.url));
const args = process.argv.slice(2);
const opt = (name, fallback) => {
  const i = args.indexOf(name);
  return i >= 0 ? args.splice(i, 2)[1] : fallback;
};
const top = Number(opt('--top', 40));
const wordArg = opt('--word', '');
const [only, onlyKind] = wordArg.includes(',') ? ['', ''] : wordArg.split(':');
const summary = wordArg.includes(',') ? wordArg.split(',') : [];
const dataFile = args[0] || path.join(here, '..', 'testData.js');

// codeTheme.ts has only type imports, so the stripped module loads from a data URL.
const ts = fs.readFileSync(path.join(here, '..', 'src', 'codeTheme.ts'), 'utf8');
const { code: js } = await transform(ts, { loader: 'ts', format: 'esm' });
const { codeTheme, renderCode, uncommentUsing } = await import('data:text/javascript,' + encodeURIComponent(js));

const raw = fs.readFileSync(dataFile, 'utf8');
const data = JSON.parse(raw.slice(raw.indexOf('{', raw.indexOf('TEST_DATA')), raw.lastIndexOf('}') + 1));
const LANGS = { cpp: 'cpp', python: 'python', rust: 'rust' };
const ALIAS = { this: 'self', nullptr: 'None', True: 'true', False: 'false' };
const PARTIAL_KEYWORDS = new Set([
  'from', 'in', 'new', 'match', 'loop', 'fn', 'type', 'mod', 'ref', 'move', 'is', 'not', 'and', 'or',
  'delete', 'where', 'with', 'pass', 'as', 'use', 'impl', 'crate', 'super', 'def', 'dyn', 'mut',
]);

const hl = await createHighlighter({ themes: [codeTheme], langs: Object.values(LANGS) });

const unesc = (s) => s.replace(/&lt;/g, '<').replace(/&gt;/g, '>').replace(/&amp;/g, '&');

/** Runs of one rendered line as [text, kind] ('' = uncoloured). */
function runs(html) {
  const out = [];
  const re = /<span class="(\w+)">([^<]*)<\/span>|([^<]+)/g;
  let m;
  while ((m = re.exec(html)) !== null) out.push([unesc(m[2] ?? m[3]), m[1] || '']);
  return out;
}

/** Identifiers of one rendered line as [word, kind, start, end]; Rust macros keep their `!`. */
function words(html, lang) {
  const out = [];
  const rs = runs(html);
  const line = rs.map((r) => r[0]).join('');
  let at = 0;
  for (const [text, kind] of rs) {
    if (kind !== 'hs' && kind !== 'hc') {
      for (const m of text.matchAll(/(?<!\w)[A-Za-z_]\w*/g)) {
        let w = m[0];
        const end = at + m.index + w.length;
        if (lang === 'rust' && kind === 'hx' && line[end] === '!') w += '!';
        out.push([w, kind || '-', at + m.index, end]);
      }
    }
    at += text.length;
  }
  return { line, out };
}

// counts[word][lang][kind] = n
const counts = new Map();
// Kernel import lines: statement keywords, module path segments, imported names (the name compares
// across languages, keyword and path kinds pool per language).
const KERNEL_IMPORT = /^\s*(using\s+session_cpp::|use\s+crate::|from\s+session_py\b)/;
const imports = { lines: {}, keyword: {}, path: {}, names: new Map() };
const tally = (obj, lang, kind) => {
  obj[lang] ??= {};
  obj[lang][kind] = (obj[lang][kind] || 0) + 1;
};
/** One kernel import line: `from` / `import` / `use` / `using` keywords, then path, then names. */
function importLine(out, lang) {
  tally(imports.lines, lang, 'n');
  const at = lang === 'python' ? out.findIndex(([w]) => w === 'import') : out.length - 1;
  out.forEach(([w, k], n) => {
    if (n === 0 || (lang === 'python' && n === at)) tally(imports.keyword, lang, k);
    else if (n < at) tally(imports.path, lang, k);
    else {
      if (!imports.names.has(w)) imports.names.set(w, {});
      tally(imports.names.get(w), lang, k);
    }
  });
}
const samples = [];
let blocks = 0;
for (const [key, tests] of Object.entries(data)) {
  const m = key.match(/_(cpp|python|rust)$/);
  if (!m || !Array.isArray(tests)) continue;
  const lang = m[1];
  for (const t of tests) {
    if (!t.code) continue;
    blocks++;
    const code = lang === 'cpp' ? uncommentUsing(t.code) : t.code;
    for (const html of renderCode(hl, code, LANGS[lang]).split('\n')) {
      const { line, out } = words(html, lang);
      if (KERNEL_IMPORT.test(line)) importLine(out, lang);
      let marked = '';
      let last = 0;
      for (const [raw, k, i, j] of out) {
        const w = ALIAS[raw] ?? raw;
        if (!counts.has(w)) counts.set(w, {});
        const byLang = counts.get(w);
        byLang[lang] ??= {};
        byLang[lang][k] = (byLang[lang][k] || 0) + 1;
        if (only && raw === only && (!onlyKind || onlyKind === k)) {
          marked += line.slice(last, i) + `[${k} ${raw}]`;
          last = j;
        }
      }
      if (marked && samples.length < 40) samples.push(`${lang.padEnd(6)} ${(marked + line.slice(last)).trim()}`);
    }
  }
}

const dominant = (kinds) => Object.entries(kinds).sort((a, b) => b[1] - a[1])[0][0];
const total = (kinds) => Object.values(kinds).reduce((a, b) => a + b, 0);
const fmt = (kinds) =>
  Object.entries(kinds)
    .sort((a, b) => b[1] - a[1])
    .map(([k, n]) => `${k}:${n}`)
    .join(' ');
const cols = (byLang) =>
  ['cpp', 'python', 'rust']
    .map((l) => (byLang[l] ? `${l}[${fmt(byLang[l])}]` : ''))
    .filter(Boolean)
    .join('  ');

let shared = 0;
let agree = 0;
let occ = 0;
let occAgree = 0;
const mismatches = [];
for (const [w, byLang] of counts) {
  const langs = Object.keys(byLang);
  if (langs.length < 2 || /^__\w+__$/.test(w) || PARTIAL_KEYWORDS.has(w)) continue;
  shared++;
  const doms = langs.map((l) => dominant(byLang[l]));
  const pooled = {};
  for (const l of langs) for (const [k, n] of Object.entries(byLang[l])) pooled[k] = (pooled[k] || 0) + n;
  occ += total(pooled);
  occAgree += pooled[dominant(pooled)];
  if (new Set(doms).size === 1) agree++;
  else mismatches.push({ w, n: total(pooled), byLang });
}

if (summary.length) {
  for (const w of summary) console.log(`${w.padEnd(16)} ${counts.has(ALIAS[w] ?? w) ? cols(counts.get(ALIAS[w] ?? w)) : '-'}`);
  process.exit(0);
}
if (only) {
  console.log(samples.join('\n'));
  if (counts.has(only)) console.log(`\n${only}  ${cols(counts.get(only))}`);
  process.exit(0);
}
mismatches.sort((a, b) => b.n - a.n);
const pct = (a, b) => ((100 * a) / b).toFixed(1) + '%';
console.log(`${blocks} test blocks, ${counts.size} identifiers, ${shared} compared (in 2+ languages)`);
console.log(`identifiers whose main kind agrees in every language: ${agree}/${shared} = ${pct(agree, shared)}`);
console.log(`occurrences with their identifier's consensus kind:   ${occAgree}/${occ} = ${pct(occAgree, occ)}`);
console.log(`\ntop ${Math.min(top, mismatches.length)} of ${mismatches.length} mismatches (kind:count per language):`);
for (const { w, n, byLang } of mismatches.slice(0, top)) console.log(`  ${w.padEnd(24)} ${String(n).padStart(5)}  ${cols(byLang)}`);

const byName = [...imports.names].filter(([, b]) => Object.keys(b).length > 1);
const importMiss = byName.filter(([, b]) => new Set(Object.values(b).map(dominant)).size > 1);
const perLang = (obj) => ['cpp', 'python', 'rust'].map((l) => `${l}[${obj[l] ? fmt(obj[l]) : '-'}]`).join('  ');
console.log(`\nkernel import lines: ${perLang(imports.lines)}`);
console.log(`  keywords          ${perLang(imports.keyword)}`);
console.log(`  module path       ${perLang(imports.path)}`);
console.log(`  imported names agreeing in every language: ${byName.length - importMiss.length}/${byName.length}`);
for (const [w, b] of importMiss.slice(0, top)) console.log(`    ${w.padEnd(22)} ${cols(b)}`);
