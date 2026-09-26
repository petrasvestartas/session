// Code colours for every code block on the site: the Kernel API tests, the course and Install.
// Shiki tokenizes with scope names only; each token is classified into one kind below from its
// TextMate scopes, and unscoped identifiers by their neighbours, so C++, Python, Rust and WGSL
// colour the same thing the same way. The colours live in App.vue (`pre .h?`).

import type { HighlighterCore } from 'shiki/core';

export const THEME_NAME = 'session-light';

/** Tokenizer-only theme: the colours come from the classes, not from this theme. */
export const codeTheme = {
  name: THEME_NAME,
  type: 'light' as const,
  colors: { 'editor.background': '#f6f6f6', 'editor.foreground': '#1f2328' },
  tokenColors: [{ settings: { foreground: '#1f2328' } }],
};

/**
 * Kinds as short classes:
 * hk keyword, ht type or class, hu module, ha attribute or decorator, hf function or method,
 * hx macro, hv variable, hp parameter, hm field, hn number or constant, hs string,
 * hc comment, ho operator or punctuation.
 */
type Kind = 'hk' | 'ht' | 'hu' | 'ha' | 'hf' | 'hx' | 'hv' | 'hp' | 'hm' | 'hn' | 'hs' | 'hc' | 'ho' | '';

const DECLARE = new Set([
  'let', 'var', 'const', 'struct', 'enum', 'fn', 'def', 'class', 'type', 'impl', 'trait', 'union',
  'typedef', 'typename', 'template', 'alias', 'lambda', 'mod', 'auto', 'override', 'namespace',
]);
const IMPORT = new Set(['from', 'import', 'use', 'mod', 'namespace', 'using', 'package', 'as']);
const PASCAL = /^[A-Z][a-z0-9]\w*$|^[A-Z][A-Z0-9]*[a-z]\w*$/;
const CAPS = /^[A-Z][A-Z0-9_]+$/;

const has = (scopes: string[], prefix: string): boolean => scopes.some((s) => s.startsWith(prefix));

/** Kind from the whole scope stack (outer rules) or the innermost scope; null means unscoped. */
function scopeKind(scopes: string[], text: string): Kind | null {
  if (has(scopes, 'comment') || has(scopes, 'punctuation.definition.comment')) return 'hc';
  if (has(scopes, 'string') || has(scopes, 'punctuation.definition.string')) return 'hs';
  if (
    has(scopes, 'meta.function.decorator') ||
    has(scopes, 'meta.decorator') ||
    has(scopes, 'meta.attribute.rust') ||
    has(scopes, 'entity.name.attribute') ||
    has(scopes, 'keyword.operator.attribute')
  )
    return 'ha';
  if (has(scopes, 'entity.name.function.macro') || has(scopes, 'entity.name.function.preprocessor')) return 'hx';
  if (has(scopes, 'meta.preprocessor') || has(scopes, 'keyword.control.directive') || has(scopes, 'punctuation.definition.directive'))
    return 'hk';
  const s = scopes[scopes.length - 1] ?? '';
  const word = /\w/.test(text);
  if (s.startsWith('keyword.operator')) return word ? 'hk' : 'ho';
  if (s.startsWith('keyword')) return 'hk';
  if (s.startsWith('storage.type') || s.startsWith('support.type') || s.startsWith('entity.name.type') || s.startsWith('support.class'))
    return DECLARE.has(text.trim()) || s.startsWith('storage.type.function') || s.startsWith('storage.type.class') ? 'hk' : 'ht';
  if (s.startsWith('entity.name.class') || s.startsWith('entity.other.inherited-class')) return 'ht';
  if (s.startsWith('storage.modifier') || s.startsWith('variable.language')) return word ? 'hk' : 'ho';
  if (s.startsWith('entity.name.function') || s.startsWith('support.function') || s.startsWith('meta.function-call.generic')) return 'hf';
  if (s.startsWith('entity.name.namespace') || s.startsWith('entity.name.scope-resolution') || s.startsWith('entity.name.module')) return 'hu';
  if (s.startsWith('variable.parameter')) return 'hp';
  if (
    s.startsWith('variable.other.property') ||
    s.startsWith('variable.other.member') ||
    s.startsWith('variable.other.field') ||
    s.startsWith('meta.attribute.python') ||
    s.startsWith('entity.name.variable.field')
  )
    return 'hm';
  if (s.startsWith('constant') || s.startsWith('variable.other.constant') || s.startsWith('variable.other.enummember')) return 'hn';
  if (s.startsWith('variable') || s.startsWith('entity.name.variable')) return 'hv';
  if (s.startsWith('punctuation.separator.dot.decimal')) return 'hn';
  if (s.startsWith('punctuation')) return 'ho';
  return null;
}

