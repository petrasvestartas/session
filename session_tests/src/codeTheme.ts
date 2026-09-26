// Code colours for every code block on the site: the Kernel API tests, the course and Install.
// Shiki tokenizes with scope names only; each token is classified into one kind below from its
// TextMate scopes, and identifiers by their neighbours and by what the whole block says of them, so
// C++, Python, Rust and WGSL colour the same thing the same way (scripts/check-colors.mjs measures
// it). The colours live in App.vue (`pre .h?`).

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
const SELF = new Set(['self', 'this', 'Self']);
const CONSTANTS = new Set(['None', 'nullptr', 'NULL', 'true', 'false', 'True', 'False']);
const IMPORT_LINE = /^\s*(pub\s+)?(from|import|use|using|mod)\b/;
/** The standard `_t` types; any other `x_t` the C++ grammar calls a type is a user's name. */
const STD_T = /^(u?int(8|16|32|64|ptr|max)?|u?int_(fast|least)\d+|size|ssize|ptrdiff|wchar|char(8|16|32)|max_align|nullptr|time|clock|off)_t$/;
const CPP_TYPES = new Set(['auto', 'bool', 'char', 'double', 'float', 'int', 'long', 'short', 'unsigned', 'size_t', 'string']);

const has = (scopes: string[], prefix: string): boolean => scopes.some((s) => s.startsWith(prefix));

