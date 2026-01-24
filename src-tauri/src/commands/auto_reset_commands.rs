use crate::models::{AutoResetConfig, OperationLog, OperationType, OperationStatus, ResetRecord, AccountResetStats};
use crate::repository::DataStore;
use crate::services::WindsurfService;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use tauri::{State, Manager};
use chrono::Utc;
use std::fs;
use std::path::PathBuf;
use tokio::sync::RwLock;

/// 自动重置配置存储
pub struct AutoResetStore {
    configs: Arc<RwLock<Vec<AutoResetConfig>>>,
    config_path: PathBuf,
}

impl AutoResetStore {
    pub fn new(app_handle: &tauri::AppHandle) -> Result<Self, String> {
        let app_data_dir = app_handle.path().app_data_dir()
            .map_err(|e| format!("Failed to get app data dir: {}", e))?;
        
        fs::create_dir_all(&app_data_dir).map_err(|e| e.to_string())?;
        
        let config_path = app_data_dir.join("auto_reset_configs.json");
        let configs = Self::load_configs(&config_path)?;
        
        Ok(Self {
            configs: Arc::new(RwLock::new(configs)),
            config_path,
        })
    }
    
    fn load_configs(path: &PathBuf) -> Result<Vec<AutoResetConfig>, String> {
        if path.exists() {
            let data = fs::read_to_string(path).map_err(|e| e.to_string())?;
            match serde_json::from_str(&data) {
                Ok(configs) => Ok(configs),
                Err(e) => {
                    // 如果解析失败，删除损坏的配置文件并返回空数组
                    println!("[AutoResetStore] 配置文件格式错误，将重置: {}", e);
                    let _ = fs::remove_file(path);
                    Ok(Vec::new())
                }
            }
        } else {
            Ok(Vec::new())
        }
    }
    
    pub async fn save(&self) -> Result<(), String> {
        let configs = self.configs.read().await;
        let data = serde_json::to_string_pretty(&*configs).map_err(|e| e.to_string())?;
        fs::write(&self.config_path, data).map_err(|e| e.to_string())
    }
    
    pub async fn get_all(&self) -> Vec<AutoResetConfig> {
        self.configs.read().await.clone()
    }
    
    pub async fn get_by_id(&self, id: &str) -> Option<AutoResetConfig> {
        self.configs.read().await.iter().find(|c| c.id == id).cloned()
    }
    
    pub async fn add(&self, config: AutoResetConfig) -> Result<AutoResetConfig, String> {
        let mut configs = self.configs.write().await;
        
        // 检查是否已存在相同目标的配置
        if configs.iter().any(|c| c.target_type == config.target_type && c.target_id == config.target_id) {
            return Err(format!("配置已存在: {} - {}", config.target_type, config.target_id));
        }
        
        configs.push(config.clone());
        drop(configs);
        self.save().await?;
        Ok(config)
    }
    
    pub async fn update(&self, config: AutoResetConfig) -> Result<AutoResetConfig, String> {
        let mut configs = self.configs.write().await;
        
        if let Some(pos) = configs.iter().position(|c| c.id == config.id) {
            configs[pos] = config.clone();
            drop(configs);
            self.save().await?;
            Ok(config)
        } else {
            Err("配置不存在".to_string())
        }
    }
    
    pub async fn delete(&self, id: &str) -> Result<(), String> {
        let mut configs = self.configs.write().await;
        
        if let Some(pos) = configs.iter().position(|c| c.id == id) {
            configs.remove(pos);
            drop(configs);
            self.save().await?;
            Ok(())
        } else {
            Err("配置不存在".to_string())
        }
    }
    
    pub async fn update_check_time(&self, id: &str) -> Result<(), String> {
        let mut configs = self.configs.write().await;
        
        if let Some(pos) = configs.iter().position(|c| c.id == id) {
            configs[pos].last_check_at = Some(Utc::now());
            drop(configs);
            self.save().await?;
            Ok(())
        } else {
            Err("配置不存在".to_string())
        }
    }
    
