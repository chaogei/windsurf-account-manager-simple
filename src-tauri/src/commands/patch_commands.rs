use tauri::command;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use regex::Regex;
use chrono::Local;
use std::sync::Arc;
use tauri::State;
use crate::repository::DataStore;

/// 获取 extension.js 相对路径（跨平台）
fn get_extension_js_relative_path() -> PathBuf {
    #[cfg(target_os = "macos")]
    {
        // macOS: Windsurf.app/Contents/Resources/app/extensions/windsurf/dist/extension.js
        PathBuf::from("Contents")
            .join("Resources")
            .join("app")
            .join("extensions")
            .join("windsurf")
            .join("dist")
            .join("extension.js")
    }
    #[cfg(not(target_os = "macos"))]
    {
        // Windows/Linux: resources/app/extensions/windsurf/dist/extension.js
        PathBuf::from("resources")
            .join("app")
            .join("extensions")
            .join("windsurf")
            .join("dist")
            .join("extension.js")
    }
}

/// 获取Windsurf的安装路径
#[command]
pub async fn get_windsurf_path() -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        // Windows: 首先尝试从开始菜单快捷方式获取
        let start_menu_path = std::env::var("APPDATA")
            .map(|p| PathBuf::from(p).join("Microsoft\\Windows\\Start Menu\\Programs\\Windsurf"))
            .ok();
        
        if let Some(start_menu) = start_menu_path {
            if let Ok(entries) = fs::read_dir(&start_menu) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().and_then(|s| s.to_str()) == Some("lnk") {
                        if let Ok(target) = resolve_shortcut(&path) {
                            if let Some(parent) = target.parent() {
                                let windsurf_root = parent.to_path_buf();
                                let extension_file = windsurf_root.join(get_extension_js_relative_path());
                                
                                if extension_file.exists() {
                                    return Ok(windsurf_root.to_string_lossy().to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // Windows: 尝试常见的安装位置
        let possible_locations = vec![
            std::env::var("LOCALAPPDATA").ok().map(|p| PathBuf::from(p).join("Programs\\Windsurf")),
            Some(PathBuf::from("C:\\Program Files\\Windsurf")),
            Some(PathBuf::from("C:\\Program Files (x86)\\Windsurf")),
            Some(PathBuf::from("D:\\Program\\Windsurf")),
        ];
        
        for location in possible_locations.into_iter().flatten() {
            let extension_file = location.join(get_extension_js_relative_path());
            if extension_file.exists() {
                return Ok(location.to_string_lossy().to_string());
            }
        }
    }
    
    #[cfg(target_os = "macos")]
    {
        // macOS: 检查 /Applications/Windsurf.app
        let possible_locations = vec![
            PathBuf::from("/Applications/Windsurf.app"),
            std::env::var("HOME").ok().map(|h| PathBuf::from(h).join("Applications/Windsurf.app")).unwrap_or_default(),
        ];
        
        for location in possible_locations {
            let extension_file = location.join(get_extension_js_relative_path());
            if extension_file.exists() {
                return Ok(location.to_string_lossy().to_string());
            }
        }
    }
    
    #[cfg(target_os = "linux")]
    {
        // Linux: 检查常见安装位置
        let possible_locations = vec![
            PathBuf::from("/opt/Windsurf"),
            PathBuf::from("/usr/share/windsurf"),
            std::env::var("HOME").ok().map(|h| PathBuf::from(h).join(".local/share/Windsurf")).unwrap_or_default(),
        ];
        
        for location in possible_locations {
            let extension_file = location.join(get_extension_js_relative_path());
            if extension_file.exists() {
                return Ok(location.to_string_lossy().to_string());
            }
        }
    }
    
    Err("未找到Windsurf安装路径".to_string())
}

/// 解析Windows快捷方式
#[cfg(target_os = "windows")]
fn resolve_shortcut(lnk_path: &Path) -> Result<PathBuf, String> {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x08000000;
    
    let output = Command::new("powershell")
        .creation_flags(CREATE_NO_WINDOW)
        .args(&[
            "-NoProfile",
            "-Command",
            &format!(
                "$sh = New-Object -ComObject WScript.Shell; $sh.CreateShortcut('{}').TargetPath",
                lnk_path.display()
            )
        ])
        .output()
        .map_err(|e| e.to_string())?;
    
    if output.status.success() {
        let target = String::from_utf8_lossy(&output.stdout)
            .trim()
            .to_string();
        
        if !target.is_empty() {
            Ok(PathBuf::from(target))
        } else {
            Err("快捷方式目标为空".to_string())
        }
    } else {
        Err("解析快捷方式失败".to_string())
    }
}

#[cfg(not(target_os = "windows"))]
#[allow(dead_code)]
fn resolve_shortcut(_lnk_path: &Path) -> Result<PathBuf, String> {
    Err("不支持的操作系统".to_string())
}

/// 应用无感换号补丁
#[command]
pub async fn apply_seamless_patch(
    windsurf_path: String,
    data_store: State<'_, Arc<DataStore>>,
) -> Result<serde_json::Value, String> {
    let extension_file = PathBuf::from(&windsurf_path).join(get_extension_js_relative_path());
    
    if !extension_file.exists() {
        return Err(format!("extension.js 文件不存在: {:?}", extension_file));
    }
    
    // 1. 管理备份文件（最多保留3份）
    let parent_dir = extension_file.parent()
        .ok_or("无法获取父目录")?;
    
    // 查找所有现有备份文件
    let mut backup_files: Vec<PathBuf> = fs::read_dir(parent_dir)
        .map_err(|e| format!("读取目录失败: {}", e))?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .map(|name| name.starts_with("extension.js.backup."))
                .unwrap_or(false)
        })
        .collect();
    
    // 按修改时间排序（最早的在前）
    backup_files.sort_by_key(|path| {
        fs::metadata(path)
            .and_then(|meta| meta.modified())
            .ok()
    });
    
    // 如果备份文件数量达到3个或更多，删除最早的备份
    while backup_files.len() >= 3 {
        if let Some(oldest) = backup_files.first() {
            fs::remove_file(oldest)
                .map_err(|e| format!("删除旧备份失败: {}", e))?;
            println!("删除旧备份文件: {:?}", oldest);
            backup_files.remove(0);
        } else {
            break;
        }
    }
    
    // 创建新的备份文件
    let backup_file = extension_file.with_extension(&format!(
        "js.backup.{}",
        Local::now().format("%Y%m%d_%H%M%S")
    ));
    
    fs::copy(&extension_file, &backup_file)
        .map_err(|e| format!("备份失败: {}", e))?;
    
    // 2. 读取文件内容
    let content = fs::read_to_string(&extension_file)
        .map_err(|e| format!("读取文件失败: {}", e))?;
    
    let mut modified_content = content.clone();
    let mut modifications = vec![];
    
    // 3. 应用修改1: 添加全局 OAuth 回调处理器
    let pattern1_str = r#"this\._uriHandler\.event\((\w+)=>\{"/refresh-authentication-session"===(\w+)\.path&&\(0,(\w+)\.refreshAuthenticationSession\)\(\)\}\)"#;
    let pattern1 = Regex::new(pattern1_str)
        .map_err(|e| format!("正则表达式错误: {}", e))?;
    
    if let Some(captures) = pattern1.captures(&modified_content) {
        let var_name1 = &captures[1];
        let var_name2 = &captures[2];
        let module_name = &captures[3];
        
        // 检查两个变量名是否相同
        if var_name1 == var_name2 {
            let replacement = format!(
                r#"this._uriHandler.event(async {}=>{{if("/refresh-authentication-session"==={}.path){{(0,{}.refreshAuthenticationSession)()}}else{{try{{const t=u.handleUri({});await this.handleAuthToken(t)}}catch(e){{console.error("[Windsurf] Failed to handle OAuth callback:",e)}}}}}})"#,
                var_name1, var_name1, module_name, var_name1
            );
            
            let full_match = captures.get(0).unwrap().as_str();
            modified_content = modified_content.replace(full_match, &replacement);
            modifications.push("OAuth回调处理器");
        }
    }
    
    // 4. 应用修改2: 移除180秒超时限制
    let pattern2_str = r#",new Promise\(\((\w+),(\w+)\)=>setTimeout\(\(\)=>\{(\w+)\(new (\w+)\)\},18e4\)\)"#;
    let pattern2 = Regex::new(pattern2_str)
        .map_err(|e| format!("正则表达式错误2: {}", e))?;
    
    if let Some(captures) = pattern2.captures(&modified_content) {
        let reject_var1 = &captures[2];  // 第二个参数
        let reject_var2 = &captures[3];  // setTimeout中的变量
        
        // 检查是否是同一个reject变量
        if reject_var1 == reject_var2 {
            let full_match = captures.get(0).unwrap().as_str();
            modified_content = modified_content.replace(full_match, "");
            modifications.push("移除超时限制");
        }
    }
    
    // 5. 验证修改
    if modified_content == content {
        // 如果内容没有变化，说明已经打过补丁
        return Ok(serde_json::json!({
            "success": true,
            "already_patched": true,
            "message": "补丁已经应用过了"
        }));
    }
    
    // 6. 写入修改后的文件
    fs::write(&extension_file, &modified_content)
        .map_err(|e| format!("写入文件失败: {}", e))?;
    
    // 7. 保存补丁状态到设置
    let mut settings = data_store.get_settings().await.map_err(|e| e.to_string())?;
    settings.seamless_switch_enabled = true;
    settings.windsurf_path = Some(windsurf_path.clone());
    settings.patch_backup_path = Some(backup_file.to_string_lossy().to_string());
    data_store.update_settings(settings).await.map_err(|e| e.to_string())?;
    
    // 8. 重启Windsurf
    restart_windsurf().await?;
    
    Ok(serde_json::json!({
        "success": true,
        "modifications": modifications,
        "backup_file": backup_file.to_string_lossy().to_string(),
        "message": "补丁应用成功，Windsurf正在重启"
    }))
}

/// 还原无感换号补丁
#[command]
pub async fn restore_seamless_patch(
    data_store: State<'_, Arc<DataStore>>,
) -> Result<serde_json::Value, String> {
    let settings = data_store.get_settings().await.map_err(|e| e.to_string())?;
    
    let windsurf_path = settings.windsurf_path
        .ok_or_else(|| "未找到Windsurf路径".to_string())?;
    
    let extension_file = PathBuf::from(&windsurf_path).join(get_extension_js_relative_path());
    let extension_dir = extension_file.parent()
        .ok_or("无法获取扩展目录")?
        .to_path_buf();
    
    // 尝试找到可用的备份文件
    let backup_path = find_latest_backup(&extension_dir, &settings.patch_backup_path)?;
    
    println!("使用备份文件还原: {:?}", backup_path);
    
    // 还原备份文件
    fs::copy(&backup_path, &extension_file)
        .map_err(|e| format!("还原失败: {} (备份文件: {:?})", e, backup_path))?;
    
    // 更新设置
    let mut settings = data_store.get_settings().await.map_err(|e| e.to_string())?;
    settings.seamless_switch_enabled = false;
    data_store.update_settings(settings).await.map_err(|e| e.to_string())?;
    
    // 重启Windsurf
    restart_windsurf().await?;
    
    Ok(serde_json::json!({
        "success": true,
        "message": "补丁已还原，Windsurf正在重启",
        "backup_used": backup_path.to_string_lossy().to_string()
    }))
}

/// 查找最新的可用备份文件
fn find_latest_backup(extension_dir: &Path, saved_backup_path: &Option<String>) -> Result<PathBuf, String> {
    // 1. 首先尝试使用设置中保存的备份路径
    if let Some(ref saved_path) = saved_backup_path {
        let saved = PathBuf::from(saved_path);
        if saved.exists() {
            return Ok(saved);
        }
        println!("设置中保存的备份文件不存在: {:?}", saved);
    }
    
    // 2. 查找目录中所有备份文件，按时间排序使用最新的
    let mut backup_files: Vec<PathBuf> = fs::read_dir(extension_dir)
        .map_err(|e| format!("读取目录失败: {}", e))?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .map(|name| name.starts_with("extension.js.backup."))
                .unwrap_or(false)
        })
        .collect();
    
    if backup_files.is_empty() {
        return Err("未找到任何备份文件，无法还原。请手动重新安装 Windsurf 或从官方下载 extension.js 文件".to_string());
    }
    
    // 按修改时间排序（最新的在前）
    backup_files.sort_by(|a, b| {
        let time_a = fs::metadata(a).and_then(|m| m.modified()).ok();
        let time_b = fs::metadata(b).and_then(|m| m.modified()).ok();
        time_b.cmp(&time_a)
    });
    
    // 返回最新的备份文件
    Ok(backup_files.remove(0))
}

