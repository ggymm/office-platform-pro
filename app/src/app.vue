<template>
  <div class="app-container">
    <n-config-provider :theme-overrides="themeOverrides">
      <n-loading-bar-provider>
        <n-notification-provider :max="3">
          <n-message-provider :max="3">
            <router-view />
          </n-message-provider>
        </n-notification-provider>
      </n-loading-bar-provider>
    </n-config-provider>
  </div>
</template>

<script setup>
import { computed } from 'vue'
import { NLoadingBarProvider, NNotificationProvider, NMessageProvider } from 'naive-ui'

import { lighten } from '~/utils'

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
