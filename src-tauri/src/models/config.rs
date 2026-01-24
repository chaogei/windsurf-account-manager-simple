use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// 支付页面延迟的默认值
fn default_payment_page_delay() -> i32 {
    2  // 默认2秒
}

// 卡头的默认值
fn default_card_bin() -> String {
    "626202".to_string()  // 默认6位卡头
}

/// 全局标签定义（带默认颜色）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalTag {
    pub name: String,
    pub color: String, // 默认颜色，RGBA或HEX格式
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub accounts: Vec<super::Account>,
    pub groups: Vec<String>,
    #[serde(default)]
    pub tags: Vec<GlobalTag>,  // 全局标签列表
    pub settings: Settings,
    #[serde(default)]  // 保留字段以便向后兼容，但不再使用
    pub logs: Vec<super::OperationLog>,  // 日志现在存储在独立的 logs.json 文件中
}

/// 账户排序字段
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SortField {
    Email,            // 邮箱/账户名称
    CreatedAt,        // 创建时间
    UsedQuota,        // 已用积分
    RemainingQuota,   // 剩余积分
    TokenExpiresAt,   // Token过期时间
    SubscriptionExpiresAt, // 订阅到期时间
    PlanName,         // 套餐类型
}

impl Default for SortField {
    fn default() -> Self {
        SortField::CreatedAt
    }
}

/// 排序方向
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SortDirection {
    Asc,
    Desc,
}

impl Default for SortDirection {
    fn default() -> Self {
        SortDirection::Asc
    }
}

/// 排序配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SortConfig {
    pub field: SortField,
    pub direction: SortDirection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub auto_refresh_token: bool,
    pub seat_count_options: Vec<i32>,
    pub retry_times: i32,
    pub theme: String,
    pub concurrent_limit: usize,
    #[serde(default)]
    pub show_seats_result_dialog: bool,  // 是否显示座位更新结果对话框
    #[serde(default, rename = "sortConfig")]
    pub sort_config: SortConfig,  // 账户排序配置
    #[serde(default, rename = "autoOpenPaymentLinkInWebview")]
    pub auto_open_payment_link_in_webview: bool,  // 是否自动在内置浏览器中打开支付链接
    #[serde(default, rename = "autoFillPaymentForm")]
    pub auto_fill_payment_form: bool,  // 是否自动填写支付表单
    #[serde(default, rename = "autoSubmitPaymentForm")]
    pub auto_submit_payment_form: bool,  // 是否自动提交支付表单
    #[serde(default = "default_payment_page_delay", rename = "paymentPageDelay")]
    pub payment_page_delay: i32,  // 支付页面加载延迟（秒）
    #[serde(default, rename = "showVirtualCardInfo")]
    pub show_virtual_card_info: bool,  // 是否显示虚拟卡信息弹窗
    #[serde(default = "default_card_bin", rename = "customCardBin")]
    pub custom_card_bin: String,  // 自定义卡头（4-12位数字）
    #[serde(default, rename = "customCardBinRange")]
    pub custom_card_bin_range: Option<String>,  // 卡段范围（如 626200-626300）
    #[serde(default = "default_card_bind_retry", rename = "cardBindRetryTimes")]
    pub card_bind_retry_times: i32,  // 绑卡失败重试次数
    #[serde(default, rename = "testModeEnabled")]
    pub test_mode_enabled: bool,  // 测试模式：自动收集成功的卡BIN
    #[serde(default, rename = "useLocalSuccessBins")]
    pub use_local_success_bins: bool,  // 使用本地成功BIN池
    #[serde(default, rename = "testModeLastBin")]
    pub test_mode_last_bin: Option<String>,  // 测试模式下上次使用的BIN（用于顺序遍历）
    #[serde(default, rename = "seamlessSwitchEnabled")]
    pub seamless_switch_enabled: bool,  // 是否启用无感换号
    #[serde(default, rename = "windsurfPath")]
    pub windsurf_path: Option<String>,  // Windsurf安装路径
    #[serde(default, rename = "patchBackupPath")]
    pub patch_backup_path: Option<String>,  // 补丁备份文件路径
    #[serde(default = "default_true", rename = "autoOpenBrowser")]
    pub auto_open_browser: bool,  // 是否自动打开浏览器
    #[serde(default = "default_browser_mode", rename = "browserMode")]
    pub browser_mode: String,  // 浏览器模式: "incognito" 或 "normal"
    #[serde(default, rename = "privacyMode")]
    pub privacy_mode: bool,  // 隐私模式，隐藏邮箱地址
    #[serde(default = "default_true", rename = "unlimitedConcurrentRefresh")]
    pub unlimited_concurrent_refresh: bool,  // 自动刷新Token时不限制并发数
    #[serde(default, rename = "proxyEnabled")]
    pub proxy_enabled: bool,  // 是否启用代理
    #[serde(default, rename = "proxyUrl")]
    pub proxy_url: Option<String>,  // 代理地址 (如 http://127.0.0.1:7890)
    #[serde(default, rename = "useLightweightApi")]
    pub use_lightweight_api: bool,  // 使用轻量级API(GetPlanStatus)获取配额信息，否则使用GetCurrentUser
    #[serde(default = "default_subscription_plan", rename = "subscriptionPlan")]
    pub subscription_plan: i32,  // 订阅计划: 1=Teams, 2=Pro
    #[serde(default = "default_payment_period", rename = "paymentPeriod")]
    pub payment_period: i32,  // 支付周期: 1=月付, 2=年付
    #[serde(default, rename = "teamName")]
    pub team_name: String,  // Teams 计划的团队名称
    #[serde(default = "default_seat_count", rename = "seatCount")]
    pub seat_count: i32,  // Teams 计划的席位数量
}

