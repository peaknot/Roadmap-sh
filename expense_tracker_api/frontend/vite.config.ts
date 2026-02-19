import { defineConfig } from 'vite'

export default defineConfig({
  server: {
    proxy: {
      '/users': 'http://localhost:3000',
      '/login': 'http://localhost:3000',
      '/home/expense/add': 'http://localhost:3000',
      '/home/expense/list': 'http://localhost:3000',
    }
  }
})
