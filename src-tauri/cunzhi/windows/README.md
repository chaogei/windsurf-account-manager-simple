# 🛑 Windsurf 寸止 MCP

> **AI 对话"早泄"终结者，让交流持续到底** - 专为 Windsurf 优化

当 AI 想要"草草了事"时，寸止会及时弹出对话框，让你能够继续深入交流，直到真正解决问题为止。

## 🌟 核心特性

- 🛑 **智能拦截**：AI 想结束时自动弹出继续选项
- ⚡ **轻量设计**：专为 Windsurf 优化，无冗余功能
- 🎯 **预定义选项**：快速选择常用回复
- ✏️ **自由输入**：支持自定义反馈内容
- 🔧 **可配置**：自定义提示词和选项

## 🚀 快速开始

### 安装

#### Windows

```powershell
# 在项目目录运行安装脚本
.\install.ps1
```

#### macOS / Linux

```bash
# 添加执行权限并运行
chmod +x install.sh
./install.sh

# 可选参数
./install.sh --no-build      # 跳过编译，使用预编译文件
./install.sh --build-tauri   # 同时编译 Tauri UI
./install.sh --uninstall     # 卸载
```

### 手动编译

```bash
# 编译 MCP 服务器
cargo build --release

# 编译 Tauri UI
npm install
npm run tauri build
```

**可执行文件位置：**

| 平台 | MCP 服务器 | UI 工具 |
|------|-----------|---------|
| Windows | `target/release/windsurf-cunzhi.exe` | `src-tauri/target/release/windsurf-cunzhi-ui.exe` |
| macOS | `target/release/windsurf-cunzhi` | `src-tauri/target/release/bundle/macos/*.app` |
| Linux | `target/release/windsurf-cunzhi` | `src-tauri/target/release/windsurf-cunzhi-ui` |

### 配置 MCP

MCP 配置文件位置：

| 平台 | 路径 |
|------|------|
| Windows | `%USERPROFILE%\.codeium\windsurf\mcp_config.json` |
| macOS | `~/.codeium/windsurf/mcp_config.json` |
| Linux | `~/.codeium/windsurf/mcp_config.json` |

配置示例：

```json
{
  "mcpServers": {
    "windsurf-cunzhi": {
      "command": "/path/to/windsurf-cunzhi"
    }
  }
}
```

**各平台默认安装路径：**

| 平台 | 命令路径 |
|------|----------|
| Windows | `C:\Users\<用户名>\AppData\Local\windsurf-cunzhi\windsurf-cunzhi.exe` |
| macOS | `~/Library/Application Support/windsurf-cunzhi/windsurf-cunzhi` |
| Linux | `~/.local/share/windsurf-cunzhi/windsurf-cunzhi` |

## 🔧 工具说明

### zhi - 智能对话拦截

当 AI 准备结束任务时调用此工具，弹出交互对话框让用户决定下一步。

**参数：**

| 参数 | 类型 | 必需 | 说明 |
|------|------|------|------|
| message | string | ✅ | 显示给用户的消息，支持 Markdown |
| predefined_options | string[] | ❌ | 预定义选项列表 |
| is_markdown | boolean | ❌ | 是否为 Markdown 格式，默认 true |

**示例调用：**

```json
{
  "message": "## 任务完成\n\n我已完成了代码重构，主要改动包括：\n- 优化了函数结构\n- 添加了错误处理\n\n你想要继续还是有其他需求？",
  "predefined_options": ["继续", "详细解释", "换个方案", "暂停"]
}
```

## ⚙️ 配置文件

配置文件位置：

| 平台 | 路径 |
|------|------|
| Windows | `%APPDATA%\windsurf-cunzhi\config.json` |
| macOS | `~/Library/Application Support/windsurf-cunzhi/config.json` |
| Linux | `~/.config/windsurf-cunzhi/config.json` |

```json
{
  "enabled": true,
  "continue_prompt": "请按照最佳实践继续",
  "default_options": [
    "继续",
    "详细解释",
    "换个方案",
    "暂停，让我想想"
  ],
  "auto_detect_keywords": [
    "希望这对你有帮助",
    "如果还有问题",
    "任务完成",
    "已完成"
  ]
}
```

## 🎯 使用提示词

在你的 AI 助手提示词中添加：

```
当你准备结束当前任务或认为工作已完成时，必须调用 zhi 工具与用户确认。
不要擅自结束对话，让用户决定是否继续。
```

## 📁 项目结构

```
windsurf-cunzhi/
├── Cargo.toml              # 项目配置
├── install.ps1             # 安装脚本
├── README.md               # 说明文档
└── src/
    ├── lib.rs              # 库入口
    ├── bin/
    │   ├── mcp_server.rs   # MCP 服务器入口
    │   └── ui.rs           # UI 工具入口
    ├── config/
    │   └── settings.rs     # 配置管理
    ├── mcp/
    │   ├── server.rs       # MCP 服务器实现
    │   ├── tools.rs        # 工具实现
    │   └── types.rs        # 类型定义
    ├── ui/
    │   └── dialog.rs       # 对话框实现
    └── utils/
        ├── common.rs       # 通用工具
        └── logger.rs       # 日志工具
```

## 📄 许可证

MIT License
