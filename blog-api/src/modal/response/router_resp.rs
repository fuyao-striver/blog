use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RouterResp {
    pub name: String,
    pub path: String,
    pub component: String,
    pub always_show: Option<bool>,
    pub redirect: Option<String>,
    pub meta: Option<MetaResp>,
    pub children: Option<Vec<RouterResp>>,
}

#[derive(Debug, Serialize)]
pub struct MetaResp {
    pub title: String,
    pub icon: Option<String>,
    pub hidden: bool,
}
