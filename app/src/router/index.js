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
      path: '/dev-box',
      name: 'DevBox',
      component: () => import('~/views/dev-box/index.vue')
    }
  ]
})

router.beforeEach(async (to, from, next) => {
  next()
})

export default router
