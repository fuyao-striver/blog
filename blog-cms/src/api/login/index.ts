import requests from "@/utils/request";
import type { LoginForm } from "./type";
import type { AxiosPromise } from "axios";
import type { Result } from "@/modal";

/**
 * 用户登录请求
 *
 * @param data 用户登录信息
 * @returns token
 */
export const login = (data: LoginForm): AxiosPromise<Result<string>> => {
  return requests({
    url: "/login",
    method: "post",
    data,
  });
};

/**
 * 用户退出
 */
export const logout = (): AxiosPromise<Result<null>> => {
  return requests({
    url: "/logout",
    method: "GET",
  });
};
