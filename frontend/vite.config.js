import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react()],
  server: {
    port: 5173,
    allowedHosts: ['*.ngrok-free.app', 's25u.oelabs'],
    proxy: {
      '/api': {
        target: 'http://localhost:1025',
        changeOrigin: true,
      },
    },
  },
})
