import type { Result } from "@/modal";
import { type AxiosPromise } from "axios";
import type {Password, UserInfo} from "@/api/user/types";
import request from "@/utils/request.ts";

/**
 * 获取用户信息
 * @returns 用户信息
 */
export const getUserInfo = (): AxiosPromise<Result<UserInfo>> => {
  return request({
    url: "/admin/user/getUserInfo",
    method: "get",
  });
};

/**
 * 修改管理员密码
 * @param data 密码
 */
export const updateAdminPassword = (data: Password) =>{
  return request({
    url: "/admin/password",
    method: "put",
    data,
  });
}
