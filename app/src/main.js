import { createApp } from 'vue'
import { createPinia } from 'pinia'

import '~/styles/index.scss'

import router from './router'
import App from './app.vue'

import VueDOMPurifyHTML from 'vue-dompurify-html'

const app = createApp(App)
app.use(VueDOMPurifyHTML)
app.use(router).use(createPinia()).mount('#app')
