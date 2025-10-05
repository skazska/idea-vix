import { defineConfig } from 'vite'
import solid from 'vite-plugin-solid'
import tailwindcss from '@tailwindcss/vite'

console.log('vite config', { WS_PORT: process.env.WS_PORT });

export default defineConfig({
  plugins: [solid(), tailwindcss()],
  server: {
    proxy: {
      '/api': {
        target: process.env.WS_PORT ? `http://localhost:${process.env.WS_PORT}` : 'http://localhost:7878',
        changeOrigin: true,
        // rewrite: (path) => path.replace(/^\/api/, '')
      },
      '/socket.io': {
        target: process.env.WS_PORT ? `http://localhost:${process.env.WS_PORT}` : 'http://localhost:7878',
        changeOrigin: true,
        ws: true,
        // rewrite: (path) => path.replace(/^\/socket.io/, '/socket.io')
      }
    }
  },
})
