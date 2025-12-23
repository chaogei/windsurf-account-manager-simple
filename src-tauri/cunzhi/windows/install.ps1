<#
.SYNOPSIS
    Windsurf 寸止 MCP 安装脚本
.DESCRIPTION
    安装 Windsurf 寸止 MCP 服务器和 Tauri UI
.PARAMETER InstallPath
    安装目标路径，默认为用户本地目录
.PARAMETER NoBuild
    跳过编译，直接使用已有的可执行文件
.PARAMETER BuildTauri
    编译 Tauri UI（需要 Node.js 环境）
#>
param(
    [string]$InstallPath = "$env:LOCALAPPDATA\windsurf-cunzhi",
    [switch]$NoBuild,
    [switch]$BuildTauri
)

$ErrorActionPreference = "Stop"
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path

function Write-Info { param($m) Write-Host "[INFO] $m" -ForegroundColor Cyan }
function Write-Ok { param($m) Write-Host "[OK] $m" -ForegroundColor Green }
function Write-Err { param($m) Write-Host "[ERROR] $m" -ForegroundColor Red }
function Write-Warn { param($m) Write-Host "[WARN] $m" -ForegroundColor Yellow }

Write-Host ""
Write-Host "╔════════════════════════════════════════════════════════════╗" -ForegroundColor Magenta
Write-Host "║       Windsurf Cunzhi MCP - Installer                      ║" -ForegroundColor Magenta
Write-Host "╚════════════════════════════════════════════════════════════╝" -ForegroundColor Magenta
Write-Host ""

# 定义文件路径
$mcpExe = Join-Path $scriptDir "target\release\windsurf-cunzhi.exe"
$tauriUiExe = Join-Path $scriptDir "src-tauri\target\release\windsurf-cunzhi-ui.exe"

# 设置 Rust 环境
$env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"

# 检查预编译文件
$hasMcpExe = Test-Path $mcpExe
$hasTauriUi = Test-Path $tauriUiExe

if ($hasMcpExe) {
    Write-Ok "Found MCP server: $mcpExe"
}
if ($hasTauriUi) {
    Write-Ok "Found Tauri UI: $tauriUiExe"
}

# 编译 MCP 服务器
if (-not $hasMcpExe -and -not $NoBuild) {
    Write-Info "Building MCP server..."
    $rustVersion = & rustc --version 2>$null
    if (-not $rustVersion) {
        Write-Err "Rust not found. Please install: https://rustup.rs"
        exit 1
    }
    Write-Ok "Rust: $rustVersion"
    
    Push-Location $scriptDir
    try {
        & cargo build --release
        if ($LASTEXITCODE -ne 0) { Write-Err "Build failed"; exit 1 }
        Write-Ok "MCP server build successful"
        $hasMcpExe = $true
    } finally { Pop-Location }
}

# 编译 Tauri UI
if ($BuildTauri -and -not $hasTauriUi) {
    Write-Info "Building Tauri UI..."
    
    $npmVersion = & npm --version 2>$null
    if (-not $npmVersion) {
        Write-Err "Node.js/npm not found. Please install: https://nodejs.org"
        exit 1
    }
    Write-Ok "npm: $npmVersion"
    
    Push-Location $scriptDir
    try {
        Write-Info "Installing npm dependencies..."
        & npm install
        Write-Info "Building Tauri application..."
        & npx tauri build
        if ($LASTEXITCODE -ne 0) { Write-Err "Tauri build failed"; exit 1 }
        Write-Ok "Tauri UI build successful"
        $hasTauriUi = $true
    } finally { Pop-Location }
}

# 验证必要文件
if (-not (Test-Path $mcpExe)) {
    Write-Err "MCP server not found. Run without -NoBuild to compile."
    exit 1
}

# 确定 UI 文件
$uiExeSource = if ($hasTauriUi) { $tauriUiExe } else { $null }

# 创建安装目录
Write-Info "Install directory: $InstallPath"
if (-not (Test-Path $InstallPath)) {
    New-Item -ItemType Directory -Path $InstallPath -Force | Out-Null
}

# 复制文件
Write-Info "Copying files..."
Copy-Item $mcpExe "$InstallPath\" -Force
if ($uiExeSource) {
    Copy-Item $uiExeSource "$InstallPath\windsurf-cunzhi-ui.exe" -Force
    Write-Ok "Copied MCP server and UI"
} else {
    Write-Warn "UI not found, only MCP server installed"
}

# 添加到 PATH
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($userPath -notlike "*$InstallPath*") {
    Write-Info "Adding to user PATH..."
    [Environment]::SetEnvironmentVariable("Path", "$userPath;$InstallPath", "User")
    Write-Ok "Added to PATH (restart terminal to take effect)"
} else {
    Write-Info "Already in PATH"
}

# ============================================
# 自动配置 Windsurf MCP
# ============================================
$windsurfConfigDir = "$env:USERPROFILE\.codeium\windsurf"
$windsurfConfigPath = "$windsurfConfigDir\mcp_config.json"
$mcpCommand = "$InstallPath\windsurf-cunzhi.exe"

Write-Info "Configuring Windsurf MCP..."

# 确保配置目录存在
if (-not (Test-Path $windsurfConfigDir)) {
    New-Item -ItemType Directory -Path $windsurfConfigDir -Force | Out-Null
}

# 读取或创建 MCP 配置
$mcpConfigUpdated = $false
if (Test-Path $windsurfConfigPath) {
    try {
        $mcpConfig = Get-Content $windsurfConfigPath -Raw | ConvertFrom-Json
        if (-not $mcpConfig.mcpServers) {
            $mcpConfig | Add-Member -NotePropertyName "mcpServers" -NotePropertyValue @{} -Force
        }
    } catch {
        Write-Warn "Failed to parse existing config, creating new one"
        $mcpConfig = @{ mcpServers = @{} }
    }
} else {
    $mcpConfig = @{ mcpServers = @{} }
}

