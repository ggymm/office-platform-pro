<template>
  <div class="common-container">
    <div class="common-header">
      <back />
    </div>
    <div class="common-body">
      <div class="handler">
        <n-button type="success" @click="handleRecordStart">开始录制</n-button>
      </div>
      <video ref="videoRef" class="video" controls autoplay playsinline />
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { Back } from '@comps/fragment'

const videoRef = ref()

onMounted(() => {
  if (!navigator.getDisplayMedia && !navigator.mediaDevices.getDisplayMedia) {
    let error = 'Your browser does NOT support the getDisplayMedia API.'
    throw new Error(error)
  }
})

const handleRecordStart = async() => {
  try {
    const stream = await navigator.mediaDevices.getDisplayMedia({ video: true })
    const mediaRecorder = new MediaRecorder(stream, {
      mimeType: 'video/webm'
    })

    mediaRecorder.addEventListener('dataavailable', (event) => {
      if (event.data && event.data.size > 0) {
        videoRef.value.src = videoRef.value.srcObject = null
        videoRef.value.src = URL.createObjectURL(event.data)
        videoRef.value.blob = event.data
      }
    })

    mediaRecorder.start()
    videoRef.value.srcObject = stream
    videoRef.value.controls = true
  } catch (e) {
    // 点击取消
    window['$message'].error(e.message)
  }
}
</script>

<style lang="scss">
@import '../assets/common';

.common-body {
  display: flex;
  flex-direction: column;

  .handler {
    height: 48px;
    display: flex;
    align-items: center;
    margin-bottom: 12px;

    .n-button {
      width: 100%;
    }
  }

  .video {
    width: 100%;
    height: 100%;
    min-height: 300px;
    object-fit: contain;
  }
}
</style>
