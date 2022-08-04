import { defineStore } from "pinia"


const useConfigStore = defineStore('configStore', {
  state: () => {
    return {
      // 左侧菜单宽度
      sidebarWidth: 72
    }
  },
  actions: {},
  getters: {}
})
export default useConfigStore
