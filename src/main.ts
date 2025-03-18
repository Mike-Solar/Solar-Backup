import { createApp } from "vue";
import {createRouter, createWebHashHistory} from "vue-router";
import App from "./App.vue";
import Home from"./Home.vue"
import PlanHome from './plan/Home.vue'
import BackupHome from './backup/Home.vue'
import { createI18n } from 'vue-i18n'
// 语言包
import zhCn from './i18n/zh-cn'

const i18n = createI18n({
    legacy: false, // 设置为 false，启用 composition API 模式
    locale: sessionStorage.getItem('localeLang') || 'zhCn',
    messages: {
        zhCn,
    },
});
const routes = [
    { path: '/', component: Home },
    { path: '/plan/home', component: PlanHome},
    { path: '/backup/home', component: BackupHome}
  ]
const router=createRouter({
    routes: routes,
    history: createWebHashHistory()
})
createApp(App).use(i18n).use(router).mount("#app");
