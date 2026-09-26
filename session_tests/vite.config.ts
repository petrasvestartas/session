import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import { resolve } from 'path'
import { watch } from 'fs'
import coursePlugin from './plugins/course'

function watchPublicPlugin() {
  let reloadTimeout = null
  return {
    name: 'watch-public',
    configureServer(server) {
      const publicDir = resolve(__dirname, 'public')
      watch(publicDir, { recursive: true }, (event, filename) => {
        if (filename && filename.endsWith('.js')) {
          clearTimeout(reloadTimeout)
          reloadTimeout = setTimeout(() => {
            server.ws.send({ type: 'full-reload', path: '*' })
          }, 1000)
        }
      })
    }
  }
}

// Production is served at /session/docs/ next to the viewer (DOCS_BASE overrides it); the dev
// server keeps /session/ so the minitest URL localhost:8769/session/tests#/tests still works.
export default defineConfig(({ command }) => ({
  base: command === 'build' ? process.env.DOCS_BASE || '/session/docs/' : '/session/',
  plugins: [vue(), watchPublicPlugin(), coursePlugin()],
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
    open: false
  },
  publicDir: 'public'
}))
