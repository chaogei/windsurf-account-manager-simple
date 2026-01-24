use serde_json::json;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::Command;
use std::thread;
use std::time::Duration;
use tauri::Manager;
use zip::ZipArchive;


/// 获取 MCP 可执行文件名（单二进制，包含 MCP + UI）
fn get_mcp_exe_name() -> &'static str {
    #[cfg(target_os = "windows")]
    { "windsurf-cunzhi.exe" }
    #[cfg(not(target_os = "windows"))]
    { "windsurf-cunzhi" }
}

/// 获取平台特定的资源子目录名
fn get_platform_subdir() -> &'static str {
    #[cfg(target_os = "windows")]
    { "windows" }
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    { "macos-arm64" }
    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    { "macos-x64" }
    #[cfg(target_os = "linux")]
    { "linux" }
}

/// 解压 zip 文件到指定目录
#[allow(dead_code)]
fn extract_zip(zip_path: &PathBuf, dest_dir: &PathBuf) -> Result<Vec<String>, String> {
    let file = File::open(zip_path)
        .map_err(|e| format!("打开 zip 文件失败: {}", e))?;
    
    let mut archive = ZipArchive::new(file)
        .map_err(|e| format!("读取 zip 文件失败: {}", e))?;
    
    let mut extracted_files = Vec::new();
    
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)
            .map_err(|e| format!("读取 zip 条目失败: {}", e))?;
        
        let outpath = match file.enclosed_name() {
            Some(path) => dest_dir.join(path),
            None => continue,
        };
        
        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath)
                .map_err(|e| format!("创建目录失败: {}", e))?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p)
                        .map_err(|e| format!("创建父目录失败: {}", e))?;
                }
            }
            let mut outfile = File::create(&outpath)
                .map_err(|e| format!("创建文件失败: {}", e))?;
            
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)
                .map_err(|e| format!("读取文件内容失败: {}", e))?;
            
            outfile.write_all(&buffer)
                .map_err(|e| format!("写入文件失败: {}", e))?;
            
            // 在 Unix 系统上设置可执行权限
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if let Some(mode) = file.unix_mode() {
                    fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).ok();
                } else {
                    // 默认设置可执行权限
                    fs::set_permissions(&outpath, fs::Permissions::from_mode(0o755)).ok();
                }
            }
            
            extracted_files.push(outpath.to_string_lossy().to_string());
        }
    }
    
    Ok(extracted_files)
}

/// 递归查找可执行文件
/// pattern: 文件名必须包含的字符串
/// exclude: 文件名不能包含的字符串（可选）
fn find_executable_recursive(dir: &PathBuf, pattern: &str, exclude: Option<&str>) -> Option<PathBuf> {
    if !dir.exists() || !dir.is_dir() {
        return None;
    }
    
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            
            if path.is_file() {
                if let Some(name) = path.file_name() {
                    let name_str = name.to_string_lossy();
                    let matches_pattern = name_str.contains(pattern);
                    let not_excluded = exclude.map_or(true, |ex| !name_str.contains(ex));
                    
                    if matches_pattern && not_excluded {
                        return Some(path);
                    }
                }
            } else if path.is_dir() {
                // 递归搜索子目录
                if let Some(found) = find_executable_recursive(&path, pattern, exclude) {
                    return Some(found);
                }
            }
        }
    }
    
    None
}

/// 递归复制目录（用于 macOS .app 目录）
#[allow(dead_code)]
fn copy_dir_all(src: &PathBuf, dst: &PathBuf) -> Result<(), String> {
    fs::create_dir_all(dst)
        .map_err(|e| format!("创建目标目录失败: {}", e))?;
    
    for entry in fs::read_dir(src).map_err(|e| format!("读取源目录失败: {}", e))? {
        let entry = entry.map_err(|e| format!("读取目录条目失败: {}", e))?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        
        if src_path.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)
                .map_err(|e| format!("复制文件失败: {}", e))?;
            
            // 在 Unix 系统上保留权限
            #[cfg(unix)]
            {
                if let Ok(metadata) = fs::metadata(&src_path) {
                    fs::set_permissions(&dst_path, metadata.permissions()).ok();
                }
            }
        }
    }
    
    Ok(())
}

/// 关闭 Windsurf 进程
fn kill_windsurf() -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        let _ = Command::new("taskkill")
            .args(["/f", "/im", "Windsurf.exe"])
            .output();
        thread::sleep(Duration::from_millis(500));
    }
    #[cfg(target_os = "macos")]
    {
        let _ = Command::new("pkill")
            .args(["-f", "Windsurf"])
            .output();
        thread::sleep(Duration::from_millis(500));
    }
    #[cfg(target_os = "linux")]
    {
        let _ = Command::new("pkill")
            .args(["-f", "windsurf"])
            .output();
        thread::sleep(Duration::from_millis(500));
    }
    Ok(())
}

