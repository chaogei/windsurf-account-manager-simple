use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use base64::{Engine as _, engine::general_purpose};

/// Protobuf Wire Types
#[derive(Debug, Clone, Copy, PartialEq)]
enum WireType {
    Varint = 0,
    Fixed64 = 1,
    LengthDelimited = 2,
    StartGroup = 3,     // Deprecated
    EndGroup = 4,       // Deprecated  
    Fixed32 = 5,
}

/// 简单的Protobuf解析器
pub struct ProtobufParser {
    data: Vec<u8>,
    position: usize,
}

impl ProtobufParser {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data, position: 0 }
    }

    /// 从Base64字符串解析
    #[allow(dead_code)]
    pub fn from_base64(base64_str: &str) -> Result<Self, String> {
        // 处理可能的前缀
        let base64_data = if base64_str.starts_with("data:application/proto;base64,") {
            &base64_str[31..]
        } else {
            base64_str
        };

        // Base64解码
        let decoded = general_purpose::STANDARD.decode(base64_data)
            .map_err(|e| format!("Base64解码失败: {}", e))?;

        // 解析Protobuf
        Ok(Self::new(decoded))
    }

    /// 解析消息
    pub fn parse_message(&mut self) -> Result<Value, String> {
        let mut message = serde_json::Map::new();

        while self.position < self.data.len() {
            match self.read_tag() {
                Ok((field_number, wire_type)) => {
                    if field_number == 0 {
                        break;
                    }

                    let value = self.read_field(wire_type)?;
                    let field_name = self.get_field_name(field_number, wire_type, &value);

                    // 处理重复字段
                    if message.contains_key(&field_name) {
                        let existing = message.get_mut(&field_name).unwrap();
                        if !existing.is_array() {
                            let temp = existing.clone();
                            *existing = json!([temp]);
                        }
                        if let Some(arr) = existing.as_array_mut() {
                            arr.push(value);
                        }
                    } else {
                        message.insert(field_name, value);
                    }
                }
                Err(_) => break,
            }
        }

        Ok(Value::Object(message))
    }

    /// 读取标签（field number和wire type）
    fn read_tag(&mut self) -> Result<(u32, WireType), String> {
        if self.position >= self.data.len() {
            return Ok((0, WireType::Varint));
        }

        let tag = self.read_varint()?;
        let field_number = tag >> 3;
        let wire_type = match tag & 0x07 {
            0 => WireType::Varint,
            1 => WireType::Fixed64,
            2 => WireType::LengthDelimited,
            3 => WireType::StartGroup,
            4 => WireType::EndGroup,
            5 => WireType::Fixed32,
            _ => return Err(format!("未知的wire type: {}", tag & 0x07)),
        };

        Ok((field_number as u32, wire_type))
    }

    /// 读取变长整数
    fn read_varint(&mut self) -> Result<u64, String> {
        let mut result: u64 = 0;
        let mut shift = 0;

        while self.position < self.data.len() {
            let byte = self.data[self.position];
            self.position += 1;

            result |= ((byte & 0x7F) as u64) << shift;

            if (byte & 0x80) == 0 {
                return Ok(result);
            }

            shift += 7;

            if shift >= 64 {
                return Err("Varint过长".to_string());
            }
        }

        Err("数据意外结束".to_string())
    }

    /// 读取固定32位
    fn read_fixed32(&mut self) -> Result<f32, String> {
        if self.position + 4 > self.data.len() {
            return Err("数据不足以读取fixed32".to_string());
        }

        let bytes = &self.data[self.position..self.position + 4];
        self.position += 4;
        
        Ok(f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    /// 读取固定64位
    fn read_fixed64(&mut self) -> Result<f64, String> {
        if self.position + 8 > self.data.len() {
            return Err("数据不足以读取fixed64".to_string());
        }

        let bytes = &self.data[self.position..self.position + 8];
        self.position += 8;
        
        let mut arr = [0u8; 8];
        arr.copy_from_slice(bytes);
        Ok(f64::from_le_bytes(arr))
    }

    /// 读取长度分隔的数据
    fn read_length_delimited(&mut self) -> Result<Vec<u8>, String> {
        let length = self.read_varint()? as usize;

        if self.position + length > self.data.len() {
            return Err(format!("数据不足: 需要{}字节，剩余{}字节", 
                length, self.data.len() - self.position));
        }

        let value = self.data[self.position..self.position + length].to_vec();
        self.position += length;
        Ok(value)
    }

    /// 读取字段值
    fn read_field(&mut self, wire_type: WireType) -> Result<Value, String> {
        match wire_type {
            WireType::Varint => {
                let value = self.read_varint()?;
                Ok(json!(value))
            }
            WireType::Fixed64 => {
                let value = self.read_fixed64()?;
                Ok(json!(value))
            }
            WireType::LengthDelimited => {
                let data = self.read_length_delimited()?;

                // 尝试解析为UTF-8字符串
                if let Ok(text) = String::from_utf8(data.clone()) {
                    if text.chars().all(|c| c.is_ascii_graphic() || c.is_ascii_whitespace()) && !text.is_empty() {
                        return Ok(json!(text));
                    }
                }

                // 尝试解析为子消息
                let mut parser = ProtobufParser::new(data.clone());
                if let Ok(sub_message) = parser.parse_message() {
                    if sub_message.as_object().map_or(false, |o| !o.is_empty()) {
                        return Ok(sub_message);
                    }
                }

                // 返回原始字节（如果较短）
                if data.len() <= 32 {
                    let obj: HashMap<usize, u8> = data.iter()
                        .enumerate()
                        .map(|(i, &b)| (i, b))
                        .collect();
                    Ok(json!(obj))
                } else {
                    Ok(json!({
                        "length": data.len(),
                        "preview": &data[..32.min(data.len())]
                    }))
                }
            }
            WireType::StartGroup | WireType::EndGroup => {
                Err("不支持Group类型".to_string())
            }
            WireType::Fixed32 => {
                let value = self.read_fixed32()?;
                Ok(json!(value))
            }
        }
    }

    /// 获取字段名称
    fn get_field_name(&self, field_number: u32, wire_type: WireType, value: &Value) -> String {
        match wire_type {
            WireType::Varint => format!("int_{}", field_number),
            WireType::LengthDelimited => {
                if value.is_string() {
                    format!("string_{}", field_number)
                } else if value.is_object() {
                    format!("subMesssage_{}", field_number)
                } else {
                    format!("bytes_{}", field_number)
                }
            }
            WireType::Fixed32 => format!("float_{}", field_number),
            WireType::Fixed64 => format!("double_{}", field_number),
            _ => format!("field_{}", field_number),
        }
    }
}

/// 用户信息结构
#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub user: UserBasicInfo,
    pub roles: Option<String>,           // string_2: 角色字符串 (如 "root.admin")
    pub subscription: Option<SubscriptionInfo>,
    pub plan: Option<PlanInfo>,
    pub role: Option<AdminInfo>,         // subMesssage_7: 角色详情
    pub admin: Option<AdminInfo>,        // 兼容旧代码
    pub is_root_admin: bool,
    pub team: Option<TeamInfo>,
    pub permissions: Option<serde_json::Value>,  // subMesssage_8: 权限对象
    pub plan_features: Option<serde_json::Value>, // subMesssage_6.subMesssage_24: 功能配置
}

