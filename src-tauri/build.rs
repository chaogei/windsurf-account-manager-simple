use std::env;
use std::fs;
use std::path::Path;

/// 复制平台特定的 cunzhi 文件到打包目录
fn copy_platform_cunzhi() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    
    // 确定源目录名
    let platform_dir = match (target_os.as_str(), target_arch.as_str()) {
        ("windows", _) => "windows",
        ("macos", "x86_64") => "macos-x64",
        ("macos", "aarch64") => "macos-arm64",
        ("linux", _) => "linux",
        _ => {
            println!("cargo:warning=Unknown platform: {}-{}, skipping cunzhi copy", target_os, target_arch);
            return;
        }
    };
    
    let src_dir = Path::new("cunzhi").join(platform_dir);
    let dest_dir = Path::new("cunzhi-bundle");
    
    // 清理并重建目标目录
    if dest_dir.exists() {
        let _ = fs::remove_dir_all(dest_dir);
    }
    fs::create_dir_all(dest_dir).expect("Failed to create cunzhi-bundle directory");
    
    // 复制平台特定文件
    if src_dir.exists() {
        copy_dir_contents(&src_dir, dest_dir);
        println!("cargo:warning=Copied cunzhi files from {:?} to {:?}", src_dir, dest_dir);
    } else {
        println!("cargo:warning=Source directory {:?} not found", src_dir);
    }
    
    // 触发重新构建
    println!("cargo:rerun-if-changed=cunzhi/{}", platform_dir);
}

/// 递归复制目录内容
fn copy_dir_contents(src: &Path, dest: &Path) {
    if let Ok(entries) = fs::read_dir(src) {
        for entry in entries.flatten() {
            let path = entry.path();
            let file_name = path.file_name().unwrap();
            let dest_path = dest.join(file_name);
            
            if path.is_dir() {
                fs::create_dir_all(&dest_path).ok();
                copy_dir_contents(&path, &dest_path);
            } else {
                fs::copy(&path, &dest_path).ok();
            }
        }
    }
}

fn main() {
    // 复制平台特定的 cunzhi 文件
    copy_platform_cunzhi();
    
    // Windows特定配置：请求管理员权限
    #[cfg(windows)]
    {

        // 从 Cargo.toml 获取版本号，统一版本管理
        let version = env!("CARGO_PKG_VERSION");
        let version_with_build = format!("{}.0", version);

        // 通过环境变量控制是否需要管理员权限
        let require_admin = env::var("REQUIRE_ADMIN").unwrap_or_else(|_| "true".to_string());

        if require_admin == "true" {
            // 嵌入完整的管理员权限清单
            // 必须包含 dependency 和 compatibility 部分
            let manifest = format!(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
  <assemblyIdentity
    version="{}"
    processorArchitecture="*"
    name="com.chao.windsurf-account-manager"
    type="win32"
  />
  <description>Windsurf Account Manager</description>
  <dependency>
    <dependentAssembly>
      <assemblyIdentity
        type="win32"
        name="Microsoft.Windows.Common-Controls"
        version="6.0.0.0"
        processorArchitecture="*"
        publicKeyToken="6595b64144ccf1df"
        language="*"
      />
    </dependentAssembly>
  </dependency>
  <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
    <security>
      <requestedPrivileges>
        <requestedExecutionLevel level="requireAdministrator" uiAccess="false"/>
      </requestedPrivileges>
    </security>
  </trustInfo>
  <compatibility xmlns="urn:schemas-microsoft-com:compatibility.v1">
    <application>
      <supportedOS Id="{{e2011457-1546-43c5-a5fe-008deee3d3f0}}"/>
      <supportedOS Id="{{35138b9a-5d96-4fbd-8e2d-a2440225f93a}}"/>
      <supportedOS Id="{{4a2f28e3-53b9-4441-ba9c-d69d4a4a6e38}}"/>
      <supportedOS Id="{{1f676c76-80e1-4239-95bb-83d0f6d0da78}}"/>
      <supportedOS Id="{{8e0f7a12-bfb3-4fe8-b9a5-48fd50a15a9a}}"/>
    </application>
  </compatibility>
</assembly>"#, version_with_build);

            let windows = tauri_build::WindowsAttributes::new()
                .app_manifest(manifest);

            tauri_build::try_build(
                tauri_build::Attributes::new()
                    .windows_attributes(windows)
            ).expect("failed to run build script");
        } else {
            tauri_build::build();
        }
    }

    #[cfg(not(windows))]
    {
        tauri_build::build();
    }
}
