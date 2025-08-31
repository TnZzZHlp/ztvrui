import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueJsx from '@vitejs/plugin-vue-jsx'
import vueDevTools from 'vite-plugin-vue-devtools'
import tailwindcss from '@tailwindcss/vite'
// https://vite.dev/config/

export default defineConfig({
  plugins: [vue(), vueJsx(), vueDevTools(), tailwindcss()],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url)),
    },
  },
  server: {
    proxy: {
      '/ztapi/': {
        target: 'http://127.0.0.1:3000',
        changeOrigin: true,
      },
      '/api/': {
        target: 'http://127.0.0.1:3000',
        changeOrigin: true,
      },
    },
  },
})
