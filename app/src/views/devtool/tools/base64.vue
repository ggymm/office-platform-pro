<template>
  <div class="base64-container">
    <div class="base64-header">
      <back />
    </div>
    <div id="base64-body" class="base64-body">
      <div class="origin">
        <span class="title">原始内容</span>
        <div class="editor">
          <div ref="originEditorRef" :style="originEditorStyle" />
        </div>
      </div>
      <div class="encode">
        <span class="title">编码内容</span>
        <div class="editor">
          <div ref="encodeEditorRef" :style="encodeEditorStyle" />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api'
import { $ } from '~/utils/dom'
import { monaco } from '~/plugins/monaco'

import { Back } from '@comps/fragment'
import { useNotification } from "naive-ui"

let originEditor
const originEditorRef = ref()
const originEditorStyle = reactive({
  width: '200px',
  height: '100%'
})

let encodeEditor
const encodeEditorRef = ref()
const encodeEditorStyle = reactive({
  width: '200px',
  height: '100%'
})

const notification = useNotification()

onMounted(() => {
  resizeEditor()
  window.onresize = async() => {
    // 重设代码编辑器宽高
    resizeEditor()
  }

  originEditor = monaco.editor.create(originEditorRef.value, {
    value: '',
    theme: 'vs',
    minimap: {
      enabled: false
    },
    language: 'txt',
    automaticLayout: true
  })
  originEditor.onDidChangeModelContent(() => {
    handleOriginEditorChange()
  })

  encodeEditor = monaco.editor.create(encodeEditorRef.value, {
    value: '',
    theme: 'vs',
    minimap: {
      enabled: false
    },
    language: 'txt',
    automaticLayout: true
  })
  encodeEditor.onDidChangeModelContent(() => {
    handleEncodeEditorChange()
  })
})

const resizeEditor = () => {
  const padding = 8

  const $body = $("#base64-body")
  // 中间间距(base64-body:grid-gap): 24; 两侧边距(base64-body): 20 * 2
  const width = $body.clientWidth - 24 - 40 - padding * 4
  originEditorStyle.width = width / 2 + 'px'
  encodeEditorStyle.width = width / 2 + 'px'

  // 标题高度(title): 24; 底部边距(base64-body): 20
  const height = $body.clientHeight - 24 - 20 - padding * 2
  originEditorStyle.height = height + 'px'
  encodeEditorStyle.height = height + 'px'
}

const handleOriginEditorChange = () => {
  if (encodeEditor.hasTextFocus()) {
    return
  }
  invoke('plugin:devtool|base64_encode', {
    txt: originEditor.getValue()
  }).then((resp) => {
    if (resp.success) {
      encodeEditor.setValue(resp.data)
    }
  })
}

const handleEncodeEditorChange = () => {
  if (originEditor.hasTextFocus()) {
    return
  }
  invoke('plugin:devtool|base64_decode', {
    txt: encodeEditor.getValue()
  }).then((resp) => {
    if (resp.success) {
      originEditor.setValue(resp.data)
    } else {
      originEditor.setValue('')
      notification['error']({
        content: "解码失败",
        meta: resp.message,
        duration: 2500,
        keepAliveOnHover: true
      })
    }
  })
}

</script>

<style lang="scss">
@import '../index';
</style>