/// 启动 Windsurf
fn start_windsurf(windsurf_path: Option<&str>) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        // 优先使用传入的路径
        if let Some(path_str) = windsurf_path {
            let windsurf_exe = PathBuf::from(path_str).join("Windsurf.exe");
            println!("[Cunzhi] Using configured Windsurf path: {:?}", windsurf_exe);
            if windsurf_exe.exists() {
                match Command::new(&windsurf_exe).spawn() {
                    Ok(_) => {
                        println!("[Cunzhi] Windsurf started successfully");
                        return Ok(());
                    }
                    Err(e) => {
                        println!("[Cunzhi] Failed to start Windsurf: {}", e);
                    }
                }
            }
        }
        // 后备：尝试通过 cmd /c start 启动
        println!("[Cunzhi] Trying to start Windsurf via cmd...");
        match Command::new("cmd")
            .args(["/c", "start", "", "Windsurf"])
            .spawn() {
            Ok(_) => {
                println!("[Cunzhi] Windsurf started via cmd");
                return Ok(());
            }
            Err(e) => {
                println!("[Cunzhi] Failed to start Windsurf via cmd: {}", e);
            }
        }
    }
    #[cfg(target_os = "macos")]
    {
        // 优先使用传入的路径
        if let Some(path_str) = windsurf_path {
            let windsurf_app = PathBuf::from(path_str);
            println!("[Cunzhi] Using configured Windsurf path: {:?}", windsurf_app);
            if windsurf_app.exists() {
                match Command::new("open").arg("-a").arg(&windsurf_app).spawn() {
                    Ok(_) => {
                        println!("[Cunzhi] Windsurf started successfully");
                        return Ok(());
                    }
                    Err(e) => {
                        println!("[Cunzhi] Failed to start Windsurf: {}", e);
                    }
                }
            }
        }
        // 后备：尝试通过 open 命令启动
        println!("[Cunzhi] Trying to start Windsurf via open...");
        match Command::new("open").arg("-a").arg("Windsurf").spawn() {
            Ok(_) => {
                println!("[Cunzhi] Windsurf started via open");
                return Ok(());
            }
            Err(e) => {
                println!("[Cunzhi] Failed to start Windsurf via open: {}", e);
            }
        }
    }
    #[cfg(target_os = "linux")]
    {
        // 优先使用传入的路径
        if let Some(path_str) = windsurf_path {
            let windsurf_exe = PathBuf::from(path_str).join("windsurf");
            println!("[Cunzhi] Using configured Windsurf path: {:?}", windsurf_exe);
            if windsurf_exe.exists() {
                match Command::new(&windsurf_exe).spawn() {
                    Ok(_) => {
                        println!("[Cunzhi] Windsurf started successfully");
                        return Ok(());
                    }
                    Err(e) => {
                        println!("[Cunzhi] Failed to start Windsurf: {}", e);
                    }
                }
            }
        }
        // 后备：尝试通过 PATH 启动
        println!("[Cunzhi] Trying to start Windsurf via PATH...");
        match Command::new("windsurf").spawn() {
            Ok(_) => {
                println!("[Cunzhi] Windsurf started via PATH");
                return Ok(());
            }
            Err(e) => {
                println!("[Cunzhi] Failed to start Windsurf via PATH: {}", e);
            }
        }
    }
    Ok(())
}

/// 重启 Windsurf
fn restart_windsurf(windsurf_path: Option<&str>) -> Result<(), String> {
    kill_windsurf()?;
    thread::sleep(Duration::from_millis(500));
    start_windsurf(windsurf_path)?;
    Ok(())
}

/// 获取伟哥(寸止) MCP 的安装目录
fn get_cunzhi_install_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        let local_app_data = std::env::var("LOCALAPPDATA")
            .unwrap_or_else(|_| std::env::var("APPDATA").unwrap_or_default());
        PathBuf::from(local_app_data).join("windsurf-cunzhi")
    }
    #[cfg(target_os = "macos")]
    {
        let home = std::env::var("HOME").unwrap_or_default();
        PathBuf::from(home).join("Library").join("Application Support").join("windsurf-cunzhi")
    }
    #[cfg(target_os = "linux")]
    {
        let home = std::env::var("HOME").unwrap_or_default();
        PathBuf::from(home).join(".local").join("share").join("windsurf-cunzhi")
    }
}

