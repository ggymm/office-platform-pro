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
import { reactive } from 'vue'
import { useRouter } from 'vue-router'

const menuList = reactive([
  { icon: 'icons8-topic', title: '消息', router: '/message', checked: true },
  { icon: 'icons8-folder', title: '文件', router: '/file', checked: false },
  { icon: 'icons8-snippet', title: '代码', router: '/snippet', checked: false },
  { icon: 'icons8-document', title: '文档', router: '/document', checked: false },
  { icon: 'icons8-app', title: '应用', router: '/app', checked: false },
  { icon: 'icons8-contact', title: '通讯录', router: '/contact', checked: false }
])

const router = useRouter()
// const route = useRoute()

function getIcon(index) {
  const menu = menuList[index]
  return new URL(`/src/assets/icons/${menu.icon}${menu.checked ? '-checked' : ''}.png`, import.meta.url).href
}

function goRouter(index) {
  for (let i = 0; i < menuList.length; i++) {
    menuList[i].checked = index === i;
  }
  console.log(menuList[index].router)
  router.push({ path: menuList[index].router })
}

</script>
