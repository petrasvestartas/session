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
}

.viewer-area {
  height: 100%;
  border-radius: 8px;
  border: 2px dashed #ddd;
  background: #ffffff;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #999;
}

.viewer-placeholder-text {
  font-size: 18px;
}
</style>
