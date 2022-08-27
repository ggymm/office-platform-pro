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

import devtoolList from './assets/tools.json'

const router = useRouter()

const search = ref()

const goDevtool = (devtool) => {
  if (devtool.router) {
    router.push({ path: devtool.router})
  }
}
</script>

<style lang="scss">
.devtool-container {
  width: calc(100vw - 72px);
  height: 100vh;
  display: flex;
  flex-direction: column;

  .devtool-header {
    height: 36px;
    padding: 20px;
    display: flex;
    align-items: center;
    justify-content: space-between;

    .left {
      width: 480px;
    }
  }

  .devtool-list-container {
    padding: 8px 20px 20px;
    overflow: auto;

    &::-webkit-scrollbar {
      display: none;
    }

    .devtool-list {
      display: grid;
      grid-gap: 20px;
      grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));

      .devtool {
        color: #555555;
        height: 80px;
        display: flex;
        padding: 16px 0 16px 16px;
        align-items: center;
        border-radius: 8px;
        background-color: #ffffff;
        box-shadow: 0 0 8px #00000010;

        &:hover {
          cursor: pointer;
          box-shadow: 0 0 0 2px #2d8cf0, 0 0 8px #00000010;
        }

        .icon {
          width: 50px;
          height: 50px;
        }

        .body {
          margin-left: 10px;

          .title {
            height: 36px;
            font-size: 18px;
            line-height: 36px;
          }

          .description {
            height: 24px;
            font-size: 14px;
            line-height: 24px;
          }
        }
      }
    }
  }
}
</style>
