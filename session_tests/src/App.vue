<template>
  <MainLayout></MainLayout>
</template>

<script>
import MainLayout from './components/layout/MainLayout.vue'

export default {
  components: {
    MainLayout
  }
}
</script>

<style>
* {
  box-sizing: border-box;
}

html {
  font-family: system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
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
