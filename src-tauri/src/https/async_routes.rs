use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "showLink")]
    pub show_link: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RouteConfig {
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
    pub meta: Meta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<RouteConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<String>,
}

#[tauri::command]
pub fn get_async_routes() -> Vec<RouteConfig> {
    let permission_router = RouteConfig {
        path: "/permission".to_string(),
        name: Some("PermissionRoot".to_string()),
        component: Some("Layout".to_string()),
        meta: Meta {
            title: "设备管理".to_string(),
            icon: Some("ep:lollipop".to_string()),
            rank: Some(10),
            show_link: Some(true),
        },
        redirect: None,
        children: Some(vec![
            RouteConfig {
                path: "/permission/page".to_string(),
                name: Some("PermissionPage".to_string()),
                component: Some("src/views/permission/page/index.vue".to_string()),
                meta: Meta {
                    title: "页面权限".to_string(),
                    icon: None,
                    rank: None,
                    show_link: Some(true),
                },
                redirect: None,
                children: None,
            },
            RouteConfig {
                path: "/permission/button".to_string(),
                name: Some("PermissionButton".to_string()),
                component: Some("src/views/permission/button/index.vue".to_string()),
                meta: Meta {
                    title: "按钮权限".to_string(),
                    icon: None,
                    rank: None,
                    show_link: Some(true),
                },
                redirect: None,
                children: None,
            },
        ]),
    };

    vec![permission_router]
}
