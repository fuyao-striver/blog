import request from "@/utils/request.ts";
import type { AxiosPromise } from "axios";
import type { Result } from "@/modal";
import type { RouteRecordRaw } from "vue-router";

/**
 * 获取登录用户菜单
 * @returns 登录用户菜单
 */
export const getUserMenu = (): AxiosPromise<Result<RouteRecordRaw[]>> => {
  return request({
    url: "/admin/user/getUserMenu",
    method: "get",
  });
};
