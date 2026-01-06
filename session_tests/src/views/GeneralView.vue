<template>
  <div class="viewer-root">
    <div class="viewer-area">
      <span class="viewer-placeholder-text">3D Viewer placeholder</span>
    </div>
  </div>
</template>

<style scoped>
.viewer-root {
  height: 100%;
  background: #000000;
}

.viewer-area {
  height: 100%;
  border-radius: 0;
  border: none;
  background: #000000;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #444444;
}

.viewer-placeholder-text {
  font-size: 18px;
}
</style>
