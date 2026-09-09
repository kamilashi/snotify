import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueDevTools from 'vite-plugin-vue-devtools'

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    vueDevTools(),
  ],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url)),
    },
  },
  build: {
    outDir: "../static",
    emptyOutDir: true,
    sourcemap: true, // remove when project is done
  },
  server: {
    port: 5180,
    strictPort: true, 
    proxy: {
      "/api": "http://localhost:6767",
    },
  },
})
