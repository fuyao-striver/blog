import type { Result } from "@/modal";
import type { AxiosPromise } from "axios";
import type { UserInfo } from "./type";
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