/// User message (seat_management_pb.User)
/// 根据官方proto定义：api_key=1, name=2, email=3, signup_time=4, last_update_time=5, id=6, ...
#[derive(Debug, Serialize, Deserialize)]
pub struct UserBasicInfo {
    pub api_key: String,         // field 1: API Key (UUID格式)
    pub name: String,            // field 2: 用户显示名称
    pub email: String,           // field 3: 邮箱
    pub id: String,              // field 6: Firebase UID (身份标识)
    pub team_id: String,         // field 7: 所属团队ID
    pub team_status: i32,        // field 8: UserTeamStatus (0=未指定,1=待定,2=已批准,3=已拒绝)
    pub username: String,        // field 9: 用户名 (如 righteously-handsome-kite-82267)
    pub timezone: String,        // field 10: preferred_time_zone (如 Asia/Shanghai)
    pub public_profile_enabled: bool,  // field 11: 公开资料
    pub pro: bool,               // field 13: Pro用户标识
    pub disable_codeium: bool,   // field 16: 是否禁用Codeium
    pub newsletter: bool,        // field 19: 订阅邮件
    pub disabled_telemetry: bool, // field 20: 禁用遥测
    pub signup_stage: Option<String>,  // field 22: 注册阶段
    pub used_trial: bool,        // field 25: 已使用试用
    pub used_prompt_credits: i64, // field 28: 已用Prompt积分
    pub used_flow_credits: i64,   // field 29: 已用Flow积分
    pub referral_code: Option<String>,  // field 30: 推荐码
    // Timestamp fields
    pub signup_time: Option<i64>,       // field 4: 注册时间 (Timestamp)
    pub last_update_time: Option<i64>,  // field 5: 最后更新时间 (Timestamp)
    pub first_windsurf_use_time: Option<i64>,  // field 26: 首次使用Windsurf时间
    pub windsurf_pro_trial_end_time: Option<i64>,  // field 27: Pro试用结束时间
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionInfo {
    pub id: String,
    pub email: String,
    pub stripe_subscription_id: String,
    pub stripe_customer_id: String,
    pub seats: i32,
    pub usage: i32,
    pub quota: i32,
    pub used_quota: i32,
    pub expires_at: Option<i64>, // Unix时间戳（秒）
    pub subscription_active: bool,
    pub on_trial: bool,
}

/// Team message (seat_management_pb.Team)
/// 根据官方proto定义的完整字段映射
#[derive(Debug, Serialize, Deserialize)]
pub struct TeamInfo {
    pub id: String,                      // field 1: 团队ID
    pub name: String,                    // field 2: 团队名称
    pub signup_time: Option<i64>,        // field 3: 团队创建时间 (Timestamp)
    pub invite_id: Option<String>,       // field 4: 邀请码ID
    pub used_trial: bool,                // field 5: 是否已使用试用
    pub stripe_subscription_id: Option<String>,   // field 6: Stripe订阅ID
    pub subscription_active: bool,       // field 7: 订阅是否激活
    pub stripe_customer_id: Option<String>,       // field 8: Stripe客户ID
    pub current_billing_period_start: Option<i64>, // field 9: 计费周期开始 (Timestamp)
    pub num_seats_current_billing_period: i32,    // field 10: 当前计费周期席位数
    pub attribution_enabled: bool,       // field 11: 是否启用归因
    pub sso_provider_id: Option<String>, // field 12: SSO提供商ID
    pub offers_enabled: bool,            // field 13: 是否启用优惠
    pub teams_tier: i32,                 // field 14: TeamsTier枚举 (1=Teams,2=Pro,3=EnterpriseSaaS...)
    pub flex_credit_quota: i64,          // field 15: Flex积分配额
    pub used_flow_credits: i64,          // field 16: 已用Flow积分
    pub used_prompt_credits: i64,        // field 17: 已用Prompt积分
    pub current_billing_period_end: Option<i64>,  // field 18: 计费周期结束 (Timestamp)
    pub num_cascade_seats: i32,          // field 19: Cascade席位数
    pub cascade_usage_month_start: Option<i64>,   // field 20: Cascade使用月开始 (Timestamp)
    pub cascade_usage_month_end: Option<i64>,     // field 21: Cascade使用月结束 (Timestamp)
    pub cascade_seat_type: i32,          // field 22: CascadeSeatType枚举
    pub top_up_enabled: bool,            // field 23: 是否启用充值
    pub monthly_top_up_amount: i64,      // field 24: 月度充值金额
    pub top_up_spent: i64,               // field 25: 已花费充值
    pub top_up_increment: i64,           // field 26: 充值增量
    pub used_flex_credits: i64,          // field 27: 已用Flex积分
    pub num_users: i32,                  // 计算字段
}

/// PlanInfo message (codeium_common_pb.PlanInfo)
/// 根据官方proto定义的完整字段映射
#[derive(Debug, Serialize, Deserialize)]
pub struct PlanInfo {
    pub teams_tier: i32,             // field 1: TeamsTier枚举
    pub plan_name: String,           // field 2: 套餐名称 (如 "Teams")
    pub has_autocomplete_fast_mode: bool,  // field 3: 快速自动补全
    pub allow_sticky_premium_models: bool, // field 4: 允许使用高级模型
    pub has_forge_access: bool,      // field 5: Forge访问权限
    pub max_num_premium_chat_messages: i64, // field 6: 最大高级聊天消息数
    pub max_num_chat_input_tokens: i64,     // field 7: 最大聊天输入tokens
    pub max_custom_chat_instruction_characters: i64, // field 8: 最大自定义指令字符
    pub max_num_pinned_context_items: i64,  // field 9: 最大固定上下文项数
    pub max_local_index_size: i64,   // field 10: 最大本地索引大小
    pub disable_code_snippet_telemetry: bool, // field 11: 禁用代码片段遥测
    pub monthly_prompt_credits: i32, // field 12: 月度Prompt积分
    pub monthly_flow_credits: i32,   // field 13: 月度Flow积分
    pub monthly_flex_credit_purchase_amount: i32, // field 14: 月度Flex积分购买额度
    pub allow_premium_command_models: bool, // field 15: 允许高级命令模型
    pub is_enterprise: bool,         // field 16: 是否企业版
    pub is_teams: bool,              // field 17: 是否团队版
    pub can_buy_more_credits: bool,  // field 18: 是否可购买更多积分
    pub cascade_web_search_enabled: bool, // field 19: Cascade网络搜索
    pub can_customize_app_icon: bool, // field 20: 可自定义应用图标
    pub cascade_can_auto_run_commands: bool, // field 22: Cascade可自动运行命令
    pub has_tab_to_jump: bool,       // field 23: Tab跳转功能
    pub can_generate_commit_messages: bool, // field 25: 可生成提交消息
    pub max_unclaimed_sites: i32,    // field 26: 最大未认领站点数
    pub knowledge_base_enabled: bool, // field 27: 知识库功能
    pub can_share_conversations: bool, // field 28: 可分享对话
    pub can_allow_cascade_in_background: bool, // field 29: 允许Cascade后台运行
    pub browser_enabled: bool,       // field 31: 浏览器功能
}

/// UserRole message (seat_management_pb.UserRole)
/// 根据官方proto定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRole {
    pub api_key: String,         // field 1: API Key
    pub roles: Vec<String>,      // field 2: 角色列表 (repeated string)
    pub role_id: String,         // field 3: 角色ID (如 "root.admin")
    pub role_name: String,       // field 4: 角色名称 (如 "Admin")
}

// 兼容旧代码的别名
pub type AdminInfo = UserRole;

