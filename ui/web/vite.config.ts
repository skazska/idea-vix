import { defineConfig } from 'vite'
import solid from 'vite-plugin-solid'
import tailwindcss from '@tailwindcss/vite'

export default defineConfig({
  plugins: [solid(), tailwindcss()],
  server: {
    proxy: {
      '/api': {
        target: 'http://localhost:7878',
        changeOrigin: true,
        // rewrite: (path) => path.replace(/^\/api/, '')
      },
      '/socket.io': {
        target: 'http://localhost:7878',
        changeOrigin: true,
        ws: true,
        // rewrite: (path) => path.replace(/^\/socket.io/, '/socket.io')
      }
    }
  },
})
