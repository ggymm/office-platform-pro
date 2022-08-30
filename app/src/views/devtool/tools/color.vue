<template>
  <div class="common-container">
    <div class="common-header">
      <back />
    </div>
    <div class="common-body color-body">
      <n-form v-model="colorVal" label-placement="left" label-width="60">
        <n-form-item label="颜色选择">
          <n-color-picker
            :to="false"
            :show="true"
            :modes="['rgb']"
            :show-preview="true"
            :on-update:value="handleChange"
            placement="bottom-end"
          />
        </n-form-item>
        <n-form-item label="HEX">
          <n-input v-model:value="colorVal.hex" type="text" placeholder="" readonly="true" />
        </n-form-item>
        <n-form-item label="HEX8">
          <n-input v-model:value="colorVal.hex8" type="text" placeholder="" readonly="true" />
        </n-form-item>
        <n-form-item label="RGB">
          <n-input v-model:value="colorVal.rgb" type="text" placeholder="" readonly="true" />
        </n-form-item>
        <n-form-item label="HSV">
          <n-input v-model:value="colorVal.hsv" type="text" placeholder="" readonly="true" />
        </n-form-item>
        <n-form-item label="HSL">
          <n-input v-model:value="colorVal.hsl" type="text" placeholder="" readonly="true" />
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

  // 格式化颜色
  colorVal.value.hex = color.toHexString()
  colorVal.value.hex8 = color.toHex8String()
  colorVal.value.rgb = color.toRgbString()
  colorVal.value.hsv = color.toHsvString()
  colorVal.value.hsl = color.toHslString()
}
</script>

<style lang="scss">
@import '../assets/common';

.color-body {
  display: flex;
  flex-direction: column;

  .n-form {
    margin-top: 8px;

    .n-input {
      width: 60%;
    }
  }
}
</style>
