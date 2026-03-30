import type { RouteLocationNormalized, RouteRecordRaw } from "vue-router";

/**
 * 用户
 */
export interface UserState {
  /**
   * 用户id
   */
  id: number | null;
  /**
   * 用户头像
   */
  avatar: string;
  /**
   * 角色
   */
  roleList: string[];
  /**
   * 权限
   */
  permissionList: string[];
}


/**
 * 设置
 */
export interface SettingState {
  /**
   * 是否显示 tagView
   */
  tagView: boolean;
  /**
   * 是否固定头部
   */
  fixedHeader: boolean;
  /**
   * 是否显示Logo
   */
  sidebarLogo: boolean;
}

export interface TagView extends Partial<RouteLocationNormalized> {
  title?: string;
}

export interface TagViewState {
  visitedViews: TagView[];
}

/**
 * 权限
 */
export interface PermissionState {
  /**
   * 路由
   */
  routes: RouteRecordRaw[];
}

/**
 * 应用
 */
export interface AppState {
  /**
   * 侧边栏是否展开
   */
  isCollapse: boolean;
  /**
   * 响应式
   */
  device: string;
  /**
   * 大小
   */
  size: string;
}

