<template>
  <el-container class="layout-container">
    <el-aside class="layout-sider" width="72px">
      <side-bar />
    </el-aside>
    <el-container>
      <el-main>
        <router-view />
      </el-main>
    </el-container>
  </el-container>
</template>

<script setup>
import { SideBar } from '~/layout/components'


</script>

<style lang="scss">
@import "index.scss";
</style>
