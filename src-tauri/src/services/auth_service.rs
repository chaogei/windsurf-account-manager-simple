use crate::utils::{AppError, AppResult};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

const FIREBASE_API_KEY: &str = "AIzaSyDsOl-1XpT5err0Tcnx8FFod1H8gVGIycY";

#[derive(Debug, Serialize, Deserialize)]
pub struct SignInRequest {
    #[serde(rename = "returnSecureToken")]
    return_secure_token: bool,
    email: String,
    password: String,
    #[serde(rename = "clientType")]
    client_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignInResponse {
    #[serde(rename = "idToken")]
    pub id_token: String,
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
    #[serde(rename = "expiresIn")]
    pub expires_in: String,
    #[serde(rename = "localId")]
    pub local_id: String,
    pub email: String,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct RefreshTokenRequest {
    grant_type: String,
    refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenResponse {
    pub access_token: String,
    pub expires_in: String,
    pub token_type: String,
    pub refresh_token: String,
    pub id_token: String,
    pub user_id: String,
    pub project_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfo {
    #[serde(rename = "localId")]
    pub local_id: String,
    pub email: String,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    #[serde(rename = "emailVerified")]
    pub email_verified: bool,
    #[serde(rename = "passwordHash", skip_serializing_if = "Option::is_none")]
    pub password_hash: Option<String>,
    #[serde(rename = "passwordUpdatedAt", skip_serializing_if = "Option::is_none")]
    pub password_updated_at: Option<i64>,
    #[serde(rename = "validSince", skip_serializing_if = "Option::is_none")]
    pub valid_since: Option<String>,
    #[serde(rename = "disabled", default)]
    pub disabled: bool,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "lastLoginAt")]
    pub last_login_at: Option<String>,
    #[serde(rename = "lastRefreshAt", skip_serializing_if = "Option::is_none")]
    pub last_refresh_at: Option<String>,
    #[serde(rename = "providerUserInfo", skip_serializing_if = "Option::is_none")]
    pub provider_user_info: Option<Vec<ProviderInfo>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProviderInfo {
    #[serde(rename = "providerId")]
    pub provider_id: String,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "federatedId", skip_serializing_if = "Option::is_none")]
    pub federated_id: Option<String>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "rawId", skip_serializing_if = "Option::is_none")]
    pub raw_id: Option<String>,
    #[serde(rename = "photoUrl", skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,
}

pub struct AuthService {
    client: Arc<reqwest::Client>,
}

impl AuthService {
    pub fn new() -> Self {
        // 使用专门用于 googleapis 的 HTTP 客户端（支持代理）
        Self {
            client: super::get_google_api_client(),
        }
    }
    
    /// 重新获取客户端（用于代理配置更新后）
    #[allow(dead_code)]
    pub fn refresh_client(&mut self) -> Result<(), String> {
        self.client = super::get_google_api_client();
        Ok(())
    }

    pub async fn sign_in(&self, email: &str, password: &str) -> AppResult<(String, String, DateTime<Utc>)> {
        let url = format!(
            "https://identitytoolkit.googleapis.com/v1/accounts:signInWithPassword?key={}",
            FIREBASE_API_KEY
        );

        let request = SignInRequest {
            return_secure_token: true,
            email: email.to_string(),
            password: password.to_string(),
            client_type: "CLIENT_TYPE_WEB".to_string(),
        };

        let response = match self.client
            .post(&url)
            .json(&request)
            .header("Content-Type", "application/json")
            .header("X-Client-Version", "Chrome/JsCore/11.0.0/FirebaseCore-web")
            .send()
            .await
        {
            Ok(resp) => {
                super::report_request_success();
                resp
            }
            Err(e) => {
                // 检查是否是超时错误
                if e.is_timeout() || e.is_connect() {
                    super::report_timeout_error();
                } else {
                    super::report_request_failure();
                }
                return Err(AppError::Network(e.to_string()));
            }
        };

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            
            // 解析Firebase错误并提供友好提示
            if error_text.contains("TOO_MANY_ATTEMPTS_TRY_LATER") {
                return Err(AppError::AuthFailed("登录尝试次数过多，请15-30分钟后再试".to_string()));
            } else if error_text.contains("INVALID_LOGIN_CREDENTIALS") {
                return Err(AppError::AuthFailed("邮箱或密码错误，请检查后重试".to_string()));
            } else if error_text.contains("EMAIL_NOT_FOUND") {
                return Err(AppError::AuthFailed("该邮箱未注册".to_string()));
            } else if error_text.contains("USER_DISABLED") {
                return Err(AppError::AuthFailed("该账号已被禁用".to_string()));
            }
            
            return Err(AppError::AuthFailed(error_text));
        }

