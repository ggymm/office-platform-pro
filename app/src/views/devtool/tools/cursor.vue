<template>
  <div class="common-container">
    <div class="common-header">
      <back />
      <span class="tips">点击即可复制到剪切板</span>
    </div>
    <div class="common-body">
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
import { Back } from '@comps/fragment'
import { writeText } from '@tauri-apps/api/clipboard'

const cursorList = [
  'default', 'context-menu', 'help', 'pointer', 'progress', 'wait', 'cell',
  'crosshair', 'text', 'vertical-text', 'alias', 'copy', 'move', 'no-drop', 'not-allowed',
  'grab', 'grabbing', 'all-scroll', 'col-resize', 'row-resize',
  'n-resize', 'e-resize', 's-resize', 'w-resize',
  'ne-resize', 'nw-resize', 'se-resize', 'sw-resize',
  'ew-resize', 'ns-resize', 'nesw-resize', 'nwse-resize',
  'zoom-in', 'zoom-out'
]

const getCursor = (value) => {
  return { cursor: value }
}

const getIcon = (value) => {
  return new URL(`/src/assets/cursor/${value}.png`, import.meta.url).href
}

const copy = (value) => {
  writeText(value).then(() => {
    window['$message'].success(`已复制 ${value} 到剪切板`)
  })
}
</script>

<style lang="scss">
@import '../assets/common';

.common-header {
  .tips {
    font-size: 12px;
    margin-left: 16px;
  }
}

.common-body {
  display: grid;
  grid-gap: 20px;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));

  overflow: auto;

  &::-webkit-scrollbar {
    display: none;
  }

  .cursor-item {
    display: flex;
    padding: 32px 16px;
    border-radius: 6px;
    align-items: center;
    flex-direction: column;
    background-color: #ffffff;

    &:hover {
      color: #ffffff;
      background-color: #409EFF;

      .icon {
        filter: invert(0);
      }
    }

    .icon {
      width: 24px;
      height: 24px;
      filter: invert(1);
    }

    .text {
      font-size: 14px;
      margin-top: 16px;
    }
  }
}
</style>
