<template>
  <div class="sider-container">
    <div class="menu-container">
      <div v-for="(menu,i) in menuList" :key="i" class="menu-item" @click="goRouter(i)">
        <div class="menu-layout" :class="menu.checked ? 'checked' : ''">
          <img class="icon" :src="getIcon(i)" :alt="menu.title">
          <span class="title">{{ menu.title }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { reactive, onMounted } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()

const menuList = reactive([
  { icon: 'icons8-topic', title: '消息', router: '/message', checked: true },
  { icon: 'icons8-folder', title: '文件', router: '', checked: false },
  { icon: 'icons8-document', title: '文档', router: '', checked: false },
  { icon: 'icons8-devtool', title: '工具', router: '/devtool', checked: false },
  { icon: 'icons8-app', title: '应用', router: '', checked: false },
  { icon: 'icons8-debug', title: '调试', router: '/debug', checked: false }
])

onMounted(() => {
  // 初始化选择路由对应的图标
  const routerPath = window.location.pathname
  if (routerPath === '/') {
    menuList[0].checked = true
    return
  }
  for (let i = 0; i < menuList.length; i++) {
    const path = menuList[i].router
    if (path) {
      menuList[i].checked = routerPath.startsWith(path)
    }
  }
})

function getIcon(index) {
  const menu = menuList[index]
  return new URL(`/src/assets/icons/${menu.icon}${menu.checked ? '-checked' : ''}.png`, import.meta.url).href
}

function goRouter(index) {
  for (let i = 0; i < menuList.length; i++) {
    menuList[i].checked = index === i
  }
  const path = menuList[index].router
  if (path) {
    router.push({ path: path })
  }
}

</script>
