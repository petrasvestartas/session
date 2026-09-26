// One Shiki highlighter, fine-grained: only the grammars imported below are bundled (no on-demand
// grammar chunks for the ~40 languages the full `shiki` entry would register). Shared singleton.
// Course pages are highlighted at build time (plugins/course.ts); this one serves the Kernel API and
// Install pages.
import { createHighlighterCore, type HighlighterCore } from 'shiki/core';
import { createOnigurumaEngine } from 'shiki/engine/oniguruma';
import rust from '@shikijs/langs/rust';
import cpp from '@shikijs/langs/cpp';
import python from '@shikijs/langs/python';
import json from '@shikijs/langs/json';
import bash from '@shikijs/langs/bash';
import toml from '@shikijs/langs/toml';
import { greyTheme, THEME_NAME } from './greyTheme';

export const THEME = THEME_NAME;
export const LANGS = ['rust', 'cpp', 'python', 'json', 'bash', 'toml'];

let promise: Promise<HighlighterCore> | null = null;

export function getHighlighter(): Promise<HighlighterCore> {
  return (promise ??= createHighlighterCore({
    themes: [greyTheme],
    langs: [rust, cpp, python, json, bash, toml],
    engine: createOnigurumaEngine(import('shiki/wasm')),
  }));
}
