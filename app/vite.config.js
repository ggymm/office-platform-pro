import path from 'path'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

import { ElementPlusResolver } from 'unplugin-vue-components/resolvers'

import AutoImport from 'unplugin-auto-import/vite'
import Components from 'unplugin-vue-components/vite'


export default defineConfig({
  resolve: {
    alias: {
      '~': path.resolve(__dirname, 'src'),
      'comps': path.resolve(__dirname, 'src/components')
    }
  },
  plugins: [
    vue(),
    AutoImport({
      resolvers: [ElementPlusResolver()]
    }),
    Components({
      resolvers: [
        ElementPlusResolver()
      ]
    })
  ],
  css: {
    preprocessorOptions: {
      scss: {
      }
    }
  },
  server: {
    port: 8888,
    host: '0.0.0.0'
  }
})