/// 获取 MCP 配置文件路径
fn get_mcp_config_path() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        let home = std::env::var("USERPROFILE").unwrap_or_default();
        PathBuf::from(home).join(".codeium").join("windsurf").join("mcp_config.json")
    }
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    {
        let home = std::env::var("HOME").unwrap_or_default();
        PathBuf::from(home).join(".codeium").join("windsurf").join("mcp_config.json")
    }
}

/// 获取全局规则文件路径
fn get_global_rules_path() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        let home = std::env::var("USERPROFILE").unwrap_or_default();
        PathBuf::from(home).join(".codeium").join("windsurf").join("memories").join("global_rules.md")
    }
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    {
        let home = std::env::var("HOME").unwrap_or_default();
        PathBuf::from(home).join(".codeium").join("windsurf").join("memories").join("global_rules.md")
    }
}

/// 寸止规则标记
const ZHI_RULE_MARKER: &str = "PRIORITY RULE - read_user_input";

/// Rule content
const ZHI_RULE_CONTENT: &str = r#"# PRIORITY RULE - read_user_input (MANDATORY)

**You MUST call the `read_user_input` tool at the end of every response!**

Rules:
- Always call read_user_input tool before ending any response
- Always call read_user_input tool before saying goodbye phrases
- Never end a response without calling read_user_input tool

If read_user_input tool fails, inform user to restart the IDE.

---
"#;

/// Old rule markers to clean up
const OLD_RULE_MARKERS: &[&str] = &[
    "最高优先级规则 - confirm",
    "最高优先级规则 - 寸止",
    "PRIORITY RULE - prompt",
    "PRIORITY RULE - pro",
    "Filesystem User Input",
];

/// 检查伟哥(寸止)状态
#[tauri::command]
pub async fn check_cunzhi_status() -> Result<serde_json::Value, String> {
    let install_dir = get_cunzhi_install_dir();
    let mcp_exe = install_dir.join(get_mcp_exe_name());
    let mcp_config_path = get_mcp_config_path();
    let global_rules_path = get_global_rules_path();
    
    // 检查 MCP 服务器是否存在
    let mcp_installed = mcp_exe.exists();
    
    // 检查 MCP 配置是否存在
    let mcp_configured = if mcp_config_path.exists() {
        if let Ok(content) = fs::read_to_string(&mcp_config_path) {
            content.contains("filesystem") && content.contains("windsurf-cunzhi")
        } else {
            false
        }
    } else {
        false
    };
    
    // 检查全局规则是否存在
    let rules_configured = if global_rules_path.exists() {
        if let Ok(content) = fs::read_to_string(&global_rules_path) {
            content.contains(ZHI_RULE_MARKER)
        } else {
            false
        }
    } else {
        false
    };
    
    let installed = mcp_installed && mcp_configured && rules_configured;
    
    Ok(json!({
        "installed": installed,
        "mcp_installed": mcp_installed,
        "mcp_configured": mcp_configured,
        "rules_configured": rules_configured,
        "install_dir": install_dir.to_string_lossy(),
        "error": ""
    }))
}

