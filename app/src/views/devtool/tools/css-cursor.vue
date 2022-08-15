<template>
  <div class="css-cursor-container">
    <div class="css-cursor-header">
      <back />
      <span class="tips">点击即可复制到剪切板</span>
    </div>
    <div class="css-cursor-body">
      <div
        v-for="(cursor, i) in cursorList" :key="i" class="cursor-item"
        :style="getCursor(cursor)" @click="copy(cursor)"
      >
        <img class="icon" alt="" :src="getIcon(cursor)">
        <div class="text">{{ cursor }}</div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { useMessage } from 'naive-ui'
import { Back } from '@comps/fragment'

const cursorList = [
  'default', 'context-menu', 'help', 'pointer', 'progress', 'wait', 'cell',
  'crosshair', 'text', 'vertical-text', 'alias', 'copy', 'move', 'no-drop', 'not-allowed',
  'grab', 'grabbing', 'all-scroll', 'col-resize', 'row-resize',
  'n-resize', 'e-resize', 's-resize', 'w-resize',
  'ne-resize', 'nw-resize', 'se-resize', 'sw-resize',
  'ew-resize', 'ns-resize', 'nesw-resize', 'nwse-resize',
  'zoom-in', 'zoom-out'
]
const message = useMessage()

const getCursor = (value) => {
  return { cursor: value }
}

const getIcon = (value) => {
  return new URL(`/src/assets/cursor/${value}.png`, import.meta.url).href
}

const copy = (value) => {
  navigator.clipboard.writeText(value).then(() => {
    message.success(`已复制 ${value} 到剪切板`)
  })
}
</script>

<style lang="scss">
@import '../index';
</style>
