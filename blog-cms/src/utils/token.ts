import Cookies from "js-cookie";

// token前缀
export const token_prefix = "Bearer ";

const TokenKey = "Token";

// 获取token
export const getToken = () => {
  return Cookies.get(TokenKey);
};
// 设置token
export const setToken = (token: string) => {
  Cookies.set(TokenKey, token);
};
// 移除token
export const removeToken = () => {
  Cookies.remove(TokenKey);
};
