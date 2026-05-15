// 常量定义
pub mod common_constant {
    /// 根节点 parent_id
    pub const PARENT_ID: i32 = 0;
    /// DB 中 menu_type = 'M'，表示菜单目录（可包含子节点）
    pub const MENU_TYPE_DIR: &str = "M";
    /// DB 中 menu_type = 'C'，表示具体页面/组件
    pub const MENU_TYPE_PAGE: &str = "C";
    /// 前端 Layout 组件名
    pub const LAYOUT: &str = "Layout";
    /// 前端 ParentView 组件名
    pub const PARENT_VIEW: &str = "ParentView";
}