/** Next and previous non-space text around [i, j) of a line. */
const after = (line: string, j: number): string => line.slice(j).trimStart();
const before = (line: string, i: number): string => line.slice(0, i).trimEnd();

/** Kind of one identifier from its neighbours; `scoped` is the grammar's kind when it had one. */
function wordKind(word: string, line: string, i: number, j: number, scoped: Kind | null): Kind {
  const next = after(line, j);
  const prev = before(line, i);
  const call = next.startsWith('(');
  const member = /(\.|->)$/.test(prev);
  const path = prev.endsWith('::');
  if (scoped === 'hf') {
    if (CAPS.test(word) && call) return 'hx';
    return PASCAL.test(word) && !member ? 'ht' : 'hf';
  }
  const prevWord = (prev.match(/(\w+)$/) || [])[1];
  if (PASCAL.test(word)) return 'ht';
  if (call) return CAPS.test(word) ? 'hx' : 'hf';
  if (CAPS.test(word)) return 'hn';
  if (scoped === 'hu' || (prevWord && IMPORT.has(prevWord) && prev.endsWith(prevWord)) || /^\s*(from|import)\s/.test(line)) return 'hu';
  if (member) return 'hm';
  if (/^:(?!:)/.test(next) && /\bfn\b/.test(line.slice(0, i)) && /[(,]$/.test(prev)) return 'hp';
  if (path) return next.startsWith('::') ? 'hu' : 'ht';
  return 'hv';
}

const esc = (s: string): string => s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');

/** Inner HTML of a code block (no <pre>/<code>), one span per coloured run. */
export function renderCode(hl: HighlighterCore, code: string, lang: string): string {
  const { tokens } = hl.codeToTokens(code, { lang, theme: THEME_NAME, includeExplanation: 'scopeName' });
  const lines: string[] = [];
  for (const row of tokens) {
    const pieces: Array<{ text: string; scopes: string[] }> = [];
    for (const t of row) {
      for (const e of t.explanation ?? []) {
        pieces.push({ text: e.content, scopes: e.scopes.map((x) => x.scopeName) });
      }
    }
    const line = pieces.map((p) => p.text).join('');
    let out = '';
    let cls: Kind = '';
    let buf = '';
    const emit = (k: Kind, text: string) => {
      if (k !== cls) {
        if (buf) out += cls ? `<span class="${cls}">${esc(buf)}</span>` : esc(buf);
        buf = '';
        cls = k;
      }
      buf += text;
    };
    let at = 0;
    for (const p of pieces) {
      const scoped = scopeKind(p.scopes, p.text);
      if (scoped && scoped !== 'hf' && scoped !== 'hv' && scoped !== 'hu') {
        emit(/^\s+$/.test(p.text) ? cls : scoped, p.text);
        at += p.text.length;
        continue;
      }
      const re = /([A-Za-z_]\w*)|(\s+)|([^\w\s]+)|(\d[\w.]*)/g;
      let m: RegExpExecArray | null;
      while ((m = re.exec(p.text)) !== null) {
        const i = at + m.index;
        if (m[1]) emit(wordKind(m[1], line, i, i + m[1].length, scoped), m[1]);
        else if (m[2]) emit(cls, m[2]);
        else if (m[3]) emit('ho', m[3]);
        else emit('hn', m[4]);
      }
      at += p.text.length;
    }
    if (buf) out += cls ? `<span class="${cls}">${esc(buf)}</span>` : esc(buf);
    lines.push(out);
  }
  return lines.join('\n');
}
