<template>
  <MainLayout></MainLayout>
</template>

<script setup lang="ts">
import MainLayout from './components/layout/MainLayout.vue'
</script>

<style>
/* Self-hosted JetBrains Mono (public/fonts/) — one font for UI, code snippets, and (later) the
   viewer. Served under the app base /session/. */
@font-face {
  font-family: 'JetBrains Mono';
  font-style: normal;
  font-weight: 400;
  font-display: swap;
  src: url('/session/fonts/jetbrains-mono-400.woff2') format('woff2');
}
@font-face {
  font-family: 'JetBrains Mono';
  font-style: normal;
  font-weight: 700;
  font-display: swap;
  src: url('/session/fonts/jetbrains-mono-700.woff2') format('woff2');
}

* {
  box-sizing: border-box;
}

html {
  font-family: 'JetBrains Mono', ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
}

body {
  margin: 0;
  padding: 0;
  background: #000000;
  color: #ffffff;
  font-size: 14px;
}

button,
input,
textarea {
  font-family: inherit;
  font-size: 14px;
}
</style>
