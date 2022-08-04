<template>
  <div class="code-snippet-container">
    <div class="code-snippet-catalog" :style="catalogStyle()">
      <div>
        <span>文件夹</span>
      </div>
      <el-tree :data="tempTreeData" :props="defaultProps" />
    </div>
    <div id="resizer" :style="resizerStyle()">
      <div id="resizer-bar" class="resizer-bar" />
    </div>
    <div class="code-snippet-content pt-16" :style="contentStyle()">
      <div class="header">
        <div class="top px-8">
          <div class="name">
            <input v-model="codeSnippet.name" type="text">
          </div>
          <div class="action">
            <el-icon>
              <plus @click="handleFragmentAdd" />
            </el-icon>
          </div>
        </div>
        <div class="bottom mt-8">
          <div class="tags px-8">
            <el-tag
              v-for="tag in codeSnippet.tags" :key="tag"
              :disable-transitions="false" closable class="mx-1" @close="handleTagsClose(tag)"
            >
              {{ tag }}
            </el-tag>
            <el-input
              v-if="tagsInputVisible"
              ref="tagsInputRef"
              v-model="tagsInputValue"
              class="tags-input"
              size="small"
              @keyup.enter="handleTagsInputConfirm"
              @blur="handleTagsInputConfirm"
            />
            <el-button v-else size="small" @click="showTagsInput">新建标签</el-button>
          </div>
          <div class="fragments mt-8">
            <el-menu mode="horizontal" @select="handleFragmentSelect">
              <el-menu-item v-for="(fragment,i) in codeSnippet.fragments" :key="i" :index="i + ''">{{ fragment.title }}</el-menu-item>
            </el-menu>
          </div>
        </div>
      </div>
      <div class="code-editor">
        <div ref="editor" class="editor" :style="editorStyle" />
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive, onMounted, nextTick } from 'vue'
import { monaco } from '~/plugins/monaco'

import useConfigStore from '~/store/config'

const configStore = useConfigStore()
const defaultProps = {
  children: 'children',
  label: 'label'
}

const tempTreeData = []
const codeSnippet = reactive({
  name: '测试标题',
  tags: [],
  fragments: [
    {
      title: 'default'
    }
  ]
})

let catalogWidth = ref(280 + configStore.sidebarWidth)
const catalogStyle = () => {
  return {
    width: `${catalogWidth.value - configStore.sidebarWidth}px`,
    flex: `0 0 ${catalogWidth.value - configStore.sidebarWidth}px`,
    maxWidth: `${catalogWidth.value - configStore.sidebarWidth}px`,
    minWidth: `${catalogWidth.value - configStore.sidebarWidth}px`
  }
}

const resizerStyle = () => {
  return {
    left: catalogWidth.value + 'px'
  }
}

const contentStyle = () => {
  return {
    width: `calc(100% - ${catalogWidth.value}px)`
  }
}

const tagsInputRef = ref()
const tagsInputValue = ref('')
const tagsInputVisible = ref(false)

const editor = ref()
const editorStyle = reactive({
  width: '200px',
  height: '100%'
})

onMounted(() => {
  handleResize()
  resizeEditor()
  window.onresize = async() => {
    // 重设代码编辑器宽高
    resizeEditor()
  }

  monaco.editor.create(editor.value, {
    value: 'console.log("Hello, world")',
    language: 'javascript',
    automaticLayout: true
  })
})

const handleResize = () => {
  const resizer = document.getElementById('resizer')
  const resizerBar = document.getElementById('resizer-bar')

  let currCatalogWidth = catalogWidth
  resizer.onmousedown = function(e) {
    e.preventDefault()

    resizer.onmousemove = function(e) {
      currCatalogWidth = e.pageX
      if (e.pageX < 150) {
        currCatalogWidth = 150
      }
      if (e.pageX > 1000) {
        currCatalogWidth = 1000
      }
      resizer.className = 'dragging'
      resizer.style.left = '0'

      resizerBar.style.left = currCatalogWidth + 'px'
    }

    resizer.onmouseup = function() {
      catalogWidth.value = currCatalogWidth
      resizeEditor()

      resizer.className = ''
      resizer.onmouseup = null
      resizer.onmousemove = null
      resizer.style.left = currCatalogWidth + 'px'

      resizerBar.style.left = ''
    }
  }
}

const resizeEditor = () => {
  const catalog = catalogWidth.value - configStore.sidebarWidth
  editorStyle.width = window.innerWidth - configStore.sidebarWidth - catalog + 'px'
}

const showTagsInput = () => {
  tagsInputVisible.value = true
  nextTick(() => {
    tagsInputRef.value.focus()
  })
}

const handleTagsClose = (tag) => {
  codeSnippet.tags.splice(codeSnippet.tags.indexOf(tag), 1)
}

const handleTagsInputConfirm = () => {
  if (tagsInputValue.value) {
    codeSnippet.tags.push(tagsInputValue.value)
  }
  tagsInputValue.value = ''
  tagsInputVisible.value = false
}

const handleFragmentSelect = () => {

}

const handleFragmentAdd = () => {
  codeSnippet.fragments.push({
    title: 'title'
  })
}

</script>

<style lang="scss">
@import 'index.scss';
</style>
