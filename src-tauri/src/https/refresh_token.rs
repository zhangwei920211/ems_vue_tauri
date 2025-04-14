use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    pub success: bool,
    pub data: Option<TokenResponse>,
}

#[tauri::command]
pub fn refresh_token(request: RefreshTokenRequest) -> ApiResponse {
    // 在实际应用中，你需要：
    // 1. 验证 refresh_token 的有效性
    // 2. 检查 refresh_token 是否过期
    // 3. 生成新的 access_token 和 refresh_token
    // 4. 更新数据库中的 token 信息

    if !request.refresh_token.is_empty() {
        // 生成新的过期时间（这里设置为30天后）
        let expires = Utc::now() + Duration::days(30);

        ApiResponse {
            success: true,
            data: Some(TokenResponse {
                access_token: "eyJhbGciOiJIUzUxMiJ9.newAdmin".to_string(),
                refresh_token: "eyJhbGciOiJIUzUxMiJ9.newAdminRefresh".to_string(),
                expires: expires.format("%Y/%m/%d %H:%M:%S").to_string(),
            }),
        }
    } else {
        ApiResponse {
            success: false,
            data: None,
        }
    }
}
