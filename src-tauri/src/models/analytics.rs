use serde::{Deserialize, Serialize};

// ============== 代码贡献百分比 (Field 9: percent_code_written) ==============

/// 代码贡献百分比统计
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PercentCodeWritten {
    /// AI 编写代码占比 (%)
    pub percent_code_written: f64,
    /// 通过自动补全产生的字节数
    pub codeium_bytes_by_autocomplete: i64,
    /// 通过命令产生的字节数
    pub codeium_bytes_by_command: i64,
    /// 用户编写的字节数
    pub user_bytes: i64,
    /// AI 总共产生的字节数
    pub codeium_bytes: i64,
    /// 总字节数
    pub total_bytes: i64,
    /// 通过 Supercomplete 产生的字节数
    pub codeium_bytes_by_supercomplete: i64,
    /// 通过 Cascade 产生的字节数
    pub codeium_bytes_by_cascade: i64,
}


// ============== 补全统计 (Field 1: completion_stats) ==============

/// 代码补全统计
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CompletionStatistics {
    /// 接受次数
    pub num_acceptances: i64,
    /// 拒绝次数
    pub num_rejections: i64,
    /// 接受的代码行数
    pub num_lines_accepted: i64,
    /// 接受的字节数
    pub num_bytes_accepted: i64,
    /// 用户数
    pub num_users: i64,
    /// 活跃开发天数
    pub active_developer_days: i64,
    /// 活跃开发小时数
    pub active_developer_hours: i64,
    /// 接受率 (计算字段)
    pub acceptance_rate: f64,
}


/// 按日期的补全统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionByDay {
    /// 日期时间戳
    pub timestamp: i64,
    /// 日期字符串
    pub date: String,
    /// 补全统计
    pub statistics: CompletionStatistics,
}

/// 按语言的补全统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionByLanguage {
    /// 语言ID
    pub language_id: i32,
    /// 语言名称
    pub language_name: String,
    /// 补全统计
    pub statistics: CompletionStatistics,
}

// ============== Chat 统计 (Field 11: chat_stats) ==============

/// Chat 统计
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatStats {
    /// 发送的聊天数
    pub chats_sent: i64,
    /// 接收的聊天数
    pub chats_received: i64,
    /// 接受的聊天数
    pub chats_accepted: i64,
    /// 在光标处插入的次数
    pub chats_inserted_at_cursor: i64,
    /// 应用的次数
    pub chats_applied: i64,
    /// 使用的代码行数
    pub chat_loc_used: i64,
    /// 使用的代码块数
    pub chat_code_blocks_used: i64,
    /// 函数解释次数
    pub function_explain_count: i64,
    /// 文档字符串生成次数
    pub function_docstring_count: i64,
    /// 函数重构次数
    pub function_refactor_count: i64,
    /// 代码块解释次数
    pub code_block_explain_count: i64,
    /// 代码块重构次数
    pub code_block_refactor_count: i64,
    /// 问题解释次数
    pub problem_explain_count: i64,
    /// 单元测试生成次数
    pub function_unit_tests_count: i64,
    /// 活跃开发天数
    pub active_developer_days: i64,
}


/// 按日期的 Chat 统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatStatsByDay {
    /// 日期时间戳
    pub timestamp: i64,
    /// 日期字符串
    pub date: String,
    /// Chat 统计
    pub stats: ChatStats,
}

/// 按模型的 Chat 统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatStatsByModel {
    /// 模型ID
    pub model_id: i32,
    /// 模型名称
    pub model_name: String,
    /// Chat 统计
    pub stats: ChatStats,
}

// ============== 自定义查询 ==============

/// 自定义查询数据源
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum QueryDataSource {
    Unspecified = 0,
    UserData = 1,
    ChatData = 2,
    CommandData = 3,
    CascadeData = 4,
    PcwData = 5,
}

/// 自定义查询响应项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomQueryResponseItem {
    /// 键值对数据
    pub data: std::collections::HashMap<String, String>,
}

/// 自定义查询响应
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CustomQueryResponse {
    /// 响应项列表
    pub items: Vec<CustomQueryResponseItem>,
}


// ============== 原有结构 ==============

/// 每日 Cascade 代码行数统计 (Field 18: cascade_lines)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyCascadeLinesCount {
    /// 日期时间戳
    pub timestamp: i64,
    /// 日期字符串 (YYYY-MM-DD)
    pub date: String,
    /// 接受的代码行数 (Accepted lines)
    pub accepted_lines: i64,
    /// 建议的代码行数 (Suggested lines)
    pub suggested_lines: i64,
}