/// 从解析的Protobuf数据中提取用户信息
pub fn extract_user_info(parsed_data: &Value) -> Result<UserInfo, String> {
    let obj = parsed_data.as_object()
        .ok_or("解析数据不是对象")?;

    // 提取User信息 (field 1 = subMesssage_1)
    // 根据官方proto: api_key=1, name=2, email=3, signup_time=4, last_update_time=5, id=6, team_id=7...
    let user = if let Some(u) = obj.get("subMesssage_1").and_then(|v| v.as_object()) {
        UserBasicInfo {
            api_key: u.get("string_1").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            name: u.get("string_2").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            email: u.get("string_3").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            id: u.get("string_6").and_then(|v| v.as_str()).unwrap_or("").to_string(),  // Firebase UID
            team_id: u.get("string_7").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            team_status: u.get("int_8").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
            username: u.get("string_9").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            timezone: u.get("string_10").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            public_profile_enabled: u.get("int_11").and_then(|v| v.as_i64()).map(|v| v == 1).unwrap_or(false),
            pro: u.get("int_13").and_then(|v| v.as_i64()).map(|v| v == 1).unwrap_or(false),
            disable_codeium: u.get("int_16").and_then(|v| v.as_i64()).map(|v| v == 1).unwrap_or(false),
            newsletter: u.get("int_19").and_then(|v| v.as_i64()).map(|v| v == 1).unwrap_or(false),
            disabled_telemetry: u.get("int_20").and_then(|v| v.as_i64()).map(|v| v == 1).unwrap_or(false),
            signup_stage: u.get("string_22").and_then(|v| v.as_str()).map(|s| s.to_string()),
            used_trial: u.get("int_25").and_then(|v| v.as_i64()).map(|v| v == 1).unwrap_or(false),
            used_prompt_credits: u.get("int_28").and_then(|v| v.as_i64()).unwrap_or(0),
            used_flow_credits: u.get("int_29").and_then(|v| v.as_i64()).unwrap_or(0),
            referral_code: u.get("string_30").and_then(|v| v.as_str()).map(|s| s.to_string()),
            // Timestamp fields (protobuf Timestamp has seconds in field 1)
            signup_time: u.get("subMesssage_4").and_then(|v| v.get("int_1")).and_then(|v| v.as_i64()),
            last_update_time: u.get("subMesssage_5").and_then(|v| v.get("int_1")).and_then(|v| v.as_i64()),
            first_windsurf_use_time: u.get("subMesssage_26").and_then(|v| v.get("int_1")).and_then(|v| v.as_i64()),
            windsurf_pro_trial_end_time: u.get("subMesssage_27").and_then(|v| v.get("int_1")).and_then(|v| v.as_i64()),
        }
    } else {
        return Err("缺少用户基本信息".to_string());
    };

    // 提取Team信息 (field 4 = subMesssage_4)
    // 根据官方proto: id=1, name=2, signup_time=3, invite_id=4, used_trial=5, stripe_subscription_id=6...
    let team = obj.get("subMesssage_4").and_then(|v| v.as_object()).map(|t| {
        TeamInfo {
            id: t.get("string_1").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            name: t.get("string_2").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            signup_time: t.get("subMesssage_3").and_then(|v| v.get("int_1")).and_then(|v| v.as_i64()),
            invite_id: t.get("string_4").and_then(|v| v.as_str()).map(|s| s.to_string()),
            used_trial: t.get("int_5").and_then(|v| v.as_i64()).map(|v| v == 1).unwrap_or(false),
            stripe_subscription_id: t.get("string_6").and_then(|v| v.as_str()).map(|s| s.to_string()),
            subscription_active: t.get("int_7").and_then(|v| v.as_i64()).map(|v| v == 1).unwrap_or(false),
            stripe_customer_id: t.get("string_8").and_then(|v| v.as_str()).map(|s| s.to_string()),
            current_billing_period_start: t.get("subMesssage_9").and_then(|v| v.get("int_1")).and_then(|v| v.as_i64()),
            num_seats_current_billing_period: t.get("int_10").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
            attribution_enabled: t.get("int_11").and_then(|v| v.as_i64()).map(|v| v == 1).unwrap_or(false),
            sso_provider_id: t.get("string_12").and_then(|v| v.as_str()).map(|s| s.to_string()),
            offers_enabled: t.get("int_13").and_then(|v| v.as_i64()).map(|v| v == 1).unwrap_or(false),
            teams_tier: t.get("int_14").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
            flex_credit_quota: t.get("int_15").and_then(|v| v.as_i64()).unwrap_or(0),
            used_flow_credits: t.get("int_16").and_then(|v| v.as_i64()).unwrap_or(0),
            used_prompt_credits: t.get("int_17").and_then(|v| v.as_i64()).unwrap_or(0),
            current_billing_period_end: t.get("subMesssage_18").and_then(|v| v.get("int_1")).and_then(|v| v.as_i64()),
            num_cascade_seats: t.get("int_19").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
            cascade_usage_month_start: t.get("subMesssage_20").and_then(|v| v.get("int_1")).and_then(|v| v.as_i64()),
            cascade_usage_month_end: t.get("subMesssage_21").and_then(|v| v.get("int_1")).and_then(|v| v.as_i64()),
            cascade_seat_type: t.get("int_22").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
            top_up_enabled: t.get("int_23").and_then(|v| v.as_i64()).map(|v| v == 1).unwrap_or(false),
            monthly_top_up_amount: t.get("int_24").and_then(|v| v.as_i64()).unwrap_or(0),
            top_up_spent: t.get("int_25").and_then(|v| v.as_i64()).unwrap_or(0),
            top_up_increment: t.get("int_26").and_then(|v| v.as_i64()).unwrap_or(0),
            used_flex_credits: t.get("int_27").and_then(|v| v.as_i64()).unwrap_or(0),
            num_users: 1,  // 计算字段，默认1
        }
    });

    // 提取订阅信息 (实际也在Team对象内)
    let subscription = obj.get("subMesssage_4").and_then(|sub| {
        // 获取基础配额（从plan）
        let base_quota = obj.get("subMesssage_6")
            .and_then(|plan| plan.get("int_12"))
            .and_then(|v| v.as_i64())
            .unwrap_or(0) as i32;

        // 获取额外配额
        let extra_quota = sub.get("int_15")
            .and_then(|v| v.as_i64())
            .unwrap_or(0) as i32;

        // 实际总配额 = 基础配额 + 额外配额
        let total_quota = base_quota + extra_quota;

        // 提取订阅到期时间 (subMesssage_18.int_1)
        let expires_at = sub.get("subMesssage_18")
            .and_then(|v| v.get("int_1"))
            .and_then(|v| v.as_i64());

        Some(SubscriptionInfo {
            id: sub.get("string_1")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            email: sub.get("string_2")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            stripe_subscription_id: sub.get("string_6")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            stripe_customer_id: sub.get("string_8")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            seats: sub.get("int_10")
                .and_then(|v| v.as_i64())
                .unwrap_or(sub.get("int_14").and_then(|v| v.as_i64()).unwrap_or(0)) as i32,
            usage: 1,  // 默认1个用户使用
            quota: total_quota,  // 使用计算后的总配额
            used_quota: sub.get("int_17")
                .and_then(|v| v.as_i64())
                .unwrap_or(0) as i32,
            expires_at,  // 订阅到期时间
            subscription_active: sub.get("int_7")
                .and_then(|v| v.as_i64())
                .map(|v| v == 1)
                .unwrap_or(false),
            on_trial: false,  // 从其他字段判断
        })
    });

    // 提取PlanInfo (field 6 = subMesssage_6)
    // 根据官方codeium_common_pb.PlanInfo: teams_tier=1, plan_name=2, has_autocomplete_fast_mode=3...
    let plan = obj.get("subMesssage_6").and_then(|v| v.as_object()).map(|p| {
        PlanInfo {
            teams_tier: p.get("int_1").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
            plan_name: p.get("string_2").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            has_autocomplete_fast_mode: p.get("int_3").and_then(|v| v.as_i64()).map(|v| v == 1).unwrap_or(false),
            allow_sticky_premium_models: p.get("int_4").and_then(|v| v.as_i64()).map(|v| v == 1).unwrap_or(false),
            has_forge_access: p.get("int_5").and_then(|v| v.as_i64()).map(|v| v == 1).unwrap_or(false),
            max_num_premium_chat_messages: p.get("int_6").and_then(|v| v.as_i64()).unwrap_or(0),
            max_num_chat_input_tokens: p.get("int_7").and_then(|v| v.as_i64()).unwrap_or(0),
            max_custom_chat_instruction_characters: p.get("int_8").and_then(|v| v.as_i64()).unwrap_or(0),
            max_num_pinned_context_items: p.get("int_9").and_then(|v| v.as_i64()).unwrap_or(0),
            max_local_index_size: p.get("int_10").and_then(|v| v.as_i64()).unwrap_or(0),
            disable_code_snippet_telemetry: p.get("int_11").and_then(|v| v.as_i64()).map(|v| v == 1).unwrap_or(false),
            monthly_prompt_credits: p.get("int_12").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
            monthly_flow_credits: p.get("int_13").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
            monthly_flex_credit_purchase_amount: p.get("int_14").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
            allow_premium_command_models: p.get("int_15").and_then(|v| v.as_i64()).map(|v| v == 1).unwrap_or(false),
            is_enterprise: p.get("int_16").and_then(|v| v.as_i64()).map(|v| v == 1).unwrap_or(false),
            is_teams: p.get("int_17").and_then(|v| v.as_i64()).map(|v| v == 1).unwrap_or(false),
            can_buy_more_credits: p.get("int_18").and_then(|v| v.as_i64()).map(|v| v == 1).unwrap_or(false),
            cascade_web_search_enabled: p.get("int_19").and_then(|v| v.as_i64()).map(|v| v == 1).unwrap_or(false),
            can_customize_app_icon: p.get("int_20").and_then(|v| v.as_i64()).map(|v| v == 1).unwrap_or(false),
            cascade_can_auto_run_commands: p.get("int_22").and_then(|v| v.as_i64()).map(|v| v == 1).unwrap_or(false),
            has_tab_to_jump: p.get("int_23").and_then(|v| v.as_i64()).map(|v| v == 1).unwrap_or(false),
            can_generate_commit_messages: p.get("int_25").and_then(|v| v.as_i64()).map(|v| v == 1).unwrap_or(false),
            max_unclaimed_sites: p.get("int_26").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
            knowledge_base_enabled: p.get("int_27").and_then(|v| v.as_i64()).map(|v| v == 1).unwrap_or(false),
            can_share_conversations: p.get("int_28").and_then(|v| v.as_i64()).map(|v| v == 1).unwrap_or(false),
            can_allow_cascade_in_background: p.get("int_29").and_then(|v| v.as_i64()).map(|v| v == 1).unwrap_or(false),
            browser_enabled: p.get("int_31").and_then(|v| v.as_i64()).map(|v| v == 1).unwrap_or(false),
        }
    });

    // 提取角色字符串 (string_2: roles)
    let roles = obj.get("string_2")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    // 提取UserRole (field 7 = subMesssage_7)
    // 根据官方proto: api_key=1, roles=2 (repeated), role_id=3, role_name=4
    let role_info = obj.get("subMesssage_7").and_then(|v| v.as_object()).map(|r| {
        // 提取repeated string roles (field 2)
        let roles_vec: Vec<String> = r.get("string_2")
            .and_then(|v| v.as_str())
            .map(|s| vec![s.to_string()])
            .or_else(|| {
                r.get("repeated_2").and_then(|v| v.as_array()).map(|arr| {
                    arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect()
                })
            })
            .unwrap_or_default();
        
        UserRole {
            api_key: r.get("string_1").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            roles: roles_vec,
            role_id: r.get("string_3").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            role_name: r.get("string_4").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        }
    });
    
    // 提取权限对象 (Permissions - field 8 / subMesssage_8 或 bytes_8)
    let permissions = obj.get("subMesssage_8")
        .or_else(|| obj.get("bytes_8"))
        .cloned();

    // 提取套餐功能配置 (subMesssage_6.subMesssage_24)
    let plan_features = obj.get("subMesssage_6")
        .and_then(|v| v.get("subMesssage_24"))
        .cloned();

    // 检查是否为root admin
    let is_root_admin = roles.as_ref()
        .map(|s| s == "root.admin")
        .unwrap_or(false);

    Ok(UserInfo {
        user,
        roles,
        subscription,
        plan,
        role: role_info.clone(),
        admin: role_info,
        is_root_admin,
        team,
        permissions,
        plan_features,
    })
}

