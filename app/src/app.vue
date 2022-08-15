<template>
  <div class="app-container">
    <n-config-provider :theme-overrides="themeOverrides">
      <n-notification-provider :max="3">
        <n-message-provider :max="3">
          <layout />
        </n-message-provider>
      </n-notification-provider>
    </n-config-provider>
  </div>
</template>

<script setup>
import { computed } from 'vue'
import { NNotificationProvider, NMessageProvider } from 'naive-ui'
import { lighten } from '~/utils'

import Layout from '~/layout/index.vue'

const themeOverrides = computed(() => {
  const appTheme = '#2d8cf0'
  const lightenTheme = lighten(appTheme, 6)
  return {
    common: {
      primaryColor: appTheme,
      primaryColorHover: lightenTheme,
      primaryColorPressed: lightenTheme
    }
  }
})
</script>