    pub async fn update_reset_time(&self, id: &str) -> Result<(), String> {
        let mut configs = self.configs.write().await;
        
        if let Some(pos) = configs.iter().position(|c| c.id == id) {
            configs[pos].last_reset_at = Some(Utc::now());
            drop(configs);
            self.save().await?;
            Ok(())
        } else {
            Err("配置不存在".to_string())
        }
    }
}

/// 重置记录存储
pub struct ResetRecordStore {
    records: Arc<RwLock<Vec<ResetRecord>>>,
    records_path: PathBuf,
}

impl ResetRecordStore {
    pub fn new(app_handle: &tauri::AppHandle) -> Result<Self, String> {
        let app_data_dir = app_handle.path().app_data_dir()
            .map_err(|e| format!("Failed to get app data dir: {}", e))?;
        
        fs::create_dir_all(&app_data_dir).map_err(|e| e.to_string())?;
        
        let records_path = app_data_dir.join("reset_records.json");
        let records = Self::load_records(&records_path)?;
        
        Ok(Self {
            records: Arc::new(RwLock::new(records)),
            records_path,
        })
    }
    
    fn load_records(path: &PathBuf) -> Result<Vec<ResetRecord>, String> {
        if path.exists() {
            let data = fs::read_to_string(path).map_err(|e| e.to_string())?;
            match serde_json::from_str(&data) {
                Ok(records) => Ok(records),
                Err(e) => {
                    println!("[ResetRecordStore] 记录文件格式错误，将重置: {}", e);
                    let _ = fs::remove_file(path);
                    Ok(Vec::new())
                }
            }
        } else {
            Ok(Vec::new())
        }
    }
    
    pub async fn save(&self) -> Result<(), String> {
        let records = self.records.read().await;
        let data = serde_json::to_string_pretty(&*records).map_err(|e| e.to_string())?;
        fs::write(&self.records_path, data).map_err(|e| e.to_string())
    }
    
    pub async fn add(&self, record: ResetRecord) -> Result<ResetRecord, String> {
        let mut records = self.records.write().await;
        records.push(record.clone());
        drop(records);
        self.save().await?;
        Ok(record)
    }
    
    #[allow(dead_code)]
    pub async fn get_all(&self) -> Result<Vec<ResetRecord>, String> {
        Ok(self.records.read().await.clone())
    }
    
    pub async fn get_paginated(&self, page: usize, page_size: usize) -> (Vec<ResetRecord>, usize) {
        let records = self.records.read().await;
        let total = records.len();
        let start = (page - 1) * page_size;
        let end = (start + page_size).min(total);
        
        if start >= total {
            return (Vec::new(), total);
        }
        
        // 按时间倒序排列
        let mut sorted: Vec<_> = records.iter().cloned().collect();
        sorted.sort_by(|a, b| b.reset_at.cmp(&a.reset_at));
        
        (sorted[start..end].to_vec(), total)
    }
    
    pub async fn get_stats(&self) -> Vec<AccountResetStats> {
        let records = self.records.read().await;
        let mut stats_map: std::collections::HashMap<String, AccountResetStats> = std::collections::HashMap::new();
        
        for record in records.iter() {
            let stats = stats_map.entry(record.account_email.clone()).or_insert_with(|| {
                AccountResetStats::new(
                    record.account_id.clone(),
                    record.account_email.clone(),
                    record.account_nickname.clone(),
                )
            });
            stats.add_reset(record.used_quota_before, record.reset_at);
        }
        
        let mut result: Vec<_> = stats_map.into_values().collect();
        result.sort_by(|a, b| b.reset_count.cmp(&a.reset_count));
        result
    }
    
    pub async fn clear_all(&self) -> Result<(), String> {
        let mut records = self.records.write().await;
        records.clear();
        drop(records);
        self.save().await
    }
}

/// 获取所有自动重置配置
#[tauri::command]
pub async fn get_auto_reset_configs(
    store: State<'_, Arc<AutoResetStore>>,
) -> Result<Vec<AutoResetConfig>, String> {
    Ok(store.get_all().await)
}

