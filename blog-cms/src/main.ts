import "@/assets/styles/index.scss";
import { createApp } from "vue";
import { createPinia } from "pinia";

import App from "./App.vue";
import router from "./router";
import SvgIcon from "@/components/SvgIcon/index.vue";
import RightToolbar from "@/components/RightToolBar/index.vue";
import Pagination from "@/components/Pagination/index.vue";
import "virtual:svg-icons-register";
import "@/permission.ts";
import * as ElementPlusIconsVue from "@element-plus/icons-vue";

const app = createApp(App);
for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
  app.component(key, component);
}
app.use(createPinia());
app.use(router);
app.component("svg-icon", SvgIcon);
app.component("RightToolbar", RightToolbar);
app.component("Pagination", Pagination);
app.mount("#app");
