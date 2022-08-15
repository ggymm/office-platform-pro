<template>
  <div>
    <div>{{ aaa }}</div>
    <el-button type="primary" @click="testInvoke()">测试命令调用</el-button>
  </div>
</template>

<script setup>
import { ref, getCurrentInstance } from 'vue'
import { invoke } from '@tauri-apps/api'

const aaa = ref()

const { appContext } = getCurrentInstance()

const testInvoke = () => {
  invoke('plugin:devtool|test_command', { args: 'World' }).then((response) => {
    aaa.value = 11
    console.log(response)
  })
}

</script>

<style scoped>

</style>
