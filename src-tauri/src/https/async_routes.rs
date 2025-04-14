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
    let home_router = RouteConfig {
        path: "/".to_string(),
        name: Some("Home".to_string()),
        component: Some("Layout".to_string()),
        meta: Meta {
            title: "首页".to_string(),
            icon: Some("ep:home-filled".to_string()),
            rank: Some(0),
            show_link: Some(true),
        },
        redirect: Some("/welcome".to_string()),
        children: Some(vec![RouteConfig {
            path: "/welcome".to_string(),
            name: Some("Welcome".to_string()),
            component: Some("src/views/welcome/index.vue".to_string()),
            meta: Meta {
                title: "首页".to_string(),
                icon: None,
                rank: None,
                show_link: Some(true),
            },
            redirect: None,
            children: None,
        }]),
    };

    let error_router = RouteConfig {
        path: "/error".to_string(),
        name: Some("Error".to_string()),
        component: Some("Layout".to_string()),
        meta: Meta {
            title: "异常页面".to_string(),
            icon: Some("ep:warning".to_string()),
            rank: Some(9),
            show_link: Some(true),
        },
        redirect: None,
        children: Some(vec![
            RouteConfig {
                path: "/error/403".to_string(),
                name: Some("403".to_string()),
                component: Some("src/views/error/403.vue".to_string()),
                meta: Meta {
                    title: "403".to_string(),
                    icon: None,
                    rank: None,
                    show_link: Some(true),
                },
                redirect: None,
                children: None,
            },
            RouteConfig {
                path: "/error/404".to_string(),
                name: Some("404".to_string()),
                component: Some("src/views/error/404.vue".to_string()),
                meta: Meta {
                    title: "404".to_string(),
                    icon: None,
                    rank: None,
                    show_link: Some(true),
                },
                redirect: None,
                children: None,
            },
            RouteConfig {
                path: "/error/500".to_string(),
                name: Some("500".to_string()),
                component: Some("src/views/error/500.vue".to_string()),
                meta: Meta {
                    title: "500".to_string(),
                    icon: None,
                    rank: None,
                    show_link: Some(true),
                },
                redirect: None,
                children: None,
            },
        ]),
    };

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

    vec![home_router, error_router, permission_router]
}