/// 添加自动重置配置
#[tauri::command]
pub async fn add_auto_reset_config(
    target_type: String,
    target_id: String,
    check_interval: i32,
    usage_threshold: i32,
    remaining_threshold: i32,
    store: State<'_, Arc<AutoResetStore>>,
    data_store: State<'_, Arc<DataStore>>,
) -> Result<AutoResetConfig, String> {
    let mut config = AutoResetConfig::new(target_type.clone(), target_id.clone());
    config.check_interval = check_interval;
    config.usage_threshold = usage_threshold;
    config.remaining_threshold = remaining_threshold;
    
    let result = store.add(config).await?;
    
    // 记录日志
    let log = OperationLog::new(
        OperationType::BatchOperation,
        OperationStatus::Success,
        format!("添加自动重置配置: {} - {}", target_type, target_id),
    );
    let _ = data_store.add_log(log).await;
    
    Ok(result)
}

/// 更新自动重置配置
#[tauri::command]
pub async fn update_auto_reset_config(
    id: String,
    enabled: Option<bool>,
    check_interval: Option<i32>,
    usage_threshold: Option<i32>,
    remaining_threshold: Option<i32>,
    store: State<'_, Arc<AutoResetStore>>,
) -> Result<AutoResetConfig, String> {
    let mut config = store.get_by_id(&id).await.ok_or("配置不存在")?;
    
    if let Some(v) = enabled {
        config.enabled = v;
    }
    if let Some(v) = check_interval {
        config.check_interval = v;
    }
    if let Some(v) = usage_threshold {
        config.usage_threshold = v;
    }
    if let Some(v) = remaining_threshold {
        config.remaining_threshold = v;
    }
    
    store.update(config).await
}

/// 删除自动重置配置
#[tauri::command]
pub async fn delete_auto_reset_config(
    id: String,
    store: State<'_, Arc<AutoResetStore>>,
    data_store: State<'_, Arc<DataStore>>,
) -> Result<(), String> {
    let config = store.get_by_id(&id).await.ok_or("配置不存在")?;
    store.delete(&id).await?;
    
    // 记录日志
    let log = OperationLog::new(
        OperationType::BatchOperation,
        OperationStatus::Success,
        format!("删除自动重置配置: {} - {}", config.target_type, config.target_id),
    );
    let _ = data_store.add_log(log).await;
    
    Ok(())
}

