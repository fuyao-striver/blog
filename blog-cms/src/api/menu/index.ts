import type { Result } from "@/model";
import request from "@/utils/request";
import type { AxiosPromise } from "axios";
import type { RouteRecordRaw } from "vue-router";
import type { Menu, MenuForm, MenuOption, MenuQuery, MenuTree } from "./types";

/**
 * 获取登录用户菜单
 * @returns 登录用户菜单
 */
export const getUserMenu = (): AxiosPromise<Result<RouteRecordRaw[]>> => {
  return request({
    url: "/admin/user/getUserMenu",
    method: "get",
  });
};

/**
 * 获取菜单树
 * @returns 菜单树
 */
export const getMenuTree = (): AxiosPromise<Result<MenuTree[]>> => {
  return request({
    url: "/admin/menu/getMenuTree",
    method: "get",
  });
};

/**
 * 获取菜单下拉树
 * @returns 菜单下拉树
 */
export const getMenuOptions = (): AxiosPromise<Result<MenuOption[]>> => {
  return request({
    url: "/admin/menu/getMenuOptions",
    method: "get",
  });
};

/**
 * 查看菜单列表
 * @param params 查询条件
 * @returns 菜单列表
 */
export const getMenuList = (params: MenuQuery): AxiosPromise<Result<Menu[]>> => {
  return request({
    url: "/admin/menu/list",
    method: "get",
    params,
  });
};

/**
 * 编辑菜单
 * @param menuId 菜单id
 * @returns 菜单
 */
export const editMenu = (menuId: number): AxiosPromise<Result<MenuForm>> => {
  return request({
    url: `/admin/menu/edit/${menuId}`,
    method: "get",
  });
};

/**
 * 添加菜单
 * @param data 菜单
 */
export const addMenu = (data: MenuForm): AxiosPromise<Result<null>> => {
  return request({
    url: "/admin/menu/add",
    method: "post",
    data,
  });
};

/**
 * 修改菜单
 * @param data 菜单
 */
export const updateMenu = (data: MenuForm): AxiosPromise<Result<null>> => {
  return request({
    url: "/admin/menu/update",
    method: "put",
    data,
  });
};

/**
 * 删除菜单
 * @param menuId 菜单id
 */
export const deleteMenu = (menuId: number): AxiosPromise<Result<null>> => {
  return request({
    url: `/admin/menu/delete/${menuId}`,
    method: "delete",
  });
};
