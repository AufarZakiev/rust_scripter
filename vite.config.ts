import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import { quasar, transformAssetUrls } from '@quasar/vite-plugin'
import { plugin as mdPlugin, Mode } from 'vite-plugin-markdown'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    vue({
      template: { transformAssetUrls }
    }),
    mdPlugin({ mode: [Mode.VUE] }),
    quasar()
  ],
  base: "/rust_scripter",
  build: {
    rollupOptions: {
      external: '/dist/rust/rust_scripter.js',
      output: {
        paths: {
          '/dist/rust/rust_scripter.js': '/rust_scripter/rust/rust_scripter.js'
        }
      }
    },
  }
})
