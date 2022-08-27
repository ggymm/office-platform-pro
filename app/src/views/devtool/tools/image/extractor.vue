<template>
  <div class="common-container">
    <div class="common-header">
      <back />
    </div>
    <div class="common-body">
      <div class="url">
        <n-input v-model:value="url" type="text" />
        <n-button type="primary" @click="handleExtract">提取</n-button>
        <n-button type="primary" @click="handleDownloadAll">下载全部</n-button>
      </div>
      <div class="image-list">
        <div v-for="(image, i) in imageList" :key="i" class="image">
          <n-image :src="image" />
          <n-text>
            <n-ellipsis style="max-width: 100%">{{ image }}</n-ellipsis>
          </n-text>
          <n-button secondary strong @click="handleDownload">下载</n-button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from "vue"

import { Back } from '@comps/fragment'

const url = ref('https://app.dev-box.app/#css-cursors')
const imageList = ref([
  'https://07akioni.oss-cn-beijing.aliyuncs.com/07akioni.jpeg',
  'https://07akioni.oss-cn-beijing.aliyuncs.com/07akioni.jpeg',
  'https://07akioni.oss-cn-beijing.aliyuncs.com/07akioni.jpeg',
  'https://07akioni.oss-cn-beijing.aliyuncs.com/07akioni.jpeg',
  'https://07akioni.oss-cn-beijing.aliyuncs.com/07akioni.jpeg',
  'https://07akioni.oss-cn-beijing.aliyuncs.com/07akioni.jpeg',
  'https://07akioni.oss-cn-beijing.aliyuncs.com/07akioni.jpeg',
  'https://07akioni.oss-cn-beijing.aliyuncs.com/07akioni.jpeg',
  'https://07akioni.oss-cn-beijing.aliyuncs.com/07akioni.jpeg',
  'https://07akioni.oss-cn-beijing.aliyuncs.com/07akioni.jpeg',
  'https://07akioni.oss-cn-beijing.aliyuncs.com/07akioni.jpeg',
  'https://07akioni.oss-cn-beijing.aliyuncs.com/07akioni.jpeg',
  'https://07akioni.oss-cn-beijing.aliyuncs.com/07akioni.jpeg'
])


const handleExtract = () => {
  // 暂时无法实现
}

const handleDownload = () => {

}

const handleDownloadAll = () => {

}

</script>

<style lang="scss">
@import '../../assets/common';

.common-header {
  justify-content: space-between;
}

.common-body {
  .url {
    height: 48px;
    display: flex;
    align-items: center;
    flex-direction: row;

    .n-input {
      width: 60%;
    }

    .n-button {
      margin-left: 24px;
    }
  }

  .image-list {
    height: calc(100vh - 72px - 64px);
    overflow: auto;
    display: grid;
    grid-gap: 20px;
    margin-top: 8px;
    padding-bottom: 8px;
    grid-template-columns: repeat(auto-fill, minmax(480px, 1fr));

    &::-webkit-scrollbar {
      display: none;
    }

    .image {
      display: flex;
      align-items: center;
      border-radius: 8px;
      flex-direction: column;
      background-color: #ffffff;
      box-shadow: 0 0 8px #00000010;

      .n-image {
        width: 100%;
        height: 220px;
        display: flex;
        padding: 20px 0;
        justify-content: center;

        img {
          width: 100%;
          height: 100%;
          object-fit: contain !important;
        }
      }

      .n-text {
        width: calc(100% - 40px);
        height: 48px;
        line-height: 48px;
      }

      .n-button {
        width: 100%;
        height: 48px;
        line-height: 48px;
        border-bottom-left-radius: 8px;
        border-bottom-right-radius: 8px;
      }
    }
  }
}
</style>
