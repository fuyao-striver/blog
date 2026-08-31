import "@/assets/styles/index.scss";

import { createApp } from "vue";
import { createPinia } from "pinia";

import App from "./App.vue";
import router from "./router";
import RightToolBar from "@/components/RightToolBar/index.vue";
// 自定义svg插件
import SvgIcon from "@/components/SvgIcon/index.vue";
import "virtual:svg-icons-register";

// 引入md
import { MdEditor } from "md-editor-v3";
import "md-editor-v3/lib/style.css";

import "@/permission";

const app = createApp(App);

app.use(createPinia());
app.use(router);
app.component("svg-icon", SvgIcon);
app.component("md-editor", MdEditor);
app.component("RightToolBar", RightToolBar);
app.mount("#app");
