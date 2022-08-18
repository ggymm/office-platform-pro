import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      redirect: '/message'
    },
    {
      path: '/message',
      component: () => import('~/views/message/index.vue')
    },
    {
      path: '/devtool',
      children: [
        {
          path: '',
          component: () => import('~/views/devtool/index.vue')
        },
        {
          path: 'base64',
          component: () => import('~/views/devtool/tools/base64.vue')
        },
        {
          path: 'url',
          component: () => import('~/views/devtool/tools/url.vue')
        },
        {
          path: 'json-format',
          component: () => import('~/views/devtool/tools/json-format.vue')
        },
        {
          path: 'css-cursor',
          component: () => import('~/views/devtool/tools/cursor.vue')
        },
        {
          path: 'keycode',
          component: () => import('~/views/devtool/tools/keycode.vue')
        }
      ]
    },
    {
      path: '/debug',
      component: () => import('~/views/debug/index.vue')
    }
  ]
})

export function createRouteGuard(router) {
  router.beforeEach(async () => {
    const loading = window['$loading'] || null
    loading && loading.start()
  })

  router.afterEach(() => {
    const loading = window['$loading'] || null
    loading && loading.finish()
  })
}

export async function setupRouter(app) {
  app.use(router)
  createRouteGuard(router)
}

export default router