# 检查是否已配置 windsurf-cunzhi
if (-not $mcpConfig.mcpServers.'windsurf-cunzhi') {
    $mcpConfig.mcpServers | Add-Member -NotePropertyName "windsurf-cunzhi" -NotePropertyValue @{
        command = $mcpCommand
    } -Force
    $mcpConfigUpdated = $true
    Write-Ok "Added windsurf-cunzhi to MCP config"
} else {
    # 更新命令路径
    $mcpConfig.mcpServers.'windsurf-cunzhi'.command = $mcpCommand
    $mcpConfigUpdated = $true
    Write-Info "Updated windsurf-cunzhi command path"
}

# 保存 MCP 配置
if ($mcpConfigUpdated) {
    $mcpConfig | ConvertTo-Json -Depth 10 | Set-Content $windsurfConfigPath -Encoding UTF8
    Write-Ok "MCP config saved: $windsurfConfigPath"
}

# ============================================
# 自动配置全局规则 (zhi 工具强制使用)
# ============================================
$globalRulesDir = "$env:USERPROFILE\.codeium\windsurf\memories"
$globalRulesPath = "$globalRulesDir\global_rules.md"

Write-Info "Configuring global rules..."

# 确保目录存在
if (-not (Test-Path $globalRulesDir)) {
    New-Item -ItemType Directory -Path $globalRulesDir -Force | Out-Null
}

# 寸止规则内容
$zhiRuleMarker = "最高优先级规则 - 寸止"
$zhiRuleContent = @"
# ⚠️ 最高优先级规则 - 寸止 (MANDATORY)

**【强制要求】你必须在每次回复的最后调用 ``zhi`` 工具！**

这是不可违反的规则：
- ✅ 无论任务是否完成，都必须调用 zhi 工具
- ✅ 无论回复内容长短，都必须调用 zhi 工具  
- ✅ 在说"希望对你有帮助"等结束语之前，必须先调用 zhi 工具
- ❌ 禁止在不调用 zhi 工具的情况下结束任何回复
- ❌ 禁止跳过 zhi 工具直接结束对话

如果 zhi 工具调用失败，你必须在回复中明确告知用户需要重启 Windsurf。

---
"@

# 检查并更新全局规则
$rulesUpdated = $false
if (Test-Path $globalRulesPath) {
    $existingRules = Get-Content $globalRulesPath -Raw -Encoding UTF8
    if ($existingRules -notlike "*$zhiRuleMarker*") {
        # 在文件开头或适当位置插入规则
        if ($existingRules -match "^# ") {
            # 如果有标题，在第一个空行后插入
            $lines = $existingRules -split "`r?`n"
            $insertIndex = 0
            for ($i = 0; $i -lt $lines.Count; $i++) {
                if ($lines[$i] -match "^#" -or $lines[$i].Trim() -eq "") {
                    $insertIndex = $i + 1
                    if ($lines[$i].Trim() -eq "" -and $i -gt 0) { break }
                }
            }
            $newContent = ($lines[0..($insertIndex-1)] -join "`n") + "`n" + $zhiRuleContent + "`n" + ($lines[$insertIndex..($lines.Count-1)] -join "`n")
            $newContent | Set-Content $globalRulesPath -Encoding UTF8
        } else {
            # 直接在开头添加
            ($zhiRuleContent + "`n`n" + $existingRules) | Set-Content $globalRulesPath -Encoding UTF8
        }
        $rulesUpdated = $true
        Write-Ok "Added zhi tool rule to global_rules.md"
    } else {
        Write-Info "Zhi tool rule already exists in global_rules.md"
    }
} else {
    # 创建新的全局规则文件
    $defaultRules = @"
$zhiRuleContent

# Role: 高级软件开发助手
- 使用中文回复
- 遵循最佳实践
- 需求不明确时向用户询问澄清
"@
    $defaultRules | Set-Content $globalRulesPath -Encoding UTF8
    $rulesUpdated = $true
    Write-Ok "Created global_rules.md with zhi tool rule"
}

# ============================================
# 安装完成
# ============================================
Write-Host ""
Write-Host "═══════════════════════════════════════════════════════════════" -ForegroundColor Green
Write-Ok "Installation complete!"
Write-Host "═══════════════════════════════════════════════════════════════" -ForegroundColor Green
Write-Host ""
Write-Host "Installed files:" -ForegroundColor Yellow
Write-Host "  - MCP server: $InstallPath\windsurf-cunzhi.exe" -ForegroundColor White
if ($uiExeSource) {
    Write-Host "  - Tauri UI:   $InstallPath\windsurf-cunzhi-ui.exe" -ForegroundColor White
}
Write-Host ""
Write-Host "Configuration:" -ForegroundColor Yellow
Write-Host "  - MCP config:    $windsurfConfigPath" -ForegroundColor White
Write-Host "  - Global rules:  $globalRulesPath" -ForegroundColor White
Write-Host ""
Write-Host "Commands:" -ForegroundColor Yellow
Write-Host "  windsurf-cunzhi      - MCP server" -ForegroundColor White
if ($uiExeSource) {
    Write-Host "  windsurf-cunzhi-ui   - Standalone UI test" -ForegroundColor White
}
Write-Host ""
Write-Warn "Please restart Windsurf to apply changes!"
Write-Host ""
if (-not $hasTauriUi -and -not $BuildTauri) {
    Write-Warn "Tip: Run with -BuildTauri to build the beautiful Tauri UI"
}
Write-Host ""
