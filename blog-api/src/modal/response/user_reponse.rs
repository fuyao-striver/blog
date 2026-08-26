use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserBackInfo {
    // 用户id
    pub id: i32,
    // 用户头像
    pub avatar: String,
    // 用户角色
    pub role_list: Vec<String>,
    // 用户权限
    pub permission_list: Vec<String>,
}

/**
 * 用户菜单Response
 *
 **/
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserMenu {
    /**
     * 菜单id
     */
    pub id: i32,

    /**
     * 父菜单id
     */
    pub parent_id: i32,

    /**
     * 菜单名称
     */
    pub menu_name: String,

    /**
     * 类型（M目录 C菜单 B按钮）
     */
    pub menu_type: String,

    /**
     * 路由地址
     */
    pub path: Option<String>,

    /**
     * 菜单图标
     */
    pub icon: Option<String>,

    /**
     * 菜单组件
     */
    pub component: Option<String>,

    /**
     * 是否隐藏 (0否 1是)
     */
    pub is_hidden: i8,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
/// 路由菜单
pub struct RouterResp {
    /// 菜单名称
    pub name: String,
    /// 菜单路径
    pub path: Option<String>,
    /// 菜单组件
    pub component: Option<String>,
    /// 是否显示
    pub always_show: Option<bool>,
    /// 从定向地址
    pub redirect: Option<String>,
    /// 菜单信息
    pub meta: Option<MetaResp>,
    /// 子菜单
    pub children: Option<Vec<RouterResp>>,
}

#[derive(Debug, Serialize)]
pub struct MetaResp {
    /// 菜单图标
    pub title: String,
    /// 菜单图标
    pub icon: Option<String>,
    /// 是否隐藏
    pub hidden: bool,
}