/// 安装伟哥(寸止)
#[tauri::command]
pub async fn install_cunzhi(app_handle: tauri::AppHandle, windsurf_path: Option<String>) -> Result<serde_json::Value, String> {
    let install_dir = get_cunzhi_install_dir();
    let mcp_config_path = get_mcp_config_path();
    let global_rules_path = get_global_rules_path();
    
    // 1. 创建安装目录
    if !install_dir.exists() {
        fs::create_dir_all(&install_dir)
            .map_err(|e| format!("创建安装目录失败: {}", e))?;
    }
    
    // 2. 从资源目录复制 MCP 服务器（单二进制，包含 MCP + UI）
    // 资源目录结构（按平台分目录）：
    //   cunzhi/windows/windsurf-cunzhi.exe
    //   cunzhi/macos-arm64/windsurf-cunzhi
    //   cunzhi/macos-x64/windsurf-cunzhi
    //   cunzhi/linux/windsurf-cunzhi
    let resource_dir = app_handle.path().resource_dir()
        .map(|p: PathBuf| p.join("cunzhi").join(get_platform_subdir()))
        .unwrap_or_default();
    
    let mcp_exe_name = get_mcp_exe_name();
    
    println!("[Cunzhi] Resource dir: {:?}", resource_dir);
    
    let mcp_exe_dest = install_dir.join(mcp_exe_name);
    
    let mut mcp_installed = false;
    
    // 复制 MCP 服务器（单二进制）
    // 源路径: cunzhi/{platform}/windsurf-cunzhi[.exe]
    let mcp_source = resource_dir.join(mcp_exe_name);
    
    println!("[Cunzhi] Looking for MCP at: {:?}", mcp_source);
    
    if mcp_source.exists() {
        fs::copy(&mcp_source, &mcp_exe_dest)
            .map_err(|e| format!("复制 MCP 服务器失败: {}", e))?;
        
        // 在 Unix 系统上设置可执行权限
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&mcp_exe_dest, fs::Permissions::from_mode(0o755)).ok();
        }
        
        mcp_installed = true;
        println!("[Cunzhi] Installed MCP: {:?}", mcp_exe_dest);
    } else {
        // 后备：递归搜索（向上一级目录）
        let fallback_dir = app_handle.path().resource_dir()
            .map(|p: PathBuf| p.join("cunzhi"))
            .unwrap_or_default();
        
        if let Some(found) = find_executable_recursive(&fallback_dir, "windsurf-cunzhi", None) {
            fs::copy(&found, &mcp_exe_dest)
                .map_err(|e| format!("复制 MCP 服务器失败: {}", e))?;
            
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                fs::set_permissions(&mcp_exe_dest, fs::Permissions::from_mode(0o755)).ok();
            }
            
            mcp_installed = true;
            println!("[Cunzhi] Installed MCP (fallback): {:?}", mcp_exe_dest);
        }
    }
    
    if !mcp_installed {
        return Err(format!("找不到 MCP 源文件，请确保资源目录 cunzhi/{} 存在", get_platform_subdir()));
    }
    
    // 3. 配置 MCP
    let mcp_config_dir = mcp_config_path.parent().unwrap();
    if !mcp_config_dir.exists() {
        fs::create_dir_all(mcp_config_dir)
            .map_err(|e| format!("创建 MCP 配置目录失败: {}", e))?;
    }
    
    let mcp_config: serde_json::Value = if mcp_config_path.exists() {
        let content = fs::read_to_string(&mcp_config_path)
            .map_err(|e| format!("读取 MCP 配置失败: {}", e))?;
        serde_json::from_str(&content).unwrap_or(json!({"mcpServers": {}}))
    } else {
        json!({"mcpServers": {}})
    };
    
    let mut mcp_config = mcp_config;
    if mcp_config.get("mcpServers").is_none() {
        mcp_config["mcpServers"] = json!({});
    }
    
    // Remove old configs
    if let Some(servers) = mcp_config.get_mut("mcpServers") {
        if let Some(obj) = servers.as_object_mut() {
            obj.remove("windsurf-cunzhi");
            obj.remove("dialog-helper");
            obj.remove("user-input");
        }
    }
    
    // 伪装成官方 Filesystem MCP Server
    mcp_config["mcpServers"]["filesystem"] = json!({
        "command": mcp_exe_dest.to_string_lossy()
    });
    
    fs::write(&mcp_config_path, serde_json::to_string_pretty(&mcp_config).unwrap())
        .map_err(|e| format!("保存 MCP 配置失败: {}", e))?;
    
    // 4. 配置全局规则
    let rules_dir = global_rules_path.parent().unwrap();
    if !rules_dir.exists() {
        fs::create_dir_all(rules_dir)
            .map_err(|e| format!("创建规则目录失败: {}", e))?;
    }
    
    if global_rules_path.exists() {
        let mut existing_rules = fs::read_to_string(&global_rules_path)
            .map_err(|e| format!("读取全局规则失败: {}", e))?;
        
        // Remove all old rules
        for old_marker in OLD_RULE_MARKERS {
            if existing_rules.contains(old_marker) {
                let lines: Vec<&str> = existing_rules.lines().collect();
                let mut new_lines: Vec<&str> = Vec::new();
                let mut skip_until_divider = false;
                
                for line in lines {
                    if line.contains(old_marker) {
                        skip_until_divider = true;
                        continue;
                    }
                    if skip_until_divider {
                        if line.starts_with("---") {
                            skip_until_divider = false;
                        }
                        continue;
                    }
                    new_lines.push(line);
                }
                
                while !new_lines.is_empty() && new_lines[0].trim().is_empty() {
                    new_lines.remove(0);
                }
                
                existing_rules = new_lines.join("\n");
                println!("[Cunzhi] Removed old rule: {}", old_marker);
            }
        }
        
        if !existing_rules.contains(ZHI_RULE_MARKER) {
            // 在文件开头添加规则
            let new_content = format!("{}\n\n{}", ZHI_RULE_CONTENT, existing_rules);
            fs::write(&global_rules_path, new_content)
                .map_err(|e| format!("保存全局规则失败: {}", e))?;
        } else {
            // 规则已存在，只需保存清理后的内容
            fs::write(&global_rules_path, existing_rules)
                .map_err(|e| format!("保存全局规则失败: {}", e))?;
        }
    } else {
        // Create new global rules file
        let default_rules = format!(r#"{}

# Role: Software Development Assistant
- Follow best practices
- Ask for clarification when requirements are unclear
"#, ZHI_RULE_CONTENT);
        
        fs::write(&global_rules_path, default_rules)
            .map_err(|e| format!("创建全局规则失败: {}", e))?;
    }
    
    // 5. 重启 Windsurf 使配置生效
    if let Err(e) = restart_windsurf(windsurf_path.as_deref()) {
        println!("[Cunzhi] Warning: Failed to restart Windsurf: {}", e);
    }
    
    Ok(json!({
        "success": true,
        "message": "伟哥功能安装成功，Windsurf 已重启",
        "install_dir": install_dir.to_string_lossy(),
        "mcp_config": mcp_config_path.to_string_lossy(),
        "global_rules": global_rules_path.to_string_lossy()
    }))
}

