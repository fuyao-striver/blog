import NProgress from "nprogress";
import router from "@/router";
import useStore from "@/stores";
import { getToken } from "@/utils/token.ts";
import { isRelogin } from "@/utils/request.ts";

NProgress.configure({
  easing: "ease",
  speed: 500,
  showSpinner: false,
  trickleSpeed: 200,
  minimum: 0.3,
});

// 白名单路由
const whiteList = ["/login"];

router.beforeEach(async (to, _from) => {
  NProgress.start();
  const { user, permission } = useStore();

  // 判断是否有token
  if (getToken()) {
    if (to.path === "/login") {
      NProgress.done();
      return { path: "/" }; // 已登录，跳转首页
    }

    if (user.roleList.length === 0) {
      isRelogin.show = true;
      try {
        // 获取用户信息
        await user.GetInfo();
        isRelogin.show = false;

        // 生成动态路由
        const accessRoutes = await permission.generateRoutes();
        accessRoutes.forEach((route) => {
          router.addRoute(route);
        });

        // 重试当前导航
        return { ...to, replace: true };
      } catch (err) {
        // 获取信息失败，登出
        await user.LogOut();
        ElMessage.error(err);
        return { path: "/login" };
      }
    }

    // 已有角色信息，直接放行
    return true;
  }

  // 未登录
  if (whiteList.indexOf(to.path) !== -1) {
    return true; // 白名单放行
  }

  NProgress.done();
  return `/login?redirect=${to.path}`; // 重定向到登录页
});

router.afterEach(() => {
  NProgress.done();
});
