import { createApp } from "vue";
import { createWebHashHistory } from "vue-router";
import App from "./App.vue";
import Home from"./Home.vue"
import PlanHome from './plan/Home.vue'
import BackupHome from './backup/Home.vue'
const routes = [
    { path: '/', component: Home },
    { path: '/plan/home', component: PlanHome},
    { path: '/backup/home', component: BackupHome}
  ]
const router={
    routes: routes,
    history: createWebHashHistory()
}
createApp(App).use(router).mount("#app");
