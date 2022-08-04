<template>
  <el-container class="layout-container">
    <el-aside class="layout-sider" :width="`${configStore.sidebarWidth}px`">
      <side-bar />
    </el-aside>
    <el-container>
      <router-view />
    </el-container>
  </el-container>
</template>

<script setup>
import { SideBar } from '~/layout/components'

import useConfigStore from '~/store/config'
const configStore = useConfigStore()


</script>

<style lang="scss">
@import "index.scss";
</style>