/// 检查补丁状态
#[command]
pub async fn check_patch_status(
    windsurf_path: String,
) -> Result<serde_json::Value, String> {
    let extension_file = PathBuf::from(&windsurf_path).join(get_extension_js_relative_path());
    
    if !extension_file.exists() {
        return Ok(serde_json::json!({
            "installed": false,
            "error": "extension.js文件不存在"
        }));
    }
    
    let content = fs::read_to_string(&extension_file)
        .map_err(|e| format!("读取文件失败: {}", e))?;
    
    // 检查是否包含补丁标识
    let has_oauth_handler = content.contains("Failed to handle OAuth callback");
    let has_timeout_removed = !content.contains("18e4");
    
    Ok(serde_json::json!({
        "installed": has_oauth_handler,
        "oauth_handler": has_oauth_handler,
        "timeout_removed": has_timeout_removed
    }))
}

/// 重启Windsurf
async fn restart_windsurf() -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        
        // 1. 关闭Windsurf
        Command::new("taskkill")
            .creation_flags(CREATE_NO_WINDOW)
            .args(&["/F", "/IM", "Windsurf.exe"])
            .output()
            .map_err(|e| format!("关闭Windsurf失败: {}", e))?;
        
        // 等待进程完全结束
        std::thread::sleep(std::time::Duration::from_secs(2));
        
        // 2. 尝试从开始菜单启动Windsurf
        let start_menu = std::env::var("APPDATA")
            .map(|p| PathBuf::from(p).join("Microsoft\\Windows\\Start Menu\\Programs\\Windsurf"))
            .map_err(|e| format!("获取开始菜单路径失败: {}", e))?;
        
        if let Ok(entries) = fs::read_dir(&start_menu) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.file_name().and_then(|n| n.to_str()).map(|n| n.contains("Windsurf")).unwrap_or(false) 
                   && path.extension().and_then(|s| s.to_str()) == Some("lnk") {
                    
                    Command::new("cmd")
                        .creation_flags(CREATE_NO_WINDOW)
                        .args(&["/C", "start", "", &path.to_string_lossy()])
                        .spawn()
                        .map_err(|e| format!("启动Windsurf失败: {}", e))?;
                    
                    return Ok(());
                }
            }
        }
        
        return Err("未找到Windsurf快捷方式".to_string());
    }
    
    #[cfg(target_os = "macos")]
    {
        // 1. 关闭Windsurf
        Command::new("pkill")
            .args(&["-f", "Windsurf"])
            .output()
            .map_err(|e| format!("关闭Windsurf失败: {}", e))?;
        
        std::thread::sleep(std::time::Duration::from_secs(2));
        
        // 2. 启动Windsurf
        Command::new("open")
            .args(&["-a", "Windsurf"])
            .spawn()
            .map_err(|e| format!("启动Windsurf失败: {}", e))?;
        
        return Ok(());
    }
    
    #[cfg(target_os = "linux")]
    {
        // 1. 关闭Windsurf
        Command::new("pkill")
            .args(&["-f", "windsurf"])
            .output()
            .map_err(|e| format!("关闭Windsurf失败: {}", e))?;
        
        std::thread::sleep(std::time::Duration::from_secs(2));
        
        // 2. 启动Windsurf
        Command::new("windsurf")
            .spawn()
            .map_err(|e| format!("启动Windsurf失败: {}", e))?;
        
        return Ok(());
    }
    
    #[allow(unreachable_code)]
    Err("不支持的操作系统".to_string())
}
