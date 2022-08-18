import { createApp } from 'vue'
import { createPinia } from 'pinia'

import '~/styles/index.scss'

import { setupRouter } from './router'
import App from './app.vue'

import VueDOMPurifyHTML from 'vue-dompurify-html'

const app = createApp(App)

await setupRouter(app)
app.use(VueDOMPurifyHTML)
app.use(createPinia())
app.mount('#app')