/// 解析GetCurrentUser API的响应
pub fn parse_get_current_user_response(response_body: &[u8]) -> Result<Value, String> {
    // 尝试将响应转换为字符串
    let response_str = String::from_utf8_lossy(response_body);
    
    // 检查是否是base64编码的响应（带前缀）
    if response_str.starts_with("data:application/proto;base64,") {
        // 解析Protobuf（去掉前缀）
        let base64_data = &response_str[31..];
        let decoded = general_purpose::STANDARD.decode(base64_data)
            .map_err(|e| format!("Base64解码失败: {}", e))?;
        
        let mut parser = ProtobufParser::new(decoded);
        let parsed = parser.parse_message()?;
        let user_info = extract_user_info(&parsed)?;
        
        Ok(json!({
            "parsed_data": parsed,
            "user_info": user_info
        }))
    } else if response_str.starts_with("AAEAAQ==") || response_str.chars().all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '=') {
        // 看起来像是没有前缀的Base64编码
        let decoded = general_purpose::STANDARD.decode(response_str.trim())
            .map_err(|e| format!("Base64解码失败: {}", e))?;
        
        let mut parser = ProtobufParser::new(decoded);
        let parsed = parser.parse_message()?;
        let user_info = extract_user_info(&parsed)?;
        
        Ok(json!({
            "parsed_data": parsed,
            "user_info": user_info
        }))
    } else {
        // 尝试直接解析为二进制Protobuf
        let mut parser = ProtobufParser::new(response_body.to_vec());
        match parser.parse_message() {
            Ok(parsed) => {
                match extract_user_info(&parsed) {
                    Ok(user_info) => {
                        return Ok(json!({
                            "parsed_data": parsed,
                            "user_info": user_info
                        }))
                    }
                    Err(_) => {
                        // 解析成功但无法提取用户信息，可能不是用户数据
                        Ok(json!({
                            "parsed_data": parsed,
                            "error": "无法提取用户信息"
                        }))
                    }
                }
            }
            Err(e) => {
                // 不是有效的Protobuf，返回错误
                Ok(json!({
                    "error": format!("解析失败: {}", e),
                    "raw": response_str.to_string()
                }))
            }
        }
    }
}

impl ProtobufParser {
    #[allow(dead_code)]
    pub fn parse_update_seats_response(response_body: &[u8]) -> Result<Value, String> {
        // 检查是否是base64编码的响应
        let decoded_body = if response_body.starts_with(b"data:application/proto;base64,") {
            let base64_str = std::str::from_utf8(&response_body[30..])
                .map_err(|e| format!("Invalid UTF-8: {}", e))?;
            general_purpose::STANDARD.decode(base64_str.trim())
                .map_err(|e| format!("Base64 decode error: {}", e))?
        } else {
            response_body.to_vec()
        };
        
        let mut parser = ProtobufParser::new(decoded_body);
        let parsed = parser.parse_message().map_err(|e| format!("Parse error: {}", e))?;
        
        // 解析更新座位响应 (UpdateSeatsResponse)
        let mut result = json!({
            "success": true,
            "raw_data": parsed.clone()
        });
        
        // 提取BillingUpdate信息 (field 1)
        if let Some(billing_update) = parsed.get("subMesssage_1") {
            // amount_due_immediately (field 1): 立即应付金额
            if let Some(amount_due) = billing_update.get("float_1").and_then(|v: &Value| v.as_f64()) {
                result["amount_due_immediately"] = json!(amount_due);
            }
            
            // price_per_seat (field 3): 每座位价格
            if let Some(price_per_seat) = billing_update.get("float_3").and_then(|v: &Value| v.as_f64()) {
                result["price_per_seat"] = json!(price_per_seat);
            }
            
            // num_seats (field 4): 座位数量
            if let Some(num_seats) = billing_update.get("int_4").and_then(|v: &Value| v.as_i64()) {
                result["total_seats"] = json!(num_seats);
            }
            
            // sub_interval (field 5): 订阅周期 (1=月度, 2=年度)
            if let Some(interval) = billing_update.get("int_5").and_then(|v: &Value| v.as_i64()) {
                result["billing_interval"] = json!(if interval == 1 { "monthly" } else { "yearly" });
            }
            
            // amount_per_interval (field 6): 每周期金额
            if let Some(total_price) = billing_update.get("float_6").and_then(|v: &Value| v.as_f64()) {
                result["total_monthly_price"] = json!(total_price);
            }
            
            // billing_start (field 7): 计费开始时间
            if let Some(timestamp_obj) = billing_update.get("subMesssage_7") {
                if let Some(timestamp) = timestamp_obj.get("int_1").and_then(|v: &Value| v.as_i64()) {
                    use chrono::DateTime;
                    if let Some(dt) = DateTime::from_timestamp(timestamp, 0) {
                        result["billing_start_time"] = json!(dt.format("%Y-%m-%d %H:%M:%S").to_string());
                        result["billing_start_timestamp"] = json!(timestamp);
                    }
                }
            }
            
            // billing_end (field 8): 计费结束时间
            if let Some(timestamp_obj) = billing_update.get("subMesssage_8") {
                if let Some(timestamp) = timestamp_obj.get("int_1").and_then(|v: &Value| v.as_i64()) {
                    use chrono::DateTime;
                    if let Some(dt) = DateTime::from_timestamp(timestamp, 0) {
                        result["next_billing_time"] = json!(dt.format("%Y-%m-%d %H:%M:%S").to_string());
                        result["next_billing_timestamp"] = json!(timestamp);
                    }
                }
            }
            
            // unused_plan_refunded (field 9)
            if let Some(refunded) = billing_update.get("int_9").and_then(|v: &Value| v.as_i64()) {
                result["unused_plan_refunded"] = json!(refunded == 1);
            }
            
            // has_sso_add_on (field 10)
            if let Some(sso) = billing_update.get("int_10").and_then(|v: &Value| v.as_i64()) {
                result["has_sso_add_on"] = json!(sso == 1);
            }
        }
        
        Ok(result)
    }
    
