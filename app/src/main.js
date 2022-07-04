import { createApp } from 'vue'
import { createPinia } from 'pinia'

import '~/styles/index.scss'

import router from './router'
import app from './app.vue'

createApp(app)
  .use(router)
  .use(createPinia())
  .mount('#app')
