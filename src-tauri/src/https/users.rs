use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub avatar: String,
    pub username: String,
    pub nickname: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    #[serde(rename = "accessToken")]
    pub access_token: String,
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
    pub expires: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub data: Option<UserInfo>,
}

#[tauri::command]
pub fn login(request: LoginRequest) -> LoginResponse {
    if request.username == "admin" && request.password == "admin123" {
        LoginResponse {
            success: true,
            data: Some(UserInfo {
                avatar: "https://avatars.githubusercontent.com/u/44761321".to_string(),
                username: "admin".to_string(),
                nickname: "小铭".to_string(),
                roles: vec!["admin".to_string()],
                permissions: vec!["*:*:*".to_string()],
                access_token: "eyJhbGciOiJIUzUxMiJ9.admin".to_string(),
                refresh_token: "eyJhbGciOiJIUzUxMiJ9.adminRefresh".to_string(),
                expires: "2030/10/30 00:00:00".to_string(),
            }),
        }
    } else {
        LoginResponse {
            success: true,
            data: Some(UserInfo {
                avatar: "https://avatars.githubusercontent.com/u/52823142".to_string(),
                username: "common".to_string(),
                nickname: "小林".to_string(),
                roles: vec!["common".to_string()],
                permissions: vec![
                    "permission:btn:add".to_string(),
                    "permission:btn:edit".to_string(),
                ],
                access_token: "eyJhbGciOiJIUzUxMiJ9.common".to_string(),
                refresh_token: "eyJhbGciOiJIUzUxMiJ9.commonRefresh".to_string(),
                expires: "2030/10/30 00:00:00".to_string(),
            }),
        }
    }
}