    /// 解析 UpdatePlan API 响应
    /// 
    /// UpdatePlanResponse 结构:
    /// - Field 1: billing_update (BillingUpdate)
    /// - Field 2: applied_changes (bool)
    /// - Field 3: next_action_client_secret (string)
    /// - Field 4: payment_failure_reason (string)
    /// - Field 5: requires_password_reset (bool)
    #[allow(dead_code)]
    pub fn parse_update_plan_response(response_body: &[u8]) -> Result<Value, String> {
        // 检查是否是base64编码的响应
        let decoded_body = if response_body.starts_with(b"data:application/proto;base64,") {
            let base64_str = std::str::from_utf8(&response_body[30..])
                .map_err(|e| format!("Invalid UTF-8: {}", e))?;
            general_purpose::STANDARD.decode(base64_str.trim())
                .map_err(|e| format!("Base64 decode error: {}", e))?
        } else {
            response_body.to_vec()
        };
        
        let mut parser = ProtobufParser::new(decoded_body);
        let parsed = parser.parse_message().map_err(|e| format!("Parse error: {}", e))?;
        
        let mut result = json!({
            "success": true,
            "raw_data": parsed.clone()
        });
        
        // Field 1: billing_update (BillingUpdate)
        if let Some(billing_update) = parsed.get("subMesssage_1") {
            let mut billing = json!({});
            
            // amount_due_immediately (field 1)
            if let Some(amount) = billing_update.get("float_1").and_then(|v: &Value| v.as_f64()) {
                billing["amount_due_immediately"] = json!(amount);
            }
            
            // price_per_seat (field 3)
            if let Some(price) = billing_update.get("float_3").and_then(|v: &Value| v.as_f64()) {
                billing["price_per_seat"] = json!(price);
            }
            
            // num_seats (field 4)
            if let Some(seats) = billing_update.get("int_4").and_then(|v: &Value| v.as_i64()) {
                billing["num_seats"] = json!(seats);
            }
            
            // sub_interval (field 5): 1=月度, 2=年度
            if let Some(interval) = billing_update.get("int_5").and_then(|v: &Value| v.as_i64()) {
                billing["sub_interval"] = json!(interval);
                billing["sub_interval_name"] = json!(if interval == 1 { "monthly" } else { "yearly" });
            }
            
            // amount_per_interval (field 6)
            if let Some(amount) = billing_update.get("float_6").and_then(|v: &Value| v.as_f64()) {
                billing["amount_per_interval"] = json!(amount);
            }
            
            // billing_start (field 7)
            if let Some(timestamp_obj) = billing_update.get("subMesssage_7") {
                if let Some(timestamp) = timestamp_obj.get("int_1").and_then(|v: &Value| v.as_i64()) {
                    use chrono::DateTime;
                    if let Some(dt) = DateTime::from_timestamp(timestamp, 0) {
                        billing["billing_start"] = json!(dt.format("%Y-%m-%d %H:%M:%S").to_string());
                        billing["billing_start_timestamp"] = json!(timestamp);
                    }
                }
            }
            
            // billing_end (field 8)
            if let Some(timestamp_obj) = billing_update.get("subMesssage_8") {
                if let Some(timestamp) = timestamp_obj.get("int_1").and_then(|v: &Value| v.as_i64()) {
                    use chrono::DateTime;
                    if let Some(dt) = DateTime::from_timestamp(timestamp, 0) {
                        billing["billing_end"] = json!(dt.format("%Y-%m-%d %H:%M:%S").to_string());
                        billing["billing_end_timestamp"] = json!(timestamp);
                    }
                }
            }
            
            // unused_plan_refunded (field 9)
            if let Some(refunded) = billing_update.get("int_9").and_then(|v: &Value| v.as_i64()) {
                billing["unused_plan_refunded"] = json!(refunded == 1);
            }
            
            // has_sso_add_on (field 10)
            if let Some(sso) = billing_update.get("int_10").and_then(|v: &Value| v.as_i64()) {
                billing["has_sso_add_on"] = json!(sso == 1);
            }
            
            result["billing_update"] = billing;
        }
        
        // Field 2: applied_changes (bool)
        if let Some(applied) = parsed.get("int_2").and_then(|v: &Value| v.as_i64()) {
            result["applied_changes"] = json!(applied == 1);
        }
        
        // Field 3: next_action_client_secret (string)
        if let Some(secret) = parsed.get("string_3").and_then(|v: &Value| v.as_str()) {
            result["next_action_client_secret"] = json!(secret);
        }
        
        // Field 4: payment_failure_reason (string)
        if let Some(reason) = parsed.get("string_4").and_then(|v: &Value| v.as_str()) {
            result["payment_failure_reason"] = json!(reason);
            result["success"] = json!(false); // 有支付失败原因表示失败
        }
        
        // Field 5: requires_password_reset (bool)
        if let Some(reset) = parsed.get("int_5").and_then(|v: &Value| v.as_i64()) {
            result["requires_password_reset"] = json!(reset == 1);
        }
        
        Ok(result)
    }
    
    #[allow(dead_code)]
    pub fn parse_get_team_billing_response(response_body: &[u8]) -> Result<Value, String> {
        // 检查是否是base64编码的响应
        let decoded_body = if response_body.starts_with(b"data:application/proto;base64,") {
            let base64_str = std::str::from_utf8(&response_body[30..])
                .map_err(|e| format!("Invalid UTF-8: {}", e))?;
            general_purpose::STANDARD.decode(base64_str.trim())
                .map_err(|e| format!("Base64 decode error: {}", e))?
        } else {
            response_body.to_vec()
        };
        
        let mut parser = ProtobufParser::new(decoded_body);
        let parsed = parser.parse_message().map_err(|e| format!("Parse error: {}", e))?;
        
        // 解析账单信息
        let mut billing_info = json!({
            "success": true,
            "raw_data": parsed.clone()
        });
        
        // 提取基础订阅信息 (fields 1-8)
        if let Some(active) = parsed.get("int_1").and_then(|v: &Value| v.as_i64()) {
            billing_info["subscription_active"] = json!(active == 1);
        }
        if let Some(trial) = parsed.get("int_2").and_then(|v: &Value| v.as_i64()) {
            billing_info["on_trial"] = json!(trial == 1);
        }
        if let Some(renewal_time) = parsed.get("subMesssage_3")
            .and_then(|v| v.get("int_1"))
            .and_then(|v: &Value| v.as_i64()) {
            use chrono::DateTime;
            if let Some(dt) = DateTime::from_timestamp(renewal_time, 0) {
                billing_info["subscription_renewal_time"] = json!(dt.format("%Y-%m-%d %H:%M:%S").to_string());
                billing_info["next_billing_date"] = json!(dt.format("%Y-%m-%d").to_string());
            }
        }
        if let Some(num_seats) = parsed.get("int_5").and_then(|v: &Value| v.as_i64()) {
            billing_info["num_seats"] = json!(num_seats);
        }
        if let Some(price) = parsed.get("float_6").and_then(|v: &Value| v.as_f64()) {
            billing_info["plan_unit_amount"] = json!(price);
            billing_info["monthly_price"] = json!(price); // 兼容性
        }
        if let Some(interval) = parsed.get("int_7").and_then(|v: &Value| v.as_i64()) {
            billing_info["sub_interval"] = json!(if interval == 1 { "monthly" } else { "yearly" });
        }
        if let Some(cancel) = parsed.get("int_8").and_then(|v: &Value| v.as_i64()) {
            billing_info["cancel_at_period_end"] = json!(cancel == 1);
        }
        
        // 提取用户和席位数 (fields 14-19)
        if let Some(num_users) = parsed.get("int_14").and_then(|v: &Value| v.as_i64()) {
            billing_info["num_users"] = json!(num_users);
        }
        if let Some(seats_current) = parsed.get("int_15").and_then(|v: &Value| v.as_i64()) {
            billing_info["num_seats_current_billing_period"] = json!(seats_current);
        }
        if let Some(cascade_users) = parsed.get("int_16").and_then(|v: &Value| v.as_i64()) {
            billing_info["num_cascade_users"] = json!(cascade_users);
        }
        if let Some(cascade_seats) = parsed.get("int_17").and_then(|v: &Value| v.as_i64()) {
            billing_info["num_cascade_seats"] = json!(cascade_seats);
        }
        if let Some(core_users) = parsed.get("int_18").and_then(|v: &Value| v.as_i64()) {
            billing_info["num_core_users"] = json!(core_users);
        }
        if let Some(core_seats) = parsed.get("int_19").and_then(|v: &Value| v.as_i64()) {
            billing_info["num_core_seats"] = json!(core_seats);
        }
        
        // 提取支付失败信息或发票URL (field 20)
        if let Some(failed_payment) = parsed.get("subMesssage_20") {
            if let Some(url_or_msg) = failed_payment.get("string_1").and_then(|v: &Value| v.as_str()) {
                // 判断是否是URL
                if url_or_msg.starts_with("http") {
                    billing_info["invoice_url"] = json!(url_or_msg);
                } else {
                    billing_info["failed_payment_message"] = json!(url_or_msg);
                }
            }
        }
        
        // 提取充值错误信息 (field 21)
        if let Some(top_up_error) = parsed.get("string_21").and_then(|v: &Value| v.as_str()) {
            billing_info["top_up_error"] = json!(top_up_error);
        }
        
        // 提取套餐信息
        if let Some(subscription) = parsed.get("subMesssage_12") {
            // 提取套餐名称
            if let Some(plan) = subscription.get("subMesssage_1") {
                if let Some(plan_name) = plan.get("string_2").and_then(|v: &Value| v.as_str()) {
                    billing_info["plan_name"] = json!(plan_name);
                }
                // 从套餐信息中提取基础配额
                if let Some(base_quota) = plan.get("int_12").and_then(|v: &Value| v.as_i64()) {
                    billing_info["base_quota"] = json!(base_quota);
                }
            }
            
            // 提取实际额度信息
            // int_4: 额外积分（赠送或购买的额外额度，可能不存在）
            // int_6: 使用积分（已使用额度，可能不存在）
            // int_8: 套餐额度（基础套餐额度）
            // int_9: 套餐缓存限额
            
            // 套餐基础额度（必须存在）
            let base_quota = subscription.get("int_8")
                .and_then(|v: &Value| v.as_i64())
                .unwrap_or(0);
            billing_info["base_quota"] = json!(base_quota);
            
            // 额外积分（可选，默认为0）
            let extra_credits = subscription.get("int_4")
                .and_then(|v: &Value| v.as_i64())
                .unwrap_or(0);
            if extra_credits > 0 {
                billing_info["extra_credits"] = json!(extra_credits);
            }
            
            // 总额度 = 套餐额度 + 额外积分
            let total_quota = base_quota + extra_credits;
            billing_info["total_quota"] = json!(total_quota);
            
            // 已使用额度（可选，默认为0）
            let used_quota = subscription.get("int_6")
                .and_then(|v: &Value| v.as_i64())
                .unwrap_or(0);
            billing_info["used_quota"] = json!(used_quota);
            
            // 缓存限额（最大缓存额度）
            let cache_limit = subscription.get("int_9")
                .and_then(|v: &Value| v.as_i64())
                .unwrap_or(total_quota); // 如果没有缓存限额，默认使用总额度
            billing_info["cache_limit"] = json!(cache_limit);
        }
        
        // 提取支付方式 (field 10)
        if let Some(payment) = parsed.get("subMesssage_10") {
            // PaymentMethod 嵌套在 subMesssage_2 中
            if let Some(payment_data) = payment.get("subMesssage_2") {
                let payment_info = json!({
                    "type": payment_data.get("string_1").and_then(|v: &Value| v.as_str()).unwrap_or("unknown"),
                    "last4": payment_data.get("string_4").and_then(|v: &Value| v.as_str()).unwrap_or(""),
                    "exp_month": payment_data.get("int_2").and_then(|v: &Value| v.as_i64()).unwrap_or(0),
                    "exp_year": payment_data.get("int_3").and_then(|v: &Value| v.as_i64()).unwrap_or(0)
                });
                billing_info["payment_method"] = payment_info;
            }
        }
        
        // 提取发票列表 (field 9) - 取第一个发票的URL
        if let Some(invoices) = parsed.get("subMesssage_9") {
            if let Some(invoice_url) = invoices.get("string_1").and_then(|v: &Value| v.as_str()) {
                billing_info["invoice_url"] = json!(invoice_url);
            }
        }
        
        Ok(billing_info)
    }
    
