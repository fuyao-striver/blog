/**
 * 用户菜单
 */
export interface UserMenu {
  /**
   * 菜单id
   */
  id: number;
  /**
   * 菜单名
   */
  menuName: string;
  /**
   * 菜单路径
   */
  path: string;
  /**
   * 菜单图标
   */
  icon: string;
  /**
   * 菜单路径
   */
  component: string;
  /**
   * 是否隐藏（0 否 1是）
   */
  isHidden: number;
}
