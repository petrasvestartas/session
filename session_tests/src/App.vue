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

/* Greyscale code classes written by the course build (plugins/course.ts). */
pre .c { color: #8a8a8a; font-style: italic; }
pre .s { color: #5c5c5c; }
pre .k { color: #000000; font-weight: 600; }
</style>
