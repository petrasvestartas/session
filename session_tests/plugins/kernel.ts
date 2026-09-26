// Kernel classes for the docs site, read from the C++ kernel (the ground truth): one entry per
// session_cpp/src/<name>_test.cpp with its MINI_TEST group, test names and a one-line description
// taken from the class docstring in <name>.h (Rust module docs as the fallback).
import fs from 'node:fs';
import path from 'node:path';

export interface KernelClass {
  suite: string;
  label: string;
  description: string;
  tests: string[];
}

const norm = (s: string) => s.toLowerCase().replace(/[^a-z0-9]/g, '');

/** First sentence of a docstring block. */
function firstSentence(lines: string[]): string {
  const text = lines.join(' ').replace(/\s+/g, ' ').trim();
  const m = text.match(/^(.+?[.!?])(\s|$)/);
  return (m ? m[1] : text).replace(/`/g, '');
}

/** The `///` lines right above line `at`, skipping template and attribute lines. */
function docAbove(lines: string[], at: number): string[] {
  const out: string[] = [];
  let i = at - 1;
  while (i >= 0 && /^\s*(template\s*<|#\[)/.test(lines[i])) i--;
  for (; i >= 0; i--) {
    const m = lines[i].match(/^\s*\/\/\/\s?(.*)$/);
    if (!m) break;
    out.unshift(m[1]);
  }
  return out;
}

function describe(session: string, name: string, label: string): string {
  const header = path.join(session, 'session_cpp/src', name + '.h');
  if (fs.existsSync(header)) {
    const lines = fs.readFileSync(header, 'utf8').split('\n');
    const keys = new Set([norm(name), norm(label)]);
    for (let i = 0; i < lines.length; i++) {
      const m = lines[i].match(/^\s*(?:template\s*<[^>]*>\s*)?(?:class|struct)\s+(?:\w+\s+)?(\w+)\s*(?:final\s*)?(?:[:{]|$)/);
      if (m && keys.has(norm(m[1]))) {
        const doc = docAbove(lines, i);
        if (doc.length) return firstSentence(doc);
      }
    }
    for (let i = 0; i < lines.length; i++) {
      if (!/^\s*\/\/\/ /.test(lines[i]) || /^\s*\/\/\//.test(lines[i - 1] || '')) continue;
      const doc: string[] = [];
      for (let j = i; j < lines.length && /^\s*\/\/\//.test(lines[j]); j++) doc.push(lines[j].replace(/^\s*\/\/\/\s?/, ''));
      return firstSentence(doc);
    }
  }
  const module = path.join(session, 'session_rust/src', name + '.rs');
  if (fs.existsSync(module)) {
    const doc = fs.readFileSync(module, 'utf8').split('\n').filter((l) => l.startsWith('//!')).map((l) => l.slice(3).trim());
    if (doc.length) return firstSentence(doc);
  }
  return '';
}

export function kernelClasses(session: string): KernelClass[] {
  const dir = path.join(session, 'session_cpp/src');
  if (!fs.existsSync(dir)) return [];
  const out: KernelClass[] = [];
  for (const file of fs.readdirSync(dir).filter((f) => f.endsWith('_test.cpp')).sort()) {
    const src = fs.readFileSync(path.join(dir, file), 'utf8');
    const suite = file.replace(/\.cpp$/, '');
    const name = suite.replace(/_test$/, '');
    const tests: string[] = [];
    const groups: string[] = [];
    for (const m of src.matchAll(/MINI_TEST\(\s*"([^"]+)"\s*,\s*"([^"]+)"\s*\)/g)) {
      groups.push(m[1]);
      if (!tests.includes(m[2])) tests.push(m[2]);
    }
    if (!tests.length) continue;
    const label = groups.find((g) => norm(g) === norm(name)) || groups[0];
    out.push({ suite, label, description: describe(session, name, label), tests });
  }
  return out;
}
