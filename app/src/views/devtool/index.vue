<template>
  <div class="devtool-container">
    <div class="devtool-header">
      <div class="left">
        <n-input placeholder="搜索" />
      </div>
      <div class="right">
        右侧
      </div>
    </div>
    <div class="devtool-list-container">
      <div class="devtool-list">
        <div v-for="(devtool,i) in devtoolList" :key="i" class="devtool" @click="goDevtool(devtool)">
          <div v-dompurify-html="devtool.icon" class="icon" />
          <div class="body">
            <div class="title">{{ devtool.title }}</div>
            <div class="description">{{ devtool.description }}</div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { useRouter } from 'vue-router'

import devtoolList from './tools.json'

const router = useRouter()

const search = ref()

const goDevtool = (devtool) => {
  if (devtool.router) {
    router.push({ path: devtool.router})
  }
}
</script>

<style lang="scss">
@import 'index.scss';
</style>
