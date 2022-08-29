import { createRouter, createWebHistory } from 'vue-router'

import { Blank, Layout } from '~/layout'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      redirect: '/message'
    },
    {
      path: '/message',
      component: Layout,
      children: [
        {
          path: '',
          component: () => import('~/views/message/index.vue')
        }
      ]
    },
    {
      path: '/devtool',
      component: Layout,
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
          path: 'binary',
          component: () => import('~/views/devtool/tools/binary.vue')
        },

        {
          path: 'record',
          component: () => import('~/views/devtool/tools/record.vue')
        },

        {
          path: 'format',
          component: () => import('~/views/devtool/tools/format.vue')
        },
        {
          path: 'json',
          component: () => import('~/views/devtool/tools/json.vue')
        },

        {
          path: 'color',
          component: () => import('~/views/devtool/tools/color.vue')
        },
        {
          path: 'cursor',
          component: () => import('~/views/devtool/tools/cursor.vue')
        },
        {
          path: 'keycode',
          component: () => import('~/views/devtool/tools/keycode.vue')
        },

        {
          path: 'image/extractor',
          component: () => import('~/views/devtool/tools/image/extractor.vue')
        }
      ]
    },
    {
      path: '/debug',
      component: Layout,
      children: [
        {
          path: '',
          component: () => import('~/views/debug/index.vue')
        }
      ]
    },
    {
      path: '/blank',
      component: Blank,
      children: []
    }
  ]
})

export function createRouteGuard(router) {
  router.beforeEach(async() => {
    const loading = window['$loading'] || null
    loading && loading.start()
  })

  router.afterEach(() => {
    const loading = window['$loading'] || null
    loading && loading.finish()
  })
}

export function setupRouter(app) {
  app.use(router)
  createRouteGuard(router)
}
