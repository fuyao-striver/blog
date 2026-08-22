import axios, { AxiosError, type AxiosResponse } from "axios";
import { getToken, token_prefix } from "./token";
import { messageConfirm } from "./modal";

// 是否显示重新登录
export let isRelogin = { show: false };

const requests = axios.create({
  baseURL: "/api",
  timeout: 10000,
  // 请求头
  headers: {
    "Content-Type": "application/json;charset=UTF-8",
  },
});

// 请求拦截器
requests.interceptors.request.use(
  (config) => {
    // 请求带token
    if (getToken()) {
      config.headers["Authorization"] = token_prefix + getToken();
    }
    return config;
  },
  (error) => {
    return Promise.reject(error);
  },
);

// 配置响应拦截器
requests.interceptors.response.use(
  (response: AxiosResponse) => {
    switch (response.data.code) {
      case 400:
        ElNotification({
          title: "失败",
          message: response.data.msg,
          type: "error",
        });
        break;
      case 402:
        const { user } = useStore();
        if (!isRelogin.show) {
          isRelogin.show = true;
          messageConfirm("登录状态已过期，您可以继续留在该页面，或者重新登录")
            .then(() => {
              isRelogin.show = false;
              user.LogOut().then(() => {
                location.href = "/login";
              });
            })
            .catch(() => {
              isRelogin.show = false;
            });
        }
        break;
      case 500:
        ElNotification({
          title: "失败",
          message: response.data.msg,
          type: "error",
        });
        break;
    }
    return response;
  },
  (error: AxiosError) => {
    ElMessage({
      message: error.response?.data.msg,
      type: "error",
      duration: 5 * 1000,
    });
    return Promise.reject(error);
  },
);

// 对外暴露
export default requests;
