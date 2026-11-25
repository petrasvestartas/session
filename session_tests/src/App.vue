<template>
  <MainLayout></MainLayout>
  <div class="docs-build">Docs built <time :datetime="builtAt">{{ buildLabel }}</time></div>
</template>

<script setup lang="ts">
import MainLayout from './components/layout/MainLayout.vue'

const builtAt = import.meta.env.DOCS_BUILT_AT
const buildLabel = builtAt.replace('T', ' ').replace(/\.\d{3}Z$/, ' UTC')
</script>

<style>
.main-layout {
  padding-bottom: max(28px, calc(24px + env(safe-area-inset-bottom)));
}

.docs-build {
  position: fixed;
  right: max(36px, env(safe-area-inset-right));
  bottom: max(4px, env(safe-area-inset-bottom));
  z-index: 201;
  padding: 3px 6px;
  background: #ffffff;
  color: #6f7290;
  font: 11px/1.4 var(--sans);
  white-space: nowrap;
  pointer-events: none;
}

/* Wood documentation palette: white paper, 1px hairlines, wood green for active/links, square corners. */
:root {
  --fg: #16181a;
  --ink: #16181a;
  --paper: #ffffff;
  --muted: #5d5f58;
  --faint: #a3a59d;
  --rule: #dadad6;
  --wash: #f5f6f6;
  --hover: #f5f6f6;
  --accent: #007c55;
  --mark: #99cbbb;
  --code-bg: #f5f6f6;
  --fail: #b42318;
  --sans: Outfit, -apple-system, BlinkMacSystemFont, 'Segoe UI', Helvetica, Arial, sans-serif;
  --mono: 'JetBrains Mono', ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
}

*, *::before, *::after {
  box-sizing: border-box;
  border-radius: 0;
}

html {
  font-family: var(--sans);
}

body {
  margin: 0;
  padding: 0;
  background: var(--paper);
  color: var(--fg);
  font-size: 15.5px;
  -webkit-font-smoothing: antialiased;
}

button,
input,
textarea {
  font-family: inherit;
  font-size: 14px;
}

code,
pre,
kbd {
  font-family: var(--mono);
}

/* Code colours (src/codeTheme.ts), only code is coloured; each is at least 4.7:1 on #f6f6f6. */
pre { color: #1f2328; }
.hk { color: #8e0b61; font-weight: 600; }   /* keyword: plum from the palette, saturated */
.ht { color: #1f2328; font-weight: 700; }   /* type or class: bold black */
.hu { color: #8e0b61; }                      /* module or namespace: plum from the palette, saturated */
.ha { color: #647906; }                      /* attribute or decorator: leaf green, saturated */
.hf { color: #d97757; }                      /* function or method: Claude orange */
.hx { color: #d97757; font-weight: 600; }    /* macro: Claude orange bold */
.hv { color: #1f50ad; }                      /* variable: slate blue, saturated */
.hp { color: #1f50ad; font-style: italic; }  /* parameter: slate blue italic */
.hm { color: #647906; }                      /* field: leaf green, saturated */
.hn { color: #1f2328; }                      /* number or constant: black, like any typed-in value */
.hs { color: #1f2328; }                      /* string: black, like any typed-in value */
.hc { color: #6e7781; font-style: italic; }  /* comment: grey */
.ho { color: #57606a; }                      /* operator or punctuation: grey */
</style>
