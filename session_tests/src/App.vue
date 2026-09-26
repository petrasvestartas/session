<template>
  <MainLayout></MainLayout>
</template>

<script setup lang="ts">
import MainLayout from './components/layout/MainLayout.vue'
</script>

<style>
/* Plain look: system fonts, black text, greys for rules and muted text, white background. */
:root {
  --fg: #000000;
  --muted: #666666;
  --faint: #999999;
  --rule: #e2e2e2;
  --hover: #f2f2f2;
  --code-bg: #f6f6f6;
  --fail: #a00000;
  --sans: system-ui, -apple-system, 'Segoe UI', Roboto, sans-serif;
  --mono: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
}

* {
  box-sizing: border-box;
}

html {
  font-family: var(--sans);
}

body {
  margin: 0;
  padding: 0;
  background: #ffffff;
  color: var(--fg);
  font-size: 15px;
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
.hk { color: #cf222e; }                      /* keyword */
.ht { color: #953800; }                      /* type or class */
.hu { color: #6639ba; }                      /* module or namespace */
.ha { color: #116329; }                      /* attribute or decorator */
.hf { color: #1f2328; }                      /* function or method: black */
.hx { color: #a0169e; font-weight: 600; }    /* macro */
.hv { color: #0969da; }                      /* variable */
.hp { color: #bc4c00; font-style: italic; }  /* parameter */
.hm { color: #1b7c83; }                      /* field */
.hn { color: #9a6700; }                      /* number or constant */
.hs { color: #0a3069; }                      /* string */
.hc { color: #5c6570; font-style: italic; }  /* comment */
.ho { color: #57606a; }                      /* operator or punctuation */
</style>
