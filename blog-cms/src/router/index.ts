import { createRouter, createWebHashHistory, type RouteRecordRaw } from "vue-router";

export const constantRoutes: RouteRecordRaw[]=[
    {
    path: "/:pathMatch(.*)*",
    component: () => import("@/views/error/404.vue"),
    meta: {
      hidden: true,
    },
  },
];

const router = createRouter({
  history: createWebHashHistory(),
  routes: constantRoutes,
});

export default router;
