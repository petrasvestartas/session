import { defineConfig } from 'vite';

export default defineConfig({
  base: './',
  publicDir: 'web/public',
  server: {
    port: 8769,
    strictPort: true,
  },
  build: {
    outDir: 'dist',
  },
});
