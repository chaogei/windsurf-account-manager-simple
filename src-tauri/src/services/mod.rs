pub mod auth_service;
pub mod windsurf_service;
pub mod proto_parser;
pub mod analytics_service;

pub use auth_service::*;
pub use windsurf_service::*;
// pub use proto_parser::*;
pub use analytics_service::*;  // Not used directly yet, commented to avoid warnings

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{OnceLock, RwLock, Mutex};
use std::sync::Arc;

/// 全局共享的 HTTP 客户端，使用 RwLock 支持在故障时重建
static GLOBAL_HTTP_CLIENT: OnceLock<RwLock<Arc<reqwest::Client>>> = OnceLock::new();

/// 专门用于 googleapis 的 HTTP 客户端（支持代理）
static GOOGLE_API_CLIENT: OnceLock<RwLock<Arc<reqwest::Client>>> = OnceLock::new();

/// 代理配置缓存
static PROXY_CONFIG: OnceLock<Mutex<ProxyConfig>> = OnceLock::new();

/// 代理配置结构
#[derive(Clone, Default)]
struct ProxyConfig {
    enabled: bool,
    url: Option<String>,
}

/// 连续失败计数器，用于判断是否需要重建客户端
static CONSECUTIVE_FAILURES: AtomicU32 = AtomicU32::new(0);

/// 连续失败阈值，超过此值后重建HTTP客户端
const FAILURE_THRESHOLD: u32 = 3;  // 降低阈值，更快重建

/// 创建一个配置完善的 HTTP 客户端
fn create_http_client() -> reqwest::Client {
    reqwest::Client::builder()
        // 请求总超时：30秒
        .timeout(std::time::Duration::from_secs(30))
        // 连接超时：10秒（避免连接卡住）
        .connect_timeout(std::time::Duration::from_secs(10))
        // 连接池配置 - 更激进的配置以避免僵死连接
        .pool_max_idle_per_host(2)  // 减少空闲连接数
        .pool_idle_timeout(std::time::Duration::from_secs(30))  // 空闲连接保持30秒（缩短）
        // TCP keep-alive 配置
        .tcp_keepalive(std::time::Duration::from_secs(15))  // 缩短keep-alive间隔
        // 不自动重定向，避免无限重定向
        .redirect(reqwest::redirect::Policy::limited(5))
        // 禁用HTTP/2（某些服务器的HTTP/2实现有问题）
        .http1_only()
        // 禁用连接池复用（每次请求使用新连接，更可靠但稍慢）
        // .no_proxy()  // 如果需要完全禁用代理
        .build()
        .expect("Failed to create HTTP client")
}

/// 创建用于 googleapis 的 HTTP 客户端（支持代理）
fn create_google_api_client(proxy_url: Option<&str>) -> reqwest::Client {
    let mut builder = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .connect_timeout(std::time::Duration::from_secs(15))
        .pool_max_idle_per_host(2)
        .pool_idle_timeout(std::time::Duration::from_secs(30))
        .tcp_keepalive(std::time::Duration::from_secs(15))
        .redirect(reqwest::redirect::Policy::limited(5))
        .http1_only();
    
    // 如果提供了代理地址，配置代理
    if let Some(url) = proxy_url {
        if !url.is_empty() {
            match reqwest::Proxy::all(url) {
                Ok(proxy) => {
                    println!("[Google API Client] Using proxy: {}", url);
                    builder = builder.proxy(proxy);
                }
                Err(e) => {
                    println!("[Google API Client] Failed to parse proxy URL: {}", e);
                }
            }
        }
    }
    
    builder.build().expect("Failed to create Google API client")
}

/// 获取全局共享的 HTTP 客户端
pub fn get_http_client() -> Arc<reqwest::Client> {
    let client_lock = GLOBAL_HTTP_CLIENT.get_or_init(|| {
        RwLock::new(Arc::new(create_http_client()))
    });
    
    // 检查是否需要重建客户端
    let failures = CONSECUTIVE_FAILURES.load(Ordering::Relaxed);
    if failures >= FAILURE_THRESHOLD {
        // 尝试获取写锁重建客户端
        if let Ok(mut guard) = client_lock.try_write() {
            // 双重检查，避免重复重建
            if CONSECUTIVE_FAILURES.load(Ordering::Relaxed) >= FAILURE_THRESHOLD {
                println!("[HTTP Client] Rebuilding client after {} consecutive failures", failures);
                *guard = Arc::new(create_http_client());
                CONSECUTIVE_FAILURES.store(0, Ordering::Relaxed);
            }
        }
    }
    
    client_lock.read().unwrap().clone()
}

/// 报告请求成功，重置失败计数
pub fn report_request_success() {
    CONSECUTIVE_FAILURES.store(0, Ordering::Relaxed);
}

/// 报告请求失败，增加失败计数
pub fn report_request_failure() {
    let count = CONSECUTIVE_FAILURES.fetch_add(1, Ordering::Relaxed) + 1;
    println!("[HTTP Client] Request failed, consecutive failures: {}", count);
    
    // 如果达到阈值，立即触发重建
    if count >= FAILURE_THRESHOLD {
        println!("[HTTP Client] Threshold reached, triggering rebuild...");
        rebuild_http_client();
    }
}

/// 报告网络超时错误，立即重建客户端
pub fn report_timeout_error() {
    println!("[HTTP Client] Timeout error detected, forcing client rebuild");
    rebuild_http_client();
}

/// 强制重建HTTP客户端（用于手动恢复）
pub fn rebuild_http_client() {
    let client_lock = GLOBAL_HTTP_CLIENT.get_or_init(|| {
        RwLock::new(Arc::new(create_http_client()))
    });
    
    if let Ok(mut guard) = client_lock.write() {
        println!("[HTTP Client] Force rebuilding client");
        *guard = Arc::new(create_http_client());
        CONSECUTIVE_FAILURES.store(0, Ordering::Relaxed);
    }
}

/// 更新代理配置并重建 Google API 客户端
pub fn update_proxy_config(enabled: bool, url: Option<String>) {
    let config_lock = PROXY_CONFIG.get_or_init(|| {
        Mutex::new(ProxyConfig::default())
    });
    
    // 更新配置
    if let Ok(mut config) = config_lock.lock() {
        config.enabled = enabled;
        config.url = url.clone();
    }
    
    // 重建 Google API 客户端
    let client_lock = GOOGLE_API_CLIENT.get_or_init(|| {
        RwLock::new(Arc::new(create_google_api_client(None)))
    });
    
    if let Ok(mut guard) = client_lock.write() {
        let proxy_url = if enabled { url.as_deref() } else { None };
        println!("[Google API Client] Rebuilding with proxy: {:?}", proxy_url);
        *guard = Arc::new(create_google_api_client(proxy_url));
    }
}

/// 获取用于 googleapis 的 HTTP 客户端（支持代理）
pub fn get_google_api_client() -> Arc<reqwest::Client> {
    let config_lock = PROXY_CONFIG.get_or_init(|| {
        Mutex::new(ProxyConfig::default())
    });
    
    let (enabled, url) = {
        let config = config_lock.lock().unwrap();
        (config.enabled, config.url.clone())
    };
    
    let client_lock = GOOGLE_API_CLIENT.get_or_init(|| {
        let proxy_url = if enabled { url.as_deref() } else { None };
        RwLock::new(Arc::new(create_google_api_client(proxy_url)))
    });
    
    client_lock.read().unwrap().clone()
}
