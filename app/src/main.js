import { createApp } from 'vue'
import { createPinia } from 'pinia'

import '~/styles/index.scss'

import router from './router'
import App from './app.vue'

import * as Icons from "@element-plus/icons-vue"


const app = createApp(App)
Object.keys(Icons).forEach(key => {
  app.component(key, Icons[key])
})

app.use(router).use(createPinia()).mount('#app')
