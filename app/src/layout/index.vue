<template>
  <n-layout class="layout-container" has-sider>
    <n-layout-sider class="layout-sider" :width="72" :native-scrollbar="false">
      <side-bar />
    </n-layout-sider>
    <n-layout-content class="layout-content">
      <router-view />
    </n-layout-content>
  </n-layout>
</template>

<script setup>
import { SideBar } from '~/layout/components'

import useConfigStore from '~/store/config'
const configStore = useConfigStore()

</script>

<style lang="scss">
@import "index.scss";
</style>
