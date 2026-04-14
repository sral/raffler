import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react()],
  root: 'static',
  publicDir: 'public',
  build: {
    outDir: '../dist',
    emptyOutDir: true,
    copyPublicDir: true,
  },
  server: {
    proxy: {
      '/v1': 'http://localhost:8000'
    }
  }
})
