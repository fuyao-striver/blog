import request from "@/utils/request.ts";
import type { LoginForm } from "./types";
import type { AxiosPromise } from "axios";
import type { Result } from "@/modal";

/**
 * 用户登录
 * @param data 登录信息
 * @returns Token
 */
export const login = (data: LoginForm): AxiosPromise<Result<string>> => {
  return request({
    url: "/login",
    method: "post",
    data,
  });
};

/**
 * 用户退出
 */
export const logout = (): AxiosPromise<Result<null>> => {
  return request({
    url: "/logout",
    method: "get",
  });
};
