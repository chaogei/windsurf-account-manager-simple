use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// 带颜色的标签
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagWithColor {
    pub name: String,
    pub color: String, // RGBA格式，如 "rgba(255, 100, 100, 1)"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: Uuid,
    pub email: String,
    pub password: String, // 加密后的密码
    pub nickname: String,
    pub tags: Vec<String>,
    #[serde(default, rename = "tagColors")]
    pub tag_colors: Vec<TagWithColor>, // 带颜色的标签
    pub group: Option<String>,
    pub token: Option<String>, // 加密后的Token
    pub refresh_token: Option<String>, // 加密后的Refresh Token
    pub token_expires_at: Option<DateTime<Utc>>,
    pub last_seat_count: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub last_login_at: Option<DateTime<Utc>>,
    pub status: AccountStatus,
    // 配额和套餐信息
    pub plan_name: Option<String>,
    pub used_quota: Option<i32>,
    pub total_quota: Option<i32>,
    pub last_quota_update: Option<DateTime<Utc>>,
    // 订阅到期时间
    pub subscription_expires_at: Option<DateTime<Utc>>,
    // 订阅是否激活 (从 GetCurrentUser API 的 subscription.subscription_active 获取)
    #[serde(default)]
    pub subscription_active: Option<bool>,
    // Windsurf API Key (用户的 UUID，从 GetCurrentUser API 获取)
    pub windsurf_api_key: Option<String>,
    // 账户是否被禁用 (从 GetCurrentUser API 的 user.disable_codeium 获取)
    #[serde(default)]
    pub is_disabled: Option<bool>,
    // 是否为团队所有者（Admin角色，有团队成员的主账号）
    #[serde(default)]
    pub is_team_owner: Option<bool>,
    // 自定义排序顺序（用于拖拽排序）
    #[serde(default, rename = "sortOrder")]
    pub sort_order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountStatus {
    Active,
    Inactive,
    Error(String),
}

impl Account {
    pub fn new(email: String, password: String, nickname: String, tags: Vec<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            email,
            password,
            nickname,
            tags,
            tag_colors: Vec::new(),
            group: None,
            token: None,
            refresh_token: None,
            token_expires_at: None,
            last_seat_count: None,
            created_at: Utc::now(),
            last_login_at: None,
            status: AccountStatus::Inactive,
            plan_name: None,
            used_quota: None,
            total_quota: None,
            last_quota_update: None,
            subscription_expires_at: None,
            subscription_active: None,
            windsurf_api_key: None,
            is_disabled: None,
            is_team_owner: None,
            sort_order: 0,
        }
    }

    #[allow(dead_code)]
    pub fn is_token_valid(&self) -> bool {
        if let Some(expires_at) = self.token_expires_at {
            expires_at > Utc::now()
        } else {
            false
        }
    }
}