    /// 解析GetPlanStatus API响应
    /// 基于官方 windsurf-grpc proto 定义:
    /// - GetPlanStatusResponse { PlanStatus plan_status = 1 }
    /// - PlanStatus { PlanInfo=1, plan_start=2, plan_end=3, available_flex_credits=4, 
    ///   used_flow_credits=5, used_prompt_credits=6, used_flex_credits=7, 
    ///   available_prompt_credits=8, available_flow_credits=9, top_up_status=10 }
    /// - PlanInfo { teams_tier=1, plan_name=2, monthly_prompt_credits=12, monthly_flow_credits=13, ... }
    #[allow(dead_code)]
    pub fn parse_get_plan_status_response(response_body: &[u8]) -> Result<Value, String> {
        let decoded_body = if response_body.starts_with(b"data:application/proto;base64,") {
            let base64_str = std::str::from_utf8(&response_body[30..])
                .map_err(|e| format!("Invalid UTF-8: {}", e))?;
            general_purpose::STANDARD.decode(base64_str.trim())
                .map_err(|e| format!("Base64 decode error: {}", e))?
        } else {
            response_body.to_vec()
        };
        
        let mut parser = ProtobufParser::new(decoded_body);
        let parsed = parser.parse_message().map_err(|e| format!("Parse error: {}", e))?;
        
        let mut result = json!({
            "success": true,
            "raw_data": parsed.clone()
        });
        
        // 提取 PlanStatus (field 1)
        if let Some(plan_status) = parsed.get("subMesssage_1") {
            // 提取 PlanInfo (field 1)
            if let Some(plan_info) = plan_status.get("subMesssage_1") {
                // field 1: TeamsTier 枚举 (0=UNSPECIFIED, 1=TEAMS, 2=PRO, 3=ENTERPRISE_SAAS, ...)
                if let Some(tier) = plan_info.get("int_1").and_then(|v| v.as_i64()) {
                    result["teams_tier"] = json!(tier);
                    // 转换为可读名称
                    result["teams_tier_name"] = json!(match tier {
                        0 => "UNSPECIFIED",
                        1 => "TEAMS",
                        2 => "PRO",
                        3 => "ENTERPRISE_SAAS",
                        4 => "HYBRID",
                        5 => "ENTERPRISE_SELF_HOSTED",
                        6 => "WAITLIST_PRO",
                        7 => "TEAMS_ULTIMATE",
                        8 => "PRO_ULTIMATE",
                        9 => "TRIAL",
                        10 => "ENTERPRISE_SELF_SERVE",
                        _ => "UNKNOWN"
                    });
                }
                // field 2: plan_name
                if let Some(name) = plan_info.get("string_2").and_then(|v| v.as_str()) {
                    result["plan_name"] = json!(name);
                }
                // field 3: has_autocomplete_fast_mode
                if let Some(v) = plan_info.get("int_3").and_then(|v| v.as_i64()) {
                    result["has_autocomplete_fast_mode"] = json!(v != 0);
                }
                // field 4: allow_sticky_premium_models
                if let Some(v) = plan_info.get("int_4").and_then(|v| v.as_i64()) {
                    result["allow_sticky_premium_models"] = json!(v != 0);
                }
                // field 5: has_forge_access
                if let Some(v) = plan_info.get("int_5").and_then(|v| v.as_i64()) {
                    result["has_forge_access"] = json!(v != 0);
                }
                // field 6: max_num_premium_chat_messages
                if let Some(v) = plan_info.get("int_6").and_then(|v| v.as_i64()) {
                    result["max_num_premium_chat_messages"] = json!(v);
                }
                // field 7: max_num_chat_input_tokens
                if let Some(v) = plan_info.get("int_7").and_then(|v| v.as_i64()) {
                    result["max_num_chat_input_tokens"] = json!(v);
                }
                // field 8: max_custom_chat_instruction_characters
                if let Some(v) = plan_info.get("int_8").and_then(|v| v.as_i64()) {
                    result["max_custom_chat_instruction_characters"] = json!(v);
                }
                // field 9: max_num_pinned_context_items
                if let Some(v) = plan_info.get("int_9").and_then(|v| v.as_i64()) {
                    result["max_num_pinned_context_items"] = json!(v);
                }
                // field 10: max_local_index_size
                if let Some(v) = plan_info.get("int_10").and_then(|v| v.as_i64()) {
                    result["max_local_index_size"] = json!(v);
                }
                // field 11: disable_code_snippet_telemetry
                if let Some(v) = plan_info.get("int_11").and_then(|v| v.as_i64()) {
                    result["disable_code_snippet_telemetry"] = json!(v != 0);
                }
                // field 12: monthly_prompt_credits
                if let Some(v) = plan_info.get("int_12").and_then(|v| v.as_i64()) {
                    result["monthly_prompt_credits"] = json!(v);
                }
                // field 13: monthly_flow_credits
                if let Some(v) = plan_info.get("int_13").and_then(|v| v.as_i64()) {
                    result["monthly_flow_credits"] = json!(v);
                }
                // field 14: monthly_flex_credit_purchase_amount
                if let Some(v) = plan_info.get("int_14").and_then(|v| v.as_i64()) {
                    result["monthly_flex_credit_purchase_amount"] = json!(v);
                }
                // field 15: allow_premium_command_models
                if let Some(v) = plan_info.get("int_15").and_then(|v| v.as_i64()) {
                    result["allow_premium_command_models"] = json!(v != 0);
                }
                // field 16: is_enterprise
                if let Some(v) = plan_info.get("int_16").and_then(|v| v.as_i64()) {
                    result["is_enterprise"] = json!(v != 0);
                }
                // field 17: is_teams
                if let Some(v) = plan_info.get("int_17").and_then(|v| v.as_i64()) {
                    result["is_teams"] = json!(v != 0);
                }
                // field 18: can_buy_more_credits
                if let Some(v) = plan_info.get("int_18").and_then(|v| v.as_i64()) {
                    result["can_buy_more_credits"] = json!(v != 0);
                }
                // field 19: cascade_web_search_enabled
                if let Some(v) = plan_info.get("int_19").and_then(|v| v.as_i64()) {
                    result["cascade_web_search_enabled"] = json!(v != 0);
                }
                // field 22: cascade_can_auto_run_commands
                if let Some(v) = plan_info.get("int_22").and_then(|v| v.as_i64()) {
                    result["cascade_can_auto_run_commands"] = json!(v != 0);
                }
                // field 23: has_tab_to_jump
                if let Some(v) = plan_info.get("int_23").and_then(|v| v.as_i64()) {
                    result["has_tab_to_jump"] = json!(v != 0);
                }
                // field 25: can_generate_commit_messages
                if let Some(v) = plan_info.get("int_25").and_then(|v| v.as_i64()) {
                    result["can_generate_commit_messages"] = json!(v != 0);
                }
                // field 26: max_unclaimed_sites
                if let Some(v) = plan_info.get("int_26").and_then(|v| v.as_i64()) {
                    result["max_unclaimed_sites"] = json!(v);
                }
                // field 27: knowledge_base_enabled
                if let Some(v) = plan_info.get("int_27").and_then(|v| v.as_i64()) {
                    result["knowledge_base_enabled"] = json!(v != 0);
                }
                // field 28: can_share_conversations
                if let Some(v) = plan_info.get("int_28").and_then(|v| v.as_i64()) {
                    result["can_share_conversations"] = json!(v != 0);
                }
                // field 29: can_allow_cascade_in_background
                if let Some(v) = plan_info.get("int_29").and_then(|v| v.as_i64()) {
                    result["can_allow_cascade_in_background"] = json!(v != 0);
                }
                // field 31: browser_enabled
                if let Some(v) = plan_info.get("int_31").and_then(|v| v.as_i64()) {
                    result["browser_enabled"] = json!(v != 0);
                }
            }
            
            // 提取计费周期 (Timestamp 类型)
            // field 2: plan_start
            if let Some(start) = plan_status.get("subMesssage_2")
                .and_then(|v| v.get("int_1"))
                .and_then(|v| v.as_i64()) {
                result["plan_start"] = json!(start);
            }
            // field 3: plan_end
            if let Some(end) = plan_status.get("subMesssage_3")
                .and_then(|v| v.get("int_1"))
                .and_then(|v| v.as_i64()) {
                result["plan_end"] = json!(end);
            }
            
            // 提取积分信息
            // field 4: available_flex_credits
            if let Some(v) = plan_status.get("int_4").and_then(|v| v.as_i64()) {
                result["available_flex_credits"] = json!(v);
            }
            // field 5: used_flow_credits
            if let Some(v) = plan_status.get("int_5").and_then(|v| v.as_i64()) {
                result["used_flow_credits"] = json!(v);
            }
            // field 6: used_prompt_credits
            if let Some(v) = plan_status.get("int_6").and_then(|v| v.as_i64()) {
                result["used_prompt_credits"] = json!(v);
            }
            // field 7: used_flex_credits
            if let Some(v) = plan_status.get("int_7").and_then(|v| v.as_i64()) {
                result["used_flex_credits"] = json!(v);
            }
            // field 8: available_prompt_credits
            if let Some(v) = plan_status.get("int_8").and_then(|v| v.as_i64()) {
                result["available_prompt_credits"] = json!(v);
            }
            // field 9: available_flow_credits
            if let Some(v) = plan_status.get("int_9").and_then(|v| v.as_i64()) {
                result["available_flow_credits"] = json!(v);
            }
            
            // field 10: TopUpStatus (子消息)
            if let Some(top_up) = plan_status.get("subMesssage_10") {
                if let Some(status) = top_up.get("int_1").and_then(|v| v.as_i64()) {
                    result["top_up_status"] = json!(status);
                }
            }
        }
        
        Ok(result)
    }
    
