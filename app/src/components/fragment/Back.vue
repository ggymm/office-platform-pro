<template>
  <n-button type="info" @click="handleBack">返 回</n-button>
</template>

<script setup>
import { useRouter } from 'vue-router'
const router = useRouter()

const handleBack = () => {
  router.back()
}
</script>
