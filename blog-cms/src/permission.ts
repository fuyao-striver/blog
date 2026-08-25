import nProgress from "nprogress";
import router from "./router";
import useStore from "./store";
import { getToken } from "./utils/token";
import { isRelogin } from "./utils/request";

nProgress.configure({
  easing: "ease",
  speed: 500,
  showSpinner: true,
  trickleSpeed: 200,
  minimum: 0.3,
});

// 白名单路由
const whiteList = ["/login"];

// 路由前卫
router.beforeEach((to, _from, next) => {
  nProgress.start();
  const { user, permission } = useStore();
  // 判断是否有token
  if (getToken()) {
    if (to.path === "/login") {
      nProgress.done();
      return "/";
    } else {
      if (user.roleList.length === 0) {
        isRelogin.show = false;
        // 如果没有拉取用户信息，则拉取用户信息
        user.GetInfo().then(() => {
          isRelogin.show = false;
          permission.generateRoutes().then((accessRoutes) => {
            accessRoutes.forEach((route) => {
              router.addRoute(route)
            });
            next({ ...to, replace: true })
          })
        });
      }
    }
  } else {
    // 未登录可以访问白名单
    if (whiteList.indexOf(to.path) !== -1) {
      return;
    } else {
      nProgress.done();
      return `/login?redirect=${to.path}`;
    }
  }
});

router.afterEach(() => {
  nProgress.done();
});