    /// 解析GetUsers API响应
    #[allow(dead_code)]
    pub fn parse_get_users_response(response_body: &[u8]) -> Result<Value, String> {
        let decoded_body = if response_body.starts_with(b"data:application/proto;base64,") {
            let base64_str = std::str::from_utf8(&response_body[30..])
                .map_err(|e| format!("Invalid UTF-8: {}", e))?;
            general_purpose::STANDARD.decode(base64_str.trim())
                .map_err(|e| format!("Base64 decode error: {}", e))?
        } else {
            response_body.to_vec()
        };
        
        let mut parser = ProtobufParser::new(decoded_body);
        let parsed = parser.parse_message().map_err(|e| format!("Parse error: {}", e))?;
        
        let mut users_list = Vec::new();
        let mut roles_list = Vec::new();
        let mut cascade_details = Vec::new();
        
        // 解析用户数组
        for i in 1..100 {  // 假设最多100个用户
            let field_name = format!("subMesssage_{}", i);
            if let Some(item) = parsed.get(&field_name) {
                // 检查是否是User对象 (包含api_key, name, email等)
                if item.get("string_1").is_some() && item.get("string_3").is_some() {
                    let mut user = json!({});
                    
                    // 提取用户基本信息
                    if let Some(api_key) = item.get("string_1").and_then(|v| v.as_str()) {
                        user["api_key"] = json!(api_key);
                    }
                    if let Some(name) = item.get("string_2").and_then(|v| v.as_str()) {
                        user["name"] = json!(name);
                    }
                    if let Some(email) = item.get("string_3").and_then(|v| v.as_str()) {
                        user["email"] = json!(email);
                    }
                    if let Some(id) = item.get("string_6").and_then(|v| v.as_str()) {
                        user["firebase_id"] = json!(id);
                    }
                    if let Some(team_id) = item.get("string_7").and_then(|v| v.as_str()) {
                        user["team_id"] = json!(team_id);
                    }
                    if let Some(status) = item.get("int_8").and_then(|v| v.as_i64()) {
                        user["team_status"] = json!(status);
                    }
                    if let Some(username) = item.get("string_9").and_then(|v| v.as_str()) {
                        user["username"] = json!(username);
                    }
                    if let Some(timezone) = item.get("string_10").and_then(|v| v.as_str()) {
                        user["timezone"] = json!(timezone);
                    }
                    if let Some(referral) = item.get("string_30").and_then(|v| v.as_str()) {
                        user["referral_code"] = json!(referral);
                    }
                    
                    // 判断类型
                    if user.get("email").is_some() {
                        users_list.push(user);
                    } else if item.get("string_4").is_some() {  // UserRole
                        let role = json!({
                            "api_key": item.get("string_1").and_then(|v| v.as_str()),
                            "roles": item.get("string_2").and_then(|v| v.as_str()),
                            "role_id": item.get("string_3").and_then(|v| v.as_str()),
                            "role_name": item.get("string_4").and_then(|v| v.as_str()),
                        });
                        roles_list.push(role);
                    }
                } else if item.get("int_2").is_some() && item.get("string_1").is_some() {
                    // UserCascadeDetails
                    cascade_details.push(json!({
                        "user_id": item.get("string_1").and_then(|v| v.as_str()),
                        "usage_amount": item.get("int_2").and_then(|v| v.as_i64()),
                    }));
                }
            }
        }
        
        Ok(json!({
            "success": true,
            "users": users_list,
            "user_roles": roles_list,
            "user_cascade_details": cascade_details,
            "raw_data": parsed
        }))
    }
    
