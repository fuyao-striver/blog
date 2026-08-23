import { createRouter, createWebHistory, type RouteRecordRaw } from "vue-router";
import Layouts from "@/layouts/index.vue";

const constantRouters: RouteRecordRaw[] = [
  {
    path: "/:pathMatch(.*)*",
    component: () => import("@/views/error/404.vue"),
    meta: {
      hidden: true,
    },
  },
  {
    path: "/login",
    name: "登录",
    component: () => import("@/views/login/index.vue"),
    meta: {
      hidden: true,
    },
  },
  {
    path: "",
    component: Layouts,
  },
];

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: constantRouters,
});

export default router;
