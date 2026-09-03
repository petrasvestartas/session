import { defineConfig } from 'vite';

export default defineConfig({
  base: './',
  server: {
    port: 8769,
    strictPort: true,
  },
  build: {
    outDir: 'dist',
  },
});