        let sign_in_response: SignInResponse = response.json().await?;
        
        // 计算Token过期时间
        let expires_in_secs: i64 = sign_in_response.expires_in.parse()
            .unwrap_or(3600);
        let expires_at = Utc::now() + Duration::seconds(expires_in_secs);

        Ok((sign_in_response.id_token, sign_in_response.refresh_token, expires_at))
    }

    pub async fn refresh_token(&self, refresh_token: &str) -> AppResult<(String, String, DateTime<Utc>)> {
        let url = format!(
            "https://securetoken.googleapis.com/v1/token?key={}",
            FIREBASE_API_KEY
        );

        let body = format!("grant_type=refresh_token&refresh_token={}", refresh_token);

        let response = match self.client
            .post(&url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Accept", "*/*")
            .header("Accept-Language", "zh-CN,zh;q=0.9")
            .header("Cache-Control", "no-cache")
            .header("Pragma", "no-cache")
            .header("Sec-Ch-Ua", r#""Chromium";v="142", "Google Chrome";v="142", "Not_A Brand";v="99""#)
            .header("Sec-Ch-Ua-Mobile", "?0")
            .header("Sec-Ch-Ua-Platform", r#""Windows""#)
            .header("Sec-Fetch-Dest", "empty")
            .header("Sec-Fetch-Mode", "cors")
            .header("Sec-Fetch-Site", "cross-site")
            .header("X-Browser-Channel", "stable")
            .header("X-Browser-Copyright", "Copyright 2025 Google LLC. All Rights reserved.")
            .header("X-Browser-Validation", "Aj9fzfu+SaGLBY9Oqr3S7RokOtM=")
            .header("X-Browser-Year", "2025")
            .header("X-Client-Data", "CIu2yQEIo7bJAQipncoBCIiSywEIlqHLAQiFoM0BCPOYzwEI1prPAQ==")
            .header("X-Client-Version", "Chrome/JsCore/11.0.0/FirebaseCore-web")
            .header("Referer", "https://windsurf.com/")
            .body(body)
            .send()
            .await
        {
            Ok(resp) => {
                super::report_request_success();
                resp
            }
            Err(e) => {
                // 检查是否是超时错误
                if e.is_timeout() || e.is_connect() {
                    super::report_timeout_error();
                } else {
                    super::report_request_failure();
                }
                return Err(AppError::Network(e.to_string()));
            }
        };

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            
            // 如果refresh token失败，返回特定错误
            if error_text.contains("TOKEN_EXPIRED") || error_text.contains("INVALID_REFRESH_TOKEN") {
                return Err(AppError::TokenExpired);
            }
            
            return Err(AppError::Api(error_text));
        }

        let refresh_response: RefreshTokenResponse = response.json().await?;
        
        // 计算Token过期时间
        let expires_in_secs: i64 = refresh_response.expires_in.parse()
            .unwrap_or(3600);
        let expires_at = Utc::now() + Duration::seconds(expires_in_secs);

        Ok((refresh_response.id_token, refresh_response.refresh_token, expires_at))
    }

    pub async fn get_account_info(&self, id_token: &str) -> AppResult<AccountInfo> {
        let url = format!(
            "https://identitytoolkit.googleapis.com/v1/accounts:lookup?key={}",
            FIREBASE_API_KEY
        );

        let body = serde_json::json!({
            "idToken": id_token
        });

        let response = match self.client
            .post(&url)
            .json(&body)
            .header("Content-Type", "application/json")
            .send()
            .await
        {
            Ok(resp) => {
                super::report_request_success();
                resp
            }
            Err(e) => {
                // 检查是否是超时错误
                if e.is_timeout() || e.is_connect() {
                    super::report_timeout_error();
                } else {
                    super::report_request_failure();
                }
                return Err(AppError::Network(e.to_string()));
            }
        };

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::Api(error_text));
        }

        let response_data: serde_json::Value = response.json().await?;
        
        if let Some(users) = response_data.get("users").and_then(|u| u.as_array()) {
            if let Some(user) = users.first() {
                let account_info: AccountInfo = serde_json::from_value(user.clone())
                    .map_err(|e| AppError::Api(e.to_string()))?;
                return Ok(account_info);
            }
        }

        Err(AppError::Api("No user info found".to_string()))
    }

    pub fn is_token_expired(expires_at: &DateTime<Utc>) -> bool {
        Utc::now() >= *expires_at
    }

    #[allow(dead_code)]
    pub fn should_refresh_token(expires_at: Option<DateTime<Utc>>) -> bool {
        // 如果Token在5分钟内过期，则刷新
        let buffer = Duration::minutes(5);
        expires_at.map_or(false, |dt| Utc::now() + buffer >= dt)
    }
}