/// 检查并执行自动重置（由前端定时调用）
/// 只检测团队成员的积分，不检测主号
#[tauri::command]
pub async fn check_and_auto_reset(
    config_id: String,
    auto_reset_store: State<'_, Arc<AutoResetStore>>,
    data_store: State<'_, Arc<DataStore>>,
    record_store: State<'_, Arc<ResetRecordStore>>,
) -> Result<serde_json::Value, String> {
    let config = auto_reset_store.get_by_id(&config_id).await.ok_or("配置不存在")?;
    
    if !config.enabled {
        return Ok(json!({ "skipped": true, "reason": "配置已禁用" }));
    }
    
    // 更新检查时间
    auto_reset_store.update_check_time(&config_id).await?;
    
    // 获取目标账号列表（这些是主号，用于获取团队成员）
    let master_accounts = if config.target_type == "group" {
        data_store.get_all_accounts().await
            .map_err(|e| e.to_string())?
            .into_iter()
            .filter(|a| a.group.as_ref().map(|g| g == &config.target_id).unwrap_or(false))
            .collect::<Vec<_>>()
    } else {
        // 单个账号
        let uuid = uuid::Uuid::parse_str(&config.target_id)
            .map_err(|_| "无效的账号ID".to_string())?;
        match data_store.get_account(uuid).await {
            Ok(account) => vec![account],
            Err(_) => return Err("账号不存在".to_string()),
        }
    };
    
    if master_accounts.is_empty() {
        return Ok(json!({ "skipped": true, "reason": "没有符合条件的主号" }));
    }
    
    let windsurf_service = WindsurfService::new();
    let mut checked_count = 0;
    let mut reset_count = 0;
    let mut results = Vec::new();
    
    // 遍历每个主号，获取其团队成员并检测
    for master_account in master_accounts {
        // 检查主号是否有有效的 token
        let master_token = match &master_account.token {
            Some(t) if !t.is_empty() => t.clone(),
            _ => {
                results.push(json!({
                    "master_account_id": master_account.id,
                    "master_email": master_account.email,
                    "skipped": true,
                    "reason": "主号无有效Token"
                }));
                continue;
            }
        };
        
        // 获取团队成员列表和使用量数据
        let (team_members, cascade_details) = match windsurf_service.get_team_members(&master_token, None).await {
            Ok(result) => {
                if result.get("success").and_then(|v| v.as_bool()).unwrap_or(false) {
                    let data = result.get("data").cloned().unwrap_or(json!({}));
                    
                    // subMesssage_1 是用户列表
                    let users = data.get("subMesssage_1").cloned();
                    let users_vec = match users {
                        Some(serde_json::Value::Array(arr)) => arr,
                        Some(obj @ serde_json::Value::Object(_)) => vec![obj],
                        _ => {
                            let alt_users = data.get("subMessage_1").cloned();
                            match alt_users {
                                Some(serde_json::Value::Array(arr)) => arr,
                                Some(obj @ serde_json::Value::Object(_)) => vec![obj],
                                _ => Vec::new()
                            }
                        }
                    };
                    
                    // subMesssage_4 是 UserCascadeDetails（包含积分使用情况）
                    let cascade = data.get("subMesssage_4").cloned();
                    let cascade_vec = match cascade {
                        Some(serde_json::Value::Array(arr)) => arr,
                        Some(obj @ serde_json::Value::Object(_)) => vec![obj],
                        _ => {
                            let alt_cascade = data.get("subMessage_4").cloned();
                            match alt_cascade {
                                Some(serde_json::Value::Array(arr)) => arr,
                                Some(obj @ serde_json::Value::Object(_)) => vec![obj],
                                _ => Vec::new()
                            }
                        }
                    };
                    
                    (users_vec, cascade_vec)
                } else {
                    results.push(json!({
                        "master_account_id": master_account.id,
                        "master_email": master_account.email,
                        "skipped": true,
                        "reason": "获取团队成员失败"
                    }));
                    continue;
                }
            }
            Err(e) => {
                results.push(json!({
                    "master_account_id": master_account.id,
                    "master_email": master_account.email,
                    "skipped": true,
                    "reason": format!("获取团队成员失败: {}", e)
                }));
                continue;
            }
        };
        
        if team_members.is_empty() {
            results.push(json!({
                "master_account_id": master_account.id,
                "master_email": master_account.email,
                "skipped": true,
                "reason": "没有团队成员"
            }));
            continue;
        }
        
        // 获取主号的配额信息（只获取一次）
        let total_quota = match windsurf_service.get_plan_status(&master_token).await {
            Ok(plan_status) => {
                plan_status.get("plan_status")
                    .and_then(|ps| ps.get("total_quota"))
                    .and_then(|v| v.as_i64())
                    .unwrap_or(9500) as i32
            }
            Err(_) => 9500
        };
        
        // 获取主号的邮箱用于排除
        let master_email_lower = master_account.email.to_lowercase();
        
        // 遍历团队成员，检测积分（排除主号）
        for member in team_members {
            // 提取成员信息
            let member_api_key = member.get("string_1")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let member_email = member.get("string_3")
                .and_then(|v| v.as_str())
                .unwrap_or("未知邮箱");
            let member_name = member.get("string_2")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let firebase_id = member.get("string_6")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            
            // 跳过主号（通过邮箱匹配）
            if member_email.to_lowercase() == master_email_lower {
                continue;
            }
            
            // 跳过没有 api_key 的成员
            if member_api_key.is_empty() {
                continue;
            }
            
            // 从账号管理器中查找成员账号（需要其 token 来调用 API）
            let all_accounts = data_store.get_all_accounts().await.unwrap_or_default();
            let member_account = all_accounts.iter().find(|acc| {
                acc.email.to_lowercase() == member_email.to_lowercase()
            });
            
            // 使用成员的 token 调用 GetPlanStatus API 获取实时配额
            // 注意：账户面板的取值逻辑是：
            //   - used_quota = used_prompt_credits + used_flex_credits
            //   - total_quota = available_flex_credits + available_prompt_credits（这是总配额！）
            //   - remaining = total_quota - used_quota
            let (used_quota, total_member_quota) = if let Some(acc) = member_account {
                if let Some(member_token) = &acc.token {
                    if !member_token.is_empty() {
                        // 调用 GetPlanStatus API 获取实时数据
                        match windsurf_service.get_plan_status(member_token).await {
                            Ok(plan_status) => {
                                let ps = plan_status.get("plan_status").unwrap_or(&plan_status);
                                
                                // 提取已使用积分（used_prompt_credits + used_flex_credits）
                                let used_prompt = ps.get("used_prompt_credits")
                                    .and_then(|v| v.as_i64())
                                    .unwrap_or(0);
                                let used_flex = ps.get("used_flex_credits")
                                    .and_then(|v| v.as_i64())
                                    .unwrap_or(0);
                                let used = (used_prompt + used_flex) as i32;
                                
                                // 提取总配额（available_flex + available_prompt 是总配额）
                                let available_flex = ps.get("available_flex_credits")
                                    .and_then(|v| v.as_i64())
                                    .unwrap_or(0);
                                let available_prompt = ps.get("available_prompt_credits")
                                    .and_then(|v| v.as_i64())
                                    .unwrap_or(0);
                                let total = (available_flex + available_prompt) as i32;
                                
                                println!("[AutoReset] 从 GetPlanStatus API 获取 {} 的配额: used={} (prompt={}, flex={}), total={} (flex={}, prompt={})", 
                                    member_email, used, used_prompt, used_flex, total, available_flex, available_prompt);
                                (used, total)
                            }
                            Err(e) => {
                                println!("[AutoReset] 获取 {} 的 GetPlanStatus 失败: {}, 使用账号管理器静态值", member_email, e);
                                let used = acc.used_quota.unwrap_or(0);
                                let total = acc.total_quota.unwrap_or(50000);
                                (used, total)
                            }
                        }
                    } else {
                        println!("[AutoReset] 成员 {} 的 token 为空，使用账号管理器静态值", member_email);
                        let used = acc.used_quota.unwrap_or(0);
                        let total = acc.total_quota.unwrap_or(50000);
                        (used, total)
                    }
                } else {
                    println!("[AutoReset] 成员 {} 没有 token，使用账号管理器静态值", member_email);
                    let used = acc.used_quota.unwrap_or(0);
                    let total = acc.total_quota.unwrap_or(50000);
                    (used, total)
                }
            } else {
                // 如果账号管理器中没有该成员，使用 cascadeDetails 的数据（降级方案）
                let cascade_match = cascade_details.iter()
                    .find(|c| c.get("string_1").and_then(|v| v.as_str()).unwrap_or("") == firebase_id);
                
                let used = cascade_match
                    .and_then(|c| c.get("int_2"))
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0) as i32;
                
                println!("[AutoReset] 成员 {} 不在账号管理器中，使用 cascadeDetails: used={}, total={}", 
                    member_email, used, total_quota);
                (used, total_quota)
            };
            
            checked_count += 1;
            
            // 计算剩余积分 = 总配额 - 已用
            let remaining_quota = total_member_quota - used_quota;
            
            // 计算使用率
            let usage_percent = if total_member_quota > 0 {
                (used_quota as f64 / total_member_quota as f64 * 100.0) as i32
            } else {
                0
            };
            
            // 剩余阈值需要乘以 100（因为存储的值是实际值的 100 倍）
            let remaining_threshold_scaled = config.remaining_threshold * 100;
            
            // 检查是否满足重置条件（同时满足）
            let usage_condition = usage_percent >= config.usage_threshold;
            let remaining_condition = remaining_quota <= remaining_threshold_scaled;
            let should_reset = usage_condition && remaining_condition;
            
            println!("[AutoReset] 检查 {}: used={}, total={}, remaining={}, usage={}% (阈值{}%), remaining阈值={}*100={}, 使用率条件={}, 剩余条件={}, 需重置={}",
                member_email, used_quota, total_member_quota, remaining_quota, usage_percent, config.usage_threshold,
                config.remaining_threshold, remaining_threshold_scaled, usage_condition, remaining_condition, should_reset);
            
            if should_reset {
                // 执行重置（移除成员再重新邀请）
                match windsurf_service.reset_member_credits(&master_token, member_api_key, member_name, member_email).await {
                    Ok(reset_result) => {
                        let reset_success = reset_result.get("success").and_then(|v| v.as_bool()).unwrap_or(false);
                        
                        if reset_success {
                            reset_count += 1;
                            
                            // Step 3: 自动接受邀请（查找管理器中匹配的账号）
                            let mut auto_join_success = false;
                            let all_accounts = data_store.get_all_accounts().await.unwrap_or_default();
                            
                            // 查找邮箱匹配的账号（忽略大小写）
                            if let Some(matched_account) = all_accounts.iter().find(|acc| {
                                acc.email.to_lowercase() == member_email.to_lowercase()
                            }) {
                                // 尝试使用该账号接受邀请
                                if let Some(member_token) = &matched_account.token {
                                    if !member_token.is_empty() {
                                        // 先获取待处理的邀请
                                        if let Ok(preapproval) = windsurf_service.get_preapproval_for_user(member_token).await {
                                            if let Some(approval_id) = preapproval.get("data")
                                                .and_then(|d| d.get("subMesssage_1"))
                                                .and_then(|s| s.get("string_1"))
                                                .and_then(|v| v.as_str()) 
                                            {
                                                // 接受邀请
                                                if let Ok(accept_result) = windsurf_service.accept_preapproval(member_token, approval_id).await {
                                                    auto_join_success = accept_result.get("success")
                                                        .and_then(|v| v.as_bool())
                                                        .unwrap_or(false);
                                                    println!("[AutoReset] 自动接受邀请: {} -> {}", member_email, auto_join_success);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            
                            // 记录重置
                            let member_account_opt = all_accounts.iter().find(|acc| {
                                acc.email.to_lowercase() == member_email.to_lowercase()
                            });
                            let record = ResetRecord::new(
                                config_id.clone(),
                                member_account_opt.map(|a| a.id.to_string()).unwrap_or_default(),
                                member_email.to_string(),
                                member_account_opt.map(|a| Some(a.nickname.clone())).flatten(),
                                master_account.email.clone(),
                                used_quota,
                                total_member_quota,
                                auto_join_success,
                            );
                            let _ = record_store.add(record).await;
                            
                            results.push(json!({
                                "master_email": master_account.email,
                                "member_email": member_email,
                                "member_name": member_name,
                                "member_api_key": member_api_key,
                                "reset": true,
                                "auto_joined": auto_join_success,
                                "before": {
                                    "used_quota": used_quota,
                                    "total_quota": total_member_quota,
                                    "usage_percent": usage_percent,
                                    "remaining": remaining_quota
                                },
                                "result": reset_result
                            }));
                        } else {
                            results.push(json!({
                                "master_email": master_account.email,
                                "member_email": member_email,
                                "member_name": member_name,
                                "reset": false,
                                "error": reset_result.get("error").and_then(|v| v.as_str()).unwrap_or("重置失败")
                            }));
                        }
                    }
                    Err(e) => {
                        results.push(json!({
                            "master_email": master_account.email,
                            "member_email": member_email,
                            "member_name": member_name,
                            "reset": false,
                            "error": e.to_string()
                        }));
                    }
                }
            } else {
                results.push(json!({
                    "master_email": master_account.email,
                    "member_email": member_email,
                    "member_name": member_name,
                    "reset": false,
                    "reason": "未达到重置条件",
                    "current": {
                        "used_quota": used_quota,
                        "total_quota": total_member_quota,
                        "usage_percent": usage_percent,
                        "remaining": remaining_quota
                    },
                    "threshold": {
                        "usage_threshold": config.usage_threshold,
                        "remaining_threshold": config.remaining_threshold
                    }
                }));
            }
        }
    }
    
    // 如果有重置操作，更新重置时间并记录日志
    if reset_count > 0 {
        auto_reset_store.update_reset_time(&config_id).await?;
        
        let log = OperationLog::new(
            OperationType::BatchOperation,
            OperationStatus::Success,
            format!(
                "自动重置积分: {} - {}, 检查 {} 个账号, 重置 {} 个",
                config.target_type, config.target_id, checked_count, reset_count
            ),
        );
        let _ = data_store.add_log(log).await;
    }
    
    Ok(json!({
        "config_id": config_id,
        "target_type": config.target_type,
        "target_id": config.target_id,
        "checked_count": checked_count,
        "reset_count": reset_count,
        "results": results
    }))
}

/// 强制重置配置下的所有成员（不检查阈值条件）
#[tauri::command]
pub async fn force_reset_config(
    config_id: String,
    auto_reset_store: State<'_, Arc<AutoResetStore>>,
    data_store: State<'_, Arc<DataStore>>,
    record_store: State<'_, Arc<ResetRecordStore>>,
) -> Result<serde_json::Value, String> {
    let config = auto_reset_store.get_by_id(&config_id).await.ok_or("配置不存在")?;
    
    // 获取目标账号列表（这些是主号，用于获取团队成员）
    let master_accounts = if config.target_type == "group" {
        data_store.get_all_accounts().await
            .map_err(|e| e.to_string())?
            .into_iter()
            .filter(|a| a.group.as_ref().map(|g| g == &config.target_id).unwrap_or(false))
            .collect::<Vec<_>>()
    } else {
        // 单个账号
        let uuid = uuid::Uuid::parse_str(&config.target_id)
            .map_err(|_| "无效的账号ID".to_string())?;
        match data_store.get_account(uuid).await {
            Ok(account) => vec![account],
            Err(_) => return Err("账号不存在".to_string()),
        }
    };
    
    if master_accounts.is_empty() {
        return Ok(json!({ "skipped": true, "reason": "没有符合条件的主号" }));
    }
    
    let windsurf_service = WindsurfService::new();
    let mut reset_count = 0;
    let mut results = Vec::new();
    
    // 遍历每个主号，获取其团队成员并重置
    for master_account in master_accounts {
        let master_token = match &master_account.token {
            Some(t) if !t.is_empty() => t.clone(),
            _ => continue,
        };
        
        // 获取团队成员列表
        let team_members = match windsurf_service.get_team_members(&master_token, None).await {
            Ok(result) => {
                if result.get("success").and_then(|v| v.as_bool()).unwrap_or(false) {
                    let data = result.get("data").cloned().unwrap_or(json!({}));
                    let users = data.get("subMesssage_1").cloned();
                    match users {
                        Some(serde_json::Value::Array(arr)) => arr,
                        Some(obj @ serde_json::Value::Object(_)) => vec![obj],
                        _ => {
                            let alt_users = data.get("subMessage_1").cloned();
                            match alt_users {
                                Some(serde_json::Value::Array(arr)) => arr,
                                Some(obj @ serde_json::Value::Object(_)) => vec![obj],
                                _ => Vec::new()
                            }
                        }
                    }
                } else {
                    continue;
                }
            }
            Err(_) => continue,
        };
        
        let master_email_lower = master_account.email.to_lowercase();
        
        // 遍历团队成员并重置
        for member in team_members {
            let member_api_key = member.get("string_1")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let member_email = member.get("string_3")
                .and_then(|v| v.as_str())
                .unwrap_or("未知邮箱");
            let member_name = member.get("string_2")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            
            // 跳过主号
            if member_email.to_lowercase() == master_email_lower {
                continue;
            }
            
            // 跳过没有 api_key 的成员
            if member_api_key.is_empty() {
                continue;
            }
            
            // 执行重置
            match windsurf_service.reset_member_credits(&master_token, member_api_key, member_name, member_email).await {
                Ok(reset_result) => {
                    let reset_success = reset_result.get("success").and_then(|v| v.as_bool()).unwrap_or(false);
                    
                    if reset_success {
                        reset_count += 1;
                        
                        // 自动接受邀请
                        let mut auto_join_success = false;
                        let all_accounts = data_store.get_all_accounts().await.unwrap_or_default();
                        
                        if let Some(matched_account) = all_accounts.iter().find(|acc| {
                            acc.email.to_lowercase() == member_email.to_lowercase()
                        }) {
                            if let Some(member_token) = &matched_account.token {
                                if !member_token.is_empty() {
                                    if let Ok(preapproval) = windsurf_service.get_preapproval_for_user(member_token).await {
                                        if let Some(approval_id) = preapproval.get("data")
                                            .and_then(|d| d.get("subMesssage_1"))
                                            .and_then(|s| s.get("string_1"))
                                            .and_then(|v| v.as_str()) 
                                        {
                                            if let Ok(accept_result) = windsurf_service.accept_preapproval(member_token, approval_id).await {
                                                auto_join_success = accept_result.get("success")
                                                    .and_then(|v| v.as_bool())
                                                    .unwrap_or(false);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        
                        // 记录重置（强制重置不知道具体使用量，记录为0）
                        let member_account_opt = all_accounts.iter().find(|acc| {
                            acc.email.to_lowercase() == member_email.to_lowercase()
                        });
                        let record = ResetRecord::new(
                            config_id.clone(),
                            member_account_opt.map(|a| a.id.to_string()).unwrap_or_default(),
                            member_email.to_string(),
                            member_account_opt.map(|a| Some(a.nickname.clone())).flatten(),
                            master_account.email.clone(),
                            member_account_opt.and_then(|a| a.used_quota).unwrap_or(0),
                            member_account_opt.and_then(|a| a.total_quota).unwrap_or(50000),
                            auto_join_success,
                        );
                        let _ = record_store.add(record).await;
                        
                        results.push(json!({
                            "master_email": master_account.email,
                            "member_email": member_email,
                            "member_name": member_name,
                            "reset": true,
                            "auto_joined": auto_join_success
                        }));
                    }
                }
                Err(_) => {}
            }
        }
    }
    
    // 更新重置时间并记录日志
    if reset_count > 0 {
        auto_reset_store.update_reset_time(&config_id).await?;
        
        let log = OperationLog::new(
            OperationType::BatchOperation,
            OperationStatus::Success,
            format!(
                "强制重置积分: {} - {}, 重置 {} 个",
                config.target_type, config.target_id, reset_count
            ),
        );
        let _ = data_store.add_log(log).await;
    }
    
    Ok(json!({
        "config_id": config_id,
        "target_type": config.target_type,
        "target_id": config.target_id,
        "reset_count": reset_count,
        "results": results
    }))
}

/// 分页响应结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedRecords {
    pub records: Vec<ResetRecord>,
    pub total: usize,
    pub page: usize,
    pub page_size: usize,
    pub total_pages: usize,
}

/// 获取重置记录（分页）
#[tauri::command]
pub async fn get_reset_records(
    page: usize,
    page_size: usize,
    record_store: State<'_, Arc<ResetRecordStore>>,
) -> Result<PaginatedRecords, String> {
    let (records, total) = record_store.get_paginated(page, page_size).await;
    let total_pages = (total + page_size - 1) / page_size;
    
    Ok(PaginatedRecords {
        records,
        total,
        page,
        page_size,
        total_pages,
    })
}

/// 获取账号统计数据（分页）
#[tauri::command]
pub async fn get_reset_stats(
    page: usize,
    page_size: usize,
    record_store: State<'_, Arc<ResetRecordStore>>,
) -> Result<serde_json::Value, String> {
    let all_stats = record_store.get_stats().await;
    let total = all_stats.len();
    let total_pages = (total + page_size - 1) / page_size;
    
    let start = (page - 1) * page_size;
    let end = (start + page_size).min(total);
    
    let stats = if start < total {
        all_stats[start..end].to_vec()
    } else {
        Vec::new()
    };
    
    Ok(json!({
        "stats": stats,
        "total": total,
        "page": page,
        "page_size": page_size,
        "total_pages": total_pages
    }))
}

/// 清空重置记录
#[tauri::command]
pub async fn clear_reset_records(
    record_store: State<'_, Arc<ResetRecordStore>>,
) -> Result<(), String> {
    record_store.clear_all().await
}