/// 工具使用统计条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolUsageEntry {
    /// 工具名称
    pub tool_name: String,
    /// 使用次数
    pub count: i64,
    /// 使用占比（百分比）
    pub percentage: f64,
}

/// 模型使用统计条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelUsageEntry {
    /// 日期时间戳
    pub timestamp: i64,
    /// 日期字符串
    pub date: String,
    /// 模型名称
    pub model_name: String,
    /// 运行模式
    pub mode: String,
    /// 会话数/消息数
    pub session_count: i64,
    /// Token使用量
    pub token_usage: i64,
    /// 会话ID
    pub session_id: String,
}

/// 模型使用汇总
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelUsageSummary {
    /// 模型名称
    pub model_name: String,
    /// 总使用次数
    pub total_count: i64,
    /// 总Token消耗
    pub total_tokens: i64,
    /// 使用占比
    pub percentage: f64,
}

/// 使用分析响应数据
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AnalyticsData {
    /// 每日 Cascade 代码行数统计 (Field 18: cascade_lines)
    pub daily_cascade_lines: Vec<DailyCascadeLinesCount>,
    /// 工具使用统计 (Field 19: cascade_tool_usage)
    pub tool_usage: Vec<ToolUsageEntry>,
    /// 模型使用详情 (Field 20: cascade_runs)
    pub model_usage_details: Vec<ModelUsageEntry>,
    /// 模型使用汇总
    pub model_usage_summary: Vec<ModelUsageSummary>,
    /// 总体统计
    pub summary: AnalyticsSummary,
    
    // ===== 新增字段 =====
    /// 代码贡献百分比 (Field 9: percent_code_written)
    pub percent_code_written: PercentCodeWritten,
    /// 补全统计 (Field 1: completion_stats)
    pub completion_stats: CompletionStatistics,
    /// 按日期的补全统计 (Field 2: completions_by_day)
    pub completions_by_day: Vec<CompletionByDay>,
    /// 按语言的补全统计 (Field 3: completions_by_language)
    pub completions_by_language: Vec<CompletionByLanguage>,
    /// Chat 统计 (Field 11: chat_stats)
    pub chat_stats: ChatStats,
    /// 按日期的 Chat 统计 (Field 6: chats_by_day)
    pub chats_by_day: Vec<ChatStatsByDay>,
    /// 按模型的 Chat 统计 (Field 7: chats_by_model)
    pub chats_by_model: Vec<ChatStatsByModel>,
    /// 自定义查询结果 (Field 16: custom_stats)
    pub custom_query_results: CustomQueryResponse,
}

/// 总体统计摘要
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AnalyticsSummary {
    /// 总代码行数（接受的）
    pub total_accepted_lines: i64,
    /// 总代码行数（建议的）
    pub total_suggested_lines: i64,
    /// 平均每日代码行数（接受的）
    pub avg_daily_accepted_lines: f64,
    /// 峰值日期
    pub peak_date: String,
    /// 峰值代码行数
    pub peak_lines: i64,
    /// 总工具使用次数
    pub total_tool_usage: i64,
    /// 总会话数
    pub total_sessions: i64,
    /// 总Token消耗
    pub total_tokens: i64,
    /// 主要使用的模型
    pub primary_model: String,
    /// 主要使用的工具
    pub primary_tool: String,
}

impl AnalyticsData {
    /// 创建空的分析数据
    #[allow(dead_code)]
    pub fn empty() -> Self {
        Self {
            daily_cascade_lines: Vec::new(),
            tool_usage: Vec::new(),
            model_usage_details: Vec::new(),
            model_usage_summary: Vec::new(),
            summary: AnalyticsSummary {
                total_accepted_lines: 0,
                total_suggested_lines: 0,
                avg_daily_accepted_lines: 0.0,
                peak_date: String::new(),
                peak_lines: 0,
                total_tool_usage: 0,
                total_sessions: 0,
                total_tokens: 0,
                primary_model: String::new(),
                primary_tool: String::new(),
            },
            // 新增字段
            percent_code_written: PercentCodeWritten::default(),
            completion_stats: CompletionStatistics::default(),
            completions_by_day: Vec::new(),
            completions_by_language: Vec::new(),
            chat_stats: ChatStats::default(),
            chats_by_day: Vec::new(),
            chats_by_model: Vec::new(),
            custom_query_results: CustomQueryResponse::default(),
        }
    }
}

