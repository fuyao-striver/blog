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
router.beforeEach((to, from, next) => {
  nProgress.start();
  const { user } = useStore();
  // 判断是否有token
  if (getToken()) {
    if (to.path === "/login") {
      next({ path: "/" });
      nProgress.done();
    } else {
      if (user.roleList.length === 0) {
        isRelogin.show = false;
        // 如果没有拉取用户信息，则拉取用户信息
        user.GetInfo().then(() => {
          isRelogin.show = false;
        });
      }
    }
  }
});

router.afterEach(() => {
  nProgress.done();
});