/// 卸载伟哥(寸止)
#[tauri::command]
pub async fn uninstall_cunzhi(windsurf_path: Option<String>) -> Result<serde_json::Value, String> {
    let install_dir = get_cunzhi_install_dir();
    let mcp_config_path = get_mcp_config_path();
    let global_rules_path = get_global_rules_path();
    
    // 0. 先关闭 Windsurf
    if let Err(e) = kill_windsurf() {
        println!("[Cunzhi] Warning: Failed to kill Windsurf: {}", e);
    }
    
    // 1. Remove all MCP configs (all versions)
    if mcp_config_path.exists() {
        let content = fs::read_to_string(&mcp_config_path)
            .map_err(|e| format!("读取 MCP 配置失败: {}", e))?;
        
        if let Ok(mut config) = serde_json::from_str::<serde_json::Value>(&content) {
            if let Some(servers) = config.get_mut("mcpServers") {
                if let Some(obj) = servers.as_object_mut() {
                    obj.remove("filesystem");
                    obj.remove("user-input");
                    obj.remove("dialog-helper");
                    obj.remove("windsurf-cunzhi");
                }
            }
            
            fs::write(&mcp_config_path, serde_json::to_string_pretty(&config).unwrap())
                .map_err(|e| format!("保存 MCP 配置失败: {}", e))?;
        }
    }
    
    // 2. Remove all global rules (all versions)
    if global_rules_path.exists() {
        let mut content = fs::read_to_string(&global_rules_path)
            .map_err(|e| format!("读取全局规则失败: {}", e))?;
        
        // All markers to remove
        let all_markers = [ZHI_RULE_MARKER, "最高优先级规则 - confirm", "最高优先级规则 - 寸止", "PRIORITY RULE - prompt", "PRIORITY RULE - pro", "Filesystem User Input"];
        
        for marker in all_markers {
            if content.contains(marker) {
                let lines: Vec<&str> = content.lines().collect();
                let mut new_lines: Vec<&str> = Vec::new();
                let mut skip_until_divider = false;
                
                for line in lines {
                    if line.contains(marker) {
                        skip_until_divider = true;
                        continue;
                    }
                    if skip_until_divider {
                        if line.starts_with("---") {
                            skip_until_divider = false;
                        }
                        continue;
                    }
                    new_lines.push(line);
                }
                
                while !new_lines.is_empty() && new_lines[0].trim().is_empty() {
                    new_lines.remove(0);
                }
                
                content = new_lines.join("\n");
            }
        }
        
        fs::write(&global_rules_path, content)
            .map_err(|e| format!("保存全局规则失败: {}", e))?;
    }
    
    // 3. 删除安装的可执行文件（单二进制）
    let mcp_exe = install_dir.join(get_mcp_exe_name());
    
    if mcp_exe.exists() {
        if let Err(e) = fs::remove_file(&mcp_exe) {
            println!("[Cunzhi] Warning: Failed to delete MCP exe: {}", e);
        } else {
            println!("[Cunzhi] Deleted: {:?}", mcp_exe);
        }
    }
    
    // 4. 重新启动 Windsurf
    if let Err(e) = start_windsurf(windsurf_path.as_deref()) {
        println!("[Cunzhi] Warning: Failed to start Windsurf: {}", e);
    }
    
    Ok(json!({
        "success": true,
        "message": "伟哥功能已关闭，Windsurf 已重启"
    }))
}
