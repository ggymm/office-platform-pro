<template>
  <div class="common-container">
    <div class="common-header linux-header">
      <back />
      <n-space class="handle">
        <n-input v-model:value="searchInfo" placeholder="" @keyup.enter="handleSearch" />
        <n-button type="info" @click="handleSearch">搜索</n-button>
      </n-space>
    </div>
    <div id="linux-body" class="common-body linux-body">
      <n-data-table
        :row-props="rowProps"
        :columns="columns"
        :data="commandData"
        :max-height="tableHeight"
        virtual-scroll :single-line="false"
      />
    </div>
  </div>
</template>

<script setup>
import { onMounted, ref } from "vue"
import { $ } from "~/utils/dom"
import commands from '~/assets/linux/command-data.min.json'

import { Back } from '@comps/fragment'
import { WebviewWindow } from '@tauri-apps/api/window'

const columns = [
  {
    title: '名称',
    key: 'n',
    width: 240
  },
  {
    title: '描述',
    key: 'd'
  }
]
const commandData = ref([])
const tableHeight = ref(300)

const searchInfo = ref()

onMounted(() => {
  resizeTableHeight()
  window.onresize = async() => {
    resizeTableHeight()
  }
  for (let k in commands) {
    commandData.value.push(commands[k])
  }
  window['commands'] = commandData.value
})

const resizeTableHeight = () => {
  const $body = $("#linux-body")
  tableHeight.value = $body.clientHeight - 8 - 48
}

const handleSearch = () => {
  const filterCommandData = []
  for (let k in commands) {
    const command = commands[k]
    if (command['n'].includes(searchInfo.value) || command['d'].includes(searchInfo.value)) {
      filterCommandData.push(command)
    }
  }
  commandData.value = filterCommandData
}

const rowProps = (row) => {
  return {
    onClick: async () => {
      let webview = WebviewWindow.getByLabel('LinuxCommand')
      if (webview) {
        await webview.close()
      }
      webview = new WebviewWindow('LinuxCommand', {
        url: `/linux-command${row['p']}.md.html`
      })
      await webview.once('tauri://created', function () {
      })
      await webview.once('tauri://error', function (e) {
      })
    }
  }
}

</script>

<style lang="scss">
@import '../assets/common';

.linux-header {
  justify-content: space-between;

  .handle {
    display: flex;
    align-items: center;

    .n-input {
      width: 320px;

      .n-input__input-el {
        height: 28px;
        line-height: 28px;
      }
    }
  }
}

.linux-body {
  overflow: auto;

  &::-webkit-scrollbar {
    display: none;
  }

  .n-data-table {
    padding-top: 8px;
  }
}
</style>
