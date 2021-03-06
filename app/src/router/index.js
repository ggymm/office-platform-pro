import { createRouter, createWebHashHistory } from 'vue-router'

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    {
      path: '/',
      redirect: '/message'
    },
    {
      path: '/message',
      name: 'Message',
      component: () => import('~/views/message/index.vue')
    },
    {
      path: '/snippet',
      name: 'CodeSnippet',
      component: () => import('~/views/snippet/index.vue')
    }
  ]
})

router.beforeEach(async (to, from, next) => {
  next()
})

export default router