fn default_browser_mode() -> String {
    "incognito".to_string()
}

fn default_card_bind_retry() -> i32 {
    5  // 默认重试5次
}

fn default_true() -> bool {
    true
}

fn default_subscription_plan() -> i32 {
    2  // 默认 Pro 计划
}

fn default_payment_period() -> i32 {
    1  // 默认月付
}

fn default_seat_count() -> i32 {
    1  // 默认1个席位
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            auto_refresh_token: true,
            seat_count_options: vec![18, 19, 20],
            retry_times: 2,
            theme: "light".to_string(),
            concurrent_limit: 5,
            show_seats_result_dialog: false,  // 默认关闭
            sort_config: SortConfig::default(),  // 默认自定义排序
            auto_open_payment_link_in_webview: false,  // 默认关闭
            auto_fill_payment_form: false,  // 默认关闭
            auto_submit_payment_form: false,  // 默认关闭
            payment_page_delay: 2,  // 默认2秒
            show_virtual_card_info: false,  // 默认关闭虚拟卡信息弹窗
            custom_card_bin: "626202".to_string(),  // 默认卡头
            custom_card_bin_range: None,  // 默认不使用卡段范围
            card_bind_retry_times: 5,  // 默认绑卡重试5次
            test_mode_enabled: false,  // 默认关闭测试模式
            use_local_success_bins: false,  // 默认不使用本地成功BIN池
            test_mode_last_bin: None,  // 测试模式进度
            seamless_switch_enabled: false,  // 默认关闭无感换号
            windsurf_path: None,
            patch_backup_path: None,
            auto_open_browser: true,  // 默认自动打开浏览器
            browser_mode: "incognito".to_string(),  // 默认无痕模式
            privacy_mode: false,  // 默认关闭隐私模式
            unlimited_concurrent_refresh: true,  // 默认开启无限制并发刷新
            proxy_enabled: false,  // 默认关闭代理
            proxy_url: None,  // 默认无代理地址
            use_lightweight_api: false,  // 默认关闭轻量级API
            subscription_plan: 2,  // 默认 Pro 计划
            payment_period: 1,  // 默认月付
            team_name: String::new(),  // 默认空团队名称
            seat_count: 1,  // 默认1个席位
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            accounts: Vec::new(),
            groups: vec!["默认分组".to_string()],
            tags: Vec::new(),
            settings: Settings::default(),
            logs: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct FirebaseAuthInfo {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct TokenInfo {
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub refresh_token: Option<String>,
}

/// 自动重置配置（按分组或账号）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoResetConfig {
    pub id: String,
    #[serde(rename = "targetType")]
    pub target_type: String,  // "group" 或 "account"
    #[serde(rename = "targetId")]
    pub target_id: String,    // 分组名称或账号ID
    pub enabled: bool,
    #[serde(rename = "checkInterval")]
    pub check_interval: i32,  // 检查间隔（分钟）
    #[serde(rename = "usageThreshold")]
    pub usage_threshold: i32, // 使用率阈值（百分比）
    #[serde(rename = "remainingThreshold")]
    pub remaining_threshold: i32, // 剩余积分阈值
    #[serde(rename = "lastCheckAt")]
    pub last_check_at: Option<DateTime<Utc>>,
    #[serde(rename = "lastResetAt")]
    pub last_reset_at: Option<DateTime<Utc>>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}

impl AutoResetConfig {
    pub fn new(target_type: String, target_id: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            target_type,
            target_id,
            enabled: true,
            check_interval: 5,
            usage_threshold: 80,
            remaining_threshold: 1000,
            last_check_at: None,
            last_reset_at: None,
            created_at: Utc::now(),
        }
    }
}
