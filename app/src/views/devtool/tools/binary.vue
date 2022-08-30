<template>
  <div class="common-container">
    <div class="common-header">
      <back />
    </div>
    <div class="common-body">
      <div class="input">
        <n-form v-model="input" label-placement="left">
          <n-form-item>
            <n-input v-model:value="input.num" type="text" placeholder="" />
          </n-form-item>
          <n-form-item>
            <n-radio-group v-model:value="input.base">
              <n-space>
                <n-radio v-for="type in types" :key="type.value" :value="type.value">
                  {{ type.label }}
                </n-radio>
              </n-space>
            </n-radio-group>
          </n-form-item>
          <n-form-item style="justify-items: center">
            <n-button type="primary" style="width: 180px" @click="handleConvert">转换</n-button>
          </n-form-item>
        </n-form>
      </div>
      <div class="output">
        <n-data-table :columns="columns" :data="data" :single-line="false" />
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from "vue"
import { Back } from '@comps/fragment'
import { invoke } from '@tauri-apps/api'

const input = ref({
  num: '10',
  base: 2
})

const types = [
  { value: 2, label: '2进制' },
  { value: 8, label: '8进制' },
  { value: 10, label: '10进制' },
  { value: 16, label: '16进制' }
]

const columns = [
  {
    title: '进制',
    key: 'base',
    width: 240,
    render: (val, _) => {
      return `${val.base} 进制`
    }
  },
  { title: '值', key: 'num' }
]
const data = ref([
  { base: 2, num: 10 },
  { base: 8, num: 2 },
  { base: 10, num: 2 },
  { base: 16, num: 2 }
])

const handleConvert = () => {
  invoke('plugin:devtool|number_base_convert', {
    'num': input.value.num,
    'numBase': input.value.base
  }).then((resp) => {
    if (resp.success) {
      const resData = []
      for (let k in resp.data) {
        resData.push({
          num: resp.data[k],
          base: k
        })
      }
      data.value = resData
    } else {
      window['$notification']['error']({
        content: "解析失败", meta: resp.message,
        duration: 2500, keepAliveOnHover: true
      })
    }
  })
}

</script>

<style lang="scss">
@import '../assets/common';

.common-body {
  .input {
    margin-top: 20px
  }
}
</style>