    /// 解析GetTeamCreditEntries API响应
    pub fn parse_get_team_credit_entries_response(response_body: &[u8]) -> Result<Value, String> {
        let decoded_body = if response_body.starts_with(b"data:application/proto;base64,") {
            let base64_str = std::str::from_utf8(&response_body[30..])
                .map_err(|e| format!("Invalid UTF-8: {}", e))?;
            general_purpose::STANDARD.decode(base64_str.trim())
                .map_err(|e| format!("Base64 decode error: {}", e))?
        } else {
            response_body.to_vec()
        };
        
        let mut parser = ProtobufParser::new(decoded_body);
        let parsed = parser.parse_message().map_err(|e| format!("Parse error: {}", e))?;
        
        let mut entries = Vec::new();
        
        // 解析积分记录数组 (field 1 is repeated FlexCreditChronicleEntry)
        // 首先尝试作为数组处理
        if let Some(entries_value) = parsed.get("subMesssage_1") {
            // 如果是数组
            if let Value::Array(entries_array) = entries_value {
            for entry_value in entries_array {
                if let Value::Object(entry) = entry_value {
                    let mut credit_entry = json!({});
                    
                    // team_id (field 1)
                    if let Some(team_id) = entry.get("string_1").and_then(|v| v.as_str()) {
                        credit_entry["team_id"] = json!(team_id);
                    }
                    
                    // grant_date (field 2) - timestamp with seconds and nanos
                    if let Some(date_msg) = entry.get("subMesssage_2") {
                        if let Some(seconds) = date_msg.get("int_1").and_then(|v| v.as_i64()) {
                            // 转换为日期字符串
                            use chrono::DateTime;
                            if let Some(dt) = DateTime::from_timestamp(seconds, 0) {
                                credit_entry["grant_date"] = json!(dt.format("%Y-%m-%d %H:%M:%S").to_string());
                                credit_entry["grant_date_timestamp"] = json!(seconds);
                            }
                        }
                    }
                    
                    // num_credits (field 3)
                    if let Some(credits) = entry.get("int_3").and_then(|v| v.as_i64()) {
                        credit_entry["num_credits"] = json!(credits);
                    }
                    
                    // type (field 4): 1=FLEX, 2=PROMPT, 3=FLOW
                    if let Some(credit_type) = entry.get("int_4").and_then(|v| v.as_i64()) {
                        let type_name = match credit_type {
                            1 => "FLEX",
                            2 => "PROMPT",
                            3 => "FLOW",
                            _ => "UNKNOWN"
                        };
                        credit_entry["type"] = json!(type_name);
                        credit_entry["type_code"] = json!(credit_type);
                    }
                    
                    // referral_id (field 5)
                    if let Some(referral_id) = entry.get("int_5").and_then(|v| v.as_i64()) {
                        credit_entry["referral_id"] = json!(referral_id);
                    }
                    
                    // invoice_id (field 6)
                    if let Some(invoice_id) = entry.get("string_6").and_then(|v| v.as_str()) {
                        credit_entry["invoice_id"] = json!(invoice_id);
                    }
                    
                    // reason: referrer (field 7) - oneof reason
                    if let Some(referrer) = entry.get("subMesssage_7") {
                        let mut reason = json!({
                            "type": "referrer"
                        });
                        if let Some(user_email) = referrer.get("string_1").and_then(|v| v.as_str()) {
                            reason["referrer_email"] = json!(user_email);
                        }
                        if let Some(avery_email) = referrer.get("string_2").and_then(|v| v.as_str()) {
                            reason["referred_email"] = json!(avery_email);
                        }
                        credit_entry["reason"] = reason;
                    }
                    
                    // reason: avery (field 8)
                    else if let Some(avery) = entry.get("subMesssage_8") {
                        let mut reason = json!({
                            "type": "avery"
                        });
                        if let Some(user_email) = avery.get("string_1").and_then(|v| v.as_str()) {
                            reason["user_email"] = json!(user_email);
                        }
                        credit_entry["reason"] = reason;
                    }
                    
                    // reason: purchase (field 9)
                    else if let Some(purchase) = entry.get("subMesssage_9") {
                        let mut reason = json!({
                            "type": "purchase"
                        });
                        if let Some(purchase_type) = purchase.get("int_1").and_then(|v| v.as_i64()) {
                            reason["purchase_type"] = json!(purchase_type);
                        }
                        credit_entry["reason"] = reason;
                    }
                    
                    if !credit_entry.is_null() && credit_entry.get("team_id").is_some() {
                        entries.push(credit_entry);
                    }
                }
            }
            }
            // 如果不是数组，可能是单个对象
            else if let Value::Object(entry) = entries_value {
                let mut credit_entry = json!({});
                
                // team_id (field 1)
                if let Some(team_id) = entry.get("string_1").and_then(|v| v.as_str()) {
                    credit_entry["team_id"] = json!(team_id);
                }
                
                // grant_date (field 2)
                if let Some(date_msg) = entry.get("subMesssage_2") {
                    if let Some(seconds) = date_msg.get("int_1").and_then(|v| v.as_i64()) {
                        use chrono::DateTime;
                        if let Some(dt) = DateTime::from_timestamp(seconds, 0) {
                            credit_entry["grant_date"] = json!(dt.format("%Y-%m-%d %H:%M:%S").to_string());
                            credit_entry["grant_date_timestamp"] = json!(seconds);
                        }
                    }
                }
                
                // num_credits (field 3)
                if let Some(credits) = entry.get("int_3").and_then(|v| v.as_i64()) {
                    credit_entry["num_credits"] = json!(credits);
                }
                
                // type (field 4)
                if let Some(credit_type) = entry.get("int_4").and_then(|v| v.as_i64()) {
                    let type_name = match credit_type {
                        1 => "FLEX",
                        2 => "PROMPT",
                        3 => "FLOW",
                        _ => "UNKNOWN"
                    };
                    credit_entry["type"] = json!(type_name);
                    credit_entry["type_code"] = json!(credit_type);
                }
                
                // referral_id (field 5)
                if let Some(referral_id) = entry.get("int_5").and_then(|v| v.as_i64()) {
                    credit_entry["referral_id"] = json!(referral_id);
                }
                
                // reason: referrer (field 7)
                if let Some(referrer) = entry.get("subMesssage_7") {
                    let mut reason = json!({
                        "type": "referrer"
                    });
                    if let Some(user_email) = referrer.get("string_1").and_then(|v| v.as_str()) {
                        reason["referrer_email"] = json!(user_email);
                    }
                    if let Some(avery_email) = referrer.get("string_2").and_then(|v| v.as_str()) {
                        reason["referred_email"] = json!(avery_email);
                    }
                    credit_entry["reason"] = reason;
                }
                // reason: avery (field 8)
                else if let Some(avery) = entry.get("subMesssage_8") {
                    let mut reason = json!({
                        "type": "avery"
                    });
                    if let Some(user_email) = avery.get("string_1").and_then(|v| v.as_str()) {
                        reason["avery_email"] = json!(user_email);
                    }
                    if let Some(target_email) = avery.get("string_2").and_then(|v| v.as_str()) {
                        reason["target_email"] = json!(target_email);
                    }
                    credit_entry["reason"] = reason;
                }
                // reason: purchase (field 9)
                else if let Some(purchase) = entry.get("subMesssage_9") {
                    let mut reason = json!({
                        "type": "purchase"
                    });
                    if let Some(purchase_type) = purchase.get("int_1").and_then(|v| v.as_i64()) {
                        reason["purchase_type"] = json!(purchase_type);
                    }
                    credit_entry["reason"] = reason;
                }
                
                if !credit_entry.is_null() && credit_entry.get("team_id").is_some() {
                    entries.push(credit_entry);
                }
            }
        }
        // 处理多个单独的字段（如 subMesssage_1, subMesssage_2, ...)
        else {
            for i in 1..100 {  
                let field_name = format!("subMesssage_{}", i);
                if let Some(entry) = parsed.get(&field_name) {
                    let mut credit_entry = json!({});
                    
                    // team_id (field 1)
                    if let Some(team_id) = entry.get("string_1").and_then(|v| v.as_str()) {
                        credit_entry["team_id"] = json!(team_id);
                    }
                    
                    // grant_date (field 2)
                    if let Some(date_msg) = entry.get("subMesssage_2") {
                        if let Some(seconds) = date_msg.get("int_1").and_then(|v| v.as_i64()) {
                            use chrono::DateTime;
                            if let Some(dt) = DateTime::from_timestamp(seconds, 0) {
                                credit_entry["grant_date"] = json!(dt.format("%Y-%m-%d %H:%M:%S").to_string());
                                credit_entry["grant_date_timestamp"] = json!(seconds);
                            }
                        }
                    }
                    
                    // num_credits (field 3)
                    if let Some(credits) = entry.get("int_3").and_then(|v| v.as_i64()) {
                        credit_entry["num_credits"] = json!(credits);
                    }
                    
                    // type (field 4)
                    if let Some(credit_type) = entry.get("int_4").and_then(|v| v.as_i64()) {
                        let type_name = match credit_type {
                            1 => "FLEX",
                            2 => "PROMPT",
                            3 => "FLOW",
                            _ => "UNKNOWN"
                        };
                        credit_entry["type"] = json!(type_name);
                        credit_entry["type_code"] = json!(credit_type);
                    }
                    
                    // referral_id (field 5)
                    if let Some(referral_id) = entry.get("int_5").and_then(|v| v.as_i64()) {
                        credit_entry["referral_id"] = json!(referral_id);
                    }
                    
                    // reason (field 7)
                    if let Some(referrer) = entry.get("subMesssage_7") {
                        let mut reason = json!({
                            "type": "referrer"
                        });
                        if let Some(user_email) = referrer.get("string_1").and_then(|v| v.as_str()) {
                            reason["referrer_email"] = json!(user_email);
                        }
                        if let Some(avery_email) = referrer.get("string_2").and_then(|v| v.as_str()) {
                            reason["referred_email"] = json!(avery_email);
                        }
                        credit_entry["reason"] = reason;
                    }
                    
                    if !credit_entry.is_null() && credit_entry.get("team_id").is_some() {
                        entries.push(credit_entry);
                    }
                }
            }
        }
        
        Ok(json!({
            "success": true,
            "entries": entries,
            "total_entries": entries.len(),
            "raw_data": parsed
        }))
    }
}

/// 解析 GetAnalytics API 响应
pub fn parse_get_analytics_response(response_body: &[u8]) -> Result<Value, String> {
    // 处理 base64 编码的响应
    let decoded_body = if response_body.starts_with(b"data:application/proto;base64,") {
        let base64_str = std::str::from_utf8(&response_body[30..])
            .map_err(|e| format!("Invalid UTF-8: {}", e))?;
        general_purpose::STANDARD.decode(base64_str.trim())
            .map_err(|e| format!("Base64 decode error: {}", e))?
    } else {
        response_body.to_vec()
    };

    let mut parser = ProtobufParser::new(decoded_body);
    let parsed = parser.parse_message().map_err(|e| format!("Parse error: {}", e))?;

    println!("[parse_get_analytics_response] Parsed data: {}", serde_json::to_string_pretty(&parsed).unwrap_or_default());

    Ok(json!({
        "success": true,
        "raw_data": parsed.clone(),
        "parsed_data": parsed
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_protobuf() {
        let base64_data = "CqECCiQ2N2Q5ZjIzNi1hNDBhLTRiYzUtYjRjMi1kZmViZWJmMzdjNjMSC2NoamFvaSB3YW5nGhAzMDI3MTgyNDlAcXEuY29t";

        let result = ProtobufParser::from_base64(base64_data);
        assert!(result.is_ok());

        let parsed = result.unwrap();
        assert!(parsed.is_object());
    }
}
