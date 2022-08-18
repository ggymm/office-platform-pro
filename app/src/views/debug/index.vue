<template>
  <div>
    <div>{{ aaa }}</div>
    <n-button type="primary" @click="testInvoke()">测试命令调用</n-button>
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
