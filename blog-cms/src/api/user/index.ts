import request from "@/utils/request";
import type { Password, UserInfo } from "./types";
import type { AxiosPromise } from "axios";
import type { Result } from "@/model";

/**
 * 修改管理员密码
 * @param data 密码
 */
export const updateAdminPassword = (data: Password) => {
  return request({
    url: "/admin/password",
    method: "put",
    data,
  });
}

/**
 * 获取用户信息
 * @returns 用户信息
 */
export const getUserInfo = (): AxiosPromise<Result<UserInfo>> => {
  return request({
    url: "/admin/user/getUserInfo",
    method: "get",
  });
}
