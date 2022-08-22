<template>
  <div class="color-container">
    <div class="color-header">
      <back />
    </div>
    <div class="color-body">
      <n-form v-model="colorVal" label-placement="left" label-width="60">
        <n-form-item label="颜色选择">
          <n-color-picker
            :show="true"
            :show-preview="true"
            :to="false"
            :modes="['hex']"
            :on-update:value="handleChange"
            placement="bottom-end"
          />
        </n-form-item>
        <n-form-item label="HEX">
          <n-input v-model:value="colorVal.hex" type="text" placeholder="hex" />
        </n-form-item>
        <n-form-item label="HEX8">
          <n-input v-model:value="colorVal.hex8" type="text" placeholder="hex8" />
        </n-form-item>
        <n-form-item label="RGB">
          <n-input v-model:value="colorVal.rgb" type="text" placeholder="rgb" />
        </n-form-item>
        <n-form-item label="HSV">
          <n-input v-model:value="colorVal.hsv" type="text" placeholder="hsv" />
        </n-form-item>
        <n-form-item label="HSL">
          <n-input v-model:value="colorVal.hsl" type="text" placeholder="hsl" />
        </n-form-item>
      </n-form>
    </div>
  </div>
</template>

<script setup>
import { ref } from "vue"
import tinycolor from 'tinycolor2'

import { Back } from '@comps/fragment'

const colorVal = ref({
  hex: '',
  hex8: '',
  rgb: '',
  hsv: '',
  hsl: ''
})

const handleChange = (val) => {
  const color = tinycolor(val)
  colorVal.value.hex = color.toHexString()
  colorVal.value.hex8 = color.toHex8String()

  colorVal.value.rgb = color.toRgbString()
  colorVal.value.hsv = color.toHsvString()
  colorVal.value.hsl = color.toHslString()
}
</script>

<style lang="scss">
.color-container {
  width: calc(100vw - 72px);
  height: 100vh;
  display: flex;
  flex-direction: column;

  .color-header {
    height: 48px;
    min-height: 48px;
    padding: 0 20px;
    display: flex;
    align-items: center;
  }

  .color-body {
    display: flex;
    flex-direction: column;
    height: calc(100vh - 72px);
    padding: 0 20px 24px 20px;

    .n-form {
      margin-top: 8px;

      .n-input {
        width: 60%;
      }
    }
  }
}
</style>
