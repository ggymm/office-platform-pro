<template>
  <div class="keycode-container">
    <div class="keycode-header">
      <back />
    </div>
    <div class="keycode-body">
      键位编码信息
    </div>
  </div>
</template>

<script setup>
import { Back } from '@comps/fragment'

</script>

<style lang="scss">
.keycode-container {
  width: calc(100vw - 72px);
  height: 100vh;
  display: flex;
  flex-direction: column;

  .keycode-header {
    height: 48px;
    min-height: 48px;
    padding: 0 20px;
    display: flex;
    align-items: center;
  }

  .keycode-body {

  }
}
</style>
