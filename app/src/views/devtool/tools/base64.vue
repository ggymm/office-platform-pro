<template>
  <div class="common-container">
    <div class="common-header">
      <back />
      <n-space class="handle">
        <n-switch :round="false" @update:value="handleBase64TypeChange">
          <template #checked>图片</template>
          <template #unchecked>字符串</template>
        </n-switch>
        <n-button v-show="base64Image" type="info" @click="handleResetBase64Image">删除当前图片</n-button>
      </n-space>
    </div>
    <div id="base64-body" class="common-body">
      <div class="origin">
        <span class="title">{{ base64Image ? base64ImagePath : '原始内容' }}</span>
        <div v-show="!base64Image" class="editor">
          <div ref="originEditorRef" :style="editorStyle" />
        </div>
        <div v-show="base64Image" class="image">
          <div v-show="base64ImagePath.length === 0" class="upload" @click="handleChooseBase64Image">
            <div class="upload-dragger">
              <div style="margin-bottom: 12px">
                <n-icon size="48" :depth="3">
                  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
                    <path
                      d="M80 152v256a40.12 40.12 0 0 0 40 40h272a40.12 40.12 0 0 0 40-40V152" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round"
                      stroke-width="32"
                    />
                    <rect
                      x="48" y="64" width="416" height="80" rx="28"
                      ry="28" fill="none" stroke="currentColor" stroke-linejoin="round" stroke-width="32"
                    />
                    <path
                      fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="32"
                      d="M320 304l-64 64l-64-64"
                    />
                    <path
                      fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="32"
                      d="M256 345.89V224"
                    />
                  </svg>
                </n-icon>
              </div>
              <n-text style="font-size: 16px">点击或者拖动文件到该区域</n-text>
            </div>
          </div>
          <div v-show="base64ImagePath.length !== 0" class="show" :style="editorStyle">
            <n-image :src="base64ImageEncode" />
          </div>
        </div>
      </div>
      <div class="encode">
        <span class="title">编码内容</span>
        <div class="editor">
          <div ref="encodeEditorRef" :style="editorStyle" />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive, onMounted } from 'vue'
import { $ } from '~/utils/dom'
import { monaco } from '~/plugins/monaco'

import { Back } from '@comps/fragment'
import { invoke, dialog } from '@tauri-apps/api'

let originEditor
let encodeEditor
const originEditorRef = ref()
const encodeEditorRef = ref()

const base64Image = ref(false)
const base64ImagePath = ref('untitled')
const base64ImageEncode = ref('')

const editorStyle = reactive({ width: '', height: '' })

onMounted(() => {
  resizeEditor()
  window.onresize = async() => {
    resizeEditor() // 重设代码编辑器宽高
  }

  const option = {
    value: '',
    theme: 'vs',
    minimap: {
      enabled: false
    },
    wordWrap: 'on',
    language: 'txt',
    automaticLayout: true
  }

  originEditor = monaco.editor.create(originEditorRef.value, option)
  originEditor.onDidChangeModelContent(() => {
    handleEncode()
  })

  encodeEditor = monaco.editor.create(encodeEditorRef.value, option)
  encodeEditor.onDidChangeModelContent(() => {
    handleDecode()
  })
})

const resizeEditor = () => {
  const padding = 8
  const $body = $("#base64-body")

  // 两侧边距(base64-body): 20 * 2
  const width = $body.clientWidth - 40 - padding * 2
  editorStyle.width = width + 'px'

  // 标题高度(title): 24 * 2; 中间间距(base64-body:grid-gap): 12; 底部边距(base64-body): 24
  const height = $body.clientHeight - 84 - padding * 4
  editorStyle.height = height / 2 + 'px'
}

const handleEncode = () => {
  // 处于图片编码模式, 禁用此监听
  if (!base64Image.value) {
    // encode 处于编辑状态
    if (encodeEditor.hasTextFocus()) {
      return
    }

    invoke('plugin:devtool|base64_encode', {
      val: originEditor.getValue()
    }).then((resp) => {
      if (resp.success) {
        encodeEditor.setValue(resp.data)
      }
    })
  }
}

const handleDecode = () => {
  if (base64Image.value) {
    if (!encodeEditor.getValue()) {
      base64ImagePath.value = ''
      base64ImageEncode.value = ''
      return
    }
    base64ImageEncode.value = 'data:image/png;base64,' + encodeEditor.getValue()
  } else {
    if (!encodeEditor.getValue()) {
      originEditor.setValue('')
      return
    }

    // origin 处于编辑状态
    if (originEditor.hasTextFocus()) {
      return
    }

    invoke('plugin:devtool|base64_decode', {
      val: encodeEditor.getValue()
    }).then((resp) => {
      if (resp.success) {
        originEditor.setValue(resp.data)
      } else {
        originEditor.setValue('')
        window['$notification']['error']({
          content: "解码失败",
          meta: resp.message,
          duration: 2500,
          keepAliveOnHover: true
        })
      }
    })
  }
}

const handleBase64TypeChange = (value) => {
  base64Image.value = value

  // 重置当前图片数据
  handleResetBase64Image()
}

const handleChooseBase64Image = async() => {
  base64ImagePath.value = await dialog.open({
    title: '选择图片',
    multiple: false,
    filters: [{
      name: 'Image',
      extensions: ['png']
    }]
  })

  // 选择文件后进行编码
  invoke('plugin:devtool|base64_file_encode', {
    filepath: base64ImagePath.value
  }).then((resp) => {
    if (resp.success) {
      encodeEditor.setValue(resp.data)
      // 判断图片类型
      base64ImageEncode.value = 'data:image/png;base64,' + resp.data
    } else {
      window['$notification']['error']({
        content: "图片编码失败",
        meta: resp.message,
        duration: 2500,
        keepAliveOnHover: true
      })
    }
  })
}

const handleResetBase64Image = () => {
  // 移除当前图片
  base64ImagePath.value = ''
  base64ImageEncode.value = ''

  // 清空编辑器内容
  originEditor.setValue('')
  encodeEditor.setValue('')
}
</script>

<style lang="scss">
@import '../assets/common';

.common-header {
  justify-content: space-between;

  .handle {
    display: flex;
    align-items: center;
  }
}

.common-body {
  display: grid;
  grid-gap: 12px 0;
  grid-template-rows: 1fr 1fr;
  grid-template-columns: 1fr;

  .origin, .encode {
    display: flex;
    flex-direction: column;
  }

  .title {
    height: 20px;
    line-height: 20px;
    font-size: 14px;
    margin-bottom: 4px;
  }

  .editor {
    padding: 8px;
    border-radius: 8px;
    background-color: #ffffff;
    box-shadow: 0 0 4px #00000010;
  }

  .image {
    width: 100%;
    height: 100%;

    .upload {
      width: 100%;
      height: 100%;
      display: flex;
      flex-direction: column;

      .upload-dragger {
        width: 100%;
        height: 100%;
        cursor: pointer;
        display: flex;
        align-items: center;
        flex-direction: column;
        justify-content: center;
        box-sizing: border-box;
        border: 1px dashed #e0e0e6;
        background-color: #ffffff;

        &:hover {
          border: 1px dashed #2d8cf0;
        }
      }
    }

    .show {
      width: 100%;
      height: 100%;
      padding: 8px;
      display: flex;
      flex-direction: column;

      img {
        width: 100%;
        height: 100%;
        object-fit: contain !important;
      }
    }
  }
}
</style>
