import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

export default defineConfig({
  plugins: [vue()],
  build: {
    outDir: 'dist',
    assetsDir: 'assets',
    // Copy testData.js to dist during build
    rollupOptions: {
      input: {
        main: './index.html'
      }
    }
  },
  server: {
    port: 8769,
    strictPort: true,
    open: true
  },
  publicDir: 'public'
})
