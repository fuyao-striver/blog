import type { Result } from "@/modal";
import { type AxiosPromise } from "axios";
import type { UserInfo } from "@/api/user/types";
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
