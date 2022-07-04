<template>
  <div class="app-container">
    <layout />
  </div>
</template>

<script setup>
import { useRouter, useRoute } from 'vue-router'
import Layout from '~/layout/index.vue'

const router = useRouter()
const route = useRoute()

function test() {
  router.push({ path: '/message' })
}

</script>

<style>
</style>
