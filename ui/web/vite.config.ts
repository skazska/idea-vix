import { defineConfig } from 'vite'
import solid from 'vite-plugin-solid'
import tailwindcss from '@tailwindcss/vite'

console.log('vite config', { WS_PORT: process.env.WS_PORT });

export default defineConfig({
  plugins: [solid(), tailwindcss()],
  server: {
    proxy: {
      '/api': {
        target: `http://localhost:${process.env.WS_PORT || 7878}`,
        changeOrigin: true,
        // rewrite: (path) => path.replace(/^\/api/, '')
      },
      '/socket.io': {
        target: `http://localhost:${process.env.WS_PORT || 7878}`,
        changeOrigin: true,
        ws: true,
        // rewrite: (path) => path.replace(/^\/socket.io/, '/socket.io')
      }
    }
  },
})