/** Kind from the whole scope stack (outer rules) or the innermost scope; null means unscoped. */
function scopeKind(scopes: string[], text: string): Kind | null {
  if (has(scopes, 'comment') || has(scopes, 'punctuation.definition.comment')) return 'hc';
  if (has(scopes, 'string') || has(scopes, 'punctuation.definition.string')) return 'hs';
  if (has(scopes, 'constant.numeric')) return 'hn';
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
  if (SELF.has(text)) return 'hk';
  if (CONSTANTS.has(text)) return 'hn';
  if (s.startsWith('keyword.other.unit')) return 'hn';
  if (s.startsWith('keyword.other.crate')) return 'hu';
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

/**
 * Whether a grammar-scoped word is re-judged by its neighbours: calls, names, members and modules
 * always; an ALL_CAPS word (AABB is a type, CAPACITY a constant); a C++ "parameter type" (the
 * grammar reads `Plane pl0(origin, x)` as a function declaration); a Python builtin type (`str` is
 * a type in `list[str]` but a function in `str(mesh)`).
 */
function rejudge(scoped: Kind | null, inner: string, text: string): boolean {
  if (scoped === null || scoped === 'hf' || scoped === 'hv' || scoped === 'hu' || scoped === 'hm') return true;
  return (
    (scoped === 'ht' && CAPS.test(text)) ||
    inner.startsWith('constant.other.caps') ||
    inner.startsWith('entity.name.type.parameter') ||
    inner.startsWith('support.type.python')
  );
}

/** What a whole block tells about its names, so every use of a name gets one kind. */
interface Block {
  lang: string;
  types: Set<string>;
  modules: Set<string>;
  calls: Set<string>;
}

/**
 * ALL_CAPS types by their use (`AABB::empty`, `OBB box`, `AABB.from_points(`, while
 * `TOLERANCE.is_close(` stays a constant), Python modules (imported, then `name.`) and called names.
 */
function scanBlock(code: string, lang: string): Block {
  const types = new Set<string>();
  const modules = new Set<string>();
  for (const w of new Set(code.match(/\b[A-Z][A-Z0-9_]+\b/g) ?? [])) {
    const used = [
      `\\b${w}::\\w`,
      `<\\s*${w}\\s*[>,]`,
      `,\\s*${w}\\s*>`,
      `->\\s*${w}\\b`,
      `\\b${w}\\.from_\\w*\\s*\\(`,
      lang === 'cpp' ? `^[ \\t]*(?:const\\s+)?${w}\\s*[&*]*\\s+[a-z_]\\w*\\s*[;=({]` : `\\b${w}\\s*\\(`,
    ];
    if (used.some((re) => new RegExp(re, 'm').test(code))) types.add(w);
  }
  if (lang === 'python') {
    for (const m of code.matchAll(/^\s*(?:from\s+\S+\s+)?import\s+([^\n#]+)/gm)) {
      for (const part of m[1].replace(/[()]/g, '').split(',')) {
        const name = part.trim().split(/\s+as\s+/).pop()!.split('.')[0].trim();
        if (/^[a-z_]\w*$/.test(name) && new RegExp(`\\b${name}\\.\\w`).test(code)) modules.add(name);
      }
    }
  }
  const calls = new Set([...code.matchAll(/(?<![\w.])([a-z_]\w*)\s*\(/g)].map((m) => m[1]));
  return { lang, types, modules, calls };
}

/** Whether the text before a C++ `name(` ends in a type: `Point`, `double`, `vector<Point>&`. */
function cppHead(prev: string, block: Block): boolean {
  const m = prev.match(/(\w+|>)\s*[&*]*$/);
  if (!m) return false;
  if (m[1] === '>') return /<[\w\s:,<>*]*>\s*[&*]*$/.test(prev);
  return PASCAL.test(m[1]) || block.types.has(m[1]) || CPP_TYPES.has(m[1]) || /^u?int\d+_t$/.test(m[1]);
}

/** A C++ `Type name(args)` whose parenthesis at `open` closes into `;` or `,` declares a variable. */
function cppDeclares(code: string, open: number): boolean {
  let depth = 0;
  for (let k = open; k < code.length; k++) {
    const c = code[k];
    if (c === '"' || c === "'") {
      for (k++; k < code.length && code[k] !== c; k++) if (code[k] === '\\') k++;
    } else if (c === '(') depth++;
    else if (c === ')' && --depth === 0) return /^\s*[;,]/.test(code.slice(k + 1));
  }
  return false;
}

/** Next and previous non-space text around [i, j) of a line. */
const after = (line: string, j: number): string => line.slice(j).trimStart();
const before = (line: string, i: number): string => line.slice(0, i).trimEnd();

/**
 * Kind of one identifier from its neighbours; `scoped` is the grammar's kind when it had one and
 * `at` the offset of the line in `code`.
 */
function wordKind(word: string, line: string, i: number, j: number, scoped: Kind | null, block: Block, code: string, at: number): Kind {
  const next = after(line, j);
  const prev = before(line, i);
  const call = /^(::)?(<[\w\s:,<>&*]*>\s*)?\(/.test(next);
  const member = /(^|[^.])\.$|->$/.test(prev);
  const path = prev.endsWith('::');
  const scope = /^::(?!<)/.test(next);
  if (CONSTANTS.has(word)) return 'hn';
  if (block.types.has(word)) return 'ht';
  if (PASCAL.test(word)) return call && member ? 'hf' : 'ht';
  if (CAPS.test(word)) {
    // A C++ macro starts a statement; `AABB(...)` inside an expression constructs a value.
    if (call) return block.lang === 'cpp' && !/(^|[(,={]|\breturn)$/.test(prev) ? 'hx' : 'ht';
    return scope ? 'ht' : 'hn';
  }
  if (scoped === 'ht' && !call) return 'ht';
  const prevWord = (prev.match(/(\w+)$/) || [])[1];
  if (IMPORT_LINE.test(line) || (prevWord && IMPORT.has(prevWord))) return block.calls.has(word) ? 'hf' : 'hu';
  if (scoped === 'hu' || scope || (block.modules.has(word) && next.startsWith('.') && !member)) return 'hu';
  if (call) return block.lang === 'cpp' && !member && !path && cppHead(prev, block) && cppDeclares(code, at + line.length - next.length) ? 'hv' : 'hf';
  if (scoped === 'hf') return 'hf';
  if (member) return 'hm';
  if (/^:(?!:)/.test(next) && (block.lang === 'rust' || block.lang === 'wgsl') && !/\b(let|var|const|static|mut)$/.test(prev)) {
    // `name: T` is a parameter in a fn signature (after `(`, `,` or an `@location(0)`), else a field.
    return /\bfn\b/.test(line.slice(0, i)) && /([(,]|@\w+(\([^)]*\))?)$/.test(prev) ? 'hp' : 'hm';
  }
  if (path) return 'ht';
  if (scoped === 'hm') return 'hm';
  return 'hv';
}

const esc = (s: string): string => s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');

/** Inner HTML of a code block (no <pre>/<code>), one span per coloured run. */
export function renderCode(hl: HighlighterCore, code: string, lang: string): string {
  const { tokens } = hl.codeToTokens(code, { lang, theme: THEME_NAME, includeExplanation: 'scopeName' });
  const block = scanBlock(code, lang);
  const lines: string[] = [];
  let lineAt = 0;
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
      const inner = p.scopes[p.scopes.length - 1] ?? '';
      let scoped = scopeKind(p.scopes, p.text);
      if (
        inner.startsWith('entity.name.type.parameter') ||
        inner.startsWith('support.function.builtin') ||
        (inner.includes('posix-reserved') && !STD_T.test(p.text))
      )
        scoped = null;
      if (!rejudge(scoped, inner, p.text)) {
        emit(/^\s+$/.test(p.text) ? cls : scoped, p.text);
        at += p.text.length;
        continue;
      }
      const re = /([A-Za-z_]\w*)|(\s+)|([^\w\s]+)|(\d[\w.]*)/g;
      let m: RegExpExecArray | null;
      while ((m = re.exec(p.text)) !== null) {
        const i = at + m.index;
        if (m[1]) emit(wordKind(m[1], line, i, i + m[1].length, scoped, block, code, lineAt), m[1]);
        else if (m[2]) emit(cls, m[2]);
        else if (m[3]) emit('ho', m[3]);
        else emit('hn', m[4]);
      }
      at += p.text.length;
    }
    if (buf) out += cls ? `<span class="${cls}">${esc(buf)}</span>` : esc(buf);
    lines.push(out);
    lineAt += line.length + 1;
  }
  return lines.join('\n');
}
