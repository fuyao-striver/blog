import type { Result } from "@/modal";
import type { AxiosPromise } from "axios";
import type { Password, UserInfo } from "./type";
import requests from "@/utils/request";

/**
 * 获取用户信息
 * @returns 用户信息
 */
export const getUserInfo = (): AxiosPromise<Result<UserInfo>> => {
  return requests({
    url: "/admin/user/getUserInfo",
    method: "GET",
  });
};

/**
 * 修改管理员密码
 * @param data 密码信息
 */
export const updateAdminPassword = (data: Password): AxiosPromise<Result<null>> => {
  return requests({
    url: "/admin/user/password",
    method: "POST",
    data
  })
}
