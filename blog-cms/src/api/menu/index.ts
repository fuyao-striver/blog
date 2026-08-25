import type { Result } from "@/modal";
import requests from "@/utils/request";
import type { AxiosPromise } from "axios";
import type { RouteRecordRaw } from "vue-router";


/**
 * 获取登录用户菜单
 * @returns 登录用户菜单
 */
export const getUserMenu = (): AxiosPromise<Result<RouteRecordRaw[]>> => {
  return requests({
    url: "/admin/user/getUserMenu",
    method: "GET"
  })
} 
