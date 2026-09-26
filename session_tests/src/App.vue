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
.hk { color: #8e0b61; font-weight: 600; }   /* keyword: plum from the palette, saturated */
.ht { color: #1f2328; font-weight: 700; }   /* type or class: bold black */
.hu { color: #8e0b61; }                      /* module or namespace: plum from the palette, saturated */
.ha { color: #647906; }                      /* attribute or decorator: leaf green, saturated */
.hf { color: #b82ea0; }                      /* function or method: mauve, saturated */
.hx { color: #b82ea0; font-weight: 600; }    /* macro: mauve bold */
.hv { color: #1f50ad; }                      /* variable: slate blue, saturated */
.hp { color: #1f50ad; font-style: italic; }  /* parameter: slate blue italic */
.hm { color: #647906; }                      /* field: leaf green, saturated */
.hn { color: #1f2328; }                      /* number or constant: black, like any typed-in value */
.hs { color: #1f2328; }                      /* string: black, like any typed-in value */
.hc { color: #7580c7; font-style: italic; }  /* comment: lavender grey, saturated */
.ho { color: #7580c7; }                      /* operator or punctuation: lavender grey, saturated */
</style>
