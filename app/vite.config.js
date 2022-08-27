import path from 'path'

import { defineConfig } from 'vite'

import vue from '@vitejs/plugin-vue'
import mkcert from'vite-plugin-mkcert'
import monacoEditorPlugin from 'vite-plugin-monaco-editor'

import AutoImport from 'unplugin-auto-import/vite'
import Components from 'unplugin-vue-components/vite'

import { NaiveUiResolver } from 'unplugin-vue-components/resolvers'

// noinspection JSUnusedGlobalSymbols
export default defineConfig({
  resolve: {
    alias: {
      '~': path.resolve(__dirname, 'src'),
      '@comps': path.resolve(__dirname, 'src/components')
    }
  },
  plugins: [
    vue(),
    mkcert(),
    monacoEditorPlugin({}),
    AutoImport({
      resolvers: [
        NaiveUiResolver()
      ]
    }),
    Components({
      resolvers: [
        NaiveUiResolver()
      ]
    })
  ],
  css: {
    preprocessorOptions: {
      scss: {}
    }
  },
  server: {
    port: 8888,
    host: '0.0.0.0',
    https: true
  }
})
