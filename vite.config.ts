import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import { quasar, transformAssetUrls } from '@quasar/vite-plugin'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    vue({
      template: { transformAssetUrls }
    }),
    quasar()
  ],
  base: "/rust_scripter",
  build: {
    rollupOptions: {
      external: '/dist/rust/rust_scripter.js',
      output: {
        paths: {
          '/dist/rust/rust_scripter.js': '/rust_scripter/dist/rust/rust_scripter.js'
        }
      }
    },
  }
})
