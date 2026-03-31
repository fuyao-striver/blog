import request from "@/utils/request";
import type { Password } from "./types";

/**
 * 修改管理员密码
 * @param data 密码
 */
export function updateAdminPassword(data: Password) {
  return request({
    url: "/admin/password",
    method: "put",
    data,
  });
}
